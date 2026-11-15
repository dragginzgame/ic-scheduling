use super::*;
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
