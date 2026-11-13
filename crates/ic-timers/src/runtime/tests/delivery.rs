use super::*;

#[test]
fn dropped_ordinary_deliveries_retire_confirmed_scheduled_and_running_work() {
    for after_completion in [false, true] {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            for started in [false, true] {
                let _fixture = setup();
                let timer = identity("dropped-ordinary");
                let gate = SuspendedWork::default();
                let context = Rc::new(RefCell::new(None));
                let captured = Rc::new(());
                let weak = Rc::downgrade(&captured);
                let claim = suspended_ordinary_registration(
                    after_completion,
                    timer.clone(),
                    lifetime,
                    gate.clone(),
                    Rc::clone(&context),
                    captured,
                );
                if lifetime == DeclarationLifetime::Retained {
                    reconcile_ordinary_claim(Declared(&claim), Some(TimerSchedule::At(10)))
                        .unwrap();
                }
                set_time(10);
                if started {
                    assert!(run_next_due());
                    assert_eq!(gate.polls.get(), 1);
                    gate.resume();
                }
                set_time(20);
                // Drop only; the native fake neither traps nor rolls messages back.
                assert!(discard_next_due());
                assert_eq!(timer_count(), 0);
                if let Some(token) = context.borrow().as_ref() {
                    assert!(matches!(
                        cancel_claim(Executing(token)),
                        Err(TimerError::RegistrationExpired)
                    ));
                }
                if lifetime == DeclarationLifetime::Retained {
                    let snapshot = registered_snapshot(&timer);
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::Abandoned,
                        }
                    );
                    assert_eq!(snapshot.next_deadline_ns(), None);
                    assert!(!has_armed_wakeup_claim(&claim).unwrap());
                    let observations = snapshot.observability();
                    let counters = observations.counters();
                    assert_counters!(
                        counters,
                        (unacknowledged, 1),
                        (work_started, u128::from(started)),
                        (work_completed, 0)
                    );
                    let performance = observations.performance();
                    assert_eq!(performance.work_instructions().aggregate().samples(), 0);
                    assert_eq!(
                        observations.outcomes().last_outcome(),
                        Some(TimerLastOutcome::Unacknowledged)
                    );
                    assert_eq!(
                        observations.outcomes().last_unacknowledged_at_ns(),
                        Some(20)
                    );
                    assert!(weak.upgrade().is_some());
                    if !started {
                        gate.ready.set(true);
                    }
                    reconcile_ordinary_claim(Declared(&claim), Some(TimerSchedule::At(30)))
                        .unwrap();
                    set_time(30);
                    assert!(run_next_due());
                    let completed = registered_snapshot(&timer);
                    let counters = completed.observability().counters();
                    assert_counters!(counters, (work_completed, 1), (unacknowledged, 1));
                    unregister_claim(&claim).unwrap();
                } else {
                    assert!(timer_snapshot(&timer).unwrap().is_none());
                    assert!(matches!(
                        has_armed_wakeup_claim(&claim),
                        Err(TimerError::RegistrationExpired)
                    ));
                }
                assert!(
                    weak.upgrade().is_none(),
                    "removed callbacks must release captures"
                );
                assert_eq!(timer_inventory().unwrap().timers(), []);
            }
        }
    }
}

#[test]
fn stale_delivery_drop_cannot_retire_rearmed_or_reused_identity() {
    let _fixture = setup();
    let timer = identity("stale-delivery-drop");
    let registration = retained_once(timer.clone(), || {});
    registration
        .reconcile_schedule(Some(TimerSchedule::At(20)))
        .unwrap();
    let detached = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
        .unwrap()
        .unwrap();
    let (token, handle) = detached;
    bind_provider_handle(&token, handle, ProviderBinding::Stage).unwrap();
    let old_delivery = dispatch_wakeup(WakeupToken::Ordinary(token.clone()));
    registration
        .reconcile_schedule(Some(TimerSchedule::At(30)))
        .unwrap();
    let before = timer_inventory().unwrap();
    drop(old_delivery);
    assert_eq!(timer_inventory().unwrap(), before);
    registration.unregister().unwrap();
    let replacement = retained_once(timer, || {});
    replacement
        .reconcile_schedule(Some(TimerSchedule::At(40)))
        .unwrap();
    let before = timer_inventory().unwrap();
    drop(dispatch_wakeup(WakeupToken::Ordinary(token)));
    assert_eq!(timer_inventory().unwrap(), before);
    assert!(replacement.has_armed_wakeup().unwrap());
    replacement.unregister().unwrap();
}

