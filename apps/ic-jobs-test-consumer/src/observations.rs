//! Commit-only coordination of the consumer's shared Metrics and Logging adapters.

use crate::{
    api::{Error, JobLogPage, JobLogRequest, JobMetrics},
    logging::JobLogs,
    metrics::{Counts, JobWindow},
};
use ic_jobs::{Attempt, Outcome};

pub(crate) enum Event {
    Dispatch {
        attempt: Attempt,
        retry: bool,
    },
    Outcome {
        attempt: Attempt,
        outcome: Outcome,
        reconciled: bool,
    },
}

pub(crate) struct Observations {
    metrics: JobWindow,
    logs: JobLogs,
}

impl Observations {
    pub(crate) fn new() -> Self {
        Self {
            metrics: JobWindow::new(Counts::EMPTY),
            logs: JobLogs::new(),
        }
    }

    pub(crate) fn metrics(&self, version: u64) -> JobMetrics {
        self.metrics
            .state()
            .snapshot(self.metrics.sequence(), version)
    }

    pub(crate) fn reset_metrics(&mut self, version: u64) -> JobMetrics {
        self.metrics.reset(Counts::EMPTY);
        self.metrics(version)
    }

    pub(crate) fn logs(&self, request: JobLogRequest, version: u64) -> Result<JobLogPage, Error> {
        self.logs.page(request, version)
    }

    // Called only after durable commit. Refused diagnostics never undo state or authorize retry.
    pub(crate) fn record(&mut self, now: u64, event: Event) {
        match &event {
            Event::Dispatch { retry, .. } => self.metrics.state_mut().dispatch(*retry),
            Event::Outcome {
                outcome,
                reconciled,
                ..
            } => {
                self.metrics.state_mut().outcome(*outcome, *reconciled);
            }
        }
        self.logs.record(now, event);
    }
}
