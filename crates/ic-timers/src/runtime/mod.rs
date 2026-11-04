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
use std::{cell::RefCell, future::Future, rc::Rc};
use thiserror::Error;

mod effects;
use effects::{clear_provider_handles, finish_transition};
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

/// Delegated control capability scoped to one exact `Once` work attempt.
///
/// Identity remains inspectable after work returns or is abandoned, but mutation
/// methods then return [`TimerError::RegistrationExpired`]. Retain the
/// [`OnceRegistration`] for longer-lived ownership.
pub struct OnceContext {
    token: CallbackToken,
}

impl OnceContext {
    const fn new(token: CallbackToken) -> Self {
        Self { token }
    }

    /// Return the logical timer identity executing this work.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.token.identity()
    }

    /// Schedule a `Once` declaration while this exact work attempt is active.
    ///
    /// A context retained after completion or delivery abandonment is expired
    /// and returns [`TimerError::RegistrationExpired`].
    pub fn ensure_scheduled(&self, schedule: TimerSchedule) -> Result<(), TimerError> {
        ensure_once_claim(Executing(&self.token), schedule)
    }

    /// Reconcile the executing declaration to one exact schedule.
    ///
    /// A winning exact schedule replaces the returned scheduling proposal
    /// before validation. Explicit consumer invariant failure still stops.
    ///
    /// `None` requests idle state at normal completion without recording an
    /// explicit cancellation. Retained callback authority remains; a
    /// remove-on-stop declaration is removed when that request wins arbitration.
    /// A stored context cannot mutate the registration after its exact work
    /// attempt ends.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_ordinary_claim(Executing(&self.token), schedule)
    }

    /// Request cancellation while this exact work attempt is active.
    /// Cancellation allocates no callback generation.
    ///
    /// Cancellation does not interrupt the current invocation; normal
    /// completion applies it before any callback successor is retained.
    /// A retained declaration becomes inactive. A remove-on-stop declaration
    /// is removed when cancellation wins arbitration.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Executing(&self.token))
    }
}

/// Delegated control capability scoped to one exact after-completion work
/// attempt.
///
/// Mutation authority expires when the callback completes or its delivery is
/// abandoned. Retain the [`AfterCompletionRegistration`] for longer-lived
/// ownership.
pub struct AfterCompletionContext {
    token: CallbackToken,
}

impl AfterCompletionContext {
    const fn new(token: CallbackToken) -> Self {
        Self { token }
    }

    /// Return the logical timer identity executing this work.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.token.identity()
    }

    /// Request configured recurrence after this exact work attempt.
    pub fn ensure_scheduled(&self) -> Result<(), TimerError> {
        ensure_recurring_claim(Executing(&self.token))
    }

    /// Reconcile the executing declaration to one exact schedule without
    /// changing its configured after-completion cadence.
    ///
    /// A winning exact schedule replaces the returned scheduling proposal
    /// before validation. Explicit consumer invariant failure still stops.
    ///
    /// `None` requests idle state at normal completion without recording an
    /// explicit cancellation. A retained declaration keeps callback authority;
    /// a remove-on-stop declaration is removed when that request wins.
    /// A stored context cannot mutate the registration after its exact work
    /// attempt ends.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_ordinary_claim(Executing(&self.token), schedule)
    }

    /// Request cancellation while this exact work attempt is active.
    /// Cancellation allocates no callback generation.
    ///
    /// Cancellation does not interrupt the current invocation; normal
    /// completion applies it before any callback successor is retained. A
    /// retained declaration becomes inactive; a remove-on-stop declaration is
    /// removed when cancellation wins arbitration.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Executing(&self.token))
    }
}

/// Delegated control capability scoped to one exact watchdog work attempt.
///
/// Mutation authority expires when the callback completes. Retain the
/// [`WatchdogRegistration`] for longer-lived ownership.
pub struct WatchdogContext {
    token: CallbackToken,
}

impl WatchdogContext {
    const fn new(token: CallbackToken) -> Self {
        Self { token }
    }

    /// Return the logical timer identity executing this work.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.token.identity()
    }

    /// Request that the pre-armed cadence successor remain scheduled.
    ///
    /// This is idempotent unless it supersedes a nested cancellation request.
    /// It never arms an additional Watchdog successor and cannot move a
    /// pending immediate successor later.
    pub fn ensure_scheduled(&self) -> Result<(), TimerError> {
        ensure_recurring_claim(Executing(&self.token))
    }

    /// Request that this work attempt's pre-armed successor run immediately.
    ///
    /// Repeated requests coalesce, and a cadence ensure cannot move the
    /// pending immediate deadline later. Normal completion replaces the exact
    /// cadence successor with a zero-delay scheduler callback. Consumer work
    /// still runs only in the scheduler's separately queued later message.
    pub fn ensure_scheduled_immediately(&self) -> Result<(), TimerError> {
        ensure_watchdog_immediately_claim(Executing(&self.token))
    }

    /// Replace this attempt's successor with an exact schedule, or cancel it.
    ///
    /// Applied on normal completion; a trap preserves the committed cadence
    /// successor. The latest reconciliation wins over prior scheduling demand,
    /// but unregistration and invariant failure remain terminal.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_watchdog_claim(Executing(&self.token), schedule)
    }

    /// Request cancellation while this exact work attempt is active.
    /// Cancellation allocates no callback generation.
    ///
    /// Cancellation does not interrupt the current invocation; normal
    /// completion clears the already-armed successor when cancellation wins.
    /// A retained declaration becomes inactive; a remove-on-stop declaration
    /// is removed.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Executing(&self.token))
    }
}

