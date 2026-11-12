//! Pure callback-generation state for one ordinary timer.
//!
//! This module owns no task execution, platform timer handles, persistence, or
//! time source. Ordinary request selection, pending-command arbitration and
//! completion planning live here; the registry owns authorization and effects.

use crate::{
    callback::OrdinaryRunResult,
    schedule::{OrdinaryDirective, ResolvedSchedule, TimerCadence},
    snapshot::{
        InactiveReason, TimerCompletionOutcome, TimerControlFailure, TimerDirectiveSnapshot,
    },
};

/// A command selected during ordinary work after registry authorization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrdinaryPending {
    Cancel,
    ReconcileInactive,
    Reconcile(ResolvedSchedule),
    Unregister,
    Schedule(ResolvedSchedule),
}

/// Current registration state for one timer identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerRegistration {
    /// No callback is scheduled or running.
    Inactive {
        /// Why the retained declaration has no scheduled or running work.
        reason: InactiveReason,
    },
    /// One generation is scheduled for an absolute nanosecond deadline.
    Scheduled {
        /// Absolute IC timestamp in nanoseconds.
        deadline_ns: u64,
    },
    /// One generation currently owns logical execution.
    Running {
        /// The selected command for this authorized running attempt.
        pending: Option<OrdinaryPending>,
    },
}

impl Default for TimerRegistration {
    fn default() -> Self {
        Self::Inactive {
            reason: InactiveReason::NeverScheduled,
        }
    }
}

/// Whether one arm fills an empty wake-up slot or replaces its current handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WakeupArm {
    Initial,
    Replacement,
}

impl WakeupArm {
    pub(crate) const fn replaces_existing(self) -> bool {
        matches!(self, Self::Replacement)
    }
}

/// Pure state machine for one logical timer identity.
#[derive(Debug, Default)]
pub struct TimerControl {
    // Active callbacks use this generation; inactive control retains allocation
    // history. State changes never keep an active callback from an older generation.
    generation: u64,
    pub(crate) registration: TimerRegistration,
}

#[derive(Clone, Copy)]
pub enum OrdinaryRequest {
    Ensure,
    Reconcile,
}

/// An authorized request to leave ordinary work inactive.
#[derive(Clone, Copy)]
pub enum OrdinaryStop {
    Cancel,
    ReconcileInactive,
}

impl OrdinaryRequest {
    const fn replaces(self, current_deadline_ns: u64, requested_deadline_ns: u64) -> bool {
        match self {
            Self::Ensure => requested_deadline_ns < current_deadline_ns,
            Self::Reconcile => requested_deadline_ns != current_deadline_ns,
        }
    }

    const fn select_pending(
        self,
        current: Option<OrdinaryPending>,
        requested: ResolvedSchedule,
    ) -> OrdinaryPending {
        match (current, self) {
            (Some(OrdinaryPending::Unregister), _) => OrdinaryPending::Unregister,
            (_, Self::Reconcile) => OrdinaryPending::Reconcile(requested),
            (
                Some(OrdinaryPending::Reconcile(current) | OrdinaryPending::Schedule(current)),
                Self::Ensure,
            ) if current.deadline_ns <= requested.deadline_ns => OrdinaryPending::Schedule(current),
            (_, Self::Ensure) => OrdinaryPending::Schedule(requested),
        }
    }
}

impl TimerControl {
    /// Return the latest allocated callback generation.
    #[must_use]
    pub(crate) const fn generation(&self) -> u64 {
        self.generation
    }

    /// Return the command associated with the current running work, if any.
    #[must_use]
    pub(crate) const fn pending(&self) -> Option<OrdinaryPending> {
        match self.registration {
            TimerRegistration::Running { pending } => pending,
            TimerRegistration::Inactive { .. } | TimerRegistration::Scheduled { .. } => None,
        }
    }

    /// Defer removal until the running callback completes or is abandoned.
    pub(crate) const fn defer_unregister(&mut self) -> bool {
        if let TimerRegistration::Running { pending } = &mut self.registration {
            *pending = Some(OrdinaryPending::Unregister);
            return true;
        }
        false
    }

    /// Stop ordinary control with the registry-selected inactive reason.
    ///
    /// Returns whether a scheduled wake-up must be cleared. A running callback
    /// has already consumed its provider wake-up.
    pub(crate) const fn terminate(&mut self, reason: InactiveReason) -> bool {
        let clear_wakeup = matches!(self.registration, TimerRegistration::Scheduled { .. });
        self.registration = TimerRegistration::Inactive { reason };
        clear_wakeup
    }

    /// Stop scheduled work or defer the stop until running work completes.
    /// Unregister retains precedence; inactive declarations keep their reason.
    pub(crate) const fn stop(&mut self, request: OrdinaryStop) -> bool {
        let (reason, command) = match request {
            OrdinaryStop::Cancel => (InactiveReason::Cancelled, OrdinaryPending::Cancel),
            OrdinaryStop::ReconcileInactive => {
                (InactiveReason::Stopped, OrdinaryPending::ReconcileInactive)
            }
        };
        match &mut self.registration {
            TimerRegistration::Inactive { .. } => false,
            TimerRegistration::Scheduled { .. } => self.terminate(reason),
            TimerRegistration::Running { pending } => {
                if !matches!(*pending, Some(OrdinaryPending::Unregister)) {
                    *pending = Some(command);
                }
                false
            }
        }
    }

