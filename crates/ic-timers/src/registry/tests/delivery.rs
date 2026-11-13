use super::*;

fn assert_running_work_rejected(registry: &mut TimerRegistry, token: &CallbackToken) {
    let before = registry.inventory();
    assert_eq!(
        registry.validate_running_context(token),
        Err(RegistryError::StaleCallback)
    );
    assert_eq!(
        registry
            .complete_ordinary(
                token,
                20,
                OrdinaryRunResult::new(TimerCompletion::no_work(), OrdinaryDirective::Stop),
            )
            .map(|_| ()),
        Err(RegistryError::StaleCallback)
    );
    assert_eq!(
        registry
            .complete_watchdog_work(
                token,
                20,
                WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
            )
            .map(|_| ()),
        Err(RegistryError::StaleCallback)
    );
    assert_eq!(registry.inventory(), before);
}

#[test]
fn running_work_boundaries_reject_unstarted_stale_and_completed_tokens() {
    for policy in [
        TimerPolicy::Once,
        TimerPolicy::AfterCompletion {
            cadence: cadence(5),
        },
        TimerPolicy::Watchdog {
            cadence: cadence(5),
        },
    ] {
        let mut registry = registry();
        let timer = identity("running-work-authority");
        let claim = register_policy(&mut registry, timer, policy, DeclarationLifetime::Retained);
        let watchdog = matches!(policy, TimerPolicy::Watchdog { .. });
        let token = if watchdog {
            let (scheduler, _) = arm(registry.ensure_recurring(&claim, 0).unwrap());
            let (_, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 5));
            work
        } else {
            let (work, _) = arm(registry
                .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(5)))
                .unwrap());
            work
        };
        assert_running_work_rejected(&mut registry, &token);
        let accepted = if watchdog {
            registry.begin_watchdog_work(&token).is_some()
        } else {
            registry.begin_ordinary(&token).is_some()
        };
        assert!(accepted);
        if watchdog {
            assert!(registry.begin_watchdog_work(&token).is_none());
        } else {
            assert!(registry.begin_ordinary(&token).is_none());
        }
        assert_eq!(registry.validate_running_context(&token), Ok(()));

        for role in [
            CallbackRole::OrdinaryWork,
            CallbackRole::WatchdogScheduler,
            CallbackRole::WatchdogWork,
        ] {
            if role == token.role() {
                continue;
            }
            assert_running_work_rejected(
                &mut registry,
                &CallbackToken {
                    role,
                    ..token.clone()
                },
            );
        }
        let current_claim_generation = token.claim().claim_generation();
        for (claim_generation, callback_generation) in [
            (current_claim_generation + 1, token.callback_generation),
            (current_claim_generation, token.callback_generation + 1),
        ] {
            assert_running_work_rejected(
                &mut registry,
                &CallbackToken::new(
                    token.identity().clone(),
                    claim_generation,
                    callback_generation,
                    token.role(),
                ),
            );
        }

        if watchdog {
            registry
                .complete_watchdog_work(
                    &token,
                    20,
                    WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
                )
                .unwrap();
        } else {
            registry
                .complete_ordinary(
                    &token,
                    20,
                    OrdinaryRunResult::new(TimerCompletion::no_work(), OrdinaryDirective::Stop),
                )
                .unwrap();
        }
        assert_running_work_rejected(&mut registry, &token);
        registry.unregister(&claim).unwrap();
        assert_running_work_rejected(&mut registry, &token);
    }
}

