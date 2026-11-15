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
        let plan = self.schedule.catch_up(self.progress.through(), now, limit);
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
        if self.faulted || self.progress.paused() {
            return OnceDemand::Inactive;
        }
        if let Some(selection) = self.progress.selection() {
            return OnceDemand::At(selection.occurrence.scheduled_at);
        }
        if self.start_batch(now).is_err() {
            self.faulted = true;
            return OnceDemand::Inactive;
        }
        let at = self
            .batch
            .as_ref()
            .map(|batch| batch.pending[0])
            .or_else(|| {
                self.schedule
                    .next_after(now.max(self.progress.through()), u64::MAX)
            });
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

    /// Re-evaluate delayed delivery before accepting business work. A frozen
    /// batch is never recomputed, including after a pause or host reconstruction.
    pub(super) fn prepare(&mut self, now: u64) -> Result<bool, Error> {
        let at = self
            .progress
            .selection()
            .ok_or(Error::Faulted)?
            .occurrence
            .scheduled_at;
        let age = now.checked_sub(at).ok_or(Error::Faulted)?;
        match self.policy {
            Policy::SkipLate { lateness_ns } => Ok(age <= lateness_ns),
            Policy::CatchUp(_) if self.batch.is_some() => Ok(true),
            Policy::CatchUp(limit) => {
                if age > limit.max_age_ns {
                    return Ok(false);
                }
                // Preserve the committed selected identity even if the consumer's
                // initial message traps and rolls this batch construction back.
                let plan = self.schedule.catch_up(at - 1, now, limit);
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

    /// Called only after normal settlement or explicit effect reconciliation.
    pub(super) fn finish_batch(&mut self, selection: Selection, now: u64) -> Result<(), Error> {
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
