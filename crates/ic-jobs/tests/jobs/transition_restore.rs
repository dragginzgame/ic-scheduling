//! Producer/restore agreement across bounded histories, independent of storage.

use std::{
    collections::{BTreeSet, HashSet},
    mem::discriminant,
};

use ic_jobs::{
    Attempt, ExecutionId, Job, JobError, JobId, JobRecord, JobState, Outcome, RetryPolicy, Schedule,
};

use super::support::recurring_schedules;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Start,
    Finish(Outcome),
    Resolve(Outcome),
    Pause,
    Resume,
    Reschedule(Schedule),
    Cancel,
}

impl Operation {
    fn apply(self, job: &mut Job, now: u64) -> Result<(), JobError> {
        let attempt = job.active_attempt().unwrap_or(Attempt {
            execution: ExecutionId {
                job: JobId(0),
                occurrence: 0,
            },
            sequence: 0,
        });
        match self {
            Self::Start => job.start(now).map(|_| ()),
            Self::Finish(outcome) => job.finish(attempt, now, outcome),
            Self::Resolve(outcome) => job.resolve(attempt, now, outcome),
            Self::Pause => job.pause(now),
            Self::Resume => job.resume(now),
            Self::Reschedule(schedule) => job.reschedule(schedule, now),
            Self::Cancel => job.cancel(now),
        }
    }
}

#[test]
fn public_transition_histories_remain_restorable_and_failures_preserve_records() {
    let [after_completion, skip, catch_up] = recurring_schedules(100, 10);
    let schedules = [
        Schedule::Once { at_ns: 100 },
        after_completion,
        catch_up,
        skip,
    ];
    let mut operations = vec![
        Operation::Start,
        Operation::Pause,
        Operation::Resume,
        Operation::Cancel,
        Operation::Reschedule(Schedule::Once { at_ns: 0 }),
    ];
    operations.extend(schedules.map(Operation::Reschedule));
    for outcome in [
        Outcome::Success,
        Outcome::RetryableFailure,
        Outcome::PermanentFailure,
        Outcome::Uncertain,
    ] {
        operations.extend([Operation::Finish(outcome), Operation::Resolve(outcome)]);
    }

    let mut frontier = Vec::new();
    let mut visited = BTreeSet::new();
    let mut states = HashSet::new();
    for schedule in schedules {
        for retry in [RetryPolicy::NONE, RetryPolicy::new(3, 5, 10).unwrap()] {
            let record = Job::new(JobId(1), schedule, retry).record();
            states.insert(discriminant(&record.state));
            visited.insert(serde_json::to_vec(&record).unwrap());
            frontier.push(record);
        }
    }

    // Include retry, uncertainty resolution and management after recurrence.
    // Deduplicate records rather than repeating equivalent operation histories.
    for _ in 0..5 {
        let mut next = Vec::new();
        for record in frontier {
            let mut times = vec![0, 100, 110, u64::MAX, record.updated_ns];
            if let Some(due) = Job::restore(record).unwrap().next_due_ns() {
                times.push(due);
                times.extend(due.checked_sub(1));
            }
            times.sort_unstable();
            times.dedup();
            for now in times {
                for &operation in &operations {
                    let mut job = Job::restore(record).unwrap();
                    if operation.apply(&mut job, now).is_err() {
                        assert_eq!(job.record(), record, "{operation:?} at {now}");
                        continue;
                    }
                    let bytes = serde_json::to_vec(&job).unwrap();
                    // Equivalent retained records have the same codec/restore
                    // contract; keep every transition and refusal assertion above.
                    if visited.contains(&bytes) {
                        continue;
                    }
                    assert_eq!(bytes, serde_json::to_vec(&job.record()).unwrap());
                    let decoded: JobRecord = serde_json::from_slice(&bytes).unwrap();
                    let restored = Job::restore(decoded).unwrap_or_else(|error| {
                        panic!("{operation:?} at {now}: {error}; {decoded:?}")
                    });
                    assert_eq!(restored.record(), job.record());
                    assert_eq!(restored.next_due_ns(), job.next_due_ns());
                    assert_eq!(restored.active_attempt(), job.active_attempt());
                    states.insert(discriminant(&restored.record().state));
                    visited.insert(bytes);
                    next.push(restored.record());
                }
            }
        }
        frontier = next;
    }
    for state in [
        JobState::Pending { due_ns: 0 },
        JobState::Paused { due_ns: 0 },
        JobState::Running,
        JobState::Uncertain { started_ns: 0 },
        JobState::Completed,
        JobState::Failed,
        JobState::Cancelled,
    ] {
        assert!(
            states.contains(&discriminant(&state)),
            "not covered: {state:?}"
        );
    }
}
