//! Actual synchronous watchdog execution and committed successor observations.
use super::{Cell, Observation, RefCell, ic0};
use ic_timers::{Watchdog, WatchdogDemand, WatchdogNext, WatchdogResult, WatchdogStatus};
use std::num::NonZeroU64;
thread_local! {
    static HANDLE: RefCell<Option<Watchdog>> = const { RefCell::new(None) };
    static CALLS: Cell<u8> = const { Cell::new(0) };
    static MODE: Cell<u8> = const { Cell::new(0) };
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
    ic_timers::initialize().unwrap();
    let handle = ic_timers::bind_watchdog(
        ["watchdog", "probe", "work"],
        NonZeroU64::new(1_000_000_000).unwrap(),
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
                assert!(at > ic0::time());
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
