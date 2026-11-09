use super::*;

#[test]
fn reports_preserve_wide_totals_and_actual_overflow_quality() {
    let mut exact_cap = MeasurementCounter::ZERO;
    exact_cap.add(u128::MAX);
    let mut overflowed = exact_cap;
    overflowed.add(1);
    let counts = Counts {
        dispatched: exact_cap,
        succeeded: overflowed,
        ..Counts::EMPTY
    };
    let report = counts.snapshot(None, 7);
    assert_eq!(
        report.dispatched,
        CounterMetric {
            total: u128::MAX,
            overflowed: false
        }
    );
    assert_eq!(
        report.succeeded,
        CounterMetric {
            total: u128::MAX,
            overflowed: true
        }
    );
    assert_eq!(report.reset_sequence, None);
    assert_eq!(report.canister_version, 7);
}
