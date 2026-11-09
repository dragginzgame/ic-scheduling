use super::*;

struct RemovalCapture {
    timer: TimerIdentity,
    dropped: Rc<Cell<bool>>,
}

impl Drop for RemovalCapture {
    fn drop(&mut self) {
        assert!(timer_snapshot(&self.timer).unwrap().is_none());
        assert_eq!(
            timer_count(),
            0,
            "provider cleanup must precede capture Drop"
        );
        // Reuse the removed identity and capacity from consumer destruction.
        // This also proves nested mutation can run without a registry borrow.
        let replacement = register_once(
            self.timer.clone(),
            TimerStart::Retained { initial: None },
            stopping_once(|| {}),
        )
        .unwrap();
        replacement.ensure_scheduled(TimerSchedule::At(20)).unwrap();
        replacement.unregister().unwrap();
        self.dropped.set(true);
    }
}

#[test]
fn cancellation_and_unregistration_release_captures_after_provider_cleanup() {
    for policy in TestPolicy::ALL {
        for (armed, unregister) in [(false, true), (true, true), (true, false)] {
            let _fixture = setup();
            let timer = identity("normal-removal-capture");
            let dropped = Rc::new(Cell::new(false));
            let captured = RemovalCapture {
                timer: timer.clone(),
                dropped: Rc::clone(&dropped),
            };
            let start = if unregister {
                TimerStart::Retained {
                    initial: armed.then_some(TimerSchedule::At(20)),
                }
            } else {
                TimerStart::RemoveWhenStopped {
                    initial: TimerSchedule::At(20),
                }
            };
            let claim = initial_registration_with_capture(policy, timer, start, captured).unwrap();
            if unregister {
                unregister_claim(&claim).unwrap();
            } else {
                cancel_claim(Declared(&claim)).unwrap();
            }
            assert!(dropped.get());
        }
    }
}

#[test]
fn rejected_registration_releases_its_captures_outside_registry_borrow() {
    struct RejectedCapture(Rc<Cell<bool>>);
    impl Drop for RejectedCapture {
        fn drop(&mut self) {
            assert_eq!(timer_inventory().unwrap().len(), 1);
            self.0.set(true);
        }
    }
    let _fixture = setup();
    let timer = identity("duplicate-capture");
    let original = register_once(
        timer.clone(),
        TimerStart::Retained {
            initial: Some(TimerSchedule::At(20)),
        },
        stopping_once(|| {}),
    )
    .unwrap();
    for policy in TestPolicy::ALL {
        let dropped = Rc::new(Cell::new(false));
        let captured = RejectedCapture(Rc::clone(&dropped));
        let rejected = initial_registration_with_capture(
            policy,
            timer.clone(),
            TimerStart::Retained {
                initial: Some(TimerSchedule::At(20)),
            },
            captured,
        );
        assert!(matches!(
            rejected,
            Err(TimerError::Register(
                RegisterError::IdentityAlreadyRegistered(_)
            ))
        ));
        assert!(dropped.get());
    }
    original.unregister().unwrap();
}

#[test]
fn terminal_provider_and_scheduler_failures_release_captures_outside_registry_borrow() {
    let _fixture = setup();
    let timer = identity("binding-failure-capture");
    let dropped = Rc::new(Cell::new(false));
    let captured = RemovalCapture {
        timer: timer.clone(),
        dropped: Rc::clone(&dropped),
    };
    let registration = register_once(
        timer,
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(30),
        },
        stopping_once(move || {
            std::hint::black_box(&captured);
        }),
    )
    .unwrap();
    ProviderFault::BINDING.inject();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(20)),
        Err(TimerError::OwnershipInvariant)
    ));
    assert!(dropped.get());

    let timer = identity("scheduler-failure-capture");
    let dropped = Rc::new(Cell::new(false));
    let captured = RemovalCapture {
        timer: timer.clone(),
        dropped: Rc::clone(&dropped),
    };
    let _registration = register_watchdog(
        timer,
        test_cadence(5),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(u64::MAX),
        },
        stopping_watchdog(move || {
            std::hint::black_box(&captured);
        }),
    )
    .unwrap();
    set_time(u64::MAX);
    assert!(run_next_due());
    assert!(dropped.get());
    assert_eq!(timer_inventory().unwrap().timers(), []);
}

struct CancelContextOnDrop(Rc<RefCell<Option<OnceContext>>>);

impl Drop for CancelContextOnDrop {
    fn drop(&mut self) {
        self.0.borrow().as_ref().unwrap().cancel().unwrap();
    }
}

