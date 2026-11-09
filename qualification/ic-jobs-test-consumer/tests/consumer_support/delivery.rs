//! One durable effect with fallible commits; collection behavior stays with Queue.

use super::{Result, Store};
use ic_jobs::{
    Attempt, ExecutionId, Job, JobError, JobId, JobRecord, Outcome, RetryPolicy, Schedule,
    start_next,
};
use ic_storage::MemoryRequest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Snapshot<J = JobRecord> {
    pub(crate) job: J,
    pub(crate) payload: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Delivery {
    pub(crate) attempt: Attempt,
    pub(crate) payload: String,
}

pub(crate) struct DeliveryFixture {
    pub(crate) store: Store,
}

impl DeliveryFixture {
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            store: Store::from_bytes(bytes),
        }
    }

    pub(crate) fn memory_request() -> Result<MemoryRequest> {
        Ok(MemoryRequest::new(
            "test-notifications",
            "test.notifications.queue.v1",
        )?)
    }

    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            store: Store::new(&Snapshot {
                job: Job::new(
                    JobId(1),
                    Schedule::Once { at_ns: 100 },
                    RetryPolicy::new(2, 10, 10).unwrap(),
                ),
                payload: "Your report is ready".into(),
            })?,
        })
    }

    pub(crate) fn load(&self) -> Result<Snapshot<Job>> {
        let snapshot: Snapshot = self.store.read()?;
        Ok(Snapshot {
            job: Job::restore(snapshot.job)?,
            payload: snapshot.payload,
        })
    }

    pub(crate) fn transition(
        &mut self,
        operation: impl FnOnce(&mut Job) -> std::result::Result<(), JobError>,
    ) -> Result<()> {
        let mut snapshot = self.load()?;
        operation(&mut snapshot.job)?;
        self.store.write(&snapshot)
    }

    pub(crate) fn dispatch_due(&mut self, now_ns: u64) -> Result<Option<Delivery>> {
        let mut snapshot = self.load()?;
        let Some((_, attempt)) = start_next(std::slice::from_mut(&mut snapshot.job), now_ns)?
        else {
            return Ok(None);
        };
        // An envelope escapes only after the intent and immutable payload commit.
        self.store.write(&snapshot)?;
        Ok(Some(Delivery {
            attempt,
            payload: snapshot.payload,
        }))
    }

    pub(crate) fn finish(&mut self, attempt: Attempt, now_ns: u64, outcome: Outcome) -> Result<()> {
        self.transition(|job| job.finish(attempt, now_ns, outcome))
    }

    pub(crate) fn reconcile(
        &mut self,
        attempt: Attempt,
        evidence: &Evidence,
        now_ns: u64,
    ) -> Result<bool> {
        let mut snapshot = self.load()?;
        let Some(outcome) = evidence.recorded_result(attempt, &snapshot.payload) else {
            return Ok(false);
        };
        snapshot.job.resolve(attempt, now_ns, outcome)?;
        self.store.write(&snapshot)?;
        Ok(true)
    }
}

// Facts supplied independently by the effect owner; no second receiver engine.
#[derive(Default)]
pub(crate) struct Evidence {
    pub(crate) receipts: Vec<(ExecutionId, String)>,
    pub(crate) rejections: Vec<(Attempt, String)>,
}

impl Evidence {
    fn recorded_result(&self, attempt: Attempt, payload: &str) -> Option<Outcome> {
        if self
            .receipts
            .iter()
            .any(|(id, value)| *id == attempt.execution && value == payload)
        {
            Some(Outcome::Success)
        } else if self
            .rejections
            .iter()
            .any(|(id, value)| *id == attempt && value == payload)
        {
            Some(Outcome::RetryableFailure)
        } else {
            None // Absence of evidence never authorizes retry.
        }
    }
}
