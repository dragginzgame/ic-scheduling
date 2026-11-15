//! UTC calendar scheduling backed by Saffron and optional IC execution.
//!
//! Enable the opt-in `calendar` feature for five-field UTC cron schedules.
//! Bounded catch-up previews are available through `Schedule::catch_up` when
//! `calendar` is enabled. The opt-in `execution` feature adds wasm32 execution
//! with explicit skip-late or bounded catch-up policy on
//! retained Once timers. Default builds include neither calendar nor IC execution
//! dependencies. Host-owned checkpoints support settled/queued reconstruction;
//! unresolved-work persistence and coalescing remain unfinished.

#[cfg(feature = "calendar")]
mod schedule;

#[cfg(feature = "calendar")]
pub use schedule::{Schedule, ScheduleError};

#[cfg(feature = "calendar")]
mod catch_up;
#[cfg(feature = "calendar")]
pub use catch_up::{CatchUpLimit, CatchUpLimitError, CatchUpPlan, Policy};

#[cfg_attr(
    all(not(test), not(all(feature = "execution", target_arch = "wasm32"))),
    expect(dead_code, reason = "this build excludes the wasm32 execution adapter")
)]
mod progress;

#[cfg(any(feature = "execution", test))]
mod checkpoint;
#[cfg(feature = "execution")]
pub use checkpoint::{CatchUpProgress, Checkpoint, CheckpointError};

#[cfg(all(feature = "execution", target_arch = "wasm32"))]
mod execution;
#[cfg(all(feature = "execution", target_arch = "wasm32"))]
pub use execution::{Context, Error, Job, Outcome, Recovery, Status, bind, restore};
