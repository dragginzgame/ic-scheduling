//! Private boundary to IC system facts and owned one-shot dispatch.
//!
//! Recurrence, identity, arbitration, inventory, and consumer work belong to
//! the canonical runtime above this module.

#![expect(
    clippy::redundant_pub_crate,
    reason = "Crate visibility forbids indirect public re-exports of platform authority."
)]

use crate::snapshot::MemoryPageExtent;
pub(crate) const DISPATCH_METHOD: &str = "<ic-timers internal> dispatch";

#[cfg(not(test))]
mod backend;
#[cfg(not(test))]
mod ffi;
mod ownership;
mod queue;
mod slots;

#[cfg(not(test))]
pub(crate) use backend::{TimerHandle, clear_timer, try_set_timer};
pub(crate) use queue::AdmissionError;

/// Return current IC message time in nanoseconds.
#[cfg(not(test))]
pub(crate) fn time_ns() -> u64 {
    ic_platform_runtime::ic0::time()
}

/// Return the current canister version.
#[cfg(not(test))]
pub(crate) fn canister_version() -> u64 {
    ic_platform_runtime::ic0::canister_version()
}

/// Return call-context instruction consumption on the IC.
#[cfg(not(test))]
pub(crate) fn instruction_counter() -> u64 {
    // Native tests own their fake; production retains the IC binding contract.
    ic_platform_runtime::ic0::performance_counter(1)
}

/// Return current Wasm and stable memory extents in 64 KiB pages without
/// allocation.
#[cfg(not(test))]
pub(crate) fn memory_pages() -> MemoryPageExtent {
    #[cfg(target_arch = "wasm32")]
    let wasm = std::arch::wasm32::memory_size::<0>() as u64;
    #[cfg(not(target_arch = "wasm32"))]
    let wasm = 0;

    MemoryPageExtent::new(wasm, ic_platform_runtime::ic0::stable64_size())
}

/// Abort the current message when an internal callback invariant is violated.
#[cfg(not(test))]
pub(crate) fn trap(message: &str) -> ! {
    ic_platform_runtime::ic0::trap(message.as_bytes())
}

#[cfg(test)]
mod tests;
#[cfg(test)]
pub(crate) use tests::{
    ADMISSION_FAULT, TimerHandle, advance_instructions, canister_version, clear_tasks, clear_timer,
    discard_next_due, discard_timer_at, grow_memory_pages, instruction_counter, memory_pages,
    reset, run_next_due, set_time, set_timer, time_ns, timer_count, trap, try_set_timer,
};
