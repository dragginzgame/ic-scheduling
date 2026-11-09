use super::*;

#[test]
fn running_and_uncertain_survive_reopen_and_require_exact_receipts() {
    for uncertain in [false, true] {
        let backing = VectorMemory::default();
        let destination = Principal::self_authenticating(b"receiver");
        let mut sender = fresh_queue(backing.clone());
        let id = sender
            .create(Create {
                timing: Timing::Once(100),
                task: Task::Deliver(DeliveryPayload {
                    destination,
                    message: "report".into(),
                }),
            })
            .unwrap()
            .value;
        let Committed {
            value: DispatchBatch { deliveries, .. },
            ..
        } = sender.dispatch(100).unwrap();
        let delivery = &deliveries[0];
        if uncertain {
            sender.finish(delivery, 101, Outcome::Uncertain).unwrap();
        }
        drop(sender);
        let mut sender = Queue::open(backing, None).unwrap().0;
        assert_eq!(sender.outstanding(id).unwrap(), *delivery);
        assert!(sender.cancel(id, 102).is_err());
        assert_eq!(sender.dispatch(102).unwrap(), local_batch(0, None));
        let receiver_memory = VectorMemory::default();
        let mut receiver = Queue::open(
            receiver_memory.clone(),
            Some(Init {
                manager: manager(),
                delivery_sender: Some(manager()),
            }),
        )
        .unwrap()
        .0;
        assert_eq!(receiver.receipt(manager(), delivery).unwrap(), None);
        assert_eq!(
            receiver.receive(Principal::anonymous(), delivery.clone()),
            Err(Error::Unauthorized)
        );
        receiver.receive(manager(), delivery.clone()).unwrap();
        receiver.receive(manager(), delivery.clone()).unwrap();
        drop(receiver);
        let receiver = Queue::open(receiver_memory, None).unwrap().0;
        assert_eq!(receiver.summary().unwrap().value.receipts, 1);
        assert_eq!(
            receiver.receipt(manager(), delivery).unwrap(),
            Some(delivery.clone())
        );
        let mut stale = delivery.clone();
        stale.sequence += 1;
        let before = sender.cell.get().as_slice().to_vec();
        assert_eq!(
            sender.finish(&stale, 102, Outcome::Success),
            Err(Error::ReceiptMismatch)
        );
        assert_eq!(sender.cell.get().as_slice(), before);
        sender.finish(delivery, 102, Outcome::Success).unwrap();
        assert_eq!(sender.inspect(id).unwrap().status, Status::Completed);
        assert_eq!(sender.dispatch(103).unwrap().value.work, 0);
    }
}

