mod faults;
mod registration_fixture;
use faults::ProviderFault;
use registration_fixture::ReconciliationSlot;

use super::effects::{apply_effect, bind_provider_handle, clear_entry_provider_handles};
use super::*;
use crate::{
    AfterCompletionDecision, InactiveReason, MemoryPageSample, OnceDecision, TimerLastOutcome,
    TimerPolicy, TimerRegistrationStatus, TimerRuntimeStateSnapshot, TimerSchedulingMode,
    WatchdogDecision, WatchdogRuntimeStateSnapshot,
    platform::{
        advance_instructions, discard_next_due, grow_memory_pages, run_next_due, set_time,
        timer_count,
    },
};
use std::{
    cell::{Cell, RefCell},
    panic::{AssertUnwindSafe, catch_unwind},
    rc::Rc,
    task::{Poll, Waker},
    time::Duration,
};

fn reset_for_test(now_ns: u64, canister_version: u64) {
    platform::reset(now_ns, canister_version);
    platform::ADMISSION_FAULT.set(None);
    WATCHDOG_COMPLETION_FAULT.with(|fault| fault.set(false));
    PROVIDER_INSTALL_FAULT_AFTER.with(|fault| fault.set(None));
    RUNTIME.with(|runtime| {
        *runtime.borrow_mut() = None;
    });
}

thread_local! {
    static WATCHDOG_COMPLETION_FAULT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static PROVIDER_INSTALL_FAULT_AFTER: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
}

fn inject_watchdog_completion_fault() {
    WATCHDOG_COMPLETION_FAULT.with(|fault| fault.set(true));
}

pub(super) fn take_watchdog_completion_fault() -> bool {
    WATCHDOG_COMPLETION_FAULT.with(|fault| fault.replace(false))
}

fn inject_provider_install_fault_after(successful_installs: u64) {
    PROVIDER_INSTALL_FAULT_AFTER.with(|fault| fault.set(Some(successful_installs)));
}

