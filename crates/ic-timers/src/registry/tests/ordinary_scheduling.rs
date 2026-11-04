use super::*;

#[test]
fn once_coalesces_and_rotates_generations_while_nested_schedule_wins() {
    let mut registry = registry();
    let timer = identity("once");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");

    let first_transition = registry
        .ensure_once(&claim, 10, TimerSchedule::At(100))
        .expect("initial ensure should succeed");
    confirm(&mut registry, &first_transition);
    confirm(&mut registry, &first_transition);
    assert_eq!(
        registry
            .snapshot(&timer)
            .expect("snapshot should exist")
            .observability()
            .counters()
            .wakeups_armed()
            .exact_total()
            .unwrap(),
        1
    );
    let (first, arm_kind) = arm(first_transition);
    let deadline = scheduled_deadline(&registry, &first);
    assert_eq!(deadline, 100);
    assert_eq!(arm_kind, WakeupArm::Initial);
    assert_eq!(first.callback_generation(), 1);

    let duplicate = registry
        .ensure_once(&claim, 10, TimerSchedule::At(200))
        .expect("duplicate ensure should succeed");
    assert_eq!(duplicate.effect(), &RegistryEffect::None);
    assert!(registry.begin_ordinary(&first).is_some());

    let nested = registry
        .ensure_once(&claim, 20, TimerSchedule::At(80))
        .expect("nested ensure should succeed");
    assert_eq!(nested.effect(), &RegistryEffect::None);
    let equal = registry
        .ensure_once(&claim, 20, TimerSchedule::After(Duration::from_nanos(60)))
        .expect("equal relative demand should coalesce");
    assert_eq!(equal.effect(), &RegistryEffect::None);
    let second_transition = registry
        .complete_ordinary(
            &first,
            30,
            OrdinaryRunResult::new(TimerCompletion::success(1), OrdinaryDirective::Stop),
        )
        .expect("completion should succeed");
    confirm(&mut registry, &second_transition);
    let (second, _) = arm(second_transition);
    let deadline = scheduled_deadline(&registry, &second);
    assert_eq!(deadline, 80);
    let snapshot = registry.snapshot(&timer).unwrap();
    assert_eq!(snapshot.scheduling_mode(), TimerSchedulingMode::Deadline);
    assert_eq!(snapshot.latest_requested_delay_ns(), None);
    assert_eq!(second.callback_generation(), 2);
    assert!(registry.begin_ordinary(&first).is_none());
    assert!(registry.begin_ordinary(&second).is_some());
    registry
        .complete_ordinary(
            &second,
            40,
            OrdinaryRunResult::new(TimerCompletion::no_work(), OrdinaryDirective::Stop),
        )
        .expect("terminal completion should succeed");

    let snapshot = registry.snapshot(&timer).expect("retained snapshot exists");
    assert_eq!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped,
        }
    );
    let counters = snapshot.observability().counters();
    assert_eq!(counters.schedule_requests().exact_total().unwrap(), 4);
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 2);
    assert_eq!(counters.coalesced().exact_total().unwrap(), 3);
    assert_eq!(counters.stale_wakeups().exact_total().unwrap(), 1);
    assert_eq!(counters.work_started().exact_total().unwrap(), 2);
    assert_eq!(counters.work_completed().exact_total().unwrap(), 2);
    assert_eq!(counters.succeeded().exact_total().unwrap(), 1);
    assert_eq!(counters.no_work().exact_total().unwrap(), 1);
}

#[test]
fn nested_cancel_and_ensure_use_latest_request_order() {
    let mut registry = registry();
    let timer = identity("nested");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");

    let (first, _) = arm(registry
        .ensure_once(&claim, 0, TimerSchedule::At(10))
        .expect("ensure should succeed"));
    assert!(registry.begin_ordinary(&first).is_some());
    registry
        .ensure_once(&claim, 10, TimerSchedule::At(30))
        .expect("nested ensure should succeed");
    registry
        .cancel(&claim)
        .expect("nested cancel should succeed");
    let cancelled = registry
        .complete_ordinary(
            &first,
            11,
            OrdinaryRunResult::new(
                TimerCompletion::success(1),
                OrdinaryDirective::ContinueImmediately,
            ),
        )
        .expect("completion should succeed");
    assert_eq!(cancelled.effect(), &RegistryEffect::None);
    let cancelled_snapshot = registry
        .snapshot(&timer)
        .expect("retained timer should exist");
    assert_eq!(
        cancelled_snapshot.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Cancelled,
        }
    );
    assert_eq!(
        cancelled_snapshot
            .observability()
            .counters()
            .cancelled()
            .exact_total()
            .unwrap(),
        1
    );

    let (second, _) = arm(registry
        .ensure_once(&claim, 20, TimerSchedule::At(40))
        .expect("retained declaration should re-enable"));
    assert!(registry.begin_ordinary(&second).is_some());
    registry
        .cancel(&claim)
        .expect("nested cancel should succeed");
    registry
        .ensure_once(&claim, 21, TimerSchedule::At(50))
        .expect("later nested ensure should succeed");
    let (scheduled_token, _) = arm(registry
        .complete_ordinary(
            &second,
            22,
            OrdinaryRunResult::new(TimerCompletion::no_work(), OrdinaryDirective::Stop),
        )
        .expect("later ensure should override cancellation"));
    let deadline = scheduled_deadline(&registry, &scheduled_token);
    assert_eq!(deadline, 50);
    assert_eq!(
        registry
            .snapshot(&timer)
            .expect("later ensure should keep the retained timer")
            .observability()
            .counters()
            .cancelled()
            .exact_total()
            .unwrap(),
        1,
        "a later ensure supersedes cancellation without counting another cancel"
    );
}

