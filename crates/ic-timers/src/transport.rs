//! One global timer and bounded isolated flights for Once and watchdog work.

use crate::{
    api::{Error, OnceDemand, OnceNext, OnceResult, WatchdogDemand, WatchdogResult},
    bindings::{Accepted, Bindings, Handle, RecoveryKey, Scheduled},
    engine::{Demand, State},
    metrics::Event,
};
use ic_platform_runtime::{
    Principal,
    call::{Call, CallFailed, WaitPolicy},
    executor, ic0,
};
use std::{
    cell::{Cell, RefCell},
    future::Future,
    num::NonZeroU64,
    rc::{Rc, Weak},
};

const MAX_FLIGHTS: usize = 8;
const PER_TURN: usize = 4;
const RETRY_NS: u64 = 1_000_000_000;
const METHOD: &str = "__ic_scheduling_once";

struct Flight {
    id: u64,
    dispatch: Scheduled,
    accepted: Option<Accepted>,
}
struct Runtime {
    bindings: Bindings,
    flights: RefCell<Vec<Flight>>,
    sequence: Cell<u64>,
    incarnation: u64,
    faulted: Cell<bool>,
    timer: Cell<u64>,
    retry_at: Cell<u64>,
    #[cfg(feature = "metrics")]
    metrics: Rc<crate::metrics::Recorder>,
}
thread_local! { static RUNTIME: RefCell<Option<Rc<Runtime>>> = const { RefCell::new(None) }; }

/// Retained, non-cloneable control capability. Dropping it does not cancel work.
pub struct Once {
    runtime: Weak<Runtime>,
    handle: Handle,
}
/// Recovery authority for one abandoned attempt of one declaration.
pub struct RecoveryToken(RecoveryKey);
/// Bounded status for this declaration only.
pub enum OnceStatus {
    /// No future eligibility.
    Inactive,
    /// Armed at an absolute Unix nanosecond timestamp.
    Armed(u64),
    /// One local attempt is accepted; owner commands cannot overlap it.
    Running,
    /// Reconcile effects before explicitly recovering this attempt.
    RecoveryRequired(RecoveryToken),
    /// Provider or entry fault prevents further dispatch.
    Faulted,
}

