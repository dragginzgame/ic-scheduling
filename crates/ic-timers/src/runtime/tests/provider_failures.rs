use super::*;

fn assert_retained_provider_binding_failure(
    timer: &TimerIdentity,
    schedule_requests: u128,
    wakeups_armed: u128,
) {
    let failed = registered_snapshot(timer);
    let failed_counters = failed.observability().counters();
    assert_eq!(
        failed.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::ControlFailure(TimerControlFailure::ProviderBindingFailed),
        }
    );
    assert_eq!(failed.generation(), None);
    assert_counters!(
        failed_counters,
        (schedule_requests, schedule_requests),
        (wakeups_armed, wakeups_armed)
    );
}

#[test]
fn immediate_watchdog_completion_fault_traps_and_leaves_cadence_successor_armed() {
    let _fixture = setup();
    let timer = identity("watchdog-completion-fault");
    let registration = register_watchdog(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        |_context| {
            grow_memory_pages(2, 3);
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            )
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    inject_watchdog_completion_fault();
    let trapped = catch_unwind(AssertUnwindSafe(run_next_due));
    assert!(trapped.is_err(), "an internal completion failure must trap");
    assert_eq!(
        timer_count(),
        1,
        "the cadence successor was committed by the earlier scheduler message"
    );
    let snapshot = registered_snapshot(&timer);
    let counters = snapshot.observability().counters();
    assert_counters!(counters, (work_started, 1), (work_completed, 0));
    let performance = snapshot.observability().performance();
    assert_eq!(performance.scheduler_memory_pages().samples(), 1);
    assert_eq!(performance.work_memory_pages().samples(), 0);
    assert_eq!(performance.work_memory_pages().latest(), None);
}

#[test]
fn immediate_watchdog_provider_replacement_failure_traps_for_message_rollback() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-provider-fault");
    let registration = register_watchdog(
        timer,
        test_cadence(5),
        TimerStart::Retained { initial: None },
        |_context| {
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            )
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due(), "scheduler should commit cadence safety");

    ProviderFault::BINDING.inject();
    let trapped = catch_unwind(AssertUnwindSafe(run_next_due));
    assert!(
        trapped.is_err(),
        "replacement binding failure must trap the work message"
    );
    // The native provider mock is deliberately not transactional. On the IC,
    // the trap rolls the clear/install and registry transition back together,
    // restoring the cadence successor committed by the scheduler message.
}