#[test]
fn exact_ordinary_reconciliation_discards_invalid_callback_proposals() {
    for directive in [
        OrdinaryDirective::RetryAfter(Duration::MAX),
        OrdinaryDirective::RetryAfter(Duration::from_nanos(10)),
        OrdinaryDirective::RecurAfterCompletion,
    ] {
        for once in [false, true] {
            let mut registry = registry();
            let timer = identity("discarded-directive");
            let claim = if once {
                registry
                    .register_once(timer.clone(), DeclarationLifetime::Retained)
                    .unwrap()
            } else {
                registry
                    .register_after_completion(
                        timer.clone(),
                        cadence(10),
                        DeclarationLifetime::Retained,
                    )
                    .unwrap()
            };
            let (token, _) = arm(registry
                .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(1)))
                .unwrap());
            assert!(registry.begin_ordinary(&token).is_some());
            registry
                .reconcile_ordinary(&claim, u64::MAX - 5, Some(TimerSchedule::At(u64::MAX)))
                .unwrap();
            let transition = registry
                .complete_ordinary(
                    &token,
                    u64::MAX - 5,
                    OrdinaryRunResult::new(TimerCompletion::success(1), directive),
                )
                .unwrap();
            assert_eq!(transition.failure(), None);
            let (token, _) = arm(transition);
            assert_eq!(scheduled_deadline(&registry, &token), u64::MAX);
            let snapshot = registry.snapshot(&timer).unwrap();
            assert_eq!(
                snapshot.latest_directive(),
                Some(TimerDirectiveSnapshot::ScheduleAt {
                    deadline_ns: u64::MAX
                })
            );
            assert_eq!(
                snapshot
                    .observability()
                    .counters()
                    .succeeded()
                    .exact_total()
                    .unwrap(),
                1
            );
            assert_eq!(
                snapshot
                    .observability()
                    .counters()
                    .invariant_failure()
                    .exact_total()
                    .unwrap(),
                0
            );
        }
    }
}

#[test]
fn relative_exact_reconciliation_preserves_request_observations_after_suspension() {
    for ensure_equal in [false, true] {
        let mut registry = registry();
        let timer = identity("relative-exact-completion");
        let claim = registry
            .register_after_completion(timer.clone(), cadence(3), DeclarationLifetime::Retained)
            .unwrap();
        let (token, _) = arm(registry
            .reconcile_ordinary(&claim, 10, Some(TimerSchedule::At(10)))
            .unwrap());
        assert!(registry.begin_ordinary(&token).is_some());
        registry
            .reconcile_ordinary(
                &claim,
                10,
                Some(TimerSchedule::After(Duration::from_nanos(3))),
            )
            .unwrap();
        let directive = if ensure_equal {
            registry.ensure_recurring(&claim, 10).unwrap();
            OrdinaryDirective::ScheduleAt(13)
        } else {
            OrdinaryDirective::RetryAfter(Duration::MAX)
        };
        let transition = registry
            .complete_ordinary(
                &token,
                20,
                OrdinaryRunResult::new(TimerCompletion::success(1), directive),
            )
            .unwrap();
        assert_eq!(transition.failure(), None);
        assert!(matches!(
            transition.effect(),
            RegistryEffect::ArmWakeup { delay_ns: 0, .. }
        ));
        confirm(&mut registry, &transition);
        let snapshot = registry.snapshot(&timer).unwrap();
        assert_eq!(snapshot.next_deadline_ns(), Some(13));
        assert_eq!(
            snapshot.latest_directive(),
            Some(TimerDirectiveSnapshot::ScheduleAt { deadline_ns: 13 })
        );
        assert_eq!(snapshot.scheduling_mode(), TimerSchedulingMode::Once);
        assert_eq!(snapshot.latest_requested_delay_ns(), Some(3));
        assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
    }
}

