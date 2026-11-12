//! Qualification framing with the selected Wire codec and raw Testkit transport.

use ic_platform_types::Principal;
use ic_testkit::pocket_ic::{CanisterId, RejectResponse, nonblocking::PocketIc};
use ic_wire::{Limits, native};
use serde::{Serialize, de::DeserializeOwned};

/// Consumer-owned method identity and directional budgets.
#[derive(Clone, Copy, Debug)]
pub struct WireCallSpec {
    /// Exported method name.
    pub method: &'static str,
    /// Wire frame identity.
    pub method_id: u64,
    /// Request admission limits.
    pub request_limits: Limits,
    /// Response admission limits.
    pub response_limits: Limits,
}

/// Exact transport identity retained with every failure.
#[derive(Clone, Debug)]
pub struct WireCallContext {
    /// Raw PocketIC operation.
    pub operation: &'static str,
    /// Destination canister.
    pub canister_id: Principal,
    /// Caller identity.
    pub caller: Principal,
    /// Exported method name.
    pub method: &'static str,
    /// Wire frame identity.
    pub method_id: u64,
}

/// Distinguishes admission failures from effects attempted by the transport.
#[derive(Debug)]
pub enum WireCallError {
    /// Request refused before transport.
    Encode {
        /// Selected transport identity.
        context: Box<WireCallContext>,
        /// Canonical Wire error.
        source: ic_wire::Error,
    },
    /// Original canister rejection; transport panics propagate without retry.
    Call {
        /// Selected transport identity.
        context: Box<WireCallContext>,
        /// Original upstream rejection.
        source: Box<RejectResponse>,
    },
    /// Reply refused by the consumer's codec contract.
    Decode {
        /// Selected transport identity.
        context: Box<WireCallContext>,
        /// Canonical Wire error.
        source: ic_wire::Error,
    },
}

impl std::fmt::Display for WireCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for WireCallError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Encode { source, .. } | Self::Decode { source, .. } => Some(source),
            Self::Call { .. } => None,
        }
    }
}

/// Query with canonical framing and no transport fallback or retry.
///
/// # Errors
/// Retains encoding, rejection and decoding failures with their exact context.
/// Upstream transport panics propagate without retrying the operation.
pub async fn query_wire<Request: Serialize, Response: DeserializeOwned>(
    pic: &PocketIc,
    canister_id: Principal,
    caller: Principal,
    spec: WireCallSpec,
    request: &Request,
) -> Result<Response, WireCallError> {
    call_wire(pic, canister_id, caller, spec, request, true).await
}

/// Update with canonical framing; uncertain effects remain caller-owned.
///
/// # Errors
/// Retains encoding, rejection and decoding failures with their exact context.
/// Upstream transport panics propagate without retrying the operation.
pub async fn update_wire<Request: Serialize, Response: DeserializeOwned>(
    pic: &PocketIc,
    canister_id: Principal,
    caller: Principal,
    spec: WireCallSpec,
    request: &Request,
) -> Result<Response, WireCallError> {
    call_wire(pic, canister_id, caller, spec, request, false).await
}

async fn call_wire<Request: Serialize, Response: DeserializeOwned>(
    pic: &PocketIc,
    canister_id: Principal,
    caller: Principal,
    spec: WireCallSpec,
    request: &Request,
    query: bool,
) -> Result<Response, WireCallError> {
    let context = Box::new(WireCallContext {
        operation: if query { "query_call" } else { "update_call" },
        canister_id,
        caller,
        method: spec.method,
        method_id: spec.method_id,
    });
    let frame =
        native::encode_request(spec.method_id, request, spec.request_limits).map_err(|source| {
            WireCallError::Encode {
                context: context.clone(),
                source,
            }
        })?;
    let destination = CanisterId::from_slice(canister_id.as_slice());
    let sender = CanisterId::from_slice(caller.as_slice());
    let reply = if query {
        pic.query_call(destination, sender, spec.method, frame)
            .await
    } else {
        pic.update_call(destination, sender, spec.method, frame)
            .await
    }
    .map_err(|source| WireCallError::Call {
        context: context.clone(),
        source: Box::new(source),
    })?;
    native::decode_response(spec.method_id, &reply, spec.response_limits)
        .map_err(|source| WireCallError::Decode { context, source })
}
