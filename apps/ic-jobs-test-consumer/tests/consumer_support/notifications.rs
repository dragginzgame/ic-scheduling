use super::{Result, Store};
use ic_jobs::{
    Attempt, ExecutionId, Job, JobError, JobId, JobRecord, Outcome, RetryPolicy, Schedule,
    next_due_ns, start_next,
};
use ic_storage::MemoryRequest;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct Notification {
    pub(crate) recipient: String,
    pub(crate) body: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct NotificationSnapshot<J = JobRecord> {
    pub(crate) job: J,
    pub(crate) payload: Notification,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Delivery {
    pub(crate) attempt: Attempt,
    pub(crate) payload: Notification,
}

// The synchronous scheduler produces committed intent for a separate delivery
// continuation. Neither the scheduler nor a watchdog callback calls a receiver.
// This fixture owns no endpoint/authentication API; the real Queue tests do.
pub(crate) struct Notifications {
    pub(crate) store: Store,
}

const MAX_NOTIFICATIONS: usize = 4;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Dispatch {
    pub(crate) delivery: Option<Delivery>,
    // The application projects this committed queue deadline into its watchdog.
    // Native execution does not register or qualify a platform timer.
    pub(crate) next_due_ns: Option<u64>,
}

impl Notifications {
    // Reconstruction resets only volatile fault injection. Reads still admit
    // the committed snapshot and every JobRecord through load/Job::restore.
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            store: Store::from_bytes(bytes),
        }
    }

    fn index(jobs: &[NotificationSnapshot<Job>], id: JobId) -> Option<usize> {
        jobs.iter().position(|entry| entry.job.record().id == id)
    }

    // One consumer-owned store holds records and immutable payloads together.
    // The eventual host supplies allocation policy and owns one bootstrap.
    pub(crate) fn memory_request() -> Result<MemoryRequest> {
        Ok(MemoryRequest::new(
            "test-notifications",
            "test.notifications.queue.v1",
        )?)
    }

    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            store: Store::new(&[NotificationSnapshot {
                job: Job::new(
                    JobId(1),
                    Schedule::Once { at_ns: 100 },
                    RetryPolicy::new(2, 10, 10).unwrap(),
                ),
                payload: Notification {
                    recipient: "alice".into(),
                    body: "Your report is ready".into(),
                },
            }])?,
        })
    }

    pub(crate) fn load(&self) -> Result<Vec<NotificationSnapshot<Job>>> {
        let snapshots: Vec<NotificationSnapshot> = self.store.read()?;
        if snapshots.len() > MAX_NOTIFICATIONS {
            return Err(
                io::Error::new(io::ErrorKind::InvalidData, "too many notifications").into(),
            );
        }
        let mut jobs = Vec::with_capacity(snapshots.len());
        for snapshot in snapshots {
            if Self::index(&jobs, snapshot.job.id).is_some() {
                return Err(
                    io::Error::new(io::ErrorKind::InvalidData, "duplicate job identity").into(),
                );
            }
            jobs.push(NotificationSnapshot {
                job: Job::restore(snapshot.job)?,
                payload: snapshot.payload,
            });
        }
        Ok(jobs)
    }

    pub(crate) fn commit(&mut self, jobs: &[NotificationSnapshot<Job>]) -> Result<()> {
        self.store.write(&jobs)
    }

    pub(crate) fn enqueue(&mut self, id: JobId, at_ns: u64, payload: Notification) -> Result<()> {
        let mut jobs = self.load()?;
        if jobs.len() == MAX_NOTIFICATIONS || Self::index(&jobs, id).is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "queue full or identity reused",
            )
            .into());
        }
        let job = Job::new(id, Schedule::Once { at_ns }, RetryPolicy::NONE);
        jobs.push(NotificationSnapshot { job, payload });
        self.commit(&jobs)
    }

    pub(crate) fn cancel(&mut self, id: JobId, now_ns: u64) -> Result<()> {
        let mut jobs = self.load()?;
        let index = Self::index(&jobs, id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "job not retained"))?;
        jobs[index].job.cancel(now_ns)?;
        self.commit(&jobs)
    }

    pub(crate) fn dispatch_due(&mut self, now_ns: u64) -> Result<Dispatch> {
        let mut jobs = self.load()?;
        let Some((index, attempt)) =
            start_next(jobs.iter_mut().map(|entry| &mut entry.job), now_ns)?
        else {
            return Ok(Dispatch {
                delivery: None,
                next_due_ns: next_due_ns(jobs.iter().map(|entry| &entry.job)),
            });
        };
        self.commit(&jobs)?;
        // No delivery capability escapes before the intent and payload commit.
        let due = next_due_ns(jobs.iter().map(|entry| &entry.job));
        let payload = jobs.swap_remove(index).payload;
        Ok(Dispatch {
            delivery: Some(Delivery { attempt, payload }),
            next_due_ns: due,
        })
    }

    pub(crate) fn next_due_ns(&self) -> Result<Option<u64>> {
        let jobs = self.load()?;
        Ok(next_due_ns(jobs.iter().map(|entry| &entry.job)))
    }

    pub(crate) fn outstanding(&self) -> Result<Vec<Delivery>> {
        // Inspect unresolved envelopes for reconciliation, never for automatic replay.
        let jobs = self.load()?;
        Ok(jobs
            .into_iter()
            .filter_map(|entry| {
                entry.job.active_attempt().map(|attempt| Delivery {
                    attempt,
                    payload: entry.payload,
                })
            })
            .collect())
    }

    pub(crate) fn finish(&mut self, attempt: Attempt, now_ns: u64, outcome: Outcome) -> Result<()> {
        let mut jobs = self.load()?;
        let index = Self::index(&jobs, attempt.execution.job).ok_or(JobError::StaleAttempt)?;
        jobs[index].job.finish(attempt, now_ns, outcome)?;
        self.commit(&jobs)
    }

    pub(crate) fn reconcile(
        &mut self,
        attempt: Attempt,
        destination: &Destination,
        now_ns: u64,
    ) -> Result<bool> {
        let mut jobs = self.load()?;
        let index = Self::index(&jobs, attempt.execution.job).ok_or(JobError::StaleAttempt)?;
        let Some(outcome) = destination.recorded_result(attempt, &jobs[index].payload) else {
            // A missing receipt is not proof that delivery failed.
            return Ok(false);
        };
        jobs[index].job.resolve(attempt, now_ns, outcome)?;
        self.commit(&jobs)?;
        Ok(true)
    }
}

