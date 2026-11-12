//! Installed scheduling qualification subjects.

use super::*;

#[tokio::test]
async fn self_call_queue_pressure_defers_timer_submission_without_losing_work() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    update_unit(pic, canister_id, Method::PressureAndStart).await;
    advance(pic, Duration::ZERO, 8).await;
    let (failed, replies): (u64, u64) = query_wire(
        pic,
        canister_id,
        Principal::anonymous(),
        Method::PressureObservation,
        (),
    )
    .await
    .expect("pressure counters");
    println!("ic_timers_queue_pressure failed={failed} replies={replies}");
    assert!(failed > 0);
    assert!(replies >= 500);
    assert!(pic.cycle_balance(canister_id).await > INIT_CYCLES / 2);
    assert_eq!(snapshot(pic, canister_id).await.scheduler_started.total, 0);
    update_unit(pic, canister_id, Method::ReleasePressure).await;
    advance(pic, Duration::ZERO, 16).await;
    assert_eq!(snapshot(pic, canister_id).await.scheduler_started.total, 0);
    advance(pic, Duration::from_secs(1), 16).await;
    let completed = snapshot(pic, canister_id).await;
    assert_eq!(completed.scheduler_started.total, 1);
    assert_eq!(completed.completed_work, 1);
    advance(pic, Duration::ZERO, 8).await;
    assert_eq!(snapshot(pic, canister_id).await.completed_work, 1);

    close_instance(owned).await;
}

#[tokio::test]
async fn deferred_submission_cancellation_and_replacement_do_not_revive_old_work() {
    for replace in [false, true] {
        let (owned, canister_id) = installed_probe().await;
        let pic = owned.instance();
        update_unit(pic, canister_id, Method::PressureAndStart).await;
        advance(pic, Duration::ZERO, 8).await;
        let original = snapshot(pic, canister_id).await;
        assert_eq!(original.scheduler_started.total, 0);
        assert!(pic.cycle_balance(canister_id).await > INIT_CYCLES / 2);
        update_unit(
            pic,
            canister_id,
            if replace {
                Method::ReplaceRegistration
            } else {
                Method::Stop
            },
        )
        .await;
        let changed = snapshot(pic, canister_id).await;
        if replace {
            assert_ne!(changed.registration_id, original.registration_id);
        } else {
            assert_eq!(changed.next_deadline_ns, None);
        }
        update_unit(pic, canister_id, Method::ReleasePressure).await;
        advance(pic, Duration::ZERO, 16).await;
        // A fresh canister's first backend identity belongs to the old scheduler.
        // Even an authenticated late delivery must not revive it after removal.
        let stale_delivery: bool = update_wire(
            pic,
            canister_id,
            Principal::anonymous(),
            Method::FixtureDispatchPayload,
            (1_u64.to_le_bytes().to_vec(),),
        )
        .await
        .expect("stale self-call result");
        assert!(stale_delivery);
        assert_eq!(snapshot(pic, canister_id).await.completed_work, 0);
        advance(pic, Duration::from_secs(1), 16).await;
        let recovered = snapshot(pic, canister_id).await;
        assert_eq!(recovered.completed_work, u64::from(replace));
        assert_eq!(recovered.scheduler_started.total, u128::from(replace));
        if !replace {
            update_wire::<(), _>(
                pic,
                canister_id,
                Principal::anonymous(),
                Method::ScheduleAt,
                (0_u64,),
            )
            .await
            .expect("reschedule retained registration");
            advance(pic, Duration::ZERO, 16).await;
            assert_eq!(snapshot(pic, canister_id).await.completed_work, 1);
        }
        advance(pic, Duration::ZERO, 8).await;
        assert_eq!(snapshot(pic, canister_id).await.completed_work, 1);

        close_instance(owned).await;
    }
}

