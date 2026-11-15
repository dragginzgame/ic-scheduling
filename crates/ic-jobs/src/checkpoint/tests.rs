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
            3,
            1,
            Some(0),
            (!paused).then_some(minute),
            paused,
            Some(CatchUpProgress::new(&pending, 65 * minute).unwrap()),
        )
        .unwrap();
        // A frozen batch remains valid even with zero age tolerance on restoration.
        assert!(checkpoint.matches_definition(&schedule, full));
        assert!(!checkpoint.matches_definition(&schedule, short));
        assert!(!checkpoint.matches_definition(&schedule, skip));
        let mut invalid_pending = pending.clone();
        invalid_pending[63] += 1;
        let invalid = Checkpoint::new(
            3,
            1,
            Some(0),
            (!paused).then_some(minute),
            paused,
            Some(CatchUpProgress::new(&invalid_pending, 65 * minute).unwrap()),
        )
        .unwrap();
        assert!(!invalid.matches_definition(&schedule, full));
    }
    let single = Checkpoint::new(3, 1, Some(0), Some(minute), false, None).unwrap();
    assert!(single.matches_definition(&schedule, skip));
    let off_calendar = Checkpoint::new(3, 1, Some(0), Some(minute + 1), false, None).unwrap();
    assert!(!off_calendar.matches_definition(&schedule, skip));
}
#[test]
fn rejects_invalid_host_progress_without_repairing_it() {
    assert_eq!(
        Checkpoint::new(1, 1, Some(0), None, false, None),
        Err(CheckpointError::Version)
    );
    for (revision, through, selected, paused) in [
        (0, 0, None, false),
        (1, 10, Some(10), false),
        (1, 10, Some(9), false),
        (1, 10, Some(20), true),
    ] {
        assert_eq!(
            Checkpoint::new(3, revision, Some(through), selected, paused, None),
            Err(CheckpointError::Progress)
        );
    }
    let checkpoint = Checkpoint::new(3, 9, Some(100), Some(200), false, None).unwrap();
    assert_eq!(
        (
            checkpoint.version(),
            checkpoint.revision(),
            checkpoint.through(),
            checkpoint.selected_at(),
            checkpoint.paused()
        ),
        (3, 9, Some(100), Some(200), false)
    );
    assert!(Checkpoint::new(3, 9, Some(u64::MAX), None, true, None).is_ok());
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
            Checkpoint::new(3, 1, Some(through), selected, paused, Some(batch.clone())),
            Err(CheckpointError::Progress)
        );
    }
    let armed = Checkpoint::new(3, 1, Some(10), Some(20), false, Some(batch.clone())).unwrap();
    assert_eq!(armed.catch_up(), Some(&batch));
    let paused = Checkpoint::new(3, 1, Some(10), None, true, Some(batch.clone())).unwrap();
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
            3,
            1,
            None,
            (!paused).then_some(0),
            paused,
            Some(batch.clone()),
        )
        .unwrap();
        assert!(checkpoint.matches_definition(&schedule, policy));
        assert!(
            Checkpoint::new(
                3,
                1,
                Some(0),
                (!paused).then_some(0),
                paused,
                Some(batch.clone())
            )
            .is_err()
        );
    }
    assert!(Checkpoint::new(2, 1, None, None, false, None).is_err());
}

#[cfg(feature = "execution")]
#[test]
fn one_shot_restore_requires_the_exact_deadline() {
    let schedule = crate::Schedule::once(0);
    let policy = crate::Policy::SkipLate {
        lateness_ns: u64::MAX,
    };
    let initial = Checkpoint::new(3, 1, None, Some(0), false, None).unwrap();
    assert!(initial.matches_definition(&schedule, policy));
    assert!(!initial.matches_definition(&crate::Schedule::once(1), policy));
    let batch = CatchUpProgress::new(&[0, 1], 1).unwrap();
    let invalid = Checkpoint::new(3, 1, None, Some(0), false, Some(batch)).unwrap();
    assert!(!invalid.matches_definition(
        &schedule,
        crate::Policy::CatchUp(crate::CatchUpLimit::new(u64::MAX, 2).unwrap())
    ));
}
