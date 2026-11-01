//! Fixture-only Wire checkpoint; both sides of an upgrade use this one format.

use ic_timers_test_protocol::rpc::{CHECKPOINT_ID, REQUEST_LIMITS};

use ic_timers_test_protocol::RuntimeCheckpoint;

pub(super) fn save(state: &RuntimeCheckpoint) {
    let frame = ic_wire::native::encode_request(CHECKPOINT_ID, state, REQUEST_LIMITS)
        .unwrap_or_else(|_| {
            ic_platform::cdk::api::trap("runtime probe checkpoint encoding failed")
        });
    let len = u32::try_from(frame.len()).expect("bounded checkpoint length");
    // The admitted frame and its length prefix always fit in one 64 KiB page.
    if ic_platform::cdk::api::stable_size() == 0 {
        ic_platform::cdk::stable::stable_grow(1).unwrap_or_else(|_| {
            ic_platform::cdk::api::trap("runtime probe checkpoint allocation failed")
        });
    }
    ic_platform::cdk::stable::stable_write(0, &len.to_le_bytes());
    ic_platform::cdk::stable::stable_write(4, &frame);
}

pub(super) fn load() -> RuntimeCheckpoint {
    if ic_platform::cdk::api::stable_size() == 0 {
        ic_platform::cdk::api::trap("runtime probe checkpoint missing");
    }
    let mut prefix = [0; 4];
    ic_platform::cdk::stable::stable_read(0, &mut prefix);
    let len = u32::from_le_bytes(prefix) as usize;
    if len > REQUEST_LIMITS.bytes {
        ic_platform::cdk::api::trap("runtime probe checkpoint exceeds limit");
    }
    let mut frame = vec![0; len];
    ic_platform::cdk::stable::stable_read(4, &mut frame);
    ic_wire::native::decode_request(CHECKPOINT_ID, &frame, REQUEST_LIMITS)
        .unwrap_or_else(|_| ic_platform::cdk::api::trap("runtime probe checkpoint decoding failed"))
}
