//! Shared installed-runtime fixture and bounded observations.

use super::harness::{INIT_CYCLES, fresh_pocket_ic, install_probe, update_unit, wait_for_probe};
use super::harness::{canister_rejection, query_wire, update_wire};
use ic_platform_types::principal::ExternalPrincipal;
use ic_testkit::pic::{PocketIcInstance, advance_and_tick};
use ic_testkit::pocket_ic::{CanisterSettings, ErrorCode, PocketIc, RejectCode};
use ic_timers_test_protocol::rpc::{
    Method, OrdinaryPolicy, ProbeLifetime, ProbePolicy, SchedulerReconcileMode,
};
use ic_timers_test_protocol::{
    ChurnObservation, DispatchBatchObservation, OrdinaryObservation, ProbeMemorySummary,
    ProbeSnapshot, TimerProbeSnapshot,
};
use std::{env, fs, path::PathBuf, time::Duration};

const TIMER_EXECUTOR_METHOD: &str = "<ic-timers internal> dispatch";

mod ingress;
mod lifecycle;
mod ordinary;
mod scheduling;
mod watchdog;

// Each invocation owns a fresh server, IC instance and funded canister.
// Returning the owner keeps it alive until the caller finishes its assertions.
async fn installed_probe() -> (PocketIcInstance<PocketIc>, ExternalPrincipal) {
    let owned = fresh_pocket_ic().await;
    let canister_id = install_probe(owned.instance(), probe_wasm()).await;
    (owned, canister_id)
}

async fn freeze_for_five_minutes(
    pic: &PocketIc,
    canister_id: ExternalPrincipal,
    freezing_seconds: u64,
    freezing_reserve: u128,
) {
    pic.update_canister_settings(
        canister_id,
        None,
        CanisterSettings {
            freezing_threshold: Some(freezing_seconds.into()),
            ..CanisterSettings::default()
        },
    )
    .await
    .expect("raise freezing reserve");
    pic.advance_time(Duration::from_secs(300))
        .await
        .expect("advance IC time");
    advance_and_tick(pic, Duration::ZERO, 16)
        .await
        .expect("advance and tick PocketIC");
    let frozen_balance = pic
        .cycle_balance(canister_id)
        .await
        .expect("read canister cycles");
    assert!(frozen_balance > 0, "frozen, not uninstalled");
    assert!(frozen_balance < freezing_reserve);
    let rejected = canister_rejection(
        pic.update_call(
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::Stop.name(),
            ic_wire::native::encode_request(
                Method::Stop as u64,
                &(),
                ic_timers_test_protocol::rpc::REQUEST_LIMITS,
            )
            .unwrap(),
        )
        .await
        .expect_err("a valid timer control update must reject while frozen"),
    );
    assert_eq!(rejected.reject_code, RejectCode::SysTransient);
    assert_eq!(rejected.error_code, ErrorCode::CanisterOutOfCycles);
}

fn probe_wasm() -> Vec<u8> {
    let path = env::var_os("IC_TIMERS_PROBE_WASM")
        .map(PathBuf::from)
        .expect("IC_TIMERS_PROBE_WASM must name the built runtime probe");
    fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

async fn update_bool(pic: &PocketIc, canister_id: ExternalPrincipal, method: Method) -> bool {
    update_wire(pic, canister_id, ExternalPrincipal::anonymous(), method, ())
        .await
        .expect("probe update succeeds")
}

async fn update_u64(pic: &PocketIc, canister_id: ExternalPrincipal, method: Method) -> u64 {
    update_wire(pic, canister_id, ExternalPrincipal::anonymous(), method, ())
        .await
        .expect("probe update succeeds")
}

async fn snapshot(pic: &PocketIc, canister_id: ExternalPrincipal) -> ProbeSnapshot {
    let observed: ProbeSnapshot = query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::Snapshot,
        (),
    )
    .await
    .expect("probe query succeeds");
    super::harness::assert_exact_counters(&[
        observed.schedule_requests,
        observed.scheduler_started,
        observed.wakeups_armed,
        observed.work_dispatched,
        observed.work_started,
        observed.work_completed,
        observed.coalesced,
        observed.unacknowledged,
    ]);
    observed
}

async fn update_deadline(
    pic: &PocketIc,
    canister_id: ExternalPrincipal,
    method: Method,
    deadline: u64,
) {
    update_wire::<(), _>(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        method,
        (deadline,),
    )
    .await
    .expect("probe update succeeds");
}

async fn ordinary_observation(
    pic: &PocketIc,
    canister_id: ExternalPrincipal,
) -> OrdinaryObservation {
    let observed: OrdinaryObservation = query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::OrdinaryObservation,
        (),
    )
    .await
    .expect("probe query succeeds");
    assert!(!observed.work_started.total_overflowed);
    assert!(!observed.work_completed.total_overflowed);
    assert!(!observed.unacknowledged.total_overflowed);
    assert!(!observed.cancellations.total_overflowed);
    observed
}
