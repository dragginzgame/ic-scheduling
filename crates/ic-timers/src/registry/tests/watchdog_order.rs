use super::*;

#[derive(Clone, Copy)]
enum WatchdogOrderCommand {
    Cancel,
    Unregister,
    Immediate,
    Cadence,
    Reconcile(u64),
}

#[derive(Clone, Copy)]
enum WatchdogOrderState {
    Removed,
    Inactive(InactiveReason),
    Scheduled(u64),
}

struct WatchdogOrderCase {
    name: &'static str,
    commands: &'static [WatchdogOrderCommand],
    invariant_failure: bool,
    expected: WatchdogOrderState,
    checked_effect: Option<&'static RegistryEffect>,
}

const WATCHDOG_ORDER_CASES: [WatchdogOrderCase; 9] = {
    use WatchdogOrderCommand::{Cadence, Cancel, Immediate, Reconcile, Unregister};
    use WatchdogOrderState::{Inactive, Removed, Scheduled};
    const CLEAR: RegistryEffect = RegistryEffect::ClearCallbacks {
        handles: CallbacksToClear::Wakeup,
    };
    // Inputs and independent expected states/effects stay together in each row.
    [
        WatchdogOrderCase {
            name: "cancel",
            commands: &[Cancel],
            invariant_failure: false,
            expected: Inactive(InactiveReason::Cancelled),
            checked_effect: Some(&CLEAR),
        },
        WatchdogOrderCase {
            name: "unregister then reconcile",
            commands: &[Unregister, Reconcile(200)],
            invariant_failure: false,
            expected: Removed,
            checked_effect: Some(&CLEAR),
        },
        WatchdogOrderCase {
            name: "immediate",
            commands: &[Immediate],
            invariant_failure: false,
            expected: Scheduled(10),
            checked_effect: None,
        },
        WatchdogOrderCase {
            name: "cadence",
            commands: &[Cadence],
            invariant_failure: false,
            expected: Scheduled(100),
            checked_effect: None,
        },
        WatchdogOrderCase {
            name: "cancel then reconcile",
            commands: &[Cancel, Reconcile(200)],
            invariant_failure: false,
            expected: Scheduled(200),
            checked_effect: None,
        },
        WatchdogOrderCase {
            name: "earlier deadline",
            commands: &[Reconcile(15)],
            invariant_failure: false,
            expected: Scheduled(15),
            checked_effect: Some(&RegistryEffect::None),
        },
        WatchdogOrderCase {
            name: "invariant failure",
            commands: &[],
            invariant_failure: true,
            expected: Inactive(InactiveReason::InvariantFailure),
            checked_effect: Some(&CLEAR),
        },
        WatchdogOrderCase {
            name: "cancel then invariant failure",
            commands: &[Cancel],
            invariant_failure: true,
            expected: Inactive(InactiveReason::InvariantFailure),
            checked_effect: Some(&CLEAR),
        },
        WatchdogOrderCase {
            name: "unregister then invariant failure",
            commands: &[Unregister, Reconcile(200)],
            invariant_failure: true,
            expected: Removed,
            checked_effect: Some(&CLEAR),
        },
    ]
};

