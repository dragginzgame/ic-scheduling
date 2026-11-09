use super::*;

#[test]
fn job_metrics_count_committed_local_success_and_permanent_failure() {
    let mut queue = fresh_queue(VectorMemory::default());
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
    assert_eq!(queue.summary().unwrap().value.counter, u64::MAX);
    assert_eq!(queue.dispatch(200).unwrap().value.work, 0);
    assert_eq!(queue.job_metrics(7), observed);
}

#[test]
fn job_metrics_separate_retries_uncertainty_and_receipt_reconciliation() {
    let mut queue = fresh_queue(VectorMemory::default());
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
fn job_metrics_exclude_rejected_mutations_and_leave_running_intent_blocked() {
    let backing = VectorMemory::default();
    let mut queue = fresh_queue(backing.clone());
    queue
        .create(Create {
            timing: Timing::AfterCompletion {
                first_ns: 100,
                every_ns: u64::MAX,
            },
            task: Task::Increment(1),
        })
        .unwrap();
    let before = queue.cell.get().as_slice().to_vec();
    let counters = queue.job_metrics(7);
    assert!(queue.dispatch(100).is_err());
    assert_eq!(queue.cell.get().as_slice(), before);
    assert_eq!(queue.job_metrics(7), counters);

    let backing = VectorMemory::default();
    let mut queue = fresh_queue(backing.clone());
    queue
        .create(Create {
            timing: Timing::AfterCompletion {
                first_ns: 100,
                every_ns: u64::MAX,
            },
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "keep intent".into(),
            }),
        })
        .unwrap();
    let delivery = queue.dispatch(100).unwrap().value.deliveries.pop().unwrap();
    let before = queue.cell.get().as_slice().to_vec();
    let counters = queue.job_metrics(7);
    assert!(queue.finish(&delivery, 101, Outcome::Success).is_err());
    assert_eq!(queue.cell.get().as_slice(), before);
    assert_eq!(queue.job_metrics(7), counters);
    assert_eq!(queue.inspect(1).unwrap().status, Status::Running);
    drop(queue);
    let mut restored = Queue::open(backing, None).unwrap().0;
    assert_eq!(restored.job_metrics(8).dispatched.total, 0);
    assert_eq!(restored.job_metrics(8).reset_sequence, Some(0));
    assert!(restored.dispatch(200).unwrap().value.deliveries.is_empty());
    assert_eq!(restored.outstanding(1).unwrap(), delivery);
}

#[test]
fn job_metrics_authorize_reset_and_never_change_durable_authority() {
    let mut queue = fresh_queue(VectorMemory::default());
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
    let before = queue.cell.get().as_slice().to_vec();
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
    assert_eq!(queue.cell.get().as_slice(), before);
    assert!(queue.dispatch(200).unwrap().value.deliveries.is_empty());
    assert_eq!(queue.outstanding(1).unwrap(), delivery);
    queue.corrupt_record(1).unwrap();
    assert!(queue.dispatch(201).is_err());
    assert_eq!(queue.job_metrics(7), reset);
}