#[test]
fn repeated_reconciliation_releases_unused_factory_before_applying_control() {
    let _fixture = setup();
    let timer = identity("unused-factory-ordering");
    let gate = SuspendedWork::default();
    let context = Rc::new(RefCell::new(None));
    let callback_context = Rc::clone(&context);
    let callback_gate = gate.clone();
    let mut slot = Some(
        register_once(
            timer.clone(),
            TimerStart::Retained {
                initial: Some(TimerSchedule::At(10)),
            },
            move |context| {
                *callback_context.borrow_mut() = Some(context);
                let gate = callback_gate.clone();
                async move {
                    gate.wait().await;
                    OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
                }
            },
        )
        .unwrap(),
    );
    assert!(run_next_due());
    let captured = CancelContextOnDrop(context);
    reconcile_once(
        &mut slot,
        &timer,
        Some(TimerSchedule::At(20)),
        stopping_once(move || {
            std::hint::black_box(&captured);
        }),
    )
    .unwrap();
    gate.resume();
    assert!(run_next_due());
    assert_eq!(registered_snapshot(&timer).next_deadline_ns(), Some(20));
    assert_eq!(timer_count(), 1);
    slot.take().unwrap().unregister().unwrap();
}

#[test]
fn failed_first_reconciliation_retires_all_policies_before_retry() {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Rejection {
        DeadlineOverflow,
        ProviderBinding,
    }

    for policy in TestPolicy::ALL {
        for rejection in [Rejection::DeadlineOverflow, Rejection::ProviderBinding] {
            let _fixture = setup();
            let timer = identity("first-reconciliation");
            let dropped = Rc::new(Cell::new(false));
            let captured = RemovalCapture {
                timer: timer.clone(),
                dropped: Rc::clone(&dropped),
            };
            let failed_calls = Rc::new(Cell::new(0));
            let mut slot = ReconciliationSlot::new(policy);
            let cadence = TimerCadence::from_nanos(if rejection == Rejection::DeadlineOverflow {
                u64::MAX
            } else {
                5
            })
            .unwrap();
            if rejection == Rejection::ProviderBinding {
                ProviderFault::BINDING.inject();
            }
            let result = slot.reconcile(
                &timer,
                cadence,
                true,
                Some(captured),
                Rc::clone(&failed_calls),
            );
            assert!(matches!(
                (rejection, result),
                (
                    Rejection::DeadlineOverflow,
                    Err(TimerError::Schedule(ScheduleError::DeadlineOverflow))
                ) | (
                    Rejection::ProviderBinding,
                    Err(TimerError::OwnershipInvariant)
                )
            ));
            assert!(slot.claim().is_none());
            assert!(dropped.get());
            assert_eq!(timer_inventory().unwrap().timers(), []);
            assert_eq!(timer_count(), 0);
            assert!(!run_next_due());

            let retry_calls = Rc::new(Cell::new(0));
            slot.reconcile(&timer, test_cadence(5), true, (), Rc::clone(&retry_calls))
                .unwrap();
            assert!(has_armed_wakeup_claim(slot.claim().unwrap()).unwrap());
            assert_eq!(timer_count(), 1);
            set_time(15);
            while run_next_due() {}
            assert_eq!(retry_calls.get(), 1);
            assert_eq!(failed_calls.get(), 0);
            assert_eq!(timer_count(), 0);
            unregister_claim(slot.claim().unwrap()).unwrap();
        }
    }
}

#[test]
fn first_once_reconciliation_rejects_unencodable_delay_without_retaining_authority() {
    let _fixture = setup();
    let timer = identity("unencodable-reconciliation");
    let dropped = Rc::new(Cell::new(false));
    let captured = RemovalCapture {
        timer: timer.clone(),
        dropped: Rc::clone(&dropped),
    };
    let mut slot = None;
    let result = reconcile_once(
        &mut slot,
        &timer,
        Some(TimerSchedule::After(Duration::MAX)),
        stopping_once(move || {
            std::hint::black_box(&captured);
        }),
    );
    assert!(matches!(
        result,
        Err(TimerError::Schedule(ScheduleError::DelayOutOfRange))
    ));
    assert!(slot.is_none());
    assert!(dropped.get());
    assert_eq!(timer_inventory().unwrap().timers(), []);
    assert_eq!(timer_count(), 0);
}

