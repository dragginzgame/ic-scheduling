use super::*;
use crate::{
    TimerCadence, TimerSchedule,
    callback::OrdinaryRunResult,
    registry::{RegistryEffect, TimerRegistry},
    schedule::OrdinaryDirective,
};
use std::time::Duration;

fn identity(owner: &str, subsystem: &str, name: &str) -> TimerIdentity {
    TimerIdentity::try_new(owner, subsystem, name).expect("fixture identity should be valid")
}

#[test]
fn identity_components_are_bounded_in_utf8_bytes() {
    assert!(
        TimerIdentity::try_new(
            "owner",
            "subsystem",
            "a".repeat(MAX_TIMER_IDENTITY_COMPONENT_BYTES),
        )
        .is_ok()
    );
    assert_eq!(
        TimerIdentity::try_new(
            "owner",
            "subsystem",
            "a".repeat(MAX_TIMER_IDENTITY_COMPONENT_BYTES + 1),
        ),
        Err(TimerIdentityError::TooLong {
            field: TimerIdentityField::Name,
            actual_bytes: MAX_TIMER_IDENTITY_COMPONENT_BYTES + 1,
            max_bytes: MAX_TIMER_IDENTITY_COMPONENT_BYTES,
        })
    );
    assert!(
        TimerIdentity::try_new(
            "owner",
            "subsystem",
            "é".repeat(MAX_TIMER_IDENTITY_COMPONENT_BYTES / 2),
        )
        .is_ok()
    );
    assert!(matches!(
        TimerIdentity::try_new(
            "owner",
            "subsystem",
            "é".repeat(MAX_TIMER_IDENTITY_COMPONENT_BYTES / 2 + 1),
        ),
        Err(TimerIdentityError::TooLong {
            field: TimerIdentityField::Name,
            ..
        })
    ));
}

#[test]
fn identity_components_reject_ambiguous_operator_text() {
    assert_eq!(
        TimerIdentity::try_new("", "subsystem", "name"),
        Err(TimerIdentityError::Empty {
            field: TimerIdentityField::Owner,
        })
    );
    assert_eq!(
        TimerIdentity::try_new("owner", " subsystem", "name"),
        Err(TimerIdentityError::SurroundingWhitespace {
            field: TimerIdentityField::Subsystem,
        })
    );
    assert_eq!(
        TimerIdentity::try_new("owner", "subsystem", "timer\nname"),
        Err(TimerIdentityError::ControlCharacter {
            field: TimerIdentityField::Name,
            byte_index: 5,
            character: '\n',
        })
    );
}

#[test]
fn identities_validate_components_and_order_deterministically() {
    assert_eq!(
        TimerIdentity::try_new("canic", "", "renewal"),
        Err(TimerIdentityError::Empty {
            field: TimerIdentityField::Subsystem,
        })
    );

    let mut identities = [
        identity("icydb", "recovery", "drive"),
        identity("canic", "cycles", "top_up"),
        identity("canic", "auth", "renewal"),
        identity("canic", "auth", "cleanup"),
    ];
    identities.sort();
    let components = identities
        .iter()
        .map(|value| (value.owner(), value.subsystem(), value.name()))
        .collect::<Vec<_>>();
    assert_eq!(
        components,
        vec![
            ("canic", "auth", "cleanup"),
            ("canic", "auth", "renewal"),
            ("canic", "cycles", "top_up"),
            ("icydb", "recovery", "drive"),
        ]
    );
}

#[test]
fn identity_clones_preserve_value_semantics_and_outlive_the_original() {
    let original = identity("canic", "auth", "renewal");
    let cloned = original.clone();
    drop(original);
    let independently_created = identity("canic", "auth", "renewal");
    let _: &(dyn Send + Sync) = &cloned;

    assert_eq!(cloned, independently_created);
}

#[test]
fn policies_preserve_validated_cadence() {
    let cadence = TimerCadence::from_nanos(5_000).expect("fixture cadence should be valid");
    let policy = TimerPolicy::Watchdog { cadence };
    assert_eq!(policy.label(), "watchdog");
    assert_eq!(policy.cadence().map(TimerCadence::as_nanos), Some(5_000));
}

