use super::snapshot::{FORMAT, LIMITS, MAX_MESSAGE};
use super::*;
use crate::api::{DeliveryPayload, Status};
use ic_storage::{VectorMemory, testing::MeteredMemory};
use ic_wire::{Limits, native};

fn retained_cell(backing: VectorMemory, maximum: usize) -> StableCell<Vec<u8>, VectorMemory> {
    StableCell::load(queue_memory(backing).unwrap(), maximum).unwrap()
}

fn manager() -> Principal {
    Principal::self_authenticating(b"manager")
}
fn config() -> Init {
    Init {
        manager: manager(),
        delivery_sender: None,
    }
}
fn increment(at: u64) -> Create {
    Create {
        timing: Timing::Once(at),
        task: Task::Increment(1),
    }
}

// Inspect raw records for corruption fixtures without skipping production restore.
fn stored_records(bytes: &[u8]) -> Result<Snapshot, ic_wire::Error> {
    native::decode(
        bytes.strip_prefix(FORMAT).ok_or(ic_wire::Error::Identity)?,
        LIMITS,
    )
}

// Deliberately bypass Job::restore only while producing a corrupt current
// snapshot. Assertions still exercise normal production readers and refusals.
fn mutate_records(queue: &mut Queue<VectorMemory>, mutate: impl FnOnce(&mut Snapshot)) {
    let mut snapshot = stored_records(queue.cell.get().as_slice()).unwrap();
    mutate(&mut snapshot);
    queue.cell.set(encode(&snapshot).unwrap()).unwrap();
}

// These fixtures start with a fresh Once job. Build only valid states through
// the public transitions; malformed-record fixtures remain independent.
fn set_job_state(job: &mut Job, state: JobState, at_ns: u64) {
    match state {
        JobState::Pending { .. } => {}
        JobState::Paused { .. } => job.pause(at_ns).unwrap(),
        JobState::Cancelled => job.cancel(at_ns).unwrap(),
        _ => {
            let attempt = job.start(at_ns).unwrap();
            match state {
                JobState::Running => {}
                JobState::Uncertain { .. } => {
                    job.finish(attempt, at_ns, Outcome::Uncertain).unwrap()
                }
                JobState::Completed => job.finish(attempt, at_ns, Outcome::Success).unwrap(),
                JobState::Failed => job
                    .finish(attempt, at_ns, Outcome::PermanentFailure)
                    .unwrap(),
                _ => unreachable!(),
            }
        }
    }
    assert_eq!(job.record().state, state);
}

mod admission;
mod dispatch;
mod management;
mod observability;
mod persistence;
mod recovery;
