//! Observable scheduling, persistence and external-effect recovery contracts.
use ic_jobs::{
    Attempt, Job, JobError, JobId, JobState, MissedRunPolicy, Outcome, RetryPolicy, Schedule,
    next_due_ns, start_next,
};

use super::support::{non_catchup_schedules, recurring_schedules, roundtrip};

fn retry() -> RetryPolicy {
    RetryPolicy::new(4, 10, 15).unwrap()
}

fn once() -> Job {
    Job::new(JobId(42), Schedule::Once { at_ns: 100 }, retry())
}

#[test]
fn scheduler_starts_one_earliest_due_job_and_preserves_batch_order() {
    let mut empty = [];
    assert_eq!(next_due_ns(&empty), None);
    assert_eq!(start_next(&mut empty, 100), Ok(None));
    let mut jobs = [
        (
            Job::new(JobId(1), Schedule::Once { at_ns: 200 }, retry()),
            "later",
        ),
        (
            Job::new(JobId(2), Schedule::Once { at_ns: 100 }, retry()),
            "first",
        ),
        (
            Job::new(JobId(3), Schedule::Once { at_ns: 100 }, retry()),
            "second",
        ),
    ];
    let before = jobs.each_ref().map(|(job, _)| job.record());
    assert_eq!(next_due_ns(jobs.iter().map(|(job, _)| job)), Some(100));
    assert_eq!(
        start_next(jobs.iter_mut().map(|(job, _)| job), 99),
        Ok(None)
    );
    assert_eq!(jobs.each_ref().map(|(job, _)| job.record()), before);

    let (index, attempt) = start_next(jobs.iter_mut().map(|(job, _)| job), 200)
        .unwrap()
        .unwrap();
    assert_eq!(index, 1);
    assert_eq!(attempt.execution.job, JobId(2));
    let record = jobs[index].0.record();
    assert_eq!(record.state, JobState::Running);
    assert_eq!(jobs[0].0.record(), before[0]);
    assert_eq!(jobs[2].0.record(), before[2]);
    assert_eq!(next_due_ns(jobs.iter().map(|(job, _)| job)), Some(100));

    let (index, attempt) = start_next(jobs.iter_mut().map(|(job, _)| job), 200)
        .unwrap()
        .unwrap();
    assert_eq!(index, 2);
    assert_eq!(attempt.execution.job, JobId(3));
    let (index, attempt) = start_next(jobs.iter_mut().map(|(job, _)| job), 200)
        .unwrap()
        .unwrap();
    assert_eq!(index, 0);
    assert_eq!(attempt.execution.job, JobId(1));
    assert_eq!(next_due_ns(jobs.iter().map(|(job, _)| job)), None);
    assert_eq!(
        start_next(jobs.iter_mut().map(|(job, _)| job), 300),
        Ok(None)
    );
    assert_eq!(
        jobs.map(|(_, payload)| payload),
        ["later", "first", "second"]
    );
}

#[test]
fn scheduler_restores_blocked_work_without_redispatch_until_disposition() {
    let mut running = Job::new(JobId(1), Schedule::Once { at_ns: 100 }, retry());
    running.start(100).unwrap();
    let mut uncertain = Job::new(JobId(2), Schedule::Once { at_ns: 100 }, retry());
    let uncertain_attempt = uncertain.start(100).unwrap();
    uncertain
        .finish(uncertain_attempt, 101, Outcome::Uncertain)
        .unwrap();
    let mut cancelled = Job::new(JobId(3), Schedule::Once { at_ns: 100 }, retry());
    cancelled.cancel(90).unwrap();
    let mut completed = Job::new(JobId(4), Schedule::Once { at_ns: 100 }, retry());
    let completed_attempt = completed.start(100).unwrap();
    completed
        .finish(completed_attempt, 101, Outcome::Success)
        .unwrap();
    let mut failed = Job::new(JobId(5), Schedule::Once { at_ns: 100 }, retry());
    let failed_attempt = failed.start(100).unwrap();
    failed
        .finish(failed_attempt, 101, Outcome::PermanentFailure)
        .unwrap();
    let mut jobs = [
        roundtrip(&running),
        roundtrip(&uncertain),
        roundtrip(&cancelled),
        roundtrip(&completed),
        roundtrip(&failed),
    ];
    let before = jobs.each_ref().map(Job::record);
    assert_eq!(next_due_ns(&jobs), None);
    assert_eq!(start_next(&mut jobs, 200), Ok(None));
    assert_eq!(jobs.each_ref().map(Job::record), before);

    jobs[1]
        .resolve(uncertain_attempt, 200, Outcome::RetryableFailure)
        .unwrap();
    assert_eq!(next_due_ns(&jobs), Some(210));
    assert_eq!(start_next(&mut jobs, 209), Ok(None));
    let (_, attempt) = start_next(&mut jobs, 210).unwrap().unwrap();
    assert_eq!(attempt.execution, uncertain_attempt.execution);
    assert_ne!(attempt.sequence, uncertain_attempt.sequence);
    assert_eq!(jobs[0].record(), before[0]);
}

