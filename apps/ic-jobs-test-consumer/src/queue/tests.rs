use super::*;
use ic_memory::{BackingIdentity, VectorMemory, testing::memory_from_sparse_hex};
use ic_testkit_native::memory::MeteredMemory;

// The test host supplies physical identity; Tooling still owns IO accounting.
#[derive(Clone)]
struct MeteredBacking(MeteredMemory);

impl MeteredBacking {
    fn new(backing: VectorMemory) -> Self {
        Self(MeteredMemory::new(backing))
    }
}

impl std::ops::Deref for MeteredBacking {
    type Target = MeteredMemory;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Memory for MeteredBacking {
    fn size(&self) -> u64 {
        self.0.size()
    }

    fn grow(&self, pages: u64) -> i64 {
        self.0.grow(pages)
    }

    fn read(&self, offset: u64, bytes: &mut [u8]) {
        self.0.read(offset, bytes);
    }

    fn write(&self, offset: u64, bytes: &[u8]) {
        self.0.write(offset, bytes);
    }
}

impl MemoryBacking for MeteredBacking {
    fn backing_identity(&self) -> BackingIdentity {
        self.backing().backing_identity()
    }
}

fn retained_cell(backing: VectorMemory, maximum: usize) -> FixtureCell<VectorMemory> {
    FixtureCell::open_bounded(
        backing,
        "test.jobs",
        "test.jobs.",
        KEY,
        MemoryManagerConfig::new(NonZeroU16::MIN),
        maximum,
    )
    .unwrap()
}

fn manager() -> Principal {
    Principal::self_authenticating(b"manager")
}
fn config() -> Init {
    Init {
        manager: manager(),
        delivery_sender: None,
    }
}
fn increment(at: u64) -> Create {
    Create {
        timing: Timing::Once(at),
        task: Task::Increment(1),
    }
}

// Inspect raw records for corruption fixtures without skipping production restore.
fn stored_records(bytes: &[u8]) -> Result<Snapshot, ic_wire::Error> {
    native::decode(
        bytes.strip_prefix(FORMAT).ok_or(ic_wire::Error::Identity)?,
        LIMITS,
    )
}

// Deliberately bypass Job::restore only while producing a corrupt current
// snapshot. Assertions still exercise normal production readers and refusals.
fn mutate_records(queue: &mut Queue<VectorMemory>, mutate: impl FnOnce(&mut Snapshot)) {
    let mut snapshot = stored_records(queue.cell.bytes()).unwrap();
    mutate(&mut snapshot);
    queue.cell.try_set(encode(&snapshot).unwrap()).unwrap();
}

#[test]
fn aliased_backing_refuses_reopen_until_the_final_handle_drops() {
    let backing = VectorMemory::default();
    let mut queue = Queue::open(MeteredBacking::new(backing.clone()), Some(config()))
        .unwrap()
        .0;
    let id = queue.create(increment(100)).unwrap().value;
    let before = queue.cell.bytes().to_vec();
    // Independent metering wrappers must identify the same physical backing.
    let alias = MeteredBacking::new(backing);
    let refused = || {
        assert!(matches!(
            Queue::open(alias.clone(), None),
            Err(Error::Memory(_))
        ));
        assert_eq!(alias.counts(), Default::default());
    };
    refused();
    let handle = queue.cell.into_memory();
    refused();
    drop(handle);
    let reopened = Queue::open(alias, None).unwrap().0;
    assert_eq!(reopened.cell.bytes(), before);
    assert_eq!(reopened.inspect(id).unwrap().id, id);
}

// These fixtures start with a fresh Once job. Build only valid states through
// the public transitions; malformed-record fixtures remain independent.
fn set_job_state(job: &mut Job, state: JobState, at_ns: u64) {
    match state {
        JobState::Pending { .. } => {}
        JobState::Paused { .. } => job.pause(at_ns).unwrap(),
        JobState::Cancelled => job.cancel(at_ns).unwrap(),
        _ => {
            let attempt = job.start(at_ns).unwrap();
            match state {
                JobState::Running => {}
                JobState::Uncertain { .. } => {
                    job.finish(attempt, at_ns, Outcome::Uncertain).unwrap()
                }
                JobState::Completed => job.finish(attempt, at_ns, Outcome::Success).unwrap(),
                JobState::Failed => job
                    .finish(attempt, at_ns, Outcome::PermanentFailure)
                    .unwrap(),
                _ => unreachable!(),
            }
        }
    }
    assert_eq!(job.record().state, state);
}

#[test]
fn paused_rescheduled_work_retains_identity_payload_and_deadline_across_reopen() {
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
    assert_eq!(queue.dispatch(100).unwrap().value.work, 1);
    queue.pause(id, 101).unwrap();
    assert_eq!(
        queue.summary().unwrap(),
        Committed {
            value: QueueTotals {
                counter: 2,
                receipts: 0
            },
            next_due_ns: None
        }
    );
    drop(queue);
    let mut queue = Queue::open(memory.clone(), None).unwrap().0;
    assert_eq!(queue.inspect(id).unwrap().status, Status::Paused);
    assert_eq!(
        queue.dispatch(400).unwrap(),
        Committed {
            value: DispatchBatch {
                deliveries: vec![],
                work: 0
            },
            next_due_ns: None
        }
    );
    queue
        .reschedule(
            id,
            Timing::AfterCompletion {
                first_ns: 500,
                every_ns: 10,
            },
            400,
        )
        .unwrap();
    let before = queue.inspect(id).unwrap();
    assert_eq!((before.sequence, before.occurrence), (1, 1));
    assert_eq!(before.task, Task::Increment(2));
    assert_eq!(before.status, Status::Paused);
    drop(queue);
    let mut queue = Queue::open(memory, None).unwrap().0;
    assert_eq!(queue.inspect(id).unwrap(), before);
    queue.resume(id, 450).unwrap();
    assert_eq!(queue.summary().unwrap().next_due_ns, Some(500));
    assert_eq!(queue.dispatch(499).unwrap().value.work, 0);
    assert_eq!(queue.dispatch(500).unwrap().value.work, 1);
    assert_eq!(
        queue.summary().unwrap(),
        Committed {
            value: QueueTotals {
                counter: 4,
                receipts: 0
            },
            next_due_ns: Some(510)
        }
    );
    assert_eq!(
        (
            queue.inspect(id).unwrap().sequence,
            queue.inspect(id).unwrap().occurrence
        ),
        (2, 2)
    );
    queue.pause(id, 500).unwrap();
    queue.cancel(id, 501).unwrap();
    assert_eq!(queue.inspect(id).unwrap().status, Status::Cancelled);
    assert_eq!(queue.summary().unwrap().next_due_ns, None);
}

#[test]
fn resume_preserves_overdue_policy_and_bounded_batch() {
    for catch_up in [false, true] {
        let memory = VectorMemory::default();
        let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
        let id = queue
            .create(Create {
                timing: Timing::FixedRate {
                    first_ns: 100,
                    every_ns: 10,
                    catch_up,
                },
                task: Task::Increment(1),
            })
            .unwrap()
            .value;
        queue.pause(id, 50).unwrap();
        drop(queue);
        let mut queue = Queue::open(memory, None).unwrap().0;
        queue.resume(id, 200).unwrap();
        assert_eq!(queue.summary().unwrap().next_due_ns, Some(100));
        assert_eq!(
            queue.dispatch(200).unwrap().value.work,
            if catch_up { 2 } else { 1 }
        );
        assert_eq!(
            queue.summary().unwrap().next_due_ns,
            Some(if catch_up { 120 } else { 210 })
        );
    }
}

#[test]
fn refused_management_preserves_stable_bytes_and_blocked_delivery() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    let id = queue
        .create(Create {
            timing: Timing::Once(0),
            task: Task::Deliver(DeliveryPayload {
                destination: Principal::self_authenticating(b"receiver"),
                message: "retained".into(),
            }),
        })
        .unwrap()
        .value;
    let Committed {
        value: DispatchBatch { deliveries, .. },
        ..
    } = queue.dispatch(0).unwrap();
    for uncertain in [false, true] {
        if uncertain {
            queue.finish(&deliveries[0], 1, Outcome::Uncertain).unwrap();
        }
        let before = queue.cell.bytes().to_vec();
        assert!(queue.pause(id, 2).is_err());
        assert!(queue.resume(id, 2).is_err());
        assert!(queue.reschedule(id, Timing::Once(100), 2).is_err());
        assert_eq!(queue.cell.bytes(), before);
        assert_eq!(queue.dispatch(2).unwrap().value.work, 0);
        assert_eq!(queue.cell.bytes(), before);
    }
    let pending = queue.create(increment(100)).unwrap().value;
    let before = queue.cell.bytes().to_vec();
    assert_eq!(queue.pause(999, 2), Err(Error::UnknownJob));
    assert!(queue.resume(pending, 2).is_err());
    assert!(
        queue
            .reschedule(
                pending,
                Timing::FixedRate {
                    first_ns: 200,
                    every_ns: 0,
                    catch_up: true
                },
                2
            )
            .is_err()
    );
    assert_eq!(queue.cell.bytes(), before);
}

