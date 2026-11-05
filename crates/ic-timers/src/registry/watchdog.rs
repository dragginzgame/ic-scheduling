//! Private watchdog control and pure scheduling decisions.
//!
//! The parent registry retains claim admission, entry mutation and provider-handle
//! ownership. This module owns no provider calls, callbacks or runtime state.

use super::{
    CallbacksToClear, ProviderBinding, RegistrationClaim, RegistryEffect, RegistryTransition,
    WakeupToken, clear_wakeup_if, effect::WatchdogDispatch,
};
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
    fn resolve(
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
    generation: u64,
    state: WatchdogState,
    // Only confirmed destruction of the current bound scheduler sets this.
    // A missing handle alone can also be a synchronous effect's detached state.
    scheduler_abandoned: bool,
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
    pub(super) const fn generation(&self) -> u64 {
        self.generation
    }

    // Read-only state projection for registry lifetime and snapshot observations.
    pub(super) const fn state(&self) -> WatchdogState {
        self.state
    }

    pub(super) const fn can_bind_scheduler(
        &self,
        generation: u64,
        binding: ProviderBinding,
    ) -> bool {
        self.generation == generation
            && !matches!(self.state, WatchdogState::Inactive { .. })
            && (matches!(binding, ProviderBinding::Stage)
                || matches!(self.state, WatchdogState::Scheduled { .. })
                || self.scheduler_abandoned)
    }

    pub(super) const fn can_bind_work(&self, generation: u64) -> bool {
        self.generation == generation
            && matches!(
                self.state,
                WatchdogState::AwaitingWork {
                    attempt_status: WatchdogAttemptStatus::Dispatched,
                    ..
                }
            )
    }

    pub(super) const fn accepts_scheduler(&self, generation: u64) -> bool {
        self.generation == generation
            && matches!(
                self.state,
                WatchdogState::Scheduled { .. }
                    | WatchdogState::AwaitingWork {
                        attempt_status: WatchdogAttemptStatus::Dispatched,
                        ..
                    }
            )
    }

    pub(super) const fn begin_work(&mut self, generation: u64) -> bool {
        if self.can_bind_work(generation)
            && let WatchdogState::AwaitingWork { attempt_status, .. } = &mut self.state
        {
            *attempt_status = WatchdogAttemptStatus::Running;
            return true;
        }
        false
    }

    pub(super) const fn defer_unregister(&mut self) -> bool {
        if let WatchdogState::AwaitingWork {
            attempt_status: WatchdogAttemptStatus::Running,
            pending,
            ..
        } = &mut self.state
        {
            *pending = Some(WatchdogPending::Unregister);
            return true;
        }
        false
    }

    pub(super) const fn abandon_scheduler(&mut self) {
        self.scheduler_abandoned = true;
    }

    pub(super) const fn bind_scheduler(&mut self) {
        self.scheduler_abandoned = false;
    }

    pub(super) const fn fail(&mut self, failure: TimerControlFailure) {
        self.state = WatchdogState::Inactive {
            reason: InactiveReason::ControlFailure(failure),
        };
    }

    pub(super) fn request(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        cadence: TimerCadence,
        request: WatchdogScheduleRequest,
    ) -> Result<(ResolvedSchedule, RegistryTransition), ScheduleError> {
        let requested = request.resolve(self.state, cadence, now_ns)?;
        let transition = match &mut self.state {
            WatchdogState::Inactive { .. } => {
                self.arm_scheduler(claim, now_ns, requested.deadline_ns, WakeupArm::Initial)
            }
            WatchdogState::Scheduled { deadline_ns }
                if self.scheduler_abandoned || *deadline_ns != requested.deadline_ns =>
            {
                let arm = if self.scheduler_abandoned {
                    WakeupArm::Initial
                } else {
                    WakeupArm::Replacement
                };
                self.arm_scheduler(claim, now_ns, requested.deadline_ns, arm)
            }
            WatchdogState::Scheduled { .. } => RegistryTransition::normal(RegistryEffect::None),
            WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Dispatched,
                pending,
                ..
            } => {
                if let WatchdogScheduleRequest::Reconcile(requested) = request {
                    *pending = Some(WatchdogPending::Reconcile(requested));
                }
                RegistryTransition::normal(RegistryEffect::None)
            }
            WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Running,
                pending,
                ..
            } => {
                *pending = Some(select_pending_watchdog(*pending, request, now_ns));
                RegistryTransition::normal(RegistryEffect::None)
            }
        };
        let transition = if self.scheduler_abandoned
            && let WatchdogState::AwaitingWork {
                successor_deadline_ns,
                ..
            } = self.state
        {
            // Restore only the committed successor; the current work may be queued.
            RegistryTransition::normal(RegistryEffect::ArmWakeup {
                token: WakeupToken::scheduler(claim, self.generation),
                delay_ns: successor_deadline_ns.saturating_sub(now_ns),
                arm: WakeupArm::Initial,
            })
        } else {
            transition
        };
        Ok((requested, transition))
    }

    pub(super) const fn cancel(&mut self) -> Option<CallbacksToClear> {
        let handles = match &mut self.state {
            WatchdogState::Inactive { .. } => None,
            WatchdogState::Scheduled { .. } => Some(CallbacksToClear::Wakeup),
            WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Dispatched,
                ..
            } => Some(CallbacksToClear::WakeupAndWork),
            WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Running,
                pending,
                ..
            } => {
                if !matches!(*pending, Some(WatchdogPending::Unregister)) {
                    *pending = Some(WatchdogPending::Cancel);
                }
                None
            }
        };
        if handles.is_some() {
            self.state = WatchdogState::Inactive {
                reason: InactiveReason::Cancelled,
            };
        }
        handles
    }

    pub(super) fn dispatch(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        cadence: TimerCadence,
    ) -> RegistryTransition {
        let next_dispatch = self
            .generation
            .checked_add(1)
            .ok_or(TimerControlFailure::GenerationExhausted)
            .and_then(|generation| {
                cadence
                    .deadline_after(now_ns)
                    .map(|deadline_ns| (generation, deadline_ns))
                    .map_err(|_| TimerControlFailure::DeadlineOverflow)
            });
        let (generation, successor_deadline_ns) = match next_dispatch {
            Ok(next) => next,
            Err(failure) => {
                return self.terminate(
                    RegistryEffect::ClearCallbacks {
                        handles: CallbacksToClear::Work,
                    },
                    failure,
                );
            }
        };
        self.generation = generation;
        self.scheduler_abandoned = false;
        self.state = WatchdogState::AwaitingWork {
            successor_deadline_ns,
            attempt_status: WatchdogAttemptStatus::Dispatched,
            pending: None,
        };
        RegistryTransition::normal(RegistryEffect::DispatchWatchdog {
            dispatch: WatchdogDispatch::new(claim, generation),
            successor_delay_ns: cadence.as_nanos(),
        })
    }

    pub(super) fn complete(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        successor_deadline_ns: u64,
        completion: &WatchdogCompletion,
    ) -> RegistryTransition {
        match completion {
            WatchdogCompletion::Schedule {
                requested,
                retain_successor,
            } => {
                if *retain_successor {
                    self.state = WatchdogState::Scheduled {
                        deadline_ns: successor_deadline_ns,
                    };
                    RegistryTransition::normal(RegistryEffect::None)
                } else {
                    self.arm_scheduler(claim, now_ns, requested.deadline_ns, WakeupArm::Replacement)
                }
            }
            WatchdogCompletion::Stop(reason) => {
                self.state = WatchdogState::Inactive { reason: *reason };
                RegistryTransition::normal(RegistryEffect::ClearCallbacks {
                    handles: CallbacksToClear::Wakeup,
                })
            }
        }
    }

    fn arm_scheduler(
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

    const fn terminate(
        &mut self,
        effect: RegistryEffect,
        failure: TimerControlFailure,
    ) -> RegistryTransition {
        self.fail(failure);
        RegistryTransition::terminal(effect, failure)
    }
}

#[cfg(test)]
mod tests;

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

const fn select_pending_watchdog(
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
