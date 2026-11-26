//! Once eligibility and attempt custody, without callbacks or platform effects.
//!
//! Each value belongs to one retained declaration. Bindings authenticate the
//! table, slot and incarnation before passing a token to this state machine.

#[cfg(test)]
mod tests;

pub(super) use crate::OnceDemand as Demand;

///
/// Ticket
/// Identifies a reservation for delivery within one declaration.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Ticket(u64);

///
/// Attempt
/// Identifies the accepted callback that owns settlement.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Attempt(u64);

///
/// Recovery
/// Binds explicit recovery to the abandoned attempt.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Recovery(Attempt);

///
/// ControlError
/// Reports why a requested Once state transition could not proceed.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ControlError {
    RecoveryRequired,
    StaleRecovery,
    Exhausted,
}

///
/// State
/// Tracks eligibility, callback custody and pending owner demand.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum State {
    Inactive,
    Armed(u64),
    Running {
        attempt: Attempt,
        owner: Option<Demand>,
    },
    RecoveryRequired(Recovery),
    Faulted,
}

///
/// OnceState
/// One authoritative entry's Once state. Tokens are declaration-local, not handles.
///

#[derive(Debug)]
pub(super) struct OnceState {
    state: State,
    ticket: Option<Ticket>,
    retry_at: u64,
    ticket_counter: u64,
    attempt_counter: u64,
}

impl OnceState {
    pub(super) const fn new() -> Self {
        Self {
            state: State::Inactive,
            ticket: None,
            retry_at: 0,
            ticket_counter: 0,
            attempt_counter: 0,
        }
    }

    pub(super) const fn state(&self) -> State {
        self.state
    }

    /// Owner decisions during a run always override its result, even if equal.
    pub(super) fn set(&mut self, demand: Demand) -> Result<(), ControlError> {
        match &mut self.state {
            State::Faulted => Err(ControlError::Exhausted),
            State::RecoveryRequired(_) => match demand {
                Demand::Inactive => Ok(()),
                Demand::At(_) => Err(ControlError::RecoveryRequired),
            },
            State::Running { owner, .. } => {
                *owner = Some(demand);

                Ok(())
            }
            State::Armed(at) if demand == Demand::At(*at) => Ok(()),
            State::Inactive | State::Armed(_) => {
                self.apply(demand);

                Ok(())
            }
        }
    }

    pub(super) fn has_ticket(&self, ticket: Ticket) -> bool {
        self.ticket == Some(ticket)
    }

    pub(super) fn eligible_at(&self) -> Option<u64> {
        match self.state {
            State::Armed(at) => Some(at.max(self.retry_at)),
            _ => None,
        }
    }

    /// Retire only the rejected reservation. Owner status retains its original deadline.
    pub(super) fn defer(&mut self, ticket: Ticket, now: u64) -> Result<bool, ControlError> {
        const BACKOFF_NS: u64 = 60_000_000_000;
        if !self.has_ticket(ticket) {
            return Ok(false);
        }
        let Some(at) = now.checked_add(BACKOFF_NS) else {
            return self.exhaust();
        };
        self.ticket = None;
        self.retry_at = at;

        Ok(true)
    }

    /// Reserve one ticket for due work. Repeated selection retains that ticket;
    /// transport must track submission separately and must not submit it twice.
    pub(super) fn due(&mut self, now: u64) -> Result<Option<Ticket>, ControlError> {
        if self.state == State::Faulted {
            return Err(ControlError::Exhausted);
        }
        if self.eligible_at().is_none_or(|at| at > now) {
            return Ok(None);
        }
        if let Some(ticket) = self.ticket {
            return Ok(Some(ticket));
        }
        let Some(next) = self.ticket_counter.checked_add(1) else {
            return self.exhaust();
        };
        self.ticket_counter = next;
        let ticket = Ticket(next);
        self.ticket = Some(ticket);

        Ok(Some(ticket))
    }

    /// Enter Running before calling a callback factory. Stale tickets do nothing.
    pub(super) fn accept(&mut self, ticket: Ticket) -> Result<Option<Attempt>, ControlError> {
        if self.ticket != Some(ticket) || !matches!(self.state, State::Armed(_)) {
            return Ok(None);
        }
        let Some(next) = self.attempt_counter.checked_add(1) else {
            return self.exhaust();
        };
        self.attempt_counter = next;
        let attempt = Attempt(next);
        self.ticket = None;
        self.state = State::Running {
            attempt,
            owner: None,
        };

        Ok(Some(attempt))
    }

    /// Settle exactly the current attempt; a stale completion has no authority.
    pub(super) fn complete(&mut self, accepted: Attempt, next: Demand) -> bool {
        match self.state {
            State::Running { attempt, owner } if attempt == accepted => {
                self.apply(owner.unwrap_or(next));

                true
            }
            _ => false,
        }
    }

    /// Only call with proof of local termination, never merely a timeout.
    /// Discard queued demand because it predates discovery of uncertain effects.
    pub(super) fn abandon(&mut self, accepted: Attempt) -> bool {
        match self.state {
            State::Running { attempt, .. } if attempt == accepted => {
                self.state = State::RecoveryRequired(Recovery(attempt));

                true
            }
            _ => false,
        }
    }

    pub(super) fn recover(&mut self, token: Recovery, demand: Demand) -> Result<(), ControlError> {
        if self.state != State::RecoveryRequired(token) {
            return Err(ControlError::StaleRecovery);
        }
        self.apply(demand);

        Ok(())
    }

    fn apply(&mut self, demand: Demand) {
        self.retry_at = 0;
        self.ticket = None;
        self.state = match demand {
            Demand::Inactive => State::Inactive,
            Demand::At(at) => State::Armed(at),
        };
    }

    fn exhaust<T>(&mut self) -> Result<T, ControlError> {
        self.ticket = None;
        self.state = State::Faulted;

        Err(ControlError::Exhausted)
    }
}
