//! Bounded, unpublished Jobs consumer used for canister qualification.
//! Storage, authorization, payloads and lifecycle belong to this application.

pub mod api;
#[cfg(target_arch = "wasm32")]
mod canister;
pub mod delivery_wire;
pub mod management_wire;
#[cfg(any(test, target_arch = "wasm32"))]
mod metrics;
#[cfg(any(test, target_arch = "wasm32"))]
mod queue;
