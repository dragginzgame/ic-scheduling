use super::*;

#[test]
fn fixture_cleanup_drops_queued_and_suspended_work_before_tls_teardown() {
    struct CaptureDropCheck(Rc<Cell<bool>>);

    impl Drop for CaptureDropCheck {
        fn drop(&mut self) {
            assert_eq!(timer_count(), 0, "cleanup must release the task-map borrow");
            assert!(
                timer_inventory().is_ok(),
                "runtime TLS must remain available"
            );
            self.0.set(true);
        }
    }

    for (lifetime, scheduled, started) in [
        (DeclarationLifetime::RemoveWhenStopped, true, false),
        (DeclarationLifetime::RemoveWhenStopped, true, true),
        (DeclarationLifetime::Retained, false, false),
        (DeclarationLifetime::Retained, true, false),
        (DeclarationLifetime::Retained, true, true),
    ] {
        let dropped = Rc::new(Cell::new(false));
        {
            let _fixture = setup();
            let gate = SuspendedWork::default();
            let captured = Rc::new(CaptureDropCheck(Rc::clone(&dropped)));
            let _registration = register_once(
                identity("fixture-cleanup"),
                if scheduled {
                    scheduled_start(lifetime, TimerSchedule::At(10))
                } else {
                    TimerStart::Retained { initial: None }
                },
                move |_| {
                    let gate = gate.clone();
                    let captured = Rc::clone(&captured);
                    async move {
                        gate.wait().await;
                        std::hint::black_box(captured);
                        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
                    }
                },
            )
            .unwrap();
            if started {
                assert!(run_next_due());
            }
            assert!(!dropped.get());
        }
        assert!(
            dropped.get(),
            "fixture cleanup must release callback captures"
        );
        assert_eq!(timer_count(), 0);
        assert_eq!(timer_inventory().unwrap().timers(), []);
        assert!(!run_next_due());
    }
}

#[test]
fn rejected_watchdog_cadence_preserves_the_complete_snapshot() {
    let _fixture = setup();
    let timer = identity("rejected-watchdog-cadence");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(u64::MAX).expect("positive cadence is valid"),
        TimerStart::Retained { initial: None },
        stopping_watchdog(|| {}),
    )
    .expect("declaration should register without arming");
    let before = timer_snapshot(&timer).expect("snapshot should be readable");

    assert!(matches!(
        registration.ensure_scheduled(),
        Err(TimerError::Schedule(ScheduleError::DeadlineOverflow))
    ));
    assert_eq!(
        timer_snapshot(&timer).expect("snapshot should be readable"),
        before
    );
    assert_eq!(timer_count(), 0);
    assert!(
        !registration
            .has_armed_wakeup()
            .expect("claim remains valid")
    );

    registration
        .ensure_scheduled_immediately()
        .expect("a valid request remains possible after rejection");
    assert_eq!(timer_count(), 1);
    assert_eq!(
        registered_snapshot(&timer)
            .observability()
            .counters()
            .schedule_requests()
            .exact_total()
            .unwrap(),
        1
    );
}

#[test]
fn initialization_is_required_and_idempotent() {
    reset_for_test(10, 7);
    assert!(matches!(timer_inventory(), Err(TimerError::NotInitialized)));

    let first = initialize_runtime().expect("first initialization should succeed");
    set_time(20);
    let second = initialize_runtime().expect("repeated initialization should succeed");
    assert_eq!(first, second);
    assert_eq!(first.canister_version(), 7);
    assert_eq!(first.started_at_ns(), 10);
    let busy = RUNTIME.with(|runtime| {
        let _borrow = runtime.borrow();
        initialize_runtime()
    });
    assert!(matches!(busy, Err(TimerError::RuntimeBusy)));
    let inventory = timer_inventory().expect("initialized inventory should be available");
    assert_eq!(inventory.epoch(), first);
    assert!(inventory.is_empty());
    assert_eq!(inventory.timers(), []);
    assert_eq!(inventory.into_timers(), []);
}