#[test]
fn scheduler_rechecks_retry_and_recurring_deadlines_after_completion() {
    let job = Job::new(
        JobId(1),
        Schedule::FixedRate {
            first_at_ns: 100,
            every_ns: std::num::NonZeroU64::new(100).unwrap(),
            missed: MissedRunPolicy::CatchUp,
        },
        retry(),
    );
    let mut jobs = [job];
    let (index, first) = start_next(&mut jobs, 100).unwrap().unwrap();
    let intent = jobs[index].record();
    jobs[0] = Job::restore(intent).unwrap();
    jobs[0]
        .finish(first, 100, Outcome::RetryableFailure)
        .unwrap();
    jobs[0] = roundtrip(&jobs[0]);
    assert_eq!(next_due_ns(&jobs), Some(110));
    assert_eq!(start_next(&mut jobs, 109), Ok(None));
    let (index, second) = start_next(&mut jobs, 110).unwrap().unwrap();
    let intent = jobs[index].record();
    assert_eq!(first.execution, second.execution);
    assert_ne!(first.sequence, second.sequence);
    jobs[0] = Job::restore(intent).unwrap();
    jobs[0].finish(second, 120, Outcome::Success).unwrap();
    jobs[0] = roundtrip(&jobs[0]);
    assert_eq!(next_due_ns(&jobs), Some(200));
    assert_eq!(start_next(&mut jobs, 199), Ok(None));
    let (_, third) = start_next(&mut jobs, 200).unwrap().unwrap();
    assert_ne!(third.execution, first.execution);
}

#[test]
fn scheduler_transition_errors_preserve_all_jobs_without_fallthrough() {
    let mut catch_up = Job::new(
        JobId(1),
        Schedule::FixedRate {
            first_at_ns: 100,
            every_ns: std::num::NonZeroU64::new(10).unwrap(),
            missed: MissedRunPolicy::CatchUp,
        },
        retry(),
    );
    let attempt = catch_up.start(100).unwrap();
    catch_up.finish(attempt, 200, Outcome::Success).unwrap();
    let mut jobs = [
        catch_up,
        Job::new(JobId(2), Schedule::Once { at_ns: 120 }, retry()),
    ];
    let before = jobs.each_ref().map(Job::record);
    assert_eq!(start_next(&mut jobs, 150), Err(JobError::TimeWentBackwards));
    assert_eq!(jobs.each_ref().map(Job::record), before);

    let mut exhausted = once();
    let attempt = exhausted.start(100).unwrap();
    exhausted
        .finish(attempt, 100, Outcome::RetryableFailure)
        .unwrap();
    let mut record = exhausted.record();
    record.sequence = u64::MAX;
    record.occurrence = u64::MAX - 1;
    record.schedule_origin = record.occurrence;
    jobs[0] = Job::restore(record).unwrap();
    let before = jobs.each_ref().map(Job::record);
    assert_eq!(start_next(&mut jobs, 150), Err(JobError::CounterExhausted));
    assert_eq!(jobs.each_ref().map(Job::record), before);
}

#[test]
fn persistence_preserves_pending_running_and_terminal_states() {
    let mut job = roundtrip(&once());
    assert_eq!(job.start(99), Err(JobError::NotDue));
    let attempt = job.start(100).unwrap();
    let mut job = roundtrip(&job);
    assert_eq!(job.start(101), Err(JobError::NotPending));
    assert_eq!(job.active_attempt(), Some(attempt));
    job.finish(attempt, 101, Outcome::Success).unwrap();
    let mut job = roundtrip(&job);
    assert_eq!(job.record().state, JobState::Completed);
    assert_eq!(
        job.finish(attempt, 102, Outcome::Success),
        Err(JobError::StaleAttempt)
    );
    assert_eq!(job.next_due_ns(), None);
}

