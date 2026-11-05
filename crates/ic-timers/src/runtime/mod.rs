//! Canister-local owner and live timer execution.

use ControlAuthority::{Executing, Registration as Declared};

use crate::{
    callback::{AfterCompletionRunResult, OnceRunResult, OrdinaryRunResult},
    platform::{self, TimerHandle},
    registry::{
        CallbackRole, CallbackToken, OrdinaryCallback, ProviderBinding, ProviderHandles,
        RegisterError, RegistrationClaim, RegistryEffect, RegistryError, RegistryTransition,
        TimerRegistry, WakeupToken, WatchdogCallback,
    },
    schedule::{OrdinaryDirective, ScheduleError, TimerCadence, TimerSchedule},
    snapshot::{
        DeclarationLifetime, MemoryPageExtent, TimerCompletion, TimerControlFailure, TimerEpoch,
        TimerIdentity, TimerInventorySnapshot, TimerPolicy, TimerSnapshot, WatchdogRunResult,
    },
};
use std::cell::RefCell;
use thiserror::Error;

mod effects;
use effects::{clear_provider_handles, finish_transition};
mod capability;
mod dispatch;
mod registration;

pub use capability::{
    AfterCompletionContext, AfterCompletionRegistration, OnceContext, OnceRegistration,
    TimerReconcileState, WatchdogContext, WatchdogReconcileState, WatchdogRegistration,
};
#[cfg(test)]
use dispatch::{CallbackMeasurementStart, dispatch_watchdog_scheduler};
use dispatch::{dispatch_wakeup, dispatch_watchdog_work, fail_claim_provider_binding};
#[cfg(test)]
use registration::erase_ordinary_callback;
pub use registration::{
    reconcile_after_completion, reconcile_once, reconcile_watchdog, register_after_completion,
    register_once, register_watchdog,
};
#[cfg(test)]
use tests::{take_provider_install_fault, take_watchdog_completion_fault};

thread_local! {
    static RUNTIME: RefCell<Option<TimerRegistry>> = const { RefCell::new(None) };
}

/// Initial schedule and declaration retention for a direct registration.
///
/// Retained declarations may start inactive. A declaration removed when stopped
/// always starts with a schedule; all three registration policies use this same
/// configuration. Durations and deadlines are checked when registering.
///
/// A remove-on-stop declaration cannot start inactive:
///
/// ```compile_fail,E0308
/// use ic_timers::TimerStart;
/// let start = TimerStart::RemoveWhenStopped { initial: None };
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerStart {
    /// Keep callback authority after stopping, optionally starting scheduled.
    Retained {
        /// Initial wake-up, or `None` for an inactive declaration.
        initial: Option<TimerSchedule>,
    },
    /// Start scheduled and remove callback authority after stopping.
    RemoveWhenStopped {
        /// Required initial wake-up.
        initial: TimerSchedule,
    },
}

impl TimerStart {
    const fn into_parts(self) -> (DeclarationLifetime, Option<TimerSchedule>) {
        match self {
            Self::Retained { initial } => (DeclarationLifetime::Retained, initial),
            Self::RemoveWhenStopped { initial } => {
                (DeclarationLifetime::RemoveWhenStopped, Some(initial))
            }
        }
    }
}

