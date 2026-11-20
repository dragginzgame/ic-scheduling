//! Inert aggregate recording; no callbacks, allocation, clocks or scheduling authority.

#[cfg(feature = "metrics")]
mod enabled;

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
pub use enabled::{Metrics, MetricsStorage};
