//! Shared declarations and admission for the installed timer runtime.

#[cfg(feature = "metrics")]
use crate::metrics::Event;
use crate::{
    api::{Error, Lifetime, OnceDemand, OnceNext, OnceResult, WatchdogDemand, WatchdogResult},
    bindings::{Bindings, Handle, RecoveryKey},
    engine::{Demand, State},
};
use ic_platform_runtime::system;
use std::{
    cell::{Cell, RefCell},
    future::Future,
    num::NonZeroU64,
    rc::{Rc, Weak},
};

///
/// Runtime
/// Live timer bindings and their installed transport authority.
///

pub(super) struct Runtime {
    pub(super) bindings: Bindings,
    pub(super) faulted: Cell<bool>,
    #[cfg(feature = "metrics")]
    pub(super) metrics: Rc<crate::MetricsStorage>,
    canister: Canister,
}

thread_local! { static RUNTIME: RefCell<Option<Rc<Runtime>>> = const { RefCell::new(None) }; }

/// Retained, non-cloneable control capability. Dropping it does not cancel work.
pub struct Once {
    runtime: Weak<Runtime>,
    handle: Handle,
}
/// Recovery authority for one abandoned attempt of one declaration.
pub struct RecoveryToken(RecoveryKey);
/// Bounded status for this declaration only.
pub enum OnceStatus {
    /// No future eligibility.
    Inactive,
    /// Armed at an absolute Unix nanosecond timestamp.
    Armed(u64),
    /// One local attempt is accepted; owner commands cannot overlap it.
    Running,
    /// Reconcile effects before explicitly recovering this attempt.
    RecoveryRequired(RecoveryToken),
    /// Provider or entry fault prevents further dispatch.
    Faulted,
}

/// Initialize once per live canister runtime. Does not reset existing declarations.
/// Snapshot restoration is unsupported; normal upgrades bind fresh volatile state.
/// The host must admit upgrades only after settling work, or use a separately
/// qualified reconciliation procedure. This function does not preserve accepted
/// attempts, recover uncertainty, or install an upgrade guard.
///
/// # Errors
/// Returns Uninitialized outside an installed canister, or Faulted if this
/// runtime has encountered a terminal provider fault.
pub fn initialize() -> Result<(), Error> {
    initialize_runtime(
        #[cfg(feature = "metrics")]
        None,
    )
}

/// Initialize with shared heap storage supplied by the canister host.
///
/// Repeating initialization with the same storage preserves all observations.
/// Supply fresh storage on a normal upgrade.
///
/// # Errors
/// Returns `Busy` if the live runtime already uses different storage.
/// Also returns the installation or provider error documented by [`initialize`].
#[cfg(feature = "metrics")]
pub fn initialize_with_metrics(storage: Rc<crate::MetricsStorage>) -> Result<(), Error> {
    initialize_runtime(Some(storage))
}

fn initialize_runtime(
    #[cfg(feature = "metrics")] storage: Option<Rc<crate::MetricsStorage>>,
) -> Result<(), Error> {
    RUNTIME.with_borrow_mut(|slot| {
        if let Some(runtime) = slot {
            runtime.check()?;

            #[cfg(feature = "metrics")]
            if storage.is_some_and(|storage| !Rc::ptr_eq(&runtime.metrics, &storage)) {
                return Err(Error::Busy);
            }

            return Ok(());
        }

        let canister = Canister::acquire()?;
        *slot = Some(Rc::new(Runtime {
            bindings: Bindings::new(),
            faulted: Cell::new(false),
            #[cfg(feature = "metrics")]
            metrics: storage.unwrap_or_default(),
            canister,
        }));

        Ok(())
    })
}

pub(super) fn runtime() -> Result<Rc<Runtime>, Error> {
    RUNTIME
        .with_borrow(Clone::clone)
        .ok_or(Error::Uninitialized)
}

/// Read aggregate counters without changing demand. Available even after a
/// provider fault. Host code owns encoding/export; no exporter is invoked here.
/// # Errors
/// Returns Uninitialized before provider initialization.
#[cfg(feature = "metrics")]
pub fn metrics() -> Result<crate::Metrics, Error> {
    Ok(runtime()?.metrics.snapshot())
}

