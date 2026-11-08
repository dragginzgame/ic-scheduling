use super::*;

#[test]
fn watchdog_claim_observes_the_prearmed_successor_not_queued_work() {
    let _fixture = setup();
    let watchdog = register_watchdog(
        identity("liveness-watchdog-successor"),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("watchdog registration should succeed");
    watchdog
        .ensure_scheduled()
        .expect("watchdog wake-up should arm");

    set_time(15);
    assert!(run_next_due(), "scheduler callback should execute");
    assert_eq!(timer_count(), 2, "successor and work should both be queued");
    assert!(
        watchdog
            .has_armed_wakeup()
            .expect("successor ownership should be readable")
    );

    watchdog
        .cancel()
        .expect("cancellation should clear successor and work");
    assert!(
        !watchdog
            .has_armed_wakeup()
            .expect("retained claim should remain readable")
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn immediate_watchdog_reconciliation_arms_one_zero_delay_scheduler() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-reconcile");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = None;
    reconcile_watchdog(
        &mut registration,
        &timer,
        test_cadence(5),
        WatchdogReconcileState::ScheduledImmediately,
        stopping_watchdog(move || {
            callback_calls.set(callback_calls.get().saturating_add(1));
        }),
    )
    .expect("fresh immediate reconciliation should succeed");

    assert_eq!(timer_count(), 1);
    let scheduled = registered_snapshot(&timer);
    assert_eq!(scheduled.next_deadline_ns(), Some(10));
    assert_eq!(
        scheduled.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(scheduled.latest_requested_delay_ns(), Some(0));
    assert_eq!(scheduled.latest_armed_delay_ns(), Some(0));
    let counters = scheduled.observability().counters();
    assert_eq!(counters.schedule_requests().exact_total().unwrap(), 1);
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 1);

    assert!(run_next_due(), "zero-delay scheduler should run later");
    assert_eq!(calls.get(), 0, "the scheduler must not invoke work inline");
    assert_eq!(
        timer_count(),
        2,
        "cadence successor and work should be armed"
    );
    assert!(run_next_due(), "separate work callback should run later");
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 0);
}

#[test]
fn immediate_watchdog_ensure_moves_cadence_earlier_and_repeats_idempotently() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-ensure");
    let registration = register_watchdog(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        stopping_watchdog(|| {}),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("cadence scheduler should arm");
    assert_eq!(timer_count(), 1);
    registration
        .ensure_scheduled_immediately()
        .expect("cadence scheduler should move to now");
    registration
        .ensure_scheduled_immediately()
        .expect("repeated immediate ensure should coalesce");

    assert_eq!(timer_count(), 1, "replacement must own exactly one handle");
    let snapshot = registered_snapshot(&timer);
    assert_eq!(snapshot.next_deadline_ns(), Some(10));
    assert_eq!(
        snapshot.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
    let counters = snapshot.observability().counters();
    assert_eq!(counters.schedule_requests().exact_total().unwrap(), 3);
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 2);
    assert_eq!(counters.coalesced().exact_total().unwrap(), 1);

    registration
        .cancel()
        .expect("fixture cleanup should succeed");
    assert_eq!(timer_count(), 0);
}

#[test]
fn immediate_watchdog_ensure_coalesces_an_overdue_cadence_wakeup() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-overdue-cadence");
    let registration = register_watchdog(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        stopping_watchdog(|| {}),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("cadence scheduler should arm");

    set_time(20);
    registration
        .ensure_scheduled_immediately()
        .expect("overdue cadence scheduler should satisfy immediate demand");

    assert_eq!(timer_count(), 1, "coalescing must not add a provider timer");
    let snapshot = registered_snapshot(&timer);
    assert_eq!(snapshot.next_deadline_ns(), Some(15));
    assert_eq!(snapshot.scheduling_mode(), TimerSchedulingMode::Watchdog);
    assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(5));
    let counters = snapshot.observability().counters();
    assert_eq!(counters.schedule_requests().exact_total().unwrap(), 2);
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 1);
    assert_eq!(counters.coalesced().exact_total().unwrap(), 1);

    registration
        .cancel()
        .expect("fixture cleanup should succeed");
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_scheduler_prearms_successor_before_synchronous_work() {
    let _fixture = setup();
    let timer = identity("watchdog-prearm");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        move |_context| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            assert_eq!(timer_count(), 1, "successor must exist during work");
            let decision = if call == 1 {
                WatchdogDecision::Continue
            } else {
                WatchdogDecision::Stop
            };
            WatchdogRunResult::new(TimerCompletion::success(1), decision)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    assert_eq!(timer_count(), 1);

    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 0, "scheduler must not invoke consumer work");
    assert_eq!(timer_count(), 2, "successor and work must both be owned");
    let dispatched = registered_snapshot(&timer);
    let dispatched_counters = dispatched.observability().counters();
    assert!(matches!(
        dispatched.state(),
        TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::AwaitingWork { .. })
    ));
    assert_eq!(
        dispatched_counters
            .scheduler_started()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(
        dispatched_counters.wakeups_armed().exact_total().unwrap(),
        2
    );
    assert_eq!(
        dispatched_counters.work_dispatched().exact_total().unwrap(),
        1
    );

    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    set_time(20);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 2);
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_immediate_decision_replaces_successor_without_synchronous_recursion() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-decision");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        move |_context| {
            let call = callback_calls.get().saturating_add(1);
            callback_calls.set(call);
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                if call == 1 {
                    WatchdogDecision::ContinueImmediately
                } else {
                    WatchdogDecision::Stop
                },
            )
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(15);
    assert!(run_next_due(), "scheduler should pre-arm cadence safety");
    assert_eq!(timer_count(), 2);
    assert!(run_next_due(), "first work callback should run");
    assert_eq!(calls.get(), 1);
    assert_eq!(
        timer_count(),
        1,
        "cadence successor must be replaced, not added to"
    );
    let immediate = registered_snapshot(&timer);
    assert_eq!(immediate.next_deadline_ns(), Some(15));
    assert_eq!(
        immediate.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(immediate.latest_requested_delay_ns(), Some(0));
    assert_eq!(immediate.latest_armed_delay_ns(), Some(0));
    let counters = immediate.observability().counters();
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 3);
    assert_eq!(counters.work_dispatched().exact_total().unwrap(), 1);

    assert!(
        run_next_due(),
        "replacement scheduler should be a later message"
    );
    assert_eq!(
        calls.get(),
        1,
        "scheduler still must not invoke consumer work"
    );
    assert_eq!(timer_count(), 2);
    assert!(run_next_due(), "second separate work callback should run");
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_running_immediate_context_targets_that_attempts_successor() {
    let _fixture = setup();
    let timer = identity("watchdog-context-immediate");
    let registration = register_watchdog(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        |context| {
            context
                .ensure_scheduled_immediately()
                .expect("running immediate request should succeed");
            context
                .ensure_scheduled()
                .expect("later cadence ensure must not delay immediate demand");
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(timer_count(), 1);
    let snapshot = registered_snapshot(&timer);
    let snapshot_counters = snapshot.observability().counters();
    assert_eq!(snapshot.next_deadline_ns(), Some(15));
    assert_eq!(
        snapshot.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(
        snapshot_counters.schedule_requests().exact_total().unwrap(),
        3
    );
    assert_eq!(snapshot_counters.coalesced().exact_total().unwrap(), 2);

    registration
        .cancel()
        .expect("fixture cleanup should succeed");
}

#[test]
fn watchdog_cancellation_clears_successor_and_queued_work() {
    let _fixture = setup();
    let timer = identity("watchdog-cancel");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    let cancelled_generation = registered_snapshot(&timer).generation().unwrap();

    registration
        .cancel()
        .expect("cancellation should clear both handles");
    assert_eq!(timer_count(), 0);
    assert!(!run_next_due());
    assert_eq!(calls.get(), 0);
    let snapshot = registered_snapshot(&timer);
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .cancelled()
            .exact_total()
            .unwrap(),
        1
    );

    registration.ensure_scheduled().unwrap();
    assert_eq!(
        registered_snapshot(&timer).generation(),
        Some(cancelled_generation + 1)
    );
    assert_eq!(timer_count(), 1);
    set_time(20);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    registration.cancel().unwrap();
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_invariant_failure_overrides_nested_cancellation_and_clears_handles() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        let _fixture = setup();
        let timer = identity("watchdog-cancel-invariant");
        let registration = register_watchdog(
            timer.clone(),
            test_cadence(5),
            scheduled_start(lifetime, TimerSchedule::After(Duration::from_nanos(5))),
            |context: WatchdogContext| {
                context.cancel().unwrap();
                WatchdogRunResult::new(
                    TimerCompletion::invariant_failure(1),
                    WatchdogDecision::ContinueImmediately,
                )
            },
        )
        .unwrap();
        set_time(15);
        assert!(run_next_due(), "scheduler should pre-arm its successor");
        assert_eq!(timer_count(), 2);
        assert!(
            run_next_due(),
            "work should complete with invariant failure"
        );
        assert_eq!(
            timer_count(),
            0,
            "terminal failure must clear the successor"
        );

        let snapshot = timer_snapshot(&timer).unwrap();
        if lifetime == DeclarationLifetime::Retained {
            let snapshot = snapshot.unwrap();
            assert_eq!(
                snapshot.state(),
                TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::InvariantFailure,
                }
            );
            let counters = snapshot.observability().counters();
            assert_eq!(counters.work_completed().exact_total().unwrap(), 1);
            assert_eq!(counters.invariant_failure().exact_total().unwrap(), 1);
            assert_eq!(counters.cancelled().exact_total().unwrap(), 0);
        } else {
            assert!(snapshot.is_none());
            assert!(matches!(
                registration.cancel(),
                Err(TimerError::RegistrationExpired)
            ));
        }
    }
}