#[test]
fn provider_roles_keep_paired_handles_distinct_and_reject_policy_mismatches() {
    let provider_count_before = crate::platform::timer_count();
    let mut registry = registry();
    let ordinary = registry
        .register_once(identity("ordinary-role"), DeclarationLifetime::Retained)
        .unwrap();
    let watchdog = registry
        .register_watchdog(
            identity("watchdog-role"),
            cadence(5),
            DeclarationLifetime::Retained,
        )
        .unwrap();
    let (ordinary_token, _) = arm(registry
        .ensure_once(&ordinary, 0, TimerSchedule::At(10))
        .unwrap());
    let (scheduler, _) = arm(registry.ensure_recurring(&watchdog, 0).unwrap());
    let (successor, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 5));
    assert_eq!(successor.callback_generation(), work.callback_generation());
    assert_ne!(successor, work);
    let tokens = [ordinary_token, successor, work];
    for token in &tokens {
        let binding = match token.role() {
            CallbackRole::OrdinaryWork => ProviderBinding::Commit { delay_ns: 10 },
            CallbackRole::WatchdogScheduler => ProviderBinding::Stage,
            CallbackRole::WatchdogWork => ProviderBinding::Commit { delay_ns: 5 },
        };
        registry
            .install_provider_handle(
                token,
                crate::platform::set_timer(Duration::ZERO, async {}),
                binding,
            )
            .unwrap();
        if token.role() == CallbackRole::WatchdogScheduler {
            let staged = registry.snapshot(token.identity()).unwrap();
            assert_eq!(staged.latest_armed_delay_ns(), None);
            assert_counters!(
                staged.observability().counters(),
                (wakeups_armed, 0),
                (work_dispatched, 0)
            );
        }
    }
    for (token, delay_ns, work) in [(&tokens[0], 10, 0), (&tokens[2], 5, 1)] {
        let committed = registry.snapshot(token.identity()).unwrap();
        assert_eq!(committed.latest_armed_delay_ns(), Some(delay_ns));
        assert_counters!(
            committed.observability().counters(),
            (wakeups_armed, 1),
            (work_dispatched, work)
        );
        let before = registry.inventory();
        let (error, handle) = registry
            .install_provider_handle(
                token,
                crate::platform::set_timer(Duration::ZERO, async {}),
                ProviderBinding::Commit { delay_ns: 99 },
            )
            .unwrap_err();
        assert_eq!(error, RegistryError::ProviderHandleAlreadyOwned);
        crate::platform::clear_timer(handle);
        assert_eq!(registry.inventory(), before);
    }

    for token in &tokens {
        for stale in [
            CallbackToken {
                callback_generation: token.callback_generation + 1,
                ..token.clone()
            },
            CallbackToken::new(
                token.identity().clone(),
                token.claim().claim_generation() + 1,
                token.callback_generation,
                token.role,
            ),
        ] {
            let before = registry.inventory();
            let (error, handle) = registry
                .install_provider_handle(
                    &stale,
                    crate::platform::set_timer(Duration::ZERO, async {}),
                    ProviderBinding::Commit { delay_ns: 99 },
                )
                .unwrap_err();
            assert_eq!(error, RegistryError::StaleCallback);
            crate::platform::clear_timer(handle);
            assert_eq!(registry.inventory(), before);
            registry
                .entries
                .get_mut(stale.identity())
                .unwrap()
                .consume_provider_handle(&stale);
        }
        for role in [
            CallbackRole::OrdinaryWork,
            CallbackRole::WatchdogScheduler,
            CallbackRole::WatchdogWork,
        ] {
            if role == token.role() {
                continue;
            }
            let malformed = CallbackToken {
                role,
                ..token.clone()
            };
            let (error, rejected) = registry
                .install_provider_handle(
                    &malformed,
                    crate::platform::set_timer(Duration::ZERO, async {}),
                    ProviderBinding::Stage,
                )
                .unwrap_err();
            let expected = if matches!(role, CallbackRole::OrdinaryWork)
                == matches!(token.role(), CallbackRole::OrdinaryWork)
            {
                // Swapping the paired Watchdog role constructs the other valid
                // token. Its occupied slot must reject replacement, not its stamp.
                RegistryError::ProviderHandleAlreadyOwned
            } else {
                RegistryError::StaleCallback
            };
            assert_eq!(error, expected);
            crate::platform::clear_timer(rejected);
            if error == RegistryError::StaleCallback {
                registry
                    .entries
                    .get_mut(malformed.identity())
                    .unwrap()
                    .consume_provider_handle(&malformed);
            }
            assert_eq!(registry.has_armed_wakeup(&ordinary), Ok(true));
            assert_eq!(registry.has_armed_wakeup(&watchdog), Ok(true));
        }
    }

    for token in &tokens {
        let detached = match token.role() {
            CallbackRole::OrdinaryWork | CallbackRole::WatchdogScheduler => {
                registry.take_wakeup_handle(token.identity())
            }
            CallbackRole::WatchdogWork => {
                let entry = registry.entry_mut(token.claim()).unwrap();
                entry.work.take().map(|owned| {
                    (
                        CallbackToken::new(
                            token.identity().clone(),
                            entry.claim_generation,
                            owned.callback_generation,
                            CallbackRole::WatchdogWork,
                        ),
                        owned.handle,
                    )
                })
            }
        }
        .expect("malformed consumption must leave every owned handle intact");
        let (detached_token, handle) = detached;
        assert_eq!(&detached_token, token);
        registry
            .install_provider_handle(&detached_token, handle, ProviderBinding::Stage)
            .unwrap();
    }
    for claim in [&ordinary, &watchdog] {
        let mut handles = registry.take_provider_handles(claim.identity());
        for handle in [handles.take_wakeup(), handles.take_work()]
            .into_iter()
            .flatten()
        {
            crate::platform::clear_timer(handle);
        }
    }
    assert_eq!(crate::platform::timer_count(), provider_count_before);
}

