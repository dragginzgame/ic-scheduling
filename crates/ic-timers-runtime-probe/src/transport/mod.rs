//! Real self-call queue pressure without changing the timer backend.

use std::cell::Cell;

thread_local! {
    static SUBMISSION_FAILURES: Cell<u64> = const { Cell::new(0) };
    static REPLIES: Cell<u64> = const { Cell::new(0) };
    static ENABLED: Cell<bool> = const { Cell::new(true) };
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::PressureAndStart as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::PressureAndStart as u64 }>"
)]
fn pressure_and_start() {
    start_pressure();
    assert!(super::start_immediately());
}

pub(super) fn start_pressure() {
    for _ in 0..600 {
        send_pressure();
    }
}

fn send_pressure() {
    ic_platform_runtime::cdk::futures::spawn(async {
        match ic_platform_runtime::cdk::call::Call::unbounded_wait(
            ic_platform_runtime::cdk::api::canister_self(),
            "pressure_reply",
        )
        .take_raw_args(
            ic_wire::native::encode_request(
                ic_timers_test_protocol::rpc::Method::PressureReply as u64,
                &(),
                ic_timers_test_protocol::rpc::REQUEST_LIMITS,
            )
            .expect("gate request"),
        )
        .await
        {
            Ok(_) => {
                REPLIES.set(REPLIES.get() + 1);
                if ENABLED.get() {
                    send_pressure();
                }
            }
            Err(ic_platform_runtime::cdk::call::CallFailed::CallPerformFailed(_)) => {
                SUBMISSION_FAILURES.set(SUBMISSION_FAILURES.get() + 1);
            }
            Err(error) => {
                ic_platform_runtime::cdk::api::trap(format!("unexpected pressure error: {error}"))
            }
        }
    });
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ReleasePressure as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::ReleasePressure as u64 }>"
)]
fn release_pressure() {
    ENABLED.set(false);
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::PressureReply as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::PressureReply as u64 }>"
)]
fn pressure_reply() {}

#[ic_platform_runtime::cdk::query(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::PressureObservation as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::PressureObservation as u64 }>"
)]
fn pressure_observation() -> (u64, u64) {
    (SUBMISSION_FAILURES.get(), REPLIES.get())
}
