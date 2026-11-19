//! One owner for job progress, selection policy and reconstruction. Timers owns execution custody.
//!
//! The timer adapter must authenticate a job before forwarding a selection and
//! prove termination before blocking it. These tokens are local, not persisted
//! capabilities or permission to replay business effects.

mod planning;
#[cfg(test)]
mod tests;

use crate::{
    CatchUpProgress, Checkpoint, Policy, RecordedOutcome, RetryPolicy, RetryProgress, Schedule,
};

/// Owner disposition after reconciling the effects of an uncertain attempt.
/// Neither choice resumes the job; call `Job::resume` separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryDisposition {
    /// Retire this occurrence, discard its retry/batch and skip through now.
    Retire,
    /// The owner has established that repeating this occurrence's effects is
    /// safe. Preserve its identity and consumed budget, with backoff from now.
    /// Refuse if the definition changed, budget is exhausted or deadline overflows.
    RetrySafe,
}
/// Owner-controlled eligibility, persisted separately from execution custody.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlState {
    /// Work may become eligible according to its schedule and policy.
    Active,
    /// Work is suspended and may be resumed.
    Paused,
    /// Terminal cancellation; resume and schedule edits are refused.
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Occurrence {
    pub(super) revision: u64,
    pub(super) scheduled_at: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Selection {
    pub(super) occurrence: Occurrence,
    sequence: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Context {
    pub(super) occurrence: Occurrence,
    pub(super) started_at: u64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Queued,
    Accepted { started_at: u64 },
    Blocked,
    Restored(u32),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Active {
    selection: Selection,
    phase: Phase,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Error {
    Cancelled,
    Paused,
    Busy,
    RecoveryRequired,
    BeforeCursor,
    NotDue,
    Stale,
    Exhausted,
    RetryUnavailable,
}

/// One job's revision and exclusive progress cursor. The host's stable job key
/// must accompany Occurrence when constructing durable business identities.
pub(super) struct State {
    revision: u64,
    through: Option<u64>,
    control: ControlState,
    active: Option<Active>,
    sequence: u64,
    pub(super) faulted: bool,
    pub(super) schedule: Schedule,
    pub(super) policy: Policy,
    pub(super) retry_policy: RetryPolicy,
    pub(super) batch: Option<CatchUpProgress>,
    pub(super) retry: Option<RetryProgress>,
    pub(super) last_outcome: Option<RecordedOutcome>,
}
impl State {
    pub(super) fn unresolved_progress(&self) -> Result<crate::checkpoint::Checkpoint, Error> {
        self.check()?;
        // The adapter supplies provider Running/RecoveryRequired evidence. A
        // worker-message rollback can leave this local selection queued.
        if self.active.is_none() {
            return Err(Error::Stale);
        }

        Ok(crate::checkpoint::Checkpoint {
            unresolved: None,
            revision: self.revision,
            through: self.through,
            selected_at: None,
            control: ControlState::Paused,
            catch_up: self.batch.clone(),
            retry: None,
            last_outcome: self.last_outcome,
        })
    }

    pub(super) fn checkpoint(&self) -> Result<crate::checkpoint::Checkpoint, Error> {
        if self.faulted {
            return Err(Error::Exhausted);
        }
        let selected_at = match self.active {
            None => None,
            Some(Active {
                selection,
                phase: Phase::Queued,
            }) => Some(selection.occurrence.scheduled_at),
            Some(Active {
                phase: Phase::Blocked | Phase::Restored(_),
                ..
            }) => return Err(Error::RecoveryRequired),
            Some(_) => return Err(Error::Busy),
        };

        Ok(crate::checkpoint::Checkpoint {
            unresolved: None,
            revision: self.revision,
            through: self.through,
            selected_at,
            control: self.control,
            catch_up: self.batch.clone(),
            retry: self.retry.clone(),
            last_outcome: self.last_outcome,
        })
    }

    pub(super) fn from_checkpoint(
        schedule: Schedule,
        policy: Policy,
        retry_policy: RetryPolicy,
        checkpoint: Checkpoint,
    ) -> Self {
        let occurrence = checkpoint
            .unresolved
            .map(|(attempt, _)| Occurrence {
                revision: attempt.revision,
                scheduled_at: attempt.scheduled_at,
            })
            .or_else(|| {
                checkpoint.selected_at.map(|scheduled_at| Occurrence {
                    revision: checkpoint.revision,
                    scheduled_at,
                })
            });

        Self {
            revision: checkpoint.revision,
            through: checkpoint.through,
            control: checkpoint.control,
            active: occurrence.map(|occurrence| Active {
                selection: Selection {
                    occurrence,
                    sequence: 1,
                },
                phase: checkpoint.unresolved.map_or(Phase::Queued, |(attempt, _)| {
                    Phase::Restored(attempt.number)
                }),
            }),
            sequence: u64::from(occurrence.is_some()),
            faulted: false,
            schedule,
            policy,
            retry_policy,
            batch: checkpoint.catch_up,
            retry: checkpoint.retry,
            last_outcome: checkpoint.last_outcome,
        }
    }

    pub(super) const fn through(&self) -> Option<u64> {
        self.through
    }

    pub(super) const fn revision(&self) -> u64 {
        self.revision
    }

    pub(super) fn skip_through(&mut self, through: u64) -> Result<(), Error> {
        self.check()?;
        if self.active.is_some() {
            return Err(Error::Busy);
        }
        self.through = self.through.max(Some(through));

        Ok(())
    }

    pub(super) fn selection(&self) -> Option<Selection> {
        self.active.map(|active| active.selection)
    }

    pub(super) const fn paused(&self) -> bool {
        matches!(self.control, ControlState::Paused)
    }

    pub(super) const fn cancelled(&self) -> bool {
        matches!(self.control, ControlState::Cancelled)
    }

    /// Admit a supplier-selected occurrence strictly after settled progress.
    /// Calendar and missed-run policy are deliberately outside this state core.
    pub(super) fn select(&mut self, scheduled_at: u64) -> Result<Selection, Error> {
        self.check()?;
        if let Some(active) = self.active {
            return Err(
                if matches!(active.phase, Phase::Blocked | Phase::Restored(_)) {
                    Error::RecoveryRequired
                } else {
                    Error::Busy
                },
            );
        }
        if self.control == ControlState::Paused {
            return Err(Error::Paused);
        }
        if self.through.is_some_and(|through| scheduled_at <= through) {
            return Err(Error::BeforeCursor);
        }
        let Some(sequence) = self.sequence.checked_add(1) else {
            return self.exhaust();
        };
        self.sequence = sequence;
        let selection = Selection {
            occurrence: Occurrence {
                revision: self.revision,
                scheduled_at,
            },
            sequence,
        };
        self.active = Some(Active {
            selection,
            phase: Phase::Queued,
        });

        Ok(selection)
    }

    /// Record identity before invoking consumer code, including its future factory.
    pub(super) fn accept(&mut self, selection: Selection, now: u64) -> Result<Context, Error> {
        self.check()?;
        if self.active
            != Some(Active {
                selection,
                phase: Phase::Queued,
            })
        {
            return Err(Error::Stale);
        }
        if now < selection.occurrence.scheduled_at {
            return Err(Error::NotDue);
        }
        self.active = Some(Active {
            selection,
            phase: Phase::Accepted { started_at: now },
        });

        Ok(Context {
            occurrence: selection.occurrence,
            started_at: now,
        })
    }

    /// Normal completion settles the occurrence regardless of business outcome.
    /// An old schedule's completion cannot rewrite newer schedule progress.
    pub(super) fn complete(&mut self, selection: Selection) -> Result<(), Error> {
        self.check()?;
        if !matches!(self.active, Some(Active { selection: current, phase: Phase::Accepted { .. } }) if current == selection)
        {
            return Err(Error::Stale);
        }
        self.retire(selection);

        Ok(())
    }

    /// A normally returned retry-safe failure may queue a fresh delivery token
    /// for the same occurrence. It does not retire progress or clear uncertainty.
    pub(super) fn retry(&mut self, selection: Selection) -> Result<(), Error> {
        self.check()?;
        if selection.occurrence.revision != self.revision
            || !matches!(self.active, Some(Active { selection: current, phase: Phase::Accepted { .. } }) if current == selection)
        {
            return Err(Error::Stale);
        }
        self.active = None;
        if self.control == ControlState::Active {
            self.select(selection.occurrence.scheduled_at)?;
        }

        Ok(())
    }

    /// Timer recovery evidence may refer to queued state after an initial-message
    /// rollback. Neither a timeout nor a returned business error is such evidence.
    pub(super) fn block(&mut self, selection: Selection) -> Result<(), Error> {
        self.check()?;
        if !self
            .active
            .is_some_and(|active| active.selection == selection)
        {
            return Err(Error::Stale);
        }
        if !self.restored() {
            self.active = Some(Active {
                selection,
                phase: Phase::Blocked,
            });
        }

        Ok(())
    }

    /// Owner reconciliation retires the uncertain occurrence without replaying it.
    /// The adapter must also reconcile the exact current Timers recovery token.
    pub(super) fn reconcile(&mut self, selection: Selection) -> Result<(), Error> {
        self.check()?;
        if !matches!(self.active, Some(Active { selection: current, phase: Phase::Blocked | Phase::Restored(_) }) if current == selection)
        {
            return Err(Error::Stale);
        }
        self.retire(selection);

        Ok(())
    }

    /// Explicit retry-safety evidence releases only this exact blocked attempt.
    /// Leave selection to resume, retaining the cursor and forcing owner pause.
    pub(super) fn reconcile_retry(&mut self, selection: Selection) -> Result<(), Error> {
        self.check()?;
        if selection.occurrence.revision != self.revision
            || !matches!(self.active, Some(Active { selection: current, phase: Phase::Blocked | Phase::Restored(_) }) if current == selection)
        {
            return Err(Error::Stale);
        }
        self.active = None;
        self.control = ControlState::Paused;

        Ok(())
    }

    pub(super) fn pause(&mut self) -> Result<(), Error> {
        self.check()?;
        self.control = ControlState::Paused;
        self.cancel_queued();

        Ok(())
    }

    pub(super) fn resume(&mut self) -> Result<(), Error> {
        self.check()?;
        self.control = ControlState::Active;

        Ok(())
    }

    /// A schedule edit invalidates unaccepted selection but preserves accepted or
    /// uncertain identity. The new schedule cannot move progress backward.
    pub(super) fn edit(&mut self, through: u64) -> Result<(), Error> {
        self.check()?;
        let Some(revision) = self.revision.checked_add(1) else {
            return self.exhaust();
        };
        self.revision = revision;
        self.through = self.through.max(Some(through));
        self.cancel_queued();

        Ok(())
    }

    /// Cancel only before acceptance. Neither cancellation nor losing a handle
    /// can erase running/uncertain identity or authorize effect replay.
    pub(super) fn cancel(&mut self) -> Result<(), Error> {
        self.check()?;
        if let Some(active) = self.active {
            match active.phase {
                Phase::Accepted { .. } => return Err(Error::Busy),
                Phase::Blocked | Phase::Restored(_) => return Err(Error::RecoveryRequired),
                Phase::Queued => {}
            }
        }
        self.active = None;
        self.control = ControlState::Cancelled;

        Ok(())
    }

    fn cancel_queued(&mut self) {
        if self
            .active
            .is_some_and(|active| active.phase == Phase::Queued)
        {
            self.active = None;
        }
    }

    pub(super) fn restored(&self) -> bool {
        matches!(
            self.active,
            Some(Active {
                phase: Phase::Restored(_),
                ..
            })
        )
    }

    fn retire(&mut self, selection: Selection) {
        if selection.occurrence.revision == self.revision {
            self.through = self.through.max(Some(selection.occurrence.scheduled_at));
        }
        self.active = None;
    }

    fn check(&self) -> Result<(), Error> {
        if self.faulted {
            Err(Error::Exhausted)
        } else if self.control == ControlState::Cancelled {
            Err(Error::Cancelled)
        } else {
            Ok(())
        }
    }

    fn exhaust<T>(&mut self) -> Result<T, Error> {
        // Retain unresolved identity for diagnosis rather than erasing uncertainty.
        self.faulted = true;

        Err(Error::Exhausted)
    }
}
