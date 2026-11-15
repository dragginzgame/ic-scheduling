//! Synchronous watchdog callbacks in the shared retained declaration table.
use super::{Bindings, Error, Handle, Kind, Table};
use crate::{
    engine::ControlError,
    watchdog::{Demand, Next, Ticket, WatchdogState},
};
use std::{
    cell::RefCell,
    num::NonZeroU64,
    rc::{Rc, Weak},
};

pub(super) type Callback = Rc<RefCell<Box<dyn FnMut() -> Next>>>;

pub(crate) struct WatchdogHandle(Handle);
#[derive(Clone)]
pub(crate) struct Dispatch {
    pub(super) table: Weak<RefCell<Table>>,
    pub(super) slot: usize,
    pub(super) ticket: Ticket,
}
impl Bindings {
    pub(crate) fn bind_watchdog<F>(
        &self,
        identity: [&str; 3],
        cadence: NonZeroU64,
        work: F,
    ) -> Result<WatchdogHandle, Error>
    where
        F: FnMut() -> Next + 'static,
    {
        self.insert(
            identity,
            Kind::Watchdog {
                state: WatchdogState::new(cadence),
                callback: Rc::new(RefCell::new(Box::new(work))),
            },
        )
        .map(WatchdogHandle)
    }

    /// Advance all watchdog successors in the control message, independently of
    /// transport capacity. This bounded pass invokes no consumer code.
    pub(crate) fn advance_watchdogs(&self, now: u64) {
        for entry in &mut self.0.borrow_mut().entries {
            if let Kind::Watchdog { state, .. } = &mut entry.kind {
                // Exhaustion is retained as terminal entry state for status.
                let _ = state.advance(now);
            }
        }
    }
    #[cfg(test)]
    pub(crate) fn watchdog_due(&self) -> Option<Dispatch> {
        let mut table = self.0.borrow_mut();
        let count = table.entries.len();
        for _ in 0..count {
            let slot = table.cursor;
            table.cursor = (slot + 1) % count;
            if let Kind::Watchdog { state, .. } = &table.entries[slot].kind
                && let Some(ticket) = state.pending()
            {
                return Some(Dispatch {
                    table: Rc::downgrade(&self.0),
                    slot,
                    ticket,
                });
            }
        }
        None
    }
    /// Invoke only in an isolated work message, after the control turn commits.
    /// No table borrow spans callback invocation or reentrant owner commands.
    pub(crate) fn run_watchdog(&self, dispatch: &Dispatch) -> Result<bool, Error> {
        if !Weak::ptr_eq(&dispatch.table, &Rc::downgrade(&self.0)) {
            return Err(Error::StaleAuthority);
        }
        let callback = {
            let mut table = self.0.borrow_mut();
            let entry = table
                .entries
                .get_mut(dispatch.slot)
                .ok_or(Error::StaleAuthority)?;
            let Kind::Watchdog { state, callback } = &mut entry.kind else {
                return Err(Error::StaleAuthority);
            };
            if !state.accept(dispatch.ticket) {
                return Ok(false);
            }
            Rc::clone(callback)
        };
        let next = (callback.borrow_mut())();
        dispatch
            .handle()
            .with_state(|state| state.complete(dispatch.ticket, next))
    }
}
impl WatchdogHandle {
    fn with_state<T>(&self, f: impl FnOnce(&mut WatchdogState) -> T) -> Result<T, Error> {
        let table = self.0.table.upgrade().ok_or(Error::StaleAuthority)?;
        let mut table = table.borrow_mut();
        let entry = table
            .entries
            .get_mut(self.0.slot)
            .ok_or(Error::StaleAuthority)?;
        let Kind::Watchdog { state, .. } = &mut entry.kind else {
            return Err(Error::StaleAuthority);
        };
        Ok(f(state))
    }
    pub(crate) fn set(&self, demand: Demand, now: u64) -> Result<(), Error> {
        self.with_state(|state| state.set(demand, now))?
            .map_err(|_| Error::Control(ControlError::Exhausted))
    }
    pub(crate) fn status(&self) -> Result<(Option<u64>, bool, bool), Error> {
        self.with_state(|state| (state.deadline(), state.outstanding(), state.faulted()))
    }
}
impl Dispatch {
    fn handle(&self) -> WatchdogHandle {
        WatchdogHandle(Handle {
            table: self.table.clone(),
            slot: self.slot,
        })
    }
    /// Definitive transport settlement only; never elapsed-time inference.
    pub(crate) fn settle(&self) -> Result<bool, Error> {
        self.handle().with_state(|state| state.settle(self.ticket))
    }
}

#[cfg(test)]
mod tests;

impl Dispatch {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn same(&self, other: &Self) -> bool {
        self.slot == other.slot
            && self.ticket == other.ticket
            && Weak::ptr_eq(&self.table, &other.table)
    }
}
