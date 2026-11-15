//! Actual Once provider installed-canister fixture. No alternate provider.

use ic_platform_runtime::{
    Principal,
    call::{Call, WaitPolicy},
    ic0,
};
use ic_timers::{Observation, Once, OnceDemand, OnceNext, OnceResult, OnceStatus};
use std::cell::{Cell, RefCell};
thread_local! {
    static HANDLE: RefCell<Option<Once>> = const { RefCell::new(None) };
    static MODE: Cell<u8> = const { Cell::new(0) };
    static STARTS: Cell<u8> = const { Cell::new(0) };
    static FINISHES: Cell<u8> = const { Cell::new(0) };
}
fn self_id() -> Principal {
    let mut bytes = [0; 29];
    let len = ic0::canister_self_size();
    ic0::canister_self_copy(&mut bytes[..len], 0);
    Principal::from_slice(&bytes[..len])
}
/// Bind once and arm without inline execution. Modes 1/2 trap before/after await.
///
/// # Panics
/// Traps if the fixture is called out of order or a provider assertion fails.
#[expect(unsafe_code, reason = "qualification owns this entrypoint")]
#[unsafe(export_name = "canister_update arm")]
pub extern "C" fn arm() {
    if ic0::msg_arg_data_size() != 1 {
        ic0::trap(b"one byte required")
    }
    let mut mode = [0];
    ic0::msg_arg_data_copy(&mut mode, 0);
    MODE.with(|v| v.set(mode[0]));
    ic_timers::initialize().expect("initialize");
    HANDLE.with(|slot| {
        if slot.borrow().is_none() {
            let handle = ic_timers::bind_once(["probe", "once", "work"], || {
                if MODE.with(Cell::get) == 1 {
                    ic0::trap(b"factory trap")
                }
                async {
                    let _cleanup = CleanupTrap(MODE.with(Cell::get) == 5);
                    STARTS.with(|v| v.set(v.get() + 1));
                    let result = Call::new(self_id(), "echo", WaitPolicy::Unbounded)
                        .execute()
                        .await;
                    assert!(result.is_ok());
                    if matches!(MODE.with(Cell::get), 2 | 5) {
                        ic0::trap(b"after await")
                    }
                    FINISHES.with(|v| v.set(v.get() + 1));
                    OnceResult {
                        observation: Observation::Success,
                        next: if MODE.with(Cell::get) == 3 {
                            OnceNext::At(0)
                        } else {
                            OnceNext::Stop
                        },
                    }
                }
            })
            .expect("bind");
            *slot.borrow_mut() = Some(handle);
        }
        let outcome = slot.borrow().as_ref().unwrap().set(OnceDemand::At(0));
        ic0::msg_reply_data_append(&[u8::from(outcome.is_ok()), STARTS.with(Cell::get)]);
    });
    ic0::msg_reply();
}
/// Reconcile this synthetic fixture by explicitly stopping the abandoned attempt.
///
/// # Panics
/// Traps if the fixture is called out of order or a provider assertion fails.
#[expect(unsafe_code, reason = "qualification owns this entrypoint")]
#[unsafe(export_name = "canister_update recover")]
pub extern "C" fn recover() {
    HANDLE.with(|slot| {
        let slot = slot.borrow();
        let handle = slot.as_ref().unwrap();
        let OnceStatus::RecoveryRequired(token) = handle.status().unwrap() else {
            ic0::trap(b"not blocked")
        };
        handle.recover(token, OnceDemand::Inactive).unwrap();
    });
    ic0::msg_reply();
}
/// Genuine await target.
///
/// # Panics
/// Traps if the fixture is called out of order or a provider assertion fails.
#[expect(unsafe_code, reason = "qualification owns this entrypoint")]
#[unsafe(export_name = "canister_update echo")]
pub extern "C" fn echo() {
    let mode = MODE.with(Cell::get);
    if mode == 3 || mode == 4 {
        HANDLE.with(|slot| {
            let slot = slot.borrow();
            let handle = slot.as_ref().unwrap();
            assert!(matches!(handle.status().unwrap(), OnceStatus::Running));
            let demand = if mode == 3 {
                OnceDemand::Inactive
            } else {
                OnceDemand::At(0)
            };
            handle.set(demand).unwrap();
            handle.set(demand).unwrap();
        });
        if mode == 4 {
            MODE.with(|value| value.set(0));
        }
    }
    ic0::msg_reply();
}
/// State: 0 inactive, 1 armed, 2 running, 3 recovery, 4 fault; then start/finish counts.
///
/// # Panics
/// Traps if the fixture is called out of order or a provider assertion fails.
#[expect(unsafe_code, reason = "qualification owns this entrypoint")]
#[unsafe(export_name = "canister_query state")]
pub extern "C" fn state() {
    let status = HANDLE.with(
        |slot| match slot.borrow().as_ref().unwrap().status().unwrap() {
            OnceStatus::Inactive => 0,
            OnceStatus::Armed(_) => 1,
            OnceStatus::Running => 2,
            OnceStatus::RecoveryRequired(_) => 3,
            OnceStatus::Faulted => 4,
        },
    );
    ic0::msg_reply_data_append(&[status, STARTS.with(Cell::get), FINISHES.with(Cell::get)]);
    ic0::msg_reply();
}

mod pressure;

mod metrics;

struct CleanupTrap(bool);
impl Drop for CleanupTrap {
    fn drop(&mut self) {
        if self.0 && ic_platform_runtime::executor::is_recovering_from_trap() {
            ic0::trap(b"deliberate consumer cleanup trap");
        }
    }
}

mod lifecycle;

mod refusal;

mod watchdog;

mod costs;

mod dispatch_costs;

mod jobs;

mod jobs_pressure;

mod jobs_boundaries;

mod jobs_catch_up;
