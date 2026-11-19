//! Validated progress data; hosts own encoding, storage and matching job definitions.

#[cfg(test)]
mod tests;

use crate::{Attempt, UnresolvedPhase, state::ControlState};

/// Invalid host-restored progress. No timer declaration has been admitted yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointError {
    /// This library does not support the supplied progress schema.
    Version,
    /// Revision zero or inconsistent selection, cursor, control or remaining batch state.
    Progress,
}
impl std::fmt::Display for CheckpointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CheckpointError {}

/// Reconstructable job progress, optionally carrying an unresolved attempt. This is data, not a
/// recovery token. The host must persist the stable job key, schedule and execution
/// policy alongside it, atomically, and validate their association on restore.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Checkpoint {
    pub(crate) unresolved: Option<(Attempt, UnresolvedPhase)>,
    pub(crate) revision: u64,
    pub(crate) through: Option<u64>,
    pub(crate) selected_at: Option<u64>,
    pub(crate) control: ControlState,
    pub(crate) catch_up: Option<CatchUpProgress>,
    pub(crate) retry: Option<crate::retry::RetryProgress>,
    pub(crate) last_outcome: Option<crate::outcome::RecordedOutcome>,
}

/// Remaining oldest-first work and the fixed endpoint of a catch-up batch.
/// Persist this alongside the checkpoint; omitting it would lose the batch limit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CatchUpProgress {
    pub(crate) pending: Vec<u64>,
    pub(crate) through: u64,
}
impl CatchUpProgress {
    /// Validate decoded remaining work before allocating a bounded copy.
    /// # Errors
    /// Rejects empty/over-64 batches, non-increasing timestamps, and
    /// occurrences beyond the fixed batch endpoint.
    pub fn new(pending: &[u64], through: u64) -> Result<Self, CheckpointError> {
        if pending.is_empty()
            || pending.len() > 64
            || pending.windows(2).any(|pair| pair[0] >= pair[1])
            || pending.last().is_some_and(|&at| at > through)
        {
            return Err(CheckpointError::Progress);
        }

        Ok(Self {
            pending: pending.to_vec(),
            through,
        })
    }

    /// Remaining selected timestamps, including a currently queued occurrence.
    #[must_use]
    pub fn pending(&self) -> &[u64] {
        &self.pending
    }