/// Opaque non-clone claim for one registered `Once` callback.
///
/// Dropping this capability does not cancel or unregister its declaration. Keep
/// it in volatile owner state and call `unregister` for explicit removal. Losing
/// a retained claim can leave a declaration occupying registry capacity until
/// the next Wasm instance; dropping the claim does not release callback captures.
#[must_use = "retain the registration claim so the timer remains controllable"]
pub struct OnceRegistration {
    claim: RegistrationClaim,
}

impl OnceRegistration {
    /// Return the claimed logical identity.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.claim.identity()
    }

    /// Return whether this exact claim currently owns an armed provider wake-up.
    ///
    /// This is a volatile observation, not durable scheduling authority or a
    /// delivery guarantee. Call [`Self::ensure_scheduled`] unconditionally when
    /// a wake-up is required rather than using this value as a scheduling guard.
    pub fn has_armed_wakeup(&self) -> Result<bool, TimerError> {
        has_armed_wakeup_claim(&self.claim)
    }

    /// Ensure one invocation is armed or retained as the running work's successor.
    pub fn ensure_scheduled(&self, schedule: TimerSchedule) -> Result<(), TimerError> {
        ensure_once_claim(Declared(&self.claim), schedule)
    }

    /// Reconcile to one exact desired schedule, replacing a later or earlier
    /// live deadline as necessary.
    ///
    /// During running work, a winning exact schedule replaces the callback
    /// scheduling proposal before validation; explicit invariant failure stops.
    ///
    /// `None` leaves a retained declaration idle after any running work
    /// completes or is abandoned. A remove-on-stop declaration is removed and
    /// this claim expires when the transition finalizes. Clearing scheduled
    /// demand is distinct from explicit cancellation; an already inactive
    /// declaration retains its existing reason.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_ordinary_claim(Declared(&self.claim), schedule)
    }

    /// Cancel the armed callback or the running work's successor.
    /// Cancellation allocates no callback generation, even at the counter limit.
    ///
    /// Consumer work already running is not interrupted.
    /// A retained declaration keeps callback authority. A remove-on-stop
    /// declaration and this claim expire when cancellation finalizes.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Declared(&self.claim))
    }

    /// Consume the claim and unregister its callback authority.
    ///
    /// When called from running work, removal is deferred until that invocation
    /// completes normally or its confirmed delivery is abandoned.
    pub fn unregister(self) -> Result<(), TimerError> {
        unregister_claim(&self.claim)
    }
}

/// Opaque non-clone claim for one callback with configured
/// after-completion recurrence.
///
/// Dropping this capability does not cancel or unregister its declaration. Keep
/// it in volatile owner state and call `unregister` for explicit removal. Losing
/// a retained claim can leave a declaration occupying registry capacity until
/// the next Wasm instance; dropping the claim does not release callback captures.
#[must_use = "retain the registration claim so the timer remains controllable"]
pub struct AfterCompletionRegistration {
    claim: RegistrationClaim,
}

/// Opaque non-clone claim for one pre-armed watchdog callback.
///
/// Dropping this capability does not cancel or unregister its declaration. Keep
/// it in volatile owner state and call `unregister` for explicit removal. Losing
/// a retained claim can leave a declaration occupying registry capacity until
/// the next Wasm instance; dropping the claim does not release callback captures.
#[must_use = "retain the registration claim so the timer remains controllable"]
pub struct WatchdogRegistration {
    claim: RegistrationClaim,
}

/// Desired volatile after-completion state during lifecycle reconciliation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerReconcileState {
    /// Request inactive state immediately unless consumer work is running.
    Inactive,
    /// Preserve scheduling demand now or through the running work's successor.
    Scheduled,
}

/// Desired volatile watchdog state during synchronous lifecycle reconciliation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatchdogReconcileState {
    /// Request inactive state immediately unless consumer work is running.
    Inactive,
    /// Preserve or arm one scheduler wake-up at the configured cadence.
    Scheduled,
    /// Preserve an earlier wake-up or arm/move one scheduler wake-up to now.
    ///
    /// The zero-delay provider callback executes as a later replicated
    /// scheduler message and does not invoke consumer work synchronously.
    ScheduledImmediately,
    /// Reconcile the scheduler to an exact absolute IC timestamp.
    ScheduledAt(u64),
}

