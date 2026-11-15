//! Private occurrence progress; owns neither recurrence calculation nor execution.
//!
//! The timer adapter must authenticate a job before forwarding a selection and
//! prove termination before blocking it. These tokens are local, not persisted
//! capabilities or permission to replay business effects.

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
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Active {
    selection: Selection,
    phase: Phase,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Error {
    Paused,
    Busy,
    RecoveryRequired,
    BeforeCursor,
    NotDue,
    Stale,
    Exhausted,
}

/// One job's revision and exclusive progress cursor. The host's stable job key
/// must accompany Occurrence when constructing durable business identities.
pub(super) struct Progress {
    revision: u64,
    through: Option<u64>,
    paused: bool,
    active: Option<Active>,
    sequence: u64,
    faulted: bool,
}
impl Progress {
    #[cfg(any(feature = "execution", test))]
    pub(super) fn checkpoint(&self) -> Result<crate::checkpoint::Checkpoint, Error> {
        self.check()?;
        let selected_at = match self.active {
            None => None,
            Some(Active {
                selection,
                phase: Phase::Queued,
            }) => Some(selection.occurrence.scheduled_at),
            Some(Active {
                phase: Phase::Blocked,
                ..
            }) => return Err(Error::RecoveryRequired),
            Some(_) => return Err(Error::Busy),
        };
        Ok(crate::checkpoint::Checkpoint {
            revision: self.revision,
            through: self.through,
            selected_at,
            paused: self.paused,
            catch_up: None,
        })
    }

    #[cfg(any(feature = "execution", test))]
    pub(super) fn restore(checkpoint: &crate::checkpoint::Checkpoint) -> Self {
        Self {
            revision: checkpoint.revision,
            through: checkpoint.through,
            paused: checkpoint.paused,
            active: checkpoint.selected_at.map(|scheduled_at| Active {
                selection: Selection {
                    occurrence: Occurrence {
                        revision: checkpoint.revision,
                        scheduled_at,
                    },
                    sequence: 1,
                },
                phase: Phase::Queued,
            }),
            sequence: u64::from(checkpoint.selected_at.is_some()),
            faulted: false,
        }
    }
    pub(super) const fn new(through: u64) -> Self {
        Self {
            revision: 1,
            through: Some(through),
            paused: false,
            active: None,
            sequence: 0,
            faulted: false,
        }
    }
    #[cfg(any(test, all(feature = "execution", target_arch = "wasm32")))]
    pub(super) fn unstarted() -> Self {
        Self {
            through: None,
            ..Self::new(0)
        }
    }
    pub(super) const fn through(&self) -> Option<u64> {
        self.through
    }

    #[cfg(all(feature = "execution", target_arch = "wasm32"))]
    pub(super) fn skip_through(&mut self, through: u64) -> Result<(), Error> {
        self.check()?;
        if self.active.is_some() {
            return Err(Error::Busy);
        }
        self.through = self.through.max(Some(through));
        Ok(())
    }

    #[cfg(all(feature = "execution", target_arch = "wasm32"))]
    pub(super) fn selection(&self) -> Option<Selection> {
        self.active.map(|active| active.selection)
    }

    #[cfg(all(feature = "execution", target_arch = "wasm32"))]
    pub(super) const fn paused(&self) -> bool {
        self.paused
    }

    /// Admit a supplier-selected occurrence strictly after settled progress.
    /// Calendar and missed-run policy are deliberately outside this state core.
    pub(super) fn select(&mut self, scheduled_at: u64) -> Result<Selection, Error> {
        self.check()?;
        if let Some(active) = self.active {
            return Err(if active.phase == Phase::Blocked {
                Error::RecoveryRequired
            } else {
                Error::Busy
            });
        }
        if self.paused {
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
        self.active = Some(Active {
            selection,
            phase: Phase::Blocked,
        });
        Ok(())
    }

    /// Owner reconciliation retires the uncertain occurrence without replaying it.
    /// The adapter must also reconcile the exact current Timers recovery token.
    pub(super) fn reconcile(&mut self, selection: Selection) -> Result<(), Error> {
        self.check()?;
        if self.active
            != Some(Active {
                selection,
                phase: Phase::Blocked,
            })
        {
            return Err(Error::Stale);
        }
        self.retire(selection);
        Ok(())
    }

    pub(super) fn pause(&mut self) -> Result<(), Error> {
        self.check()?;
        self.paused = true;
        self.cancel_queued();
        Ok(())
    }
    pub(super) fn resume(&mut self) -> Result<(), Error> {
        self.check()?;
        self.paused = false;
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
    fn cancel_queued(&mut self) {
        if self
            .active
            .is_some_and(|active| active.phase == Phase::Queued)
        {
            self.active = None;
        }
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

#[cfg(test)]
mod tests;
