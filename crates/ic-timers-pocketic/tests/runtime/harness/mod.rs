//! Fresh, caller-owned testkit servers and IC instances from the verified binary.

use ic_platform::pocket_ic::{PocketIc, PocketIcBuilder};
use ic_platform::principal::ExternalPrincipal;
use ic_testkit::pic::{
    CanisterInstallExt, InstallSpec, PocketIcInstance, PocketIcStartupConfig, WireCallError,
    WireCallSpec, tick_until,
};
use ic_timers_test_protocol::rpc::{Method, REQUEST_LIMITS, RESPONSE_LIMITS};
use serde::{Serialize, de::DeserializeOwned};
use std::{convert::Infallible, env, fmt::Debug, path::PathBuf, task::Poll, time::Duration};

const STARTUP_TIMEOUT: Duration = Duration::from_secs(60);
pub const INIT_CYCLES: u128 = 2_000_000_000_000;

/// Preserve tick-before-observation boundaries while Testkit owns bounded progress.
pub fn wait_for_probe<T: Debug>(
    pic: &PocketIc,
    rounds: u32,
    mut observe: impl FnMut(&PocketIc) -> T,
    ready: impl Fn(&T) -> bool,
) -> T {
    let remaining = rounds.checked_sub(1).expect("positive probe round budget");
    let mut last = None;
    pic.tick();
    tick_until(pic, remaining, Duration::ZERO, |pic| {
        let observation = observe(pic);
        if ready(&observation) {
            Poll::Ready(Ok::<_, Infallible>(observation))
        } else {
            last = Some(observation);
            Poll::Pending
        }
    })
    .unwrap_or_else(|error| panic!("{error}; last probe observation: {last:?}"))
}

/// Install one probe with the fixtures' explicit funding and empty init payload.
pub fn install_probe(pic: &PocketIc, wasm: Vec<u8>) -> ExternalPrincipal {
    pic.try_create_and_install(InstallSpec {
        wasm,
        init_bytes: Vec::new(),
        cycles: INIT_CYCLES,
        install_sender: None,
        label: Some("ic-timers probe".to_owned()),
    })
    .expect("install funded timer probe")
}

/// Testkit retains the server and drops the instance before stopping it.
/// Startup has its own deadline. A five-minute hard TTL also covers each server
/// while the driver bounds the worker, including synchronous instance deletion.
pub fn fresh_pocket_ic() -> PocketIcInstance<PocketIc> {
    let binary = env::var_os("POCKET_IC_BIN")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .expect("run the Testkit pocketic-check gate and supply its admitted POCKET_IC_BIN");
    let diagnostics = env::var_os("IC_TIMERS_DIAGNOSTICS")
        .map(PathBuf::from)
        .expect("qualification diagnostics directory");
    let logs = ic_host_fs::directory::create_unique_directory(
        &diagnostics,
        "pocketic",
        0o700,
        std::num::NonZeroUsize::new(128),
    )
    .expect("allocate per-instance server logs");
    PocketIcStartupConfig::Spawn {
        server_binary: binary,
        timeout: STARTUP_TIMEOUT,
        server_hard_ttl: Some(Duration::from_secs(5 * 60)),
        server_idle_ttl: None,
        server_output_files: Some((logs.join("stdout.log"), logs.join("stderr.log"))),
    }
    .build_instance(PocketIcBuilder::new().with_application_subnet())
    .expect("construct a fresh owned PocketIC instance through ic-testkit")
}

/// Invoke an anonymous update with no arguments or return values.
pub fn update_unit(pic: &PocketIc, canister_id: ExternalPrincipal, method: Method) {
    update_wire::<(), _>(pic, canister_id, ExternalPrincipal::anonymous(), method, ())
        .expect("probe update succeeds");
}

/// Keep probe protocol selection local; Testkit owns typed transport and errors.
fn wire_call(method: Method) -> WireCallSpec {
    WireCallSpec {
        method: method.name(),
        method_id: method as u64,
        request_limits: REQUEST_LIMITS,
        response_limits: RESPONSE_LIMITS,
    }
}

/// Send one method-bound Wire update without retrying consumer work.
pub fn update_wire<T: DeserializeOwned, A: Serialize>(
    pic: &PocketIc,
    canister_id: ExternalPrincipal,
    caller: ExternalPrincipal,
    method: Method,
    args: A,
) -> Result<T, WireCallError> {
    ic_testkit::pic::update_wire(pic, canister_id, caller, wire_call(method), &args)
}

/// Decode a bounded Wire query reply without changing its caller or timing.
pub fn query_wire<T: DeserializeOwned, A: Serialize>(
    pic: &PocketIc,
    canister_id: ExternalPrincipal,
    caller: ExternalPrincipal,
    method: Method,
    args: A,
) -> Result<T, WireCallError> {
    ic_testkit::pic::query_wire(pic, canister_id, caller, wire_call(method), &args)
}
