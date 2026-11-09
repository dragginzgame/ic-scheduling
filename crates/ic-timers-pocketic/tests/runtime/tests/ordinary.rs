//! Installed ordinary qualification subjects.

use super::*;

#[derive(Clone, Copy, Eq, PartialEq)]
enum OrdinaryCommand {
    Keep,
    Cancel,
    Reconcile,
    Unregister,
}

impl OrdinaryCommand {
    const ALL: [Self; 4] = [Self::Keep, Self::Cancel, Self::Reconcile, Self::Unregister];

    async fn apply(self, pic: &PocketIc, canister_id: ExternalPrincipal, deadline: u64) {
        match self {
            Self::Keep => {}
            Self::Cancel => update_unit(pic, canister_id, Method::CancelOrdinary).await,
            Self::Reconcile => {
                update_deadline(pic, canister_id, Method::ReconcileOrdinaryAt, deadline).await
            }
            Self::Unregister => update_unit(pic, canister_id, Method::UnregisterOrdinary).await,
        }
    }
}

#[tokio::test]
async fn once_frozen_await_failure_retires_without_replay() {
    assert_frozen_await_failure_retires(OrdinaryPolicy::Once).await;
}

#[tokio::test]
async fn after_completion_frozen_await_failure_retires_without_replay() {
    assert_frozen_await_failure_retires(OrdinaryPolicy::AfterCompletion).await;
}

async fn assert_frozen_await_failure_retires(policy: OrdinaryPolicy) {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    let started_at = now_ns(pic).await;
    let budget = freezing_budget(pic, canister_id).await;
    start_ordinary(pic, canister_id, policy, ProbeLifetime::Retained).await;
    let suspended = wait_for_probe(
        pic,
        32,
        async |pic| ordinary_observation(pic, canister_id).await,
        |s| s.running && s.waiting && s.gate_replies > 0,
    )
    .await;
    assert!(suspended.registration_id.is_some());
    assert_eq!(suspended.work_started.total, 1);
    assert_eq!(suspended.completed, 0);
    assert_eq!(suspended.next_deadline_ns, None);
    assert_eq!(suspended.gate_error, None);

    freeze_for_five_minutes(pic, canister_id, budget.seconds, budget.reserve).await;
    pic.add_cycles(canister_id, budget.balance * 16)
        .await
        .expect("fund canister cycles");
    // Query before another replica round: funding cannot hide a completion,
    // abandoned attempt or ownership change that already happened while frozen.
    let thawed = ordinary_observation(pic, canister_id).await;
    assert_eq!(thawed.registration_id, suspended.registration_id);
    assert!(thawed.declared, "{thawed:?}");
    assert_eq!(
        thawed.termination,
        Some(OrdinaryTermination::InvariantFailure)
    );
    assert!(!thawed.running && !thawed.waiting && !thawed.armed);
    assert_eq!(thawed.work_started.total, 1);
    // A pre-funded response can resume while frozen. This probe's next call
    // fails for lack of liquid cycles; its callback returns InvariantFailure.
    // That is a normally completed failed attempt, not delivery abandonment.
    assert_eq!(thawed.work_completed.total, 1);
    assert_eq!(thawed.completed, 0);
    assert_eq!(thawed.completed_at_ns, None);
    assert_eq!(thawed.next_deadline_ns, None);
    assert_eq!(thawed.unacknowledged.total, 0);
    assert!(
        thawed.gate_error.as_ref().is_some_and(
            |error| error.starts_with("insufficient liquid cycles balance, available: 0,")
        ),
        "{thawed:?}"
    );
    assert!(update_bool(pic, canister_id, Method::ExpiredOrdinaryContextRejected).await);
    advance(pic, Duration::from_secs(1), 16).await;
    let settled = ordinary_observation(pic, canister_id).await;
    assert_eq!(settled.registration_id, suspended.registration_id);
    assert_eq!(
        settled.termination,
        Some(OrdinaryTermination::InvariantFailure)
    );
    assert!(!settled.running && !settled.armed);
    assert_eq!(
        settled.work_started.total, 1,
        "funding must not replay accepted work"
    );
    assert_eq!(settled.work_completed.total, 1);
    assert_eq!(settled.completed, 0);
    assert_eq!(settled.unacknowledged.total, 0);

    // The application explicitly schedules a fresh attempt after funding and
    // permits its awaited operation to succeed. No automatic retry is claimed.
    update_unit(pic, canister_id, Method::ReleaseOrdinaryWork).await;
    update_deadline(
        pic,
        canister_id,
        Method::ReconcileOrdinaryAt,
        now_ns(pic).await,
    )
    .await;
    let completed = wait_for_probe(
        pic,
        32,
        async |pic| ordinary_observation(pic, canister_id).await,
        |s| s.completed == 1,
    )
    .await;
    assert_eq!(completed.registration_id, suspended.registration_id);
    assert!(!completed.running && !completed.waiting);
    assert!(!matches!(
        completed.termination,
        Some(OrdinaryTermination::InvariantFailure | OrdinaryTermination::Abandoned)
    ));
    assert_eq!(completed.work_started.total, 2);
    assert_eq!(completed.work_completed.total, 2);
    assert_eq!(completed.unacknowledged.total, 0);
    // This existing diagnostic keeps the most recent gate error across attempts.
    assert_eq!(completed.gate_error, thawed.gate_error);
    assert!(completed.gate_replies > suspended.gate_replies);
    if policy == OrdinaryPolicy::Once {
        assert!(!completed.armed);
        assert_eq!(completed.next_deadline_ns, None);
        advance(pic, Duration::from_secs(5), 16).await;
        let settled = ordinary_observation(pic, canister_id).await;
        assert_eq!(settled.completed, 1);
        assert_eq!(settled.work_started.total, 2);
        assert_eq!(settled.work_completed.total, 2);
    } else {
        let completed_at = completed.completed_at_ns.unwrap();
        assert!(completed_at >= started_at + 300_000_000_000);
        assert_eq!(
            completed.next_deadline_ns,
            Some(completed_at + 1_000_000_000)
        );
        advance(pic, Duration::ZERO, 16).await;
        assert_eq!(
            ordinary_observation(pic, canister_id)
                .await
                .work_started
                .total,
            2
        );
        pic.advance_time(Duration::from_secs(1))
            .await
            .expect("advance IC time");
        let recurring = wait_for_probe(
            pic,
            32,
            async |pic| ordinary_observation(pic, canister_id).await,
            |s| s.completed == 2,
        )
        .await;
        assert_eq!(recurring.registration_id, suspended.registration_id);
        assert_eq!(recurring.work_started.total, 3);
        assert_eq!(recurring.work_completed.total, 3);
        assert!(!recurring.running && !recurring.waiting);
        assert_eq!(recurring.unacknowledged.total, 0);
        assert_eq!(recurring.gate_error, thawed.gate_error);
        assert_eq!(
            recurring.next_deadline_ns,
            Some(recurring.completed_at_ns.unwrap() + 1_000_000_000)
        );
    }
    let after = pic.canister_status(canister_id, None).await.unwrap();
    assert_eq!(after.module_hash, budget.module_hash);
    assert_eq!(after.settings.freezing_threshold, budget.seconds);

    close_instance(owned).await;
}

