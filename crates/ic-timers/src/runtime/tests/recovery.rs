use super::*;

#[test]
fn abandoned_initial_and_successor_schedulers_reconcile_all_owner_contracts() {
    for successor in [false, true] {
        let expected_deadline = if successor { 20 } else { 15 };
        for desired in [
            WatchdogReconcileState::Scheduled,
            WatchdogReconcileState::ScheduledImmediately,
            WatchdogReconcileState::ScheduledAt(expected_deadline),
        ] {
            let _fixture = setup();
            let timer = identity("abandoned-scheduler");
            let calls = Rc::new(Cell::new(0));
            let callback_calls = Rc::clone(&calls);
            let cadence = TimerCadence::from_nanos(5).unwrap();
            let mut owner = Some(
                register_watchdog(
                    timer.clone(),
                    cadence,
                    TimerStart::Retained {
                        initial: Some(TimerSchedule::At(15)),
                    },
                    move |_| {
                        callback_calls.set(callback_calls.get() + 1);
                        WatchdogRunResult::new(
                            TimerCompletion::success(1),
                            WatchdogDecision::Continue,
                        )
                    },
                )
                .unwrap(),
            );
            if successor {
                set_time(15);
                assert!(run_next_due());
                assert!(run_next_due());
            }
            let before = timer_snapshot(&timer).unwrap().unwrap();
            let deadline = before.next_deadline_ns().unwrap();
            assert_eq!(deadline, expected_deadline);
            let old = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
                .unwrap()
                .unwrap();
            let (old_token, handle) = old;
            bind_provider_handle(&old_token, handle, ProviderBinding::Stage).unwrap();
            let stale = dispatch_wakeup(WakeupToken::Scheduler(old_token));
            set_time(deadline);
            assert!(discard_next_due());
            assert!(!owner.as_ref().unwrap().has_armed_wakeup().unwrap());
            assert_eq!(timer_snapshot(&timer).unwrap().unwrap(), before);
            for _ in 0..2 {
                reconcile_watchdog(&mut owner, &timer, cadence, desired, |_| {
                    unreachable!("existing callback retained")
                })
                .unwrap();
            }
            drop(stale); // old generation cannot mark the replacement abandoned
            assert!(owner.as_ref().unwrap().has_armed_wakeup().unwrap());
            assert_eq!(timer_count(), 1);
            assert!(run_next_due());
            assert!(run_next_due());
            assert_eq!(calls.get(), u64::from(successor) + 1);
            let after = timer_snapshot(&timer).unwrap().unwrap();
            assert_eq!(after.registration_id(), before.registration_id());
            assert_eq!(
                after
                    .observability()
                    .counters()
                    .unacknowledged()
                    .exact_total(),
                Ok(0)
            );
        }
    }
}

#[test]
fn abandoned_successor_repair_keeps_queued_work_and_pending_reconciliation() {
    for discard_work in [false, true] {
        let _fixture = setup();
        let timer = identity("lost-successor-with-pending-work");
        let calls = Rc::new(Cell::new(0));
        let callback_calls = Rc::clone(&calls);
        let cadence = TimerCadence::from_nanos(5).unwrap();
        let mut owner = Some(
            register_watchdog(
                timer.clone(),
                cadence,
                TimerStart::Retained {
                    initial: Some(TimerSchedule::At(10)),
                },
                move |_| {
                    callback_calls.set(callback_calls.get() + 1);
                    WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue)
                },
            )
            .unwrap(),
        );
        assert!(run_next_due()); // successor at 15, work at 10
        if discard_work {
            assert!(discard_next_due());
        }
        assert!(platform::discard_timer_at(15));
        assert!(!owner.as_ref().unwrap().has_armed_wakeup().unwrap());
        reconcile_watchdog(
            &mut owner,
            &timer,
            cadence,
            WatchdogReconcileState::ScheduledAt(100),
            |_| unreachable!(),
        )
        .unwrap();
        assert!(owner.as_ref().unwrap().has_armed_wakeup().unwrap());
        assert_eq!(timer_count(), if discard_work { 1 } else { 2 });
        if discard_work {
            set_time(15);
            assert!(run_next_due());
        }
        assert!(run_next_due());
        assert_eq!(calls.get(), 1);
        let after = timer_snapshot(&timer).unwrap().unwrap();
        assert_eq!(
            after.next_deadline_ns(),
            Some(if discard_work { 20 } else { 100 })
        );
        assert_eq!(
            after
                .observability()
                .counters()
                .unacknowledged()
                .exact_total(),
            Ok(u128::from(discard_work))
        );
        assert_eq!(timer_count(), 1);
    }
}

