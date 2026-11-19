//! Shared retained declarations and callback ownership for Once and watchdogs.
//!
//! This private runtime is not a provider: setting demand does not arm a platform
//! timer. Wire tokens and host lifecycle initialization belong to transport.

#[cfg(test)]
mod tests;
pub(super) mod watchdog;

use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::{Rc, Weak},
};

use crate::engine::{Attempt, ControlError, Demand, OnceState, Recovery, State, Ticket};

pub(super) const CAPACITY: usize = 64;
type WorkFuture = Pin<Box<dyn Future<Output = (Demand, crate::Observation)>>>;
type Factory = Rc<RefCell<dyn FnMut() -> WorkFuture>>;

#[derive(Debug, Eq, PartialEq)]
pub(super) enum Error {
    InvalidIdentity,
    DuplicateIdentity,
    Capacity,
    Busy,
    StaleAuthority,
    Control(ControlError),
}

struct Entry {
    incarnation: u64,
    lifetime: crate::Lifetime,
    #[cfg(feature = "metrics")]
    measurements: crate::TimerMeasurements,
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
    entries: Vec<Option<Entry>>,
    next_incarnation: u64,
    cursor: usize,
}
impl Table {
    fn entry(&mut self, slot: usize, incarnation: u64) -> Result<&mut Entry, Error> {
        self.entries
            .get_mut(slot)
            .and_then(Option::as_mut)
            .filter(|entry| entry.incarnation == incarnation)
            .ok_or(Error::StaleAuthority)
    }
}

/// Only this owner retains the runtime. Dropping a handle leaves its entry intact.
pub(super) struct Bindings(Rc<RefCell<Table>>);

/// Authority includes table allocation, slot and a never-reused incarnation.
/// Neither handles nor tickets are serialized.
pub(super) struct Handle {
    table: Weak<RefCell<Table>>,
    slot: usize,
    incarnation: u64,
}

#[derive(Clone)]
pub(super) struct Dispatch {
    table: Weak<RefCell<Table>>,
    slot: usize,
    incarnation: u64,
    ticket: Ticket,
}

#[derive(Clone)]
pub(super) struct Accepted {
    table: Weak<RefCell<Table>>,
    slot: usize,
    incarnation: u64,
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
    incarnation: u64,
    recovery: Recovery,
}

impl Bindings {
    #[cfg(feature = "metrics")]
    pub(super) fn inventory(&self) -> Vec<crate::TimerSnapshot> {
        use crate::{TimerKind, TimerPhase};
        let table = self.0.borrow();
        let mut rows: Vec<_> = table
            .entries
            .iter()
            .flatten()
            .map(|entry| {
                let (kind, phase, deadline, outstanding) = match &entry.kind {
                    Kind::Once { state, .. } => {
                        let (phase, deadline) = match state.state() {
                            State::Inactive => (TimerPhase::Inactive, None),
                            State::Armed(at) => (TimerPhase::Armed, Some(at)),
                            State::Running { .. } => (TimerPhase::Running, None),
                            State::RecoveryRequired(_) => (TimerPhase::RecoveryRequired, None),
                            State::Faulted => (TimerPhase::Faulted, None),
                        };
                        (TimerKind::Once, phase, deadline, false)
                    }
                    Kind::Watchdog { state, .. } => (
                        TimerKind::Watchdog,
                        if state.faulted() {
                            TimerPhase::Faulted
                        } else if state.deadline().is_some() {
                            TimerPhase::Armed
                        } else {
                            TimerPhase::Inactive
                        },
                        state.deadline(),
                        state.outstanding(),
                    ),
                };
                crate::TimerSnapshot {
                    identity: entry.identity.clone(),
                    incarnation: entry.incarnation,
                    kind,
                    lifetime: entry.lifetime,
                    phase,
                    deadline,
                    outstanding,
                    measurements: entry.measurements,
                }
            })
            .collect();
        rows.sort_by(|a, b| a.identity.cmp(&b.identity));

        rows
    }

    pub(super) fn new() -> Self {
        Self(Rc::new(RefCell::new(Table {
            entries: Vec::new(),
            next_incarnation: 0,
            cursor: 0,
        })))
    }

    pub(super) fn bind<F, Fut>(
        &self,
        identity: [&str; 3],
        lifetime: crate::Lifetime,
        mut work: F,
    ) -> Result<Handle, Error>
    where
        F: FnMut() -> Fut + 'static,
        Fut: Future<Output = (Demand, crate::Observation)> + 'static,
    {
        self.insert(
            identity,
            lifetime,
            Kind::Once {
                state: OnceState::new(),
                factory: Rc::new(RefCell::new(move || -> WorkFuture { Box::pin(work()) })),
            },
        )
    }