#[test]
fn canonical_registry_snapshot_keeps_state_and_measurements_coherent() {
    let mut registry = TimerRegistry::new(TimerEpoch::new(4, 100));
    let timer = identity("canic", "auth", "renewal");
    let cadence = TimerCadence::from_nanos(1_000_000_000).expect("fixture cadence should be valid");
    let claim = registry
        .register_after_completion(timer.clone(), cadence, DeclarationLifetime::Retained)
        .expect("registration should succeed");
    let initial_transition = registry
        .ensure_recurring(&claim, 100)
        .expect("ensure should succeed");
    registry
        .confirm_effect_applied(initial_transition.effect())
        .expect("fixture provider effect should apply");
    let token = match initial_transition.into_effect() {
        RegistryEffect::ArmWakeup { token, .. } => token.token().clone(),
        effect => panic!("expected arm effect, got {effect:?}"),
    };
    assert!(registry.begin_ordinary(&token).is_some());
    let completion_transition = registry
        .complete_ordinary(
            &token,
            200,
            OrdinaryRunResult::new(
                TimerCompletion::retryable_failure(2),
                OrdinaryDirective::RetryAfter(Duration::from_secs(2)),
            ),
        )
        .expect("completion should succeed");
    registry
        .confirm_effect_applied(completion_transition.effect())
        .expect("fixture provider effect should apply");
    registry
        .ensure_recurring(&claim, 200)
        .expect("coalesced demand should succeed");
    registry
        .ensure_recurring(&claim, 200)
        .expect("repeated coalesced demand should succeed");

    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(
        snapshot.registration_id(),
        TimerRegistrationId::new(TimerEpoch::new(4, 100), 1)
    );
    assert_eq!(snapshot.identity(), &timer);
    assert_eq!(snapshot.policy(), TimerPolicy::AfterCompletion { cadence });
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(2_000_000_000));
    assert_eq!(snapshot.scheduling_mode(), TimerSchedulingMode::Retry);
    assert_eq!(
        snapshot.registration_status(),
        TimerRegistrationStatus::Scheduled
    );
    assert_eq!(snapshot.generation(), Some(2));
    assert_eq!(snapshot.next_deadline_ns(), Some(2_000_000_200));
    let observations = snapshot.observability();
    let counters = observations.counters();
    let outcomes = observations.outcomes();
    assert_eq!(
        outcomes.last_outcome(),
        Some(TimerLastOutcome::Completed(
            TimerCompletionOutcome::RetryableFailure
        ))
    );
    assert_eq!(outcomes.last_work_count(), Some(2));
    assert_eq!(outcomes.last_failure_at_ns(), Some(200));
    assert_eq!(outcomes.consecutive_expected_failures(), 1);
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 2);
    assert_eq!(counters.work_started().exact_total().unwrap(), 1);
    assert_eq!(counters.work_completed().exact_total().unwrap(), 1);
    assert_eq!(counters.succeeded().exact_total().unwrap(), 0);
    assert_eq!(counters.no_work().exact_total().unwrap(), 0);
    assert_eq!(counters.retryable_failure().exact_total().unwrap(), 1);
    assert_eq!(counters.invariant_failure().exact_total().unwrap(), 0);
    assert_eq!(counters.stale_wakeups().exact_total().unwrap(), 0);
    assert_eq!(counters.stale_work().exact_total().unwrap(), 0);
    assert_eq!(
        observations
            .performance()
            .work_instructions()
            .aggregate()
            .total(),
        0
    );
}