// This checks the real provider and message boundary for the two ordinary policies.
#[tokio::test]
async fn absent_ordinary_demand_is_idle_and_explicit_cancellation_stays_distinct() {
    for policy in OrdinaryPolicy::ALL {
        let (owned, canister_id) = installed_probe().await;
        let pic = owned.instance();
        start_ordinary(pic, canister_id, policy, ProbeLifetime::Retained).await;
        let running = wait_for_probe(
            pic,
            32,
            async |pic| ordinary_observation(pic, canister_id).await,
            |state| state.running && state.waiting && state.gate_replies > 0,
        )
        .await;
        update_unit(pic, canister_id, Method::ReconcileOrdinaryInactive).await;
        assert!(
            ordinary_observation(pic, canister_id).await.running,
            "demand removal must not interrupt accepted work"
        );
        update_unit(pic, canister_id, Method::ReleaseOrdinaryWork).await;
        let stopped = wait_for_probe(
            pic,
            32,
            async |pic| ordinary_observation(pic, canister_id).await,
            |state| state.completed == 1 && !state.running,
        )
        .await;
        assert!(stopped.declared && !stopped.armed, "{stopped:?}");
        assert_eq!(stopped.termination, Some(OrdinaryTermination::Stopped));
        assert_eq!(stopped.registration_id, running.registration_id);
        assert_eq!(stopped.cancellations.total, 0);
        assert_eq!(stopped.work_completed.total, 1);
        assert!(update_bool(pic, canister_id, Method::ExpiredOrdinaryContextRejected).await);
        qualify_scheduled_demand_removal(pic, canister_id, running.registration_id).await;
        update_deadline(
            pic,
            canister_id,
            Method::ReconcileOrdinaryAt,
            now_ns(pic).await,
        )
        .await;
        let rearmed = wait_for_probe(
            pic,
            32,
            async |pic| ordinary_observation(pic, canister_id).await,
            |state| state.completed == 2,
        )
        .await;
        assert_eq!(rearmed.registration_id, running.registration_id);
        assert_eq!(rearmed.work_started.total, 2);
        assert_eq!(rearmed.work_completed.total, 2);
        assert_eq!(rearmed.cancellations.total, 1);

        close_instance(owned).await;
    }
}