/// Read a coherent, identity-ordered inventory with at most 64 declarations.
/// Observations never carry control/recovery authority. Incarnations are local
/// to this runtime; hosts identify upgrade/reinstall boundaries when exporting.
/// # Errors
/// Returns Uninitialized before initialization. Faulted providers remain observable.
#[cfg(feature = "metrics")]
pub fn inventory() -> Result<crate::Inventory, Error> {
    let runtime = runtime()?;

    Ok(crate::Inventory {
        provider_faulted: runtime.faulted.get(),
        timers: runtime.bindings.inventory(),
    })
}

/// Bind one inactive callback with nonempty owner/group/name components (64 bytes each).
///
/// # Errors
/// Returns initialization, identity, capacity or provider errors without replacing work.
pub fn bind_once<F, Fut>(
    identity: [&str; 3],
    lifetime: Lifetime,
    mut work: F,
) -> Result<Once, Error>
where
    F: FnMut() -> Fut + 'static,
    Fut: Future<Output = OnceResult> + 'static,
{
    let runtime = runtime()?;
    runtime.check()?;
    #[cfg(feature = "metrics")]
    let metrics = Rc::clone(&runtime.metrics);
    let handle = runtime.bindings.bind(identity, lifetime, move || {
        #[cfg(feature = "metrics")]
        metrics.record(Event::Accepted);
        let future = work();
        #[cfg(feature = "metrics")]
        let metrics = Rc::clone(&metrics);
        async move {
            let result = future.await;
            #[cfg(feature = "metrics")]
            metrics.record(Event::Returned(result.observation));
            let next = match result.next {
                OnceNext::Stop => Demand::Inactive,
                OnceNext::At(at) => Demand::At(at),
                OnceNext::After(delay) => system::time()
                    .checked_add(delay.get())
                    .map_or(Demand::Inactive, Demand::At),
            };
            (next, result.observation)
        }
    })?;

    Ok(Once {
        runtime: Rc::downgrade(&runtime),
        handle,
    })
}
impl Once {
    /// Remove unaccepted or settled work, releasing its declaration and captures.
    /// Old handles and queued deliveries cannot control a replacement. Dropping
    /// this handle alone never removes work.
    /// # Errors
    /// Refuses accepted work with Busy and uncertain work with RecoveryRequired;
    /// otherwise returns stale authority or terminal provider errors.
    pub fn remove(&self) -> Result<(), Error> {
        let runtime = self.runtime()?;
        runtime.check()?;
        self.handle.remove()?;

        runtime.arm()
    }

    fn runtime(&self) -> Result<Rc<Runtime>, Error> {
        self.runtime.upgrade().ok_or(Error::StaleAuthority)
    }

    /// Change eligibility; never invokes work inline. A provider failure is terminal.
    /// # Errors
    /// Returns stale authority, recovery-required or provider/counter fault.
    pub fn set(&self, demand: OnceDemand) -> Result<(), Error> {
        let runtime = self.runtime()?;
        runtime.check()?;
        self.handle.set(demand)?;

        runtime.arm()
    }

    /// Observe this declaration without affecting eligibility.
    /// # Errors
    /// Returns stale authority if the runtime is gone.
    pub fn status(&self) -> Result<OnceStatus, Error> {
        let runtime = self.runtime()?;
        if runtime.faulted.get() {
            return Ok(OnceStatus::Faulted);
        }

        Ok(match self.handle.state()? {
            State::Inactive => OnceStatus::Inactive,
            State::Armed(at) => OnceStatus::Armed(at),
            State::Running { .. } => OnceStatus::Running,
            State::Faulted => OnceStatus::Faulted,
            State::RecoveryRequired(_) => OnceStatus::RecoveryRequired(RecoveryToken(
                self.handle.recovery()?.ok_or(Error::StaleRecovery)?,
            )),
        })
    }

