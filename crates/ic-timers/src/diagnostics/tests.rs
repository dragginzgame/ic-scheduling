use super::*;
#[test]
fn failure_streak_uses_canonical_overflow_and_normal_result_reset() {
    let mut measurements = TimerMeasurements::default();
    measurements.returned(Observation::ExpectedFailure);
    measurements.returned(Observation::ExpectedFailure);
    assert_eq!(
        measurements.consecutive_expected_failures.exact_total(),
        Ok(2)
    );
    measurements.abandoned.add(1);
    assert_eq!(
        measurements.consecutive_expected_failures.exact_total(),
        Ok(2)
    );
    measurements.consecutive_expected_failures.add(u128::MAX);
    assert!(
        measurements
            .consecutive_expected_failures
            .total_overflowed()
    );
    measurements.returned(Observation::NoWork);
    assert_eq!(
        measurements.consecutive_expected_failures.exact_total(),
        Ok(0)
    );
    assert_eq!(measurements.last_observation, Some(Observation::NoWork));
}
