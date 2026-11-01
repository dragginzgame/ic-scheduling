use super::harness::{INIT_CYCLES, fresh_pocket_ic, install_probe, update_unit, wait_for_probe};
use super::harness::{query_wire, update_wire};
use ic_platform::pocket_ic::{CanisterSettings, ErrorCode, PocketIc, RejectCode};
use ic_platform::principal::ExternalPrincipal;
use ic_testkit::pic::{PocketIcInstance, advance_and_tick};
use ic_timers_test_protocol::rpc::{
    Method, OrdinaryPolicy, ProbeLifetime, ProbePolicy, SchedulerReconcileMode,
};
use ic_timers_test_protocol::{
    ChurnObservation, DispatchBatchObservation, OrdinaryObservation, ProbeMemorySummary,
    ProbeSnapshot, TimerProbeSnapshot,
};
use std::{env, fs, path::PathBuf, time::Duration};

const TIMER_EXECUTOR_METHOD: &str = "<ic-timers internal> dispatch";

#[test]
fn competing_global_timer_writes_trap_and_roll_back_timer_control() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    let caller = ExternalPrincipal::anonymous();
    let now = pic.get_time().as_nanos_since_unix_epoch();
    let foreign = now + 100_000_000_000;
    let set_foreign = |timestamp| {
        update_wire::<u64, _>(
            pic,
            canister_id,
            caller,
            Method::FixtureGlobalTimerSet,
            (timestamp,),
        )
        .unwrap()
    };
    let assert_conflict = |method: Method, args: Vec<u8>| {
        let rejected = pic
            .update_call(canister_id, caller, method.name(), args)
            .unwrap_err();
        assert!(
            rejected
                .reject_message
                .contains("competing global timer owner"),
            "{rejected:?}"
        );
    };
    use ic_timers_test_protocol::rpc::REQUEST_LIMITS;
    use ic_wire::native::encode_request;

    // First acquisition refuses an already occupied global timer. Trapping
    // rolls back registration, queue publication and the system timer write.
    assert_eq!(set_foreign(foreign), 0);
    assert_conflict(
        Method::Start,
        encode_request(Method::Start as u64, &(), REQUEST_LIMITS).unwrap(),
    );
    assert!(!snapshot(pic, canister_id).registered);
    assert_eq!(set_foreign(0), foreign);
    assert!(update_bool(pic, canister_id, Method::Start));
    let armed = snapshot(pic, canister_id);
    let deadline = armed.next_deadline_ns.unwrap();

    assert_eq!(set_foreign(foreign), deadline);
    assert_conflict(
        Method::ScheduleAt,
        encode_request(Method::ScheduleAt as u64, &(deadline + 1,), REQUEST_LIMITS).unwrap(),
    );
    assert_eq!(snapshot(pic, canister_id), armed);
    assert_eq!(set_foreign(deadline), foreign);

    assert_eq!(set_foreign(0), deadline);
    assert_conflict(
        Method::Stop,
        encode_request(Method::Stop as u64, &(), REQUEST_LIMITS).unwrap(),
    );
    assert_eq!(snapshot(pic, canister_id), armed);
    assert_eq!(set_foreign(deadline), 0);

    // Restore the legitimate state solely within this adversarial fixture.
    // Normal rearm, actual firing (prior timestamp zero) and cancellation work.
    update_wire::<(), _>(
        pic,
        canister_id,
        caller,
        Method::ScheduleAt,
        (deadline + 1,),
    )
    .unwrap();
    pic.advance_time(Duration::from_nanos(1_000_000_001));
    let completed = wait_for_probe(
        pic,
        32,
        |pic| snapshot(pic, canister_id),
        |s| s.completed_work == 1,
    );
    assert_eq!(completed.scheduler_started.total, 1);
    update_unit(pic, canister_id, Method::Stop);
    assert_eq!(snapshot(pic, canister_id).next_deadline_ns, None);
    advance_and_tick(pic, Duration::from_secs(2), 16);
    assert_eq!(snapshot(pic, canister_id).completed_work, 1);
}

#[test]
fn host_inspection_refuses_timer_ingress_but_foreign_calls_still_authenticate() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    let relay_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, Method::Start));
    let before = snapshot(pic, canister_id);
    let balance = pic.cycle_balance(canister_id);
    // The first pending dispatch ID is valid: admission must not rely on a
    // malformed payload or a nonexistent task to prevent early execution.
    let payload = 1_u64.to_le_bytes().to_vec();
    let denied = pic
        .update_call(
            canister_id,
            ExternalPrincipal::anonymous(),
            TIMER_EXECUTOR_METHOD,
            payload.clone(),
        )
        .unwrap_err();
    assert_eq!(denied.error_code, ErrorCode::CanisterCalledTrap);
    assert!(
        denied
            .reject_message
            .contains("probe ingress: timer dispatch is internal")
    );
    assert_eq!(snapshot(pic, canister_id), before);
    assert_eq!(pic.cycle_balance(canister_id), balance);

    // The host's existing payload policy still runs for non-timer methods.
    let denied = pic
        .update_call(
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::Start.name(),
            vec![0; ic_timers_test_protocol::rpc::REQUEST_LIMITS.bytes + 1],
        )
        .unwrap_err();
    assert_eq!(denied.error_code, ErrorCode::CanisterCalledTrap);
    assert!(
        denied
            .reject_message
            .contains("probe ingress: host payload limit")
    );
    assert_eq!(snapshot(pic, canister_id), before);

    let balance = pic.cycle_balance(canister_id);
    let rejected: String = update_wire(
        pic,
        relay_id,
        ExternalPrincipal::anonymous(),
        Method::FixtureForeignDispatch,
        (canister_id.as_slice().to_vec(), payload),
    )
    .unwrap();
    assert!(rejected.contains("self-call required"), "{rejected}");
    assert_eq!(snapshot(pic, canister_id), before);
    let foreign_cost = balance - pic.cycle_balance(canister_id);
    assert!(
        foreign_cost > 0,
        "endpoint authentication still costs target cycles"
    );
    println!("foreign_dispatch_authentication_cycles={foreign_cost}");

    pic.advance_time(Duration::from_secs(1));
    let completed = wait_for_probe(
        pic,
        32,
        |pic| snapshot(pic, canister_id),
        |s| s.completed_work == 1,
    );
    assert_eq!(completed.scheduler_started.total, 1);
    assert_eq!(completed.work_started.total, 1);
}

#[test]
fn typed_entrypoints_reject_invalid_wire_before_mutating_timer_state() {
    use ic_timers_test_protocol::rpc::{REQUEST_LIMITS, RESPONSE_LIMITS};
    use ic_wire::native::{decode_response, encode_request, encode_response};
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    let id = Method::StartOrdinary as u64;
    let valid = encode_request(
        id,
        &(OrdinaryPolicy::Once, ProbeLifetime::Retained),
        REQUEST_LIMITS,
    )
    .unwrap();
    let mut truncated = valid.clone();
    truncated.pop();
    for arguments in [
        b"not wire".to_vec(),
        encode_request(id, &(2_u8, ProbeLifetime::Retained), REQUEST_LIMITS).unwrap(),
        encode_request(id, &(OrdinaryPolicy::Once, 2_u8), REQUEST_LIMITS).unwrap(),
        encode_request(id, &(false,), REQUEST_LIMITS).unwrap(),
        encode_request(id, &(false, false, 0_u8), REQUEST_LIMITS).unwrap(),
        encode_request(
            Method::TrapOrdinaryWork as u64,
            &(false, false),
            REQUEST_LIMITS,
        )
        .unwrap(),
        encode_response(id, &(false, false), RESPONSE_LIMITS).unwrap(),
        vec![0; REQUEST_LIMITS.bytes + 1],
        truncated,
    ] {
        assert!(
            pic.update_call(
                canister_id,
                ExternalPrincipal::anonymous(),
                Method::StartOrdinary.name(),
                arguments
            )
            .is_err(),
            "invalid Wire arguments must reject before registration"
        );
        assert!(!ordinary_observation(pic, canister_id).declared);
        assert_eq!(snapshot(pic, canister_id).scheduler_started.total, 0);
    }
    // Wire 0.7 rejects nonminimal varints during decoding, before the endpoint
    // can reach its missing-registration trap or reconcile a timer deadline.
    let mut nonminimal =
        encode_request(Method::ScheduleAt as u64, &(0_u64,), REQUEST_LIMITS).unwrap();
    assert_eq!(nonminimal.pop(), Some(0));
    nonminimal.extend_from_slice(&[0x80, 0]);
    let rejection = pic
        .update_call(
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::ScheduleAt.name(),
            nonminimal,
        )
        .unwrap_err();
    assert!(
        rejection
            .reject_message
            .contains("probe request: wire Malformed")
    );
    assert_eq!(snapshot(pic, canister_id).scheduler_started.total, 0);
    // Exercise our protocol-to-Testkit adapter at its three failure boundaries.
    // Raw malformed frames above remain direct calls so no client normalizes them.
    let caller = ExternalPrincipal::anonymous();
    let error = update_wire::<(), _>(
        pic,
        canister_id,
        caller,
        Method::StartOrdinary,
        vec![0_u8; REQUEST_LIMITS.bytes + 1],
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ic_testkit::pic::WireCallError::Encode { .. }
    ));
    let error = update_wire::<(), _>(
        pic,
        canister_id,
        caller,
        Method::StartOrdinary,
        (2_u8, false),
    )
    .unwrap_err();
    let ic_testkit::pic::WireCallError::Call { context, source } = error else {
        panic!("expected probe rejection, got {error}");
    };
    assert_eq!(context.operation, "update_call");
    assert_eq!(context.canister_id, canister_id);
    assert_eq!(context.caller, caller);
    assert_eq!(context.method, Method::StartOrdinary.name());
    assert_eq!(context.method_id, id);
    assert!(matches!(
        source.as_ref(),
        ic_testkit::pic::PocketIcCallFailure::Rejected(_)
    ));
    let error = query_wire::<bool, _>(pic, canister_id, caller, Method::OrdinaryObservation, ())
        .unwrap_err();
    let ic_testkit::pic::WireCallError::Decode { context, .. } = error else {
        panic!("expected response shape rejection, got {error}");
    };
    assert_eq!(context.operation, "query_call");
    assert_eq!(context.method_id, Method::OrdinaryObservation as u64);
    assert!(!ordinary_observation(pic, canister_id).declared);
    let reply = pic
        .update_call(
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::StartOrdinary.name(),
            valid,
        )
        .expect("valid arguments still work after rejected input");
    decode_response::<()>(id, &reply, RESPONSE_LIMITS).expect("method-bound unit reply");
    // Responses need Deserialize only, including through our native adapter.
    #[derive(Debug, serde::Deserialize)]
    #[serde(transparent)]
    struct ResponseOnly(OrdinaryObservation);
    let observed =
        query_wire::<ResponseOnly, _>(pic, canister_id, caller, Method::OrdinaryObservation, ())
            .unwrap();
    assert!(observed.0.declared);
}

