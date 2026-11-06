use super::*;

#[test]
fn unrelated_corruption_precedes_completion_lookup_receipt_and_clock_checks() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue
        .create(Create {
            timing: Timing::Once(10),
            task: Task::Deliver(DeliveryPayload {
                destination: Principal::self_authenticating(b"receiver"),
                message: "report".into(),
            }),
        })
        .unwrap();

    let corrupt = queue.create(increment(20)).unwrap().value;
    let delivery = queue.dispatch(10).unwrap().value.deliveries.remove(0);
    queue.corrupt_record(corrupt).unwrap();
    let before = queue.cell.get().as_slice().to_vec();
    for candidate in [
        Delivery {
            job: corrupt + 1,
            ..delivery.clone()
        },
        Delivery {
            sequence: delivery.sequence + 1,
            ..delivery.clone()
        },
        delivery,
    ] {
        assert_eq!(
            queue.finish(&candidate, 9, Outcome::Success),
            Err(Error::InvalidStorage)
        );
        assert_eq!(queue.cell.get().as_slice(), before);
    }
}

#[test]
fn corrupt_future_record_blocks_due_dispatch_and_recovery_without_reset() {
    let backing = VectorMemory::default();
    let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
    queue.create(increment(100)).unwrap();
    let corrupt = queue.create(increment(200)).unwrap().value;
    queue.corrupt_record(corrupt).unwrap();
    let bytes = queue.cell.get().as_slice().to_vec();
    assert!(queue.dispatch(100).is_err());
    assert_eq!(queue.cell.get().as_slice(), bytes);
    drop(queue);
    assert!(Queue::open(backing, None).is_err());
}

fn assert_invalid_record_blocks_queue(
    mut queue: Queue<VectorMemory>,
    backing: VectorMemory,
    now: u64,
    healthy: u64,
) {
    let bytes = queue.cell.get().as_slice().to_vec();
    let expected = job_error(ic_jobs::JobError::InvalidRecord);
    assert_eq!(queue.dispatch(now), Err(expected.clone()));
    assert_eq!(queue.summary(), Err(expected.clone()));
    assert_eq!(queue.inspect(healthy), Err(expected.clone()));
    assert_eq!(queue.cell.get().as_slice(), bytes);
    drop(queue);
    assert!(matches!(Queue::open(backing.clone(), None), Err(error) if error == expected));
    assert_eq!(retained_cell(backing, MAX_BYTES).get().as_slice(), bytes);
}

#[test]
fn contradictory_dispatch_counts_block_the_complete_stable_queue() {
    for uncertain in [false, true] {
        let backing = VectorMemory::default();
        let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
        let corrupt = queue
            .create(Create {
                timing: Timing::Once(100),
                task: Task::Deliver(DeliveryPayload {
                    destination: Principal::self_authenticating(b"receiver"),
                    message: "retained intent".into(),
                }),
            })
            .unwrap()
            .value;
        let healthy = queue.create(increment(200)).unwrap().value;
        let Committed {
            value: DispatchBatch { deliveries, .. },
            ..
        } = queue.dispatch(100).unwrap();
        if uncertain {
            queue
                .finish(&deliveries[0], 101, Outcome::Uncertain)
                .unwrap();
        }
        mutate_records(&mut queue, |snapshot| {
            let entry = snapshot
                .jobs
                .iter_mut()
                .find(|entry| entry.job.id == JobId(u128::from(corrupt)))
                .unwrap();
            entry.job.sequence += 1;
        });
        assert_invalid_record_blocks_queue(queue, backing, 200, healthy);
    }
}

#[test]
fn retained_timing_corruption_blocks_the_complete_stable_queue() {
    let mut retry = Job::new(
        JobId(2),
        Schedule::Once { at_ns: 100 },
        RetryPolicy::new(3, 10, 20).unwrap(),
    );
    let first = retry.start(100).unwrap();
    retry.finish(first, 100, Outcome::RetryableFailure).unwrap();
    let mut cancelled_retry = Job::restore(retry.record()).unwrap();
    cancelled_retry.cancel(105).unwrap();
    let mut cancelled_retry = cancelled_retry.record();
    cancelled_retry.last_outcome.as_mut().unwrap().at_ns = 99;
    retry.start(110).unwrap();
    let mut running_retry = retry.record();
    running_retry.state = JobState::Running;
    running_retry.updated_ns = 109;

    let mut recurring = Job::new(
        JobId(2),
        Schedule::AfterCompletion {
            first_at_ns: 100,
            every_ns: NonZeroU64::new(10).unwrap(),
        },
        RetryPolicy::NONE,
    );
    let first = recurring.start(100).unwrap();
    recurring.finish(first, 200, Outcome::Success).unwrap();
    let mut cancelled_recurring = Job::restore(recurring.record()).unwrap();
    cancelled_recurring.cancel(300).unwrap();
    let mut cancelled_recurring = cancelled_recurring.record();
    cancelled_recurring.scheduled_ns = 110;
    recurring.start(300).unwrap();
    let mut running_recurring = recurring.record();
    running_recurring.scheduled_ns = 110;

    for corrupt in [
        cancelled_retry,
        running_retry,
        cancelled_recurring,
        running_recurring,
    ] {
        let backing = VectorMemory::default();
        let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
        let healthy = queue.create(increment(100)).unwrap().value;
        queue.create(increment(200)).unwrap();
        mutate_records(&mut queue, |snapshot| snapshot.jobs[1].job = corrupt);
        assert_invalid_record_blocks_queue(queue, backing, 100, healthy);
    }
}

