//! Retained declarations and callback ownership, awaiting the IC transport.
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

const CAPACITY: usize = 64;
type WorkFuture = Pin<Box<dyn Future<Output = Demand>>>;
type Factory = Rc<RefCell<Box<dyn FnMut() -> WorkFuture>>>;

#[derive(Debug, Eq, PartialEq)]
pub(super) enum Error {
    InvalidIdentity,
    DuplicateIdentity,
    Capacity,
    StaleAuthority,
    Control(ControlError),
}

#[derive(Eq, PartialEq)]
struct Identity([String; 3]);

struct Entry {
    identity: Identity,
    state: OnceState,
    factory: Factory,
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

pub(super) struct Dispatch {
    table: Weak<RefCell<Table>>,
    slot: usize,
    ticket: Ticket,
}

pub(super) struct Accepted {
    table: Weak<RefCell<Table>>,
    slot: usize,
    attempt: Attempt,
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
        if identity
            .iter()
            .any(|part| part.is_empty() || part.len() > 64)
        {
            return Err(Error::InvalidIdentity);
        }
        let identity = Identity(identity.map(str::to_owned));
        // The outer owner ensures rejected captures drop after the table borrow.
        let mut factory: Option<Factory> =
            Some(Rc::new(RefCell::new(Box::new(move || Box::pin(work())))));
        let result = {
            let mut table = self.0.borrow_mut();
            if table.entries.iter().any(|entry| entry.identity == identity) {
                Err(Error::DuplicateIdentity)
            } else if table.entries.len() == CAPACITY {
                Err(Error::Capacity)
            } else {
                let slot = table.entries.len();
                table.entries.push(Entry {
                    identity,
                    state: OnceState::new(),
                    factory: factory.take().expect("factory retained until admission"),
                });
                Ok(Handle {
                    table: Rc::downgrade(&self.0),
                    slot,
                })
            }
        };
        drop(factory);
        result
    }

    /// Return one due reservation, rotating across the bounded table. Transport
    /// must reserve flight capacity and suppress duplicate submission itself.
    pub(super) fn due(&self, now: u64) -> Option<Dispatch> {
        let mut table = self.0.borrow_mut();
        let count = table.entries.len();
        for _ in 0..count {
            let slot = table.cursor;
            table.cursor = (slot + 1) % count;
            if let Ok(Some(ticket)) = table.entries[slot].state.due(now) {
                return Some(Dispatch {
                    table: Rc::downgrade(&self.0),
                    slot,
                    ticket,
                });
            }
        }
        None
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
            entry
                .state
                .accept(dispatch.ticket)
                .map_err(Error::Control)?
                .map(|attempt| (attempt, Rc::clone(&entry.factory)))
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
        Ok(f(&mut entry.state))
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
