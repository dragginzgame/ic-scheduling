//! Closed policy, state, outcome, and epoch values.

use crate::{
    callback::{AfterCompletionDecision, OnceDecision},
    schedule::{OrdinaryDirective, ScheduleError, TimerCadence, duration_ns},
};

/// Configured recurrence policy for one logical timer.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerPolicy {
    /// Run at most once unless an explicit directive or request schedules it.
    Once,
    /// Permit recurrence at a configured cadence after callback completion.
    AfterCompletion {
        /// Validated configured cadence.
        cadence: TimerCadence,
    },
    /// Commit a successor before dispatching synchronous fallible work.
    Watchdog {
        /// Validated configured cadence.
        cadence: TimerCadence,
    },
}

impl TimerPolicy {
    /// Return the stable configured-policy label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Once => "once",
            Self::AfterCompletion { .. } => "after_completion",
            Self::Watchdog { .. } => "watchdog",
        }
    }

    /// Return a configured cadence when the policy recurs.
    #[must_use]
    pub const fn cadence(self) -> Option<TimerCadence> {
        match self {
            Self::Once => None,
            Self::AfterCompletion { cadence } | Self::Watchdog { cadence } => Some(cadence),
        }
    }
}

/// Whether a stopped declaration remains in the bounded registry.
///
/// This is an observation of declaration lifetime. Direct registration uses
/// [`crate::TimerStart`] to couple retention with a valid initial schedule.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeclarationLifetime {
    /// Keep callback authority available for a later ensure request.
    Retained,
    /// Remove the declaration after terminal completion or cancellation.
    RemoveWhenStopped,
}

/// Latest effective scheduling reason for a declaration.
///
/// The value remains observable while inactive and is not itself evidence of
/// an armed callback.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerSchedulingMode {
    /// An explicit relative-delay schedule, including initial one-shot work.
    Once,
    /// Recurrence delayed from the previous completion.
    AfterCompletion,
    /// An explicit absolute deadline.
    Deadline,
    /// A delayed retry following an expected failure.
    Retry,
    /// Immediate continuation of bounded work.
    Continuation,
    /// A successor committed before fallible watchdog work.
    Watchdog,
}

impl TimerSchedulingMode {
    /// Return a stable adapter-friendly label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Once => "once",
            Self::AfterCompletion => "after_completion",
            Self::Deadline => "deadline",
            Self::Retry => "retry",
            Self::Continuation => "continuation",
            Self::Watchdog => "watchdog",
        }
    }
}

/// Portable representation of the effective ordinary scheduling directive
/// after authoritative completion commands have been applied.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerDirectiveSnapshot {
    /// Stop after the completed invocation.
    Stop,
    /// Continue in the next available timer message.
    ContinueImmediately,
    /// Retry after a relative delay.
    RetryAfter {
        /// Requested delay in nanoseconds.
        delay_ns: u64,
    },
    /// Schedule at one absolute IC timestamp.
    ScheduleAt {
        /// Absolute IC timestamp in nanoseconds.
        deadline_ns: u64,
    },
    /// Recur using the registration's configured cadence.
    RecurAfterCompletion,
}

impl TryFrom<OrdinaryDirective> for TimerDirectiveSnapshot {
    type Error = ScheduleError;

    fn try_from(value: OrdinaryDirective) -> Result<Self, Self::Error> {
        Ok(match value {
            OrdinaryDirective::Stop => Self::Stop,
            OrdinaryDirective::ContinueImmediately => Self::ContinueImmediately,
            OrdinaryDirective::RetryAfter(delay) => Self::RetryAfter {
                delay_ns: duration_ns(delay)?,
            },
            OrdinaryDirective::ScheduleAt(deadline_ns) => Self::ScheduleAt { deadline_ns },
            OrdinaryDirective::RecurAfterCompletion => Self::RecurAfterCompletion,
        })
    }
}

impl TryFrom<OnceDecision> for TimerDirectiveSnapshot {
    type Error = ScheduleError;

    fn try_from(value: OnceDecision) -> Result<Self, Self::Error> {
        Self::try_from(OrdinaryDirective::from(value))
    }
}

impl TryFrom<AfterCompletionDecision> for TimerDirectiveSnapshot {
    type Error = ScheduleError;

    fn try_from(value: AfterCompletionDecision) -> Result<Self, Self::Error> {
        Self::try_from(OrdinaryDirective::from(value))
    }
}

