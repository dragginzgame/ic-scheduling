//! Current delivery RPC identities and resource limits. Candid is not accepted.
//!
//! These native Serde contracts use the declaration order of `Delivery`, its
//! payload and `Error`. Incompatible layouts require new identities and a joint
//! sender/receiver cut. Queue storage and management method identities remain independent.

use ic_wire::Limits;
#[cfg(any(test, target_arch = "wasm32"))]
use serde::de::DeserializeOwned;

#[cfg(any(test, target_arch = "wasm32"))]
use crate::api::Error;

/// `receive_delivery`: Delivery -> Result<Delivery, Error>.
pub const RECEIVE_ID: u64 = u64::from_le_bytes(*b"JOBSRCV1");
/// `delivery_receipt`: Delivery -> Result<Option<Delivery>, Error>.
pub const RECEIPT_ID: u64 = u64::from_le_bytes(*b"JOBSRCP1");

/// Closed outbound selection keeps each exported name with its Wire identity.
#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Clone, Copy)]
#[repr(u64)]
pub(crate) enum Method {
    Receive = RECEIVE_ID,
    Receipt = RECEIPT_ID,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl Method {
    pub(crate) const fn id(self) -> u64 {
        self as u64
    }

    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Receive => "receive_delivery",
            Self::Receipt => "delivery_receipt",
        }
    }
}

/// Complete request frame and native Serde traversal allowances.
pub const REQUEST_LIMITS: Limits = Limits {
    bytes: 1024,
    allocation: 16 * 1024,
    nodes: 128,
    depth: 16,
};
/// Complete response frame and native Serde traversal allowances.
pub const RESPONSE_LIMITS: Limits = Limits {
    bytes: 8 * 1024,
    allocation: 64 * 1024,
    nodes: 1024,
    depth: 16,
};

// Decode only. Callers own exact evidence comparison and uncertain-effect policy.
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn decode_reply<T, E>(method: u64, response: Result<impl AsRef<[u8]>, E>) -> Option<T>
where
    T: DeserializeOwned,
{
    ic_wire::native::decode_response::<Result<T, Error>>(
        method,
        response.ok()?.as_ref(),
        RESPONSE_LIMITS,
    )
    .ok()?
    .ok()
}

#[cfg(test)]
mod tests;