#[test]
fn fresh_inactive_reconciliation_reserves_complete_retained_inventory() {
    let _fixture = setup();
    let once_identity = identity("built-in-once");
    let after_identity = identity("built-in-after");
    let watchdog_identity = identity("built-in-watchdog");
    let cadence = test_cadence(5);
    let mut once = None;
    let mut after = None;
    let mut watchdog = None;

    reconcile_once(&mut once, &once_identity, None, stopping_once(|| {}))
        .expect("fresh inactive once declaration should be retained");
    reconcile_after_completion(
        &mut after,
        &after_identity,
        cadence,
        TimerReconcileState::Inactive,
        stopping_after_completion(|| {}),
    )
    .expect("fresh inactive after-completion declaration should be retained");
    reconcile_watchdog(
        &mut watchdog,
        &watchdog_identity,
        cadence,
        WatchdogReconcileState::Inactive,
        stopping_watchdog(|| {}),
    )
    .expect("fresh inactive watchdog declaration should be retained");

    assert!(once.is_some());
    assert!(after.is_some());
    assert!(watchdog.is_some());
    assert_eq!(timer_count(), 0);
    let inventory = timer_inventory().expect("inventory should be available");
    assert_eq!(inventory.epoch(), TimerEpoch::new(7, 10));
    assert_eq!(inventory.len(), 3);
    let snapshots = inventory.timers();
    assert_eq!(
        snapshots
            .iter()
            .map(TimerSnapshot::policy)
            .collect::<Vec<_>>(),
        vec![
            TimerPolicy::AfterCompletion { cadence },
            TimerPolicy::Once,
            TimerPolicy::Watchdog { cadence },
        ]
    );
    for snapshot in snapshots {
        assert_eq!(snapshot.lifetime(), DeclarationLifetime::Retained);
        assert_eq!(
            snapshot.state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::NeverScheduled,
            }
        );
        assert_eq!(
            snapshot.registration_status(),
            TimerRegistrationStatus::Unregistered
        );
        assert_eq!(snapshot.generation(), None);
    }
}