#[test]
fn retained_queue_restores_payloads_deadlines_cancellation_and_id_authority() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
    queue.authorize(manager()).unwrap();
    assert_eq!(
        queue.authorize(Principal::anonymous()),
        Err(Error::Unauthorized)
    );
    let first = queue.create(increment(100)).unwrap().value;
    let second = queue.create(increment(200)).unwrap().value;
    queue.cancel(first, 10).unwrap();
    let expected = queue.page(None, 8).unwrap();
    drop(queue);
    let mut restored = Queue::open(memory.clone(), None).unwrap().0;
    assert_eq!(restored.page(None, 8).unwrap(), expected);
    assert_eq!(
        restored.summary().unwrap(),
        Committed {
            value: QueueTotals {
                counter: 0,
                receipts: 0
            },
            next_due_ns: Some(200)
        }
    );
    assert_eq!(restored.page(Some(first), 1).unwrap()[0].id, second);
    assert_eq!(restored.page(None, 0), Err(Error::InvalidInput));
    assert_eq!(restored.create(increment(300)).unwrap().value, 3);
    drop(restored);
    assert!(Queue::open(memory, Some(config())).is_err());
}

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
fn skip_and_refused_mutations_preserve_committed_bytes() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    let id = queue
        .create(Create {
            timing: Timing::FixedRate {
                first_ns: 100,
                every_ns: 10,
                catch_up: false,
            },
            task: Task::Increment(1),
        })
        .unwrap()
        .value;
    assert_eq!(queue.dispatch(125).unwrap().value.work, 1);
    assert_eq!(
        queue.inspect(id).unwrap().status,
        Status::Pending { due_ns: 130 }
    );
    let before = queue.cell.bytes().to_vec();
    assert!(
        queue
            .create(Create {
                timing: Timing::FixedRate {
                    first_ns: 0,
                    every_ns: 0,
                    catch_up: true
                },
                task: Task::Increment(1)
            })
            .is_err()
    );
    assert_eq!(queue.cell.bytes(), before);
    assert_eq!(queue.cancel(999, 125), Err(Error::UnknownJob));
    assert_eq!(queue.cell.bytes(), before);
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
        let before = sender.cell.bytes().to_vec();
        assert_eq!(
            sender.finish(&stale, 102, Outcome::Success),
            Err(Error::ReceiptMismatch)
        );
        assert_eq!(sender.cell.bytes(), before);
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
            let before = queue.cell.bytes().to_vec();
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
                assert_eq!(queue.cell.bytes(), before, "{name}: stable bytes changed");
            }
        }
    }
}

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
    let before = queue.cell.bytes().to_vec();
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
        assert_eq!(queue.cell.bytes(), before);
    }
}

