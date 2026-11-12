//! Installed lifecycle qualification subjects.

use super::*;

// Scheduling owns this composition proof against its selected Runtime cohort.
// Both transports share the same Wire method, budgets and timer callback.
#[tokio::test]
async fn runtime_composition_preserves_calls_cleanup_and_lifecycle() {
    for prepared in [false, true] {
        let (owned, canister_id) = installed_probe().await;
        let pic = owned.instance();
        assert!(update_bool(pic, canister_id, Method::ReinitializeRuntime).await);
        update_wire::<(), _>(
            pic,
            canister_id,
            Principal::anonymous(),
            Method::UsePreparedCalls,
            (prepared,),
        )
        .await
        .expect("select Runtime call transport");
        start_ordinary(
            pic,
            canister_id,
            OrdinaryPolicy::Once,
            ProbeLifetime::Retained,
        )
        .await;
        let waiting = wait_for_probe(
            pic,
            32,
            async |pic| ordinary_observation(pic, canister_id).await,
            |s| s.running && s.waiting && s.gate_replies > 0,
        )
        .await;
        assert!(update_bool(pic, canister_id, Method::ReinitializeRuntime).await);
        let repeated = ordinary_observation(pic, canister_id).await;
        assert_eq!(repeated.registration_id, waiting.registration_id);
        assert_eq!(repeated.work_started.total, 1);
        assert!(repeated.running && repeated.waiting);
        assert_eq!(repeated.completed, 0);
        assert_eq!(repeated.unacknowledged.total, 0);

        let completed = async |count| {
            wait_for_probe(
                pic,
                32,
                async |pic| ordinary_observation(pic, canister_id).await,
                |s| s.completed == count && !s.running,
            )
            .await
        };
        update_unit(pic, canister_id, Method::ReleaseOrdinaryWork).await;
        assert_eq!(completed(1).await.work_completed.total, 1);

        let deadline = now_ns(pic).await + 60_000_000_000;
        update_deadline(pic, canister_id, Method::ReconcileOrdinaryAt, deadline).await;
        update_unit(pic, canister_id, Method::CancelOrdinary).await;
        advance(pic, Duration::from_secs(61), 8).await;
        let cancelled = ordinary_observation(pic, canister_id).await;
        assert_eq!(
            cancelled.completed, 1,
            "cancelled work must not mutate state"
        );
        assert_eq!(cancelled.work_started.total, 1);
        assert_eq!(cancelled.termination, Some(OrdinaryTermination::Cancelled));
        update_deadline(
            pic,
            canister_id,
            Method::ReconcileOrdinaryAt,
            now_ns(pic).await,
        )
        .await;
        assert_eq!(completed(2).await.work_completed.total, 2);

        update_wire::<(), _>(
            pic,
            canister_id,
            Principal::anonymous(),
            Method::TrapOrdinaryWork,
            (false,),
        )
        .await
        .expect("trap the continuation after a Runtime call");
        update_deadline(
            pic,
            canister_id,
            Method::ReconcileOrdinaryAt,
            now_ns(pic).await,
        )
        .await;
        let abandoned = wait_for_probe(
            pic,
            32,
            async |pic| ordinary_observation(pic, canister_id).await,
            |s| s.termination == Some(OrdinaryTermination::Abandoned) && !s.running,
        )
        .await;
        assert_eq!(abandoned.completed, 2);
        assert_eq!(abandoned.work_completed.total, 2);
        assert_eq!(abandoned.unacknowledged.total, 1);
        assert!(update_bool(pic, canister_id, Method::ExpiredOrdinaryContextRejected).await);
        update_unit(pic, canister_id, Method::ClearOrdinaryTrap).await;
        update_deadline(
            pic,
            canister_id,
            Method::ReconcileOrdinaryAt,
            now_ns(pic).await,
        )
        .await;
        assert_eq!(completed(3).await.work_completed.total, 3);

        pic.upgrade_canister(canister_id, probe_wasm(), Vec::new(), None)
            .await
            .expect("upgrade the composing canister");
        assert!(update_bool(pic, canister_id, Method::ReinitializeRuntime).await);
        update_wire::<(), _>(
            pic,
            canister_id,
            Principal::anonymous(),
            Method::UsePreparedCalls,
            (prepared,),
        )
        .await
        .expect("select transport after upgrade");
        start_ordinary(
            pic,
            canister_id,
            OrdinaryPolicy::Once,
            ProbeLifetime::Retained,
        )
        .await;
        update_unit(pic, canister_id, Method::ReleaseOrdinaryWork).await;
        let upgraded = completed(1).await;
        assert_eq!(upgraded.work_completed.total, 1);
        assert_eq!(upgraded.unacknowledged.total, 0);
        close_instance(owned).await;
    }
}

