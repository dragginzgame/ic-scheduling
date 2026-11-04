//! Provider-neutral identity and coherent canonical snapshot values.
//!
//! Public snapshots are inert observations with private top-level fields. The
//! registry is their only constructor and mutation authority.

mod identity;
mod metrics;
mod model;

/// Inert identity of one registration and its cumulative measurements.
///
/// Equality proves a shared counter lifetime only within one canister's
/// observed runtime history. Reinstalls or restored/forked canister state are
/// separate histories. The sequence never wraps: exhaustion rejects registration.
/// Values are not provider handles or mutation capabilities.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimerRegistrationId {
    epoch: TimerEpoch,
    sequence: u64,
}

impl TimerRegistrationId {
    pub(crate) const fn new(epoch: TimerEpoch, sequence: u64) -> Self {
        Self { epoch, sequence }
    }

    /// Return the runtime reset boundary containing this registration.
    #[must_use]
    pub const fn epoch(self) -> TimerEpoch {
        self.epoch
    }

    /// Return the nonzero registration sequence within this runtime epoch.
    #[must_use]
    pub const fn sequence(self) -> u64 {
        self.sequence
    }
}

/// Shared measurement recorders; registration identity remains timer-owned.
pub use ic_metrics::{MeasurementCounter, MeasurementSummary};
pub use identity::{
    MAX_TIMER_IDENTITY_COMPONENT_BYTES, TimerIdentity, TimerIdentityError, TimerIdentityField,
};
pub use metrics::{
    MemoryPageExtent, MemoryPageSample, MemoryPageSummary, TimerCounters,
    TimerObservabilitySnapshot, TimerPerformance,
};
pub use model::{
    DeclarationLifetime, InactiveReason, OrdinaryRuntimeStateSnapshot, TimerCompletion,
    TimerCompletionOutcome, TimerControlFailure, TimerDirectiveSnapshot, TimerEpoch,
    TimerLastOutcome, TimerOutcomeSnapshot, TimerPolicy, TimerRegistrationStatus,
    TimerRuntimeStateSnapshot, TimerSchedulingMode, WatchdogAttemptStatus, WatchdogDecision,
    WatchdogRunResult, WatchdogRuntimeStateSnapshot,
};

/// Atomic provider-neutral snapshot of one complete canister-local inventory.
///
/// The epoch remains observable even when no timer is declared. Timer order is
/// deterministic by [`TimerIdentity`], every contained timer belongs to the
/// returned epoch, and the bounded registry is the only constructor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimerInventorySnapshot {
    epoch: TimerEpoch,
    timers: Vec<TimerSnapshot>,
}

impl TimerInventorySnapshot {
    pub(crate) const fn new(epoch: TimerEpoch, timers: Vec<TimerSnapshot>) -> Self {
        Self { epoch, timers }
    }

    /// Return the volatile runtime epoch shared by the complete inventory.
    #[must_use]
    pub const fn epoch(&self) -> TimerEpoch {
        self.epoch
    }

    /// Return all timer snapshots in deterministic identity order.
    #[must_use]
    pub fn timers(&self) -> &[TimerSnapshot] {
        &self.timers
    }

    /// Consume the inventory and return its ordered timer snapshots.
    #[must_use]
    pub fn into_timers(self) -> Vec<TimerSnapshot> {
        self.timers
    }

    /// Return the number of declared logical timers.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.timers.len()
    }

    /// Return whether the initialized registry has no declarations.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.timers.is_empty()
    }
}

/// Canonical provider-neutral operator snapshot for one logical timer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimerSnapshot {
    identity: TimerIdentity,
    registration_id: TimerRegistrationId,
    policy: TimerPolicy,
    lifetime: DeclarationLifetime,
    state: TimerRuntimeStateSnapshot,
    scheduling_mode: TimerSchedulingMode,
    latest_directive: Option<TimerDirectiveSnapshot>,
    latest_requested_delay_ns: Option<u64>,
    latest_armed_delay_ns: Option<u64>,
    observability: TimerObservabilitySnapshot,
}