#[test]
fn watchdog_exact_pending_order_preserves_terminal_and_immediate_precedence() {
    use WatchdogOrderCommand::{Cadence, Cancel, Immediate, Reconcile, Unregister};
    use WatchdogOrderState::{Inactive, Removed, Scheduled};
    for WatchdogOrderCase {
        name,
        commands,
        invariant_failure,
        expected,
        checked_effect,
    } in WATCHDOG_ORDER_CASES
    {
        let mut registry = registry();
        let timer = identity("deadline-arbitration");
        let claim = registry
            .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
            .unwrap();
        let (scheduler, _) = arm(registry.ensure_watchdog_immediately(&claim, 10).unwrap());
        let (_, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 10));
        assert!(registry.begin_watchdog_work(&work).is_some());
        registry
            .reconcile_watchdog_schedule(&claim, 10, Some(TimerSchedule::At(100)))
            .unwrap();
        for command in commands {
            match *command {
                Cancel => registry.cancel(&claim),
                Unregister => registry.unregister(&claim),
                Immediate => registry.ensure_watchdog_immediately(&claim, 10),
                Cadence => registry.ensure_recurring(&claim, 10),
                Reconcile(deadline) => registry.reconcile_watchdog_schedule(
                    &claim,
                    10,
                    Some(TimerSchedule::At(deadline)),
                ),
            }
            .unwrap();
        }
        let completion = if invariant_failure {
            TimerCompletion::invariant_failure(0)
        } else {
            TimerCompletion::success(1)
        };
        let transition = registry
            .complete_watchdog_work(
                &work,
                10,
                WatchdogRunResult::new(completion, WatchdogDecision::ScheduleAt(300)),
            )
            .unwrap();
        match expected {
            Removed => assert!(registry.snapshot(&timer).is_none(), "{name}"),
            Inactive(_) | Scheduled(_) => {
                let snapshot = registry.snapshot(&timer).unwrap();
                let deadline = match expected {
                    Scheduled(deadline) => Some(deadline),
                    Inactive(reason) => {
                        assert_eq!(
                            snapshot.state(),
                            TimerRuntimeStateSnapshot::Inactive { reason },
                            "{name}"
                        );
                        None
                    }
                    Removed => unreachable!("removed cases handled above"),
                };
                assert_eq!(snapshot.next_deadline_ns(), deadline, "{name}");
                let counters = snapshot.observability().counters();
                assert_eq!(
                    counters.work_completed().exact_total().unwrap(),
                    1,
                    "{name}"
                );
                assert_eq!(
                    counters.cancelled().exact_total().unwrap(),
                    u128::from(matches!(expected, Inactive(InactiveReason::Cancelled))),
                    "{name}"
                );
                assert_eq!(
                    counters.invariant_failure().exact_total().unwrap(),
                    u128::from(invariant_failure),
                    "{name}"
                );
            }
        }
        if let Some(effect) = checked_effect {
            assert_eq!(transition.effect(), effect, "{name}");
        }
    }
}

#[test]
fn watchdog_exact_replacement_rejects_stale_delivery_and_generation_exhaustion() {
    let mut registry = registry();
    let timer = identity("exact-generation");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .unwrap();
    let EntryKind::Watchdog { control, .. } = &mut registry.entries.get_mut(&timer).unwrap().kind
    else {
        panic!("watchdog")
    };
    control.set_generation_for_test(u64::MAX - 3);
    let (old, _) = arm(registry.ensure_recurring(&claim, 10).unwrap());
    let (new, kind) = arm(registry
        .reconcile_watchdog_schedule(
            &claim,
            10,
            Some(TimerSchedule::After(Duration::from_nanos(90))),
        )
        .unwrap());
    let deadline = scheduled_deadline(&registry, &new);
    assert_eq!((deadline, kind), (100, WakeupArm::Replacement));
    let armed = registry.snapshot(&timer).unwrap();
    assert_eq!(armed.scheduling_mode(), TimerSchedulingMode::Once);
    assert_eq!(armed.latest_requested_delay_ns(), Some(90));
    assert_eq!(
        registry
            .reconcile_watchdog_schedule(&claim, 10, Some(TimerSchedule::At(100)))
            .unwrap()
            .effect(),
        &RegistryEffect::None
    );
    let exact = registry.snapshot(&timer).unwrap();
    assert_eq!(exact.scheduling_mode(), TimerSchedulingMode::Once);
    assert_eq!(exact.latest_requested_delay_ns(), None);
    assert_eq!(
        registry
            .ensure_recurring(&claim, u64::MAX)
            .unwrap()
            .effect(),
        &RegistryEffect::None
    );
    let coalesced = registry.snapshot(&timer).unwrap();
    assert_eq!(coalesced.state(), armed.state());
    assert_eq!(coalesced.scheduling_mode(), TimerSchedulingMode::Once);
    assert_eq!(coalesced.latest_requested_delay_ns(), Some(5));
    assert_eq!(
        registry.begin_watchdog_scheduler(&old, 15).into_effect(),
        RegistryEffect::None
    );
    let (successor, work) = dispatch(registry.begin_watchdog_scheduler(&new, 100));
    assert_eq!(successor.callback_generation(), u64::MAX);
    assert!(registry.begin_watchdog_work(&work).is_some());
    let transition = registry
        .complete_watchdog_work(
            &work,
            100,
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ScheduleAt(200),
            ),
        )
        .unwrap();
    assert!(matches!(
        transition.into_effect(),
        RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Wakeup,
            ..
        }
    ));
    assert_eq!(
        registry.snapshot(&timer).unwrap().state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::ControlFailure(TimerControlFailure::GenerationExhausted)
        }
    );
}