async fn qualify_scheduled_demand_removal(
    pic: &PocketIc,
    canister_id: ExternalPrincipal,
    identity: Option<(u64, u64, u64)>,
) {
    let deadline = now_ns(pic).await + 60_000_000_000;
    update_deadline(pic, canister_id, Method::ReconcileOrdinaryAt, deadline).await;
    let armed = ordinary_observation(pic, canister_id).await;
    assert!(armed.armed);
    assert_eq!(armed.next_deadline_ns, Some(deadline));
    update_unit(pic, canister_id, Method::ReconcileOrdinaryInactive).await;
    let idle = ordinary_observation(pic, canister_id).await;
    assert_eq!(idle.termination, Some(OrdinaryTermination::Stopped));
    assert!(!idle.armed);
    assert_eq!(idle.next_deadline_ns, None);
    assert_eq!(idle.cancellations.total, 0);
    assert_eq!(idle.registration_id, identity);
    update_deadline(pic, canister_id, Method::ReconcileOrdinaryAt, deadline).await;
    update_unit(pic, canister_id, Method::CancelOrdinary).await;
    let cancelled = ordinary_observation(pic, canister_id).await;
    assert_eq!(cancelled.termination, Some(OrdinaryTermination::Cancelled));
    assert!(!cancelled.armed);
    assert_eq!(cancelled.cancellations.total, 1);
    update_unit(pic, canister_id, Method::ReconcileOrdinaryInactive).await;
    let still_cancelled = ordinary_observation(pic, canister_id).await;
    assert_eq!(
        still_cancelled.termination,
        Some(OrdinaryTermination::Cancelled)
    );
    assert_eq!(still_cancelled.cancellations.total, 1);
    assert_eq!(still_cancelled.registration_id, identity);
    assert_eq!(still_cancelled.work_started.total, 1);
}

#[tokio::test]
async fn ordinary_await_allows_ingress_and_other_timers_before_completion() {
    for policy in OrdinaryPolicy::ALL {
        for command in OrdinaryCommand::ALL {
            if command == OrdinaryCommand::Keep && policy == OrdinaryPolicy::Once {
                continue;
            }
            let (owned, canister_id) = installed_probe().await;
            let pic = owned.instance();
            let lifetime = if command == OrdinaryCommand::Cancel {
                ProbeLifetime::RemoveWhenStopped
            } else {
                ProbeLifetime::Retained
            };
            start_ordinary(pic, canister_id, policy, lifetime).await;
            let suspended = wait_for_probe(
                pic,
                32,
                async |pic| ordinary_observation(pic, canister_id).await,
                |current| current.running && current.waiting && current.gate_replies > 0,
            )
            .await;
            assert_eq!(suspended.gate_error, None);
            assert!(update_bool(pic, canister_id, Method::Start).await);
            advance(pic, Duration::from_secs(2), 16).await;
            assert_eq!(
                snapshot(pic, canister_id).await.completed_work,
                1,
                "other timer must progress while ordinary work awaits"
            );
            let waiting = ordinary_observation(pic, canister_id).await;
            assert!(waiting.running);
            assert!(waiting.waiting);
            assert_eq!(waiting.gate_error, None);
            assert_eq!(waiting.completed, 0);
            assert_eq!(waiting.work_completed.total, 0);
            let exact_deadline =
                snapshot(pic, canister_id).await.next_deadline_ns.unwrap() + 60_000_000_000;
            command.apply(pic, canister_id, exact_deadline).await;
            assert!(
                ordinary_observation(pic, canister_id).await.running,
                "control does not interrupt consumer work"
            );
            update_unit(pic, canister_id, Method::ReleaseOrdinaryWork).await;
            advance(pic, Duration::ZERO, 16).await;
            let completed = ordinary_observation(pic, canister_id).await;
            assert_eq!(completed.completed, 1);
            assert!(!completed.running);
            assert!(!completed.waiting);
            assert_eq!(completed.gate_error, None);
            assert!(completed.gate_replies > suspended.gate_replies);
            if matches!(
                command,
                OrdinaryCommand::Cancel | OrdinaryCommand::Unregister
            ) {
                assert!(!completed.declared);
                assert_eq!(completed.next_deadline_ns, None);
            } else {
                assert!(completed.declared);
                assert_eq!(completed.work_completed.total, 1);
                let expected = if command == OrdinaryCommand::Reconcile {
                    exact_deadline
                } else {
                    completed.completed_at_ns.unwrap() + 1_000_000_000
                };
                assert_eq!(completed.next_deadline_ns, Some(expected));
            }

            close_instance(owned).await;
        }
    }
}

