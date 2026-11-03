//! Bounded command interleavings against the real runtime and native provider.
//! This checks authority/handle ownership, not IC message rollback or recovery.

use super::*;

fn declaration(
    policy: TestPolicy,
    timer: &TimerIdentity,
    lifetime: DeclarationLifetime,
) -> RegistrationClaim {
    let start = scheduled_start(lifetime, TimerSchedule::After(Duration::from_nanos(5)));
    let cadence = TimerCadence::from_nanos(5).unwrap();
    match policy {
        TestPolicy::Once => {
            register_once(timer.clone(), start, |_| async {
                OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
            })
            .unwrap()
            .claim
        }
        TestPolicy::AfterCompletion => {
            register_after_completion(timer.clone(), cadence, start, |_| async {
                AfterCompletionRunResult::new(
                    TimerCompletion::no_work(),
                    AfterCompletionDecision::RecurAfterCompletion,
                )
            })
            .unwrap()
            .claim
        }
        TestPolicy::Watchdog => {
            register_watchdog(timer.clone(), cadence, start, |_| {
                WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue)
            })
            .unwrap()
            .claim
        }
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "Keep the bounded trace driver and its invariant checks together."
)]
fn generated_command_sequences_preserve_authority_and_handle_ownership() {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Command {
        Ensure,
        Reconcile,
        ReconcileWithAdmissionFailure,
        ReconcileWithBindingFailure,
        Cancel,
        Replace,
        Tick,
        Discard,
    }
    const COMMANDS: [Command; 8] = [
        Command::Ensure,
        Command::Reconcile,
        Command::ReconcileWithAdmissionFailure,
        Command::ReconcileWithBindingFailure,
        Command::Cancel,
        Command::Replace,
        Command::Tick,
        Command::Discard,
    ];

    // Exhaust every four-command word over eight operations, for all three
    // policies and both lifetimes: 24,576 traces. Include each command prefix.
    for policy in TestPolicy::ALL {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            let mut failures = [0; 2];
            for word in 0..COMMANDS.len().pow(4) {
                let _fixture = setup();
                let timer = identity("sequence");
                let sentinel = register_once(
                    identity("untouched"),
                    TimerStart::Retained {
                        initial: Some(TimerSchedule::At(u64::MAX)),
                    },
                    |_| async { panic!("unrelated timer must not run") },
                )
                .unwrap();
                let sentinel_before = timer_snapshot(sentinel.identity()).unwrap();
                let mut claim = declaration(policy, &timer, lifetime);
                let mut retired = Vec::new();
                let mut commands = word;
                for step in 0..4 {
                    let command = COMMANDS[commands % COMMANDS.len()];
                    commands /= COMMANDS.len();
                    let trace = (policy, lifetime, word, step, command);
                    let exists = timer_snapshot(&timer).unwrap().is_some();
                    // Capture a delivery only when this command invalidates it.
                    let stale = if exists && matches!(command, Command::Cancel | Command::Replace) {
                        let detached =
                            with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
                                .unwrap();
                        detached.map(|handle| {
                            let (token, handle) = handle;
                            bind_provider_handle(&token, handle, ProviderBinding::Stage).unwrap();
                            token
                        })
                    } else {
                        None
                    };
                    let fault = match command {
                        Command::ReconcileWithAdmissionFailure => ProviderFault::ADMISSION,
                        Command::ReconcileWithBindingFailure => ProviderFault::BINDING,
                        _ => ProviderFault::None,
                    };
                    fault.inject();
                    let result = match command {
                        Command::Ensure if policy == TestPolicy::Once => ensure_once_claim(
                            Declared(&claim),
                            TimerSchedule::After(Duration::from_nanos(5)),
                        ),
                        Command::Ensure => ensure_recurring_claim(Declared(&claim)),
                        Command::Reconcile
                        | Command::ReconcileWithAdmissionFailure
                        | Command::ReconcileWithBindingFailure => {
                            // A faulted request differs from prior deadlines so
                            // it can reach provider admission rather than coalesce.
                            let delay = if fault == ProviderFault::None {
                                7
                            } else {
                                11 + step
                            };
                            let schedule = Some(TimerSchedule::After(Duration::from_nanos(delay)));
                            if policy == TestPolicy::Watchdog {
                                reconcile_watchdog_claim(Declared(&claim), schedule)
                            } else {
                                reconcile_ordinary_claim(Declared(&claim), schedule)
                            }
                        }
                        Command::Cancel => cancel_claim(Declared(&claim)),
                        Command::Replace => {
                            if exists {
                                unregister_claim(&claim).unwrap();
                            }
                            retired.push(claim);
                            claim = declaration(policy, &timer, lifetime);
                            Ok(())
                        }
                        Command::Tick => {
                            set_time(platform::time_ns() + 10);
                            // Leave watchdog work pending in some prefixes;
                            // later commands must honor its lifecycle ownership.
                            let _ran = run_next_due();
                            Ok(())
                        }
                        Command::Discard => {
                            let _discarded = discard_next_due();
                            Ok(())
                        }
                    };
                    // Deferred or coalesced requests need not consume the fault.
                    // Never let it affect stale-delivery checks or later commands.
                    ProviderFault::clear();
                    let failed = match (&result, fault) {
                        (Err(TimerError::BackendCapacity), ProviderFault::ADMISSION) => {
                            failures[0] += 1;
                            true
                        }
                        (Err(TimerError::OwnershipInvariant), ProviderFault::BINDING) => {
                            failures[1] += 1;
                            true
                        }
                        _ => false,
                    };
                    assert!(
                        result.is_ok()
                            || matches!(result, Err(TimerError::RegistrationExpired))
                            || failed,
                        "{trace:?}: {result:?}"
                    );
                    if failed {
                        let snapshot = timer_snapshot(&timer).unwrap();
                        if lifetime == DeclarationLifetime::Retained {
                            assert_eq!(
                                snapshot.unwrap().state(),
                                TimerRuntimeStateSnapshot::Inactive {
                                    reason: InactiveReason::ControlFailure(
                                        crate::TimerControlFailure::ProviderBindingFailed,
                                    ),
                                },
                                "failed control: {trace:?}"
                            );
                        } else {
                            assert!(snapshot.is_none(), "failed declaration: {trace:?}");
                        }
                        assert_eq!(timer_count(), 1, "failed handles: {trace:?}");
                    }
                    for old in &retired {
                        assert!(
                            matches!(
                                cancel_claim(Declared(old)),
                                Err(TimerError::RegistrationExpired)
                            ),
                            "{trace:?}"
                        );
                    }
                    if let Some(token) = stale {
                        // Scheduler cancellation while work is pending can be
                        // deferred. Deliver only after its claim really retires,
                        // or ordinary cancellation has invalidated the generation.
                        if policy != TestPolicy::Watchdog
                            || command == Command::Replace
                            || timer_snapshot(&timer)
                                .unwrap()
                                .is_none_or(|s| s.next_deadline_ns().is_none())
                        {
                            let before = timer_snapshot(&timer).unwrap();
                            let handles = timer_count();
                            if policy == TestPolicy::Watchdog {
                                dispatch_watchdog_scheduler(&token);
                            } else {
                                let _delivery = platform::set_timer(
                                    Duration::ZERO,
                                    dispatch_wakeup(WakeupToken::Ordinary(token)),
                                );
                                assert!(run_next_due());
                            }
                            let after = timer_snapshot(&timer).unwrap();
                            assert_eq!(
                                after.as_ref().map(|s| (
                                    s.registration_id(),
                                    s.state(),
                                    s.next_deadline_ns(),
                                    s.observability().performance()
                                )),
                                before.as_ref().map(|s| (
                                    s.registration_id(),
                                    s.state(),
                                    s.next_deadline_ns(),
                                    s.observability().performance()
                                )),
                                "stale authority: {trace:?}"
                            );
                            assert_eq!(timer_count(), handles, "stale handles: {trace:?}");
                        }
                    }
                    assert_eq!(
                        timer_snapshot(sentinel.identity()).unwrap(),
                        sentinel_before,
                        "{trace:?}"
                    );
                    assert!(timer_inventory().unwrap().len() <= 2, "{trace:?}");
                    assert!(
                        (1..=3).contains(&timer_count()),
                        "orphan handles: {trace:?}"
                    );
                }
                if timer_snapshot(&timer).unwrap().is_some() {
                    unregister_claim(&claim).unwrap();
                }
                // A pending watchdog invocation can defer removal. Drain it at
                // the same time; no future cadence should survive unregister.
                for _ in 0..3 {
                    if !run_next_due() {
                        break;
                    }
                }
                assert_eq!(
                    timer_count(),
                    1,
                    "cleanup: policy={policy:?} lifetime={lifetime:?} word={word}"
                );
                sentinel.unregister().unwrap();
                assert_eq!(timer_count(), 0);
            }
            assert!(
                failures.into_iter().all(|count| count > 0),
                "both provider failures must execute: {policy:?}/{lifetime:?}"
            );
        }
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "Keep each suspended trace, its independent outcome oracle and cleanup together."
)]
fn suspended_command_sequences_retire_authority_across_completion_and_abandonment() {
    #[derive(Clone, Copy, Debug)]
    enum Command {
        Cancel,
        Ensure,
        Reconcile,
        Unregister,
    }
    const COMMANDS: [Command; 4] = [
        Command::Cancel,
        Command::Ensure,
        Command::Reconcile,
        Command::Unregister,
    ];

    for recurring in [false, true] {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            for abandon in [false, true] {
                // Abandonment never installs a successor, so a provider fault
                // cannot distinguish another execution path there.
                for fault in [
                    ProviderFault::None,
                    ProviderFault::ADMISSION,
                    ProviderFault::BINDING,
                ]
                .into_iter()
                .filter(|fault| !abandon || *fault == ProviderFault::None)
                {
                    for word in 0..COMMANDS.len().pow(3) {
                        let _fixture = setup();
                        let timer = identity("suspended-sequence");
                        let gate = SuspendedWork::default();
                        let context = Rc::new(RefCell::new(None));
                        let captured = Rc::new(());
                        let claim = suspended_ordinary_registration(
                            recurring,
                            timer.clone(),
                            lifetime,
                            gate.clone(),
                            Rc::clone(&context),
                            Rc::clone(&captured),
                        );
                        reconcile_ordinary_claim(Declared(&claim), Some(TimerSchedule::At(10)))
                            .unwrap();
                        set_time(10);
                        assert!(run_next_due());
                        let token = context.borrow().as_ref().unwrap().clone();
                        let mut commands = word;
                        let mut expected_deadline: Option<u64> = None;
                        let mut unregistered = false;
                        for _ in 0..3 {
                            let command = COMMANDS[commands % COMMANDS.len()];
                            if !unregistered {
                                match command {
                                    Command::Cancel => expected_deadline = None,
                                    Command::Ensure => {
                                        let requested = if recurring { 15 } else { 20 };
                                        expected_deadline = Some(
                                            expected_deadline
                                                .map_or(requested, |old| old.min(requested)),
                                        );
                                    }
                                    Command::Reconcile => expected_deadline = Some(30),
                                    Command::Unregister => {
                                        unregistered = true;
                                        expected_deadline = None;
                                    }
                                }
                            }
                            match command {
                                Command::Cancel => cancel_claim(Declared(&claim)),
                                Command::Ensure if recurring => {
                                    ensure_recurring_claim(Declared(&claim))
                                }
                                Command::Ensure => {
                                    ensure_once_claim(Declared(&claim), TimerSchedule::At(20))
                                }
                                Command::Reconcile => reconcile_ordinary_claim(
                                    Declared(&claim),
                                    Some(TimerSchedule::At(30)),
                                ),
                                Command::Unregister => unregister_claim(&claim),
                            }
                            .unwrap();
                            commands /= COMMANDS.len();
                            assert_eq!(timer_count(), 0, "running work owns no pending wakeup");
                        }
                        fault.inject();
                        gate.resume();
                        if abandon {
                            assert!(discard_next_due());
                        } else {
                            assert!(run_next_due());
                        }
                        ProviderFault::clear();
                        let binding_failed =
                            !abandon && fault != ProviderFault::None && expected_deadline.is_some();
                        if abandon || binding_failed {
                            expected_deadline = None;
                        }
                        let removed = unregistered
                            || (lifetime == DeclarationLifetime::RemoveWhenStopped
                                && expected_deadline.is_none());
                        let snapshot = timer_snapshot(&timer).unwrap();
                        assert_eq!(
                            snapshot.is_none(),
                            removed,
                            "declaration: {word}/{recurring}/{abandon}/{fault:?}"
                        );
                        assert_eq!(timer_count(), usize::from(expected_deadline.is_some()));
                        if let Some(snapshot) = snapshot {
                            assert_eq!(snapshot.next_deadline_ns(), expected_deadline);
                            if expected_deadline.is_none() {
                                let reason = if abandon {
                                    InactiveReason::Abandoned
                                } else if binding_failed {
                                    InactiveReason::ControlFailure(
                                        crate::TimerControlFailure::ProviderBindingFailed,
                                    )
                                } else {
                                    InactiveReason::Cancelled
                                };
                                assert_eq!(
                                    snapshot.state(),
                                    TimerRuntimeStateSnapshot::Inactive { reason }
                                );
                            }
                            assert_eq!(
                                snapshot.observability().counters().work_completed().total(),
                                u128::from(!abandon)
                            );
                            assert_eq!(
                                snapshot.observability().counters().unacknowledged().total(),
                                u128::from(abandon)
                            );
                        }

                        assert!(
                            matches!(
                                cancel_claim(Executing(&token)),
                                Err(TimerError::RegistrationExpired)
                            ),
                            "stale context: recurring={recurring} lifetime={lifetime:?} abandon={abandon} fault={fault:?} word={word}"
                        );
                        if timer_snapshot(&timer).unwrap().is_some() {
                            unregister_claim(&claim).unwrap();
                        }
                        assert_eq!(timer_count(), 0);
                        assert!(timer_inventory().unwrap().is_empty());
                        assert_eq!(Rc::strong_count(&captured), 1);
                    }
                }
            }
        }
    }
}

