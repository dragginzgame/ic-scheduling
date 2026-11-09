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

    pub(super) fn register(policy: TestPolicy, timer: TimerIdentity, start: TimerStart) -> Self {
        match policy {
            TestPolicy::Once => Self::Once(Some(
                register_once(timer, start, stopping_once(|| {})).unwrap(),
            )),
            TestPolicy::AfterCompletion => Self::AfterCompletion(Some(
                register_after_completion(
                    timer,
                    test_cadence(5),
                    start,
                    stopping_after_completion(|| {}),
                )
                .unwrap(),
            )),
            TestPolicy::Watchdog => Self::Watchdog(Some(
                register_watchdog(timer, test_cadence(5), start, stopping_watchdog(|| {})).unwrap(),
            )),
        }
    }

    // Dispatch through the public typed methods, retaining their forwarding proof.
    // Once requires a deadline; recurring policies use their declared cadence.
    pub(super) fn ensure_scheduled(&self, once_at_ns: u64) -> Result<(), TimerError> {
        match self {
            Self::Once(slot) => slot
                .as_ref()
                .unwrap()
                .ensure_scheduled(TimerSchedule::At(once_at_ns)),
            Self::AfterCompletion(slot) => slot.as_ref().unwrap().ensure_scheduled(),
            Self::Watchdog(slot) => slot.as_ref().unwrap().ensure_scheduled(),
        }
    }

    pub(super) fn cancel(&self) -> Result<(), TimerError> {
        match self {
            Self::Once(slot) => slot.as_ref().unwrap().cancel(),
            Self::AfterCompletion(slot) => slot.as_ref().unwrap().cancel(),
            Self::Watchdog(slot) => slot.as_ref().unwrap().cancel(),
        }
    }

    pub(super) fn has_armed_wakeup(&self) -> Result<bool, TimerError> {
        match self {
            Self::Once(slot) => slot.as_ref().unwrap().has_armed_wakeup(),
            Self::AfterCompletion(slot) => slot.as_ref().unwrap().has_armed_wakeup(),
            Self::Watchdog(slot) => slot.as_ref().unwrap().has_armed_wakeup(),
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
