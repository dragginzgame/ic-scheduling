//! Retained timing must account for positive backoff, even after evidence changes.
use ic_jobs::{
    Job, JobError, JobId, JobState, MissedRunPolicy, Outcome, RecordedOutcome, RetryPolicy,
    Schedule,
};
use std::num::NonZeroU64;

#[test]
fn uncertain_and_rescheduled_retry_histories_preserve_elapsed_time() {
    for delay in [1, 5, u64::MAX] {
        let mut job = Job::new(
            JobId(1),
            Schedule::FixedRate {
                first_at_ns: 0,
                every_ns: NonZeroU64::new(1).unwrap(),
                missed: MissedRunPolicy::CatchUp,
            },
            RetryPolicy::new(2, delay, delay).unwrap(),
        );
        let first = job.start(0).unwrap();
        job.finish(first, 0, Outcome::RetryableFailure).unwrap();
        let before = job.record();
        assert_eq!(job.start(delay - 1), Err(JobError::NotDue));
        assert_eq!(job.record(), before);
        let second = job.start(delay).unwrap();
        job.finish(second, delay, Outcome::Uncertain).unwrap();
        assert!(Job::restore(job.record()).is_ok());
        let mut impossible = job.record();
        impossible.state = JobState::Uncertain {
            started_ns: delay - 1,
        };
        assert_eq!(
            Job::restore(impossible).unwrap_err(),
            JobError::InvalidRecord
        );
        job.resolve(second, delay, Outcome::Success).unwrap();
        job.reschedule(Schedule::Once { at_ns: 0 }, delay).unwrap();
        assert!(Job::restore(job.record()).is_ok());
        let mut impossible = job.record();
        impossible.updated_ns = delay - 1;
        impossible.last_outcome.as_mut().unwrap().at_ns = delay - 1;
        assert_eq!(
            Job::restore(impossible).unwrap_err(),
            JobError::InvalidRecord
        );
    }
}

#[test]
fn retry_time_bound_uses_wide_arithmetic_for_untrusted_counts() {
    for initial in [1, u64::MAX] {
        let mut record = Job::new(
            JobId(2),
            Schedule::Once { at_ns: 0 },
            RetryPolicy::new(u32::MAX, initial, u64::MAX).unwrap(),
        )
        .record();
        record.sequence = u64::from(u32::MAX);
        record.attempts = u32::MAX;
        record.state = JobState::Uncertain {
            started_ns: u64::MAX,
        };
        record.updated_ns = u64::MAX;
        record.last_outcome = Some(RecordedOutcome {
            outcome: Outcome::Uncertain,
            at_ns: u64::MAX,
        });
        assert_eq!(Job::restore(record).unwrap_err(), JobError::InvalidRecord);
    }
}

#[test]
fn uncertain_retry_retains_exponential_and_capped_delay_floor() {
    for cap in [2, u64::MAX] {
        let mut job = Job::new(
            JobId(3),
            Schedule::Once { at_ns: 0 },
            RetryPolicy::new(4, 1, cap).unwrap(),
        );
        let mut time = 0;
        for delay in [1, 2, cap.min(4)] {
            let attempt = job.start(time).unwrap();
            job.finish(attempt, time, Outcome::RetryableFailure)
                .unwrap();
            time += delay;
            assert!(Job::restore(job.record()).is_ok());
        }
        let attempt = job.start(time).unwrap();
        job.finish(attempt, time, Outcome::Uncertain).unwrap();
        assert!(Job::restore(job.record()).is_ok());
        let mut impossible = job.record();
        impossible.state = JobState::Uncertain {
            started_ns: time - 1,
        };
        assert_eq!(
            Job::restore(impossible).unwrap_err(),
            JobError::InvalidRecord
        );
    }
}

#[test]
fn late_dispatch_cannot_hide_impossible_prior_retry_completion() {
    let mut job = Job::new(
        JobId(4),
        Schedule::FixedRate {
            first_at_ns: 0,
            every_ns: NonZeroU64::new(1).unwrap(),
            missed: MissedRunPolicy::CatchUp,
        },
        RetryPolicy::new(2, 5, 5).unwrap(),
    );
    let first = job.start(0).unwrap();
    job.finish(first, 0, Outcome::RetryableFailure).unwrap();
    let second = job.start(5).unwrap();
    job.finish(second, 5, Outcome::Success).unwrap();
    job.start(10).unwrap();
    assert!(Job::restore(job.record()).is_ok());
    let mut impossible = job.record();
    impossible.last_outcome.as_mut().unwrap().at_ns = 4;
    assert_eq!(
        Job::restore(impossible).unwrap_err(),
        JobError::InvalidRecord
    );
}