impl WatchdogRegistration {
    /// Return the claimed logical identity.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.claim.identity()
    }

    /// Return whether this exact claim currently owns an armed scheduler wake-up.
    ///
    /// The separately queued work callback is not itself a wake-up. During
    /// watchdog work this returns `true` only because the scheduler has already
    /// committed and installed the cadence successor. This is a volatile
    /// observation, not a delivery guarantee.
    /// Confirmed scheduler-delivery abandonment clears this observation; an
    /// owner can restore the wake-up through ensure or reconciliation, including
    /// an unchanged exact deadline. A stuck delivery that is never destroyed
    /// cannot be inferred from this observation.
    pub fn has_armed_wakeup(&self) -> Result<bool, TimerError> {
        has_armed_wakeup_claim(&self.claim)
    }

    /// Synchronously ensure one watchdog scheduler wake-up is authoritative.
    ///
    /// An existing earlier immediate deadline is retained and never delayed.
    pub fn ensure_scheduled(&self) -> Result<(), TimerError> {
        ensure_recurring_claim(Declared(&self.claim))
    }

    /// Synchronously ensure one watchdog scheduler wake-up is due now.
    ///
    /// An inactive declaration arms one zero-delay scheduler. A later cadence
    /// deadline is replaced in place; an already earlier or equivalent
    /// deadline and repeated immediate requests coalesce. If work is already
    /// dispatched, that zero-delay work satisfies the request. If work is
    /// running, the request applies to that attempt's pre-armed successor.
    pub fn ensure_scheduled_immediately(&self) -> Result<(), TimerError> {
        ensure_watchdog_immediately_claim(Declared(&self.claim))
    }

    /// Reconcile to one exact scheduler deadline, or cancel with `None`.
    ///
    /// Unlike ensure, this can move a wake-up later. Identical deadlines
    /// coalesce. Dispatched or running work is not postponed: reconciliation
    /// selects its successor on normal completion. Until then, the committed
    /// cadence successor remains the recovery wake-up. A subsequent recovery
    /// attempt retires the interrupted attempt's pending proposal.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_watchdog_claim(Declared(&self.claim), schedule)
    }

    /// Cancel the scheduler and any work callback that has not started.
    /// Cancellation allocates no callback generation, even at the counter limits.
    ///
    /// Consumer work already running is not interrupted; normal completion
    /// clears its pre-armed successor.
    /// A retained declaration keeps callback authority. A remove-on-stop
    /// declaration and this claim expire when cancellation finalizes.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Declared(&self.claim))
    }

    /// Consume the claim and unregister its callback authority.
    ///
    /// When called from running work, removal is deferred until that invocation
    /// completes normally.
    pub fn unregister(self) -> Result<(), TimerError> {
        unregister_claim(&self.claim)
    }
}

impl AfterCompletionRegistration {
    /// Return the claimed logical identity.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.claim.identity()
    }

    /// Return whether this exact claim currently owns an armed provider wake-up.
    ///
    /// A callback currently running without an installed successor returns
    /// `false`. This is a volatile observation, not durable scheduling authority
    /// or a delivery guarantee.
    pub fn has_armed_wakeup(&self) -> Result<bool, TimerError> {
        has_armed_wakeup_claim(&self.claim)
    }

    /// Ensure configured recurrence is armed or retained as the running work's
    /// successor.
    pub fn ensure_scheduled(&self) -> Result<(), TimerError> {
        ensure_recurring_claim(Declared(&self.claim))
    }

    /// Reconcile to one exact desired schedule without changing the configured
    /// after-completion cadence. `None` makes a retained declaration inactive
    /// after any running work completes or is abandoned; it removes a
    /// remove-on-stop declaration and expires this claim when the transition
    /// finalizes.
    ///
    /// During running work, a winning exact schedule replaces the callback
    /// scheduling proposal before validation; explicit invariant failure stops.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_ordinary_claim(Declared(&self.claim), schedule)
    }

    /// Cancel the armed callback or the running work's successor.
    /// Cancellation allocates no callback generation, even at the counter limit.
    ///
    /// Consumer work already running is not interrupted.
    /// A retained declaration keeps callback authority. A remove-on-stop
    /// declaration and this claim expire when cancellation finalizes.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Declared(&self.claim))
    }

    /// Consume the claim and unregister its callback authority.
    ///
    /// When called from running work, removal is deferred until that invocation
    /// completes normally or its confirmed delivery is abandoned.
    pub fn unregister(self) -> Result<(), TimerError> {
        unregister_claim(&self.claim)
    }
}

