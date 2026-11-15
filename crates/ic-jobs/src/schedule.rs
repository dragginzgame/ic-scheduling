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

/// A one-shot deadline, UTC cron rule, fixed-rate or after-completion interval.
///
/// Saffron owns parsing, calendar arithmetic and next-occurrence search. No time
/// zones, DST policies, seconds dialect or custom calendar engine are provided.
#[derive(Clone, Debug)]
pub struct Schedule {
    rule: Rule,
}
#[derive(Clone, Debug)]
enum Rule {
    Once(u64),
    AfterCompletion { first: u64, every: NonZeroU64 },
    Cron(Cron),
    FixedRate { first: u64, every: NonZeroU64 },
}

impl Schedule {
    /// Start at `first_at_ns`, then wait `every_ns` after each normal settlement.
    /// Settlement includes NoWork, ExpectedFailure and lateness skipping. Traps
    /// block for explicit recovery. Overflow ends recurrence. Preview methods
    /// can show only the initial deadline; later deadlines require completion time.
    #[must_use]
    pub const fn after_completion(first_at_ns: u64, every_ns: NonZeroU64) -> Self {
        Self {
            rule: Rule::AfterCompletion {
                first: first_at_ns,
                every: every_ns,
            },
        }
    }

    /// Run once at the supplied Unix nanosecond timestamp, including zero.
    /// Binding admits overdue deadlines; the explicit delivery policy controls
    /// whether work runs. Settlement exhausts this schedule without an implicit retry.
    #[must_use]
    pub const fn once(at_ns: u64) -> Self {
        Self {
            rule: Rule::Once(at_ns),
        }
    }

    /// Recur at `first_at_ns + n * every_ns` for nonnegative integer n.
    /// This is elapsed-time arithmetic, not a cron field or local calendar rule.
    /// Overflow ends the recurrence; no timestamp wraps. Like cron previews,
    /// `next_after` excludes its lower bound. Binding selects the first timestamp,
    /// including zero or an overdue deadline; the explicit policy controls delivery.
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
    /// After-completion schedules preview only their initial deadline; subsequent
    /// deadlines depend on actual settlement and are selected by execution.
    #[must_use]
    pub fn next_after(&self, after_ns: u64, through_ns: u64) -> Option<u64> {
        self.next_from(after_ns.checked_add(1)?, through_ns)
    }

    /// None means calendar/fixed-rate selection; Some(None) means completion
    /// recurrence exhausted the timestamp domain. The cursor includes completion,
    /// explicit reconciliation and edit horizons, so paused/restored work keeps
    /// its deadline rather than adding a delay from the time of resumption.
    pub(crate) fn after_settlement(&self, through: Option<u64>) -> Option<Option<u64>> {
        let Rule::AfterCompletion { first, every } = self.rule else {
            return None;
        };
        Some(match through {
            Some(through) if through >= first => through.checked_add(every.get()),
            _ => Some(first),
        })
    }

    #[cfg(all(feature = "execution", target_arch = "wasm32"))]
    pub(crate) fn first_at(&self) -> Option<u64> {
        match self.rule {
            Rule::Once(at) => Some(at),
            Rule::FixedRate { first, .. } | Rule::AfterCompletion { first, .. } => Some(first),
            Rule::Cron(_) => None,
        }
    }

    /// Inclusive search used by admission, catch-up and checkpoint validation.
    pub(crate) fn next_from(&self, from_ns: u64, through_ns: u64) -> Option<u64> {
        if from_ns > through_ns {
            return None;
        }
        let cron = match &self.rule {
            Rule::Once(at) => {
                return (from_ns <= *at && *at <= through_ns).then_some(*at);
            }
            Rule::AfterCompletion { .. } => {
                return self
                    .after_settlement(None)
                    .flatten()
                    .filter(|&at| from_ns <= at && at <= through_ns);
            }
            Rule::Cron(cron) => cron,
            Rule::FixedRate { first, every } => {
                let at = if from_ns <= *first {
                    *first
                } else {
                    let steps = (from_ns - first).div_ceil(every.get());
                    first.checked_add(steps.checked_mul(every.get())?)?
                };
                return (at <= through_ns).then_some(at);
            }
        };
        // All u64 Unix nanoseconds fit Chrono's seconds range. Saffron's
        // searches truncate to minutes. Only use inclusive search at an exact
        // minute boundary; otherwise advance through the supplier's exclusive API.
        let start = DateTime::<Utc>::from_timestamp(
            (from_ns / 1_000_000_000) as i64,
            (from_ns % 1_000_000_000) as u32,
        )?;
        let next = if from_ns.is_multiple_of(60_000_000_000) {
            cron.next_from(start)
        } else {
            cron.next_after(start)
        }?;
        // Avoid Chrono's signed-i64 nanosecond limit: timers use unsigned u64.
        let ns = i128::from(next.timestamp()) * 1_000_000_000
            + i128::from(next.timestamp_subsec_nanos());
        u64::try_from(ns)
            .ok()
            .filter(|&ns| ns >= from_ns && ns <= through_ns)
    }
}

#[cfg(test)]
mod tests;
