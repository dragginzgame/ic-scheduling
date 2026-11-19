//! Retry-budget calculation and validated pending retry data; neither authorizes replay of uncertainty.

#[cfg(test)]
mod tests;

/// Invalid attempt budget or delay bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryPolicyError;
impl std::fmt::Display for RetryPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            "retry attempts and delays must be positive, with initial delay at most the cap",
        )
    }
}
impl std::error::Error for RetryPolicyError {}

/// Bounded exponential backoff for a single logical occurrence.
///
/// The attempt budget includes the initial attempt. The executor uses this only
/// after an explicit retry-safe normal outcome or owner reconciliation.
/// A deadline alone is not permission to retry uncertain work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryPolicy {
    max_attempts: u32,
    initial_delay_ns: u64,
    max_delay_ns: u64,
}

/// One occurrence explicitly deemed retry-safe, waiting for its next attempt. Hosts persist
/// this with the checkpoint and matching retry policy, not as recovery authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetryProgress {
    pub(crate) scheduled_at: u64,
    pub(crate) failed_attempts: u32,
    pub(crate) failed_at: u64,
    pub(crate) retry_at: u64,
}
impl RetryProgress {
    /// Validate decoded pending retry data. Policy agreement is checked at restore.
    /// # Errors
    /// Rejects zero/exhausted counters, failure before the occurrence or a
    /// nonpositive delay. A pending retry must leave room for another attempt.
    pub fn new(
        scheduled_at: u64,
        failed_attempts: u32,
        failed_at: u64,
        retry_at: u64,
    ) -> Result<Self, crate::checkpoint::CheckpointError> {
        if failed_attempts == 0
            || failed_attempts == u32::MAX
            || failed_at < scheduled_at
            || retry_at <= failed_at
        {
            return Err(crate::checkpoint::CheckpointError::Progress);
        }

        Ok(Self {
            scheduled_at,
            failed_attempts,
            failed_at,
            retry_at,
        })
    }

    /// Logical occurrence identity, unchanged across attempts.
    #[must_use]
    pub const fn scheduled_at(&self) -> u64 {
        self.scheduled_at
    }

    /// Consumed attempts, including the initial attempt and any reconciled uncertainty.
    #[must_use]
    pub const fn failed_attempts(&self) -> u32 {
        self.failed_attempts
    }

    /// Time of the last retry-safe failure or explicit retry-safe reconciliation.
    #[must_use]
    pub const fn failed_at(&self) -> u64 {
        self.failed_at
    }

    /// Earliest eligibility for the next attempt.
    #[must_use]
    pub const fn retry_at(&self) -> u64 {
        self.retry_at
    }
}
impl RetryPolicy {
    /// One initial attempt and no retries.
    pub const NONE: Self = Self {
        max_attempts: 1,
        initial_delay_ns: 1,
        max_delay_ns: 1,
    };

    /// Validate a total-attempt budget and positive nanosecond delay bounds.
    /// # Errors
    /// Rejects zero attempts/delays and a cap below the initial delay.
    pub fn new(
        max_attempts: u32,
        initial_delay_ns: u64,
        max_delay_ns: u64,
    ) -> Result<Self, RetryPolicyError> {
        if max_attempts == 0 || initial_delay_ns == 0 || max_delay_ns < initial_delay_ns {
            return Err(RetryPolicyError);
        }

        Ok(Self {
            max_attempts,
            initial_delay_ns,
            max_delay_ns,
        })
    }

    /// Total accepted attempts allowed for one occurrence, including its first.
    #[must_use]
    pub const fn max_attempts(self) -> u32 {
        self.max_attempts
    }

    /// Delay after the first failed or reconciled retry-safe attempt, in nanoseconds.
    #[must_use]
    pub const fn initial_delay_ns(self) -> u64 {
        self.initial_delay_ns
    }

    /// Maximum delay between a normal failure and its next attempt.
    #[must_use]
    pub const fn max_delay_ns(self) -> u64 {
        self.max_delay_ns
    }

    /// Calculate a retry deadline after a retry-safe failure or owner reconciliation.
    /// `failed_attempts` counts accepted attempts for this occurrence, starting
    /// at one. The timestamp is failure completion or reconciliation time.
    /// Delay doubles per consumed attempt until capped; evaluation is constant
    /// time even for large counters. No clock is read and no state is changed.
    ///
    /// Returns None for zero/exhausted attempt counts or deadline overflow.
    /// Reconstruct this policy through `new` when decoding host-owned data.
    #[must_use]
    pub fn next_retry(self, failed_attempts: u32, finished_at_ns: u64) -> Option<u64> {
        if failed_attempts == 0 || failed_attempts >= self.max_attempts {
            return None;
        }
        let delay = 1_u64
            .checked_shl(failed_attempts - 1)
            .and_then(|factor| self.initial_delay_ns.checked_mul(factor))
            .unwrap_or(self.max_delay_ns)
            .min(self.max_delay_ns);

        finished_at_ns.checked_add(delay)
    }
}
