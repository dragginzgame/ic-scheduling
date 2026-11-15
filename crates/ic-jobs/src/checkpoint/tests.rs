use super::*;

#[cfg(feature = "execution")]
#[test]
fn definition_validation_covers_paused_batches_and_the_last_pending_occurrence() {
    use crate::{CatchUpLimit, Policy, Schedule};
    let minute = 60_000_000_000;
    let schedule = Schedule::parse("* * * * *").unwrap();
    let skip = Policy::SkipLate { lateness_ns: 0 };
    let full = Policy::CatchUp(CatchUpLimit::new(0, 64).unwrap());
    let short = Policy::CatchUp(CatchUpLimit::new(u64::MAX, 63).unwrap());
    let pending: Vec<_> = (1..=64).map(|index| index * minute).collect();
    for paused in [false, true] {
        let checkpoint = Checkpoint::new(
            6,
            1,
            Some(0),
            (!paused).then_some(minute),
            control(paused),
            Some(CatchUpProgress::new(&pending, 65 * minute).unwrap()),
            None,
        )
        .unwrap();
        // A frozen batch remains valid even with zero age tolerance on restoration.
        assert!(checkpoint.matches_definition(&schedule, full, crate::retry::RetryPolicy::NONE));
        assert!(!checkpoint.matches_definition(&schedule, short, crate::retry::RetryPolicy::NONE));
        assert!(!checkpoint.matches_definition(&schedule, skip, crate::retry::RetryPolicy::NONE));
        let mut invalid_pending = pending.clone();
        invalid_pending[63] += 1;
        let invalid = Checkpoint::new(
            6,
            1,
            Some(0),
            (!paused).then_some(minute),
            control(paused),
            Some(CatchUpProgress::new(&invalid_pending, 65 * minute).unwrap()),
            None,
        )
        .unwrap();
        assert!(!invalid.matches_definition(&schedule, full, crate::retry::RetryPolicy::NONE));
    }
    let single = Checkpoint::new(
        6,
        1,
        Some(0),
        Some(minute),
        ControlState::Active,
        None,
        None,
    )
    .unwrap();
    assert!(single.matches_definition(&schedule, skip, crate::retry::RetryPolicy::NONE));
    let off_calendar = Checkpoint::new(
        6,
        1,
        Some(0),
        Some(minute + 1),
        ControlState::Active,
        None,
        None,
    )
    .unwrap();
    assert!(!off_calendar.matches_definition(&schedule, skip, crate::retry::RetryPolicy::NONE));
}
#[test]
fn rejects_invalid_host_progress_without_repairing_it() {
    for version in 0..6 {
        assert_eq!(
            Checkpoint::new(version, 1, Some(0), None, ControlState::Active, None, None),
            Err(CheckpointError::Version)
        );
    }
    for (revision, through, selected, paused) in [
        (0, 0, None, false),
        (1, 10, Some(10), false),
        (1, 10, Some(9), false),
        (1, 10, Some(20), true),
    ] {
        assert_eq!(
            Checkpoint::new(
                6,
                revision,
                Some(through),
                selected,
                control(paused),
                None,
                None
            ),
            Err(CheckpointError::Progress)
        );
    }
    let checkpoint =
        Checkpoint::new(6, 9, Some(100), Some(200), ControlState::Active, None, None).unwrap();
    assert_eq!(
        (
            checkpoint.version(),
            checkpoint.revision(),
            checkpoint.through(),
            checkpoint.selected_at(),
            checkpoint.paused()
        ),
        (6, 9, Some(100), Some(200), false)
    );
    assert!(Checkpoint::new(6, 9, Some(u64::MAX), None, ControlState::Paused, None, None).is_ok());
}

