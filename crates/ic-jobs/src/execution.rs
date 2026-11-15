//! Initial skip-late Jobs adapter. One retained Once owns execution and recovery.
use crate::{
    Schedule,
    progress::{Progress, Selection},
};
use ic_platform_runtime::ic0;
use ic_timers::{Once, OnceDemand, OnceNext, OnceResult, OnceStatus};
use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::{Rc, Weak},
};

/// Calendar identity and actual start; combine revision/time with the host's job key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Context {
    /// Schedule revision local to this binding; not durable across reconstruction.
    pub revision: u64,
    /// Intended UTC occurrence, in Unix nanoseconds.
    pub scheduled_at: u64,
    /// Actual callback start, in Unix nanoseconds.
    pub started_at: u64,
}
/// Business outcome. No variant automatically retries an occurrence.
pub enum Outcome {
    /// No business work was needed.
    NoWork,
    /// Business work completed successfully.
    Success,
    /// Expected business failure; future calendar occurrences remain independent.
    ExpectedFailure,
}
/// Admission or control failure.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// Timers refused admission or control.
    Timer(ic_timers::Error),
    /// Recovery belongs to a different job or obsolete occurrence.
    StaleRecovery,
    /// Terminal progress/adapter fault; no new work is eligible.
    Faulted,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
impl From<ic_timers::Error> for Error {
    fn from(value: ic_timers::Error) -> Self {
        Self::Timer(value)
    }
}
/// Exact current job and timer recovery authority. Not serializable.
pub struct Recovery {
    state: Weak<RefCell<State>>,
    selection: Selection,
    timer: ic_timers::RecoveryToken,
}
/// Bounded projection of this job's eligibility and uncertainty.
pub enum Status {
    /// Explicitly paused, with no accepted work.
    Paused,
    /// No future occurrence exists in the supported timestamp domain.
    Idle,
    /// An occurrence is selected for delivery.
    Armed {
        /// Schedule revision.
        revision: u64,
        /// Intended occurrence time.
        scheduled_at: u64,
    },
    /// Consumer work has been accepted. Owner edits apply after settlement.
    Running {
        /// Accepted schedule revision.
        revision: u64,
        /// Accepted occurrence time.
        scheduled_at: u64,
    },
    /// Reconcile business effects before resuming this job.
    RecoveryRequired {
        /// Uncertain schedule revision.
        revision: u64,
        /// Uncertain intended occurrence time.
        scheduled_at: u64,
        /// Exact job and timer reconciliation authority.
        recovery: Recovery,
    },
    /// Terminal job or provider fault.
    Faulted,
}
struct State {
    schedule: Schedule,
    lateness_ns: u64,
    progress: Progress,
    faulted: bool,
}
impl State {
    fn plan(&mut self, now: u64) -> OnceDemand {
        if self.faulted || self.progress.paused() {
            return OnceDemand::Inactive;
        }
        if let Some(selection) = self.progress.selection() {
            return OnceDemand::At(selection.occurrence.scheduled_at);
        }
        let after = now.max(self.progress.through());
        let Some(at) = self.schedule.next_after(after, u64::MAX) else {
            return OnceDemand::Inactive;
        };
        match self.progress.select(at) {
            Ok(_) => OnceDemand::At(at),
            Err(_) => {
                self.faulted = true;
                OnceDemand::Inactive
            }
        }
    }
}
/// Retained job control. Dropping it does not cancel the timer declaration.
/// Host persistence/reconstruction is not yet supported by this preview API.
pub struct Job {
    state: Rc<RefCell<State>>,
    timer: Once,
}
type Work = Pin<Box<dyn Future<Output = OnceResult>>>;