async fn churn_observation(pic: &PocketIc, canister_id: Principal) -> ChurnObservation {
    query_wire(
        pic,
        canister_id,
        Principal::anonymous(),
        Method::ChurnObservation,
        (),
    )
    .await
    .expect("probe query succeeds")
}

#[tokio::test]
async fn provider_replacement_churn_preserves_cancellation_and_reports_memory() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    let before = churn_observation(pic, canister_id).await;
    update_wire::<(), _>(
        pic,
        canister_id,
        Principal::anonymous(),
        Method::ReplaceDistantDeadlines,
        (32_768_u32,),
    )
    .await
    .expect("probe update succeeds");
    let replaced = churn_observation(pic, canister_id).await;
    assert_eq!(replaced.inventory_len, 1);
    assert!(!replaced.armed);
    assert_eq!(replaced.completed, 0);
    // Churn may reuse existing allocation capacity. Report page extents without
    // requiring memory growth for cancellation and continued work to be correct.
    update_unit(pic, canister_id, Method::StartImmediateChurn).await;
    let immediate = wait_for_probe(
        pic,
        768,
        async |pic| churn_observation(pic, canister_id).await,
        |current| current.completed == 64,
    )
    .await;
    assert_eq!(immediate.completed, 64);
    assert_eq!(immediate.inventory_len, 1);
    assert!(!immediate.armed);
    assert!(immediate.wasm_pages >= replaced.wasm_pages);
    advance(pic, Duration::from_secs(3_601), 16).await;
    let drained = churn_observation(pic, canister_id).await;
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

    close_instance(owned).await;
}

#[tokio::test]
async fn armed_callback_removal_releases_captures_with_registry_access() {
    for policy in ProbePolicy::ALL {
        let (owned, canister_id) = installed_probe().await;
        let pic = owned.instance();
        update_wire::<(), _>(
            pic,
            canister_id,
            Principal::anonymous(),
            Method::RemoveArmedCallback,
            (policy,),
        )
        .await
        .expect("probe update succeeds");
        let removed = ordinary_observation(pic, canister_id).await;
        assert!(!removed.declared);
        assert!(!removed.armed);
        assert_eq!(removed.captures_dropped, 1);
        assert!(removed.capture_registry_accessible);
        advance(pic, Duration::from_secs(2), 8).await;
        let after_deadline = ordinary_observation(pic, canister_id).await;
        assert_eq!(after_deadline.completed, 0);
        assert_eq!(after_deadline.captures_dropped, 1);
        assert_eq!(after_deadline.inventory_len, 0);

        close_instance(owned).await;
    }
}

#[tokio::test]
async fn rejected_initial_schedules_preserve_ic_inventory_and_release_captures() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    for policy in ProbePolicy::ALL {
        for lifetime in ProbeLifetime::ALL {
            let result: (bool, u64, bool) = update_wire(
                pic,
                canister_id,
                Principal::anonymous(),
                Method::RejectInitialSchedule,
                (policy, lifetime),
            )
            .await
            .expect("initial schedule rejection observation");
            assert_eq!(result, (true, 1, true));
        }
    }
    advance(pic, Duration::from_secs(2), 8).await;
    let observation = ordinary_observation(pic, canister_id).await;
    assert_eq!(observation.inventory_len, 0);
    assert_eq!(observation.completed, 0);
    assert_eq!(observation.captures_dropped, 6);

    close_instance(owned).await;
}

