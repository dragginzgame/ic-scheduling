//! Observable and recovery-aware timer primitives for Internet Computer
//! canisters.
//!
//! The private one-shot backend owns IC timer dispatch and cancellation. The
//! Platform executor owns async polling and call-context cleanup; this crate
//! owns bounded coordination, scheduling, and observation.
//!
//! The runtime executes `Once`, after-completion, and pre-armed synchronous
//! watchdog callbacks through one volatile canister-local registry.
//! Consumers retain durable application authority and synchronously reconstruct
//! retained registrations during their existing lifecycle hooks. Transient
//! remove-on-stop callbacks use direct registration with an initial schedule.
//! Direct registration returns scheduled authority only after provider binding;
//! only retained declarations may start inactive. [`TimerStart`] selects
//! retention together with the initial schedule. Registration capabilities can
//! observe exact armed provider-wakeup ownership without making snapshots
//! or observations into scheduling authority.
//! [`timer_inventory`] returns the epoch together with the complete ordered
//! timer set, including for an initialized empty registry.
//!
//! Direct creation uses [`register_once`], [`register_after_completion`] or
//! [`register_watchdog`]. Retained lifecycle slots use [`reconcile_once`],
//! [`reconcile_after_completion`] or [`reconcile_watchdog`]. Consumer custody
//! checks must account for all six capabilities, including renamed imports,
//! function values and macro-contained calls.

#![deny(unsafe_code)]
#![forbid(private_interfaces)]
#![deny(rustdoc::broken_intra_doc_links)]
#[forbid(unsafe_code)]
mod callback;
#[forbid(unsafe_code)]
mod control;
#[forbid(unsafe_code)]
mod ingress;
mod platform;
#[forbid(unsafe_code)]
mod registry;
#[forbid(unsafe_code)]
mod runtime;
#[forbid(unsafe_code)]
mod schedule;
#[forbid(unsafe_code)]
mod snapshot;

pub use callback::{
    AfterCompletionDecision, AfterCompletionRunResult, OnceDecision, OnceRunResult,
};
pub use ingress::is_timer_dispatch_method;
pub use registry::{MAX_TIMER_REGISTRATIONS, RegisterError};
pub use runtime::{
    AfterCompletionContext, AfterCompletionRegistration, OnceContext, OnceRegistration, TimerError,
    TimerReconcileState, TimerStart, WatchdogContext, WatchdogReconcileState, WatchdogRegistration,
    consecutive_expected_failures, initialize_runtime, reconcile_after_completion, reconcile_once,
    reconcile_watchdog, register_after_completion, register_once, register_watchdog,
    timer_inventory, timer_snapshot,
};
pub use schedule::{ScheduleError, TimerCadence, TimerSchedule};
pub use snapshot::{
    DeclarationLifetime, InactiveReason, MAX_TIMER_IDENTITY_COMPONENT_BYTES, MeasurementCounter,
    MeasurementSummary, MemoryPageExtent, MemoryPageSample, MemoryPageSummary,
    OrdinaryRuntimeStateSnapshot, TimerCompletion, TimerCompletionOutcome, TimerControlFailure,
    TimerCounters, TimerDirectiveSnapshot, TimerEpoch, TimerIdentity, TimerIdentityError,
    TimerIdentityField, TimerInventorySnapshot, TimerLastOutcome, TimerObservabilitySnapshot,
    TimerOutcomeSnapshot, TimerPerformance, TimerPolicy, TimerRegistrationId,
    TimerRegistrationStatus, TimerRuntimeStateSnapshot, TimerSchedulingMode, TimerSnapshot,
    WatchdogAttemptStatus, WatchdogDecision, WatchdogRunResult, WatchdogRuntimeStateSnapshot,
};
