use super::*;
use crate::{TimerLastOutcome, TimerRegistrationStatus};
use std::time::Duration;

impl TimerRegistry {
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn register_once(
        &mut self,
        identity: TimerIdentity,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register_once_with_callback(identity, lifetime, ordinary_callback_fixture())
    }

    pub(crate) fn register_after_completion(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register_after_completion_with_callback(
            identity,
            cadence,
            lifetime,
            ordinary_callback_fixture(),
        )
    }

    pub(crate) fn register_watchdog(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register_watchdog_with_callback(
            identity,
            cadence,
            lifetime,
            watchdog_callback_fixture(),
        )
    }
}

// Pure transition fixtures install real typed payloads but never execute work.
fn ordinary_callback_fixture() -> OrdinaryCallback {
    Rc::new(RefCell::new(|_| {
        panic!("pure registry fixture must not execute ordinary work")
    }))
}

fn watchdog_callback_fixture() -> WatchdogCallback {
    Rc::new(RefCell::new(|_| {
        panic!("pure registry fixture must not execute watchdog work")
    }))
}

fn identity(name: &str) -> TimerIdentity {
    TimerIdentity::try_new("test", "registry", name).expect("fixture identity should be valid")
}

fn registry() -> TimerRegistry {
    TimerRegistry::new(TimerEpoch::new(7, 10))
}

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
        assert_eq!(
            snapshot
                .observability()
                .counters()
                .cancelled()
                .exact_total()
                .unwrap(),
            0
        );
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
        assert_eq!(
            snapshot
                .observability()
                .counters()
                .cancelled()
                .exact_total()
                .unwrap(),
            1
        );
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

