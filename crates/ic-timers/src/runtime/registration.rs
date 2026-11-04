//! Direct creation and retained lifecycle reconciliation.
//! Initial provider binding and capture release complete before authority escapes.

use super::{
    AfterCompletionContext, AfterCompletionRegistration, AfterCompletionRunResult, CallbackToken,
    DeclarationLifetime, Declared, OnceContext, OnceRegistration, OnceRunResult, OrdinaryCallback,
    OrdinaryRunResult, ProviderHandles, RegisterError, RegistrationClaim, RegistryError,
    RegistryTransition, TimerCadence, TimerError, TimerIdentity, TimerPolicy, TimerReconcileState,
    TimerRegistry, TimerSchedule, TimerStart, WatchdogCallback, WatchdogContext,
    WatchdogReconcileState, WatchdogRegistration, WatchdogRunResult, apply_claim_transition,
    finish_transition, platform, unregister_claim, with_registry, with_registry_mut,
};
use std::{cell::RefCell, future::Future, rc::Rc};

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
    let callback = erase_watchdog_callback(callback);
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
            let callback = erase_watchdog_callback(callback);
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

fn erase_watchdog_callback(
    mut callback: impl FnMut(WatchdogContext) -> WatchdogRunResult + 'static,
) -> WatchdogCallback {
    Rc::new(RefCell::new(move |token| {
        callback(WatchdogContext::new(token))
    }))
}

pub(super) fn erase_ordinary_callback<Context: 'static, Output: 'static, F, Fut>(
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