#[tokio::test]
async fn immediate_initial_watchdog_runs_without_advancing_cadence_time() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();

    assert!(update_bool(pic, canister_id, Method::StartImmediately).await);
    let initial = snapshot(pic, canister_id).await;
    assert_eq!(initial.completed_work, 0);
    // Initial After(0) is an explicit request, not a callback continuation.
    assert!(!initial.immediate_scheduling);
    assert_eq!(initial.latest_requested_delay_ns, Some(0));
    assert_eq!(initial.latest_armed_delay_ns, Some(0));
    assert_eq!(initial.schedule_requests.total, 1);
    assert_eq!(initial.wakeups_armed.total, 1);

    advance(pic, Duration::ZERO, 16).await;
    let completed = snapshot(pic, canister_id).await;
    assert_eq!(completed.completed_work, 1);
    assert_eq!(completed.scheduler_started.total, 1);
    assert_eq!(completed.work_dispatched.total, 1);
    assert_eq!(completed.work_completed.total, 1);
    assert!(completed.next_deadline_ns.is_some());

    close_instance(owned).await;
}

#[tokio::test]
async fn successful_immediate_continuation_runs_again_without_waiting_for_cadence() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::Start).await);
    set_work_decision(pic, canister_id, WorkDecision::ContinueImmediately).await;

    let cycles_before_first = pic.cycle_balance(canister_id).await;
    pic.advance_time(Duration::from_secs(1)).await;
    let immediate = wait_for_probe(
        pic,
        16,
        async |pic| snapshot(pic, canister_id).await,
        |current| current.completed_work == 1,
    )
    .await;
    assert!(immediate.immediate_scheduling);
    assert_eq!(immediate.latest_requested_delay_ns, Some(0));
    assert_eq!(immediate.latest_armed_delay_ns, Some(0));
    assert_eq!(immediate.schedule_requests.total, 1);
    assert_eq!(immediate.wakeups_armed.total, 3);
    assert_eq!(immediate.work_dispatched.total, 1);
    let first_dispatch_cycles =
        cycles_before_first.saturating_sub(pic.cycle_balance(canister_id).await);

    let cycles_before_second = pic.cycle_balance(canister_id).await;
    advance(pic, Duration::ZERO, 16).await;
    let continued = snapshot(pic, canister_id).await;
    assert_eq!(continued.completed_work, 2);
    assert_eq!(continued.scheduler_started.total, 2);
    assert_eq!(continued.work_dispatched.total, 2);
    assert_eq!(continued.work_completed.total, 2);
    assert_eq!(continued.wakeups_armed.total, 4);
    assert_eq!(continued.schedule_requests.total, 1);
    assert_eq!(continued.coalesced.total, 0);
    let second_dispatch_cycles =
        cycles_before_second.saturating_sub(pic.cycle_balance(canister_id).await);
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

    close_instance(owned).await;
}

#[tokio::test]
async fn commit_window_ensure_stop_resume_and_large_overdue_time_remain_bounded() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::Start).await);

    update_unit(pic, canister_id, Method::Stop).await;
    let inactive = snapshot(pic, canister_id).await;
    assert_eq!(inactive.next_deadline_ns, None);
    assert!(!update_bool(pic, canister_id, Method::EnsureFromReturnedError).await);
    let ensured = snapshot(pic, canister_id).await;
    assert!(ensured.next_deadline_ns.is_some());
    let arms_after_ensure = ensured.wakeups_armed.total;
    assert!(!update_bool(pic, canister_id, Method::EnsureFromReturnedError).await);
    assert_eq!(
        snapshot(pic, canister_id).await.wakeups_armed.total,
        arms_after_ensure
    );

    pic.stop_canister(canister_id, None)
        .await
        .expect("canister stop should succeed");
    advance(pic, Duration::from_secs(300), 4).await;
    pic.start_canister(canister_id, None)
        .await
        .expect("canister resume should succeed");
    advance(pic, Duration::ZERO, 16).await;
    let resumed = snapshot(pic, canister_id).await;
    assert_eq!(resumed.completed_work, 1);
    assert_eq!(resumed.scheduler_started.total, 1);
    assert_eq!(resumed.work_dispatched.total, 1);
    assert_eq!(resumed.work_started.total, 1);
    assert_eq!(resumed.work_completed.total, 1);
    assert!(resumed.next_deadline_ns.is_some());

    set_work_decision(pic, canister_id, WorkDecision::Stop).await;
    advance(pic, Duration::from_secs(1), 16).await;
    let terminal = snapshot(pic, canister_id).await;
    assert_eq!(terminal.completed_work, 2);
    assert_eq!(terminal.next_deadline_ns, None);
    advance(pic, Duration::from_secs(2), 8).await;
    assert_eq!(snapshot(pic, canister_id).await.completed_work, 2);

    close_instance(owned).await;
}

