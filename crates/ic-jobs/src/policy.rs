use std::num::NonZeroU64;

use serde::{Deserialize, Serialize};

use crate::JobError;

/// What a fixed-rate schedule does when several occurrences are overdue.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MissedRunPolicy {
    /// Advance to the first aligned deadline strictly after completion.
    Skip,
    /// Run one overdue occurrence at a time; consumers bound each work batch.
    CatchUp,
}

/// An absolute schedule. All timestamps and durations are nanoseconds.
///
/// Recurring intervals are nonzero; absolute deadlines may be zero.
///
/// ```compile_fail
/// use ic_jobs::Schedule;
/// let schedule = Schedule::AfterCompletion { first_at_ns: 0, every_ns: 0 };
/// ```
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Schedule {
    /// Run one occurrence.
    Once {
        /// Earliest dispatch time.
        at_ns: u64,
    },
    /// Run at fixed intervals relative to the original deadline.
    FixedRate {
        /// First occurrence's deadline.
        first_at_ns: u64,
        /// Positive interval between scheduled occurrences.
        every_ns: NonZeroU64,
        /// Policy for overdue occurrences.
        missed: MissedRunPolicy,
    },
    /// Wait a fixed interval after each successful completion.
    AfterCompletion {
        /// First occurrence's deadline.
        first_at_ns: u64,
        /// Positive delay after successful completion.
        every_ns: NonZeroU64,
    },
}

impl Schedule {
    pub(crate) const fn first(self) -> u64 {
        match self {
            Self::Once { at_ns } => at_ns,
            Self::FixedRate { first_at_ns, .. } | Self::AfterCompletion { first_at_ns, .. } => {
                first_at_ns
            }
        }
    }

    pub(crate) fn successor(
        self,
        scheduled_ns: u64,
        finished_ns: u64,
    ) -> Result<Option<u64>, JobError> {
        let next = match self {
            Self::Once { .. } => return Ok(None),
            Self::AfterCompletion { every_ns, .. } => finished_ns.checked_add(every_ns.get()),
            Self::FixedRate {
                every_ns,
                missed: MissedRunPolicy::CatchUp,
                ..
            } => scheduled_ns.checked_add(every_ns.get()),
            Self::FixedRate {
                every_ns,
                missed: MissedRunPolicy::Skip,
                ..
            } => {
                let steps = (finished_ns - scheduled_ns) / every_ns;
                steps
                    .checked_add(1)
                    .and_then(|steps| steps.checked_mul(every_ns.get()))
                    .and_then(|delta| scheduled_ns.checked_add(delta))
            }
        };
        next.map(Some).ok_or(JobError::DeadlineOverflow)
    }
}

/// A bounded retry budget for each occurrence, including its first attempt.
///
/// Exponential delays double after each failure and stop growing at the cap.
/// The same execution identity is retained across retries.
///
/// Construct with [RetryPolicy::new] or [RetryPolicy::NONE]. Fields cannot be
/// replaced after validation.
///
/// ```compile_fail
/// use ic_jobs::RetryPolicy;
/// let policy = RetryPolicy { max_attempts: 0, initial_delay_ns: 1, max_delay_ns: 1 };
/// ```
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "RetryPolicyFields", into = "RetryPolicyFields")]
pub struct RetryPolicy(RetryPolicyFields);

impl RetryPolicy {
    /// One attempt with no automatic retries.
    pub const NONE: Self = Self(RetryPolicyFields {
        max_attempts: 1,
        initial_delay_ns: 1,
        max_delay_ns: 1,
    });

    /// Check an immutable retry budget and delays, in nanoseconds.
    ///
    /// The attempt budget and initial delay must be positive, and the maximum
    /// delay must be at least the initial delay. Deserialization uses this check.
    pub const fn new(
        max_attempts: u32,
        initial_delay_ns: u64,
        max_delay_ns: u64,
    ) -> Result<Self, JobError> {
        if max_attempts == 0 || initial_delay_ns == 0 || max_delay_ns < initial_delay_ns {
            Err(JobError::InvalidRetryPolicy)
        } else {
            Ok(Self(RetryPolicyFields {
                max_attempts,
                initial_delay_ns,
                max_delay_ns,
            }))
        }
    }

    /// Total allowed attempts, including the initial dispatch.
    #[must_use]
    pub const fn max_attempts(self) -> u32 {
        self.0.max_attempts
    }

    /// First retry delay, in nanoseconds.
    #[must_use]
    pub const fn initial_delay_ns(self) -> u64 {
        self.0.initial_delay_ns
    }

    /// Maximum retry delay, in nanoseconds.
    #[must_use]
    pub const fn max_delay_ns(self) -> u64 {
        self.0.max_delay_ns
    }

    pub(crate) fn delay(self, failed_attempts: u32) -> u64 {
        // Overflow means the mathematical delay exceeds every representable cap.
        let factor = 1_u64.checked_shl(failed_attempts - 1).unwrap_or(u64::MAX);
        self.0
            .initial_delay_ns
            .saturating_mul(factor)
            .min(self.0.max_delay_ns)
    }

    // Callers admit at least one failed attempt before calculating a retry.
    pub(crate) fn retry_due_ns(self, failed_at_ns: u64, failed_attempts: u32) -> Option<u64> {
        failed_at_ns.checked_add(self.delay(failed_attempts))
    }

    pub(crate) fn elapsed_retry_delay(self, retries: u32) -> u128 {
        let mut elapsed = 0;
        let mut failure = 1;
        // Doubling reaches the checked cap within 65 steps; aggregate its tail.
        while failure <= retries {
            let delay = self.delay(failure);
            if delay == self.0.max_delay_ns {
                return elapsed + u128::from(delay) * u128::from(retries - failure + 1);
            }
            elapsed += u128::from(delay);
            failure += 1;
        }
        elapsed
    }
}

// Decode the existing numeric object shape, then admit it through the same
// constructor used by ordinary callers. Keep the writer's public structure name
// for formats that retain names; the projection itself stays private.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "RetryPolicy")]
struct RetryPolicyFields {
    /// Total allowed attempts, including the initial dispatch. Must be positive.
    max_attempts: u32,
    /// Positive first retry delay.
    initial_delay_ns: u64,
    /// Maximum retry delay, at least the initial delay.
    max_delay_ns: u64,
}

impl From<RetryPolicy> for RetryPolicyFields {
    fn from(policy: RetryPolicy) -> Self {
        policy.0
    }
}

impl TryFrom<RetryPolicyFields> for RetryPolicy {
    type Error = JobError;

    fn try_from(fields: RetryPolicyFields) -> Result<Self, Self::Error> {
        Self::new(
            fields.max_attempts,
            fields.initial_delay_ns,
            fields.max_delay_ns,
        )
    }
}
