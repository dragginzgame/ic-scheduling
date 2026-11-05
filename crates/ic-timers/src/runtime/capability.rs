//! Policy-specific registration and running-context capabilities.
//! Shared authority checks stay with the runtime owner.

use super::{
    CallbackToken, Declared, Executing, RegistrationClaim, TimerError, TimerIdentity,
    TimerSchedule, cancel_claim, ensure_once_claim, ensure_recurring_claim,
    ensure_watchdog_immediately_claim, has_armed_wakeup_claim, reconcile_ordinary_claim,
    reconcile_watchdog_claim, unregister_claim,
};

/// Delegated control capability scoped to one exact `Once` work attempt.
///
/// Identity remains inspectable after work returns or is abandoned, but mutation
/// methods then return [`TimerError::RegistrationExpired`]. Retain the
/// [`OnceRegistration`] for longer-lived ownership.
pub struct OnceContext {
    pub(super) token: CallbackToken,
}

impl OnceContext {
    pub(super) const fn new(token: CallbackToken) -> Self {
        Self { token }
    }

    /// Return the logical timer identity executing this work.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.token.identity()
    }

    /// Schedule a `Once` declaration while this exact work attempt is active.
    ///
    /// A context retained after completion or delivery abandonment is expired
    /// and returns [`TimerError::RegistrationExpired`].
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn ensure_scheduled(&self, schedule: TimerSchedule) -> Result<(), TimerError> {
        ensure_once_claim(Executing(&self.token), schedule)
    }

    /// Reconcile the executing declaration to one exact schedule.
    ///
    /// A winning exact schedule replaces the returned scheduling proposal
    /// before validation. Explicit consumer invariant failure still stops.
    ///
    /// `None` requests idle state at normal completion without recording an
    /// explicit cancellation. Retained callback authority remains; a
    /// remove-on-stop declaration is removed when that request wins arbitration.
    /// A stored context cannot mutate the registration after its exact work
    /// attempt ends.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_ordinary_claim(Executing(&self.token), schedule)
    }

    /// Request cancellation while this exact work attempt is active.
    /// Cancellation allocates no callback generation.
    ///
    /// Cancellation does not interrupt the current invocation; normal
    /// completion applies it before any callback successor is retained.
    /// A retained declaration becomes inactive. A remove-on-stop declaration
    /// is removed when cancellation wins arbitration.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Executing(&self.token))
    }
}

/// Delegated control capability scoped to one exact after-completion work
/// attempt.
///
/// Mutation authority expires when the callback completes or its delivery is
/// abandoned. Retain the [`AfterCompletionRegistration`] for longer-lived
/// ownership.
pub struct AfterCompletionContext {
    pub(super) token: CallbackToken,
}

impl AfterCompletionContext {
    pub(super) const fn new(token: CallbackToken) -> Self {
        Self { token }
    }

    /// Return the logical timer identity executing this work.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.token.identity()
    }

    /// Request configured recurrence after this exact work attempt.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn ensure_scheduled(&self) -> Result<(), TimerError> {
        ensure_recurring_claim(Executing(&self.token))
    }

    /// Reconcile the executing declaration to one exact schedule without
    /// changing its configured after-completion cadence.
    ///
    /// A winning exact schedule replaces the returned scheduling proposal
    /// before validation. Explicit consumer invariant failure still stops.
    ///
    /// `None` requests idle state at normal completion without recording an
    /// explicit cancellation. A retained declaration keeps callback authority;
    /// a remove-on-stop declaration is removed when that request wins.
    /// A stored context cannot mutate the registration after its exact work
    /// attempt ends.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_ordinary_claim(Executing(&self.token), schedule)
    }

    /// Request cancellation while this exact work attempt is active.
    /// Cancellation allocates no callback generation.
    ///
    /// Cancellation does not interrupt the current invocation; normal
    /// completion applies it before any callback successor is retained. A
    /// retained declaration becomes inactive; a remove-on-stop declaration is
    /// removed when cancellation wins arbitration.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Executing(&self.token))
    }
}

/// Delegated control capability scoped to one exact watchdog work attempt.
///
/// Mutation authority expires when the callback completes. Retain the
/// [`WatchdogRegistration`] for longer-lived ownership.
pub struct WatchdogContext {
    token: CallbackToken,
}

impl WatchdogContext {
    pub(super) const fn new(token: CallbackToken) -> Self {
        Self { token }
    }