#[test]
fn abandoned_transient_releases_capacity_and_drops_captures_outside_registry_borrow() {
    struct CaptureDropCheck {
        timer: TimerIdentity,
        dropped: Rc<Cell<bool>>,
    }
    impl Drop for CaptureDropCheck {
        fn drop(&mut self) {
            assert!(timer_snapshot(&self.timer).unwrap().is_none());
            self.dropped.set(true);
        }
    }

    let _fixture = setup();
    let timer = identity("abandoned-capacity");
    let dropped = Rc::new(Cell::new(false));
    let captured = Rc::new(CaptureDropCheck {
        timer: timer.clone(),
        dropped: Rc::clone(&dropped),
    });
    let registration = register_once(
        timer.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::ZERO),
        },
        move |_| {
            let captured = Rc::clone(&captured);
            async move {
                std::hint::black_box(captured);
                OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
            }
        },
    )
    .unwrap();
    let remaining = fill_registry();
    assert_eq!(
        timer_inventory().unwrap().timers().len(),
        crate::MAX_TIMER_REGISTRATIONS
    );
    assert!(discard_next_due());
    assert!(dropped.get());
    let replacement = retained_once(timer, || {});
    assert!(matches!(
        registration.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    replacement.unregister().unwrap();
    for registration in remaining {
        registration.unregister().unwrap();
    }
}

#[test]
fn unconfirmed_ordinary_delivery_drop_preserves_binding_failure_accounting() {
    for after_completion in [false, true] {
        let _fixture = setup();
        let timer = identity("unconfirmed-delivery-drop");
        let claim = suspended_ordinary_registration(
            after_completion,
            timer.clone(),
            DeclarationLifetime::Retained,
            SuspendedWork::default(),
            Rc::new(RefCell::new(None)),
            Rc::new(()),
        );
        ProviderFault::BINDING.inject();
        assert!(reconcile_ordinary_claim(Declared(&claim), Some(TimerSchedule::At(10))).is_err());
        let snapshot = registered_snapshot(&timer);
        assert_eq!(
            snapshot.state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::ControlFailure(TimerControlFailure::ProviderBindingFailed),
            }
        );
        assert_counters!(snapshot.observability().counters(), (unacknowledged, 0));
        assert_eq!(snapshot.observability().outcomes().last_outcome(), None);
        assert_eq!(timer_count(), 0);
        unregister_claim(&claim).unwrap();
    }
}