#[test]
fn retries_keep_effect_identity_cap_backoff_and_stop_at_budget() {
    let mut job = once();
    let first = job.start(100).unwrap();
    job.finish(first, 100, Outcome::RetryableFailure).unwrap();
    assert_eq!(job.next_due_ns(), Some(110));
    let second = job.start(110).unwrap();
    assert_eq!(first.execution, second.execution);
    assert_ne!(first.sequence, second.sequence);
    assert_eq!(
        job.finish(first, 110, Outcome::Success),
        Err(JobError::StaleAttempt)
    );
    job.finish(second, 110, Outcome::RetryableFailure).unwrap();
    assert_eq!(job.next_due_ns(), Some(125));
    let third = job.start(125).unwrap();
    job.finish(third, 125, Outcome::RetryableFailure).unwrap();
    assert_eq!(job.next_due_ns(), Some(140));
    let fourth = job.start(140).unwrap();
    job.finish(fourth, 140, Outcome::RetryableFailure).unwrap();
    assert_eq!(roundtrip(&job).record().state, JobState::Failed);
    assert_eq!(job.start(1_000), Err(JobError::NotPending));
}

#[test]
fn uncertain_effect_requires_explicit_reconciliation_after_restore() {
    let mut job = once();
    let attempt = job.start(100).unwrap();
    job.finish(attempt, 101, Outcome::Uncertain).unwrap();
    let mut job = roundtrip(&job);
    assert_eq!(job.next_due_ns(), None);
    assert_eq!(job.start(200), Err(JobError::NotPending));
    assert_eq!(job.cancel(200), Err(JobError::InFlight));
    assert_eq!(
        job.finish(attempt, 200, Outcome::Success),
        Err(JobError::StaleAttempt)
    );
    job.resolve(attempt, 200, Outcome::RetryableFailure)
        .unwrap();
    assert_eq!(job.next_due_ns(), Some(210));
    assert_eq!(roundtrip(&job).record(), job.record());
    let repeated = job.start(210).unwrap();
    assert_eq!(attempt.execution, repeated.execution);
    assert_ne!(attempt.sequence, repeated.sequence);
}

#[test]
fn interrupted_running_intent_can_be_marked_uncertain_then_confirmed() {
    let mut job = once();
    let attempt = job.start(100).unwrap();
    let mut restored = roundtrip(&job);
    restored.finish(attempt, 120, Outcome::Uncertain).unwrap();
    restored.resolve(attempt, 130, Outcome::Success).unwrap();
    assert_eq!(roundtrip(&restored).record().state, JobState::Completed);
}

#[test]
fn missed_run_policy_distinguishes_skip_catchup_and_after_completion() {
    let [after_completion, skip, catch_up] = recurring_schedules(100, 10);
    let schedules = [(skip, 140), (catch_up, 110), (after_completion, 145)];
    for (schedule, next_due) in schedules {
        let mut job = Job::new(JobId(1), schedule, RetryPolicy::NONE);
        let attempt = job.start(130).unwrap();
        job.finish(attempt, 135, Outcome::Success).unwrap();
        assert_eq!(job.next_due_ns(), Some(next_due));
        assert_eq!(job.record().occurrence, 1);
        assert_eq!(job.record().attempts, 0);
        assert_eq!(roundtrip(&job).record(), job.record());
    }
}

#[test]
fn recurring_occurrence_has_fresh_effect_identity_and_retry_budget() {
    let mut job = Job::new(
        JobId(1),
        Schedule::FixedRate {
            first_at_ns: 100,
            every_ns: std::num::NonZeroU64::new(100).unwrap(),
            missed: MissedRunPolicy::Skip,
        },
        retry(),
    );
    let first = job.start(100).unwrap();
    job.finish(first, 100, Outcome::RetryableFailure).unwrap();
    let retry = job.start(110).unwrap();
    job.finish(retry, 110, Outcome::Success).unwrap();
    let next = job.start(200).unwrap();
    assert_ne!(first.execution, next.execution);
    assert_eq!(job.record().attempts, 1);
    assert_eq!(roundtrip(&job).record(), job.record());
}

