#[cfg(test)]
mod tests;

use super::Event;
use crate::Observation;
use ic_metrics::MeasurementCounter;
use std::cell::Cell;

///
/// Metrics
/// Aggregate diagnostics for this volatile provider incarnation. Initialization
/// does not reset these; a normal upgrade does. No declaration inventory or
/// async-lifetime instruction attribution is included.
///
/// Counters clip at u128::MAX with a sticky actual-overflow flag, using
/// ic-metrics arithmetic. An exactly reached cap remains exact. Recording
/// cannot wrap, trap on overflow or change timer eligibility.
///

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Metrics {
    /// Committed logical acceptances, including conservative accounting when
    /// a current initial delivery rejects. Not proof of business invocation or effects.
    pub accepted: MeasurementCounter,
    /// Callbacks that returned normally, regardless of successor disposition.
    pub completed: MeasurementCounter,
    /// Confirmed Once abandonment or rejected synchronous watchdog delivery.
    pub abandoned: MeasurementCounter,
    /// Normal returns classified ExpectedFailure by the consumer; no retry implied.
    pub expected_failures: MeasurementCounter,
    /// Deliveries deferred by submission refusal or a recognized pre-acceptance Once rejection.
    pub deferred_submissions: MeasurementCounter,
    /// First terminal provider fault in this incarnation; excludes isolated entry faults.
    pub provider_faults: MeasurementCounter,
}
///
/// MetricsStorage
/// Heap storage for one timer runtime's aggregate counters.
///
/// A host can retain this storage alongside its other metric sources and
/// supply it during timer initialization. Only Timers records into it.
///

#[derive(Default)]
pub struct MetricsStorage(Cell<Metrics>);

impl MetricsStorage {
    /// Read this storage without recording a new observation.
    #[must_use]
    pub fn snapshot(&self) -> Metrics {
        self.0.get()
    }

    pub(crate) fn record(&self, event: Event) {
        let mut counters = self.0.get();

        match event {
            Event::Accepted => counters.accepted.add(1),
            Event::Returned(outcome) => {
                counters.completed.add(1);
                if outcome == Observation::ExpectedFailure {
                    counters.expected_failures.add(1);
                }
            }
            Event::Abandoned => counters.abandoned.add(1),
            Event::Deferred => counters.deferred_submissions.add(1),
            Event::ProviderFault => counters.provider_faults.add(1),
        }

        self.0.set(counters);
    }
}