    fn insert(
        &self,
        identity: [&str; 3],
        lifetime: crate::Lifetime,
        kind: Kind,
    ) -> Result<Handle, Error> {
        if identity
            .iter()
            .any(|part| part.is_empty() || part.len() > 64)
        {
            return Err(Error::InvalidIdentity);
        }
        // Function parameters drop after local guards: rejected captures are
        // released only after the table borrow, including on early returns.
        let mut table = self.0.borrow_mut();
        if table
            .entries
            .iter()
            .flatten()
            .any(|entry| entry.identity == identity)
        {
            return Err(Error::DuplicateIdentity);
        }
        let slot = table
            .entries
            .iter()
            .position(Option::is_none)
            .unwrap_or(table.entries.len());
        if slot == CAPACITY {
            return Err(Error::Capacity);
        }
        let incarnation = table
            .next_incarnation
            .checked_add(1)
            .ok_or(Error::Control(ControlError::Exhausted))?;
        table.next_incarnation = incarnation;
        let entry = Some(Entry {
            incarnation,
            lifetime,
            #[cfg(feature = "metrics")]
            measurements: crate::TimerMeasurements::default(),
            identity: identity.map(str::to_owned),
            kind,
        });
        if slot == table.entries.len() {
            table.entries.push(entry);
        } else {
            table.entries[slot] = entry;
        }

        Ok(Handle {
            table: Rc::downgrade(&self.0),
            slot,
            incarnation,
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
            let Some(entry) = &mut table.entries[slot] else {
                continue;
            };
            let incarnation = entry.incarnation;
            match &mut entry.kind {
                Kind::Once { state, .. } => {
                    if let Ok(Some(ticket)) = state.due(now) {
                        return Some(Scheduled::Once(Dispatch {
                            table: Rc::downgrade(&self.0),
                            slot,
                            incarnation,
                            ticket,
                        }));
                    }
                }
                Kind::Watchdog { state, .. } => {
                    if let Some(ticket) = state.pending() {
                        return Some(Scheduled::Watchdog(watchdog::Dispatch {
                            table: Rc::downgrade(&self.0),
                            slot,
                            incarnation,
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
            let Some(entry) = entry else {
                continue;
            };
            let at = match &entry.kind {
                Kind::Once { state, .. } => match state.state() {
                    State::Armed(at)
                        if !submitted.clone().any(|item| {
                            matches!(item, Scheduled::Once(dispatch)
                            if dispatch.slot == slot && dispatch.incarnation == entry.incarnation && Weak::ptr_eq(&dispatch.table, &owner)
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
                            if dispatch.slot == slot && dispatch.incarnation == entry.incarnation && dispatch.ticket == *ticket
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
    #[cfg_attr(
        not(target_arch = "wasm32"),
        expect(dead_code, reason = "only the IC transport reports delivery rejection")
    )]
    pub(super) fn rejected(&self, dispatch: &Dispatch) -> Result<bool, Error> {
        if !Weak::ptr_eq(&dispatch.table, &Rc::downgrade(&self.0)) {
            return Err(Error::StaleAuthority);
        }
        let mut table = self.0.borrow_mut();
        let entry = match table.entry(dispatch.slot, dispatch.incarnation) {
            Ok(entry) => entry,
            Err(_) => return Ok(false),
        };
        let (state, _) = entry.kind.once()?;
        if let Some(attempt) = state.accept(dispatch.ticket).map_err(Error::Control)? {
            let abandoned = state.abandon(attempt);
            #[cfg(feature = "metrics")]
            if abandoned {
                entry.measurements.accepted.add(1);
                entry.measurements.abandoned.add(1);
            }

            return Ok(abandoned);
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
            let entry = match table.entry(dispatch.slot, dispatch.incarnation) {
                Ok(entry) => entry,
                Err(_) => return Ok(None),
            };
            let (state, factory) = entry.kind.once()?;
            let attempt = state.accept(dispatch.ticket).map_err(Error::Control)?;
            #[cfg(feature = "metrics")]
            if attempt.is_some() {
                entry.measurements.accepted.add(1);
            }
            attempt.map(|attempt| (attempt, Rc::clone(factory)))
        };
        let Some((attempt, factory)) = selected else {
            return Ok(None);
        };
        let accepted = Accepted {
            table: Rc::downgrade(&self.0),
            slot: dispatch.slot,
            incarnation: dispatch.incarnation,
            attempt,
        };
        let future = (factory.borrow_mut())();

        Ok(Some((accepted, future)))
    }
}

impl Handle {
    #[cfg(feature = "metrics")]
    fn measure(&self, record: impl FnOnce(&mut crate::TimerMeasurements)) -> Result<(), Error> {
        let table = self.table.upgrade().ok_or(Error::StaleAuthority)?;
        record(
            &mut table
                .borrow_mut()
                .entry(self.slot, self.incarnation)?
                .measurements,
        );

        Ok(())
    }

    pub(super) fn remove(&self) -> Result<(), Error> {
        let table = self.table.upgrade().ok_or(Error::StaleAuthority)?;
        let removed = {
            let mut table = table.borrow_mut();
            let entry = table.entry(self.slot, self.incarnation)?;
            match &entry.kind {
                Kind::Once { state, .. } => match state.state() {
                    State::Running { .. } => return Err(Error::Busy),
                    State::RecoveryRequired(_) => {
                        return Err(Error::Control(ControlError::RecoveryRequired));
                    }
                    _ => {}
                },
                Kind::Watchdog { state, .. }
                    if state.outstanding() && state.pending().is_none() =>
                {
                    return Err(Error::Busy);
                }
                Kind::Watchdog { .. } => {}
            }
            table.entries[self.slot].take()
        };
        // Captures may reenter the runtime or bind a replacement in this slot.
        drop(removed);

        Ok(())
    }

    fn prune_stopped(&self) -> Result<(), Error> {
        let table = self.table.upgrade().ok_or(Error::StaleAuthority)?;
        let removed = {
            let mut table = table.borrow_mut();
            let entry = table.entry(self.slot, self.incarnation)?;
            let stopped = match &entry.kind {
                Kind::Once { state, .. } => state.state() == State::Inactive,
                Kind::Watchdog { state, .. } => state.deadline().is_none() && !state.outstanding(),
            };
            if entry.lifetime == crate::Lifetime::RemoveWhenStopped && stopped {
                table.entries[self.slot].take()
            } else {
                None
            }
        };
        drop(removed);

        Ok(())
    }

    fn with_state<T>(&self, f: impl FnOnce(&mut OnceState) -> T) -> Result<T, Error> {
        let table = self.table.upgrade().ok_or(Error::StaleAuthority)?;
        let mut table = table.borrow_mut();
        let entry = table.entry(self.slot, self.incarnation)?;

        Ok(f(entry.kind.once()?.0))
    }

    pub(super) fn set(&self, demand: Demand) -> Result<(), Error> {
        self.with_state(|state| state.set(demand))?
            .map_err(Error::Control)?;

        self.prune_stopped()
    }

    pub(super) fn state(&self) -> Result<State, Error> {
        self.with_state(|state| state.state())
    }

    pub(super) fn recovery(&self) -> Result<Option<RecoveryKey>, Error> {
        Ok(match self.state()? {
            State::RecoveryRequired(recovery) => Some(RecoveryKey {
                table: self.table.clone(),
                slot: self.slot,
                incarnation: self.incarnation,
                recovery,
            }),
            _ => None,
        })
    }

    pub(super) fn recover(&self, key: &RecoveryKey, demand: Demand) -> Result<(), Error> {
        if self.slot != key.slot
            || self.incarnation != key.incarnation
            || !Weak::ptr_eq(&self.table, &key.table)
        {
            return Err(Error::StaleAuthority);
        }
        self.with_state(|state| state.recover(key.recovery, demand))?
            .map_err(Error::Control)?;

        self.prune_stopped()
    }
}

impl Accepted {
    pub(super) fn complete(
        &self,
        next: Demand,
        _observation: crate::Observation,
    ) -> Result<bool, Error> {
        let _owner = self.table.upgrade().ok_or(Error::StaleAuthority)?;
        let handle = Handle {
            table: self.table.clone(),
            slot: self.slot,
            incarnation: self.incarnation,
        };
        let completed = match handle.with_state(|state| state.complete(self.attempt, next)) {
            Err(Error::StaleAuthority) => return Ok(false),
            result => result?,
        };
        if completed {
            #[cfg(feature = "metrics")]
            handle.measure(|measurements| measurements.returned(_observation))?;
            handle.prune_stopped()?;
        }

        Ok(completed)
    }

    /// Requires transport/executor proof of local termination. Dropping the
    /// future or an Accepted value does not automatically authorize recovery.
    pub(super) fn abandon(&self) -> Result<bool, Error> {
        let handle = Handle {
            table: self.table.clone(),
            slot: self.slot,
            incarnation: self.incarnation,
        };
        let abandoned = handle.with_state(|state| state.abandon(self.attempt))?;
        #[cfg(feature = "metrics")]
        if abandoned {
            handle.measure(|measurements| measurements.abandoned.add(1))?;
        }

        Ok(abandoned)
    }
}

impl Dispatch {
    #[cfg_attr(
        not(target_arch = "wasm32"),
        expect(dead_code, reason = "only the IC transport compares submitted flights")
    )]
    pub(super) fn same(&self, other: &Self) -> bool {
        self.slot == other.slot
            && self.incarnation == other.incarnation
            && self.ticket == other.ticket
            && Weak::ptr_eq(&self.table, &other.table)
    }
}

impl Accepted {
    #[cfg_attr(
        not(target_arch = "wasm32"),
        expect(
            dead_code,
            reason = "only IC transport cleanup checks accepted flights"
        )
    )]
    pub(super) fn is_running(&self) -> Result<bool, Error> {
        match (Handle { table: self.table.clone(), slot: self.slot, incarnation: self.incarnation }).with_state(|state|
            matches!(state.state(), State::Running { attempt, .. } if attempt == self.attempt)) {
            Err(Error::StaleAuthority) => Ok(false),
            result => result,
        }
    }
}
