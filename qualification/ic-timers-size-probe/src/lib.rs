//! Matched policy cohorts and instruction/memory calibration for installed Timers.
//! Exactly one cohort feature selects the private measurement canister.

use ic_timers::{TimerIdentity, initialize_runtime, timer_inventory, timer_snapshot};
use ic_timers_test_protocol::{
    CohortObservation, CounterObservation, MemorySamplingMeasurement, OperationMeasurement,
};
use std::cell::Cell;

#[cfg(not(feature = "baseline"))]
use ic_timers::{TimerSchedule, TimerStart};
#[cfg(not(feature = "baseline"))]
use std::cell::RefCell;

#[cfg(feature = "after-completion")]
use ic_timers::{
    AfterCompletionDecision, AfterCompletionRegistration, AfterCompletionRunResult, TimerCadence,
    TimerCompletion, register_after_completion,
};
#[cfg(feature = "once")]
use ic_timers::{OnceDecision, OnceRegistration, OnceRunResult, TimerCompletion, register_once};
#[cfg(feature = "watchdog")]
use ic_timers::{
    TimerCadence, TimerCompletion, WatchdogDecision, WatchdogRegistration, WatchdogRunResult,
    register_watchdog,
};

const _: () = assert!(
    cfg!(feature = "baseline") as u8
        + cfg!(feature = "once") as u8
        + cfg!(feature = "after-completion") as u8
        + cfg!(feature = "watchdog") as u8
        == 1,
    "select exactly one size-probe cohort feature",
);

#[cfg(not(feature = "baseline"))]
const CADENCE_NS: u64 = 1_000_000_000;

#[cfg(feature = "once")]
type CohortRegistration = OnceRegistration;
#[cfg(feature = "after-completion")]
type CohortRegistration = AfterCompletionRegistration;
#[cfg(feature = "watchdog")]
type CohortRegistration = WatchdogRegistration;

#[cfg(not(feature = "baseline"))]
thread_local! {
    static REGISTRATION: RefCell<Option<CohortRegistration>> = const { RefCell::new(None) };
}

thread_local! {
    static CALLBACKS: Cell<u64> = const { Cell::new(0) };
}

#[cfg(feature = "watchdog")]
thread_local! {
    static REPRESENTATIVE_WATCHDOG_WORK: Cell<bool> = const { Cell::new(false) };
}

#[ic_platform_runtime::cdk::init]
fn init() {
    if initialize_runtime().is_err() {
        ic_platform_runtime::cdk::api::trap("size-probe runtime initialization failed");
    }
}

ic_timers_test_protocol::probe_endpoints! {
    #[endpoint(update, CohortStart)]
    fn cohort_start() -> OperationMeasurement {
        let (changed, instructions) = measure(start_inner);
        OperationMeasurement {
            changed,
            instructions,
        }
    }

    #[endpoint(update, EnsureAgain)]
    fn ensure_again() -> OperationMeasurement {
        let (changed, instructions) = measure(ensure_again_inner);
        OperationMeasurement {
            changed,
            instructions,
        }
    }

    #[endpoint(update, Cancel)]
    fn cancel() -> OperationMeasurement {
        let (changed, instructions) = measure(cancel_inner);
        OperationMeasurement {
            changed,
            instructions,
        }
    }

    #[endpoint(update, MemorySamplingOverhead)]
    /// Isolate the raw instruction cost of the start/end page reads used by one
    /// normally completed callback. This probe-only endpoint does not alter the
    /// runtime's callback instruction aggregate.
    fn memory_sampling_overhead() -> MemorySamplingMeasurement {
        let ((), empty_bracket_instructions) = measure(|| {
            std::hint::black_box((0_u64, 0_u64));
            std::hint::black_box((0_u64, 0_u64));
        });
        let ((), sampled_bracket_instructions) = measure(|| {
            std::hint::black_box(memory_page_extents());
            std::hint::black_box(memory_page_extents());
        });
        MemorySamplingMeasurement {
            empty_bracket_instructions,
            sampled_bracket_instructions,
        }
    }
}

#[cfg(feature = "watchdog")]
ic_timers_test_protocol::probe_endpoints! {
    #[endpoint(update, UseRepresentativeWatchdogWork)]
    /// Select a bounded non-trivial callback body for the probe's next Watchdog
    /// registration. This endpoint exists only in the Watchdog test canister.
    fn use_representative_watchdog_work() {
        REPRESENTATIVE_WATCHDOG_WORK.with(|enabled| enabled.set(true));
    }

}

