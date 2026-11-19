//! One global timer and bounded isolated flights for Once and watchdog work.

use crate::{
    Error,
    bindings::{Accepted, Scheduled},
    metrics::Event,
    provider::{Runtime, runtime},
};
use ic_platform_runtime::{
    call::{Call, Outcome, SubmitError, Wait},
    executor::{self, TaskHandle, TaskState},
    raw as ic0, system,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

// timer
// Processes due declarations in the IC system task, with up to four submissions per turn and eight outstanding flights.
// limits : system task, no request payload or reply
#[expect(
    unsafe_code,
    reason = "this crate owns the canister global timer export"
)]
#[unsafe(export_name = "canister_global_timer")]
extern "C" fn timer() {
    let Ok(runtime) = runtime() else { return };
    runtime.transport().timer.set(0);
    if runtime.check().is_err() {
        return;
    }
    runtime.bindings.advance_watchdogs(system::time());
    if system::time() < runtime.transport().retry_at.get() {
        let _ = runtime.arm();

        return;
    }
    let mut submitted = 0;
    for _ in 0..crate::bindings::CAPACITY {
        if submitted == PER_TURN || runtime.transport().flights.borrow().len() == MAX_FLIGHTS {
            break;
        }
        let Some(dispatch) = runtime.bindings.select(system::time()) else {
            break;
        };
        if runtime
            .transport()
            .flights
            .borrow()
            .iter()
            .any(|flight| same(&flight.dispatch, &dispatch))
        {
            continue;
        }
        let Some(id) = runtime.transport().sequence.get().checked_add(1) else {
            runtime.fail();
            break;
        };
        runtime.transport().sequence.set(id);
        runtime.transport().flights.borrow_mut().push(Flight {
            id,
            dispatch,
            accepted: None,
            work: None,
        });
        let owner = Rc::clone(&runtime);
        let task = executor::run_global_timer(async move {
            let mut guard = FlightGuard {
                runtime: Rc::clone(&owner),
                finished: false,
            };
            let envelope = [
                owner.transport().incarnation.to_le_bytes(),
                id.to_le_bytes(),
            ];
            let response = Call::new(system::canister_self(), METHOD, Wait::Unbounded)
                .args(envelope.as_flattened())
                .execute()
                .await;
            let flight = {
                let mut flights = owner.transport().flights.borrow_mut();
                flights
                    .iter()
                    .position(|flight| flight.id == id)
                    .map(|index| flights.remove(index))
            };
            if let Some(flight) = flight {
                match response {
                    Err(SubmitError::Refused(_) | SubmitError::InsufficientBalance { .. }) => {
                        owner.record(Event::Deferred);
                        match system::time().checked_add(RETRY_NS) {
                            Some(at) => owner.transport().retry_at.set(at),
                            None => owner.fail(),
                        }
                    }
                    Err(_) => owner.fail(),
                    Ok(completion) => {
                        let rejected = !matches!(completion.outcome(), Outcome::Reply(_));
                        if let Scheduled::Watchdog(dispatch) = &flight.dispatch {
                            // Work is synchronous; a rejected work message leaves the
                            // prearmed successor committed and definitively settles custody.
                            match dispatch.settle(rejected) {
                                Ok(true) if rejected => {
                                    owner.record(Event::Accepted);
                                    owner.record(Event::Abandoned);
                                }
                                Err(_) => owner.fail(),
                                _ => {}
                            }
                        } else if rejected {
                            if let Some(accepted) = flight.accepted {
                                // Only committed successful destruction proves termination.
                                // A rejected response or Running task does not authorize replay.
                                match flight.work.as_ref().map(TaskHandle::state) {
                                    Some(TaskState::TerminatedByCleanup) => {
                                        match accepted.abandon() {
                                            Ok(true) => owner.record(Event::Abandoned),
                                            Err(_) => owner.fail(),
                                            _ => {}
                                        }
                                    }
                                    Some(TaskState::Completed)
                                        if !accepted.is_running().unwrap_or(true) => {}
                                    _ => owner.fail(),
                                }
                            } else if let Scheduled::Once(dispatch) = &flight.dispatch {
                                // An initial trap rolls back the task handle and acceptance;
                                // the earlier committed dispatch still owns the uncertain work.
                                match owner.bindings.rejected(dispatch) {
                                    Ok(true) => {
                                        owner.record(Event::Accepted);
                                        owner.record(Event::Abandoned);
                                    }
                                    Err(_) => owner.fail(),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            guard.finished = true;
            let _ = owner.arm();
        });
        if task.is_err() {
            runtime.fail();
            break;
        }
        submitted += 1;
        // Root entry polls immediately. A synchronous submission refusal can
        // establish backoff before this control turn returns; honor it now.
        if runtime.check().is_err() || system::time() < runtime.transport().retry_at.get() {
            break;
        }
    }
    let _ = runtime.arm();
}

// dispatch
// Accepts isolated timer work from this canister and replies after settlement.
// limits : 16-byte flight identity request, empty reply
#[expect(
    unsafe_code,
    reason = "this crate owns its authenticated work endpoint"
)]
#[unsafe(export_name = "canister_update __ic_scheduling_once")]
extern "C" fn dispatch() {
    if system::caller() != system::canister_self() {
        ic0::trap(b"self only");
    }
    if ic0::msg_arg_data_size() != 16 {
        ic0::trap(b"invalid dispatch envelope");
    }
    let mut bytes = [0; 16];
    ic0::msg_arg_data_copy(&mut bytes, 0);
    let incarnation = u64::from_le_bytes(bytes[..8].try_into().expect("fixed envelope"));
    let id = u64::from_le_bytes(bytes[8..].try_into().expect("fixed envelope"));
    let Ok(runtime) = runtime() else {
        ic0::trap(b"uninitialized")
    };
    if runtime.check().is_err() {
        ic0::trap(b"provider fault");
    }
    if incarnation != runtime.transport().incarnation {
        ic0::msg_reply();

        return;
    }
    let selected = runtime
        .transport()
        .flights
        .borrow()
        .iter()
        .find(|flight| flight.id == id && flight.accepted.is_none())
        .map(|flight| flight.dispatch.clone());
    let Some(selected) = selected else {
        ic0::msg_reply();

        return;
    };
    let selected = match selected {
        Scheduled::Watchdog(dispatch) => {
            if runtime.bindings.run_watchdog(&dispatch).is_err() {
                runtime.fail();
                ic0::trap(b"watchdog acceptance failed");
            }
            let _ = runtime.arm();
            ic0::msg_reply();

            return;
        }
        Scheduled::Once(dispatch) => dispatch,
    };
    let owner = Rc::clone(&runtime);
    let task = executor::run_update(async move {
        let (accepted, future) = match owner.bindings.start(&selected) {
            Ok(Some(value)) => value,
            Ok(None) => {
                system::reply(&[]).expect("work reply authority");

                return;
            }
            Err(_) => {
                owner.fail();
                ic0::trap(b"acceptance failed")
            }
        };
        if let Some(flight) = owner
            .transport()
            .flights
            .borrow_mut()
            .iter_mut()
            .find(|flight| flight.id == id)
        {
            flight.accepted = Some(accepted.clone());
        }
        let (next, observation) = future.await;
        if accepted.complete(next, observation).is_err() {
            owner.fail();
        }
        let _ = owner.arm();
        system::reply(&[]).expect("work reply authority");
    });
    let task = match task {
        Ok(task) => task,
        Err(_) => {
            runtime.fail();
            ic0::trap(b"work task admission failed")
        }
    };

    if let Some(flight) = runtime
        .transport()
        .flights
        .borrow_mut()
        .iter_mut()
        .find(|flight| flight.id == id)
    {
        flight.work = Some(task);
    }
}

const MAX_FLIGHTS: usize = 8;
const PER_TURN: usize = 4;
const RETRY_NS: u64 = 1_000_000_000;
const METHOD: &str = "__ic_scheduling_once";

struct Flight {
    id: u64,
    dispatch: Scheduled,
    accepted: Option<Accepted>,
    work: Option<TaskHandle>,
}
pub(super) struct Transport {
    flights: RefCell<Vec<Flight>>,
    sequence: Cell<u64>,
    incarnation: u64,
    timer: Cell<u64>,
    retry_at: Cell<u64>,
}

impl Transport {
    pub(super) fn new() -> Self {
        Self {
            flights: RefCell::new(Vec::new()),
            sequence: Cell::new(0),
            incarnation: system::canister_version(),
            timer: Cell::new(0),
            retry_at: Cell::new(0),
        }
    }

    pub(super) fn arm(&self, runtime: &Runtime) -> Result<(), Error> {
        runtime.check()?;
        let (eligible, cadence, full) = {
            let flights = self.flights.borrow();
            let (eligible, cadence) = runtime
                .bindings
                .wake(&flights.iter().map(|flight| &flight.dispatch));
            (eligible, cadence, flights.len() == MAX_FLIGHTS)
        };
        let deadline = match eligible {
            None => 0,
            Some(at) => {
                let now = system::time();
                let at = at.max(self.retry_at.get());
                if at > now {
                    at
                } else {
                    // Only full capacity defers other due work. Submitted tickets
                    // were excluded above and cannot cause repeated wakeups.
                    let delay = if full { RETRY_NS } else { 1 };
                    let Some(next) = now.checked_add(delay) else {
                        runtime.fail();

                        return Err(Error::Faulted);
                    };
                    next
                }
            }
        };
        // Cadence control must keep advancing even when delivery is backpressured.
        let deadline = match cadence {
            Some(at) => {
                let Some(earliest) = system::time().checked_add(1) else {
                    runtime.fail();

                    return Err(Error::Faulted);
                };
                let at = at.max(earliest);
                if deadline == 0 { at } else { deadline.min(at) }
            }
            None => deadline,
        };
        let previous = system::global_timer_set(deadline);
        if previous != self.timer.replace(deadline) {
            runtime.fail();

            return Err(Error::Faulted);
        }

        Ok(())
    }
}

impl Runtime {
    fn record(&self, event: Event) {
        #[cfg(feature = "metrics")]
        self.metrics.record(event);
        #[cfg(not(feature = "metrics"))]
        let _ = event;
    }

    fn fail(&self) {
        if !self.faulted.replace(true) {
            self.record(Event::ProviderFault);
        }
        system::global_timer_set(0);
        self.transport().timer.set(0);
    }
}

struct FlightGuard {
    runtime: Rc<Runtime>,
    finished: bool,
}
impl Drop for FlightGuard {
    fn drop(&mut self) {
        if !self.finished {
            self.runtime.fail();
        }
    }
}

fn same(a: &Scheduled, b: &Scheduled) -> bool {
    match (a, b) {
        (Scheduled::Once(a), Scheduled::Once(b)) => a.same(b),
        (Scheduled::Watchdog(a), Scheduled::Watchdog(b)) => a.same(b),
        _ => false,
    }
}