// A separate destination owns receipts/idempotency, outside the sender's
// reconstructed history. This fixture retains at most four logical deliveries.
#[derive(Default)]
pub(crate) struct Destination {
    pub(crate) receipts: Vec<(ExecutionId, Notification)>,
    pub(crate) rejections: Vec<(Attempt, Notification)>,
    pub(crate) reject_next: bool,
    pub(crate) calls: usize,
}

impl Destination {
    fn receipt_payloads(&self, execution: ExecutionId) -> impl Iterator<Item = &Notification> {
        self.receipts
            .iter()
            .filter_map(move |(recorded, payload)| (*recorded == execution).then_some(payload))
    }

    fn rejection_payloads(&self, attempt: Attempt) -> impl Iterator<Item = &Notification> {
        self.rejections
            .iter()
            .filter_map(move |(recorded, payload)| (*recorded == attempt).then_some(payload))
    }

    // Match the immutable payload, preferring a logical execution receipt over
    // an exact-attempt rejection. No recorded result grants no retry permission.
    fn recorded_result(&self, attempt: Attempt, payload: &Notification) -> Option<Outcome> {
        if self
            .receipt_payloads(attempt.execution)
            .any(|recorded| recorded == payload)
        {
            Some(Outcome::Success)
        } else if self
            .rejection_payloads(attempt)
            .any(|recorded| recorded == payload)
        {
            // Exact destination evidence confirms this attempt had no effect.
            Some(Outcome::RetryableFailure)
        } else {
            None
        }
    }

    pub(crate) fn deliver(&mut self, delivery: &Delivery) -> bool {
        self.calls += 1;
        if let Some(payload) = self.receipt_payloads(delivery.attempt.execution).next() {
            assert_eq!(payload, &delivery.payload);
            return true;
        }
        if let Some(payload) = self.rejection_payloads(delivery.attempt).next() {
            assert_eq!(payload, &delivery.payload);
            return false;
        }
        if std::mem::take(&mut self.reject_next) {
            // At most two attempts for each of this fixture's four jobs.
            assert!(self.rejections.len() < 8);
            self.rejections
                .push((delivery.attempt, delivery.payload.clone()));
            return false;
        }
        assert!(self.receipts.len() < 4);
        self.receipts
            .push((delivery.attempt.execution, delivery.payload.clone()));
        true
    }
}
