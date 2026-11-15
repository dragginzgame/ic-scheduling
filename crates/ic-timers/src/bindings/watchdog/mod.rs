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

pub(super) type Callback = Rc<RefCell<dyn FnMut() -> (Next, crate::Observation)>>;

pub(crate) struct WatchdogHandle(Handle);
#[derive(Clone)]
pub(crate) struct Dispatch {
    pub(super) table: Weak<RefCell<Table>>,
    pub(super) slot: usize,
    pub(super) incarnation: u64,
    pub(super) ticket: Ticket,
}
impl Bindings {
    pub(crate) fn bind_watchdog<F>(
        &self,
        identity: [&str; 3],
        cadence: NonZeroU64,
        lifetime: crate::Lifetime,
        work: F,
    ) -> Result<WatchdogHandle, Error>
    where
        F: FnMut() -> (Next, crate::Observation) + 'static,
    {
        self.insert(
            identity,
            lifetime,
            Kind::Watchdog {
                state: WatchdogState::new(cadence),
                callback: Rc::new(RefCell::new(work)),
            },
        )
        .map(WatchdogHandle)
    }

    /// Advance all watchdog successors in the control message, independently of
    /// transport capacity. This bounded pass invokes no consumer code.
    pub(crate) fn advance_watchdogs(&self, now: u64) {
        for entry in self.0.borrow_mut().entries.iter_mut().flatten() {
            if let Kind::Watchdog { state, .. } = &mut entry.kind {
                // Exhaustion is retained as terminal entry state for status.
                let _ = state.advance(now);
            }
        }
    }
    /// Invoke only in an isolated work message, after the control turn commits.
    /// No table borrow spans callback invocation or reentrant owner commands.
    pub(crate) fn run_watchdog(&self, dispatch: &Dispatch) -> Result<bool, Error> {
        if !Weak::ptr_eq(&dispatch.table, &Rc::downgrade(&self.0)) {
            return Err(Error::StaleAuthority);
        }
        let callback = {
            let mut table = self.0.borrow_mut();
            let entry = match table.entry(dispatch.slot, dispatch.incarnation) {
                Ok(entry) => entry,
                Err(_) => return Ok(false),
            };
            let Kind::Watchdog { state, callback } = &mut entry.kind else {
                return Err(Error::StaleAuthority);
            };
            if !state.accept(dispatch.ticket) {
                return Ok(false);
            }
            #[cfg(feature = "metrics")]
            entry.measurements.accepted.add(1);
            Rc::clone(callback)
        };
        let (next, _observation) = (callback.borrow_mut())();
        #[cfg(feature = "metrics")]
        dispatch
            .handle()
            .0
            .measure(|measurements| measurements.returned(_observation))?;
        dispatch
            .handle()
            .with_state(|state| state.complete(dispatch.ticket, next))
    }
}
impl WatchdogHandle {
    fn with_state<T>(&self, f: impl FnOnce(&mut WatchdogState) -> T) -> Result<T, Error> {
        let table = self.0.table.upgrade().ok_or(Error::StaleAuthority)?;
        let mut table = table.borrow_mut();
        let entry = table.entry(self.0.slot, self.0.incarnation)?;
        let Kind::Watchdog { state, .. } = &mut entry.kind else {
            return Err(Error::StaleAuthority);
        };
        Ok(f(state))
    }
    pub(crate) fn set(&self, demand: Demand, now: u64) -> Result<(), Error> {
        self.with_state(|state| state.set(demand, now))?
            .map_err(|_| Error::Control(ControlError::Exhausted))?;
        self.0.prune_stopped()
    }
    pub(crate) fn remove(&self) -> Result<(), Error> {
        self.0.remove()
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
            incarnation: self.incarnation,
        })
    }
    /// Definitive transport settlement only; never elapsed-time inference.
    pub(crate) fn settle(&self, _abandoned: bool) -> Result<bool, Error> {
        let _owner = self.table.upgrade().ok_or(Error::StaleAuthority)?;
        let handle = self.handle();
        let settled = match handle.with_state(|state| state.settle(self.ticket)) {
            Err(Error::StaleAuthority) => return Ok(false),
            result => result?,
        };
        if settled {
            #[cfg(feature = "metrics")]
            if _abandoned {
                handle.0.measure(|measurements| {
                    measurements.accepted.add(1);
                    measurements.abandoned.add(1);
                })?;
            }
            handle.0.prune_stopped()?;
        }
        Ok(settled)
    }
}

#[cfg(test)]
mod tests;

impl Dispatch {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn same(&self, other: &Self) -> bool {
        self.slot == other.slot
            && self.incarnation == other.incarnation
            && self.ticket == other.ticket
            && Weak::ptr_eq(&self.table, &other.table)
    }
}
