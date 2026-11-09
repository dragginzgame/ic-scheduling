use super::*;

#[test]
fn running_watchdog_unregistration_clears_its_committed_successor() {
    let mut registry = registry();
    let timer = identity("unregister-watchdog");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let initial = registry
        .ensure_recurring(&claim, 0)
        .expect("ensure should succeed");
    confirm(&mut registry, &initial);
    let (scheduler, _) = arm(initial);
    let dispatched = registry.begin_watchdog_scheduler(&scheduler, 10);
    confirm(&mut registry, &dispatched);
    let (_successor, work) = dispatch(dispatched);
    assert!(registry.begin_watchdog_work(&work).is_some());
    assert_eq!(
        registry
            .unregister(&claim)
            .expect("running unregistration should defer")
            .effect(),
        &RegistryEffect::None
    );
    assert_eq!(
        registry.cancel(&claim).unwrap().effect(),
        &RegistryEffect::None,
        "cancellation must preserve pending unregistration and its successor"
    );
    assert_eq!(
        registry
            .snapshot(&timer)
            .unwrap()
            .observability()
            .counters()
            .cancelled()
            .exact_total()
            .unwrap(),
        0,
        "pending cancellation must not count as an immediate stop"
    );
    let completed = registry
        .complete_watchdog_work(
            &work,
            11,
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue),
        )
        .expect("normal completion should finalize removal");
    assert_eq!(
        completed.effect(),
        &RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Wakeup,
        }
    );
    assert!(registry.snapshot(&timer).is_none());
}

#[test]
fn watchdog_completion_arbitrates_immediate_cancellation_and_unregistration() {
    #[derive(Clone, Copy, Debug)]
    enum Pending {
        None,
        Cancel,
        Unregister,
    }

    let mut registry = registry();
    for pending in [Pending::None, Pending::Cancel, Pending::Unregister] {
        let timer = identity(&format!("watchdog-precedence-{pending:?}"));
        let claim = registry
            .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
            .expect("claim should succeed");
        let (scheduler, _) = arm(registry
            .ensure_recurring(&claim, 10)
            .expect("ensure should succeed"));
        let (cadence_successor, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 20));
        assert!(registry.begin_watchdog_work(&work).is_some());
        if !matches!(pending, Pending::None) {
            registry.ensure_watchdog_immediately(&claim, 21).unwrap();
        }
        match pending {
            Pending::None => {}
            Pending::Cancel => {
                registry.cancel(&claim).unwrap();
            }
            Pending::Unregister => {
                registry.unregister(&claim).unwrap();
                registry
                    .ensure_watchdog_immediately(&claim, 21)
                    .expect("unregistration should remain sticky");
            }
        }
        let completed = registry
            .complete_watchdog_work(
                &work,
                21,
                WatchdogRunResult::new(
                    TimerCompletion::success(1),
                    WatchdogDecision::ContinueImmediately,
                ),
            )
            .expect("pending control should arbitrate continuation");
        match pending {
            Pending::None => {
                assert!(matches!(
                    completed.effect(),
                    RegistryEffect::ArmWakeup {
                        delay_ns: 0,
                        arm: WakeupArm::Replacement,
                        ..
                    }
                ));
                let (successor, _) = arm(completed);
                assert_eq!(scheduled_deadline(&registry, &successor), 21);
                assert_ne!(
                    successor.callback_generation(),
                    cadence_successor.callback_generation()
                );
            }
            Pending::Cancel | Pending::Unregister => {
                assert!(matches!(
                    completed.effect(),
                    RegistryEffect::ClearCallbacks {
                        handles: CallbacksToClear::Wakeup,
                        ..
                    }
                ));
                let expected = matches!(pending, Pending::Cancel).then_some(
                    TimerRuntimeStateSnapshot::Inactive {
                        reason: InactiveReason::Cancelled,
                    },
                );
                assert_eq!(
                    registry.snapshot(&timer).map(|snapshot| snapshot.state()),
                    expected
                );
            }
        }
    }
}

