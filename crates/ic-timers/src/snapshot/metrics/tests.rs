use super::*;

#[test]
fn all_counters_saturate() {
    let mut exact_cap = MeasurementCounter::ZERO;
    exact_cap.add(u128::MAX);
    let mut counters = TimerCounters {
        schedule_requests: exact_cap,
        wakeups_armed: exact_cap,
        work_dispatched: exact_cap,
        scheduler_started: exact_cap,
        work_started: exact_cap,
        succeeded: exact_cap,
        no_work: exact_cap,
        retryable_failure: exact_cap,
        invariant_failure: exact_cap,
        cancelled: exact_cap,
        stale_wakeups: exact_cap,
        stale_work: exact_cap,
        coalesced: exact_cap,
        unacknowledged: exact_cap,
    };
    assert!(!counters.schedule_requests().total_overflowed());
    counters.record_schedule_request();
    counters.record_wakeup_armed();
    counters.record_work_dispatched();
    counters.record_scheduler_started();
    counters.record_work_started();
    for outcome in [
        TimerCompletionOutcome::Success,
        TimerCompletionOutcome::NoWork,
        TimerCompletionOutcome::RetryableFailure,
        TimerCompletionOutcome::InvariantFailure,
    ] {
        counters.record_completion(outcome);
    }
    counters.record_cancellation();
    counters.record_stale_wakeup();
    counters.record_stale_work();
    counters.record_coalesced();
    counters.record_unacknowledged();

    // Metrics owns overflow arithmetic; each event must forward to its counter.
    let mut expected = exact_cap;
    expected.add(1);
    for observed in [
        counters.schedule_requests(),
        counters.wakeups_armed(),
        counters.work_dispatched(),
        counters.scheduler_started(),
        counters.work_started(),
        counters.succeeded(),
        counters.no_work(),
        counters.retryable_failure(),
        counters.invariant_failure(),
        counters.cancelled(),
        counters.stale_wakeups(),
        counters.stale_work(),
        counters.coalesced(),
        counters.unacknowledged(),
    ] {
        assert_eq!(observed, expected);
    }
}

#[test]
fn completion_totals_follow_classifications_and_saturate_across_outcomes() {
    let mut counters = TimerCounters::EMPTY;
    assert_eq!(counters.work_completed().exact_total(), Ok(0));
    for (outcome, total) in [
        (TimerCompletionOutcome::Success, 1),
        (TimerCompletionOutcome::NoWork, 2),
        (TimerCompletionOutcome::RetryableFailure, 3),
        (TimerCompletionOutcome::InvariantFailure, 4),
    ] {
        counters.record_completion(outcome);
        assert_eq!(counters.work_completed().exact_total(), Ok(total));
    }
    assert_eq!(counters.succeeded().exact_total(), Ok(1));
    assert_eq!(counters.no_work().exact_total(), Ok(1));
    assert_eq!(counters.retryable_failure().exact_total(), Ok(1));
    assert_eq!(counters.invariant_failure().exact_total(), Ok(1));
    assert_eq!(counters.work_started().exact_total(), Ok(0));

    let mut near_cap = MeasurementCounter::ZERO;
    near_cap.add(u128::MAX - 2);
    let mut counters = TimerCounters {
        succeeded: near_cap,
        ..TimerCounters::EMPTY
    };
    counters.record_completion(TimerCompletionOutcome::NoWork);
    assert_eq!(counters.work_completed().exact_total(), Ok(u128::MAX - 1));
    counters.record_completion(TimerCompletionOutcome::RetryableFailure);
    assert_eq!(counters.work_completed().exact_total(), Ok(u128::MAX));
    counters.record_completion(TimerCompletionOutcome::InvariantFailure);
    assert!(counters.work_completed().exact_total().is_err());
    // Merge must preserve a source's overflow, even without a new carry.
    let mut overflowed = MeasurementCounter::ZERO;
    overflowed.add(u128::MAX);
    overflowed.add(1);
    counters = TimerCounters {
        succeeded: overflowed,
        ..TimerCounters::EMPTY
    };
    assert!(counters.work_completed().total_overflowed());
}