#[test]
fn self_call_queue_pressure_defers_timer_submission_without_losing_work() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    update_unit(pic, canister_id, Method::PressureAndStart);
    advance_and_tick(pic, Duration::ZERO, 8);
    let (failed, replies): (u64, u64) = query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::PressureObservation,
        (),
    )
    .expect("pressure counters");
    println!("ic_timers_queue_pressure failed={failed} replies={replies}");
    assert!(failed > 0);
    assert!(replies >= 500);
    assert!(pic.cycle_balance(canister_id) > INIT_CYCLES / 2);
    assert_eq!(snapshot(pic, canister_id).scheduler_started.total, 0);
    update_unit(pic, canister_id, Method::ReleasePressure);
    advance_and_tick(pic, Duration::ZERO, 16);
    assert_eq!(snapshot(pic, canister_id).scheduler_started.total, 0);
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 16);
    let completed = snapshot(pic, canister_id);
    assert_eq!(completed.scheduler_started.total, 1);
    assert_eq!(completed.completed_work, 1);
    advance_and_tick(pic, Duration::ZERO, 8);
    assert_eq!(snapshot(pic, canister_id).completed_work, 1);
}

#[test]
fn deferred_submission_cancellation_and_replacement_do_not_revive_old_work() {
    for replace in [false, true] {
        let owned = fresh_pocket_ic();
        let pic = owned.instance();
        let canister_id = install_probe(pic, probe_wasm());
        update_unit(pic, canister_id, Method::PressureAndStart);
        advance_and_tick(pic, Duration::ZERO, 8);
        let original = snapshot(pic, canister_id);
        assert_eq!(original.scheduler_started.total, 0);
        assert!(pic.cycle_balance(canister_id) > INIT_CYCLES / 2);
        update_unit(
            pic,
            canister_id,
            if replace {
                Method::ReplaceRegistration
            } else {
                Method::Stop
            },
        );
        let changed = snapshot(pic, canister_id);
        if replace {
            assert_ne!(changed.registration_id, original.registration_id);
        } else {
            assert_eq!(changed.next_deadline_ns, None);
        }
        update_unit(pic, canister_id, Method::ReleasePressure);
        advance_and_tick(pic, Duration::ZERO, 16);
        // A fresh canister's first backend identity belongs to the old scheduler.
        // Even an authenticated late delivery must not revive it after removal.
        let stale_delivery: bool = update_wire(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::FixtureDispatchPayload,
            (1_u64.to_le_bytes().to_vec(),),
        )
        .expect("stale self-call result");
        assert!(stale_delivery);
        assert_eq!(snapshot(pic, canister_id).completed_work, 0);
        pic.advance_time(Duration::from_secs(1));
        advance_and_tick(pic, Duration::ZERO, 16);
        let recovered = snapshot(pic, canister_id);
        assert_eq!(recovered.completed_work, u64::from(replace));
        assert_eq!(recovered.scheduler_started.total, u128::from(replace));
        if !replace {
            update_wire::<(), _>(
                pic,
                canister_id,
                ExternalPrincipal::anonymous(),
                Method::ScheduleAt,
                (0_u64,),
            )
            .expect("reschedule retained registration");
            advance_and_tick(pic, Duration::ZERO, 16);
            assert_eq!(snapshot(pic, canister_id).completed_work, 1);
        }
        advance_and_tick(pic, Duration::ZERO, 8);
        assert_eq!(snapshot(pic, canister_id).completed_work, 1);
    }
}

#[test]
fn immediate_initial_watchdog_runs_without_advancing_cadence_time() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());

    assert!(update_bool(pic, canister_id, Method::StartImmediately));
    let initial = snapshot(pic, canister_id);
    assert_eq!(initial.completed_work, 0);
    // Initial After(0) is an explicit request, not a callback continuation.
    assert!(!initial.immediate_scheduling);
    assert_eq!(initial.latest_requested_delay_ns, Some(0));
    assert_eq!(initial.latest_armed_delay_ns, Some(0));
    assert_eq!(initial.schedule_requests.total, 1);
    assert_eq!(initial.wakeups_armed.total, 1);

    advance_and_tick(pic, Duration::ZERO, 16);
    let completed = snapshot(pic, canister_id);
    assert_eq!(completed.completed_work, 1);
    assert_eq!(completed.scheduler_started.total, 1);
    assert_eq!(completed.work_dispatched.total, 1);
    assert_eq!(completed.work_completed.total, 1);
    assert!(completed.next_deadline_ns.is_some());
}

#[test]
fn successful_immediate_continuation_runs_again_without_waiting_for_cadence() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, Method::Start));
    update_unit(pic, canister_id, Method::ContinueImmediatelyOnNextWork);

    let cycles_before_first = pic.cycle_balance(canister_id);
    pic.advance_time(Duration::from_secs(1));
    let immediate = wait_for_probe(
        pic,
        16,
        |pic| snapshot(pic, canister_id),
        |current| current.completed_work == 1,
    );
    assert!(immediate.immediate_scheduling);
    assert_eq!(immediate.latest_requested_delay_ns, Some(0));
    assert_eq!(immediate.latest_armed_delay_ns, Some(0));
    assert_eq!(immediate.schedule_requests.total, 1);
    assert_eq!(immediate.wakeups_armed.total, 3);
    assert_eq!(immediate.work_dispatched.total, 1);
    let first_dispatch_cycles = cycles_before_first.saturating_sub(pic.cycle_balance(canister_id));

    let cycles_before_second = pic.cycle_balance(canister_id);
    advance_and_tick(pic, Duration::ZERO, 16);
    let continued = snapshot(pic, canister_id);
    assert_eq!(continued.completed_work, 2);
    assert_eq!(continued.scheduler_started.total, 2);
    assert_eq!(continued.work_dispatched.total, 2);
    assert_eq!(continued.work_completed.total, 2);
    assert_eq!(continued.wakeups_armed.total, 4);
    assert_eq!(continued.schedule_requests.total, 1);
    assert_eq!(continued.coalesced.total, 0);
    let second_dispatch_cycles =
        cycles_before_second.saturating_sub(pic.cycle_balance(canister_id));
    println!(
        "ic_timers_immediate first_scheduler={} first_work={} first_cycles={} second_scheduler={} second_work={} second_cycles={}",
        immediate.scheduler_instruction_total,
        immediate.work_instruction_total,
        first_dispatch_cycles,
        continued
            .scheduler_instruction_total
            .saturating_sub(immediate.scheduler_instruction_total),
        continued
            .work_instruction_total
            .saturating_sub(immediate.work_instruction_total),
        second_dispatch_cycles,
    );
}

#[test]
fn trapped_work_keeps_committed_successor_and_executor_is_private() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());

    assert!(update_bool(pic, canister_id, Method::Start));
    assert!(!update_bool(pic, canister_id, Method::Start));
    assert!(update_bool(pic, canister_id, Method::StartSecondary));
    assert!(!update_bool(pic, canister_id, Method::StartSecondary));

    let before_external = snapshot(pic, canister_id);
    let external = pic.update_call(
        canister_id,
        ExternalPrincipal::anonymous(),
        TIMER_EXECUTOR_METHOD,
        0_u64.to_be_bytes().to_vec(),
    );
    assert!(
        external.is_err(),
        "provider executor must reject external ingress"
    );
    assert_eq!(snapshot(pic, canister_id), before_external);

    let trap_at = update_u64(pic, canister_id, Method::ArmTrap);
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 12);
    let trapped = snapshot(pic, canister_id);
    assert_eq!(trapped.registration_id, before_external.registration_id);
    assert_eq!(trapped.completed_work, 0);
    assert_eq!(trapped.secondary_completed_work, 1);
    assert_eq!(trapped.scheduler_started.total, 1);
    assert_eq!(trapped.work_dispatched.total, 1);
    assert_eq!(trapped.work_completed.total, 0);
    assert_eq!(trapped.scheduler_instruction_samples, 1);
    assert!(trapped.scheduler_instruction_total > 0);
    assert_eq!(trapped.work_instruction_samples, 0);
    let trapped_scheduler_memory = trapped
        .scheduler_memory
        .as_ref()
        .expect("normally completed scheduler must retain a memory sample");
    assert_eq!(trapped_scheduler_memory.samples, 1);
    assert_memory_summary_is_coherent(trapped_scheduler_memory);
    assert_eq!(trapped.work_memory, None);
    assert_eq!(trapped.next_deadline_ns, Some(trap_at + 1_000_000_000));

    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 12);
    let recovered = snapshot(pic, canister_id);
    assert_eq!(recovered.registration_id, trapped.registration_id);
    assert_eq!(recovered.completed_work, 1);
    assert_eq!(recovered.secondary_completed_work, 2);
    assert_eq!(recovered.scheduler_started.total, 2);
    assert_eq!(recovered.work_dispatched.total, 2);
    assert_eq!(recovered.work_completed.total, 1);
    assert_eq!(recovered.unacknowledged.total, 1);
    assert!(!recovered.last_unacknowledged);
    assert_eq!(recovered.scheduler_instruction_samples, 2);
    assert!(recovered.scheduler_instruction_total > trapped.scheduler_instruction_total);
    assert_eq!(recovered.work_instruction_samples, 1);
    assert!(recovered.work_instruction_total > 0);
    let recovered_scheduler_memory = recovered
        .scheduler_memory
        .as_ref()
        .expect("second normal scheduler completion must retain a memory sample");
    assert_eq!(recovered_scheduler_memory.samples, 2);
    assert_memory_summary_is_coherent(recovered_scheduler_memory);
    let recovered_work_memory = recovered
        .work_memory
        .as_ref()
        .expect("normally completed work must retain a memory sample");
    assert_eq!(recovered_work_memory.samples, 1);
    assert_memory_summary_is_coherent(recovered_work_memory);
    assert!(
        recovered
            .next_deadline_ns
            .is_some_and(|deadline| deadline > trap_at)
    );

    pic.upgrade_canister(canister_id, probe_wasm(), Vec::new(), None)
        .expect("upgrade should succeed");
    let upgraded = snapshot(pic, canister_id);
    assert_ne!(upgraded.registration_id, recovered.registration_id);
    assert!(upgraded.registered);
    assert!(upgraded.secondary_registered);
    assert!(upgraded.post_upgrade_reconstructed);
    assert_eq!(upgraded.completed_work, recovered.completed_work);
    assert_eq!(
        upgraded.secondary_completed_work,
        recovered.secondary_completed_work
    );
    assert_eq!(upgraded.scheduler_started.total, 0);
    assert_eq!(upgraded.wakeups_armed.total, 1);
    assert!(upgraded.next_deadline_ns.is_some());

    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 12);
    let after_upgrade = snapshot(pic, canister_id);
    assert_eq!(after_upgrade.completed_work, recovered.completed_work + 1);
    assert_eq!(
        after_upgrade.secondary_completed_work,
        recovered.secondary_completed_work + 1
    );
    println!(
        "ic_timers_watchdog scheduler_samples={} scheduler_instructions={} work_samples={} work_instructions={} unacknowledged={}",
        recovered.scheduler_instruction_samples,
        recovered.scheduler_instruction_total,
        recovered.work_instruction_samples,
        recovered.work_instruction_total,
        recovered.unacknowledged.total,
    );
}