#[test]
fn watchdog_nested_cancel_then_ensure_retains_committed_successor() {
    let _fixture = setup();
    let timer = identity("watchdog-nested");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        move |context: WatchdogContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            if call == 1 {
                context.cancel().expect("nested cancel should succeed");
                context
                    .ensure_scheduled()
                    .expect("later nested ensure should succeed");
            }
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    assert!(matches!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .map(|snapshot| snapshot.state()),
        Some(TimerRuntimeStateSnapshot::Watchdog(
            WatchdogRuntimeStateSnapshot::Scheduled { .. }
        ))
    ));
}

#[test]
fn retained_watchdog_context_expires_without_clearing_successor() {
    let _fixture = setup();
    let timer = identity("watchdog-context-expiry");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let retained_context = Rc::new(RefCell::new(None));
    let callback_context = Rc::clone(&retained_context);
    let registration = register_watchdog(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        move |context: WatchdogContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            if call == 1 {
                *callback_context.borrow_mut() = Some(context);
            }
            let decision = if call == 1 {
                WatchdogDecision::Continue
            } else {
                WatchdogDecision::Stop
            };
            WatchdogRunResult::new(TimerCompletion::no_work(), decision)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1, "committed successor should remain armed");

    let expired = retained_context
        .borrow_mut()
        .take()
        .expect("first work callback should retain its context");
    assert_eq!(
        expired.identity(),
        &timer,
        "identity remains inert metadata"
    );
    assert!(matches!(
        expired.cancel(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        expired.ensure_scheduled(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        expired.ensure_scheduled_immediately(),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(
        timer_count(),
        1,
        "expired context must not clear or duplicate the successor"
    );

    set_time(20);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn overdue_watchdog_coalesces_to_one_dispatch_and_schedules_from_now() {
    let _fixture = setup();
    let timer = identity("watchdog-overdue");
    let registration = register_watchdog(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(1_000);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(1_005)
    );
    assert!(run_next_due());
    assert_eq!(timer_count(), 1);
    assert!(!run_next_due(), "no historical cadence point may replay");
}

#[test]
fn remove_on_stop_watchdog_clears_successor_before_dropping_entry() {
    let _fixture = setup();
    let timer = identity("watchdog-remove");
    let _registration = register_watchdog(
        timer.clone(),
        test_cadence(5),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_nanos(5)),
        },
        stopping_watchdog(|| {}),
    )
    .expect("watchdog registration should succeed");
    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    assert!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .is_none()
    );
}
