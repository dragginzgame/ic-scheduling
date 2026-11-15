//! One-shot, UTC cron and elapsed-interval scheduling with optional IC execution.
//!
//! Enable the opt-in `calendar` feature for five-field UTC cron schedules and
//! fixed-rate/after-completion intervals and one-shot deadlines. Saffron owns calendar calculation.
//! Bounded catch-up previews are available through `Schedule::catch_up` when
//! `calendar` is enabled. The opt-in `execution` feature adds wasm32 execution
//! with explicit skip-late or bounded catch-up policy on
//! retained Once timers. Default builds include neither calendar nor IC execution
//! dependencies. Host-owned checkpoints support settled/queued reconstruction;
//! explicit retry-safe outcomes support bounded retries and pending-retry checkpoints.
//! Explicit unresolved records reconstruct blocked in a fresh runtime; hosts own
//! storage and lifecycle. Latest outcome records are bounded; coalescing is deferred.

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
#[cfg(feature = "execution")]
pub use progress::ControlState;

#[cfg(any(feature = "execution", test))]
mod retry;
#[cfg(feature = "execution")]
pub use retry::{RetryPolicy, RetryPolicyError, RetryProgress};

#[cfg(any(feature = "execution", test))]
mod unresolved;
#[cfg(feature = "execution")]
pub use unresolved::{Attempt, UnresolvedCheckpoint, UnresolvedPhase};

#[cfg(any(feature = "execution", test))]
mod outcome;
#[cfg(feature = "execution")]
pub use outcome::{Outcome, RecordedOutcome, RecordedResult};

#[cfg(all(feature = "execution", target_arch = "wasm32"))]
mod execution;
#[cfg(all(feature = "execution", target_arch = "wasm32"))]
pub use execution::{
    Context, Error, Job, Recovery, RecoveryDisposition, Status, bind, restore, restore_unresolved,
};
