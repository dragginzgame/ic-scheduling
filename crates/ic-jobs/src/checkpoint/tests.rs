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
            2,
            1,
            0,
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
            2,
            1,
            0,
            (!paused).then_some(minute),
            paused,
            Some(CatchUpProgress::new(&invalid_pending, 65 * minute).unwrap()),
        )
        .unwrap();
        assert!(!invalid.matches_definition(&schedule, full));
    }
    let single = Checkpoint::new(2, 1, 0, Some(minute), false, None).unwrap();
    assert!(single.matches_definition(&schedule, skip));
    let off_calendar = Checkpoint::new(2, 1, 0, Some(minute + 1), false, None).unwrap();
    assert!(!off_calendar.matches_definition(&schedule, skip));
}
#[test]
fn rejects_invalid_host_progress_without_repairing_it() {
    assert_eq!(
        Checkpoint::new(1, 1, 0, None, false, None),
        Err(CheckpointError::Version)
    );
    for (revision, through, selected, paused) in [
        (0, 0, None, false),
        (1, 10, Some(10), false),
        (1, 10, Some(9), false),
        (1, 10, Some(20), true),
    ] {
        assert_eq!(
            Checkpoint::new(2, revision, through, selected, paused, None),
            Err(CheckpointError::Progress)
        );
    }
    let checkpoint = Checkpoint::new(2, 9, 100, Some(200), false, None).unwrap();
    assert_eq!(
        (
            checkpoint.version(),
            checkpoint.revision(),
            checkpoint.through(),
            checkpoint.selected_at(),
            checkpoint.paused()
        ),
        (2, 9, 100, Some(200), false)
    );
    assert!(Checkpoint::new(2, 9, u64::MAX, None, true, None).is_ok());
}

#[test]
fn remaining_batch_must_match_progress_and_selection() {
    for pending in [&[][..], &[0][..], &[20, 20][..], &[30, 20][..], &[101][..]] {
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
            Checkpoint::new(2, 1, through, selected, paused, Some(batch.clone())),
            Err(CheckpointError::Progress)
        );
    }
    let armed = Checkpoint::new(2, 1, 10, Some(20), false, Some(batch.clone())).unwrap();
    assert_eq!(armed.catch_up(), Some(&batch));
    let paused = Checkpoint::new(2, 1, 10, None, true, Some(batch.clone())).unwrap();
    assert_eq!(paused.catch_up(), Some(&batch));
}
