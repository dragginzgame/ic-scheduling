//! Provider cancellation churn observations, separate from live handle bounds.

use ic_timers::{
    TimerCadence, TimerCompletion, TimerIdentity, TimerSchedule, TimerStart, WatchdogDecision,
    WatchdogRegistration, WatchdogRunResult, register_watchdog, timer_inventory,
};
use ic_timers_test_protocol::ChurnObservation;
use std::cell::{Cell, RefCell};

const CADENCE_NS: u64 = 3_600_000_000_000;
const MAX_REPLACEMENTS: u32 = 32_768;
const IMMEDIATE_STEPS: u64 = 64;

thread_local! {
    static REGISTRATION: RefCell<Option<WatchdogRegistration>> = const { RefCell::new(None) };
    static COMPLETED: Cell<u64> = const { Cell::new(0) };
}

#[ic_wire_endpoint::update(
    id = ic_timers_test_protocol::rpc::Method::ReplaceDistantDeadlines as u64,
    request = ic_timers_test_protocol::rpc::REQUEST_LIMITS,
    response = ic_timers_test_protocol::rpc::RESPONSE_LIMITS
)]
fn replace_distant_deadlines((count,): (u32,)) {
    assert!(
        count > 0 && count <= MAX_REPLACEMENTS,
        "bounded churn fixture"
    );
    REGISTRATION.with_borrow_mut(|slot| {
        if slot.is_none() {
            *slot = Some(
                register_watchdog(
                    TimerIdentity::try_new("ic-timers", "churn-probe", "watchdog")
                        .expect("fixed identity"),
                    TimerCadence::from_nanos(CADENCE_NS).expect("fixed cadence"),
                    TimerStart::Retained { initial: None },
                    |_| {
                        let completed = COMPLETED.with(|count| {
                            let next = count.get() + 1;
                            count.set(next);
                            next
                        });
                        WatchdogRunResult::new(
                            TimerCompletion::success(1),
                            if completed < IMMEDIATE_STEPS {
                                WatchdogDecision::ContinueImmediately
                            } else {
                                WatchdogDecision::Stop
                            },
                        )
                    },
                )
                .expect("churn registration"),
            );
        }
        let timer = slot.as_ref().expect("registered");
        let base = ic_platform_runtime::cdk::api::time()
            .checked_add(CADENCE_NS)
            .expect("distant deadline");
        for index in 0..count {
            timer
                .reconcile_schedule(Some(TimerSchedule::At(
                    base.checked_add(u64::from(index)).expect("deadline"),
                )))
                .expect("replace deadline");
        }
        timer.cancel().expect("clear final owned handle");
    });
}

#[ic_wire_endpoint::update(
    id = ic_timers_test_protocol::rpc::Method::StartImmediateChurn as u64,
    request = ic_timers_test_protocol::rpc::REQUEST_LIMITS,
    response = ic_timers_test_protocol::rpc::RESPONSE_LIMITS
)]
fn start_immediate_churn() {
    REGISTRATION.with_borrow(|slot| {
        slot.as_ref()
            .expect("registered")
            .ensure_scheduled_immediately()
            .expect("immediate churn")
    });
}

#[ic_wire_endpoint::query(
    id = ic_timers_test_protocol::rpc::Method::ChurnObservation as u64,
    request = ic_timers_test_protocol::rpc::REQUEST_LIMITS,
    response = ic_timers_test_protocol::rpc::RESPONSE_LIMITS
)]
fn churn_observation() -> ChurnObservation {
    #[cfg(target_arch = "wasm32")]
    let wasm_pages = std::arch::wasm32::memory_size::<0>() as u64;
    #[cfg(not(target_arch = "wasm32"))]
    let wasm_pages = 0;
    ChurnObservation {
        wasm_pages,
        inventory_len: timer_inventory().expect("inventory").len() as u64,
        armed: REGISTRATION.with_borrow(|slot| {
            slot.as_ref()
                .is_some_and(|timer| timer.has_armed_wakeup().expect("wakeup observation"))
        }),
        completed: COMPLETED.with(Cell::get),
    }
}
