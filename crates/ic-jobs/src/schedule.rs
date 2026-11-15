//! Input admission and timestamp adaptation around Saffron's UTC engine.

use std::{error::Error, fmt, num::NonZeroU64};

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

/// A five-field UTC cron recurrence or anchored elapsed-time interval.
///
/// Saffron owns parsing, calendar arithmetic and next-occurrence search. No time
/// zones, DST policies, seconds dialect or custom calendar engine are provided.
#[derive(Clone, Debug)]
pub struct Schedule {
    rule: Rule,
}
#[derive(Clone, Debug)]
enum Rule {
    Cron(Cron),
    FixedRate { first: u64, every: NonZeroU64 },
}

impl Schedule {
    /// Recur at `first_at_ns + n * every_ns` for nonnegative integer n.
    /// This is elapsed-time arithmetic, not a cron field or local calendar rule.
    /// Overflow ends the recurrence; no timestamp wraps. Like cron previews,
    /// `next_after` excludes its lower bound. Binding selects after current time;
    /// host-restored progress and CatchUp control overdue execution.
    #[must_use]
    pub const fn fixed_rate(first_at_ns: u64, every_ns: NonZeroU64) -> Self {
        Self {
            rule: Rule::FixedRate {
                first: first_at_ns,
                every: every_ns,
            },
        }
    }

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
        Ok(Self {
            rule: Rule::Cron(cron),
        })
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
        let cron = match &self.rule {
            Rule::Cron(cron) => cron,
            Rule::FixedRate { first, every } => {
                let at = if after_ns < *first {
                    *first
                } else {
                    let steps = ((after_ns - first) / every.get()).checked_add(1)?;
                    first.checked_add(steps.checked_mul(every.get())?)?
                };
                return (at <= through_ns).then_some(at);
            }
        };
        // All u64 Unix nanoseconds fit Chrono's seconds range. Saffron's
        // exclusive search truncates the input minute before advancing it.
        let start = DateTime::<Utc>::from_timestamp(
            (after_ns / 1_000_000_000) as i64,
            (after_ns % 1_000_000_000) as u32,
        )?;
        let next = cron.next_after(start)?;
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
