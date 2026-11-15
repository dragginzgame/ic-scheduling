//! Calendar Jobs adapter with explicit missed-run policy. One retained Once owns execution and recovery.
use crate::{
    Policy, Schedule,
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
    /// Schedule revision, preserved when the host restores a matching checkpoint.
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
    /// Accepted or uncertain work must be settled before checkpointing.
    Unsettled,
    /// Selected checkpoint occurrence does not match the supplied schedule.
    InvalidCheckpoint,
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
mod state;
use state::State;
/// Retained job control. Dropping it does not cancel the timer declaration.
/// Hosts can persist settled/queued progress using [`Job::checkpoint`] and [`restore`].
pub struct Job {
    state: Rc<RefCell<State>>,
    timer: Once,
}
type Work = Pin<Box<dyn Future<Output = OnceResult>>>;

/// Bind a UTC job and select its first future occurrence.
///
/// SkipLate applies a delivery tolerance and then selects future work. CatchUp
/// freezes an age/count-bounded oldest-first batch when work becomes due; pause
/// and reconstruction retain its remaining occurrences. Batch completion skips
/// excess and newly arrived occurrences before selecting future work.
/// Neither policy automatically retries an uncertain occurrence.
/// Full calendar search budgets and unresolved-work persistence remain release gates.
/// Timers must already be initialized. Each job consumes one timer declaration.
/// # Errors
/// Returns timer admission/control errors or terminal progress failure.
pub fn bind<F, Fut>(
    identity: [&str; 3],
    schedule: Schedule,
    policy: Policy,
    work: F,
) -> Result<Job, Error>
where
    F: FnMut(Context) -> Fut + 'static,
    Fut: Future<Output = Outcome> + 'static,
{
    bind_progress(
        identity,
        schedule,
        policy,
        Progress::new(ic0::time()),
        None,
        work,
    )
}

/// Rebind fresh callbacks from host-owned progress in a fresh runtime.
/// The host must supply the matching stable job identity, schedule and execution
/// policy. This is not a heap-snapshot or unresolved-work restoration procedure.
/// # Errors
/// Rejects selected/batch occurrences inconsistent with the schedule or policy before admission;
/// otherwise returns timer/control errors.
pub fn restore<F, Fut>(
    identity: [&str; 3],
    schedule: Schedule,
    policy: Policy,
    checkpoint: crate::Checkpoint,
    work: F,
) -> Result<Job, Error>
where
    F: FnMut(Context) -> Fut + 'static,
    Fut: Future<Output = Outcome> + 'static,
{
    if !checkpoint.matches_definition(&schedule, policy) {
        return Err(Error::InvalidCheckpoint);
    }
    let progress = Progress::restore(&checkpoint);
    let batch = checkpoint.catch_up;
    bind_progress(identity, schedule, policy, progress, batch, work)
}

fn bind_progress<F, Fut>(
    identity: [&str; 3],
    schedule: Schedule,
    policy: Policy,
    progress: Progress,
    batch: Option<crate::CatchUpProgress>,
    mut work: F,
) -> Result<Job, Error>
where
    F: FnMut(Context) -> Fut + 'static,
    Fut: Future<Output = Outcome> + 'static,
{
    let state = Rc::new(RefCell::new(State {
        schedule,
        policy,
        progress,
        batch,
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
            let deliver = match state.prepare(now) {
                Ok(deliver) => deliver,
                Err(_) => {
                    state.faulted = true;
                    return Box::pin(async { stopped() });
                }
            };
            if state.progress.accept(selection, now).is_err() {
                state.faulted = true;
                return Box::pin(async { stopped() });
            }
            (selection, !deliver)
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
            if state.progress.complete(selection).is_err()
                || state.finish_batch(selection, ic0::time()).is_err()
            {
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
    /// Export reconstructable progress without persisting timer capabilities.
    /// Call synchronously from the host upgrade-admission boundary and store it
    /// atomically with the matching job definition. Running/uncertain work refuses.
    /// # Errors
    /// Returns Unsettled for accepted/recovery work, or timer/terminal errors.
    pub fn checkpoint(&self) -> Result<crate::Checkpoint, Error> {
        match self.status()? {
            Status::Running { .. } | Status::RecoveryRequired { .. } => {
                return Err(Error::Unsettled);
            }
            Status::Faulted => return Err(Error::Faulted),
            _ => {}
        }
        let state = self.state.borrow();
        let mut checkpoint = state.progress.checkpoint().map_err(|_| Error::Faulted)?;
        checkpoint.catch_up = state.batch.clone();
        Ok(checkpoint)
    }

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
    /// Resume according to policy without bypassing unresolved work. A paused
    /// catch-up batch retains its original selection and remaining count.
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
            state.batch = None;
            Ok(())
        })
    }
    /// After effect reconciliation, retire the uncertain occurrence, discard any
    /// remaining catch-up batch and skip through current time; remain paused.
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
            state.batch = None;
            state
                .progress
                .skip_through(ic0::time())
                .map_err(|_| Error::Faulted)?;
            state.progress.pause().map_err(|_| Error::Faulted)?;
        }
        if let Err(error) = self.timer.recover(recovery.timer, OnceDemand::Inactive) {
            self.state.borrow_mut().faulted = true;
            return Err(error.into());
        }
        Ok(())
    }
}
