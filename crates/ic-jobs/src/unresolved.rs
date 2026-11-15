//! Host-owned unresolved attempt records; restoration never authorizes replay.
use crate::{
    checkpoint::{Checkpoint, CheckpointError},
    progress::ControlState,
};

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
/// An unresolved attempt and the owner's future progress, stored atomically with
/// its job key, schedule and policies. Use only for reconstruction in a fresh
/// runtime after the old execution has been discarded; not for live cloning or
/// heap snapshots. Remote effects may still require business reconciliation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnresolvedCheckpoint {
    pub(crate) progress: Checkpoint,
    pub(crate) attempt: Attempt,
    phase: UnresolvedPhase,
}
impl UnresolvedCheckpoint {
    /// Validate host-decoded unresolved progress before runtime admission.
    /// The inner checkpoint must be paused, with no queued selection or retry.
    /// It may retain the committed catch-up batch for the unresolved occurrence.
    /// # Errors
    /// Rejects unknown versions, inconsistent revision/cursor/batch data or
    /// progress that could independently admit another attempt.
    pub fn new(
        version: u8,
        progress: Checkpoint,
        attempt: Attempt,
        phase: UnresolvedPhase,
    ) -> Result<Self, CheckpointError> {
        if version != 1 {
            return Err(CheckpointError::Version);
        }
        if progress.control != ControlState::Paused
            || progress.selected_at.is_some()
            || progress.retry.is_some()
            || attempt.revision > progress.revision
            || (attempt.revision == progress.revision
                && progress
                    .through
                    .is_some_and(|end| attempt.scheduled_at <= end))
            || progress.catch_up.as_ref().is_some_and(|batch| {
                attempt.revision != progress.revision || batch.pending[0] != attempt.scheduled_at
            })
        {
            return Err(CheckpointError::Progress);
        }
        Ok(Self {
            progress,
            attempt,
            phase,
        })
    }
    /// Unresolved record schema; the nested progress has its own schema version.
    #[must_use]
    pub const fn version(&self) -> u8 {
        1
    }
    /// Future progress; paused independently of the unresolved execution barrier.
    #[must_use]
    pub const fn progress(&self) -> &Checkpoint {
        &self.progress
    }
    /// Exact unresolved correlation data; not a live recovery capability.
    #[must_use]
    pub const fn attempt(&self) -> Attempt {
        self.attempt
    }
    /// Observation at export; neither variant permits automatic replay.
    #[must_use]
    pub const fn phase(&self) -> UnresolvedPhase {
        self.phase
    }

    #[cfg(all(feature = "execution", any(test, target_arch = "wasm32")))]
    pub(crate) fn matches_definition(
        &self,
        schedule: &crate::Schedule,
        policy: crate::Policy,
        retry_policy: crate::RetryPolicy,
    ) -> bool {
        if self.attempt.number > retry_policy.max_attempts()
            || !self
                .progress
                .matches_definition(schedule, policy, retry_policy)
        {
            return false;
        }
        if self.attempt.revision != self.progress.revision {
            return true;
        }
        match schedule.after_settlement(self.progress.through) {
            Some(next) => next == Some(self.attempt.scheduled_at),
            None => {
                schedule.next_from(self.attempt.scheduled_at, self.attempt.scheduled_at)
                    == Some(self.attempt.scheduled_at)
            }
        }
    }
}

#[cfg(test)]
mod tests;