pub(super) fn take_provider_install_fault() -> bool {
    PROVIDER_INSTALL_FAULT_AFTER.with(|fault| match fault.get() {
        Some(0) => {
            fault.set(None);
            true
        }
        Some(remaining) => {
            fault.set(Some(remaining - 1));
            false
        }
        None => false,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TestPolicy {
    Once,
    AfterCompletion,
    Watchdog,
}

impl TestPolicy {
    const ALL: [Self; 3] = [Self::Once, Self::AfterCompletion, Self::Watchdog];
}

// Share only immediate stopping callbacks. Suspension and reentrant callback
// subjects keep their own bodies and authority checks.
fn stopping_once(
    mut work: impl FnMut() + 'static,
) -> impl FnMut(OnceContext) -> std::future::Ready<OnceRunResult> {
    move |_| {
        work();
        std::future::ready(OnceRunResult::new(
            TimerCompletion::no_work(),
            OnceDecision::Stop,
        ))
    }
}

// Successful inactive declarations only; registration refusals and capture-drop
// boundary tests keep their explicit public calls and callbacks.
#[track_caller]
fn retained_once(timer: TimerIdentity, work: impl FnMut() + 'static) -> OnceRegistration {
    register_once(
        timer,
        TimerStart::Retained { initial: None },
        stopping_once(work),
    )
    .expect("fixture registration should succeed")
}

fn stopping_after_completion(
    mut work: impl FnMut() + 'static,
) -> impl FnMut(AfterCompletionContext) -> std::future::Ready<AfterCompletionRunResult> {
    move |_| {
        work();
        std::future::ready(AfterCompletionRunResult::new(
            TimerCompletion::no_work(),
            AfterCompletionDecision::Stop,
        ))
    }
}

fn stopping_watchdog(
    mut work: impl FnMut() + 'static,
) -> impl FnMut(WatchdogContext) -> WatchdogRunResult {
    move |_| {
        work();
        WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
    }
}

#[derive(Clone, Default)]
struct SuspendedWork {
    ready: Rc<Cell<bool>>,
    waker: Rc<RefCell<Option<Waker>>>,
    polls: Rc<Cell<u64>>,
}

impl SuspendedWork {
    #[expect(
        clippy::future_not_send,
        reason = "The native executor models single-threaded IC callbacks with canister-local state."
    )]
    async fn wait(&self) {
        std::future::poll_fn(|context| {
            self.polls.set(self.polls.get() + 1);
            if self.ready.get() {
                Poll::Ready(())
            } else {
                *self.waker.borrow_mut() = Some(context.waker().clone());
                Poll::Pending
            }
        })
        .await;
    }

    fn resume(&self) {
        self.ready.set(true);
        self.waker
            .borrow_mut()
            .take()
            .expect("work suspended")
            .wake();
    }
}

fn scheduled_start(lifetime: DeclarationLifetime, initial: TimerSchedule) -> TimerStart {
    match lifetime {
        DeclarationLifetime::Retained => TimerStart::Retained {
            initial: Some(initial),
        },
        DeclarationLifetime::RemoveWhenStopped => TimerStart::RemoveWhenStopped { initial },
    }
}

fn suspended_ordinary_registration(
    after_completion: bool,
    timer: TimerIdentity,
    lifetime: DeclarationLifetime,
    gate: SuspendedWork,
    context: Rc<RefCell<Option<CallbackToken>>>,
    captured: Rc<()>,
) -> RegistrationClaim {
    let start = match lifetime {
        DeclarationLifetime::Retained => TimerStart::Retained { initial: None },
        DeclarationLifetime::RemoveWhenStopped => TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(10),
        },
    };
    if after_completion {
        register_after_completion(
            timer,
            TimerCadence::from_nanos(5).unwrap(),
            start,
            move |work| {
                *context.borrow_mut() = Some(work.token);
                let gate = gate.clone();
                let captured = Rc::clone(&captured);
                async move {
                    gate.wait().await;
                    std::hint::black_box(captured);
                    AfterCompletionRunResult::new(
                        TimerCompletion::success(1),
                        AfterCompletionDecision::Stop,
                    )
                }
            },
        )
        .unwrap()
        .claim
    } else {
        register_once(timer, start, move |work| {
            *context.borrow_mut() = Some(work.token);
            let gate = gate.clone();
            let captured = Rc::clone(&captured);
            async move {
                gate.wait().await;
                std::hint::black_box(captured);
                OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
            }
        })
        .unwrap()
        .claim
    }
}

fn identity(name: &str) -> TimerIdentity {
    TimerIdentity::try_new("test", "runtime", name).expect("fixture identity should be valid")
}

// Successful setup/observation preconditions belong here. Tests of missing
// registrations, unavailable runtime or invalid cadence keep the raw boundary.
#[track_caller]
fn registered_snapshot(timer: &TimerIdentity) -> TimerSnapshot {
    timer_snapshot(timer)
        .expect("fixture snapshot lookup should succeed")
        .expect("fixture registration should be present")
}

#[track_caller]
fn test_cadence(nanoseconds: u64) -> TimerCadence {
    TimerCadence::from_nanos(nanoseconds).expect("fixture cadence should be valid")
}

#[must_use = "retain the fixture until the test scope ends"]
struct RuntimeFixture {
    epoch: TimerEpoch,
}

impl Drop for RuntimeFixture {
    fn drop(&mut self) {
        // The production delivery guard must run while runtime TLS is available.
        // Native thread teardown is not an IC lifecycle or rollback event.
        platform::clear_tasks();
        // Retained declarations also own callback captures, even when inactive.
        // Detach them while TLS is live, exposing an empty initialized registry
        // to their destructors without holding the runtime borrow.
        let retired = RUNTIME.with(|runtime| {
            let mut runtime = runtime.borrow_mut();
            let empty = runtime
                .as_ref()
                .map(|registry| TimerRegistry::new(registry.epoch()));
            std::mem::replace(&mut *runtime, empty)
        });
        drop(retired);
    }
}

fn setup() -> RuntimeFixture {
    reset_for_test(10, 7);
    RuntimeFixture {
        epoch: initialize_runtime().expect("runtime initialization should succeed"),
    }
}

mod sequences;

mod capacity;
mod capture_cleanup;
mod delivery;
mod ordinary_scheduling;
mod provider_failures;
mod reconciliation;
mod recovery;
mod registration;
mod watchdog;
