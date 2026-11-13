use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{MissedRunPolicy, RetryPolicy, Schedule};

/// Application-assigned identity. Never reuse it within a retained job history.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct JobId(pub u128);

/// Stable logical effect identity, shared by retries of the same occurrence.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExecutionId {
    /// Owning job.
    pub job: JobId,
    /// Zero-based successful-run sequence.
    pub occurrence: u64,
}

/// Identity of one dispatch. Completion must match the current attempt exactly.
///
/// This is a correlation value, not an authorization credential.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Attempt {
    /// Logical effect identity for destination-side idempotency.
    pub execution: ExecutionId,
    /// Strictly increasing dispatch sequence within this job.
    pub sequence: u64,
}

/// Current durable state of a job.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum JobState {
    /// Eligible at or after the supplied time.
    Pending {
        /// Next dispatch deadline, including retry backoff.
        due_ns: u64,
    },
    /// Pending work is suspended; its retained deadline grants no dispatch.
    Paused {
        /// Original pending deadline, including any retry backoff.
        due_ns: u64,
    },
    /// Dispatch intent exists; its start time is [JobRecord::updated_ns].
    Running,
    /// The effect may have occurred. Automatic dispatch is blocked.
    Uncertain {
        /// Time at which the unresolved attempt started.
        started_ns: u64,
    },
    /// A one-shot job succeeded.
    Completed,
    /// A permanent failure or exhausted retry budget stopped the job.
    Failed,
    /// Pending work was cancelled.
    Cancelled,
}

/// Application classification of an attempt's result.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Outcome {
    /// The effect succeeded.
    Success,
    /// Failure is known and repeating the effect is safe.
    RetryableFailure,
    /// The job should terminate.
    PermanentFailure,
    /// The effect's result is unknown; reconciliation is required.
    Uncertain,
}

/// One recorded outcome and its timestamp, independent of later management.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RecordedOutcome {
    /// Application classification of this result.
    pub outcome: Outcome,
    /// Time the result was recorded, in application-supplied nanoseconds.
    pub at_ns: u64,
}

/// Canonical, fixed-size metadata to persist beside the application's payload.
///
/// Decode within your storage boundary and pass through [Job::restore].
/// The application owns atomic writes, access control and retained-history identity.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JobRecord {
    /// Application-assigned job identity.
    pub id: JobId,
    /// Original scheduling policy.
    pub schedule: Schedule,
    /// Retry policy for each occurrence.
    pub retry: RetryPolicy,
    /// Number of dispatches allocated so far.
    pub sequence: u64,
    /// Logical occurrence ordinal; successful recurrence advances it, never management.
    pub occurrence: u64,
    /// Logical occurrence at which the current scheduling policy began.
    pub schedule_origin: u64,
    /// Dispatch count for the current occurrence.
    pub attempts: u32,
    /// Original deadline of the current occurrence, before retry backoff.
    pub scheduled_ns: u64,
    /// Current execution state.
    pub state: JobState,
    /// Time of the last transition; clocks must not move behind it.
    pub updated_ns: u64,
    /// Latest outcome and its timestamp; this is not a full execution history.
    pub last_outcome: Option<RecordedOutcome>,
}

impl JobRecord {
    // A retained retry must be safe to repeat, within budget and representable.
    // Cancellation retains this evidence without retaining a pending deadline.
    fn retained_retry_due_ns(&self) -> Option<u64> {
        let RecordedOutcome { outcome, at_ns } = self.last_outcome?;
        if self.attempts == 0
            || self.attempts >= self.retry.max_attempts()
            || outcome != Outcome::RetryableFailure
            || at_ns < self.scheduled_ns
        {
            return None;
        }
        self.retry.retry_due_ns(at_ns, self.attempts)
    }