async fn trapping_ordinary_probe(
    policy: OrdinaryPolicy,
    lifetime: ProbeLifetime,
    before_await: bool,
) -> (PocketIcInstance, ExternalPrincipal) {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    update_wire::<(), _>(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::TrapOrdinaryWork,
        (before_await,),
    )
    .await
    .expect("probe update succeeds");
    start_ordinary(pic, canister_id, policy, lifetime).await;
    (owned, canister_id)
}

fn assert_ordinary_abandonment(observed: &OrdinaryObservation, removed: bool) {
    assert!(
        !observed.running,
        "discarded futures must release execution authority: {observed:?}"
    );
    assert!(!observed.armed);
    assert_eq!(observed.declared, !removed);
    assert_eq!(
        observed.termination,
        (!removed).then_some(OrdinaryTermination::Abandoned)
    );
    assert_eq!(observed.next_deadline_ns, None);
    assert_eq!(observed.completed, 0);
    assert_eq!(observed.work_completed.total, 0);
    assert_eq!(observed.instruction_samples, 0);
    assert_eq!(observed.unacknowledged.total, u128::from(!removed));
    assert_eq!(observed.captures_dropped, u64::from(removed));
    assert!(observed.capture_registry_accessible);
    assert_eq!(observed.inventory_len, u64::from(!removed));
}

#[tokio::test]
async fn dispatch_response_trap_cleanup_preserves_pending_work_for_retry() {
    for policy in OrdinaryPolicy::ALL {
        let (owned, canister_id) = installed_probe().await;
        let pic = owned.instance();
        update_wire::<(), _>(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::TrapOrdinaryCaptureDrop,
            (true,),
        )
        .await
        .expect("enable destructor trap");
        update_wire::<(), _>(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::TrapOrdinaryWork,
            (true,),
        )
        .await
        .expect("enable pre-await trap");
        start_ordinary(pic, canister_id, policy, ProbeLifetime::RemoveWhenStopped).await;
        advance(pic, Duration::ZERO, 32).await;
        let pending = ordinary_observation(pic, canister_id).await;
        assert!(pending.declared && pending.armed);
        assert!(!pending.running);
        assert_eq!(pending.captures_dropped, 0);
        assert_eq!(pending.completed, 0);

        update_wire::<(), _>(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::TrapOrdinaryCaptureDrop,
            (false,),
        )
        .await
        .expect("disable destructor trap");
        advance(pic, Duration::ZERO, 8).await;
        assert!(ordinary_observation(pic, canister_id).await.declared);
        advance(pic, Duration::from_secs(1), 32).await;
        assert_ordinary_abandonment(&ordinary_observation(pic, canister_id).await, true);

        // Confirm the backend still dispatches independent work after cleanup.
        assert!(update_bool(pic, canister_id, Method::Start).await);
        advance(pic, Duration::from_secs(2), 16).await;
        assert_eq!(snapshot(pic, canister_id).await.completed_work, 1);

        close_instance(owned).await;
    }
}

#[tokio::test]
async fn ordinary_trap_before_await_retires_the_rolled_back_scheduled_delivery() {
    for policy in OrdinaryPolicy::ALL {
        for lifetime in ProbeLifetime::ALL {
            let (owned, canister_id) = trapping_ordinary_probe(policy, lifetime, true).await;
            let pic = owned.instance();
            advance(pic, Duration::ZERO, 32).await;
            let observed = ordinary_observation(pic, canister_id).await;
            assert_ordinary_abandonment(&observed, lifetime == ProbeLifetime::RemoveWhenStopped);
            assert_eq!(observed.gate_replies, 0);
            assert!(update_bool(pic, canister_id, Method::Start).await);
            advance(pic, Duration::from_secs(2), 16).await;
            assert_eq!(snapshot(pic, canister_id).await.completed_work, 1);
            if lifetime == ProbeLifetime::Retained {
                update_unit(pic, canister_id, Method::UnregisterOrdinary).await;
                assert_eq!(
                    ordinary_observation(pic, canister_id)
                        .await
                        .captures_dropped,
                    1
                );
            }

            close_instance(owned).await;
        }
    }
}

