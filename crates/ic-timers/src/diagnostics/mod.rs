//! Bounded inert inventory; no callback, recovery token or scheduling authority.

#[cfg(test)]
mod tests;

use crate::{Lifetime, Observation};
use ic_metrics::MeasurementCounter;

/// Registration kind in an inventory row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerKind {
    /// Isolated async Once work.
    Once,
    /// Synchronous prearmed watchdog.
    Watchdog,
}
/// Local eligibility/custody projection, without a recovery capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerPhase {
    /// No deadline.
    Inactive,
    /// Future or overdue eligibility.
    Armed,
    /// Accepted Once work.
    Running,
    /// Once requires owner reconciliation.
    RecoveryRequired,
    /// Terminal entry fault.
    Faulted,
}
/// Per-declaration observations, reset only by replacing the declaration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TimerMeasurements {
    /// Committed logical acceptances, not proof of remote business effects.
    pub accepted: MeasurementCounter,
    /// Normally returned callbacks.
    pub completed: MeasurementCounter,
    /// Confirmed abandonment/rejected watchdog delivery.
    pub abandoned: MeasurementCounter,
    /// Consecutive normally returned ExpectedFailure observations. Normal Success
    /// or NoWork resets this; traps do not manufacture a business result.
    pub consecutive_expected_failures: MeasurementCounter,
    /// Latest normal business observation; separate from eligibility or traps.
    pub last_observation: Option<Observation>,
}
impl TimerMeasurements {
    pub(crate) fn returned(&mut self, observation: Observation) {
        self.completed.add(1);
        self.last_observation = Some(observation);
        if observation == Observation::ExpectedFailure {
            self.consecutive_expected_failures.add(1);
        } else {
            self.consecutive_expected_failures = MeasurementCounter::default();
        }
    }
}
/// One inert declaration snapshot. Incarnation distinguishes slot reuse within
/// this volatile runtime only; hosts must identify upgrade/reinstall boundaries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimerSnapshot {
    /// Stable owner/group/name labels, sorted lexically by inventory.
    pub identity: [String; 3],
    /// Nonzero runtime-local declaration incarnation; never a control handle.
    pub incarnation: u64,
    /// Declaration kind.
    pub kind: TimerKind,
    /// Retention policy.
    pub lifetime: Lifetime,
    /// Local state; watchdog deadline and custody are separate fields.
    pub phase: TimerPhase,
    /// Next eligibility/cadence timestamp, if any.
    pub deadline: Option<u64>,
    /// A watchdog ticket remains pending, running or settling; not a consumer async lease.
    pub outstanding: bool,
    /// Declaration-scoped observations using ic-metrics overflow semantics.
    pub measurements: TimerMeasurements,
}
/// Coherent bounded inventory, available with the metrics feature.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inventory {
    /// Terminal provider fault; preserved separately from individual entry state.
    pub provider_faulted: bool,
    /// At most the declaration capacity, ordered by identity.
    pub timers: Vec<TimerSnapshot>,
}
