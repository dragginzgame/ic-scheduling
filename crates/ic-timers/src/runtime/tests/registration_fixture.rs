//! Typed native owner slots shared by reconciliation and capture scenarios.

use super::*;

pub(super) enum ReconciliationSlot {
    Once(Option<OnceRegistration>),
    AfterCompletion(Option<AfterCompletionRegistration>),
    Watchdog(Option<WatchdogRegistration>),
}

impl ReconciliationSlot {
    pub(super) const fn new(policy: TestPolicy) -> Self {
        match policy {
            TestPolicy::Once => Self::Once(None),
            TestPolicy::AfterCompletion => Self::AfterCompletion(None),
            TestPolicy::Watchdog => Self::Watchdog(None),
        }
    }

    pub(super) fn claim(&self) -> Option<&RegistrationClaim> {
        match self {
            Self::Once(slot) => slot.as_ref().map(|registration| &registration.claim),
            Self::AfterCompletion(slot) => slot.as_ref().map(|registration| &registration.claim),
            Self::Watchdog(slot) => slot.as_ref().map(|registration| &registration.claim),
        }
    }

    pub(super) fn reconcile<C: 'static>(
        &mut self,
        timer: &TimerIdentity,
        cadence: TimerCadence,
        scheduled: bool,
        captured: C,
        calls: Rc<Cell<u64>>,
    ) -> Result<(), TimerError> {
        let record = move || {
            std::hint::black_box(&captured);
            calls.set(calls.get() + 1);
        };
        match self {
            Self::Once(slot) => reconcile_once(
                slot,
                timer,
                scheduled.then_some(TimerSchedule::After(Duration::from_nanos(
                    cadence.as_nanos(),
                ))),
                stopping_once(record),
            ),
            Self::AfterCompletion(slot) => reconcile_after_completion(
                slot,
                timer,
                cadence,
                if scheduled {
                    TimerReconcileState::Scheduled
                } else {
                    TimerReconcileState::Inactive
                },
                stopping_after_completion(record),
            ),
            Self::Watchdog(slot) => reconcile_watchdog(
                slot,
                timer,
                cadence,
                if scheduled {
                    WatchdogReconcileState::Scheduled
                } else {
                    WatchdogReconcileState::Inactive
                },
                stopping_watchdog(record),
            ),
        }
    }
}