    // Establish arithmetic prerequisites before schedule or state validation.
    fn validate_history(&self) -> Result<u64, JobError> {
        let outcome_ns = self.last_outcome.map(|result| result.at_ns);
        let invalid = || Err(JobError::InvalidRecord);
        let previous_dispatches = self
            .sequence
            .checked_sub(u64::from(self.attempts))
            .ok_or(JobError::InvalidRecord)?;
        if self.attempts > self.retry.max_attempts()
            || self.schedule_origin > self.occurrence
            || self.scheduled_ns < self.schedule.first()
            || (self.occurrence == self.schedule_origin
                && self.scheduled_ns != self.schedule.first())
            || outcome_ns
                .is_some_and(|time| time > self.updated_ns)
            // Every completed occurrence used at most the immutable retry
            // budget. Rescheduling preserves that history; use the total
            // occurrence count, with wide arithmetic for large valid records.
            || u128::from(self.sequence)
                > u128::from(self.occurrence) * u128::from(self.retry.max_attempts())
                    + u128::from(self.attempts)
            || previous_dispatches < self.occurrence
        {
            return invalid();
        }
        // Every allocated retry consumed at least the immutable initial delay.
        // Wide arithmetic also rejects histories needing more than u64 time;
        // this lower bound does not iterate over untrusted counters.
        let retries = self.sequence - self.occurrence - u64::from(self.attempts != 0);
        let previous_retries = previous_dispatches - self.occurrence;
        let delay = u128::from(self.retry.initial_delay_ns());
        let evidence_ns = match self.state {
            JobState::Running => self.updated_ns,
            JobState::Uncertain { started_ns } => started_ns,
            _ => outcome_ns.unwrap_or(0),
        };
        if u128::from(outcome_ns.unwrap_or(0)) < u128::from(previous_retries) * delay
            || u128::from(evidence_ns) < u128::from(retries) * delay
            || (self.attempts > 0
                && u128::from(evidence_ns)
                    < u128::from(self.scheduled_ns)
                        + self.retry.elapsed_retry_delay(self.attempts - 1))
        {
            return invalid();
        }
        Ok(evidence_ns)
    }

    // History admission proves these subtractions cannot underflow.
    fn validate_progress(&self) -> Result<(), JobError> {
        let last_outcome = self.last_outcome.map(|result| result.outcome);
        let outcome_ns = self.last_outcome.map(|result| result.at_ns);
        let occurrences_since_origin = self.occurrence - self.schedule_origin;
        let invalid = || Err(JobError::InvalidRecord);
        match self.schedule {
            Schedule::Once { .. } if self.occurrence != self.schedule_origin => {
                return invalid();
            }
            Schedule::FixedRate {
                first_at_ns,
                every_ns,
                missed,
            } => {
                let delta = self.scheduled_ns - first_at_ns;
                let intervals = delta / every_ns;
                if !delta.is_multiple_of(every_ns.get())
                    || match missed {
                        MissedRunPolicy::CatchUp => intervals != occurrences_since_origin,
                        MissedRunPolicy::Skip => intervals < occurrences_since_origin,
                    }
                {
                    return invalid();
                }
            }
            Schedule::AfterCompletion {
                first_at_ns,
                every_ns,
            } if occurrences_since_origin
                .checked_mul(every_ns.get())
                .and_then(|n| first_at_ns.checked_add(n))
                .is_none_or(|n| n > self.scheduled_ns) =>
            {
                return invalid();
            }
            _ => {}
        }
        if self.sequence == 0 {
            if last_outcome.is_some()
                || !matches!(
                    self.state,
                    JobState::Pending { .. } | JobState::Paused { .. } | JobState::Cancelled
                )
            {
                return invalid();
            }
        } else if self.attempts == 0
            && (self.occurrence == 0 || last_outcome != Some(Outcome::Success))
        {
            return invalid();
        }
        if occurrences_since_origin > 0
            && (self.attempts == 0
                || (self.attempts == 1 && matches!(self.state, JobState::Running)))
        {
            let every_ns = match self.schedule {
                Schedule::FixedRate { every_ns, .. }
                | Schedule::AfterCompletion { every_ns, .. } => every_ns,
                Schedule::Once { .. } => return invalid(),
            };
            // The next deadline minus its interval is the prior completion
            // (AfterCompletion) or its earliest possible time (FixedRate).
            let floor = self
                .scheduled_ns
                .checked_sub(every_ns.get())
                .ok_or(JobError::InvalidRecord)?;
            let Some(completed_ns) = outcome_ns.filter(|&time| time >= floor) else {
                return invalid();
            };
            // Management and dispatch retain this successful completion.
            // Validate its successor even when no pending deadline remains.
            if self.schedule.successor(floor, completed_ns).ok() != Some(Some(self.scheduled_ns)) {
                return invalid();
            }
        }
        Ok(())
    }

