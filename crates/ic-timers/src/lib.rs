//! Retained timers for isolated Internet Computer work.
//!
//! The wasm32 API provides retained Once and synchronous watchdog callbacks.
//! Local installed tests cover Once recovery and initial watchdog prearm safety.
//! Opt-in `metrics` exposes inert aggregate counters; hosts own their export.
//! Measured pressure limits and remaining lifecycle qualification are unfinished.

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

#[cfg_attr(
    all(feature = "metrics", not(test), not(target_arch = "wasm32")),
    expect(dead_code, reason = "native build has no IC transport")
)]
mod metrics;
pub use api::{
    Error, Observation, OnceDemand, OnceNext, OnceResult, WatchdogDemand, WatchdogNext,
    WatchdogResult,
};
#[cfg(feature = "metrics")]
pub use metrics::Metrics;
#[cfg(all(feature = "metrics", target_arch = "wasm32"))]
pub use transport::metrics;

#[cfg(target_arch = "wasm32")]
mod transport;
#[cfg(target_arch = "wasm32")]
pub use transport::{
    Once, OnceStatus, RecoveryToken, Watchdog, WatchdogStatus, bind_once, bind_watchdog, initialize,
};
