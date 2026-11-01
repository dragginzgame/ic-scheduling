//! Bounded canonical registry and provider-neutral policy transition engine.
//!
//! The registry emits provider-neutral effects. The runtime binds them to
//! linear provider handles while the registry remains the sole logical state
//! authority.

mod effect;

use effect::WatchdogDispatch;
pub use effect::{RegistryEffect, WakeupToken};

use crate::{
    callback::OrdinaryRunResult,
    control::{OrdinaryPending, TimerControl, TimerRegistration, WakeupArm},
    platform::TimerHandle,
    schedule::{OrdinaryDirective, ResolvedSchedule, ScheduleError, TimerCadence, TimerSchedule},
    snapshot::{
        DeclarationLifetime, InactiveReason, MemoryPageExtent, MemoryPageSample,
        OrdinaryRuntimeStateSnapshot, TimerCompletion, TimerCompletionOutcome, TimerControlFailure,
        TimerDirectiveSnapshot, TimerEpoch, TimerIdentity, TimerInventorySnapshot,
        TimerObservabilitySnapshot, TimerPolicy, TimerRegistrationId, TimerRuntimeStateSnapshot,
        TimerSchedulingMode, TimerSnapshot, WatchdogAttemptStatus, WatchdogDecision,
        WatchdogRunResult, WatchdogRuntimeStateSnapshot,
    },
};
use std::{cell::RefCell, collections::BTreeMap, fmt, future::Future, pin::Pin, rc::Rc};
use thiserror::Error;

/// Maximum declarations owned by one canonical registry.
pub const MAX_TIMER_REGISTRATIONS: usize = 64;

/// Opaque logical ownership claim returned by pure registration.
#[derive(Debug, Eq, PartialEq)]
pub struct RegistrationClaim {
    identity: TimerIdentity,
    claim_generation: u64,
}

/// Internal callback role carried by a generation-checked dispatch token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbackRole {
    OrdinaryWork,
    WatchdogScheduler,
    WatchdogWork,
}

/// Identity and generations an internal callback must present.
#[derive(Debug, Eq, PartialEq)]
pub struct CallbackToken {
    claim: RegistrationClaim,
    callback_generation: u64,
    role: CallbackRole,
}

impl CallbackToken {
    const fn new(
        identity: TimerIdentity,
        claim_generation: u64,
        callback_generation: u64,
        role: CallbackRole,
    ) -> Self {
        Self {
            claim: RegistrationClaim {
                identity,
                claim_generation,
            },
            callback_generation,
            role,
        }
    }

    pub(crate) const fn identity(&self) -> &TimerIdentity {
        self.claim.identity()
    }

    pub(crate) const fn claim(&self) -> &RegistrationClaim {
        &self.claim
    }

    #[cfg(test)]
    pub(crate) const fn callback_generation(&self) -> u64 {
        self.callback_generation
    }

    pub(crate) const fn role(&self) -> CallbackRole {
        self.role
    }
}

impl Clone for CallbackToken {
    fn clone(&self) -> Self {
        // Queued deliveries need owned tokens. Retained registration capabilities
        // remain non-clone, and running contexts borrow this token's exact claim.
        Self::new(
            self.identity().clone(),
            self.claim.claim_generation,
            self.callback_generation,
            self.role,
        )
    }
}

impl RegistrationClaim {
    pub(crate) const fn identity(&self) -> &TimerIdentity {
        &self.identity
    }

    pub(crate) const fn claim_generation(&self) -> u64 {
        self.claim_generation
    }
}

/// Whether a provider binding stages a pair or commits its arm observations.
#[derive(Clone, Copy, Debug)]
pub enum ProviderBinding {
    Stage,
    Commit { delay_ns: u64 },
}

/// Non-empty subset of provider callbacks cleared by one effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbacksToClear {
    Wakeup,
    Work,
    WakeupAndWork,
}

impl CallbacksToClear {
    pub(crate) const fn includes_wakeup(self) -> bool {
        matches!(self, Self::Wakeup | Self::WakeupAndWork)
    }

    pub(crate) const fn includes_work(self) -> bool {
        matches!(self, Self::Work | Self::WakeupAndWork)
    }
}

/// One pure transition and any terminal checked-control failure it produced.
pub struct RegistryTransition {
    effect: RegistryEffect,
    failure: Option<TimerControlFailure>,
    // Removal transfers callback captures out of the registry borrow. Keep
    // them alive until runtime has also applied the provider cleanup.
    removed: Option<Box<Entry>>,
}

impl fmt::Debug for RegistryTransition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegistryTransition")
            .field("effect", &self.effect)
            .field("failure", &self.failure)
            .field("removed", &self.removed.is_some())
            .finish()
    }
}

impl RegistryTransition {
    const fn normal(effect: RegistryEffect) -> Self {
        Self {
            effect,
            failure: None,
            removed: None,
        }
    }

    const fn terminal(effect: RegistryEffect, failure: TimerControlFailure) -> Self {
        Self {
            effect,
            failure: Some(failure),
            removed: None,
        }
    }

    pub(crate) const fn effect(&self) -> &RegistryEffect {
        &self.effect
    }

    pub(crate) const fn failure(&self) -> Option<TimerControlFailure> {
        self.failure
    }

    #[cfg(test)]
    pub(crate) fn into_effect(self) -> RegistryEffect {
        self.effect
    }
}

/// Failure to claim one canonical timer identity.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum RegisterError {
    /// The identity already has one canonical claimant.
    #[error("timer identity is already registered: {0:?}")]
    IdentityAlreadyRegistered(TimerIdentity),
    /// The fixed registry capacity has been reached.
    #[error("timer registry capacity of {max} declarations has been reached")]
    CapacityExceeded {
        /// Fixed registry capacity.
        max: usize,
    },
    /// Logical registration claim generations are exhausted.
    #[error("timer registration claim generation exhausted")]
    ClaimGenerationExhausted,
}