#[test]
fn pending_cancellation_survives_restore_and_cannot_abort_inflight_work() {
    let mut job = once();
    job.cancel(50).unwrap();
    let mut restored = roundtrip(&job);
    assert_eq!(restored.record().state, JobState::Cancelled);
    assert_eq!(restored.start(100), Err(JobError::NotPending));
    let mut running = once();
    running.start(100).unwrap();
    assert_eq!(running.cancel(101), Err(JobError::InFlight));
}

#[test]
fn timestamp_errors_leave_dispatch_intent_unchanged() {
    let mut job = Job::new(JobId(1), Schedule::Once { at_ns: u64::MAX }, retry());
    let attempt = job.start(u64::MAX).unwrap();
    let before = job.record();
    assert_eq!(
        job.finish(attempt, u64::MAX - 1, Outcome::Success),
        Err(JobError::TimeWentBackwards)
    );
    assert_eq!(job.record(), before);
    assert_eq!(
        job.finish(attempt, u64::MAX, Outcome::RetryableFailure),
        Err(JobError::DeadlineOverflow)
    );
    assert_eq!(job.record(), before);
    job.finish(attempt, u64::MAX, Outcome::PermanentFailure)
        .unwrap();
    assert_eq!(roundtrip(&job).record().state, JobState::Failed);
}

#[test]
fn recurrence_overflow_preserves_active_attempt() {
    let mut job = Job::new(
        JobId(1),
        Schedule::FixedRate {
            first_at_ns: u64::MAX - 1,
            every_ns: std::num::NonZeroU64::new(2).unwrap(),
            missed: MissedRunPolicy::Skip,
        },
        RetryPolicy::NONE,
    );
    let attempt = job.start(u64::MAX - 1).unwrap();
    let before = job.record();
    assert_eq!(
        job.finish(attempt, u64::MAX, Outcome::Success),
        Err(JobError::DeadlineOverflow)
    );
    assert_eq!(job.record(), before);
}

#[test]
fn restore_rejects_dispatch_sequences_exceeding_history_budget() {
    let mut job = once();
    let attempt = job.start(100).unwrap();
    let running = job.record();
    let mut records = vec![running];
    for outcome in [
        Outcome::Success,
        Outcome::RetryableFailure,
        Outcome::PermanentFailure,
        Outcome::Uncertain,
    ] {
        let mut job = Job::restore(running).unwrap();
        job.finish(attempt, 100, outcome).unwrap();
        records.push(job.record());
        if outcome == Outcome::RetryableFailure {
            job.pause(101).unwrap();
            records.push(job.record());
            job.cancel(102).unwrap();
            records.push(job.record());
        }
    }
    for record in records {
        assert_eq!(Job::restore(record).unwrap().record(), record);
        let mut corrupt = record;
        corrupt.sequence += 1;
        assert_eq!(
            Job::restore(corrupt).unwrap_err(),
            JobError::InvalidRecord,
            "extra dispatch without a prior completed occurrence: {corrupt:?}"
        );
    }
}

#[test]
fn restore_keeps_history_budget_after_schedule_replacement() {
    let mut job = Job::new(
        JobId(42),
        Schedule::AfterCompletion {
            first_at_ns: 100,
            every_ns: std::num::NonZeroU64::new(10).unwrap(),
        },
        retry(),
    );
    for now in [100, 110, 125] {
        let attempt = job.start(now).unwrap();
        job.finish(attempt, now, Outcome::RetryableFailure).unwrap();
    }
    let attempt = job.start(140).unwrap();
    job.finish(attempt, 140, Outcome::Success).unwrap();
    let completed = job.record();
    assert_eq!((completed.sequence, completed.occurrence), (4, 1));

    for paused in [false, true] {
        let mut job = Job::restore(completed).unwrap();
        if paused {
            job.pause(140).unwrap();
        }
        job.reschedule(Schedule::Once { at_ns: 200 }, 140).unwrap();
        assert_eq!(job.record().schedule_origin, 1);
        let record = job.record();
        assert_eq!(Job::restore(record).unwrap().record(), record);
        let mut corrupt = record;
        corrupt.sequence += 1;
        assert_eq!(Job::restore(corrupt).unwrap_err(), JobError::InvalidRecord);
        if paused {
            job.resume(200).unwrap();
        }
        job.start(200).unwrap();
        let record = job.record();
        assert_eq!((record.sequence, record.attempts), (5, 1));
        assert_eq!(Job::restore(record).unwrap().record(), record);
        let mut corrupt = record;
        corrupt.sequence += 1;
        assert_eq!(Job::restore(corrupt).unwrap_err(), JobError::InvalidRecord);
    }
}

