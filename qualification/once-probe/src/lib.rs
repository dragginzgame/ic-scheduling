//! Actual Once provider installed-canister fixture. No alternate provider.

mod costs;
mod dispatch_costs;
mod jobs;
mod jobs_boundaries;
mod jobs_catch_up;
mod jobs_completion;
mod jobs_initial;
mod jobs_pressure;
mod jobs_retry;
mod jobs_unresolved;
mod lifecycle;
mod lifetimes;
mod metrics;
mod outcomes;
mod pressure;
mod refusal;
mod retention;
mod watchdog;

use ic_platform_runtime::{
    call::{Call, Completion, Outcome as CallOutcome, SubmitError, Wait},
    raw as ic0, system,
};
use ic_platform_types::Principal;
use ic_timers::{Observation, Once, OnceDemand, OnceNext, OnceResult, OnceStatus};
use std::cell::{Cell, RefCell};

thread_local! {
    static HANDLE: RefCell<Option<Once>> = const { RefCell::new(None) };
    static MODE: Cell<u8> = const { Cell::new(0) };
    static STARTS: Cell<u8> = const { Cell::new(0) };
    static FINISHES: Cell<u8> = const { Cell::new(0) };
    static FINISHED_AT: Cell<u64> = const { Cell::new(0) };
}

fn self_id() -> Principal {
    system::canister_self()
}

fn expect_reply(result: Result<Completion, SubmitError>) -> Vec<u8> {
    match result.expect("call submission").into_parts().0 {
        CallOutcome::Reply(bytes) => bytes,
        outcome => panic!("expected a reply, received {outcome:?}"),
    }
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
            let handle = ic_timers::bind_once(
                ["probe", "once", "work"],
                ic_timers::Lifetime::Retained,
                || {
                    if MODE.with(Cell::get) == 1 {
                        ic0::trap(b"factory trap")
                    }
                    async {
                        let _cleanup = CleanupTrap(MODE.with(Cell::get) == 5);
                        STARTS.with(|v| v.set(v.get() + 1));
                        let calls = if MODE.with(Cell::get) == 6 && STARTS.with(Cell::get) == 1 {
                            100
                        } else {
                            1
                        };
                        for _ in 0..calls {
                            let result = Call::new(self_id(), "echo", Wait::Unbounded)
                                .execute()
                                .await;
                            drop(crate::expect_reply(result));
                        }
                        if matches!(MODE.with(Cell::get), 2 | 5) {
                            ic0::trap(b"after await")
                        }
                        FINISHES.with(|v| v.set(v.get() + 1));
                        FINISHED_AT.with(|v| v.set(system::time()));
                        OnceResult {
                            observation: Observation::Success,
                            next: match MODE.with(Cell::get) {
                                3 => OnceNext::At(0),
                                6 if FINISHES.with(Cell::get) == 1 => OnceNext::After(
                                    std::num::NonZeroU64::new(10_000_000_000).unwrap(),
                                ),
                                7 => OnceNext::After(
                                    std::num::NonZeroU64::new(10_000_000_000).unwrap(),
                                ),
                                8 => OnceNext::After(std::num::NonZeroU64::new(u64::MAX).unwrap()),
                                _ => OnceNext::Stop,
                            },
                        }
                    }
                },
            )
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
    if matches!(mode, 3 | 4 | 7) {
        HANDLE.with(|slot| {
            let slot = slot.borrow();
            let handle = slot.as_ref().unwrap();
            assert!(matches!(handle.status().unwrap(), OnceStatus::Running));
            let demand = if mode == 3 || mode == 7 {
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

/// Completion time and current absolute deadline for the recurrence fixture.
/// # Panics
/// Traps if called before binding or if provider status fails.
#[expect(unsafe_code, reason = "qualification owns this entrypoint")]
#[unsafe(export_name = "canister_query completion_state")]
pub extern "C" fn completion_state() {
    let at = HANDLE.with_borrow(|slot| match slot.as_ref().unwrap().status().unwrap() {
        OnceStatus::Armed(at) => at,
        _ => 0,
    });
    ic0::msg_reply_data_append(&FINISHED_AT.with(Cell::get).to_le_bytes());
    ic0::msg_reply_data_append(&at.to_le_bytes());
    ic0::msg_reply();
}

struct CleanupTrap(bool);
impl Drop for CleanupTrap {
    fn drop(&mut self) {
        if self.0 {
            ic0::trap(b"deliberate consumer cleanup trap");
        }
    }
}
