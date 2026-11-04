use super::*;

#[test]
fn running_and_uncertain_survive_reopen_and_require_exact_receipts() {
    for uncertain in [false, true] {
        let backing = VectorMemory::default();
        let destination = Principal::self_authenticating(b"receiver");
        let mut sender = Queue::open(backing.clone(), Some(config())).unwrap().0;
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
        assert_eq!(
            sender.dispatch(102).unwrap(),
            Committed {
                value: DispatchBatch {
                    deliveries: vec![],
                    work: 0
                },
                next_due_ns: None
            }
        );
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
            let mut queue = Queue::open(VectorMemory::default(), Some(config()))
                .unwrap()
                .0;
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
    queue.cell.set(b"not a snapshot".to_vec()).unwrap();
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
