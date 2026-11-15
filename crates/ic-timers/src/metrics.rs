//! Inert aggregate recording; no callbacks, allocation, clocks or scheduling authority.
use crate::Observation;

#[cfg_attr(
    not(feature = "metrics"),
    expect(dead_code, reason = "recording is opt-in")
)]
#[derive(Clone, Copy)]
pub(crate) enum Event {
    Accepted,
    Returned(Observation),
    Abandoned,
    Deferred,
    ProviderFault,
}

#[cfg(feature = "metrics")]
mod enabled {
    use super::Event;
    use crate::Observation;
    use ic_metrics::MeasurementCounter;
    use std::cell::Cell;

    /// Aggregate diagnostics for this volatile provider incarnation. Initialization
    /// does not reset these; a normal upgrade does. No declaration inventory or
    /// async-lifetime instruction attribution is included.
    ///
    /// Counters clip at u128::MAX with a sticky actual-overflow flag, using
    /// ic-metrics arithmetic. An exactly reached cap remains exact. Recording
    /// cannot wrap, trap on overflow or change timer eligibility.
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
        /// Self-call submissions refused before acceptance, each deferred submission counted.
        pub deferred_submissions: MeasurementCounter,
        /// First terminal provider fault in this incarnation; excludes isolated entry faults.
        pub provider_faults: MeasurementCounter,
    }
    pub(crate) struct Recorder(Cell<Metrics>);
    impl Recorder {
        pub(crate) fn new() -> Self {
            Self(Cell::new(Metrics::default()))
        }
        pub(crate) fn snapshot(&self) -> Metrics {
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
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn overflow_stays_diagnostic_and_outcomes_stay_separate() {
            let recorder = Recorder::new();
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
    }
}
#[cfg(feature = "metrics")]
pub use enabled::Metrics;
#[cfg(all(feature = "metrics", target_arch = "wasm32"))]
pub(crate) use enabled::Recorder;
