//! Installed executor qualification fixture, not a timer provider.
//!
//! Raw byte endpoints and deliberate traps are confined to qualification.

use ic_platform_runtime::{
    Principal,
    call::{Call, WaitPolicy},
    executor, ic0,
};
use std::cell::Cell;

thread_local! {
    // control committed, work started, work completed, ordinary drop, recovery drop
    static EVENTS: Cell<[u8; 5]> = const { Cell::new([0; 5]) };
}

fn record(index: usize) {
    EVENTS.with(|events| {
        let mut value = events.get();
        value[index] = 1;
        events.set(value);
    });
}
fn self_id() -> Principal {
    let mut bytes = [0; 29];
    let len = ic0::canister_self_size();
    ic0::canister_self_copy(&mut bytes[..len], 0);
    Principal::from_slice(&bytes[..len])
}
fn authenticate() {
    let mut bytes = [0; 29];
    let len = ic0::msg_caller_size();
    if len > bytes.len() {
        ic0::trap(b"invalid caller");
    }
    ic0::msg_caller_copy(&mut bytes[..len], 0);
    if Principal::from_slice(&bytes[..len]) != self_id() {
        ic0::trap(b"self only");
    }
}
fn scenario() -> u8 {
    if ic0::msg_arg_data_size() != 1 {
        ic0::trap(b"expected one byte");
    }
    let mut bytes = [0];
    ic0::msg_arg_data_copy(&mut bytes, 0);
    if bytes[0] > 2 {
        ic0::trap(b"unknown scenario");
    }
    bytes[0]
}
struct WorkDrop;
impl Drop for WorkDrop {
    fn drop(&mut self) {
        record(if executor::is_recovering_from_trap() {
            4
        } else {
            3
        });
    }
}

/// Launch isolated work. Reply byte: 1 for success, 2 for a rejected work call.
#[expect(unsafe_code, reason = "qualification owns this canister entrypoint")]
#[unsafe(export_name = "canister_update launch")]
pub extern "C" fn launch() {
    let mode = scenario();
    EVENTS.with(|events| events.set([0; 5]));
    record(0);
    executor::in_tracking_executor_context(|| {
        let _task = executor::spawn_protected(async move {
            let response = Call::new(self_id(), "work", WaitPolicy::Unbounded)
                .take_raw_args(vec![mode])
                .execute()
                .await;
            ic0::msg_reply_data_append(&[if response.is_ok() { 1 } else { 2 }]);
            ic0::msg_reply();
        });
    });
}

/// Self-only work fixture: normal completion, initial trap, or post-await trap.
#[expect(unsafe_code, reason = "qualification owns this canister entrypoint")]
#[unsafe(export_name = "canister_update work")]
pub extern "C" fn work() {
    authenticate();
    let mode = scenario();
    executor::in_tracking_executor_context(|| {
        let _task = executor::spawn_protected(async move {
            let guard = WorkDrop;
            record(1);
            if mode == 1 {
                ic0::trap(b"initial work trap");
            }
            let response = Call::new(self_id(), "echo", WaitPolicy::Unbounded)
                .execute()
                .await;
            if response.is_err() {
                ic0::trap(b"unexpected echo failure");
            }
            if mode == 2 {
                ic0::trap(b"post-await work trap");
            }
            record(2);
            drop(guard);
            ic0::msg_reply();
        });
    });
}

/// Force a genuine asynchronous message boundary.
#[expect(unsafe_code, reason = "qualification owns this canister entrypoint")]
#[unsafe(export_name = "canister_update echo")]
pub extern "C" fn echo() {
    authenticate();
    ic0::msg_reply();
}

/// Return the committed event flags for the driver.
#[expect(unsafe_code, reason = "qualification owns this canister entrypoint")]
#[unsafe(export_name = "canister_query events")]
pub extern "C" fn events() {
    EVENTS.with(|events| ic0::msg_reply_data_append(&events.get()));
    ic0::msg_reply();
}