#[test]
fn inactive_demand_preserves_command_order_and_invariant_failure_priority() {
    for recurring in [false, true] {
        for command in ["clear", "cancel", "schedule", "unregister", "invariant"] {
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
            if command == "clear" {
                registry.cancel(&claim).unwrap();
            }
            if command == "unregister" {
                registry.unregister(&claim).unwrap();
            }
            registry.reconcile_ordinary(&claim, 2, None).unwrap();
            match command {
                "cancel" => {
                    registry.cancel(&claim).unwrap();
                }
                "schedule" => {
                    registry
                        .reconcile_ordinary(&claim, 2, Some(TimerSchedule::At(20)))
                        .unwrap();
                }
                _ => {}
            }
            let completion = if command == "invariant" {
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
            if command == "unregister" {
                assert!(registry.snapshot(&timer).is_none());
                continue;
            }
            let current = registry.snapshot(&timer).unwrap();
            assert_eq!(current.registration_id(), original);
            assert_eq!(
                current
                    .observability()
                    .counters()
                    .cancelled()
                    .exact_total()
                    .unwrap(),
                u128::from(command == "cancel")
            );
            if command == "schedule" {
                assert_eq!(current.next_deadline_ns(), Some(20));
            } else {
                let reason = match command {
                    "cancel" => InactiveReason::Cancelled,
                    "invariant" => InactiveReason::InvariantFailure,
                    _ => InactiveReason::Stopped,
                };
                assert_eq!(
                    current.state(),
                    TimerRuntimeStateSnapshot::Inactive { reason }
                );
            }
        }
    }
}

fn cadence(nanoseconds: u64) -> TimerCadence {
    TimerCadence::from_nanos(nanoseconds).expect("fixture cadence should be valid")
}

fn arm(transition: RegistryTransition) -> (CallbackToken, WakeupArm) {
    match transition.into_effect() {
        RegistryEffect::ArmWakeup { token, arm, .. } => (token.token().clone(), arm),
        effect => panic!("expected arm effect, got {effect:?}"),
    }
}

fn dispatch(transition: RegistryTransition) -> (CallbackToken, CallbackToken) {
    match transition.into_effect() {
        RegistryEffect::DispatchWatchdog { dispatch, .. } => {
            (dispatch.successor().clone(), dispatch.work())
        }
        effect => panic!("expected watchdog dispatch, got {effect:?}"),
    }
}

fn scheduled_deadline(registry: &TimerRegistry, token: &CallbackToken) -> u64 {
    registry
        .snapshot(token.identity())
        .and_then(|snapshot| snapshot.next_deadline_ns())
        .expect("scheduled generation should have an authoritative deadline")
}

fn confirm(registry: &mut TimerRegistry, transition: &RegistryTransition) {
    registry
        .confirm_effect_applied(transition.effect())
        .expect("fixture provider effect should apply");
}

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
        let claim = match policy {
            TimerPolicy::Once => registry.register_once(timer, DeclarationLifetime::Retained),
            TimerPolicy::AfterCompletion { cadence } => {
                registry.register_after_completion(timer, cadence, DeclarationLifetime::Retained)
            }
            TimerPolicy::Watchdog { cadence } => {
                registry.register_watchdog(timer, cadence, DeclarationLifetime::Retained)
            }
        }
        .unwrap();
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
fn duplicate_registration_and_capacity_fail_without_partial_state() {
    let mut registry = registry();
    let first = identity("timer-00");
    registry
        .register_once(first.clone(), DeclarationLifetime::Retained)
        .expect("first claim should succeed");

    assert_eq!(
        registry.register_watchdog(first.clone(), cadence(1), DeclarationLifetime::Retained,),
        Err(RegisterError::IdentityAlreadyRegistered(first))
    );
    assert_eq!(registry.len(), 1);

    for index in 1..MAX_TIMER_REGISTRATIONS {
        registry
            .register_once(
                identity(&format!("timer-{index:02}")),
                DeclarationLifetime::Retained,
            )
            .expect("registration within capacity should succeed");
    }
    assert_eq!(registry.len(), MAX_TIMER_REGISTRATIONS);

    assert_eq!(
        registry.register_once(identity("overflow"), DeclarationLifetime::Retained),
        Err(RegisterError::CapacityExceeded {
            max: MAX_TIMER_REGISTRATIONS,
        })
    );
    assert_eq!(registry.len(), MAX_TIMER_REGISTRATIONS);

    let names = registry
        .inventory()
        .into_timers()
        .into_iter()
        .map(|snapshot| snapshot.identity().name().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(names.first().map(String::as_str), Some("timer-00"));
    assert_eq!(names.last().map(String::as_str), Some("timer-63"));
}

#[test]
fn ordinary_ensure_rejects_other_policies_without_mutating_observations() {
    let mut registry = registry();
    let once = registry
        .register_once(identity("ensure-once"), DeclarationLifetime::Retained)
        .unwrap();
    let recurring = registry
        .register_after_completion(
            identity("ensure-recurring"),
            cadence(5),
            DeclarationLifetime::Retained,
        )
        .unwrap();
    let watchdog = registry
        .register_watchdog(
            identity("ensure-watchdog"),
            cadence(5),
            DeclarationLifetime::Retained,
        )
        .unwrap();
    let before = registry.inventory();
    for (claim, actual) in [(&recurring, "after_completion"), (&watchdog, "watchdog")] {
        assert_eq!(
            registry
                .ensure_once(claim, 0, TimerSchedule::At(10))
                .map(|_| ()),
            Err(RegistryError::PolicyMismatch { actual })
        );
        assert_eq!(registry.inventory(), before);
    }
    assert_eq!(
        registry.ensure_recurring(&once, 0).map(|_| ()),
        Err(RegistryError::PolicyMismatch { actual: "once" })
    );
    assert_eq!(registry.inventory(), before);
}

#[test]
fn removal_invalidates_old_claim_before_identity_reuse() {
    let mut registry = registry();
    let timer = identity("reused");
    let old = registry
        .register_once(timer.clone(), DeclarationLifetime::RemoveWhenStopped)
        .expect("first claim should succeed");
    let old_generation = old.claim_generation();
    let _ = arm(registry
        .ensure_once(&old, 10, TimerSchedule::At(20))
        .expect("ensure should succeed"));
    registry.cancel(&old).expect("cancel should succeed");
    assert!(registry.is_empty());
    assert_eq!(
        registry
            .ensure_once(&old, 10, TimerSchedule::At(20))
            .map(|_| ()),
        Err(RegistryError::UnknownRegistration)
    );

    let new = registry
        .register_once(timer, DeclarationLifetime::Retained)
        .expect("identity can be claimed again after removal");
    assert!(new.claim_generation() > old_generation);
    assert_eq!(
        registry
            .ensure_once(&old, 10, TimerSchedule::At(20))
            .map(|_| ()),
        Err(RegistryError::StaleRegistration)
    );
}

#[test]
fn fresh_cancellation_preserves_retained_declarations_and_releases_transients() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        for policy in [
            TimerPolicy::Once,
            TimerPolicy::AfterCompletion {
                cadence: cadence(1),
            },
            TimerPolicy::Watchdog {
                cadence: cadence(1),
            },
        ] {
            let mut registry = registry();
            let timer = identity("fresh-cancellation");
            let claim = match policy {
                TimerPolicy::Once => registry.register_once(timer, lifetime),
                TimerPolicy::AfterCompletion { cadence } => {
                    registry.register_after_completion(timer, cadence, lifetime)
                }
                TimerPolicy::Watchdog { cadence } => {
                    registry.register_watchdog(timer, cadence, lifetime)
                }
            }
            .unwrap();
            let before = registry.inventory();
            let transition = registry.cancel(&claim).unwrap();
            assert_eq!(transition.effect(), &RegistryEffect::None);
            assert_eq!(transition.failure(), None);
            if lifetime == DeclarationLifetime::Retained {
                assert_eq!(registry.inventory(), before);
                assert_eq!(
                    registry.cancel(&claim).unwrap().effect(),
                    &RegistryEffect::None
                );
                assert_eq!(registry.inventory(), before);
            } else {
                assert!(registry.is_empty());
                assert_eq!(
                    registry.cancel(&claim).map(|_| ()),
                    Err(RegistryError::UnknownRegistration)
                );
            }
        }
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One ownership matrix checks all provider roles, claims and slot generations."
)]
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
            assert_eq!(
                staged
                    .observability()
                    .counters()
                    .wakeups_armed()
                    .exact_total()
                    .unwrap(),
                0
            );
            assert_eq!(
                staged
                    .observability()
                    .counters()
                    .work_dispatched()
                    .exact_total()
                    .unwrap(),
                0
            );
        }
    }
    for (token, delay_ns, work) in [(&tokens[0], 10, 0), (&tokens[2], 5, 1)] {
        let committed = registry.snapshot(token.identity()).unwrap();
        assert_eq!(committed.latest_armed_delay_ns(), Some(delay_ns));
        assert_eq!(
            committed
                .observability()
                .counters()
                .wakeups_armed()
                .exact_total()
                .unwrap(),
            1
        );
        assert_eq!(
            committed
                .observability()
                .counters()
                .work_dispatched()
                .exact_total()
                .unwrap(),
            work
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
fn explicit_unregistration_consumes_scheduled_and_running_claims() {
    enum LaterRequest {
        Ensure,
        Reconcile,
        Cancel,
    }

    let mut registry = registry();
    let scheduled_id = identity("unregister-scheduled");
    let scheduled = registry
        .register_once(scheduled_id.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let scheduled_transition = registry
        .ensure_once(&scheduled, 0, TimerSchedule::At(10))
        .expect("ensure should succeed");
    confirm(&mut registry, &scheduled_transition);
    let (queued, _) = arm(scheduled_transition);
    let transition = registry
        .unregister(&scheduled)
        .expect("scheduled unregistration should succeed");
    assert_eq!(
        transition.effect(),
        &RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Wakeup,
        }
    );
    assert!(registry.snapshot(&scheduled_id).is_none());
    assert!(registry.begin_ordinary(&queued).is_none());

    for once in [true, false] {
        for command in [
            LaterRequest::Ensure,
            LaterRequest::Reconcile,
            LaterRequest::Cancel,
        ] {
            let running_id = identity("unregister-running");
            let running = if once {
                registry.register_once(running_id.clone(), DeclarationLifetime::Retained)
            } else {
                registry.register_after_completion(
                    running_id.clone(),
                    cadence(5),
                    DeclarationLifetime::Retained,
                )
            }
            .unwrap();
            let running_transition = registry
                .reconcile_ordinary(&running, 0, Some(TimerSchedule::At(10)))
                .unwrap();
            confirm(&mut registry, &running_transition);
            let (active, _) = arm(running_transition);
            assert!(registry.begin_ordinary(&active).is_some());
            assert_eq!(
                registry.unregister(&running).unwrap().effect(),
                &RegistryEffect::None
            );
            let later_request = match command {
                LaterRequest::Ensure => {
                    if once {
                        registry.ensure_once(&running, 10, TimerSchedule::At(20))
                    } else {
                        registry.ensure_recurring(&running, 10)
                    }
                }
                LaterRequest::Reconcile => {
                    registry.reconcile_ordinary(&running, 10, Some(TimerSchedule::At(30)))
                }
                LaterRequest::Cancel => registry.cancel(&running),
            }
            .unwrap();
            assert_eq!(later_request.effect(), &RegistryEffect::None);
            let completed = registry
                .complete_ordinary(
                    &active,
                    11,
                    OrdinaryRunResult::new(
                        TimerCompletion::success(1),
                        OrdinaryDirective::ContinueImmediately,
                    ),
                )
                .unwrap();
            assert_eq!(completed.failure(), None);
            assert_eq!(completed.effect(), &RegistryEffect::None);
            assert!(registry.snapshot(&running_id).is_none());
            assert!(registry.begin_ordinary(&active).is_none());
        }
    }
}

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
    assert_eq!(counts.wakeups_armed().exact_total().unwrap(), 2);
    assert_eq!(counts.work_dispatched().exact_total().unwrap(), 2);
}

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

#[test]
fn watchdog_dispatches_successor_first_and_retires_unacknowledged_attempt() {
    let mut registry = registry();
    let timer = identity("watchdog");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let initial_transition = registry
        .ensure_recurring(&claim, 10)
        .expect("initial ensure should succeed");
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
    assert_eq!(counters.scheduler_started().exact_total().unwrap(), 3);
    assert_eq!(counters.work_dispatched().exact_total().unwrap(), 3);
    assert_eq!(counters.unacknowledged().exact_total().unwrap(), 1);
    assert_eq!(counters.stale_work().exact_total().unwrap(), 1);
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
        let claim = registry
            .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
            .unwrap();
        let (initial, _) = arm(registry.ensure_recurring(&claim, 0).unwrap());
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
        assert_eq!(
            pending
                .observability()
                .counters()
                .schedule_requests()
                .exact_total()
                .unwrap(),
            5
        );
        assert_eq!(
            pending
                .observability()
                .counters()
                .coalesced()
                .exact_total()
                .unwrap(),
            3
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
        assert_eq!(counters.schedule_requests().exact_total().unwrap(), 2);
        assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 1);
        assert_eq!(counters.coalesced().exact_total().unwrap(), 1);

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
        assert_eq!(counters.schedule_requests().exact_total().unwrap(), 4);
        assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 2);
        assert_eq!(counters.coalesced().exact_total().unwrap(), 2);
        assert_eq!(counters.stale_wakeups().exact_total().unwrap(), 1);
    }
}