    fn validate_state(&self, evidence_ns: u64) -> Result<(), JobError> {
        let last_outcome = self.last_outcome.map(|result| result.outcome);
        let outcome_ns = self.last_outcome.map(|result| result.at_ns);
        let invalid = || Err(JobError::InvalidRecord);
        match self.state {
            JobState::Pending { due_ns } | JobState::Paused { due_ns } => {
                if due_ns < self.scheduled_ns
                    || (self.attempts == 0 && due_ns != self.scheduled_ns)
                    || (self.attempts > 0 && self.retained_retry_due_ns() != Some(due_ns))
                {
                    return invalid();
                }
            }
            JobState::Running | JobState::Uncertain { .. } => {
                let started_ns = evidence_ns;
                if self.attempts == 0
                    || started_ns < self.scheduled_ns
                    || started_ns > self.updated_ns
                {
                    return invalid();
                }
                if matches!(self.state, JobState::Running) {
                    let previous_outcome = if self.attempts > 1 {
                        Some(Outcome::RetryableFailure)
                    } else if self.occurrence > 0 {
                        Some(Outcome::Success)
                    } else {
                        None
                    };
                    if last_outcome != previous_outcome
                        || (self.attempts > 1
                            && (outcome_ns.is_none_or(|time| time < self.scheduled_ns)
                                || outcome_ns
                                    .and_then(|time| {
                                        self.retry.retry_due_ns(time, self.attempts - 1)
                                    })
                                    .is_none_or(|due| due > started_ns)))
                    {
                        return invalid();
                    }
                } else if last_outcome != Some(Outcome::Uncertain)
                    || outcome_ns != Some(self.updated_ns)
                {
                    return invalid();
                }
            }

            JobState::Completed | JobState::Failed => {
                let valid_outcome = match (self.state, last_outcome) {
                    (JobState::Completed, Some(Outcome::Success)) => {
                        matches!(self.schedule, Schedule::Once { .. })
                    }
                    (JobState::Failed, Some(Outcome::PermanentFailure)) => true,
                    (JobState::Failed, Some(Outcome::RetryableFailure)) => {
                        self.attempts == self.retry.max_attempts()
                    }
                    _ => false,
                };
                if self.attempts == 0
                    || self.updated_ns < self.scheduled_ns
                    || outcome_ns != Some(self.updated_ns)
                    || !valid_outcome
                {
                    return invalid();
                }
            }
            JobState::Cancelled => {
                if self.attempts > 0 && self.retained_retry_due_ns().is_none() {
                    return invalid();
                }
            }
        }
        Ok(())
    }
}

/// Checked transitions of one application-owned durable job.
///
/// Persist each successful transition. No method performs an external effect.
/// Errors leave the record unchanged.
/// Serialization delegates to the canonical [JobRecord]. Decode a record and
/// use [Self::restore] to validate it; `Job` cannot be deserialized directly.
///
/// ```compile_fail
/// let job: ic_jobs::Job = serde_json::from_str("{}")?;
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct Job(JobRecord);

/// A rejected configuration, stored record or state transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JobError {
    /// Retry budget or delays were invalid.
    InvalidRetryPolicy,
    /// A timestamp calculation exceeded the nanosecond range.
    DeadlineOverflow,
    /// A dispatch or occurrence counter cannot advance.
    CounterExhausted,
    /// Stored fields contradict the job contract.
    InvalidRecord,
    /// Dispatch was requested before its deadline.
    NotDue,
    /// This operation requires pending work.
    NotPending,
    /// Resuming requires paused work.
    NotPaused,
    /// The current occurrence already allocated an attempt and cannot be rescheduled.
    AlreadyAttempted,
    /// The supplied completion does not identify the active dispatch.
    StaleAttempt,
    /// The job has an unresolved effect and cannot be cancelled.
    InFlight,
    /// A supplied timestamp precedes the previous transition.
    TimeWentBackwards,
    /// Explicit reconciliation requires an uncertain attempt.
    NotUncertain,
}

impl fmt::Display for JobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidRetryPolicy => "invalid retry budget or delay",
            Self::DeadlineOverflow => "job deadline overflow",
            Self::CounterExhausted => "job counter exhausted",
            Self::InvalidRecord => "inconsistent stored job record",
            Self::NotDue => "job is not due",
            Self::NotPending => "job is not pending",
            Self::NotPaused => "job is not paused",
            Self::AlreadyAttempted => "current occurrence has already been attempted",
            Self::StaleAttempt => "completion does not match the active attempt",
            Self::InFlight => "job has an unresolved effect",
            Self::TimeWentBackwards => "time precedes the previous transition",
            Self::NotUncertain => "job is not awaiting reconciliation",
        };
        f.write_str(message)
    }
}

impl std::error::Error for JobError {}