fn assert_memory_summary_is_coherent(summary: &ProbeMemorySummary) {
    assert!(summary.latest_wasm_end_pages >= summary.latest_wasm_start_pages);
    assert!(summary.latest_stable_end_pages >= summary.latest_stable_start_pages);
    assert!(
        summary.maximum_wasm_growth_pages
            >= summary
                .latest_wasm_end_pages
                .saturating_sub(summary.latest_wasm_start_pages)
    );
    assert!(
        summary.maximum_stable_growth_pages
            >= summary
                .latest_stable_end_pages
                .saturating_sub(summary.latest_stable_start_pages)
    );
}

#[test]
fn instruction_exhaustion_leaves_successor_for_later_progress() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, Method::Start));

    let exhaust_at = update_u64(pic, canister_id, Method::ArmExhaustion);
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 30);
    let exhausted = snapshot(pic, canister_id);
    assert_eq!(exhausted.completed_work, 0);
    assert_eq!(exhausted.scheduler_started.total, 1);
    assert_eq!(exhausted.work_dispatched.total, 1);
    assert_eq!(exhausted.work_completed.total, 0);
    assert_eq!(exhausted.work_instruction_samples, 0);
    let exhausted_scheduler_memory = exhausted
        .scheduler_memory
        .as_ref()
        .expect("normally completed scheduler must retain a memory sample");
    assert_eq!(exhausted_scheduler_memory.samples, 1);
    assert_memory_summary_is_coherent(exhausted_scheduler_memory);
    assert_eq!(exhausted.work_memory, None);
    assert_eq!(exhausted.next_deadline_ns, Some(exhaust_at + 1_000_000_000));

    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 16);
    let recovered = snapshot(pic, canister_id);
    assert_eq!(recovered.completed_work, 1);
    assert_eq!(recovered.scheduler_started.total, 2);
    assert_eq!(recovered.work_completed.total, 1);
    assert_eq!(recovered.unacknowledged.total, 1);
    assert_eq!(recovered.work_instruction_samples, 1);
    let recovered_scheduler_memory = recovered
        .scheduler_memory
        .as_ref()
        .expect("second normal scheduler completion must retain a memory sample");
    assert_eq!(recovered_scheduler_memory.samples, 2);
    assert_memory_summary_is_coherent(recovered_scheduler_memory);
    let recovered_work_memory = recovered
        .work_memory
        .as_ref()
        .expect("normally completed recovery work must retain a memory sample");
    assert_eq!(recovered_work_memory.samples, 1);
    assert_memory_summary_is_coherent(recovered_work_memory);
}

#[test]
fn scheduler_abandonment_under_round_pressure_recovers_after_stop() {
    for successor in [false, true] {
        for mode in SchedulerReconcileMode::ALL {
            let owned = fresh_pocket_ic();
            let pic = owned.instance();
            let canister_id = install_probe(pic, probe_wasm());
            if successor {
                assert!(update_bool(pic, canister_id, Method::Start));
                pic.advance_time(Duration::from_secs(1));
                wait_for_probe(
                    pic,
                    32,
                    |pic| snapshot(pic, canister_id),
                    |s| s.completed_work == 1,
                );
            }
            update_wire::<(), _>(
                pic,
                canister_id,
                ExternalPrincipal::anonymous(),
                Method::StartSchedulerPressure,
                (successor,),
            )
            .unwrap();
            let armed = snapshot(pic, canister_id);
            let deadline = armed.next_deadline_ns.unwrap();
            pic.advance_time(Duration::from_nanos(
                deadline
                    .checked_sub(pic.get_time().as_nanos_since_unix_epoch())
                    .unwrap(),
            ));
            let pressure: u64 = wait_for_probe(
                pic,
                8,
                |pic| {
                    query_wire(
                        pic,
                        canister_id,
                        ExternalPrincipal::anonymous(),
                        Method::SchedulerPressureCompleted,
                        (),
                    )
                    .expect("observe committed round pressure")
                },
                |completed| *completed > 0,
            );
            let before_stop = snapshot(pic, canister_id);
            println!("successor={successor} mode={mode:?} committed_pressure={pressure}");
            assert_eq!(
                before_stop.scheduler_started.total, armed.scheduler_started.total,
                "must reach the queued scheduler boundary"
            );
            pic.stop_canister(canister_id, None)
                .expect("stop queued scheduler");
            pic.start_canister(canister_id, None)
                .expect("restart queued scheduler");
            // Time and ticks alone must not be mistaken for explicit owner repair.
            advance_and_tick(pic, Duration::from_secs(2), 16);
            let stopped = snapshot(pic, canister_id);
            assert_eq!(stopped.registration_id, armed.registration_id);
            assert_eq!(stopped.work_dispatched, armed.work_dispatched);
            assert_eq!(stopped.work_started, armed.work_started);
            assert_eq!(stopped.work_completed, armed.work_completed);
            assert_eq!(stopped.unacknowledged, armed.unacknowledged);
            assert_eq!(
                stopped.scheduler_started.total, armed.scheduler_started.total,
                "stop must abandon the scheduler before its first poll"
            );
            assert_eq!(stopped.completed_work, armed.completed_work);
            let repaired: bool = update_wire(
                pic,
                canister_id,
                ExternalPrincipal::anonymous(),
                Method::ReconcileSchedulerPressure,
                (mode, deadline),
            )
            .unwrap();
            assert!(repaired);
            let recovered = wait_for_probe(
                pic,
                64,
                |pic| snapshot(pic, canister_id),
                |s| s.completed_work == armed.completed_work + 1,
            );
            assert_eq!(
                recovered.scheduler_started.total,
                armed.scheduler_started.total + 1
            );
            assert_eq!(
                recovered.work_dispatched.total,
                armed.work_dispatched.total + 1
            );
            assert_eq!(
                recovered.work_completed.total,
                armed.work_completed.total + 1
            );
            assert_eq!(recovered.work_started.total, armed.work_started.total + 1);
            assert_eq!(recovered.registration_id, armed.registration_id);
            assert_eq!(recovered.unacknowledged, armed.unacknowledged);
            advance_and_tick(pic, Duration::ZERO, 16);
            assert_eq!(
                snapshot(pic, canister_id).completed_work,
                armed.completed_work + 1
            );
        }
    }
}

#[test]
fn commit_window_ensure_stop_resume_and_large_overdue_time_remain_bounded() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, Method::Start));

    update_unit(pic, canister_id, Method::Stop);
    let inactive = snapshot(pic, canister_id);
    assert_eq!(inactive.next_deadline_ns, None);
    assert!(!update_bool(
        pic,
        canister_id,
        Method::EnsureFromReturnedError
    ));
    let ensured = snapshot(pic, canister_id);
    assert!(ensured.next_deadline_ns.is_some());
    let arms_after_ensure = ensured.wakeups_armed.total;
    assert!(!update_bool(
        pic,
        canister_id,
        Method::EnsureFromReturnedError
    ));
    assert_eq!(
        snapshot(pic, canister_id).wakeups_armed.total,
        arms_after_ensure
    );

    pic.stop_canister(canister_id, None)
        .expect("canister stop should succeed");
    pic.advance_time(Duration::from_secs(300));
    advance_and_tick(pic, Duration::ZERO, 4);
    pic.start_canister(canister_id, None)
        .expect("canister resume should succeed");
    advance_and_tick(pic, Duration::ZERO, 16);
    let resumed = snapshot(pic, canister_id);
    assert_eq!(resumed.completed_work, 1);
    assert_eq!(resumed.scheduler_started.total, 1);
    assert_eq!(resumed.work_dispatched.total, 1);
    assert_eq!(resumed.work_started.total, 1);
    assert_eq!(resumed.work_completed.total, 1);
    assert!(resumed.next_deadline_ns.is_some());

    update_unit(pic, canister_id, Method::StopOnNextWork);
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 16);
    let terminal = snapshot(pic, canister_id);
    assert_eq!(terminal.completed_work, 2);
    assert_eq!(terminal.next_deadline_ns, None);
    pic.advance_time(Duration::from_secs(2));
    advance_and_tick(pic, Duration::ZERO, 8);
    assert_eq!(snapshot(pic, canister_id).completed_work, 2);
}