#[test]
fn explicit_invariant_failure_still_overrides_exact_ordinary_reconciliation() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        let mut registry = registry();
        let timer = identity("invariant-overrides-exact");
        let claim = registry.register_once(timer.clone(), lifetime).unwrap();
        let (token, _) = arm(registry
            .ensure_once(&claim, 0, TimerSchedule::At(1))
            .unwrap());
        assert!(registry.begin_ordinary(&token).is_some());
        registry
            .reconcile_ordinary(&claim, 1, Some(TimerSchedule::At(50)))
            .unwrap();
        let transition = registry
            .complete_ordinary(
                &token,
                2,
                OrdinaryRunResult::new(
                    TimerCompletion::invariant_failure(1),
                    OrdinaryDirective::ContinueImmediately,
                ),
            )
            .unwrap();
        assert_eq!(transition.effect(), &RegistryEffect::None);
        let snapshot = registry.snapshot(&timer);
        if lifetime == DeclarationLifetime::Retained {
            assert_eq!(
                snapshot.unwrap().state(),
                TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::InvariantFailure
                }
            );
        } else {
            assert!(snapshot.is_none());
        }
    }
}

#[test]
fn ordinary_reconciliation_is_authoritative_and_registry_pending_is_ordered() {
    let mut registry = registry();
    let timer = identity("reconcile-ordinary");
    let claim = registry
        .register_after_completion(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");

    let (replaced, arm_kind) = arm(registry
        .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(100)))
        .expect("initial reconciliation should arm"));
    let deadline = scheduled_deadline(&registry, &replaced);
    assert_eq!(deadline, 100);
    assert_eq!(arm_kind, WakeupArm::Initial);
    let (current, arm_kind) = arm(registry
        .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(200)))
        .expect("authoritative reconciliation may move later"));
    let deadline = scheduled_deadline(&registry, &current);
    assert_eq!(deadline, 200);
    assert_eq!(arm_kind, WakeupArm::Replacement);
    let before = registry.snapshot(&timer).unwrap();
    let equal = registry
        .reconcile_ordinary(
            &claim,
            0,
            Some(TimerSchedule::After(Duration::from_nanos(200))),
        )
        .unwrap();
    assert_eq!(equal.effect(), &RegistryEffect::None);
    let exact = registry.snapshot(&timer).unwrap();
    assert_eq!(exact.state(), before.state());
    assert_eq!(exact.scheduling_mode(), TimerSchedulingMode::Once);
    assert_eq!(exact.latest_requested_delay_ns(), Some(200));
    assert!(registry.begin_ordinary(&replaced).is_none());
    assert!(registry.begin_ordinary(&current).is_some());

    registry
        .ensure_recurring(&claim, 200)
        .expect("nested ensure should establish an earlier pending schedule");
    registry
        .reconcile_ordinary(&claim, 200, Some(TimerSchedule::At(300)))
        .expect("later authoritative request should supersede the ensure");
    let (successor, _) = arm(registry
        .complete_ordinary(
            &current,
            201,
            OrdinaryRunResult::new(
                TimerCompletion::success(1),
                OrdinaryDirective::ScheduleAt(225),
            ),
        )
        .expect("authoritative request should replace the callback directive"));
    let deadline = scheduled_deadline(&registry, &successor);
    assert_eq!(deadline, 300);
    assert!(registry.begin_ordinary(&successor).is_some());

    registry
        .reconcile_ordinary(&claim, 300, Some(TimerSchedule::At(400)))
        .expect("nested reconciliation should succeed");
    registry
        .ensure_recurring(&claim, 301)
        .expect("a later ensure should supersede reconciliation by request order");
    let (scheduled_token, _) = arm(registry
        .complete_ordinary(
            &successor,
            302,
            OrdinaryRunResult::new(
                TimerCompletion::no_work(),
                OrdinaryDirective::ScheduleAt(375),
            ),
        )
        .expect("completion should use the ordered pending request"));
    let deadline = scheduled_deadline(&registry, &scheduled_token);
    assert_eq!(deadline, 306);

    registry
        .reconcile_ordinary(&claim, 400, None)
        .expect("inactive reconciliation should stop the retained declaration");
    assert_eq!(
        registry.snapshot(&timer).map(|snapshot| snapshot.state()),
        Some(TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped,
        })
    );
}

