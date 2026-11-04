//! Registration-local Metrics counters and bounded callback measurements.

use super::{TimerCompletion, TimerCompletionOutcome, TimerOutcomeSnapshot};
use ic_metrics::{MeasurementCounter, MeasurementSummary};

/// Registration-local timer event counters.
///
/// Metrics owns arithmetic and actual-overflow tracking; the registry is the
/// sole writer of these named events. Completed work merges its four classified
/// outcomes. Getters return inert copies, not mutation authority.
///
/// Check the parent snapshot's full registration identity before comparing
/// observations; epoch equality alone does not prove continuity. Reporting and
/// interval admission belong to the consumer. Use checked counter observations
/// rather than inferring overflow
/// from a numeric cap: an exact `u128::MAX` remains distinguishable from overflow.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerCounters {
    schedule_requests: MeasurementCounter,
    wakeups_armed: MeasurementCounter,
    work_dispatched: MeasurementCounter,
    scheduler_started: MeasurementCounter,
    work_started: MeasurementCounter,
    succeeded: MeasurementCounter,
    no_work: MeasurementCounter,
    retryable_failure: MeasurementCounter,
    invariant_failure: MeasurementCounter,
    cancelled: MeasurementCounter,
    stale_wakeups: MeasurementCounter,
    stale_work: MeasurementCounter,
    coalesced: MeasurementCounter,
    unacknowledged: MeasurementCounter,
}

impl TimerCounters {
    const EMPTY: Self = Self {
        schedule_requests: MeasurementCounter::ZERO,
        wakeups_armed: MeasurementCounter::ZERO,
        work_dispatched: MeasurementCounter::ZERO,
        scheduler_started: MeasurementCounter::ZERO,
        work_started: MeasurementCounter::ZERO,
        succeeded: MeasurementCounter::ZERO,
        no_work: MeasurementCounter::ZERO,
        retryable_failure: MeasurementCounter::ZERO,
        invariant_failure: MeasurementCounter::ZERO,
        cancelled: MeasurementCounter::ZERO,
        stale_wakeups: MeasurementCounter::ZERO,
        stale_work: MeasurementCounter::ZERO,
        coalesced: MeasurementCounter::ZERO,
        unacknowledged: MeasurementCounter::ZERO,
    };

    pub(crate) const fn record_schedule_request(&mut self) {
        self.schedule_requests.add(1);
    }

    pub(crate) const fn record_wakeup_armed(&mut self) {
        self.wakeups_armed.add(1);
    }

    pub(crate) const fn record_work_dispatched(&mut self) {
        self.work_dispatched.add(1);
    }

    pub(crate) const fn record_scheduler_started(&mut self) {
        self.scheduler_started.add(1);
    }

    pub(crate) const fn record_work_started(&mut self) {
        self.work_started.add(1);
    }

    pub(crate) const fn record_completion(&mut self, outcome: TimerCompletionOutcome) {
        match outcome {
            TimerCompletionOutcome::Success => {
                self.succeeded.add(1);
            }
            TimerCompletionOutcome::NoWork => {
                self.no_work.add(1);
            }
            TimerCompletionOutcome::RetryableFailure => {
                self.retryable_failure.add(1);
            }
            TimerCompletionOutcome::InvariantFailure => {
                self.invariant_failure.add(1);
            }
        }
    }

    pub(crate) const fn record_cancellation(&mut self) {
        self.cancelled.add(1);
    }

    pub(crate) const fn record_stale_wakeup(&mut self) {
        self.stale_wakeups.add(1);
    }

    pub(crate) const fn record_stale_work(&mut self) {
        self.stale_work.add(1);
    }

    pub(crate) const fn record_coalesced(&mut self) {
        self.coalesced.add(1);
    }

    pub(crate) const fn record_unacknowledged(&mut self) {
        self.unacknowledged.add(1);
    }

    /// Return validated activation and reconciliation requests.
    #[must_use]
    pub const fn schedule_requests(self) -> MeasurementCounter {
        self.schedule_requests
    }

