use super::harness::{advance, fresh_pocket_ic, install_probe, update_unit};
use super::harness::{query_wire, update_wire};
use ic_testkit::pocket_ic::{CanisterId as Principal, nonblocking::PocketIc};
use ic_timers_pocketic::{BuildContext, Cohort};
use ic_timers_pocketic::{CalibrationMetrics, Measurements, PolicyMetrics};
use ic_timers_test_protocol::rpc::Method;
use ic_timers_test_protocol::{CohortObservation, MemorySamplingMeasurement, OperationMeasurement};
use std::{env, fs, path::PathBuf, time::Duration};

#[tokio::test]
async fn comparable_policy_cohorts_report_size_and_instruction_subjects() {
    let context: BuildContext = serde_json::from_str(
        &env::var("IC_TIMERS_COHORT_CONTEXT").expect("run the qualification driver"),
    )
    .expect("driver context");
    let mut measurements = Measurements::default();
    for &cohort in Cohort::ALL {
        measurements[cohort] = run_cohort(cohort).await;
    }
    measurements.minimal = run_watchdog_calibration(false).await;
    measurements.representative = run_watchdog_calibration(true).await;
    assert!(measurements.representative.work_instructions > measurements.minimal.work_instructions);
    let report = context
        .complete(measurements)
        .expect("complete cohort receipt");
    let output = env::var_os("IC_TIMERS_COHORT_REPORT").expect("driver report path");
    // This is invocation-local evidence. The parent admits it and owns durable
    // publication only after successful subjects and post-measurement checks.
    fs::write(output, serde_json::to_vec_pretty(&report).unwrap())
        .expect("retain candidate cohort report");
}

async fn run_cohort(cohort: Cohort) -> PolicyMetrics {
    let owned = fresh_pocket_ic().await;
    let pic = owned.instance();
    let wasm = cohort_wasm(cohort);
    let wasm_bytes = wasm.len();
    let canister_id = install_probe(pic, wasm).await;

    let initial = observe(pic, canister_id).await;
    assert_eq!(initial.cohort, cohort.name());
    assert!(!initial.registered);
    assert_eq!(initial.callbacks, 0);
    assert_eq!(initial.inventory_len, 0);

    let start = update_measurement(pic, canister_id, Method::CohortStart).await;
    let duplicate = update_measurement(pic, canister_id, Method::EnsureAgain).await;
    assert_eq!(start.changed, cohort != Cohort::Baseline);
    assert!(!duplicate.changed);

    let scheduled = observe(pic, canister_id).await;
    if cohort == Cohort::Baseline {
        assert!(!scheduled.registered);
        assert_eq!(scheduled.schedule_requests.total, 0);
    } else {
        assert!(scheduled.registered);
        assert_eq!(scheduled.schedule_requests.total, 2);
        assert_eq!(scheduled.wakeups_armed.total, 1);
        assert_eq!(scheduled.coalesced.total, 1);
        assert!(scheduled.next_deadline_ns.is_some());
    }

    let cycles_before_dispatch = pic.cycle_balance(canister_id).await;
    pic.advance_time(Duration::from_secs(1)).await;
    advance(pic, Duration::ZERO, 16).await;
    let dispatch_cycles =
        cycles_before_dispatch.saturating_sub(pic.cycle_balance(canister_id).await);
    let completed = observe(pic, canister_id).await;
    match cohort {
        Cohort::Baseline => {
            assert_eq!(completed.callbacks, 0);
            assert_eq!(completed.work_started.total, 0);
        }
        Cohort::Once => {
            assert_eq!(completed.callbacks, 1);
            assert_eq!(completed.work_started.total, 1);
            assert_eq!(completed.work_completed.total, 1);
            assert_eq!(completed.next_deadline_ns, None);
            assert_eq!(completed.work_instruction_samples, 1);
        }
        Cohort::AfterCompletion => {
            assert_eq!(completed.callbacks, 1);
            assert_eq!(completed.work_started.total, 1);
            assert_eq!(completed.work_completed.total, 1);
            assert!(completed.next_deadline_ns.is_some());
            assert_eq!(completed.work_instruction_samples, 1);
        }
        Cohort::Watchdog => {
            assert_eq!(completed.callbacks, 1);
            assert_eq!(completed.scheduler_started.total, 1);
            assert_eq!(completed.work_dispatched.total, 1);
            assert_eq!(completed.work_started.total, 1);
            assert_eq!(completed.work_completed.total, 1);
            assert!(completed.next_deadline_ns.is_some());
            assert_eq!(completed.scheduler_instruction_samples, 1);
            assert_eq!(completed.work_instruction_samples, 1);
        }
    }

    let cancellation = update_measurement(pic, canister_id, Method::Cancel).await;
    assert_eq!(
        cancellation.changed,
        matches!(cohort, Cohort::AfterCompletion | Cohort::Watchdog)
    );
    assert_eq!(observe(pic, canister_id).await.next_deadline_ns, None);
    assert!(
        !update_measurement(pic, canister_id, Method::Cancel)
            .await
            .changed
    );
    pic.advance_time(Duration::from_secs(1)).await;
    advance(pic, Duration::ZERO, 16).await;
    let cancelled = observe(pic, canister_id).await;
    assert_eq!(cancelled.callbacks, completed.callbacks);
    assert_eq!(cancelled.next_deadline_ns, None);

    let memory_sampling = if cohort == Cohort::Watchdog {
        let measurement: MemorySamplingMeasurement = update_wire(
            pic,
            canister_id,
            Principal::anonymous(),
            Method::MemorySamplingOverhead,
            (),
        )
        .await
        .expect("probe update succeeds");
        assert!(measurement.sampled_bracket_instructions >= measurement.empty_bracket_instructions);
        Some(measurement)
    } else {
        None
    };
    let (memory_sampling_empty, memory_sampling_sampled, memory_sampling_overhead) =
        memory_sampling.map_or((0, 0, 0), |measurement| {
            (
                measurement.empty_bracket_instructions,
                measurement.sampled_bracket_instructions,
                measurement
                    .sampled_bracket_instructions
                    .saturating_sub(measurement.empty_bracket_instructions),
            )
        });

    let metrics = PolicyMetrics {
        start_instructions: u128::from(start.instructions),
        duplicate_instructions: u128::from(duplicate.instructions),
        cancel_instructions: u128::from(cancellation.instructions),
        snapshot_instructions: u128::from(completed.snapshot_instructions),
        inventory_instructions: u128::from(completed.inventory_instructions),
        scheduler_instructions: completed.scheduler_instruction_total,
        work_instructions: completed.work_instruction_total,
        memory_sampling_empty: u128::from(memory_sampling_empty),
        memory_sampling_sampled: u128::from(memory_sampling_sampled),
        memory_sampling_overhead: u128::from(memory_sampling_overhead),
        dispatch_cycles,
        wasm_bytes: wasm_bytes as u128,
    };
    owned.close().await;
    metrics
}

