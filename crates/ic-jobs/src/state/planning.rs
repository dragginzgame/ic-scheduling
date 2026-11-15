//! Recurrence and retry decisions on the authoritative job state.
use super::{Error, Phase, RecoveryDisposition, Selection, State};
use crate::{CatchUpProgress, Policy};
impl State {
    fn start_batch(&mut self, now: u64) -> Result<(), Error> {
        let Policy::CatchUp(limit) = self.policy else {
            return Ok(());
        };
        if self.batch.is_some() {
            return Ok(());
        }
        let plan = self.schedule.catch_up_from(self.through(), now, limit);
        if plan.occurrences().is_empty() {
            self.skip_through(plan.through())
                .map_err(|_| Error::Exhausted)?;
        } else {
            self.batch = Some(
                CatchUpProgress::new(plan.occurrences(), plan.through())
                    .map_err(|_| Error::Exhausted)?,
            );
        }
        Ok(())
    }

    pub(crate) fn plan(&mut self, now: u64) -> Option<u64> {
        if self.faulted || self.restored() || self.paused() || self.cancelled() {
            return None;
        }
        if let Some(retry) = self.retry.clone() {
            if self.selection().is_none() && self.select(retry.scheduled_at).is_err() {
                self.faulted = true;
                return None;
            }
            return Some(retry.retry_at);
        }
        if let Some(selection) = self.selection() {
            return Some(selection.occurrence.scheduled_at);
        }
        let at = if let Some(next) = self.schedule.after_settlement(self.through()) {
            next
        } else {
            if self.start_batch(now).is_err() {
                self.faulted = true;
                return None;
            }
            self.batch
                .as_ref()
                .map(|batch| batch.pending[0])
                .or_else(|| {
                    self.schedule
                        .next_after(now.max(self.through().unwrap_or(0)), u64::MAX)
                })
        };
        let at = at?;
        match self.select(at) {
            Ok(_) => Some(at),
            Err(_) => {
                self.faulted = true;
                None
            }
        }
    }

    /// Commit the current selection before the adapter invokes consumer code.
    pub(crate) fn accept_dispatch(&mut self, now: u64) -> Result<(Selection, bool, u32), Error> {
        if self.faulted {
            return Err(Error::Exhausted);
        }
        let selection = self.selection().ok_or(Error::Exhausted)?;
        let deliver = if let Some(retry) = &self.retry {
            if now < retry.retry_at || retry.scheduled_at != selection.occurrence.scheduled_at {
                return Err(Error::Exhausted);
            }
            true
        } else {
            self.prepare(selection.occurrence.scheduled_at, now)?
        };
        self.accept(selection, now).map_err(|_| Error::Exhausted)?;
        Ok((selection, deliver, self.attempt()))
    }

    pub(crate) fn attempt(&self) -> u32 {
        if let Some(active) = self.active
            && let Phase::Restored(number) = active.phase
        {
            return number;
        }
        self.retry
            .as_ref()
            .map_or(1, |retry| retry.failed_attempts + 1)
    }

    /// A frozen batch is never recomputed, including after pause or restoration.
    fn prepare(&mut self, at: u64, now: u64) -> Result<bool, Error> {
        let age = now.checked_sub(at).ok_or(Error::Exhausted)?;
        match self.policy {
            Policy::SkipLate { lateness_ns } => Ok(age <= lateness_ns),
            Policy::CatchUp(limit) if self.schedule.after_settlement(self.through()).is_some() => {
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
                    return Err(Error::Exhausted);
                }
                self.batch = Some(
                    CatchUpProgress::new(plan.occurrences(), plan.through())
                        .map_err(|_| Error::Exhausted)?,
                );
                Ok(true)
            }
        }
    }

    /// Only the current definition may advance its completion horizon. A newer
    /// owner edit retains its own horizon even when old accepted work finishes.
    pub(crate) fn finish(
        &mut self,
        selection: Selection,
        now: u64,
        retry_safe: bool,
    ) -> Result<(), Error> {
        let current = selection.occurrence.revision == self.revision();
        if current
            && retry_safe
            && let Some(retry_at) = self.retry_policy.next_retry(self.attempt(), now)
        {
            let retry = crate::RetryProgress::new(
                selection.occurrence.scheduled_at,
                self.attempt(),
                now,
                retry_at,
            )
            .map_err(|_| Error::Exhausted)?;
            self.retry(selection).map_err(|_| Error::Exhausted)?;
            self.retry = Some(retry);
            return Ok(());
        }
        self.complete(selection).map_err(|_| Error::Exhausted)?;
        self.retry = None;
        if current && self.schedule.after_settlement(self.through()).is_some() {
            self.skip_through(now).map_err(|_| Error::Exhausted)?;
        }
        self.finish_batch(selection, now)
    }

    /// Called only after normal settlement or explicit effect reconciliation.
    fn finish_batch(&mut self, selection: Selection, now: u64) -> Result<(), Error> {
        let Some(batch) = &mut self.batch else {
            return Ok(());
        };
        if batch.pending[0] != selection.occurrence.scheduled_at {
            return Err(Error::Exhausted);
        }
        batch.pending.remove(0);
        if batch.pending.is_empty() {
            // Do not replenish the quota from work that arrived during this batch.
            let through = now.max(batch.through);
            self.skip_through(through).map_err(|_| Error::Exhausted)?;
            self.batch = None;
        }
        Ok(())
    }

    pub(crate) fn recover(
        &mut self,
        selection: Selection,
        disposition: RecoveryDisposition,
        now: u64,
    ) -> Result<(), Error> {
        // Check exact authority before considering policy or mutating progress.
        if self.selection() != Some(selection) {
            return Err(Error::Stale);
        }
        let attempt = crate::Attempt::new(
            selection.occurrence.revision,
            selection.occurrence.scheduled_at,
            self.attempt(),
        )
        .map_err(|_| Error::Exhausted)?;
        let result = if disposition == RecoveryDisposition::RetrySafe {
            crate::RecordedResult::RetryAuthorized
        } else {
            crate::RecordedResult::Retired
        };
        let record =
            crate::RecordedOutcome::new(attempt, result, now).map_err(|_| Error::Exhausted)?;
        if disposition == RecoveryDisposition::RetrySafe {
            if selection.occurrence.revision != self.revision() {
                return Err(Error::RetryUnavailable);
            }
            let retry_at = self
                .retry_policy
                .next_retry(self.attempt(), now)
                .ok_or(Error::RetryUnavailable)?;
            let retry = crate::RetryProgress::new(
                selection.occurrence.scheduled_at,
                self.attempt(),
                now,
                retry_at,
            )
            .map_err(|_| Error::RetryUnavailable)?;
            self.reconcile_retry(selection).map_err(|_| Error::Stale)?;
            self.retry = Some(retry);
        } else {
            self.reconcile(selection).map_err(|_| Error::Stale)?;
            self.batch = None;
            self.retry = None;
            self.skip_through(now).map_err(|_| Error::Exhausted)?;
            self.pause().map_err(|_| Error::Exhausted)?;
        }
        self.last_outcome = Some(record);
        Ok(())
    }
}
