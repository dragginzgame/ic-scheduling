mod snapshot;

use snapshot::{Entry, MAX_BYTES, MAX_JOBS, Snapshot, decode, delivery_valid, encode, task_valid};

use std::num::NonZeroU64;

use ic_jobs::{
    Job, JobId, JobRecord, JobState, MissedRunPolicy, Outcome, RetryPolicy, Schedule, start_next,
};
use ic_platform_types::Principal;
use ic_storage::{
    AllocationGrants, GenericAllocationPolicy, Memory, MemoryAuthority, MemoryBacking,
    MemoryRequest, MemoryRuntime, RuntimeMemory, SealedDeclarationSnapshot, StableKey,
    adapter::{CollectionError, StableCell},
};

use crate::api::{Create, Delivery, Error, Init, Task, Timing, View};
use crate::observations::{Event, Observations};

pub(crate) const BATCH: usize = 2;
const KEY: &str = "test.jobs.consumer.management.v1";

/// Consumer-local reply and deadline from successfully admitted durable state.
/// Mutated state is committed before return; idle dispatch does not write.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Committed<T> {
    pub(crate) value: T,
    pub(crate) next_due_ns: Option<u64>,
}

/// One bounded dispatch batch; work counts attempts, not confirmed effects.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct DispatchBatch {
    pub(crate) deliveries: Vec<Delivery>,
    pub(crate) work: u64,
}

/// Counts projected from the admitted queue and its independent receipts.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct QueueTotals {
    pub(crate) counter: u64,
    pub(crate) receipts: u64,
}

// Storage owns checked allocation and Cell admission; the consumer owns
// its snapshot codec, collection semantics and explicit lifecycle calls.
pub(crate) struct Queue<M: Memory> {
    cell: StableCell<Vec<u8>, M>,
    init: Init,
    observations: Observations,
}

fn job_error(error: ic_jobs::JobError) -> Error {
    Error::Job(error.to_string())
}
fn memory_error(error: impl std::fmt::Display) -> Error {
    Error::Memory(error.to_string())
}

fn cell_error(error: CollectionError) -> Error {
    match error {
        CollectionError::NotEmpty
        | CollectionError::Cell(_)
        | CollectionError::Format
        | CollectionError::Checksum
        | CollectionError::Corrupt(_) => Error::InvalidStorage,
        other => memory_error(other),
    }
}

// The consumer host composes one namespace; Storage alone assigns allocation IDs.
fn queue_memory<M: MemoryBacking>(memory: M) -> Result<RuntimeMemory<M>, Error> {
    let mut runtime = MemoryRuntime::new(memory).map_err(memory_error)?;
    let requests = SealedDeclarationSnapshot::new(vec![
        MemoryRequest::new("test-jobs", KEY).map_err(memory_error)?,
    ])
    .map_err(memory_error)?;
    let grants = AllocationGrants::new(vec![
        MemoryAuthority::new("test-jobs", "test.jobs.").map_err(memory_error)?,
    ])
    .map_err(memory_error)?;
    runtime
        .bootstrap(&requests, &grants, &GenericAllocationPolicy)
        .map_err(memory_error)?;
    runtime
        .open_memory(&StableKey::parse(KEY).map_err(memory_error)?)
        .map_err(memory_error)
}

fn schedule(timing: Timing) -> Result<Schedule, Error> {
    Ok(match timing {
        Timing::Once(at_ns) => Schedule::Once { at_ns },
        Timing::FixedRate {
            first_ns,
            every_ns,
            catch_up,
        } => Schedule::FixedRate {
            first_at_ns: first_ns,
            every_ns: NonZeroU64::new(every_ns).ok_or(Error::InvalidInput)?,
            missed: if catch_up {
                MissedRunPolicy::CatchUp
            } else {
                MissedRunPolicy::Skip
            },
        },
        Timing::AfterCompletion { first_ns, every_ns } => Schedule::AfterCompletion {
            first_at_ns: first_ns,
            every_ns: NonZeroU64::new(every_ns).ok_or(Error::InvalidInput)?,
        },
    })
}

impl<M: Memory> Queue<M> {
    pub(crate) fn open(memory: M, initial: Option<Init>) -> Result<(Self, Option<u64>), Error>
    where
        M: MemoryBacking,
    {
        if initial.is_none() && memory.size() == 0 {
            return Err(Error::InvalidStorage);
        }
        let memory = queue_memory(memory)?;
        let cell = if let Some(init) = initial {
            // Fresh admission cannot overwrite a retained allocation.
            StableCell::create(
                memory,
                encode(&Snapshot::<JobRecord> {
                    init,
                    next_id: 1,
                    jobs: vec![],
                    counter: 0,
                    receipts: vec![],
                })?,
            )
            .map_err(cell_error)?
        } else {
            StableCell::load(memory, MAX_BYTES).map_err(cell_error)?
        };
        let snapshot = decode(cell.get())?;
        let next_due_ns = snapshot.next_due_ns();
        Ok((
            Self {
                cell,
                init: snapshot.init,
                observations: Observations::new(),
            },
            next_due_ns,
        ))
    }

