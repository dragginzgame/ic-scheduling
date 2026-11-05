//! One registry entry: typed policy payload, linear provider handles and observations.
//! The parent registry keeps claim admission, transitions and terminal removal.

use super::{
    CallbackRole, CallbackToken, OrdinaryCallback, ProviderBinding, RegistryError,
    RegistryTransition, WatchdogCallback,
    watchdog::{WatchdogControl, WatchdogState},
};
use crate::{
    control::{TimerControl, TimerRegistration},
    platform::TimerHandle,
    schedule::TimerCadence,
    snapshot::{
        DeclarationLifetime, OrdinaryRuntimeStateSnapshot, TimerDirectiveSnapshot, TimerEpoch,
        TimerIdentity, TimerObservabilitySnapshot, TimerPolicy, TimerRegistrationId,
        TimerRuntimeStateSnapshot, TimerSchedulingMode, TimerSnapshot, WatchdogAttemptStatus,
        WatchdogRuntimeStateSnapshot,
    },
};

pub(super) struct OwnedProviderHandle {
    pub(super) callback_generation: u64,
    pub(super) handle: TimerHandle,
}

/// The at-most-two provider capabilities owned by one timer entry.
#[must_use = "clear every detached provider handle"]
#[derive(Default)]
pub struct ProviderHandles {
    pub(super) wakeup: Option<TimerHandle>,
    pub(super) work: Option<TimerHandle>,
}

impl ProviderHandles {
    pub(crate) const fn take_wakeup(&mut self) -> Option<TimerHandle> {
        self.wakeup.take()
    }

    pub(crate) const fn take_work(&mut self) -> Option<TimerHandle> {
        self.work.take()
    }
}

// One payload owns policy, callback type and control together. An ordinary
// entry without cadence is Once; one with cadence is AfterCompletion.
pub(super) enum EntryKind {
    Ordinary {
        cadence: Option<TimerCadence>,
        callback: OrdinaryCallback,
        control: TimerControl,
    },
    Watchdog {
        cadence: TimerCadence,
        callback: WatchdogCallback,
        control: WatchdogControl,
    },
}

impl EntryKind {
    pub(super) const fn wakeup_role(&self) -> CallbackRole {
        match self {
            Self::Ordinary { .. } => CallbackRole::OrdinaryWork,
            Self::Watchdog { .. } => CallbackRole::WatchdogScheduler,
        }
    }

    pub(super) fn ordinary(cadence: Option<TimerCadence>, callback: OrdinaryCallback) -> Self {
        Self::Ordinary {
            cadence,
            callback,
            control: TimerControl::default(),
        }
    }

    pub(super) fn watchdog(cadence: TimerCadence, callback: WatchdogCallback) -> Self {
        Self::Watchdog {
            cadence,
            callback,
            control: WatchdogControl::default(),
        }
    }

    pub(super) const fn policy(&self) -> TimerPolicy {
        match self {
            Self::Ordinary { cadence: None, .. } => TimerPolicy::Once,
            Self::Ordinary {
                cadence: Some(cadence),
                ..
            } => TimerPolicy::AfterCompletion { cadence: *cadence },
            Self::Watchdog { cadence, .. } => TimerPolicy::Watchdog { cadence: *cadence },
        }
    }
}

pub(super) struct Entry {
    pub(super) claim_generation: u64,
    pub(super) lifetime: DeclarationLifetime,
    pub(super) kind: EntryKind,
    pub(super) scheduling_mode: TimerSchedulingMode,
    pub(super) latest_directive: Option<TimerDirectiveSnapshot>,
    pub(super) latest_requested_delay_ns: Option<u64>,
    latest_armed_delay_ns: Option<u64>,
    pub(super) confirmed_wakeup_generation: Option<u64>,
    pub(super) wakeup: Option<OwnedProviderHandle>,
    pub(super) work: Option<OwnedProviderHandle>,
    pub(super) observability: TimerObservabilitySnapshot,
}

impl Entry {
    pub(super) const fn new(
        claim_generation: u64,
        kind: EntryKind,
        lifetime: DeclarationLifetime,
    ) -> Self {
        let scheduling_mode = match kind.policy() {
            TimerPolicy::Once => TimerSchedulingMode::Once,
            TimerPolicy::AfterCompletion { .. } => TimerSchedulingMode::AfterCompletion,
            TimerPolicy::Watchdog { .. } => TimerSchedulingMode::Watchdog,
        };

        Self {
            claim_generation,
            lifetime,
            kind,
            scheduling_mode,
            latest_directive: None,
            latest_requested_delay_ns: None,
            latest_armed_delay_ns: None,
            confirmed_wakeup_generation: None,
            wakeup: None,
            work: None,
            observability: TimerObservabilitySnapshot::EMPTY,
        }
    }