#[test]
fn dropping_public_claims_does_not_cancel_or_unregister() {
    for policy in TestPolicy::ALL {
        let _fixture = setup();
        let timer = identity("dropped-claim");
        let start = TimerStart::Retained {
            initial: Some(TimerSchedule::At(10)),
        };
        let cadence = TimerCadence::from_nanos(5).unwrap();
        match policy {
            TestPolicy::Once => drop(
                register_once(timer.clone(), start, |_| async {
                    OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
                })
                .unwrap(),
            ),
            TestPolicy::AfterCompletion => drop(
                register_after_completion(timer.clone(), cadence, start, |_| async {
                    AfterCompletionRunResult::new(
                        TimerCompletion::no_work(),
                        AfterCompletionDecision::Stop,
                    )
                })
                .unwrap(),
            ),
            TestPolicy::Watchdog => drop(
                register_watchdog(timer.clone(), cadence, start, |_| {
                    WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
                })
                .unwrap(),
            ),
        }
        assert_eq!(timer_count(), 1);
        assert_eq!(timer_inventory().unwrap().len(), 1);
        set_time(10);
        assert!(run_next_due());
        if policy == TestPolicy::Watchdog {
            assert!(run_next_due());
        }
        assert_eq!(timer_count(), 0);
        let snapshot = timer_snapshot(&timer).unwrap().unwrap();
        assert_eq!(
            snapshot.observability().counters().work_completed().total(),
            1
        );
        assert_eq!(
            timer_inventory().unwrap().len(),
            1,
            "retained declaration survives its lost claim"
        );
    }
}
