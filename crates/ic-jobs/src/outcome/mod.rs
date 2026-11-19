//! Bounded latest-result data; hosts own any historical journal.

#[cfg(test)]
mod tests;

use crate::{checkpoint::CheckpointError, unresolved::Attempt};

/// Business outcome. Only an explicit retry-safe failure permits policy retries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    /// No business work was needed.
    NoWork,
    /// Business work completed successfully.
    Success,
    /// Expected business failure; this occurrence settles.
    ExpectedFailure,
    /// Normal return explicitly permits repeating this occurrence's effects.
    RetryableFailure,
}
/// Evidence recorded at normal completion or explicit reconciliation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordedResult {
    /// The callback returned this business outcome normally.
    Returned(Outcome),
    /// The owner retired uncertain work; this does not claim business success.
    Retired,
    /// The owner explicitly established that retrying uncertain work is safe.
    RetryAuthorized,
}
/// Latest result and its exact attempt identity. This is observation, not recovery
/// authority. Persist it with the matching job definition/progress; archive older
/// records in application storage if historical retention is required.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecordedOutcome {
    attempt: Attempt,
    result: RecordedResult,
    at: u64,
}
impl RecordedOutcome {
    /// Validate decoded result data.
    /// # Errors
    /// Rejects a result timestamp before its logical occurrence.
    pub fn new(attempt: Attempt, result: RecordedResult, at: u64) -> Result<Self, CheckpointError> {
        if at < attempt.scheduled_at() {
            return Err(CheckpointError::Progress);
        }
        Ok(Self {
            attempt,
            result,
            at,
        })
    }

    /// Exact attempt; combine with the host's stable job key.
    #[must_use]
    pub const fn attempt(self) -> Attempt {
        self.attempt
    }

    /// Returned outcome or explicit owner disposition.
    #[must_use]
    pub const fn result(self) -> RecordedResult {
        self.result
    }

    /// Normal completion or reconciliation time, in Unix nanoseconds.
    #[must_use]
    pub const fn at(self) -> u64 {
        self.at
    }
}
