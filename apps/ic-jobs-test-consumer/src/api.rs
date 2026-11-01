//! Consumer-owned DTOs for Wire RPCs and bounded Wire snapshots.
//! JobRecord remains an internal stored value.

use ic_platform_types::Principal;
use serde::{Deserialize, Serialize};

/// Installation configuration, retained across upgrades.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Init {
    /// Identity permitted to manage tasks and fixture controls.
    pub manager: Principal,
    /// Optional sender permitted to write this canister's delivery receipts.
    pub delivery_sender: Option<Principal>,
}

/// Immutable delivery payload shared by retained tasks and receipt envelopes.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeliveryPayload {
    /// Independent receipt canister.
    pub destination: Principal,
    /// Immutable UTF-8 payload, at most 256 bytes.
    pub message: String,
}

/// Immutable effect payload.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Task {
    /// A local effect committed atomically with its job completion.
    Increment(u64),
    /// Delivery to the independent consumer receipt endpoint.
    Deliver(DeliveryPayload),
}

/// Consumer's supported scheduling policies.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Timing {
    /// One occurrence.
    Once(u64),
    /// Fixed rate with bounded catch-up or skip behavior.
    FixedRate {
        /// Original first deadline.
        first_ns: u64,
        /// Positive interval.
        every_ns: u64,
        /// Catch up two occurrences per batch; otherwise skip missed intervals.
        catch_up: bool,
    },
    /// Wait after each successful completion.
    AfterCompletion {
        /// First occurrence's deadline.
        first_ns: u64,
        /// Positive delay after completion.
        every_ns: u64,
    },
}

/// Task creation request. Identities are allocated by the retained queue.
#[derive(Debug, Deserialize, Serialize)]
pub struct Create {
    /// Scheduling policy.
    pub timing: Timing,
    /// Immutable execution payload.
    pub task: Task,
}

/// Projection of the checked job state.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Status {
    /// Pending work with a durable deadline.
    Pending {
        /// Pending dispatch deadline, including retry backoff.
        due_ns: u64,
    },
    /// Suspended pending work, with no armed dispatch deadline.
    Paused,
    /// Outstanding committed attempt, never replayed during restore.
    Running,
    /// Unknown effect, blocked until receipt reconciliation.
    Uncertain,
    /// Successful one-shot completion.
    Completed,
    /// Terminal failure.
    Failed,
    /// Cancelled pending work.
    Cancelled,
}

/// Authorized inspection response.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct View {
    /// Consumer-allocated job identity.
    pub id: u64,
    /// Retained payload.
    pub task: Task,
    /// Checked state projection.
    pub status: Status,
    /// Exact allocated dispatch sequence.
    pub sequence: u64,
    /// Recurring occurrence sequence.
    pub occurrence: u64,
}

/// Exact delivery identity and payload. Correlation is not authentication.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Delivery {
    /// Job identity within the authenticated sending canister.
    pub job: u64,
    /// Logical occurrence, shared by safe retries.
    pub occurrence: u64,
    /// Exact dispatch sequence.
    pub sequence: u64,
    /// Immutable destination and bounded message.
    pub payload: DeliveryPayload,
}

/// Bounded state summary used by the host harness.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Summary {
    /// Earliest pending deadline over the entire bounded queue.
    pub next_due_ns: Option<u64>,
    /// Total local counter effects.
    pub counter: u64,
    /// Number of independently retained logical delivery receipts.
    pub receipts: u64,
    /// Whether the volatile watchdog currently owns an armed wakeup.
    pub armed: bool,
}

/// One wide diagnostic counter, including actual-overflow quality.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct CounterMetric {
    /// Total observations; a saturated value is exact only while overflowed is false.
    pub total: u128,
    /// At least one observation was lost to actual numeric overflow.
    pub overflowed: bool,
}

/// Consumer-owned heap diagnostics; never durable scheduling or retry authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct JobMetrics {
    /// Host-supplied IC version; compare only within the same canister/version.
    pub canister_version: u64,
    /// Heap-local reset sequence, unavailable after exhaustion; not a durable epoch.
    pub reset_sequence: Option<u64>,
    /// Successfully committed dispatch attempts, including local work and retries.
    pub dispatched: CounterMetric,
    /// Dispatches after the first attempt of the same occurrence.
    pub retry_dispatches: CounterMetric,
    /// Committed success outcomes, including receipt-confirmed success.
    pub succeeded: CounterMetric,
    /// Committed outcomes whose owner explicitly established retry safety.
    pub retryable_failures: CounterMetric,
    /// Committed permanent-failure outcomes.
    pub permanent_failures: CounterMetric,
    /// Committed uncertain outcomes, including unresolved reconciliation observations.
    pub uncertain: CounterMetric,
    /// Committed explicit resolutions of previously uncertain effects.
    pub reconciliations: CounterMetric,
}

/// Ordered severity for the consumer's protected diagnostic history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum JobLogLevel {
    /// Development detail.
    Debug,
    /// Dispatch observation.
    Info,
    /// Successful outcome.
    Ok,
    /// Retryable or uncertain outcome.
    Warn,
    /// Permanent failure.
    Error,
}

/// One bounded newest-first scan; keep the same filter across cursor pages.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct JobLogRequest {
    /// Exclusive heap-local sequence cursor; absent starts at the newest event.
    pub before: Option<u64>,
    /// Include this severity and higher when present.
    pub min_level: Option<JobLogLevel>,
    /// Matching rows to return, from one through eight.
    pub limit: u8,
}

/// Redacted observation of a committed transition, never proof of a remote effect.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct JobLogEntry {
    /// Monotonic identity within this volatile history.
    pub sequence: u64,
    /// Whole IC seconds supplied by the transition owner.
    pub created_at_secs: u64,
    /// Observed severity.
    pub level: JobLogLevel,
    /// Fixed consumer topic: dispatch, outcome or reconciliation.
    pub topic: String,
    /// Bounded job/occurrence/attempt context; excludes principals and payloads.
    pub message: String,
    /// Whether the engine truncated the original message.
    pub truncated: bool,
}

/// Manager-only heap history, independently bounded from durable jobs and metrics.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct JobLogPage {
    /// Host-supplied IC version; history is discarded on reconstruction.
    pub canister_version: u64,
    /// Newest matching observations first, at most eight.
    pub entries: Vec<JobLogEntry>,
    /// Rows examined, at most 32; not the total number of matches.
    pub scanned: u64,
    /// Continue even after an empty filtered page; absent means the scan is done.
    pub next_before: Option<u64>,
    /// Refused appends, including timestamp regression; excludes count eviction.
    pub omitted: CounterMetric,
}

/// Refused management, storage or execution boundary.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Error {
    /// Caller lacks management or receiver authority.
    Unauthorized,
    /// Job identity was not retained.
    UnknownJob,
    /// Bounded history or receipts are full.
    Capacity,
    /// Payload, page or arithmetic input is invalid.
    InvalidInput,
    /// Stored bytes contradict the current consumer contract.
    InvalidStorage,
    /// Canonical Jobs transition refused the operation.
    Job(String),
    /// Allocation bootstrap or opening failed.
    Memory(String),
    /// Receipt does not identify this immutable delivery.
    ReceiptMismatch,
}