#[test]
fn public_immediate_watchdog_replacement_failure_retires_false_scheduled_state() {
    let _fixture = setup();
    let timer = identity("watchdog-public-immediate-provider-fault");
    let registration = retained_watchdog(timer.clone(), || {});
    registration
        .ensure_scheduled()
        .expect("cadence scheduler should arm");

    ProviderFault::BINDING.inject();
    assert!(matches!(
        registration.ensure_scheduled_immediately(),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(timer_count(), 0);
    assert_retained_provider_binding_failure(&timer, 2, 1);
}

#[test]
fn provider_cleanup_borrow_failure_is_returned_instead_of_discarded() {
    let _fixture = setup();
    let timer = identity("provider-cleanup-fault");
    let registration = retained_watchdog(timer.clone(), || {});
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    let failure = RUNTIME.with(|runtime| {
        let _borrow = runtime.borrow();
        clear_entry_provider_handles(&timer)
    });
    assert!(matches!(failure, Err(TimerError::RuntimeBusy)));
    assert_eq!(timer_count(), 1, "failed cleanup must not lose the handle");
    registration
        .cancel()
        .expect("cleanup remains possible after the borrow is released");
    assert_eq!(timer_count(), 0);
    clear_entry_provider_handles(&timer).expect("empty entry cleanup should be harmless");
    registration.unregister().unwrap();
    clear_entry_provider_handles(&timer).expect("missing entry cleanup should be harmless");
    assert_eq!(timer_count(), 0);
}

#[test]
fn rejected_provider_binding_clears_handles_for_unavailable_or_expired_authority() {
    let _fixture = setup();
    let timer = identity("provider-binding-authority");
    let registration = retained_once(timer.clone(), || {});
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("provider wake-up should arm");
    let detached = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
        .expect("registry should be available")
        .expect("provider handle should detach");
    let (token, handle) = detached;

    let rejected = RUNTIME.with(|runtime| {
        let _borrow = runtime.borrow();
        bind_provider_handle(&token, handle, ProviderBinding::Stage)
    });
    assert!(matches!(rejected, Err(TimerError::RuntimeBusy)));
    assert_eq!(timer_count(), 0);

    registration
        .unregister()
        .expect("declaration should be removable after the borrow is released");
    let handle = platform::set_timer(Duration::ZERO, async {});
    assert!(matches!(
        bind_provider_handle(&token, handle, ProviderBinding::Stage),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(timer_count(), 0);

    RUNTIME.with(|runtime| *runtime.borrow_mut() = None);
    let handle = platform::set_timer(Duration::ZERO, async {});
    assert!(matches!(
        bind_provider_handle(&token, handle, ProviderBinding::Stage),
        Err(TimerError::NotInitialized)
    ));
    assert_eq!(timer_count(), 0);
}

#[test]
fn unchanged_watchdog_successor_and_queued_work_stay_bound_without_reinstallation() {
    for abandon_successor in [false, true] {
        let _fixture = setup();
        let timer = identity("provider-unaffected-handles");
        let registration = register_watchdog(
            timer.clone(),
            test_cadence(5),
            TimerStart::Retained {
                initial: Some(TimerSchedule::At(15)),
            },
            |_| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
        )
        .unwrap();
        set_time(15);
        assert!(run_next_due());
        assert_eq!(timer_count(), 2);
        if abandon_successor {
            assert!(platform::discard_timer_at(20));
            assert_eq!(timer_count(), 1);
            // Only the replacement scheduler binds. Queued work must remain
            // installed, without consuming another provider admission.
            ProviderFault::Binding { after: 1 }.inject();
            registration.ensure_scheduled().unwrap();
            assert_eq!(timer_count(), 2);
        } else {
            ProviderFault::BINDING.inject();
        }
        // Continue retains its prearmed successor without rebinding it.
        assert!(run_next_due());
        assert_eq!(timer_count(), 1);
        assert!(registration.has_armed_wakeup().unwrap());
        let snapshot = registered_snapshot(&timer);
        assert_eq!(snapshot.next_deadline_ns(), Some(20));
        assert_eq!(
            snapshot
                .observability()
                .counters()
                .work_completed()
                .exact_total(),
            Ok(1)
        );
        // The injected fault remains for the next real binding, proving neither
        // unaffected capability passed through the provider installer.
        assert!(matches!(
            registration.ensure_scheduled_immediately(),
            Err(TimerError::OwnershipInvariant)
        ));
        assert_eq!(timer_count(), 0);
        registration.unregister().unwrap();
    }
}

#[test]
fn terminal_scheduler_failure_clears_queued_work_before_transient_removal() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        let _fixture = setup();
        let timer = identity("terminal-scheduler-cleanup");
        let work = Rc::new(Cell::new(0));
        let observed_work = Rc::clone(&work);
        let registration = register_watchdog(
            timer.clone(),
            test_cadence(5),
            scheduled_start(lifetime, TimerSchedule::After(Duration::from_nanos(5))),
            move |_| {
                observed_work.set(observed_work.get() + 1);
                WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue)
            },
        )
        .unwrap();
        set_time(15);
        assert!(run_next_due());
        assert_eq!(timer_count(), 2);

        // Deliver the successor before its deferred work. Consume the fake
        // provider callback exactly as a scheduler entry would, retaining work.
        let wakeup = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
            .unwrap()
            .unwrap();
        let (scheduler, handle) = wakeup;
        platform::clear_timer(handle);
        assert_eq!(timer_count(), 1);
        set_time(u64::MAX);
        dispatch_watchdog_scheduler(&scheduler);

        assert_eq!(timer_count(), 0);
        assert_eq!(work.get(), 0);
        assert!(!run_next_due());
        if lifetime == DeclarationLifetime::Retained {
            assert!(!registration.has_armed_wakeup().unwrap());
            assert_eq!(
                registered_snapshot(&timer).state(),
                TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::ControlFailure(TimerControlFailure::DeadlineOverflow),
                }
            );
        } else {
            assert!(timer_snapshot(&timer).unwrap().is_none());
            assert!(matches!(
                registration.has_armed_wakeup(),
                Err(TimerError::RegistrationExpired)
            ));
            assert!(
                register_once(
                    timer,
                    TimerStart::Retained { initial: None },
                    stopping_once(|| {})
                )
                .is_ok()
            );
        }
    }
}