    pub(crate) fn authorize(&self, caller: Principal) -> Result<(), Error> {
        if caller != self.init.manager {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }

    // Management entrypoints authorize before projecting heap diagnostics, just
    // as they do before ordinary queue operations. These reads need no reload.
    pub(crate) fn job_metrics(&self, version: u64) -> crate::api::JobMetrics {
        self.observations.metrics(version)
    }

    pub(crate) fn reset_job_metrics(&mut self, version: u64) -> crate::api::JobMetrics {
        self.observations.reset_metrics(version)
    }

    pub(crate) fn job_logs(
        &self,
        request: crate::api::JobLogRequest,
        version: u64,
    ) -> Result<crate::api::JobLogPage, Error> {
        self.observations.logs(request, version)
    }

    fn load(&self) -> Result<Snapshot<Job>, Error> {
        let snapshot = decode(self.cell.get())?;
        if snapshot.init != self.init {
            return Err(Error::InvalidStorage);
        }
        Ok(snapshot)
    }

    fn commit<T>(&mut self, snapshot: Snapshot<Job>, value: T) -> Result<Committed<T>, Error> {
        let next_due_ns = snapshot.next_due_ns();
        let bytes = encode(&snapshot)?;
        snapshot.check_collection()?;
        self.cell.set(bytes).map_err(memory_error)?;
        Ok(Committed { value, next_due_ns })
    }

    pub(crate) fn create(&mut self, input: Create) -> Result<Committed<u64>, Error> {
        let mut snapshot = self.load()?;
        if snapshot.jobs.len() == MAX_JOBS {
            return Err(Error::Capacity);
        }
        if !task_valid(&input.task) {
            return Err(Error::InvalidInput);
        }
        let schedule = schedule(input.timing)?;
        let id = snapshot.next_id;
        snapshot.next_id = id.checked_add(1).ok_or(Error::Capacity)?;
        snapshot.jobs.push(Entry {
            job: Job::new(JobId(u128::from(id)), schedule, RetryPolicy::NONE),
            task: input.task,
        });
        self.commit(snapshot, id)
    }

    pub(crate) fn page(&self, after: Option<u64>, limit: u8) -> Result<Vec<View>, Error> {
        if !(1..=8).contains(&limit) {
            return Err(Error::InvalidInput);
        }
        let snapshot = self.load()?;
        Ok(snapshot
            .jobs
            .into_iter()
            .filter(|entry| entry.job.record().id.0 > u128::from(after.unwrap_or(0)))
            .take(usize::from(limit))
            .map(Entry::into_view)
            .collect())
    }

    pub(crate) fn inspect(&self, id: u64) -> Result<View, Error> {
        Ok(self.load()?.take_entry(id)?.into_view())
    }

    pub(crate) fn cancel(&mut self, id: u64, now: u64) -> Result<Committed<()>, Error> {
        self.manage(id, |job| job.cancel(now).map_err(job_error))
    }

    pub(crate) fn pause(&mut self, id: u64, now: u64) -> Result<Committed<()>, Error> {
        self.manage(id, |job| job.pause(now).map_err(job_error))
    }

    pub(crate) fn resume(&mut self, id: u64, now: u64) -> Result<Committed<()>, Error> {
        self.manage(id, |job| job.resume(now).map_err(job_error))
    }

    pub(crate) fn reschedule(
        &mut self,
        id: u64,
        timing: Timing,
        now: u64,
    ) -> Result<Committed<()>, Error> {
        self.manage(id, |job| {
            job.reschedule(schedule(timing)?, now).map_err(job_error)
        })
    }

    fn manage(
        &mut self,
        id: u64,
        operation: impl FnOnce(&mut Job) -> Result<(), Error>,
    ) -> Result<Committed<()>, Error> {
        let mut snapshot = self.load()?;
        let index = snapshot.index(id)?;
        operation(&mut snapshot.jobs[index].job)?;
        self.commit(snapshot, ())
    }

    pub(crate) fn summary(&self) -> Result<Committed<QueueTotals>, Error> {
        let snapshot = self.load()?;
        Ok(Committed {
            value: QueueTotals {
                counter: snapshot.counter,
                receipts: snapshot.receipts.len() as u64,
            },
            next_due_ns: snapshot.next_due_ns(),
        })
    }

    // No await: the complete bounded batch commits local effects and outstanding
    // external intent before any caller is given a delivery envelope.
    pub(crate) fn dispatch(&mut self, now: u64) -> Result<Committed<DispatchBatch>, Error> {
        let mut snapshot = self.load()?;
        let mut deliveries = vec![];
        let mut work = 0;
        let mut events = Vec::with_capacity(BATCH * 2);
        for _ in 0..BATCH {
            let Some((index, attempt)) =
                start_next(snapshot.jobs.iter_mut().map(|entry| &mut entry.job), now)
                    .map_err(job_error)?
            else {
                break;
            };
            work += 1;
            let retry = snapshot.jobs[index].job.record().attempts > 1;
            events.push(Event::Dispatch { attempt, retry });
            match &snapshot.jobs[index].task {
                Task::Increment(amount) => {
                    // Known local failure has no effect; dispose of this job
                    // without rolling back unrelated work in the same batch.
                    let outcome = if let Some(counter) = snapshot.counter.checked_add(*amount) {
                        snapshot.counter = counter;
                        Outcome::Success
                    } else {
                        Outcome::PermanentFailure
                    };
                    snapshot.jobs[index]
                        .job
                        .finish(attempt, now, outcome)
                        .map_err(job_error)?;
                    events.push(Event::Outcome {
                        attempt,
                        outcome,
                        reconciled: false,
                    });
                }
                Task::Deliver(_) => deliveries.push(snapshot.jobs[index].delivery()?),
            }
        }
        if work > 0 {
            // Commit the complete batch before delivery envelopes escape.
            let committed = self.commit(snapshot, DispatchBatch { deliveries, work })?;
            for event in events {
                self.observations.record(now, event);
            }
            Ok(committed)
        } else {
            // Idle batches project existing durable state without writing it.
            Ok(Committed {
                value: DispatchBatch { deliveries, work },
                next_due_ns: snapshot.next_due_ns(),
            })
        }
    }

    pub(crate) fn outstanding(&self, id: u64) -> Result<Delivery, Error> {
        self.load()?.take_entry(id)?.delivery()
    }

    pub(crate) fn finish(
        &mut self,
        delivery: &Delivery,
        now: u64,
        outcome: Outcome,
    ) -> Result<Committed<()>, Error> {
        let mut snapshot = self.load()?;
        let index = snapshot.index(delivery.job)?;
        let entry = &snapshot.jobs[index];
        let (attempt, payload) = entry.delivery_parts()?;
        // Exhaustive projection keeps every envelope field in the comparison
        // without allocating an owned copy of the expected payload.
        let Delivery {
            job,
            occurrence,
            sequence,
            payload: supplied_payload,
        } = delivery;
        if (*job, *occurrence, *sequence, supplied_payload)
            != (
                attempt.execution.job.0 as u64,
                attempt.execution.occurrence,
                attempt.sequence,
                payload,
            )
        {
            return Err(Error::ReceiptMismatch);
        }
        let job = &mut snapshot.jobs[index].job;
        let reconciled = matches!(job.record().state, JobState::Uncertain { .. });
        if reconciled {
            job.resolve(attempt, now, outcome).map_err(job_error)?;
        } else {
            job.finish(attempt, now, outcome).map_err(job_error)?;
        }
        let committed = self.commit(snapshot, ())?;
        self.observations.record(
            now,
            Event::Outcome {
                attempt,
                outcome,
                reconciled,
            },
        );
        Ok(committed)
    }

    fn authorize_delivery_sender(&self, caller: Principal) -> Result<(), Error> {
        if self.init.delivery_sender != Some(caller) {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }

    pub(crate) fn receive(
        &mut self,
        caller: Principal,
        delivery: Delivery,
    ) -> Result<Delivery, Error> {
        self.authorize_delivery_sender(caller)?;
        if !delivery_valid(&delivery) {
            return Err(Error::InvalidInput);
        }
        let mut snapshot = self.load()?;
        if let Some(index) = snapshot.receipt_index(&delivery) {
            return if snapshot.receipts[index] == delivery {
                Ok(delivery)
            } else {
                Err(Error::ReceiptMismatch)
            };
        }
        if snapshot.receipts.len() == MAX_JOBS {
            return Err(Error::Capacity);
        }
        snapshot.receipts.push(delivery.clone());
        self.commit(snapshot, ())?;
        Ok(delivery)
    }

    pub(crate) fn receipt(
        &self,
        caller: Principal,
        delivery: &Delivery,
    ) -> Result<Option<Delivery>, Error> {
        self.authorize_delivery_sender(caller)?;
        let mut snapshot = self.load()?;
        Ok(snapshot
            .receipt_index(delivery)
            .map(|index| snapshot.receipts.swap_remove(index)))
    }

    // Deliberate corruption endpoint in an unpublished qualification artifact.
    // Validate the selected record first, then truncate the current Wire snapshot.
    // Normal readers, including post_upgrade, reject it during bounded decoding
    // before the aggregate Job::restore boundary. No retired codec is involved.
    pub(crate) fn corrupt_record(&mut self, id: u64) -> Result<(), Error> {
        let snapshot = self.load()?;
        snapshot.index(id)?;
        let mut bytes = encode(&snapshot)?;
        bytes.pop();
        self.cell.set(bytes).map_err(memory_error)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
