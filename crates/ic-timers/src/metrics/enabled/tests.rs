use super::*;

#[test]
fn overflow_stays_diagnostic_and_outcomes_stay_separate() {
    let recorder = MetricsStorage::default();
    let mut counters = recorder.snapshot();
    counters.accepted.add(u128::MAX);
    assert!(!counters.accepted.total_overflowed());
    recorder.0.set(counters);
    recorder.record(Event::Accepted);
    recorder.record(Event::Returned(Observation::NoWork));
    recorder.record(Event::Returned(Observation::ExpectedFailure));
    recorder.record(Event::Abandoned);
    recorder.record(Event::Deferred);
    recorder.record(Event::ProviderFault);
    let counters = recorder.snapshot();
    assert_eq!(counters.accepted.total(), u128::MAX);
    assert!(counters.accepted.total_overflowed());
    assert_eq!(counters.completed.exact_total(), Ok(2));
    assert_eq!(counters.expected_failures.exact_total(), Ok(1));
    assert_eq!(counters.abandoned.exact_total(), Ok(1));
    assert_eq!(counters.deferred_submissions.exact_total(), Ok(1));
    assert_eq!(counters.provider_faults.exact_total(), Ok(1));
}
