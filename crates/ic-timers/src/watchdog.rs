//! Watchdog cadence and one outstanding synchronous work ticket.
//!
//! Transport must commit `advance` in a control message before invoking work in
//! another message. This core cannot establish that platform commit boundary.
use std::num::NonZeroU64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Demand {
    Inactive,
    KeepCadence,
    At(u64),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Next {
    Stop,
    Continue,
    At(u64),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Ticket(u64);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Pending,
    Running { owner_changed: bool },
    Settling,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Work {
    ticket: Ticket,
    phase: Phase,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Exhausted;

/// Authority is local to one retained declaration; binding authenticates its owner.
#[derive(Debug)]
pub(super) struct WatchdogState {
    cadence: NonZeroU64,
    deadline: Option<u64>,
    work: Option<Work>,
    sequence: u64,
    faulted: bool,
}
impl WatchdogState {
    pub(super) const fn new(cadence: NonZeroU64) -> Self {
        Self {
            cadence,
            deadline: None,
            work: None,
            sequence: 0,
            faulted: false,
        }
    }
    pub(super) const fn deadline(&self) -> Option<u64> {
        self.deadline
    }
    pub(super) const fn outstanding(&self) -> bool {
        self.work.is_some()
    }
    pub(super) const fn faulted(&self) -> bool {
        self.faulted
    }

    pub(super) fn set(&mut self, demand: Demand, now: u64) -> Result<(), Exhausted> {
        self.check()?;
        let deadline = match demand {
            Demand::Inactive => None,
            Demand::At(at) => Some(at),
            Demand::KeepCadence => Some(match self.deadline {
                Some(at) => at,
                None => self.successor(now)?,
            }),
        };
        if let Some(work) = &mut self.work {
            match &mut work.phase {
                // A single synchronous callback can only have one pending owner
                // override. A flag records equal commands without a revision counter.
                Phase::Running { owner_changed } => *owner_changed = true,
                Phase::Pending if deadline != self.deadline || deadline.is_none() => {
                    self.work = None;
                }
                Phase::Pending | Phase::Settling => {}
            }
        }
        self.deadline = deadline;
        Ok(())
    }

    /// Skip missed periods and prearm from control time, never from old phase.
    /// An outstanding ticket survives cadence advancement, even while settling.
    pub(super) fn advance(&mut self, now: u64) -> Result<(), Exhausted> {
        self.check()?;
        if !self.deadline.is_some_and(|at| at <= now) {
            return Ok(());
        }
        let successor = self.successor(now)?;
        if self.work.is_none() {
            let Some(sequence) = self.sequence.checked_add(1) else {
                return self.exhaust();
            };
            self.sequence = sequence;
            self.work = Some(Work {
                ticket: Ticket(sequence),
                phase: Phase::Pending,
            });
        }
        self.deadline = Some(successor);
        Ok(())
    }

    /// Pending delivery remains eligible between cadence turns after refusal.
    pub(super) fn pending(&self) -> Option<Ticket> {
        match self.work {
            Some(Work {
                ticket,
                phase: Phase::Pending,
            }) => Some(ticket),
            _ => None,
        }
    }
    pub(super) fn accept(&mut self, ticket: Ticket) -> bool {
        if self.pending() != Some(ticket) {
            return false;
        }
        self.work = Some(Work {
            ticket,
            phase: Phase::Running {
                owner_changed: false,
            },
        });
        true
    }
    pub(super) fn complete(&mut self, ticket: Ticket, next: Next) -> bool {
        let Some(Work {
            ticket: current,
            phase: Phase::Running { owner_changed },
        }) = self.work
        else {
            return false;
        };
        if current != ticket {
            return false;
        }
        if !owner_changed {
            match next {
                Next::Stop => self.deadline = None,
                Next::At(at) => self.deadline = Some(at),
                Next::Continue => {}
            }
        }
        self.work = Some(Work {
            ticket,
            phase: Phase::Settling,
        });
        true
    }

    /// Call only after definitive transport settlement/local termination. A work
    /// trap preserves the prearmed successor; mere timeout is not settlement.
    pub(super) fn settle(&mut self, ticket: Ticket) -> bool {
        if !self.work.is_some_and(|work| work.ticket == ticket) {
            return false;
        }
        self.work = None;
        true
    }
    fn successor(&mut self, now: u64) -> Result<u64, Exhausted> {
        match now.checked_add(self.cadence.get()) {
            Some(at) => Ok(at),
            None => self.exhaust(),
        }
    }
    fn check(&self) -> Result<(), Exhausted> {
        if self.faulted { Err(Exhausted) } else { Ok(()) }
    }
    fn exhaust<T>(&mut self) -> Result<T, Exhausted> {
        self.faulted = true;
        self.deadline = None;
        self.work = None;
        Err(Exhausted)
    }
}

#[cfg(test)]
mod tests;
