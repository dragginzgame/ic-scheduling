//! Wire records shared by unpublished canister probes and their native decoder.
//! This package owns no timer state, runtime authority or platform sampling.
//! Rebuild private probes and host decoders together: ordinary termination uses
//! an enum and a fresh method identity. Upgrade checkpoint field order is retained.
//! Next-work decisions share one typed method; the three former setters are retired.
//! The main snapshot also observes the volatile policy pair under a fresh method
//! identity; the separate startup-probe protocol is retired.

use serde::{Deserialize, Serialize};

pub mod rpc;

#[cfg(test)]
mod tests;

/// Stable field order for the runtime probe's existing upgrade checkpoint.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeCheckpoint {
    /// Primary policy's scheduling demand saved for reconstruction.
    pub desired_scheduled: bool,
    /// Secondary policy's scheduling demand saved for reconstruction.
    pub secondary_desired_scheduled: bool,
    /// Primary callbacks completed before checkpoint capture.
    pub completed_work: u64,
    /// Secondary callbacks completed before checkpoint capture.
    pub secondary_completed_work: u64,
}

/// Lossless wire projection of a diagnostic counter; arithmetic stays in Metrics.
#[derive(Serialize, Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CounterObservation {
    /// Accumulated counter value, preserving its full width.
    pub total: u128,
    /// Whether the source counter has saturated.
    pub total_overflowed: bool,
}

/// Flattened callback memory-page observations.
#[derive(Serialize, Debug, Deserialize, Eq, PartialEq)]
pub struct ProbeMemorySummary {
    /// Number of paired memory observations.
    pub samples: u64,
    /// Wasm pages at the start of the latest sample.
    pub latest_wasm_start_pages: u64,
    /// Wasm pages at the end of the latest sample.
    pub latest_wasm_end_pages: u64,
    /// Stable pages at the start of the latest sample.
    pub latest_stable_start_pages: u64,
    /// Stable pages at the end of the latest sample.
    pub latest_stable_end_pages: u64,
    /// Largest observed Wasm growth within a sample.
    pub maximum_wasm_growth_pages: u64,
    /// Largest observed stable growth within a sample.
    pub maximum_stable_growth_pages: u64,
}

/// Runtime recovery and upgrade observations, with absent timer fields empty.
#[derive(Serialize, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct ProbeSnapshot {
    /// Primary registration identity, when a snapshot is available.
    pub registration_id: Option<(u64, u64, u64)>,
    /// Whether the primary probe reports a registration.
    pub registered: bool,
    /// Completed primary application callbacks.
    pub completed_work: u64,
    /// Primary policy's next absolute deadline, in nanoseconds.
    pub next_deadline_ns: Option<u64>,
    /// Whether the current scheduling decision requests immediate work.
    pub immediate_scheduling: bool,
    /// Most recently requested provider delay, in nanoseconds.
    pub latest_requested_delay_ns: Option<u64>,
    /// Most recently armed provider delay, in nanoseconds.
    pub latest_armed_delay_ns: Option<u64>,
    /// Scheduling requests observed by the primary timer.
    pub schedule_requests: CounterObservation,
    /// Scheduler callbacks that started.
    pub scheduler_started: CounterObservation,
    /// Provider wakeups armed.
    pub wakeups_armed: CounterObservation,
    /// Work messages dispatched by the scheduler.
    pub work_dispatched: CounterObservation,
    /// Work callbacks that started.
    pub work_started: CounterObservation,
    /// Work callbacks that completed.
    pub work_completed: CounterObservation,
    /// Scheduling demand coalesced with existing work.
    pub coalesced: CounterObservation,
    /// Work attempts left unacknowledged.
    pub unacknowledged: CounterObservation,
    /// Whether the latest work attempt was unacknowledged.
    pub last_unacknowledged: bool,
    /// Number of scheduler instruction samples.
    pub scheduler_instruction_samples: u64,
    /// Total instructions in sampled scheduler callbacks.
    pub scheduler_instruction_total: u128,
    /// Number of work instruction samples.
    pub work_instruction_samples: u64,
    /// Total instructions in sampled work callbacks.
    pub work_instruction_total: u128,
    /// Paired scheduler memory observations, when sampled.
    pub scheduler_memory: Option<ProbeMemorySummary>,
    /// Paired work memory observations, when sampled.
    pub work_memory: Option<ProbeMemorySummary>,
    /// Whether the probe reconstructed its declarations after upgrade.
    pub post_upgrade_reconstructed: bool,
    /// Whether the secondary probe reports a registration.
    pub secondary_registered: bool,
    /// Completed secondary application callbacks.
    pub secondary_completed_work: u64,
    /// Latest primary callback completion time, in nanoseconds.
    pub completed_at_ns: Option<u64>,
    /// Whether the independent application timer reports a registration.
    pub application_registered: bool,
    /// Completed callbacks of the independent application timer.
    pub application_completed: CounterObservation,
    /// Latest independent application callback completion time, in nanoseconds.
    pub application_completed_at_ns: Option<u64>,
}

/// Mutually exclusive terminal reasons reported by ordinary recovery subjects.
/// Reported terminal reasons are mutually exclusive; other states have none.
#[derive(Serialize, Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum OrdinaryTermination {
    /// The runtime lost the suspended callback without explicit cancellation.
    Abandoned,
    /// The callback completed with a stop decision.
    Stopped,
    /// The owner explicitly cancelled the registration.
    Cancelled,
    /// Runtime validation refused the callback's control state.
    InvariantFailure,
}