#[test]
fn watchdog_successor_retires_an_unacknowledged_dispatched_attempt() {
    let _fixture = setup();
    let timer = identity("watchdog-unacknowledged");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    registration
        .reconcile_schedule(Some(TimerSchedule::At(100)))
        .expect("dispatched reconciliation should queue for this attempt");
    assert!(
        discard_next_due(),
        "simulate work without committed completion"
    );
    assert_eq!(timer_count(), 1);

    set_time(20);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .unacknowledged()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .work_started()
            .exact_total()
            .unwrap(),
        0
    );
    assert_eq!(
        snapshot.observability().outcomes().last_outcome(),
        Some(TimerLastOutcome::Unacknowledged)
    );

    assert!(run_next_due());
    let completed = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(
        completed.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped,
        },
        "the retired attempt's reconciliation must not override the successor's Stop"
    );
    assert_eq!(
        completed
            .observability()
            .counters()
            .unacknowledged()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(
        completed
            .observability()
            .counters()
            .work_completed()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(timer_count(), 0);
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered reconstruction and external-demand sequence."
)]
fn reconstruction_and_external_demand_are_synchronous_and_idempotent() {
    let _fixture = setup();
    let timer = identity("reconstructed-watchdog");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");
    let result = Rc::new(Cell::new(WatchdogRunResult::new(
        TimerCompletion::success(1),
        WatchdogDecision::ContinueImmediately,
    )));
    let pages = Rc::new(Cell::new(0_u64));
    let mut registration = None;

    let callback_result = Rc::clone(&result);
    let callback_pages = Rc::clone(&pages);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
        move |_context| {
            advance_instructions(13);
            let result = callback_result.get();
            callback_pages.set(callback_pages.get() + result.completion().work_count());
            result
        },
    )
    .expect("fresh reconstruction should register and schedule");
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
        |_context| {
            WatchdogRunResult::new(
                TimerCompletion::invariant_failure(0),
                WatchdogDecision::Stop,
            )
        },
    )
    .expect("repeated reconstruction should reuse the original callback claim");
    assert_eq!(timer_count(), 1);

    run_watchdog_round(15);
    assert_eq!((pages.get(), timer_count()), (1, 1));

    result.set(WatchdogRunResult::new(
        TimerCompletion::retryable_failure(0),
        WatchdogDecision::Continue,
    ));
    run_watchdog_round(20);
    assert_eq!((pages.get(), timer_count()), (1, 1));
    let retrying = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retryable failure must retain the watchdog");
    assert!(matches!(
        retrying.state(),
        TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled { .. })
    ));
    let retrying_counters = retrying.observability().counters();
    assert_eq!(retrying_counters.work_completed().exact_total().unwrap(), 2);
    assert_eq!(
        retrying_counters.retryable_failure().exact_total().unwrap(),
        1
    );

    result.set(WatchdogRunResult::new(
        TimerCompletion::no_work(),
        WatchdogDecision::Stop,
    ));
    run_watchdog_round(25);
    assert_eq!((pages.get(), timer_count()), (1, 0));
    let stopped = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    assert_lifecycle_measurements(&stopped);

    result.set(WatchdogRunResult::new(
        TimerCompletion::retryable_failure(0),
        WatchdogDecision::Continue,
    ));
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("returned-error commit guard should synchronously restore a wake-up");
    assert_eq!(timer_count(), 1);

    set_time(30);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    result.set(WatchdogRunResult::new(
        TimerCompletion::invariant_failure(0),
        WatchdogDecision::Stop,
    ));
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    let terminal = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    assert_eq!(
        terminal.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::InvariantFailure,
        }
    );
    let terminal_counters = terminal.observability().counters();
    assert_eq!(terminal_counters.work_completed().exact_total().unwrap(), 4);
    assert_eq!(
        terminal_counters.invariant_failure().exact_total().unwrap(),
        1
    );
    assert_eq!(terminal_counters.succeeded().exact_total().unwrap(), 1);
    assert_eq!(terminal_counters.no_work().exact_total().unwrap(), 1);
    assert_eq!(
        terminal_counters.retryable_failure().exact_total().unwrap(),
        1
    );

    result.set(WatchdogRunResult::new(
        TimerCompletion::success(1),
        WatchdogDecision::ContinueImmediately,
    ));
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("retained terminal declaration can be reconstructed from durable demand");
    assert_eq!(timer_count(), 1);
    set_time(35);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("terminal durable authority should cancel successor and queued work");
    assert_eq!(timer_count(), 0);

    let mismatch = reconcile_watchdog(
        &mut registration,
        &timer,
        TimerCadence::from_nanos(6).expect("fixture cadence should be valid"),
        WatchdogReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    );
    assert!(matches!(mismatch, Err(TimerError::ReconciliationConflict)));
}

fn run_watchdog_round(now_ns: u64) {
    set_time(now_ns);
    assert!(run_next_due(), "scheduler message must run at {now_ns}");
    assert!(run_next_due(), "work message must run at {now_ns}");
}

fn assert_lifecycle_measurements(snapshot: &TimerSnapshot) {
    let performance = snapshot.observability().performance();
    for (measurement, samples) in [
        (
            "scheduler instructions",
            performance.scheduler_instructions().aggregate().samples(),
        ),
        (
            "work instructions",
            performance.work_instructions().aggregate().samples(),
        ),
        (
            "scheduler memory",
            performance.scheduler_memory_pages().samples(),
        ),
        ("work memory", performance.work_memory_pages().samples()),
    ] {
        assert_eq!(
            samples, 3,
            "{measurement} after three completed work rounds"
        );
    }
    assert_eq!(performance.scheduler_instructions().latest(), Some(10));
    assert!(
        performance
            .work_instructions()
            .latest()
            .is_some_and(|value| value >= 13)
    );
}
