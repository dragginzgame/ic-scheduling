//! Durable management keeps effect identity and independent timing evidence.
use ic_jobs::{
    Job, JobError, JobId, JobRecord, JobState, MissedRunPolicy, Outcome, RecordedOutcome,
    RetryPolicy, Schedule, next_due_ns, start_next,
};

use super::support::{recurring_schedules, roundtrip};

fn recurring(missed: MissedRunPolicy) -> Job {
    Job::new(
        JobId(7),
        Schedule::FixedRate {
            first_at_ns: 100,
            every_ns: std::num::NonZeroU64::new(10).unwrap(),
            missed,
        },
        RetryPolicy::new(3, 10, 20).unwrap(),
    )
}

#[test]
fn pause_restore_resume_preserves_overdue_policy_and_scheduler_exclusion() {
    for missed in [MissedRunPolicy::Skip, MissedRunPolicy::CatchUp] {
        let mut job = recurring(missed);
        job.pause(50).unwrap();
        let mut jobs = [roundtrip(&job)];
        assert_eq!(next_due_ns(&jobs), None);
        assert_eq!(start_next(&mut jobs, 200), Ok(None));
        jobs[0].resume(200).unwrap();
        assert_eq!(roundtrip(&jobs[0]).next_due_ns(), Some(100));
        let attempt = jobs[0].start(200).unwrap();
        assert_eq!((attempt.execution.occurrence, attempt.sequence), (0, 1));
        jobs[0].finish(attempt, 200, Outcome::Success).unwrap();
        assert_eq!(
            roundtrip(&jobs[0]).next_due_ns(),
            Some(if missed == MissedRunPolicy::Skip {
                210
            } else {
                110
            })
        );
        jobs[0].pause(201).unwrap();
        jobs[0].resume(300).unwrap();
        assert_eq!(
            roundtrip(&jobs[0])
                .record()
                .last_outcome
                .map(|result| result.at_ns),
            Some(200)
        );
        jobs[0].pause(300).unwrap();
        jobs[0].cancel(301).unwrap();
        assert_eq!(roundtrip(&jobs[0]).record().state, JobState::Cancelled);
    }
}

#[test]
fn retry_pause_retains_backoff_and_refuses_rescheduling_after_an_attempt() {
    let mut job = recurring(MissedRunPolicy::CatchUp);
    let attempt = job.start(100).unwrap();
    job.finish(attempt, 100, Outcome::RetryableFailure).unwrap();
    job.pause(105).unwrap();
    job = roundtrip(&job);
    let before = job.record();
    assert_eq!(
        job.reschedule(Schedule::Once { at_ns: 0 }, 105),
        Err(JobError::AlreadyAttempted)
    );
    assert_eq!(job.record(), before);
    job.resume(106).unwrap();
    job = roundtrip(&job);
    assert_eq!(job.next_due_ns(), Some(110));
    assert_eq!(job.start(109), Err(JobError::NotDue));
    let second = job.start(110).unwrap();
    assert_eq!(second.execution, attempt.execution);
    assert_eq!(second.sequence, 2);
    job.finish(second, 110, Outcome::RetryableFailure).unwrap();
    job.pause(120).unwrap();
    job.resume(200).unwrap();
    assert_eq!(roundtrip(&job).next_due_ns(), Some(130));
    assert_eq!(
        job.record().last_outcome.map(|result| result.at_ns),
        Some(110)
    );
    let mut corrupt = job.record();
    corrupt.last_outcome.as_mut().unwrap().at_ns = 111;
    assert_eq!(Job::restore(corrupt).unwrap_err(), JobError::InvalidRecord);
}

