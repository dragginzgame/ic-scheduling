use super::*;

#[test]
fn completed_batch_keeps_work_classification_and_deadline_independent() {
    for work_count in [0, 1, u64::MAX] {
        for next_due_ns in [None, Some(0), Some(u64::MAX)] {
            let result = WatchdogRunResult::completed_batch(work_count, next_due_ns);
            assert_eq!(result.completion().work_count(), work_count);
            assert_eq!(
                result.completion().outcome(),
                if work_count == 0 {
                    TimerCompletionOutcome::NoWork
                } else {
                    TimerCompletionOutcome::Success
                }
            );
            assert_eq!(
                result.decision(),
                next_due_ns.map_or(WatchdogDecision::Stop, WatchdogDecision::ScheduleAt)
            );
        }
    }
}

#[test]
fn latest_outcome_keeps_work_count_and_independent_history_coherent() {
    let mut outcomes = TimerOutcomeSnapshot::EMPTY;
    assert_eq!(outcomes.last_outcome(), None);
    assert_eq!(outcomes.last_work_count(), None);

    for (completion, completed_at_ns) in [
        (TimerCompletion::success(0), 10),
        (TimerCompletion::no_work(), 11),
        (TimerCompletion::retryable_failure(7), 12),
    ] {
        outcomes.record_completion(completion, completed_at_ns);
        assert_eq!(
            outcomes.last_outcome(),
            Some(TimerLastOutcome::Completed(completion.outcome()))
        );
        assert_eq!(outcomes.last_work_count(), Some(completion.work_count()));
    }
    outcomes.record_unacknowledged(20);
    assert_eq!(
        outcomes.last_outcome(),
        Some(TimerLastOutcome::Unacknowledged)
    );
    assert_eq!(outcomes.last_work_count(), None);
    assert_eq!(outcomes.last_success_at_ns(), Some(11));
    assert_eq!(outcomes.last_failure_at_ns(), Some(12));
    assert_eq!(outcomes.last_unacknowledged_at_ns(), Some(20));
    assert_eq!(outcomes.consecutive_expected_failures(), 1);

    outcomes.record_completion(TimerCompletion::invariant_failure(2), 30);
    assert_eq!(
        outcomes.last_outcome(),
        Some(TimerLastOutcome::Completed(
            TimerCompletionOutcome::InvariantFailure
        ))
    );
    assert_eq!(outcomes.last_work_count(), Some(2));
    assert_eq!(outcomes.last_success_at_ns(), Some(11));
    assert_eq!(outcomes.last_failure_at_ns(), Some(30));
    assert_eq!(outcomes.last_unacknowledged_at_ns(), Some(20));
    assert_eq!(outcomes.consecutive_expected_failures(), 0);

    outcomes.record_completion(TimerCompletion::success(4), 40);
    assert_eq!(
        outcomes.last_outcome(),
        Some(TimerLastOutcome::Completed(TimerCompletionOutcome::Success))
    );
    assert_eq!(outcomes.last_work_count(), Some(4));
    assert_eq!(outcomes.last_success_at_ns(), Some(40));
    assert_eq!(outcomes.last_failure_at_ns(), Some(30));
    assert_eq!(outcomes.last_unacknowledged_at_ns(), Some(20));
}

#[test]
fn expected_failure_streak_saturates() {
    let mut outcomes = TimerOutcomeSnapshot {
        consecutive_expected_failures: u64::MAX,
        ..TimerOutcomeSnapshot::EMPTY
    };

    outcomes.record_completion(TimerCompletion::retryable_failure(0), 10);

    assert_eq!(outcomes.consecutive_expected_failures(), u64::MAX);
}

#[test]
fn invariant_results_are_forced_to_stop() {
    let watchdog = WatchdogRunResult::new(
        TimerCompletion::invariant_failure(3),
        WatchdogDecision::ContinueImmediately,
    );
    assert_eq!(watchdog.decision(), WatchdogDecision::Stop);
}
