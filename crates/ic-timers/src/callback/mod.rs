//! Policy-specific ordinary callback proposals and private dispatch erasure.
//!
//! Public decisions contain only operations valid for their callback policy.
//! Erasure feeds the existing registry arbitration; it grants no control authority.

use crate::{
    schedule::OrdinaryDirective,
    snapshot::{TimerCompletion, TimerCompletionOutcome},
};
use std::time::Duration;

// Policy-specific result types share storage access and terminal normalization.
macro_rules! run_result_methods {
    ($result:ident, $decision:ident, $completion_doc:literal, $decision_doc:literal) => {
        impl $result {
            /// Construct a result, forcing invariant failures to stop.
            #[must_use]
            pub const fn new(completion: TimerCompletion, decision: $decision) -> Self {
                Self {
                    decision: if matches!(
                        completion.outcome(),
                        TimerCompletionOutcome::InvariantFailure
                    ) {
                        $decision::Stop
                    } else {
                        decision
                    },
                    completion,
                }
            }

            #[doc = $completion_doc]
            #[must_use]
            pub const fn completion(self) -> TimerCompletion {
                self.completion
            }

            #[doc = $decision_doc]
            #[must_use]
            pub const fn decision(self) -> $decision {
                self.decision
            }
        }
    };
}
pub(super) use run_result_methods;

/// Scheduling proposal after a normally returned `Once` invocation.
///
/// Once permits explicit rescheduling, but has no configured recurrence:
///
/// ```compile_fail,E0599
/// use ic_timers::OnceDecision;
/// let decision = OnceDecision::RecurAfterCompletion;
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnceDecision {
    /// Stop unless a pending authoritative command selects another schedule.
    Stop,
    /// Continue in a later timer message with zero delay.
    ContinueImmediately,
    /// Retry after a checked relative delay.
    RetryAfter(Duration),
    /// Schedule at an absolute IC timestamp in nanoseconds.
    ScheduleAt(u64),
}

/// Scheduling proposal after a normally returned after-completion invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AfterCompletionDecision {
    /// Stop unless a pending authoritative command selects another schedule.
    Stop,
    /// Continue in a later timer message with zero delay.
    ContinueImmediately,
    /// Retry after a checked relative delay.
    RetryAfter(Duration),
    /// Schedule at an absolute IC timestamp in nanoseconds.
    ScheduleAt(u64),
    /// Recur using the registration's configured cadence after completion.
    RecurAfterCompletion,
}

/// A classified `Once` completion with one policy-valid scheduling proposal.
///
/// Winning exact reconciliation and terminal pending commands discard the
/// proposal before validation. An explicit invariant failure always stops.
/// Other completion classifications preserve the supplied decision: a
/// retryable failure does not automatically retry, and success does not
/// automatically continue.
///
/// Construction does not validate relative delays. If the proposal survives
/// command arbitration, the runtime checks duration encoding and deadline
/// arithmetic. An invalid selected delay stops the declaration with a
/// [`crate::TimerControlFailure`]; a retained declaration exposes that failure
/// in its snapshot.
///
/// Registration and lifecycle reconstruction use the same typed results:
///
/// ```no_run
/// use ic_timers::{register_once, register_after_completion, reconcile_once,
///     reconcile_after_completion, OnceDecision, OnceRunResult,
///     AfterCompletionDecision, AfterCompletionRunResult, TimerIdentity,
///     TimerCadence, TimerCompletion, TimerReconcileState, TimerStart, initialize_runtime};
/// # fn main() -> Result<(), ic_timers::TimerError> {
/// initialize_runtime()?;
/// let once_identity = TimerIdentity::try_new("app", "jobs", "once").unwrap();
/// let recurring_identity = TimerIdentity::try_new("app", "jobs", "recurring").unwrap();
/// let direct_once_identity = TimerIdentity::try_new("app", "jobs", "direct-once").unwrap();
/// let direct_recurring_identity = TimerIdentity::try_new("app", "jobs", "direct-recurring").unwrap();
/// let cadence = TimerCadence::from_nanos(5)?;
/// let _once = register_once(direct_once_identity, TimerStart::Retained { initial: None }, |_| async {
///     OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
/// })?;
/// let _recurring = register_after_completion(direct_recurring_identity, cadence,
///     TimerStart::Retained { initial: None }, |_| async {
///         AfterCompletionRunResult::new(TimerCompletion::no_work(),
///             AfterCompletionDecision::RecurAfterCompletion)
///     })?;
/// let mut once = None;
/// reconcile_once(&mut once, &once_identity, None, |_| async {
///     OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
/// })?;
/// let mut recurring = None;
/// reconcile_after_completion(&mut recurring, &recurring_identity, cadence,
///     TimerReconcileState::Inactive, |_| async {
///         AfterCompletionRunResult::new(TimerCompletion::no_work(), AfterCompletionDecision::Stop)
///     })?;
/// # Ok(())
/// # }
/// ```
///
/// The result accepts only a [`OnceDecision`]:
///
/// ```compile_fail,E0308
/// use ic_timers::{AfterCompletionDecision, OnceRunResult, TimerCompletion};
/// let result = OnceRunResult::new(
///     TimerCompletion::no_work(), AfterCompletionDecision::Stop,
/// );
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OnceRunResult {
    completion: TimerCompletion,
    decision: OnceDecision,
}

