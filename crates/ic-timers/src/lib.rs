//! Retained timers for isolated Internet Computer work.
//!
//! The wasm32 API provides retained Once and synchronous watchdog callbacks.
//! Local installed tests cover Once recovery and initial watchdog prearm safety.
//! Metrics, measured pressure limits and remaining lifecycle qualification are unfinished.

// The IC provider is wasm32-only; native builds retain the pure engine tests.
#[cfg_attr(
    all(not(test), not(target_arch = "wasm32")),
    expect(dead_code, reason = "native build has no IC transport")
)]
mod engine;

#[cfg_attr(
    all(not(test), not(target_arch = "wasm32")),
    expect(dead_code, reason = "native build has no IC transport")
)]
mod watchdog;

#[cfg_attr(
    all(not(test), not(target_arch = "wasm32")),
    expect(dead_code, reason = "native build has no IC transport")
)]
mod bindings;

mod api;
pub use api::{
    Error, Observation, OnceDemand, OnceNext, OnceResult, WatchdogDemand, WatchdogNext,
    WatchdogResult,
};

#[cfg(target_arch = "wasm32")]
mod transport;
#[cfg(target_arch = "wasm32")]
pub use transport::{
    Once, OnceStatus, RecoveryToken, Watchdog, WatchdogStatus, bind_once, bind_watchdog, initialize,
};