#[test]
fn insufficient_cycles_defer_due_work_until_top_up() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, Method::Start));
    let (_burned, call_cost): (u128, u128) = update_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::BurnToBelowTimerCallCost,
        (),
    )
    .expect("probe multi-result update succeeds");
    assert!(call_cost > 0);

    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 8);
    let deferred = snapshot(pic, canister_id);
    assert_eq!(deferred.completed_work, 0);
    assert_eq!(deferred.scheduler_started.total, 0);

    pic.add_cycles(canister_id, INIT_CYCLES);
    advance_and_tick(pic, Duration::ZERO, 4);
    assert_eq!(snapshot(pic, canister_id).scheduler_started.total, 0);
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 16);
    let progressed = snapshot(pic, canister_id);
    assert_eq!(progressed.completed_work, 1);
    assert_eq!(progressed.scheduler_started.total, 1);
}

#[test]
fn frozen_canister_resumes_overdue_timer_after_top_up_only() {
    assert_frozen_watchdog_resumes_after_top_up(false);
}

#[test]
fn freeze_between_scheduler_and_work_resumes_after_top_up_only() {
    assert_frozen_watchdog_resumes_after_top_up(true);
}

fn assert_frozen_watchdog_resumes_after_top_up(after_scheduler: bool) {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    // Raise the real freezing reserve above the existing balance without burning
    // all cycles or uninstalling the canister. Derive it from this instance's
    // storage charge before making work due: management observation must not
    // silently consume the scheduler/work gap being qualified.
    let status = pic.canister_status(canister_id, None).unwrap();
    let balance = pic.cycle_balance(canister_id);
    let idle_per_day: u128 = status.idle_cycles_burned_per_day.0.try_into().unwrap();
    assert!(idle_per_day > 0);
    let freezing_seconds = u64::try_from((balance * 4 * 86_400).div_ceil(idle_per_day))
        .expect("fixture freezing threshold fits u64");
    let freezing_reserve = idle_per_day * u128::from(freezing_seconds) / 86_400;
    assert!(update_bool(pic, canister_id, Method::Start));
    let armed = snapshot(pic, canister_id);
    assert_eq!(armed.scheduler_started.total, 0);
    assert_eq!(armed.completed_work, 0);
    let before = if after_scheduler {
        pic.advance_time(Duration::from_secs(1));
        let dispatched = wait_for_probe(
            pic,
            12,
            |pic| snapshot(pic, canister_id),
            |s| s.work_dispatched.total == 1 && s.work_started.total == 0,
        );
        assert_eq!(dispatched.scheduler_started.total, 1);
        assert_eq!(dispatched.completed_work, 0);
        assert!(dispatched.next_deadline_ns > armed.next_deadline_ns);
        dispatched
    } else {
        armed
    };
    let deadline = before
        .next_deadline_ns
        .expect("committed watchdog deadline");
    freeze_for_five_minutes(pic, canister_id, freezing_seconds, freezing_reserve);
    assert!(pic.get_time().as_nanos_since_unix_epoch() > deadline);
    // No ensure/reconcile call, threshold reset, raw timer write or upgrade can
    // repair the wakeup here. A top-up and replica rounds must suffice.
    pic.add_cycles(canister_id, balance * 16);
    // Read-only observation before another round proves the frozen interval
    // left the overdue registration intact and executed no timer work.
    let thawed = snapshot(pic, canister_id);
    assert!(thawed.registered);
    assert_eq!(thawed.registration_id, before.registration_id);
    assert_eq!(
        thawed.scheduler_started.total,
        before.scheduler_started.total
    );
    assert_eq!(thawed.work_dispatched.total, before.work_dispatched.total);
    assert_eq!(thawed.work_started.total, 0);
    assert_eq!(thawed.completed_work, 0);
    assert_eq!(thawed.next_deadline_ns, Some(deadline));
    // The gap case has both queued work and an overdue committed successor.
    // Each may execute once. Freezing before the initial scheduler has only one
    // overdue attempt; neither case replays five minutes of missed cadence.
    let settled_count = if after_scheduler { 2 } else { 1 };
    let resumed = wait_for_probe(
        pic,
        32,
        |pic| snapshot(pic, canister_id),
        |s| s.completed_work > 0,
    );
    assert!(resumed.registered);
    assert_eq!(resumed.completed_work, 1);
    assert!((1..=u128::from(settled_count)).contains(&resumed.scheduler_started.total));
    assert_eq!(resumed.work_started.total, 1);
    assert_eq!(resumed.work_completed.total, 1);
    assert!(!resumed.post_upgrade_reconstructed);
    advance_and_tick(pic, Duration::ZERO, 16);
    let settled = snapshot(pic, canister_id);
    assert!(settled.registered);
    assert_eq!(settled.registration_id, before.registration_id);
    assert_eq!(settled.completed_work, settled_count);
    assert_eq!(settled.scheduler_started.total, u128::from(settled_count));
    assert_eq!(settled.work_dispatched.total, u128::from(settled_count));
    assert_eq!(settled.work_started.total, u128::from(settled_count));
    assert_eq!(settled.work_completed.total, u128::from(settled_count));
    assert_eq!(settled.unacknowledged.total, 0);
    assert!(
        settled
            .next_deadline_ns
            .is_some_and(|next| next > pic.get_time().as_nanos_since_unix_epoch())
    );
    pic.advance_time(Duration::from_secs(1));
    let recurring = wait_for_probe(
        pic,
        32,
        |pic| snapshot(pic, canister_id),
        |s| s.completed_work > settled_count,
    );
    assert_eq!(recurring.registration_id, before.registration_id);
    assert_eq!(recurring.completed_work, settled_count + 1);
    assert_eq!(
        recurring.scheduler_started.total,
        u128::from(settled_count + 1)
    );
    assert_eq!(recurring.work_started.total, u128::from(settled_count + 1));
    assert_eq!(
        recurring.work_completed.total,
        u128::from(settled_count + 1)
    );
    let after = pic.canister_status(canister_id, None).unwrap();
    assert_eq!(after.module_hash, status.module_hash);
    assert_eq!(after.settings.freezing_threshold, freezing_seconds);
}

fn freeze_for_five_minutes(
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
    .expect("raise freezing reserve");
    pic.advance_time(Duration::from_secs(300));
    advance_and_tick(pic, Duration::ZERO, 16);
    let frozen_balance = pic.cycle_balance(canister_id);
    assert!(frozen_balance > 0, "frozen, not uninstalled");
    assert!(frozen_balance < freezing_reserve);
    let rejected = pic
        .update_call(
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
        .expect_err("a valid timer control update must reject while frozen");
    assert_eq!(rejected.reject_code, RejectCode::SysTransient);
    assert_eq!(rejected.error_code, ErrorCode::CanisterOutOfCycles);
}

#[test]
fn full_inventory_is_bounded_deterministic_and_measured() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, Method::Start));
    update_unit(pic, canister_id, Method::Stop);
    assert_eq!(update_u64(pic, canister_id, Method::FillInventory), 64);
    assert_eq!(update_u64(pic, canister_id, Method::FillInventory), 64);

    let (len, instructions, ordered): (u64, u64, bool) = query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::InventoryMeasurement,
        (),
    )
    .expect("probe inventory query succeeds");
    assert_eq!(len, 64);
    assert!(instructions > 0);
    assert!(ordered);
    println!("ic_timers_inventory entries={len} instructions={instructions}");
}

#[test]
fn cancellation_between_scheduler_and_work_makes_late_provider_delivery_harmless() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, Method::Start));
    pic.advance_time(Duration::from_secs(1));

    let dispatched = wait_for_probe(
        pic,
        12,
        |pic| snapshot(pic, canister_id),
        |current| current.work_dispatched.total == 1 && current.work_started.total == 0,
    );
    assert_eq!(dispatched.completed_work, 0);
    assert!(dispatched.next_deadline_ns.is_some());

    update_unit(pic, canister_id, Method::Stop);
    advance_and_tick(pic, Duration::ZERO, 12);
    let cancelled = snapshot(pic, canister_id);
    assert_eq!(cancelled.completed_work, 0);
    assert_eq!(cancelled.work_started.total, 0);
    assert_eq!(cancelled.work_completed.total, 0);
    assert_eq!(cancelled.next_deadline_ns, None);
}

fn probe_wasm() -> Vec<u8> {
    let path = env::var_os("IC_TIMERS_PROBE_WASM")
        .map(PathBuf::from)
        .expect("IC_TIMERS_PROBE_WASM must name the built runtime probe");
    fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn update_bool(pic: &PocketIc, canister_id: ExternalPrincipal, method: Method) -> bool {
    update_wire(pic, canister_id, ExternalPrincipal::anonymous(), method, ())
        .expect("probe update succeeds")
}

fn update_u64(pic: &PocketIc, canister_id: ExternalPrincipal, method: Method) -> u64 {
    update_wire(pic, canister_id, ExternalPrincipal::anonymous(), method, ())
        .expect("probe update succeeds")
}

fn snapshot(pic: &PocketIc, canister_id: ExternalPrincipal) -> ProbeSnapshot {
    let observed: ProbeSnapshot = query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::Snapshot,
        (),
    )
    .expect("probe query succeeds");
    assert!(!observed.schedule_requests.total_overflowed);
    assert!(!observed.scheduler_started.total_overflowed);
    assert!(!observed.wakeups_armed.total_overflowed);
    assert!(!observed.work_dispatched.total_overflowed);
    assert!(!observed.work_started.total_overflowed);
    assert!(!observed.work_completed.total_overflowed);
    assert!(!observed.coalesced.total_overflowed);
    assert!(!observed.unacknowledged.total_overflowed);
    observed
}