#[test]
fn corrupt_future_record_blocks_due_dispatch_and_recovery_without_reset() {
    let backing = VectorMemory::default();
    let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
    queue.create(increment(100)).unwrap();
    let corrupt = queue.create(increment(200)).unwrap().value;
    queue.corrupt_record(corrupt).unwrap();
    let bytes = queue.cell.bytes().to_vec();
    assert!(queue.dispatch(100).is_err());
    assert_eq!(queue.cell.bytes(), bytes);
    drop(queue);
    assert!(Queue::open(backing, None).is_err());
}

fn assert_invalid_record_blocks_queue(
    mut queue: Queue<VectorMemory>,
    backing: VectorMemory,
    now: u64,
    healthy: u64,
) {
    let bytes = queue.cell.bytes().to_vec();
    let expected = job_error(ic_jobs::JobError::InvalidRecord);
    assert_eq!(queue.dispatch(now), Err(expected.clone()));
    assert_eq!(queue.summary(), Err(expected.clone()));
    assert_eq!(queue.inspect(healthy), Err(expected.clone()));
    assert_eq!(queue.cell.bytes(), bytes);
    drop(queue);
    assert!(matches!(Queue::open(backing.clone(), None), Err(error) if error == expected));
    assert_eq!(retained_cell(backing, MAX_BYTES).bytes(), bytes);
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
    let bytes = queue.cell.bytes().to_vec();
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
    assert_eq!(queue.cell.bytes(), bytes);
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
    let mut snapshot: Snapshot = stored_records(queue.cell.bytes()).unwrap();

    for invalid_id in [u128::from(u64::MAX), u128::from(u64::MAX) + 1, u128::MAX] {
        snapshot.jobs[0].job.id = JobId(invalid_id);
        let bytes = encode(&snapshot).unwrap();
        queue.cell.try_set(bytes.clone()).unwrap();
        assert_eq!(queue.inspect(id), Err(Error::InvalidStorage));
        assert_eq!(queue.page(None, 8), Err(Error::InvalidStorage));
        assert_eq!(queue.outstanding(id), Err(Error::InvalidStorage));
        assert_eq!(queue.cell.bytes(), bytes);
    }
}

#[test]
fn typed_commit_preserves_canonical_snapshot_bytes_and_metadata() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(
        memory.clone(),
        Some(Init {
            manager: manager(),
            delivery_sender: Some(manager()),
        }),
    )
    .unwrap()
    .0;
    queue.create(increment(100)).unwrap();
    queue
        .create(Create {
            timing: Timing::Once(100),
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "paired payload".into(),
            }),
        })
        .unwrap();
    let delivery = queue.dispatch(100).unwrap().value.deliveries.pop().unwrap();
    queue.receive(manager(), delivery.clone()).unwrap();
    queue.finish(&delivery, 101, Outcome::Uncertain).unwrap();
    let later = queue.create(increment(200)).unwrap().value;
    queue.pause(later, 102).unwrap();
    // The raw canonical writer and typed writer must retain the same metadata,
    // payloads, receipt and blocked/terminal/paused records byte for byte.
    let stored: Snapshot = stored_records(queue.cell.bytes()).unwrap();
    let expected = encode(&stored).unwrap();
    let loaded = queue.load().unwrap();
    assert_eq!(queue.commit(loaded, ()).unwrap().next_due_ns, None);
    assert_eq!(queue.cell.bytes(), expected);
    drop(queue);
    let (queue, due) = Queue::open(memory, None).unwrap();
    assert_eq!(due, None);
    assert_eq!(
        queue.summary().unwrap(),
        Committed {
            value: QueueTotals {
                counter: 1,
                receipts: 1
            },
            next_due_ns: None
        }
    );
    assert_eq!(queue.outstanding(delivery.job).unwrap(), delivery);
    assert_eq!(queue.inspect(later).unwrap().status, Status::Paused);
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
        let before = queue.cell.bytes().to_vec();
        let mut loaded = queue.load().unwrap();
        mutate(&mut loaded, &receipt);
        assert_eq!(
            queue.commit(loaded, ()),
            Err(Error::InvalidStorage),
            "{name}"
        );
        assert_eq!(queue.cell.bytes(), before, "{name}");
    }
    // Encoded capacity still takes precedence over invalid collection metadata.
    let before = queue.cell.bytes().to_vec();
    let mut loaded = queue.load().unwrap();
    loaded.next_id = 0;
    loaded.jobs[0].task = Task::Deliver(DeliveryPayload {
        destination: manager(),
        message: "x".repeat(MAX_BYTES),
    });
    assert_eq!(queue.commit(loaded, ()), Err(Error::Capacity));
    assert_eq!(queue.cell.bytes(), before);
}

#[test]
fn retired_snapshot_is_refused_without_replacing_the_cell() {
    // Exact old producer bytes remain only as a refusal/preservation fixture.
    for frozen in [
        include_bytes!("fixtures/snapshot-075.json").as_slice(),
        include_bytes!("fixtures/snapshot-running-015.json").as_slice(),
        include_bytes!("fixtures/snapshot-receipt-015.json").as_slice(),
    ] {
        let bytes = frozen.to_vec();
        let backing = VectorMemory::default();
        let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
        queue.cell.try_set(bytes.clone()).unwrap();
        drop(queue);
        for initial in [None, Some(config())] {
            assert!(matches!(
                Queue::open(backing.clone(), initial),
                Err(Error::InvalidStorage)
            ));
            assert_eq!(retained_cell(backing.clone(), MAX_BYTES).bytes(), bytes);
        }
    }
}

