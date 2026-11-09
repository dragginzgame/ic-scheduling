//! The private probes' current method identities and codec resource budgets.
//! Transport, IC callback contexts and traps remain with each probe/harness.

/// All control requests are small; this cap is checked before IC argument copying.
pub const REQUEST_LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 1024,
    allocation: 16 * 1024,
    nodes: 1024,
    depth: 16,
};

/// Snapshots retain the previous 64 KiB reply-byte ceiling with Wire budgets.
pub const RESPONSE_LIMITS: ic_wire::Limits = ic_wire::Limits {
    bytes: 64 * 1024,
    allocation: 1024 * 1024,
    nodes: 16 * 1024,
    depth: 32,
};

/// Ordinary callback policy selected by a probe request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OrdinaryPolicy {
    Once,
    AfterCompletion,
}

impl OrdinaryPolicy {
    pub const ALL: [Self; 2] = [Self::Once, Self::AfterCompletion];
}

/// Proposal consumed by the next successful watchdog work message.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WorkDecision {
    ContinueImmediately,
    ScheduleAt(u64),
    Stop,
}

/// All policies exercised by registration-failure and removal probes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProbePolicy {
    Once,
    AfterCompletion,
    Watchdog,
}

impl ProbePolicy {
    pub const ALL: [Self; 3] = [Self::Once, Self::AfterCompletion, Self::Watchdog];
}

/// Declaration retention selected by a probe request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProbeLifetime {
    Retained,
    RemoveWhenStopped,
}

impl ProbeLifetime {
    pub const ALL: [Self; 2] = [Self::Retained, Self::RemoveWhenStopped];
}

/// Owner reconciliation contracts exercised after confirmed scheduler abandonment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SchedulerReconcileMode {
    Scheduled,
    ScheduledImmediately,
    ScheduledAt,
}

impl SchedulerReconcileMode {
    pub const ALL: [Self; 3] = [
        Self::Scheduled,
        Self::ScheduledImmediately,
        Self::ScheduledAt,
    ];
}

/// Frame identity for the four-field fixture checkpoint; not an RPC endpoint.
pub const CHECKPOINT_ID: u64 = 0x5449_4d45_5253_4350;

// One roster owns the private endpoint names and frame IDs on both sides.
macro_rules! methods {
    ($($variant:ident = $id:literal => $name:literal,)*) => {
        /// Current private probe method identities; not timer-runtime API.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(u64)]
        pub enum Method {
            $(#[doc = $name] $variant = $id,)*
        }

        impl Method {
            /// Endpoint name paired with this method's frame identity.
            pub const fn name(self) -> &'static str {
                match self {
                    $(Self::$variant => $name,)*
                }
            }
        }
    };
}

methods! {
    UsePreparedCalls = 70 => "use_prepared_calls",
    ReinitializeRuntime = 71 => "reinitialize_runtime",
    SetWorkDecision = 66 => "set_work_decision",
    CohortStart = 58 => "cohort_start",
    ArmExhaustion = 1 => "arm_exhaustion",
    ArmTrap = 2 => "arm_trap",
    BurnToBelowTimerCallCost = 3 => "burn_to_below_timer_call_cost",
    Cancel = 4 => "cancel",
    CancelOrdinary = 5 => "cancel_ordinary",
    ChangePendingDispatchBatch = 6 => "change_pending_dispatch_batch",
    ChurnObservation = 7 => "churn_observation",
    ClearOrdinaryTrap = 8 => "clear_ordinary_trap",
    DiscardExpiredOrdinaryClaim = 10 => "discard_expired_ordinary_claim",
    DispatchBatchGate = 11 => "dispatch_batch_gate",
    DispatchBatchObservation = 12 => "dispatch_batch_observation",
    EnsureAgain = 13 => "ensure_again",
    EnsureFromReturnedError = 14 => "ensure_from_returned_error",
    ExpiredOrdinaryContextRejected = 15 => "expired_ordinary_context_rejected",
    FillInventory = 16 => "fill_inventory",
    FixtureDispatchPayload = 17 => "fixture_dispatch_payload",
    FixtureForeignDispatch = 59 => "fixture_foreign_dispatch",
    FixtureGlobalTimerSet = 60 => "fixture_global_timer_set",
    InventoryMeasurement = 18 => "inventory_measurement",
    MemorySamplingOverhead = 19 => "memory_sampling_overhead",
    Observe = 20 => "observe",
    OrdinaryGate = 21 => "ordinary_gate",
    // The enum observation layout replaces the retired four-boolean layout (22).
    OrdinaryObservation = 65 => "ordinary_observation",
    PressureAndStart = 23 => "pressure_and_start",
    PressureObservation = 24 => "pressure_observation",
    PressureReply = 25 => "pressure_reply",
    RearmCancelledDispatchBatchTimer = 26 => "rearm_cancelled_dispatch_batch_timer",
    ReconcileOrdinaryAt = 27 => "reconcile_ordinary_at",
    ReconcileOrdinaryInactive = 64 => "reconcile_ordinary_inactive",
    ReconcileSchedulerPressure = 63 => "reconcile_scheduler_pressure",
    RejectInitialReconciliation = 28 => "reject_initial_reconciliation",
    RejectInitialSchedule = 29 => "reject_initial_schedule",
    ReleaseDispatchBatch = 30 => "release_dispatch_batch",
    ReleaseOrdinaryWork = 31 => "release_ordinary_work",
    ReleasePressure = 32 => "release_pressure",
    RemoveArmedCallback = 33 => "remove_armed_callback",
    ReplaceDistantDeadlines = 34 => "replace_distant_deadlines",
    ReplaceRegistration = 35 => "replace_registration",
    ScheduleAt = 36 => "schedule_at",
    SchedulerPressureCompleted = 62 => "scheduler_pressure_completed",
    SchedulingTrace = 38 => "scheduling_trace",
    // Snapshot 39 and startup-probe methods 47..51 are retired.
    Snapshot = 69 => "snapshot",
    StartVolatilePair = 67 => "start_volatile_pair",
    StopVolatilePair = 68 => "stop_volatile_pair",
    Start = 40 => "start",
    StartDispatchBatch = 41 => "start_dispatch_batch",
    StartImmediateChurn = 42 => "start_immediate_churn",
    StartImmediately = 43 => "start_immediately",
    StartOrdinary = 44 => "start_ordinary",
    StartSchedulerPressure = 61 => "start_scheduler_pressure",
    StartSchedulingTrace = 45 => "start_scheduling_trace",
    StartSecondary = 46 => "start_secondary",
    Stop = 52 => "stop",
    TrapOrdinaryCaptureDrop = 54 => "trap_ordinary_capture_drop",
    TrapOrdinaryWork = 55 => "trap_ordinary_work",
    UnregisterOrdinary = 56 => "unregister_ordinary",
    UseRepresentativeWatchdogWork = 57 => "use_representative_watchdog_work",
}

/// Bind unpublished probe functions to this protocol's methods and budgets.
///
/// Each `#[endpoint(update, Method)]` or `#[endpoint(query, Method)]` declaration
/// retains its function body and static export name. Wire owns decoding/export
/// generation; callers depend on ic-wire-endpoint. No runtime registry is added.
#[macro_export]
macro_rules! probe_endpoints {
    ($(#[endpoint($kind:ident, $method:ident)] $endpoint:item)+) => {
        $(
            #[ic_wire_endpoint::$kind(
                id = $crate::rpc::Method::$method as u64,
                request = $crate::rpc::REQUEST_LIMITS,
                response = $crate::rpc::RESPONSE_LIMITS
            )]
            $endpoint
        )+
    };
}
