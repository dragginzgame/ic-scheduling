//! Callback delivery, abandonment, completion and paired measurement.
//! Provider effects are applied after releasing registry borrows.

#[cfg(test)]
use super::take_watchdog_completion_fault;
use super::{
    CallbackRole, CallbackToken, MemoryPageExtent, OrdinaryDirective, OrdinaryRunResult,
    ProviderHandles, RUNTIME, RegistrationClaim, RegistryEffect, RegistryTransition,
    TimerCompletion, TimerControlFailure, TimerError, WakeupToken, WatchdogRunResult,
    clear_provider_handles, finish_transition, platform, with_registry_mut,
};
use std::{future::Future, mem::DropGuard};

pub(super) fn dispatch_wakeup(token: WakeupToken) -> impl Future<Output = ()> {
    // Construct outside the async body: a provider rejection can discard the
    // future before its first poll, after rolling registry acceptance back.
    let delivery = DropGuard::new(token, |token| abandon_wakeup(&token));
    async move {
        match &*delivery {
            WakeupToken::Ordinary(token) => dispatch_ordinary(token).await,
            WakeupToken::Scheduler(token) => dispatch_watchdog_scheduler(token),
        }
        drop(DropGuard::dismiss(delivery));
    }
}

fn abandon_wakeup(token: &WakeupToken) {
    let removed = RUNTIME.with(|runtime| {
        let Ok(mut runtime) = runtime.try_borrow_mut() else {
            // Rejected binding may clear a detached scheduled handle
            // while another shared borrow exists. Its transition owns
            // cleanup; do not turn that typed failure into a Drop trap.
            let runtime = runtime.try_borrow().map_err(|_| TimerError::RuntimeBusy)?;
            return if runtime.as_ref().is_some_and(|registry| match token {
                WakeupToken::Ordinary(token) => registry.owns_ordinary_delivery(token),
                WakeupToken::Scheduler(token) => registry.owns_scheduler_delivery(token),
            }) {
                Err(TimerError::RuntimeBusy)
            } else {
                Ok(None)
            };
        };
        Ok(runtime.as_mut().and_then(|registry| match token {
            WakeupToken::Ordinary(token) => registry.abandon_ordinary(token, platform::time_ns()),
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

pub(super) fn dispatch_watchdog_scheduler(token: &CallbackToken) {
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

pub(super) fn dispatch_watchdog_work(token: &CallbackToken) {
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

pub(super) fn fail_claim_provider_binding(claim: &RegistrationClaim) -> Result<(), TimerError> {
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
pub(super) struct CallbackMeasurementStart {
    pub(super) instructions_before: u64,
    pub(super) memory_start: MemoryPageExtent,
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

    pub(super) fn record(self, token: &CallbackToken) {
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