#[test]
fn rejected_and_coalesced_requests_preserve_bound_handles_without_reinstallation() {
    let _fixture = setup();
    let timer = identity("request-handle-preservation");
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        stopping_once(|| {}),
    )
    .unwrap();
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .unwrap();
    let before = registered_snapshot(&timer);

    // If either request attempts a handle reinstall it consumes this fault.
    ProviderFault::BINDING.inject();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::After(Duration::MAX)),
        Err(TimerError::Schedule(ScheduleError::DelayOutOfRange))
    ));
    assert_eq!(registered_snapshot(&timer), before);
    registration
        .ensure_scheduled(TimerSchedule::At(20))
        .unwrap();
    assert!(registration.has_armed_wakeup().unwrap());
    assert_eq!(timer_count(), 1);
    let after = registered_snapshot(&timer);
    assert_eq!(after.generation(), before.generation());
    assert_eq!(after.next_deadline_ns(), before.next_deadline_ns());
    assert_counters!(after.observability().counters(), (coalesced, 1));

    // A real replacement must still encounter the unconsumed installation fault.
    assert!(matches!(
        registration.reconcile_schedule(Some(TimerSchedule::At(25))),
        Err(TimerError::OwnershipInvariant)
    ));
    assert!(!registration.has_armed_wakeup().unwrap());
    assert_eq!(timer_count(), 0);
    registration.unregister().unwrap();
}

#[test]
fn public_provider_install_failure_retires_false_scheduled_state() {
    let _fixture = setup();
    let timer = identity("provider-install-fault");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = retained_once(timer.clone(), move || {
        callback_calls.set(callback_calls.get() + 1);
    });

    registration
        .ensure_scheduled(TimerSchedule::At(100))
        .expect("initial provider arm should succeed");
    assert_eq!(timer_count(), 1);
    ProviderFault::BINDING.inject();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(15)),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(
        timer_count(),
        0,
        "failed replacement must clear both old and new provider arms"
    );
    assert_retained_provider_binding_failure(&timer, 2, 1);

    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("retained registration should recover on a later request");
    assert_eq!(timer_count(), 1);
    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 0);
}

#[test]
fn inactive_ordinary_provider_install_failure_retires_false_scheduled_state() {
    for recurring in [false, true] {
        let _fixture = setup();
        let timer = identity("inactive-ordinary-provider-fault");
        // Keep both public control entry points covered; initial registration
        // and replacement failures have distinct lifecycle tests.
        let result = if recurring {
            let registration = register_after_completion(
                timer.clone(),
                test_cadence(5),
                TimerStart::Retained { initial: None },
                stopping_after_completion(|| {}),
            )
            .unwrap();
            ProviderFault::BINDING.inject();
            registration.ensure_scheduled()
        } else {
            let registration = retained_once(timer.clone(), || {});
            ProviderFault::BINDING.inject();
            registration.ensure_scheduled(TimerSchedule::At(15))
        };
        assert!(matches!(result, Err(TimerError::OwnershipInvariant)));
        assert_eq!(timer_count(), 0);
        assert_retained_provider_binding_failure(&timer, 1, 0);
    }
}

