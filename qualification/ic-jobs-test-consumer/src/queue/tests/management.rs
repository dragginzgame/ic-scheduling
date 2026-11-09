use super::*;

#[test]
fn paused_rescheduled_work_retains_identity_payload_and_deadline_across_reopen() {
    let memory = VectorMemory::default();
    let mut queue = fresh_queue(memory.clone());
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
    assert_eq!(queue.summary().unwrap(), totals(2, 0, None));
    drop(queue);
    let mut queue = Queue::open(memory.clone(), None).unwrap().0;
    assert_eq!(queue.inspect(id).unwrap().status, Status::Paused);
    assert_eq!(queue.dispatch(400).unwrap(), local_batch(0, None));
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
    assert_eq!(queue.summary().unwrap(), totals(4, 0, Some(510)));
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
        let mut queue = fresh_queue(memory.clone());
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
    let mut queue = fresh_queue(VectorMemory::default());
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
        let before = queue.cell.get().as_slice().to_vec();
        assert!(queue.pause(id, 2).is_err());
        assert!(queue.resume(id, 2).is_err());
        assert!(queue.reschedule(id, Timing::Once(100), 2).is_err());
        assert_eq!(queue.cell.get().as_slice(), before);
        assert_eq!(queue.dispatch(2).unwrap().value.work, 0);
        assert_eq!(queue.cell.get().as_slice(), before);
    }
    let pending = queue.create(increment(100)).unwrap().value;
    let before = queue.cell.get().as_slice().to_vec();
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
    assert_eq!(queue.cell.get().as_slice(), before);
}

#[test]
fn retained_queue_restores_payloads_deadlines_cancellation_and_id_authority() {
    let memory = VectorMemory::default();
    let mut queue = fresh_queue(memory.clone());
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
    assert_eq!(restored.summary().unwrap(), totals(0, 0, Some(200)));
    assert_eq!(restored.page(Some(first), 1).unwrap()[0].id, second);
    assert_eq!(restored.page(None, 0), Err(Error::InvalidInput));
    assert_eq!(restored.create(increment(300)).unwrap().value, 3);
    drop(restored);
    assert!(Queue::open(memory, Some(config())).is_err());
}

#[test]
fn skip_and_refused_mutations_preserve_committed_bytes() {
    let mut queue = fresh_queue(VectorMemory::default());
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
    let before = queue.cell.get().as_slice().to_vec();
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
    assert_eq!(queue.cell.get().as_slice(), before);
    assert_eq!(queue.cancel(999, 125), Err(Error::UnknownJob));
    assert_eq!(queue.cell.get().as_slice(), before);
}

#[test]
fn raw_invalid_intervals_preserve_storage_and_unknown_job_precedence() {
    let mut queue = fresh_queue(VectorMemory::default());
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
        let before = queue.cell.get().as_slice().to_vec();
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
        assert_eq!(queue.cell.get().as_slice(), before);
    }
    queue.corrupt_record(id).unwrap();
    let before = queue.cell.get().as_slice().to_vec();
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
    assert_eq!(queue.cell.get().as_slice(), before);
}

#[test]
fn mutation_replies_return_the_fresh_committed_deadline() {
    let memory = VectorMemory::default();
    let mut queue = fresh_queue(memory.clone());
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
    assert_eq!(queue.dispatch(200).unwrap(), local_batch(1, None));
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