#[test]
fn remaining_batch_must_match_progress_and_selection() {
    for pending in [&[][..], &[20, 20][..], &[30, 20][..], &[101][..]] {
        assert_eq!(
            CatchUpProgress::new(pending, 100),
            Err(CheckpointError::Progress)
        );
    }
    assert_eq!(
        CatchUpProgress::new(&[1; 65], 100),
        Err(CheckpointError::Progress)
    );
    let batch = CatchUpProgress::new(&[20, 30], 100).unwrap();
    assert_eq!(batch.pending(), &[20, 30]);
    assert_eq!(batch.through(), 100);
    for (through, selected, paused) in [
        (20, Some(30), false),
        (10, Some(30), false),
        (10, None, false),
    ] {
        assert_eq!(
            Checkpoint::new(
                6,
                1,
                Some(through),
                selected,
                control(paused),
                Some(batch.clone()),
                None
            ),
            Err(CheckpointError::Progress)
        );
    }
    let armed = Checkpoint::new(
        6,
        1,
        Some(10),
        Some(20),
        ControlState::Active,
        Some(batch.clone()),
        None,
    )
    .unwrap();
    assert_eq!(armed.catch_up(), Some(&batch));
    let paused = Checkpoint::new(
        6,
        1,
        Some(10),
        None,
        ControlState::Paused,
        Some(batch.clone()),
        None,
    )
    .unwrap();
    assert_eq!(paused.catch_up(), Some(&batch));
}

#[cfg(feature = "execution")]
#[test]
fn unstarted_checkpoint_and_zero_batch_are_distinct_from_settled_zero() {
    let batch = CatchUpProgress::new(&[0, 10, 20], 30).unwrap();
    let schedule = crate::Schedule::fixed_rate(0, std::num::NonZeroU64::new(10).unwrap());
    let policy = crate::Policy::CatchUp(crate::CatchUpLimit::new(0, 3).unwrap());
    for paused in [false, true] {
        let checkpoint = Checkpoint::new(
            6,
            1,
            None,
            (!paused).then_some(0),
            control(paused),
            Some(batch.clone()),
            None,
        )
        .unwrap();
        assert!(checkpoint.matches_definition(&schedule, policy, crate::retry::RetryPolicy::NONE));
        assert!(
            Checkpoint::new(
                6,
                1,
                Some(0),
                (!paused).then_some(0),
                control(paused),
                Some(batch.clone()),
                None
            )
            .is_err()
        );
    }
    assert!(Checkpoint::new(2, 1, None, None, ControlState::Active, None, None).is_err());
}

#[cfg(feature = "execution")]
#[test]
fn one_shot_restore_requires_the_exact_deadline() {
    let schedule = crate::Schedule::once(0);
    let policy = crate::Policy::SkipLate {
        lateness_ns: u64::MAX,
    };
    let initial = Checkpoint::new(6, 1, None, Some(0), ControlState::Active, None, None).unwrap();
    assert!(initial.matches_definition(&schedule, policy, crate::retry::RetryPolicy::NONE));
    assert!(!initial.matches_definition(
        &crate::Schedule::once(1),
        policy,
        crate::retry::RetryPolicy::NONE
    ));
    let batch = CatchUpProgress::new(&[0, 1], 1).unwrap();
    let invalid =
        Checkpoint::new(6, 1, None, Some(0), ControlState::Active, Some(batch), None).unwrap();
    assert!(!invalid.matches_definition(
        &schedule,
        crate::Policy::CatchUp(crate::CatchUpLimit::new(u64::MAX, 2).unwrap()),
        crate::retry::RetryPolicy::NONE
    ));
}

#[cfg(feature = "execution")]
#[test]
fn completion_deadline_must_match_the_persisted_horizon() {
    use crate::{CatchUpLimit, Policy, Schedule};
    let schedule = Schedule::after_completion(0, std::num::NonZeroU64::new(10).unwrap());
    for policy in [
        Policy::SkipLate { lateness_ns: 0 },
        Policy::CatchUp(CatchUpLimit::new(0, 3).unwrap()),
    ] {
        for (through, selected, paused, valid) in [
            (None, Some(0), false, true),
            (Some(23), Some(33), false, true),
            (Some(23), Some(30), false, false),
            (Some(23), None, false, false),
            (Some(23), None, true, true),
            (Some(u64::MAX - 9), None, false, true),
        ] {
            let checkpoint =
                Checkpoint::new(6, 1, through, selected, control(paused), None, None).unwrap();
            assert_eq!(
                checkpoint.matches_definition(&schedule, policy, crate::retry::RetryPolicy::NONE),
                valid
            );
        }
        let batch = CatchUpProgress::new(&[33], 33).unwrap();
        let checkpoint = Checkpoint::new(
            6,
            1,
            Some(23),
            Some(33),
            ControlState::Active,
            Some(batch),
            None,
        )
        .unwrap();
        assert!(
            !checkpoint.matches_definition(&schedule, policy, crate::retry::RetryPolicy::NONE),
            "completion-based work has no accumulated backlog"
        );
    }
}

