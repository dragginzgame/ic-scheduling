//! Wire records shared by unpublished canister probes and their native decoder.
//! This package owns no timer state, runtime authority or platform sampling.

use serde::{Deserialize, Serialize};

pub mod rpc;

#[cfg(test)]
mod tests;

/// Lossless wire projection of a diagnostic counter; arithmetic stays in Metrics.
#[derive(Serialize, Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CounterObservation {
    pub total: u128,
    pub total_overflowed: bool,
}

/// Flattened callback memory-page observations.
#[derive(Serialize, Debug, Deserialize, Eq, PartialEq)]
pub struct ProbeMemorySummary {
    pub samples: u64,
    pub latest_wasm_start_pages: u64,
    pub latest_wasm_end_pages: u64,
    pub latest_stable_start_pages: u64,
    pub latest_stable_end_pages: u64,
    pub maximum_wasm_growth_pages: u64,
    pub maximum_stable_growth_pages: u64,
}

/// Runtime recovery and upgrade observations, with absent timer fields empty.
#[derive(Serialize, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct ProbeSnapshot {
    pub registration_id: Option<(u64, u64, u64)>,
    pub registered: bool,
    pub completed_work: u64,
    pub next_deadline_ns: Option<u64>,
    pub immediate_scheduling: bool,
    pub latest_requested_delay_ns: Option<u64>,
    pub latest_armed_delay_ns: Option<u64>,
    pub schedule_requests: CounterObservation,
    pub scheduler_started: CounterObservation,
    pub wakeups_armed: CounterObservation,
    pub work_dispatched: CounterObservation,
    pub work_started: CounterObservation,
    pub work_completed: CounterObservation,
    pub coalesced: CounterObservation,
    pub unacknowledged: CounterObservation,
    pub last_unacknowledged: bool,
    pub scheduler_instruction_samples: u64,
    pub scheduler_instruction_total: u128,
    pub work_instruction_samples: u64,
    pub work_instruction_total: u128,
    pub scheduler_memory: Option<ProbeMemorySummary>,
    pub work_memory: Option<ProbeMemorySummary>,
    pub post_upgrade_reconstructed: bool,
    pub secondary_registered: bool,
    pub secondary_completed_work: u64,
}

/// Ordinary callback suspension, abandonment and capture-release observations.
#[derive(Serialize, Debug, Deserialize)]
pub struct OrdinaryObservation {
    pub registration_id: Option<(u64, u64, u64)>,
    pub declared: bool,
    pub running: bool,
    pub waiting: bool,
    pub gate_replies: u64,
    pub gate_error: Option<String>,
    pub completed: u64,
    pub completed_at_ns: Option<u64>,
    pub next_deadline_ns: Option<u64>,
    pub work_started: CounterObservation,
    pub work_completed: CounterObservation,
    pub abandoned: bool,
    pub stopped: bool,
    pub cancelled: bool,
    pub cancellations: CounterObservation,
    pub invariant_failure: bool,
    pub armed: bool,
    pub unacknowledged: CounterObservation,
    pub instruction_samples: u64,
    pub captures_dropped: u64,
    pub capture_registry_accessible: bool,
    pub inventory_len: u64,
}

/// Provider replacement and cancellation churn observations.
#[derive(Serialize, Debug, Deserialize)]
pub struct ChurnObservation {
    pub wasm_pages: u64,
    pub inventory_len: u64,
    pub armed: bool,
    pub completed: u64,
}

/// One measured registration/control operation.
#[derive(Serialize, Debug, Deserialize)]
pub struct OperationMeasurement {
    pub changed: bool,
    pub instructions: u64,
}

/// Instruction intervals for the probe's memory-sampling calibration.
#[derive(Serialize, Debug, Deserialize)]
pub struct MemorySamplingMeasurement {
    pub empty_bracket_instructions: u64,
    pub sampled_bracket_instructions: u64,
}

/// Policy-cohort observations with a borrowed producer or owned decoder label.
#[derive(Serialize, Debug, Default, Deserialize)]
pub struct CohortObservation<Label = String> {
    pub cohort: Label,
    pub registered: bool,
    pub callbacks: u64,
    pub next_deadline_ns: Option<u64>,
    pub schedule_requests: CounterObservation,
    pub wakeups_armed: CounterObservation,
    pub work_dispatched: CounterObservation,
    pub scheduler_started: CounterObservation,
    pub work_started: CounterObservation,
    pub work_completed: CounterObservation,
    pub coalesced: CounterObservation,
    pub scheduler_instruction_samples: u64,
    pub scheduler_instruction_total: u128,
    pub work_instruction_samples: u64,
    pub work_instruction_total: u128,
    pub snapshot_instructions: u64,
    pub inventory_len: u64,
    pub inventory_instructions: u64,
}

/// Mixed Watchdog/after-completion isolation and volatile upgrade observations.
#[derive(Serialize, Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct TimerProbeSnapshot {
    pub watchdog_registered: bool,
    pub application_registered: bool,
    pub watchdog_callbacks: u32,
    pub application_callbacks: u32,
    pub watchdog_times: Vec<u64>,
    pub application_times: Vec<u64>,
    pub trap_at: Option<u64>,
    pub exhaust_at: Option<u64>,
}

/// Per-callback counts for a maximum-capacity batch with suspended work.
#[derive(Serialize, Debug, Deserialize, Eq, PartialEq)]
pub struct DispatchBatchObservation {
    pub starts: Vec<u64>,
    pub completions: Vec<u64>,
    pub active: u64,
    pub peak_active: u64,
    pub inventory_len: u64,
}
