//! Bounded calendar planning; execution and effect reconciliation remain separate.
use crate::Schedule;

/// Invalid catch-up count. The supported range is 1 through 64.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CatchUpLimitError;
impl std::fmt::Display for CatchUpLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("catch-up count must be between 1 and 64")
    }
}
impl std::error::Error for CatchUpLimitError {}

/// Explicit age and count limits for a single catch-up planning window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CatchUpLimit {
    pub(crate) max_age_ns: u64,
    pub(crate) max_count: u8,
}

/// Explicit handling of delayed calendar occurrences.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Policy {
    /// Skip consumer work if delivery exceeds this tolerance, then select a future occurrence.
    SkipLate {
        /// Maximum delivery age in nanoseconds, inclusive.
        lateness_ns: u64,
    },
    /// Execute an oldest-first bounded batch. Age is measured when the batch is
    /// selected; pause/restore preserves that selection. On final settlement,
    /// skip excess and occurrences arriving during the batch, then select future work.
    CatchUp(CatchUpLimit),
}
impl CatchUpLimit {
    /// Admit at most 64 occurrences, each no older than `max_age_ns` at the
    /// supplied planning time. Zero age admits only an occurrence exactly then.
    /// # Errors
    /// Rejects a zero count or a count above 64.
    pub fn new(max_age_ns: u64, max_count: u8) -> Result<Self, CatchUpLimitError> {
        if !(1..=64).contains(&max_count) {
            return Err(CatchUpLimitError);
        }
        Ok(Self {
            max_age_ns,
            max_count,
        })
    }
    /// Maximum age at batch selection, for host-owned policy persistence.
    #[must_use]
    pub const fn max_age_ns(&self) -> u64 {
        self.max_age_ns
    }
    /// Maximum occurrences per batch, for host-owned policy persistence.
    #[must_use]
    pub const fn max_count(&self) -> u8 {
        self.max_count
    }
}

/// A bounded preview of oldest-first catch-up with excess occurrences skipped.
///
/// This is not execution or replay authority. Hosts must not use it to bypass
/// running or uncertain work. The CatchUp execution policy uses this calculation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CatchUpPlan {
    occurrences: Vec<u64>,
    through: u64,
    excess: bool,
}
impl CatchUpPlan {
    /// Intended UTC timestamps, oldest first, at most the admitted count.
    #[must_use]
    pub fn occurrences(&self) -> &[u64] {
        &self.occurrences
    }
    /// Window endpoint to retire only after every selected occurrence is settled
    /// or explicitly reconciled. Older and excess occurrences are then skipped.
    /// This never rewinds the supplied settled cursor.
    #[must_use]
    pub const fn through(&self) -> u64 {
        self.through
    }
    /// Whether another age-eligible occurrence was omitted by the count limit.
    /// Does not report occurrences excluded by age.
    #[must_use]
    pub const fn has_excess(&self) -> bool {
        self.excess
    }
}

impl Schedule {
    /// Preview bounded catch-up after an exclusive settled cursor through `now`.
    /// Age is inclusive: an occurrence exactly `max_age_ns` old is eligible.
    /// Uses at most count + 1 Saffron searches and stores at most count timestamps;
    /// it never enumerates the older excluded interval. Individual supplier search
    /// cost remains separate from this bound. No clock is read and no job is changed.
    /// An after-completion schedule can preview only its initial deadline; it
    /// cannot accumulate a backlog before earlier work has completed.
    #[must_use]
    pub fn catch_up(&self, after: u64, now: u64, limit: CatchUpLimit) -> CatchUpPlan {
        self.catch_up_from(Some(after), now, limit)
    }

    pub(crate) fn catch_up_from(
        &self,
        after: Option<u64>,
        now: u64,
        limit: CatchUpLimit,
    ) -> CatchUpPlan {
        let mut plan = CatchUpPlan {
            occurrences: Vec::new(),
            through: after.unwrap_or(0).max(now),
            excess: false,
        };
        let mut cursor = match after {
            Some(after) => match after.checked_add(1) {
                Some(from) => from,
                None => return plan,
            },
            None => 0,
        }
        .max(now.saturating_sub(limit.max_age_ns));
        for _ in 0..limit.max_count {
            let Some(next) = self.next_from(cursor, now) else {
                return plan;
            };
            plan.occurrences.push(next);
            let Some(from) = next.checked_add(1) else {
                return plan;
            };
            cursor = from;
        }
        plan.excess = self.next_from(cursor, now).is_some();
        plan
    }
}

#[cfg(test)]
mod tests;