#[test]
fn sequence_exhaustion_cannot_reuse_attempt_identity() {
    let mut job = once();
    let attempt = job.start(100).unwrap();
    job.finish(attempt, 100, Outcome::RetryableFailure).unwrap();
    let mut record = job.record();
    record.sequence = u64::MAX;
    record.occurrence = u64::MAX - 1;
    record.schedule_origin = record.occurrence;
    let mut job = Job::restore(record).unwrap();
    let before = job.record();
    assert_eq!(job.start(110), Err(JobError::CounterExhausted));
    assert_eq!(job.record(), before);
}

#[test]
fn corrupt_decoded_configuration_and_state_are_rejected() {
    for (name, attempts, state) in [
        ("exhausted budget", 5, JobState::Pending { due_ns: 100 }),
        ("unstarted active work", 0, JobState::Running),
        ("earlier deadline", 0, JobState::Pending { due_ns: 99 }),
        ("unstarted completion", 0, JobState::Completed),
    ] {
        let mut record = once().record();
        record.attempts = attempts;
        record.state = state;
        assert_eq!(
            Job::restore(record).unwrap_err(),
            JobError::InvalidRecord,
            "{name}"
        );
    }
}

#[test]
fn restore_rejects_successful_work_rewritten_as_a_first_pending_occurrence() {
    let mut job = once();
    let attempt = job.start(100).unwrap();
    job.finish(attempt, 200, Outcome::Success).unwrap();
    let mut record = roundtrip(&job).record();
    record.attempts = 0;
    record.state = JobState::Pending { due_ns: 100 };
    assert_eq!(Job::restore(record).unwrap_err(), JobError::InvalidRecord);
    record.state = JobState::Cancelled;
    assert_eq!(Job::restore(record).unwrap_err(), JobError::InvalidRecord);
}

#[test]
fn restore_rejects_retry_completion_before_the_occurrence_deadline() {
    let mut job = once();
    let attempt = job.start(100).unwrap();
    job.finish(attempt, 100, Outcome::RetryableFailure).unwrap();
    let mut record = roundtrip(&job).record();
    record.updated_ns = 99;
    record.state = JobState::Pending { due_ns: 109 };
    assert_eq!(Job::restore(record).unwrap_err(), JobError::InvalidRecord);
}

#[test]
fn restore_keeps_the_first_occurrence_at_its_original_deadline() {
    for schedule in non_catchup_schedules(100, 10) {
        let mut job = Job::new(JobId(7), schedule, retry());
        let attempt = job.start(200).unwrap();
        let running = roundtrip(&job).record();
        job.finish(attempt, 200, Outcome::Uncertain).unwrap();
        let uncertain = roundtrip(&job).record();
        job.resolve(attempt, 200, Outcome::RetryableFailure)
            .unwrap();
        let retrying = roundtrip(&job).record();
        job.cancel(201).unwrap();
        let cancelled = roundtrip(&job).record();
        let mut failed = Job::restore(running).unwrap();
        failed
            .finish(attempt, 200, Outcome::PermanentFailure)
            .unwrap();
        for mut record in [running, uncertain, retrying, cancelled, failed.record()] {
            record.scheduled_ns = 110;
            assert_eq!(Job::restore(record).unwrap_err(), JobError::InvalidRecord);
        }
    }
}

#[test]
fn restore_rejects_cancellation_before_the_prior_recurring_completion() {
    for schedule in recurring_schedules(100, 10) {
        let mut job = Job::new(JobId(7), schedule, retry());
        let attempt = job.start(100).unwrap();
        job.finish(attempt, 200, Outcome::Success).unwrap();
        let pending = roundtrip(&job).record();
        for cancelled_ns in [200, 300] {
            let mut cancelled = Job::restore(pending).unwrap();
            cancelled.cancel(cancelled_ns).unwrap();
            let mut record = roundtrip(&cancelled).record();
            record.updated_ns = record.scheduled_ns - 11;
            assert_eq!(Job::restore(record).unwrap_err(), JobError::InvalidRecord);
        }
    }
}