/// Register one asynchronous `Once` callback with its initial schedule.
///
/// [`TimerStart::Retained`] permits an inactive declaration;
/// [`TimerStart::RemoveWhenStopped`] always supplies its initial schedule.
/// Success returns installed callback authority and,
/// when requested, a confirmed provider wakeup. A returned error retires this
/// creation's declaration and handles before releasing callback captures.
///
/// The future must return [`OnceRunResult`]; a recurring policy's result cannot
/// be used here, even when its decision is Stop:
///
/// ```compile_fail,E0271
/// use ic_timers::{register_once, TimerIdentity, TimerStart,
///     TimerCompletion, AfterCompletionDecision, AfterCompletionRunResult};
/// let identity = TimerIdentity::try_new("app", "jobs", "once").unwrap();
/// let _ = register_once(identity, TimerStart::Retained { initial: None }, |_| async {
///     AfterCompletionRunResult::new(TimerCompletion::no_work(), AfterCompletionDecision::Stop)
/// });
/// ```
pub fn register_once<F, Fut>(
    identity: TimerIdentity,
    start: TimerStart,
    callback: F,
) -> Result<OnceRegistration, TimerError>
where
    F: FnMut(OnceContext) -> Fut + 'static,
    Fut: Future<Output = OnceRunResult> + 'static,
{
    let callback = erase_ordinary_callback(callback, OnceContext::new, OrdinaryRunResult::from);
    let claim = register_initial(start, |registry, lifetime| {
        registry.register_once_with_callback(identity, lifetime, Rc::clone(&callback))
    });
    // Release factory captures only after initial registry/provider cleanup.
    drop(callback);
    Ok(OnceRegistration { claim: claim? })
}

/// Register one asynchronous callback with configured after-completion recurrence.
///
/// The explicit initial schedule selects the first deadline independently of
/// the recurrence cadence. [`TimerStart::Retained`] may start inactive.
/// Initial scheduling failure removes the new declaration and provider handles.
///
/// The future must return [`AfterCompletionRunResult`]:
///
/// ```compile_fail,E0271
/// use ic_timers::{register_after_completion, TimerIdentity, TimerCadence,
///     TimerStart, TimerCompletion, OnceDecision, OnceRunResult};
/// let identity = TimerIdentity::try_new("app", "jobs", "recurring").unwrap();
/// let cadence = TimerCadence::from_nanos(5).unwrap();
/// let _ = register_after_completion(identity, cadence, TimerStart::Retained { initial: None }, |_| async {
///     OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
/// });
/// ```
pub fn register_after_completion<F, Fut>(
    identity: TimerIdentity,
    cadence: TimerCadence,
    start: TimerStart,
    callback: F,
) -> Result<AfterCompletionRegistration, TimerError>
where
    F: FnMut(AfterCompletionContext) -> Fut + 'static,
    Fut: Future<Output = AfterCompletionRunResult> + 'static,
{
    let callback = erase_ordinary_callback(
        callback,
        AfterCompletionContext::new,
        OrdinaryRunResult::from,
    );
    let claim = register_initial(start, |registry, lifetime| {
        registry.register_after_completion_with_callback(
            identity,
            cadence,
            lifetime,
            Rc::clone(&callback),
        )
    });
    // Release factory captures only after initial registry/provider cleanup.
    drop(callback);
    Ok(AfterCompletionRegistration { claim: claim? })
}

/// Register one synchronous pre-armed watchdog callback with its initial schedule.
///
/// The initial schedule selects the first scheduler message, never synchronous
/// consumer work. [`TimerStart::Retained`] may start inactive. Initial
/// scheduling failure removes the new declaration and provider handles.
///
/// The callback cannot be async: it runs only in the work message after a
/// separate scheduler message has armed the next cadence successor. Its
/// `WatchdogDecision` retains the cadence successor, replaces it with a
/// deadline of now or an exact timestamp, or clears it.
pub fn register_watchdog<F>(
    identity: TimerIdentity,
    cadence: TimerCadence,
    start: TimerStart,
    callback: F,
) -> Result<WatchdogRegistration, TimerError>
where
    F: FnMut(WatchdogContext) -> WatchdogRunResult + 'static,
{
    let mut callback = callback;
    let callback: WatchdogCallback = Rc::new(RefCell::new(move |token| {
        callback(WatchdogContext::new(token))
    }));
    let claim = register_initial(start, |registry, lifetime| {
        registry.register_watchdog_with_callback(identity, cadence, lifetime, Rc::clone(&callback))
    });
    // Release factory captures only after initial registry/provider cleanup.
    drop(callback);
    Ok(WatchdogRegistration { claim: claim? })
}

