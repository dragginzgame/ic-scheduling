//! Shared fixture encoding for bounded latest-result data, independent of products.
use ic_jobs::{Attempt, Checkpoint, Outcome, RecordedOutcome, RecordedResult};
pub(super) fn append(record: Option<RecordedOutcome>, bytes: &mut Vec<u8>) {
    let fields = record.map_or([0; 5], |record| {
        let kind = match record.result() {
            RecordedResult::Returned(Outcome::NoWork) => 1,
            RecordedResult::Returned(Outcome::Success) => 2,
            RecordedResult::Returned(Outcome::ExpectedFailure) => 3,
            RecordedResult::Returned(Outcome::RetryableFailure) => 4,
            RecordedResult::Retired => 5,
            RecordedResult::RetryAuthorized => 6,
        };
        [
            record.attempt().revision(),
            record.attempt().scheduled_at(),
            u64::from(record.attempt().number()),
            record.at(),
            kind,
        ]
    });
    for field in fields {
        bytes.extend(field.to_le_bytes());
    }
}
pub(super) fn restore(checkpoint: Checkpoint, bytes: &[u8]) -> Checkpoint {
    assert_eq!(bytes.len(), 40);
    let v: [u64; 5] =
        std::array::from_fn(|i| u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap()));
    if v[4] == 0 {
        assert_eq!(v, [0; 5]);
        return checkpoint;
    }
    let result = match v[4] {
        1 => RecordedResult::Returned(Outcome::NoWork),
        2 => RecordedResult::Returned(Outcome::Success),
        3 => RecordedResult::Returned(Outcome::ExpectedFailure),
        4 => RecordedResult::Returned(Outcome::RetryableFailure),
        5 => RecordedResult::Retired,
        6 => RecordedResult::RetryAuthorized,
        _ => panic!("invalid fixture result"),
    };
    checkpoint
        .with_last_outcome(
            RecordedOutcome::new(
                Attempt::new(v[0], v[1], u32::try_from(v[2]).unwrap()).unwrap(),
                result,
                v[3],
            )
            .unwrap(),
        )
        .unwrap()
}