#[test]
fn raw_invalid_intervals_preserve_storage_and_unknown_job_precedence() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    let id = queue.create(increment(100)).unwrap().value;
    for timing in [
        Timing::FixedRate {
            first_ns: 0,
            every_ns: 0,
            catch_up: false,
        },
        Timing::AfterCompletion {
            first_ns: 0,
            every_ns: 0,
        },
    ] {
        let before = queue.cell.bytes().to_vec();
        assert_eq!(
            queue.create(Create {
                timing: timing.clone(),
                task: Task::Increment(1)
            }),
            Err(Error::InvalidInput)
        );
        assert_eq!(
            queue.reschedule(id, timing.clone(), 0),
            Err(Error::InvalidInput)
        );
        assert_eq!(queue.reschedule(999, timing, 0), Err(Error::UnknownJob));
        assert_eq!(queue.cell.bytes(), before);
    }
    queue.corrupt_record(id).unwrap();
    let before = queue.cell.bytes().to_vec();
    assert_eq!(queue.dispatch(100), Err(Error::InvalidStorage));
    assert_eq!(
        queue.reschedule(
            999,
            Timing::AfterCompletion {
                first_ns: 0,
                every_ns: 0
            },
            0
        ),
        Err(Error::InvalidStorage)
    );
    assert_eq!(queue.cell.bytes(), before);
}

// Sparse nonzero runs retain the complete backing, including manager/Cell headers.
#[derive(Deserialize)]
struct FrozenMemory {
    producer: String,
    pages: u64,
    runs: Vec<(u64, String)>,
    snapshot: String,
}

fn assert_frozen_memory_refused(frozen: &FrozenMemory) {
    // Memory owns expansion; Jobs owns provenance and the three-page ceiling.
    let backing = memory_from_sparse_hex(frozen.pages, &frozen.runs, 3).unwrap();
    let memory = MeteredBacking::new(backing);
    let before = memory.backing().borrow().clone();
    for initial in [None, Some(config())] {
        assert!(matches!(
            Queue::open(memory.clone(), initial),
            Err(Error::Memory(_))
        ));
        assert!(!memory.counts().overflowed);
        assert_eq!(memory.counts().writes, 0);
        assert_eq!(memory.size(), frozen.pages);
        assert_eq!(*memory.backing().borrow(), before);
    }
}

#[test]
fn prior_memory_ledgers_are_refused_without_writes_or_reset() {
    for (encoded, producer) in [
        (
            include_str!("fixtures/stable-image-075.json"),
            "060fb9d226389bab702132ba23f42bd0b887a690",
        ),
        (
            include_str!("fixtures/stable-image-memory040.json"),
            "ic-memory 0.40.0",
        ),
    ] {
        let frozen: FrozenMemory = serde_json::from_str(encoded).unwrap();
        assert_eq!(frozen.producer, producer);
        // Both layers are retired. The outer Memory ledger must refuse before
        // payload admission, publishing an allocation or overwriting old bytes.
        assert!(matches!(
            decode(frozen.snapshot.as_bytes()),
            Err(Error::InvalidStorage)
        ));
        assert_frozen_memory_refused(&frozen);
    }
}

#[test]
fn memory_wire_cut_refuses_cbor_without_releasing_blocked_work() {
    let frozen: FrozenMemory =
        serde_json::from_str(include_str!("fixtures/stable-image-memory0552.json")).unwrap();
    assert_eq!(
        frozen.producer,
        "cdb48cc9d1c3281e2b898bcfe4a19e2a673b4560 / ic-memory 0.55.2"
    );
    // Frozen producer metadata still establishes the unresolved obligation.
    // Neither retired JSON nor the outer CBOR ledger is a current reader path.
    let snapshot = serde_json::from_str::<Snapshot>(&frozen.snapshot)
        .unwrap()
        .admit()
        .unwrap();
    assert_eq!(snapshot.jobs.len(), 1);
    assert_eq!(snapshot.jobs[0].job.record().state, JobState::Running);
    assert_eq!(snapshot.next_due_ns(), None);
    assert!(matches!(
        decode(frozen.snapshot.as_bytes()),
        Err(Error::InvalidStorage)
    ));
    assert_frozen_memory_refused(&frozen);
}

#[test]
fn json_queue_cut_preserves_current_ledger_and_outstanding_effect_evidence() {
    let frozen: FrozenMemory =
        serde_json::from_str(include_str!("fixtures/stable-image-json01749.json")).unwrap();
    assert_eq!(
        frozen.producer,
        "26108182368b3ef9687f195feaf2ce6c38021dfc / ic-memory 0.56.0 / incoming 2026-10-30 lock"
    );
    // Only this frozen-fixture assertion reads the old format. The old producer
    // cold-opened both outstanding attempts blocked before capturing these bytes.
    let snapshot = serde_json::from_str::<Snapshot>(&frozen.snapshot)
        .unwrap()
        .admit()
        .unwrap();
    assert_eq!(
        snapshot.jobs[0].job.record().state,
        JobState::Uncertain { started_ns: 10 }
    );
    assert_eq!(snapshot.jobs[1].job.record().state, JobState::Running);
    assert_eq!(snapshot.receipts.len(), 2);
    for entry in &snapshot.jobs {
        assert_eq!(
            entry.delivery().unwrap(),
            snapshot.receipts[entry.job.record().id.0 as usize - 1]
        );
    }
    assert_eq!(snapshot.next_due_ns(), None);
    assert!(matches!(
        decode(frozen.snapshot.as_bytes()),
        Err(Error::InvalidStorage)
    ));

    let backing = memory_from_sparse_hex(frozen.pages, &frozen.runs, 3).unwrap();
    let cell_memory = retained_cell(backing.clone(), MAX_BYTES).into_memory();
    let mut before = vec![0; ic_memory::STABLE_CELL_HEADER_SIZE + frozen.snapshot.len()];
    cell_memory.read(0, &mut before);
    drop(cell_memory);
    let memory = MeteredBacking::new(backing.clone());
    for initial in [None, Some(config())] {
        assert!(matches!(
            Queue::open(memory.clone(), initial),
            Err(Error::InvalidStorage)
        ));
        assert!(!memory.counts().overflowed);
        // Bootstrap may write allocation bookkeeping. The refused Cell header,
        // payload, outstanding attempts and receipts must remain byte-identical.
        let cell = retained_cell(backing.clone(), MAX_BYTES);
        assert_eq!(cell.bytes(), frozen.snapshot.as_bytes());
        let cell_memory = cell.into_memory();
        let mut after = vec![0; before.len()];
        cell_memory.read(0, &mut after);
        assert_eq!(after, before);
    }
}