/// Invalid request against a logical registration claim.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum RegistryError {
    #[error("timer registration is no longer present")]
    UnknownRegistration,
    #[error("timer registration claim is stale")]
    StaleRegistration,
    #[error("timer policy invariant failed for {actual}")]
    PolicyMismatch { actual: &'static str },
    #[error("timer callback token is stale")]
    StaleCallback,
    #[error("timer registration already owns the provider handle for this callback role")]
    ProviderHandleAlreadyOwned,
    #[error(transparent)]
    Schedule(#[from] ScheduleError),
}

type OrdinaryFuture = Pin<Box<dyn Future<Output = OrdinaryRunResult>>>;
pub type OrdinaryCallback = Rc<RefCell<dyn FnMut(CallbackToken) -> OrdinaryFuture>>;
pub type WatchdogCallback = Rc<RefCell<dyn FnMut(CallbackToken) -> WatchdogRunResult>>;

struct OwnedProviderHandle {
    callback_generation: u64,
    handle: TimerHandle,
}

/// The at-most-two provider capabilities owned by one timer entry.
#[must_use = "clear every detached provider handle"]
#[derive(Default)]
pub struct ProviderHandles {
    wakeup: Option<TimerHandle>,
    work: Option<TimerHandle>,
}

impl ProviderHandles {
    pub(crate) const fn take_wakeup(&mut self) -> Option<TimerHandle> {
        self.wakeup.take()
    }

    pub(crate) const fn take_work(&mut self) -> Option<TimerHandle> {
        self.work.take()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OrdinaryRequest {
    Ensure,
    Reconcile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WatchdogPending {
    Cancel,
    Ensure,
    EnsureImmediately,
    Reconcile(ResolvedSchedule),
    Unregister,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WatchdogScheduleRequest {
    Cadence,
    Immediate,
    Reconcile(ResolvedSchedule),
}

impl WatchdogScheduleRequest {
    // Resolve one requested schedule before mutation. Cadence overflow is checked
    // only for a fresh arm; coalesced and awaiting requests keep their successor.
    fn resolve(
        self,
        state: WatchdogState,
        cadence: TimerCadence,
        now_ns: u64,
    ) -> Result<ResolvedSchedule, ScheduleError> {
        Ok(match self {
            Self::Cadence => ResolvedSchedule {
                deadline_ns: match state {
                    WatchdogState::Inactive { .. } => cadence.deadline_after(now_ns)?,
                    WatchdogState::Scheduled { deadline_ns } => deadline_ns,
                    WatchdogState::AwaitingWork {
                        successor_deadline_ns,
                        ..
                    } => successor_deadline_ns,
                },
                mode: TimerSchedulingMode::Watchdog,
                requested_delay_ns: Some(cadence.as_nanos()),
            },
            Self::Immediate => ResolvedSchedule {
                deadline_ns: match state {
                    WatchdogState::Scheduled { deadline_ns } => deadline_ns.min(now_ns),
                    _ => now_ns,
                },
                mode: TimerSchedulingMode::Continuation,
                requested_delay_ns: Some(0),
            },
            Self::Reconcile(requested) => requested,
        })
    }
}

// One payload owns policy, callback type and control together. An ordinary
// entry without cadence is Once; one with cadence is AfterCompletion.
enum EntryKind {
    Ordinary {
        cadence: Option<TimerCadence>,
        callback: OrdinaryCallback,
        control: TimerControl,
    },
    Watchdog {
        cadence: TimerCadence,
        callback: WatchdogCallback,
        control: WatchdogControl,
    },
}

impl EntryKind {
    const fn wakeup_role(&self) -> CallbackRole {
        match self {
            Self::Ordinary { .. } => CallbackRole::OrdinaryWork,
            Self::Watchdog { .. } => CallbackRole::WatchdogScheduler,
        }
    }

    fn ordinary(cadence: Option<TimerCadence>, callback: OrdinaryCallback) -> Self {
        Self::Ordinary {
            cadence,
            callback,
            control: TimerControl::default(),
        }
    }

    fn watchdog(cadence: TimerCadence, callback: WatchdogCallback) -> Self {
        Self::Watchdog {
            cadence,
            callback,
            control: WatchdogControl::default(),
        }
    }

    const fn policy(&self) -> TimerPolicy {
        match self {
            Self::Ordinary { cadence: None, .. } => TimerPolicy::Once,
            Self::Ordinary {
                cadence: Some(cadence),
                ..
            } => TimerPolicy::AfterCompletion { cadence: *cadence },
            Self::Watchdog { cadence, .. } => TimerPolicy::Watchdog { cadence: *cadence },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WatchdogState {
    Inactive {
        reason: InactiveReason,
    },
    Scheduled {
        deadline_ns: u64,
    },
    AwaitingWork {
        successor_deadline_ns: u64,
        attempt_status: WatchdogAttemptStatus,
        pending: Option<WatchdogPending>,
    },
}

#[derive(Debug)]
struct WatchdogControl {
    // One allocation history survives inactive state. A dispatch stamps its
    // successor and work with one generation, distinguished by callback role.
    // Requests while awaiting work cannot replace that successor until completion.
    generation: u64,
    state: WatchdogState,
    // Only confirmed destruction of the current bound scheduler sets this.
    // A missing handle alone can also be a synchronous effect's detached state.
    scheduler_abandoned: bool,
}

impl Default for WatchdogControl {
    fn default() -> Self {
        Self {
            generation: 0,
            state: WatchdogState::Inactive {
                reason: InactiveReason::NeverScheduled,
            },
            scheduler_abandoned: false,
        }
    }
}

impl WatchdogControl {
    fn arm_scheduler(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        deadline_ns: u64,
        arm: WakeupArm,
    ) -> RegistryTransition {
        let Some(generation) = self.generation.checked_add(1) else {
            let cleanup = clear_wakeup_if(arm.replaces_existing());
            return self.terminate(cleanup, TimerControlFailure::GenerationExhausted);
        };
        self.generation = generation;
        self.state = WatchdogState::Scheduled { deadline_ns };
        self.scheduler_abandoned = false;
        RegistryTransition::normal(RegistryEffect::ArmWakeup {
            token: WakeupToken::scheduler(claim, generation),
            delay_ns: deadline_ns.saturating_sub(now_ns),
            arm,
        })
    }

    const fn terminate(
        &mut self,
        effect: RegistryEffect,
        failure: TimerControlFailure,
    ) -> RegistryTransition {
        self.state = WatchdogState::Inactive {
            reason: InactiveReason::ControlFailure(failure),
        };
        RegistryTransition::terminal(effect, failure)
    }
}

struct Entry {
    claim_generation: u64,
    lifetime: DeclarationLifetime,
    kind: EntryKind,
    scheduling_mode: TimerSchedulingMode,
    latest_directive: Option<TimerDirectiveSnapshot>,
    latest_requested_delay_ns: Option<u64>,
    latest_armed_delay_ns: Option<u64>,
    confirmed_wakeup_generation: Option<u64>,
    wakeup: Option<OwnedProviderHandle>,
    work: Option<OwnedProviderHandle>,
    observability: TimerObservabilitySnapshot,
}

impl Entry {
    const fn new(claim_generation: u64, kind: EntryKind, lifetime: DeclarationLifetime) -> Self {
        let scheduling_mode = match kind.policy() {
            TimerPolicy::Once => TimerSchedulingMode::Once,
            TimerPolicy::AfterCompletion { .. } => TimerSchedulingMode::AfterCompletion,
            TimerPolicy::Watchdog { .. } => TimerSchedulingMode::Watchdog,
        };

        Self {
            claim_generation,
            lifetime,
            kind,
            scheduling_mode,
            latest_directive: None,
            latest_requested_delay_ns: None,
            latest_armed_delay_ns: None,
            confirmed_wakeup_generation: None,
            wakeup: None,
            work: None,
            observability: TimerObservabilitySnapshot::EMPTY,
        }
    }

    /// Validate the role, live generation and state for each provider binding.
    const fn provider_handle_slot(
        &mut self,
        token: &CallbackToken,
        binding: ProviderBinding,
    ) -> Result<&mut Option<OwnedProviderHandle>, RegistryError> {
        match (&self.kind, token.role) {
            (EntryKind::Ordinary { control, .. }, CallbackRole::OrdinaryWork)
                if matches!(
                    control.registration,
                    TimerRegistration::Scheduled { .. }
                        if control.generation() == token.callback_generation
                ) =>
            {
                Ok(&mut self.wakeup)
            }
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogScheduler)
                if matches!(
                    control.state,
                    WatchdogState::Scheduled { .. } | WatchdogState::AwaitingWork { .. }
                        if control.generation == token.callback_generation
                            && (matches!(binding, ProviderBinding::Stage) || matches!(control.state, WatchdogState::Scheduled { .. })
                                || control.scheduler_abandoned)
                ) =>
            {
                Ok(&mut self.wakeup)
            }
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogWork)
                if matches!(
                    control.state,
                    WatchdogState::AwaitingWork {
                        attempt_status: WatchdogAttemptStatus::Dispatched,
                        ..
                    } if control.generation == token.callback_generation
                ) =>
            {
                Ok(&mut self.work)
            }
            _ => Err(RegistryError::StaleCallback),
        }
    }

    fn record_arm(&mut self, token: &CallbackToken, delay_ns: u64) {
        if self.confirmed_wakeup_generation == Some(token.callback_generation) {
            return;
        }
        self.confirmed_wakeup_generation = Some(token.callback_generation);
        self.latest_armed_delay_ns = Some(delay_ns);
        let counters = self.observability.counters_mut();
        counters.record_wakeup_armed();
        if token.role == CallbackRole::WatchdogWork {
            counters.record_work_dispatched();
        }
    }

    const fn snapshot(&self, identity: TimerIdentity, epoch: TimerEpoch) -> TimerSnapshot {
        let state = match &self.kind {
            EntryKind::Ordinary { control, .. } => match control.registration {
                TimerRegistration::Inactive { reason } => {
                    TimerRuntimeStateSnapshot::Inactive { reason }
                }
                TimerRegistration::Scheduled { deadline_ns } => {
                    TimerRuntimeStateSnapshot::Ordinary(OrdinaryRuntimeStateSnapshot::Scheduled {
                        generation: control.generation(),
                        deadline_ns,
                    })
                }
                TimerRegistration::Running { .. } => {
                    TimerRuntimeStateSnapshot::Ordinary(OrdinaryRuntimeStateSnapshot::Running {
                        generation: control.generation(),
                    })
                }
            },
            EntryKind::Watchdog { control, .. } => match control.state {
                WatchdogState::Inactive { reason } => {
                    TimerRuntimeStateSnapshot::Inactive { reason }
                }
                WatchdogState::Scheduled { deadline_ns } => {
                    TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled {
                        scheduler_generation: control.generation,
                        deadline_ns,
                    })
                }
                WatchdogState::AwaitingWork {
                    successor_deadline_ns,
                    attempt_status,
                    ..
                } => TimerRuntimeStateSnapshot::Watchdog(
                    WatchdogRuntimeStateSnapshot::AwaitingWork {
                        successor_generation: control.generation,
                        successor_deadline_ns,
                        attempt_status,
                    },
                ),
            },
        };

        TimerSnapshot::new(
            identity,
            TimerRegistrationId::new(epoch, self.claim_generation),
            self.kind.policy(),
            self.lifetime,
            state,
            self.scheduling_mode,
            self.latest_directive,
            self.latest_requested_delay_ns,
            self.latest_armed_delay_ns,
            &self.observability,
        )
    }

    /// Decide terminal removal from the entry already selected and authorized
    /// by the transition owner. Runtime detaches handles before that transition.
    const fn remove_on_failure(&self, transition: &RegistryTransition) -> bool {
        transition.failure().is_some()
            && matches!(self.lifetime, DeclarationLifetime::RemoveWhenStopped)
    }

    fn take_wakeup_handle(&mut self) -> Option<TimerHandle> {
        self.wakeup.take().map(|owned| owned.handle)
    }

    fn take_work_handle(&mut self) -> Option<TimerHandle> {
        self.work.take().map(|owned| owned.handle)
    }

    fn take_provider_handles(&mut self) -> ProviderHandles {
        ProviderHandles {
            wakeup: self.take_wakeup_handle(),
            work: self.take_work_handle(),
        }
    }

    // The caller selected this entry by identity. Fired handles are consumed
    // independently of callback acceptance, but only for the exact claim,
    // policy role and owned slot generation.
    fn consume_provider_handle(&mut self, token: &CallbackToken) {
        if !self.owns_token_claim(token) {
            return;
        }
        let slot = match token.role {
            CallbackRole::OrdinaryWork | CallbackRole::WatchdogScheduler
                if token.role == self.kind.wakeup_role() =>
            {
                &mut self.wakeup
            }
            CallbackRole::WatchdogWork if matches!(self.kind, EntryKind::Watchdog { .. }) => {
                &mut self.work
            }
            _ => return,
        };
        if slot
            .as_ref()
            .is_some_and(|owned| owned.callback_generation == token.callback_generation)
        {
            *slot = None;
        }
    }

    const fn owns_token_claim(&self, token: &CallbackToken) -> bool {
        self.claim_generation == token.claim().claim_generation()
    }

    // Registry lookups select this entry by token identity first.
    const fn owns_running_work(&self, token: &CallbackToken) -> bool {
        if !self.owns_token_claim(token) {
            return false;
        }
        match (&self.kind, token.role) {
            (EntryKind::Ordinary { control, .. }, CallbackRole::OrdinaryWork) => matches!(
                control.registration,
                TimerRegistration::Running { .. }
                    if control.generation() == token.callback_generation
            ),
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogWork) => matches!(
                control.state,
                WatchdogState::AwaitingWork {
                    attempt_status: WatchdogAttemptStatus::Running,
                    ..
                } if control.generation == token.callback_generation
            ),
            (
                EntryKind::Ordinary { .. },
                CallbackRole::WatchdogScheduler | CallbackRole::WatchdogWork,
            )
            | (
                EntryKind::Watchdog { .. },
                CallbackRole::OrdinaryWork | CallbackRole::WatchdogScheduler,
            ) => false,
        }
    }
}

/// Pure, fixed-capacity canonical registry.
pub struct TimerRegistry {
    epoch: TimerEpoch,
    next_claim_generation: u64,
    entries: BTreeMap<TimerIdentity, Entry>,
}

impl TimerRegistry {
    pub(crate) const fn new(epoch: TimerEpoch) -> Self {
        Self {
            epoch,
            next_claim_generation: 0,
            entries: BTreeMap::new(),
        }
    }

    pub(crate) const fn epoch(&self) -> TimerEpoch {
        self.epoch
    }

    pub(crate) fn register_once_with_callback(
        &mut self,
        identity: TimerIdentity,
        lifetime: DeclarationLifetime,
        callback: OrdinaryCallback,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register(identity, EntryKind::ordinary(None, callback), lifetime)
    }

    pub(crate) fn register_after_completion_with_callback(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
        callback: OrdinaryCallback,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register(
            identity,
            EntryKind::ordinary(Some(cadence), callback),
            lifetime,
        )
    }

    pub(crate) fn register_watchdog_with_callback(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
        callback: WatchdogCallback,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register(identity, EntryKind::watchdog(cadence, callback), lifetime)
    }

    fn register(
        &mut self,
        identity: TimerIdentity,
        kind: EntryKind,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        if self.entries.contains_key(&identity) {
            return Err(RegisterError::IdentityAlreadyRegistered(identity));
        }
        if self.entries.len() == MAX_TIMER_REGISTRATIONS {
            return Err(RegisterError::CapacityExceeded {
                max: MAX_TIMER_REGISTRATIONS,
            });
        }
        let claim_generation = self
            .next_claim_generation
            .checked_add(1)
            .ok_or(RegisterError::ClaimGenerationExhausted)?;

        self.next_claim_generation = claim_generation;
        self.entries.insert(
            identity.clone(),
            Entry::new(claim_generation, kind, lifetime),
        );
        Ok(RegistrationClaim {
            identity,
            claim_generation,
        })
    }

    /// Apply public control with handles left in place for no-op and rejected
    /// requests. Removed entries carry their handles and captures out together.
    pub(crate) fn transition_claim(
        &mut self,
        claim: &RegistrationClaim,
        operation: impl FnOnce(&mut Self) -> Result<RegistryTransition, RegistryError>,
    ) -> Result<(RegistryTransition, ProviderHandles), RegistryError> {
        self.entry(claim)?;
        let mut transition = operation(self)?;
        let handles = self.detach_transition_handles(claim.identity(), &mut transition);
        Ok((transition, handles))
    }

    /// Leave unaffected capabilities bound. Detached handles belong solely to
    /// cleanup; removed declarations must release every handle before captures.
    pub(crate) fn detach_transition_handles(
        &mut self,
        identity: &TimerIdentity,
        transition: &mut RegistryTransition,
    ) -> ProviderHandles {
        if let Some(entry) = &mut transition.removed {
            return entry.take_provider_handles();
        }
        let selected = match transition.effect() {
            RegistryEffect::None
            | RegistryEffect::ArmWakeup {
                arm: WakeupArm::Initial,
                ..
            } => return ProviderHandles::default(),
            RegistryEffect::ArmWakeup {
                arm: WakeupArm::Replacement,
                ..
            } => CallbacksToClear::Wakeup,
            RegistryEffect::ClearCallbacks { handles } => *handles,
            RegistryEffect::DispatchWatchdog { .. } => CallbacksToClear::WakeupAndWork,
        };
        let Some(entry) = self.entries.get_mut(identity) else {
            return ProviderHandles::default();
        };
        ProviderHandles {
            wakeup: if selected.includes_wakeup() {
                entry.take_wakeup_handle()
            } else {
                None
            },
            work: if selected.includes_work() {
                entry.take_work_handle()
            } else {
                None
            },
        }
    }

    /// Apply an initial schedule already checked before claim allocation.
    pub(crate) fn reconcile_initial(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        requested: ResolvedSchedule,
    ) -> Result<RegistryTransition, RegistryError> {
        match self.entry(claim)?.kind {
            EntryKind::Ordinary { .. } => {
                self.request_ordinary(claim, now_ns, requested, OrdinaryRequest::Reconcile)
            }
            EntryKind::Watchdog { .. } => {
                self.ensure_watchdog(claim, now_ns, WatchdogScheduleRequest::Reconcile(requested))
            }
        }
    }

    pub(crate) fn ensure_once(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        schedule: TimerSchedule,
    ) -> Result<RegistryTransition, RegistryError> {
        let requested = schedule.resolve(now_ns)?;
        let entry = self.entry(claim)?;
        if !matches!(entry.kind, EntryKind::Ordinary { cadence: None, .. }) {
            return Err(RegistryError::PolicyMismatch {
                actual: entry.kind.policy().label(),
            });
        }
        self.request_ordinary(claim, now_ns, requested, OrdinaryRequest::Ensure)
    }

    /// Reconcile an ordinary declaration to one exact desired schedule.
    ///
    /// Unlike `ensure`, reconciliation may move an existing deadline later.
    /// `None` stops demand without recording explicit cancellation. Declaration
    /// lifetime determines whether callback authority remains registered.
    pub(crate) fn reconcile_ordinary(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        schedule: Option<TimerSchedule>,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry(claim)?;
        if !matches!(entry.kind, EntryKind::Ordinary { .. }) {
            return Err(RegistryError::PolicyMismatch {
                actual: entry.kind.policy().label(),
            });
        }
        let Some(schedule) = schedule else {
            return self.reconcile_ordinary_inactive(claim);
        };
        self.request_ordinary(
            claim,
            now_ns,
            schedule.resolve(now_ns)?,
            OrdinaryRequest::Reconcile,
        )
    }

    pub(crate) fn ensure_recurring(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry(claim)?;
        match &entry.kind {
            EntryKind::Ordinary { cadence: None, .. } => {
                Err(RegistryError::PolicyMismatch { actual: "once" })
            }
            EntryKind::Ordinary {
                cadence: Some(cadence),
                control,
                ..
            } => {
                let cadence = *cadence;
                // Existing authority satisfies recurrence without calculating
                // an unused successor, even when now + cadence would overflow.
                let deadline_ns = match control.registration {
                    TimerRegistration::Scheduled { deadline_ns } => deadline_ns,
                    TimerRegistration::Inactive { .. } | TimerRegistration::Running { .. } => {
                        cadence.deadline_after(now_ns)?
                    }
                };
                self.request_ordinary(
                    claim,
                    now_ns,
                    ResolvedSchedule {
                        deadline_ns,
                        requested_delay_ns: Some(cadence.as_nanos()),
                        mode: TimerSchedulingMode::AfterCompletion,
                    },
                    OrdinaryRequest::Ensure,
                )
            }
            EntryKind::Watchdog { .. } => {
                self.ensure_watchdog(claim, now_ns, WatchdogScheduleRequest::Cadence)
            }
        }
    }

    pub(crate) fn ensure_watchdog_immediately(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
    ) -> Result<RegistryTransition, RegistryError> {
        self.ensure_watchdog(claim, now_ns, WatchdogScheduleRequest::Immediate)
    }

    pub(crate) fn reconcile_watchdog_schedule(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        schedule: Option<TimerSchedule>,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry(claim)?;
        if !matches!(entry.kind, EntryKind::Watchdog { .. }) {
            return Err(RegistryError::PolicyMismatch {
                actual: entry.kind.policy().label(),
            });
        }
        let Some(schedule) = schedule else {
            return self.cancel(claim);
        };
        let requested = schedule.resolve(now_ns)?;
        self.ensure_watchdog(claim, now_ns, WatchdogScheduleRequest::Reconcile(requested))
    }

    fn request_ordinary(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        requested: ResolvedSchedule,
        request: OrdinaryRequest,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry_mut(claim)?;
        let actual = entry.kind.policy().label();
        let EntryKind::Ordinary { control, .. } = &mut entry.kind else {
            return Err(RegistryError::PolicyMismatch { actual });
        };
        entry.observability.counters_mut().record_schedule_request();
        entry.latest_requested_delay_ns = requested.requested_delay_ns;

        let arm = match request {
            OrdinaryRequest::Ensure => control.schedule(requested.deadline_ns),
            OrdinaryRequest::Reconcile => control.reconcile(requested.deadline_ns),
        };

        let arm = match arm {
            Ok(arm) => arm,
            Err(error) => {
                let clear_wakeup = control.terminate(InactiveReason::ControlFailure(error));
                let transition = RegistryTransition::terminal(clear_wakeup_if(clear_wakeup), error);
                let remove = entry.remove_on_failure(&transition);
                return Ok(remove_after(
                    &mut self.entries,
                    claim.identity(),
                    transition,
                    remove,
                ));
            }
        };

        if let TimerRegistration::Running { pending } = &mut control.registration {
            *pending = Some(select_pending_ordinary(*pending, request, requested));
        }
        if arm.is_some() || matches!(request, OrdinaryRequest::Reconcile) {
            entry.scheduling_mode = requested.mode;
        }

        let Some(arm) = arm else {
            entry.observability.counters_mut().record_coalesced();
            return Ok(RegistryTransition::normal(RegistryEffect::None));
        };
        Ok(RegistryTransition::normal(RegistryEffect::ArmWakeup {
            token: WakeupToken::ordinary(claim, control.generation()),
            delay_ns: requested.deadline_ns.saturating_sub(now_ns),
            arm,
        }))
    }

    fn ensure_watchdog(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        request: WatchdogScheduleRequest,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry_mut(claim)?;
        let actual = entry.kind.policy().label();
        let EntryKind::Watchdog {
            cadence, control, ..
        } = &mut entry.kind
        else {
            return Err(RegistryError::PolicyMismatch { actual });
        };
        let requested = request.resolve(control.state, *cadence, now_ns)?;
        let transition = match &mut control.state {
            WatchdogState::Inactive { .. } => {
                control.arm_scheduler(claim, now_ns, requested.deadline_ns, WakeupArm::Initial)
            }
            WatchdogState::Scheduled { deadline_ns }
                if control.scheduler_abandoned || *deadline_ns != requested.deadline_ns =>
            {
                let arm = if control.scheduler_abandoned {
                    WakeupArm::Initial
                } else {
                    WakeupArm::Replacement
                };
                control.arm_scheduler(claim, now_ns, requested.deadline_ns, arm)
            }
            WatchdogState::Scheduled { .. } => RegistryTransition::normal(RegistryEffect::None),
            WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Dispatched,
                pending,
                ..
            } => {
                if let WatchdogScheduleRequest::Reconcile(requested) = request {
                    *pending = Some(WatchdogPending::Reconcile(requested));
                }
                RegistryTransition::normal(RegistryEffect::None)
            }
            WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Running,
                pending,
                ..
            } => {
                *pending = Some(select_pending_watchdog(*pending, request, now_ns));
                RegistryTransition::normal(RegistryEffect::None)
            }
        };
        let transition = if control.scheduler_abandoned
            && let WatchdogState::AwaitingWork {
                successor_deadline_ns,
                ..
            } = control.state
        {
            // Keep the current work attempt and its committed recovery deadline.
            // Reconciliation can select its eventual successor through pending;
            // it must not duplicate work that might already be queued.
            RegistryTransition::normal(RegistryEffect::ArmWakeup {
                token: WakeupToken::scheduler(claim, control.generation),
                delay_ns: successor_deadline_ns.saturating_sub(now_ns),
                arm: WakeupArm::Initial,
            })
        } else {
            transition
        };
        if matches!(transition.effect(), RegistryEffect::ArmWakeup { .. })
            && matches!(control.state, WatchdogState::Scheduled { .. })
        {
            entry.scheduling_mode = requested.mode;
        }
        if matches!(
            (transition.effect(), transition.failure()),
            (RegistryEffect::None, None)
        ) {
            entry.observability.counters_mut().record_coalesced();
        }
        entry.observability.counters_mut().record_schedule_request();
        entry.latest_requested_delay_ns = requested.requested_delay_ns;
        let remove = entry.remove_on_failure(&transition);
        Ok(remove_after(
            &mut self.entries,
            claim.identity(),
            transition,
            remove,
        ))
    }

    fn reconcile_ordinary_inactive(
        &mut self,
        claim: &RegistrationClaim,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry_mut(claim)?;
        let EntryKind::Ordinary { control, .. } = &mut entry.kind else {
            return Err(RegistryError::PolicyMismatch {
                actual: entry.kind.policy().label(),
            });
        };
        let effect = match &mut control.registration {
            TimerRegistration::Inactive { .. } => RegistryEffect::None,
            TimerRegistration::Scheduled { .. } => {
                control.terminate(InactiveReason::Stopped);
                RegistryEffect::ClearCallbacks {
                    handles: CallbacksToClear::Wakeup,
                }
            }
            TimerRegistration::Running { pending } => {
                if !matches!(*pending, Some(OrdinaryPending::Unregister)) {
                    *pending = Some(OrdinaryPending::ReconcileInactive);
                }
                RegistryEffect::None
            }
        };
        let remove = matches!(control.registration, TimerRegistration::Inactive { .. })
            && matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
        Ok(remove_after(
            &mut self.entries,
            claim.identity(),
            RegistryTransition::normal(effect),
            remove,
        ))
    }

    pub(crate) fn cancel(
        &mut self,
        claim: &RegistrationClaim,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = claim.identity();
        let (effect, remove) = {
            let entry = self.entry_mut(claim)?;
            let (effect, stopped) = match &mut entry.kind {
                EntryKind::Ordinary { control, .. } => {
                    let effect = match &mut control.registration {
                        TimerRegistration::Inactive { .. } => RegistryEffect::None,
                        TimerRegistration::Scheduled { .. } => {
                            control.terminate(InactiveReason::Cancelled);
                            entry.observability.counters_mut().record_cancellation();
                            RegistryEffect::ClearCallbacks {
                                handles: CallbacksToClear::Wakeup,
                            }
                        }
                        TimerRegistration::Running { pending } => {
                            if !matches!(*pending, Some(OrdinaryPending::Unregister)) {
                                *pending = Some(OrdinaryPending::Cancel);
                            }
                            RegistryEffect::None
                        }
                    };
                    (
                        effect,
                        matches!(control.registration, TimerRegistration::Inactive { .. }),
                    )
                }
                EntryKind::Watchdog { control, .. } => {
                    let handles = match &mut control.state {
                        WatchdogState::Inactive { .. } => None,
                        WatchdogState::Scheduled { .. } => Some(CallbacksToClear::Wakeup),
                        WatchdogState::AwaitingWork {
                            attempt_status: WatchdogAttemptStatus::Dispatched,
                            ..
                        } => Some(CallbacksToClear::WakeupAndWork),
                        WatchdogState::AwaitingWork {
                            attempt_status: WatchdogAttemptStatus::Running,
                            pending,
                            ..
                        } => {
                            if !matches!(*pending, Some(WatchdogPending::Unregister)) {
                                *pending = Some(WatchdogPending::Cancel);
                            }
                            None
                        }
                    };
                    let effect = handles.map_or(RegistryEffect::None, |handles| {
                        control.state = WatchdogState::Inactive {
                            reason: InactiveReason::Cancelled,
                        };
                        entry.observability.counters_mut().record_cancellation();
                        RegistryEffect::ClearCallbacks { handles }
                    });
                    let stopped = matches!(control.state, WatchdogState::Inactive { .. });
                    (effect, stopped)
                }
            };
            (
                effect,
                stopped && matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped),
            )
        };

        Ok(remove_after(
            &mut self.entries,
            identity,
            RegistryTransition::normal(effect),
            remove,
        ))
    }

    /// Remove the declaration owned by one exact logical claim.
    ///
    /// Removal requested by running work is finalized by normal completion or,
    /// for ordinary work, confirmed delivery abandonment. A request made in a
    /// trapping message rolls back with that message's heap mutations; an
    /// already committed pending request remains eligible for finalization.
    pub(crate) fn unregister(
        &mut self,
        claim: &RegistrationClaim,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = claim.identity();
        let entry = self.entry_mut(claim)?;
        match &mut entry.kind {
            EntryKind::Ordinary {
                control:
                    TimerControl {
                        registration: TimerRegistration::Running { pending },
                        ..
                    },
                ..
            } => {
                *pending = Some(OrdinaryPending::Unregister);
                Ok(RegistryTransition::normal(RegistryEffect::None))
            }
            EntryKind::Watchdog {
                control:
                    WatchdogControl {
                        state:
                            WatchdogState::AwaitingWork {
                                attempt_status: WatchdogAttemptStatus::Running,
                                pending,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                *pending = Some(WatchdogPending::Unregister);
                Ok(RegistryTransition::normal(RegistryEffect::None))
            }
            EntryKind::Ordinary { .. } | EntryKind::Watchdog { .. } => {
                let transition = self.cancel(claim)?;
                Ok(remove_after(&mut self.entries, identity, transition, true))
            }
        }
    }

    /// Consume this delivery's handle, then accept ordinary work and return its
    /// callback.
    pub(crate) fn begin_ordinary(&mut self, token: &CallbackToken) -> Option<OrdinaryCallback> {
        let entry = self.entries.get_mut(token.identity())?;
        entry.consume_provider_handle(token);
        if token.role != CallbackRole::OrdinaryWork || !entry.owns_token_claim(token) {
            entry.observability.counters_mut().record_stale_wakeup();
            return None;
        }
        let EntryKind::Ordinary {
            control, callback, ..
        } = &mut entry.kind
        else {
            entry.observability.counters_mut().record_stale_wakeup();
            return None;
        };
        if control.begin(token.callback_generation) {
            entry.observability.counters_mut().record_work_started();
            Some(Rc::clone(callback))
        } else {
            entry.observability.counters_mut().record_stale_wakeup();
            None
        }
    }

    pub(crate) fn complete_ordinary(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
        result: OrdinaryRunResult,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = token.identity();
        let (transition, remove) = {
            let entry = self.running_work_entry_mut(token)?;
            let EntryKind::Ordinary {
                cadence, control, ..
            } = &mut entry.kind
            else {
                return Err(RegistryError::StaleCallback);
            };
            let cadence = *cadence;
            let completion = result.completion();
            let pending_command = control.pending();
            let remove_on_stop = matches!(pending_command, Some(OrdinaryPending::Unregister))
                || matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
            if completion.outcome() == TimerCompletionOutcome::InvariantFailure {
                let transition = stop_ordinary_completion(entry, completion, now_ns, None);
                (transition, remove_on_stop)
            } else {
                let prepared = prepare_ordinary_completion(
                    pending_command,
                    result.directive(),
                    now_ns,
                    cadence,
                )
                .and_then(|prepared| {
                    if let OrdinaryCompletion::Schedule { requested, .. } = prepared {
                        control.arm_deadline(requested.deadline_ns)?;
                    }
                    Ok(prepared)
                });
                match prepared {
                    Err(failure) => (
                        stop_ordinary_completion(entry, completion, now_ns, Some(failure)),
                        remove_on_stop,
                    ),
                    Ok(prepared) => {
                        entry.latest_directive = Some(prepared.directive());
                        entry.observability.record_completion(completion, now_ns);

                        let remove = !matches!(prepared, OrdinaryCompletion::Schedule { .. })
                            && remove_on_stop;
                        let transition = match prepared {
                            OrdinaryCompletion::Schedule { requested, .. } => {
                                entry.scheduling_mode = requested.mode;
                                entry.latest_requested_delay_ns = requested.requested_delay_ns;
                                RegistryTransition::normal(RegistryEffect::ArmWakeup {
                                    token: WakeupToken::ordinary(
                                        token.claim(),
                                        control.generation(),
                                    ),
                                    delay_ns: requested.deadline_ns.saturating_sub(now_ns),
                                    arm: WakeupArm::Initial,
                                })
                            }
                            OrdinaryCompletion::Stop | OrdinaryCompletion::Cancel => {
                                let reason = if matches!(prepared, OrdinaryCompletion::Cancel) {
                                    entry.observability.counters_mut().record_cancellation();
                                    InactiveReason::Cancelled
                                } else {
                                    InactiveReason::Stopped
                                };
                                control.terminate(reason);
                                RegistryTransition::normal(RegistryEffect::None)
                            }
                        };
                        (transition, remove)
                    }
                }
            }
        };

        Ok(remove_after(
            &mut self.entries,
            identity,
            transition,
            remove,
        ))
    }

    /// Whether a confirmed ordinary delivery still owns scheduled or running work.
    /// Detached scheduled handles belong to their synchronous transition owner.
    pub(crate) fn owns_ordinary_delivery(&self, token: &CallbackToken) -> bool {
        let Some(entry) = self.entries.get(token.identity()) else {
            return false;
        };
        let EntryKind::Ordinary { control, .. } = &entry.kind else {
            return false;
        };
        token.role == CallbackRole::OrdinaryWork
            && entry.owns_token_claim(token)
            && control.generation() == token.callback_generation
            && entry.confirmed_wakeup_generation == Some(token.callback_generation)
            && match control.registration {
                TimerRegistration::Running { .. } => true,
                TimerRegistration::Scheduled { .. } => entry
                    .wakeup
                    .as_ref()
                    .is_some_and(|owned| owned.callback_generation == token.callback_generation),
                TimerRegistration::Inactive { .. } => false,
            }
    }

    /// Retire a dropped delivery without provider calls or fabricated completion.
    /// Return removed callback ownership so capture destructors run after the
    /// runtime releases its registry borrow, including during CDK cleanup.
    pub(crate) fn abandon_ordinary(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
    ) -> Option<OrdinaryCallback> {
        if !self.owns_ordinary_delivery(token) {
            return None;
        }
        let entry = self.entries.get_mut(token.identity())?;
        let EntryKind::Ordinary { control, .. } = &mut entry.kind else {
            return None;
        };
        let remove = matches!(control.pending(), Some(OrdinaryPending::Unregister))
            || matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
        control.terminate(InactiveReason::Abandoned);
        // The provider/executor is already dropping this future. Forget the
        // consumed handle; calling clear_timer here could reborrow its task map.
        entry.wakeup = None;
        entry.observability.record_unacknowledged(now_ns);
        if !remove {
            return None;
        }
        self.entries.remove(token.identity()).and_then(|entry| {
            if let EntryKind::Ordinary { callback, .. } = entry.kind {
                Some(callback)
            } else {
                None
            }
        })
    }

    pub(crate) fn owns_scheduler_delivery(&self, token: &CallbackToken) -> bool {
        self.entries.get(token.identity()).is_some_and(|entry| {
            token.role == CallbackRole::WatchdogScheduler
                && entry.owns_token_claim(token)
                && matches!(&entry.kind, EntryKind::Watchdog { control, .. }
                    if control.generation == token.callback_generation
                        && !matches!(control.state, WatchdogState::Inactive { .. }))
                && entry
                    .wakeup
                    .as_ref()
                    .is_some_and(|owned| owned.callback_generation == token.callback_generation)
        })
    }

    /// Record loss without provider calls, changing work authority or fabricating
    /// completion. Explicit owner reconciliation can rearm this scheduler later.
    pub(crate) fn abandon_scheduler(&mut self, token: &CallbackToken) {
        if !self.owns_scheduler_delivery(token) {
            return;
        }
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return;
        };
        if let EntryKind::Watchdog { control, .. } = &mut entry.kind {
            entry.wakeup = None;
            control.scheduler_abandoned = true;
        }
    }

    pub(crate) fn begin_watchdog_scheduler(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
    ) -> RegistryTransition {
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return RegistryTransition::normal(RegistryEffect::None);
        };
        if token.role != CallbackRole::WatchdogScheduler || !entry.owns_token_claim(token) {
            entry.observability.counters_mut().record_stale_wakeup();
            return RegistryTransition::normal(RegistryEffect::None);
        }
        let EntryKind::Watchdog {
            cadence, control, ..
        } = &mut entry.kind
        else {
            entry.observability.counters_mut().record_stale_wakeup();
            return RegistryTransition::normal(RegistryEffect::None);
        };
        let cadence = *cadence;

        let accepted = match control.state {
            WatchdogState::Scheduled { .. }
            | WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Dispatched,
                ..
            } => control.generation == token.callback_generation,
            WatchdogState::Inactive { .. }
            | WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Running,
                ..
            } => false,
        };
        if !accepted {
            entry.observability.counters_mut().record_stale_wakeup();
            return RegistryTransition::normal(RegistryEffect::None);
        }

        entry
            .observability
            .counters_mut()
            .record_scheduler_started();
        if matches!(control.state, WatchdogState::AwaitingWork { .. }) {
            entry.observability.record_unacknowledged(now_ns);
        }
        let next_dispatch = control
            .generation
            .checked_add(1)
            .ok_or(TimerControlFailure::GenerationExhausted)
            .and_then(|generation| {
                cadence
                    .deadline_after(now_ns)
                    .map(|deadline_ns| (generation, deadline_ns))
                    .map_err(|_| TimerControlFailure::DeadlineOverflow)
            });
        let (generation, successor_deadline_ns) = match next_dispatch {
            Ok(next) => next,
            Err(failure) => {
                let transition = control.terminate(
                    RegistryEffect::ClearCallbacks {
                        handles: CallbacksToClear::Work,
                    },
                    failure,
                );
                let remove = entry.remove_on_failure(&transition);
                return remove_after(&mut self.entries, token.identity(), transition, remove);
            }
        };

        control.generation = generation;
        control.scheduler_abandoned = false;
        control.state = WatchdogState::AwaitingWork {
            successor_deadline_ns,
            attempt_status: WatchdogAttemptStatus::Dispatched,
            pending: None,
        };
        entry.scheduling_mode = TimerSchedulingMode::Watchdog;
        RegistryTransition::normal(RegistryEffect::DispatchWatchdog {
            dispatch: WatchdogDispatch::new(token.claim(), generation),
            successor_delay_ns: cadence.as_nanos(),
        })
    }

    /// Consume this delivery's work handle, then accept Watchdog work and return
    /// its callback.
    pub(crate) fn begin_watchdog_work(
        &mut self,
        token: &CallbackToken,
    ) -> Option<WatchdogCallback> {
        let entry = self.entries.get_mut(token.identity())?;
        entry.consume_provider_handle(token);
        if token.role != CallbackRole::WatchdogWork || !entry.owns_token_claim(token) {
            entry.observability.counters_mut().record_stale_work();
            return None;
        }
        let EntryKind::Watchdog {
            control, callback, ..
        } = &mut entry.kind
        else {
            entry.observability.counters_mut().record_stale_work();
            return None;
        };
        match &mut control.state {
            WatchdogState::AwaitingWork { attempt_status, .. }
                if control.generation == token.callback_generation
                    && *attempt_status == WatchdogAttemptStatus::Dispatched =>
            {
                *attempt_status = WatchdogAttemptStatus::Running;
                entry.observability.counters_mut().record_work_started();
                Some(Rc::clone(callback))
            }
            WatchdogState::Inactive { .. }
            | WatchdogState::Scheduled { .. }
            | WatchdogState::AwaitingWork { .. } => {
                entry.observability.counters_mut().record_stale_work();
                None
            }
        }
    }

    pub(crate) fn complete_watchdog_work(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
        result: WatchdogRunResult,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = token.identity();
        let (transition, remove) = {
            let entry = self.running_work_entry_mut(token)?;
            let EntryKind::Watchdog {
                cadence, control, ..
            } = &mut entry.kind
            else {
                return Err(RegistryError::StaleCallback);
            };
            let cadence = *cadence;
            let WatchdogState::AwaitingWork {
                successor_deadline_ns,
                pending,
                ..
            } = control.state
            else {
                return Err(RegistryError::StaleCallback);
            };

            let completion = result.completion();
            entry.observability.record_completion(completion, now_ns);
            let transition = match prepare_watchdog_completion(
                pending,
                result,
                now_ns,
                successor_deadline_ns,
                cadence,
            ) {
                WatchdogCompletion::Schedule {
                    requested,
                    retain_successor,
                } => {
                    entry.scheduling_mode = requested.mode;
                    entry.latest_requested_delay_ns = requested.requested_delay_ns;
                    if retain_successor {
                        control.state = WatchdogState::Scheduled {
                            deadline_ns: successor_deadline_ns,
                        };
                        RegistryTransition::normal(RegistryEffect::None)
                    } else {
                        control.arm_scheduler(
                            token.claim(),
                            now_ns,
                            requested.deadline_ns,
                            WakeupArm::Replacement,
                        )
                    }
                }
                WatchdogCompletion::Stop(reason) => {
                    if reason == InactiveReason::Cancelled {
                        entry.observability.counters_mut().record_cancellation();
                    }
                    control.state = WatchdogState::Inactive { reason };
                    RegistryTransition::normal(RegistryEffect::ClearCallbacks {
                        handles: CallbacksToClear::Wakeup,
                    })
                }
            };
            let remove = matches!(control.state, WatchdogState::Inactive { .. })
                && (matches!(pending, Some(WatchdogPending::Unregister))
                    || matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped));
            (transition, remove)
        };

        Ok(remove_after(
            &mut self.entries,
            identity,
            transition,
            remove,
        ))
    }

    pub(crate) fn snapshot(&self, identity: &TimerIdentity) -> Option<TimerSnapshot> {
        self.entries
            .get(identity)
            .map(|entry| entry.snapshot(identity.clone(), self.epoch))
    }

    pub(crate) fn inventory(&self) -> TimerInventorySnapshot {
        TimerInventorySnapshot::new(
            self.epoch,
            self.entries
                .iter()
                .map(|(identity, entry)| entry.snapshot(identity.clone(), self.epoch))
                .collect(),
        )
    }

    pub(crate) fn consecutive_expected_failures(&self, identity: &TimerIdentity) -> Option<u64> {
        self.entries.get(identity).map(|entry| {
            entry
                .observability
                .outcomes()
                .consecutive_expected_failures()
        })
    }

    pub(crate) fn has_armed_wakeup(
        &self,
        claim: &RegistrationClaim,
    ) -> Result<bool, RegistryError> {
        Ok(self.entry(claim)?.wakeup.is_some())
    }

    pub(crate) fn declaration_matches(
        &self,
        claim: &RegistrationClaim,
        policy: TimerPolicy,
        lifetime: DeclarationLifetime,
    ) -> Result<bool, RegistryError> {
        let entry = self.entry(claim)?;
        Ok(entry.kind.policy() == policy && entry.lifetime == lifetime)
    }

    pub(crate) fn record_callback_measurements(
        &mut self,
        token: &CallbackToken,
        instructions: u64,
        memory_start: MemoryPageExtent,
        memory_end: MemoryPageExtent,
    ) -> Result<(), RegistryError> {
        // A normal remove-on-stop completion can delete its entry before the
        // post-run measurement is committed, leaving nothing to observe.
        // Identity reuse must not let a late callback write into a newer
        // registration's observations.
        let Ok(entry) = self.entry_mut(token.claim()) else {
            return Ok(());
        };

        let memory = MemoryPageSample::new(memory_start, memory_end);
        match (&entry.kind, token.role) {
            (EntryKind::Watchdog { .. }, CallbackRole::WatchdogScheduler) => entry
                .observability
                .record_scheduler_measurements(instructions, memory),
            (EntryKind::Ordinary { .. }, CallbackRole::OrdinaryWork)
            | (EntryKind::Watchdog { .. }, CallbackRole::WatchdogWork) => entry
                .observability
                .record_work_measurements(instructions, memory),
            _ => {
                return Err(RegistryError::PolicyMismatch {
                    actual: entry.kind.policy().label(),
                });
            }
        }
        Ok(())
    }

    pub(crate) fn fail_registration(
        &mut self,
        claim: &RegistrationClaim,
        failure: TimerControlFailure,
    ) -> Result<(ProviderHandles, RegistryTransition), RegistryError> {
        let identity = claim.identity();
        let (handles, remove) = {
            let entry = self.entry_mut(claim)?;
            match &mut entry.kind {
                EntryKind::Ordinary { control, .. } => {
                    control.terminate(InactiveReason::ControlFailure(failure));
                }
                EntryKind::Watchdog { control, .. } => {
                    control.state = WatchdogState::Inactive {
                        reason: InactiveReason::ControlFailure(failure),
                    };
                }
            }
            let handles = entry.take_provider_handles();
            (
                handles,
                matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped),
            )
        };
        let transition = remove_after(
            &mut self.entries,
            identity,
            RegistryTransition::normal(RegistryEffect::None),
            remove,
        );
        Ok((handles, transition))
    }

    pub(crate) fn validate_running_context(
        &self,
        token: &CallbackToken,
    ) -> Result<(), RegistryError> {
        let entry = self
            .entries
            .get(token.identity())
            .ok_or(RegistryError::StaleCallback)?;
        if entry.owns_running_work(token) {
            Ok(())
        } else {
            Err(RegistryError::StaleCallback)
        }
    }

    /// Stage a provider binding or commit its successful-arm facts explicitly.
    pub(crate) fn install_provider_handle(
        &mut self,
        token: &CallbackToken,
        handle: TimerHandle,
        binding: ProviderBinding,
    ) -> Result<(), (RegistryError, TimerHandle)> {
        let entry = match self.entry_by_token_mut(token) {
            Ok(entry) => entry,
            Err(error) => return Err((error, handle)),
        };
        let slot = match entry.provider_handle_slot(token, binding) {
            Ok(slot) => slot,
            Err(error) => return Err((error, handle)),
        };
        if slot.is_some() {
            return Err((RegistryError::ProviderHandleAlreadyOwned, handle));
        }
        *slot = Some(OwnedProviderHandle {
            callback_generation: token.callback_generation,
            handle,
        });
        if token.role == CallbackRole::WatchdogScheduler
            && let EntryKind::Watchdog { control, .. } = &mut entry.kind
        {
            control.scheduler_abandoned = false;
        }
        if let ProviderBinding::Commit { delay_ns } = binding {
            entry.record_arm(token, delay_ns);
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn take_wakeup_handle(
        &mut self,
        identity: &TimerIdentity,
    ) -> Option<(CallbackToken, TimerHandle)> {
        let entry = self.entries.get_mut(identity)?;
        let owned = entry.wakeup.take()?;
        Some((
            CallbackToken::new(
                identity.clone(),
                entry.claim_generation,
                owned.callback_generation,
                entry.kind.wakeup_role(),
            ),
            owned.handle,
        ))
    }

    pub(crate) fn take_provider_handles(&mut self, identity: &TimerIdentity) -> ProviderHandles {
        self.entries
            .get_mut(identity)
            .map_or_else(ProviderHandles::default, |entry| {
                entry.take_provider_handles()
            })
    }

    /// Consume the delivered scheduler before transition/removal, then detach
    /// only capabilities that the selected effect must clear.
    pub(crate) fn transition_watchdog_scheduler(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
    ) -> (RegistryTransition, ProviderHandles) {
        // Removed or superseded claims are normal stale callback delivery.
        if let Ok(entry) = self.entry_mut(token.claim()) {
            entry.consume_provider_handle(token);
        }
        let mut transition = self.begin_watchdog_scheduler(token, now_ns);
        let handles = self.detach_transition_handles(token.identity(), &mut transition);
        (transition, handles)
    }

    /// Model successful platform application in provider-neutral transition tests.
    #[cfg(test)]
    pub(crate) fn confirm_effect_applied(
        &mut self,
        effect: &RegistryEffect,
    ) -> Result<(), RegistryError> {
        let (token, delay_ns) = match effect {
            RegistryEffect::None | RegistryEffect::ClearCallbacks { .. } => return Ok(()),
            RegistryEffect::ArmWakeup {
                token, delay_ns, ..
            } => (token.token().clone(), *delay_ns),
            RegistryEffect::DispatchWatchdog {
                dispatch,
                successor_delay_ns,
            } => (dispatch.work(), *successor_delay_ns),
        };
        let entry = self.entry_by_token_mut(&token)?;
        entry.provider_handle_slot(&token, ProviderBinding::Commit { delay_ns })?;
        entry.record_arm(&token, delay_ns);
        Ok(())
    }

    fn entry(&self, claim: &RegistrationClaim) -> Result<&Entry, RegistryError> {
        let entry = self
            .entries
            .get(claim.identity())
            .ok_or(RegistryError::UnknownRegistration)?;
        if entry.claim_generation != claim.claim_generation() {
            return Err(RegistryError::StaleRegistration);
        }
        Ok(entry)
    }

    fn entry_mut(&mut self, claim: &RegistrationClaim) -> Result<&mut Entry, RegistryError> {
        let entry = self
            .entries
            .get_mut(claim.identity())
            .ok_or(RegistryError::UnknownRegistration)?;
        if entry.claim_generation != claim.claim_generation() {
            return Err(RegistryError::StaleRegistration);
        }
        Ok(entry)
    }

    // Select identity and exact claim; callers validate the role and generation
    // against their operation's policy state.
    fn entry_by_token_mut(&mut self, token: &CallbackToken) -> Result<&mut Entry, RegistryError> {
        // Missing and superseded claims are both stale callback delivery.
        self.entry_mut(token.claim())
            .map_err(|_| RegistryError::StaleCallback)
    }

    fn running_work_entry_mut(
        &mut self,
        token: &CallbackToken,
    ) -> Result<&mut Entry, RegistryError> {
        let entry = self
            .entries
            .get_mut(token.identity())
            .ok_or(RegistryError::StaleCallback)?;
        if entry.owns_running_work(token) {
            Ok(entry)
        } else {
            Err(RegistryError::StaleCallback)
        }
    }
}

fn token_for(
    claim: &RegistrationClaim,
    callback_generation: u64,
    role: CallbackRole,
) -> CallbackToken {
    CallbackToken::new(
        claim.identity.clone(),
        claim.claim_generation,
        callback_generation,
        role,
    )
}

const fn clear_wakeup_if(clear_wakeup: bool) -> RegistryEffect {
    if clear_wakeup {
        RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Wakeup,
        }
    } else {
        RegistryEffect::None
    }
}