#[tokio::test]
async fn full_inventory_is_bounded_deterministic_and_measured() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::Start).await);
    update_unit(pic, canister_id, Method::Stop).await;
    assert_eq!(
        update_u64(pic, canister_id, Method::FillInventory).await,
        64
    );
    assert_eq!(
        update_u64(pic, canister_id, Method::FillInventory).await,
        64
    );

    let (len, instructions, ordered): (u64, u64, bool) = query_wire(
        pic,
        canister_id,
        Principal::anonymous(),
        Method::InventoryMeasurement,
        (),
    )
    .await
    .expect("probe inventory query succeeds");
    assert_eq!(len, 64);
    assert!(instructions > 0);
    assert!(ordered);
    println!("ic_timers_inventory entries={len} instructions={instructions}");

    close_instance(owned).await;
}

#[tokio::test]
async fn cancellation_between_scheduler_and_work_makes_late_provider_delivery_harmless() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::Start).await);
    pic.advance_time(Duration::from_secs(1)).await;

    let dispatched = wait_for_probe(
        pic,
        12,
        async |pic| snapshot(pic, canister_id).await,
        |current| current.work_dispatched.total == 1 && current.work_started.total == 0,
    )
    .await;
    assert_eq!(dispatched.completed_work, 0);
    assert!(dispatched.next_deadline_ns.is_some());

    update_unit(pic, canister_id, Method::Stop).await;
    advance(pic, Duration::ZERO, 12).await;
    let cancelled = snapshot(pic, canister_id).await;
    assert_eq!(cancelled.completed_work, 0);
    assert_eq!(cancelled.work_started.total, 0);
    assert_eq!(cancelled.work_completed.total, 0);
    assert_eq!(cancelled.next_deadline_ns, None);

    close_instance(owned).await;
}

