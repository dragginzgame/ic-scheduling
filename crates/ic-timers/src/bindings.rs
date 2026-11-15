//! Shared retained declarations and callback ownership for Once and watchdogs.
//!
//! This private runtime is not a provider: setting demand does not arm a platform
//! timer. Wire tokens and host lifecycle initialization belong to transport.

use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::{Rc, Weak},
};

use crate::engine::{Attempt, ControlError, Demand, OnceState, Recovery, State, Ticket};

pub(super) const CAPACITY: usize = 64;
type WorkFuture = Pin<Box<dyn Future<Output = Demand>>>;
type Factory = Rc<RefCell<dyn FnMut() -> WorkFuture>>;

#[derive(Debug, Eq, PartialEq)]
pub(super) enum Error {
    InvalidIdentity,
    DuplicateIdentity,
    Capacity,
    StaleAuthority,
    Control(ControlError),
}

struct Entry {
    identity: [String; 3],
    kind: Kind,
}

enum Kind {
    Once {
        state: OnceState,
        factory: Factory,
    },
    Watchdog {
        state: crate::watchdog::WatchdogState,
        callback: watchdog::Callback,
    },
}
impl Kind {
    fn once(&mut self) -> Result<(&mut OnceState, &Factory), Error> {
        match self {
            Self::Once { state, factory } => Ok((state, factory)),
            Self::Watchdog { .. } => Err(Error::StaleAuthority),
        }
    }
}

struct Table {
    entries: Vec<Entry>,
    cursor: usize,
}

/// Only this owner retains the runtime. Dropping a handle leaves its entry intact.
pub(super) struct Bindings(Rc<RefCell<Table>>);

/// Authority is the exact live table allocation plus an append-only slot.
/// No removal/reuse operation exists. Neither handles nor tickets are serialized.
pub(super) struct Handle {
    table: Weak<RefCell<Table>>,
    slot: usize,
}

#[derive(Clone)]
pub(super) struct Dispatch {
    table: Weak<RefCell<Table>>,
    slot: usize,
    ticket: Ticket,
}

#[derive(Clone)]
pub(super) struct Accepted {
    table: Weak<RefCell<Table>>,
    slot: usize,
    attempt: Attempt,
}

#[derive(Clone)]
pub(super) enum Scheduled {
    Once(Dispatch),
    Watchdog(watchdog::Dispatch),
}

/// Recovery authority includes its declaration, not only an attempt counter.
pub(super) struct RecoveryKey {
    table: Weak<RefCell<Table>>,
    slot: usize,
    recovery: Recovery,
}

impl Bindings {
    pub(super) fn new() -> Self {
        Self(Rc::new(RefCell::new(Table {
            entries: Vec::new(),
            cursor: 0,
        })))
    }

    pub(super) fn bind<F, Fut>(&self, identity: [&str; 3], mut work: F) -> Result<Handle, Error>
    where
        F: FnMut() -> Fut + 'static,
        Fut: Future<Output = Demand> + 'static,
    {
        self.insert(
            identity,
            Kind::Once {
                state: OnceState::new(),
                factory: Rc::new(RefCell::new(move || -> WorkFuture { Box::pin(work()) })),
            },
        )
    }

    fn insert(&self, identity: [&str; 3], kind: Kind) -> Result<Handle, Error> {
        if identity
            .iter()
            .any(|part| part.is_empty() || part.len() > 64)
        {
            return Err(Error::InvalidIdentity);
        }
        // Function parameters drop after local guards: rejected captures are
        // released only after the table borrow, including on early returns.
        let mut table = self.0.borrow_mut();
        if table.entries.iter().any(|entry| entry.identity == identity) {
            return Err(Error::DuplicateIdentity);
        }
        if table.entries.len() == CAPACITY {
            return Err(Error::Capacity);
        }
        let slot = table.entries.len();
        table.entries.push(Entry {
            identity: identity.map(str::to_owned),
            kind,
        });
        Ok(Handle {
            table: Rc::downgrade(&self.0),
            slot,
        })
    }

    /// Return one due reservation, rotating across the bounded table. Transport
    /// must reserve flight capacity and suppress duplicate submission itself.
    pub(super) fn select(&self, now: u64) -> Option<Scheduled> {
        let mut table = self.0.borrow_mut();
        let count = table.entries.len();
        for _ in 0..count {
            let slot = table.cursor;
            table.cursor = (slot + 1) % count;
            match &mut table.entries[slot].kind {
                Kind::Once { state, .. } => {
                    if let Ok(Some(ticket)) = state.due(now) {
                        return Some(Scheduled::Once(Dispatch {
                            table: Rc::downgrade(&self.0),
                            slot,
                            ticket,
                        }));
                    }
                }
                Kind::Watchdog { state, .. } => {
                    if let Some(ticket) = state.pending() {
                        return Some(Scheduled::Watchdog(watchdog::Dispatch {
                            table: Rc::downgrade(&self.0),
                            slot,
                            ticket,
                        }));
                    }
                }
            }
        }
        None
    }