impl Job {
    /// Create a pending job with an application-assigned identity.
    ///
    /// Schedule intervals and retry configuration are checked before construction.
    #[must_use]
    pub const fn new(id: JobId, schedule: Schedule, retry: RetryPolicy) -> Self {
        Self(JobRecord {
            id,
            schedule,
            retry,
            sequence: 0,
            occurrence: 0,
            schedule_origin: 0,
            attempts: 0,
            scheduled_ns: schedule.first(),
            state: JobState::Pending {
                due_ns: schedule.first(),
            },
            updated_ns: 0,
            last_outcome: None,
        })
    }

    /// Validate a decoded record without changing its execution state.
    ///
    /// Running and uncertain records remain blocked after reconstruction.
    /// Restoring does not grant permission to repeat an external effect.
    pub fn restore(record: JobRecord) -> Result<Self, JobError> {
        let evidence_ns = record.validate_history()?;
        record.validate_progress()?;
        record.validate_state(evidence_ns)?;
        Ok(Self(record))
    }

    /// Return the metadata the application must persist.
    #[must_use]
    pub const fn record(&self) -> JobRecord {
        self.0
    }

    /// Return the next dispatch deadline, excluding unresolved and terminal jobs.
    #[must_use]
    pub const fn next_due_ns(&self) -> Option<u64> {
        match self.0.state {
            JobState::Pending { due_ns } => Some(due_ns),
            _ => None,
        }
    }

    /// Return the correlation value of an unresolved attempt.
    #[must_use]
    pub const fn active_attempt(&self) -> Option<Attempt> {
        match self.0.state {
            JobState::Running | JobState::Uncertain { .. } => Some(self.attempt()),
            _ => None,
        }
    }

    const fn attempt(&self) -> Attempt {
        Attempt {
            execution: ExecutionId {
                job: self.0.id,
                occurrence: self.0.occurrence,
            },
            sequence: self.0.sequence,
        }
    }

    fn check_time(&self, now_ns: u64) -> Result<(), JobError> {
        if now_ns < self.0.updated_ns {
            Err(JobError::TimeWentBackwards)
        } else {
            Ok(())
        }
    }

    fn pending_or_paused_state(&self) -> Result<JobState, JobError> {
        match self.0.state {
            state @ (JobState::Pending { .. } | JobState::Paused { .. }) => Ok(state),
            JobState::Running | JobState::Uncertain { .. } => Err(JobError::InFlight),
            _ => Err(JobError::NotPending),
        }
    }

    /// Allocate dispatch intent. Commit the resulting record before external work.
    ///
    /// The application must not reuse this occurrence's effect identity with a
    /// changed payload, including after an upgrade.
    pub fn start(&mut self, now_ns: u64) -> Result<Attempt, JobError> {
        self.check_time(now_ns)?;
        let JobState::Pending { due_ns } = self.0.state else {
            return Err(JobError::NotPending);
        };
        if now_ns < due_ns {
            return Err(JobError::NotDue);
        }
        let sequence = self
            .0
            .sequence
            .checked_add(1)
            .ok_or(JobError::CounterExhausted)?;
        // Admitted Pending jobs have attempts < max_attempts <= u32::MAX.
        let attempts = self.0.attempts + 1;
        self.0.sequence = sequence;
        self.0.attempts = attempts;
        self.0.state = JobState::Running;
        self.0.updated_ns = now_ns;
        Ok(self.attempt())
    }

    /// Record the exact running attempt's result.
    ///
    /// Classify lost replies as uncertain. A retryable failure asserts repeating
    /// the effect is safe. Counter/deadline errors leave dispatch intent intact.
    pub fn finish(
        &mut self,
        attempt: Attempt,
        now_ns: u64,
        outcome: Outcome,
    ) -> Result<(), JobError> {
        if !matches!(self.0.state, JobState::Running) || attempt != self.attempt() {
            return Err(JobError::StaleAttempt);
        }
        self.transition(now_ns, outcome)
    }

    /// Explicitly resolve an uncertain effect after application reconciliation.
    ///
    /// Success records a confirmed effect. RetryableFailure asserts retry safety;
    /// PermanentFailure records a terminal disposition. Uncertain keeps it blocked.
    pub fn resolve(
        &mut self,
        attempt: Attempt,
        now_ns: u64,
        outcome: Outcome,
    ) -> Result<(), JobError> {
        if !matches!(self.0.state, JobState::Uncertain { .. }) {
            return Err(JobError::NotUncertain);
        }
        if attempt != self.attempt() {
            return Err(JobError::StaleAttempt);
        }
        self.transition(now_ns, outcome)
    }