#[test]
fn bounded_wire_queue_preserves_full_history_payloads_receipts_and_schedule_shapes() {
    let backing = VectorMemory::default();
    let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
    let mut snapshot = queue.load().unwrap();
    snapshot.next_id = u64::MAX;
    snapshot.counter = u64::MAX;
    let states = [
        JobState::Pending { due_ns: 100 },
        JobState::Paused { due_ns: 100 },
        JobState::Running,
        JobState::Uncertain { started_ns: 100 },
        JobState::Completed,
        JobState::Failed,
        JobState::Cancelled,
    ];
    for index in 0..MAX_JOBS {
        let id = u64::MAX - MAX_JOBS as u64 + index as u64;
        let schedule = if index < states.len() {
            Schedule::Once { at_ns: 100 }
        } else {
            match index % 3 {
                0 => Schedule::FixedRate {
                    first_at_ns: 100,
                    every_ns: NonZeroU64::new(10).unwrap(),
                    missed: MissedRunPolicy::Skip,
                },
                1 => Schedule::FixedRate {
                    first_at_ns: 100,
                    every_ns: NonZeroU64::new(10).unwrap(),
                    missed: MissedRunPolicy::CatchUp,
                },
                _ => Schedule::AfterCompletion {
                    first_at_ns: 100,
                    every_ns: NonZeroU64::new(10).unwrap(),
                },
            }
        };
        let mut job = Job::new(
            JobId(u128::from(id)),
            schedule,
            RetryPolicy::new(3, 5, 20).unwrap(),
        );
        if let Some(state) = states.get(index) {
            set_job_state(&mut job, *state, 100);
        }
        let payload = DeliveryPayload {
            destination: manager(),
            message: "x".repeat(MAX_MESSAGE),
        };
        snapshot.jobs.push(Entry {
            job,
            task: Task::Deliver(payload.clone()),
        });
        snapshot.receipts.push(Delivery {
            job: id,
            occurrence: u64::MAX,
            sequence: u64::MAX,
            payload,
        });
    }
    let expected = encode(&snapshot).unwrap();
    assert!(expected.len() <= MAX_BYTES);
    stored_records(&expected).unwrap();
    assert_eq!(queue.commit(snapshot, ()).unwrap().next_due_ns, Some(100));
    assert_eq!(queue.cell.bytes(), expected);
    assert_eq!(queue.create(increment(200)), Err(Error::Capacity));
    drop(queue);
    let (queue, due) = Queue::open(backing, None).unwrap();
    assert_eq!(due, Some(100));
    let loaded = queue.load().unwrap();
    assert_eq!(loaded.jobs.len(), MAX_JOBS);
    assert_eq!(loaded.receipts.len(), MAX_JOBS);
    assert_eq!(loaded.counter, u64::MAX);
    assert_eq!(encode(&loaded).unwrap(), expected);
}

#[test]
fn wire_queue_refuses_wrong_identity_malformed_trailing_and_exhausted_budgets() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue.create(increment(100)).unwrap();
    let original = queue.cell.bytes().to_vec();
    let mut wrong_identity = original.clone();
    wrong_identity[0] ^= 1;
    let mut trailing = original.clone();
    trailing.push(0);
    for bytes in [
        vec![],
        wrong_identity,
        original[..original.len() - 1].to_vec(),
        trailing,
        vec![0; MAX_BYTES + 1],
    ] {
        assert!(matches!(decode(&bytes), Err(Error::InvalidStorage)));
        if bytes.len() <= MAX_BYTES {
            let memory = VectorMemory::default();
            let mut damaged = Queue::open(memory.clone(), Some(config())).unwrap().0;
            damaged.cell.try_set(bytes.clone()).unwrap();
            assert_eq!(damaged.dispatch(100), Err(Error::InvalidStorage));
            assert_eq!(damaged.cell.bytes(), bytes);
            drop(damaged);
            assert!(matches!(
                Queue::open(memory.clone(), None),
                Err(Error::InvalidStorage)
            ));
            assert_eq!(retained_cell(memory, MAX_BYTES).bytes(), bytes);
        }
    }
    let payload = original.strip_prefix(FORMAT).unwrap();
    for limits in [
        Limits {
            bytes: payload.len() - 1,
            ..LIMITS
        },
        Limits {
            allocation: LIMITS.bytes - 1,
            ..LIMITS
        },
        Limits { nodes: 1, ..LIMITS },
        Limits { depth: 0, ..LIMITS },
    ] {
        assert!(matches!(
            native::decode::<Snapshot>(payload, limits),
            Err(ic_wire::Error::Limit)
        ));
        assert_eq!(
            native::encode(&queue.load().unwrap(), limits),
            Err(ic_wire::Error::Limit)
        );
    }
    assert_eq!(queue.cell.bytes(), original);
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
        let memory = MeteredBacking::new(VectorMemory::default());
        let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
        let id = state.map(|_| queue.create(increment(100)).unwrap().value);
        let mut loaded = queue.load().unwrap();
        if let Some((target, at_ns)) = state {
            set_job_state(&mut loaded.jobs[0].job, target, at_ns);
        }
        queue.commit(loaded, ()).unwrap();
        let expected_due = queue.summary().unwrap().next_due_ns;
        let before = queue.cell.bytes().to_vec();
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
        assert_eq!(queue.cell.bytes(), before, "state {state:?}");
        if let Some(id) = id {
            queue.corrupt_record(id).unwrap();
            let before = queue.cell.bytes().to_vec();
            memory.reset();
            assert!(queue.dispatch(99).is_err(), "state {state:?}");
            assert!(!memory.counts().overflowed);
            assert_eq!(memory.counts().writes, 0);
            assert_eq!(queue.cell.bytes(), before);
        }
    }
    // Real due work still writes its metadata and local effect together.
    let memory = MeteredBacking::new(VectorMemory::default());
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
    let before = queue.cell.bytes().to_vec();
    drop(queue);
    let (mut queue, due) = Queue::open(memory.clone(), None).unwrap();
    assert_eq!(due, Some(100));
    assert_eq!(queue.cell.bytes(), before);
    queue.pause(first, 0).unwrap();
    drop(queue);
    let (mut queue, due) = Queue::open(memory.clone(), None).unwrap();
    assert_eq!(due, Some(200));
    queue.corrupt_record(first).unwrap();
    drop(queue);
    // Corruption in inactive work still blocks the entire restoration result.
    assert!(Queue::open(memory, None).is_err());
}

