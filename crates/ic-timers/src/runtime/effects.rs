//! Apply registry effects after clearing their selected linear provider handles.
//! Captures are released only after the registry borrow and provider cleanup end.

#[cfg(test)]
use super::take_provider_install_fault;
use super::{
    CallbackToken, ProviderBinding, ProviderHandles, RUNTIME, RegistryEffect, RegistryTransition,
    TimerError, TimerHandle, TimerIdentity, WakeupToken, dispatch_wakeup, dispatch_watchdog_work,
    platform, with_registry_mut,
};
use std::time::Duration;

pub(super) fn finish_transition(
    transition: RegistryTransition,
    handles: ProviderHandles,
) -> Result<(), TimerError> {
    let failure = transition.failure();
    let applied = apply_effect(transition.effect(), handles);
    // Callback capture destructors may inspect or control other timers. Drop
    // them after both the registry borrow and provider cleanup have ended.
    drop(transition);
    applied?;
    failure.map_or(Ok(()), |failure| Err(TimerError::ControlFailure(failure)))
}

pub(super) fn apply_effect(
    effect: &RegistryEffect,
    handles: ProviderHandles,
) -> Result<(), TimerError> {
    // The registry leaves unaffected handles installed. Every detached handle
    // here is selected for cleanup, including all handles of a removed entry.
    clear_provider_handles(handles);
    match effect {
        RegistryEffect::None | RegistryEffect::ClearCallbacks { .. } => Ok(()),
        RegistryEffect::ArmWakeup {
            token, delay_ns, ..
        } => {
            let handle = platform::try_set_timer(
                Duration::from_nanos(*delay_ns),
                dispatch_wakeup(token.clone()),
            )?;
            bind_provider_handle(
                token.token(),
                handle,
                ProviderBinding::Commit {
                    delay_ns: *delay_ns,
                },
            )
        }
        RegistryEffect::DispatchWatchdog {
            dispatch,
            successor_delay_ns,
        } => {
            let successor = dispatch.successor();
            let work = dispatch.work();
            let successor_handle = platform::try_set_timer(
                Duration::from_nanos(*successor_delay_ns),
                dispatch_wakeup(WakeupToken::Scheduler(successor.clone())),
            )?;
            bind_provider_handle(successor, successor_handle, ProviderBinding::Stage)?;

            let work_token = work.clone();
            let work_handle = platform::try_set_timer(Duration::ZERO, async move {
                dispatch_watchdog_work(&work_token);
            })?;
            if let Err(error) = bind_provider_handle(
                &work,
                work_handle,
                ProviderBinding::Commit {
                    delay_ns: *successor_delay_ns,
                },
            ) {
                clear_entry_provider_handles(successor.identity())?;
                return Err(error);
            }
            Ok(())
        }
    }
}

/// Consume a provider handle by installing it or clearing it on rejection.
pub(super) fn bind_provider_handle(
    token: &CallbackToken,
    handle: TimerHandle,
    binding: ProviderBinding,
) -> Result<(), TimerError> {
    RUNTIME
        .with(|runtime| {
            #[cfg(test)]
            if take_provider_install_fault() {
                return Err((TimerError::OwnershipInvariant, handle));
            }
            let Ok(mut runtime) = runtime.try_borrow_mut() else {
                return Err((TimerError::RuntimeBusy, handle));
            };
            let Some(registry) = runtime.as_mut() else {
                return Err((TimerError::NotInitialized, handle));
            };
            match registry.install_provider_handle(token, handle, binding) {
                Ok(()) => Ok(()),
                Err((error, handle)) => Err((TimerError::from(error), handle)),
            }
        })
        .map_err(|(error, handle)| {
            // Release the registry borrow before calling the provider.
            platform::clear_timer(handle);
            error
        })
}

pub(super) fn clear_provider_handles(mut handles: ProviderHandles) {
    if let Some(wakeup) = handles.take_wakeup() {
        platform::clear_timer(wakeup);
    }
    if let Some(work) = handles.take_work() {
        platform::clear_timer(work);
    }
}

pub(super) fn clear_entry_provider_handles(identity: &TimerIdentity) -> Result<(), TimerError> {
    let handles = with_registry_mut(|registry| Ok(registry.take_provider_handles(identity)))?;
    clear_provider_handles(handles);
    Ok(())
}