    fn transition(&mut self, now_ns: u64, outcome: Outcome) -> Result<(), JobError> {
        self.check_time(now_ns)?;
        let mut next = self.0;
        next.updated_ns = now_ns;
        next.last_outcome = Some(RecordedOutcome {
            outcome,
            at_ns: now_ns,
        });
        match outcome {
            Outcome::Success => {
                if let Some(deadline) = next.schedule.successor(next.scheduled_ns, now_ns)? {
                    next.occurrence = next
                        .occurrence
                        .checked_add(1)
                        .ok_or(JobError::CounterExhausted)?;
                    next.attempts = 0;
                    next.scheduled_ns = deadline;
                    next.state = JobState::Pending { due_ns: deadline };
                } else {
                    next.state = JobState::Completed;
                }
            }
            Outcome::RetryableFailure if next.attempts < next.retry.max_attempts() => {
                let due_ns = next
                    .retry
                    .retry_due_ns(now_ns, next.attempts)
                    .ok_or(JobError::DeadlineOverflow)?;
                next.state = JobState::Pending { due_ns };
            }
            Outcome::RetryableFailure | Outcome::PermanentFailure => next.state = JobState::Failed,
            Outcome::Uncertain => {
                // The only callers admit Running or Uncertain. Reconciliation
                // that remains uncertain preserves the existing start time.
                if matches!(next.state, JobState::Running) {
                    next.state = JobState::Uncertain {
                        started_ns: self.0.updated_ns,
                    };
                }
            }
        }
        self.0 = next;
        Ok(())
    }

    /// Suspend pending work without changing its deadline, identity or retry evidence.
    ///
    /// Persist the paused record before reconciling the consumer's wakeup. A
    /// paused job has no dispatch deadline, including after reconstruction.
    pub fn pause(&mut self, now_ns: u64) -> Result<(), JobError> {
        self.check_time(now_ns)?;
        let due_ns = match self.0.state {
            JobState::Pending { due_ns } => due_ns,
            JobState::Running | JobState::Uncertain { .. } => {
                return Err(JobError::InFlight);
            }
            _ => return Err(JobError::NotPending),
        };
        self.0.state = JobState::Paused { due_ns };
        self.0.updated_ns = now_ns;
        Ok(())
    }

    /// Resume the retained deadline and policy without allocating an attempt.
    ///
    /// Overdue work remains overdue; existing Skip/CatchUp semantics and the
    /// application's bounded dispatch budget apply. Retry backoff is not reset.
    pub fn resume(&mut self, now_ns: u64) -> Result<(), JobError> {
        self.check_time(now_ns)?;
        let due_ns = match self.0.state {
            JobState::Paused { due_ns } => due_ns,
            JobState::Running | JobState::Uncertain { .. } => {
                return Err(JobError::InFlight);
            }
            _ => return Err(JobError::NotPaused),
        };
        self.0.state = JobState::Pending { due_ns };
        self.0.updated_ns = now_ns;
        Ok(())
    }

    /// Replace scheduling intent from the current unstarted occurrence forward.
    ///
    /// Keep the job, logical occurrence, dispatch sequence, retry policy and
    /// latest outcome. Paused work stays paused. Already-attempted occurrences
    /// cannot be rescheduled, including known-safe retry waiting. Payloads remain
    /// consumer-owned and must retain their effect identity.
    pub fn reschedule(&mut self, schedule: Schedule, now_ns: u64) -> Result<(), JobError> {
        self.check_time(now_ns)?;
        let paused = matches!(self.pending_or_paused_state()?, JobState::Paused { .. });
        if self.0.attempts != 0 {
            return Err(JobError::AlreadyAttempted);
        }
        let due_ns = schedule.first();
        self.0.schedule = schedule;
        self.0.schedule_origin = self.0.occurrence;
        self.0.scheduled_ns = due_ns;
        self.0.state = if paused {
            JobState::Paused { due_ns }
        } else {
            JobState::Pending { due_ns }
        };
        self.0.updated_ns = now_ns;
        Ok(())
    }

    /// Cancel pending or paused work. An unresolved effect must be reconciled first.
    pub fn cancel(&mut self, now_ns: u64) -> Result<(), JobError> {
        self.check_time(now_ns)?;
        self.pending_or_paused_state()?;
        self.0.state = JobState::Cancelled;
        self.0.updated_ns = now_ns;
        Ok(())
    }
}