#[test]
fn exact_deadline_sleeps_and_registration_replacement_marks_counter_regrowth() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, Method::Start));
    let initial = snapshot(pic, canister_id);
    let first_deadline = initial.next_deadline_ns.expect("scheduled");
    let later = first_deadline + 10_000_000_000;
    update_deadline(pic, canister_id, Method::ScheduleAt, later);
    update_deadline(pic, canister_id, Method::ScheduleAt, later);
    pic.advance_time(Duration::from_secs(2));
    advance_and_tick(pic, Duration::ZERO, 16);
    let sleeping = snapshot(pic, canister_id);
    assert_eq!(sleeping.completed_work, 0);
    assert_eq!(sleeping.scheduler_started.total, 0);
    assert_eq!(sleeping.next_deadline_ns, Some(later));
    assert_eq!(sleeping.wakeups_armed.total, 2);
    assert_eq!(sleeping.registration_id, initial.registration_id);

    let continuation = later + 10_000_000_000;
    // Last request wins; superseded stop/immediate requests cannot leak into
    // this callback or a later one.
    update_unit(pic, canister_id, Method::StopOnNextWork);
    update_unit(pic, canister_id, Method::ContinueImmediatelyOnNextWork);
    update_deadline(pic, canister_id, Method::ScheduleAtOnNextWork, continuation);
    update_deadline(pic, canister_id, Method::ScheduleAt, first_deadline);
    advance_and_tick(pic, Duration::ZERO, 16);
    let completed = snapshot(pic, canister_id);
    assert_eq!(completed.completed_work, 1);
    assert_eq!(completed.next_deadline_ns, Some(continuation));
    assert_eq!(completed.registration_id, initial.registration_id);
    pic.advance_time(Duration::from_secs(2));
    advance_and_tick(pic, Duration::ZERO, 16);
    assert_eq!(snapshot(pic, canister_id).completed_work, 1);

    update_unit(pic, canister_id, Method::Stop);
    assert_eq!(
        snapshot(pic, canister_id).registration_id,
        completed.registration_id
    );
    update_unit(pic, canister_id, Method::ReplaceRegistration);
    let replaced = snapshot(pic, canister_id);
    assert_eq!(replaced.work_completed.total, 0);
    assert_ne!(replaced.registration_id, completed.registration_id);
    let old_id = completed.registration_id.expect("old id");
    let new_id = replaced.registration_id.expect("new id");
    assert_eq!((old_id.0, old_id.1), (new_id.0, new_id.1));
    assert!(new_id.2 > old_id.2);
    for _ in 0..2 {
        pic.advance_time(Duration::from_secs(1));
        advance_and_tick(pic, Duration::ZERO, 16);
    }
    let regrown = snapshot(pic, canister_id);
    assert!(regrown.work_completed.total > completed.work_completed.total);
    assert!(regrown.scheduler_instruction_total > completed.scheduler_instruction_total);
    assert_eq!(regrown.registration_id, replaced.registration_id);
}

fn update_deadline(pic: &PocketIc, canister_id: ExternalPrincipal, method: Method, deadline: u64) {
    update_wire::<(), _>(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        method,
        (deadline,),
    )
    .expect("probe update succeeds");
}

fn ordinary_observation(pic: &PocketIc, canister_id: ExternalPrincipal) -> OrdinaryObservation {
    let observed: OrdinaryObservation = query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::OrdinaryObservation,
        (),
    )
    .expect("probe query succeeds");
    assert!(!observed.work_started.total_overflowed);
    assert!(!observed.work_completed.total_overflowed);
    assert!(!observed.unacknowledged.total_overflowed);
    assert!(!observed.cancellations.total_overflowed);
    observed
}

#[test]
fn once_frozen_await_failure_retires_without_replay() {
    assert_frozen_await_failure_retires(OrdinaryPolicy::Once);
}

#[test]
fn after_completion_frozen_await_failure_retires_without_replay() {
    assert_frozen_await_failure_retires(OrdinaryPolicy::AfterCompletion);
}

fn assert_frozen_await_failure_retires(policy: OrdinaryPolicy) {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    let started_at = pic.get_time().as_nanos_since_unix_epoch();
    let status = pic.canister_status(canister_id, None).unwrap();
    let balance = pic.cycle_balance(canister_id);
    let idle_per_day: u128 = status.idle_cycles_burned_per_day.0.try_into().unwrap();
    assert!(idle_per_day > 0);
    let freezing_seconds = u64::try_from((balance * 4 * 86_400).div_ceil(idle_per_day)).unwrap();
    let freezing_reserve = idle_per_day * u128::from(freezing_seconds) / 86_400;
    update_wire::<(), _>(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::StartOrdinary,
        (policy, ProbeLifetime::Retained),
    )
    .unwrap();
    let suspended = wait_for_probe(
        pic,
        32,
        |pic| ordinary_observation(pic, canister_id),
        |s| s.running && s.waiting && s.gate_replies > 0,
    );
    assert!(suspended.registration_id.is_some());
    assert_eq!(suspended.work_started.total, 1);
    assert_eq!(suspended.completed, 0);
    assert_eq!(suspended.next_deadline_ns, None);
    assert_eq!(suspended.gate_error, None);

    freeze_for_five_minutes(pic, canister_id, freezing_seconds, freezing_reserve);
    pic.add_cycles(canister_id, balance * 16);
    // Query before another replica round: funding cannot hide a completion,
    // abandoned attempt or ownership change that already happened while frozen.
    let thawed = ordinary_observation(pic, canister_id);
    assert_eq!(thawed.registration_id, suspended.registration_id);
    assert!(thawed.declared && thawed.invariant_failure, "{thawed:?}");
    assert!(!thawed.running && !thawed.waiting && !thawed.armed && !thawed.abandoned);
    assert_eq!(thawed.work_started.total, 1);
    // A pre-funded response can resume while frozen. This probe's next call
    // fails for lack of liquid cycles; its callback returns InvariantFailure.
    // That is a normally completed failed attempt, not delivery abandonment.
    assert_eq!(thawed.work_completed.total, 1);
    assert_eq!(thawed.completed, 0);
    assert_eq!(thawed.completed_at_ns, None);
    assert_eq!(thawed.next_deadline_ns, None);
    assert_eq!(thawed.unacknowledged.total, 0);
    assert!(
        thawed.gate_error.as_ref().is_some_and(
            |error| error.starts_with("insufficient liquid cycles balance, available: 0,")
        ),
        "{thawed:?}"
    );
    assert!(update_bool(
        pic,
        canister_id,
        Method::ExpiredOrdinaryContextRejected
    ));
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 16);
    let settled = ordinary_observation(pic, canister_id);
    assert_eq!(settled.registration_id, suspended.registration_id);
    assert!(settled.invariant_failure && !settled.running && !settled.armed);
    assert_eq!(
        settled.work_started.total, 1,
        "funding must not replay accepted work"
    );
    assert_eq!(settled.work_completed.total, 1);
    assert_eq!(settled.completed, 0);
    assert_eq!(settled.unacknowledged.total, 0);

    // The application explicitly schedules a fresh attempt after funding and
    // permits its awaited operation to succeed. No automatic retry is claimed.
    update_unit(pic, canister_id, Method::ReleaseOrdinaryWork);
    update_deadline(
        pic,
        canister_id,
        Method::ReconcileOrdinaryAt,
        pic.get_time().as_nanos_since_unix_epoch(),
    );
    let completed = wait_for_probe(
        pic,
        32,
        |pic| ordinary_observation(pic, canister_id),
        |s| s.completed == 1,
    );
    assert_eq!(completed.registration_id, suspended.registration_id);
    assert!(!completed.running && !completed.waiting);
    assert!(!completed.invariant_failure && !completed.abandoned);
    assert_eq!(completed.work_started.total, 2);
    assert_eq!(completed.work_completed.total, 2);
    assert_eq!(completed.unacknowledged.total, 0);
    // This existing diagnostic keeps the most recent gate error across attempts.
    assert_eq!(completed.gate_error, thawed.gate_error);
    assert!(completed.gate_replies > suspended.gate_replies);
    if policy == OrdinaryPolicy::Once {
        assert!(!completed.armed);
        assert_eq!(completed.next_deadline_ns, None);
        advance_and_tick(pic, Duration::from_secs(5), 16);
        let settled = ordinary_observation(pic, canister_id);
        assert_eq!(settled.completed, 1);
        assert_eq!(settled.work_started.total, 2);
        assert_eq!(settled.work_completed.total, 2);
    } else {
        let completed_at = completed.completed_at_ns.unwrap();
        assert!(completed_at >= started_at + 300_000_000_000);
        assert_eq!(
            completed.next_deadline_ns,
            Some(completed_at + 1_000_000_000)
        );
        advance_and_tick(pic, Duration::ZERO, 16);
        assert_eq!(ordinary_observation(pic, canister_id).work_started.total, 2);
        pic.advance_time(Duration::from_secs(1));
        let recurring = wait_for_probe(
            pic,
            32,
            |pic| ordinary_observation(pic, canister_id),
            |s| s.completed == 2,
        );
        assert_eq!(recurring.registration_id, suspended.registration_id);
        assert_eq!(recurring.work_started.total, 3);
        assert_eq!(recurring.work_completed.total, 3);
        assert!(!recurring.running && !recurring.waiting);
        assert_eq!(recurring.unacknowledged.total, 0);
        assert_eq!(recurring.gate_error, thawed.gate_error);
        assert_eq!(
            recurring.next_deadline_ns,
            Some(recurring.completed_at_ns.unwrap() + 1_000_000_000)
        );
    }
    let after = pic.canister_status(canister_id, None).unwrap();
    assert_eq!(after.module_hash, status.module_hash);
    assert_eq!(after.settings.freezing_threshold, freezing_seconds);
}