// Callers have validated the exact running generation. Checked completion
// failures leave that state unchanged before this shared stop finalization.
fn stop_ordinary_completion(
    entry: &mut Entry,
    completion: TimerCompletion,
    now_ns: u64,
    failure: Option<TimerControlFailure>,
) -> RegistryTransition {
    let (reason, completion, transition) = failure.map_or_else(
        || {
            (
                InactiveReason::InvariantFailure,
                completion,
                RegistryTransition::normal(RegistryEffect::None),
            )
        },
        |failure| {
            (
                InactiveReason::ControlFailure(failure),
                TimerCompletion::invariant_failure(completion.work_count()),
                RegistryTransition::terminal(RegistryEffect::None, failure),
            )
        },
    );
    let EntryKind::Ordinary { control, .. } = &mut entry.kind else {
        return transition;
    };
    control.terminate(reason);
    entry.latest_directive = Some(TimerDirectiveSnapshot::Stop);
    entry.observability.record_completion(completion, now_ns);
    transition
}

// Keep the selected action and its callback observation together. An ensure can
// select a pending schedule while retaining a callback's Stop observation.
enum OrdinaryCompletion {
    Schedule {
        requested: ResolvedSchedule,
        directive: TimerDirectiveSnapshot,
    },
    Stop,
    Cancel,
}

