use super::*;
use crate::{TimerLastOutcome, TimerRegistrationStatus, WatchdogDecision};
use std::time::Duration;

impl CallbackToken {
    pub(crate) const fn callback_generation(&self) -> u64 {
        self.callback_generation
    }
}

impl RegistryTransition {
    pub(crate) fn into_effect(self) -> RegistryEffect {
        self.effect
    }
}

impl TimerRegistry {
    pub(crate) fn take_wakeup_handle(
        &mut self,
        identity: &TimerIdentity,
    ) -> Option<(CallbackToken, TimerHandle)> {
        let entry = self.entries.get_mut(identity)?;
        let owned = entry.wakeup.take()?;
        Some((
            CallbackToken::new(
                identity.clone(),
                entry.claim_generation,
                owned.callback_generation,
                entry.kind.wakeup_role(),
            ),
            owned.handle,
        ))
    }
    /// Model successful platform application in provider-neutral transition tests.
    pub(crate) fn confirm_effect_applied(
        &mut self,
        effect: &RegistryEffect,
    ) -> Result<(), RegistryError> {
        let (token, delay_ns) = match effect {
            RegistryEffect::None | RegistryEffect::ClearCallbacks { .. } => return Ok(()),
            RegistryEffect::ArmWakeup {
                token, delay_ns, ..
            } => (token.token().clone(), *delay_ns),
            RegistryEffect::DispatchWatchdog {
                dispatch,
                successor_delay_ns,
            } => (dispatch.work(), *successor_delay_ns),
        };
        let entry = self.entry_by_token_mut(&token)?;
        entry.provider_handle_slot(&token, ProviderBinding::Commit { delay_ns })?;
        entry.record_arm(&token, delay_ns);
        Ok(())
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn register_once(
        &mut self,
        identity: TimerIdentity,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register_once_with_callback(identity, lifetime, ordinary_callback_fixture())
    }

    pub(crate) fn register_after_completion(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register_after_completion_with_callback(
            identity,
            cadence,
            lifetime,
            ordinary_callback_fixture(),
        )
    }

    pub(crate) fn register_watchdog(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register_watchdog_with_callback(
            identity,
            cadence,
            lifetime,
            watchdog_callback_fixture(),
        )
    }
}

// Pure transition fixtures install real typed payloads but never execute work.
fn ordinary_callback_fixture() -> OrdinaryCallback {
    Rc::new(RefCell::new(|_| {
        panic!("pure registry fixture must not execute ordinary work")
    }))
}

fn watchdog_callback_fixture() -> WatchdogCallback {
    Rc::new(RefCell::new(|_| {
        panic!("pure registry fixture must not execute watchdog work")
    }))
}

fn identity(name: &str) -> TimerIdentity {
    TimerIdentity::try_new("test", "registry", name).expect("fixture identity should be valid")
}

fn registry() -> TimerRegistry {
    TimerRegistry::new(TimerEpoch::new(7, 10))
}

fn cadence(nanoseconds: u64) -> TimerCadence {
    TimerCadence::from_nanos(nanoseconds).expect("fixture cadence should be valid")
}

fn arm(transition: RegistryTransition) -> (CallbackToken, WakeupArm) {
    match transition.into_effect() {
        RegistryEffect::ArmWakeup { token, arm, .. } => (token.token().clone(), arm),
        effect => panic!("expected arm effect, got {effect:?}"),
    }
}

fn dispatch(transition: RegistryTransition) -> (CallbackToken, CallbackToken) {
    match transition.into_effect() {
        RegistryEffect::DispatchWatchdog { dispatch, .. } => {
            (dispatch.successor().clone(), dispatch.work())
        }
        effect => panic!("expected watchdog dispatch, got {effect:?}"),
    }
}

fn scheduled_deadline(registry: &TimerRegistry, token: &CallbackToken) -> u64 {
    registry
        .snapshot(token.identity())
        .and_then(|snapshot| snapshot.next_deadline_ns())
        .expect("scheduled generation should have an authoritative deadline")
}

fn confirm(registry: &mut TimerRegistry, transition: &RegistryTransition) {
    registry
        .confirm_effect_applied(transition.effect())
        .expect("fixture provider effect should apply");
}

mod delivery;
mod inactive_demand;
mod lifetime;
mod ordinary_scheduling;
mod registration;
mod watchdog_completion;
mod watchdog_order;
mod watchdog_scheduling;
