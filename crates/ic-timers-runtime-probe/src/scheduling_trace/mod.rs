//! Observable deadline order, callback-created timers and self-cancellation.

use ic_timers::{
    AfterCompletionDecision, AfterCompletionRunResult, OnceDecision, OnceRunResult, TimerCadence,
    TimerCompletion, TimerIdentity, TimerSchedule, TimerStart, register_after_completion,
    register_once,
};
use std::{
    cell::{Cell, RefCell},
    time::Duration,
};

// The registry retains callbacks; this trace controls them only through their
// callback contexts, so it deliberately discards registration capabilities.
thread_local! {
    static EVENTS: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static STARTED: Cell<bool> = const { Cell::new(false) };
}

fn identity(name: &str) -> TimerIdentity {
    TimerIdentity::try_new("ic-timers", "scheduling-trace", name).expect("fixed trace identity")
}

fn install_once(name: &str, seconds: u64, event: u8) {
    let _ = register_once(
        identity(name),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_secs(seconds)),
        },
        move |_| async move {
            EVENTS.with_borrow_mut(|events| events.push(event));
            OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
        },
    )
    .expect("trace registration");
}

#[ic_wire_endpoint::update(
    id = ic_timers_test_protocol::rpc::Method::StartSchedulingTrace as u64,
    request = ic_timers_test_protocol::rpc::REQUEST_LIMITS,
    response = ic_timers_test_protocol::rpc::RESPONSE_LIMITS
)]
fn start_scheduling_trace() {
    assert!(!STARTED.replace(true), "fresh trace");
    let _ = register_once(
        identity("parent"),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_secs(1)),
        },
        |_| async {
            EVENTS.with_borrow_mut(|events| events.push(1));
            install_once("child", 2, 3);
            OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
        },
    )
    .expect("parent registration");
    install_once("middle", 2, 2);
    install_once("late", 4, 4);
    let _ = register_after_completion(
        identity("self-cancelling-periodic"),
        TimerCadence::new(Duration::from_secs(1)).expect("trace cadence"),
        TimerStart::Retained {
            initial: Some(TimerSchedule::After(Duration::from_secs(7))),
        },
        |context| async move {
            EVENTS.with_borrow_mut(|events| events.push(5));
            context.cancel().expect("periodic self-cancellation");
            AfterCompletionRunResult::new(
                TimerCompletion::success(1),
                AfterCompletionDecision::RecurAfterCompletion,
            )
        },
    )
    .expect("periodic trace registration");
    let _ = register_once(
        identity("self-cancelling-once"),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_secs(8)),
        },
        |context| async move {
            EVENTS.with_borrow_mut(|events| events.push(6));
            context.cancel().expect("one-shot self-cancellation");
            OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
        },
    )
    .expect("self-cancelling one-shot registration");
}

#[ic_wire_endpoint::query(
    id = ic_timers_test_protocol::rpc::Method::SchedulingTrace as u64,
    request = ic_timers_test_protocol::rpc::REQUEST_LIMITS,
    response = ic_timers_test_protocol::rpc::RESPONSE_LIMITS
)]
fn scheduling_trace() -> Vec<u8> {
    EVENTS.with_borrow(Clone::clone)
}
