//! Paired metrics-feature workload. Same scheduling and message exports in both builds.
use ic_platform_runtime::{
    call::{Call, Outcome, Wait},
    raw as ic0, system,
};
use ic_platform_types::Principal;
use ic_timers::{
    Observation, OnceDemand, OnceNext, OnceResult, WatchdogDemand, WatchdogNext, WatchdogResult,
};
use std::{cell::Cell, num::NonZeroU64};
thread_local! {
    static COUNTS: Cell<[u8; 3]> = const { Cell::new([0; 3]) };
    static COSTS: Cell<[u64; 4]> = const { Cell::new([0; 4]) };
}
fn cost(index: usize, value: u64) {
    COSTS.with(|costs| {
        let mut values = costs.get();
        values[index] = value;
        costs.set(values);
    });
}
fn count(index: usize) {
    COUNTS.with(|counts| {
        let mut values = counts.get();
        values[index] += 1;
        counts.set(values);
    });
}
fn self_id() -> Principal {
    system::canister_self()
}

/// Initialize and arm one async Once and one synchronous watchdog.
/// # Panics
/// Traps on provider/call failure or invalid instruction accounting.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update setup")]
pub extern "C" fn setup() {
    let before = ic0::instruction_counter();
    ic_timers::initialize().unwrap();
    let once = ic_timers::bind_once(
        ["metrics-cost", "once", "work"],
        ic_timers::Lifetime::Retained,
        || {
            cost(1, ic0::instruction_counter());
            async {
                count(0);
                assert!(matches!(
                    Call::new(self_id(), "echo", Wait::Unbounded)
                        .execute()
                        .await
                        .expect("echo submission")
                        .outcome(),
                    Outcome::Reply(_)
                ));
                cost(2, ic0::instruction_counter());
                count(1);
                OnceResult {
                    observation: Observation::ExpectedFailure,
                    next: OnceNext::Stop,
                }
            }
        },
    )
    .unwrap();
    once.set(OnceDemand::At(0)).unwrap();
    let watchdog = ic_timers::bind_watchdog(
        ["metrics-cost", "watchdog", "work"],
        NonZeroU64::new(1_000_000_000).unwrap(),
        ic_timers::Lifetime::Retained,
        || {
            cost(3, ic0::instruction_counter());
            count(2);
            WatchdogResult {
                observation: Observation::NoWork,
                next: WatchdogNext::Stop,
            }
        },
    )
    .unwrap();
    watchdog.set(WatchdogDemand::At(0)).unwrap();
    cost(0, ic0::instruction_counter().checked_sub(before).unwrap());
    ic0::msg_reply();
}

/// Actual await target.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update echo")]
pub extern "C" fn echo() {
    ic0::msg_reply();
}

/// Counts, four instruction brackets, six metric totals and six overflow flags.
/// # Panics
/// Traps if the enabled provider cannot supply diagnostics.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query state")]
pub extern "C" fn state() {
    COUNTS.with(|counts| ic0::msg_reply_data_append(&counts.get()));
    COSTS.with(|costs| {
        for value in costs.get() {
            ic0::msg_reply_data_append(&value.to_le_bytes());
        }
    });
    #[cfg(feature = "metrics")]
    {
        let metrics = ic_timers::metrics().unwrap();
        let counters = [
            metrics.accepted,
            metrics.completed,
            metrics.abandoned,
            metrics.expected_failures,
            metrics.deferred_submissions,
            metrics.provider_faults,
        ];
        for counter in counters {
            ic0::msg_reply_data_append(&counter.total().to_le_bytes());
        }
        for counter in counters {
            ic0::msg_reply_data_append(&[u8::from(counter.total_overflowed())]);
        }
    }
    #[cfg(not(feature = "metrics"))]
    ic0::msg_reply_data_append(&[0; 102]);
    ic0::msg_reply();
}