/// Typed terminal failure in pure timer control.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerControlFailure {
    /// A new callback would exceed its generation counter's maximum.
    /// Cancellation does not allocate a generation and cannot cause this failure.
    GenerationExhausted,
    /// Checked successor deadline arithmetic overflowed.
    DeadlineOverflow,
    /// A requested relative delay cannot be encoded as `u64` nanoseconds.
    DelayOutOfRange,
    /// An internal erased proposal is inconsistent with its configured policy.
    /// Public callback result types cannot express a policy-invalid decision.
    DirectiveNotAllowed,
    /// A checked registry effect could not establish canonical provider ownership.
    ProviderBindingFailed,
}

impl TimerControlFailure {
    /// Return a stable adapter-friendly label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::GenerationExhausted => "generation_exhausted",
            Self::DeadlineOverflow => "deadline_overflow",
            Self::DelayOutOfRange => "delay_out_of_range",
            Self::DirectiveNotAllowed => "directive_not_allowed",
            Self::ProviderBindingFailed => "provider_binding_failed",
        }
    }
}

/// Why a declaration currently has no scheduled or running callback generation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InactiveReason {
    /// The declaration has not yet been scheduled.
    NeverScheduled,
    /// Work returned a terminal stop decision.
    Stopped,
    /// Explicit cancellation won request arbitration.
    Cancelled,
    /// A confirmed ordinary delivery was dropped without normal completion.
    /// This records abandonment, not the cause or recovery of application work.
    Abandoned,
    /// Consumer work reported an invariant or terminal failure.
    InvariantFailure,
    /// Checked pure control reached a terminal failure.
    ControlFailure(TimerControlFailure),
}

/// Coherent ordinary timer state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrdinaryRuntimeStateSnapshot {
    /// One callback generation is scheduled.
    Scheduled {
        /// Generation the callback must present.
        generation: u64,
        /// Authoritative absolute deadline.
        deadline_ns: u64,
    },
    /// One callback generation owns logical execution.
    Running {
        /// Generation owned by the running callback.
        generation: u64,
    },
}

/// Status of the watchdog attempt paired with a committed successor.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WatchdogAttemptStatus {
    /// The scheduler committed the work callback, which has not committed a start.
    Dispatched,
    /// The accepted work callback is currently executing synchronously.
    Running,
}

/// Coherent watchdog timer state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WatchdogRuntimeStateSnapshot {
    /// One scheduler generation is authoritative and no work is outstanding.
    Scheduled {
        /// Generation the scheduler callback must present.
        scheduler_generation: u64,
        /// Authoritative absolute successor deadline.
        deadline_ns: u64,
    },
    /// A successor is authoritative while one work attempt is outstanding.
    AwaitingWork {
        /// Dispatch generation the successor scheduler must present, shared
        /// with the paired work attempt.
        successor_generation: u64,
        /// Absolute successor deadline.
        successor_deadline_ns: u64,
        /// Whether the paired work is dispatched or running.
        attempt_status: WatchdogAttemptStatus,
    },
}

/// Closed policy-specific runtime state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerRuntimeStateSnapshot {
    /// The declaration has no scheduled or running callback generation.
    Inactive {
        /// Reason scheduling is inactive.
        reason: InactiveReason,
    },
    /// State legal only for `Once` and `AfterCompletion`.
    Ordinary(OrdinaryRuntimeStateSnapshot),
    /// State legal only for `Watchdog`.
    Watchdog(WatchdogRuntimeStateSnapshot),
}

impl TimerRuntimeStateSnapshot {
    pub(crate) const fn next_deadline_ns(self) -> Option<u64> {
        match self {
            Self::Inactive { .. }
            | Self::Ordinary(OrdinaryRuntimeStateSnapshot::Running { .. }) => None,
            Self::Ordinary(OrdinaryRuntimeStateSnapshot::Scheduled { deadline_ns, .. })
            | Self::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled { deadline_ns, .. }) => {
                Some(deadline_ns)
            }
            Self::Watchdog(WatchdogRuntimeStateSnapshot::AwaitingWork {
                successor_deadline_ns,
                ..
            }) => Some(successor_deadline_ns),
        }
    }
}

/// Portable projection of provider callback-generation state.
///
/// This is independent of declaration lifetime: a retained declaration may
/// report `Unregistered` while it keeps callback authority for a later ensure.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerRegistrationStatus {
    /// No callback generation is scheduled or running.
    Unregistered,
    /// A wake-up generation is authoritative and consumer work is not running.
    Scheduled,
    /// Consumer work currently owns logical execution.
    Running,
}

impl TimerRegistrationStatus {
    /// Return a stable adapter-friendly label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Unregistered => "unregistered",
            Self::Scheduled => "scheduled",
            Self::Running => "running",
        }
    }
}

