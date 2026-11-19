use super::*;
use crate::{Checkpoint, ControlState};

fn progress(revision: u64, through: Option<u64>) -> Checkpoint {
    Checkpoint::new(7, revision, through, None, ControlState::Paused, None, None).unwrap()
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
        let record = progress(1, None).with_unresolved(attempt, phase).unwrap();
        assert_eq!(
            (
                record.version(),
                record.unresolved().unwrap().0,
                record.unresolved().unwrap().1
            ),
            (7, attempt, phase)
        );
        assert!(record.paused());
        assert_eq!(
            progress(1, Some(0)).with_unresolved(attempt, phase),
            Err(CheckpointError::Progress)
        );
        assert!(
            progress(2, Some(100))
                .with_unresolved(attempt, phase)
                .is_ok()
        );
        assert_eq!(
            progress(1, None).with_unresolved(Attempt::new(2, 0, 1).unwrap(), phase),
            Err(CheckpointError::Progress)
        );
    }
    for control in [ControlState::Active, ControlState::Cancelled] {
        let checkpoint = Checkpoint::new(7, 1, None, None, control, None, None).unwrap();
        assert_eq!(
            checkpoint.with_unresolved(attempt, UnresolvedPhase::Running),
            Err(CheckpointError::Progress)
        );
    }
    let retry = crate::retry::RetryProgress::new(0, 1, 10, 20).unwrap();
    let checkpoint =
        Checkpoint::new(7, 1, None, None, ControlState::Paused, None, Some(retry)).unwrap();
    assert_eq!(
        checkpoint.with_unresolved(attempt, UnresolvedPhase::Uncertain),
        Err(CheckpointError::Progress)
    );
}

#[test]
fn restore_checks_current_definition_budget_and_committed_batch() {
    use crate::{CatchUpLimit, CatchUpProgress, Policy, RetryPolicy, Schedule};
    let policy = Policy::CatchUp(CatchUpLimit::new(u64::MAX, 2).unwrap());
    let budget = RetryPolicy::new(3, 1, 2).unwrap();
    let attempt = Attempt::new(1, 0, 2).unwrap();
    let record = progress(1, None)
        .with_unresolved(attempt, UnresolvedPhase::Running)
        .unwrap();
    assert!(record.matches_definition(&Schedule::once(0), policy, budget));
    assert!(!record.matches_definition(&Schedule::once(1), policy, budget));
    assert!(!record.matches_definition(&Schedule::once(0), policy, RetryPolicy::NONE));
    let edited = progress(2, Some(100))
        .with_unresolved(attempt, UnresolvedPhase::Uncertain)
        .unwrap();
    assert!(edited.matches_definition(&Schedule::once(200), policy, budget));

    for (revision, pending, valid) in [
        (1, [0, 10], true),
        (1, [10, 20], false),
        (2, [0, 10], false),
    ] {
        let checkpoint = Checkpoint::new(
            7,
            revision,
            None,
            None,
            ControlState::Paused,
            Some(CatchUpProgress::new(&pending, 20).unwrap()),
            None,
        )
        .unwrap();
        let record = checkpoint.with_unresolved(attempt, UnresolvedPhase::Running);
        if valid {
            let record = record.unwrap();
            assert!(record.matches_definition(
                &Schedule::fixed_rate(0, std::num::NonZeroU64::new(10).unwrap()),
                policy,
                budget
            ));
            assert!(!record.matches_definition(&Schedule::once(0), policy, budget));
        } else {
            assert_eq!(record, Err(CheckpointError::Progress));
        }
    }
}
