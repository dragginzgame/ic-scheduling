//! IC entrypoints and callback registration, authored from the system API contract.
//!
//! Callback environments are bounded table indices, never raw pointers. The IC
//! delivers one response or (after response rollback) cleanup per successful call.
//! A failed submission retains ownership here. No index is reused before that
//! terminal event. Message rollback restores the table if a callback traps.
#![expect(
    unsafe_code,
    reason = "Only this boundary exports IC entrypoints and registers unsafe response/cleanup callbacks."
)]

use super::{
    DISPATCH_METHOD,
    backend::{self, DISPATCH_LIMIT},
};

// SAFETY: one timer owner per final Wasm; the boundary checker confines exports
// here. The IC invokes this no-argument entrypoint in global-timer context.
#[unsafe(export_name = "canister_global_timer")]
extern "C" fn global_timer() {
    ic_platform_runtime::executor::in_tracking_executor_context(|| {
        let now = ic_platform_runtime::ic0::time();
        let mut principal = [0; 29];
        let len = ic_platform_runtime::ic0::canister_self_size();
        ic_platform_runtime::ic0::canister_self_copy(&mut principal[..len], 0);
        for _ in 0..DISPATCH_LIMIT {
            let Some(id) = backend::due(now) else {
                break;
            };
            let Some(slot) = backend::reserve(id) else {
                backend::defer(now);
                break;
            };
            // Reserve memory before checking liquid cycles. There are no further
            // allocations between this check and submitting the call.
            if ic_platform_runtime::ic0::canister_liquid_cycle_balance128()
                < ic_platform_runtime::ic0::cost_call(DISPATCH_METHOD.len() as u64, 8)
            {
                drop(backend::take_flight(slot));
                backend::defer(now);
                break;
            }
            // SAFETY: both callbacks receive the same live table slot. Exactly
            // one terminal path consumes it; nonzero call_perform consumes it
            // locally, and callback rollback restores it for cleanup.
            unsafe {
                ic_platform_runtime::ic0::call_new(
                    &principal[..len],
                    DISPATCH_METHOD,
                    response,
                    slot,
                    response,
                    slot,
                );
                ic_platform_runtime::ic0::call_on_cleanup(cleanup, slot);
            }
            ic_platform_runtime::ic0::call_data_append(&id.to_le_bytes());
            ic_platform_runtime::ic0::call_with_best_effort_response(300);
            if ic_platform_runtime::ic0::call_perform() != 0 {
                drop(backend::take_flight(slot));
                backend::defer(now);
                break;
            }
            backend::submitted(id);
        }
        backend::rearm();
    });
}

// SAFETY: this export is owned only by this backend; callers are admitted before
// any state access. Native builds use an ordinary symbol, avoiding spaces.
#[cfg_attr(
    target_family = "wasm",
    unsafe(export_name = "canister_update <ic-timers internal> dispatch")
)]
#[cfg_attr(
    not(target_family = "wasm"),
    unsafe(export_name = "ic_timers_internal_dispatch")
)]
extern "C" fn dispatch() {
    ic_platform_runtime::executor::in_tracking_executor_context(|| {
        let mut caller = [0; 29];
        let mut own = [0; 29];
        let own_len = ic_platform_runtime::ic0::canister_self_size();
        if ic_platform_runtime::ic0::msg_caller_size() != own_len {
            ic_platform_runtime::ic0::trap(b"ic-timers: self-call required");
        }
        ic_platform_runtime::ic0::msg_caller_copy(&mut caller[..own_len], 0);
        ic_platform_runtime::ic0::canister_self_copy(&mut own[..own_len], 0);
        if caller[..own_len] != own[..own_len] {
            ic_platform_runtime::ic0::trap(b"ic-timers: self-call required");
        }
        if ic_platform_runtime::ic0::msg_arg_data_size() != 8 {
            ic_platform_runtime::ic0::trap(b"ic-timers: invalid dispatch payload");
        }
        let mut bytes = [0; 8];
        ic_platform_runtime::ic0::msg_arg_data_copy(&mut bytes, 0);
        let task = backend::remove(u64::from_le_bytes(bytes));
        backend::rearm();
        if let Some(task) = task {
            ic_platform_runtime::executor::spawn_protected(async move {
                task.await;
                ic_platform_runtime::ic0::msg_reply();
            });
        } else {
            ic_platform_runtime::ic0::msg_reply();
        }
    });
}

// SAFETY: callable only with a live FLIGHTS slot supplied by global_timer.
unsafe extern "C" fn response(slot: usize) {
    let flight = backend::take_flight(slot);
    ic_platform_runtime::executor::in_callback_executor_context_for(flight.method, || {
        match ic_platform_runtime::ic0::msg_reject_code() {
            2 | 6 => backend::retry(flight.task),
            _ => {
                drop(backend::remove(flight.task));
            }
        }
        backend::rearm();
    });
}

// SAFETY: response rollback restores the live slot before the IC invokes this
// cleanup. The method handle is consumed through the executor's recovery API.
unsafe extern "C" fn cleanup(slot: usize) {
    let flight = backend::take_flight(slot);
    ic_platform_runtime::executor::in_trap_recovery_context_for(flight.method, || {
        ic_platform_runtime::executor::cancel_all_tasks_attached_to_current_method();
        backend::retry(flight.task);
        backend::rearm();
    });
}