/// Failure from the canister-local timer runtime API.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum TimerError {
    /// The lifecycle owner has not initialized the volatile runtime.
    #[error("timer runtime is not initialized")]
    NotInitialized,
    /// A nested internal borrow indicates unsupported re-entrancy.
    #[error("timer runtime is already borrowed")]
    RuntimeBusy,
    /// Claiming one canonical timer identity failed.
    #[error(transparent)]
    Register(#[from] RegisterError),
    /// Cadence or deadline validation failed.
    #[error(transparent)]
    Schedule(#[from] ScheduleError),
    /// The logical registration was removed or superseded.
    #[error("timer registration is no longer authoritative")]
    RegistrationExpired,
    /// Pure checked control reached a terminal failure after effects were applied.
    #[error("timer control failed: {0:?}")]
    ControlFailure(TimerControlFailure),
    /// Canonical callback or provider-handle ownership was internally inconsistent.
    #[error("timer runtime ownership invariant failed")]
    OwnershipInvariant,
    /// The bounded pending-callback store is full.
    #[error("timer backend pending capacity exhausted")]
    BackendCapacity,
    /// The backend cannot allocate another non-reusable callback identity.
    #[error("timer backend callback identities exhausted")]
    BackendIdentityExhausted,
    /// A retained lifecycle claim no longer matches its canonical declaration.
    #[error("timer lifecycle reconciliation conflicts with the canonical declaration")]
    ReconciliationConflict,
}

impl From<platform::AdmissionError> for TimerError {
    fn from(error: platform::AdmissionError) -> Self {
        match error {
            platform::AdmissionError::Capacity => Self::BackendCapacity,
            platform::AdmissionError::IdentityExhausted => Self::BackendIdentityExhausted,
            platform::AdmissionError::DeadlineOverflow => {
                Self::Schedule(ScheduleError::DeadlineOverflow)
            }
        }
    }
}

impl From<RegistryError> for TimerError {
    fn from(value: RegistryError) -> Self {
        match value {
            RegistryError::UnknownRegistration
            | RegistryError::StaleRegistration
            | RegistryError::StaleCallback => Self::RegistrationExpired,
            RegistryError::PolicyMismatch { .. } | RegistryError::ProviderHandleAlreadyOwned => {
                Self::OwnershipInvariant
            }
            RegistryError::Schedule(error) => Self::Schedule(error),
        }
    }
}

/// Initialize the volatile canister-local runtime once for this Wasm instance.
///
/// Repeated calls are idempotent and return the original epoch. This function
/// exports no lifecycle hook; the canister's existing lifecycle owner calls it.
///
/// # Errors
///
/// Returns [`TimerError::RuntimeBusy`] if initialization re-enters an
/// active registry borrow.
pub fn initialize_runtime() -> Result<TimerEpoch, TimerError> {
    RUNTIME.with(|runtime| {
        let mut runtime = runtime
            .try_borrow_mut()
            .map_err(|_| TimerError::RuntimeBusy)?;
        let registry = runtime.get_or_insert_with(|| {
            TimerRegistry::new(TimerEpoch::new(
                platform::canister_version(),
                platform::time_ns(),
            ))
        });
        Ok(registry.epoch())
    })
}

/// Return one coherent inert snapshot by identity.
///
/// # Errors
///
/// Returns [`TimerError::NotInitialized`] before runtime initialization,
/// or [`TimerError::RuntimeBusy`] during a conflicting registry borrow.
pub fn timer_snapshot(identity: &TimerIdentity) -> Result<Option<TimerSnapshot>, TimerError> {
    with_registry(|registry| Ok(registry.snapshot(identity)))
}

/// Return one atomic bounded inventory with its volatile runtime epoch.
///
/// Timer snapshots are ordered deterministically by identity. An initialized
/// empty registry still returns its epoch and an empty timer slice.
///
/// # Errors
///
/// Returns [`TimerError::NotInitialized`] before runtime initialization,
/// or [`TimerError::RuntimeBusy`] during a conflicting registry borrow.
pub fn timer_inventory() -> Result<TimerInventorySnapshot, TimerError> {
    with_registry(|registry| Ok(registry.inventory()))
}

/// Return functional expected-failure state by identity without a full inventory.
///
/// # Errors
///
/// Returns [`TimerError::NotInitialized`] before runtime initialization,
/// or [`TimerError::RuntimeBusy`] during a conflicting registry borrow.
pub fn consecutive_expected_failures(identity: &TimerIdentity) -> Result<Option<u64>, TimerError> {
    with_registry(|registry| Ok(registry.consecutive_expected_failures(identity)))
}

fn has_armed_wakeup_claim(claim: &RegistrationClaim) -> Result<bool, TimerError> {
    with_registry(|registry| registry.has_armed_wakeup(claim).map_err(TimerError::from))
}

// A callback carries its own claim; callers cannot pair it with unrelated authority.
#[derive(Clone, Copy)]
enum ControlAuthority<'a> {
    Registration(&'a RegistrationClaim),
    Executing(&'a CallbackToken),
}

impl<'a> ControlAuthority<'a> {
    const fn claim(self) -> &'a RegistrationClaim {
        match self {
            Self::Registration(claim) => claim,
            Self::Executing(token) => token.claim(),
        }
    }
}

fn apply_claim_transition(
    authority: ControlAuthority<'_>,
    operation: impl FnOnce(
        &mut TimerRegistry,
        &RegistrationClaim,
    ) -> Result<RegistryTransition, RegistryError>,
) -> Result<(), TimerError> {
    let claim = authority.claim();
    let (transition, handles) = with_registry_mut(|registry| {
        if let Executing(token) = authority {
            registry
                .validate_running_context(token)
                .map_err(TimerError::from)?;
        }
        registry
            .transition_claim(claim, |registry| operation(registry, claim))
            .map_err(TimerError::from)
    })?;
    match finish_transition(transition, handles) {
        result @ (Ok(()) | Err(TimerError::ControlFailure(_))) => result,
        Err(error) => retire_failed_claim(claim, error),
    }
}

fn ensure_once_claim(
    authority: ControlAuthority<'_>,
    schedule: TimerSchedule,
) -> Result<(), TimerError> {
    apply_claim_transition(authority, |registry, claim| {
        registry.ensure_once(claim, platform::time_ns(), schedule)
    })
}

fn reconcile_ordinary_claim(
    authority: ControlAuthority<'_>,
    schedule: Option<TimerSchedule>,
) -> Result<(), TimerError> {
    apply_claim_transition(authority, |registry, claim| {
        registry.reconcile_ordinary(claim, platform::time_ns(), schedule)
    })
}

fn ensure_recurring_claim(authority: ControlAuthority<'_>) -> Result<(), TimerError> {
    apply_claim_transition(authority, |registry, claim| {
        registry.ensure_recurring(claim, platform::time_ns())
    })
}

fn ensure_watchdog_immediately_claim(authority: ControlAuthority<'_>) -> Result<(), TimerError> {
    apply_claim_transition(authority, |registry, claim| {
        registry.ensure_watchdog_immediately(claim, platform::time_ns())
    })
}

fn reconcile_watchdog_claim(
    authority: ControlAuthority<'_>,
    schedule: Option<TimerSchedule>,
) -> Result<(), TimerError> {
    apply_claim_transition(authority, |registry, claim| {
        registry.reconcile_watchdog_schedule(claim, platform::time_ns(), schedule)
    })
}

fn cancel_claim(authority: ControlAuthority<'_>) -> Result<(), TimerError> {
    apply_claim_transition(authority, TimerRegistry::cancel)
}

fn unregister_claim(claim: &RegistrationClaim) -> Result<(), TimerError> {
    apply_claim_transition(Declared(claim), TimerRegistry::unregister)
}

fn retire_failed_claim(claim: &RegistrationClaim, error: TimerError) -> Result<(), TimerError> {
    match fail_claim_provider_binding(claim) {
        Ok(()) | Err(TimerError::RegistrationExpired) => Err(error),
        Err(cleanup_error) => Err(cleanup_error),
    }
}

fn with_registry<T>(
    operation: impl FnOnce(&TimerRegistry) -> Result<T, TimerError>,
) -> Result<T, TimerError> {
    RUNTIME.with(|runtime| {
        let runtime = runtime.try_borrow().map_err(|_| TimerError::RuntimeBusy)?;
        let registry = runtime.as_ref().ok_or(TimerError::NotInitialized)?;
        operation(registry)
    })
}

fn with_registry_mut<T>(
    operation: impl FnOnce(&mut TimerRegistry) -> Result<T, TimerError>,
) -> Result<T, TimerError> {
    RUNTIME.with(|runtime| {
        let mut runtime = runtime
            .try_borrow_mut()
            .map_err(|_| TimerError::RuntimeBusy)?;
        let registry = runtime.as_mut().ok_or(TimerError::NotInitialized)?;
        operation(registry)
    })
}

#[cfg(test)]
mod tests;
