//! Input admission and timestamp adaptation around Saffron's UTC engine.

use std::{error::Error, fmt};

use chrono::{DateTime, Utc};
use saffron::{Cron, parse::CronParseError};

/// A failed schedule admission.
#[derive(Debug)]
pub enum ScheduleError {
    /// Expression exceeds 256 bytes or does not contain exactly five fields.
    InvalidLayout,
    /// Saffron rejected the cron expression.
    Parse(CronParseError),
}

impl fmt::Display for ScheduleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLayout => {
                f.write_str("expected at most 256 bytes and exactly five cron fields")
            }
            Self::Parse(error) => error.fmt(f),
        }
    }
}
impl Error for ScheduleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidLayout => None,
            Self::Parse(error) => Some(error),
        }
    }
}

/// A five-field UTC cron recurrence, evaluated at minute boundaries.
///
/// Saffron owns parsing, calendar arithmetic and next-occurrence search. No time
/// zones, DST policies, seconds dialect or custom calendar engine are provided.
#[derive(Clone, Debug)]
pub struct Schedule {
    cron: Cron,
}

impl Schedule {
    /// Compile minute, hour, day-of-month, month and weekday fields using
    /// Saffron's grammar. Names and calendar extensions follow the supplier.
    ///
    /// # Errors
    /// Returns a layout or supplier parse error. A valid but impossible rule
    /// is accepted and produces no occurrences.
    pub fn parse(expression: &str) -> Result<Self, ScheduleError> {
        if expression.len() > 256 || expression.split_whitespace().count() != 5 {
            return Err(ScheduleError::InvalidLayout);
        }
        let cron = expression.parse().map_err(ScheduleError::Parse)?;
        Ok(Self { cron })
    }

    /// Find a UTC occurrence after an exclusive lower bound and through an
    /// inclusive upper bound, both in unsigned Unix nanoseconds.
    ///
    /// Returns `None` for an empty window or no occurrence in that window.
    /// The horizon filters results; it does not bound supplier search work.
    /// This synchronous preview has no resumable budget and reads no clock.
    #[must_use]
    pub fn next_after(&self, after_ns: u64, through_ns: u64) -> Option<u64> {
        if after_ns >= through_ns {
            return None;
        }
        // All u64 Unix nanoseconds fit Chrono's seconds range. Saffron's
        // exclusive search truncates the input minute before advancing it.
        let start = DateTime::<Utc>::from_timestamp(
            (after_ns / 1_000_000_000) as i64,
            (after_ns % 1_000_000_000) as u32,
        )?;
        let next = self.cron.next_after(start)?;
        // Avoid Chrono's signed-i64 nanosecond limit: timers use unsigned u64.
        let ns = i128::from(next.timestamp()) * 1_000_000_000
            + i128::from(next.timestamp_subsec_nanos());
        u64::try_from(ns)
            .ok()
            .filter(|&ns| ns > after_ns && ns <= through_ns)
    }
}

#[cfg(test)]
mod tests;