async fn run_watchdog_calibration(representative_work: bool) -> CalibrationMetrics {
    let owned = fresh_pocket_ic().await;
    let pic = owned.instance();
    let canister_id = install_probe(pic, cohort_wasm(Cohort::Watchdog)).await;
    if representative_work {
        update_unit(pic, canister_id, Method::UseRepresentativeWatchdogWork).await;
    }
    let start = update_measurement(pic, canister_id, Method::CohortStart).await;
    assert!(start.changed);

    let cycles_before_dispatch = pic.cycle_balance(canister_id).await;
    pic.advance_time(Duration::from_secs(1)).await;
    advance(pic, Duration::ZERO, 16).await;
    let dispatch_cycles =
        cycles_before_dispatch.saturating_sub(pic.cycle_balance(canister_id).await);
    let completed = observe(pic, canister_id).await;
    assert_eq!(completed.scheduler_instruction_samples, 1);
    assert_eq!(completed.work_instruction_samples, 1);
    assert_eq!(completed.work_completed.total, 1);

    let metrics = CalibrationMetrics {
        scheduler_instructions: completed.scheduler_instruction_total,
        work_instructions: completed.work_instruction_total,
        dispatch_cycles,
    };
    owned.close().await;
    metrics
}

fn cohort_wasm(cohort: Cohort) -> Vec<u8> {
    let variable = cohort.environment();
    let path = env::var_os(variable)
        .map(PathBuf::from)
        .unwrap_or_else(|| panic!("{variable} must name the resolved cohort Wasm artifact"));
    let wasm = fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    let report: BuildContext =
        serde_json::from_str(&env::var("IC_TIMERS_COHORT_CONTEXT").expect("driver context"))
            .expect("valid context");
    report.verify_artifact(cohort.name(), &wasm);
    wasm
}

async fn update_measurement(
    pic: &PocketIc,
    canister_id: Principal,
    method: Method,
) -> OperationMeasurement {
    update_wire(pic, canister_id, Principal::anonymous(), method, ())
        .await
        .expect("probe update succeeds")
}

async fn observe(pic: &PocketIc, canister_id: Principal) -> CohortObservation {
    let observed: CohortObservation = query_wire(
        pic,
        canister_id,
        Principal::anonymous(),
        Method::Observe,
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
    ]);
    observed
}
