//! Pure callback-generation state for one ordinary timer.
//!
//! This module owns no task execution, platform timer handles, persistence, or
//! time source. The canonical registry owns running-work authorization and
//! pending-command arbitration.

use crate::{
    schedule::ResolvedSchedule,
    snapshot::{InactiveReason, TimerControlFailure},
};

/// A command arbitrated by the registry during ordinary work.
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
        /// Registry-owned command arbitration for this running attempt.
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
enum DeadlineSelection {
    Earliest,
    Exact,
}

impl DeadlineSelection {
    const fn replaces(self, current_deadline_ns: u64, requested_deadline_ns: u64) -> bool {
        match self {
            Self::Earliest => requested_deadline_ns < current_deadline_ns,
            Self::Exact => requested_deadline_ns != current_deadline_ns,
        }
    }
}

impl TimerControl {
    /// Return the latest allocated callback generation.
    #[must_use]
    pub(crate) const fn generation(&self) -> u64 {
        self.generation
    }

    #[cfg(test)]
    pub(crate) const fn seed_generation_for_test(&mut self, generation: u64) {
        assert!(matches!(
            self.registration,
            TimerRegistration::Inactive { .. }
        ));
        self.generation = generation;
    }

    /// Return the command associated with the current running work, if any.
    #[must_use]
    pub(crate) const fn pending(&self) -> Option<OrdinaryPending> {
        match self.registration {
            TimerRegistration::Running { pending } => pending,
            TimerRegistration::Inactive { .. } | TimerRegistration::Scheduled { .. } => None,
        }
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

    /// Schedule a deadline, retaining an already scheduled earlier deadline.
    /// Return the arm kind when scheduled state changes. Success with an arm
    /// installs the supplied deadline and a newly allocated generation.
    pub(crate) fn schedule(
        &mut self,
        deadline_ns: u64,
    ) -> Result<Option<WakeupArm>, TimerControlFailure> {
        self.request_deadline(deadline_ns, DeadlineSelection::Earliest)
    }

    /// Reconcile this timer to one authoritative deadline.
    pub(crate) fn reconcile(
        &mut self,
        deadline_ns: u64,
    ) -> Result<Option<WakeupArm>, TimerControlFailure> {
        self.request_deadline(deadline_ns, DeadlineSelection::Exact)
    }

    fn request_deadline(
        &mut self,
        deadline_ns: u64,
        selection: DeadlineSelection,
    ) -> Result<Option<WakeupArm>, TimerControlFailure> {
        let kind = match self.registration {
            TimerRegistration::Inactive { .. } => WakeupArm::Initial,
            TimerRegistration::Scheduled {
                deadline_ns: current_deadline_ns,
            } if selection.replaces(current_deadline_ns, deadline_ns) => WakeupArm::Replacement,
            TimerRegistration::Scheduled { .. } | TimerRegistration::Running { .. } => {
                return Ok(None);
            }
        };

        self.arm_deadline(deadline_ns)?;
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

    /// Install a selected deadline after request eligibility or exact running
    /// work authorization. Check allocation before replacing the current state.
    pub(crate) fn arm_deadline(&mut self, deadline_ns: u64) -> Result<(), TimerControlFailure> {
        let generation = self
            .generation
            .checked_add(1)
            .ok_or(TimerControlFailure::GenerationExhausted)?;
        self.generation = generation;
        self.registration = TimerRegistration::Scheduled { deadline_ns };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arm(control: &mut TimerControl, deadline_ns: u64) -> u64 {
        assert_eq!(control.schedule(deadline_ns), Ok(Some(WakeupArm::Initial)));
        control.generation()
    }

    #[test]
    fn duplicate_and_later_schedules_keep_one_earliest_handle() {
        let mut control = TimerControl::default();
        assert_eq!(arm(&mut control, 100), 1);
        assert_eq!(control.schedule(100), Ok(None));
        assert_eq!(control.schedule(200), Ok(None));
        assert_eq!(control.generation(), 1);
        assert_eq!(
            control.registration,
            TimerRegistration::Scheduled { deadline_ns: 100 }
        );
    }

    #[test]
    fn earlier_schedule_replaces_and_invalidates_old_generation() {
        let mut control = TimerControl::default();
        let old_generation = arm(&mut control, 100);
        assert_eq!(control.schedule(50), Ok(Some(WakeupArm::Replacement)));
        assert_eq!(control.generation(), 2);
        assert_eq!(
            control.registration,
            TimerRegistration::Scheduled { deadline_ns: 50 }
        );
        assert!(!control.begin(old_generation));
        assert!(control.begin(2));
    }

    #[test]
    fn authoritative_reconciliation_can_move_scheduled_deadline_later() {
        let mut control = TimerControl::default();
        let old_generation = arm(&mut control, 100);
        assert_eq!(control.reconcile(200), Ok(Some(WakeupArm::Replacement)));
        assert_eq!(control.generation(), 2);
        assert_eq!(
            control.registration,
            TimerRegistration::Scheduled { deadline_ns: 200 }
        );
        assert!(!control.begin(old_generation));
        assert!(control.begin(2));
    }

    #[test]
    fn running_requests_leave_completion_selection_to_the_registry() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        let TimerRegistration::Running { pending } = &mut control.registration else {
            panic!("accepted fixture should be running");
        };
        *pending = Some(OrdinaryPending::Unregister);
        assert_eq!(control.reconcile(300), Ok(None));
        assert_eq!(control.schedule(90), Ok(None));
        assert_eq!(
            control.registration,
            TimerRegistration::Running {
                pending: Some(OrdinaryPending::Unregister),
            }
        );
        assert_eq!(control.generation(), generation);
    }

    #[test]
    fn completion_arms_the_supplied_deadline() {
        for deadline_ns in [90, 300] {
            let mut control = TimerControl::default();
            let generation = arm(&mut control, 100);
            assert!(control.begin(generation));
            assert_eq!(control.arm_deadline(deadline_ns), Ok(()));
            assert_eq!(
                control.registration,
                TimerRegistration::Scheduled { deadline_ns }
            );
            assert_eq!(control.generation(), 2);
        }
    }

    #[test]
    fn completion_without_a_successor_stops_running_work() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        assert!(!control.terminate(InactiveReason::Stopped));
        assert_eq!(
            control.registration,
            TimerRegistration::Inactive {
                reason: InactiveReason::Stopped,
            }
        );
    }

    #[test]
    fn stopping_invalidates_delivery_without_allocating_and_rearm_uses_next_generation() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.terminate(InactiveReason::Cancelled));
        assert!(!control.begin(generation));
        assert_eq!(control.generation(), generation);
        assert_eq!(
            control.registration,
            TimerRegistration::Inactive {
                reason: InactiveReason::Cancelled,
            }
        );
        assert!(!control.terminate(InactiveReason::Cancelled));
        assert_eq!(control.generation(), generation);
        assert_eq!(
            control.registration,
            TimerRegistration::Inactive {
                reason: InactiveReason::Cancelled,
            }
        );
        assert_eq!(arm(&mut control, 200), generation + 1);
        assert!(!control.begin(generation));
        assert!(control.begin(generation + 1));
    }

    #[test]
    fn exhausted_generation_rejects_successors_without_preventing_stop() {
        let mut control = TimerControl::default();
        control.seed_generation_for_test(u64::MAX - 1);
        assert_eq!(arm(&mut control, 100), u64::MAX);

        assert_eq!(
            control.schedule(50),
            Err(TimerControlFailure::GenerationExhausted)
        );
        assert_eq!(
            control.registration,
            TimerRegistration::Scheduled { deadline_ns: 100 }
        );
        assert_eq!(control.generation(), u64::MAX);

        assert!(control.begin(u64::MAX));
        let TimerRegistration::Running { pending } = &mut control.registration else {
            panic!("accepted fixture should be running");
        };
        *pending = Some(OrdinaryPending::Cancel);
        assert_eq!(
            control.arm_deadline(200),
            Err(TimerControlFailure::GenerationExhausted)
        );
        assert_eq!(
            control.registration,
            TimerRegistration::Running {
                pending: Some(OrdinaryPending::Cancel),
            }
        );
        assert_eq!(control.generation(), u64::MAX);
        assert!(!control.terminate(InactiveReason::Cancelled));
        assert_eq!(control.generation(), u64::MAX);
        assert_eq!(
            control.registration,
            TimerRegistration::Inactive {
                reason: InactiveReason::Cancelled,
            }
        );
    }
}