#[test]
fn suspended_once_work_allows_other_timers_and_arbitrates_external_commands() {
    for command in ["cancel", "reconcile", "ensure", "unregister"] {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            let _fixture = setup();
            let timer = identity("suspended-once");
            let gate = SuspendedWork::default();
            let callback_gate = gate.clone();
            let context_slot = Rc::new(RefCell::new(None));
            let callback_slot = Rc::clone(&context_slot);
            let registration = register_once(
                timer.clone(),
                scheduled_start(lifetime, TimerSchedule::At(15)),
                move |context| {
                    let gate = callback_gate.clone();
                    *callback_slot.borrow_mut() = Some(context);
                    async move {
                        gate.wait().await;
                        let completion = TimerCompletion::success(1);
                        OnceRunResult::new(completion, OnceDecision::ScheduleAt(80))
                    }
                },
            )
            .unwrap();
            let unrelated_calls = Rc::new(Cell::new(0));
            let callback_calls = Rc::clone(&unrelated_calls);
            let _unrelated = register_watchdog(
                identity("unrelated-watchdog"),
                test_cadence(5),
                TimerStart::Retained {
                    initial: Some(TimerSchedule::At(15)),
                },
                move |_| {
                    callback_calls.set(callback_calls.get() + 1);
                    WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
                },
            )
            .unwrap();
            set_time(15);
            assert!(run_next_due()); // Ordinary work suspends without an armed handle.
            assert!(!registration.has_armed_wakeup().unwrap());
            assert_eq!(timer_count(), 1);
            assert!(run_next_due()); // Unrelated scheduler and work must still run.
            assert!(run_next_due());
            assert_eq!(unrelated_calls.get(), 1);
            assert!(!run_next_due());
            assert_eq!(
                gate.polls.get(),
                1,
                "sleeping work must not be polled again"
            );
            match command {
                "cancel" => registration.cancel().unwrap(),
                "reconcile" => {
                    registration.cancel().unwrap();
                    registration
                        .reconcile_schedule(Some(TimerSchedule::At(50)))
                        .unwrap();
                }
                "ensure" => registration
                    .ensure_scheduled(TimerSchedule::At(40))
                    .unwrap(),
                "unregister" => registration.unregister().unwrap(),
                _ => unreachable!("fixed command matrix"),
            }
            set_time(30);
            gate.resume();
            assert!(run_next_due());
            let observed = timer_snapshot(&timer).unwrap();
            match command {
                "reconcile" | "ensure" => {
                    let snapshot = observed.unwrap();
                    let counters = snapshot.observability().counters();
                    let expected_deadline = if command == "ensure" { 40 } else { 50 };
                    assert_eq!(snapshot.next_deadline_ns(), Some(expected_deadline));
                    assert_counters!(counters, (work_completed, 1));
                    assert_eq!(timer_count(), 1);
                }
                "cancel" if lifetime == DeclarationLifetime::Retained => {
                    assert_eq!(
                        observed.unwrap().state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::Cancelled
                        }
                    );
                    assert_eq!(timer_count(), 0);
                }
                _ => {
                    assert!(observed.is_none());
                    assert_eq!(timer_count(), 0);
                }
            }
            assert!(matches!(
                context_slot.borrow().as_ref().unwrap().cancel(),
                Err(TimerError::RegistrationExpired)
            ));
        }
    }
}