#[test]
fn rescheduling_preserves_history_and_rebases_only_the_current_cadence() {
    for paused in [false, true] {
        for replacement in
            std::iter::once(Schedule::Once { at_ns: 250 }).chain(recurring_schedules(250, 20))
        {
            let mut job = recurring(MissedRunPolicy::CatchUp);
            let previous = job.start(100).unwrap();
            job.finish(previous, 100, Outcome::Success).unwrap();
            if paused {
                job.pause(150).unwrap();
            }
            job.reschedule(replacement, 200).unwrap();
            job = roundtrip(&job);
            let record = job.record();
            assert_eq!(
                (
                    record.id,
                    record.occurrence,
                    record.schedule_origin,
                    record.sequence,
                    record.attempts
                ),
                (JobId(7), 1, 1, 1, 0)
            );
            assert_eq!(record.last_outcome.map(|result| result.at_ns), Some(100));
            assert_eq!(job.next_due_ns(), if paused { None } else { Some(250) });
            if paused {
                job.resume(210).unwrap();
            }
            assert_eq!(job.start(249), Err(JobError::NotDue));
            let next = job.start(300).unwrap();
            assert_eq!((next.execution.occurrence, next.sequence), (1, 2));
            assert_eq!(
                job.finish(previous, 300, Outcome::Success),
                Err(JobError::StaleAttempt)
            );
            job.finish(next, 300, Outcome::Success).unwrap();
            job = roundtrip(&job);
            match replacement {
                Schedule::Once { .. } => assert_eq!(job.record().state, JobState::Completed),
                Schedule::FixedRate {
                    missed: MissedRunPolicy::CatchUp,
                    ..
                } => assert_eq!(job.next_due_ns(), Some(270)),
                Schedule::FixedRate { .. } => assert_eq!(job.next_due_ns(), Some(310)),
                Schedule::AfterCompletion { .. } => assert_eq!(job.next_due_ns(), Some(320)),
            }
        }
    }
}

#[test]
fn fresh_rescheduling_retains_identity_and_overflow_keeps_the_active_attempt() {
    for paused in [false, true] {
        for replacement in
            std::iter::once(Schedule::Once { at_ns: 250 }).chain(recurring_schedules(250, 20))
        {
            let mut job = Job::new(JobId(7), Schedule::Once { at_ns: 100 }, RetryPolicy::NONE);
            if paused {
                job.pause(50).unwrap();
            }
            job.reschedule(replacement, 200).unwrap();
            job = roundtrip(&job);
            assert_eq!(job.record().sequence, 0);
            assert_eq!(job.record().occurrence, 0);
            assert_eq!(job.record().last_outcome, None);
            if paused {
                assert_eq!(job.next_due_ns(), None);
                job.resume(200).unwrap();
            }
            assert_eq!(job.next_due_ns(), Some(250));
            let attempt = job.start(250).unwrap();
            assert_eq!(
                (
                    attempt.execution.job,
                    attempt.execution.occurrence,
                    attempt.sequence
                ),
                (JobId(7), 0, 1)
            );
            job.finish(attempt, 250, Outcome::Success).unwrap();
            assert_eq!(roundtrip(&job).record(), job.record());
        }
    }
    let mut job = recurring(MissedRunPolicy::CatchUp);
    job.reschedule(
        Schedule::AfterCompletion {
            first_at_ns: u64::MAX,
            every_ns: std::num::NonZeroU64::new(1).unwrap(),
        },
        0,
    )
    .unwrap();
    job = roundtrip(&job);
    let attempt = job.start(u64::MAX).unwrap();
    let before = job.record();
    assert_eq!(
        job.finish(attempt, u64::MAX, Outcome::Success),
        Err(JobError::DeadlineOverflow)
    );
    assert_eq!(job.record(), before);
    assert_eq!(roundtrip(&job).record(), before);
}

#[test]
fn management_refuses_blocked_work_invalid_policies_and_backward_clocks_without_mutation() {
    let mut job = recurring(MissedRunPolicy::Skip);
    job.pause(50).unwrap();
    let before = job.record();
    assert_eq!(job.resume(49), Err(JobError::TimeWentBackwards));
    assert_eq!(job.record(), before);
    job.resume(50).unwrap();
    let attempt = job.start(100).unwrap();
    for uncertain in [false, true] {
        if uncertain {
            job.finish(attempt, 101, Outcome::Uncertain).unwrap();
        }
        let before = job.record();
        assert_eq!(job.pause(102), Err(JobError::InFlight));
        assert_eq!(job.resume(102), Err(JobError::InFlight));
        assert_eq!(
            job.reschedule(Schedule::Once { at_ns: 0 }, 102),
            Err(JobError::InFlight)
        );
        assert_eq!(job.cancel(102), Err(JobError::InFlight));
        assert_eq!(job.record(), before);
        assert_eq!(roundtrip(&job).record(), before);
    }
    job.resolve(attempt, 103, Outcome::PermanentFailure)
        .unwrap();
    let before = job.record();
    assert_eq!(job.pause(104), Err(JobError::NotPending));
    assert_eq!(job.resume(104), Err(JobError::NotPaused));
    assert_eq!(
        job.reschedule(Schedule::Once { at_ns: 200 }, 104),
        Err(JobError::NotPending)
    );
    assert_eq!(job.record(), before);
}

