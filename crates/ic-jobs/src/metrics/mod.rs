//! Job-level diagnostics for one canister incarnation.

#[cfg(test)]
mod tests;

use crate::Outcome;
use ic_metrics::MeasurementCounter;
use std::{
    cell::{Cell, OnceCell},
    rc::Rc,
};

///
/// Metrics
/// Committed job observations across all bindings in this canister incarnation.
///

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Metrics {
    /// Attempts admitted to the business callback. An initial trap can roll back the start.
    pub started: MeasurementCounter,
    /// Started attempts numbered two or higher, also included in `started`.
    pub retry_attempts: MeasurementCounter,
    /// Selected dispatches suppressed by the delivery policy's lateness or age limit.
    pub skipped_late: MeasurementCounter,
    /// Normal returns classified as no business work.
    pub no_work: MeasurementCounter,
    /// Normal returns classified as business success.
    pub succeeded: MeasurementCounter,
    /// Normal returns reporting an expected, settled failure.
    pub expected_failures: MeasurementCounter,
    /// Normal returns permitting a retry, whether the policy has remaining budget or not.
    pub retryable_failures: MeasurementCounter,
}

///
/// MetricsStorage
/// Heap storage for aggregate job counters. Hosts can retain it in their metrics store.
///

#[derive(Default)]
pub struct MetricsStorage(Cell<Metrics>);

impl MetricsStorage {
    /// Copy the current counters without recording an observation.
    #[must_use]
    pub fn snapshot(&self) -> Metrics {
        self.0.get()
    }

    fn record(&self, event: Event) {
        let mut metrics = self.0.get();

        match event {
            Event::Started(attempt) => {
                metrics.started.add(1);
                if attempt > 1 {
                    metrics.retry_attempts.add(1);
                }
            }
            Event::SkippedLate => metrics.skipped_late.add(1),
            Event::Returned(Outcome::NoWork) => metrics.no_work.add(1),
            Event::Returned(Outcome::Success) => metrics.succeeded.add(1),
            Event::Returned(Outcome::ExpectedFailure) => metrics.expected_failures.add(1),
            Event::Returned(Outcome::RetryableFailure) => metrics.retryable_failures.add(1),
        }

        self.0.set(metrics);
    }
}

///
/// MetricsStorageConflict
/// The current incarnation already uses a different Jobs metrics store.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MetricsStorageConflict;

impl std::error::Error for MetricsStorageConflict {}

impl std::fmt::Display for MetricsStorageConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Jobs metrics storage is already initialized")
    }
}

///
/// Event
/// Committed execution observations at the callback and delivery-policy boundaries.
///

#[derive(Clone, Copy)]
pub(crate) enum Event {
    Started(u32),
    SkippedLate,
    Returned(Outcome),
}

thread_local! {
    static STORAGE: OnceCell<Rc<MetricsStorage>> = const { OnceCell::new() };
}

/// Attach host-owned Jobs metrics storage before binding or restoring jobs.
///
/// This selects the diagnostic store only. Timers still requires its own
/// initialization. A normal upgrade starts a fresh heap incarnation.
///
/// # Errors
/// Returns a conflict if a different store already owns this incarnation's counters.
pub fn initialize_with_metrics(storage: Rc<MetricsStorage>) -> Result<(), MetricsStorageConflict> {
    STORAGE.with(|slot| {
        if slot
            .get()
            .is_some_and(|current| Rc::ptr_eq(current, &storage))
        {
            return Ok(());
        }

        slot.set(storage).map_err(|_| MetricsStorageConflict)
    })
}

/// Read the aggregate Jobs counters, if a host attached a store or a job was bound.
#[must_use]
pub fn metrics() -> Option<Metrics> {
    STORAGE.with(|slot| slot.get().map(|storage| storage.snapshot()))
}

pub(crate) fn ensure_storage() {
    STORAGE.with(|slot| {
        slot.get_or_init(Rc::default);
    });
}

pub(crate) fn record(event: Event) {
    STORAGE.with(|slot| {
        if let Some(storage) = slot.get() {
            storage.record(event);
        }
    });
}