    /// Fixed endpoint of the batch's original planning window.
    #[must_use]
    pub const fn through(&self) -> u64 {
        self.through
    }
}
impl Checkpoint {
    /// Validate the definition-dependent part before the adapter admits a timer.
    /// Batch age was frozen at selection; do not reapply it during restoration.
    #[cfg_attr(
        all(not(test), not(target_arch = "wasm32")),
        expect(dead_code, reason = "only the IC adapter restores live jobs")
    )]
    pub(crate) fn matches_definition(
        &self,
        schedule: &crate::Schedule,
        policy: crate::Policy,
        retry_policy: crate::retry::RetryPolicy,
    ) -> bool {
        if let Some((attempt, _)) = self.unresolved {
            if attempt.number > retry_policy.max_attempts() {
                return false;
            }
            if attempt.revision == self.revision {
                let matches = match schedule.after_settlement(self.through) {
                    Some(next) => next == Some(attempt.scheduled_at),
                    None => {
                        schedule.next_from(attempt.scheduled_at, attempt.scheduled_at)
                            == Some(attempt.scheduled_at)
                    }
                };
                if !matches {
                    return false;
                }
            }
        }
        if let Some(retry) = &self.retry
            && retry_policy.next_retry(retry.failed_attempts, retry.failed_at)
                != Some(retry.retry_at)
        {
            return false;
        }
        if self.control == ControlState::Cancelled {
            return true; // Constructor forbids any selected or remaining work.
        }
        if let Some(next) = schedule.after_settlement(self.through) {
            return self.catch_up.is_none()
                && self
                    .retry
                    .as_ref()
                    .is_none_or(|retry| Some(retry.scheduled_at) == next)
                && (if self.control == ControlState::Paused {
                    self.selected_at.is_none()
                } else {
                    self.selected_at == next
                });
        }
        let matches = |at| schedule.next_from(at, at) == Some(at);
        if self
            .retry
            .as_ref()
            .is_some_and(|retry| !matches(retry.scheduled_at))
        {
            return false;
        }
        if self.selected_at.is_some_and(|at| !matches(at)) {
            return false;
        }

        self.catch_up.as_ref().is_none_or(|batch| {
            let crate::Policy::CatchUp(limit) = policy else {
                return false;
            };
            batch.pending.len() <= usize::from(limit.max_count)
                && batch.pending.iter().all(|&at| matches(at))
        })
    }

    /// Validate decoded progress before any timer binding is created.
    /// # Errors
    /// Rejects unsupported schemas, revision zero, selected work while paused,
    /// a selected occurrence at/before settled progress, or a batch inconsistent
    /// with the cursor/selection. Schema 7 includes pending retry identity and an optional latest outcome,
    /// alongside consumed attempts and deadline. Cancellation cannot retain pending work.
    pub fn new(
        version: u8,
        revision: u64,
        through: Option<u64>,
        selected_at: Option<u64>,
        control: ControlState,
        catch_up: Option<CatchUpProgress>,
        retry: Option<crate::retry::RetryProgress>,
    ) -> Result<Self, CheckpointError> {
        if version != 7 {
            return Err(CheckpointError::Version);
        }
        if revision == 0
            || selected_at.is_some_and(|at| {
                control != ControlState::Active || through.is_some_and(|end| at <= end)
            })
            || (control == ControlState::Cancelled && (catch_up.is_some() || retry.is_some()))
        {
            return Err(CheckpointError::Progress);
        }
        if let Some(retry) = &retry
            && (through.is_some_and(|end| retry.scheduled_at <= end)
                || (control == ControlState::Active && selected_at != Some(retry.scheduled_at))
                || catch_up
                    .as_ref()
                    .is_some_and(|batch| batch.pending[0] != retry.scheduled_at))
        {
            return Err(CheckpointError::Progress);
        }
        if let Some(batch) = &catch_up
            && (through.is_some_and(|end| batch.pending[0] <= end)
                || (control == ControlState::Active && selected_at != Some(batch.pending[0])))
        {
            return Err(CheckpointError::Progress);
        }

        Ok(Self {
            unresolved: None,
            revision,
            through,
            selected_at,
            control,
            catch_up,
            retry,
            last_outcome: None,
        })
    }

    /// Attach unresolved correlation data to paused progress. This record can
    /// only reconstruct blocked work in a fresh runtime; it is not replay authority.
    /// # Errors
    /// Rejects queued/retry work or inconsistent revision, cursor and batch data.
    pub fn with_unresolved(
        mut self,
        attempt: Attempt,
        phase: UnresolvedPhase,
    ) -> Result<Self, CheckpointError> {
        if self.control != ControlState::Paused
            || self.selected_at.is_some()
            || self.retry.is_some()
            || attempt.revision > self.revision
            || (attempt.revision == self.revision
                && self.through.is_some_and(|end| attempt.scheduled_at <= end))
            || self.catch_up.as_ref().is_some_and(|batch| {
                attempt.revision != self.revision || batch.pending[0] != attempt.scheduled_at
            })
        {
            return Err(CheckpointError::Progress);
        }
        self.unresolved = Some((attempt, phase));

        Ok(self)
    }

    /// Unresolved attempt and its export-time observation, if any. Both phases
    /// reconstruct blocked. None identifies ordinary settled/queued progress.
    #[must_use]
    pub const fn unresolved(&self) -> Option<(Attempt, UnresolvedPhase)> {
        self.unresolved
    }

    /// Progress schema version for the host's encoded record.
    #[must_use]
    pub const fn version(&self) -> u8 {
        7
    }

    /// Preserved schedule revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Exclusive settled cursor in Unix nanoseconds; None means nothing settled.
    /// For completion-relative schedules this also records the latest completion,
    /// reconciliation or owner-edit horizon used to calculate the next deadline.
    #[must_use]
    pub const fn through(&self) -> Option<u64> {
        self.through
    }

    /// Selected but unaccepted occurrence, if any.
    #[must_use]
    pub const fn selected_at(&self) -> Option<u64> {
        self.selected_at
    }

    /// Whether owner pause must survive reconstruction.
    #[must_use]
    pub const fn paused(&self) -> bool {
        matches!(self.control, ControlState::Paused)
    }

    /// Owner eligibility state to preserve in the host record.
    #[must_use]
    pub const fn control(&self) -> ControlState {
        self.control
    }

    /// Remaining batch state, if catch-up is in progress.
    #[must_use]
    pub const fn catch_up(&self) -> Option<&CatchUpProgress> {
        self.catch_up.as_ref()
    }

    /// Work explicitly deemed retry-safe and waiting for its next attempt, if any.
    #[must_use]
    pub const fn retry(&self) -> Option<&crate::retry::RetryProgress> {
        self.retry.as_ref()
    }

    /// Attach the decoded latest observation; it does not change eligibility.
    /// # Errors
    /// Rejects an outcome from a revision newer than this job definition.
    pub fn with_last_outcome(
        mut self,
        outcome: crate::outcome::RecordedOutcome,
    ) -> Result<Self, CheckpointError> {
        if outcome.attempt().revision() > self.revision {
            return Err(CheckpointError::Progress);
        }
        self.last_outcome = Some(outcome);

        Ok(self)
    }

    /// Latest completion/reconciliation observation; older history belongs to the host.
    #[must_use]
    pub const fn last_outcome(&self) -> Option<crate::outcome::RecordedOutcome> {
        self.last_outcome
    }
}
