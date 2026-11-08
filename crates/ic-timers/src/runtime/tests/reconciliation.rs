use super::*;

fn assert_reconciliation_conflict_preserves_runtime(
    result: &Result<(), TimerError>,
    before: &TimerInventorySnapshot,
) {
    assert!(matches!(result, Err(TimerError::ReconciliationConflict)));
    assert_eq!(&timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
}

fn assert_original_callback_completes(calls: &Cell<u64>, now_ns: u64) {
    set_time(now_ns);
    assert!(run_next_due());
    assert_eq!(
        calls.get(),
        1,
        "the original callback must remain installed"
    );
    assert_eq!(timer_count(), 0);
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Conflict {
    Identity,
    Lifetime,
    Cadence,
}

fn assert_conflict_keeps_original_work(conflict: Conflict) {
    let _fixture = setup();
    let timer = identity("reconciliation-conflict");
    let cadence = TimerCadence::from_nanos(5).unwrap();
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut slot = if conflict == Conflict::Identity {
        let registration = register_once(
            timer.clone(),
            TimerStart::Retained { initial: None },
            move |_| {
                callback_calls.set(callback_calls.get() + 1);
                async { OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop) }
            },
        )
        .unwrap();
        registration
            .ensure_scheduled(TimerSchedule::At(20))
            .unwrap();
        ReconciliationSlot::Once(Some(registration))
    } else {
        let start = if conflict == Conflict::Lifetime {
            TimerStart::RemoveWhenStopped {
                initial: TimerSchedule::After(Duration::from_nanos(5)),
            }
        } else {
            TimerStart::Retained { initial: None }
        };
        let registration = register_after_completion(timer.clone(), cadence, start, move |_| {
            callback_calls.set(callback_calls.get() + 1);
            async {
                AfterCompletionRunResult::new(
                    TimerCompletion::success(1),
                    AfterCompletionDecision::Stop,
                )
            }
        })
        .unwrap();
        if conflict == Conflict::Cadence {
            registration.ensure_scheduled().unwrap();
        }
        ReconciliationSlot::AfterCompletion(Some(registration))
    };
    let before = timer_inventory().unwrap();
    let requested_identity = if conflict == Conflict::Identity {
        identity("other-owner")
    } else {
        timer.clone()
    };
    let requested_cadence = if conflict == Conflict::Cadence {
        TimerCadence::from_nanos(6).unwrap()
    } else {
        cadence
    };
    let ignored_calls = Rc::new(Cell::new(0));
    let rejected = slot.reconcile(
        &requested_identity,
        requested_cadence,
        false,
        (),
        Rc::clone(&ignored_calls),
    );
    assert_reconciliation_conflict_preserves_runtime(&rejected, &before);
    let original = slot.claim().unwrap();
    assert_eq!(original.identity(), &timer);
    assert!(has_armed_wakeup_claim(original).unwrap());
    set_time(14);
    assert!(!run_next_due());
    assert_eq!(calls.get(), 0);
    assert_original_callback_completes(
        &calls,
        if conflict == Conflict::Identity {
            20
        } else {
            15
        },
    );
    assert_eq!(ignored_calls.get(), 0);
    if conflict == Conflict::Lifetime {
        assert!(timer_snapshot(&timer).unwrap().is_none());
    }
}

#[test]
fn once_reconciliation_rejects_identity_mismatch_without_disturbing_live_work() {
    assert_conflict_keeps_original_work(Conflict::Identity);
}

#[test]
fn after_completion_reconciliation_rejects_transient_lifetime_without_cancelling_work() {
    assert_conflict_keeps_original_work(Conflict::Lifetime);
}

#[test]
fn after_completion_reconciliation_rejects_cadence_mismatch_without_disturbing_live_work() {
    assert_conflict_keeps_original_work(Conflict::Cadence);
}

#[test]
fn watchdog_reconciliation_rejects_reused_identity_claim_without_clearing_replacement() {
    let _fixture = setup();
    let timer = identity("reconcile-watchdog-expired");
    let cadence = TimerCadence::from_nanos(5).unwrap();
    let calls = Rc::new(Cell::new(0_u64));
    let expired_calls = Rc::clone(&calls);
    let mut expired = Some(
        register_watchdog(
            timer.clone(),
            cadence,
            TimerStart::RemoveWhenStopped {
                initial: TimerSchedule::After(Duration::from_nanos(5)),
            },
            move |_| {
                expired_calls.set(expired_calls.get() + 1);
                WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
            },
        )
        .unwrap(),
    );
    expired.as_ref().unwrap().cancel().unwrap();
    assert!(timer_snapshot(&timer).unwrap().is_none());
    let replacement_count = Rc::new(Cell::new(0_u64));
    let replacement_calls = Rc::clone(&replacement_count);
    let replacement = register_watchdog(
        timer.clone(),
        cadence,
        TimerStart::Retained { initial: None },
        move |_| {
            replacement_calls.set(replacement_calls.get() + 1);
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
        },
    )
    .unwrap();
    replacement.ensure_scheduled().unwrap();
    let before = timer_inventory().unwrap();

    let rejected = reconcile_watchdog(
        &mut expired,
        &timer,
        cadence,
        WatchdogReconcileState::Inactive,
        |_| {
            WatchdogRunResult::new(
                TimerCompletion::invariant_failure(0),
                WatchdogDecision::Stop,
            )
        },
    );
    assert!(matches!(rejected, Err(TimerError::RegistrationExpired)));
    assert!(matches!(
        expired.as_ref().unwrap().has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    assert!(replacement.has_armed_wakeup().unwrap());

    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 0, "expired work must not execute");
    assert_eq!(
        replacement_count.get(),
        1,
        "replacement work must execute once"
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn after_completion_reconstruction_reuses_its_exact_claim() {
    let _fixture = setup();
    let timer = identity("after-reconstruct");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = None;
    reconcile_after_completion(
        &mut registration,
        &timer,
        cadence,
        TimerReconcileState::Scheduled,
        move |_context| {
            callback_calls.set(callback_calls.get().saturating_add(1));
            async {
                AfterCompletionRunResult::new(
                    TimerCompletion::no_work(),
                    AfterCompletionDecision::Stop,
                )
            }
        },
    )
    .expect("fresh reconstruction should succeed");
    reconcile_after_completion(
        &mut registration,
        &timer,
        cadence,
        TimerReconcileState::Scheduled,
        |_context| async {
            AfterCompletionRunResult::new(
                TimerCompletion::invariant_failure(0),
                AfterCompletionDecision::Stop,
            )
        },
    )
    .expect("repeated reconstruction should coalesce");
    registration
        .as_ref()
        .expect("reconstruction should retain its claim")
        .reconcile_schedule(Some(TimerSchedule::At(25)))
        .expect("ordinary reconciliation should replace a later deadline exactly");
    assert_eq!(timer_count(), 1);
    set_time(25);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
}

#[test]
fn once_reconciliation_owns_one_exact_deadline_and_retains_its_callback() {
    let _fixture = setup();
    let timer = identity("once-reconstruct");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = None;

    reconcile_once(
        &mut registration,
        &timer,
        Some(TimerSchedule::At(20)),
        stopping_once(move || {
            callback_calls.set(callback_calls.get().saturating_add(1));
        }),
    )
    .expect("fresh reconstruction should register and arm");
    reconcile_once(
        &mut registration,
        &timer,
        Some(TimerSchedule::At(40)),
        |_context| async {
            OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
        },
    )
    .expect("authoritative reconciliation should move the deadline later");
    assert_eq!(timer_count(), 1);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(40)
    );

    reconcile_once(&mut registration, &timer, None, |_context| async {
        OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
    })
    .expect("inactive reconciliation should clear the exact handle");
    assert_eq!(timer_count(), 0);

    registration
        .as_ref()
        .expect("retained claim should remain available")
        .reconcile_schedule(Some(TimerSchedule::At(50)))
        .expect("retained registration should re-arm");
    set_time(50);
    assert!(run_next_due());
    assert_eq!(
        calls.get(),
        1,
        "reconciliation must retain the first callback"
    );
}

#[test]
fn watchdog_exact_deadline_reconstruction_moves_both_directions_and_sleeps() {
    let _fixture = setup();
    let timer = identity("deadline");
    let mut registration = None;
    reconcile_watchdog(
        &mut registration,
        &timer,
        TimerCadence::from_nanos(5).unwrap(),
        WatchdogReconcileState::ScheduledAt(20),
        |_| WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop),
    )
    .unwrap();
    let registration = registration.unwrap();
    let initial = timer_snapshot(&timer).unwrap().unwrap();
    for deadline in [40, 15, 15] {
        registration
            .reconcile_schedule(Some(TimerSchedule::At(deadline)))
            .unwrap();
        let current = timer_snapshot(&timer).unwrap().unwrap();
        assert_eq!(current.next_deadline_ns(), Some(deadline));
        assert_eq!(current.registration_id(), initial.registration_id());
        assert_eq!(timer_count(), 1);
    }
    let scheduled = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(
        scheduled
            .observability()
            .counters()
            .wakeups_armed()
            .exact_total()
            .unwrap(),
        3
    );
    assert_eq!(
        scheduled
            .observability()
            .counters()
            .coalesced()
            .exact_total()
            .unwrap(),
        1
    );
    assert!(!run_next_due());
    registration.reconcile_schedule(None).unwrap();
    assert_eq!(timer_count(), 0);
    registration
        .reconcile_schedule(Some(TimerSchedule::At(0)))
        .unwrap();
    assert_eq!(
        timer_snapshot(&timer)
            .unwrap()
            .unwrap()
            .latest_armed_delay_ns(),
        Some(0)
    );
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().registration_id(),
        initial.registration_id()
    );
}

#[test]
fn watchdog_deadline_decision_replaces_only_the_prearmed_successor() {
    let _fixture = setup();
    let timer = identity("deadline-result");
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        move |_| {
            callback_calls.set(callback_calls.get() + 1);
            assert_eq!(timer_count(), 1);
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                if callback_calls.get() == 1 {
                    WatchdogDecision::ScheduleAt(100)
                } else {
                    WatchdogDecision::Stop
                },
            )
        },
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    assert!(run_next_due());
    let completed = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(completed.next_deadline_ns(), Some(100));
    assert_eq!(completed.scheduling_mode(), TimerSchedulingMode::Deadline);
    assert_eq!(completed.latest_requested_delay_ns(), None);
    assert_eq!(completed.latest_armed_delay_ns(), Some(90));
    assert_eq!(timer_count(), 1);
    set_time(99);
    assert!(!run_next_due());
    set_time(100);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_dispatched_reconciliation_preserves_recovery_until_completion() {
    let _fixture = setup();
    let timer = identity("dispatched-deadline");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        |_| WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop),
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    registration
        .reconcile_schedule(Some(TimerSchedule::At(100)))
        .unwrap();
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().next_deadline_ns(),
        Some(15)
    );
    assert_eq!(timer_count(), 2);
    assert!(run_next_due());
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().next_deadline_ns(),
        Some(100)
    );
    assert_eq!(timer_count(), 1);
    registration.cancel().unwrap();
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_deadline_context_arbitrates_and_expires() {
    let _fixture = setup();
    let timer = identity("deadline-context");
    let saved = Rc::new(RefCell::new(None));
    let callback_saved = Rc::clone(&saved);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        move |context| {
            context.cancel().unwrap();
            context.ensure_scheduled_immediately().unwrap();
            context
                .reconcile_schedule(Some(TimerSchedule::After(Duration::from_nanos(30))))
                .unwrap();
            context.ensure_scheduled().unwrap();
            *callback_saved.borrow_mut() = Some(context);
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
        },
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    assert!(run_next_due());
    let current = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(current.next_deadline_ns(), Some(40));
    assert_eq!(current.latest_requested_delay_ns(), Some(30));
    assert!(matches!(
        saved
            .borrow()
            .as_ref()
            .unwrap()
            .reconcile_schedule(Some(TimerSchedule::At(60))),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        saved.borrow().as_ref().unwrap().reconcile_schedule(None),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(timer_count(), 1);
}

#[test]
fn invalid_watchdog_deadline_request_preserves_the_live_schedule() {
    let _fixture = setup();
    let timer = identity("invalid-deadline");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        |_| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .unwrap();
    registration.ensure_scheduled().unwrap();
    let before = timer_snapshot(&timer).unwrap();
    for delay in [
        Duration::from_secs(u64::MAX),
        Duration::from_nanos(u64::MAX),
    ] {
        assert!(matches!(
            registration.reconcile_schedule(Some(TimerSchedule::After(delay))),
            Err(TimerError::Schedule(_))
        ));
        assert_eq!(timer_snapshot(&timer).unwrap(), before);
        assert_eq!(timer_count(), 1);
    }
}

#[test]
fn transient_watchdog_deadline_cancellation_clears_both_owned_handles() {
    for dispatched in [false, true] {
        let _fixture = setup();
        let timer = identity("transient-deadline");
        let registration = register_watchdog(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            TimerStart::RemoveWhenStopped {
                initial: TimerSchedule::At(10),
            },
            |_| panic!("cancelled work must not execute"),
        )
        .unwrap();
        if dispatched {
            assert!(run_next_due());
        }
        assert_eq!(timer_count(), if dispatched { 2 } else { 1 });
        registration.reconcile_schedule(None).unwrap();
        assert!(timer_snapshot(&timer).unwrap().is_none());
        assert_eq!(timer_count(), 0);
        assert!(matches!(
            registration.reconcile_schedule(Some(TimerSchedule::At(20))),
            Err(TimerError::RegistrationExpired)
        ));
    }
}

#[test]
fn recovery_retires_an_interrupted_attempts_exact_deadline_proposal() {
    let _fixture = setup();
    let timer = identity("retired-deadline");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        |_| WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue),
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    let before = timer_snapshot(&timer).unwrap().unwrap();
    registration
        .reconcile_schedule(Some(TimerSchedule::At(100)))
        .unwrap();
    assert!(discard_next_due());
    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    let recovered = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(recovered.next_deadline_ns(), Some(20));
    assert_eq!(
        recovered
            .observability()
            .counters()
            .unacknowledged()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(recovered.registration_id(), before.registration_id());
    assert_eq!(timer_count(), 1);
}

#[test]
fn coalesced_watchdog_requests_preserve_paired_handles_without_reinstallation() {
    let _fixture = setup();
    let timer = identity("paired-request-preservation");
    let registration = register_watchdog(
        timer,
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        |context| {
            ProviderFault::BINDING.inject();
            context.ensure_scheduled().unwrap();
            context.ensure_scheduled_immediately().unwrap();
            // The pending immediate request would replace the successor; Stop
            // via exact cancellation wins and needs no handle installation.
            context.cancel().unwrap();
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
        },
    )
    .unwrap();
    registration.ensure_scheduled().unwrap();
    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    ProviderFault::BINDING.inject();
    registration.ensure_scheduled().unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert_eq!(timer_count(), 2);
    // Read/reset the injection so actual work can complete without an unrelated
    // injected failure; a no-op demand must not have consumed it.
    assert!(take_provider_install_fault());
    assert!(run_next_due());
    assert!(take_provider_install_fault());
    assert_eq!(timer_count(), 0);
    registration.unregister().unwrap();
}
