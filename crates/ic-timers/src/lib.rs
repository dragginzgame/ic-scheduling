//! Retained timers for isolated Internet Computer work.
//!
//! The wasm32 API provides retained Once callbacks and authenticated self-dispatch.
//! Initial/post-await recovery and owner precedence have local installed tests.
//! Watchdogs, metrics, pressure limits and lifecycle qualification remain unfinished.

// The IC provider is wasm32-only; native builds retain the pure engine tests.
#[cfg_attr(
    all(not(test), not(target_arch = "wasm32")),
    expect(dead_code, reason = "native build has no IC transport")
)]
mod engine;

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "watchdog core awaits binding and IC transport integration"
    )
)]
mod watchdog;

#[cfg_attr(
    all(not(test), not(target_arch = "wasm32")),
    expect(dead_code, reason = "native build has no IC transport")
)]
mod bindings;

mod api;
pub use api::{Error, Observation, OnceDemand, OnceNext, OnceResult};

#[cfg(target_arch = "wasm32")]
mod transport;
#[cfg(target_arch = "wasm32")]
pub use transport::{Once, OnceStatus, RecoveryToken, bind_once, initialize};
