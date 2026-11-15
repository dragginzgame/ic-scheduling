//! Paired metrics-feature workload. Same scheduling and message exports in both builds.
use ic_platform_runtime::{
    Principal,
    call::{Call, WaitPolicy},
    ic0,
};
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
    let mut bytes = [0; 29];
    let len = ic0::canister_self_size();
    ic0::canister_self_copy(&mut bytes[..len], 0);
    Principal::from_slice(&bytes[..len])
}

/// Initialize and arm one async Once and one synchronous watchdog.
/// # Panics
/// Traps on provider/call failure or invalid instruction accounting.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update setup")]
pub extern "C" fn setup() {
    let before = ic0::performance_counter(0);
    ic_timers::initialize().unwrap();
    let once = ic_timers::bind_once(
        ["metrics-cost", "once", "work"],
        ic_timers::Lifetime::Retained,
        || {
            cost(1, ic0::performance_counter(0));
            async {
                count(0);
                assert!(
                    Call::new(self_id(), "echo", WaitPolicy::Unbounded)
                        .execute()
                        .await
                        .is_ok()
                );
                cost(2, ic0::performance_counter(0));
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
            cost(3, ic0::performance_counter(0));
            count(2);
            WatchdogResult {
                observation: Observation::NoWork,
                next: WatchdogNext::Stop,
            }
        },
    )
    .unwrap();
    watchdog.set(WatchdogDemand::At(0)).unwrap();
    cost(0, ic0::performance_counter(0).checked_sub(before).unwrap());
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