#[test]
fn after_completion_owns_cadence_and_failure_state() {
    let mut registry = registry();
    let timer = identity("after-completion");
    let claim = registry
        .register_after_completion(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let (first, _) = arm(registry
        .ensure_recurring(&claim, 10)
        .expect("initial ensure should succeed"));
    let deadline = scheduled_deadline(&registry, &first);
    assert_eq!(deadline, 15);
    assert!(registry.begin_ordinary(&first).is_some());
    let (second, _) = arm(registry
        .complete_ordinary(
            &first,
            20,
            OrdinaryRunResult::new(
                TimerCompletion::retryable_failure(2),
                OrdinaryDirective::RecurAfterCompletion,
            ),
        )
        .expect("recurrence should succeed"));
    let deadline = scheduled_deadline(&registry, &second);
    assert_eq!(deadline, 25);
    assert_eq!(registry.consecutive_expected_failures(&timer), Some(1));
    assert!(registry.begin_ordinary(&second).is_some());
    registry
        .complete_ordinary(
            &second,
            30,
            OrdinaryRunResult::new(TimerCompletion::no_work(), OrdinaryDirective::Stop),
        )
        .expect("stop should succeed");
    assert_eq!(registry.consecutive_expected_failures(&timer), Some(0));
}

#[test]
fn ordinary_directive_matrix_preserves_mode_and_checked_deadline() {
    let mut registry = registry();
    let timer = identity("directive-matrix");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let initial = registry
        .ensure_once(&claim, 0, TimerSchedule::At(1))
        .expect("ensure should succeed");
    confirm(&mut registry, &initial);
    let (first, _) = arm(initial);
    assert!(registry.begin_ordinary(&first).is_some());

    let immediate = registry
        .complete_ordinary(
            &first,
            10,
            OrdinaryRunResult::new(
                TimerCompletion::success(1),
                OrdinaryDirective::ContinueImmediately,
            ),
        )
        .expect("immediate continuation should succeed");
    confirm(&mut registry, &immediate);
    let (second, _) = arm(immediate);
    let deadline = scheduled_deadline(&registry, &second);
    assert_eq!(deadline, 10);
    assert_eq!(
        registry
            .snapshot(&timer)
            .map(|value| value.scheduling_mode()),
        Some(TimerSchedulingMode::Continuation)
    );
    assert!(registry.begin_ordinary(&second).is_some());

    let retry = registry
        .complete_ordinary(
            &second,
            20,
            OrdinaryRunResult::new(
                TimerCompletion::retryable_failure(0),
                OrdinaryDirective::RetryAfter(Duration::from_nanos(5)),
            ),
        )
        .expect("retry should succeed");
    confirm(&mut registry, &retry);
    let (third, _) = arm(retry);
    let deadline = scheduled_deadline(&registry, &third);
    assert_eq!(deadline, 25);
    assert_eq!(
        registry
            .snapshot(&timer)
            .map(|value| value.scheduling_mode()),
        Some(crate::TimerSchedulingMode::Retry)
    );
    assert!(registry.begin_ordinary(&third).is_some());

    let absolute = registry
        .complete_ordinary(
            &third,
            30,
            OrdinaryRunResult::new(
                TimerCompletion::success(1),
                OrdinaryDirective::ScheduleAt(40),
            ),
        )
        .expect("absolute scheduling should succeed");
    confirm(&mut registry, &absolute);
    let (fourth, _) = arm(absolute);
    let deadline = scheduled_deadline(&registry, &fourth);
    assert_eq!(deadline, 40);
    assert_eq!(
        registry
            .snapshot(&timer)
            .map(|value| value.scheduling_mode()),
        Some(TimerSchedulingMode::Deadline)
    );
    assert!(registry.begin_ordinary(&fourth).is_some());
    registry
        .complete_ordinary(
            &fourth,
            41,
            OrdinaryRunResult::new(TimerCompletion::no_work(), OrdinaryDirective::Stop),
        )
        .expect("stop should succeed");
    assert_eq!(
        registry.snapshot(&timer).map(|value| value.state()),
        Some(TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped,
        })
    );
}