#[test]
fn watchdog_running_immediate_request_replaces_exact_successor_and_beats_cadence() {
    let mut registry = registry();
    let timer = identity("watchdog-running-immediate");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let initial = registry
        .ensure_recurring(&claim, 10)
        .expect("initial ensure should succeed");
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
    assert_eq!(counters.schedule_requests().exact_total().unwrap(), 4);
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 3);
    assert_eq!(counters.work_dispatched().exact_total().unwrap(), 1);
    assert_eq!(counters.coalesced().exact_total().unwrap(), 3);
    assert_eq!(counters.stale_wakeups().exact_total().unwrap(), 1);
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One precedence matrix over three independent declarations."
)]
fn watchdog_completion_arbitrates_immediate_cancellation_and_unregistration() {
    let mut registry = registry();
    let continue_timer = identity("watchdog-decision-immediate");
    let continue_claim = registry
        .register_watchdog(continue_timer, cadence(5), DeclarationLifetime::Retained)
        .expect("continue claim should succeed");
    let (scheduler, _) = arm(registry
        .ensure_recurring(&continue_claim, 10)
        .expect("continue ensure should succeed"));
    let (cadence_successor, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 20));
    assert!(registry.begin_watchdog_work(&work).is_some());
    let immediate = registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            ),
        )
        .expect("immediate decision should succeed");
    assert!(matches!(
        immediate.effect(),
        RegistryEffect::ArmWakeup {
            delay_ns: 0,
            arm: WakeupArm::Replacement,
            ..
        }
    ));
    let (immediate_successor, _) = arm(immediate);
    let deadline = scheduled_deadline(&registry, &immediate_successor);
    assert_eq!(deadline, 21);
    assert_ne!(
        immediate_successor.callback_generation(),
        cadence_successor.callback_generation()
    );

    let cancel_timer = identity("watchdog-immediate-then-cancel");
    let cancel_claim = registry
        .register_watchdog(
            cancel_timer.clone(),
            cadence(5),
            DeclarationLifetime::Retained,
        )
        .expect("cancel claim should succeed");
    let (scheduler, _) = arm(registry
        .ensure_recurring(&cancel_claim, 10)
        .expect("cancel ensure should succeed"));
    let (_successor, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 20));
    assert!(registry.begin_watchdog_work(&work).is_some());
    registry
        .ensure_watchdog_immediately(&cancel_claim, 21)
        .expect("immediate request should pend");
    registry
        .cancel(&cancel_claim)
        .expect("later cancellation should pend");
    let cancelled = registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            ),
        )
        .expect("cancellation should override continuation");
    assert!(matches!(
        cancelled.effect(),
        RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Wakeup,
            ..
        }
    ));
    assert_eq!(
        registry
            .snapshot(&cancel_timer)
            .map(|snapshot| snapshot.state()),
        Some(TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Cancelled,
        })
    );

    let unregister_timer = identity("watchdog-immediate-then-unregister");
    let unregister_claim = registry
        .register_watchdog(
            unregister_timer.clone(),
            cadence(5),
            DeclarationLifetime::Retained,
        )
        .expect("unregister claim should succeed");
    let (scheduler, _) = arm(registry
        .ensure_recurring(&unregister_claim, 10)
        .expect("unregister ensure should succeed"));
    let (_successor, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 20));
    assert!(registry.begin_watchdog_work(&work).is_some());
    registry
        .ensure_watchdog_immediately(&unregister_claim, 21)
        .expect("immediate request should pend");
    registry
        .unregister(&unregister_claim)
        .expect("unregistration should pend");
    registry
        .ensure_watchdog_immediately(&unregister_claim, 21)
        .expect("unregistration should remain sticky");
    let unregistered = registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            ),
        )
        .expect("unregistration should override every continuation");
    assert!(matches!(
        unregistered.effect(),
        RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Wakeup,
            ..
        }
    ));
    assert!(registry.snapshot(&unregister_timer).is_none());
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
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .cancelled()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .stale_work()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .stale_wakeups()
            .exact_total()
            .unwrap(),
        1
    );

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
        control.generation = u64::MAX;
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
        control.generation = u64::MAX - 1;
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
            control.generation,
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
            let claim = match policy {
                TimerPolicy::Once => registry.register_once(timer.clone(), lifetime).unwrap(),
                TimerPolicy::AfterCompletion { cadence } => registry
                    .register_after_completion(timer.clone(), cadence, lifetime)
                    .unwrap(),
                TimerPolicy::Watchdog { .. } => unreachable!("ordinary fixture policies"),
            };
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
            control.generation = u64::MAX - if dispatched { 2 } else { 1 };
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
                assert_eq!(control.generation, u64::MAX);
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
                let claim = match policy {
                    TimerPolicy::Once => registry.register_once(timer.clone(), lifetime).unwrap(),
                    TimerPolicy::AfterCompletion { cadence } => registry
                        .register_after_completion(timer.clone(), cadence, lifetime)
                        .unwrap(),
                    TimerPolicy::Watchdog { .. } => unreachable!("ordinary fixture policies"),
                };
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
                "initial" => control.generation = u64::MAX,
                "replacement" | "dispatch-generation" => {
                    control.generation = u64::MAX - 1;
                }
                "completion-immediate" | "completion-deadline" => {
                    control.generation = u64::MAX - 2;
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
            control.generation = u64::MAX - 2;
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

#[test]
fn initial_snapshots_are_policy_specific_and_coherent() {
    let mut registry = registry();
    let once_id = identity("a-once");
    let after_id = identity("b-after");
    let watchdog_id = identity("c-watchdog");
    registry
        .register_once(once_id.clone(), DeclarationLifetime::Retained)
        .expect("once registration should succeed");
    registry
        .register_after_completion(after_id.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("after-completion registration should succeed");
    registry
        .register_watchdog(
            watchdog_id.clone(),
            cadence(7),
            DeclarationLifetime::Retained,
        )
        .expect("watchdog registration should succeed");

    let inventory = registry.inventory();
    assert_eq!(inventory.epoch(), TimerEpoch::new(7, 10));
    assert_eq!(inventory.len(), 3);
    assert_eq!(inventory.timers()[0].identity(), &once_id);
    assert_eq!(inventory.timers()[1].identity(), &after_id);
    assert_eq!(inventory.timers()[2].identity(), &watchdog_id);
    for snapshot in inventory.timers() {
        assert_eq!(
            snapshot.state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::NeverScheduled,
            }
        );
        assert_eq!(snapshot.next_deadline_ns(), None);
        assert_eq!(snapshot.generation(), None);
        assert_eq!(snapshot.registration_id().epoch(), inventory.epoch());
        assert_eq!(
            registry.snapshot(snapshot.identity()).as_ref(),
            Some(snapshot)
        );
    }
}

#[test]
fn registration_sequence_exhaustion_never_reuses_a_snapshot_identity() {
    let mut registry = registry();
    registry.next_claim_generation = u64::MAX - 1;
    let timer = identity("last-registration");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .unwrap();
    let last = registry.snapshot(&timer).unwrap().registration_id();
    assert_eq!(last.sequence(), u64::MAX);
    registry.unregister(&claim).unwrap();
    assert_eq!(
        registry.register_once(timer.clone(), DeclarationLifetime::Retained),
        Err(RegisterError::ClaimGenerationExhausted)
    );
    assert!(registry.snapshot(&timer).is_none());
}

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
    control.generation = u64::MAX - 3;
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