/// Bind a UTC job and select its first future occurrence.
///
/// Delivery more than `lateness_ns` after the intended time skips consumer work.
/// After completion or a skip, Saffron selects strictly after current time and
/// settled progress. This initial policy has no catch-up, coalescing or retry.
/// Calendar search costs and persistence remain release qualification gates.
/// Timers must already be initialized. Each job consumes one timer declaration.
/// # Errors
/// Returns timer admission/control errors or terminal progress failure.
pub fn bind<F, Fut>(
    identity: [&str; 3],
    schedule: Schedule,
    lateness_ns: u64,
    mut work: F,
) -> Result<Job, Error>
where
    F: FnMut(Context) -> Fut + 'static,
    Fut: Future<Output = Outcome> + 'static,
{
    let state = Rc::new(RefCell::new(State {
        schedule,
        lateness_ns,
        progress: Progress::new(ic0::time()),
        faulted: false,
    }));
    let captured = Rc::clone(&state);
    let timer = ic_timers::bind_once(identity, move || -> Work {
        let now = ic0::time();
        let selected = {
            let mut state = captured.borrow_mut();
            if state.faulted {
                return Box::pin(async { stopped() });
            }
            let Some(selection) = state.progress.selection() else {
                state.faulted = true;
                return Box::pin(async { stopped() });
            };
            if state.progress.accept(selection, now).is_err() {
                state.faulted = true;
                return Box::pin(async { stopped() });
            }
            let late = now - selection.occurrence.scheduled_at > state.lateness_ns;
            (selection, late)
        };
        let (selection, late) = selected;
        let context = Context {
            revision: selection.occurrence.revision,
            scheduled_at: selection.occurrence.scheduled_at,
            started_at: now,
        };
        // Accept before invoking the factory, but never hold job state across it.
        let future = (!late).then(|| work(context));
        let state = Rc::clone(&captured);
        Box::pin(async move {
            let outcome = match future {
                Some(future) => future.await,
                None => Outcome::NoWork,
            };
            let mut state = state.borrow_mut();
            if state.progress.complete(selection).is_err() {
                state.faulted = true;
            }
            let next = match state.plan(ic0::time()) {
                OnceDemand::Inactive => OnceNext::Stop,
                OnceDemand::At(at) => OnceNext::At(at),
            };
            OnceResult {
                observation: match outcome {
                    Outcome::NoWork => ic_timers::Observation::NoWork,
                    Outcome::Success => ic_timers::Observation::Success,
                    Outcome::ExpectedFailure => ic_timers::Observation::ExpectedFailure,
                },
                next,
            }
        })
    })?;
    let job = Job { state, timer };
    let demand = job.state.borrow_mut().plan(ic0::time());
    job.timer.set(demand)?;
    Ok(job)
}
fn stopped() -> OnceResult {
    OnceResult {
        observation: ic_timers::Observation::NoWork,
        next: OnceNext::Stop,
    }
}
impl Job {
    /// Observe status and associate timer recovery with the selected occurrence.
    /// # Errors
    /// Returns a timer status error or terminal mapping fault.
    pub fn status(&self) -> Result<Status, Error> {
        let timer = self.timer.status()?;
        let mut state = self.state.borrow_mut();
        if state.faulted || matches!(timer, OnceStatus::Faulted) {
            return Ok(Status::Faulted);
        }
        if matches!(timer, OnceStatus::Inactive) {
            return Ok(if state.progress.paused() {
                Status::Paused
            } else {
                Status::Idle
            });
        }
        let Some(selection) = state.progress.selection() else {
            state.faulted = true;
            return Err(Error::Faulted);
        };
        let occurrence = selection.occurrence;
        Ok(match timer {
            OnceStatus::RecoveryRequired(token) => {
                state
                    .progress
                    .block(selection)
                    .map_err(|_| Error::Faulted)?;
                Status::RecoveryRequired {
                    revision: occurrence.revision,
                    scheduled_at: occurrence.scheduled_at,
                    recovery: Recovery {
                        state: Rc::downgrade(&self.state),
                        selection,
                        timer: token,
                    },
                }
            }
            OnceStatus::Armed(_) => Status::Armed {
                revision: occurrence.revision,
                scheduled_at: occurrence.scheduled_at,
            },
            OnceStatus::Running => Status::Running {
                revision: occurrence.revision,
                scheduled_at: occurrence.scheduled_at,
            },
            _ => return Err(Error::Faulted),
        })
    }
    fn control(
        &self,
        change: impl FnOnce(&mut State, u64) -> Result<(), crate::progress::Error>,
    ) -> Result<(), Error> {
        let status = self.status()?;
        if matches!(status, Status::Faulted) {
            return Err(Error::Faulted);
        }
        let running = matches!(
            status,
            Status::Running { .. } | Status::RecoveryRequired { .. }
        );
        let demand = {
            let mut state = self.state.borrow_mut();
            if change(&mut state, ic0::time()).is_err() {
                state.faulted = true;
                return Err(Error::Faulted);
            }
            if running {
                None
            } else {
                Some(state.plan(ic0::time()))
            }
        };
        if let Some(demand) = demand {
            self.timer.set(demand)?;
        }
        Ok(())
    }
    /// Pause future work; accepted work and uncertainty remain owned until settlement.
    /// # Errors
    /// Returns timer or terminal progress errors.
    pub fn pause(&self) -> Result<(), Error> {
        self.control(|state, _| state.progress.pause())
    }
    /// Resume future calendar selection without bypassing unresolved work.
    /// # Errors
    /// Returns timer or terminal progress errors.
    pub fn resume(&self) -> Result<(), Error> {
        self.control(|state, _| state.progress.resume())
    }
    /// Replace the schedule for future occurrences; preserve accepted identity/pause.
    /// # Errors
    /// Returns timer or terminal revision/control errors.
    pub fn edit(&self, schedule: Schedule) -> Result<(), Error> {
        self.control(move |state, now| {
            state.progress.edit(now)?;
            state.schedule = schedule;
            Ok(())
        })
    }
    /// After effect reconciliation, retire the uncertain occurrence and remain paused.
    /// Resume explicitly to select a new future occurrence; no replay is performed.
    /// # Errors
    /// Rejects wrong-job/obsolete recovery or timer/control faults.
    pub fn recover(&self, recovery: Recovery) -> Result<(), Error> {
        if !Weak::ptr_eq(&recovery.state, &Rc::downgrade(&self.state)) {
            return Err(Error::StaleRecovery);
        }
        {
            let mut state = self.state.borrow_mut();
            if state.faulted {
                return Err(Error::Faulted);
            }
            state
                .progress
                .reconcile(recovery.selection)
                .map_err(|_| Error::StaleRecovery)?;
            state.progress.pause().map_err(|_| Error::Faulted)?;
        }
        if let Err(error) = self.timer.recover(recovery.timer, OnceDemand::Inactive) {
            self.state.borrow_mut().faulted = true;
            return Err(error.into());
        }
        Ok(())
    }
}