#[test]
fn mutation_replies_return_the_fresh_committed_deadline() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(memory.clone(), Some(config())).unwrap().0;
    let later = queue.create(increment(300)).unwrap();
    assert_eq!(later.next_due_ns, Some(300));
    let early = queue.create(increment(100)).unwrap();
    assert_eq!(early.next_due_ns, Some(100));
    assert_eq!(queue.pause(early.value, 0).unwrap().next_due_ns, Some(300));
    assert_eq!(
        queue
            .reschedule(later.value, Timing::Once(200), 0)
            .unwrap()
            .next_due_ns,
        Some(200)
    );
    assert_eq!(queue.resume(early.value, 0).unwrap().next_due_ns, Some(100));
    assert_eq!(queue.cancel(early.value, 0).unwrap().next_due_ns, Some(200));
    assert_eq!(
        queue.dispatch(200).unwrap(),
        Committed {
            value: DispatchBatch {
                deliveries: vec![],
                work: 1
            },
            next_due_ns: None
        }
    );
    let external = queue
        .create(Create {
            timing: Timing::Once(250),
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "exact immutable payload".into(),
            }),
        })
        .unwrap();
    assert_eq!(external.next_due_ns, Some(250));
    let Committed {
        value: DispatchBatch { deliveries, .. },
        next_due_ns: due,
    } = queue.dispatch(250).unwrap();
    assert_eq!(due, None);
    assert_eq!(
        queue
            .finish(&deliveries[0], 251, Outcome::Uncertain)
            .unwrap()
            .next_due_ns,
        None
    );
    // Another message changes the queue before the old delivery is reconciled.
    queue.create(increment(260)).unwrap();
    let reply = queue.finish(&deliveries[0], 252, Outcome::Success).unwrap();
    assert_eq!(reply.next_due_ns, Some(260));
    assert_eq!(
        queue.inspect(external.value).unwrap().status,
        Status::Completed
    );
    drop(queue);
    let (queue, due) = Queue::open(memory, None).unwrap();
    assert_eq!(due, reply.next_due_ns);
    assert_eq!(queue.summary().unwrap().next_due_ns, reply.next_due_ns);
}

#[test]
fn receipt_extraction_and_duplicate_acknowledgement_leave_storage_unchanged() {
    let memory = MeteredBacking::new(VectorMemory::default());
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
    let before = queue.cell.bytes().to_vec();
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
    assert_eq!(queue.cell.bytes(), before);
    drop(queue);
    let mut queue = Queue::open(memory, None).unwrap().0;
    assert_eq!(queue.cell.bytes(), before);
    assert_eq!(queue.summary().unwrap().value.receipts, 3);
    // Both receiver entrypoints reject an unauthenticated caller before decoding
    // corrupt private storage, and neither refusal replaces the retained bytes.
    queue.cell.try_set(b"not a snapshot".to_vec()).unwrap();
    let corrupt = queue.cell.bytes().to_vec();
    assert_eq!(
        queue.receive(Principal::anonymous(), receipts[0].clone()),
        Err(Error::Unauthorized)
    );
    assert_eq!(
        queue.receipt(Principal::anonymous(), &receipts[0]),
        Err(Error::Unauthorized)
    );
    assert_eq!(queue.cell.bytes(), corrupt);
}

#[test]
fn oversized_retained_cell_is_refused_without_replacing_its_payload() {
    let backing = VectorMemory::default();
    let queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
    drop(queue);
    // Deliberately raise only the corruption fixture's write ceiling. The
    // consumer's normal limit must still refuse the retained oversized payload.
    let mut oversized = retained_cell(backing.clone(), MAX_BYTES + 1);
    oversized.try_set(vec![b'x'; MAX_BYTES + 1]).unwrap();
    let memory = oversized.into_memory();
    let mut before = vec![0; ic_memory::STABLE_CELL_HEADER_SIZE + MAX_BYTES + 1];
    memory.read(0, &mut before);
    drop(memory);
    for initial in [None, Some(config())] {
        assert!(matches!(
            Queue::open(backing.clone(), initial),
            Err(Error::InvalidStorage)
        ));
        let memory = retained_cell(backing.clone(), MAX_BYTES + 1).into_memory();
        let mut after = vec![0; before.len()];
        memory.read(0, &mut after);
        // Bootstrap may update allocation metadata. The refused cell's header
        // and payload must retain their exact bytes across both entry paths.
        assert_eq!(after, before);
    }
}

// Restrict only this fixture's writer after releasing its prior allocation lease.
// Failed Cell admission preserves committed payload bytes without simulating IC rollback.
fn with_current_write_ceiling(
    queue: Queue<VectorMemory>,
    backing: VectorMemory,
) -> Queue<VectorMemory> {
    let Queue {
        cell,
        init,
        metrics,
        logs,
    } = queue;
    let maximum = cell.bytes().len();
    drop(cell);
    Queue {
        cell: retained_cell(backing, maximum),
        init,
        metrics,
        logs,
    }
}