#[test]
fn later_reconciliation_deadline_error_preserves_existing_inactive_authority() {
    for policy in TestPolicy::ALL {
        let _fixture = setup();
        let timer = identity("later-reconciliation-deadline");
        let cadence = test_cadence(u64::MAX);
        let dropped = Rc::new(Cell::new(false));
        let captured = RemovalCapture {
            timer: timer.clone(),
            dropped: Rc::clone(&dropped),
        };
        let mut slot = ReconciliationSlot::new(policy);
        slot.reconcile(
            &timer,
            cadence,
            false,
            Some(captured),
            Rc::new(Cell::new(0)),
        )
        .unwrap();
        let before = timer_snapshot(&timer).unwrap();
        let result = slot.reconcile(&timer, cadence, true, (), Rc::new(Cell::new(0)));
        assert!(matches!(
            result,
            Err(TimerError::Schedule(ScheduleError::DeadlineOverflow))
        ));
        assert!(slot.claim().is_some());
        assert_eq!(timer_snapshot(&timer).unwrap(), before);
        assert!(!dropped.get());
        assert_eq!(timer_count(), 0);
        unregister_claim(slot.claim().unwrap()).unwrap();
        assert!(dropped.get());
    }
}

#[test]
fn later_reconciliation_provider_error_retains_original_callback_for_retry() {
    for policy in TestPolicy::ALL {
        let _fixture = setup();
        let timer = identity("later-reconciliation-provider");
        let cadence = test_cadence(5);
        let dropped = Rc::new(Cell::new(false));
        let captured = RemovalCapture {
            timer: timer.clone(),
            dropped: Rc::clone(&dropped),
        };
        let original_calls = Rc::new(Cell::new(0));
        let ignored_calls = Rc::new(Cell::new(0));
        let mut slot = ReconciliationSlot::new(policy);
        slot.reconcile(
            &timer,
            cadence,
            false,
            Some(captured),
            Rc::clone(&original_calls),
        )
        .unwrap();
        let original_id = registered_snapshot(&timer).registration_id();
        ProviderFault::BINDING.inject();
        let result = slot.reconcile(&timer, cadence, true, (), Rc::clone(&ignored_calls));
        assert!(matches!(result, Err(TimerError::OwnershipInvariant)));
        assert!(slot.claim().is_some());
        assert!(!dropped.get());
        let failed = registered_snapshot(&timer);
        assert_eq!(failed.registration_id(), original_id);
        assert_eq!(
            failed.state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::ControlFailure(TimerControlFailure::ProviderBindingFailed),
            }
        );
        assert_eq!(timer_count(), 0);
        slot.reconcile(&timer, cadence, true, (), Rc::clone(&ignored_calls))
            .unwrap();
        set_time(15);
        while run_next_due() {}
        assert_eq!(original_calls.get(), 1);
        assert_eq!(ignored_calls.get(), 0);
        unregister_claim(slot.claim().unwrap()).unwrap();
        assert!(dropped.get());
    }
}

fn initial_registration_with_capture<C: 'static>(
    policy: TestPolicy,
    timer: TimerIdentity,
    start: TimerStart,
    captured: C,
) -> Result<RegistrationClaim, TimerError> {
    let work = move || {
        std::hint::black_box(&captured);
    };
    match policy {
        TestPolicy::Once => {
            register_once(timer, start, stopping_once(work)).map(|registration| registration.claim)
        }
        TestPolicy::AfterCompletion => register_after_completion(
            timer,
            test_cadence(5),
            start,
            stopping_after_completion(work),
        )
        .map(|registration| registration.claim),
        TestPolicy::Watchdog => {
            register_watchdog(timer, test_cadence(5), start, stopping_watchdog(work))
                .map(|registration| registration.claim)
        }
    }
}

#[test]
fn initial_registration_confirms_exact_wakeup_before_returning_authority() {
    for policy in TestPolicy::ALL {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            let _fixture = setup();
            let timer = identity("initial-schedule");
            let dropped = Rc::new(Cell::new(false));
            let captured = RemovalCapture {
                timer: timer.clone(),
                dropped: Rc::clone(&dropped),
            };
            let claim = initial_registration_with_capture(
                policy,
                timer.clone(),
                scheduled_start(lifetime, TimerSchedule::At(20)),
                captured,
            )
            .unwrap();
            assert!(has_armed_wakeup_claim(&claim).unwrap());
            assert_eq!(timer_count(), 1);
            let snapshot = registered_snapshot(&timer);
            assert_eq!(snapshot.next_deadline_ns(), Some(20));
            assert_eq!(
                snapshot
                    .observability()
                    .counters()
                    .schedule_requests()
                    .exact_total()
                    .unwrap(),
                1
            );
            assert_eq!(
                snapshot
                    .observability()
                    .counters()
                    .wakeups_armed()
                    .exact_total()
                    .unwrap(),
                1
            );
            assert!(!run_next_due());
            unregister_claim(&claim).unwrap();
            assert!(dropped.get());
        }
    }
}

