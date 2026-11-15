//! Initial Once-only public contract. Watchdog support is not implemented yet.

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
    /// Consumer outcome. Metrics integration is not yet implemented.
    pub observation: Observation,
    /// Proposed next eligibility.
    pub next: OnceNext,
}
/// Initialization, admission or control failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Initialize the live runtime before binding.
    Uninitialized,
    /// Empty or over-64-byte identity component.
    InvalidIdentity,
    /// Another retained declaration has this identity.
    DuplicateIdentity,
    /// The provisional 64-declaration bound is full.
    Capacity,
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
            B::StaleAuthority => Self::StaleAuthority,
            B::Control(C::RecoveryRequired) => Self::RecoveryRequired,
            B::Control(C::StaleRecovery) => Self::StaleRecovery,
            B::Control(C::Exhausted) => Self::Faulted,
        }
    }
}
impl From<OnceDemand> for crate::engine::Demand {
    fn from(value: OnceDemand) -> Self {
        match value {
            OnceDemand::Inactive => Self::Inactive,
            OnceDemand::At(at) => Self::At(at),
        }
    }
}