#[tokio::test]
async fn rejected_initial_reconciliation_preserves_ic_inventory_and_releases_captures() {
    let owned = fresh_pocket_ic().await;
    let pic = owned.instance();
    // A positive time makes now + u64::MAX overflow; never move IC time backwards.
    pic.advance_time(Duration::from_secs(1)).await;
    let canister_id = install_probe(pic, probe_wasm()).await;
    for policy in ProbePolicy::ALL {
        let result: (bool, bool, u64, bool) = update_wire(
            pic,
            canister_id,
            Principal::anonymous(),
            Method::RejectInitialReconciliation,
            (policy,),
        )
        .await
        .expect("initial reconciliation rejection observation");
        assert_eq!(result, (true, true, 1, true));
    }
    advance(pic, Duration::from_secs(2), 8).await;
    let observation = ordinary_observation(pic, canister_id).await;
    assert_eq!(observation.inventory_len, 0);
    assert_eq!(observation.completed, 0);
    assert_eq!(observation.captures_dropped, 3);

    close_instance(owned).await;
}

// Both faults must isolate the Watchdog without stopping the application timer.
async fn assert_pair_fault_recovers(
    pic: &PocketIc,
    canister_id: Principal,
    fault_at: u64,
    exhaust: bool,
) {
    advance(pic, Duration::from_secs(1), 2).await;
    if exhaust {
        // Finish deterministic execution slices without advancing IC time.
        advance(pic, Duration::ZERO, 24).await;
    }
    let failed = snapshot(pic, canister_id).await;
    assert_eq!(failed.scheduler_started.total, 1);
    assert_eq!(failed.work_dispatched.total, 1);
    assert_eq!(failed.work_completed.total, 0);
    assert_eq!(failed.application_completed.total, 1);
    advance(pic, Duration::from_secs(1), 2).await;
    let recovered = snapshot(pic, canister_id).await;
    assert_eq!(recovered.work_completed.total, 1);
    assert_eq!(recovered.application_completed.total, 2);
    let resumed_at = recovered.completed_at_ns.expect("watchdog completion time");
    assert!(resumed_at > fault_at);
    assert!(resumed_at <= fault_at.saturating_add(1_000_000_100));
    let first_application = failed
        .application_completed_at_ns
        .expect("first application completion");
    let second_application = recovered
        .application_completed_at_ns
        .expect("second application completion");
    assert!(first_application >= fault_at);
    assert!(first_application <= fault_at.saturating_add(100));
    assert!(second_application > first_application);
    assert!(second_application <= first_application.saturating_add(1_000_000_100));
}

#[tokio::test]
async fn shared_watchdog_survives_instruction_exhaustion() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::StartVolatilePair).await);
    let exhaust_at = update_u64(pic, canister_id, Method::ArmExhaustion).await;
    assert_pair_fault_recovers(pic, canister_id, exhaust_at, true).await;

    close_instance(owned).await;
}

#[tokio::test]
async fn shared_watchdog_survives_trap_and_upgrade() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::StartVolatilePair).await);
    assert!(!update_bool(pic, canister_id, Method::StartVolatilePair).await);
    let before_external = snapshot(pic, canister_id).await;
    let external = pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            TIMER_EXECUTOR_METHOD,
            0_u64.to_le_bytes().to_vec(),
        )
        .await;
    assert!(
        external.is_err(),
        "the timer backend must reject external ingress: {external:?}"
    );
    assert_eq!(snapshot(pic, canister_id).await, before_external);
    let trap_at = update_u64(pic, canister_id, Method::ArmTrap).await;
    assert_pair_fault_recovers(pic, canister_id, trap_at, false).await;
    pic.upgrade_canister(canister_id, probe_wasm(), Vec::new(), None)
        .await
        .expect("upgrade without durable mixed-timer intent");
    let upgraded = snapshot(pic, canister_id).await;
    assert!(!upgraded.registered);
    assert!(!upgraded.application_registered);
    assert_eq!(upgraded.work_completed.total, 0);
    assert_eq!(upgraded.application_completed.total, 0);
    assert!(update_bool(pic, canister_id, Method::StartVolatilePair).await);
    advance(pic, Duration::from_secs(1), 2).await;
    let reregistered = snapshot(pic, canister_id).await;
    assert_eq!(reregistered.work_completed.total, 1);
    assert_eq!(reregistered.application_completed.total, 1);
    update_unit(pic, canister_id, Method::StopVolatilePair).await;
    let stopped = snapshot(pic, canister_id).await;
    assert!(!stopped.registered);
    assert!(!stopped.application_registered);

    close_instance(owned).await;
}

