use super::{Result, Store};
use ic_jobs::{Job, JobId, JobRecord, MissedRunPolicy, Outcome, RetryPolicy, Schedule, start_next};
use ic_storage::MemoryRequest;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Deserialize, Serialize)]
pub(crate) struct MaintenanceSnapshot<J = JobRecord> {
    pub(crate) job: J,
    // The application owns cache entries and its local effect counter.
    pub(crate) expires_ns: Vec<u64>,
    pub(crate) completed_runs: u64,
}

pub(crate) struct Maintenance {
    pub(crate) store: Store,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Wake {
    pub(crate) completed: u64,
    pub(crate) next_due_ns: Option<u64>,
}

impl Wake {
    pub(crate) fn timer_result(&self) -> ic_timers::WatchdogRunResult {
        ic_timers::WatchdogRunResult::completed_batch(self.completed, self.next_due_ns)
    }
}

impl Maintenance {
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            store: Store::from_bytes(bytes),
        }
    }

    pub(crate) fn memory_request() -> Result<MemoryRequest> {
        Ok(MemoryRequest::new(
            "test-maintenance",
            "test.maintenance.state.v1",
        )?)
    }

    pub(crate) fn new(missed: MissedRunPolicy) -> Result<Self> {
        Ok(Self {
            store: Store::new(&MaintenanceSnapshot {
                job: Job::new(
                    JobId(2),
                    Schedule::FixedRate {
                        first_at_ns: 100,
                        every_ns: std::num::NonZeroU64::new(10).unwrap(),
                        missed,
                    },
                    RetryPolicy::NONE,
                ),
                expires_ns: vec![90, 100, 125, 200],
                completed_runs: 0,
            })?,
        })
    }

    pub(crate) fn load(&self) -> Result<MaintenanceSnapshot<Job>> {
        let snapshot: MaintenanceSnapshot = self.store.read()?;
        if snapshot.expires_ns.len() > 32 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "too many entries").into());
        }
        Ok(MaintenanceSnapshot {
            job: Job::restore(snapshot.job)?,
            expires_ns: snapshot.expires_ns,
            completed_runs: snapshot.completed_runs,
        })
    }

    pub(crate) fn wake(&mut self, now_ns: u64) -> Result<Wake> {
        let mut snapshot = self.load()?;
        let mut completed = 0;
        // Exactly two runs at most, even when CatchUp leaves overdue work.
        for _ in 0..2 {
            let Some((_, attempt)) = start_next(std::slice::from_mut(&mut snapshot.job), now_ns)?
            else {
                break;
            };
            if completed == 0 {
                snapshot.expires_ns.retain(|expires| *expires > now_ns);
            }
            snapshot.completed_runs = snapshot
                .completed_runs
                .checked_add(1)
                .ok_or_else(|| io::Error::other("maintenance counter exhausted"))?;
            snapshot.job.finish(attempt, now_ns, Outcome::Success)?;
            // Local effects and metadata commit together. No external call or await.
            self.store.write(&snapshot)?;
            completed += 1;
        }
        Ok(Wake {
            completed,
            next_due_ns: snapshot.job.next_due_ns(),
        })
    }

    pub(crate) fn cancel(&mut self, now_ns: u64) -> Result<()> {
        let mut snapshot = self.load()?;
        snapshot.job.cancel(now_ns)?;
        self.store.write(&snapshot)
    }
}