run_result_methods!(
    OnceRunResult,
    OnceDecision,
    "Return the completion classification and reported work count.",
    "Return the post-run scheduling proposal."
);

/// A classified after-completion result with one policy-valid proposal.
///
/// Recurrence uses only the registration's configured cadence. Winning exact
/// reconciliation and terminal pending commands discard the proposal before
/// validation. An explicit invariant failure always stops.
/// Success, no work and retryable failure preserve the supplied decision;
/// recurrence requires [`AfterCompletionDecision::RecurAfterCompletion`].
/// A retryable failure with Stop remains Stop.
///
/// As with [`OnceRunResult`], construction does not validate relative delays;
/// the runtime checks a selected proposal after command arbitration. Configured
/// recurrence also checks the successor deadline against the completion time.
///
/// ```compile_fail,E0308
/// use ic_timers::{AfterCompletionRunResult, OnceDecision, TimerCompletion};
/// let result = AfterCompletionRunResult::new(
///     TimerCompletion::no_work(), OnceDecision::Stop,
/// );
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AfterCompletionRunResult {
    completion: TimerCompletion,
    decision: AfterCompletionDecision,
}

run_result_methods!(
    AfterCompletionRunResult,
    AfterCompletionDecision,
    "Return the completion classification and reported work count.",
    "Return the post-run scheduling proposal."
);

/// Erased result consumed only by the canonical ordinary completion owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OrdinaryRunResult {
    completion: TimerCompletion,
    directive: OrdinaryDirective,
}

impl OrdinaryRunResult {
    // Preserve raw internal proposals so the registry owns terminal arbitration.
    // Public policy-specific results normalize invariant failures before erasure.
    pub(super) const fn new(completion: TimerCompletion, directive: OrdinaryDirective) -> Self {
        Self {
            completion,
            directive,
        }
    }

    pub(super) const fn completion(self) -> TimerCompletion {
        self.completion
    }

    pub(super) const fn directive(self) -> OrdinaryDirective {
        self.directive
    }
}

impl From<OnceDecision> for OrdinaryDirective {
    fn from(value: OnceDecision) -> Self {
        match value {
            OnceDecision::Stop => Self::Stop,
            OnceDecision::ContinueImmediately => Self::ContinueImmediately,
            OnceDecision::RetryAfter(delay) => Self::RetryAfter(delay),
            OnceDecision::ScheduleAt(deadline) => Self::ScheduleAt(deadline),
        }
    }
}

impl From<AfterCompletionDecision> for OrdinaryDirective {
    fn from(value: AfterCompletionDecision) -> Self {
        match value {
            AfterCompletionDecision::Stop => Self::Stop,
            AfterCompletionDecision::ContinueImmediately => Self::ContinueImmediately,
            AfterCompletionDecision::RetryAfter(delay) => Self::RetryAfter(delay),
            AfterCompletionDecision::ScheduleAt(deadline) => Self::ScheduleAt(deadline),
            AfterCompletionDecision::RecurAfterCompletion => Self::RecurAfterCompletion,
        }
    }
}

impl From<OnceRunResult> for OrdinaryRunResult {
    fn from(value: OnceRunResult) -> Self {
        Self {
            completion: value.completion,
            directive: value.decision.into(),
        }
    }
}

impl From<AfterCompletionRunResult> for OrdinaryRunResult {
    fn from(value: AfterCompletionRunResult) -> Self {
        Self {
            completion: value.completion,
            directive: value.decision.into(),
        }
    }
}

#[cfg(test)]
mod tests;