    /// Return ordinary or scheduler provider one-shots armed.
    #[must_use]
    pub const fn wakeups_armed(self) -> MeasurementCounter {
        self.wakeups_armed
    }

    /// Return immediate watchdog work one-shots dispatched.
    #[must_use]
    pub const fn work_dispatched(self) -> MeasurementCounter {
        self.work_dispatched
    }

    /// Return accepted watchdog scheduler callbacks.
    #[must_use]
    pub const fn scheduler_started(self) -> MeasurementCounter {
        self.scheduler_started
    }

    /// Return accepted consumer-work callbacks.
    #[must_use]
    pub const fn work_started(self) -> MeasurementCounter {
        self.work_started
    }

    /// Return consumer work whose completion accounting committed.
    ///
    /// Merge the four classified counters, preserving actual-overflow quality.
    #[must_use]
    pub const fn work_completed(self) -> MeasurementCounter {
        let mut completed = self.succeeded;
        completed.merge(self.no_work);
        completed.merge(self.retryable_failure);
        completed.merge(self.invariant_failure);
        completed
    }

    /// Return successful-work completions.
    #[must_use]
    pub const fn succeeded(self) -> MeasurementCounter {
        self.succeeded
    }

    /// Return valid no-work completions.
    #[must_use]
    pub const fn no_work(self) -> MeasurementCounter {
        self.no_work
    }

    /// Return retryable expected-failure completions.
    #[must_use]
    pub const fn retryable_failure(self) -> MeasurementCounter {
        self.retryable_failure
    }

    /// Return invariant or terminal-failure completions.
    #[must_use]
    pub const fn invariant_failure(self) -> MeasurementCounter {
        self.invariant_failure
    }

    /// Return logical cancellations that changed authoritative state.
    #[must_use]
    pub const fn cancelled(self) -> MeasurementCounter {
        self.cancelled
    }

    /// Return rejected ordinary or scheduler callback generations.
    #[must_use]
    pub const fn stale_wakeups(self) -> MeasurementCounter {
        self.stale_wakeups
    }

    /// Return rejected watchdog work generations.
    #[must_use]
    pub const fn stale_work(self) -> MeasurementCounter {
        self.stale_work
    }

    /// Return scheduling demand satisfied without another logical arm.
    #[must_use]
    pub const fn coalesced(self) -> MeasurementCounter {
        self.coalesced
    }

    /// Return confirmed ordinary deliveries dropped without completion and
    /// committed Watchdog dispatches retired by their successor without completion.
    #[must_use]
    pub const fn unacknowledged(self) -> MeasurementCounter {
        self.unacknowledged
    }
}

/// Wasm and stable memory extents in 64 KiB pages at one instant.
///
/// Within one runtime epoch these are monotonic page extents, not allocator
/// liveness or exact live-byte measurements. Consumers that need live bytes
/// must supply an owner-derived bound for allocations within the final page.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryPageExtent {
    wasm: u64,
    stable: u64,
}

impl MemoryPageExtent {
    const EMPTY: Self = Self { wasm: 0, stable: 0 };

    pub(crate) const fn new(wasm: u64, stable: u64) -> Self {
        Self { wasm, stable }
    }

    /// Return the Wasm linear-memory extent in 64 KiB pages.
    #[must_use]
    pub const fn wasm_pages(self) -> u64 {
        self.wasm
    }

    /// Return the stable-memory extent in 64 KiB pages.
    #[must_use]
    pub const fn stable_pages(self) -> u64 {
        self.stable
    }
}

/// Page extents observed at the start and end of one normally completed
/// callback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryPageSample {
    start: MemoryPageExtent,
    end: MemoryPageExtent,
}

impl MemoryPageSample {
    const EMPTY: Self = Self {
        start: MemoryPageExtent::EMPTY,
        end: MemoryPageExtent::EMPTY,
    };

