//! Fresh, caller-owned testkit servers and IC instances from the verified binary.

use ic_jobs_canister_tests::wire::{WireCallError, WireCallSpec};

use ic_testkit::pic::{InstallSpec, PocketIcInstance, PocketIcStartupConfig, tick_until};
use ic_testkit::pocket_ic::{CanisterId as Principal, PocketIcBuilder, nonblocking::PocketIc};
use ic_timers_test_protocol::rpc::{Method, REQUEST_LIMITS, RESPONSE_LIMITS};
use serde::{Serialize, de::DeserializeOwned};
use std::{convert::Infallible, env, fmt::Debug, path::PathBuf, task::Poll, time::Duration};

const STARTUP_TIMEOUT: Duration = Duration::from_secs(60);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
pub const INIT_CYCLES: u128 = 2_000_000_000_000;

/// Preserve tick-before-observation boundaries while Testkit owns bounded progress.
pub async fn wait_for_probe<T: Debug>(
    pic: &PocketIc,
    rounds: u32,
    mut observe: impl AsyncFnMut(&PocketIc) -> T,
    ready: impl Fn(&T) -> bool,
) -> T {
    let remaining = rounds.checked_sub(1).expect("positive probe round budget");
    let mut last = None;
    pic.tick().await;
    tick_until(pic, remaining, Duration::ZERO, async |pic| {
        let observation = observe(pic).await;
        if ready(&observation) {
            Poll::Ready(Ok::<_, Infallible>(observation))
        } else {
            last = Some(observation);
            Poll::Pending
        }
    })
    .await
    .unwrap_or_else(|error| panic!("{error}; last probe observation: {last:?}"))
}

/// Install one probe with the fixtures' explicit funding and empty init payload.
pub async fn install_probe(pic: &PocketIc, wasm: Vec<u8>) -> Principal {
    ic_testkit::pic::create_and_install(
        pic,
        InstallSpec {
            wasm,
            init_bytes: Vec::new(),
            cycles: INIT_CYCLES,
            install_sender: None,
        },
    )
    .await
}

/// The caller awaits explicit instance deletion before the managed server stops.
/// Startup has its own deadline. A five-minute hard TTL also covers each server
/// while the driver bounds the worker, including asynchronous instance deletion.
pub async fn fresh_pocket_ic() -> PocketIcInstance {
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
    .build_instance(
        PocketIcBuilder::new()
            .with_max_request_time_ms(Some(REQUEST_TIMEOUT.as_millis().try_into().unwrap()))
            .with_application_subnet(),
    )
    .await
    .expect("construct a fresh owned PocketIC instance through ic-testkit")
}

/// Await upstream best-effort deletion while the managed server is alive.
pub async fn close_instance(owned: PocketIcInstance) {
    owned.close().await;
}

/// Invoke an anonymous update with no arguments or return values.
pub async fn update_unit(pic: &PocketIc, canister_id: Principal, method: Method) {
    update_wire::<(), _>(pic, canister_id, Principal::anonymous(), method, ())
        .await
        .expect("probe update succeeds");
}

/// Keep probe protocol selection local; the shared harness owns canonical framing and errors.
fn wire_call(method: Method) -> WireCallSpec {
    WireCallSpec {
        method: method.name(),
        method_id: method as u64,
        request_limits: REQUEST_LIMITS,
        response_limits: RESPONSE_LIMITS,
    }
}

/// Send one method-bound Wire update without retrying consumer work.
pub async fn update_wire<T: DeserializeOwned, A: Serialize>(
    pic: &PocketIc,
    canister_id: Principal,
    caller: Principal,
    method: Method,
    args: A,
) -> Result<T, WireCallError> {
    ic_jobs_canister_tests::wire::update_wire(
        pic,
        ic_platform_types::Principal::from_slice(canister_id.as_slice()),
        ic_platform_types::Principal::from_slice(caller.as_slice()),
        wire_call(method),
        &args,
    )
    .await
}

/// Decode a bounded Wire query reply without changing its caller or timing.
pub async fn query_wire<T: DeserializeOwned, A: Serialize>(
    pic: &PocketIc,
    canister_id: Principal,
    caller: Principal,
    method: Method,
    args: A,
) -> Result<T, WireCallError> {
    ic_jobs_canister_tests::wire::query_wire(
        pic,
        ic_platform_types::Principal::from_slice(canister_id.as_slice()),
        ic_platform_types::Principal::from_slice(caller.as_slice()),
        wire_call(method),
        &args,
    )
    .await
}

/// Exact measurements must not silently include overflowed counters.
pub fn assert_exact_counters(counters: &[ic_timers_test_protocol::CounterObservation]) {
    for (index, counter) in counters.iter().enumerate() {
        assert!(
            !counter.total_overflowed,
            "counter[{index}] overflowed: {counter:?}"
        );
    }
}

/// Advance the selected duration and rounds at the caller's observation boundary.
pub async fn advance(pic: &PocketIc, duration: Duration, rounds: u32) {
    ic_testkit::pic::advance_and_tick(pic, duration, rounds).await;
}

/// Read the IC clock afresh at the caller's observation boundary.
pub async fn now_ns(pic: &PocketIc) -> u64 {
    pic.get_time().await.as_nanos_since_unix_epoch()
}