#[test]
fn job_metrics_count_committed_local_success_and_permanent_failure() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    for amount in [u64::MAX, 1] {
        queue
            .create(Create {
                timing: Timing::Once(100),
                task: Task::Increment(amount),
            })
            .unwrap();
    }
    queue.dispatch(100).unwrap();
    let observed = queue.job_metrics(7);
    assert_eq!(observed.dispatched.total, 2);
    assert_eq!(observed.succeeded.total, 1);
    assert_eq!(observed.permanent_failures.total, 1);
    assert_eq!(observed.retry_dispatches.total, 0);
    assert_eq!(observed.uncertain.total, 0);
    assert_eq!(observed.reconciliations.total, 0);
    let logs = log_page(&queue);
    assert_eq!(logs.entries.len(), 4);
    assert_eq!(logs.entries[0].level, crate::api::JobLogLevel::Error);
    assert_eq!(logs.entries[2].level, crate::api::JobLogLevel::Ok);
    assert_eq!(queue.summary().unwrap().value.counter, u64::MAX);
    assert_eq!(queue.dispatch(200).unwrap().value.work, 0);
    assert_eq!(queue.job_metrics(7), observed);
}

#[test]
fn job_metrics_separate_retries_uncertainty_and_receipt_reconciliation() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue
        .create(Create {
            timing: Timing::Once(100),
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "same payload".into(),
            }),
        })
        .unwrap();
    // This consumer's ordinary creation policy has no retries. Give this fixture
    // an admitted retry budget before the first attempt without adding a new policy.
    let mut initial = queue.load().unwrap();
    initial.jobs[0].job = Job::new(
        JobId(1),
        Schedule::Once { at_ns: 100 },
        RetryPolicy::new(2, 5, 5).unwrap(),
    );
    queue.commit(initial, ()).unwrap();
    let first = queue.dispatch(100).unwrap().value.deliveries.pop().unwrap();
    queue
        .finish(&first, 101, Outcome::RetryableFailure)
        .unwrap();
    assert!(queue.dispatch(105).unwrap().value.deliveries.is_empty());
    let retry = queue.dispatch(106).unwrap().value.deliveries.pop().unwrap();
    assert_eq!(retry.payload, first.payload);
    assert_eq!(retry.occurrence, first.occurrence);
    assert_ne!(retry.sequence, first.sequence);
    queue.finish(&retry, 107, Outcome::Uncertain).unwrap();
    let blocked = queue.job_metrics(7);
    assert_eq!(blocked.dispatched.total, 2);
    assert_eq!(blocked.retry_dispatches.total, 1);
    assert_eq!(blocked.retryable_failures.total, 1);
    assert_eq!(blocked.uncertain.total, 1);
    assert_eq!(blocked.succeeded.total, 0);
    assert_eq!(blocked.reconciliations.total, 0);
    let logs = log_page(&queue);
    assert_eq!(logs.entries[1].topic, "dispatch");
    assert!(logs.entries[1].message.ends_with("retry=true"));
    assert!(queue.dispatch(200).unwrap().value.deliveries.is_empty());
    assert!(queue.finish(&first, 201, Outcome::Success).is_err());
    let mut mismatched = retry.clone();
    mismatched.payload.message.push('!');
    assert_eq!(
        queue.finish(&mismatched, 201, Outcome::Success),
        Err(Error::ReceiptMismatch)
    );
    assert_eq!(queue.job_metrics(7), blocked);
    // A successful resolve that remains uncertain is an observation, not job success.
    queue.finish(&retry, 201, Outcome::Uncertain).unwrap();
    queue.finish(&retry, 202, Outcome::Success).unwrap();
    let completed = queue.job_metrics(7);
    assert_eq!(completed.reconciliations.total, 2);
    assert_eq!(completed.uncertain.total, 2);
    assert_eq!(completed.succeeded.total, 1);
    assert_eq!(completed.dispatched, blocked.dispatched);
    assert!(queue.finish(&retry, 203, Outcome::Success).is_err());
    assert_eq!(queue.job_metrics(7), completed);
}

#[test]
fn job_metrics_exclude_rejected_commits_and_leave_running_intent_blocked() {
    let backing = VectorMemory::default();
    let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
    queue.create(increment(100)).unwrap();
    let mut queue = with_current_write_ceiling(queue, backing);
    let before = queue.cell.bytes().to_vec();
    let counters = queue.job_metrics(7);
    let logs = log_page(&queue);
    assert!(queue.dispatch(100).is_err());
    assert_eq!(queue.cell.bytes(), before);
    assert_eq!(queue.job_metrics(7), counters);
    assert_eq!(log_page(&queue), logs);

    let backing = VectorMemory::default();
    let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
    queue
        .create(Create {
            timing: Timing::Once(100),
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "keep intent".into(),
            }),
        })
        .unwrap();
    let delivery = queue.dispatch(100).unwrap().value.deliveries.pop().unwrap();
    let mut queue = with_current_write_ceiling(queue, backing.clone());
    let before = queue.cell.bytes().to_vec();
    let counters = queue.job_metrics(7);
    let logs = log_page(&queue);
    assert!(queue.finish(&delivery, 101, Outcome::Success).is_err());
    assert_eq!(queue.cell.bytes(), before);
    assert_eq!(queue.job_metrics(7), counters);
    assert_eq!(log_page(&queue), logs);
    assert_eq!(queue.inspect(1).unwrap().status, Status::Running);
    drop(queue);
    let mut restored = Queue::open(backing, None).unwrap().0;
    assert!(log_page(&restored).entries.is_empty());
    assert_eq!(restored.job_metrics(8).dispatched.total, 0);
    assert_eq!(restored.job_metrics(8).reset_sequence, Some(0));
    assert!(restored.dispatch(200).unwrap().value.deliveries.is_empty());
    assert_eq!(restored.outstanding(1).unwrap(), delivery);
}

