use super::*;

#[test]
fn watchdog_dispatches_successor_first_and_retires_unacknowledged_attempt() {
    let mut registry = registry();
    let timer = identity("watchdog");
    let (_claim, initial_transition) = scheduled_watchdog(&mut registry, timer.clone(), 10);
    confirm(&mut registry, &initial_transition);
    let (scheduler, _) = arm(initial_transition);
    let deadline = scheduled_deadline(&registry, &scheduler);
    assert_eq!(deadline, 15);

    let first_dispatch = registry.begin_watchdog_scheduler(&scheduler, 20);
    confirm(&mut registry, &first_dispatch);
    confirm(&mut registry, &first_dispatch);
    let (successor, work) = dispatch(first_dispatch);
    let deadline = scheduled_deadline(&registry, &successor);
    assert_eq!(deadline, 25);
    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::AwaitingWork {
            successor_generation: successor.callback_generation(),
            successor_deadline_ns: 25,
            attempt_status: WatchdogAttemptStatus::Dispatched,
        },)
    );
    assert_eq!(
        snapshot.registration_status(),
        TimerRegistrationStatus::Scheduled
    );
    assert!(registry.begin_watchdog_work(&work).is_some());
    assert_eq!(
        registry
            .snapshot(&timer)
            .and_then(|snapshot| match snapshot.state() {
                TimerRuntimeStateSnapshot::Watchdog(
                    WatchdogRuntimeStateSnapshot::AwaitingWork { attempt_status, .. },
                ) => Some(attempt_status),
                _ => None,
            }),
        Some(WatchdogAttemptStatus::Running)
    );
    assert_eq!(
        registry.snapshot(&timer).unwrap().registration_status(),
        TimerRegistrationStatus::Running
    );
    registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue),
        )
        .expect("watchdog completion should succeed");

    let second_dispatch = registry.begin_watchdog_scheduler(&successor, 30);
    confirm(&mut registry, &second_dispatch);
    confirm(&mut registry, &second_dispatch);
    let (next_successor, delayed_work) = dispatch(second_dispatch);
    let deadline = scheduled_deadline(&registry, &next_successor);
    assert_eq!(deadline, 35);
    let third_dispatch = registry.begin_watchdog_scheduler(&next_successor, 40);
    confirm(&mut registry, &third_dispatch);
    let (third_successor, _third_work) = dispatch(third_dispatch);
    let deadline = scheduled_deadline(&registry, &third_successor);
    assert_eq!(deadline, 45);
    assert!(registry.begin_watchdog_work(&delayed_work).is_none());

    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    let counters = snapshot.observability().counters();
    assert_counters!(
        counters,
        (scheduler_started, 3),
        (work_dispatched, 3),
        (unacknowledged, 1),
        (stale_work, 1)
    );
    assert_eq!(
        snapshot.observability().outcomes().last_outcome(),
        Some(TimerLastOutcome::Unacknowledged)
    );
}

#[test]
fn watchdog_requests_preserve_dispatch_authority_until_completion_replaces_the_pair() {
    for running in [false, true] {
        let mut registry = registry();
        let timer = identity("watchdog-dispatch-authority");
        let (claim, initial) = scheduled_watchdog(&mut registry, timer.clone(), 0);
        let (initial, _) = arm(initial);
        let (scheduler, _) = arm(registry
            .reconcile_watchdog_schedule(&claim, 0, Some(TimerSchedule::At(10)))
            .unwrap());
        assert!(scheduler.callback_generation() > initial.callback_generation());
        let (successor, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 10));
        assert_eq!(successor.callback_generation(), work.callback_generation());
        assert_eq!(
            successor.callback_generation(),
            scheduler.callback_generation() + 1
        );
        if running {
            assert!(registry.begin_watchdog_work(&work).is_some());
        }
        let before = registry.snapshot(&timer).unwrap();
        assert_eq!(
            registry.ensure_recurring(&claim, 11).unwrap().effect(),
            &RegistryEffect::None
        );
        assert_eq!(
            registry
                .ensure_watchdog_immediately(&claim, 11)
                .unwrap()
                .effect(),
            &RegistryEffect::None
        );
        assert_eq!(
            registry
                .reconcile_watchdog_schedule(&claim, 11, Some(TimerSchedule::At(30)))
                .unwrap()
                .effect(),
            &RegistryEffect::None
        );
        let pending = registry.snapshot(&timer).unwrap();
        assert_eq!(pending.state(), before.state());
        assert_eq!(pending.scheduling_mode(), before.scheduling_mode());
        assert_counters!(
            pending.observability().counters(),
            (schedule_requests, 5),
            (coalesced, 3)
        );
        assert!(registry.begin_watchdog_work(&successor).is_none());
        assert_eq!(
            registry.begin_watchdog_scheduler(&work, 15).effect(),
            &RegistryEffect::None
        );
        if running {
            assert_eq!(registry.validate_running_context(&work), Ok(()));
        } else {
            assert!(registry.begin_watchdog_work(&work).is_some());
        }
        let (replacement, kind) = arm(registry
            .complete_watchdog_work(
                &work,
                12,
                WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue),
            )
            .unwrap());
        assert_eq!(kind, WakeupArm::Replacement);
        assert_eq!(
            replacement.callback_generation(),
            successor.callback_generation() + 1
        );
        assert_eq!(scheduled_deadline(&registry, &replacement), 30);
        assert_eq!(
            registry.validate_running_context(&work),
            Err(RegistryError::StaleCallback)
        );
        assert!(registry.begin_watchdog_work(&work).is_none());
        assert_eq!(
            registry.begin_watchdog_scheduler(&successor, 30).effect(),
            &RegistryEffect::None
        );
    }
}

