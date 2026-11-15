//! Validated progress data; hosts own encoding, storage and matching job definitions.
use crate::progress::ControlState;

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

/// Progress of a job with no accepted or uncertain work. This is data, not a
/// recovery token. The host must persist the stable job key, schedule and execution
/// policy alongside it, atomically, and validate their association on restore.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Checkpoint {
    pub(crate) revision: u64,
    pub(crate) through: Option<u64>,
    pub(crate) selected_at: Option<u64>,
    pub(crate) control: ControlState,
    pub(crate) catch_up: Option<CatchUpProgress>,
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
    #[cfg(all(feature = "execution", any(test, target_arch = "wasm32")))]
    pub(crate) fn matches_definition(
        &self,
        schedule: &crate::Schedule,
        policy: crate::Policy,
    ) -> bool {
        if self.control == ControlState::Cancelled {
            return true; // Constructor forbids any selected or remaining work.
        }
        if let Some(next) = schedule.after_settlement(self.through) {
            return self.catch_up.is_none()
                && (if self.control == ControlState::Paused {
                    self.selected_at.is_none()
                } else {
                    self.selected_at == next
                });
        }
        let matches = |at| schedule.next_from(at, at) == Some(at);
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
    /// with the cursor/selection. Schema 4 records active, paused or cancelled
    /// control state. Cancellation cannot retain selected work or a batch.
    pub fn new(
        version: u8,
        revision: u64,
        through: Option<u64>,
        selected_at: Option<u64>,
        control: ControlState,
        catch_up: Option<CatchUpProgress>,
    ) -> Result<Self, CheckpointError> {
        if version != 4 {
            return Err(CheckpointError::Version);
        }
        if revision == 0
            || selected_at.is_some_and(|at| {
                control != ControlState::Active || through.is_some_and(|end| at <= end)
            })
            || (control == ControlState::Cancelled && catch_up.is_some())
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
            revision,
            through,
            selected_at,
            control,
            catch_up,
        })
    }
    /// Progress schema version for the host's encoded record.
    #[must_use]
    pub const fn version(&self) -> u8 {
        4
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
}

#[cfg(test)]
mod tests;
