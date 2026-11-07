//! Installed watchdog qualification subjects.

use super::*;

#[tokio::test]
async fn trapped_work_keeps_committed_successor_and_executor_is_private() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();

    assert!(update_bool(pic, canister_id, Method::Start).await);
    assert!(!update_bool(pic, canister_id, Method::Start).await);
    assert!(update_bool(pic, canister_id, Method::StartSecondary).await);
    assert!(!update_bool(pic, canister_id, Method::StartSecondary).await);

    let before_external = snapshot(pic, canister_id).await;
    let external = pic
        .update_call(
            canister_id,
            ExternalPrincipal::anonymous(),
            TIMER_EXECUTOR_METHOD,
            0_u64.to_be_bytes().to_vec(),
        )
        .await;
    assert!(
        matches!(
            external,
            Err(ic_testkit::pocket_ic::ClientError::Canister(_))
        ),
        "provider executor must reject external ingress: {external:?}"
    );
    assert_eq!(snapshot(pic, canister_id).await, before_external);

    let trap_at = update_u64(pic, canister_id, Method::ArmTrap).await;
    advance_and_tick(pic, Duration::from_secs(1), 12)
        .await
        .expect("advance and tick PocketIC");
    let trapped = snapshot(pic, canister_id).await;
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

    advance_and_tick(pic, Duration::from_secs(1), 12)
        .await
        .expect("advance and tick PocketIC");
    let recovered = snapshot(pic, canister_id).await;
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
        .await
        .expect("upgrade should succeed");
    let upgraded = snapshot(pic, canister_id).await;
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

    advance_and_tick(pic, Duration::from_secs(1), 12)
        .await
        .expect("advance and tick PocketIC");
    let after_upgrade = snapshot(pic, canister_id).await;
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

    owned
        .close()
        .await
        .expect("delete PocketIC instance before stopping its server");
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

#[tokio::test]
async fn instruction_exhaustion_leaves_successor_for_later_progress() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::Start).await);

    let exhaust_at = update_u64(pic, canister_id, Method::ArmExhaustion).await;
    advance_and_tick(pic, Duration::from_secs(1), 30)
        .await
        .expect("advance and tick PocketIC");
    let exhausted = snapshot(pic, canister_id).await;
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

    advance_and_tick(pic, Duration::from_secs(1), 16)
        .await
        .expect("advance and tick PocketIC");
    let recovered = snapshot(pic, canister_id).await;
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

    owned
        .close()
        .await
        .expect("delete PocketIC instance before stopping its server");
}

#[tokio::test]
async fn scheduler_abandonment_under_round_pressure_recovers_after_stop() {
    for successor in [false, true] {
        for mode in SchedulerReconcileMode::ALL {
            let (owned, canister_id) = installed_probe().await;
            let pic = owned.instance();
            if successor {
                assert!(update_bool(pic, canister_id, Method::Start).await);
                pic.advance_time(Duration::from_secs(1))
                    .await
                    .expect("advance IC time");
                wait_for_probe(
                    pic,
                    32,
                    async |pic| snapshot(pic, canister_id).await,
                    |s| s.completed_work == 1,
                )
                .await;
            }
            update_wire::<(), _>(
                pic,
                canister_id,
                ExternalPrincipal::anonymous(),
                Method::StartSchedulerPressure,
                (successor,),
            )
            .await
            .unwrap();
            let armed = snapshot(pic, canister_id).await;
            let deadline = armed.next_deadline_ns.unwrap();
            pic.advance_time(Duration::from_nanos(
                deadline
                    .checked_sub(
                        pic.get_time()
                            .await
                            .expect("read IC time")
                            .as_nanos_since_unix_epoch(),
                    )
                    .unwrap(),
            ))
            .await
            .expect("advance IC time");
            let pressure: u64 = wait_for_probe(
                pic,
                8,
                async |pic| {
                    query_wire(
                        pic,
                        canister_id,
                        ExternalPrincipal::anonymous(),
                        Method::SchedulerPressureCompleted,
                        (),
                    )
                    .await
                    .expect("observe committed round pressure")
                },
                |completed| *completed > 0,
            )
            .await;
            let before_stop = snapshot(pic, canister_id).await;
            println!("successor={successor} mode={mode:?} committed_pressure={pressure}");
            assert_eq!(
                before_stop.scheduler_started.total, armed.scheduler_started.total,
                "must reach the queued scheduler boundary"
            );
            pic.stop_canister(canister_id, None)
                .await
                .expect("stop queued scheduler");
            pic.start_canister(canister_id, None)
                .await
                .expect("restart queued scheduler");
            // Time and ticks alone must not be mistaken for explicit owner repair.
            advance_and_tick(pic, Duration::from_secs(2), 16)
                .await
                .expect("advance and tick PocketIC");
            let stopped = snapshot(pic, canister_id).await;
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
            .await
            .unwrap();
            assert!(repaired);
            let recovered = wait_for_probe(
                pic,
                64,
                async |pic| snapshot(pic, canister_id).await,
                |s| s.completed_work == armed.completed_work + 1,
            )
            .await;
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
            advance_and_tick(pic, Duration::ZERO, 16)
                .await
                .expect("advance and tick PocketIC");
            assert_eq!(
                snapshot(pic, canister_id).await.completed_work,
                armed.completed_work + 1
            );

            owned
                .close()
                .await
                .expect("delete PocketIC instance before stopping its server");
        }
    }
}

