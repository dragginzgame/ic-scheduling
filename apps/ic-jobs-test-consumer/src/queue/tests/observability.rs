use super::*;

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
fn job_metrics_exclude_rejected_mutations_and_leave_running_intent_blocked() {
    let backing = VectorMemory::default();
    let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
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
    let logs = log_page(&queue);
    assert!(queue.dispatch(100).is_err());
    assert_eq!(queue.cell.get().as_slice(), before);
    assert_eq!(queue.job_metrics(7), counters);
    assert_eq!(log_page(&queue), logs);

    let backing = VectorMemory::default();
    let mut queue = Queue::open(backing.clone(), Some(config())).unwrap().0;
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
    let logs = log_page(&queue);
    assert!(queue.finish(&delivery, 101, Outcome::Success).is_err());
    assert_eq!(queue.cell.get().as_slice(), before);
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
    let bytes = queue.cell.get().as_slice().to_vec();
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
    assert_eq!(queue.cell.get().as_slice(), bytes);
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
