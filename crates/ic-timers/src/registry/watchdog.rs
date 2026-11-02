//! Private watchdog control and pure scheduling decisions.
//!
//! The parent registry retains claim admission, entry mutation and provider-handle
//! ownership. This module owns no provider calls, callbacks or runtime state.

use super::{RegistrationClaim, RegistryEffect, RegistryTransition, WakeupToken, clear_wakeup_if};
use crate::{
    control::WakeupArm,
    schedule::{ResolvedSchedule, ScheduleError, TimerCadence},
    snapshot::{
        InactiveReason, TimerCompletionOutcome, TimerControlFailure, TimerSchedulingMode,
        WatchdogAttemptStatus, WatchdogDecision, WatchdogRunResult,
    },
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WatchdogPending {
    Cancel,
    Ensure,
    EnsureImmediately,
    Reconcile(ResolvedSchedule),
    Unregister,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WatchdogScheduleRequest {
    Cadence,
    Immediate,
    Reconcile(ResolvedSchedule),
}

impl WatchdogScheduleRequest {
    // Resolve one requested schedule before mutation. Cadence overflow is checked
    // only for a fresh arm; coalesced and awaiting requests keep their successor.
    pub(super) fn resolve(
        self,
        state: WatchdogState,
        cadence: TimerCadence,
        now_ns: u64,
    ) -> Result<ResolvedSchedule, ScheduleError> {
        Ok(match self {
            Self::Cadence => ResolvedSchedule {
                deadline_ns: match state {
                    WatchdogState::Inactive { .. } => cadence.deadline_after(now_ns)?,
                    WatchdogState::Scheduled { deadline_ns } => deadline_ns,
                    WatchdogState::AwaitingWork {
                        successor_deadline_ns,
                        ..
                    } => successor_deadline_ns,
                },
                mode: TimerSchedulingMode::Watchdog,
                requested_delay_ns: Some(cadence.as_nanos()),
            },
            Self::Immediate => ResolvedSchedule {
                deadline_ns: match state {
                    WatchdogState::Scheduled { deadline_ns } => deadline_ns.min(now_ns),
                    _ => now_ns,
                },
                mode: TimerSchedulingMode::Continuation,
                requested_delay_ns: Some(0),
            },
            Self::Reconcile(requested) => requested,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WatchdogState {
    Inactive {
        reason: InactiveReason,
    },
    Scheduled {
        deadline_ns: u64,
    },
    AwaitingWork {
        successor_deadline_ns: u64,
        attempt_status: WatchdogAttemptStatus,
        pending: Option<WatchdogPending>,
    },
}

#[derive(Debug)]
pub(super) struct WatchdogControl {
    // One allocation history survives inactive state. A dispatch stamps its
    // successor and work with one generation, distinguished by callback role.
    // Requests while awaiting work cannot replace that successor until completion.
    pub(super) generation: u64,
    pub(super) state: WatchdogState,
    // Only confirmed destruction of the current bound scheduler sets this.
    // A missing handle alone can also be a synchronous effect's detached state.
    pub(super) scheduler_abandoned: bool,
}

impl Default for WatchdogControl {
    fn default() -> Self {
        Self {
            generation: 0,
            state: WatchdogState::Inactive {
                reason: InactiveReason::NeverScheduled,
            },
            scheduler_abandoned: false,
        }
    }
}

impl WatchdogControl {
    pub(super) fn arm_scheduler(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        deadline_ns: u64,
        arm: WakeupArm,
    ) -> RegistryTransition {
        let Some(generation) = self.generation.checked_add(1) else {
            let cleanup = clear_wakeup_if(arm.replaces_existing());
            return self.terminate(cleanup, TimerControlFailure::GenerationExhausted);
        };
        self.generation = generation;
        self.state = WatchdogState::Scheduled { deadline_ns };
        self.scheduler_abandoned = false;
        RegistryTransition::normal(RegistryEffect::ArmWakeup {
            token: WakeupToken::scheduler(claim, generation),
            delay_ns: deadline_ns.saturating_sub(now_ns),
            arm,
        })
    }

    pub(super) const fn terminate(
        &mut self,
        effect: RegistryEffect,
        failure: TimerControlFailure,
    ) -> RegistryTransition {
        self.state = WatchdogState::Inactive {
            reason: InactiveReason::ControlFailure(failure),
        };
        RegistryTransition::terminal(effect, failure)
    }
}

// Resolve the winner and its observation metadata together before mutating
// control. A pending exact schedule keeps its already validated relative origin.
pub(super) enum WatchdogCompletion {
    Stop(InactiveReason),
    Schedule {
        requested: ResolvedSchedule,
        retain_successor: bool,
    },
}

pub(super) fn prepare_watchdog_completion(
    pending: Option<WatchdogPending>,
    result: WatchdogRunResult,
    now_ns: u64,
    successor_deadline_ns: u64,
    cadence: TimerCadence,
) -> WatchdogCompletion {
    if result.completion().outcome() == TimerCompletionOutcome::InvariantFailure {
        return WatchdogCompletion::Stop(InactiveReason::InvariantFailure);
    }
    let decision = match pending {
        Some(WatchdogPending::Cancel) => {
            return WatchdogCompletion::Stop(InactiveReason::Cancelled);
        }
        Some(WatchdogPending::Unregister) => {
            return WatchdogCompletion::Stop(InactiveReason::Stopped);
        }
        Some(WatchdogPending::Reconcile(requested)) => {
            return WatchdogCompletion::Schedule {
                retain_successor: requested.deadline_ns == successor_deadline_ns,
                requested,
            };
        }
        Some(WatchdogPending::Ensure) => WatchdogDecision::Continue,
        Some(WatchdogPending::EnsureImmediately) => WatchdogDecision::ContinueImmediately,
        None => result.decision(),
    };
    let (requested, retain_successor) = match decision {
        WatchdogDecision::Stop => return WatchdogCompletion::Stop(InactiveReason::Stopped),
        WatchdogDecision::Continue => (
            ResolvedSchedule {
                deadline_ns: successor_deadline_ns,
                requested_delay_ns: Some(cadence.as_nanos()),
                mode: TimerSchedulingMode::Watchdog,
            },
            true,
        ),
        WatchdogDecision::ContinueImmediately => (
            ResolvedSchedule {
                deadline_ns: now_ns,
                requested_delay_ns: Some(0),
                mode: TimerSchedulingMode::Continuation,
            },
            successor_deadline_ns <= now_ns,
        ),
        WatchdogDecision::ScheduleAt(deadline_ns) => (
            ResolvedSchedule {
                deadline_ns,
                requested_delay_ns: None,
                mode: TimerSchedulingMode::Deadline,
            },
            successor_deadline_ns == deadline_ns,
        ),
    };
    WatchdogCompletion::Schedule {
        requested,
        retain_successor,
    }
}

pub(super) const fn select_pending_watchdog(
    pending: Option<WatchdogPending>,
    request: WatchdogScheduleRequest,
    now_ns: u64,
) -> WatchdogPending {
    match (pending, request) {
        (Some(WatchdogPending::Unregister), _) => WatchdogPending::Unregister,
        (_, WatchdogScheduleRequest::Reconcile(requested)) => WatchdogPending::Reconcile(requested),
        (Some(WatchdogPending::Reconcile(requested)), WatchdogScheduleRequest::Cadence) => {
            WatchdogPending::Reconcile(requested)
        }
        (Some(WatchdogPending::Reconcile(requested)), WatchdogScheduleRequest::Immediate)
            if requested.deadline_ns <= now_ns =>
        {
            WatchdogPending::Reconcile(requested)
        }
        (Some(WatchdogPending::EnsureImmediately), _) | (_, WatchdogScheduleRequest::Immediate) => {
            WatchdogPending::EnsureImmediately
        }
        (
            Some(WatchdogPending::Cancel | WatchdogPending::Ensure) | None,
            WatchdogScheduleRequest::Cadence,
        ) => WatchdogPending::Ensure,
    }
}