// This checks the real provider and message boundary for the two ordinary policies.
#[test]
fn absent_ordinary_demand_is_idle_and_explicit_cancellation_stays_distinct() {
    for policy in OrdinaryPolicy::ALL {
        let owned = fresh_pocket_ic();
        let pic = owned.instance();
        let canister_id = install_probe(pic, probe_wasm());
        update_wire::<(), _>(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::StartOrdinary,
            (policy, ProbeLifetime::Retained),
        )
        .expect("register retained ordinary work");
        let running = wait_for_probe(
            pic,
            32,
            |pic| ordinary_observation(pic, canister_id),
            |state| state.running && state.waiting && state.gate_replies > 0,
        );
        update_unit(pic, canister_id, Method::ReconcileOrdinaryInactive);
        assert!(
            ordinary_observation(pic, canister_id).running,
            "demand removal must not interrupt accepted work"
        );
        update_unit(pic, canister_id, Method::ReleaseOrdinaryWork);
        let stopped = wait_for_probe(
            pic,
            32,
            |pic| ordinary_observation(pic, canister_id),
            |state| state.completed == 1 && !state.running,
        );
        assert!(
            stopped.declared && stopped.stopped && !stopped.cancelled && !stopped.armed,
            "{stopped:?}"
        );
        assert_eq!(stopped.registration_id, running.registration_id);
        assert_eq!(stopped.cancellations.total, 0);
        assert_eq!(stopped.work_completed.total, 1);
        assert!(update_bool(
            pic,
            canister_id,
            Method::ExpiredOrdinaryContextRejected
        ));
        qualify_scheduled_demand_removal(pic, canister_id, running.registration_id);
        update_deadline(
            pic,
            canister_id,
            Method::ReconcileOrdinaryAt,
            pic.get_time().as_nanos_since_unix_epoch(),
        );
        let rearmed = wait_for_probe(
            pic,
            32,
            |pic| ordinary_observation(pic, canister_id),
            |state| state.completed == 2,
        );
        assert_eq!(rearmed.registration_id, running.registration_id);
        assert_eq!(rearmed.work_started.total, 2);
        assert_eq!(rearmed.work_completed.total, 2);
        assert_eq!(rearmed.cancellations.total, 1);
    }
}

fn qualify_scheduled_demand_removal(
    pic: &PocketIc,
    canister_id: ExternalPrincipal,
    identity: Option<(u64, u64, u64)>,
) {
    let deadline = pic.get_time().as_nanos_since_unix_epoch() + 60_000_000_000;
    update_deadline(pic, canister_id, Method::ReconcileOrdinaryAt, deadline);
    let armed = ordinary_observation(pic, canister_id);
    assert!(armed.armed);
    assert_eq!(armed.next_deadline_ns, Some(deadline));
    update_unit(pic, canister_id, Method::ReconcileOrdinaryInactive);
    let idle = ordinary_observation(pic, canister_id);
    assert!(idle.stopped && !idle.cancelled && !idle.armed);
    assert_eq!(idle.next_deadline_ns, None);
    assert_eq!(idle.cancellations.total, 0);
    assert_eq!(idle.registration_id, identity);
    update_deadline(pic, canister_id, Method::ReconcileOrdinaryAt, deadline);
    update_unit(pic, canister_id, Method::CancelOrdinary);
    let cancelled = ordinary_observation(pic, canister_id);
    assert!(cancelled.cancelled && !cancelled.stopped && !cancelled.armed);
    assert_eq!(cancelled.cancellations.total, 1);
    update_unit(pic, canister_id, Method::ReconcileOrdinaryInactive);
    let still_cancelled = ordinary_observation(pic, canister_id);
    assert!(still_cancelled.cancelled && !still_cancelled.stopped);
    assert_eq!(still_cancelled.cancellations.total, 1);
    assert_eq!(still_cancelled.registration_id, identity);
    assert_eq!(still_cancelled.work_started.total, 1);
}

#[test]
fn ordinary_await_allows_ingress_and_other_timers_before_completion() {
    for policy in OrdinaryPolicy::ALL {
        for command in ["cancel", "reconcile", "unregister", "recur"] {
            if command == "recur" && policy == OrdinaryPolicy::Once {
                continue;
            }
            let owned = fresh_pocket_ic();
            let pic = owned.instance();
            let canister_id = install_probe(pic, probe_wasm());
            let lifetime = if command == "cancel" {
                ProbeLifetime::RemoveWhenStopped
            } else {
                ProbeLifetime::Retained
            };
            update_wire::<(), _>(
                pic,
                canister_id,
                ExternalPrincipal::anonymous(),
                Method::StartOrdinary,
                (policy, lifetime),
            )
            .expect("probe update succeeds");
            let suspended = wait_for_probe(
                pic,
                32,
                |pic| ordinary_observation(pic, canister_id),
                |current| current.running && current.waiting && current.gate_replies > 0,
            );
            assert_eq!(suspended.gate_error, None);
            assert!(update_bool(pic, canister_id, Method::Start));
            pic.advance_time(Duration::from_secs(2));
            advance_and_tick(pic, Duration::ZERO, 16);
            assert_eq!(
                snapshot(pic, canister_id).completed_work,
                1,
                "other timer must progress while ordinary work awaits"
            );
            let waiting = ordinary_observation(pic, canister_id);
            assert!(waiting.running);
            assert!(waiting.waiting);
            assert_eq!(waiting.gate_error, None);
            assert_eq!(waiting.completed, 0);
            assert_eq!(waiting.work_completed.total, 0);
            let exact_deadline =
                snapshot(pic, canister_id).next_deadline_ns.unwrap() + 60_000_000_000;
            match command {
                "cancel" => update_unit(pic, canister_id, Method::CancelOrdinary),
                "reconcile" => update_deadline(
                    pic,
                    canister_id,
                    Method::ReconcileOrdinaryAt,
                    exact_deadline,
                ),
                "unregister" => update_unit(pic, canister_id, Method::UnregisterOrdinary),
                "recur" => {}
                _ => unreachable!("fixed command matrix"),
            }
            assert!(
                ordinary_observation(pic, canister_id).running,
                "control does not interrupt consumer work"
            );
            update_unit(pic, canister_id, Method::ReleaseOrdinaryWork);
            advance_and_tick(pic, Duration::ZERO, 16);
            let completed = ordinary_observation(pic, canister_id);
            assert_eq!(completed.completed, 1);
            assert!(!completed.running);
            assert!(!completed.waiting);
            assert_eq!(completed.gate_error, None);
            assert!(completed.gate_replies > suspended.gate_replies);
            if matches!(command, "cancel" | "unregister") {
                assert!(!completed.declared);
                assert_eq!(completed.next_deadline_ns, None);
            } else {
                assert!(completed.declared);
                assert_eq!(completed.work_completed.total, 1);
                let expected = if command == "reconcile" {
                    exact_deadline
                } else {
                    completed.completed_at_ns.unwrap() + 1_000_000_000
                };
                assert_eq!(completed.next_deadline_ns, Some(expected));
            }
        }
    }
}

fn trapping_ordinary_probe(
    policy: OrdinaryPolicy,
    lifetime: ProbeLifetime,
    before_await: bool,
) -> (PocketIcInstance<PocketIc>, ExternalPrincipal) {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    update_wire::<(), _>(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::TrapOrdinaryWork,
        (before_await,),
    )
    .expect("probe update succeeds");
    update_wire::<(), _>(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::StartOrdinary,
        (policy, lifetime),
    )
    .expect("probe update succeeds");
    (owned, canister_id)
}

fn assert_ordinary_abandonment(observed: &OrdinaryObservation, removed: bool) {
    assert!(
        !observed.running,
        "discarded futures must release execution authority: {observed:?}"
    );
    assert!(!observed.armed);
    assert_eq!(observed.declared, !removed);
    assert_eq!(observed.abandoned, !removed);
    assert_eq!(observed.next_deadline_ns, None);
    assert_eq!(observed.completed, 0);
    assert_eq!(observed.work_completed.total, 0);
    assert_eq!(observed.instruction_samples, 0);
    assert_eq!(observed.unacknowledged.total, u128::from(!removed));
    assert_eq!(observed.captures_dropped, u64::from(removed));
    assert!(observed.capture_registry_accessible);
    assert_eq!(observed.inventory_len, u64::from(!removed));
}

#[test]
fn dispatch_response_trap_cleanup_preserves_pending_work_for_retry() {
    for policy in OrdinaryPolicy::ALL {
        let owned = fresh_pocket_ic();
        let pic = owned.instance();
        let canister_id = install_probe(pic, probe_wasm());
        update_wire::<(), _>(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::TrapOrdinaryCaptureDrop,
            (true,),
        )
        .expect("enable destructor trap");
        update_wire::<(), _>(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::TrapOrdinaryWork,
            (true,),
        )
        .expect("enable pre-await trap");
        update_wire::<(), _>(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::StartOrdinary,
            (policy, ProbeLifetime::RemoveWhenStopped),
        )
        .expect("start transient work");
        advance_and_tick(pic, Duration::ZERO, 32);
        let pending = ordinary_observation(pic, canister_id);
        assert!(pending.declared && pending.armed);
        assert!(!pending.running);
        assert_eq!(pending.captures_dropped, 0);
        assert_eq!(pending.completed, 0);

        update_wire::<(), _>(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::TrapOrdinaryCaptureDrop,
            (false,),
        )
        .expect("disable destructor trap");
        advance_and_tick(pic, Duration::ZERO, 8);
        assert!(ordinary_observation(pic, canister_id).declared);
        pic.advance_time(Duration::from_secs(1));
        advance_and_tick(pic, Duration::ZERO, 32);
        assert_ordinary_abandonment(&ordinary_observation(pic, canister_id), true);

        // Confirm the backend still dispatches independent work after cleanup.
        assert!(update_bool(pic, canister_id, Method::Start));
        pic.advance_time(Duration::from_secs(2));
        advance_and_tick(pic, Duration::ZERO, 16);
        assert_eq!(snapshot(pic, canister_id).completed_work, 1);
    }
}

#[test]
fn ordinary_trap_before_await_retires_the_rolled_back_scheduled_delivery() {
    for policy in OrdinaryPolicy::ALL {
        for lifetime in ProbeLifetime::ALL {
            let (owned, canister_id) = trapping_ordinary_probe(policy, lifetime, true);
            let pic = owned.instance();
            advance_and_tick(pic, Duration::ZERO, 32);
            let observed = ordinary_observation(pic, canister_id);
            assert_ordinary_abandonment(&observed, lifetime == ProbeLifetime::RemoveWhenStopped);
            assert_eq!(observed.gate_replies, 0);
            assert!(update_bool(pic, canister_id, Method::Start));
            pic.advance_time(Duration::from_secs(2));
            advance_and_tick(pic, Duration::ZERO, 16);
            assert_eq!(snapshot(pic, canister_id).completed_work, 1);
            if lifetime == ProbeLifetime::Retained {
                update_unit(pic, canister_id, Method::UnregisterOrdinary);
                assert_eq!(ordinary_observation(pic, canister_id).captures_dropped, 1);
            }
        }
    }
}

