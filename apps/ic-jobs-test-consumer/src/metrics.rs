//! Fixed consumer-owned counters. The queue publishes observations after commit.

use ic_jobs::Outcome;
use ic_metrics::{MeasurementCounter, MeasurementWindow};

use crate::api::{CounterMetric, JobMetrics};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Counts {
    dispatched: MeasurementCounter,
    retry_dispatches: MeasurementCounter,
    succeeded: MeasurementCounter,
    retryable_failures: MeasurementCounter,
    permanent_failures: MeasurementCounter,
    uncertain: MeasurementCounter,
    reconciliations: MeasurementCounter,
}

impl Counts {
    pub(crate) const EMPTY: Self = Self {
        dispatched: MeasurementCounter::ZERO,
        retry_dispatches: MeasurementCounter::ZERO,
        succeeded: MeasurementCounter::ZERO,
        retryable_failures: MeasurementCounter::ZERO,
        permanent_failures: MeasurementCounter::ZERO,
        uncertain: MeasurementCounter::ZERO,
        reconciliations: MeasurementCounter::ZERO,
    };

    pub(crate) fn dispatch(&mut self, retry: bool) {
        self.dispatched.add(1);
        if retry {
            self.retry_dispatches.add(1);
        }
    }

    // The caller supplies an outcome already admitted by Job::finish/resolve.
    pub(crate) fn outcome(&mut self, outcome: Outcome, reconciled: bool) {
        match outcome {
            Outcome::Success => &mut self.succeeded,
            Outcome::RetryableFailure => &mut self.retryable_failures,
            Outcome::PermanentFailure => &mut self.permanent_failures,
            Outcome::Uncertain => &mut self.uncertain,
        }
        .add(1);
        if reconciled {
            self.reconciliations.add(1);
        }
    }

    pub(crate) fn snapshot(&self, sequence: Option<u64>, canister_version: u64) -> JobMetrics {
        let view = |counter: MeasurementCounter| CounterMetric {
            total: counter.total(),
            overflowed: counter.total_overflowed(),
        };
        JobMetrics {
            canister_version,
            reset_sequence: sequence,
            dispatched: view(self.dispatched),
            retry_dispatches: view(self.retry_dispatches),
            succeeded: view(self.succeeded),
            retryable_failures: view(self.retryable_failures),
            permanent_failures: view(self.permanent_failures),
            uncertain: view(self.uncertain),
            reconciliations: view(self.reconciliations),
        }
    }
}

pub(crate) type JobWindow = MeasurementWindow<Counts>;

#[cfg(test)]
mod tests;
