//! Checked configuration and the current stored outcome contract.

use std::num::NonZeroU64;

use ic_jobs::{
    Job, JobError, JobId, JobRecord, JobState, Outcome, RecordedOutcome, RetryPolicy, Schedule,
};
use serde_test::{Token, assert_tokens};

#[test]
fn retry_construction_and_decoding_share_the_same_numeric_constraints() {
    for (attempts, initial, maximum) in [
        (0, 1, 1),
        (1, 0, 1),
        (1, 0, 0),
        (1, 2, 1),
        (u32::MAX, u64::MAX, u64::MAX - 1),
    ] {
        assert_eq!(
            RetryPolicy::new(attempts, initial, maximum),
            Err(JobError::InvalidRetryPolicy)
        );
        let encoded = serde_json::json!({
            "max_attempts": attempts, "initial_delay_ns": initial, "max_delay_ns": maximum
        });
        assert!(serde_json::from_value::<RetryPolicy>(encoded).is_err());
    }
    for (attempts, initial, maximum) in [(1, 1, 1), (3, 5, 10), (u32::MAX, u64::MAX, u64::MAX)] {
        let policy = RetryPolicy::new(attempts, initial, maximum).unwrap();
        assert_eq!(policy.max_attempts(), attempts);
        assert_eq!(policy.initial_delay_ns(), initial);
        assert_eq!(policy.max_delay_ns(), maximum);
        let bytes = serde_json::to_vec(&policy).unwrap();
        assert_eq!(
            serde_json::from_slice::<RetryPolicy>(&bytes).unwrap(),
            policy
        );
        // The public writer's named structure is also the reader's contract.
        // JSON ignores this metadata; the canonical token codec checks it.
        assert_tokens(
            &policy,
            &[
                Token::Struct {
                    name: "RetryPolicy",
                    len: 3,
                },
                Token::Str("max_attempts"),
                Token::U32(attempts),
                Token::Str("initial_delay_ns"),
                Token::U64(initial),
                Token::Str("max_delay_ns"),
                Token::U64(maximum),
                Token::StructEnd,
            ],
        );
    }
    assert_eq!(RetryPolicy::new(1, 1, 1).unwrap(), RetryPolicy::NONE);
}

#[test]
fn configuration_decoding_rejects_invalid_numbers_before_aggregate_restore() {
    let valid = Job::new(JobId(1), Schedule::Once { at_ns: 0 }, RetryPolicy::NONE).record();
    for retry in [
        serde_json::json!({"max_attempts": 0, "initial_delay_ns": 1, "max_delay_ns": 1}),
        serde_json::json!({"max_attempts": 1, "initial_delay_ns": 0, "max_delay_ns": 1}),
        serde_json::json!({"max_attempts": 1, "initial_delay_ns": 2, "max_delay_ns": 1}),
    ] {
        let mut encoded = serde_json::to_value(valid).unwrap();
        encoded["retry"] = retry;
        assert!(serde_json::from_value::<JobRecord>(encoded).is_err());
    }
    for schedule in [
        serde_json::json!({"AfterCompletion": {"first_at_ns": 0, "every_ns": 0}}),
        serde_json::json!({"FixedRate": {"first_at_ns": 0, "every_ns": 0, "missed": "Skip"}}),
        serde_json::json!({"AfterCompletion": {"first_at_ns": 0, "every_ns": -1}}),
    ] {
        let mut encoded = serde_json::to_value(valid).unwrap();
        encoded["schedule"] = schedule;
        assert!(serde_json::from_value::<JobRecord>(encoded).is_err());
    }
    // Checked local configuration does not replace aggregate history validation.
    let mut corrupt = valid;
    corrupt.state = JobState::Completed;
    let decoded = serde_json::from_slice(&serde_json::to_vec(&corrupt).unwrap()).unwrap();
    assert_eq!(Job::restore(decoded).unwrap_err(), JobError::InvalidRecord);
}

#[test]
fn running_uses_transition_time_and_uncertainty_retains_original_dispatch() {
    let mut job = Job::new(JobId(1), Schedule::Once { at_ns: 100 }, RetryPolicy::NONE);
    let attempt = job.start(110).unwrap();
    let encoded = serde_json::to_value(&job).unwrap();
    assert_eq!(encoded["state"], "Running");
    assert_eq!(encoded["updated_ns"], 110);
    let mut invalid = job.record();
    invalid.updated_ns = 99;
    assert_eq!(Job::restore(invalid).unwrap_err(), JobError::InvalidRecord);

    job = Job::restore(serde_json::from_value(encoded).unwrap()).unwrap();
    assert_eq!(job.next_due_ns(), None);
    assert_eq!(job.start(120), Err(JobError::NotPending));
    job.finish(attempt, 150, Outcome::Uncertain).unwrap();
    assert_eq!(job.record().state, JobState::Uncertain { started_ns: 110 });
    assert_eq!(job.record().updated_ns, 150);
    job = Job::restore(job.record()).unwrap();
    job.resolve(attempt, 200, Outcome::Uncertain).unwrap();
    assert_eq!(job.record().state, JobState::Uncertain { started_ns: 110 });
    assert_eq!(job.record().updated_ns, 200);
    assert_eq!(job.next_due_ns(), None);
    job.resolve(attempt, 201, Outcome::Success).unwrap();
    assert_eq!(job.record().state, JobState::Completed);

    assert!(
        serde_json::from_slice::<JobRecord>(include_bytes!("../fixtures/running-record-015.json"))
            .is_err()
    );
}

