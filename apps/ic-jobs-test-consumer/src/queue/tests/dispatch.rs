use super::*;

#[test]
fn local_effects_and_bounded_overdue_work_commit_together() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
    let id = queue
        .create(Create {
            timing: Timing::FixedRate {
                first_ns: 100,
                every_ns: 10,
                catch_up: true,
            },
            task: Task::Increment(2),
        })
        .unwrap()
        .value;
    assert_eq!(
        queue.dispatch(125).unwrap(),
        Committed {
            value: DispatchBatch {
                deliveries: vec![],
                work: 2
            },
            next_due_ns: Some(120)
        }
    );
    assert_eq!(
        queue.summary().unwrap(),
        Committed {
            value: QueueTotals {
                counter: 4,
                receipts: 0
            },
            next_due_ns: Some(120)
        }
    );
    drop(queue);
    let mut queue = Queue::open(memory, None).unwrap().0;
    assert_eq!(queue.inspect(id).unwrap().occurrence, 2);
    queue.dispatch(125).unwrap();
    assert_eq!(
        queue.summary().unwrap(),
        Committed {
            value: QueueTotals {
                counter: 6,
                receipts: 0
            },
            next_due_ns: Some(130)
        }
    );
    queue.cancel(id, 125).unwrap();
    assert_eq!(queue.summary().unwrap().next_due_ns, None);
}

#[test]
fn known_local_failure_is_terminal_and_preserves_bounded_batch_progress() {
    for failed_first in [false, true] {
        let memory = VectorMemory::default();
        let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
        queue.create(increment(0)).unwrap();
        assert_eq!(
            queue.dispatch(0).unwrap(),
            Committed {
                value: DispatchBatch {
                    deliveries: vec![],
                    work: 1
                },
                next_due_ns: None
            }
        );

        let failed = Create {
            timing: Timing::FixedRate {
                first_ns: 100,
                every_ns: 10,
                catch_up: true,
            },
            task: Task::Increment(u64::MAX),
        };
        let healthy = Create {
            timing: Timing::Once(100),
            task: Task::Increment(2),
        };
        let (failed, healthy) = if failed_first {
            (
                queue.create(failed).unwrap().value,
                queue.create(healthy).unwrap().value,
            )
        } else {
            let healthy = queue.create(healthy).unwrap().value;
            (queue.create(failed).unwrap().value, healthy)
        };
        let remaining = queue.create(increment(100)).unwrap().value;
        assert_eq!(
            queue.dispatch(100).unwrap(),
            Committed {
                value: DispatchBatch {
                    deliveries: vec![],
                    work: 2
                },
                next_due_ns: Some(100)
            }
        );
        assert_eq!(
            queue.summary().unwrap(),
            Committed {
                value: QueueTotals {
                    counter: 3,
                    receipts: 0
                },
                next_due_ns: Some(100)
            }
        );
        assert_eq!(queue.inspect(failed).unwrap().status, Status::Failed);
        assert_eq!(queue.inspect(healthy).unwrap().status, Status::Completed);
        assert_eq!(
            queue.inspect(remaining).unwrap().status,
            Status::Pending { due_ns: 100 }
        );
        let failed_record = queue.inspect(failed).unwrap();
        assert_eq!(failed_record.sequence, 1);
        assert_eq!(failed_record.occurrence, 0);

        drop(queue);
        let mut restored = Queue::open(memory, None).unwrap().0;
        assert_eq!(restored.inspect(failed).unwrap(), failed_record);
        assert_eq!(
            restored.dispatch(100).unwrap(),
            Committed {
                value: DispatchBatch {
                    deliveries: vec![],
                    work: 1
                },
                next_due_ns: None
            }
        );
        assert_eq!(
            restored.summary().unwrap(),
            Committed {
                value: QueueTotals {
                    counter: 4,
                    receipts: 0
                },
                next_due_ns: None
            }
        );
        assert_eq!(restored.inspect(failed).unwrap(), failed_record);
        assert_eq!(
            restored.dispatch(200).unwrap(),
            Committed {
                value: DispatchBatch {
                    deliveries: vec![],
                    work: 0
                },
                next_due_ns: None
            }
        );
        assert_eq!(
            restored.summary().unwrap(),
            Committed {
                value: QueueTotals {
                    counter: 4,
                    receipts: 0
                },
                next_due_ns: None
            }
        );
    }
}

#[test]
fn idle_dispatch_validates_the_complete_snapshot_without_stable_writes() {
    for state in [
        None,
        Some((JobState::Pending { due_ns: 100 }, 0)),
        Some((JobState::Paused { due_ns: 100 }, 0)),
        Some((JobState::Running, 100)),
        Some((JobState::Uncertain { started_ns: 100 }, 100)),
        Some((JobState::Completed, 100)),
    ] {
        let memory = MeteredMemory::new(VectorMemory::default());
        let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
        let id = state.map(|_| queue.create(increment(100)).unwrap().value);
        let mut loaded = queue.load().unwrap();
        if let Some((target, at_ns)) = state {
            set_job_state(&mut loaded.jobs[0].job, target, at_ns);
        }
        queue.commit(loaded, ()).unwrap();
        let expected_due = queue.summary().unwrap().next_due_ns;
        let before = queue.cell.get().as_slice().to_vec();
        memory.reset();
        assert_eq!(
            queue.dispatch(99).unwrap(),
            Committed {
                value: DispatchBatch {
                    deliveries: vec![],
                    work: 0
                },
                next_due_ns: expected_due
            },
            "state {state:?}"
        );
        assert!(!memory.counts().overflowed);
        assert_eq!(memory.counts().writes, 0, "state {state:?}");
        assert_eq!(queue.cell.get().as_slice(), before, "state {state:?}");
        if let Some(id) = id {
            queue.corrupt_record(id).unwrap();
            let before = queue.cell.get().as_slice().to_vec();
            memory.reset();
            assert!(queue.dispatch(99).is_err(), "state {state:?}");
            assert!(!memory.counts().overflowed);
            assert_eq!(memory.counts().writes, 0);
            assert_eq!(queue.cell.get().as_slice(), before);
        }
    }
    // Real due work still writes its metadata and local effect together.
    let memory = MeteredMemory::new(VectorMemory::default());
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
    queue.create(increment(100)).unwrap();
    memory.reset();
    assert_eq!(
        queue.dispatch(100).unwrap(),
        Committed {
            value: DispatchBatch {
                deliveries: vec![],
                work: 1
            },
            next_due_ns: None
        }
    );
    assert!(!memory.counts().overflowed);
    assert!(memory.counts().writes > 0);
    assert_eq!(queue.summary().unwrap().value.counter, 1);
}

#[test]
fn opening_returns_the_admitted_deadline_without_rewriting_retained_records() {
    let memory = VectorMemory::default();
    let (mut queue, due) = Queue::open(memory.clone(), Some(config())).unwrap();
    assert_eq!(due, None);
    queue.create(increment(200)).unwrap();
    let first = queue.create(increment(100)).unwrap().value;
    let before = queue.cell.get().as_slice().to_vec();
    drop(queue);
    let (mut queue, due) = Queue::open(memory.clone(), None).unwrap();
    assert_eq!(due, Some(100));
    assert_eq!(queue.cell.get().as_slice(), before);
    queue.pause(first, 0).unwrap();
    drop(queue);
    let (mut queue, due) = Queue::open(memory.clone(), None).unwrap();
    assert_eq!(due, Some(200));
    queue.corrupt_record(first).unwrap();
    drop(queue);
    // Corruption in inactive work still blocks the entire restoration result.
    assert!(Queue::open(memory, None).is_err());
}
