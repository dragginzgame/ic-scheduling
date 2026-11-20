use super::*;

#[test]
fn outcomes_retry_attempts_and_skips_are_distinct() {
    let storage = MetricsStorage::default();
    let mut metrics = storage.snapshot();
    metrics.started.add(u128::MAX);
    storage.0.set(metrics);

    storage.record(Event::Started(1));
    storage.record(Event::Started(2));
    storage.record(Event::SkippedLate);
    for outcome in [
        Outcome::NoWork,
        Outcome::Success,
        Outcome::ExpectedFailure,
        Outcome::RetryableFailure,
    ] {
        storage.record(Event::Returned(outcome));
    }

    let metrics = storage.snapshot();
    assert!(metrics.started.total_overflowed());
    assert_eq!(metrics.started.total(), u128::MAX);
    for counter in [
        metrics.retry_attempts,
        metrics.skipped_late,
        metrics.no_work,
        metrics.succeeded,
        metrics.expected_failures,
        metrics.retryable_failures,
    ] {
        assert_eq!(counter.exact_total(), Ok(1));
    }
}

#[test]
fn host_storage_is_shared_and_cannot_be_replaced() {
    assert_eq!(metrics(), None);
    let storage = Rc::new(MetricsStorage::default());
    assert_eq!(initialize_with_metrics(Rc::clone(&storage)), Ok(()));
    ensure_storage();
    record(Event::Started(1));

    assert_eq!(initialize_with_metrics(Rc::clone(&storage)), Ok(()));
    assert_eq!(
        initialize_with_metrics(Rc::default()),
        Err(MetricsStorageConflict)
    );
    assert_eq!(metrics(), Some(storage.snapshot()));
    assert_eq!(storage.snapshot().started.exact_total(), Ok(1));
}