#[test]
fn completed_and_stale_observations_retain_wide_measurements() {
    let mut registry = TimerRegistry::new(TimerEpoch::new(6, 0));
    let timer = identity("canic", "projection", "classes");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("registration should succeed");
    let first_transition = registry
        .ensure_once(&claim, 0, TimerSchedule::At(1))
        .expect("ensure should succeed");
    registry
        .confirm_effect_applied(first_transition.effect())
        .expect("fixture provider effect should apply");
    let first = match first_transition.into_effect() {
        RegistryEffect::ArmWakeup { token, .. } => token.token().clone(),
        effect => panic!("expected arm effect, got {effect:?}"),
    };
    assert!(registry.begin_ordinary(&first).is_some());
    let second_transition = registry
        .complete_ordinary(
            &first,
            2,
            OrdinaryRunResult::new(
                TimerCompletion::success(1),
                OrdinaryDirective::ContinueImmediately,
            ),
        )
        .expect("continuation should succeed");
    registry
        .confirm_effect_applied(second_transition.effect())
        .expect("fixture provider effect should apply");
    let pages = crate::MemoryPageExtent::new(0, 0);
    registry
        .record_callback_measurements(&first, u64::MAX, pages, pages)
        .expect("completed work measurements should be accepted");
    let second = match second_transition.into_effect() {
        RegistryEffect::ArmWakeup { token, .. } => token.token().clone(),
        effect => panic!("expected arm effect, got {effect:?}"),
    };
    assert!(registry.begin_ordinary(&second).is_some());
    registry
        .complete_ordinary(
            &second,
            3,
            OrdinaryRunResult::new(TimerCompletion::no_work(), OrdinaryDirective::Stop),
        )
        .expect("terminal completion should succeed");
    registry
        .record_callback_measurements(&second, 1, pages, pages)
        .expect("wide cumulative measurements should be accepted");
    assert!(registry.begin_ordinary(&first).is_none());

    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    let observations = snapshot.observability();
    let counters = observations.counters();
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 2);
    assert_eq!(counters.work_started().exact_total().unwrap(), 2);
    assert_eq!(counters.work_completed().exact_total().unwrap(), 2);
    assert_eq!(counters.succeeded().exact_total().unwrap(), 1);
    assert_eq!(counters.no_work().exact_total().unwrap(), 1);
    assert_eq!(counters.stale_wakeups().exact_total().unwrap(), 1);
    assert_eq!(counters.stale_work().exact_total().unwrap(), 0);
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
    assert_eq!(snapshot.generation(), None);
    assert_eq!(
        observations
            .performance()
            .work_instructions()
            .aggregate()
            .total(),
        u128::from(u64::MAX) + 1
    );
}

#[test]
fn retryable_terminal_completion_preserves_reported_failure() {
    let mut registry = TimerRegistry::new(TimerEpoch::new(5, 100));
    let timer = identity("canic", "cycles", "topup");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("registration should succeed");
    let transition = registry
        .ensure_once(&claim, 100, TimerSchedule::At(101))
        .expect("ensure should succeed");
    registry
        .confirm_effect_applied(transition.effect())
        .expect("fixture provider effect should apply");
    let token = match transition.into_effect() {
        RegistryEffect::ArmWakeup { token, .. } => token.token().clone(),
        effect => panic!("expected arm effect, got {effect:?}"),
    };
    assert!(registry.begin_ordinary(&token).is_some());
    registry
        .complete_ordinary(
            &token,
            102,
            OrdinaryRunResult::new(
                TimerCompletion::retryable_failure(0),
                OrdinaryDirective::Stop,
            ),
        )
        .expect("terminal retryable completion should succeed");

    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped
        }
    );
    assert_eq!(
        snapshot.observability().outcomes().last_outcome(),
        Some(TimerLastOutcome::Completed(
            TimerCompletionOutcome::RetryableFailure
        ))
    );
    assert_eq!(snapshot.generation(), None);
}

#[test]
fn absolute_past_deadline_remains_coherent_and_arms_with_zero_delay() {
    let mut registry = TimerRegistry::new(TimerEpoch::new(1, 0));
    let timer = identity("test", "snapshot", "past");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("registration should succeed");
    let transition = registry
        .ensure_once(&claim, 100, TimerSchedule::At(50))
        .expect("past deadline should be valid");
    registry
        .confirm_effect_applied(transition.effect())
        .expect("fixture provider effect should apply");
    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(50));
    assert_eq!(snapshot.latest_requested_delay_ns(), None);
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
}
