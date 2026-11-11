use super::*;

#[test]
fn inactive_demand_keeps_custody_idle_and_explicit_cancel_distinct() {
    for recurring in [false, true] {
        let mut registry = registry();
        let timer = identity("inactive-demand");
        let claim = if recurring {
            registry.register_after_completion(
                timer.clone(),
                cadence(10),
                DeclarationLifetime::Retained,
            )
        } else {
            registry.register_once(timer.clone(), DeclarationLifetime::Retained)
        }
        .unwrap();
        let original = registry.snapshot(&timer).unwrap().registration_id();
        let scheduled = registry
            .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(10)))
            .unwrap();
        confirm(&mut registry, &scheduled);
        let stopped = registry.reconcile_ordinary(&claim, 1, None).unwrap();
        assert!(matches!(
            stopped.effect(),
            RegistryEffect::ClearCallbacks {
                handles: CallbacksToClear::Wakeup
            }
        ));
        confirm(&mut registry, &stopped);
        let snapshot = registry.snapshot(&timer).unwrap();
        assert_eq!(
            snapshot.state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::Stopped
            }
        );
        assert_eq!(snapshot.registration_id(), original);
        assert_eq!(snapshot.next_deadline_ns(), None);
        assert_counters!(snapshot.observability().counters(), (cancelled, 0));
        let scheduled = registry
            .reconcile_ordinary(&claim, 2, Some(TimerSchedule::At(20)))
            .unwrap();
        confirm(&mut registry, &scheduled);
        registry.cancel(&claim).unwrap();
        registry.reconcile_ordinary(&claim, 3, None).unwrap();
        let snapshot = registry.snapshot(&timer).unwrap();
        assert_eq!(
            snapshot.state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::Cancelled
            }
        );
        assert_counters!(snapshot.observability().counters(), (cancelled, 1));
        assert_eq!(snapshot.registration_id(), original);
    }
}

#[test]
fn inactive_demand_waits_for_running_work_and_preserves_removal() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        let mut registry = registry();
        let timer = identity("running-inactive-demand");
        let claim = registry.register_once(timer.clone(), lifetime).unwrap();
        let scheduled = registry
            .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(1)))
            .unwrap();
        confirm(&mut registry, &scheduled);
        let (token, _) = arm(scheduled);
        registry.begin_ordinary(&token).unwrap();
        registry.reconcile_ordinary(&claim, 2, None).unwrap();
        assert_eq!(
            registry.snapshot(&timer).unwrap().registration_status(),
            TimerRegistrationStatus::Running
        );
        registry
            .complete_ordinary(
                &token,
                3,
                OrdinaryRunResult::new(
                    TimerCompletion::success(1),
                    OrdinaryDirective::RetryAfter(Duration::MAX),
                ),
            )
            .unwrap();
        match lifetime {
            DeclarationLifetime::Retained => assert_eq!(
                registry.snapshot(&timer).unwrap().state(),
                TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::Stopped
                }
            ),
            DeclarationLifetime::RemoveWhenStopped => assert!(registry.snapshot(&timer).is_none()),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Command {
    Clear,
    Cancel,
    Schedule,
    Unregister,
    Invariant,
}

#[test]
fn inactive_demand_preserves_command_order_and_invariant_failure_priority() {
    for recurring in [false, true] {
        for command in [
            Command::Clear,
            Command::Cancel,
            Command::Schedule,
            Command::Unregister,
            Command::Invariant,
        ] {
            let mut registry = registry();
            let timer = identity("inactive-command-order");
            let claim = if recurring {
                registry.register_after_completion(
                    timer.clone(),
                    cadence(5),
                    DeclarationLifetime::Retained,
                )
            } else {
                registry.register_once(timer.clone(), DeclarationLifetime::Retained)
            }
            .unwrap();
            let original = registry.snapshot(&timer).unwrap().registration_id();
            let scheduled = registry
                .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(1)))
                .unwrap();
            confirm(&mut registry, &scheduled);
            let (token, _) = arm(scheduled);
            registry.begin_ordinary(&token).unwrap();
            if command == Command::Clear {
                registry.cancel(&claim).unwrap();
            }
            if command == Command::Unregister {
                registry.unregister(&claim).unwrap();
            }
            registry.reconcile_ordinary(&claim, 2, None).unwrap();
            match command {
                Command::Cancel => {
                    registry.cancel(&claim).unwrap();
                }
                Command::Schedule => {
                    registry
                        .reconcile_ordinary(&claim, 2, Some(TimerSchedule::At(20)))
                        .unwrap();
                }
                Command::Clear | Command::Unregister | Command::Invariant => {}
            }
            let completion = if command == Command::Invariant {
                TimerCompletion::invariant_failure(1)
            } else {
                TimerCompletion::success(1)
            };
            let finished = registry
                .complete_ordinary(
                    &token,
                    3,
                    OrdinaryRunResult::new(
                        completion,
                        OrdinaryDirective::RetryAfter(Duration::MAX),
                    ),
                )
                .unwrap();
            assert_eq!(finished.failure(), None);
            confirm(&mut registry, &finished);
            if command == Command::Unregister {
                assert!(registry.snapshot(&timer).is_none());
                continue;
            }
            let current = registry.snapshot(&timer).unwrap();
            assert_eq!(current.registration_id(), original);
            assert_counters!(
                current.observability().counters(),
                (cancelled, u128::from(command == Command::Cancel))
            );
            if command == Command::Schedule {
                assert_eq!(current.next_deadline_ns(), Some(20));
            } else {
                let reason = match command {
                    Command::Cancel => InactiveReason::Cancelled,
                    Command::Invariant => InactiveReason::InvariantFailure,
                    Command::Clear | Command::Schedule | Command::Unregister => {
                        InactiveReason::Stopped
                    }
                };
                assert_eq!(
                    current.state(),
                    TimerRuntimeStateSnapshot::Inactive { reason }
                );
            }
        }
    }
}
