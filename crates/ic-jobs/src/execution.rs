//! Calendar Jobs adapter with explicit missed-run policy. One retained Once owns execution and recovery.
use crate::{
    Outcome, Policy, RecoveryDisposition, Schedule,
    state::{Selection, State},
};
use ic_platform_runtime::system;
use ic_timers::{Once, OnceDemand, OnceNext, OnceResult, OnceStatus};
use std::{
    cell::RefCell,
    future::Future,
    rc::{Rc, Weak},
};

/// Calendar identity and actual start; combine revision/time with the host's job key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Context {
    /// Accepted attempt number for this occurrence, starting at one. The host's
    /// job key, revision and scheduled time remain the same logical effect key.
    pub attempt: u32,
    /// Schedule revision, preserved when the host restores a matching checkpoint.
    pub revision: u64,
    /// Intended UTC occurrence, in Unix nanoseconds.
    pub scheduled_at: u64,
    /// Actual callback start, in Unix nanoseconds.
    pub started_at: u64,
}
/// Admission or control failure.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// This job was terminally cancelled; it cannot be resumed or edited.
    Cancelled,
    /// Timers refused admission or control.
    Timer(ic_timers::Error),
    /// Recovery belongs to a different job or obsolete occurrence.
    StaleRecovery,
    /// Recovery cannot retry a replaced definition, exhausted budget or
    /// overflowing deadline. The uncertain attempt remains blocked.
    RetryUnavailable,
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
    timer: Option<ic_timers::RecoveryToken>,
}
/// Bounded projection of this job's eligibility and uncertainty.
pub enum Status {
    /// Terminally cancelled before acceptance; its retained timer is inactive.
    Cancelled,
    /// Explicitly paused, with no accepted work.
    Paused,
    /// No future occurrence exists in the supported timestamp domain.
    Idle,
    /// An occurrence is selected for delivery.
    Armed {
        /// Delivery eligibility, which may be a later retry deadline.
        ready_at: u64,
        /// Attempt that will be accepted next for this occurrence.
        attempt: u32,
        /// Schedule revision.
        revision: u64,
        /// Intended occurrence time.
        scheduled_at: u64,
    },
    /// Consumer work has been accepted. Owner edits apply after settlement.
    Running {
        /// Current accepted attempt number.
        attempt: u32,
        /// Accepted schedule revision.
        revision: u64,
        /// Accepted occurrence time.
        scheduled_at: u64,
    },
    /// Reconcile business effects before resuming this job.
    RecoveryRequired {
        /// Uncertain attempt number; time does not permit another attempt.
        attempt: u32,
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
/// Bind a job. Cron starts after now; explicit-deadline schedules select their first
/// deadline, including zero or overdue deadlines, subject to the chosen policy.
///
/// SkipLate applies a delivery tolerance and then selects future work. CatchUp
/// freezes an age/count-bounded oldest-first batch when work becomes due; pause
/// and reconstruction retain its remaining occurrences. Batch completion skips
/// excess and newly arrived occurrences before selecting future work.
/// RetryPolicy::NONE disables retries. Only Outcome::RetryableFailure permits
/// bounded retries with the same effect identity; lateness is not reapplied.
/// No policy automatically retries an uncertain occurrence.
/// After-completion schedules have only one pending occurrence: CatchUp applies
/// its age limit without building a batch. Every normal settlement, including
/// skipping late work, schedules from settlement time; overflow leaves the job idle.
/// Hosts own storage, fresh-runtime reconstruction and business reconciliation.
/// Timers must already be initialized. Each job consumes one timer declaration.
/// # Errors
/// Returns timer admission/control errors or terminal progress failure.
pub fn bind<F, Fut>(
    identity: [&str; 3],
    schedule: Schedule,
    policy: Policy,
    retry_policy: crate::RetryPolicy,
    work: F,
) -> Result<Job, Error>
where
    F: FnMut(Context) -> Fut + 'static,
    Fut: Future<Output = Outcome> + 'static,
{
    let first = schedule.first_at();
    let checkpoint =
        crate::Checkpoint::new(7, 1, None, first, crate::ControlState::Active, None, None)
            .map_err(|_| Error::Faulted)?;

    bind_checkpoint(
        identity,
        schedule,
        policy,
        retry_policy,
        checkpoint,
        first.is_none(),
        work,
    )
}

/// Rebind fresh callbacks from host-owned progress in a fresh runtime.
/// The host must supply the matching stable job identity, schedule and execution
/// and retry policies. Use restore_unresolved for unresolved records; neither API
/// supports heap snapshots or duplicating live execution.
/// # Errors
/// Rejects selected/batch occurrences inconsistent with the schedule or policy before admission;
/// otherwise returns timer/control errors.
pub fn restore<F, Fut>(
    identity: [&str; 3],
    schedule: Schedule,
    policy: Policy,
    retry_policy: crate::RetryPolicy,
    checkpoint: crate::Checkpoint,
    work: F,
) -> Result<Job, Error>
where
    F: FnMut(Context) -> Fut + 'static,
    Fut: Future<Output = Outcome> + 'static,
{
    if checkpoint.unresolved().is_some()
        || !checkpoint.matches_definition(&schedule, policy, retry_policy)
    {
        return Err(Error::InvalidCheckpoint);
    }

    bind_checkpoint(
        identity,
        schedule,
        policy,
        retry_policy,
        checkpoint,
        false,
        work,
    )
}

/// Reconstruct an unresolved job in a fresh runtime with no surviving old callbacks.
/// Both Running and Uncertain records become RecoveryRequired with fresh local
/// recovery authority. Time, resume and schedule edits cannot unblock them.
/// Hosts must atomically associate the record with its stable key and definition;
/// reconstruction does not prove that remote business effects have terminated.
/// # Errors
/// Rejects invalid definition/budget associations before timer admission, or returns
/// provider admission errors. This is not a heap-snapshot or live-cloning API.
pub fn restore_unresolved<F, Fut>(
    identity: [&str; 3],
    schedule: Schedule,
    policy: Policy,
    retry_policy: crate::RetryPolicy,
    record: crate::Checkpoint,
    work: F,
) -> Result<Job, Error>
where
    F: FnMut(Context) -> Fut + 'static,
    Fut: Future<Output = Outcome> + 'static,
{
    if record.unresolved().is_none() || !record.matches_definition(&schedule, policy, retry_policy)
    {
        return Err(Error::InvalidCheckpoint);
    }

    bind_checkpoint(
        identity,
        schedule,
        policy,
        retry_policy,
        record,
        false,
        work,
    )
}

fn bind_checkpoint<F, Fut>(
    identity: [&str; 3],
    schedule: Schedule,
    policy: Policy,
    retry_policy: crate::RetryPolicy,
    checkpoint: crate::Checkpoint,
    anchor_cron: bool,
    mut work: F,
) -> Result<Job, Error>
where
    F: FnMut(Context) -> Fut + 'static,
    Fut: Future<Output = Outcome> + 'static,
{
    let state = Rc::new(RefCell::new(State::from_checkpoint(
        schedule,
        policy,
        retry_policy,
        checkpoint,
    )));
    let captured = Rc::clone(&state);
    let timer = ic_timers::bind_once(identity, ic_timers::Lifetime::Retained, move || {
        let now = system::time();
        let selected = {
            let mut state = captured.borrow_mut();
            match state.accept_dispatch(now) {
                Ok(selected) => Some(selected),
                Err(_) => {
                    state.faulted = true;
                    None
                }
            }
        };
        // Accept before invoking the factory, but never hold job state across it.
        let future = selected.and_then(|(selection, deliver, attempt)| {
            deliver.then(|| {
                #[cfg(feature = "metrics")]
                crate::metrics::record(crate::metrics::Event::Started(attempt));

                work(Context {
                    attempt,
                    revision: selection.occurrence.revision,
                    scheduled_at: selection.occurrence.scheduled_at,
                    started_at: now,
                })
            })
        });
        let state = Rc::clone(&captured);
        async move {
            let Some((selection, _, attempt)) = selected else {
                return stopped();
            };

            #[cfg(feature = "metrics")]
            let delivered = future.is_some();
            let outcome = match future {
                Some(future) => future.await,
                None => Outcome::NoWork,
            };
            #[cfg(feature = "metrics")]
            crate::metrics::record(if delivered {
                crate::metrics::Event::Returned(outcome)
            } else {
                crate::metrics::Event::SkippedLate
            });

            let mut state = state.borrow_mut();
            let finished_at = system::time();
            if state
                .finish(
                    selection,
                    finished_at,
                    matches!(outcome, Outcome::RetryableFailure),
                )
                .is_err()
            {
                state.faulted = true;
            }
            state.last_outcome = crate::Attempt::new(
                selection.occurrence.revision,
                selection.occurrence.scheduled_at,
                attempt,
            )
            .and_then(|attempt| {
                crate::RecordedOutcome::new(
                    attempt,
                    crate::RecordedResult::Returned(outcome),
                    finished_at,
                )
            })
            .ok();
            if state.last_outcome.is_none() {
                state.faulted = true;
            }
            let next = match state.plan(system::time()) {
                None => OnceNext::Stop,
                Some(at) => OnceNext::At(at),
            };
            OnceResult {
                observation: match outcome {
                    Outcome::NoWork => ic_timers::Observation::NoWork,
                    Outcome::Success => ic_timers::Observation::Success,
                    Outcome::ExpectedFailure | Outcome::RetryableFailure => {
                        ic_timers::Observation::ExpectedFailure
                    }
                },
                next,
            }
        }
    })?;
    let job = Job { state, timer };
    // Timer admission rejects callers without an installed runtime before this IC clock read.
    let now = system::time();
    let demand = {
        let mut state = job.state.borrow_mut();
        if anchor_cron {
            state.skip_through(now).map_err(|_| Error::Faulted)?;
        }

        state.plan(now)
    };
    job.timer
        .set(demand.map_or(OnceDemand::Inactive, OnceDemand::At))?;

    #[cfg(feature = "metrics")]
    crate::metrics::ensure_storage();

    Ok(job)
}

fn stopped() -> OnceResult {
    OnceResult {
        observation: ic_timers::Observation::NoWork,
        next: OnceNext::Stop,
    }
}

///
/// Job
/// Adapts job progress to a retained Once timer. Dropping the handle does not
/// cancel the timer declaration. Hosts can persist settled or queued progress
/// using [`Job::checkpoint`] and [`restore`].
///

pub struct Job {
    state: Rc<RefCell<State>>,
    timer: Once,
}

impl Job {
    /// Export unresolved work for host-owned reconstruction in a fresh runtime.
    /// This does not stop live work. Capture at the upgrade-admission boundary and
    /// store atomically with the matching definition; discard stale earlier exports.
    /// The exported future progress is paused. Ordinary checkpoint export still
    /// refuses unresolved work, so hosts must explicitly choose this lifecycle policy.
    /// # Errors
    /// Returns Unsettled unless work is Running/RecoveryRequired; returns provider
    /// or terminal errors on invalid runtime state.
    pub fn unresolved_checkpoint(&self) -> Result<crate::Checkpoint, Error> {
        let (revision, scheduled_at, attempt, phase) = match self.status()? {
            Status::Running {
                revision,
                scheduled_at,
                attempt,
            } => (
                revision,
                scheduled_at,
                attempt,
                crate::UnresolvedPhase::Running,
            ),
            Status::RecoveryRequired {
                revision,
                scheduled_at,
                attempt,
                ..
            } => (
                revision,
                scheduled_at,
                attempt,
                crate::UnresolvedPhase::Uncertain,
            ),
            Status::Faulted => return Err(Error::Faulted),
            _ => return Err(Error::Unsettled),
        };
        let state = self.state.borrow();
        let progress = state.unresolved_progress().map_err(|_| Error::Faulted)?;
        let attempt =
            crate::Attempt::new(revision, scheduled_at, attempt).map_err(|_| Error::Faulted)?;

        progress
            .with_unresolved(attempt, phase)
            .map_err(|_| Error::Faulted)
    }

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

        self.state.borrow().checkpoint().map_err(|_| Error::Faulted)
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
        if state.restored() {
            if !matches!(timer, OnceStatus::Inactive) {
                state.faulted = true;

                return Err(Error::Faulted);
            }
            let selection = state.selection().ok_or(Error::Faulted)?;

            return Ok(Status::RecoveryRequired {
                attempt: state.attempt(),
                revision: selection.occurrence.revision,
                scheduled_at: selection.occurrence.scheduled_at,
                recovery: Recovery {
                    state: Rc::downgrade(&self.state),
                    selection,
                    timer: None,
                },
            });
        }
        if matches!(timer, OnceStatus::Inactive) {
            return Ok(if state.cancelled() {
                Status::Cancelled
            } else if state.paused() {
                Status::Paused
            } else {
                Status::Idle
            });
        }
        let Some(selection) = state.selection() else {
            state.faulted = true;

            return Err(Error::Faulted);
        };
        let occurrence = selection.occurrence;

        Ok(match timer {
            OnceStatus::RecoveryRequired(token) => {
                state.block(selection).map_err(|_| Error::Faulted)?;
                Status::RecoveryRequired {
                    attempt: state.attempt(),
                    revision: occurrence.revision,
                    scheduled_at: occurrence.scheduled_at,
                    recovery: Recovery {
                        state: Rc::downgrade(&self.state),
                        selection,
                        timer: Some(token),
                    },
                }
            }
            OnceStatus::Armed(ready_at) => Status::Armed {
                ready_at,
                attempt: state.attempt(),
                revision: occurrence.revision,
                scheduled_at: occurrence.scheduled_at,
            },
            OnceStatus::Running => Status::Running {
                attempt: state.attempt(),
                revision: occurrence.revision,
                scheduled_at: occurrence.scheduled_at,
            },
            _ => return Err(Error::Faulted),
        })
    }

    fn control(
        &self,
        change: impl FnOnce(&mut State, u64) -> Result<(), crate::state::Error>,
    ) -> Result<(), Error> {
        let status = self.status()?;
        if matches!(status, Status::Cancelled) {
            return Err(Error::Cancelled);
        }
        if matches!(status, Status::Faulted) {
            return Err(Error::Faulted);
        }
        let running = matches!(
            status,
            Status::Running { .. } | Status::RecoveryRequired { .. }
        );
        let demand = {
            let mut state = self.state.borrow_mut();
            if change(&mut state, system::time()).is_err() {
                state.faulted = true;

                return Err(Error::Faulted);
            }
            if running {
                None
            } else {
                Some(state.plan(system::time()))
            }
        };
        if let Some(demand) = demand {
            self.timer
                .set(demand.map_or(OnceDemand::Inactive, OnceDemand::At))?;
        }

        Ok(())
    }

    /// Pause future work; accepted work and uncertainty remain owned until settlement.
    /// # Errors
    /// Returns timer or terminal progress errors.
    pub fn pause(&self) -> Result<(), Error> {
        self.control(|state, _| state.pause())
    }

    /// Terminally cancel pending or paused work and discard its remaining batch.
    /// Does not remove the retained timer declaration. Repeated cancellation is
    /// harmless; checkpoint/restore preserves it. Running or uncertain effects
    /// must settle or be explicitly reconciled first.
    /// # Errors
    /// Returns Unsettled for running/recovery work, or timer/terminal errors.
    pub fn cancel(&self) -> Result<(), Error> {
        match self.status()? {
            Status::Cancelled => return Ok(()),
            Status::Running { .. } | Status::RecoveryRequired { .. } => {
                return Err(Error::Unsettled);
            }
            Status::Faulted => return Err(Error::Faulted),
            _ => {}
        }
        self.timer.set(OnceDemand::Inactive)?;
        let mut state = self.state.borrow_mut();
        state.cancel().map_err(|_| Error::Faulted)?;
        state.batch = None;
        state.retry = None;

        Ok(())
    }

    /// Resume according to policy without bypassing unresolved work. A paused
    /// catch-up batch retains its original selection and remaining count; a pending
    /// retry retains its consumed budget and deadline.
    /// # Errors
    /// Returns timer or terminal progress errors.
    pub fn resume(&self) -> Result<(), Error> {
        self.control(|state, _| state.resume())
    }

    /// Replace the schedule for future occurrences; preserve accepted identity/pause.
    /// # Errors
    /// Returns timer or terminal revision/control errors.
    pub fn edit(&self, schedule: Schedule) -> Result<(), Error> {
        self.control(move |state, now| {
            state.edit(now)?;
            state.schedule = schedule;
            state.batch = None;
            // An accepted or uncertain old revision still owns its attempt identity.
            if state.selection().is_none() {
                state.retry = None;
            }
            Ok(())
        })
    }

    /// Apply an explicit owner disposition after effect reconciliation. Retire
    /// discards the retry/batch and skips through now. RetrySafe preserves the
    /// occurrence and consumed budget, with backoff from reconciliation time.
    /// Both leave the job paused. A rejected disposition leaves it blocked.
    /// # Errors
    /// Rejects wrong-job/obsolete recovery, unavailable retries or timer/control faults.
    pub fn recover(
        &self,
        recovery: Recovery,
        disposition: RecoveryDisposition,
    ) -> Result<(), Error> {
        if !Weak::ptr_eq(&recovery.state, &Rc::downgrade(&self.state)) {
            return Err(Error::StaleRecovery);
        }
        {
            let mut state = self.state.borrow_mut();
            if state.faulted {
                return Err(Error::Faulted);
            }
            state
                .recover(recovery.selection, disposition, system::time())
                .map_err(|error| match error {
                    crate::state::Error::Stale => Error::StaleRecovery,
                    crate::state::Error::RetryUnavailable => Error::RetryUnavailable,
                    _ => Error::Faulted,
                })?;
        }
        if let Some(token) = recovery.timer
            && let Err(error) = self.timer.recover(token, OnceDemand::Inactive)
        {
            self.state.borrow_mut().faulted = true;

            return Err(error.into());
        }

        Ok(())
    }
}