#[test]
fn rejected_initial_schedule_allocates_no_claim_and_releases_captures() {
    for policy in TestPolicy::ALL {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            for initial in [
                TimerSchedule::After(Duration::MAX),
                TimerSchedule::After(Duration::from_nanos(u64::MAX)),
            ] {
                let _fixture = setup();
                let timer = identity("rejected-initial-schedule");
                let dropped = Rc::new(Cell::new(false));
                let captured = RemovalCapture {
                    timer: timer.clone(),
                    dropped: Rc::clone(&dropped),
                };
                let result = initial_registration_with_capture(
                    policy,
                    timer,
                    scheduled_start(lifetime, initial),
                    captured,
                );
                match initial {
                    TimerSchedule::After(Duration::MAX) => assert!(matches!(
                        result,
                        Err(TimerError::Schedule(ScheduleError::DelayOutOfRange))
                    )),
                    _ => assert!(matches!(
                        result,
                        Err(TimerError::Schedule(ScheduleError::DeadlineOverflow))
                    )),
                }
                assert!(dropped.get());
                assert_eq!(timer_inventory().unwrap().timers(), []);
                assert_eq!(timer_count(), 0);
            }
        }
    }
}

#[test]
fn initial_provider_failures_retire_all_policies_and_lifetimes_before_capture_drop() {
    for policy in TestPolicy::ALL {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            let _fixture = setup();
            let timer = identity("initial-provider-failure");
            let dropped = Rc::new(Cell::new(false));
            let captured = RemovalCapture {
                timer: timer.clone(),
                dropped: Rc::clone(&dropped),
            };
            ProviderFault::BINDING.inject();
            let result = initial_registration_with_capture(
                policy,
                timer,
                scheduled_start(lifetime, TimerSchedule::At(20)),
                captured,
            );
            assert!(matches!(result, Err(TimerError::OwnershipInvariant)));
            assert!(dropped.get());
            assert_eq!(timer_inventory().unwrap().timers(), []);
            assert_eq!(timer_count(), 0);
            assert!(!run_next_due());
        }
    }
}

#[test]
fn backend_admission_failure_retires_initial_claim_before_capture_drop() {
    for policy in TestPolicy::ALL {
        for error in [
            platform::AdmissionError::Capacity,
            platform::AdmissionError::IdentityExhausted,
        ] {
            let _fixture = setup();
            let timer = identity("backend-admission-failure");
            let dropped = Rc::new(Cell::new(false));
            let captured = RemovalCapture {
                timer: timer.clone(),
                dropped: Rc::clone(&dropped),
            };
            ProviderFault::Admission { after: 0, error }.inject();
            let result = initial_registration_with_capture(
                policy,
                timer,
                TimerStart::RemoveWhenStopped {
                    initial: TimerSchedule::At(20),
                },
                captured,
            );
            assert!(matches!(
                (error, result),
                (
                    platform::AdmissionError::Capacity,
                    Err(TimerError::BackendCapacity)
                ) | (
                    platform::AdmissionError::IdentityExhausted,
                    Err(TimerError::BackendIdentityExhausted)
                )
            ));
            assert!(dropped.get());
            assert_eq!(timer_inventory().unwrap().timers(), []);
            assert_eq!(timer_count(), 0);
        }
    }
}

#[test]
fn initial_capacity_rejection_preserves_existing_declarations_and_releases_factory() {
    let _fixture = setup();
    let registrations = fill_registry();
    let before = timer_inventory().unwrap();
    let captured = Rc::new(());
    let weak = Rc::downgrade(&captured);
    let rejected = register_once(
        identity("initial-capacity-rejected"),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(20),
        },
        stopping_once(move || {
            std::hint::black_box(&captured);
        }),
    );
    assert!(matches!(
        rejected,
        Err(TimerError::Register(RegisterError::CapacityExceeded { .. }))
    ));
    assert!(weak.upgrade().is_none());
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 0);
    for registration in registrations {
        registration.unregister().unwrap();
    }
}
