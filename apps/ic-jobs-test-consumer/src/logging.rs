//! Consumer-owned event admission and projection over the shared heap log engine.

use std::num::NonZeroUsize;

use crate::observations::Event;
use ic_jobs::Outcome;
use ic_logging::{EntryInput, Filter, Level, Limits, Log, PageRequest, TopicMatch};
use ic_metrics::MeasurementCounter;

use crate::api::{CounterMetric, Error, JobLogEntry, JobLogLevel, JobLogPage, JobLogRequest};

const MAX_ENTRIES: usize = 128;
const MAX_MESSAGE_BYTES: usize = 192;
const MAX_LABEL_BYTES: usize = 32;
const LIMITS: Limits = Limits {
    max_entries: NonZeroUsize::new(MAX_ENTRIES).unwrap(),
    max_message_bytes: MAX_MESSAGE_BYTES,
    max_label_bytes: MAX_LABEL_BYTES,
    // Preserve the previous worst-case retained text allowance. This adds no
    // earlier byte eviction for rows already admitted by the individual bounds.
    max_text_bytes: MAX_ENTRIES * (MAX_MESSAGE_BYTES + 2 * MAX_LABEL_BYTES),
};
const MAX_SCANNED: NonZeroUsize = NonZeroUsize::new(32).unwrap();

pub(crate) struct JobLogs {
    history: Log,
    omitted: MeasurementCounter,
}

impl JobLogs {
    pub(crate) const fn new() -> Self {
        Self {
            history: Log::new(LIMITS),
            omitted: MeasurementCounter::ZERO,
        }
    }

    // Only admitted transitions reach this method, after their durable commit.
    // Refusing a diagnostic must not change the transition or authorize a retry.
    pub(crate) fn record(&mut self, now_ns: u64, event: Event) {
        let (attempt, topic, level, detail) = match event {
            Event::Dispatch { attempt, retry } => {
                (attempt, "dispatch", Level::Info, format!("retry={retry}"))
            }
            Event::Outcome {
                attempt,
                outcome,
                reconciled,
            } => {
                let (level, outcome) = match outcome {
                    Outcome::Success => (Level::Ok, "success"),
                    Outcome::RetryableFailure => (Level::Warn, "retryable_failure"),
                    Outcome::PermanentFailure => (Level::Error, "permanent_failure"),
                    Outcome::Uncertain => (Level::Warn, "uncertain"),
                };
                (
                    attempt,
                    if reconciled {
                        "reconciliation"
                    } else {
                        "outcome"
                    },
                    level,
                    format!("outcome={outcome}"),
                )
            }
        };
        let message = format!(
            "job={} occurrence={} attempt={} {detail}",
            attempt.execution.job.0, attempt.execution.occurrence, attempt.sequence
        );
        // The successful AppendReceipt's sequence/truncation are already on the
        // retained row. Eviction is normal retention, not an omitted observation.
        // Every typed refusal, including TextBudgetExceeded, follows one path.
        if self
            .history
            .append(EntryInput {
                created_at_secs: now_ns / 1_000_000_000,
                level,
                source: "jobs-consumer",
                topic: Some(topic),
                message: &message,
            })
            .is_err()
        {
            self.omitted.add(1);
        }
    }

    pub(crate) fn page(&self, request: JobLogRequest, version: u64) -> Result<JobLogPage, Error> {
        if !(1..=8).contains(&request.limit) {
            return Err(Error::InvalidInput);
        }
        let max_results =
            NonZeroUsize::new(usize::from(request.limit)).ok_or(Error::InvalidInput)?;
        let page = self.history.page(
            Filter {
                min_level: request.min_level.map(engine_level),
                topic: TopicMatch::Any,
                ..Filter::default()
            },
            PageRequest {
                before: request.before,
                max_results,
                max_scanned: MAX_SCANNED,
            },
        );
        Ok(JobLogPage {
            canister_version: version,
            entries: page
                .entries
                .into_iter()
                .map(|entry| JobLogEntry {
                    sequence: entry.sequence,
                    created_at_secs: entry.created_at_secs,
                    level: report_level(entry.level),
                    topic: entry.topic.clone().unwrap_or_default(),
                    message: entry.message.clone(),
                    truncated: entry.truncated,
                })
                .collect(),
            scanned: page.scanned as u64,
            next_before: page.next_before,
            omitted: CounterMetric {
                total: self.omitted.total(),
                overflowed: self.omitted.total_overflowed(),
            },
        })
    }
}

fn engine_level(level: JobLogLevel) -> Level {
    match level {
        JobLogLevel::Debug => Level::Debug,
        JobLogLevel::Info => Level::Info,
        JobLogLevel::Ok => Level::Ok,
        JobLogLevel::Warn => Level::Warn,
        JobLogLevel::Error => Level::Error,
    }
}

fn report_level(level: Level) -> JobLogLevel {
    match level {
        Level::Debug => JobLogLevel::Debug,
        Level::Info => JobLogLevel::Info,
        Level::Ok => JobLogLevel::Ok,
        Level::Warn => JobLogLevel::Warn,
        Level::Error => JobLogLevel::Error,
    }
}

#[cfg(test)]
mod tests;