    pub(crate) const fn new(start: MemoryPageExtent, end: MemoryPageExtent) -> Self {
        Self { start, end }
    }

    /// Return page extents sampled at callback start.
    #[must_use]
    pub const fn start(self) -> MemoryPageExtent {
        self.start
    }

    /// Return page extents sampled after normal callback completion.
    #[must_use]
    pub const fn end(self) -> MemoryPageExtent {
        self.end
    }

    /// Return non-negative Wasm-memory page growth observed between samples.
    ///
    /// For async ordinary work, the interval may include interleaved canister
    /// activity while the callback future is awaiting.
    #[must_use]
    pub const fn wasm_growth_pages(self) -> u64 {
        self.end.wasm.saturating_sub(self.start.wasm)
    }

    /// Return non-negative stable-memory page growth observed between samples.
    ///
    /// For async ordinary work, the interval may include interleaved canister
    /// activity while the callback future is awaiting.
    #[must_use]
    pub const fn stable_growth_pages(self) -> u64 {
        self.end.stable.saturating_sub(self.start.stable)
    }
}

/// Bounded page-extent observations for one callback role.
///
/// Absolute page extents are retained only for the latest normal completion;
/// they are never totaled. Maximums describe observed start-to-end page
/// growth, which is not exclusive allocation attribution for async work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryPageSummary {
    samples: u64,
    latest: MemoryPageSample,
    maximum_wasm_growth: u64,
    maximum_stable_growth: u64,
}

impl MemoryPageSummary {
    const EMPTY: Self = Self {
        samples: 0,
        latest: MemoryPageSample::EMPTY,
        maximum_wasm_growth: 0,
        maximum_stable_growth: 0,
    };

    const fn record(&mut self, sample: MemoryPageSample) {
        self.samples = self.samples.saturating_add(1);
        self.latest = sample;
        self.maximum_wasm_growth = max_u64(self.maximum_wasm_growth, sample.wasm_growth_pages());
        self.maximum_stable_growth =
            max_u64(self.maximum_stable_growth, sample.stable_growth_pages());
    }

    /// Return the number of normally completed callback samples.
    #[must_use]
    pub const fn samples(self) -> u64 {
        self.samples
    }

    /// Return start/end page extents for the latest normal completion.
    #[must_use]
    pub const fn latest(self) -> Option<MemoryPageSample> {
        if self.samples == 0 {
            None
        } else {
            Some(self.latest)
        }
    }

    /// Return the largest observed Wasm-memory page growth in one sample.
    #[must_use]
    pub const fn maximum_wasm_growth_pages(self) -> Option<u64> {
        if self.samples == 0 {
            None
        } else {
            Some(self.maximum_wasm_growth)
        }
    }

    /// Return the largest observed stable-memory page growth in one sample.
    #[must_use]
    pub const fn maximum_stable_growth_pages(self) -> Option<u64> {
        if self.samples == 0 {
            None
        } else {
            Some(self.maximum_stable_growth)
        }
    }
}

const fn max_u64(left: u64, right: u64) -> u64 {
    if left > right { left } else { right }
}

/// Completed instruction and memory-page measurements split by callback role.
///
/// Each interval covers the accepted `ic-timers` callback envelope: it starts
/// before callback acceptance and ends after completion processing and any
/// successor binding. It is not an exclusive measurement of consumer code.
/// Start/end page reads bracket the instruction interval from outside.
///
/// Instruction summaries retain observation ordering and track actual overflow.
/// Totals are `u128`; individual samples and counts are `u64`.
/// Totals above `u64::MAX` remain exact. Read aggregate overflow flags rather
/// than inferring overflow from the numeric cap. Consumers own report means,
/// rounding and interval admission. Event counters also track actual overflow
/// through Metrics; page-sample counts remain conservatively saturated at their
/// numeric cap.
///
/// A terminal [`DeclarationLifetime::RemoveWhenStopped`](crate::DeclarationLifetime::RemoveWhenStopped)
/// callback can remove its declaration before this final record is retained;
/// no timer then remains in the inventory to expose that sample.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerPerformance {
    scheduler_instructions: MeasurementSummary,
    work_instructions: MeasurementSummary,
    scheduler_memory_pages: MemoryPageSummary,
    work_memory_pages: MemoryPageSummary,
}