    /// Clear only the current recovery barrier after owner reconciliation.
    /// # Errors
    /// Returns stale recovery/authority or a terminal provider fault.
    pub fn recover(&self, token: RecoveryToken, demand: OnceDemand) -> Result<(), Error> {
        let runtime = self.runtime()?;
        runtime.check()?;
        let result = self.handle.recover(&token.0, demand);
        drop(token);
        result?;

        runtime.arm()
    }
}
/// Retained synchronous admission callback. Dropping the handle does not cancel it.
pub struct Watchdog {
    runtime: Weak<Runtime>,
    handle: crate::bindings::watchdog::WatchdogHandle,
}
/// Bounded watchdog eligibility and outstanding-ticket observation.
pub enum WatchdogStatus {
    /// No future cadence; an accepted/settling ticket may remain outstanding.
    Inactive {
        /// Whether transport still owns a work ticket.
        outstanding: bool,
    },
    /// Absolute next cadence deadline.
    Armed {
        /// Unix nanosecond deadline.
        at: u64,
        /// Whether transport still owns a work ticket.
        outstanding: bool,
    },
    /// Terminal entry or provider fault.
    Faulted,
}

/// Bind an inactive synchronous watchdog, with cadence in nanoseconds.
/// Consumer-owned asynchronous work must enforce its own physical-request admission.
/// # Errors
/// Returns initialization, identity, capacity or provider errors.
pub fn bind_watchdog<F>(
    identity: [&str; 3],
    cadence: NonZeroU64,
    lifetime: Lifetime,
    mut work: F,
) -> Result<Watchdog, Error>
where
    F: FnMut() -> WatchdogResult + 'static,
{
    let runtime = runtime()?;
    runtime.check()?;
    #[cfg(feature = "metrics")]
    let metrics = Rc::clone(&runtime.metrics);
    let handle = runtime
        .bindings
        .bind_watchdog(identity, cadence, lifetime, move || {
            #[cfg(feature = "metrics")]
            metrics.record(Event::Accepted);
            let result = work();
            #[cfg(feature = "metrics")]
            metrics.record(Event::Returned(result.observation));
            (result.next, result.observation)
        })?;

    Ok(Watchdog {
        runtime: Rc::downgrade(&runtime),
        handle,
    })
}
impl Watchdog {
    /// Remove future and unaccepted work. Consumer-owned async effects remain
    /// the consumer's responsibility; removal does not cancel them.
    /// # Errors
    /// Refuses accepted/settling callback custody with Busy; otherwise returns
    /// stale authority or terminal provider errors.
    pub fn remove(&self) -> Result<(), Error> {
        let runtime = self.runtime.upgrade().ok_or(Error::StaleAuthority)?;
        runtime.check()?;
        self.handle.remove()?;

        runtime.arm()
    }

    /// Update eligibility without invoking consumer work inline.
    /// # Errors
    /// Returns stale authority or terminal entry/provider fault.
    pub fn set(&self, demand: WatchdogDemand) -> Result<(), Error> {
        let runtime = self.runtime.upgrade().ok_or(Error::StaleAuthority)?;
        runtime.check()?;
        self.handle.set(demand, system::time())?;

        runtime.arm()
    }

    /// Observe this declaration without controlling consumer-owned async work.
    /// # Errors
    /// Returns stale authority if the runtime is gone.
    pub fn status(&self) -> Result<WatchdogStatus, Error> {
        let runtime = self.runtime.upgrade().ok_or(Error::StaleAuthority)?;
        let (deadline, outstanding, faulted) = self.handle.status()?;

        Ok(if runtime.faulted.get() || faulted {
            WatchdogStatus::Faulted
        } else if let Some(at) = deadline {
            WatchdogStatus::Armed { at, outstanding }
        } else {
            WatchdogStatus::Inactive { outstanding }
        })
    }
}

impl Runtime {
    pub(super) fn check(&self) -> Result<(), Error> {
        if self.faulted.get() {
            Err(Error::Faulted)
        } else {
            Ok(())
        }
    }

    pub(super) fn arm(&self) -> Result<(), Error> {
        match self.canister {
            #[cfg(target_arch = "wasm32")]
            Canister::Installed(ref transport) => transport.arm(self),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(super) fn transport(&self) -> &crate::transport::Transport {
        let Canister::Installed(transport) = &self.canister;

        transport
    }
}

// A runtime can only be constructed with an installed IC transport. The native
// type has no values, so no callback or timer operation can acquire authority.
enum Canister {
    #[cfg(target_arch = "wasm32")]
    Installed(crate::transport::Transport),
}

impl Canister {
    fn acquire() -> Result<Self, Error> {
        #[cfg(target_arch = "wasm32")]
        {
            Ok(Self::Installed(crate::transport::Transport::new()))
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            Err(Error::Uninitialized)
        }
    }
}