#[test]
fn watchdog_failed_dispatch_clears_handles_without_confirming_work() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        for failure_stage in [
            ProviderFault::ADMISSION,
            ProviderFault::Admission {
                after: 1,
                error: platform::AdmissionError::IdentityExhausted,
            },
            ProviderFault::BINDING,
            ProviderFault::Binding { after: 1 },
        ] {
            let _fixture = setup();
            let timer = identity("provider-watchdog-dispatch-fault");
            let calls = Rc::new(Cell::new(0));
            let callback_calls = Rc::clone(&calls);
            let registration = register_watchdog(
                timer.clone(),
                test_cadence(5),
                scheduled_start(lifetime, TimerSchedule::After(Duration::from_nanos(5))),
                move |_context| {
                    callback_calls.set(callback_calls.get() + 1);
                    WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue)
                },
            )
            .expect("registration should succeed");

            failure_stage.inject();
            set_time(15);
            assert!(run_next_due(), "scheduler callback should execute");
            assert_eq!(timer_count(), 0, "failed dispatch must clear every handle");
            assert!(
                !run_next_due(),
                "failed dispatch must not leave queued work"
            );
            assert_eq!(
                calls.get(),
                0,
                "failed dispatch must not invoke consumer work"
            );

            if lifetime == DeclarationLifetime::Retained {
                assert_retained_provider_binding_failure(&timer, 1, 1);
                assert!(!registration.has_armed_wakeup().unwrap());
                let snapshot = registered_snapshot(&timer);
                let counters = snapshot.observability().counters();
                assert_counters!(
                    counters,
                    (scheduler_started, 1),
                    (work_dispatched, 0),
                    (work_started, 0),
                    (work_completed, 0)
                );
            } else {
                assert!(timer_snapshot(&timer).unwrap().is_none());
                assert!(matches!(
                    registration.has_armed_wakeup(),
                    Err(TimerError::RegistrationExpired)
                ));
                let replacement = retained_once(timer, || {});
                replacement.ensure_scheduled(TimerSchedule::At(25)).unwrap();
                assert!(replacement.has_armed_wakeup().unwrap());
                assert!(matches!(
                    registration.has_armed_wakeup(),
                    Err(TimerError::RegistrationExpired)
                ));
                assert_eq!(timer_count(), 1);
                replacement.unregister().unwrap();
                assert_eq!(timer_count(), 0);
            }
        }
    }
}

#[test]
fn stale_watchdog_dispatch_clears_partial_arms_without_confirming_work() {
    let _fixture = setup();
    let registration = register_watchdog(
        identity("stale-dispatch"),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        |_| panic!("manual effects must not execute consumer work"),
    )
    .unwrap();
    registration.ensure_scheduled().unwrap();
    let handle = with_registry_mut(|registry| {
        Ok(registry
            .take_wakeup_handle(registration.identity())
            .unwrap())
    })
    .unwrap();
    let (scheduler, handle) = handle;
    platform::clear_timer(handle);
    let first = with_registry_mut(|registry| Ok(registry.begin_watchdog_scheduler(&scheduler, 15)))
        .unwrap();
    let RegistryEffect::DispatchWatchdog { dispatch, .. } = first.effect() else {
        panic!("dispatch expected")
    };
    let successor = dispatch.successor().clone();
    let second =
        with_registry_mut(|registry| Ok(registry.begin_watchdog_scheduler(&successor, 20)))
            .unwrap();
    let before = timer_inventory().unwrap();
    assert!(apply_effect(first.effect(), ProviderHandles::default()).is_err());
    assert_eq!(timer_count(), 0);
    assert_eq!(timer_inventory().unwrap(), before);
    apply_effect(second.effect(), ProviderHandles::default()).unwrap();
    assert_eq!(timer_count(), 2);
    let counts = registered_snapshot(registration.identity())
        .observability()
        .counters();
    assert_counters!(counts, (wakeups_armed, 2), (work_dispatched, 1));
}

#[test]
fn remove_on_stop_provider_failure_removes_the_expired_claim() {
    let _fixture = setup();
    let timer = identity("provider-remove-on-stop-fault");
    let registration = register_once(
        timer.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(20),
        },
        stopping_once(|| {}),
    )
    .expect("registration should succeed");

    ProviderFault::BINDING.inject();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(15)),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(timer_count(), 0);
    assert!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .is_none(),
        "remove-on-stop failure must not retain a false declaration"
    );
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(20)),
        Err(TimerError::RegistrationExpired)
    ));
}