#[test]
fn watchdog_immediate_initial_and_replacement_requests_coalesce_without_duplicates() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        let mut registry = registry();
        let immediate_timer = identity("watchdog-immediate-initial");
        let immediate_claim = registry
            .register_watchdog(immediate_timer.clone(), cadence(5), lifetime)
            .expect("claim should succeed");
        let initial = registry
            .ensure_watchdog_immediately(&immediate_claim, 10)
            .expect("immediate initial ensure should succeed");
        assert!(matches!(
            initial.effect(),
            RegistryEffect::ArmWakeup {
                delay_ns: 0,
                arm: WakeupArm::Initial,
                ..
            }
        ));
        confirm(&mut registry, &initial);
        let duplicate = registry
            .ensure_watchdog_immediately(&immediate_claim, 10)
            .expect("equivalent immediate ensure should coalesce");
        assert_eq!(duplicate.effect(), &RegistryEffect::None);
        let snapshot = registry
            .snapshot(&immediate_timer)
            .expect("snapshot should exist");
        let counters = snapshot.observability().counters();
        assert_eq!(snapshot.next_deadline_ns(), Some(10));
        assert_eq!(
            snapshot.scheduling_mode(),
            TimerSchedulingMode::Continuation
        );
        assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
        assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
        assert_counters!(
            counters,
            (schedule_requests, 2),
            (wakeups_armed, 1),
            (coalesced, 1)
        );

        let replacement_timer = identity("watchdog-immediate-replacement");
        let replacement_claim = registry
            .register_watchdog(replacement_timer.clone(), cadence(5), lifetime)
            .expect("replacement claim should succeed");
        let cadence_arm = registry
            .ensure_recurring(&replacement_claim, 10)
            .expect("cadence ensure should succeed");
        confirm(&mut registry, &cadence_arm);
        let (stale_scheduler, _) = arm(cadence_arm);
        let replacement = registry
            .ensure_watchdog_immediately(&replacement_claim, 12)
            .expect("later cadence deadline should move to now");
        assert!(matches!(
            replacement.effect(),
            RegistryEffect::ArmWakeup {
                delay_ns: 0,
                arm: WakeupArm::Replacement,
                ..
            }
        ));
        confirm(&mut registry, &replacement);
        let (immediate_scheduler, _) = arm(replacement);
        assert_eq!(
            registry
                .begin_watchdog_scheduler(&stale_scheduler, 15)
                .effect(),
            &RegistryEffect::None
        );
        let duplicate = registry
            .ensure_watchdog_immediately(&replacement_claim, 12)
            .expect("repeated replacement should coalesce");
        assert_eq!(duplicate.effect(), &RegistryEffect::None);
        let overdue_duplicate = registry
            .ensure_watchdog_immediately(&replacement_claim, 13)
            .expect("an already earlier deadline should satisfy immediate demand");
        assert_eq!(overdue_duplicate.effect(), &RegistryEffect::None);
        let snapshot = registry
            .snapshot(&replacement_timer)
            .expect("replacement snapshot should exist");
        let counters = snapshot.observability().counters();
        assert_eq!(snapshot.next_deadline_ns(), Some(12));
        assert_eq!(
            snapshot.generation(),
            Some(immediate_scheduler.callback_generation())
        );
        assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
        assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
        assert_counters!(
            counters,
            (schedule_requests, 4),
            (wakeups_armed, 2),
            (coalesced, 2),
            (stale_wakeups, 1)
        );
    }
}

#[test]
fn watchdog_running_immediate_request_replaces_exact_successor_and_beats_cadence() {
    let mut registry = registry();
    let timer = identity("watchdog-running-immediate");
    let (claim, initial) = scheduled_watchdog(&mut registry, timer.clone(), 10);
    confirm(&mut registry, &initial);
    let (scheduler, _) = arm(initial);
    let dispatched = registry.begin_watchdog_scheduler(&scheduler, 20);
    confirm(&mut registry, &dispatched);
    let (cadence_successor, work) = dispatch(dispatched);
    let cadence_deadline = scheduled_deadline(&registry, &cadence_successor);
    assert_eq!(cadence_deadline, 25);

    let dispatched_request = registry
        .ensure_watchdog_immediately(&claim, 20)
        .expect("dispatched work should satisfy immediate demand");
    assert_eq!(dispatched_request.effect(), &RegistryEffect::None);
    assert!(registry.begin_watchdog_work(&work).is_some());
    assert_eq!(
        registry
            .ensure_watchdog_immediately(&claim, 21)
            .expect("running immediate request should pend")
            .effect(),
        &RegistryEffect::None
    );
    assert_eq!(
        registry
            .ensure_recurring(&claim, 21)
            .expect("cadence ensure must not delay pending immediate work")
            .effect(),
        &RegistryEffect::None
    );
    let completed = registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop),
        )
        .expect("pending immediate request should override callback stop");
    assert!(matches!(
        completed.effect(),
        RegistryEffect::ArmWakeup {
            delay_ns: 0,
            arm: WakeupArm::Replacement,
            ..
        }
    ));
    confirm(&mut registry, &completed);
    let (immediate_successor, _) = arm(completed);
    assert_eq!(
        registry
            .begin_watchdog_scheduler(&cadence_successor, 25)
            .effect(),
        &RegistryEffect::None,
        "the replaced cadence generation must remain stale"
    );
    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(21));
    assert_eq!(
        snapshot.generation(),
        Some(immediate_successor.callback_generation())
    );
    assert_eq!(
        snapshot.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
    let counters = snapshot.observability().counters();
    assert_counters!(
        counters,
        (schedule_requests, 4),
        (wakeups_armed, 3),
        (work_dispatched, 1),
        (coalesced, 3),
        (stale_wakeups, 1)
    );
}
