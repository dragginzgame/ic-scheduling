//! Retained Once and synchronous watchdog scheduling contracts.

/// Declaration lifetime, independent of whether a handle is retained by the host.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lifetime {
    /// Keep inactive declarations until explicitly removed.
    Retained,
    /// Release the declaration after stopping and settling accepted work.
    /// Trapped Once work remains retained until explicit reconciliation.
    RemoveWhenStopped,
}

/// Absolute Unix nanosecond demand. Zero is eligible, not disabled.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnceDemand {
    /// Stop future eligibility without cancelling accepted work.
    Inactive,
    /// Become eligible at the supplied timestamp.
    At(u64),
}
/// Callback-proposed successor, subordinate to newer owner commands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnceNext {
    /// Stop future eligibility.
    Stop,
    /// Schedule another occurrence at this absolute timestamp.
    At(u64),
    /// Schedule after this many nanoseconds from callback completion, including
    /// any awaits. Overflow stops recurrence. Newer owner commands still win;
    /// traps require explicit recovery and never start a recurrence automatically.
    After(std::num::NonZeroU64),
}
/// Consumer outcome; never authority to retry uncertain work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Observation {
    /// No business work was needed.
    NoWork,
    /// Business work succeeded.
    Success,
    /// Expected business failure; successor remains explicitly chosen.
    ExpectedFailure,
}
/// A settled callback outcome and proposed successor.
pub struct OnceResult {
    /// Consumer outcome; recorded when the opt-in metrics feature is enabled.
    pub observation: Observation,
    /// Proposed next eligibility.
    pub next: OnceNext,
}
///
/// Error
/// Initialization, admission or control failure.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// No live runtime is initialized. Native callers cannot initialize one.
    Uninitialized,
    /// Empty or over-64-byte identity component.
    InvalidIdentity,
    /// Another retained declaration has this identity.
    DuplicateIdentity,
    /// The provisional 64-declaration bound is full.
    Capacity,
    /// Accepted or settling work owns the declaration, or the live runtime already owns
    /// different metrics storage.
    Busy,
    /// Authority is no longer current.
    StaleAuthority,
    /// Reconciliation is required before rearming.
    RecoveryRequired,
    /// The supplied recovery token is obsolete.
    StaleRecovery,
    /// Terminal provider/counter fault. No further dispatch is admitted.
    Faulted,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

impl From<crate::bindings::Error> for Error {
    fn from(error: crate::bindings::Error) -> Self {
        use crate::{bindings::Error as B, engine::ControlError as C};

        match error {
            B::InvalidIdentity => Self::InvalidIdentity,
            B::DuplicateIdentity => Self::DuplicateIdentity,
            B::Capacity => Self::Capacity,
            B::Busy => Self::Busy,
            B::StaleAuthority => Self::StaleAuthority,
            B::Control(C::RecoveryRequired) => Self::RecoveryRequired,
            B::Control(C::StaleRecovery) => Self::StaleRecovery,
            B::Control(C::Exhausted) => Self::Faulted,
        }
    }
}

/// Owner demand for a retained synchronous watchdog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatchdogDemand {
    /// Invalidate future and queued unaccepted work.
    Inactive,
    /// Preserve the deadline, or start at now plus cadence when inactive.
    KeepCadence,
    /// Replace eligibility with an absolute Unix nanosecond deadline.
    At(u64),
}
/// Synchronous callback decision, subordinate to newer owner commands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatchdogNext {
    /// Stop future opportunities.
    Stop,
    /// Retain the successor already committed by timer control.
    Continue,
    /// Replace the successor with an absolute deadline.
    At(u64),
}
/// Watchdog observation and scheduling decision; not an async execution lease.
pub struct WatchdogResult {
    /// Consumer outcome; recorded when the opt-in metrics feature is enabled.
    pub observation: Observation,
    /// Proposed successor.
    pub next: WatchdogNext,
}