#[test]
fn snapshot_admission_preserves_restore_and_collection_error_order() {
    type Mutation = fn(&mut Snapshot);
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue.create(increment(100)).unwrap();
    queue.create(increment(200)).unwrap();
    let bytes = queue.cell.get().as_slice().to_vec();
    let cases: [(&str, Mutation, Error); 4] = [
        (
            "header before restore",
            |snapshot| {
                snapshot.jobs[0].job.state = JobState::Completed;
                snapshot.next_id = 0;
            },
            Error::InvalidStorage,
        ),
        (
            "restore before entry identity",
            |snapshot| {
                snapshot.jobs[0].job.state = JobState::Completed;
                snapshot.jobs[0].job.id = JobId(0);
            },
            job_error(ic_jobs::JobError::InvalidRecord),
        ),
        (
            "earlier payload before later restore",
            |snapshot| {
                snapshot.jobs[1].job.state = JobState::Completed;
                snapshot.jobs[0].task = Task::Deliver(DeliveryPayload {
                    destination: manager(),
                    message: "x".repeat(MAX_MESSAGE + 1),
                });
            },
            Error::InvalidStorage,
        ),
        (
            "restore before receipts",
            |snapshot| {
                snapshot.jobs[0].job.state = JobState::Completed;
                snapshot.receipts.push(Delivery {
                    job: 0,
                    occurrence: 0,
                    sequence: 0,
                    payload: DeliveryPayload {
                        destination: manager(),
                        message: String::new(),
                    },
                });
            },
            job_error(ic_jobs::JobError::InvalidRecord),
        ),
    ];
    for (name, mutate, expected) in cases {
        let mut snapshot: Snapshot = stored_records(&bytes).unwrap();
        mutate(&mut snapshot);
        assert_eq!(
            decode(&encode(&snapshot).unwrap()).map(|_| ()),
            Err(expected),
            "{name}"
        );
    }
    assert_eq!(queue.cell.get().as_slice(), bytes);
}

#[test]
fn view_projection_preserves_large_ids_and_refuses_unadmitted_identities() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue
        .create(Create {
            timing: Timing::Once(100),
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "large identity".into(),
            }),
        })
        .unwrap();
    let id = u64::MAX - 1;
    mutate_records(&mut queue, |snapshot| {
        snapshot.next_id = u64::MAX;
        snapshot.jobs[0].job.id = JobId(u128::from(id));
    });
    assert_eq!(queue.inspect(id).unwrap().id, id);
    assert_eq!(queue.page(None, 8).unwrap()[0].id, id);
    let delivery = queue.dispatch(100).unwrap().value.deliveries.pop().unwrap();
    assert_eq!(delivery.job, id);
    assert_eq!(queue.outstanding(id).unwrap(), delivery);
    // Refused identities must never produce a view or delivery from retained intent.
    let mut snapshot: Snapshot = stored_records(queue.cell.get().as_slice()).unwrap();

    for invalid_id in [u128::from(u64::MAX), u128::from(u64::MAX) + 1, u128::MAX] {
        snapshot.jobs[0].job.id = JobId(invalid_id);
        let bytes = encode(&snapshot).unwrap();
        queue.cell.set(bytes.clone()).unwrap();
        assert_eq!(queue.inspect(id), Err(Error::InvalidStorage));
        assert_eq!(queue.page(None, 8), Err(Error::InvalidStorage));
        assert_eq!(queue.outstanding(id), Err(Error::InvalidStorage));
        assert_eq!(queue.cell.get().as_slice(), bytes);
    }
}

