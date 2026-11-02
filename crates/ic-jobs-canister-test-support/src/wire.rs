//! Native projection of the consumer-owned management contract.

use ic_jobs_test_consumer::{
    api::Init,
    management_wire::{Method, REQUEST_LIMITS, RESPONSE_LIMITS},
};
use ic_platform_pocket_ic::PocketIc;
use ic_platform_types::principal::ExternalPrincipal;
use ic_testkit::pic::{WireCallError, WireCallSpec, query_wire, update_wire};
use serde::{Serialize, de::DeserializeOwned};

// Transport and the management contract converge here. Each caller still owns
// application-error handling, test assertions and server/canister lifetimes.
/// One caller's management transport; application errors remain caller-owned.
pub struct Client<'a> {
    /// Running PocketIC instance owned by the caller.
    pub pic: &'a PocketIc,
    /// Installed qualification canister.
    pub canister: ExternalPrincipal,
    /// Identity used for the management call.
    pub caller: ExternalPrincipal,
}

impl Client<'_> {
    /// Query using the consumer's method identity and bounded Wire contract.
    pub fn query<T: DeserializeOwned>(
        &self,
        method: Method,
        args: &impl Serialize,
    ) -> Result<T, WireCallError> {
        query_wire(self.pic, self.canister, self.caller, control(method), args)
    }

    /// Update using the consumer's method identity and bounded Wire contract.
    pub fn update<T: DeserializeOwned>(
        &self,
        method: Method,
        args: &impl Serialize,
    ) -> Result<T, WireCallError> {
        update_wire(self.pic, self.canister, self.caller, control(method), args)
    }
}

/// Encode installation arguments using the consumer's bounded Wire contract.
pub fn init_bytes(config: &Init) -> Result<Vec<u8>, ic_wire::Error> {
    ic_wire::native::encode_request(Method::Init as u64, config, REQUEST_LIMITS)
}

/// Project one consumer-owned management method into the host transport spec.
pub const fn control(method: Method) -> WireCallSpec {
    WireCallSpec {
        method: method.name(),
        method_id: method as u64,
        request_limits: REQUEST_LIMITS,
        response_limits: RESPONSE_LIMITS,
    }
}
