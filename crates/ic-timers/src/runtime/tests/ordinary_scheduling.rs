use super::*;

#[test]
fn once_owns_one_provider_handle_and_executes_without_registry_borrow() {
    let _fixture = setup();
    let timer = identity("once");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let callback_identity = timer.clone();
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            advance_instructions(7);
            grow_memory_pages(2, 3);
            let visible = timer_snapshot(&callback_identity)
                .expect("consumer work must not observe a registry borrow")
                .is_some();
            async move {
                assert!(visible);
                OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
            }
        },
    )
    .expect("registration should succeed");

    registration
        .ensure_scheduled(TimerSchedule::After(Duration::from_nanos(5)))
        .expect("ensure should arm provider timer");
    assert_eq!(timer_count(), 1);
    let scheduled = registered_snapshot(&timer);
    assert_counters!(scheduled.observability().counters(), (wakeups_armed, 1));
    assert_eq!(scheduled.latest_armed_delay_ns(), Some(5));

    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    assert_eq!(calls.get(), 1);
    let stopped = registered_snapshot(&timer);
    let stopped_counters = stopped.observability().counters();
    assert_eq!(
        stopped.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped,
        }
    );
    assert_counters!(stopped_counters, (work_started, 1), (work_completed, 1));
    assert_eq!(
        stopped
            .observability()
            .performance()
            .work_instructions()
            .latest(),
        Some(7)
    );
    let memory = stopped.observability().performance().work_memory_pages();
    assert_eq!(memory.samples(), 1);
    let latest = memory.latest().expect("normal work should have one sample");
    assert_eq!(latest.start().wasm_pages(), 1);
    assert_eq!(latest.start().stable_pages(), 0);
    assert_eq!(latest.end().wasm_pages(), 3);
    assert_eq!(latest.end().stable_pages(), 3);
    assert_eq!(memory.maximum_wasm_growth_pages(), Some(2));
    assert_eq!(memory.maximum_stable_growth_pages(), Some(3));
}

#[test]
fn ordinary_callback_borrow_failure_stops_without_invoking_or_measuring_work() {
    for (recurring, lifetime) in [
        (false, DeclarationLifetime::Retained),
        (false, DeclarationLifetime::RemoveWhenStopped),
        (true, DeclarationLifetime::Retained),
        (true, DeclarationLifetime::RemoveWhenStopped),
    ] {
        let _fixture = setup();
        let timer = identity("ordinary-borrow-failure");
        let calls = Rc::new(Cell::new(0));
        let callback_calls = Rc::clone(&calls);
        let callback = if recurring {
            erase_ordinary_callback(
                move |_context| {
                    callback_calls.set(callback_calls.get() + 1);
                    async {
                        AfterCompletionRunResult::new(
                            TimerCompletion::success(1),
                            AfterCompletionDecision::RecurAfterCompletion,
                        )
                    }
                },
                AfterCompletionContext::new,
                OrdinaryRunResult::from,
            )
        } else {
            erase_ordinary_callback(
                move |_context| {
                    callback_calls.set(callback_calls.get() + 1);
                    async { OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop) }
                },
                OnceContext::new,
                OrdinaryRunResult::from,
            )
        };
        let claim = with_registry_mut(|registry| {
            let registered = if recurring {
                registry.register_after_completion_with_callback(
                    timer.clone(),
                    test_cadence(5),
                    lifetime,
                    Rc::clone(&callback),
                )
            } else {
                registry.register_once_with_callback(timer.clone(), lifetime, Rc::clone(&callback))
            };
            registered.map_err(TimerError::from)
        })
        .unwrap();
        if recurring {
            ensure_recurring_claim(Declared(&claim)).unwrap();
        } else {
            ensure_once_claim(Declared(&claim), TimerSchedule::At(15)).unwrap();
        }
        assert_eq!(timer_count(), 1);

        // Keep the callback unavailable while dispatch accepts and completes work.
        let _borrow = callback.borrow_mut();
        set_time(15);
        assert!(run_next_due());
        assert_eq!(calls.get(), 0);
        assert_eq!(timer_count(), 0);
        assert!(!run_next_due());

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
            assert_counters!(
                counters,
                (work_started, 1),
                (work_completed, 1),
                (invariant_failure, 1)
            );
            let performance = snapshot.observability().performance();
            assert_eq!(performance.work_instructions().aggregate().samples(), 0);
            assert_eq!(performance.work_memory_pages().samples(), 0);
            assert!(!has_armed_wakeup_claim(&claim).unwrap());
        } else {
            assert!(snapshot.is_none());
            assert!(matches!(
                has_armed_wakeup_claim(&claim),
                Err(TimerError::RegistrationExpired)
            ));
        }
    }
}