#[test]
fn watchdog_terminal_cancellation_makes_queued_callbacks_stale() {
    let mut registry = registry();
    let timer = identity("watchdog-cancel");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let (scheduler, _) = arm(registry
        .ensure_recurring(&claim, 0)
        .expect("ensure should succeed"));
    let before = registry.inventory();
    for schedule in [
        None,
        Some(TimerSchedule::At(20)),
        Some(TimerSchedule::After(Duration::MAX)),
    ] {
        assert_eq!(
            registry.reconcile_ordinary(&claim, 0, schedule).map(|_| ()),
            Err(RegistryError::PolicyMismatch { actual: "watchdog" })
        );
        assert_eq!(registry.inventory(), before);
    }
    let (successor, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 10));
    let cancelled = registry.cancel(&claim).expect("cancel should succeed");
    assert_eq!(
        cancelled.effect(),
        &RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::WakeupAndWork,
        }
    );
    assert!(registry.begin_watchdog_work(&work).is_none());
    assert_eq!(
        registry.begin_watchdog_scheduler(&successor, 20).effect(),
        &RegistryEffect::None
    );
    assert_eq!(
        registry
            .cancel(&claim)
            .expect("repeated cancellation should be idempotent")
            .effect(),
        &RegistryEffect::None
    );

    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(
        snapshot.registration_status(),
        TimerRegistrationStatus::Unregistered
    );
    let counters = snapshot.observability().counters();
    assert_eq!(counters.cancelled().exact_total().unwrap(), 1);
    assert_eq!(counters.stale_work().exact_total().unwrap(), 1);
    assert_eq!(counters.stale_wakeups().exact_total().unwrap(), 1);

    let (rearmed, _) = arm(registry.ensure_recurring(&claim, 20).unwrap());
    assert_eq!(
        rearmed.callback_generation(),
        successor.callback_generation() + 1
    );
    assert_eq!(
        registry.begin_watchdog_scheduler(&successor, 25).effect(),
        &RegistryEffect::None
    );
    let (next_successor, next_work) = dispatch(registry.begin_watchdog_scheduler(&rearmed, 25));
    assert_eq!(
        next_work.callback_generation(),
        next_successor.callback_generation()
    );
    assert!(next_work.callback_generation() > work.callback_generation());
    assert!(registry.begin_watchdog_work(&work).is_none());
    assert!(registry.begin_watchdog_work(&next_work).is_some());
}

#[test]
fn watchdog_result_matrix_tracks_retry_stop_and_invariant_failure() {
    let mut registry = registry();
    let timer = identity("watchdog-results");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let initial = registry
        .ensure_recurring(&claim, 0)
        .expect("ensure should succeed");
    confirm(&mut registry, &initial);
    let (scheduler, _) = arm(initial);
    let dispatched = registry.begin_watchdog_scheduler(&scheduler, 10);
    confirm(&mut registry, &dispatched);
    let (successor, work) = dispatch(dispatched);
    assert!(registry.begin_watchdog_work(&work).is_some());
    registry
        .complete_watchdog_work(
            &work,
            11,
            WatchdogRunResult::new(
                TimerCompletion::retryable_failure(1),
                WatchdogDecision::Continue,
            ),
        )
        .expect("retryable continuation should succeed");
    assert_eq!(registry.consecutive_expected_failures(&timer), Some(1));
    assert_eq!(
        registry
            .snapshot(&timer)
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(15),
        "ordinary Continue must retain the cadence successor unchanged"
    );

    let dispatched = registry.begin_watchdog_scheduler(&successor, 20);
    confirm(&mut registry, &dispatched);
    let (_next, work) = dispatch(dispatched);
    assert!(registry.begin_watchdog_work(&work).is_some());
    let stopped = registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
        )
        .expect("terminal stop should succeed");
    assert!(matches!(
        stopped.effect(),
        RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Wakeup,
            ..
        }
    ));
    assert_eq!(registry.consecutive_expected_failures(&timer), Some(0));

    let restarted = registry
        .ensure_recurring(&claim, 30)
        .expect("retained watchdog should restart");
    confirm(&mut registry, &restarted);
    let (scheduler, _) = arm(restarted);
    let dispatched = registry.begin_watchdog_scheduler(&scheduler, 40);
    confirm(&mut registry, &dispatched);
    let (_successor, work) = dispatch(dispatched);
    assert!(registry.begin_watchdog_work(&work).is_some());
    registry
        .complete_watchdog_work(
            &work,
            41,
            WatchdogRunResult::new(
                TimerCompletion::invariant_failure(2),
                WatchdogDecision::Continue,
            ),
        )
        .expect("invariant failure should stop");
    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    let counters = snapshot.observability().counters();
    assert_eq!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::InvariantFailure,
        }
    );
    assert_eq!(counters.retryable_failure().exact_total().unwrap(), 1);
    assert_eq!(counters.no_work().exact_total().unwrap(), 1);
    assert_eq!(counters.invariant_failure().exact_total().unwrap(), 1);
}

