use super::*;

fn assert_cancelled_lifetime(
    registry: &mut TimerRegistry,
    claim: &RegistrationClaim,
    lifetime: DeclarationLifetime,
) {
    if lifetime == DeclarationLifetime::RemoveWhenStopped {
        assert!(registry.is_empty());
        assert_eq!(
            registry.cancel(claim).map(|_| ()),
            Err(RegistryError::UnknownRegistration)
        );
        return;
    }
    assert_eq!(
        registry.cancel(claim).unwrap().effect(),
        &RegistryEffect::None
    );
    let snapshot = registry.snapshot(claim.identity()).unwrap();
    assert_eq!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Cancelled
        }
    );
    let counters = snapshot.observability().counters();
    assert_eq!(counters.cancelled().exact_total().unwrap(), 1);
    assert_eq!(counters.work_started().exact_total().unwrap(), 0);
}

#[test]
fn ordinary_cancellation_at_maximum_generation_preserves_lifetime_and_rejects_delivery() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        for policy in [
            TimerPolicy::Once,
            TimerPolicy::AfterCompletion {
                cadence: cadence(1),
            },
        ] {
            let mut registry = registry();
            let timer = identity("ordinary-cancel-max");
            let claim = register_policy(&mut registry, timer.clone(), policy, lifetime);
            let EntryKind::Ordinary { control, .. } =
                &mut registry.entries.get_mut(&timer).unwrap().kind
            else {
                panic!("fixture must be ordinary control");
            };
            control.seed_generation_for_test(u64::MAX - 1);
            let (token, _) = arm(registry
                .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(10)))
                .unwrap());
            assert_eq!(token.callback_generation(), u64::MAX);
            let cancelled = registry.cancel(&claim).unwrap();
            assert_eq!(cancelled.failure(), None);
            assert_eq!(
                cancelled.effect(),
                &RegistryEffect::ClearCallbacks {
                    handles: CallbacksToClear::Wakeup,
                }
            );
            assert!(registry.begin_ordinary(&token).is_none());
            assert_cancelled_lifetime(&mut registry, &claim, lifetime);
            if lifetime == DeclarationLifetime::Retained {
                let EntryKind::Ordinary { control, .. } =
                    &registry.entries.get(&timer).unwrap().kind
                else {
                    panic!("retained fixture must be ordinary control");
                };
                assert_eq!(control.generation(), u64::MAX);
                assert_eq!(
                    registry
                        .reconcile_ordinary(&claim, 10, Some(TimerSchedule::At(20)))
                        .unwrap()
                        .failure(),
                    Some(TimerControlFailure::GenerationExhausted)
                );
            }
        }
    }
}

