//! Timers for isolated Internet Computer work.
//!
//! The wasm32 API provides async Once and synchronous prearmed watchdog callbacks,
//! with explicit retained or remove-on-stop lifetimes. Abandoned Once work requires
//! owner reconciliation before another attempt. Hosts own upgrade reconstruction.
//! Opt-in `metrics` exposes inert aggregate counters and bounded declaration
//! inventory; observations never authorize scheduling or business-effect replay.

// The IC provider is wasm32-only; native builds retain the pure engine tests.
mod api;
#[cfg_attr(
    all(not(test), not(target_arch = "wasm32")),
    expect(dead_code, reason = "native build has no IC transport")
)]
mod bindings;
#[cfg(feature = "metrics")]
mod diagnostics;
#[cfg_attr(
    all(not(test), not(target_arch = "wasm32")),
    expect(dead_code, reason = "native build has no IC transport")
)]
mod engine;
#[cfg_attr(
    all(feature = "metrics", not(test), not(target_arch = "wasm32")),
    expect(dead_code, reason = "native build has no IC transport")
)]
mod metrics;
#[cfg(target_arch = "wasm32")]
mod transport;
#[cfg_attr(
    all(not(test), not(target_arch = "wasm32")),
    expect(dead_code, reason = "native build has no IC transport")
)]
mod watchdog;

pub use api::{
    Error, Lifetime, Observation, OnceDemand, OnceNext, OnceResult, WatchdogDemand, WatchdogNext,
    WatchdogResult,
};
#[cfg(feature = "metrics")]
pub use diagnostics::{Inventory, TimerKind, TimerMeasurements, TimerPhase, TimerSnapshot};
#[cfg(feature = "metrics")]
pub use metrics::Metrics;
#[cfg(all(feature = "metrics", target_arch = "wasm32"))]
pub use transport::inventory;
#[cfg(all(feature = "metrics", target_arch = "wasm32"))]
pub use transport::metrics;

#[cfg(target_arch = "wasm32")]
pub use transport::{
    Once, OnceStatus, RecoveryToken, Watchdog, WatchdogStatus, bind_once, bind_watchdog, initialize,
};