    /// Select one ordinary request, including its pending command during work.
    /// Return an arm only after installing a newly allocated generation.
    pub(crate) fn request(
        &mut self,
        requested: ResolvedSchedule,
        request: OrdinaryRequest,
    ) -> Result<Option<WakeupArm>, TimerControlFailure> {
        let kind = match &mut self.registration {
            TimerRegistration::Inactive { .. } => WakeupArm::Initial,
            TimerRegistration::Scheduled {
                deadline_ns: current_deadline_ns,
            } if request.replaces(*current_deadline_ns, requested.deadline_ns) => {
                WakeupArm::Replacement
            }
            TimerRegistration::Scheduled { .. } => return Ok(None),
            TimerRegistration::Running { pending } => {
                *pending = Some(request.select_pending(*pending, requested));
                return Ok(None);
            }
        };

        self.arm_deadline(requested.deadline_ns)?;
        Ok(Some(kind))
    }

    /// Begin the scheduled generation, rejecting stale callbacks.
    pub(crate) const fn begin(&mut self, generation: u64) -> bool {
        match self.registration {
            TimerRegistration::Scheduled { .. } if self.generation == generation => {
                self.registration = TimerRegistration::Running { pending: None };
                true
            }
            TimerRegistration::Inactive { .. }
            | TimerRegistration::Scheduled { .. }
            | TimerRegistration::Running { .. } => false,
        }
    }

    /// Select the winning command and apply its completion state after the
    /// registry checks running authority. Invariant failures override commands.
    /// Selection or generation failures leave running state unchanged.
    pub(crate) fn complete(
        &mut self,
        result: OrdinaryRunResult,
        now_ns: u64,
        cadence: Option<TimerCadence>,
    ) -> Result<OrdinaryCompletion, TimerControlFailure> {
        let completion =
            if result.completion().outcome() == TimerCompletionOutcome::InvariantFailure {
                OrdinaryCompletion::Stop(InactiveReason::InvariantFailure)
            } else {
                prepare_ordinary_completion(self.pending(), result.directive(), now_ns, cadence)?
            };
        match completion {
            OrdinaryCompletion::Schedule { requested, .. } => {
                self.arm_deadline(requested.deadline_ns)?;
            }
            OrdinaryCompletion::Stop(reason) => {
                self.terminate(reason);
            }
        }
        Ok(completion)
    }

    /// Install a selected deadline after request eligibility or exact running
    /// work authorization. Check allocation before replacing the current state.
    fn arm_deadline(&mut self, deadline_ns: u64) -> Result<(), TimerControlFailure> {
        let generation = self
            .generation
            .checked_add(1)
            .ok_or(TimerControlFailure::GenerationExhausted)?;
        self.generation = generation;
        self.registration = TimerRegistration::Scheduled { deadline_ns };
        Ok(())
    }
}

// Keep the selected action and its callback observation together. An ensure can
// select a pending schedule while retaining a callback's Stop observation.
pub enum OrdinaryCompletion {
    Schedule {
        requested: ResolvedSchedule,
        directive: TimerDirectiveSnapshot,
    },
    Stop(InactiveReason),
}

impl OrdinaryCompletion {
    pub(crate) const fn directive(&self) -> TimerDirectiveSnapshot {
        match self {
            Self::Schedule { directive, .. } => *directive,
            Self::Stop(_) => TimerDirectiveSnapshot::Stop,
        }
    }
}

// Select authoritative commands before validating a callback proposal they discard.
fn prepare_ordinary_completion(
    pending: Option<OrdinaryPending>,
    directive: OrdinaryDirective,
    now_ns: u64,
    cadence: Option<TimerCadence>,
) -> Result<OrdinaryCompletion, TimerControlFailure> {
    match pending {
        Some(OrdinaryPending::ReconcileInactive) => {
            Ok(OrdinaryCompletion::Stop(InactiveReason::Stopped))
        }
        Some(OrdinaryPending::Cancel | OrdinaryPending::Unregister) => {
            Ok(OrdinaryCompletion::Stop(InactiveReason::Cancelled))
        }
        Some(OrdinaryPending::Reconcile(requested)) => Ok(OrdinaryCompletion::Schedule {
            requested,
            directive: TimerDirectiveSnapshot::ScheduleAt {
                deadline_ns: requested.deadline_ns,
            },
        }),
        Some(OrdinaryPending::Schedule(_)) | None => {
            let (callback, directive) = directive.resolve(now_ns, cadence)?;
            let selected = if let Some(OrdinaryPending::Schedule(pending)) = pending {
                match callback {
                    Some(callback) if callback.deadline_ns < pending.deadline_ns => Some(callback),
                    Some(_) | None => Some(pending),
                }
            } else {
                callback
            };
            Ok(selected.map_or(
                OrdinaryCompletion::Stop(InactiveReason::Stopped),
                |requested| OrdinaryCompletion::Schedule {
                    requested,
                    directive,
                },
            ))
        }
    }
}

#[cfg(test)]
mod tests;