#[test]
fn watchdog_nested_cancel_then_ensure_retains_committed_successor() {
    let mut registry = registry();
    let timer = identity("watchdog-nested");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let (scheduler, _) = arm(registry
        .ensure_recurring(&claim, 0)
        .expect("ensure should succeed"));
    let (_successor, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 10));
    assert!(registry.begin_watchdog_work(&work).is_some());
    registry
        .cancel(&claim)
        .expect("nested cancel should succeed");
    registry
        .ensure_recurring(&claim, 11)
        .expect("later ensure should succeed");
    registry
        .complete_watchdog_work(
            &work,
            12,
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
        )
        .expect("later ensure should override stop");

    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert!(matches!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled { .. })
    ));
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .cancelled()
            .exact_total()
            .unwrap(),
        0
    );
}

#[test]
fn watchdog_checked_generation_exhaustion_is_terminal_and_atomic() {
    let mut registry = registry();
    let timer = identity("watchdog-overflow");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(1), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    {
        let entry = registry
            .entries
            .get_mut(&timer)
            .expect("fixture entry should exist");
        let EntryKind::Watchdog { control, .. } = &mut entry.kind else {
            panic!("fixture should be watchdog control");
        };
        control.set_generation_for_test(u64::MAX);
    }
    let transition = registry
        .ensure_recurring(&claim, 0)
        .expect("overflow is a terminal transition");
    assert_eq!(
        transition.failure(),
        Some(TimerControlFailure::GenerationExhausted)
    );
    assert_eq!(
        registry.snapshot(&timer).map(|value| value.state()),
        Some(TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::ControlFailure(TimerControlFailure::GenerationExhausted,),
        })
    );

    for (name, now_ns) in [
        ("watchdog-dispatch-overflow", 1),
        ("watchdog-combined-overflow", u64::MAX),
    ] {
        let dispatch_timer = identity(name);
        let dispatch_claim = registry
            .register_watchdog(
                dispatch_timer.clone(),
                cadence(1),
                DeclarationLifetime::Retained,
            )
            .unwrap();
        let EntryKind::Watchdog { control, .. } =
            &mut registry.entries.get_mut(&dispatch_timer).unwrap().kind
        else {
            panic!("fixture should be watchdog control");
        };
        control.set_generation_for_test(u64::MAX - 1);
        let (scheduler, _) = arm(registry.ensure_recurring(&dispatch_claim, 0).unwrap());
        assert_eq!(scheduler.callback_generation(), u64::MAX);
        let transition = registry.begin_watchdog_scheduler(&scheduler, now_ns);
        assert_eq!(
            transition.failure(),
            Some(TimerControlFailure::GenerationExhausted)
        );
        assert!(matches!(
            transition.effect(),
            RegistryEffect::ClearCallbacks {
                handles: CallbacksToClear::Work,
                ..
            }
        ));
        let EntryKind::Watchdog { control, .. } =
            &registry.entries.get(&dispatch_timer).unwrap().kind
        else {
            panic!("retained fixture should remain watchdog control");
        };
        assert_eq!(
            control.generation(),
            u64::MAX,
            "failed dispatch must not advance its generation"
        );
        assert_eq!(
            registry.snapshot(&dispatch_timer).unwrap().state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::ControlFailure(TimerControlFailure::GenerationExhausted),
            }
        );
    }
}