impl From<TimerRuntimeStateSnapshot> for TimerRegistrationStatus {
    fn from(value: TimerRuntimeStateSnapshot) -> Self {
        match value {
            TimerRuntimeStateSnapshot::Inactive { .. } => Self::Unregistered,
            TimerRuntimeStateSnapshot::Ordinary(state) => match state {
                OrdinaryRuntimeStateSnapshot::Scheduled { .. } => Self::Scheduled,
                OrdinaryRuntimeStateSnapshot::Running { .. } => Self::Running,
            },
            TimerRuntimeStateSnapshot::Watchdog(state) => match state {
                WatchdogRuntimeStateSnapshot::Scheduled { .. }
                | WatchdogRuntimeStateSnapshot::AwaitingWork {
                    attempt_status: WatchdogAttemptStatus::Dispatched,
                    ..
                } => Self::Scheduled,
                WatchdogRuntimeStateSnapshot::AwaitingWork {
                    attempt_status: WatchdogAttemptStatus::Running,
                    ..
                } => Self::Running,
            },
        }
    }
}

/// Outcome of one callback that returned to the runtime.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerCompletionOutcome {
    /// Callback completed useful work successfully.
    Success,
    /// Callback completed normally but found no work.
    NoWork,
    /// Expected failure permits retry policy.
    RetryableFailure,
    /// Unexpected invariant or terminal failure.
    InvariantFailure,
}

impl TimerCompletionOutcome {
    /// Return a stable adapter-friendly label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::NoWork => "no_work",
            Self::RetryableFailure => "retryable_failure",
            Self::InvariantFailure => "invariant_failure",
        }
    }
}

/// Latest observed terminal event for one timer invocation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerLastOutcome {
    /// One callback returned with a classified completion.
    Completed(TimerCompletionOutcome),
    /// A confirmed ordinary delivery was dropped, or a committed Watchdog
    /// dispatch was retired by its successor, without a completion.
    Unacknowledged,
}

/// Classified result of one returned consumer invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerCompletion {
    outcome: TimerCompletionOutcome,
    work_count: u64,
}

impl TimerCompletion {
    /// Construct successful completed work.
    #[must_use]
    pub const fn success(work_count: u64) -> Self {
        Self {
            outcome: TimerCompletionOutcome::Success,
            work_count,
        }
    }

    /// Construct a valid no-work completion.
    #[must_use]
    pub const fn no_work() -> Self {
        Self {
            outcome: TimerCompletionOutcome::NoWork,
            work_count: 0,
        }
    }

    /// Construct an expected failure, retaining completed partial work.
    #[must_use]
    pub const fn retryable_failure(work_count: u64) -> Self {
        Self {
            outcome: TimerCompletionOutcome::RetryableFailure,
            work_count,
        }
    }

    /// Construct an invariant failure, retaining completed partial work.
    #[must_use]
    pub const fn invariant_failure(work_count: u64) -> Self {
        Self {
            outcome: TimerCompletionOutcome::InvariantFailure,
            work_count,
        }
    }

    /// Return the completion class.
    #[must_use]
    pub const fn outcome(self) -> TimerCompletionOutcome {
        self.outcome
    }

    /// Return application work units reported by the consumer.
    #[must_use]
    pub const fn work_count(self) -> u64 {
        self.work_count
    }
}

/// Watchdog decision after one synchronous bounded work attempt.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WatchdogDecision {
    /// Retain the successor committed by the scheduler message.
    Continue,
    /// Move the committed successor to the current IC time.
    ///
    /// The replacement remains a later scheduler message; this never invokes
    /// consumer work recursively in the current message.
    ContinueImmediately,
    /// Replace the committed successor with an exact absolute IC deadline.
    ///
    /// A past deadline schedules a later message with zero delay. If work
    /// traps, this proposal rolls back and the cadence successor remains.
    ScheduleAt(u64),
    /// Terminate and clear the committed successor.
    Stop,
}

/// Synchronous watchdog work result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatchdogRunResult {
    completion: TimerCompletion,
    decision: WatchdogDecision,
}

impl WatchdogRunResult {
    /// Report a completed batch and its next consumer-selected deadline.
    ///
    /// Zero work reports no work; nonzero work reports success. Work units are
    /// defined by the caller and need not mean confirmed external effects.
    /// `None` stops the watchdog; a deadline schedules its next attempt.
    /// Use [`Self::new`] to report failures or retain the cadence successor.
    #[must_use]
    pub const fn completed_batch(work_count: u64, next_due_ns: Option<u64>) -> Self {
        let completion = if work_count == 0 {
            TimerCompletion::no_work()
        } else {
            TimerCompletion::success(work_count)
        };
        let decision = match next_due_ns {
            Some(at_ns) => WatchdogDecision::ScheduleAt(at_ns),
            None => WatchdogDecision::Stop,
        };
        Self::new(completion, decision)
    }
}