/// Finish creation before exposing a claim. The callback factory retains its
/// captures until this returns, including through rejected provider binding.
fn register_initial(
    start: TimerStart,
    register: impl FnOnce(
        &mut TimerRegistry,
        DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError>,
) -> Result<RegistrationClaim, TimerError> {
    let (lifetime, initial) = start.into_parts();
    register_initialized(
        |registry| {
            let now_ns = platform::time_ns();
            let requested = initial
                .map(|schedule| schedule.resolve(now_ns))
                .transpose()?;
            Ok((register(registry, lifetime)?, (requested, now_ns)))
        },
        |registry, claim, (requested, now_ns)| {
            requested
                .map(|schedule| {
                    registry
                        .transition_claim(claim, |registry| {
                            registry.reconcile_initial(claim, now_ns, schedule)
                        })
                        .map_err(TimerError::from)
                })
                .transpose()
        },
    )
}

/// Allocate and initialize under one borrow; publish authority only after effects.
/// Fresh initialization failures share direct registration's retirement owner.
fn register_initialized<Prepared>(
    register: impl FnOnce(&mut TimerRegistry) -> Result<(RegistrationClaim, Prepared), TimerError>,
    initialize: impl FnOnce(
        &mut TimerRegistry,
        &RegistrationClaim,
        Prepared,
    ) -> Result<Option<(RegistryTransition, ProviderHandles)>, TimerError>,
) -> Result<RegistrationClaim, TimerError> {
    let (claim, initialized) = with_registry_mut(|registry| {
        let (claim, prepared) = register(registry)?;
        let initialized = initialize(registry, &claim, prepared);
        Ok((claim, initialized))
    })?;
    let applied = initialized.and_then(|transition| {
        transition.map_or(Ok(()), |(transition, handles)| {
            finish_transition(transition, handles)
        })
    });
    applied.map_err(|error| cleanup_creation_error(&claim, error))?;
    Ok(claim)
}

/// Fresh lifecycle declarations use the same initialization transaction for all policies.
fn register_reconciled(
    register: impl FnOnce(&mut TimerRegistry) -> Result<RegistrationClaim, RegisterError>,
    reconcile: impl FnOnce(
        &mut TimerRegistry,
        &RegistrationClaim,
    ) -> Result<RegistryTransition, RegistryError>,
) -> Result<RegistrationClaim, TimerError> {
    register_initialized(
        |registry| Ok((register(registry)?, ())),
        |registry, claim, ()| {
            registry
                .transition_claim(claim, |registry| reconcile(registry, claim))
                .map(Some)
                .map_err(TimerError::from)
        },
    )
}

fn cleanup_creation_error(claim: &RegistrationClaim, error: TimerError) -> TimerError {
    match unregister_claim(claim) {
        Ok(()) | Err(TimerError::RegistrationExpired) => error,
        Err(cleanup_error) => cleanup_error,
    }
}

/// Reconstruct or reconcile one `Once` declaration synchronously.
///
/// `Some(schedule)` is authoritative and may move an existing deadline in
/// either direction. `None` retains an inactive declaration in the canonical
/// inventory, including on a fresh heap. Lifecycle reconciliation always owns
/// a [`TimerStart::Retained`] declaration; transient
/// `RemoveWhenStopped` callbacks use [`register_once`] directly.
/// An empty owner slot is filled only after initial reconciliation succeeds.
/// A returned error cleans up the new declaration and captures, leaving that
/// slot empty. Later errors preserve an existing registration's authority.
///
/// Lifecycle reconstruction retains the same policy-specific result boundary:
///
/// ```compile_fail,E0271
/// use ic_timers::{reconcile_once, TimerIdentity, TimerCompletion,
///     AfterCompletionDecision, AfterCompletionRunResult};
/// let identity = TimerIdentity::try_new("app", "jobs", "once").unwrap();
/// let mut registration = None;
/// let _ = reconcile_once(&mut registration, &identity, None, |_| async {
///     AfterCompletionRunResult::new(TimerCompletion::no_work(), AfterCompletionDecision::Stop)
/// });
/// ```
pub fn reconcile_once<F, Fut>(
    registration: &mut Option<OnceRegistration>,
    identity: &TimerIdentity,
    desired: Option<TimerSchedule>,
    callback: F,
) -> Result<(), TimerError>
where
    F: FnMut(OnceContext) -> Fut + 'static,
    Fut: Future<Output = OnceRunResult> + 'static,
{
    let reconcile = |registry: &mut TimerRegistry, claim: &RegistrationClaim| {
        registry.reconcile_ordinary(claim, platform::time_ns(), desired)
    };
    reconcile_registration(
        registration,
        identity,
        TimerPolicy::Once,
        |current: &OnceRegistration| &current.claim,
        || {
            let callback =
                erase_ordinary_callback(callback, OnceContext::new, OrdinaryRunResult::from);
            let claim = register_reconciled(
                |registry| {
                    registry.register_once_with_callback(
                        identity.clone(),
                        DeclarationLifetime::Retained,
                        Rc::clone(&callback),
                    )
                },
                reconcile,
            );
            drop(callback);
            Ok(OnceRegistration { claim: claim? })
        },
        |registration| {
            apply_claim_transition(Declared(&registration.claim), |registry, claim| {
                reconcile(registry, claim)
            })
        },
    )
}

/// Reconstruct or reconcile one after-completion declaration synchronously.
///
/// The consumer owns `registration` in volatile state. A fresh Wasm heap has
/// `None`, so this function creates callback authority and publishes it only
/// after initial active or inactive reconciliation succeeds. A returned error
/// cleans up a fresh declaration, leaving the slot empty. Later errors retain
/// existing authority. A repeated call reuses the exact claim and does not
/// replace its callback. The installed declaration is always retained;
/// transient `RemoveWhenStopped` recurrence uses
/// [`register_after_completion`] directly.
///
/// Lifecycle reconstruction also requires [`AfterCompletionRunResult`]:
///
/// ```compile_fail,E0271
/// use ic_timers::{reconcile_after_completion, TimerIdentity, TimerCadence,
///     TimerReconcileState, TimerCompletion, OnceDecision, OnceRunResult};
/// let identity = TimerIdentity::try_new("app", "jobs", "recurring").unwrap();
/// let cadence = TimerCadence::from_nanos(5).unwrap();
/// let mut registration = None;
/// let _ = reconcile_after_completion(&mut registration, &identity, cadence,
///     TimerReconcileState::Inactive, |_| async {
///         OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
///     });
/// ```
pub fn reconcile_after_completion<F, Fut>(
    registration: &mut Option<AfterCompletionRegistration>,
    identity: &TimerIdentity,
    cadence: TimerCadence,
    desired: TimerReconcileState,
    callback: F,
) -> Result<(), TimerError>
where
    F: FnMut(AfterCompletionContext) -> Fut + 'static,
    Fut: Future<Output = AfterCompletionRunResult> + 'static,
{
    let reconcile = |registry: &mut TimerRegistry, claim: &RegistrationClaim| match desired {
        TimerReconcileState::Inactive => registry.cancel(claim),
        TimerReconcileState::Scheduled => registry.ensure_recurring(claim, platform::time_ns()),
    };
    reconcile_registration(
        registration,
        identity,
        TimerPolicy::AfterCompletion { cadence },
        |current: &AfterCompletionRegistration| &current.claim,
        || {
            let callback = erase_ordinary_callback(
                callback,
                AfterCompletionContext::new,
                OrdinaryRunResult::from,
            );
            let claim = register_reconciled(
                |registry| {
                    registry.register_after_completion_with_callback(
                        identity.clone(),
                        cadence,
                        DeclarationLifetime::Retained,
                        Rc::clone(&callback),
                    )
                },
                reconcile,
            );
            drop(callback);
            Ok(AfterCompletionRegistration { claim: claim? })
        },
        |registration| {
            apply_claim_transition(Declared(&registration.claim), |registry, claim| {
                reconcile(registry, claim)
            })
        },
    )
}

/// Reconstruct or reconcile one watchdog declaration synchronously.
///
/// Durable readiness remains consumer-owned. Fresh inactive authority still
/// installs an observable retained declaration. `ScheduledImmediately` arms
/// its first scheduler at deadline now without synchronous consumer work.
/// `ScheduledAt` reconciles an exact deadline, including moving it later.
/// A confirmed abandoned scheduler is rearmed even for an unchanged deadline.
/// Still-dispatched work retains its attempt and committed recovery cadence;
/// this helper does not authorize replay of uncertain application effects.
/// This helper owns no lifecycle export and persists no policy, generation,
/// provider handle, or callback.
/// Transient `RemoveWhenStopped` watchdogs use [`register_watchdog`] directly.
/// An empty owner slot is filled only after its initial state succeeds. A
/// returned error cleans up the new declaration and captures, leaving the slot
/// empty. Later errors retain existing authority and its failure observations.
pub fn reconcile_watchdog<F>(
    registration: &mut Option<WatchdogRegistration>,
    identity: &TimerIdentity,
    cadence: TimerCadence,
    desired: WatchdogReconcileState,
    callback: F,
) -> Result<(), TimerError>
where
    F: FnMut(WatchdogContext) -> WatchdogRunResult + 'static,
{
    let reconcile = |registry: &mut TimerRegistry, claim: &RegistrationClaim| match desired {
        WatchdogReconcileState::Inactive => registry.cancel(claim),
        WatchdogReconcileState::Scheduled => registry.ensure_recurring(claim, platform::time_ns()),
        WatchdogReconcileState::ScheduledImmediately => {
            registry.ensure_watchdog_immediately(claim, platform::time_ns())
        }
        WatchdogReconcileState::ScheduledAt(deadline_ns) => registry.reconcile_watchdog_schedule(
            claim,
            platform::time_ns(),
            Some(TimerSchedule::At(deadline_ns)),
        ),
    };
    reconcile_registration(
        registration,
        identity,
        TimerPolicy::Watchdog { cadence },
        |current: &WatchdogRegistration| &current.claim,
        || {
            let mut callback = callback;
            let callback: WatchdogCallback = Rc::new(RefCell::new(move |token| {
                callback(WatchdogContext::new(token))
            }));
            let claim = register_reconciled(
                |registry| {
                    registry.register_watchdog_with_callback(
                        identity.clone(),
                        cadence,
                        DeclarationLifetime::Retained,
                        Rc::clone(&callback),
                    )
                },
                reconcile,
            );
            drop(callback);
            Ok(WatchdogRegistration { claim: claim? })
        },
        |registration| {
            apply_claim_transition(Declared(&registration.claim), |registry, claim| {
                reconcile(registry, claim)
            })
        },
    )
}

fn reconcile_registration<Registration>(
    registration: &mut Option<Registration>,
    identity: &TimerIdentity,
    policy: TimerPolicy,
    claim: impl FnOnce(&Registration) -> &RegistrationClaim,
    register: impl FnOnce() -> Result<Registration, TimerError>,
    reconcile: impl FnOnce(&Registration) -> Result<(), TimerError>,
) -> Result<(), TimerError> {
    let Some(current) = registration.as_ref() else {
        *registration = Some(register()?);
        return Ok(());
    };
    let claim = claim(current);
    if claim.identity() != identity {
        return Err(TimerError::ReconciliationConflict);
    }
    with_registry(|registry| {
        registry
            .declaration_matches(claim, policy, DeclarationLifetime::Retained)
            .map_err(TimerError::from)?
            .then_some(())
            .ok_or(TimerError::ReconciliationConflict)
    })?;
    // Repeated calls retain the original callback. Release the unused factory
    // outside the registry borrow, before applying control as in prior calls.
    drop(register);
    reconcile(current)
}

/// Return one coherent inert snapshot by identity.
pub fn timer_snapshot(identity: &TimerIdentity) -> Result<Option<TimerSnapshot>, TimerError> {
    with_registry(|registry| Ok(registry.snapshot(identity)))
}

/// Return one atomic bounded inventory with its volatile runtime epoch.
///
/// Timer snapshots are ordered deterministically by identity. An initialized
/// empty registry still returns its epoch and an empty timer slice.
pub fn timer_inventory() -> Result<TimerInventorySnapshot, TimerError> {
    with_registry(|registry| Ok(registry.inventory()))
}

/// Return functional expected-failure state by identity without a full inventory.
pub fn consecutive_expected_failures(identity: &TimerIdentity) -> Result<Option<u64>, TimerError> {
    with_registry(|registry| Ok(registry.consecutive_expected_failures(identity)))
}

fn has_armed_wakeup_claim(claim: &RegistrationClaim) -> Result<bool, TimerError> {
    with_registry(|registry| registry.has_armed_wakeup(claim).map_err(TimerError::from))
}

fn erase_ordinary_callback<Context: 'static, Output: 'static, F, Fut>(
    mut callback: F,
    context: fn(CallbackToken) -> Context,
    erase_result: fn(Output) -> OrdinaryRunResult,
) -> OrdinaryCallback
where
    F: FnMut(Context) -> Fut + 'static,
    Fut: Future<Output = Output> + 'static,
{
    Rc::new(RefCell::new(move |token| {
        let future = callback(context(token));
        Box::pin(async move { erase_result(future.await) }) as _
    }))
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

#[expect(
    clippy::future_not_send,
    reason = "IC callbacks and canister-local state are single-threaded."
)]
fn dispatch_wakeup(token: WakeupToken) -> impl Future<Output = ()> {
    // Construct outside the async body: a provider rejection can discard the
    // future before its first poll, after rolling registry acceptance back.
    let mut delivery = WakeupDelivery {
        token,
        completed: false,
    };
    async move {
        match &delivery.token {
            WakeupToken::Ordinary(token) => dispatch_ordinary(token).await,
            WakeupToken::Scheduler(token) => dispatch_watchdog_scheduler(token),
        }
        delivery.completed = true;
        // Capture the whole guard until normal completion.
        drop(delivery);
    }
}