#[test]
fn job_metrics_authorize_reset_and_never_change_durable_authority() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue
        .create(Create {
            timing: Timing::Once(100),
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "blocked".into(),
            }),
        })
        .unwrap();
    let delivery = queue.dispatch(100).unwrap().value.deliveries.pop().unwrap();
    let before = queue.cell.bytes().to_vec();
    let observed = queue.job_metrics(7);
    assert_eq!(
        queue.authorize(Principal::anonymous()),
        Err(Error::Unauthorized)
    );
    assert_eq!(queue.job_metrics(7), observed);
    let reset = queue.reset_job_metrics(7);
    assert_eq!(reset.canister_version, observed.canister_version);
    assert_eq!(reset.reset_sequence, Some(1));
    assert_eq!(reset.dispatched.total, 0);
    assert_eq!(queue.cell.bytes(), before);
    assert!(queue.dispatch(200).unwrap().value.deliveries.is_empty());
    assert_eq!(queue.outstanding(1).unwrap(), delivery);
    queue.corrupt_record(1).unwrap();
    assert!(queue.dispatch(201).is_err());
    assert_eq!(queue.job_metrics(7), reset);
}

fn log_request() -> crate::api::JobLogRequest {
    crate::api::JobLogRequest {
        before: None,
        min_level: None,
        limit: 8,
    }
}

fn log_page(queue: &Queue<VectorMemory>) -> crate::api::JobLogPage {
    queue.job_logs(log_request(), 7).unwrap()
}

#[test]
fn job_logs_capture_committed_outcomes_without_payloads_or_read_side_effects() {
    use crate::api::JobLogLevel;
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue
        .create(Create {
            timing: Timing::Once(1_000_000_000),
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "private delivery payload".into(),
            }),
        })
        .unwrap();
    let delivery = queue
        .dispatch(1_000_000_000)
        .unwrap()
        .value
        .deliveries
        .pop()
        .unwrap();
    queue
        .finish(&delivery, 2_000_000_000, Outcome::Uncertain)
        .unwrap();
    let before = log_page(&queue);
    assert_eq!(before.entries.len(), 2);
    assert_eq!(before.entries[0].topic, "outcome");
    assert_eq!(before.entries[0].level, JobLogLevel::Warn);
    assert_eq!(before.entries[0].created_at_secs, 2);
    assert_eq!(before.entries[1].topic, "dispatch");
    assert_eq!(before.entries[1].created_at_secs, 1);
    assert_eq!(before.omitted.total, 0);
    let bytes = queue.cell.bytes().to_vec();
    assert_eq!(
        queue.authorize(Principal::anonymous()),
        Err(Error::Unauthorized)
    );
    for limit in [0, 9] {
        assert_eq!(
            queue.job_logs(
                crate::api::JobLogRequest {
                    limit,
                    ..log_request()
                },
                7
            ),
            Err(Error::InvalidInput)
        );
    }
    assert_eq!(queue.dispatch(3_000_000_000).unwrap().value.work, 0);
    queue.reset_job_metrics(7);
    assert_eq!(log_page(&queue), before);
    assert_eq!(queue.cell.bytes(), bytes);
    queue
        .finish(&delivery, 4_000_000_000, Outcome::Success)
        .unwrap();
    let completed = log_page(&queue);
    assert_eq!(completed.entries[0].topic, "reconciliation");
    assert_eq!(completed.entries[0].level, JobLogLevel::Ok);
    assert_eq!(completed.entries[0].sequence, 2);
    assert!(
        queue
            .finish(&delivery, 5_000_000_000, Outcome::Success)
            .is_err()
    );
    assert_eq!(log_page(&queue), completed);
    let diagnostic = serde_json::to_string(&completed).unwrap();
    assert!(!diagnostic.contains("private delivery payload"));
    assert!(!diagnostic.contains(&manager().to_string()));
}

#[test]
fn job_logs_bound_retention_and_filtered_scans_with_continuable_empty_pages() {
    use crate::api::JobLogLevel;
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue
        .create(Create {
            timing: Timing::FixedRate {
                first_ns: 1,
                every_ns: 1,
                catch_up: true,
            },
            task: Task::Increment(1),
        })
        .unwrap();
    for now in 1..=70 {
        queue.dispatch(now).unwrap();
    }
    let mut request = log_request();
    let mut sequences = vec![];
    loop {
        let page = queue.job_logs(request, 7).unwrap();
        assert!(page.entries.len() <= 8);
        assert!(page.scanned <= 32);
        sequences.extend(page.entries.iter().map(|entry| entry.sequence));
        let Some(before) = page.next_before else {
            break;
        };
        request.before = Some(before);
    }
    assert_eq!(sequences, (12..140).rev().collect::<Vec<_>>());
    let filtered = queue
        .job_logs(
            crate::api::JobLogRequest {
                min_level: Some(JobLogLevel::Error),
                ..log_request()
            },
            7,
        )
        .unwrap();
    assert!(filtered.entries.is_empty());
    assert_eq!(filtered.scanned, 32);
    assert!(filtered.next_before.is_some());
    assert_eq!(filtered.omitted.total, 0);
}

#[test]
fn job_log_refusals_are_counted_without_refusing_committed_work() {
    let mut queue = Queue::open(VectorMemory::default(), Some(config()))
        .unwrap()
        .0;
    queue.create(increment(0)).unwrap();
    queue.dispatch(2_000_000_000).unwrap();
    let admitted = log_page(&queue);
    queue.create(increment(0)).unwrap();
    // Each job admits its own timestamp; the log protects its global watermark.
    // A refused diagnostic cannot turn already committed work into a retry.
    queue.dispatch(1_000_000_000).unwrap();
    let refused = log_page(&queue);
    assert_eq!(refused.entries, admitted.entries);
    assert_eq!(refused.omitted.total, 2);
    assert!(!refused.omitted.overflowed);
    assert_eq!(queue.summary().unwrap().value.counter, 2);
    assert_eq!(queue.job_metrics(7).succeeded.total, 2);
}
