//! Single-message instruction observations; never span an await or callback lifetime.
use super::{Observation, OnceDemand, OnceNext, OnceResult, ic0};
use ic_timers::{WatchdogDemand, WatchdogNext, WatchdogResult};
use std::num::NonZeroU64;

fn measured<T>(work: impl FnOnce() -> T) -> (u64, T) {
    let before = ic0::performance_counter(0);
    let result = work();
    (
        ic0::performance_counter(0).checked_sub(before).unwrap(),
        result,
    )
}

/// Measure owner operations with three or 64 retained declarations in one message.
/// The raw result is fourteen little-endian u64 values, in driver-declared order.
/// # Panics
/// Traps on malformed setup, admission failure, or incorrect final state.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update control_costs")]
pub extern "C" fn control_costs() {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut count = [0];
    ic0::msg_arg_data_copy(&mut count, 0);
    assert!(count[0] == 3 || count[0] == 64);
    let mut costs = Vec::with_capacity(14);
    costs.push(measured(|| ()).0);
    costs.push(measured(|| ic_timers::initialize().unwrap()).0);
    let (cost, once) = measured(|| {
        ic_timers::bind_once(
            ["cost", "once", "primary"],
            ic_timers::Lifetime::Retained,
            || async {
                OnceResult {
                    observation: Observation::NoWork,
                    next: OnceNext::Stop,
                }
            },
        )
        .unwrap()
    });
    costs.push(cost);
    let (cost, watchdog) = measured(|| {
        ic_timers::bind_watchdog(
            ["cost", "watchdog", "primary"],
            NonZeroU64::new(1_000_000_000).unwrap(),
            ic_timers::Lifetime::Retained,
            || WatchdogResult {
                observation: Observation::NoWork,
                next: WatchdogNext::Stop,
            },
        )
        .unwrap()
    });
    costs.push(cost);
    let mut last_bind = 0;
    for index in 2..count[0] {
        let name = index.to_string();
        let (cost, _) = measured(|| {
            ic_timers::bind_once(
                ["cost", "filler", &name],
                ic_timers::Lifetime::Retained,
                || async {
                    OnceResult {
                        observation: Observation::NoWork,
                        next: OnceNext::Stop,
                    }
                },
            )
            .unwrap()
        });
        last_bind = cost;
    }
    costs.push(last_bind);
    let future = ic0::time().checked_add(60_000_000_000).unwrap();
    costs.push(measured(|| once.set(OnceDemand::At(future)).unwrap()).0);
    costs.push(measured(|| once.set(OnceDemand::At(future + 1)).unwrap()).0);
    costs.push(measured(|| once.set(OnceDemand::At(future + 1)).unwrap()).0);
    costs.push(measured(|| once.set(OnceDemand::Inactive).unwrap()).0);
    costs.push(measured(|| watchdog.set(WatchdogDemand::KeepCadence).unwrap()).0);
    costs.push(measured(|| watchdog.set(WatchdogDemand::KeepCadence).unwrap()).0);
    costs.push(measured(|| watchdog.set(WatchdogDemand::At(future)).unwrap()).0);
    costs.push(measured(|| watchdog.set(WatchdogDemand::Inactive).unwrap()).0);
    // A coarse module-memory observation, including the fixture and allocator.
    costs.push(std::arch::wasm32::memory_size::<0>() as u64);
    assert!(matches!(
        once.status().unwrap(),
        ic_timers::OnceStatus::Inactive
    ));
    assert!(matches!(
        watchdog.status().unwrap(),
        ic_timers::WatchdogStatus::Inactive { outstanding: false }
    ));
    if count[0] == 64 {
        assert!(matches!(
            ic_timers::bind_once(
                ["cost", "extra", "overflow"],
                ic_timers::Lifetime::Retained,
                || async {
                    OnceResult {
                        observation: Observation::NoWork,
                        next: OnceNext::Stop,
                    }
                }
            ),
            Err(ic_timers::Error::Capacity)
        ));
    }
    for value in costs {
        ic0::msg_reply_data_append(&value.to_le_bytes());
    }
    ic0::msg_reply();
}