/// Ordinary callback suspension, abandonment and capture-release observations.
#[derive(Serialize, Debug, Default, Deserialize)]
pub struct OrdinaryObservation {
    /// Registration identity reported by the ordinary timer.
    pub registration_id: Option<(u64, u64, u64)>,
    /// Whether the probe still owns its declaration.
    pub declared: bool,
    /// Whether the runtime reports active work.
    pub running: bool,
    /// Whether the callback is waiting on the probe's gate.
    pub waiting: bool,
    /// Successful replies received from the gate.
    pub gate_replies: u64,
    /// Latest gate call error, when present.
    pub gate_error: Option<String>,
    /// Ordinary application callbacks completed.
    pub completed: u64,
    /// Latest application completion time, in nanoseconds.
    pub completed_at_ns: Option<u64>,
    /// Next absolute deadline, in nanoseconds.
    pub next_deadline_ns: Option<u64>,
    /// Work callbacks that started.
    pub work_started: CounterObservation,
    /// Work callbacks that completed.
    pub work_completed: CounterObservation,
    /// Terminal reason reported by the current runtime snapshot.
    pub termination: Option<OrdinaryTermination>,
    /// Explicit cancellations observed by the timer.
    pub cancellations: CounterObservation,
    /// Whether the runtime reports an armed provider wakeup.
    pub armed: bool,
    /// Work attempts left unacknowledged.
    pub unacknowledged: CounterObservation,
    /// Number of instruction samples recorded for work.
    pub instruction_samples: u64,
    /// Callback-owned capture guards that have been dropped.
    pub captures_dropped: u64,
    /// Whether a dropped capture could access the registry without a borrow conflict.
    pub capture_registry_accessible: bool,
    /// Number of registrations in the runtime inventory.
    pub inventory_len: u64,
}

/// Provider replacement and cancellation churn observations.
#[derive(Serialize, Debug, Deserialize)]
pub struct ChurnObservation {
    /// Current Wasm memory extent, in pages.
    pub wasm_pages: u64,
    /// Number of retained runtime registrations.
    pub inventory_len: u64,
    /// Whether the churn subject still has an armed wakeup.
    pub armed: bool,
    /// Application callbacks completed by the churn subject.
    pub completed: u64,
}

/// One measured registration/control operation.
#[derive(Serialize, Debug, Deserialize)]
pub struct OperationMeasurement {
    /// Whether the measured control operation changed timer state.
    pub changed: bool,
    /// Instructions consumed by the operation's measured bracket.
    pub instructions: u64,
}

/// Instruction intervals for the probe's memory-sampling calibration.
#[derive(Serialize, Debug, Deserialize)]
pub struct MemorySamplingMeasurement {
    /// Instructions consumed by the empty calibration bracket.
    pub empty_bracket_instructions: u64,
    /// Instructions consumed by the bracket containing memory sampling.
    pub sampled_bracket_instructions: u64,
}

/// Policy-cohort observations with a borrowed producer or owned decoder label.
#[derive(Serialize, Debug, Default, Deserialize)]
pub struct CohortObservation<Label = String> {
    /// Policy cohort selected by the build driver.
    pub cohort: Label,
    /// Whether this cohort reports a timer registration.
    pub registered: bool,
    /// Application callbacks completed in the measured run.
    pub callbacks: u64,
    /// Next absolute policy deadline, in nanoseconds.
    pub next_deadline_ns: Option<u64>,
    /// Scheduling requests observed during the run.
    pub schedule_requests: CounterObservation,
    /// Provider wakeups armed during the run.
    pub wakeups_armed: CounterObservation,
    /// Work messages dispatched during the run.
    pub work_dispatched: CounterObservation,
    /// Scheduler callbacks that started.
    pub scheduler_started: CounterObservation,
    /// Work callbacks that started.
    pub work_started: CounterObservation,
    /// Work callbacks that completed.
    pub work_completed: CounterObservation,
    /// Scheduling requests coalesced with existing work.
    pub coalesced: CounterObservation,
    /// Number of scheduler instruction samples.
    pub scheduler_instruction_samples: u64,
    /// Total sampled scheduler instructions.
    pub scheduler_instruction_total: u128,
    /// Number of work instruction samples.
    pub work_instruction_samples: u64,
    /// Total sampled work instructions.
    pub work_instruction_total: u128,
    /// Instructions consumed by snapshot observation.
    pub snapshot_instructions: u64,
    /// Number of runtime registrations observed.
    pub inventory_len: u64,
    /// Instructions consumed by inventory observation.
    pub inventory_instructions: u64,
}

/// Per-callback counts for a maximum-capacity batch with suspended work.
#[derive(Serialize, Debug, Deserialize, Eq, PartialEq)]
pub struct DispatchBatchObservation {
    /// Start count for each callback slot.
    pub starts: Vec<u64>,
    /// Completion count for each callback slot.
    pub completions: Vec<u64>,
    /// Callbacks currently suspended in the batch.
    pub active: u64,
    /// Largest simultaneous callback count observed.
    pub peak_active: u64,
    /// Number of runtime registrations observed.
    pub inventory_len: u64,
}
