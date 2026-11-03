//! Current management and installation Wire identities for this unpublished fixture.
//! Rebuild the canister and harness together; old Candid requests are refused.

use ic_wire::Limits;

// One declaration owns the management ID and its exported-name projection.
macro_rules! methods {
    ($($(#[$doc:meta])* $variant:ident = $id:literal => $name:literal,)*) => {
        /// Stable identities; Rust DTO declaration order is part of each native Serde contract.
        #[derive(Clone, Copy, Debug)]
        #[repr(u64)]
        pub enum Method { $($(#[$doc])* $variant = $id,)* }
        impl Method {
            /// Exact exported name; installation uses `init` only as a codec identity.
            #[must_use]
            pub const fn name(self) -> &'static str { match self { $(Self::$variant => $name,)* } }
        }
    };
}

methods! {
    /// Installation configuration.
    Init = 0x4a4f42534d470001 => "init",
    /// Create a job.
    Create = 0x4a4f42534d470002 => "create",
    /// Inspect a job.
    Inspect = 0x4a4f42534d470003 => "inspect",
    /// Page through jobs.
    List = 0x4a4f42534d470004 => "list",
    /// Cancel pending work.
    Cancel = 0x4a4f42534d470005 => "cancel",
    /// Pause pending work.
    Pause = 0x4a4f42534d470006 => "pause",
    /// Resume paused work.
    Resume = 0x4a4f42534d470007 => "resume",
    /// Replace an unstarted schedule.
    Reschedule = 0x4a4f42534d470008 => "reschedule",
    /// Read the bounded queue summary.
    Summary = 0x4a4f42534d470009 => "summary",
    /// Dispatch one bounded batch.
    DispatchDue = 0x4a4f42534d47000a => "dispatch_due",
    /// Reconcile an outstanding delivery.
    ReconcileDelivery = 0x4a4f42534d47000b => "reconcile_delivery",
    /// Control the deliberate watchdog trap.
    TrapWatchdogBatch = 0x4a4f42534d47000c => "fixture_trap_watchdog_batch",
    /// Trap after creating work.
    TrapAfterCreate = 0x4a4f42534d47000d => "fixture_trap_after_create",
    /// Trap after pausing work.
    TrapAfterPause = 0x4a4f42534d47000e => "fixture_trap_after_pause",
    /// Trap after replacing a schedule.
    TrapAfterReschedule = 0x4a4f42534d47000f => "fixture_trap_after_reschedule",
    /// Corrupt a retained record deliberately.
    CorruptRecord = 0x4a4f42534d470010 => "fixture_corrupt_record",
    /// Mark a retained attempt uncertain.
    MarkUncertain = 0x4a4f42534d470011 => "fixture_mark_uncertain",
    /// Trap after confirmed delivery.
    DispatchTrapResult = 0x4a4f42534d470012 => "fixture_dispatch_trap_result",
    /// Allocate intent without delivery.
    CreateBlocked = 0x4a4f42534d470013 => "fixture_create_blocked",
    /// Inspect consumer-owned heap job observations.
    JobMetrics = 0x4a4f42534d470014 => "job_metrics",
    /// Reset diagnostics without changing durable job state.
    ResetJobMetrics = 0x4a4f42534d470015 => "reset_job_metrics",
    /// Scan consumer-owned heap transition history.
    JobLogs = 0x4a4f42534d470016 => "job_logs",
}

/// Complete request frame and traversal allowances, including installation.
pub const REQUEST_LIMITS: Limits = Limits {
    bytes: 4 * 1024,
    allocation: 32 * 1024,
    nodes: 256,
    depth: 16,
};
/// Complete response allowance for eight bounded views and diagnostic errors.
pub const RESPONSE_LIMITS: Limits = Limits {
    bytes: 64 * 1024,
    allocation: 256 * 1024,
    nodes: 4096,
    depth: 32,
};
