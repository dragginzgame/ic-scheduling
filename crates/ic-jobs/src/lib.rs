//! UTC calendar preview backed by Saffron.
//!
//! Enable the opt-in `calendar` feature for five-field UTC cron schedules.
//! Default builds include no calendar dependencies. Execution is not implemented;
//! supplier search costs must be qualified before IC timer integration.

#[cfg(feature = "calendar")]
mod schedule;

#[cfg(feature = "calendar")]
pub use schedule::{Schedule, ScheduleError};
