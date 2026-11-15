use super::*;
fn progress(revision: u64, through: Option<u64>) -> Checkpoint {
    Checkpoint::new(6, revision, through, None, ControlState::Paused, None, None).unwrap()
}
#[test]
fn validates_unresolved_identity_and_owner_progress() {
    assert_eq!(Attempt::new(0, 0, 1), Err(CheckpointError::Progress));
    assert_eq!(Attempt::new(1, 0, 0), Err(CheckpointError::Progress));
    let attempt = Attempt::new(1, 0, 2).unwrap();
    assert_eq!(
        (attempt.revision(), attempt.scheduled_at(), attempt.number()),
        (1, 0, 2)
    );
    for phase in [UnresolvedPhase::Running, UnresolvedPhase::Uncertain] {
        let record = UnresolvedCheckpoint::new(1, progress(1, None), attempt, phase).unwrap();
        assert_eq!(
            (record.version(), record.attempt(), record.phase()),
            (1, attempt, phase)
        );
        assert!(record.progress().paused());
        assert_eq!(
            UnresolvedCheckpoint::new(0, progress(1, None), attempt, phase),
            Err(CheckpointError::Version)
        );
        assert!(UnresolvedCheckpoint::new(1, progress(1, Some(0)), attempt, phase).is_err());
        assert!(UnresolvedCheckpoint::new(1, progress(2, Some(100)), attempt, phase).is_ok());
        assert!(
            UnresolvedCheckpoint::new(1, progress(1, None), Attempt::new(2, 0, 1).unwrap(), phase)
                .is_err()
        );
    }
    for control in [ControlState::Active, ControlState::Cancelled] {
        let checkpoint = Checkpoint::new(6, 1, None, None, control, None, None).unwrap();
        assert!(
            UnresolvedCheckpoint::new(1, checkpoint, attempt, UnresolvedPhase::Running).is_err()
        );
    }
    let retry = crate::retry::RetryProgress::new(0, 1, 10, 20).unwrap();
    let checkpoint =
        Checkpoint::new(6, 1, None, None, ControlState::Paused, None, Some(retry)).unwrap();
    assert!(UnresolvedCheckpoint::new(1, checkpoint, attempt, UnresolvedPhase::Uncertain).is_err());
}
#[cfg(feature = "execution")]
#[test]
fn restore_checks_current_definition_budget_and_committed_batch() {
    use crate::{CatchUpLimit, CatchUpProgress, Policy, RetryPolicy, Schedule};
    let policy = Policy::CatchUp(CatchUpLimit::new(u64::MAX, 2).unwrap());
    let budget = RetryPolicy::new(3, 1, 2).unwrap();
    let attempt = Attempt::new(1, 0, 2).unwrap();
    let record =
        UnresolvedCheckpoint::new(1, progress(1, None), attempt, UnresolvedPhase::Running).unwrap();
    assert!(record.matches_definition(&Schedule::once(0), policy, budget));
    assert!(!record.matches_definition(&Schedule::once(1), policy, budget));
    assert!(!record.matches_definition(&Schedule::once(0), policy, RetryPolicy::NONE));
    let edited = UnresolvedCheckpoint::new(
        1,
        progress(2, Some(100)),
        attempt,
        UnresolvedPhase::Uncertain,
    )
    .unwrap();
    assert!(edited.matches_definition(&Schedule::once(200), policy, budget));
    for (revision, pending, valid) in [
        (1, [0, 10], true),
        (1, [10, 20], false),
        (2, [0, 10], false),
    ] {
        let checkpoint = Checkpoint::new(
            6,
            revision,
            None,
            None,
            ControlState::Paused,
            Some(CatchUpProgress::new(&pending, 20).unwrap()),
            None,
        )
        .unwrap();
        let record = UnresolvedCheckpoint::new(1, checkpoint, attempt, UnresolvedPhase::Running);
        assert_eq!(record.is_ok(), valid);
        if let Ok(record) = record {
            assert!(record.matches_definition(
                &Schedule::fixed_rate(0, std::num::NonZeroU64::new(10).unwrap()),
                policy,
                budget
            ));
            assert!(!record.matches_definition(&Schedule::once(0), policy, budget));
        }
    }
}
