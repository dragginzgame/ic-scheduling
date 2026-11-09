//! Persisted queue shape and bounded codec/admission; no storage, dispatch or effects.

use super::job_error;
use crate::api::{Delivery, DeliveryPayload, Error, Init, Status, Task, View};
use ic_jobs::{Attempt, Job, JobId, JobRecord, JobState, next_due_ns};
use ic_wire::{Limits, native};
use serde::{Deserialize, Serialize};

pub(super) const MAX_JOBS: usize = 32;
pub(super) const MAX_BYTES: usize = 24 * 1024;
// This queue's identity is independent of Storage's allocation-directory format.
pub(super) const FORMAT: &[u8; 4] = b"JQW\x01";
pub(super) const LIMITS: Limits = Limits {
    bytes: MAX_BYTES - FORMAT.len(),
    allocation: 1024 * 1024,
    nodes: 8192,
    depth: 32,
};
pub(super) const MAX_MESSAGE: usize = 256;
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Entry<J = JobRecord> {
    pub(super) job: J,
    pub(super) task: Task,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Snapshot<J = JobRecord> {
    pub(super) init: Init,
    pub(super) next_id: u64,
    pub(super) jobs: Vec<Entry<J>>,
    pub(super) counter: u64,
    pub(super) receipts: Vec<Delivery>,
}

impl<J> Snapshot<J> {
    fn check_header(&self) -> Result<(), Error> {
        if self.jobs.len() > MAX_JOBS || self.receipts.len() > MAX_JOBS || self.next_id == 0 {
            return Err(Error::InvalidStorage);
        }
        Ok(())
    }
}

fn check_entry(id: JobId, task: &Task, next_id: u64, previous: &mut u128) -> Result<(), Error> {
    if id.0 <= *previous || id.0 >= u128::from(next_id) || !task_valid(task) {
        return Err(Error::InvalidStorage);
    }
    *previous = id.0;
    Ok(())
}

fn check_receipts(receipts: &[Delivery]) -> Result<(), Error> {
    for (index, receipt) in receipts.iter().enumerate() {
        if !delivery_valid(receipt)
            || receipts[..index]
                .iter()
                .any(|other| receipt_key(other) == receipt_key(receipt))
        {
            return Err(Error::InvalidStorage);
        }
    }
    Ok(())
}

impl Snapshot {
    // Consume each record and payload together, restoring before checking that
    // entry's metadata. Typed writes use the same collection checks by borrowing.
    pub(super) fn admit(self) -> Result<Snapshot<Job>, Error> {
        self.check_header()?;
        let mut previous = 0u128;
        let jobs = self
            .jobs
            .into_iter()
            .map(|entry| {
                let id = entry.job.id;
                let job = Job::restore(entry.job).map_err(job_error)?;
                check_entry(id, &entry.task, self.next_id, &mut previous)?;
                Ok(Entry {
                    job,
                    task: entry.task,
                })
            })
            .collect::<Result<_, _>>()?;
        check_receipts(&self.receipts)?;
        Ok(Snapshot {
            init: self.init,
            next_id: self.next_id,
            jobs,
            counter: self.counter,
            receipts: self.receipts,
        })
    }
}

impl Snapshot<Job> {
    // Records came from validated Jobs and maintained public transitions.
    // Borrow collection metadata without serializing or restoring it again.
    pub(super) fn check_collection(&self) -> Result<(), Error> {
        self.check_header()?;
        let mut previous = 0;
        for entry in &self.jobs {
            check_entry(
                entry.job.record().id,
                &entry.task,
                self.next_id,
                &mut previous,
            )?;
        }
        check_receipts(&self.receipts)
    }

    pub(super) fn next_due_ns(&self) -> Option<u64> {
        next_due_ns(self.jobs.iter().map(|entry| &entry.job))
    }

    pub(super) fn index(&self, id: u64) -> Result<usize, Error> {
        self.jobs
            .iter()
            .position(|entry| entry.job.record().id == JobId(u128::from(id)))
            .ok_or(Error::UnknownJob)
    }

    pub(super) fn take_entry(mut self, id: u64) -> Result<Entry<Job>, Error> {
        let index = self.index(id)?;
        Ok(self.jobs.swap_remove(index))
    }

    pub(super) fn receipt_index(&self, delivery: &Delivery) -> Option<usize> {
        let key = receipt_key(delivery);
        self.receipts
            .iter()
            .position(|receipt| receipt_key(receipt) == key)
    }
}

impl Entry<Job> {
    pub(super) fn into_view(self) -> View {
        let record = self.job.record();
        View {
            // Admission bounds IDs below the u64 allocator; creation uses it directly.
            id: record.id.0 as u64,
            task: self.task,
            status: match record.state {
                JobState::Pending { due_ns } => Status::Pending { due_ns },
                JobState::Paused { .. } => Status::Paused,
                JobState::Running => Status::Running,
                JobState::Uncertain { .. } => Status::Uncertain,
                JobState::Completed => Status::Completed,
                JobState::Failed => Status::Failed,
                JobState::Cancelled => Status::Cancelled,
            },
            sequence: record.sequence,
            occurrence: record.occurrence,
        }
    }

    pub(super) fn delivery_parts(&self) -> Result<(Attempt, &DeliveryPayload), Error> {
        let attempt = self.job.active_attempt().ok_or(Error::InvalidInput)?;
        let Task::Deliver(payload) = &self.task else {
            return Err(Error::InvalidInput);
        };
        Ok((attempt, payload))
    }

    pub(super) fn delivery(&self) -> Result<Delivery, Error> {
        let (attempt, payload) = self.delivery_parts()?;
        Ok(Delivery {
            // The admitted entry and its active attempt retain the same bounded ID.
            job: attempt.execution.job.0 as u64,
            occurrence: attempt.execution.occurrence,
            sequence: attempt.sequence,
            payload: payload.clone(),
        })
    }
}

pub(super) fn delivery_valid(delivery: &Delivery) -> bool {
    delivery.job != 0 && delivery.sequence != 0 && payload_valid(&delivery.payload)
}

// Receipt identity is one logical occurrence within the authenticated sender.
// Exact confirmation still compares the full envelope, including its sequence.
pub(super) fn receipt_key(delivery: &Delivery) -> (u64, u64) {
    (delivery.job, delivery.occurrence)
}

pub(super) fn task_valid(task: &Task) -> bool {
    match task {
        Task::Increment(_) => true,
        Task::Deliver(payload) => payload_valid(payload),
    }
}

pub(super) fn payload_valid(payload: &DeliveryPayload) -> bool {
    payload.message.len() <= MAX_MESSAGE
}

pub(super) fn decode(bytes: &[u8]) -> Result<Snapshot<Job>, Error> {
    if bytes.len() > MAX_BYTES {
        return Err(Error::InvalidStorage);
    }
    let payload = bytes.strip_prefix(FORMAT).ok_or(Error::InvalidStorage)?;
    let snapshot: Snapshot = native::decode(payload, LIMITS).map_err(|_| Error::InvalidStorage)?;
    // Every decoded JobRecord enters through the validated restore boundary.
    snapshot.admit()
}

pub(super) fn encode<J: Serialize>(snapshot: &Snapshot<J>) -> Result<Vec<u8>, Error> {
    let payload = native::encode(snapshot, LIMITS).map_err(|error| match error {
        ic_wire::Error::Limit => Error::Capacity,
        _ => Error::InvalidStorage,
    })?;
    let mut bytes = Vec::with_capacity(FORMAT.len() + payload.len());
    bytes.extend_from_slice(FORMAT);
    bytes.extend_from_slice(&payload);
    Ok(bytes)
}