    /// Select delivery and cadence wakes in one pass without copying flight records.
    pub(super) fn wake<'a>(
        &self,
        submitted: &(impl Iterator<Item = &'a Scheduled> + Clone),
    ) -> (Option<u64>, Option<u64>) {
        let table = self.0.borrow();
        let owner = Rc::downgrade(&self.0);
        let mut eligible = None;
        let mut cadence = None;
        for (slot, entry) in table.entries.iter().enumerate() {
            let at = match &entry.kind {
                Kind::Once { state, .. } => match state.state() {
                    State::Armed(at)
                        if !submitted.clone().any(|item| {
                            matches!(item, Scheduled::Once(dispatch)
                            if dispatch.slot == slot && Weak::ptr_eq(&dispatch.table, &owner)
                                && state.has_ticket(dispatch.ticket))
                        }) =>
                    {
                        Some(at)
                    }
                    _ => None,
                },
                Kind::Watchdog { state, .. } => {
                    cadence = cadence.into_iter().chain(state.deadline()).min();
                    state
                        .pending()
                        .filter(|ticket| {
                            !submitted.clone().any(|item| {
                                matches!(item, Scheduled::Watchdog(dispatch)
                            if dispatch.slot == slot && dispatch.ticket == *ticket
                                && Weak::ptr_eq(&dispatch.table, &owner))
                            })
                        })
                        .map(|_| 0)
                }
            };
            if let Some(at) = at {
                eligible = Some(eligible.map_or(at, |current: u64| current.min(at)));
            }
        }
        (eligible, cadence)
    }

    /// Initial-message rejection rolled acceptance back; reserve its attempt
    /// without invoking user code, then block replay. Superseded tickets do nothing.
    #[cfg(target_arch = "wasm32")]
    pub(super) fn rejected(&self, dispatch: &Dispatch) -> Result<bool, Error> {
        if !Weak::ptr_eq(&dispatch.table, &Rc::downgrade(&self.0)) {
            return Err(Error::StaleAuthority);
        }
        let mut table = self.0.borrow_mut();
        let entry = table
            .entries
            .get_mut(dispatch.slot)
            .ok_or(Error::StaleAuthority)?;
        let (state, _) = entry.kind.once()?;
        if let Some(attempt) = state.accept(dispatch.ticket).map_err(Error::Control)? {
            return Ok(state.abandon(attempt));
        }
        Ok(false)
    }

    /// Authenticate local authority and enter Running before invoking the factory.
    /// No table borrow survives into consumer code or future polling/destruction.
    pub(super) fn start(
        &self,
        dispatch: &Dispatch,
    ) -> Result<Option<(Accepted, WorkFuture)>, Error> {
        if !Weak::ptr_eq(&dispatch.table, &Rc::downgrade(&self.0)) {
            return Err(Error::StaleAuthority);
        }
        let selected = {
            let mut table = self.0.borrow_mut();
            let entry = table
                .entries
                .get_mut(dispatch.slot)
                .ok_or(Error::StaleAuthority)?;
            let (state, factory) = entry.kind.once()?;
            state
                .accept(dispatch.ticket)
                .map_err(Error::Control)?
                .map(|attempt| (attempt, Rc::clone(factory)))
        };
        let Some((attempt, factory)) = selected else {
            return Ok(None);
        };
        let accepted = Accepted {
            table: Rc::downgrade(&self.0),
            slot: dispatch.slot,
            attempt,
        };
        let future = (factory.borrow_mut())();
        Ok(Some((accepted, future)))
    }
}

impl Handle {
    fn with_state<T>(&self, f: impl FnOnce(&mut OnceState) -> T) -> Result<T, Error> {
        let table = self.table.upgrade().ok_or(Error::StaleAuthority)?;
        let mut table = table.borrow_mut();
        let entry = table
            .entries
            .get_mut(self.slot)
            .ok_or(Error::StaleAuthority)?;
        Ok(f(entry.kind.once()?.0))
    }

    pub(super) fn set(&self, demand: Demand) -> Result<(), Error> {
        self.with_state(|state| state.set(demand))?
            .map_err(Error::Control)
    }

    pub(super) fn state(&self) -> Result<State, Error> {
        self.with_state(|state| state.state())
    }

    pub(super) fn recovery(&self) -> Result<Option<RecoveryKey>, Error> {
        Ok(match self.state()? {
            State::RecoveryRequired(recovery) => Some(RecoveryKey {
                table: self.table.clone(),
                slot: self.slot,
                recovery,
            }),
            _ => None,
        })
    }

    pub(super) fn recover(&self, key: &RecoveryKey, demand: Demand) -> Result<(), Error> {
        if self.slot != key.slot || !Weak::ptr_eq(&self.table, &key.table) {
            return Err(Error::StaleAuthority);
        }
        self.with_state(|state| state.recover(key.recovery, demand))?
            .map_err(Error::Control)
    }
}

impl Accepted {
    pub(super) fn complete(&self, next: Demand) -> Result<bool, Error> {
        Handle {
            table: self.table.clone(),
            slot: self.slot,
        }
        .with_state(|state| state.complete(self.attempt, next))
    }

    /// Requires transport/executor proof of local termination. Dropping the
    /// future or an Accepted value does not automatically authorize recovery.
    pub(super) fn abandon(&self) -> Result<bool, Error> {
        Handle {
            table: self.table.clone(),
            slot: self.slot,
        }
        .with_state(|state| state.abandon(self.attempt))
    }
}

#[cfg(test)]
mod tests;

pub(super) mod watchdog;

impl Dispatch {
    #[cfg(target_arch = "wasm32")]
    pub(super) fn same(&self, other: &Self) -> bool {
        self.slot == other.slot
            && self.ticket == other.ticket
            && Weak::ptr_eq(&self.table, &other.table)
    }
}

impl Accepted {
    #[cfg(target_arch = "wasm32")]
    pub(super) fn is_running(&self) -> Result<bool, Error> {
        Handle { table: self.table.clone(), slot: self.slot }.with_state(|state|
            matches!(state.state(), State::Running { attempt, .. } if attempt == self.attempt))
    }
}
