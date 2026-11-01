//! Platform entrypoint hooks: Wire owns codecs, Platform owns IC contexts.

use ic_timers_test_protocol::rpc::{REQUEST_LIMITS, RESPONSE_LIMITS};
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

#[cfg(feature = "watchdog")]
pub(crate) fn encode_unit<const ID: u64>() -> Vec<u8> {
    encode::<_, ID>(())
}