impl OrdinaryCompletion {
    const fn directive(&self) -> TimerDirectiveSnapshot {
        match self {
            Self::Schedule { directive, .. } => *directive,
            Self::Stop | Self::Cancel => TimerDirectiveSnapshot::Stop,
        }
    }
}

// Select authoritative commands before validating a callback proposal they discard.
fn prepare_ordinary_completion(
    pending: Option<OrdinaryPending>,
    directive: OrdinaryDirective,
    now_ns: u64,
    cadence: Option<TimerCadence>,
) -> Result<OrdinaryCompletion, TimerControlFailure> {
    match pending {
        Some(OrdinaryPending::ReconcileInactive) => Ok(OrdinaryCompletion::Stop),
        Some(OrdinaryPending::Cancel | OrdinaryPending::Unregister) => {
            Ok(OrdinaryCompletion::Cancel)
        }
        Some(OrdinaryPending::Reconcile(requested)) => Ok(OrdinaryCompletion::Schedule {
            requested,
            directive: TimerDirectiveSnapshot::ScheduleAt {
                deadline_ns: requested.deadline_ns,
            },
        }),
        Some(OrdinaryPending::Schedule(_)) | None => {
            let (callback, directive) = directive.resolve(now_ns, cadence)?;
            let selected = if let Some(OrdinaryPending::Schedule(pending)) = pending {
                match callback {
                    Some(callback) if callback.deadline_ns < pending.deadline_ns => Some(callback),
                    Some(_) | None => Some(pending),
                }
            } else {
                callback
            };
            Ok(selected.map_or(OrdinaryCompletion::Stop, |requested| {
                OrdinaryCompletion::Schedule {
                    requested,
                    directive,
                }
            }))
        }
    }
}