#[test]
fn fresh_transient_cancellation_expires_every_registration_policy() {
    let _fixture = setup();
    let once_identity = identity("fresh-transient-once");
    let after_identity = identity("fresh-transient-after");
    let watchdog_identity = identity("fresh-transient-watchdog");
    let cadence = test_cadence(5);

    let once = register_once(
        once_identity.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(15),
        },
        stopping_once(|| {}),
    )
    .expect("once registration should succeed");
    let after = register_after_completion(
        after_identity.clone(),
        cadence,
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_nanos(5)),
        },
        stopping_after_completion(|| {}),
    )
    .expect("after-completion registration should succeed");
    let watchdog = register_watchdog(
        watchdog_identity.clone(),
        cadence,
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_nanos(5)),
        },
        stopping_watchdog(|| {}),
    )
    .expect("watchdog registration should succeed");

    once.cancel().expect("once cancellation should succeed");
    after
        .cancel()
        .expect("after-completion cancellation should succeed");
    watchdog
        .cancel()
        .expect("watchdog cancellation should succeed");

    assert_eq!(timer_count(), 0);
    for identity in [&once_identity, &after_identity, &watchdog_identity] {
        assert!(
            timer_snapshot(identity)
                .expect("snapshot lookup should succeed")
                .is_none(),
            "transient cancellation should remove {identity:?}"
        );
    }
    assert!(matches!(
        once.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        after.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        watchdog.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
}

#[test]
fn registration_claims_report_exact_provider_wakeup_ownership() {
    let _fixture = setup();
    let once = register_once(
        identity("liveness-once"),
        TimerStart::Retained { initial: None },
        stopping_once(|| {}),
    )
    .expect("once registration should succeed");
    let after = register_after_completion(
        identity("liveness-after"),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        stopping_after_completion(|| {}),
    )
    .expect("after-completion registration should succeed");
    let watchdog = register_watchdog(
        identity("liveness-watchdog"),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        stopping_watchdog(|| {}),
    )
    .expect("watchdog registration should succeed");

    assert!(!once.has_armed_wakeup().expect("claim should be readable"));
    assert!(!after.has_armed_wakeup().expect("claim should be readable"));
    assert!(
        !watchdog
            .has_armed_wakeup()
            .expect("claim should be readable")
    );

    once.ensure_scheduled(TimerSchedule::At(100))
        .expect("once wake-up should arm");
    after
        .ensure_scheduled()
        .expect("after-completion wake-up should arm");
    watchdog
        .ensure_scheduled()
        .expect("watchdog wake-up should arm");
    assert!(once.has_armed_wakeup().expect("claim should be readable"));
    assert!(after.has_armed_wakeup().expect("claim should be readable"));
    assert!(
        watchdog
            .has_armed_wakeup()
            .expect("claim should be readable")
    );

    once.cancel().expect("once cancellation should succeed");
    after
        .cancel()
        .expect("after-completion cancellation should succeed");
    watchdog
        .cancel()
        .expect("watchdog cancellation should succeed");
    assert!(!once.has_armed_wakeup().expect("claim should be readable"));
    assert!(!after.has_armed_wakeup().expect("claim should be readable"));
    assert!(
        !watchdog
            .has_armed_wakeup()
            .expect("claim should be readable")
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn removed_transient_claim_cannot_report_wakeup_liveness() {
    let _fixture = setup();
    let timer_identity = identity("liveness-transient");
    let timer = register_once(
        timer_identity.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(15),
        },
        stopping_once(|| {}),
    )
    .expect("registration should succeed");
    assert!(timer.has_armed_wakeup().expect("claim should be readable"));

    set_time(15);
    assert!(run_next_due());
    assert!(matches!(
        timer.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));

    let replacement = register_once(
        timer_identity,
        TimerStart::Retained { initial: None },
        stopping_once(|| {}),
    )
    .expect("replacement registration should succeed");
    replacement
        .ensure_scheduled(TimerSchedule::At(25))
        .expect("replacement wake-up should arm");
    assert!(
        replacement
            .has_armed_wakeup()
            .expect("replacement claim should be readable")
    );
    assert!(matches!(
        timer.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
}

#[test]
fn stale_reused_identity_callback_cannot_change_handles_or_measurements() {
    let _fixture = setup();
    let timer_identity = identity("stale-consume-reuse");
    let old = register_once(
        timer_identity.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(15),
        },
        stopping_once(|| {}),
    )
    .expect("old registration should succeed");
    let old_handle = with_registry_mut(|registry| {
        registry
            .take_wakeup_handle(&timer_identity)
            .ok_or(TimerError::OwnershipInvariant)
    })
    .expect("old provider handle should detach");
    let (old_token, old_provider_handle) = old_handle;
    platform::clear_timer(old_provider_handle);
    old.cancel()
        .expect("old transient registration should be removed");

    // Exercise the production recording path with current platform end counters.
    // A removed or reused claim must reject the sample before updating accounting.
    let stale_measurement = CallbackMeasurementStart {
        instructions_before: 0,
        memory_start: MemoryPageExtent::new(1, 0),
    };
    stale_measurement.record(&old_token);
    assert!(timer_snapshot(old_token.identity()).unwrap().is_none());

    let replacement = register_once(
        timer_identity,
        TimerStart::Retained { initial: None },
        |_context| async {
            advance_instructions(7);
            grow_memory_pages(2, 3);
            OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
        },
    )
    .expect("replacement registration should succeed");
    replacement
        .ensure_scheduled(TimerSchedule::At(25))
        .expect("replacement wake-up should arm");
    assert_eq!(timer_count(), 1);

    let before = registered_snapshot(replacement.identity());
    let before_performance = before.observability().performance();
    let stale_token = old_token.clone();
    let _stale_delivery = platform::set_timer(Duration::ZERO, async move {
        dispatch_wakeup(WakeupToken::Ordinary(stale_token)).await;
    });
    assert!(run_next_due(), "stale callback delivery should be harmless");
    // Exercise the accounting boundary directly; stale dispatch would return
    // before taking a measurement and cannot reach this ownership check.
    stale_measurement.record(&old_token);
    let after = registered_snapshot(replacement.identity());
    let counters = after.observability().counters();
    assert_eq!(after.observability().performance(), before_performance);
    assert_eq!(counters.stale_wakeups().exact_total().unwrap(), 1);
    assert_eq!(counters.work_started().exact_total().unwrap(), 0);
    assert!(
        replacement
            .has_armed_wakeup()
            .expect("replacement claim should retain its handle")
    );
    assert_eq!(timer_count(), 1);

    set_time(25);
    assert!(run_next_due());
    let completed = registered_snapshot(replacement.identity());
    let performance = completed.observability().performance();
    assert_eq!(performance.work_instructions().aggregate().samples(), 1);
    assert_eq!(performance.work_instructions().aggregate().total(), 7);
    assert_eq!(performance.work_memory_pages().samples(), 1);
    assert_eq!(
        performance.work_memory_pages().latest(),
        Some(MemoryPageSample::new(
            MemoryPageExtent::new(1, 0),
            MemoryPageExtent::new(3, 3),
        ))
    );
    assert_eq!(
        performance.scheduler_instructions(),
        before_performance.scheduler_instructions()
    );
    assert_eq!(
        performance.scheduler_memory_pages(),
        before_performance.scheduler_memory_pages()
    );
    stale_measurement.record(&old_token);
    let after_stale = registered_snapshot(replacement.identity());
    assert_eq!(after_stale.observability().performance(), performance);

    replacement
        .cancel()
        .expect("replacement cleanup should succeed");
    assert_eq!(timer_count(), 0);
}

#[test]
fn replacement_and_cancellation_clear_actual_owned_handles() {
    let _fixture = setup();
    let timer = identity("replace-cancel");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        stopping_once(move || {
            callback_calls.set(callback_calls.get() + 1);
        }),
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(100))
        .expect("initial ensure should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(50))
        .expect("earlier deadline should replace the handle");
    assert_eq!(timer_count(), 1);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(50)
    );
    let cancelled_generation = registered_snapshot(&timer).generation().unwrap();

    registration.cancel().expect("cancel should clear handle");
    assert_eq!(timer_count(), 0);
    registration
        .cancel()
        .expect("repeated cancellation should be idempotent");
    set_time(100);
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

    registration
        .ensure_scheduled(TimerSchedule::At(150))
        .unwrap();
    assert_eq!(
        registered_snapshot(&timer).generation(),
        Some(cancelled_generation + 1)
    );
    assert_eq!(timer_count(), 1);
    set_time(150);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 0);
}

#[test]
fn duplicate_registration_and_reconstruction_preserve_live_work_and_release_capacity() {
    let _fixture = setup();
    let timer = identity("duplicate");
    let first_calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&first_calls);
    let registration = register_once(
        timer.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(20),
        },
        stopping_once(move || {
            callback_calls.set(callback_calls.get() + 1);
        }),
    )
    .expect("registration should succeed");
    let before = timer_inventory().unwrap();
    let duplicate = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        |_context| async {
            OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
        },
    );
    assert!(matches!(
        duplicate,
        Err(TimerError::Register(
            RegisterError::IdentityAlreadyRegistered(_)
        ))
    ));
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    assert!(registration.has_armed_wakeup().unwrap());

    let mut empty_slot = None;
    let reconstruction = reconcile_once(&mut empty_slot, &timer, None, |_| async {
        OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
    });
    assert!(matches!(
        reconstruction,
        Err(TimerError::Register(
            RegisterError::IdentityAlreadyRegistered(ref occupied)
        )) if occupied == &timer
    ));
    assert!(empty_slot.is_none());
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    assert!(registration.has_armed_wakeup().unwrap());

    set_time(20);
    assert!(run_next_due());
    assert_eq!(first_calls.get(), 1);
    assert!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .is_none()
    );
    assert!(
        timer_inventory()
            .expect("inventory should succeed")
            .is_empty()
    );
}