#[tokio::test]
async fn overdue_shared_timers_coalesce_to_bounded_work() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::StartVolatilePair).await);
    advance(pic, Duration::from_secs(300), 2).await;
    let overdue = snapshot(pic, canister_id).await;
    let delivered = overdue
        .work_completed
        .total
        .saturating_add(overdue.application_completed.total);
    assert!(delivered > 0);
    assert!(
        delivered <= 2,
        "one overdue round coalesces to one callback per timer"
    );
    assert!(overdue.work_completed.total > 0);
    assert!(overdue.application_completed.total > 0);

    close_instance(owned).await;
}

#[tokio::test]
async fn cached_baseline_restoration_preserves_timer_ownership_after_mutation() {
    use ic_testkit::pic::{CachedPocketIcBaseline, CanisterSnapshotTarget, SnapshotRestoreFunding};

    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    let instance_id = pic.instance_id;
    let baseline_deadline = now_ns(pic).await + 600_000_000_000;
    assert!(update_bool(pic, canister_id, Method::Start).await);
    update_wire::<(), _>(
        pic,
        canister_id,
        Principal::anonymous(),
        Method::ScheduleAt,
        (baseline_deadline,),
    )
    .await
    .unwrap();
    let captured = snapshot(pic, canister_id).await;
    assert_eq!(captured.next_deadline_ns, Some(baseline_deadline));
    assert_eq!(captured.completed_work, 0);
    pic.stop_canister(canister_id, None).await.unwrap();
    let baseline = CachedPocketIcBaseline::capture(
        owned,
        [CanisterSnapshotTarget {
            canister_id,
            sender: None,
        }],
        captured,
    )
    .await
    .expect("capture the stopped timer baseline");
    let pic = baseline.pocket_ic();

    // First exercise the complete unchanged-restore/callback control; then
    // repeat after mutation, without replacing this baseline or its instance.
    for (cycle, mutate) in [(0, false), (1, true), (2, true)] {
        if mutate {
            let mutated_deadline = baseline_deadline + cycle * 60_000_000_000;
            update_wire::<(), _>(
                pic,
                canister_id,
                Principal::anonymous(),
                Method::ScheduleAt,
                (mutated_deadline,),
            )
            .await
            .expect("mutate the live scheduler and system timer");
            assert_eq!(
                snapshot(pic, canister_id).await.next_deadline_ns,
                Some(mutated_deadline)
            );
            assert!(now_ns(pic).await < baseline_deadline);
            pic.stop_canister(canister_id, None).await.unwrap();
        }
        baseline
            .restore(SnapshotRestoreFunding::Preserve)
            .await
            .unwrap();
        pic.start_canister(canister_id, None).await.unwrap();
        assert_eq!(pic.instance_id, instance_id, "reuse the same IC instance");
        assert_eq!(baseline.metadata(), &snapshot(pic, canister_id).await);

        // A real rearm checks restored heap authority against the system timer.
        // Do not reset the timer or rebuild the instance before this operation.
        let deadline = now_ns(pic).await + 1_000_000_000;
        update_wire::<(), _>(
            pic,
            canister_id,
            Principal::anonymous(),
            Method::ScheduleAt,
            (deadline,),
        )
        .await
        .unwrap_or_else(|error| {
            panic!("restore cycle {cycle} (mutated={mutate}) must rearm without a competing-owner trap: {error}")
        });
        set_work_decision(pic, canister_id, WorkDecision::Stop).await;
        advance(pic, Duration::from_secs(1), 16).await;
        let completed = snapshot(pic, canister_id).await;
        assert_eq!(
            completed.registration_id,
            baseline.metadata().registration_id
        );
        assert_eq!(completed.completed_work, 1);
        assert_eq!(completed.scheduler_started.total, 1);
        assert_eq!(completed.work_completed.total, 1);
        assert_eq!(completed.unacknowledged.total, 0);
        assert_eq!(completed.next_deadline_ns, None);
        advance(pic, Duration::from_secs(1), 8).await;
        assert_eq!(snapshot(pic, canister_id).await.completed_work, 1);
    }

    baseline.close().await;
}