#[test]
fn watchdog_cancellation_at_maximum_generation_clears_the_selected_callbacks() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        for dispatched in [false, true] {
            let mut registry = registry();
            let timer = identity("watchdog-cancel-max");
            let claim = registry
                .register_watchdog(timer.clone(), cadence(1), lifetime)
                .unwrap();
            let EntryKind::Watchdog { control, .. } =
                &mut registry.entries.get_mut(&timer).unwrap().kind
            else {
                panic!("fixture must be a watchdog");
            };
            // Establish an exhausted history through real arming/dispatch.
            control.set_generation_for_test(u64::MAX - if dispatched { 2 } else { 1 });
            let (scheduler, _) = arm(registry.ensure_recurring(&claim, 0).unwrap());
            let (scheduler, work) = if dispatched {
                let (successor, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 1));
                assert_eq!(work.callback_generation(), u64::MAX);
                (successor, Some(work))
            } else {
                (scheduler, None)
            };
            assert_eq!(scheduler.callback_generation(), u64::MAX);
            let cancelled = registry.cancel(&claim).unwrap();
            assert_eq!(cancelled.failure(), None);
            assert_eq!(
                cancelled.effect(),
                &RegistryEffect::ClearCallbacks {
                    handles: if dispatched {
                        CallbacksToClear::WakeupAndWork
                    } else {
                        CallbacksToClear::Wakeup
                    },
                }
            );
            assert_eq!(
                registry.begin_watchdog_scheduler(&scheduler, 2).effect(),
                &RegistryEffect::None
            );
            if let Some(work) = work {
                assert!(registry.begin_watchdog_work(&work).is_none());
            }
            assert_cancelled_lifetime(&mut registry, &claim, lifetime);
            if lifetime == DeclarationLifetime::Retained {
                let EntryKind::Watchdog { control, .. } =
                    &registry.entries.get(&timer).unwrap().kind
                else {
                    panic!("retained fixture must be a watchdog");
                };
                assert_eq!(control.generation(), u64::MAX);
                assert_eq!(
                    registry.ensure_recurring(&claim, 2).unwrap().failure(),
                    Some(TimerControlFailure::GenerationExhausted)
                );
            }
        }
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One terminal-failure matrix checks both ordinary policies and lifetimes."
)]
fn ordinary_terminal_failures_respect_declaration_lifetime() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        for policy in [
            TimerPolicy::Once,
            TimerPolicy::AfterCompletion {
                cadence: cadence(1),
            },
        ] {
            for subject in [
                "initial",
                "replacement",
                "completion",
                "completion-delay",
                "completion-deadline",
            ] {
                let completing = subject.starts_with("completion");
                let mut registry = registry();
                let timer = identity(subject);
                let claim = register_policy(&mut registry, timer.clone(), policy, lifetime);
                let EntryKind::Ordinary { control, .. } =
                    &mut registry.entries.get_mut(&timer).unwrap().kind
                else {
                    panic!("fixture must be ordinary control");
                };
                control.seed_generation_for_test(if subject == "initial" {
                    u64::MAX
                } else {
                    u64::MAX - 1
                });
                let token = if subject == "initial" {
                    None
                } else {
                    let (token, _) = arm(registry
                        .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(10)))
                        .unwrap());
                    assert_eq!(token.callback_generation(), u64::MAX);
                    if completing {
                        assert!(registry.begin_ordinary(&token).is_some());
                    }
                    Some(token)
                };
                let (now_ns, directive, failure) = match subject {
                    "completion-delay" => (
                        10,
                        OrdinaryDirective::RetryAfter(Duration::MAX),
                        TimerControlFailure::DelayOutOfRange,
                    ),
                    "completion-deadline" => (
                        u64::MAX,
                        OrdinaryDirective::RetryAfter(Duration::from_nanos(1)),
                        TimerControlFailure::DeadlineOverflow,
                    ),
                    _ => (
                        10,
                        OrdinaryDirective::ContinueImmediately,
                        TimerControlFailure::GenerationExhausted,
                    ),
                };
                let transition = if completing {
                    registry
                        .complete_ordinary(
                            token.as_ref().expect("completion has a running token"),
                            now_ns,
                            OrdinaryRunResult::new(TimerCompletion::success(3), directive),
                        )
                        .unwrap()
                } else {
                    registry
                        .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(1)))
                        .unwrap()
                };
                assert_eq!(transition.failure(), Some(failure));
                if lifetime == DeclarationLifetime::Retained {
                    let snapshot = registry.snapshot(&timer).unwrap();
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::ControlFailure(failure),
                        }
                    );
                    if completing {
                        let counters = snapshot.observability().counters();
                        assert_eq!(counters.work_completed().exact_total().unwrap(), 1);
                        assert_eq!(counters.succeeded().exact_total().unwrap(), 0);
                        assert_eq!(counters.invariant_failure().exact_total().unwrap(), 1);
                        let outcomes = snapshot.observability().outcomes();
                        assert_eq!(outcomes.last_work_count(), Some(3));
                        assert_eq!(
                            outcomes.last_outcome(),
                            Some(TimerLastOutcome::Completed(
                                TimerCompletionOutcome::InvariantFailure
                            ))
                        );
                        assert_eq!(
                            snapshot.latest_directive(),
                            Some(TimerDirectiveSnapshot::Stop)
                        );
                    }
                } else {
                    assert!(registry.is_empty(), "{subject} retained a transient entry");
                    assert_eq!(
                        registry.has_armed_wakeup(&claim),
                        Err(RegistryError::UnknownRegistration)
                    );
                }
            }
        }
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One terminal-failure matrix checks both lifetimes and policy cleanup."
)]
fn watchdog_terminal_failures_respect_declaration_lifetime() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        for subject in [
            "initial",
            "replacement",
            "dispatch-generation",
            "dispatch-deadline",
            "completion-immediate",
            "completion-deadline",
        ] {
            let mut registry = registry();
            let timer = identity(subject);
            let claim = registry
                .register_watchdog(timer.clone(), cadence(1), lifetime)
                .unwrap();
            let EntryKind::Watchdog { control, .. } =
                &mut registry.entries.get_mut(&timer).unwrap().kind
            else {
                panic!("fixture must be a watchdog");
            };
            // Seed allocation history before callbacks exist. The operation
            // under test must encounter exhaustion with its real active token.
            match subject {
                "initial" => control.set_generation_for_test(u64::MAX),
                "replacement" | "dispatch-generation" => {
                    control.set_generation_for_test(u64::MAX - 1);
                }
                "completion-immediate" | "completion-deadline" => {
                    control.set_generation_for_test(u64::MAX - 2);
                }
                "dispatch-deadline" => {}
                _ => unreachable!("closed fixture subjects"),
            }
            let scheduler = if subject == "initial" {
                None
            } else {
                Some(arm(registry.ensure_recurring(&claim, 0).unwrap()).0)
            };
            let work = if subject.starts_with("completion-") {
                let (successor, work) =
                    dispatch(registry.begin_watchdog_scheduler(scheduler.as_ref().unwrap(), 1));
                assert_eq!(successor.callback_generation(), u64::MAX);
                assert!(registry.begin_watchdog_work(&work).is_some());
                Some(work)
            } else {
                None
            };
            let transition = match subject {
                "initial" => registry.ensure_recurring(&claim, 0).unwrap(),
                "replacement" => registry.ensure_watchdog_immediately(&claim, 0).unwrap(),
                "completion-immediate" | "completion-deadline" => registry
                    .complete_watchdog_work(
                        work.as_ref().unwrap(),
                        1,
                        WatchdogRunResult::new(
                            TimerCompletion::success(1),
                            if subject == "completion-immediate" {
                                WatchdogDecision::ContinueImmediately
                            } else {
                                WatchdogDecision::ScheduleAt(10)
                            },
                        ),
                    )
                    .unwrap(),
                _ => registry.begin_watchdog_scheduler(
                    scheduler.as_ref().unwrap(),
                    if subject == "dispatch-deadline" {
                        u64::MAX
                    } else {
                        1
                    },
                ),
            };
            let failure = if subject == "dispatch-deadline" {
                TimerControlFailure::DeadlineOverflow
            } else {
                TimerControlFailure::GenerationExhausted
            };
            assert_eq!(transition.failure(), Some(failure));
            if subject.starts_with("completion-") {
                assert!(matches!(
                    transition.effect(),
                    RegistryEffect::ClearCallbacks {
                        handles: CallbacksToClear::Wakeup,
                        ..
                    }
                ));
            }
            if lifetime == DeclarationLifetime::Retained {
                let snapshot = registry.snapshot(&timer).unwrap();
                if subject == "replacement" {
                    assert_eq!(snapshot.scheduling_mode(), TimerSchedulingMode::Watchdog);
                    assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
                }
                let counters = snapshot.observability().counters();
                assert_eq!(counters.cancelled().exact_total().unwrap(), 0);
                assert_eq!(counters.coalesced().exact_total().unwrap(), 0, "{subject}");
                assert_eq!(
                    counters.work_completed().exact_total().unwrap(),
                    u128::from(subject.starts_with("completion-")),
                    "{subject}"
                );
                assert_eq!(
                    counters.succeeded().exact_total().unwrap(),
                    counters.work_completed().exact_total().unwrap(),
                    "{subject}"
                );
                assert_eq!(
                    counters.invariant_failure().exact_total().unwrap(),
                    0,
                    "{subject}"
                );
                if subject.starts_with("completion-") {
                    let outcomes = snapshot.observability().outcomes();
                    assert_eq!(
                        outcomes.last_outcome(),
                        Some(TimerLastOutcome::Completed(TimerCompletionOutcome::Success))
                    );
                    assert_eq!(outcomes.last_work_count(), Some(1));
                    assert_eq!(outcomes.last_success_at_ns(), Some(1));
                    assert_eq!(outcomes.last_failure_at_ns(), None);
                }
                assert_eq!(
                    snapshot.state(),
                    TimerRuntimeStateSnapshot::Inactive {
                        reason: InactiveReason::ControlFailure(failure),
                    }
                );
            } else {
                assert!(registry.is_empty(), "{subject} retained a transient entry");
                assert_eq!(
                    registry.has_armed_wakeup(&claim),
                    Err(RegistryError::UnknownRegistration)
                );
                let replacement = registry
                    .register_once(timer, DeclarationLifetime::Retained)
                    .unwrap();
                assert!(replacement.claim_generation() > claim.claim_generation());
            }
        }
    }
}