impl TimerSnapshot {
    #[expect(
        clippy::too_many_arguments,
        reason = "Registry-only constructor keeps one coherent boundary."
    )]
    pub(crate) const fn new(
        identity: TimerIdentity,
        registration_id: TimerRegistrationId,
        policy: TimerPolicy,
        lifetime: DeclarationLifetime,
        state: TimerRuntimeStateSnapshot,
        scheduling_mode: TimerSchedulingMode,
        latest_directive: Option<TimerDirectiveSnapshot>,
        latest_requested_delay_ns: Option<u64>,
        latest_armed_delay_ns: Option<u64>,
        observability: &TimerObservabilitySnapshot,
    ) -> Self {
        Self {
            identity,
            registration_id,
            policy,
            lifetime,
            state,
            scheduling_mode,
            latest_directive,
            latest_requested_delay_ns,
            latest_armed_delay_ns,
            observability: *observability,
        }
    }

    /// Return the stable structured identity.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        &self.identity
    }

    /// Return the inert identity of this registration's counter lifetime.
    ///
    /// Cancellation, rescheduling and completion preserve it. Unregistering
    /// and registering again changes it, even within the same runtime epoch.
    /// Compare only snapshots from the same canister. This value grants no
    /// timer-control authority and cannot be used to reconstruct a registration.
    #[must_use]
    pub const fn registration_id(&self) -> TimerRegistrationId {
        self.registration_id
    }

    /// Return the configured scheduling policy.
    #[must_use]
    pub const fn policy(&self) -> TimerPolicy {
        self.policy
    }

    /// Return whether the declaration remains after terminal stop.
    #[must_use]
    pub const fn lifetime(&self) -> DeclarationLifetime {
        self.lifetime
    }

    /// Return the closed policy-specific runtime state.
    #[must_use]
    pub const fn state(&self) -> TimerRuntimeStateSnapshot {
        self.state
    }

    /// Return the effective scheduling mode.
    ///
    /// A new declaration starts with its configured policy mode. Later
    /// requests and completed directives update this value, including after
    /// the declaration becomes inactive.
    #[must_use]
    pub const fn scheduling_mode(&self) -> TimerSchedulingMode {
        self.scheduling_mode
    }

    /// Return the latest effective ordinary directive at completion.
    ///
    /// A winning exact reconciliation projects `ScheduleAt`, even if the
    /// callback returned a different or invalid proposal. Cancellation and
    /// explicit invariant failure project `Stop`.
    #[must_use]
    pub const fn latest_directive(&self) -> Option<TimerDirectiveSnapshot> {
        self.latest_directive
    }

    /// Return the latest requested relative delay.
    #[must_use]
    pub const fn latest_requested_delay_ns(&self) -> Option<u64> {
        self.latest_requested_delay_ns
    }

    /// Return the latest relative delay whose provider arm committed.
    #[must_use]
    pub const fn latest_armed_delay_ns(&self) -> Option<u64> {
        self.latest_armed_delay_ns
    }

    /// Return the next authoritative absolute deadline.
    #[must_use]
    pub const fn next_deadline_ns(&self) -> Option<u64> {
        self.state.next_deadline_ns()
    }

    /// Return a portable registration projection.
    #[must_use]
    pub fn registration_status(&self) -> TimerRegistrationStatus {
        self.state.into()
    }

    /// Return the latest authoritative callback generation.
    ///
    /// Scheduling changes this value. Use [`Self::registration_id`] to check
    /// counter continuity across observations. A Watchdog's outstanding work
    /// and successor share this dispatch generation; it is not a work count.
    #[must_use]
    pub const fn generation(&self) -> Option<u64> {
        match self.state {
            TimerRuntimeStateSnapshot::Inactive { .. } => None,
            TimerRuntimeStateSnapshot::Ordinary(
                OrdinaryRuntimeStateSnapshot::Scheduled { generation, .. }
                | OrdinaryRuntimeStateSnapshot::Running { generation },
            ) => Some(generation),
            TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled {
                scheduler_generation,
                ..
            }) => Some(scheduler_generation),
            TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::AwaitingWork {
                successor_generation,
                ..
            }) => Some(successor_generation),
        }
    }

    /// Return registration-scoped outcomes, counters, and measurements.
    #[must_use]
    pub const fn observability(&self) -> TimerObservabilitySnapshot {
        self.observability
    }
}

#[cfg(test)]
mod tests;
