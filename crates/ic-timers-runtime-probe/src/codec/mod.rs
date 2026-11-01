//! Platform entrypoint hooks: Wire owns codecs, Platform owns IC contexts.

use ic_timers_test_protocol::rpc::{Method, REQUEST_LIMITS, RESPONSE_LIMITS};
use serde::{Serialize, de::DeserializeOwned};

pub(crate) fn admit_request() -> Result<(), String> {
    if ic_platform_runtime::ic0::msg_arg_data_size() > REQUEST_LIMITS.bytes {
        return Err("probe request exceeds Wire byte limit".to_owned());
    }
    Ok(())
}

pub(crate) fn decode<T: DeserializeOwned, const ID: u64>(bytes: Vec<u8>) -> T {
    ic_wire::native::decode_request(ID, &bytes, REQUEST_LIMITS).unwrap_or_else(|error| {
        ic_platform_runtime::cdk::api::trap(format!("probe request: {error}"))
    })
}

pub(crate) fn encode<T: Serialize, const ID: u64>(value: T) -> Vec<u8> {
    ic_wire::native::encode_response(ID, &value, RESPONSE_LIMITS).unwrap_or_else(|error| {
        ic_platform_runtime::cdk::api::trap(format!("probe response: {error}"))
    })
}

pub(crate) fn encode_unit<const ID: u64>() -> Vec<u8> {
    encode::<_, ID>(())
}

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