#[test]
fn recorded_outcome_json_roundtrips_without_unblocking_an_effect() {
    // Hand-authored current-format bytes, independent of the writer under test.
    let bytes = include_str!("../fixtures/uncertain-record.json").trim_end();
    let record: JobRecord = serde_json::from_str(bytes).unwrap();
    assert_eq!(
        record.last_outcome,
        Some(RecordedOutcome {
            outcome: Outcome::Uncertain,
            at_ns: 101
        })
    );
    let mut job = Job::restore(record).unwrap();
    assert_eq!(serde_json::to_string(&job.record()).unwrap(), bytes);
    assert_eq!(serde_json::to_string(&job).unwrap(), bytes);
    assert_eq!(job.next_due_ns(), None);
    assert_eq!(job.start(200), Err(JobError::NotPending));
    assert_eq!(job.record(), record);
    job.resolve(job.active_attempt().unwrap(), 200, Outcome::Success)
        .unwrap();
    assert_eq!(job.record().state, JobState::Completed);
    assert_eq!(
        job.record().last_outcome,
        Some(RecordedOutcome {
            outcome: Outcome::Success,
            at_ns: 200
        })
    );
    assert_eq!(Job::restore(job.record()).unwrap().record(), job.record());
}

#[test]
fn outcome_decoding_rejects_incomplete_and_retired_shapes() {
    let record = Job::new(JobId(1), Schedule::Once { at_ns: 0 }, RetryPolicy::NONE).record();
    for invalid in [
        serde_json::json!("Success"),
        serde_json::json!({}),
        serde_json::json!({"outcome": "Success"}),
        serde_json::json!({"at_ns": 0}),
        serde_json::json!({"outcome": "Success", "at_ns": -1}),
        serde_json::json!({"outcome": "Success", "at_ns": 0, "last_outcome_ns": 0}),
    ] {
        let mut encoded = serde_json::to_value(record).unwrap();
        encoded["last_outcome"] = invalid;
        assert!(serde_json::from_value::<JobRecord>(encoded).is_err());
    }
    // Both variants of the previous published shape refuse, including nulls.
    for (outcome, time) in [
        (serde_json::Value::Null, serde_json::Value::Null),
        (serde_json::json!("Success"), serde_json::json!(0)),
    ] {
        let mut encoded = serde_json::to_value(record).unwrap();
        encoded["last_outcome"] = outcome;
        encoded["last_outcome_ns"] = time;
        assert!(serde_json::from_value::<JobRecord>(encoded).is_err());
    }
}

#[test]
fn terminal_restore_requires_matching_outcome_time_and_failure_budget() {
    for outcome in [
        Outcome::Success,
        Outcome::PermanentFailure,
        Outcome::RetryableFailure,
    ] {
        let mut job = Job::new(JobId(1), Schedule::Once { at_ns: 100 }, RetryPolicy::NONE);
        let attempt = job.start(100).unwrap();
        job.finish(attempt, 101, outcome).unwrap();
        let record = job.record();
        assert_eq!(Job::restore(record).unwrap().record(), record);
        let mut wrong_time = record;
        wrong_time.updated_ns += 1;
        assert_eq!(
            Job::restore(wrong_time).unwrap_err(),
            JobError::InvalidRecord
        );
        let mut wrong_state = record;
        wrong_state.state = if outcome == Outcome::Success {
            JobState::Failed
        } else {
            JobState::Completed
        };
        assert_eq!(
            Job::restore(wrong_state).unwrap_err(),
            JobError::InvalidRecord
        );
        if outcome == Outcome::Success {
            let mut recurring_completion = record;
            recurring_completion.schedule = Schedule::AfterCompletion {
                first_at_ns: 100,
                every_ns: NonZeroU64::new(10).unwrap(),
            };
            assert_eq!(
                Job::restore(recurring_completion).unwrap_err(),
                JobError::InvalidRecord
            );
        }
    }
    let mut retry = Job::new(
        JobId(2),
        Schedule::Once { at_ns: 100 },
        RetryPolicy::new(2, 10, 10).unwrap(),
    );
    let first = retry.start(100).unwrap();
    retry.finish(first, 101, Outcome::RetryableFailure).unwrap();
    let mut premature_failure = retry.record();
    premature_failure.state = JobState::Failed;
    assert_eq!(
        Job::restore(premature_failure).unwrap_err(),
        JobError::InvalidRecord
    );
}

#[test]
fn zero_absolute_deadlines_and_maximum_intervals_remain_valid() {
    for interval in [1, u64::MAX] {
        let schedule = Schedule::AfterCompletion {
            first_at_ns: 0,
            every_ns: NonZeroU64::new(interval).unwrap(),
        };
        let mut job = Job::new(JobId(1), schedule, RetryPolicy::NONE);
        assert_eq!(job.next_due_ns(), Some(0));
        let attempt = job.start(0).unwrap();
        job.finish(attempt, 0, ic_jobs::Outcome::Success).unwrap();
        assert_eq!(job.next_due_ns(), Some(interval));
        assert_eq!(Job::restore(job.record()).unwrap().record(), job.record());
    }
}