#[test]
fn ordinary_continuation_trap_retires_running_work_and_releases_expired_authority() {
    for policy in OrdinaryPolicy::ALL {
        for lifetime in ProbeLifetime::ALL {
            for command in ["none", "cancel", "reconcile", "unregister"] {
                let (owned, canister_id) = trapping_ordinary_probe(policy, lifetime, false);
                let pic = owned.instance();
                wait_for_probe(
                    pic,
                    32,
                    |pic| ordinary_observation(pic, canister_id),
                    |current| current.running && current.waiting && current.gate_replies > 0,
                );
                match command {
                    "none" => {}
                    "cancel" => update_unit(pic, canister_id, Method::CancelOrdinary),
                    "reconcile" => {
                        update_deadline(pic, canister_id, Method::ReconcileOrdinaryAt, u64::MAX)
                    }
                    "unregister" => update_unit(pic, canister_id, Method::UnregisterOrdinary),
                    _ => unreachable!("fixed command matrix"),
                }
                update_unit(pic, canister_id, Method::ReleaseOrdinaryWork);
                advance_and_tick(pic, Duration::ZERO, 32);
                let observed = ordinary_observation(pic, canister_id);
                let removed =
                    lifetime == ProbeLifetime::RemoveWhenStopped || command == "unregister";
                assert_ordinary_abandonment(&observed, removed);
                assert_eq!(observed.gate_error, None);
                assert!(update_bool(
                    pic,
                    canister_id,
                    Method::ExpiredOrdinaryContextRejected
                ));
                update_unit(pic, canister_id, Method::ClearOrdinaryTrap);
                if removed {
                    if command != "unregister" {
                        update_unit(pic, canister_id, Method::DiscardExpiredOrdinaryClaim);
                    }
                    update_wire::<(), _>(
                        pic,
                        canister_id,
                        ExternalPrincipal::anonymous(),
                        Method::StartOrdinary,
                        (policy, ProbeLifetime::Retained),
                    )
                    .expect("probe update succeeds");
                } else {
                    // The original retained capability can rearm explicitly.
                    update_deadline(pic, canister_id, Method::ReconcileOrdinaryAt, 0);
                }
                advance_and_tick(pic, Duration::ZERO, 16);
                let completed = ordinary_observation(pic, canister_id);
                assert_eq!(completed.completed, 1);
                assert_eq!(completed.work_completed.total, 1);
                assert_eq!(completed.instruction_samples, 1);
                assert_eq!(completed.unacknowledged.total, u128::from(!removed));
                update_unit(pic, canister_id, Method::UnregisterOrdinary);
                assert_eq!(
                    ordinary_observation(pic, canister_id).captures_dropped,
                    if removed { 2 } else { 1 }
                );
                assert!(update_bool(pic, canister_id, Method::Start));
                pic.advance_time(Duration::from_secs(2));
                advance_and_tick(pic, Duration::ZERO, 16);
                assert_eq!(snapshot(pic, canister_id).completed_work, 1);
            }
        }
    }
}

fn churn_observation(pic: &PocketIc, canister_id: ExternalPrincipal) -> ChurnObservation {
    query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::ChurnObservation,
        (),
    )
    .expect("probe query succeeds")
}

#[test]
fn provider_replacement_churn_preserves_cancellation_and_reports_memory() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    let before = churn_observation(pic, canister_id);
    update_wire::<(), _>(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::ReplaceDistantDeadlines,
        (32_768_u32,),
    )
    .expect("probe update succeeds");
    let replaced = churn_observation(pic, canister_id);
    assert_eq!(replaced.inventory_len, 1);
    assert!(!replaced.armed);
    assert_eq!(replaced.completed, 0);
    // Churn may reuse existing allocation capacity. Report page extents without
    // requiring memory growth for cancellation and continued work to be correct.
    update_unit(pic, canister_id, Method::StartImmediateChurn);
    let immediate = wait_for_probe(
        pic,
        768,
        |pic| churn_observation(pic, canister_id),
        |current| current.completed == 64,
    );
    assert_eq!(immediate.completed, 64);
    assert_eq!(immediate.inventory_len, 1);
    assert!(!immediate.armed);
    assert!(immediate.wasm_pages >= replaced.wasm_pages);
    pic.advance_time(Duration::from_secs(3_601));
    advance_and_tick(pic, Duration::ZERO, 16);
    let drained = churn_observation(pic, canister_id);
    assert_eq!(
        drained.completed, 64,
        "cancelled deadlines cannot run consumer work"
    );
    assert!(!drained.armed);
    println!(
        "ic_timers_churn initial_pages={} replaced_pages={} immediate_pages={} drained_pages={} registrations={} owned_wakeups=0 replacements=32768 immediate_steps=64",
        before.wasm_pages,
        replaced.wasm_pages,
        immediate.wasm_pages,
        drained.wasm_pages,
        drained.inventory_len
    );
}

#[test]
fn armed_callback_removal_releases_captures_with_registry_access() {
    for policy in ProbePolicy::ALL {
        let owned = fresh_pocket_ic();
        let pic = owned.instance();
        let canister_id = install_probe(pic, probe_wasm());
        update_wire::<(), _>(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::RemoveArmedCallback,
            (policy,),
        )
        .expect("probe update succeeds");
        let removed = ordinary_observation(pic, canister_id);
        assert!(!removed.declared);
        assert!(!removed.armed);
        assert_eq!(removed.captures_dropped, 1);
        assert!(removed.capture_registry_accessible);
        pic.advance_time(Duration::from_secs(2));
        advance_and_tick(pic, Duration::ZERO, 8);
        let after_deadline = ordinary_observation(pic, canister_id);
        assert_eq!(after_deadline.completed, 0);
        assert_eq!(after_deadline.captures_dropped, 1);
        assert_eq!(after_deadline.inventory_len, 0);
    }
}

#[test]
fn rejected_initial_schedules_preserve_ic_inventory_and_release_captures() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    for policy in ProbePolicy::ALL {
        for lifetime in ProbeLifetime::ALL {
            let result: (bool, u64, bool) = update_wire(
                pic,
                canister_id,
                ExternalPrincipal::anonymous(),
                Method::RejectInitialSchedule,
                (policy, lifetime),
            )
            .expect("initial schedule rejection observation");
            assert_eq!(result, (true, 1, true));
        }
    }
    pic.advance_time(Duration::from_secs(2));
    advance_and_tick(pic, Duration::ZERO, 8);
    let observation = ordinary_observation(pic, canister_id);
    assert_eq!(observation.inventory_len, 0);
    assert_eq!(observation.completed, 0);
    assert_eq!(observation.captures_dropped, 6);
}

#[test]
fn rejected_initial_reconciliation_preserves_ic_inventory_and_releases_captures() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    // A positive time makes now + u64::MAX overflow; never move IC time backwards.
    pic.advance_time(Duration::from_secs(1));
    let canister_id = install_probe(pic, probe_wasm());
    for policy in ProbePolicy::ALL {
        let result: (bool, bool, u64, bool) = update_wire(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::RejectInitialReconciliation,
            (policy,),
        )
        .expect("initial reconciliation rejection observation");
        assert_eq!(result, (true, true, 1, true));
    }
    pic.advance_time(Duration::from_secs(2));
    advance_and_tick(pic, Duration::ZERO, 8);
    let observation = ordinary_observation(pic, canister_id);
    assert_eq!(observation.inventory_len, 0);
    assert_eq!(observation.completed, 0);
    assert_eq!(observation.captures_dropped, 3);
}

#[test]
fn backend_rejects_malformed_self_calls_and_ignores_unknown_deliveries() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, Method::Start));
    let before = snapshot(pic, canister_id);
    for payload in [vec![], vec![0_u8; 7], vec![0_u8; 9]] {
        let accepted: bool = update_wire(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::FixtureDispatchPayload,
            (payload,),
        )
        .unwrap();
        assert!(!accepted);
    }
    for _ in 0..2 {
        let accepted: bool = update_wire(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::FixtureDispatchPayload,
            (0_u64.to_le_bytes().to_vec(),),
        )
        .unwrap();
        assert!(accepted);
    }
    assert_eq!(snapshot(pic, canister_id), before);
}

#[test]
fn accepted_ordinary_work_survives_dispatch_timeout_without_replay() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    let (): () = update_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::StartOrdinary,
        (OrdinaryPolicy::Once, ProbeLifetime::Retained),
    )
    .unwrap();
    let waiting = wait_for_probe(
        pic,
        20,
        |pic| ordinary_observation(pic, canister_id),
        |state| state.waiting,
    );
    assert!(waiting.running);
    // The first pending task identity is 1 in this fresh canister. Acceptance
    // consumed it before the ordinary callback awaited its application gate.
    let accepted: bool = update_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::FixtureDispatchPayload,
        (1_u64.to_le_bytes().to_vec(),),
    )
    .unwrap();
    assert!(accepted);
    pic.advance_time(Duration::from_secs(301));
    advance_and_tick(pic, Duration::ZERO, 20);
    let timed_out = ordinary_observation(pic, canister_id);
    assert!(timed_out.running);
    assert_eq!(timed_out.completed, 0);
    update_unit(pic, canister_id, Method::ReleaseOrdinaryWork);
    let completed = wait_for_probe(
        pic,
        30,
        |pic| ordinary_observation(pic, canister_id),
        |state| state.completed == 1,
    );
    assert!(!completed.running);
    assert_eq!(completed.gate_error, None);
    advance_and_tick(pic, Duration::ZERO, 8);
    assert_eq!(ordinary_observation(pic, canister_id).completed, 1);
}

fn mixed_snapshot(pic: &PocketIc, canister_id: ExternalPrincipal) -> TimerProbeSnapshot {
    query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::StartupTimerProbeSnapshot,
        (),
    )
    .expect("mixed timer snapshot")
}

fn advance_mixed_round(pic: &PocketIc, duration: Duration) {
    advance_and_tick(pic, duration, 2);
}

