//! Host-owned unresolved attempt records; restoration never authorizes replay.

#[cfg(test)]
mod tests;

use crate::checkpoint::CheckpointError;

/// Durable attempt identity, paired with the host's stable job key.
/// Revision and scheduled time identify the effect; number identifies this attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Attempt {
    pub(crate) revision: u64,
    pub(crate) scheduled_at: u64,
    pub(crate) number: u32,
}
impl Attempt {
    /// Validate decoded correlation data.
    /// # Errors
    /// Rejects zero revision or attempt number.
    pub fn new(revision: u64, scheduled_at: u64, number: u32) -> Result<Self, CheckpointError> {
        if revision == 0 || number == 0 {
            return Err(CheckpointError::Progress);
        }

        Ok(Self {
            revision,
            scheduled_at,
            number,
        })
    }

    /// Accepted schedule revision, which may precede an owner edit.
    #[must_use]
    pub const fn revision(self) -> u64 {
        self.revision
    }

    /// Logical occurrence time, unchanged across retries.
    #[must_use]
    pub const fn scheduled_at(self) -> u64 {
        self.scheduled_at
    }

    /// One-based accepted attempt number.
    #[must_use]
    pub const fn number(self) -> u32 {
        self.number
    }
}
/// Runtime observation when the host exported the record. Both restore blocked.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnresolvedPhase {
    /// Work was still accepted; exporting does not stop its original execution.
    Running,
    /// The runtime requires effect reconciliation.
    Uncertain,
}