#[test]
fn counter_copies_cross_the_old_cap_without_mutating_registration_state() {
    let mut old_cap = MeasurementCounter::ZERO;
    old_cap.add(u128::from(u64::MAX));
    let mut counters = TimerCounters {
        schedule_requests: old_cap,
        ..TimerCounters::EMPTY
    };
    let previous = counters.schedule_requests();
    counters.record_schedule_request();
    assert_eq!(
        counters.schedule_requests().exact_total(),
        Ok(u128::from(u64::MAX) + 1)
    );
    assert_eq!(previous.exact_total(), Ok(u128::from(u64::MAX)));
    let observed = counters.schedule_requests();
    let mut copied = counters.schedule_requests();
    copied.add(10);
    assert_eq!(counters.schedule_requests(), observed);
    assert_ne!(copied, observed);
}

#[test]
fn callback_measurements_are_role_specific_bounded_and_saturating() {
    let mut performance = TimerPerformance::EMPTY;
    performance.record_scheduler(
        20,
        MemoryPageSample::new(MemoryPageExtent::new(1, 2), MemoryPageExtent::new(2, 4)),
    );
    performance.record_work(
        30,
        MemoryPageSample::new(MemoryPageExtent::new(2, 4), MemoryPageExtent::new(5, 5)),
    );
    performance.record_work(
        10,
        MemoryPageSample::new(MemoryPageExtent::new(5, 5), MemoryPageExtent::new(6, 9)),
    );

    assert_eq!(performance.scheduler_instructions().aggregate().total(), 20);
    assert_eq!(performance.work_instructions().aggregate().samples(), 2);
    assert_eq!(performance.work_instructions().aggregate().total(), 40);
    assert_eq!(performance.work_instructions().latest(), Some(10));
    assert_eq!(
        performance.work_instructions().aggregate().maximum(),
        Some(30)
    );

    let scheduler_memory = performance.scheduler_memory_pages();
    assert_eq!(
        scheduler_memory.samples(),
        performance.scheduler_instructions().aggregate().samples()
    );
    assert_eq!(scheduler_memory.samples(), 1);
    let scheduler_latest = scheduler_memory
        .latest()
        .expect("scheduler sample should exist");
    assert_eq!(scheduler_latest.start().wasm_pages(), 1);
    assert_eq!(scheduler_latest.start().stable_pages(), 2);
    assert_eq!(scheduler_latest.end().wasm_pages(), 2);
    assert_eq!(scheduler_latest.end().stable_pages(), 4);
    assert_eq!(scheduler_latest.wasm_growth_pages(), 1);
    assert_eq!(scheduler_latest.stable_growth_pages(), 2);

    let work_memory = performance.work_memory_pages();
    assert_eq!(
        work_memory.samples(),
        performance.work_instructions().aggregate().samples()
    );
    assert_eq!(work_memory.samples(), 2);
    let work_latest = work_memory.latest().expect("work sample should exist");
    assert_eq!(work_latest.wasm_growth_pages(), 1);
    assert_eq!(work_latest.stable_growth_pages(), 4);
    assert_eq!(work_memory.maximum_wasm_growth_pages(), Some(3));
    assert_eq!(work_memory.maximum_stable_growth_pages(), Some(4));
}

#[test]
fn instruction_summaries_keep_empty_zero_and_role_boundaries() {
    let mut performance = TimerPerformance::EMPTY;
    let mut expected = MeasurementSummary::EMPTY;
    assert_eq!(performance.scheduler_instructions(), expected);
    assert_eq!(performance.work_instructions(), expected);

    // Metrics owns summary arithmetic; Timers owns forwarding and role isolation.
    for instructions in [0, u64::MAX, 1, u64::MAX] {
        let previous_work = performance.work_instructions();
        expected.record(instructions);
        performance.record_scheduler(instructions, MemoryPageSample::EMPTY);
        assert_eq!(performance.scheduler_instructions(), expected);
        assert_eq!(performance.work_instructions(), previous_work);

        performance.record_work(instructions, MemoryPageSample::EMPTY);
        assert_eq!(performance.work_instructions(), expected);
        assert_eq!(performance.scheduler_instructions(), expected);
    }
}

#[test]
fn memory_sample_count_saturates_while_latest_and_maximum_continue() {
    let mut summary = MemoryPageSummary {
        samples: u64::MAX,
        latest: MemoryPageSample::EMPTY,
        maximum_wasm_growth: 1,
        maximum_stable_growth: 1,
    };
    let sample =
        MemoryPageSample::new(MemoryPageExtent::new(10, 20), MemoryPageExtent::new(13, 25));

    summary.record(sample);

    assert_eq!(summary.samples(), u64::MAX);
    assert_eq!(summary.latest(), Some(sample));
    assert_eq!(summary.maximum_wasm_growth_pages(), Some(3));
    assert_eq!(summary.maximum_stable_growth_pages(), Some(5));
}