struct WakeupDelivery {
    token: WakeupToken,
    completed: bool,
}

impl Drop for WakeupDelivery {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        let removed = RUNTIME.with(|runtime| {
            let Ok(mut runtime) = runtime.try_borrow_mut() else {
                // Rejected binding may clear a detached scheduled handle
                // while another shared borrow exists. Its transition owns
                // cleanup; do not turn that typed failure into a Drop trap.
                let runtime = runtime.try_borrow().map_err(|_| TimerError::RuntimeBusy)?;
                return if runtime.as_ref().is_some_and(|registry| match &self.token {
                    WakeupToken::Ordinary(token) => registry.owns_ordinary_delivery(token),
                    WakeupToken::Scheduler(token) => registry.owns_scheduler_delivery(token),
                }) {
                    Err(TimerError::RuntimeBusy)
                } else {
                    Ok(None)
                };
            };
            Ok(runtime.as_mut().and_then(|registry| match &self.token {
                WakeupToken::Ordinary(token) => {
                    registry.abandon_ordinary(token, platform::time_ns())
                }
                WakeupToken::Scheduler(token) => {
                    registry.abandon_scheduler(token);
                    None
                }
            }))
        });
        let removed = removed
            .unwrap_or_else(|error| trap_callback_failure("wakeup delivery abandonment", &error));
        drop(removed);
    }
}