#[tokio::test]
async fn exact_deadline_sleeps_and_registration_replacement_marks_counter_regrowth() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::Start).await);
    let initial = snapshot(pic, canister_id).await;
    let first_deadline = initial.next_deadline_ns.expect("scheduled");
    let later = first_deadline + 10_000_000_000;
    update_deadline(pic, canister_id, Method::ScheduleAt, later).await;
    update_deadline(pic, canister_id, Method::ScheduleAt, later).await;
    advance(pic, Duration::from_secs(2), 16).await;
    let sleeping = snapshot(pic, canister_id).await;
    assert_eq!(sleeping.completed_work, 0);
    assert_eq!(sleeping.scheduler_started.total, 0);
    assert_eq!(sleeping.next_deadline_ns, Some(later));
    assert_eq!(sleeping.wakeups_armed.total, 2);
    assert_eq!(sleeping.registration_id, initial.registration_id);

    let continuation = later + 10_000_000_000;
    // Last request wins; superseded stop/immediate requests cannot leak into
    // this callback or a later one.
    set_work_decision(pic, canister_id, WorkDecision::Stop).await;
    set_work_decision(pic, canister_id, WorkDecision::ContinueImmediately).await;
    set_work_decision(pic, canister_id, WorkDecision::ScheduleAt(continuation)).await;
    update_deadline(pic, canister_id, Method::ScheduleAt, first_deadline).await;
    advance(pic, Duration::ZERO, 16).await;
    let completed = snapshot(pic, canister_id).await;
    assert_eq!(completed.completed_work, 1);
    assert_eq!(completed.next_deadline_ns, Some(continuation));
    assert_eq!(completed.registration_id, initial.registration_id);
    advance(pic, Duration::from_secs(2), 16).await;
    assert_eq!(snapshot(pic, canister_id).await.completed_work, 1);

    update_unit(pic, canister_id, Method::Stop).await;
    assert_eq!(
        snapshot(pic, canister_id).await.registration_id,
        completed.registration_id
    );
    update_unit(pic, canister_id, Method::ReplaceRegistration).await;
    let replaced = snapshot(pic, canister_id).await;
    assert_eq!(replaced.work_completed.total, 0);
    assert_ne!(replaced.registration_id, completed.registration_id);
    let old_id = completed.registration_id.expect("old id");
    let new_id = replaced.registration_id.expect("new id");
    assert_eq!((old_id.0, old_id.1), (new_id.0, new_id.1));
    assert!(new_id.2 > old_id.2);
    for _ in 0..2 {
        advance(pic, Duration::from_secs(1), 16).await;
    }
    let regrown = snapshot(pic, canister_id).await;
    assert!(regrown.work_completed.total > completed.work_completed.total);
    assert!(regrown.scheduler_instruction_total > completed.scheduler_instruction_total);
    assert_eq!(regrown.registration_id, replaced.registration_id);

    close_instance(owned).await;
}

async fn dispatch_batch_observation(
    pic: &PocketIc,
    canister_id: Principal,
) -> DispatchBatchObservation {
    query_wire(
        pic,
        canister_id,
        Principal::anonymous(),
        Method::DispatchBatchObservation,
        (),
    )
    .await
    .expect("batch observation")
}

#[tokio::test]
async fn maximum_capacity_dispatch_preserves_every_survivor_under_pressure_and_await() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    update_unit(pic, canister_id, Method::StartDispatchBatch).await;
    advance(pic, Duration::ZERO, 8).await;
    let (failures, replies): (u64, u64) = query_wire(
        pic,
        canister_id,
        Principal::anonymous(),
        Method::PressureObservation,
        (),
    )
    .await
    .expect("batch pressure observation");
    assert!(
        failures > 0 && replies >= 500,
        "real queue pressure must be established"
    );
    let blocked = dispatch_batch_observation(pic, canister_id).await;
    assert_eq!(blocked.inventory_len, 64);
    assert_eq!(blocked.starts, vec![0; 65]);
    assert_eq!(blocked.completions, vec![0; 65]);

    // Both original callbacks are still pending, so neither may run afterward.
    update_unit(pic, canister_id, Method::ChangePendingDispatchBatch).await;
    update_unit(pic, canister_id, Method::ReleasePressure).await;
    advance(pic, Duration::ZERO, 16).await;
    assert_eq!(
        dispatch_batch_observation(pic, canister_id).await.starts,
        vec![0; 65]
    );
    pic.advance_time(Duration::from_secs(1)).await;
    let expected: Vec<u64> = (0..65).map(|index| u64::from(index >= 2)).collect();
    let waiting = wait_for_probe(
        pic,
        64,
        async |pic| dispatch_batch_observation(pic, canister_id).await,
        |state| state.active == 63,
    )
    .await;
    assert_eq!(waiting.starts, expected);
    assert_eq!(waiting.completions, vec![0; 65]);
    assert_eq!(waiting.inventory_len, 64);
    assert_eq!(waiting.peak_active, 63);
    advance(pic, Duration::from_secs(10), 16).await;
    assert_eq!(dispatch_batch_observation(pic, canister_id).await, waiting);

    // Rearm the retained cancelled claim while every survivor is still awaiting.
    update_unit(pic, canister_id, Method::RearmCancelledDispatchBatchTimer).await;
    let mut full_expected = expected;
    full_expected[0] = 1;
    let full = wait_for_probe(
        pic,
        64,
        async |pic| dispatch_batch_observation(pic, canister_id).await,
        |state| state.active == 64,
    )
    .await;
    assert_eq!(full.starts, full_expected);
    assert_eq!(full.completions, vec![0; 65]);
    assert_eq!(full.inventory_len, 64);
    assert_eq!(full.peak_active, 64);

    update_unit(pic, canister_id, Method::ReleaseDispatchBatch).await;
    let finished = wait_for_probe(
        pic,
        64,
        async |pic| dispatch_batch_observation(pic, canister_id).await,
        |state| state.completions == full_expected,
    )
    .await;
    assert_eq!(finished.starts, full_expected);
    assert_eq!(finished.active, 0);
    advance(pic, Duration::from_secs(2), 16).await;
    assert_eq!(dispatch_batch_observation(pic, canister_id).await, finished);

    // The cancelled retained claim and production dispatch remain usable at capacity.
    update_unit(pic, canister_id, Method::RearmCancelledDispatchBatchTimer).await;
    let mut rearmed_expected = full_expected;
    rearmed_expected[0] = 2;
    let rearmed = wait_for_probe(
        pic,
        64,
        async |pic| dispatch_batch_observation(pic, canister_id).await,
        |state| state.completions == rearmed_expected,
    )
    .await;
    assert_eq!(rearmed.starts, rearmed_expected);
    assert_eq!(rearmed.active, 0);
    assert_eq!(rearmed.inventory_len, 64);
    advance(pic, Duration::from_secs(2), 16).await;
    assert_eq!(dispatch_batch_observation(pic, canister_id).await, rearmed);

    close_instance(owned).await;
}