#[test]
fn shared_watchdog_survives_instruction_exhaustion() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(
        pic,
        canister_id,
        Method::StartupTimerProbeStart
    ));
    let exhaust_at = update_u64(pic, canister_id, Method::StartupTimerProbeArmExhaustion);
    advance_mixed_round(pic, Duration::from_secs(1));
    // Finish deterministic execution slices without advancing IC time.
    advance_and_tick(pic, Duration::ZERO, 24);
    let exhausted = mixed_snapshot(pic, canister_id);
    assert_eq!(exhausted.exhaust_at, Some(exhaust_at));
    assert_eq!(exhausted.watchdog_callbacks, 0);
    assert_eq!(exhausted.application_callbacks, 1);
    advance_mixed_round(pic, Duration::from_secs(1));
    let recovered = mixed_snapshot(pic, canister_id);
    assert_eq!(recovered.watchdog_callbacks, 1);
    assert_eq!(recovered.application_callbacks, 2);
    let resumed_at = recovered.watchdog_times[0];
    assert!(resumed_at > exhaust_at);
    assert!(resumed_at <= exhaust_at.saturating_add(1_000_000_100));
}

#[test]
fn shared_watchdog_survives_trap_and_upgrade() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(
        pic,
        canister_id,
        Method::StartupTimerProbeStart
    ));
    assert!(!update_bool(
        pic,
        canister_id,
        Method::StartupTimerProbeStart
    ));
    let before_external = mixed_snapshot(pic, canister_id);
    let external = pic.update_call(
        canister_id,
        ExternalPrincipal::anonymous(),
        TIMER_EXECUTOR_METHOD,
        0_u64.to_le_bytes().to_vec(),
    );
    assert!(
        external.is_err(),
        "the timer backend must reject external ingress"
    );
    assert_eq!(mixed_snapshot(pic, canister_id), before_external);
    let trap_at = update_u64(pic, canister_id, Method::StartupTimerProbeArmTrap);
    advance_mixed_round(pic, Duration::from_secs(1));
    let trapped = mixed_snapshot(pic, canister_id);
    assert_eq!(trapped.trap_at, Some(trap_at));
    assert_eq!(trapped.watchdog_callbacks, 0);
    assert_eq!(trapped.application_callbacks, 1);
    advance_mixed_round(pic, Duration::from_secs(1));
    let recovered = mixed_snapshot(pic, canister_id);
    assert_eq!(recovered.watchdog_callbacks, 1);
    assert_eq!(recovered.application_callbacks, 2);
    let resumed_at = recovered.watchdog_times[0];
    assert!(resumed_at > trap_at);
    assert!(resumed_at <= trap_at.saturating_add(1_000_000_100));
    assert!(recovered.application_times[0] >= trap_at);
    assert!(recovered.application_times[0] <= trap_at.saturating_add(100));
    assert!(recovered.application_times[1] > recovered.application_times[0]);
    assert!(
        recovered.application_times[1]
            <= recovered.application_times[0].saturating_add(1_000_000_100)
    );
    pic.upgrade_canister(canister_id, probe_wasm(), Vec::new(), None)
        .expect("upgrade without durable mixed-timer intent");
    let upgraded = mixed_snapshot(pic, canister_id);
    assert!(!upgraded.watchdog_registered);
    assert!(!upgraded.application_registered);
    assert_eq!(upgraded.watchdog_callbacks, 0);
    assert_eq!(upgraded.application_callbacks, 0);
    assert!(update_bool(
        pic,
        canister_id,
        Method::StartupTimerProbeStart
    ));
    advance_mixed_round(pic, Duration::from_secs(1));
    let reregistered = mixed_snapshot(pic, canister_id);
    assert_eq!(reregistered.watchdog_callbacks, 1);
    assert_eq!(reregistered.application_callbacks, 1);
    update_unit(pic, canister_id, Method::StartupTimerProbeStop);
    let stopped = mixed_snapshot(pic, canister_id);
    assert!(!stopped.watchdog_registered);
    assert!(!stopped.application_registered);
}

#[test]
fn overdue_shared_timers_coalesce_to_bounded_work() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(
        pic,
        canister_id,
        Method::StartupTimerProbeStart
    ));
    advance_mixed_round(pic, Duration::from_secs(300));
    let overdue = mixed_snapshot(pic, canister_id);
    let delivered = overdue
        .watchdog_callbacks
        .saturating_add(overdue.application_callbacks);
    assert!(delivered > 0);
    assert!(
        delivered <= 2,
        "one overdue round coalesces to one callback per timer"
    );
    assert!(overdue.watchdog_callbacks > 0);
    assert!(overdue.application_callbacks > 0);
}

fn dispatch_batch_observation(
    pic: &PocketIc,
    canister_id: ExternalPrincipal,
) -> DispatchBatchObservation {
    query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::DispatchBatchObservation,
        (),
    )
    .expect("batch observation")
}

#[test]
fn maximum_capacity_dispatch_preserves_every_survivor_under_pressure_and_await() {
    let owned = fresh_pocket_ic();
    let pic = owned.instance();
    let canister_id = install_probe(pic, probe_wasm());
    update_unit(pic, canister_id, Method::StartDispatchBatch);
    advance_and_tick(pic, Duration::ZERO, 8);
    let (failures, replies): (u64, u64) = query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::PressureObservation,
        (),
    )
    .expect("batch pressure observation");
    assert!(
        failures > 0 && replies >= 500,
        "real queue pressure must be established"
    );
    let blocked = dispatch_batch_observation(pic, canister_id);
    assert_eq!(blocked.inventory_len, 64);
    assert_eq!(blocked.starts, vec![0; 65]);
    assert_eq!(blocked.completions, vec![0; 65]);

    // Both original callbacks are still pending, so neither may run afterward.
    update_unit(pic, canister_id, Method::ChangePendingDispatchBatch);
    update_unit(pic, canister_id, Method::ReleasePressure);
    advance_and_tick(pic, Duration::ZERO, 16);
    assert_eq!(
        dispatch_batch_observation(pic, canister_id).starts,
        vec![0; 65]
    );
    pic.advance_time(Duration::from_secs(1));
    let expected: Vec<u64> = (0..65).map(|index| u64::from(index >= 2)).collect();
    let waiting = wait_for_probe(
        pic,
        64,
        |pic| dispatch_batch_observation(pic, canister_id),
        |state| state.active == 63,
    );
    assert_eq!(waiting.starts, expected);
    assert_eq!(waiting.completions, vec![0; 65]);
    assert_eq!(waiting.inventory_len, 64);
    assert_eq!(waiting.peak_active, 63);
    advance_and_tick(pic, Duration::from_secs(10), 16);
    assert_eq!(dispatch_batch_observation(pic, canister_id), waiting);

    // Rearm the retained cancelled claim while every survivor is still awaiting.
    update_unit(pic, canister_id, Method::RearmCancelledDispatchBatchTimer);
    let mut full_expected = expected;
    full_expected[0] = 1;
    let full = wait_for_probe(
        pic,
        64,
        |pic| dispatch_batch_observation(pic, canister_id),
        |state| state.active == 64,
    );
    assert_eq!(full.starts, full_expected);
    assert_eq!(full.completions, vec![0; 65]);
    assert_eq!(full.inventory_len, 64);
    assert_eq!(full.peak_active, 64);

    update_unit(pic, canister_id, Method::ReleaseDispatchBatch);
    let finished = wait_for_probe(
        pic,
        64,
        |pic| dispatch_batch_observation(pic, canister_id),
        |state| state.completions == full_expected,
    );
    assert_eq!(finished.starts, full_expected);
    assert_eq!(finished.active, 0);
    advance_and_tick(pic, Duration::from_secs(2), 16);
    assert_eq!(dispatch_batch_observation(pic, canister_id), finished);

    // The cancelled retained claim and production dispatch remain usable at capacity.
    update_unit(pic, canister_id, Method::RearmCancelledDispatchBatchTimer);
    let mut rearmed_expected = full_expected;
    rearmed_expected[0] = 2;
    let rearmed = wait_for_probe(
        pic,
        64,
        |pic| dispatch_batch_observation(pic, canister_id),
        |state| state.completions == rearmed_expected,
    );
    assert_eq!(rearmed.starts, rearmed_expected);
    assert_eq!(rearmed.active, 0);
    assert_eq!(rearmed.inventory_len, 64);
    advance_and_tick(pic, Duration::from_secs(2), 16);
    assert_eq!(dispatch_batch_observation(pic, canister_id), rearmed);
}

fn scheduling_trace(pic: &PocketIc, canister_id: ExternalPrincipal) -> Vec<u8> {
    query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::SchedulingTrace,
        (),
    )
    .expect("scheduling trace")
}

#[test]
fn deadline_trace_covers_nested_registration_and_self_cancellation() {
    for overdue in [false, true] {
        let owned = fresh_pocket_ic();
        let pic = owned.instance();
        let canister_id = install_probe(pic, probe_wasm());
        update_unit(pic, canister_id, Method::StartSchedulingTrace);
        advance_and_tick(pic, Duration::ZERO, 8);
        assert!(scheduling_trace(pic, canister_id).is_empty());
        if overdue {
            advance_and_tick(pic, Duration::from_secs(4), 16);
            assert_eq!(scheduling_trace(pic, canister_id), [1, 2, 4]);
            advance_and_tick(pic, Duration::from_secs(1), 8);
            assert_eq!(scheduling_trace(pic, canister_id), [1, 2, 4]);
            advance_and_tick(pic, Duration::from_secs(1), 8);
            assert_eq!(scheduling_trace(pic, canister_id), [1, 2, 4, 3]);
        } else {
            for end in 1..=4 {
                advance_and_tick(pic, Duration::from_secs(1), 8);
                assert_eq!(
                    scheduling_trace(pic, canister_id),
                    (1..=end).collect::<Vec<u8>>()
                );
            }
            advance_and_tick(pic, Duration::from_secs(2), 8);
            assert_eq!(scheduling_trace(pic, canister_id), [1, 2, 3, 4]);
        }
        let mut expected = if overdue {
            vec![1, 2, 4, 3]
        } else {
            vec![1, 2, 3, 4]
        };
        for event in [5, 6] {
            advance_and_tick(pic, Duration::from_secs(1), 8);
            expected.push(event);
            assert_eq!(scheduling_trace(pic, canister_id), expected);
        }
        // The periodic callback requested recurrence but cancelled itself first.
        advance_and_tick(pic, Duration::from_secs(20), 32);
        assert_eq!(scheduling_trace(pic, canister_id), expected);
    }
}