#[test]
fn restore_rejects_missing_management_fields_and_inconsistent_anchors() {
    let job = recurring(MissedRunPolicy::Skip);
    let mut encoded = serde_json::to_value(job.record()).unwrap();
    encoded.as_object_mut().unwrap().remove("schedule_origin");
    assert!(serde_json::from_value::<JobRecord>(encoded).is_err());
    let mut invalid = job.record();
    invalid.schedule_origin = 1;
    assert_eq!(Job::restore(invalid).unwrap_err(), JobError::InvalidRecord);
    invalid = job.record();
    invalid.last_outcome = Some(RecordedOutcome {
        outcome: Outcome::Success,
        at_ns: 0,
    });
    assert_eq!(Job::restore(invalid).unwrap_err(), JobError::InvalidRecord);
    let mut once = Job::new(
        JobId(1),
        Schedule::Once { at_ns: u64::MAX },
        RetryPolicy::NONE,
    );
    once.pause(u64::MAX).unwrap();
    once = roundtrip(&once);
    once.resume(u64::MAX).unwrap();
    let attempt = once.start(u64::MAX).unwrap();
    once.finish(attempt, u64::MAX, Outcome::Success).unwrap();
    assert_eq!(roundtrip(&once).record().state, JobState::Completed);
}

#[test]
fn restore_rejects_running_retry_before_retained_backoff() {
    let mut job = recurring(MissedRunPolicy::CatchUp);
    let first = job.start(100).unwrap();
    job.finish(first, 100, Outcome::RetryableFailure).unwrap();
    job.pause(101).unwrap();
    job.resume(102).unwrap();
    assert_eq!(job.start(109), Err(JobError::NotDue));
    let second = job.start(110).unwrap();
    assert_eq!(roundtrip(&job).record(), job.record());
    for (outcome_ns, started_ns) in [(100, 109), (99, 110), (u64::MAX, u64::MAX)] {
        let mut corrupt = job.record();
        corrupt.last_outcome.as_mut().unwrap().at_ns = outcome_ns;
        corrupt.state = JobState::Running;
        corrupt.updated_ns = started_ns;
        assert_eq!(Job::restore(corrupt).unwrap_err(), JobError::InvalidRecord);
    }
    job.finish(second, 110, Outcome::Uncertain).unwrap();
    let mut blocked = roundtrip(&job);
    assert_eq!(blocked.next_due_ns(), None);
    assert_eq!(blocked.start(200), Err(JobError::NotPending));
    blocked
        .resolve(second, 200, Outcome::RetryableFailure)
        .unwrap();
    assert_eq!(roundtrip(&blocked).next_due_ns(), Some(220));
    // A representable retry deadline at the maximum timestamp remains valid.
    let mut edge = Job::new(
        JobId(9),
        Schedule::Once { at_ns: 0 },
        RetryPolicy::new(2, 1, 1).unwrap(),
    );
    let first = edge.start(0).unwrap();
    edge.finish(first, u64::MAX - 1, Outcome::RetryableFailure)
        .unwrap();
    let mut cancelled = roundtrip(&edge);
    cancelled.cancel(u64::MAX - 1).unwrap();
    assert_eq!(roundtrip(&cancelled).record(), cancelled.record());
    edge.start(u64::MAX).unwrap();
    assert_eq!(roundtrip(&edge).record(), edge.record());
}

#[test]
fn retained_retry_evidence_is_consistent_across_management_states() {
    let mut job = Job::new(
        JobId(9),
        Schedule::Once { at_ns: 100 },
        RetryPolicy::new(3, 10, 20).unwrap(),
    );
    let attempt = job.start(100).unwrap();
    job.finish(attempt, 101, Outcome::RetryableFailure).unwrap();
    for state in [
        JobState::Pending { due_ns: 111 },
        JobState::Paused { due_ns: 111 },
        JobState::Cancelled,
    ] {
        let mut valid = job.record();
        valid.state = state;
        // Management time may differ from the retained result and precede due.
        valid.updated_ns = 110;
        assert_eq!(roundtrip(&Job::restore(valid).unwrap()).record(), valid);
        for case in 0..5 {
            let mut corrupt = valid;
            match case {
                0 => corrupt.last_outcome = None,
                1 => corrupt.last_outcome.as_mut().unwrap().outcome = Outcome::Success,
                2 => corrupt.last_outcome.as_mut().unwrap().at_ns = 99,
                3 => corrupt.retry = RetryPolicy::NONE,
                4 => {
                    corrupt.last_outcome.as_mut().unwrap().at_ns = u64::MAX;
                    corrupt.updated_ns = u64::MAX;
                }
                _ => unreachable!(),
            }
            assert_eq!(
                Job::restore(corrupt).unwrap_err(),
                JobError::InvalidRecord,
                "{state:?}, case {case}"
            );
        }
        if let JobState::Pending { .. } | JobState::Paused { .. } = state {
            let mut wrong_deadline = valid;
            wrong_deadline.state = match state {
                JobState::Pending { .. } => JobState::Pending { due_ns: 112 },
                _ => JobState::Paused { due_ns: 112 },
            };
            assert_eq!(
                Job::restore(wrong_deadline).unwrap_err(),
                JobError::InvalidRecord
            );
        }
    }
}