#[test]
fn recurring_recovery_preserves_zero_and_maximum_deadlines() {
    for every_ns in [1, u64::MAX] {
        for schedule in recurring_schedules(0, every_ns) {
            let mut job = Job::new(JobId(7), schedule, retry());
            let attempt = job.start(0).unwrap();
            job = roundtrip(&job);
            job.finish(attempt, 0, Outcome::Success).unwrap();
            let pending = roundtrip(&job).record();
            assert_eq!(pending.state, JobState::Pending { due_ns: every_ns });
            let mut jobs = [Job::restore(pending).unwrap()];
            assert_eq!(next_due_ns(&jobs), Some(every_ns));
            assert_eq!(start_next(&mut jobs, 0), Ok(None));
            assert_eq!(jobs[0].record(), pending);
            for cancelled_ns in [0, u64::MAX] {
                let mut cancelled = Job::restore(pending).unwrap();
                cancelled.cancel(cancelled_ns).unwrap();
                assert_eq!(roundtrip(&cancelled).record(), cancelled.record());
            }
        }
    }
}

#[test]
fn restore_checks_successor_deadlines_against_the_recorded_completion() {
    for schedule in non_catchup_schedules(100, 10) {
        let mut job = Job::new(JobId(7), schedule, retry());
        let attempt = job.start(100).unwrap();
        job.finish(attempt, 200, Outcome::Success).unwrap();
        assert_eq!(roundtrip(&job).next_due_ns(), Some(210));
        for deadline in [110, 200, 220] {
            let mut record = job.record();
            record.scheduled_ns = deadline;
            record.state = JobState::Pending { due_ns: deadline };
            assert_eq!(Job::restore(record).unwrap_err(), JobError::InvalidRecord);
        }
        let mut record = job.record();
        record.updated_ns = 99;
        assert_eq!(Job::restore(record).unwrap_err(), JobError::InvalidRecord);
        record.updated_ns = u64::MAX;
        record.last_outcome.as_mut().unwrap().at_ns = u64::MAX;
        assert_eq!(Job::restore(record).unwrap_err(), JobError::InvalidRecord);

        // Cancellation records a later transition time, not another completion.
        job.cancel(300).unwrap();
        assert_eq!(roundtrip(&job).record(), job.record());
    }

    let mut catch_up = Job::new(
        JobId(8),
        Schedule::FixedRate {
            first_at_ns: 100,
            every_ns: std::num::NonZeroU64::new(10).unwrap(),
            missed: MissedRunPolicy::CatchUp,
        },
        retry(),
    );
    for _ in 0..2 {
        let attempt = catch_up.start(200).unwrap();
        catch_up.finish(attempt, 200, Outcome::Success).unwrap();
        assert!(roundtrip(&catch_up).next_due_ns().unwrap() < 200);
        let mut record = catch_up.record();
        record.updated_ns = record.scheduled_ns - 11;
        assert_eq!(Job::restore(record).unwrap_err(), JobError::InvalidRecord);
    }
}

#[test]
fn completion_from_another_job_cannot_change_state() {
    let mut job = once();
    let mut other = Job::new(JobId(99), Schedule::Once { at_ns: 100 }, retry());
    job.start(100).unwrap();
    let other_attempt: Attempt = other.start(100).unwrap();
    let before = job.record();
    assert_eq!(
        job.finish(other_attempt, 100, Outcome::Success),
        Err(JobError::StaleAttempt)
    );
    assert_eq!(job.record(), before);
}

#[test]
fn transition_sequences_remain_restorable() {
    for schedule in recurring_schedules(100, 20) {
        let mut job = Job::new(JobId(7), schedule, retry());
        let mut now = 200;
        for _ in 0..50 {
            now = now.max(job.next_due_ns().unwrap());
            let attempt = job.start(now).unwrap();
            job = roundtrip(&job);
            job.finish(attempt, now, Outcome::Uncertain).unwrap();
            job = roundtrip(&job);
            job.resolve(attempt, now + 1, Outcome::RetryableFailure)
                .unwrap();
            job = roundtrip(&job);
            now = job.next_due_ns().unwrap();
            let attempt = job.start(now).unwrap();
            job.finish(attempt, now + 1, Outcome::Success).unwrap();
            job = roundtrip(&job);
            now += 3;
        }
        job.cancel(now).unwrap();
        assert_eq!(roundtrip(&job).record().state, JobState::Cancelled);
    }
}