#[test]
fn provider_installation_and_confirmation_reject_removed_or_reused_claims() {
    for reuse_identity in [false, true] {
        let provider_count_before = crate::platform::timer_count();
        let mut registry = registry();
        let timer = identity("provider-expired-claim");
        let original = registry
            .register_once(timer.clone(), DeclarationLifetime::Retained)
            .unwrap();
        let transition = registry
            .ensure_once(&original, 0, TimerSchedule::At(10))
            .unwrap();
        let RegistryEffect::ArmWakeup { token, .. } = transition.effect() else {
            panic!("fixture must produce an arm effect");
        };
        let token = token.token();
        registry.unregister(&original).unwrap();
        if reuse_identity {
            let replacement = registry
                .register_once(timer, DeclarationLifetime::Retained)
                .unwrap();
            registry
                .ensure_once(&replacement, 0, TimerSchedule::At(20))
                .unwrap();
        }
        let before = registry.inventory();

        let handle = crate::platform::set_timer(Duration::ZERO, async {});
        let (error, rejected) = registry
            .install_provider_handle(token, handle, ProviderBinding::Commit { delay_ns: 10 })
            .unwrap_err();
        assert_eq!(error, RegistryError::StaleCallback);
        assert_eq!(crate::platform::timer_count(), provider_count_before + 1);
        crate::platform::clear_timer(rejected);
        assert_eq!(crate::platform::timer_count(), provider_count_before);
        assert_eq!(registry.inventory(), before);
        assert_eq!(
            registry.confirm_effect_applied(transition.effect()),
            Err(RegistryError::StaleCallback)
        );
        assert_eq!(registry.inventory(), before);
    }
}

#[test]
fn measurement_routing_rejects_a_policy_role_mismatch() {
    let mut registry = registry();
    let timer = identity("measurement-role");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("once claim should succeed");
    let invalid = CallbackToken::new(
        timer.clone(),
        claim.claim_generation(),
        1,
        CallbackRole::WatchdogScheduler,
    );
    let pages = crate::platform::memory_pages();

    assert_eq!(
        registry.ensure_watchdog_immediately(&claim, 10).map(|_| ()),
        Err(RegistryError::PolicyMismatch { actual: "once" })
    );

    assert_eq!(
        registry.record_callback_measurements(&invalid, 10, pages, pages),
        Err(RegistryError::PolicyMismatch { actual: "once" })
    );
    let performance = registry
        .snapshot(&timer)
        .expect("retained declaration should remain")
        .observability()
        .performance();
    assert_eq!(
        performance.scheduler_instructions().aggregate().samples(),
        0
    );
    assert_eq!(performance.work_instructions().aggregate().samples(), 0);
    assert_eq!(performance.scheduler_memory_pages().samples(), 0);
    assert_eq!(performance.work_memory_pages().samples(), 0);
}

#[test]
fn watchdog_dispatch_confirmation_rejects_stale_attempts() {
    let mut registry = registry();
    let timer = identity("dispatch-confirmation");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .unwrap();
    let (scheduler, _) = arm(registry.ensure_recurring(&claim, 0).unwrap());
    let first = registry.begin_watchdog_scheduler(&scheduler, 5);
    registry.confirm_effect_applied(first.effect()).unwrap();
    registry.confirm_effect_applied(first.effect()).unwrap();
    let RegistryEffect::DispatchWatchdog { dispatch, .. } = first.effect() else {
        panic!("dispatch expected")
    };
    let successor = dispatch.successor().clone();
    let second = registry.begin_watchdog_scheduler(&successor, 10);
    let before = registry.inventory();
    assert_eq!(
        registry.confirm_effect_applied(first.effect()),
        Err(RegistryError::StaleCallback)
    );
    assert_eq!(registry.inventory(), before);
    registry.confirm_effect_applied(second.effect()).unwrap();
    registry.confirm_effect_applied(second.effect()).unwrap();
    let counts = registry
        .snapshot(&timer)
        .unwrap()
        .observability()
        .counters();
    assert_counters!(counts, (wakeups_armed, 2), (work_dispatched, 2));
}