#[test]
fn restore_rejects_cancelled_retry_with_impossible_completion() {
    for paused in [false, true] {
        let mut job = recurring(MissedRunPolicy::CatchUp);
        let attempt = job.start(100).unwrap();
        job.finish(attempt, 100, Outcome::RetryableFailure).unwrap();
        if paused {
            job.pause(101).unwrap();
        }
        // Cancellation before the retry deadline is valid.
        job.cancel(105).unwrap();
        assert_eq!(roundtrip(&job).record(), job.record());
        for outcome_ns in [99, u64::MAX] {
            let mut corrupt = job.record();
            corrupt.last_outcome.as_mut().unwrap().at_ns = outcome_ns;
            corrupt.updated_ns = u64::MAX;
            assert_eq!(Job::restore(corrupt).unwrap_err(), JobError::InvalidRecord);
        }
    }
}

#[test]
fn restore_checks_retained_successor_after_cancellation_or_dispatch() {
    for schedule in [
        Schedule::AfterCompletion {
            first_at_ns: 100,
            every_ns: std::num::NonZeroU64::new(10).unwrap(),
        },
        Schedule::FixedRate {
            first_at_ns: 100,
            every_ns: std::num::NonZeroU64::new(10).unwrap(),
            missed: MissedRunPolicy::Skip,
        },
    ] {
        for running in [false, true] {
            let mut job = Job::new(JobId(8), schedule, RetryPolicy::NONE);
            let first = job.start(100).unwrap();
            job.finish(first, 200, Outcome::Success).unwrap();
            assert_eq!(job.next_due_ns(), Some(210));
            if running {
                job.start(300).unwrap();
            } else {
                job.pause(201).unwrap();
                job.cancel(300).unwrap();
            }
            assert_eq!(roundtrip(&job).record(), job.record());
            for scheduled_ns in [110, 200, 220] {
                let mut corrupt = job.record();
                corrupt.scheduled_ns = scheduled_ns;
                assert_eq!(Job::restore(corrupt).unwrap_err(), JobError::InvalidRecord);
            }
        }
    }
}

#[test]
fn repeated_rebasing_preserves_history_and_counter_exhaustion() {
    for paused in [false, true] {
        let mut job = recurring(MissedRunPolicy::Skip);
        let attempt = job.start(100).unwrap();
        job.finish(attempt, 200, Outcome::Success).unwrap();
        if paused {
            job.pause(200).unwrap();
        }
        for first_at_ns in [0, u64::MAX, 10] {
            job.reschedule(
                Schedule::AfterCompletion {
                    first_at_ns,
                    every_ns: std::num::NonZeroU64::new(10).unwrap(),
                },
                200,
            )
            .unwrap();
            job = roundtrip(&job);
            assert_eq!(
                (job.record().occurrence, job.record().schedule_origin),
                (1, 1)
            );
            assert_eq!(
                job.record().last_outcome.map(|result| result.at_ns),
                Some(200)
            );
            assert_eq!(
                job.next_due_ns(),
                if paused { None } else { Some(first_at_ns) }
            );
        }
        if paused {
            job.resume(200).unwrap();
        }
        let mut record = job.record();
        record.sequence = u64::MAX - 1;
        record.occurrence = u64::MAX - 1;
        record.schedule_origin = u64::MAX - 1;
        job = Job::restore(record).unwrap();
        let last = job.start(200).unwrap();
        job.finish(last, 200, Outcome::Success).unwrap();
        job = roundtrip(&job);
        assert_eq!(
            (job.record().sequence, job.record().occurrence),
            (u64::MAX, u64::MAX)
        );
        job.pause(200).unwrap();
        job.reschedule(Schedule::Once { at_ns: 0 }, 201).unwrap();
        job.resume(202).unwrap();
        job = roundtrip(&job);
        let before = job.record();
        assert_eq!(job.start(202), Err(JobError::CounterExhausted));
        assert_eq!(job.record(), before);
        assert_eq!(roundtrip(&job).record(), before);
    }
}