#[test]
fn recurring_duplicate_ensure_is_idempotent_without_deadline_recalculation() {
    for exact_deadline in [false, true] {
        let mut registry = registry();
        let timer = identity("recurring-idempotent");
        let claim = registry
            .register_after_completion(timer.clone(), cadence(5), DeclarationLifetime::Retained)
            .expect("claim should succeed");
        let EntryKind::Ordinary { control, .. } =
            &mut registry.entries.get_mut(&timer).unwrap().kind
        else {
            panic!("fixture must be ordinary control");
        };
        control.seed_generation_for_test(u64::MAX - 1);
        let initial = if exact_deadline {
            registry.reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(40)))
        } else {
            registry.ensure_recurring(&claim, 0)
        }
        .expect("initial schedule should succeed");
        confirm(&mut registry, &initial);
        let before = registry.snapshot(&timer).unwrap();
        let duplicate = registry
            .ensure_recurring(&claim, u64::MAX)
            .expect("existing schedule should satisfy duplicate demand without allocation");
        assert_eq!(duplicate.effect(), &RegistryEffect::None);
        assert_eq!(duplicate.failure(), None);
        let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
        assert_eq!(snapshot.state(), before.state());
        assert_eq!(
            snapshot.next_deadline_ns(),
            Some(if exact_deadline { 40 } else { 5 })
        );
        assert_eq!(snapshot.scheduling_mode(), before.scheduling_mode());
        assert_eq!(snapshot.latest_requested_delay_ns(), Some(5));
        assert_eq!(
            snapshot.latest_armed_delay_ns(),
            before.latest_armed_delay_ns()
        );
        assert_eq!(
            snapshot
                .observability()
                .counters()
                .schedule_requests()
                .exact_total()
                .unwrap(),
            2
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
        assert_eq!(
            snapshot
                .observability()
                .counters()
                .coalesced()
                .exact_total()
                .unwrap(),
            1
        );
        let (token, _) = arm(initial);
        assert_eq!(token.callback_generation(), u64::MAX);
        assert!(registry.begin_ordinary(&token).is_some());
    }
}

#[test]
fn erased_policy_mismatch_and_invariant_result_stop_truthfully() {
    // Inject a private erased-policy mismatch. Public Once results cannot
    // express recurrence; the registry still guards its internal boundary.
    let mut registry = registry();
    let illegal_id = identity("illegal");
    let illegal = registry
        .register_once(illegal_id.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let (token, _) = arm(registry
        .ensure_once(&illegal, 0, TimerSchedule::At(1))
        .expect("ensure should succeed"));
    assert!(registry.begin_ordinary(&token).is_some());
    let transition = registry
        .complete_ordinary(
            &token,
            2,
            OrdinaryRunResult::new(
                TimerCompletion::success(1),
                OrdinaryDirective::RecurAfterCompletion,
            ),
        )
        .expect("illegal directive becomes a terminal transition");
    assert_eq!(
        transition.failure(),
        Some(TimerControlFailure::DirectiveNotAllowed)
    );
    let snapshot = registry
        .snapshot(&illegal_id)
        .expect("snapshot should exist");
    assert_eq!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::ControlFailure(TimerControlFailure::DirectiveNotAllowed,),
        }
    );
    let invariant_id = identity("invariant");
    let invariant = registry
        .register_once(invariant_id.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let (token, _) = arm(registry
        .ensure_once(&invariant, 0, TimerSchedule::At(1))
        .expect("ensure should succeed"));
    assert!(registry.begin_ordinary(&token).is_some());
    let raw = OrdinaryRunResult::new(
        TimerCompletion::invariant_failure(3),
        OrdinaryDirective::ContinueImmediately,
    );
    assert_eq!(raw.directive(), OrdinaryDirective::ContinueImmediately);
    let transition = registry
        .complete_ordinary(&token, 2, raw)
        .expect("invariant completion should stop normally");
    assert_eq!(transition.failure(), None);
    assert_eq!(
        registry.snapshot(&invariant_id).map(|value| value.state()),
        Some(TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::InvariantFailure,
        })
    );
    for (timer, work_count) in [(illegal_id, 1), (invariant_id, 3)] {
        let snapshot = registry
            .snapshot(&timer)
            .expect("retained declaration should exist");
        assert_eq!(
            snapshot.latest_directive(),
            Some(TimerDirectiveSnapshot::Stop)
        );
        let counters = snapshot.observability().counters();
        assert_eq!(counters.work_completed().exact_total().unwrap(), 1);
        assert_eq!(counters.invariant_failure().exact_total().unwrap(), 1);
        assert_eq!(counters.cancelled().exact_total().unwrap(), 0);
        let outcomes = snapshot.observability().outcomes();
        assert_eq!(
            outcomes.last_outcome(),
            Some(TimerLastOutcome::Completed(
                TimerCompletionOutcome::InvariantFailure
            ))
        );
        assert_eq!(outcomes.last_work_count(), Some(work_count));
        assert_eq!(outcomes.last_failure_at_ns(), Some(2));
    }
}