#[test]
fn completion_refusals_preserve_input_receipt_and_clock_precedence() {
    type Mutation = fn(&mut Delivery);

    let destination = Principal::self_authenticating(b"receiver");
    for task in [
        Task::Increment(1),
        Task::Deliver(DeliveryPayload {
            destination,
            message: "report".into(),
        }),
    ] {
        for state in [
            JobState::Pending { due_ns: 10 },
            JobState::Paused { due_ns: 10 },
            JobState::Running,
            JobState::Uncertain { started_ns: 10 },
            JobState::Completed,
            JobState::Failed,
            JobState::Cancelled,
        ] {
            let mut queue = fresh_queue(VectorMemory::default());
            let id = queue
                .create(Create {
                    timing: Timing::Once(10),
                    task: task.clone(),
                })
                .unwrap()
                .value;
            let mut snapshot = queue.load().unwrap();
            let job = &mut snapshot.jobs[0].job;
            set_job_state(job, state, 10);
            let record = job.record();
            queue.commit(snapshot, ()).unwrap();
            let delivery = Delivery {
                job: id,
                occurrence: record.occurrence,
                sequence: record.sequence,
                payload: DeliveryPayload {
                    destination,
                    message: "report".into(),
                },
            };
            let outstanding = matches!(task, Task::Deliver(_))
                && matches!(state, JobState::Running | JobState::Uncertain { .. });
            let before = queue.cell.get().as_slice().to_vec();
            let mismatch = if outstanding {
                Error::ReceiptMismatch
            } else {
                Error::InvalidInput
            };
            let mutations: [(&str, Mutation, Error); 6] = [
                (
                    "occurrence",
                    |value| value.occurrence += 1,
                    mismatch.clone(),
                ),
                ("sequence", |value| value.sequence += 1, mismatch.clone()),
                (
                    "destination",
                    |value| value.payload.destination = Principal::anonymous(),
                    mismatch.clone(),
                ),
                ("job", |value| value.job += 1, Error::UnknownJob),
                (
                    "message",
                    |value| value.payload.message = "changed".into(),
                    mismatch,
                ),
                (
                    "unchanged",
                    |_| {},
                    if outstanding {
                        job_error(ic_jobs::JobError::TimeWentBackwards)
                    } else {
                        Error::InvalidInput
                    },
                ),
            ];
            for (name, mutate, expected) in mutations {
                let mut candidate = delivery.clone();
                mutate(&mut candidate);
                assert_eq!(
                    queue.finish(&candidate, 9, Outcome::Success),
                    Err(expected),
                    "{name}: {task:?}, {state:?}"
                );
                assert_eq!(
                    queue.cell.get().as_slice(),
                    before,
                    "{name}: stable bytes changed"
                );
            }
        }
    }
}

#[test]
fn receipt_extraction_and_duplicate_acknowledgement_leave_storage_unchanged() {
    let memory = MeteredMemory::new(VectorMemory::default());
    let mut queue = Queue::open(
        memory.clone(),
        Some(Init {
            manager: manager(),
            delivery_sender: Some(manager()),
        }),
    )
    .unwrap()
    .0;
    let receipts: Vec<_> = [10, 20, 30]
        .into_iter()
        .map(|job| Delivery {
            job,
            occurrence: 0,
            sequence: 1,
            payload: DeliveryPayload {
                destination: manager(),
                message: format!("payload-{job}"),
            },
        })
        .collect();
    for receipt in &receipts {
        assert_eq!(queue.receive(manager(), receipt.clone()).unwrap(), *receipt);
    }
    let before = queue.cell.get().as_slice().to_vec();
    memory.reset();
    for receipt in receipts.iter().rev() {
        assert_eq!(
            queue.receipt(manager(), receipt).unwrap(),
            Some(receipt.clone())
        );
        assert_eq!(queue.receive(manager(), receipt.clone()).unwrap(), *receipt);
    }
    let mut mismatched = receipts[1].clone();
    mismatched.sequence += 1;
    assert_eq!(
        queue.receive(manager(), mismatched.clone()),
        Err(Error::ReceiptMismatch)
    );
    // Queries return evidence by logical key; the sender checks its full envelope.
    assert_eq!(
        queue.receipt(manager(), &mismatched).unwrap(),
        Some(receipts[1].clone())
    );
    let mut absent = receipts[0].clone();
    absent.job = 99;
    assert_eq!(queue.receipt(manager(), &absent).unwrap(), None);
    assert_eq!(
        queue.receipt(Principal::anonymous(), &absent),
        Err(Error::Unauthorized)
    );
    assert_eq!(queue.summary().unwrap().value.receipts, 3);
    assert!(!memory.counts().overflowed);
    assert_eq!(memory.counts().writes, 0);
    assert_eq!(queue.cell.get().as_slice(), before);
    drop(queue);
    let mut queue = Queue::open(memory, None).unwrap().0;
    assert_eq!(queue.cell.get().as_slice(), before);
    assert_eq!(queue.summary().unwrap().value.receipts, 3);
    // Both receiver entrypoints reject an unauthenticated caller before decoding
    // corrupt private storage, and neither refusal replaces the retained bytes.
    queue.cell.set(b"not a snapshot".to_vec());
    let corrupt = queue.cell.get().as_slice().to_vec();
    assert_eq!(
        queue.receive(Principal::anonymous(), receipts[0].clone()),
        Err(Error::Unauthorized)
    );
    assert_eq!(
        queue.receipt(Principal::anonymous(), &receipts[0]),
        Err(Error::Unauthorized)
    );
    assert_eq!(queue.cell.get().as_slice(), corrupt);
}