async fn scheduling_trace(pic: &PocketIc, canister_id: Principal) -> Vec<u8> {
    query_wire(
        pic,
        canister_id,
        Principal::anonymous(),
        Method::SchedulingTrace,
        (),
    )
    .await
    .expect("scheduling trace")
}

#[tokio::test]
async fn deadline_trace_covers_nested_registration_and_self_cancellation() {
    for overdue in [false, true] {
        let (owned, canister_id) = installed_probe().await;
        let pic = owned.instance();
        update_unit(pic, canister_id, Method::StartSchedulingTrace).await;
        advance(pic, Duration::ZERO, 8).await;
        assert!(scheduling_trace(pic, canister_id).await.is_empty());
        if overdue {
            advance(pic, Duration::from_secs(4), 16).await;
            assert_eq!(scheduling_trace(pic, canister_id).await, [1, 2, 4]);
            advance(pic, Duration::from_secs(1), 8).await;
            assert_eq!(scheduling_trace(pic, canister_id).await, [1, 2, 4]);
            advance(pic, Duration::from_secs(1), 8).await;
            assert_eq!(scheduling_trace(pic, canister_id).await, [1, 2, 4, 3]);
        } else {
            for end in 1..=4 {
                advance(pic, Duration::from_secs(1), 8).await;
                assert_eq!(
                    scheduling_trace(pic, canister_id).await,
                    (1..=end).collect::<Vec<u8>>()
                );
            }
            advance(pic, Duration::from_secs(2), 8).await;
            assert_eq!(scheduling_trace(pic, canister_id).await, [1, 2, 3, 4]);
        }
        let mut expected = if overdue {
            vec![1, 2, 4, 3]
        } else {
            vec![1, 2, 3, 4]
        };
        for event in [5, 6] {
            advance(pic, Duration::from_secs(1), 8).await;
            expected.push(event);
            assert_eq!(scheduling_trace(pic, canister_id).await, expected);
        }
        // The periodic callback requested recurrence but cancelled itself first.
        advance(pic, Duration::from_secs(20), 32).await;
        assert_eq!(scheduling_trace(pic, canister_id).await, expected);

        close_instance(owned).await;
    }
}
