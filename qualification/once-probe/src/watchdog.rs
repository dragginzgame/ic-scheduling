//! Actual synchronous watchdog execution and committed successor observations.
use super::{Cell, Observation, RefCell, ic0};
use ic_timers::{Watchdog, WatchdogDemand, WatchdogNext, WatchdogResult, WatchdogStatus};
use std::num::NonZeroU64;
thread_local! {
    static HANDLE: RefCell<Option<Watchdog>> = const { RefCell::new(None) };
    static CALLS: Cell<u8> = const { Cell::new(0) };
    static MODE: Cell<u8> = const { Cell::new(0) };
    static TARGET: Cell<Option<super::Principal>> = const { Cell::new(None) };
    static BUSY: Cell<bool> = const { Cell::new(false) };
    static STARTS: Cell<u8> = const { Cell::new(0) };
    static FINISHES: Cell<u8> = const { Cell::new(0) };
    static SHORT_CADENCE: Cell<bool> = const { Cell::new(false) };
    static BLOCKER_FINISHES: Cell<u8> = const { Cell::new(0) };
}
fn mode() -> u8 {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut bytes = [0];
    ic0::msg_arg_data_copy(&mut bytes, 0);
    bytes[0]
}
/// Bind and arm without inline callback execution.
/// # Panics
/// Traps on malformed setup or a failed provider invariant.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update watchdog_setup")]
pub extern "C" fn setup() {
    MODE.with(|value| value.set(mode()));
    let short = MODE.with(Cell::get) == 6;
    SHORT_CADENCE.with(|value| value.set(short));
    ic_timers::initialize().unwrap();
    let handle = ic_timers::bind_watchdog(
        ["watchdog", "probe", "work"],
        NonZeroU64::new(if short { 10 } else { 1_000_000_000 }).unwrap(),
        ic_timers::Lifetime::Retained,
        || {
            HANDLE.with(|slot| {
                let slot = slot.borrow();
                let handle = slot.as_ref().unwrap();
                let WatchdogStatus::Armed {
                    at,
                    outstanding: true,
                } = handle.status().unwrap()
                else {
                    ic0::trap(b"missing prearmed successor");
                };
                // Arbitrarily delayed delivery need not see a future deadline.
                if !SHORT_CADENCE.with(Cell::get) {
                    assert!(at > ic0::time());
                }
                match MODE.with(Cell::get) {
                    1 => ic0::trap(b"watchdog work trap"),
                    2 => {
                        handle.set(WatchdogDemand::Inactive).unwrap();
                        handle.set(WatchdogDemand::Inactive).unwrap();
                    }
                    3 => {
                        handle
                            .set(WatchdogDemand::At(ic0::time() + 10_000_000_000))
                            .unwrap();
                    }
                    _ => {}
                }
            });
            if MODE.with(Cell::get) == 5 && !BUSY.with(|busy| busy.replace(true)) {
                STARTS.with(|starts| starts.set(starts.get() + 1));
                let target = TARGET.with(Cell::get).unwrap();
                let _task = ic_platform_runtime::executor::spawn_protected(async move {
                    let result = super::Call::new(target, "hold", super::WaitPolicy::Unbounded)
                        .execute()
                        .await;
                    assert!(result.is_ok());
                    FINISHES.with(|finishes| finishes.set(finishes.get() + 1));
                    BUSY.with(|busy| busy.set(false));
                });
            }
            CALLS.with(|calls| calls.set(calls.get() + 1));
            WatchdogResult {
                observation: Observation::Success,
                next: match MODE.with(Cell::get) {
                    2 => WatchdogNext::At(0),
                    3 | 4 => WatchdogNext::Stop,
                    _ => WatchdogNext::Continue,
                },
            }
        },
    )
    .unwrap();
    handle.set(WatchdogDemand::At(0)).unwrap();
    HANDLE.with(|slot| *slot.borrow_mut() = Some(handle));
    ic0::msg_reply_data_append(&[CALLS.with(Cell::get)]);
    ic0::msg_reply();
}
/// Change only the consumer behavior; do not alter timer eligibility.
/// # Panics
/// Traps on malformed input.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update watchdog_mode")]
pub extern "C" fn set_mode() {
    MODE.with(|value| value.set(mode()));
    ic0::msg_reply();
}
/// Return deadline, committed callback count, and outstanding-ticket flag.
/// # Panics
/// Traps if setup is absent or the provider is faulted.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query watchdog_state")]
pub extern "C" fn state() {
    let (deadline, outstanding) =
        HANDLE.with(
            |slot| match slot.borrow().as_ref().unwrap().status().unwrap() {
                WatchdogStatus::Armed { at, outstanding } => (at, outstanding),
                WatchdogStatus::Inactive { outstanding } => (0, outstanding),
                WatchdogStatus::Faulted => ic0::trap(b"provider fault"),
            },
        );
    ic0::msg_reply_data_append(&deadline.to_le_bytes());
    ic0::msg_reply_data_append(&[CALLS.with(Cell::get), u8::from(outstanding)]);
    ic0::msg_reply();
}

/// Set the consumer-owned async request target before watchdog setup.
/// # Panics
/// Traps on malformed principal bytes.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update watchdog_target")]
pub extern "C" fn target() {
    let len = ic0::msg_arg_data_size();
    assert!(len > 0 && len <= 29);
    let mut bytes = [0; 29];
    ic0::msg_arg_data_copy(&mut bytes[..len], 0);
    TARGET.with(|target| target.set(Some(super::Principal::from_slice(&bytes[..len]))));
    ic0::msg_reply();
}
/// Request another admission opportunity without advancing held-call time.
/// # Panics
/// Traps if setup is absent or control fails.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update watchdog_poke")]
pub extern "C" fn poke() {
    HANDLE.with(|slot| {
        slot.borrow()
            .as_ref()
            .unwrap()
            .set(WatchdogDemand::At(0))
            .unwrap()
    });
    ic0::msg_reply();
}
/// Observe physical request admission independently of timer callback counts.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query watchdog_requests")]
pub extern "C" fn requests() {
    ic0::msg_reply_data_append(&[
        STARTS.with(Cell::get),
        FINISHES.with(Cell::get),
        u8::from(BUSY.with(Cell::get)),
    ]);
    ic0::msg_reply();
}

/// Occupy all eight provider flights before admitting the watchdog.
/// # Panics
/// Traps if the target is absent or binding/control fails.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update watchdog_block")]
pub extern "C" fn block() {
    let target = TARGET.with(Cell::get).unwrap();
    ic_timers::initialize().unwrap();
    for index in 0..8 {
        let handle = ic_timers::bind_once(
            ["watchdog", "blocker", &index.to_string()],
            ic_timers::Lifetime::Retained,
            move || async move {
                let result = super::Call::new(target, "hold", super::WaitPolicy::Unbounded)
                    .execute()
                    .await;
                assert!(result.is_ok());
                BLOCKER_FINISHES.with(|count| count.set(count.get() + 1));
                super::OnceResult {
                    observation: Observation::Success,
                    next: super::OnceNext::Stop,
                }
            },
        )
        .unwrap();
        handle.set(super::OnceDemand::At(0)).unwrap();
    }
    ic0::msg_reply();
}
/// Observe settlement of the eight capacity-blocking calls.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query watchdog_blocker_finishes")]
pub extern "C" fn blocker_finishes() {
    ic0::msg_reply_data_append(&[BLOCKER_FINISHES.with(Cell::get)]);
    ic0::msg_reply();
}