ic_timers_test_protocol::probe_endpoints! {
    #[endpoint(query, Observe)]
    fn observe() -> CohortObservation<&'static str> {
        let snapshot_before = ic_platform_runtime::ic0::performance_counter(1);
        let snapshot = timer_snapshot(&cohort_identity())
            .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("size-probe snapshot failed"));
        let snapshot_instructions =
            ic_platform_runtime::ic0::performance_counter(1).saturating_sub(snapshot_before);
        let inventory_before = ic_platform_runtime::ic0::performance_counter(1);
        let inventory = timer_inventory()
            .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("size-probe inventory failed"));
        let inventory_instructions =
            ic_platform_runtime::ic0::performance_counter(1).saturating_sub(inventory_before);
        let registered = registration_present();
        let callbacks = CALLBACKS.with(Cell::get);
        let observed = CohortObservation {
            cohort: cohort_name(),
            registered,
            callbacks,
            snapshot_instructions,
            inventory_len: inventory.len() as u64,
            inventory_instructions,
            ..CohortObservation::default()
        };
        let Some(snapshot) = snapshot else {
            return observed;
        };
        let counters = snapshot.observability().counters();
        let performance = snapshot.observability().performance();
        CohortObservation {
            next_deadline_ns: snapshot.next_deadline_ns(),
            schedule_requests: counter_observation(counters.schedule_requests()),
            wakeups_armed: counter_observation(counters.wakeups_armed()),
            work_dispatched: counter_observation(counters.work_dispatched()),
            scheduler_started: counter_observation(counters.scheduler_started()),
            work_started: counter_observation(counters.work_started()),
            work_completed: counter_observation(counters.work_completed()),
            coalesced: counter_observation(counters.coalesced()),
            scheduler_instruction_samples: performance
                .scheduler_instructions()
                .aggregate()
                .samples(),
            scheduler_instruction_total: performance.scheduler_instructions().aggregate().total(),
            work_instruction_samples: performance.work_instructions().aggregate().samples(),
            work_instruction_total: performance.work_instructions().aggregate().total(),
            ..observed
        }
    }
}

fn measure<T>(operation: impl FnOnce() -> T) -> (T, u64) {
    let before = ic_platform_runtime::ic0::performance_counter(1);
    let result = operation();
    let instructions = ic_platform_runtime::ic0::performance_counter(1).saturating_sub(before);
    (result, instructions)
}

#[inline(always)]
fn memory_page_extents() -> (u64, u64) {
    #[cfg(target_arch = "wasm32")]
    let wasm = std::arch::wasm32::memory_size::<0>() as u64;
    #[cfg(not(target_arch = "wasm32"))]
    let wasm = 0;

    (wasm, ic_platform_runtime::ic0::stable64_size())
}

#[cfg(feature = "baseline")]
fn start_inner() -> bool {
    false
}

#[cfg(feature = "once")]
fn start_inner() -> bool {
    REGISTRATION.with_borrow_mut(|slot| {
        if slot.is_some() {
            return false;
        }
        let registration = register_once(
            cohort_identity(),
            TimerStart::Retained {
                initial: Some(TimerSchedule::After(std::time::Duration::from_nanos(
                    CADENCE_NS,
                ))),
            },
            |_context| async {
                CALLBACKS.with(|calls| calls.set(calls.get().saturating_add(1)));
                OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
            },
        )
        .unwrap_or_else(|_| {
            ic_platform_runtime::cdk::api::trap("size-probe Once registration failed")
        });
        *slot = Some(registration);
        true
    })
}

#[cfg(feature = "after-completion")]
fn start_inner() -> bool {
    REGISTRATION.with_borrow_mut(|slot| {
        if slot.is_some() {
            return false;
        }
        let registration = register_after_completion(
            cohort_identity(),
            cadence(),
            TimerStart::Retained {
                initial: Some(TimerSchedule::After(std::time::Duration::from_nanos(
                    CADENCE_NS,
                ))),
            },
            |_context| async {
                CALLBACKS.with(|calls| calls.set(calls.get().saturating_add(1)));
                AfterCompletionRunResult::new(
                    TimerCompletion::no_work(),
                    AfterCompletionDecision::RecurAfterCompletion,
                )
            },
        )
        .unwrap_or_else(|_| {
            ic_platform_runtime::cdk::api::trap("size-probe after-completion registration failed")
        });
        *slot = Some(registration);
        true
    })
}

