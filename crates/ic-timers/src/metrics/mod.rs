//! Inert aggregate recording; no callbacks, allocation, clocks or scheduling authority.
use crate::Observation;

#[cfg_attr(
    not(feature = "metrics"),
    expect(dead_code, reason = "recording is opt-in")
)]
#[derive(Clone, Copy)]
pub(crate) enum Event {
    Accepted,
    Returned(Observation),
    Abandoned,
    Deferred,
    ProviderFault,
}

#[cfg(feature = "metrics")]
mod enabled;
#[cfg(feature = "metrics")]
pub use enabled::Metrics;
#[cfg(all(feature = "metrics", target_arch = "wasm32"))]
pub(crate) use enabled::Recorder;