#[test]
fn retained_retry_preserves_payload_and_rejects_late_results() {
    let mut queue = fresh_queue(VectorMemory::default());
    let id = queue
        .create(Create {
            timing: Timing::Once(100),
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "report".into(),
            }),
        })
        .unwrap()
        .value;
    // Creation defaults to no retries. Qualify an admitted retained job whose
    // consumer previously selected a retry budget, without adding creation policy.
    let mut loaded = queue.load().unwrap();
    loaded.jobs[0].job = Job::new(
        JobId(u128::from(id)),
        Schedule::Once { at_ns: 100 },
        RetryPolicy::new(2, 10, 10).unwrap(),
    );
    queue.commit(loaded, ()).unwrap();
    let first = queue.dispatch(100).unwrap().value.deliveries.remove(0);
    queue
        .finish(&first, 101, Outcome::RetryableFailure)
        .unwrap();
    assert_eq!(queue.summary().unwrap().next_due_ns, Some(111));
    assert_eq!(queue.dispatch(110).unwrap(), local_batch(0, Some(111)));
    let second = queue.dispatch(111).unwrap().value.deliveries.remove(0);
    assert_eq!(first.job, second.job);
    assert_eq!(first.occurrence, second.occurrence);
    assert_ne!(first.sequence, second.sequence);
    assert_eq!(first.payload, second.payload);
    let before = queue.cell.get().as_slice().to_vec();
    assert_eq!(
        queue.finish(&first, 112, Outcome::Success),
        Err(Error::ReceiptMismatch)
    );
    assert_eq!(queue.cell.get().as_slice(), before);
    queue.finish(&second, 112, Outcome::Success).unwrap();
    assert_eq!(queue.inspect(id).unwrap().status, Status::Completed);
    assert_eq!(queue.dispatch(1000).unwrap(), local_batch(0, None));
}

#[test]
fn queue_order_and_cancellation_progress_past_unresolved_delivery() {
    let backing = VectorMemory::default();
    let mut queue = fresh_queue(backing.clone());
    for at in [100, 90, 95, 90] {
        queue
            .create(Create {
                timing: Timing::Once(at),
                task: Task::Deliver(DeliveryPayload {
                    destination: manager(),
                    message: format!("report-{at}"),
                }),
            })
            .unwrap();
    }
    assert_eq!(queue.dispatch(89).unwrap(), local_batch(0, Some(90)));
    let deliveries = queue.dispatch(90).unwrap().value.deliveries;
    assert_eq!(
        deliveries.iter().map(|d| d.job).collect::<Vec<_>>(),
        vec![2, 4]
    );
    drop(queue);
    let mut queue = Queue::open(backing, None).unwrap().0;
    queue
        .finish(&deliveries[0], 91, Outcome::Uncertain)
        .unwrap();
    queue.finish(&deliveries[1], 91, Outcome::Success).unwrap();
    assert_eq!(queue.summary().unwrap().next_due_ns, Some(95));
    let before = queue.cell.get().as_slice().to_vec();
    assert!(queue.cancel(2, 91).is_err());
    assert_eq!(queue.cell.get().as_slice(), before);
    queue.cancel(3, 91).unwrap();
    assert_eq!(queue.dispatch(99).unwrap(), local_batch(0, Some(100)));
    let last = queue.dispatch(100).unwrap().value.deliveries.remove(0);
    assert_eq!(last.job, 1);
    queue.finish(&last, 101, Outcome::Success).unwrap();
    assert_eq!(queue.dispatch(1000).unwrap(), local_batch(0, None));
    assert_eq!(queue.outstanding(2).unwrap(), deliveries[0]);
}
