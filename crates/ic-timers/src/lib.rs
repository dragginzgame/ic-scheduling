//! Timers for isolated Internet Computer work.
//!
//! The installed runtime provides async Once and synchronous prearmed watchdog callbacks,
//! with explicit retained or remove-on-stop lifetimes. Abandoned Once work requires
//! owner reconciliation before another attempt. Hosts own upgrade reconstruction.
//! Opt-in `metrics` exposes inert aggregate counters and bounded declaration
//! inventory; observations never authorize scheduling or business-effect replay.

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
mod provider;
// Transport requires the IC global timer and the executor's IC entry roots.
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
pub use {
    diagnostics::{Inventory, TimerKind, TimerMeasurements, TimerPhase, TimerSnapshot},
    metrics::{Metrics, MetricsStorage},
    provider::{initialize_with_metrics, inventory, metrics},
};

pub use provider::{
    Once, OnceStatus, RecoveryToken, Watchdog, WatchdogStatus, bind_once, bind_watchdog, initialize,
};
