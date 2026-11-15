//! Real cycle pressure, confined to an installed qualification canister.
use super::{Cell, Observation, Once, OnceDemand, OnceNext, OnceResult, OnceStatus, RefCell, ic0};
use ic_platform_runtime::cdk::api;
use ic_timers::{Watchdog, WatchdogDemand, WatchdogNext, WatchdogResult, WatchdogStatus};
use std::num::NonZeroU64;

thread_local! {
    static HANDLE: RefCell<Option<Once>> = const { RefCell::new(None) };
    static WATCHDOG: RefCell<Option<Watchdog>> = const { RefCell::new(None) };
    static STARTS: Cell<u8> = const { Cell::new(0) };
}

/// Arm work, then reduce liquid cycles below the self-call's quoted cost.
/// # Panics
/// Traps if fixture admission or cycle reduction fails.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update refusal_setup")]
pub extern "C" fn setup() {
    ic_timers::initialize().unwrap();
    if ic0::msg_arg_data_size() == 0 {
        let handle = ic_timers::bind_once(["refusal", "once", "work"], || async {
            STARTS.with(|starts| starts.set(starts.get() + 1));
            OnceResult {
                observation: Observation::Success,
                next: OnceNext::Stop,
            }
        })
        .unwrap();
        handle.set(OnceDemand::At(0)).unwrap();
        HANDLE.with(|slot| *slot.borrow_mut() = Some(handle));
    } else {
        assert_eq!(ic0::msg_arg_data_size(), 1);
        let mut args = [0];
        ic0::msg_arg_data_copy(&mut args, 0);
        assert_eq!(args, [1]);
        let handle = ic_timers::bind_watchdog(
            ["refusal", "watchdog", "work"],
            NonZeroU64::new(10).unwrap(),
            || {
                STARTS.with(|starts| starts.set(starts.get() + 1));
                WatchdogResult {
                    observation: Observation::Success,
                    next: WatchdogNext::Stop,
                }
            },
        )
        .unwrap();
        handle.set(WatchdogDemand::At(0)).unwrap();
        WATCHDOG.with(|slot| *slot.borrow_mut() = Some(handle));
    }
    let timer = ic0::global_timer_set(0);
    ic0::global_timer_set(timer);
    let cost = ic0::cost_call("__ic_scheduling_once".len() as u64, 16);
    let balance = ic0::canister_liquid_cycle_balance128();
    let burn = balance.checked_sub(cost / 2).unwrap();
    assert_eq!(api::cycles_burn(burn), burn);
    ic0::msg_reply_data_append(&timer.to_le_bytes());
    ic0::msg_reply_data_append(&cost.to_le_bytes());
    ic0::msg_reply();
}

/// Observe the timer without changing its value, plus status and callback starts.
/// # Panics
/// Traps if called before setup.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update refusal_state")]
pub extern "C" fn state() {
    // The setter is unavailable in queries. Restore the exact value so the
    // provider's ownership observation remains valid.
    let timer = ic0::global_timer_set(0);
    ic0::global_timer_set(timer);
    let status = HANDLE.with(
        |slot| match slot.borrow().as_ref().unwrap().status().unwrap() {
            OnceStatus::Armed(_) => 1,
            OnceStatus::Inactive => 0,
            _ => 2,
        },
    );
    ic0::msg_reply_data_append(&timer.to_le_bytes());
    ic0::msg_reply_data_append(&[status, STARTS.with(Cell::get)]);
    ic0::msg_reply();
}

/// Observe watchdog cadence and acceptance independently of its delivery backoff.
/// # Panics
/// Traps unless the watchdog refusal scenario was initialized or if faulted.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query refusal_watch_state")]
pub extern "C" fn watch_state() {
    let (deadline, outstanding) =
        WATCHDOG.with(
            |slot| match slot.borrow().as_ref().unwrap().status().unwrap() {
                WatchdogStatus::Armed { at, outstanding } => (at, outstanding),
                WatchdogStatus::Inactive { outstanding } => (0, outstanding),
                WatchdogStatus::Faulted => ic0::trap(b"unexpected provider fault"),
            },
        );
    ic0::msg_reply_data_append(&deadline.to_le_bytes());
    ic0::msg_reply_data_append(&[u8::from(outstanding), STARTS.with(Cell::get)]);
    ic0::msg_reply();
}