fn control(paused: bool) -> ControlState {
    if paused {
        ControlState::Paused
    } else {
        ControlState::Active
    }
}

#[test]
fn cancelled_records_cannot_retain_selected_or_batch_work() {
    for through in [None, Some(0), Some(u64::MAX)] {
        let checkpoint =
            Checkpoint::new(6, 1, through, None, ControlState::Cancelled, None, None).unwrap();
        assert_eq!(checkpoint.control(), ControlState::Cancelled);
        assert!(!checkpoint.paused());
    }
    assert_eq!(
        Checkpoint::new(6, 1, None, Some(0), ControlState::Cancelled, None, None),
        Err(CheckpointError::Progress)
    );
    let batch = CatchUpProgress::new(&[0], 0).unwrap();
    assert_eq!(
        Checkpoint::new(6, 1, None, None, ControlState::Cancelled, Some(batch), None),
        Err(CheckpointError::Progress)
    );
    assert_eq!(
        Checkpoint::new(3, 1, None, None, ControlState::Active, None, None),
        Err(CheckpointError::Version)
    );
}

#[test]
fn pending_retry_data_preserves_identity_and_consumed_budget() {
    use crate::retry::RetryProgress;
    for values in [
        (10, 0, 10, 20),
        (10, u32::MAX, 10, 20),
        (10, 1, 9, 20),
        (10, 1, 20, 20),
    ] {
        assert_eq!(
            RetryProgress::new(values.0, values.1, values.2, values.3),
            Err(CheckpointError::Progress)
        );
    }
    let retry = RetryProgress::new(10, 2, 30, 50).unwrap();
    assert_eq!(
        (
            retry.scheduled_at(),
            retry.failed_attempts(),
            retry.failed_at(),
            retry.retry_at()
        ),
        (10, 2, 30, 50)
    );
    for (through, selected, control, valid) in [
        (None, Some(10), ControlState::Active, true),
        (Some(9), None, ControlState::Paused, true),
        (Some(10), Some(10), ControlState::Active, false),
        (None, Some(11), ControlState::Active, false),
        (None, None, ControlState::Active, false),
        (None, None, ControlState::Cancelled, false),
    ] {
        let result = Checkpoint::new(6, 1, through, selected, control, None, Some(retry.clone()));
        assert_eq!(result.is_ok(), valid);
        if let Ok(checkpoint) = result {
            assert_eq!(checkpoint.retry(), Some(&retry));
        }
    }
    let batch = CatchUpProgress::new(&[20], 30).unwrap();
    assert!(
        Checkpoint::new(
            6,
            1,
            None,
            None,
            ControlState::Paused,
            Some(batch),
            Some(retry)
        )
        .is_err()
    );
    assert_eq!(
        Checkpoint::new(4, 1, None, None, ControlState::Active, None, None),
        Err(CheckpointError::Version)
    );
}

#[cfg(feature = "execution")]
#[test]
fn restore_checks_retry_budget_deadline_and_schedule_even_while_paused() {
    use crate::{Policy, RetryPolicy, RetryProgress, Schedule};
    let policy = Policy::SkipLate { lateness_ns: 0 };
    let retry = RetryProgress::new(10, 2, 30, 50).unwrap();
    let budget = RetryPolicy::new(3, 10, 20).unwrap();
    for paused in [false, true] {
        let checkpoint = Checkpoint::new(
            6,
            1,
            None,
            (!paused).then_some(10),
            control(paused),
            None,
            Some(retry.clone()),
        )
        .unwrap();
        assert!(checkpoint.matches_definition(&Schedule::once(10), policy, budget));
        assert!(checkpoint.matches_definition(
            &Schedule::after_completion(10, std::num::NonZeroU64::new(1).unwrap()),
            policy,
            budget
        ));
        assert!(!checkpoint.matches_definition(&Schedule::once(11), policy, budget));
        assert!(!checkpoint.matches_definition(&Schedule::once(10), policy, RetryPolicy::NONE));
        assert!(!checkpoint.matches_definition(
            &Schedule::once(10),
            policy,
            RetryPolicy::new(2, 10, 20).unwrap()
        ));
        assert!(!checkpoint.matches_definition(
            &Schedule::once(10),
            policy,
            RetryPolicy::new(3, 10, 15).unwrap()
        ));
    }
}
