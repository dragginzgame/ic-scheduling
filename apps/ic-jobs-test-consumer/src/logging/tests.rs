use super::*;
use ic_jobs::{Attempt, ExecutionId, JobId};

fn event() -> Event {
    Event::Dispatch {
        attempt: Attempt {
            execution: ExecutionId {
                job: JobId(u128::MAX),
                occurrence: u64::MAX,
            },
            sequence: u64::MAX,
        },
        retry: true,
    }
}

fn request() -> JobLogRequest {
    JobLogRequest {
        before: None,
        min_level: None,
        limit: 8,
    }
}

#[test]
fn text_budget_preserves_full_count_retention_for_maximum_utf8_rows() {
    let mut logs = JobLogs::new();
    let source = "é".repeat(MAX_LABEL_BYTES / 2);
    let topic = "λ".repeat(MAX_LABEL_BYTES / 2);
    let message = "界".repeat(MAX_MESSAGE_BYTES / 3);
    let input = EntryInput {
        created_at_secs: 0,
        level: Level::Info,
        source: &source,
        topic: Some(&topic),
        message: &message,
    };
    for sequence in 0..MAX_ENTRIES {
        let receipt = logs.history.append(input).unwrap();
        assert_eq!(receipt.sequence, sequence as u64);
        assert!(!receipt.truncated);
        assert_eq!(receipt.evicted, 0);
    }
    assert_eq!(logs.history.retained_text_bytes(), LIMITS.max_text_bytes);
    let receipt = logs.history.append(input).unwrap();
    assert_eq!(receipt.sequence, MAX_ENTRIES as u64);
    assert!(!receipt.truncated);
    assert_eq!(receipt.evicted, 1);
    assert_eq!(logs.history.len(), MAX_ENTRIES);
    assert_eq!(logs.history.retained_text_bytes(), LIMITS.max_text_bytes);

    // The adapter ignores successful eviction receipts rather than reporting a
    // refused observation. Its next page still starts with the new event.
    logs.record(0, event());
    let page = logs.page(request(), 7).unwrap();
    assert_eq!(page.entries[0].sequence, MAX_ENTRIES as u64 + 1);
    assert_eq!(page.entries[0].topic, "dispatch");
    assert_eq!(page.omitted.total, 0);
    assert_eq!(logs.history.len(), MAX_ENTRIES);
    assert!(logs.history.retained_text_bytes() <= LIMITS.max_text_bytes);
}

#[test]
fn truncated_receipt_projects_the_retained_row_without_an_omission() {
    // Reduce only this test engine's message ceiling so a real formatted event
    // exercises the adapter's retained truncation flag. Production stays at 192.
    let mut logs = JobLogs {
        history: Log::new(Limits {
            max_message_bytes: 16,
            ..LIMITS
        }),
        omitted: MeasurementCounter::ZERO,
    };
    logs.record(0, event());
    let page = logs.page(request(), 7).unwrap();
    assert_eq!(page.entries.len(), 1);
    assert_eq!(page.entries[0].sequence, 0);
    assert!(page.entries[0].truncated);
    assert!(page.entries[0].message.len() <= 16);
    assert_eq!(page.omitted.total, 0);
}

#[test]
fn text_budget_refusals_keep_history_and_count_only_omissions() {
    // The production allowance admits every individually bounded row. Force
    // this new typed refusal on a private test engine without a runtime option.
    let mut logs = JobLogs {
        history: Log::new(Limits {
            max_text_bytes: 1,
            ..LIMITS
        }),
        omitted: MeasurementCounter::ZERO,
    };
    logs.history
        .append(EntryInput {
            created_at_secs: 0,
            level: Level::Info,
            source: "",
            topic: None,
            message: "x",
        })
        .unwrap();
    let before = logs.history.snapshot();
    assert!(matches!(
        logs.history.append(EntryInput {
            created_at_secs: 0,
            level: Level::Info,
            source: "jobs-consumer",
            topic: Some("dispatch"),
            message: "x",
        }),
        Err(ic_logging::AppendError::TextBudgetExceeded { .. })
    ));

    logs.record(0, event());
    assert_eq!(logs.history.snapshot(), before);
    let page = logs.page(request(), 7).unwrap();
    assert_eq!(page.entries.len(), 1); // Any topic still includes absent topics.
    assert_eq!(page.entries[0].message, "x");
    assert_eq!(page.omitted.total, 1);
    assert!(!page.omitted.overflowed);
}
