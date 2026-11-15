//! Selection policy and bounded catch-up state; Timers still owns execution custody.
use super::{Error, OnceDemand, Policy, Progress, Schedule, Selection};
use crate::CatchUpProgress;

pub(super) struct State {
    pub(super) schedule: Schedule,
    pub(super) policy: Policy,
    pub(super) progress: Progress,
    pub(super) batch: Option<CatchUpProgress>,
    pub(super) faulted: bool,
}
impl State {
    fn start_batch(&mut self, now: u64) -> Result<(), Error> {
        let Policy::CatchUp(limit) = self.policy else {
            return Ok(());
        };
        if self.batch.is_some() {
            return Ok(());
        }
        let plan = self
            .schedule
            .catch_up_from(self.progress.through(), now, limit);
        if plan.occurrences().is_empty() {
            self.progress
                .skip_through(plan.through())
                .map_err(|_| Error::Faulted)?;
        } else {
            self.batch = Some(
                CatchUpProgress::new(plan.occurrences(), plan.through())
                    .map_err(|_| Error::Faulted)?,
            );
        }
        Ok(())
    }

    pub(super) fn plan(&mut self, now: u64) -> OnceDemand {
        if self.faulted || self.progress.paused() || self.progress.cancelled() {
            return OnceDemand::Inactive;
        }
        if let Some(selection) = self.progress.selection() {
            return OnceDemand::At(selection.occurrence.scheduled_at);
        }
        let at = if let Some(next) = self.schedule.after_settlement(self.progress.through()) {
            next
        } else {
            if self.start_batch(now).is_err() {
                self.faulted = true;
                return OnceDemand::Inactive;
            }
            self.batch
                .as_ref()
                .map(|batch| batch.pending[0])
                .or_else(|| {
                    self.schedule
                        .next_after(now.max(self.progress.through().unwrap_or(0)), u64::MAX)
                })
        };
        let Some(at) = at else {
            return OnceDemand::Inactive;
        };
        match self.progress.select(at) {
            Ok(_) => OnceDemand::At(at),
            Err(_) => {
                self.faulted = true;
                OnceDemand::Inactive
            }
        }
    }

    /// Commit the current selection before the adapter invokes consumer code.
    pub(super) fn accept(&mut self, now: u64) -> Result<(Selection, bool), Error> {
        if self.faulted {
            return Err(Error::Faulted);
        }
        let selection = self.progress.selection().ok_or(Error::Faulted)?;
        let deliver = self.prepare(selection.occurrence.scheduled_at, now)?;
        self.progress
            .accept(selection, now)
            .map_err(|_| Error::Faulted)?;
        Ok((selection, deliver))
    }

    /// A frozen batch is never recomputed, including after pause or restoration.
    fn prepare(&mut self, at: u64, now: u64) -> Result<bool, Error> {
        let age = now.checked_sub(at).ok_or(Error::Faulted)?;
        match self.policy {
            Policy::SkipLate { lateness_ns } => Ok(age <= lateness_ns),
            Policy::CatchUp(limit)
                if self
                    .schedule
                    .after_settlement(self.progress.through())
                    .is_some() =>
            {
                Ok(age <= limit.max_age_ns)
            }
            Policy::CatchUp(_) if self.batch.is_some() => Ok(true),
            Policy::CatchUp(limit) => {
                if age > limit.max_age_ns {
                    return Ok(false);
                }
                // Preserve the committed selected identity even if the consumer's
                // initial message traps and rolls this batch construction back.
                let plan = self.schedule.catch_up_from(at.checked_sub(1), now, limit);
                if plan.occurrences().first() != Some(&at) {
                    return Err(Error::Faulted);
                }
                self.batch = Some(
                    CatchUpProgress::new(plan.occurrences(), plan.through())
                        .map_err(|_| Error::Faulted)?,
                );
                Ok(true)
            }
        }
    }

    /// Only the current definition may advance its completion horizon. A newer
    /// owner edit retains its own horizon even when old accepted work finishes.
    pub(super) fn finish(&mut self, selection: Selection, now: u64) -> Result<(), Error> {
        let current = selection.occurrence.revision == self.progress.revision();
        self.progress
            .complete(selection)
            .map_err(|_| Error::Faulted)?;
        if current
            && self
                .schedule
                .after_settlement(self.progress.through())
                .is_some()
        {
            self.progress
                .skip_through(now)
                .map_err(|_| Error::Faulted)?;
        }
        self.finish_batch(selection, now)
    }

    /// Called only after normal settlement or explicit effect reconciliation.
    fn finish_batch(&mut self, selection: Selection, now: u64) -> Result<(), Error> {
        let Some(batch) = &mut self.batch else {
            return Ok(());
        };
        if batch.pending[0] != selection.occurrence.scheduled_at {
            return Err(Error::Faulted);
        }
        batch.pending.remove(0);
        if batch.pending.is_empty() {
            // Do not replenish the quota from work that arrived during this batch.
            self.progress
                .skip_through(now.max(batch.through))
                .map_err(|_| Error::Faulted)?;
            self.batch = None;
        }
        Ok(())
    }
}
