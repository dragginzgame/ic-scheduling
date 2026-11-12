//! Checked cadence and scheduling decisions.

use crate::snapshot::{TimerControlFailure, TimerDirectiveSnapshot, TimerSchedulingMode};
use std::time::Duration;
use thiserror::Error;

/// Validated positive recurrence cadence.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimerCadence(u64);

impl TimerCadence {
    /// Validate a recurrence cadence.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError::ZeroCadence`] for a zero cadence. A duration
    /// that cannot fit in `u64` nanoseconds returns [`ScheduleError::DelayOutOfRange`].
    pub fn new(cadence: Duration) -> Result<Self, ScheduleError> {
        let cadence_ns = duration_ns(cadence)?;
        Self::from_nanos(cadence_ns)
    }

    /// Validate an already encoded nanosecond cadence.
    ///
    /// # Errors
    ///
    /// Returns [`ScheduleError::ZeroCadence`] for a zero cadence.
    pub const fn from_nanos(cadence_ns: u64) -> Result<Self, ScheduleError> {
        if cadence_ns == 0 {
            Err(ScheduleError::ZeroCadence)
        } else {
            Ok(Self(cadence_ns))
        }
    }

    /// Return the encoded cadence in nanoseconds.
    #[must_use]
    pub const fn as_nanos(self) -> u64 {
        self.0
    }

    /// Resolve one successor relative to the supplied dispatch time.
    pub(super) const fn deadline_after(self, now_ns: u64) -> Result<u64, ScheduleError> {
        checked_deadline_after(now_ns, self.0)
    }
}

/// Initial or explicit request for a timer deadline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerSchedule {
    /// Schedule relative to the current IC time.
    After(Duration),
    /// Schedule at an absolute IC timestamp in nanoseconds.
    At(u64),
}

impl TimerSchedule {
    /// Resolve the request with its absolute deadline, relative delay and mode.
    pub(super) fn resolve(self, now_ns: u64) -> Result<ResolvedSchedule, ScheduleError> {
        match self {
            Self::After(delay) => {
                let delay_ns = duration_ns(delay)?;
                Ok(ResolvedSchedule {
                    deadline_ns: checked_deadline_after(now_ns, delay_ns)?,
                    requested_delay_ns: Some(delay_ns),
                    mode: TimerSchedulingMode::Once,
                })
            }
            Self::At(deadline_ns) => Ok(ResolvedSchedule {
                deadline_ns,
                requested_delay_ns: None,
                mode: TimerSchedulingMode::Deadline,
            }),
        }
    }
}

/// Private scheduling proposal after erasing a policy-specific callback result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrdinaryDirective {
    /// Do not schedule another invocation.
    Stop,
    /// Continue as soon as the runtime can execute another message.
    ContinueImmediately,
    /// Retry after the provided delay.
    RetryAfter(Duration),
    /// Schedule at an absolute IC timestamp in nanoseconds.
    ScheduleAt(u64),
    /// Recur using the registration's configured after-completion cadence.
    RecurAfterCompletion,
}

impl OrdinaryDirective {
    pub(super) fn resolve(
        self,
        now_ns: u64,
        cadence: Option<TimerCadence>,
    ) -> Result<(Option<ResolvedSchedule>, TimerDirectiveSnapshot), TimerControlFailure> {
        // Resolve executable proposals directly. Snapshots observe the selected
        // directive; they are not an intermediate execution representation.
        Ok(match self {
            Self::Stop => (None, TimerDirectiveSnapshot::Stop),
            Self::ContinueImmediately => (
                Some(ResolvedSchedule {
                    deadline_ns: now_ns,
                    requested_delay_ns: Some(0),
                    mode: TimerSchedulingMode::Continuation,
                }),
                TimerDirectiveSnapshot::ContinueImmediately,
            ),
            Self::RetryAfter(delay) => {
                let delay_ns = duration_ns(delay).map_err(ScheduleError::control_failure)?;
                (
                    Some(ResolvedSchedule {
                        deadline_ns: checked_deadline_after(now_ns, delay_ns)
                            .map_err(ScheduleError::control_failure)?,
                        requested_delay_ns: Some(delay_ns),
                        mode: TimerSchedulingMode::Retry,
                    }),
                    TimerDirectiveSnapshot::RetryAfter { delay_ns },
                )
            }
            Self::ScheduleAt(deadline_ns) => (
                Some(ResolvedSchedule {
                    deadline_ns,
                    requested_delay_ns: None,
                    mode: TimerSchedulingMode::Deadline,
                }),
                TimerDirectiveSnapshot::ScheduleAt { deadline_ns },
            ),
            Self::RecurAfterCompletion => {
                let cadence = cadence.ok_or(TimerControlFailure::DirectiveNotAllowed)?;
                (
                    Some(ResolvedSchedule {
                        deadline_ns: cadence
                            .deadline_after(now_ns)
                            .map_err(ScheduleError::control_failure)?,
                        requested_delay_ns: Some(cadence.as_nanos()),
                        mode: TimerSchedulingMode::AfterCompletion,
                    }),
                    TimerDirectiveSnapshot::RecurAfterCompletion,
                )
            }
        })
    }
}

/// Failure to represent a cadence or requested deadline.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ScheduleError {
    /// A recurring timer must advance by a positive duration.
    #[error("timer cadence must be greater than zero")]
    ZeroCadence,
    /// The duration cannot be represented as nanoseconds in a `u64`.
    #[error("timer delay exceeds the supported nanosecond range")]
    DelayOutOfRange,
    /// Adding the delay to the current time overflows a `u64`.
    #[error("timer deadline exceeds the supported timestamp range")]
    DeadlineOverflow,
}

impl ScheduleError {
    // Invalid callback successors are terminal control failures; explicit
    // scheduling requests keep returning ScheduleError at their input boundary.
    const fn control_failure(self) -> TimerControlFailure {
        match self {
            Self::ZeroCadence => TimerControlFailure::DirectiveNotAllowed,
            Self::DelayOutOfRange => TimerControlFailure::DelayOutOfRange,
            Self::DeadlineOverflow => TimerControlFailure::DeadlineOverflow,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedSchedule {
    pub(crate) deadline_ns: u64,
    pub(crate) requested_delay_ns: Option<u64>,
    pub(crate) mode: TimerSchedulingMode,
}

pub const fn checked_deadline_after(now_ns: u64, delay_ns: u64) -> Result<u64, ScheduleError> {
    match now_ns.checked_add(delay_ns) {
        Some(deadline_ns) => Ok(deadline_ns),
        None => Err(ScheduleError::DeadlineOverflow),
    }
}

pub fn duration_ns(duration: Duration) -> Result<u64, ScheduleError> {
    u64::try_from(duration.as_nanos()).map_err(|_| ScheduleError::DelayOutOfRange)
}

#[cfg(test)]
mod tests;