#[test]
fn typed_commit_refuses_invalid_collection_metadata_without_changing_bytes() {
    type Mutation = fn(&mut Snapshot<Job>, &Delivery);
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue.create(increment(100)).unwrap();
    let receipt = Delivery {
        job: 1,
        occurrence: 0,
        sequence: 1,
        payload: DeliveryPayload {
            destination: manager(),
            message: "confirmed".into(),
        },
    };
    let cases: [(&str, Mutation); 8] = [
        ("zero allocator", |loaded, _| loaded.next_id = 0),
        ("allocator reuses identity", |loaded, _| loaded.next_id = 1),
        ("duplicate job", |loaded, _| {
            loaded.jobs.push(Entry {
                job: Job::restore(loaded.jobs[0].job.record()).unwrap(),
                task: Task::Increment(1),
            });
        }),
        ("oversized task payload", |loaded, _| {
            loaded.jobs[0].task = Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "x".repeat(MAX_MESSAGE + 1),
            })
        }),
        ("zero receipt job", |loaded, receipt| {
            loaded.receipts.push(Delivery {
                job: 0,
                ..receipt.clone()
            })
        }),
        ("zero receipt sequence", |loaded, receipt| {
            loaded.receipts.push(Delivery {
                sequence: 0,
                ..receipt.clone()
            })
        }),
        ("oversized receipt payload", |loaded, receipt| {
            loaded.receipts.push(Delivery {
                payload: DeliveryPayload {
                    message: "x".repeat(MAX_MESSAGE + 1),
                    ..receipt.payload.clone()
                },
                ..receipt.clone()
            })
        }),
        ("duplicate logical receipt", |loaded, receipt| {
            loaded.receipts.push(receipt.clone());
            loaded.receipts.push(Delivery {
                sequence: 2,
                ..receipt.clone()
            });
        }),
    ];
    for (name, mutate) in cases {
        let before = queue.cell.get().as_slice().to_vec();
        let mut loaded = queue.load().unwrap();
        mutate(&mut loaded, &receipt);
        assert_eq!(
            queue.commit(loaded, ()),
            Err(Error::InvalidStorage),
            "{name}"
        );
        assert_eq!(queue.cell.get().as_slice(), before, "{name}");
    }
    // Encoded capacity still takes precedence over invalid collection metadata.
    let before = queue.cell.get().as_slice().to_vec();
    let mut loaded = queue.load().unwrap();
    loaded.next_id = 0;
    loaded.jobs[0].task = Task::Deliver(DeliveryPayload {
        destination: manager(),
        message: "x".repeat(MAX_BYTES),
    });
    assert_eq!(queue.commit(loaded, ()), Err(Error::Capacity));
    assert_eq!(queue.cell.get().as_slice(), before);
}

#[test]
fn oversized_retained_cell_is_refused_without_replacing_its_payload() {
    let backing = VectorMemory::default();
    let queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
    drop(queue);
    // Load with a larger fixture ceiling; the consumer's normal recovery limit
    // must still refuse the retained oversized payload before allocation.
    let mut oversized = retained_cell(backing.clone(), MAX_BYTES + 1);
    oversized.set(vec![b'x'; MAX_BYTES + 1]).unwrap();
    drop(oversized);
    let memory = queue_memory(backing.clone()).unwrap();
    let mut before = vec![0; ic_storage::STABLE_CELL_HEADER_SIZE + MAX_BYTES + 1];
    memory.read(0, &mut before);
    drop(memory);
    for initial in [None, Some(config())] {
        assert!(matches!(
            Queue::open(backing.clone(), initial),
            Err(Error::InvalidStorage)
        ));
        let memory = queue_memory(backing.clone()).unwrap();
        let mut after = vec![0; before.len()];
        memory.read(0, &mut after);
        // Bootstrap may update allocation metadata. The refused cell's header
        // and payload must retain their exact bytes across both entry paths.
        assert_eq!(after, before);
    }
}

#[test]
fn retained_queue_identity_corruption_is_refused_without_writes_or_reset() {
    let backing = VectorMemory::default();
    drop(Queue::open(backing.clone(), Some(config())).unwrap());
    // Infrastructure fault injection uses an exclusive checked lease after
    // dropping the cell. Storage has no checksum; corrupt the queue-owned
    // identity so the consumer must refuse before restoring any authority.
    let memory = queue_memory(backing.clone()).unwrap();
    let offset = ic_storage::STABLE_CELL_HEADER_SIZE as u64;
    let mut byte = [0];
    memory.read(offset, &mut byte);
    byte[0] ^= 1;
    memory.write(offset, &byte);
    drop(memory);
    let before = backing.borrow().clone();
    let memory = MeteredMemory::new(backing.clone());
    for initial in [None, Some(config())] {
        assert!(matches!(
            Queue::open(memory.clone(), initial),
            Err(Error::InvalidStorage)
        ));
        assert_eq!(memory.counts().writes, 0);
        assert!(!memory.counts().overflowed);
        assert_eq!(*backing.borrow(), before);
    }
}

#[test]
fn recovery_of_empty_backing_does_not_initialize_replacement_storage() {
    let backing = VectorMemory::default();
    let memory = MeteredMemory::new(backing.clone());
    assert!(matches!(
        Queue::open(memory.clone(), None),
        Err(Error::InvalidStorage)
    ));
    assert_eq!(memory.counts(), Default::default());
    assert!(backing.borrow().is_empty());
}