/// Initialize once per live canister runtime. Does not reset existing declarations.
/// Snapshot restoration is unsupported; normal upgrades bind fresh volatile state.
/// The host must admit upgrades only after settling work, or use a separately
/// qualified reconciliation procedure. This function does not preserve accepted
/// attempts, recover uncertainty, or install an upgrade guard.
///
/// # Errors
/// Returns Faulted if this runtime has encountered a terminal provider fault.
pub fn initialize() -> Result<(), Error> {
    RUNTIME.with(|slot| {
        let mut slot = slot.borrow_mut();
        let runtime = slot.get_or_insert_with(|| {
            Rc::new(Runtime {
                bindings: Bindings::new(),
                flights: RefCell::new(Vec::new()),
                sequence: Cell::new(0),
                incarnation: ic0::canister_version(),
                faulted: Cell::new(false),
                timer: Cell::new(0),
                retry_at: Cell::new(0),
                #[cfg(feature = "metrics")]
                metrics: Rc::new(crate::metrics::Recorder::new()),
            })
        });
        runtime.check()
    })
}
fn runtime() -> Result<Rc<Runtime>, Error> {
    RUNTIME
        .with(|slot| slot.borrow().clone())
        .ok_or(Error::Uninitialized)
}
/// Read aggregate counters without changing demand. Available even after a
/// provider fault. Host code owns encoding/export; no exporter is invoked here.
/// # Errors
/// Returns Uninitialized before provider initialization.
#[cfg(feature = "metrics")]
pub fn metrics() -> Result<crate::Metrics, Error> {
    Ok(runtime()?.metrics.snapshot())
}
/// Bind one inactive callback with nonempty owner/group/name components (64 bytes each).
///
/// # Errors
/// Returns initialization, identity, capacity or provider errors without replacing work.
pub fn bind_once<F, Fut>(identity: [&str; 3], mut work: F) -> Result<Once, Error>
where
    F: FnMut() -> Fut + 'static,
    Fut: Future<Output = OnceResult> + 'static,
{
    let runtime = runtime()?;
    runtime.check()?;
    #[cfg(feature = "metrics")]
    let metrics = Rc::clone(&runtime.metrics);
    let handle = runtime.bindings.bind(identity, move || {
        #[cfg(feature = "metrics")]
        metrics.record(Event::Accepted);
        let future = work();
        #[cfg(feature = "metrics")]
        let metrics = Rc::clone(&metrics);
        async move {
            let result = future.await;
            #[cfg(feature = "metrics")]
            metrics.record(Event::Returned(result.observation));
            match result.next {
                OnceNext::Stop => Demand::Inactive,
                OnceNext::At(at) => Demand::At(at),
            }
        }
    })?;
    Ok(Once {
        runtime: Rc::downgrade(&runtime),
        handle,
    })
}
impl Once {
    fn runtime(&self) -> Result<Rc<Runtime>, Error> {
        self.runtime.upgrade().ok_or(Error::StaleAuthority)
    }
    /// Change eligibility; never invokes work inline. A provider failure is terminal.
    /// # Errors
    /// Returns stale authority, recovery-required or provider/counter fault.
    pub fn set(&self, demand: OnceDemand) -> Result<(), Error> {
        let runtime = self.runtime()?;
        runtime.check()?;
        self.handle.set(demand.into())?;
        runtime.arm()
    }
    /// Observe this declaration without affecting eligibility.
    /// # Errors
    /// Returns stale authority if the runtime is gone.
    pub fn status(&self) -> Result<OnceStatus, Error> {
        let runtime = self.runtime()?;
        if runtime.faulted.get() {
            return Ok(OnceStatus::Faulted);
        }
        Ok(match self.handle.state()? {
            State::Inactive => OnceStatus::Inactive,
            State::Armed(at) => OnceStatus::Armed(at),
            State::Running { .. } => OnceStatus::Running,
            State::Faulted => OnceStatus::Faulted,
            State::RecoveryRequired(_) => OnceStatus::RecoveryRequired(RecoveryToken(
                self.handle.recovery()?.ok_or(Error::StaleRecovery)?,
            )),
        })
    }
    /// Clear only the current recovery barrier after owner reconciliation.
    /// # Errors
    /// Returns stale recovery/authority or a terminal provider fault.
    pub fn recover(&self, token: RecoveryToken, demand: OnceDemand) -> Result<(), Error> {
        let runtime = self.runtime()?;
        runtime.check()?;
        let result = self.handle.recover(&token.0, demand.into());
        drop(token);
        result?;
        runtime.arm()
    }
}
impl Runtime {
    fn record(&self, event: Event) {
        #[cfg(feature = "metrics")]
        self.metrics.record(event);
        #[cfg(not(feature = "metrics"))]
        let _ = event;
    }
    fn check(&self) -> Result<(), Error> {
        if self.faulted.get() {
            Err(Error::Faulted)
        } else {
            Ok(())
        }
    }
    fn fail(&self) {
        if !self.faulted.replace(true) {
            self.record(Event::ProviderFault);
        }
        ic0::global_timer_set(0);
        self.timer.set(0);
    }
    fn arm(&self) -> Result<(), Error> {
        self.check()?;
        let (eligible, cadence, full) = {
            let flights = self.flights.borrow();
            let submitted: Vec<_> = flights
                .iter()
                .filter_map(|flight| match &flight.dispatch {
                    Scheduled::Once(dispatch) => Some(dispatch),
                    _ => None,
                })
                .collect();
            let watchdogs: Vec<_> = flights
                .iter()
                .filter_map(|flight| match &flight.dispatch {
                    Scheduled::Watchdog(dispatch) => Some(dispatch),
                    _ => None,
                })
                .collect();
            let (cadence, pending) = self.bindings.watchdog_wake(&watchdogs);
            (
                self.bindings
                    .deadline(&submitted)
                    .into_iter()
                    .chain(pending.then_some(0))
                    .min(),
                cadence,
                flights.len() == MAX_FLIGHTS,
            )
        };
        let deadline = match eligible {
            None => 0,
            Some(at) => {
                let now = ic0::time();
                let at = at.max(self.retry_at.get());
                if at > now {
                    at
                } else {
                    // Only full capacity defers other due work. Submitted tickets
                    // were excluded above and cannot cause repeated wakeups.
                    let delay = if full { RETRY_NS } else { 1 };
                    let Some(next) = now.checked_add(delay) else {
                        self.fail();
                        return Err(Error::Faulted);
                    };
                    next
                }
            }
        };
        // Cadence control must keep advancing even when delivery is backpressured.
        let deadline = match cadence {
            Some(at) => {
                let Some(earliest) = ic0::time().checked_add(1) else {
                    self.fail();
                    return Err(Error::Faulted);
                };
                let at = at.max(earliest);
                if deadline == 0 { at } else { deadline.min(at) }
            }
            None => deadline,
        };
        let previous = ic0::global_timer_set(deadline);
        if previous != self.timer.replace(deadline) {
            self.fail();
            return Err(Error::Faulted);
        }
        Ok(())
    }
}
fn self_id() -> Principal {
    let mut bytes = [0; 29];
    let len = ic0::canister_self_size();
    ic0::canister_self_copy(&mut bytes[..len], 0);
    Principal::from_slice(&bytes[..len])
}

