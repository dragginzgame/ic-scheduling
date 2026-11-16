//! Installed executor qualification fixture, not a timer provider.
//!
//! Raw byte endpoints and deliberate traps are confined to qualification.

use ic_platform_runtime::{
    call::{Call, Outcome, Wait},
    executor, raw as ic0, system,
};
use ic_platform_types::Principal;
use std::cell::{Cell, RefCell};

thread_local! {
    // control committed, work started, work completed, destruction, committed cleanup
    static EVENTS: Cell<[u8; 5]> = const { Cell::new([0; 5]) };
    static WORK: RefCell<Option<executor::TaskHandle>> = const { RefCell::new(None) };
}

fn record(index: usize) {
    EVENTS.with(|events| {
        let mut value = events.get();
        value[index] = 1;
        events.set(value);
    });
}
fn self_id() -> Principal {
    system::canister_self()
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
        record(3);
    }
}

/// Launch isolated work. Reply byte: 1 for success, 2 for a rejected work call.
/// # Panics
/// Traps on invalid fixture input, task refusal or unexpected reply authority.
#[expect(unsafe_code, reason = "qualification owns this canister entrypoint")]
#[unsafe(export_name = "canister_update launch")]
pub extern "C" fn launch() {
    let mode = scenario();
    EVENTS.with(|events| events.set([0; 5]));
    WORK.with(|work| *work.borrow_mut() = None);
    record(0);
    executor::run_update(async move {
        let response = Call::new(self_id(), "work", Wait::Unbounded)
            .owned_args(vec![mode])
            .execute()
            .await;
        let completion = response.expect("work submission");
        let reply = matches!(completion.outcome(), Outcome::Reply(_));
        system::reply(&[if reply { 1 } else { 2 }]).expect("launch reply");
    })
    .expect("launch task");
}

/// Self-only work fixture: normal completion, initial trap, or post-await trap.
/// # Panics
/// Deliberately traps in failure modes; also traps on invalid task admission.
#[expect(unsafe_code, reason = "qualification owns this canister entrypoint")]
#[unsafe(export_name = "canister_update work")]
pub extern "C" fn work() {
    authenticate();
    let mode = scenario();
    let task = executor::run_update(async move {
        let guard = WorkDrop;
        record(1);
        if mode == 1 {
            ic0::trap(b"initial work trap");
        }
        let response = Call::new(self_id(), "echo", Wait::Unbounded)
            .execute()
            .await;
        if !matches!(
            response.expect("echo submission").outcome(),
            Outcome::Reply(_)
        ) {
            ic0::trap(b"unexpected echo failure");
        }
        if mode == 2 {
            ic0::trap(b"post-await work trap");
        }
        record(2);
        drop(guard);
        system::reply(&[]).expect("work reply");
    })
    .expect("work task");
    WORK.with(|work| *work.borrow_mut() = Some(task));
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
    EVENTS.with(|events| {
        let mut values = events.get();
        values[4] = u8::from(WORK.with_borrow(|work| {
            work.as_ref()
                .is_some_and(|task| task.state() == executor::TaskState::TerminatedByCleanup)
        }));
        ic0::msg_reply_data_append(&values);
    });
    ic0::msg_reply();
}