#[tokio::test]
async fn ordinary_continuation_trap_retires_running_work_and_releases_expired_authority() {
    for policy in OrdinaryPolicy::ALL {
        for lifetime in ProbeLifetime::ALL {
            for command in OrdinaryCommand::ALL {
                let (owned, canister_id) = trapping_ordinary_probe(policy, lifetime, false).await;
                let pic = owned.instance();
                wait_for_probe(
                    pic,
                    32,
                    async |pic| ordinary_observation(pic, canister_id).await,
                    |current| current.running && current.waiting && current.gate_replies > 0,
                )
                .await;
                command.apply(pic, canister_id, u64::MAX).await;
                update_unit(pic, canister_id, Method::ReleaseOrdinaryWork).await;
                advance(pic, Duration::ZERO, 32).await;
                let observed = ordinary_observation(pic, canister_id).await;
                let removed = lifetime == ProbeLifetime::RemoveWhenStopped
                    || command == OrdinaryCommand::Unregister;
                assert_ordinary_abandonment(&observed, removed);
                assert_eq!(observed.gate_error, None);
                assert!(
                    update_bool(pic, canister_id, Method::ExpiredOrdinaryContextRejected).await
                );
                update_unit(pic, canister_id, Method::ClearOrdinaryTrap).await;
                if removed {
                    if command != OrdinaryCommand::Unregister {
                        update_unit(pic, canister_id, Method::DiscardExpiredOrdinaryClaim).await;
                    }
                    start_ordinary(pic, canister_id, policy, ProbeLifetime::Retained).await;
                } else {
                    // The original retained capability can rearm explicitly.
                    update_deadline(pic, canister_id, Method::ReconcileOrdinaryAt, 0).await;
                }
                advance(pic, Duration::ZERO, 16).await;
                let completed = ordinary_observation(pic, canister_id).await;
                assert_eq!(completed.completed, 1);
                assert_eq!(completed.work_completed.total, 1);
                assert_eq!(completed.instruction_samples, 1);
                assert_eq!(completed.unacknowledged.total, u128::from(!removed));
                update_unit(pic, canister_id, Method::UnregisterOrdinary).await;
                assert_eq!(
                    ordinary_observation(pic, canister_id)
                        .await
                        .captures_dropped,
                    if removed { 2 } else { 1 }
                );
                assert!(update_bool(pic, canister_id, Method::Start).await);
                advance(pic, Duration::from_secs(2), 16).await;
                assert_eq!(snapshot(pic, canister_id).await.completed_work, 1);

                close_instance(owned).await;
            }
        }
    }
}

#[tokio::test]
async fn accepted_ordinary_work_survives_dispatch_timeout_without_replay() {
    let (owned, canister_id) = installed_probe().await;
    let pic = owned.instance();
    start_ordinary(
        pic,
        canister_id,
        OrdinaryPolicy::Once,
        ProbeLifetime::Retained,
    )
    .await;
    let waiting = wait_for_probe(
        pic,
        20,
        async |pic| ordinary_observation(pic, canister_id).await,
        |state| state.waiting,
    )
    .await;
    assert!(waiting.running);
    // The first pending task identity is 1 in this fresh canister. Acceptance
    // consumed it before the ordinary callback awaited its application gate.
    let accepted: bool = update_wire(
        pic,
        canister_id,
        ExternalPrincipal::anonymous(),
        Method::FixtureDispatchPayload,
        (1_u64.to_le_bytes().to_vec(),),
    )
    .await
    .unwrap();
    assert!(accepted);
    advance(pic, Duration::from_secs(301), 20).await;
    let timed_out = ordinary_observation(pic, canister_id).await;
    assert!(timed_out.running);
    assert_eq!(timed_out.completed, 0);
    update_unit(pic, canister_id, Method::ReleaseOrdinaryWork).await;
    let completed = wait_for_probe(
        pic,
        30,
        async |pic| ordinary_observation(pic, canister_id).await,
        |state| state.completed == 1,
    )
    .await;
    assert!(!completed.running);
    assert_eq!(completed.gate_error, None);
    advance(pic, Duration::ZERO, 8).await;
    assert_eq!(ordinary_observation(pic, canister_id).await.completed, 1);

    close_instance(owned).await;
}
