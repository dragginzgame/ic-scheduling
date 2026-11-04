use super::*;

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