    /// Validate the role, live generation and state for each provider binding.
    pub(super) const fn provider_handle_slot(
        &mut self,
        token: &CallbackToken,
        binding: ProviderBinding,
    ) -> Result<&mut Option<OwnedProviderHandle>, RegistryError> {
        match (&self.kind, token.role) {
            (EntryKind::Ordinary { control, .. }, CallbackRole::OrdinaryWork)
                if matches!(
                    control.registration,
                    TimerRegistration::Scheduled { .. }
                        if control.generation() == token.callback_generation
                ) =>
            {
                Ok(&mut self.wakeup)
            }
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogScheduler)
                if control.can_bind_scheduler(token.callback_generation, binding) =>
            {
                Ok(&mut self.wakeup)
            }
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogWork)
                if control.can_bind_work(token.callback_generation) =>
            {
                Ok(&mut self.work)
            }
            _ => Err(RegistryError::StaleCallback),
        }
    }

    pub(super) fn record_arm(&mut self, token: &CallbackToken, delay_ns: u64) {
        if self.confirmed_wakeup_generation == Some(token.callback_generation) {
            return;
        }
        self.confirmed_wakeup_generation = Some(token.callback_generation);
        self.latest_armed_delay_ns = Some(delay_ns);
        let counters = self.observability.counters_mut();
        counters.record_wakeup_armed();
        if token.role == CallbackRole::WatchdogWork {
            counters.record_work_dispatched();
        }
    }

    pub(super) const fn snapshot(
        &self,
        identity: TimerIdentity,
        epoch: TimerEpoch,
    ) -> TimerSnapshot {
        let state = match &self.kind {
            EntryKind::Ordinary { control, .. } => match control.registration {
                TimerRegistration::Inactive { reason } => {
                    TimerRuntimeStateSnapshot::Inactive { reason }
                }
                TimerRegistration::Scheduled { deadline_ns } => {
                    TimerRuntimeStateSnapshot::Ordinary(OrdinaryRuntimeStateSnapshot::Scheduled {
                        generation: control.generation(),
                        deadline_ns,
                    })
                }
                TimerRegistration::Running { .. } => {
                    TimerRuntimeStateSnapshot::Ordinary(OrdinaryRuntimeStateSnapshot::Running {
                        generation: control.generation(),
                    })
                }
            },
            EntryKind::Watchdog { control, .. } => match control.state() {
                WatchdogState::Inactive { reason } => {
                    TimerRuntimeStateSnapshot::Inactive { reason }
                }
                WatchdogState::Scheduled { deadline_ns } => {
                    TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled {
                        scheduler_generation: control.generation(),
                        deadline_ns,
                    })
                }
                WatchdogState::AwaitingWork {
                    successor_deadline_ns,
                    attempt_status,
                    ..
                } => TimerRuntimeStateSnapshot::Watchdog(
                    WatchdogRuntimeStateSnapshot::AwaitingWork {
                        successor_generation: control.generation(),
                        successor_deadline_ns,
                        attempt_status,
                    },
                ),
            },
        };

        TimerSnapshot::new(
            identity,
            TimerRegistrationId::new(epoch, self.claim_generation),
            self.kind.policy(),
            self.lifetime,
            state,
            self.scheduling_mode,
            self.latest_directive,
            self.latest_requested_delay_ns,
            self.latest_armed_delay_ns,
            &self.observability,
        )
    }

    /// Decide terminal removal from the entry already selected and authorized
    /// by the transition owner. Runtime detaches handles before that transition.
    pub(super) const fn remove_on_failure(&self, transition: &RegistryTransition) -> bool {
        transition.failure().is_some()
            && matches!(self.lifetime, DeclarationLifetime::RemoveWhenStopped)
    }

    pub(super) fn take_wakeup_handle(&mut self) -> Option<TimerHandle> {
        self.wakeup.take().map(|owned| owned.handle)
    }

    pub(super) fn take_work_handle(&mut self) -> Option<TimerHandle> {
        self.work.take().map(|owned| owned.handle)
    }

    pub(super) fn take_provider_handles(&mut self) -> ProviderHandles {
        ProviderHandles {
            wakeup: self.take_wakeup_handle(),
            work: self.take_work_handle(),
        }
    }

    // The caller selected this entry by identity. Fired handles are consumed
    // independently of callback acceptance, but only for the exact claim,
    // policy role and owned slot generation.
    pub(super) fn consume_provider_handle(&mut self, token: &CallbackToken) {
        if !self.owns_token_claim(token) {
            return;
        }
        let slot = match token.role {
            CallbackRole::OrdinaryWork | CallbackRole::WatchdogScheduler
                if token.role == self.kind.wakeup_role() =>
            {
                &mut self.wakeup
            }
            CallbackRole::WatchdogWork if matches!(self.kind, EntryKind::Watchdog { .. }) => {
                &mut self.work
            }
            _ => return,
        };
        if slot
            .as_ref()
            .is_some_and(|owned| owned.callback_generation == token.callback_generation)
        {
            *slot = None;
        }
    }

    pub(super) const fn owns_token_claim(&self, token: &CallbackToken) -> bool {
        self.claim_generation == token.claim().claim_generation()
    }

    // Registry lookups select this entry by token identity first.
    pub(super) const fn owns_running_work(&self, token: &CallbackToken) -> bool {
        if !self.owns_token_claim(token) {
            return false;
        }
        match (&self.kind, token.role) {
            (EntryKind::Ordinary { control, .. }, CallbackRole::OrdinaryWork) => matches!(
                control.registration,
                TimerRegistration::Running { .. }
                    if control.generation() == token.callback_generation
            ),
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogWork) => matches!(
                control.state(),
                WatchdogState::AwaitingWork {
                    attempt_status: WatchdogAttemptStatus::Running,
                    ..
                } if control.generation() == token.callback_generation
            ),
            (
                EntryKind::Ordinary { .. },
                CallbackRole::WatchdogScheduler | CallbackRole::WatchdogWork,
            )
            | (
                EntryKind::Watchdog { .. },
                CallbackRole::OrdinaryWork | CallbackRole::WatchdogScheduler,
            ) => false,
        }
    }
}