impl TimerPerformance {
    const EMPTY: Self = Self {
        scheduler_instructions: MeasurementSummary::EMPTY,
        work_instructions: MeasurementSummary::EMPTY,
        scheduler_memory_pages: MemoryPageSummary::EMPTY,
        work_memory_pages: MemoryPageSummary::EMPTY,
    };

    pub(crate) const fn record_scheduler(&mut self, instructions: u64, memory: MemoryPageSample) {
        self.scheduler_instructions.record(instructions);
        self.scheduler_memory_pages.record(memory);
    }

    pub(crate) const fn record_work(&mut self, instructions: u64, memory: MemoryPageSample) {
        self.work_instructions.record(instructions);
        self.work_memory_pages.record(memory);
    }

    /// Return normally completed accepted scheduler-envelope measurements.
    #[must_use]
    pub const fn scheduler_instructions(self) -> MeasurementSummary {
        self.scheduler_instructions
    }

    /// Return normally completed accepted work-envelope measurements.
    #[must_use]
    pub const fn work_instructions(self) -> MeasurementSummary {
        self.work_instructions
    }

    /// Return normally completed accepted scheduler-envelope page observations.
    #[must_use]
    pub const fn scheduler_memory_pages(self) -> MemoryPageSummary {
        self.scheduler_memory_pages
    }

    /// Return normally completed accepted work-envelope page observations.
    #[must_use]
    pub const fn work_memory_pages(self) -> MemoryPageSummary {
        self.work_memory_pages
    }
}

/// Registration-scoped outcomes, counters, and performance for one timer.
///
/// These values compare measurements, not registration identity. Keep the parent
/// snapshot's [`super::TimerSnapshot::registration_id`] alongside copied values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerObservabilitySnapshot {
    outcomes: TimerOutcomeSnapshot,
    counters: TimerCounters,
    performance: TimerPerformance,
}

impl TimerObservabilitySnapshot {
    pub(crate) const EMPTY: Self = Self {
        outcomes: TimerOutcomeSnapshot::EMPTY,
        counters: TimerCounters::EMPTY,
        performance: TimerPerformance::EMPTY,
    };

    pub(crate) const fn record_completion(
        &mut self,
        completion: TimerCompletion,
        completed_at_ns: u64,
    ) {
        self.outcomes.record_completion(completion, completed_at_ns);
        self.counters.record_completion(completion.outcome());
    }

    pub(crate) const fn record_unacknowledged(&mut self, observed_at_ns: u64) {
        self.outcomes.record_unacknowledged(observed_at_ns);
        self.counters.record_unacknowledged();
    }

    pub(crate) const fn counters_mut(&mut self) -> &mut TimerCounters {
        &mut self.counters
    }

    pub(crate) const fn record_scheduler_measurements(
        &mut self,
        instructions: u64,
        memory: MemoryPageSample,
    ) {
        self.performance.record_scheduler(instructions, memory);
    }

    pub(crate) const fn record_work_measurements(
        &mut self,
        instructions: u64,
        memory: MemoryPageSample,
    ) {
        self.performance.record_work(instructions, memory);
    }

    /// Return latest outcomes and functional failure state.
    #[must_use]
    pub const fn outcomes(self) -> TimerOutcomeSnapshot {
        self.outcomes
    }

    /// Return registration-local event counters.
    #[must_use]
    pub const fn counters(self) -> TimerCounters {
        self.counters
    }

    /// Return completed instruction and memory-page measurements.
    #[must_use]
    pub const fn performance(self) -> TimerPerformance {
        self.performance
    }
}

#[cfg(test)]
mod tests {
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
}