#[expect(
    clippy::future_not_send,
    reason = "IC callbacks and canister-local state are single-threaded."
)]
async fn dispatch_ordinary(token: &CallbackToken) {
    let measurement = CallbackMeasurementStart::capture();
    let callback = with_registry_mut(|registry| Ok(registry.begin_ordinary(token)));
    let callback = match callback {
        Ok(Some(callback)) => callback,
        Ok(None) => return,
        Err(error) => trap_callback_failure("ordinary callback acceptance", &error),
    };
    let future = {
        let Ok(mut callback) = callback.try_borrow_mut() else {
            finish_ordinary_callback(
                token,
                OrdinaryRunResult::new(
                    TimerCompletion::invariant_failure(0),
                    OrdinaryDirective::Stop,
                ),
            );
            return;
        };
        callback(token.clone())
    };
    let result = future.await;
    finish_ordinary_callback(token, result);
    measurement.record(token);
}

fn finish_ordinary_callback(token: &CallbackToken, result: OrdinaryRunResult) {
    let transition = with_registry_mut(|registry| {
        registry
            .complete_ordinary(token, platform::time_ns(), result)
            .map_err(TimerError::from)
    });
    let transition = transition
        .unwrap_or_else(|error| trap_callback_failure("ordinary callback completion", &error));
    finish_callback_transition(token, transition, ProviderHandles::default());
}

