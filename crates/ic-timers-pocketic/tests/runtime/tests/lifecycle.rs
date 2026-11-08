//! Installed lifecycle qualification subjects.

use super::*;

async fn churn_observation(pic: &PocketIc, canister_id: ExternalPrincipal) -> ChurnObservation {
    query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
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
        ExternalPrincipal::anonymous(),
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
            ExternalPrincipal::anonymous(),
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
                ExternalPrincipal::anonymous(),
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
    pic.advance_time(Duration::from_secs(1))
        .await
        .expect("advance IC time");
    let canister_id = install_probe(pic, probe_wasm()).await;
    for policy in ProbePolicy::ALL {
        let result: (bool, bool, u64, bool) = update_wire(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
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

async fn mixed_snapshot(pic: &PocketIc, canister_id: ExternalPrincipal) -> TimerProbeSnapshot {
    query_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::StartupTimerProbeSnapshot,
        (),
    )
    .await
    .expect("mixed timer snapshot")
}

async fn advance_mixed_round(pic: &PocketIc, duration: Duration) {
    advance(pic, duration, 2).await;
}

#[tokio::test]
async fn shared_watchdog_survives_instruction_exhaustion() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::StartupTimerProbeStart).await);
    let exhaust_at = update_u64(pic, canister_id, Method::StartupTimerProbeArmExhaustion).await;
    advance_mixed_round(pic, Duration::from_secs(1)).await;
    // Finish deterministic execution slices without advancing IC time.
    advance(pic, Duration::ZERO, 24).await;
    let exhausted = mixed_snapshot(pic, canister_id).await;
    assert_eq!(exhausted.exhaust_at, Some(exhaust_at));
    assert_eq!(exhausted.watchdog_callbacks, 0);
    assert_eq!(exhausted.application_callbacks, 1);
    advance_mixed_round(pic, Duration::from_secs(1)).await;
    let recovered = mixed_snapshot(pic, canister_id).await;
    assert_eq!(recovered.watchdog_callbacks, 1);
    assert_eq!(recovered.application_callbacks, 2);
    let resumed_at = recovered.watchdog_times[0];
    assert!(resumed_at > exhaust_at);
    assert!(resumed_at <= exhaust_at.saturating_add(1_000_000_100));

    close_instance(owned).await;
}

#[tokio::test]
async fn shared_watchdog_survives_trap_and_upgrade() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::StartupTimerProbeStart).await);
    assert!(!update_bool(pic, canister_id, Method::StartupTimerProbeStart).await);
    let before_external = mixed_snapshot(pic, canister_id).await;
    let external = pic
        .update_call(
            canister_id,
            ExternalPrincipal::anonymous(),
            TIMER_EXECUTOR_METHOD,
            0_u64.to_le_bytes().to_vec(),
        )
        .await;
    assert!(
        matches!(
            external,
            Err(ic_testkit::pocket_ic::ClientError::Canister(_))
        ),
        "the timer backend must reject external ingress: {external:?}"
    );
    assert_eq!(mixed_snapshot(pic, canister_id).await, before_external);
    let trap_at = update_u64(pic, canister_id, Method::StartupTimerProbeArmTrap).await;
    advance_mixed_round(pic, Duration::from_secs(1)).await;
    let trapped = mixed_snapshot(pic, canister_id).await;
    assert_eq!(trapped.trap_at, Some(trap_at));
    assert_eq!(trapped.watchdog_callbacks, 0);
    assert_eq!(trapped.application_callbacks, 1);
    advance_mixed_round(pic, Duration::from_secs(1)).await;
    let recovered = mixed_snapshot(pic, canister_id).await;
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
        .await
        .expect("upgrade without durable mixed-timer intent");
    let upgraded = mixed_snapshot(pic, canister_id).await;
    assert!(!upgraded.watchdog_registered);
    assert!(!upgraded.application_registered);
    assert_eq!(upgraded.watchdog_callbacks, 0);
    assert_eq!(upgraded.application_callbacks, 0);
    assert!(update_bool(pic, canister_id, Method::StartupTimerProbeStart).await);
    advance_mixed_round(pic, Duration::from_secs(1)).await;
    let reregistered = mixed_snapshot(pic, canister_id).await;
    assert_eq!(reregistered.watchdog_callbacks, 1);
    assert_eq!(reregistered.application_callbacks, 1);
    update_unit(pic, canister_id, Method::StartupTimerProbeStop).await;
    let stopped = mixed_snapshot(pic, canister_id).await;
    assert!(!stopped.watchdog_registered);
    assert!(!stopped.application_registered);

    close_instance(owned).await;
}

#[tokio::test]
async fn overdue_shared_timers_coalesce_to_bounded_work() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::StartupTimerProbeStart).await);
    advance_mixed_round(pic, Duration::from_secs(300)).await;
    let overdue = mixed_snapshot(pic, canister_id).await;
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

    close_instance(owned).await;
}