#[test]
fn after_completion_recurrence_follows_returned_completion_classification() {
    for completion in [
        TimerCompletion::success(3),
        TimerCompletion::no_work(),
        TimerCompletion::retryable_failure(2),
        TimerCompletion::invariant_failure(4),
    ] {
        let _fixture = setup();
        let timer = identity("classified-recurrence");
        let registration = register_after_completion(
            timer.clone(),
            test_cadence(5),
            TimerStart::Retained { initial: None },
            move |_| async move {
                AfterCompletionRunResult::new(
                    completion,
                    AfterCompletionDecision::RecurAfterCompletion,
                )
            },
        )
        .unwrap();
        registration.ensure_scheduled().unwrap();
        set_time(15);
        assert!(run_next_due());
        let snapshot = registered_snapshot(&timer);
        let invariant = completion.outcome() == crate::TimerCompletionOutcome::InvariantFailure;
        assert_eq!(
            snapshot.next_deadline_ns(),
            if invariant { None } else { Some(20) }
        );
        assert_eq!(registration.has_armed_wakeup().unwrap(), !invariant);
        assert_eq!(timer_count(), usize::from(!invariant));
        let outcomes = snapshot.observability().outcomes();
        assert_eq!(
            outcomes.last_outcome(),
            Some(TimerLastOutcome::Completed(completion.outcome()))
        );
        // Outcome history, failure streaks, counters and directive normalization
        // have owner-local matrices; this boundary checks real provider effects.
    }
}

#[test]
fn after_completion_rearms_from_actual_completion_time() {
    let _fixture = setup();
    let timer = identity("after-completion");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_after_completion(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        move |_context| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            async move {
                let directive = if call == 1 {
                    AfterCompletionDecision::RecurAfterCompletion
                } else {
                    AfterCompletionDecision::Stop
                };
                AfterCompletionRunResult::new(TimerCompletion::success(1), directive)
            }
        },
    )
    .expect("registration should succeed");

    registration
        .ensure_scheduled()
        .expect("initial ensure should succeed");
    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    assert!(!run_next_due());

    set_time(20);
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
    let snapshot = registered_snapshot(&timer);
    let snapshot_counters = snapshot.observability().counters();
    assert_counters!(snapshot_counters, (wakeups_armed, 2), (work_completed, 2));
}

#[test]
fn after_completion_context_can_restore_recurrence_after_nested_cancel() {
    let _fixture = setup();
    let timer = identity("after-context");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_after_completion(
        timer.clone(),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        move |context: AfterCompletionContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            async move {
                if call == 1 {
                    context
                        .cancel()
                        .expect("nested cancellation should succeed");
                    context
                        .ensure_scheduled()
                        .expect("later nested ensure should succeed");
                }
                AfterCompletionRunResult::new(
                    TimerCompletion::no_work(),
                    AfterCompletionDecision::Stop,
                )
            }
        },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial ensure should succeed");

    assert_nested_rearm(&timer, &calls, 20);
}

#[test]
fn once_context_can_restore_scheduling_after_nested_cancel() {
    let _fixture = setup();
    let timer = identity("nested");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        move |context: OnceContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            async move {
                if call == 1 {
                    context
                        .cancel()
                        .expect("nested cancellation should succeed");
                    context
                        .ensure_scheduled(TimerSchedule::At(30))
                        .expect("later nested ensure should succeed");
                }
                OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
            }
        },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("initial ensure should succeed");

    assert_nested_rearm(&timer, &calls, 30);
}

#[track_caller]
fn assert_nested_rearm(timer: &TimerIdentity, calls: &Cell<u64>, deadline_ns: u64) {
    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    assert_eq!(
        registered_snapshot(timer).next_deadline_ns(),
        Some(deadline_ns)
    );
    set_time(deadline_ns);
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn retained_once_context_expires_after_its_work_attempt() {
    let _fixture = setup();
    let timer = identity("ordinary-context-expiry");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let retained_context = Rc::new(RefCell::new(None));
    let callback_context = Rc::clone(&retained_context);
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        move |context: OnceContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            let directive = if call == 1 {
                *callback_context.borrow_mut() = Some(context);
                OnceDecision::ContinueImmediately
            } else {
                OnceDecision::Stop
            };
            async move { OnceRunResult::new(TimerCompletion::no_work(), directive) }
        },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("initial ensure should succeed");

    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(
        timer_count(),
        1,
        "completion should arm the next generation"
    );

    let expired = retained_context
        .borrow_mut()
        .take()
        .expect("first callback should retain its context");
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
        expired.ensure_scheduled(TimerSchedule::At(30)),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        expired.reconcile_schedule(None),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(
        timer_count(),
        1,
        "expired context must not clear or replace the next generation"
    );

    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}