crate::callback::run_result_methods!(
    WatchdogRunResult,
    WatchdogDecision,
    "Return the completion classification and work count.",
    "Return how the committed successor should be retained or cleared."
);

// One event owns both its completion classification and reported work count.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RecordedOutcome {
    Completed(TimerCompletion),
    Unacknowledged,
}

/// Latest outcome and functional failure state for one timer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerOutcomeSnapshot {
    last_event: Option<RecordedOutcome>,
    last_success_at_ns: Option<u64>,
    last_failure_at_ns: Option<u64>,
    last_unacknowledged_at_ns: Option<u64>,
    consecutive_expected_failures: u64,
}

impl TimerOutcomeSnapshot {
    pub(crate) const EMPTY: Self = Self {
        last_event: None,
        last_success_at_ns: None,
        last_failure_at_ns: None,
        last_unacknowledged_at_ns: None,
        consecutive_expected_failures: 0,
    };

    pub(crate) const fn record_completion(
        &mut self,
        completion: TimerCompletion,
        completed_at_ns: u64,
    ) {
        self.last_event = Some(RecordedOutcome::Completed(completion));
        match completion.outcome {
            TimerCompletionOutcome::Success | TimerCompletionOutcome::NoWork => {
                self.last_success_at_ns = Some(completed_at_ns);
                self.consecutive_expected_failures = 0;
            }
            TimerCompletionOutcome::RetryableFailure => {
                self.last_failure_at_ns = Some(completed_at_ns);
                self.consecutive_expected_failures =
                    self.consecutive_expected_failures.saturating_add(1);
            }
            TimerCompletionOutcome::InvariantFailure => {
                self.last_failure_at_ns = Some(completed_at_ns);
                self.consecutive_expected_failures = 0;
            }
        }
    }

    pub(crate) const fn record_unacknowledged(&mut self, observed_at_ns: u64) {
        self.last_event = Some(RecordedOutcome::Unacknowledged);
        self.last_unacknowledged_at_ns = Some(observed_at_ns);
    }

    /// Return the latest terminal event.
    #[must_use]
    pub const fn last_outcome(self) -> Option<TimerLastOutcome> {
        match self.last_event {
            Some(RecordedOutcome::Completed(completion)) => {
                Some(TimerLastOutcome::Completed(completion.outcome()))
            }
            Some(RecordedOutcome::Unacknowledged) => Some(TimerLastOutcome::Unacknowledged),
            None => None,
        }
    }

    /// Return work reported by the latest completion.
    #[must_use]
    pub const fn last_work_count(self) -> Option<u64> {
        match self.last_event {
            Some(RecordedOutcome::Completed(completion)) => Some(completion.work_count()),
            Some(RecordedOutcome::Unacknowledged) | None => None,
        }
    }

    /// Return the latest successful or valid no-work completion time.
    #[must_use]
    pub const fn last_success_at_ns(self) -> Option<u64> {
        self.last_success_at_ns
    }

    /// Return the latest expected or invariant failure completion time.
    #[must_use]
    pub const fn last_failure_at_ns(self) -> Option<u64> {
        self.last_failure_at_ns
    }

    /// Return when an ordinary delivery or dispatched Watchdog attempt was
    /// most recently retired without completion.
    #[must_use]
    pub const fn last_unacknowledged_at_ns(self) -> Option<u64> {
        self.last_unacknowledged_at_ns
    }

    /// Return consecutive retryable failures since the latest reset outcome.
    #[must_use]
    pub const fn consecutive_expected_failures(self) -> u64 {
        self.consecutive_expected_failures
    }
}

/// Identity and start time of one runtime-local observation epoch.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimerEpoch {
    canister_version: u64,
    started_at_ns: u64,
}

impl TimerEpoch {
    pub(crate) const fn new(canister_version: u64, started_at_ns: u64) -> Self {
        Self {
            canister_version,
            started_at_ns,
        }
    }

    /// Return the IC canister version that owns this volatile epoch.
    #[must_use]
    pub const fn canister_version(self) -> u64 {
        self.canister_version
    }

    /// Return the IC timestamp at which the epoch began.
    #[must_use]
    pub const fn started_at_ns(self) -> u64 {
        self.started_at_ns
    }
}

#[cfg(test)]
mod tests;