#[test]
fn registration_identity_survives_control_but_changes_on_replacement_and_restart() {
    let fixture = setup();
    let epoch = fixture.epoch;
    let timer = identity("continuity");
    let create = || {
        register_once(
            timer.clone(),
            TimerStart::Retained { initial: None },
            |_| async {
                advance_instructions(7);
                OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
            },
        )
        .unwrap()
    };
    let snapshot = || registered_snapshot(&timer);
    let first = create();
    let original = snapshot().registration_id();
    assert_eq!(original.epoch(), epoch);
    assert_eq!(original.sequence(), 1);
    for _ in 0..2 {
        first.ensure_scheduled(TimerSchedule::At(10)).unwrap();
        assert!(run_next_due());
    }
    let completed = snapshot().observability().performance();
    assert_eq!(completed.work_instructions().aggregate().samples(), 2);
    assert_eq!(completed.work_instructions().aggregate().total(), 14);
    assert!(
        !completed
            .work_instructions()
            .aggregate()
            .samples_overflowed()
    );
    assert!(!completed.work_instructions().aggregate().total_overflowed());
    assert_eq!(completed.work_instructions().latest(), Some(7));
    assert_eq!(completed.work_instructions().aggregate().maximum(), Some(7));
    assert_eq!(
        completed.scheduler_instructions(),
        crate::MeasurementSummary::EMPTY
    );
    first.cancel().unwrap();
    let before = snapshot();
    assert_eq!(before.registration_id(), original);
    assert_eq!(before.observability().performance(), completed);
    first.unregister().unwrap();
    let replacement = create();
    assert_ne!(snapshot().registration_id(), original);
    assert_eq!(snapshot().registration_id().epoch(), epoch);
    let fresh = snapshot().observability().performance();
    assert_eq!(fresh.work_instructions(), crate::MeasurementSummary::EMPTY);
    assert_eq!(
        fresh.scheduler_instructions(),
        crate::MeasurementSummary::EMPTY
    );
    for _ in 0..3 {
        replacement.ensure_scheduled(TimerSchedule::At(10)).unwrap();
        assert!(run_next_due());
    }
    let regrown = snapshot();
    assert!(
        regrown
            .observability()
            .counters()
            .work_completed()
            .exact_total()
            .unwrap()
            > before
                .observability()
                .counters()
                .work_completed()
                .exact_total()
                .unwrap()
    );
    assert_ne!(regrown.registration_id(), before.registration_id());
    let regrown_work = regrown.observability().performance().work_instructions();
    assert_eq!(regrown_work.aggregate().samples(), 3);
    assert_eq!(regrown_work.aggregate().total(), 21);
    assert!(!regrown_work.aggregate().samples_overflowed());
    assert!(!regrown_work.aggregate().total_overflowed());
    // Counts and totals grow across replacement, but different registration
    // identities forbid treating those observations as one continuous interval.
    reset_for_test(20, 8);
    initialize_runtime().unwrap();
    let _reconstructed = create();
    assert_eq!(snapshot().registration_id().sequence(), original.sequence());
    assert_ne!(snapshot().registration_id().epoch(), original.epoch());
    assert_ne!(snapshot().registration_id(), original);
    let reconstructed = snapshot().observability().performance();
    assert_eq!(
        reconstructed.work_instructions(),
        crate::MeasurementSummary::EMPTY
    );
    assert_eq!(
        reconstructed.scheduler_instructions(),
        crate::MeasurementSummary::EMPTY
    );
}