#[cfg(feature = "watchdog")]
fn start_inner() -> bool {
    REGISTRATION.with_borrow_mut(|slot| {
        if slot.is_some() {
            return false;
        }
        let registration = register_watchdog(
            cohort_identity(),
            cadence(),
            TimerStart::Retained {
                initial: Some(TimerSchedule::After(std::time::Duration::from_nanos(
                    CADENCE_NS,
                ))),
            },
            |_context| {
                CALLBACKS.with(|calls| calls.set(calls.get().saturating_add(1)));
                WatchdogRunResult::new(watchdog_completion(), WatchdogDecision::Continue)
            },
        )
        .unwrap_or_else(|_| {
            ic_platform_runtime::cdk::api::trap("size-probe Watchdog registration failed")
        });
        *slot = Some(registration);
        true
    })
}

#[cfg(feature = "watchdog")]
fn watchdog_completion() -> TimerCompletion {
    if !REPRESENTATIVE_WATCHDOG_WORK.with(Cell::get) {
        return TimerCompletion::no_work();
    }

    // Keep the calibration body dependency-free, deterministic, bounded, and
    // visibly non-trivial after optimization. It models synchronous work, not
    // IcyDB database logic or a production instruction budget.
    let mut state = std::hint::black_box(0x9e37_79b9_7f4a_7c15_u64);
    for step in 0..100_000_u64 {
        state = state.rotate_left(7) ^ step;
        state = state.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    }
    std::hint::black_box(state);
    TimerCompletion::success(1)
}

#[cfg(feature = "baseline")]
fn ensure_again_inner() -> bool {
    false
}

#[cfg(feature = "once")]
fn ensure_again_inner() -> bool {
    REGISTRATION.with_borrow(|slot| {
        let Some(registration) = slot.as_ref() else {
            return false;
        };
        if registration
            .ensure_scheduled(TimerSchedule::After(std::time::Duration::from_nanos(
                CADENCE_NS,
            )))
            .is_err()
        {
            ic_platform_runtime::cdk::api::trap("size-probe duplicate Once ensure failed");
        }
        false
    })
}

#[cfg(any(feature = "after-completion", feature = "watchdog"))]
fn ensure_again_inner() -> bool {
    REGISTRATION.with_borrow(|slot| {
        let Some(registration) = slot.as_ref() else {
            return false;
        };
        if registration.ensure_scheduled().is_err() {
            ic_platform_runtime::cdk::api::trap("size-probe duplicate recurring ensure failed");
        }
        false
    })
}

#[cfg(feature = "baseline")]
fn cancel_inner() -> bool {
    false
}

#[cfg(not(feature = "baseline"))]
fn cancel_inner() -> bool {
    REGISTRATION.with_borrow(|slot| {
        let Some(registration) = slot.as_ref() else {
            return false;
        };
        let was_scheduled = registration.has_armed_wakeup().unwrap_or_else(|_| {
            ic_platform_runtime::cdk::api::trap("size-probe wakeup observation failed")
        });
        if registration.cancel().is_err() {
            ic_platform_runtime::cdk::api::trap("size-probe cancellation failed");
        }
        was_scheduled
    })
}

#[cfg(feature = "baseline")]
const fn registration_present() -> bool {
    false
}

#[cfg(not(feature = "baseline"))]
fn registration_present() -> bool {
    REGISTRATION.with_borrow(Option::is_some)
}

#[cfg(any(feature = "after-completion", feature = "watchdog"))]
fn cadence() -> TimerCadence {
    TimerCadence::from_nanos(CADENCE_NS)
        .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("size-probe cadence is invalid"))
}

fn cohort_identity() -> TimerIdentity {
    TimerIdentity::try_new("ic-timers", "size-probe", cohort_name())
        .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("size-probe identity is invalid"))
}

#[cfg(feature = "baseline")]
const fn cohort_name() -> &'static str {
    "baseline"
}

#[cfg(feature = "once")]
const fn cohort_name() -> &'static str {
    "once"
}

#[cfg(feature = "after-completion")]
const fn cohort_name() -> &'static str {
    "after-completion"
}

#[cfg(feature = "watchdog")]
const fn cohort_name() -> &'static str {
    "watchdog"
}

fn counter_observation(counter: ic_timers::MeasurementCounter) -> CounterObservation {
    CounterObservation {
        total: counter.total(),
        total_overflowed: counter.total_overflowed(),
    }
}