    /// Return the logical timer identity executing this work.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.token.identity()
    }

    /// Request that the pre-armed cadence successor remain scheduled.
    ///
    /// This is idempotent unless it supersedes a nested cancellation request.
    /// It never arms an additional Watchdog successor and cannot move a
    /// pending immediate successor later.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn ensure_scheduled(&self) -> Result<(), TimerError> {
        ensure_recurring_claim(Executing(&self.token))
    }

    /// Request that this work attempt's pre-armed successor run immediately.
    ///
    /// Repeated requests coalesce, and a cadence ensure cannot move the
    /// pending immediate deadline later. Normal completion replaces the exact
    /// cadence successor with a zero-delay scheduler callback. Consumer work
    /// still runs only in the scheduler's separately queued later message.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn ensure_scheduled_immediately(&self) -> Result<(), TimerError> {
        ensure_watchdog_immediately_claim(Executing(&self.token))
    }

    /// Replace this attempt's successor with an exact schedule, or cancel it.
    ///
    /// Applied on normal completion; a trap preserves the committed cadence
    /// successor. The latest reconciliation wins over prior scheduling demand,
    /// but unregistration and invariant failure remain terminal.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_watchdog_claim(Executing(&self.token), schedule)
    }

    /// Request cancellation while this exact work attempt is active.
    /// Cancellation allocates no callback generation.
    ///
    /// Cancellation does not interrupt the current invocation; normal
    /// completion clears the already-armed successor when cancellation wins.
    /// A retained declaration becomes inactive; a remove-on-stop declaration
    /// is removed.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Executing(&self.token))
    }
}

/// Opaque non-clone claim for one registered `Once` callback.
///
/// Dropping this capability does not cancel or unregister its declaration. Keep
/// it in volatile owner state and call `unregister` for explicit removal. Losing
/// a retained claim can leave a declaration occupying registry capacity until
/// the next Wasm instance; dropping the claim does not release callback captures.
#[must_use = "retain the registration claim so the timer remains controllable"]
pub struct OnceRegistration {
    pub(super) claim: RegistrationClaim,
}

impl OnceRegistration {
    /// Return the claimed logical identity.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.claim.identity()
    }

    /// Return whether this exact claim currently owns an armed provider wake-up.
    ///
    /// This is a volatile observation, not durable scheduling authority or a
    /// delivery guarantee. Call [`Self::ensure_scheduled`] unconditionally when
    /// a wake-up is required rather than using this value as a scheduling guard.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable or this
    /// registration no longer owns its declaration.
    pub fn has_armed_wakeup(&self) -> Result<bool, TimerError> {
        has_armed_wakeup_claim(&self.claim)
    }

    /// Ensure one invocation is armed or retained as the running work's successor.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn ensure_scheduled(&self, schedule: TimerSchedule) -> Result<(), TimerError> {
        ensure_once_claim(Declared(&self.claim), schedule)
    }

    /// Reconcile to one exact desired schedule, replacing a later or earlier
    /// live deadline as necessary.
    ///
    /// During running work, a winning exact schedule replaces the callback
    /// scheduling proposal before validation; explicit invariant failure stops.
    ///
    /// `None` leaves a retained declaration idle after any running work
    /// completes or is abandoned. A remove-on-stop declaration is removed and
    /// this claim expires when the transition finalizes. Clearing scheduled
    /// demand is distinct from explicit cancellation; an already inactive
    /// declaration retains its existing reason.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_ordinary_claim(Declared(&self.claim), schedule)
    }

    /// Cancel the armed callback or the running work's successor.
    /// Cancellation allocates no callback generation, even at the counter limit.
    ///
    /// Consumer work already running is not interrupted.
    /// A retained declaration keeps callback authority. A remove-on-stop
    /// declaration and this claim expire when cancellation finalizes.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Declared(&self.claim))
    }

    /// Consume the claim and unregister its callback authority.
    ///
    /// When called from running work, removal is deferred until that invocation
    /// completes normally or its confirmed delivery is abandoned.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn unregister(self) -> Result<(), TimerError> {
        unregister_claim(&self.claim)
    }
}

/// Opaque non-clone claim for one callback with configured
/// after-completion recurrence.
///
/// Dropping this capability does not cancel or unregister its declaration. Keep
/// it in volatile owner state and call `unregister` for explicit removal. Losing
/// a retained claim can leave a declaration occupying registry capacity until
/// the next Wasm instance; dropping the claim does not release callback captures.
#[must_use = "retain the registration claim so the timer remains controllable"]
pub struct AfterCompletionRegistration {
    pub(super) claim: RegistrationClaim,
}

/// Opaque non-clone claim for one pre-armed watchdog callback.
///
/// Dropping this capability does not cancel or unregister its declaration. Keep
/// it in volatile owner state and call `unregister` for explicit removal. Losing
/// a retained claim can leave a declaration occupying registry capacity until
/// the next Wasm instance; dropping the claim does not release callback captures.
#[must_use = "retain the registration claim so the timer remains controllable"]
pub struct WatchdogRegistration {
    pub(super) claim: RegistrationClaim,
}

/// Desired volatile after-completion state during lifecycle reconciliation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerReconcileState {
    /// Request inactive state immediately unless consumer work is running.
    Inactive,
    /// Preserve scheduling demand now or through the running work's successor.
    Scheduled,
}

/// Desired volatile watchdog state during synchronous lifecycle reconciliation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatchdogReconcileState {
    /// Request inactive state immediately unless consumer work is running.
    Inactive,
    /// Preserve or arm one scheduler wake-up at the configured cadence.
    Scheduled,
    /// Preserve an earlier wake-up or arm/move one scheduler wake-up to now.
    ///
    /// The zero-delay provider callback executes as a later replicated
    /// scheduler message and does not invoke consumer work synchronously.
    ScheduledImmediately,
    /// Reconcile the scheduler to an exact absolute IC timestamp.
    ScheduledAt(u64),
}

