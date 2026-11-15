//! UTC calendar scheduling backed by Saffron and optional IC execution.
//!
//! Enable the opt-in `calendar` feature for five-field UTC cron schedules.
//! The opt-in `execution` feature adds a wasm32 skip-late execution preview on
//! retained Once timers. Default builds include neither calendar nor IC execution
//! dependencies. Persistence and other missed-run policies remain unfinished.

#[cfg(feature = "calendar")]
mod schedule;

#[cfg(feature = "calendar")]
pub use schedule::{Schedule, ScheduleError};

#[cfg_attr(
    all(not(test), not(all(feature = "execution", target_arch = "wasm32"))),
    expect(dead_code, reason = "this build excludes the wasm32 execution adapter")
)]
mod progress;

#[cfg(all(feature = "execution", target_arch = "wasm32"))]
mod execution;
#[cfg(all(feature = "execution", target_arch = "wasm32"))]
pub use execution::{Context, Error, Job, Outcome, Recovery, Status, bind};