// Resolve the winner and its observation metadata together before mutating
// control. A pending exact schedule keeps its already validated relative origin.
enum WatchdogCompletion {
    Stop(InactiveReason),
    Schedule {
        requested: ResolvedSchedule,
        retain_successor: bool,
    },
}

fn prepare_watchdog_completion(
    pending: Option<WatchdogPending>,
    result: WatchdogRunResult,
    now_ns: u64,
    successor_deadline_ns: u64,
    cadence: TimerCadence,
) -> WatchdogCompletion {
    if result.completion().outcome() == TimerCompletionOutcome::InvariantFailure {
        return WatchdogCompletion::Stop(InactiveReason::InvariantFailure);
    }
    let decision = match pending {
        Some(WatchdogPending::Cancel) => {
            return WatchdogCompletion::Stop(InactiveReason::Cancelled);
        }
        Some(WatchdogPending::Unregister) => {
            return WatchdogCompletion::Stop(InactiveReason::Stopped);
        }
        Some(WatchdogPending::Reconcile(requested)) => {
            return WatchdogCompletion::Schedule {
                retain_successor: requested.deadline_ns == successor_deadline_ns,
                requested,
            };
        }
        Some(WatchdogPending::Ensure) => WatchdogDecision::Continue,
        Some(WatchdogPending::EnsureImmediately) => WatchdogDecision::ContinueImmediately,
        None => result.decision(),
    };
    let (requested, retain_successor) = match decision {
        WatchdogDecision::Stop => return WatchdogCompletion::Stop(InactiveReason::Stopped),
        WatchdogDecision::Continue => (
            ResolvedSchedule {
                deadline_ns: successor_deadline_ns,
                requested_delay_ns: Some(cadence.as_nanos()),
                mode: TimerSchedulingMode::Watchdog,
            },
            true,
        ),
        WatchdogDecision::ContinueImmediately => (
            ResolvedSchedule {
                deadline_ns: now_ns,
                requested_delay_ns: Some(0),
                mode: TimerSchedulingMode::Continuation,
            },
            successor_deadline_ns <= now_ns,
        ),
        WatchdogDecision::ScheduleAt(deadline_ns) => (
            ResolvedSchedule {
                deadline_ns,
                requested_delay_ns: None,
                mode: TimerSchedulingMode::Deadline,
            },
            successor_deadline_ns == deadline_ns,
        ),
    };
    WatchdogCompletion::Schedule {
        requested,
        retain_successor,
    }
}