impl WatchdogRegistration {
    /// Return the claimed logical identity.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.claim.identity()
    }

    /// Return whether this exact claim currently owns an armed scheduler wake-up.
    ///
    /// The separately queued work callback is not itself a wake-up. During
    /// watchdog work this returns `true` only because the scheduler has already
    /// committed and installed the cadence successor. This is a volatile
    /// observation, not a delivery guarantee.
    /// Confirmed scheduler-delivery abandonment clears this observation; an
    /// owner can restore the wake-up through ensure or reconciliation, including
    /// an unchanged exact deadline. A stuck delivery that is never destroyed
    /// cannot be inferred from this observation.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable or this
    /// registration no longer owns its declaration.
    pub fn has_armed_wakeup(&self) -> Result<bool, TimerError> {
        has_armed_wakeup_claim(&self.claim)
    }

    /// Synchronously ensure one watchdog scheduler wake-up is authoritative.
    ///
    /// An existing earlier immediate deadline is retained and never delayed.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn ensure_scheduled(&self) -> Result<(), TimerError> {
        ensure_recurring_claim(Declared(&self.claim))
    }

    /// Synchronously ensure one watchdog scheduler wake-up is due now.
    ///
    /// An inactive declaration arms one zero-delay scheduler. A later cadence
    /// deadline is replaced in place; an already earlier or equivalent
    /// deadline and repeated immediate requests coalesce. If work is already
    /// dispatched, that zero-delay work satisfies the request. If work is
    /// running, the request applies to that attempt's pre-armed successor.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn ensure_scheduled_immediately(&self) -> Result<(), TimerError> {
        ensure_watchdog_immediately_claim(Declared(&self.claim))
    }

    /// Reconcile to one exact scheduler deadline, or cancel with `None`.
    ///
    /// Unlike ensure, this can move a wake-up later. Identical deadlines
    /// coalesce. Dispatched or running work is not postponed: reconciliation
    /// selects its successor on normal completion. Until then, the committed
    /// cadence successor remains the recovery wake-up. A subsequent recovery
    /// attempt retires the interrupted attempt's pending proposal.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_watchdog_claim(Declared(&self.claim), schedule)
    }

    /// Cancel the scheduler and any work callback that has not started.
    /// Cancellation allocates no callback generation, even at the counter limits.
    ///
    /// Consumer work already running is not interrupted; normal completion
    /// clears its pre-armed successor.
    /// A retained declaration keeps callback authority. A remove-on-stop
    /// declaration and this claim expire when cancellation finalizes.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Declared(&self.claim))
    }

    /// Consume the claim and unregister its callback authority.
    ///
    /// When called from running work, removal is deferred until that invocation
    /// completes normally.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn unregister(self) -> Result<(), TimerError> {
        unregister_claim(&self.claim)
    }
}

impl AfterCompletionRegistration {
    /// Return the claimed logical identity.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.claim.identity()
    }

    /// Return whether this exact claim currently owns an armed provider wake-up.
    ///
    /// A callback currently running without an installed successor returns
    /// `false`. This is a volatile observation, not durable scheduling authority
    /// or a delivery guarantee.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable or this
    /// registration no longer owns its declaration.
    pub fn has_armed_wakeup(&self) -> Result<bool, TimerError> {
        has_armed_wakeup_claim(&self.claim)
    }

    /// Ensure configured recurrence is armed or retained as the running work's
    /// successor.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn ensure_scheduled(&self) -> Result<(), TimerError> {
        ensure_recurring_claim(Declared(&self.claim))
    }

    /// Reconcile to one exact desired schedule without changing the configured
    /// after-completion cadence. `None` makes a retained declaration inactive
    /// after any running work completes or is abandoned; it removes a
    /// remove-on-stop declaration and expires this claim when the transition
    /// finalizes.
    ///
    /// During running work, a winning exact schedule replaces the callback
    /// scheduling proposal before validation; explicit invariant failure stops.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_ordinary_claim(Declared(&self.claim), schedule)
    }

    /// Cancel the armed callback or the running work's successor.
    /// Cancellation allocates no callback generation, even at the counter limit.
    ///
    /// Consumer work already running is not interrupted.
    /// A retained declaration keeps callback authority. A remove-on-stop
    /// declaration and this claim expire when cancellation finalizes.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(Declared(&self.claim))
    }

    /// Consume the claim and unregister its callback authority.
    ///
    /// When called from running work, removal is deferred until that invocation
    /// completes normally or its confirmed delivery is abandoned.
    ///
    /// # Errors
    ///
    /// Returns [`TimerError`] if the runtime is unavailable, this capability
    /// no longer has authority, or the requested control transition fails.
    pub fn unregister(self) -> Result<(), TimerError> {
        unregister_claim(&self.claim)
    }
}