fn dispatch_watchdog_scheduler(token: &CallbackToken) {
    let measurement = CallbackMeasurementStart::capture();
    let dispatched = with_registry_mut(|registry| {
        Ok(registry.transition_watchdog_scheduler(token, platform::time_ns()))
    });
    let (transition, handles) = dispatched
        .unwrap_or_else(|error| trap_callback_failure("watchdog scheduler transition", &error));
    let accepted = !matches!(transition.effect(), RegistryEffect::None);
    finish_callback_transition(token, transition, handles);
    if accepted {
        measurement.record(token);
    }
}

fn dispatch_watchdog_work(token: &CallbackToken) {
    let measurement = CallbackMeasurementStart::capture();
    let callback = with_registry_mut(|registry| Ok(registry.begin_watchdog_work(token)));
    let callback = match callback {
        Ok(Some(callback)) => callback,
        Ok(None) => return,
        Err(error) => trap_callback_failure("watchdog work acceptance", &error),
    };
    let result = {
        let Ok(mut callback) = callback.try_borrow_mut() else {
            trap_callback_failure(
                "watchdog callback ownership",
                &TimerError::OwnershipInvariant,
            );
        };
        callback(token.clone())
    };
    finish_watchdog_dispatch(token, result);
    measurement.record(token);
}

fn finish_watchdog_dispatch(token: &CallbackToken, result: WatchdogRunResult) {
    // Unlike synchronous public control, an unexpected callback-completion
    // failure must trap. IC message rollback restores the transition and any
    // replaced successor committed by the earlier scheduler message.
    let completed = with_registry_mut(|registry| {
        #[cfg(test)]
        {
            if take_watchdog_completion_fault() {
                return Err(TimerError::OwnershipInvariant);
            }
        }
        let mut transition = registry
            .complete_watchdog_work(token, platform::time_ns(), result)
            .map_err(TimerError::from)?;
        let handles = registry.detach_transition_handles(token.identity(), &mut transition);
        Ok((transition, handles))
    });
    let (transition, handles) =
        completed.unwrap_or_else(|error| trap_callback_failure("watchdog work completion", &error));
    finish_callback_transition(token, transition, handles);
}

fn finish_callback_transition(
    token: &CallbackToken,
    transition: RegistryTransition,
    handles: ProviderHandles,
) {
    // Registry control failures are already finalized. Other binding failures
    // follow the callback role's cleanup or rollback rule.
    match finish_transition(transition, handles) {
        Ok(()) | Err(TimerError::ControlFailure(_)) => {}
        Err(error) => {
            if token.role() == CallbackRole::WatchdogWork {
                trap_callback_failure("watchdog provider-handle completion", &error);
            }
            fail_claim_provider_binding(token.claim()).unwrap_or_else(|binding_error| {
                trap_callback_failure("provider-binding failure cleanup", &binding_error)
            });
        }
    }
}

fn fail_claim_provider_binding(claim: &RegistrationClaim) -> Result<(), TimerError> {
    let failed = with_registry_mut(|registry| {
        registry
            .fail_registration(claim, TimerControlFailure::ProviderBindingFailed)
            .map_err(TimerError::from)
    });
    let (handles, transition) = failed?;
    clear_provider_handles(handles);
    drop(transition);
    Ok(())
}

#[derive(Clone, Copy)]
struct CallbackMeasurementStart {
    instructions_before: u64,
    memory_start: MemoryPageExtent,
}

impl CallbackMeasurementStart {
    fn capture() -> Self {
        // Keep page observation outside the established instruction interval.
        let memory_start = platform::memory_pages();
        let instructions_before = platform::instruction_counter();
        Self {
            instructions_before,
            memory_start,
        }
    }

    fn record(self, token: &CallbackToken) {
        // Close the instruction interval before taking its paired end extent.
        let instructions = platform::instruction_counter().saturating_sub(self.instructions_before);
        let memory_end = platform::memory_pages();
        with_registry_mut(|registry| {
            registry
                .record_callback_measurements(token, instructions, self.memory_start, memory_end)
                .map_err(TimerError::from)
        })
        .unwrap_or_else(|error| trap_callback_failure("callback measurement accounting", &error));
    }
}

fn trap_callback_failure(context: &str, error: &TimerError) -> ! {
    platform::trap(&format!("ic-timers {context} failed: {error}"))
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
