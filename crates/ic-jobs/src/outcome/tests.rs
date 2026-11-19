use super::*;

#[test]
fn result_observation_is_validated_and_retained_without_replay_authority() {
    let attempt = Attempt::new(1, 10, 2).unwrap();

    for result in [
        RecordedResult::Returned(Outcome::NoWork),
        RecordedResult::Returned(Outcome::Success),
        RecordedResult::Returned(Outcome::ExpectedFailure),
        RecordedResult::Returned(Outcome::RetryableFailure),
        RecordedResult::Retired,
        RecordedResult::RetryAuthorized,
    ] {
        assert_eq!(
            RecordedOutcome::new(attempt, result, 9),
            Err(CheckpointError::Progress)
        );
        let record = RecordedOutcome::new(attempt, result, 20).unwrap();
        assert_eq!(
            (record.attempt(), record.result(), record.at()),
            (attempt, result, 20)
        );
        let checkpoint = crate::checkpoint::Checkpoint::new(
            7,
            2,
            Some(100),
            None,
            crate::state::ControlState::Paused,
            None,
            None,
        )
        .unwrap()
        .with_last_outcome(record)
        .unwrap();
        assert_eq!(checkpoint.last_outcome(), Some(record));
        let future = RecordedOutcome::new(Attempt::new(3, 0, 1).unwrap(), result, 20).unwrap();
        assert_eq!(
            checkpoint.with_last_outcome(future),
            Err(CheckpointError::Progress)
        );
    }
}