#[tokio::test]
async fn insufficient_cycles_defer_due_work_until_top_up() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::Start).await);
    let (_burned, call_cost): (u128, u128) = update_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::BurnToBelowTimerCallCost,
        (),
    )
    .await
    .expect("probe multi-result update succeeds");
    assert!(call_cost > 0);

    advance_and_tick(pic, Duration::from_secs(1), 8)
        .await
        .expect("advance and tick PocketIC");
    let deferred = snapshot(pic, canister_id).await;
    assert_eq!(deferred.completed_work, 0);
    assert_eq!(deferred.scheduler_started.total, 0);

    pic.add_cycles(canister_id, INIT_CYCLES)
        .await
        .expect("fund canister cycles");
    advance_and_tick(pic, Duration::ZERO, 4)
        .await
        .expect("advance and tick PocketIC");
    assert_eq!(snapshot(pic, canister_id).await.scheduler_started.total, 0);
    advance_and_tick(pic, Duration::from_secs(1), 16)
        .await
        .expect("advance and tick PocketIC");
    let progressed = snapshot(pic, canister_id).await;
    assert_eq!(progressed.completed_work, 1);
    assert_eq!(progressed.scheduler_started.total, 1);

    owned
        .close()
        .await
        .expect("delete PocketIC instance before stopping its server");
}

#[tokio::test]
async fn frozen_canister_resumes_overdue_timer_after_top_up_only() {
    assert_frozen_watchdog_resumes_after_top_up(false).await;
}

#[tokio::test]
async fn freeze_between_scheduler_and_work_resumes_after_top_up_only() {
    assert_frozen_watchdog_resumes_after_top_up(true).await;
}

async fn assert_frozen_watchdog_resumes_after_top_up(after_scheduler: bool) {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    // Raise the real freezing reserve above the existing balance without burning
    // all cycles or uninstalling the canister. Derive it from this instance's
    // storage charge before making work due: management observation must not
    // silently consume the scheduler/work gap being qualified.
    let status = pic.canister_status(canister_id, None).await.unwrap();
    let balance = pic
        .cycle_balance(canister_id)
        .await
        .expect("read canister cycles");
    let idle_per_day: u128 = status.idle_cycles_burned_per_day.0.try_into().unwrap();
    assert!(idle_per_day > 0);
    let freezing_seconds = u64::try_from((balance * 4 * 86_400).div_ceil(idle_per_day))
        .expect("fixture freezing threshold fits u64");
    let freezing_reserve = idle_per_day * u128::from(freezing_seconds) / 86_400;
    assert!(update_bool(pic, canister_id, Method::Start).await);
    let armed = snapshot(pic, canister_id).await;
    assert_eq!(armed.scheduler_started.total, 0);
    assert_eq!(armed.completed_work, 0);
    let before = if after_scheduler {
        pic.advance_time(Duration::from_secs(1))
            .await
            .expect("advance IC time");
        let dispatched = wait_for_probe(
            pic,
            12,
            async |pic| snapshot(pic, canister_id).await,
            |s| s.work_dispatched.total == 1 && s.work_started.total == 0,
        )
        .await;
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
    freeze_for_five_minutes(pic, canister_id, freezing_seconds, freezing_reserve).await;
    assert!(
        pic.get_time()
            .await
            .expect("read IC time")
            .as_nanos_since_unix_epoch()
            > deadline
    );
    // No ensure/reconcile call, threshold reset, raw timer write or upgrade can
    // repair the wakeup here. A top-up and replica rounds must suffice.
    pic.add_cycles(canister_id, balance * 16)
        .await
        .expect("fund canister cycles");
    // Read-only observation before another round proves the frozen interval
    // left the overdue registration intact and executed no timer work.
    let thawed = snapshot(pic, canister_id).await;
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
        async |pic| snapshot(pic, canister_id).await,
        |s| s.completed_work > 0,
    )
    .await;
    assert!(resumed.registered);
    assert_eq!(resumed.completed_work, 1);
    assert!((1..=u128::from(settled_count)).contains(&resumed.scheduler_started.total));
    assert_eq!(resumed.work_started.total, 1);
    assert_eq!(resumed.work_completed.total, 1);
    assert!(!resumed.post_upgrade_reconstructed);
    advance_and_tick(pic, Duration::ZERO, 16)
        .await
        .expect("advance and tick PocketIC");
    let settled = snapshot(pic, canister_id).await;
    assert!(settled.registered);
    assert_eq!(settled.registration_id, before.registration_id);
    assert_eq!(settled.completed_work, settled_count);
    assert_eq!(settled.scheduler_started.total, u128::from(settled_count));
    assert_eq!(settled.work_dispatched.total, u128::from(settled_count));
    assert_eq!(settled.work_started.total, u128::from(settled_count));
    assert_eq!(settled.work_completed.total, u128::from(settled_count));
    assert_eq!(settled.unacknowledged.total, 0);
    let now = pic
        .get_time()
        .await
        .expect("read IC time")
        .as_nanos_since_unix_epoch();
    assert!(settled.next_deadline_ns.is_some_and(|next| next > now));
    pic.advance_time(Duration::from_secs(1))
        .await
        .expect("advance IC time");
    let recurring = wait_for_probe(
        pic,
        32,
        async |pic| snapshot(pic, canister_id).await,
        |s| s.completed_work > settled_count,
    )
    .await;
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
    let after = pic.canister_status(canister_id, None).await.unwrap();
    assert_eq!(after.module_hash, status.module_hash);
    assert_eq!(after.settings.freezing_threshold, freezing_seconds);

    owned
        .close()
        .await
        .expect("delete PocketIC instance before stopping its server");
}