#[test]
fn watchdog_checked_deadline_overflow_is_terminal() {
    let mut registry = registry();
    let deadline_timer = identity("watchdog-deadline-overflow");
    let deadline_claim = registry
        .register_watchdog(
            deadline_timer.clone(),
            cadence(1),
            DeclarationLifetime::Retained,
        )
        .expect("deadline-overflow claim should succeed");
    let initial = registry
        .ensure_recurring(&deadline_claim, 0)
        .expect("initial deadline-overflow ensure should succeed");
    let (scheduler, _) = arm(initial);
    // A coalesced request must not compute a fresh cadence deadline. Only
    // actual dispatch needs that successor and must report overflow below.
    assert!(matches!(
        registry
            .ensure_recurring(&deadline_claim, u64::MAX)
            .unwrap()
            .effect(),
        RegistryEffect::None
    ));
    let transition = registry.begin_watchdog_scheduler(&scheduler, u64::MAX);
    assert_eq!(
        transition.failure(),
        Some(TimerControlFailure::DeadlineOverflow)
    );
    assert!(matches!(
        transition.effect(),
        RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Work,
            ..
        }
    ));
    assert_eq!(
        registry.snapshot(&deadline_timer).unwrap().state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::ControlFailure(TimerControlFailure::DeadlineOverflow),
        }
    );
}

#[test]
fn transient_watchdog_completion_keeps_retained_or_replaced_successors() {
    for decision in [
        WatchdogDecision::Continue,
        WatchdogDecision::ContinueImmediately,
        WatchdogDecision::ScheduleAt(2),
        WatchdogDecision::ScheduleAt(10),
    ] {
        let mut registry = registry();
        let timer = identity("transient-watchdog-continuation");
        let claim = registry
            .register_watchdog(
                timer.clone(),
                cadence(1),
                DeclarationLifetime::RemoveWhenStopped,
            )
            .unwrap();
        let replaces_successor = matches!(decision, WatchdogDecision::ScheduleAt(10));
        if !replaces_successor {
            let EntryKind::Watchdog { control, .. } =
                &mut registry.entries.get_mut(&timer).unwrap().kind
            else {
                panic!("fixture must be a watchdog");
            };
            control.set_generation_for_test(u64::MAX - 2);
        }
        let (scheduler, _) = arm(registry.ensure_recurring(&claim, 0).unwrap());
        let (successor, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 1));
        assert!(registry.begin_watchdog_work(&work).is_some());

        let transition = registry
            .complete_watchdog_work(
                &work,
                2,
                WatchdogRunResult::new(TimerCompletion::success(1), decision),
            )
            .unwrap();
        assert_eq!(transition.failure(), None);
        if replaces_successor {
            let (replacement, arm) = arm(transition);
            assert_eq!(arm, WakeupArm::Replacement);
            assert!(replacement.callback_generation() > successor.callback_generation());
            assert_eq!(scheduled_deadline(&registry, &replacement), 10);
        } else {
            assert_eq!(successor.callback_generation(), u64::MAX);
            assert_eq!(transition.effect(), &RegistryEffect::None);
            assert_eq!(scheduled_deadline(&registry, &successor), 2);
        }
        assert_eq!(
            registry.declaration_matches(
                &claim,
                TimerPolicy::Watchdog {
                    cadence: cadence(1)
                },
                DeclarationLifetime::RemoveWhenStopped
            ),
            Ok(true)
        );
        assert_eq!(
            registry
                .snapshot(&timer)
                .unwrap()
                .observability()
                .counters()
                .work_completed()
                .exact_total()
                .unwrap(),
            1
        );
    }
}
