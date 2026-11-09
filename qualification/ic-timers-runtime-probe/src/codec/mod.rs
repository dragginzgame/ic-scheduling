//! One real gate exchange; callers retain looping and failure policy.

use ic_timers_test_protocol::rpc::{Method, REQUEST_LIMITS, RESPONSE_LIMITS};
use std::cell::Cell;

thread_local! {
    static PREPARED_CALLS: Cell<bool> = const { Cell::new(false) };
}

ic_timers_test_protocol::probe_endpoints! {
    #[endpoint(update, UsePreparedCalls)]
    fn use_prepared_calls((prepared,): (bool,)) {
        PREPARED_CALLS.set(prepared);
    }
}

/// One real await per exchange; callers own looping and failure policy.
pub(super) async fn call_gate(method: Method) -> Result<bool, String> {
    let target = ic_platform_runtime::cdk::api::canister_self();
    let bytes =
        ic_wire::native::encode_request(method as u64, &(), REQUEST_LIMITS).expect("gate request");
    let response = if PREPARED_CALLS.get() {
        ic_platform_runtime::call::Call::new(
            target,
            method.name(),
            ic_platform_runtime::call::WaitPolicy::Unbounded,
        )
        .take_raw_args(bytes)
        .execute()
        .await
    } else {
        ic_platform_runtime::cdk::call::Call::unbounded_wait(target, method.name())
            .take_raw_args(bytes)
            .await
    }
    .map_err(|error| error.to_string())?;
    ic_wire::native::decode_response(method as u64, &response.into_bytes(), RESPONSE_LIMITS)
        .map_err(|error| error.to_string())
}