const fn select_pending_ordinary(
    current: Option<OrdinaryPending>,
    request: OrdinaryRequest,
    requested: ResolvedSchedule,
) -> OrdinaryPending {
    match (current, request) {
        (Some(OrdinaryPending::Unregister), _) => OrdinaryPending::Unregister,
        (_, OrdinaryRequest::Reconcile) => OrdinaryPending::Reconcile(requested),
        (
            Some(OrdinaryPending::Reconcile(current) | OrdinaryPending::Schedule(current)),
            OrdinaryRequest::Ensure,
        ) if current.deadline_ns <= requested.deadline_ns => OrdinaryPending::Schedule(current),
        (_, OrdinaryRequest::Ensure) => OrdinaryPending::Schedule(requested),
    }
}

const fn select_pending_watchdog(
    pending: Option<WatchdogPending>,
    request: WatchdogScheduleRequest,
    now_ns: u64,
) -> WatchdogPending {
    match (pending, request) {
        (Some(WatchdogPending::Unregister), _) => WatchdogPending::Unregister,
        (_, WatchdogScheduleRequest::Reconcile(requested)) => WatchdogPending::Reconcile(requested),
        (Some(WatchdogPending::Reconcile(requested)), WatchdogScheduleRequest::Cadence) => {
            WatchdogPending::Reconcile(requested)
        }
        (Some(WatchdogPending::Reconcile(requested)), WatchdogScheduleRequest::Immediate)
            if requested.deadline_ns <= now_ns =>
        {
            WatchdogPending::Reconcile(requested)
        }
        (Some(WatchdogPending::EnsureImmediately), _) | (_, WatchdogScheduleRequest::Immediate) => {
            WatchdogPending::EnsureImmediately
        }
        (
            Some(WatchdogPending::Cancel | WatchdogPending::Ensure) | None,
            WatchdogScheduleRequest::Cadence,
        ) => WatchdogPending::Ensure,
    }
}

fn remove_after(
    entries: &mut BTreeMap<TimerIdentity, Entry>,
    identity: &TimerIdentity,
    mut transition: RegistryTransition,
    remove: bool,
) -> RegistryTransition {
    if remove && let Some(entry) = entries.remove(identity) {
        transition.removed = Some(Box::new(entry));
    }
    transition
}

#[cfg(test)]
mod tests;
