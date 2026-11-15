//! Validated progress data; hosts own encoding, storage and matching job definitions.

/// Invalid host-restored progress. No timer declaration has been admitted yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointError {
    /// This library does not support the supplied progress schema.
    Version,
    /// Revision zero or inconsistent selected, cursor, paused or remaining batch state.
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
    pub(crate) through: u64,
    pub(crate) selected_at: Option<u64>,
    pub(crate) paused: bool,
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
    /// Rejects empty/over-64 batches, zero or non-increasing timestamps, and
    /// occurrences beyond the fixed batch endpoint.
    pub fn new(pending: &[u64], through: u64) -> Result<Self, CheckpointError> {
        if pending.is_empty()
            || pending.len() > 64
            || pending[0] == 0
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
    /// Validate decoded progress before any timer binding is created.
    /// # Errors
    /// Rejects unsupported schemas, revision zero, selected work while paused,
    /// a selected occurrence at/before settled progress, or a batch inconsistent
    /// with the cursor/selection. Schema 2 includes remaining catch-up work.
    pub fn new(
        version: u8,
        revision: u64,
        through: u64,
        selected_at: Option<u64>,
        paused: bool,
        catch_up: Option<CatchUpProgress>,
    ) -> Result<Self, CheckpointError> {
        if version != 2 {
            return Err(CheckpointError::Version);
        }
        if revision == 0 || selected_at.is_some_and(|at| paused || at <= through) {
            return Err(CheckpointError::Progress);
        }
        if let Some(batch) = &catch_up
            && (batch.pending[0] <= through || (!paused && selected_at != Some(batch.pending[0])))
        {
            return Err(CheckpointError::Progress);
        }
        Ok(Self {
            revision,
            through,
            selected_at,
            paused,
            catch_up,
        })
    }
    /// Progress schema version for the host's encoded record.
    #[must_use]
    pub const fn version(&self) -> u8 {
        2
    }
    /// Preserved schedule revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }
    /// Exclusive settled cursor in Unix nanoseconds.
    #[must_use]
    pub const fn through(&self) -> u64 {
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
        self.paused
    }
    /// Remaining batch state, if catch-up is in progress.
    #[must_use]
    pub const fn catch_up(&self) -> Option<&CatchUpProgress> {
        self.catch_up.as_ref()
    }
}

#[cfg(test)]
mod tests;
