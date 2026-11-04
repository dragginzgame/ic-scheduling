//! Constructible effect shapes; live claim and generation admission stays in the registry.

use super::{CallbackRole, CallbackToken, CallbacksToClear, RegistrationClaim, token_for};
use crate::control::WakeupArm;

/// A wakeup can only address ordinary work or a Watchdog scheduler.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WakeupToken {
    Ordinary(CallbackToken),
    Scheduler(CallbackToken),
}

impl WakeupToken {
    pub(super) fn ordinary(claim: &RegistrationClaim, generation: u64) -> Self {
        Self::Ordinary(token_for(claim, generation, CallbackRole::OrdinaryWork))
    }

    pub(super) fn scheduler(claim: &RegistrationClaim, generation: u64) -> Self {
        Self::Scheduler(token_for(
            claim,
            generation,
            CallbackRole::WatchdogScheduler,
        ))
    }

    pub(crate) const fn token(&self) -> &CallbackToken {
        match self {
            Self::Ordinary(token) | Self::Scheduler(token) => token,
        }
    }
}

/// One dispatch identity; its work token is derived rather than stored independently.
#[derive(Debug, Eq, PartialEq)]
pub struct WatchdogDispatch {
    successor: CallbackToken,
}

impl WatchdogDispatch {
    pub(super) fn new(claim: &RegistrationClaim, generation: u64) -> Self {
        Self {
            successor: token_for(claim, generation, CallbackRole::WatchdogScheduler),
        }
    }

    pub(crate) const fn successor(&self) -> &CallbackToken {
        &self.successor
    }

    pub(crate) fn work(&self) -> CallbackToken {
        token_for(
            self.successor.claim(),
            self.successor.callback_generation,
            CallbackRole::WatchdogWork,
        )
    }
}

/// Provider-neutral effect emitted by one pure transition.
/// Absolute deadlines stay in control state; arms carry resolved provider delays.
#[derive(Debug, Eq, PartialEq)]
pub enum RegistryEffect {
    None,
    ArmWakeup {
        token: WakeupToken,
        delay_ns: u64,
        arm: WakeupArm,
    },
    ClearCallbacks {
        handles: CallbacksToClear,
    },
    DispatchWatchdog {
        dispatch: WatchdogDispatch,
        successor_delay_ns: u64,
    },
}

#[cfg(test)]
mod tests;