#[expect(
    unsafe_code,
    reason = "this crate owns the canister global timer export"
)]
#[unsafe(export_name = "canister_global_timer")]
extern "C" fn timer() {
    let Ok(runtime) = runtime() else { return };
    runtime.timer.set(0); // The IC clears the fired one-shot before this message.
    if runtime.check().is_err() {
        return;
    }
    runtime.bindings.advance_watchdogs(ic0::time());
    // A cadence wake advances control state but must not bypass delivery backoff.
    if ic0::time() < runtime.retry_at.get() {
        let _ = runtime.arm();
        return;
    }
    executor::in_tracking_executor_context(|| {
        let mut submitted = 0;
        for _ in 0..crate::bindings::CAPACITY {
            if submitted == PER_TURN || runtime.flights.borrow().len() == MAX_FLIGHTS {
                break;
            }
            let Some(dispatch) = runtime.bindings.select(ic0::time(), true) else {
                break;
            };
            if runtime
                .flights
                .borrow()
                .iter()
                .any(|flight| same(&flight.dispatch, &dispatch))
            {
                continue;
            }
            let Some(id) = runtime.sequence.get().checked_add(1) else {
                runtime.fail();
                break;
            };
            runtime.sequence.set(id);
            runtime.flights.borrow_mut().push(Flight {
                id,
                dispatch,
                accepted: None,
            });
            let owner = Rc::clone(&runtime);
            let _task = executor::spawn_protected(async move {
                let mut flight_guard = FlightGuard {
                    runtime: Rc::clone(&owner),
                    finished: false,
                };
                let mut envelope = owner.incarnation.to_le_bytes().to_vec();
                envelope.extend_from_slice(&id.to_le_bytes());
                let response = Call::new(self_id(), METHOD, WaitPolicy::Unbounded)
                    .take_raw_args(envelope)
                    .execute()
                    .await;
                let flight = {
                    let mut flights = owner.flights.borrow_mut();
                    flights
                        .iter()
                        .position(|flight| flight.id == id)
                        .map(|index| flights.remove(index))
                };
                if let Some(flight) = flight {
                    if matches!(
                        response,
                        Err(CallFailed::CallPerformFailed(_)
                            | CallFailed::InsufficientLiquidCycleBalance(_))
                    ) {
                        owner.record(Event::Deferred);
                        match ic0::time().checked_add(RETRY_NS) {
                            Some(at) => owner.retry_at.set(at),
                            None => owner.fail(),
                        }
                    } else if let Scheduled::Watchdog(dispatch) = &flight.dispatch {
                        // Synchronous work has no suspended provider-owned future.
                        // A trap rolls its work message back; either reply or reject
                        // definitively settles this flight, preserving its successor.
                        match dispatch.settle() {
                            Ok(true) if response.is_err() => {
                                owner.record(Event::Accepted);
                                owner.record(Event::Abandoned);
                            }
                            Err(_) => owner.fail(),
                            _ => {}
                        }
                    } else if response.is_err() {
                        if let Some(accepted) = flight.accepted {
                            // Do not infer termination solely from a rejection. A live
                            // Running attempt remains blocked by a terminal provider fault.
                            if accepted.is_running().unwrap_or(true) {
                                owner.fail();
                            }
                        } else if let Scheduled::Once(dispatch) = &flight.dispatch {
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
                flight_guard.finished = true;
                let _ = owner.arm();
            });
            submitted += 1;
        }
    });
    let _ = runtime.arm();
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

struct WorkGuard {
    runtime: Rc<Runtime>,
    accepted: Accepted,
    finished: bool,
}
impl Drop for WorkGuard {
    fn drop(&mut self) {
        if !self.finished {
            if executor::is_recovering_from_trap() {
                match self.accepted.abandon() {
                    Ok(true) => self.runtime.record(Event::Abandoned),
                    Err(_) => self.runtime.fail(),
                    _ => {}
                }
            } else {
                self.runtime.fail(); // Unqualified cancellation does not authorize replay.
            }
        }
    }
}
#[expect(
    unsafe_code,
    reason = "this crate owns its authenticated work endpoint"
)]
#[unsafe(export_name = "canister_update __ic_scheduling_once")]
extern "C" fn dispatch() {
    let mut caller = [0; 29];
    let len = ic0::msg_caller_size();
    if len > caller.len() {
        ic0::trap(b"invalid caller")
    }
    ic0::msg_caller_copy(&mut caller[..len], 0);
    if Principal::from_slice(&caller[..len]) != self_id() {
        ic0::trap(b"self only")
    }
    if ic0::msg_arg_data_size() != 16 {
        ic0::trap(b"invalid dispatch envelope")
    }
    let mut bytes = [0; 16];
    ic0::msg_arg_data_copy(&mut bytes, 0);
    let incarnation = u64::from_le_bytes(bytes[..8].try_into().expect("fixed envelope"));
    let id = u64::from_le_bytes(bytes[8..].try_into().expect("fixed envelope"));
    let Ok(runtime) = runtime() else {
        ic0::trap(b"uninitialized")
    };
    if runtime.check().is_err() {
        ic0::trap(b"provider fault")
    }
    if incarnation != runtime.incarnation {
        ic0::msg_reply();
        return;
    }
    let selected = runtime
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
            executor::in_tracking_executor_context(|| {
                if runtime.bindings.run_watchdog(&dispatch).is_err() {
                    runtime.fail();
                    ic0::trap(b"watchdog acceptance failed");
                }
            });
            let _ = runtime.arm();
            ic0::msg_reply();
            return;
        }
        Scheduled::Once(dispatch) => dispatch,
    };
    executor::in_tracking_executor_context(|| {
        let owner = Rc::clone(&runtime);
        let _task = executor::spawn_protected(async move {
            let result = owner.bindings.start(&selected);
            let (accepted, future) = match result {
                Ok(Some(value)) => value,
                Ok(None) => {
                    ic0::msg_reply();
                    return;
                }
                Err(_) => {
                    owner.fail();
                    ic0::trap(b"acceptance failed")
                }
            };
            if let Some(flight) = owner
                .flights
                .borrow_mut()
                .iter_mut()
                .find(|flight| flight.id == id)
            {
                flight.accepted = Some(accepted.clone());
            }
            let mut guard = WorkGuard {
                runtime: Rc::clone(&owner),
                accepted,
                finished: false,
            };
            let next = future.await;
            if guard.accepted.complete(next).is_err() {
                owner.fail();
            }
            guard.finished = true;
            let _ = owner.arm();
            ic0::msg_reply();
        });
    });
}