#[test]
fn policy_specific_results_schedule_through_live_runtime() {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ResultCase {
        Stop,
        Continue,
        Retry,
        AtDeadline,
        Recur,
        UnrepresentableDelay,
        OverflowingDeadline,
        InvariantFailure,
    }
    use ResultCase::{
        AtDeadline, Continue, InvariantFailure, OverflowingDeadline, Recur, Retry, Stop,
        UnrepresentableDelay,
    };

    for recurring in [false, true] {
        for case in [
            Stop,
            Continue,
            Retry,
            AtDeadline,
            Recur,
            UnrepresentableDelay,
            OverflowingDeadline,
            InvariantFailure,
        ] {
            if !recurring && case == Recur {
                continue;
            }
            for reconcile in [false, true] {
                let _fixture = setup();
                let timer = identity("typed-results");
                let completion = if case == InvariantFailure {
                    TimerCompletion::invariant_failure(2)
                } else {
                    TimerCompletion::retryable_failure(2)
                };
                if recurring {
                    let decision = match case {
                        Stop => AfterCompletionDecision::Stop,
                        Continue | InvariantFailure => AfterCompletionDecision::ContinueImmediately,
                        Retry => AfterCompletionDecision::RetryAfter(Duration::from_nanos(7)),
                        AtDeadline => AfterCompletionDecision::ScheduleAt(50),
                        Recur => AfterCompletionDecision::RecurAfterCompletion,
                        UnrepresentableDelay => AfterCompletionDecision::RetryAfter(Duration::MAX),
                        OverflowingDeadline => {
                            AfterCompletionDecision::RetryAfter(Duration::from_nanos(u64::MAX))
                        }
                    };
                    let registration = register_after_completion(
                        timer.clone(),
                        test_cadence(5),
                        TimerStart::Retained { initial: None },
                        move |context| async move {
                            if reconcile {
                                context
                                    .reconcile_schedule(Some(TimerSchedule::At(100)))
                                    .unwrap();
                            }
                            AfterCompletionRunResult::new(completion, decision)
                        },
                    )
                    .unwrap();
                    registration
                        .reconcile_schedule(Some(TimerSchedule::At(20)))
                        .unwrap();
                } else {
                    let decision = match case {
                        Stop => OnceDecision::Stop,
                        Continue | InvariantFailure => OnceDecision::ContinueImmediately,
                        Retry => OnceDecision::RetryAfter(Duration::from_nanos(7)),
                        AtDeadline => OnceDecision::ScheduleAt(50),
                        UnrepresentableDelay => OnceDecision::RetryAfter(Duration::MAX),
                        OverflowingDeadline => {
                            OnceDecision::RetryAfter(Duration::from_nanos(u64::MAX))
                        }
                        Recur => unreachable!("Once has no configured recurrence"),
                    };
                    let registration = register_once(
                        timer.clone(),
                        TimerStart::Retained { initial: None },
                        move |context| async move {
                            if reconcile {
                                context
                                    .reconcile_schedule(Some(TimerSchedule::At(100)))
                                    .unwrap();
                            }
                            OnceRunResult::new(completion, decision)
                        },
                    )
                    .unwrap();
                    registration
                        .ensure_scheduled(TimerSchedule::At(20))
                        .unwrap();
                }
                set_time(20);
                assert!(run_next_due());
                let snapshot = registered_snapshot(&timer);
                let deadline = if reconcile && case != InvariantFailure {
                    Some(100)
                } else {
                    match case {
                        Continue => Some(20),
                        Retry => Some(27),
                        AtDeadline => Some(50),
                        Recur => Some(25),
                        _ => None,
                    }
                };
                assert_eq!(snapshot.next_deadline_ns(), deadline);
                assert_eq!(timer_count(), usize::from(deadline.is_some()));
                if case == InvariantFailure {
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::InvariantFailure,
                        }
                    );
                } else if !reconcile && matches!(case, UnrepresentableDelay | OverflowingDeadline) {
                    let failure = if case == UnrepresentableDelay {
                        TimerControlFailure::DelayOutOfRange
                    } else {
                        TimerControlFailure::DeadlineOverflow
                    };
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::ControlFailure(failure),
                        }
                    );
                } else if reconcile {
                    assert_eq!(
                        snapshot.latest_directive(),
                        Some(crate::TimerDirectiveSnapshot::ScheduleAt { deadline_ns: 100 })
                    );
                }
                assert_eq!(
                    snapshot.observability().outcomes().last_outcome(),
                    Some(TimerLastOutcome::Completed(
                        if !reconcile && matches!(case, UnrepresentableDelay | OverflowingDeadline)
                        {
                            crate::TimerCompletionOutcome::InvariantFailure
                        } else {
                            completion.outcome()
                        }
                    ))
                );
            }
        }
    }
}

#[test]
fn suspended_after_completion_uses_completion_time_and_exact_reconciliation() {
    for reconcile in [false, true] {
        let _fixture = setup();
        let timer = identity("suspended-after-completion");
        let gate = SuspendedWork::default();
        let callback_gate = gate.clone();
        let registration = register_after_completion(
            timer.clone(),
            test_cadence(5),
            TimerStart::Retained { initial: None },
            move |_| {
                let gate = callback_gate.clone();
                async move {
                    gate.wait().await;
                    AfterCompletionRunResult::new(
                        TimerCompletion::success(1),
                        AfterCompletionDecision::RecurAfterCompletion,
                    )
                }
            },
        )
        .unwrap();
        registration.ensure_scheduled().unwrap();
        set_time(15);
        assert!(run_next_due());
        assert!(!registration.has_armed_wakeup().unwrap());
        assert!(!run_next_due());
        assert_eq!(timer_count(), 0);
        if reconcile {
            registration
                .reconcile_schedule(Some(TimerSchedule::At(50)))
                .unwrap();
        }
        set_time(30);
        gate.resume();
        assert!(run_next_due());
        assert_eq!(
            registered_snapshot(&timer).next_deadline_ns(),
            Some(if reconcile { 50 } else { 35 })
        );
        assert_eq!(timer_count(), 1);
        registration.unregister().unwrap();
        assert_eq!(timer_count(), 0);
    }
}
