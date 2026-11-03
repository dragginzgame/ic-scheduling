//! One real gate exchange; callers retain looping and failure policy.

use ic_timers_test_protocol::rpc::{Method, REQUEST_LIMITS, RESPONSE_LIMITS};

/// One real await per exchange; callers own looping and failure policy.
pub(super) async fn call_gate(method: Method) -> Result<bool, String> {
    let response = ic_platform_runtime::cdk::call::Call::unbounded_wait(
        ic_platform_runtime::cdk::api::canister_self(),
        method.name(),
    )
    .take_raw_args(
        ic_wire::native::encode_request(method as u64, &(), REQUEST_LIMITS).expect("gate request"),
    )
    .await
    .map_err(|error| error.to_string())?;
    ic_wire::native::decode_response(method as u64, &response.into_bytes(), RESPONSE_LIMITS)
        .map_err(|error| error.to_string())
}