fn same(a: &Scheduled, b: &Scheduled) -> bool {
    match (a, b) {
        (Scheduled::Once(a), Scheduled::Once(b)) => a.same(b),
        (Scheduled::Watchdog(a), Scheduled::Watchdog(b)) => a.same(b),
        _ => false,
    }
}

/// Retained synchronous admission callback. Dropping the handle does not cancel it.
pub struct Watchdog {
    runtime: Weak<Runtime>,
    handle: crate::bindings::watchdog::WatchdogHandle,
}
/// Bounded watchdog eligibility and outstanding-ticket observation.
pub enum WatchdogStatus {
    /// No future cadence; an accepted/settling ticket may remain outstanding.
    Inactive {
        /// Whether transport still owns a work ticket.
        outstanding: bool,
    },
    /// Absolute next cadence deadline.
    Armed {
        /// Unix nanosecond deadline.
        at: u64,
        /// Whether transport still owns a work ticket.
        outstanding: bool,
    },
    /// Terminal entry or provider fault.
    Faulted,
}
/// Bind an inactive synchronous watchdog, with cadence in nanoseconds.
/// Consumer-owned asynchronous work must enforce its own physical-request admission.
/// # Errors
/// Returns initialization, identity, capacity or provider errors.
pub fn bind_watchdog<F>(
    identity: [&str; 3],
    cadence: NonZeroU64,
    mut work: F,
) -> Result<Watchdog, Error>
where
    F: FnMut() -> WatchdogResult + 'static,
{
    let runtime = runtime()?;
    runtime.check()?;
    #[cfg(feature = "metrics")]
    let metrics = Rc::clone(&runtime.metrics);
    let handle = runtime.bindings.bind_watchdog(identity, cadence, move || {
        #[cfg(feature = "metrics")]
        metrics.record(Event::Accepted);
        let result = work();
        #[cfg(feature = "metrics")]
        metrics.record(Event::Returned(result.observation));
        result.next.into()
    })?;
    Ok(Watchdog {
        runtime: Rc::downgrade(&runtime),
        handle,
    })
}
impl Watchdog {
    /// Update eligibility without invoking consumer work inline.
    /// # Errors
    /// Returns stale authority or terminal entry/provider fault.
    pub fn set(&self, demand: WatchdogDemand) -> Result<(), Error> {
        let runtime = self.runtime.upgrade().ok_or(Error::StaleAuthority)?;
        runtime.check()?;
        self.handle.set(demand.into(), ic0::time())?;
        runtime.arm()
    }
    /// Observe this declaration without controlling consumer-owned async work.
    /// # Errors
    /// Returns stale authority if the runtime is gone.
    pub fn status(&self) -> Result<WatchdogStatus, Error> {
        let runtime = self.runtime.upgrade().ok_or(Error::StaleAuthority)?;
        let (deadline, outstanding, faulted) = self.handle.status()?;
        Ok(if runtime.faulted.get() || faulted {
            WatchdogStatus::Faulted
        } else if let Some(at) = deadline {
            WatchdogStatus::Armed { at, outstanding }
        } else {
            WatchdogStatus::Inactive { outstanding }
        })
    }
}
