//! Public record coverage, alongside the canister's bounded Wire queue tests.
//!
//! Canic's owned stable records use Wire. Exercise the public Jobs values with
//! that codec here, without adding Wire or a storage contract to the core crate.
use std::num::NonZeroU64;

use ic_jobs::{
    Job, JobError, JobId, JobRecord, JobState, MissedRunPolicy, Outcome, RetryPolicy, Schedule,
};
use ic_wire::{Limits, native};

// Small fixture policy, not Canic's production storage budget or framing.
const LIMITS: Limits = Limits {
    bytes: 4096,
    allocation: 32 * 1024,
    nodes: 128,
    depth: 16,
};

fn roundtrip(job: &Job) {
    let bytes = native::encode(job, LIMITS).unwrap();
    assert_eq!(bytes, native::encode(&job.record(), LIMITS).unwrap());
    let record: JobRecord = native::decode(&bytes, LIMITS).unwrap();
    assert_eq!(record, job.record());
    let mut restored = Job::restore(record).unwrap();
    assert_eq!(restored.record(), job.record());
    assert_eq!(restored.next_due_ns(), job.next_due_ns());
    assert_eq!(restored.active_attempt(), job.active_attempt());
    if let Some(attempt) = restored.active_attempt() {
        let encoded = native::encode(&attempt, LIMITS).unwrap();
        assert_eq!(native::decode(&encoded, LIMITS), Ok(attempt));
        assert_eq!(restored.next_due_ns(), None);
        assert_eq!(restored.start(u64::MAX), Err(JobError::NotPending));
        assert_eq!(restored.record(), record);
    }
}

#[test]
fn wire_records_preserve_all_states_schedules_and_wide_identities() {
    let interval = NonZeroU64::new(10).unwrap();
    for id in [0, 1_u128 << 100, u128::MAX] {
        for schedule in [
            Schedule::Once { at_ns: 100 },
            Schedule::AfterCompletion {
                first_at_ns: 100,
                every_ns: interval,
            },
            Schedule::FixedRate {
                first_at_ns: 100,
                every_ns: interval,
                missed: MissedRunPolicy::Skip,
            },
            Schedule::FixedRate {
                first_at_ns: 100,
                every_ns: interval,
                missed: MissedRunPolicy::CatchUp,
            },
        ] {
            let retry = RetryPolicy::new(2, 5, 20).unwrap();
            let mut job = Job::new(JobId(id), schedule, retry);
            roundtrip(&job);
            job.pause(50).unwrap();
            roundtrip(&job);
            job.resume(60).unwrap();
            let first = job.start(100).unwrap();
            roundtrip(&job);
            job.finish(first, 101, Outcome::Uncertain).unwrap();
            roundtrip(&job);
            job.resolve(first, 102, Outcome::RetryableFailure).unwrap();
            roundtrip(&job);
            let second = job.start(107).unwrap();
            assert_eq!(second.execution, first.execution);
            assert_ne!(second.sequence, first.sequence);
            roundtrip(&job);
            job.finish(second, 108, Outcome::Success).unwrap();
            roundtrip(&job);
            if !matches!(schedule, Schedule::Once { .. }) {
                job.reschedule(Schedule::Once { at_ns: 200 }, 109).unwrap();
                assert_eq!(job.record().schedule_origin, 1);
                roundtrip(&job);
            }

            let mut cancelled = Job::new(JobId(id), schedule, retry);
            cancelled.cancel(1).unwrap();
            roundtrip(&cancelled);

            let mut failed = Job::new(JobId(id), schedule, retry);
            let attempt = failed.start(100).unwrap();
            failed
                .finish(attempt, 101, Outcome::RetryableFailure)
                .unwrap();
            let attempt = failed.start(106).unwrap();
            failed
                .finish(attempt, 107, Outcome::RetryableFailure)
                .unwrap();
            assert_eq!(failed.record().state, JobState::Failed);
            roundtrip(&failed);
        }
    }
}

#[test]
fn wire_decoding_does_not_admit_invalid_job_history() {
    let mut job = Job::new(
        JobId(u128::MAX),
        Schedule::Once { at_ns: 100 },
        RetryPolicy::NONE,
    );
    job.start(100).unwrap();
    let valid = job.record();
    let mut no_attempt = valid;
    no_attempt.attempts = 0;
    let mut no_sequence = valid;
    no_sequence.sequence = 0;
    let mut before_due = valid;
    before_due.updated_ns = 99;
    let mut false_completion = valid;
    false_completion.state = JobState::Completed;
    for corrupt in [no_attempt, no_sequence, before_due, false_completion] {
        let bytes = native::encode(&corrupt, LIMITS).unwrap();
        let decoded: JobRecord = native::decode(&bytes, LIMITS).unwrap();
        assert_eq!(decoded, corrupt);
        assert_eq!(Job::restore(decoded).unwrap_err(), JobError::InvalidRecord);
    }
}

#[test]
fn stored_wire_refuses_truncation_trailing_bytes_and_byte_budget_overflow() {
    let job = Job::new(
        JobId(u128::MAX),
        Schedule::Once { at_ns: 100 },
        RetryPolicy::NONE,
    );
    let bytes = native::encode(&job, LIMITS).unwrap();
    assert!(native::decode::<JobRecord>(&bytes[..bytes.len() - 1], LIMITS).is_err());
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert_eq!(
        native::decode::<JobRecord>(&trailing, LIMITS),
        Err(ic_wire::Error::Trailing)
    );
    let smaller = Limits {
        bytes: bytes.len() - 1,
        ..LIMITS
    };
    assert_eq!(
        native::decode::<JobRecord>(&bytes, smaller),
        Err(ic_wire::Error::Limit)
    );
    assert_eq!(native::encode(&job, smaller), Err(ic_wire::Error::Limit));
}
