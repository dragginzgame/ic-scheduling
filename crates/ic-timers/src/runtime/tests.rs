use super::*;
use crate::{
    AfterCompletionDecision, InactiveReason, MemoryPageSample, OnceDecision, TimerLastOutcome,
    TimerPolicy, TimerRegistrationStatus, TimerRuntimeStateSnapshot, TimerSchedulingMode,
    WatchdogDecision, WatchdogRuntimeStateSnapshot,
    platform::{
        advance_instructions, discard_next_due, grow_memory_pages, run_next_due, set_time,
        timer_count,
    },
};
use std::{
    cell::{Cell, RefCell},
    panic::{AssertUnwindSafe, catch_unwind},
    rc::Rc,
    task::{Poll, Waker},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TestPolicy {
    Once,
    AfterCompletion,
    Watchdog,
}

impl TestPolicy {
    const ALL: [Self; 3] = [Self::Once, Self::AfterCompletion, Self::Watchdog];
}

// Share only immediate stopping callbacks. Suspension and reentrant callback
// subjects keep their own bodies and authority checks.
fn stopping_once(
    mut work: impl FnMut() + 'static,
) -> impl FnMut(OnceContext) -> std::future::Ready<OnceRunResult> {
    move |_| {
        work();
        std::future::ready(OnceRunResult::new(
            TimerCompletion::no_work(),
            OnceDecision::Stop,
        ))
    }
}

fn stopping_after_completion(
    mut work: impl FnMut() + 'static,
) -> impl FnMut(AfterCompletionContext) -> std::future::Ready<AfterCompletionRunResult> {
    move |_| {
        work();
        std::future::ready(AfterCompletionRunResult::new(
            TimerCompletion::no_work(),
            AfterCompletionDecision::Stop,
        ))
    }
}

fn stopping_watchdog(
    mut work: impl FnMut() + 'static,
) -> impl FnMut(WatchdogContext) -> WatchdogRunResult {
    move |_| {
        work();
        WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
    }
}

#[derive(Clone, Default)]
struct SuspendedWork {
    ready: Rc<Cell<bool>>,
    waker: Rc<RefCell<Option<Waker>>>,
    polls: Rc<Cell<u64>>,
}

impl SuspendedWork {
    #[expect(
        clippy::future_not_send,
        reason = "The native executor models single-threaded IC callbacks with canister-local state."
    )]
    async fn wait(&self) {
        std::future::poll_fn(|context| {
            self.polls.set(self.polls.get() + 1);
            if self.ready.get() {
                Poll::Ready(())
            } else {
                *self.waker.borrow_mut() = Some(context.waker().clone());
                Poll::Pending
            }
        })
        .await;
    }

    fn resume(&self) {
        self.ready.set(true);
        self.waker
            .borrow_mut()
            .take()
            .expect("work suspended")
            .wake();
    }
}

fn scheduled_start(lifetime: DeclarationLifetime, initial: TimerSchedule) -> TimerStart {
    match lifetime {
        DeclarationLifetime::Retained => TimerStart::Retained {
            initial: Some(initial),
        },
        DeclarationLifetime::RemoveWhenStopped => TimerStart::RemoveWhenStopped { initial },
    }
}

fn suspended_ordinary_registration(
    after_completion: bool,
    timer: TimerIdentity,
    lifetime: DeclarationLifetime,
    gate: SuspendedWork,
    context: Rc<RefCell<Option<CallbackToken>>>,
    captured: Rc<()>,
) -> RegistrationClaim {
    let start = match lifetime {
        DeclarationLifetime::Retained => TimerStart::Retained { initial: None },
        DeclarationLifetime::RemoveWhenStopped => TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(10),
        },
    };
    if after_completion {
        register_after_completion(
            timer,
            TimerCadence::from_nanos(5).unwrap(),
            start,
            move |work| {
                *context.borrow_mut() = Some(work.token);
                let gate = gate.clone();
                let captured = Rc::clone(&captured);
                async move {
                    gate.wait().await;
                    std::hint::black_box(captured);
                    AfterCompletionRunResult::new(
                        TimerCompletion::success(1),
                        AfterCompletionDecision::Stop,
                    )
                }
            },
        )
        .unwrap()
        .claim
    } else {
        register_once(timer, start, move |work| {
            *context.borrow_mut() = Some(work.token);
            let gate = gate.clone();
            let captured = Rc::clone(&captured);
            async move {
                gate.wait().await;
                std::hint::black_box(captured);
                OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
            }
        })
        .unwrap()
        .claim
    }
}

#[test]
fn dropped_ordinary_deliveries_retire_confirmed_scheduled_and_running_work() {
    for after_completion in [false, true] {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            for started in [false, true] {
                let _fixture = setup();
                let timer = identity("dropped-ordinary");
                let gate = SuspendedWork::default();
                let context = Rc::new(RefCell::new(None));
                let captured = Rc::new(());
                let weak = Rc::downgrade(&captured);
                let claim = suspended_ordinary_registration(
                    after_completion,
                    timer.clone(),
                    lifetime,
                    gate.clone(),
                    Rc::clone(&context),
                    captured,
                );
                if lifetime == DeclarationLifetime::Retained {
                    reconcile_ordinary_claim(&claim, None, Some(TimerSchedule::At(10))).unwrap();
                }
                set_time(10);
                if started {
                    assert!(run_next_due());
                    assert_eq!(gate.polls.get(), 1);
                    gate.resume();
                }
                set_time(20);
                // Drop only; the native fake neither traps nor rolls messages back.
                assert!(discard_next_due());
                assert_eq!(timer_count(), 0);
                if let Some(token) = context.borrow().as_ref() {
                    assert!(matches!(
                        cancel_claim(token.claim(), Some(token)),
                        Err(TimerError::RegistrationExpired)
                    ));
                }
                if lifetime == DeclarationLifetime::Retained {
                    let snapshot = timer_snapshot(&timer).unwrap().unwrap();
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::Abandoned,
                        }
                    );
                    assert_eq!(snapshot.next_deadline_ns(), None);
                    assert!(!has_armed_wakeup_claim(&claim).unwrap());
                    let observations = snapshot.observability();
                    let counters = observations.counters();
                    assert_eq!(counters.unacknowledged().exact_total().unwrap(), 1);
                    assert_eq!(
                        counters.work_started().exact_total().unwrap(),
                        u128::from(started)
                    );
                    assert_eq!(counters.work_completed().exact_total().unwrap(), 0);
                    assert_eq!(
                        observations
                            .performance()
                            .work_instructions()
                            .aggregate()
                            .samples(),
                        0
                    );
                    assert_eq!(
                        observations.outcomes().last_outcome(),
                        Some(TimerLastOutcome::Unacknowledged)
                    );
                    assert_eq!(
                        observations.outcomes().last_unacknowledged_at_ns(),
                        Some(20)
                    );
                    assert!(weak.upgrade().is_some());
                    if !started {
                        gate.ready.set(true);
                    }
                    reconcile_ordinary_claim(&claim, None, Some(TimerSchedule::At(30))).unwrap();
                    set_time(30);
                    assert!(run_next_due());
                    let completed = timer_snapshot(&timer).unwrap().unwrap();
                    let counters = completed.observability().counters();
                    assert_eq!(counters.work_completed().exact_total().unwrap(), 1);
                    assert_eq!(counters.unacknowledged().exact_total().unwrap(), 1);
                    unregister_claim(&claim).unwrap();
                } else {
                    assert!(timer_snapshot(&timer).unwrap().is_none());
                    assert!(matches!(
                        has_armed_wakeup_claim(&claim),
                        Err(TimerError::RegistrationExpired)
                    ));
                }
                assert!(
                    weak.upgrade().is_none(),
                    "removed callbacks must release captures"
                );
                assert_eq!(timer_inventory().unwrap().timers(), []);
            }
        }
    }
}

#[test]
fn abandoned_running_work_discards_scheduling_commands_and_finishes_unregistration() {
    for after_completion in [false, true] {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            for command in ["cancel", "reconcile", "ensure", "unregister"] {
                let _fixture = setup();
                let timer = identity("abandoned-command");
                let gate = SuspendedWork::default();
                let context = Rc::new(RefCell::new(None));
                let claim = suspended_ordinary_registration(
                    after_completion,
                    timer.clone(),
                    lifetime,
                    gate.clone(),
                    Rc::clone(&context),
                    Rc::new(()),
                );
                if lifetime == DeclarationLifetime::Retained {
                    reconcile_ordinary_claim(&claim, None, Some(TimerSchedule::At(10))).unwrap();
                }
                set_time(10);
                assert!(run_next_due());
                match command {
                    "cancel" => cancel_claim(&claim, None).unwrap(),
                    "reconcile" => {
                        reconcile_ordinary_claim(&claim, None, Some(TimerSchedule::At(40)))
                            .unwrap();
                    }
                    "ensure" if after_completion => ensure_recurring_claim(&claim, None).unwrap(),
                    "ensure" => ensure_once_claim(&claim, None, TimerSchedule::At(40)).unwrap(),
                    "unregister" => unregister_claim(&claim).unwrap(),
                    _ => unreachable!("fixed command matrix"),
                }
                gate.resume();
                set_time(20);
                assert!(discard_next_due());
                if command == "unregister" || lifetime == DeclarationLifetime::RemoveWhenStopped {
                    assert!(timer_snapshot(&timer).unwrap().is_none());
                } else {
                    let snapshot = timer_snapshot(&timer).unwrap().unwrap();
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::Abandoned
                        }
                    );
                    assert_eq!(snapshot.next_deadline_ns(), None);
                    assert_eq!(
                        snapshot
                            .observability()
                            .counters()
                            .work_completed()
                            .exact_total()
                            .unwrap(),
                        0
                    );
                    unregister_claim(&claim).unwrap();
                }
                let token = context.borrow();
                let token = token.as_ref().unwrap();
                assert!(matches!(
                    cancel_claim(token.claim(), Some(token)),
                    Err(TimerError::RegistrationExpired)
                ));
                assert_eq!(timer_count(), 0);
            }
        }
    }
}

#[test]
fn stale_delivery_drop_cannot_retire_rearmed_or_reused_identity() {
    let _fixture = setup();
    let timer = identity("stale-delivery-drop");
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        |_| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .unwrap();
    registration
        .reconcile_schedule(Some(TimerSchedule::At(20)))
        .unwrap();
    let detached = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
        .unwrap()
        .unwrap();
    let (token, handle) = detached;
    bind_provider_handle(&token, handle, ProviderBinding::Stage).unwrap();
    let old_delivery = dispatch_wakeup(WakeupToken::Ordinary(token.clone()));
    registration
        .reconcile_schedule(Some(TimerSchedule::At(30)))
        .unwrap();
    let before = timer_inventory().unwrap();
    drop(old_delivery);
    assert_eq!(timer_inventory().unwrap(), before);
    registration.unregister().unwrap();
    let replacement = register_once(timer, TimerStart::Retained { initial: None }, |_| async {
        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
    })
    .unwrap();
    replacement
        .reconcile_schedule(Some(TimerSchedule::At(40)))
        .unwrap();
    let before = timer_inventory().unwrap();
    drop(dispatch_wakeup(WakeupToken::Ordinary(token)));
    assert_eq!(timer_inventory().unwrap(), before);
    assert!(replacement.has_armed_wakeup().unwrap());
    replacement.unregister().unwrap();
}

#[test]
fn abandoned_transient_releases_capacity_and_drops_captures_outside_registry_borrow() {
    struct CaptureDropCheck {
        timer: TimerIdentity,
        dropped: Rc<Cell<bool>>,
    }
    impl Drop for CaptureDropCheck {
        fn drop(&mut self) {
            assert!(timer_snapshot(&self.timer).unwrap().is_none());
            self.dropped.set(true);
        }
    }

    let _fixture = setup();
    let timer = identity("abandoned-capacity");
    let dropped = Rc::new(Cell::new(false));
    let captured = Rc::new(CaptureDropCheck {
        timer: timer.clone(),
        dropped: Rc::clone(&dropped),
    });
    let registration = register_once(
        timer.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::ZERO),
        },
        move |_| {
            let captured = Rc::clone(&captured);
            async move {
                std::hint::black_box(captured);
                OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
            }
        },
    )
    .unwrap();
    let mut remaining = Vec::new();
    for index in 1..crate::MAX_TIMER_REGISTRATIONS {
        remaining.push(
            register_once(
                identity(&format!("capacity-{index}")),
                TimerStart::Retained { initial: None },
                |_| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
            )
            .unwrap(),
        );
    }
    assert_eq!(
        timer_inventory().unwrap().timers().len(),
        crate::MAX_TIMER_REGISTRATIONS
    );
    assert!(discard_next_due());
    assert!(dropped.get());
    let replacement = register_once(timer, TimerStart::Retained { initial: None }, |_| async {
        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
    })
    .unwrap();
    assert!(matches!(
        registration.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    replacement.unregister().unwrap();
    for registration in remaining {
        registration.unregister().unwrap();
    }
}

#[test]
fn unconfirmed_ordinary_delivery_drop_preserves_binding_failure_accounting() {
    for after_completion in [false, true] {
        let _fixture = setup();
        let timer = identity("unconfirmed-delivery-drop");
        let claim = suspended_ordinary_registration(
            after_completion,
            timer.clone(),
            DeclarationLifetime::Retained,
            SuspendedWork::default(),
            Rc::new(RefCell::new(None)),
            Rc::new(()),
        );
        inject_provider_install_fault();
        assert!(reconcile_ordinary_claim(&claim, None, Some(TimerSchedule::At(10))).is_err());
        let snapshot = timer_snapshot(&timer).unwrap().unwrap();
        assert_eq!(
            snapshot.state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::ControlFailure(TimerControlFailure::ProviderBindingFailed),
            }
        );
        assert_eq!(
            snapshot
                .observability()
                .counters()
                .unacknowledged()
                .exact_total()
                .unwrap(),
            0
        );
        assert_eq!(snapshot.observability().outcomes().last_outcome(), None);
        assert_eq!(timer_count(), 0);
        unregister_claim(&claim).unwrap();
    }
}

#[test]
fn suspended_once_work_allows_other_timers_and_arbitrates_external_commands() {
    for command in ["cancel", "reconcile", "ensure", "unregister"] {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            let _fixture = setup();
            let timer = identity("suspended-once");
            let gate = SuspendedWork::default();
            let callback_gate = gate.clone();
            let context_slot = Rc::new(RefCell::new(None));
            let callback_slot = Rc::clone(&context_slot);
            let registration = register_once(
                timer.clone(),
                scheduled_start(lifetime, TimerSchedule::At(15)),
                move |context| {
                    let gate = callback_gate.clone();
                    *callback_slot.borrow_mut() = Some(context);
                    async move {
                        gate.wait().await;
                        let completion = TimerCompletion::success(1);
                        OnceRunResult::new(completion, OnceDecision::ScheduleAt(80))
                    }
                },
            )
            .unwrap();
            let unrelated_calls = Rc::new(Cell::new(0));
            let callback_calls = Rc::clone(&unrelated_calls);
            let _unrelated = register_watchdog(
                identity("unrelated-watchdog"),
                TimerCadence::from_nanos(5).unwrap(),
                TimerStart::Retained {
                    initial: Some(TimerSchedule::At(15)),
                },
                move |_| {
                    callback_calls.set(callback_calls.get() + 1);
                    WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
                },
            )
            .unwrap();
            set_time(15);
            assert!(run_next_due()); // Ordinary work suspends without an armed handle.
            assert!(!registration.has_armed_wakeup().unwrap());
            assert_eq!(timer_count(), 1);
            assert!(run_next_due()); // Unrelated scheduler and work must still run.
            assert!(run_next_due());
            assert_eq!(unrelated_calls.get(), 1);
            assert!(!run_next_due());
            assert_eq!(
                gate.polls.get(),
                1,
                "sleeping work must not be polled again"
            );
            match command {
                "cancel" => registration.cancel().unwrap(),
                "reconcile" => {
                    registration.cancel().unwrap();
                    registration
                        .reconcile_schedule(Some(TimerSchedule::At(50)))
                        .unwrap();
                }
                "ensure" => registration
                    .ensure_scheduled(TimerSchedule::At(40))
                    .unwrap(),
                "unregister" => registration.unregister().unwrap(),
                _ => unreachable!("fixed command matrix"),
            }
            set_time(30);
            gate.resume();
            assert!(run_next_due());
            let observed = timer_snapshot(&timer).unwrap();
            match command {
                "reconcile" | "ensure" => {
                    let snapshot = observed.unwrap();
                    let counters = snapshot.observability().counters();
                    let expected_deadline = if command == "ensure" { 40 } else { 50 };
                    assert_eq!(snapshot.next_deadline_ns(), Some(expected_deadline));
                    assert_eq!(counters.work_completed().exact_total().unwrap(), 1);
                    assert_eq!(timer_count(), 1);
                }
                "cancel" if lifetime == DeclarationLifetime::Retained => {
                    assert_eq!(
                        observed.unwrap().state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::Cancelled
                        }
                    );
                    assert_eq!(timer_count(), 0);
                }
                _ => {
                    assert!(observed.is_none());
                    assert_eq!(timer_count(), 0);
                }
            }
            assert!(matches!(
                context_slot.borrow().as_ref().unwrap().cancel(),
                Err(TimerError::RegistrationExpired)
            ));
        }
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "Keep both policy paths and their arbitration observations in one decision matrix."
)]
fn policy_specific_results_schedule_through_live_runtime() {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ResultCase {
        Stop,
        Continue,
        Retry,
        AtDeadline,
        Recur,
        UnrepresentableDelay,
        OverflowingDeadline,
        InvariantFailure,
    }
    use ResultCase::{
        AtDeadline, Continue, InvariantFailure, OverflowingDeadline, Recur, Retry, Stop,
        UnrepresentableDelay,
    };

    for recurring in [false, true] {
        for case in [
            Stop,
            Continue,
            Retry,
            AtDeadline,
            Recur,
            UnrepresentableDelay,
            OverflowingDeadline,
            InvariantFailure,
        ] {
            if !recurring && case == Recur {
                continue;
            }
            for reconcile in [false, true] {
                let _fixture = setup();
                let timer = identity("typed-results");
                let completion = if case == InvariantFailure {
                    TimerCompletion::invariant_failure(2)
                } else {
                    TimerCompletion::retryable_failure(2)
                };
                if recurring {
                    let decision = match case {
                        Stop => AfterCompletionDecision::Stop,
                        Continue | InvariantFailure => AfterCompletionDecision::ContinueImmediately,
                        Retry => AfterCompletionDecision::RetryAfter(Duration::from_nanos(7)),
                        AtDeadline => AfterCompletionDecision::ScheduleAt(50),
                        Recur => AfterCompletionDecision::RecurAfterCompletion,
                        UnrepresentableDelay => AfterCompletionDecision::RetryAfter(Duration::MAX),
                        OverflowingDeadline => {
                            AfterCompletionDecision::RetryAfter(Duration::from_nanos(u64::MAX))
                        }
                    };
                    let registration = register_after_completion(
                        timer.clone(),
                        TimerCadence::from_nanos(5).unwrap(),
                        TimerStart::Retained { initial: None },
                        move |context| async move {
                            if reconcile {
                                context
                                    .reconcile_schedule(Some(TimerSchedule::At(100)))
                                    .unwrap();
                            }
                            AfterCompletionRunResult::new(completion, decision)
                        },
                    )
                    .unwrap();
                    registration
                        .reconcile_schedule(Some(TimerSchedule::At(20)))
                        .unwrap();
                } else {
                    let decision = match case {
                        Stop => OnceDecision::Stop,
                        Continue | InvariantFailure => OnceDecision::ContinueImmediately,
                        Retry => OnceDecision::RetryAfter(Duration::from_nanos(7)),
                        AtDeadline => OnceDecision::ScheduleAt(50),
                        UnrepresentableDelay => OnceDecision::RetryAfter(Duration::MAX),
                        OverflowingDeadline => {
                            OnceDecision::RetryAfter(Duration::from_nanos(u64::MAX))
                        }
                        Recur => unreachable!("Once has no configured recurrence"),
                    };
                    let registration = register_once(
                        timer.clone(),
                        TimerStart::Retained { initial: None },
                        move |context| async move {
                            if reconcile {
                                context
                                    .reconcile_schedule(Some(TimerSchedule::At(100)))
                                    .unwrap();
                            }
                            OnceRunResult::new(completion, decision)
                        },
                    )
                    .unwrap();
                    registration
                        .ensure_scheduled(TimerSchedule::At(20))
                        .unwrap();
                }
                set_time(20);
                assert!(run_next_due());
                let snapshot = timer_snapshot(&timer).unwrap().unwrap();
                let deadline = if reconcile && case != InvariantFailure {
                    Some(100)
                } else {
                    match case {
                        Continue => Some(20),
                        Retry => Some(27),
                        AtDeadline => Some(50),
                        Recur => Some(25),
                        _ => None,
                    }
                };
                assert_eq!(snapshot.next_deadline_ns(), deadline);
                assert_eq!(timer_count(), usize::from(deadline.is_some()));
                if case == InvariantFailure {
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::InvariantFailure,
                        }
                    );
                } else if !reconcile && matches!(case, UnrepresentableDelay | OverflowingDeadline) {
                    let failure = if case == UnrepresentableDelay {
                        TimerControlFailure::DelayOutOfRange
                    } else {
                        TimerControlFailure::DeadlineOverflow
                    };
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::ControlFailure(failure),
                        }
                    );
                } else if reconcile {
                    assert_eq!(
                        snapshot.latest_directive(),
                        Some(crate::TimerDirectiveSnapshot::ScheduleAt { deadline_ns: 100 })
                    );
                }
                assert_eq!(
                    snapshot.observability().outcomes().last_outcome(),
                    Some(TimerLastOutcome::Completed(
                        if !reconcile && matches!(case, UnrepresentableDelay | OverflowingDeadline)
                        {
                            crate::TimerCompletionOutcome::InvariantFailure
                        } else {
                            completion.outcome()
                        }
                    ))
                );
            }
        }
    }
}

#[test]
fn suspended_after_completion_uses_completion_time_and_exact_reconciliation() {
    for reconcile in [false, true] {
        let _fixture = setup();
        let timer = identity("suspended-after-completion");
        let gate = SuspendedWork::default();
        let callback_gate = gate.clone();
        let registration = register_after_completion(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            TimerStart::Retained { initial: None },
            move |_| {
                let gate = callback_gate.clone();
                async move {
                    gate.wait().await;
                    AfterCompletionRunResult::new(
                        TimerCompletion::success(1),
                        AfterCompletionDecision::RecurAfterCompletion,
                    )
                }
            },
        )
        .unwrap();
        registration.ensure_scheduled().unwrap();
        set_time(15);
        assert!(run_next_due());
        assert!(!registration.has_armed_wakeup().unwrap());
        assert!(!run_next_due());
        assert_eq!(timer_count(), 0);
        if reconcile {
            registration
                .reconcile_schedule(Some(TimerSchedule::At(50)))
                .unwrap();
        }
        set_time(30);
        gate.resume();
        assert!(run_next_due());
        assert_eq!(
            timer_snapshot(&timer).unwrap().unwrap().next_deadline_ns(),
            Some(if reconcile { 50 } else { 35 })
        );
        assert_eq!(timer_count(), 1);
        registration.unregister().unwrap();
        assert_eq!(timer_count(), 0);
    }
}

fn identity(name: &str) -> TimerIdentity {
    TimerIdentity::try_new("test", "runtime", name).expect("fixture identity should be valid")
}

#[must_use = "retain the fixture until the test scope ends"]
struct RuntimeFixture {
    epoch: TimerEpoch,
}

impl Drop for RuntimeFixture {
    fn drop(&mut self) {
        // The production delivery guard must run while runtime TLS is available.
        // Native thread teardown is not an IC lifecycle or rollback event.
        platform::clear_tasks();
        // Retained declarations also own callback captures, even when inactive.
        // Detach them while TLS is live, exposing an empty initialized registry
        // to their destructors without holding the runtime borrow.
        let retired = RUNTIME.with(|runtime| {
            let mut runtime = runtime.borrow_mut();
            let empty = runtime
                .as_ref()
                .map(|registry| TimerRegistry::new(registry.epoch()));
            std::mem::replace(&mut *runtime, empty)
        });
        drop(retired);
    }
}

fn setup() -> RuntimeFixture {
    reset_for_test(10, 7);
    RuntimeFixture {
        epoch: initialize_runtime().expect("runtime initialization should succeed"),
    }
}

#[test]
fn fixture_cleanup_drops_queued_and_suspended_work_before_tls_teardown() {
    struct CaptureDropCheck(Rc<Cell<bool>>);

    impl Drop for CaptureDropCheck {
        fn drop(&mut self) {
            assert_eq!(timer_count(), 0, "cleanup must release the task-map borrow");
            assert!(
                timer_inventory().is_ok(),
                "runtime TLS must remain available"
            );
            self.0.set(true);
        }
    }

    for (lifetime, scheduled, started) in [
        (DeclarationLifetime::RemoveWhenStopped, true, false),
        (DeclarationLifetime::RemoveWhenStopped, true, true),
        (DeclarationLifetime::Retained, false, false),
        (DeclarationLifetime::Retained, true, false),
        (DeclarationLifetime::Retained, true, true),
    ] {
        let dropped = Rc::new(Cell::new(false));
        {
            let _fixture = setup();
            let gate = SuspendedWork::default();
            let captured = Rc::new(CaptureDropCheck(Rc::clone(&dropped)));
            let _registration = register_once(
                identity("fixture-cleanup"),
                if scheduled {
                    scheduled_start(lifetime, TimerSchedule::At(10))
                } else {
                    TimerStart::Retained { initial: None }
                },
                move |_| {
                    let gate = gate.clone();
                    let captured = Rc::clone(&captured);
                    async move {
                        gate.wait().await;
                        std::hint::black_box(captured);
                        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
                    }
                },
            )
            .unwrap();
            if started {
                assert!(run_next_due());
            }
            assert!(!dropped.get());
        }
        assert!(
            dropped.get(),
            "fixture cleanup must release callback captures"
        );
        assert_eq!(timer_count(), 0);
        assert_eq!(timer_inventory().unwrap().timers(), []);
        assert!(!run_next_due());
    }
}

#[test]
fn rejected_watchdog_cadence_preserves_the_complete_snapshot() {
    let _fixture = setup();
    let timer = identity("rejected-watchdog-cadence");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(u64::MAX).expect("positive cadence is valid"),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("declaration should register without arming");
    let before = timer_snapshot(&timer).expect("snapshot should be readable");

    assert!(matches!(
        registration.ensure_scheduled(),
        Err(TimerError::Schedule(ScheduleError::DeadlineOverflow))
    ));
    assert_eq!(
        timer_snapshot(&timer).expect("snapshot should be readable"),
        before
    );
    assert_eq!(timer_count(), 0);
    assert!(
        !registration
            .has_armed_wakeup()
            .expect("claim remains valid")
    );

    registration
        .ensure_scheduled_immediately()
        .expect("a valid request remains possible after rejection");
    assert_eq!(timer_count(), 1);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot should be readable")
            .expect("armed declaration exists")
            .observability()
            .counters()
            .schedule_requests()
            .exact_total()
            .unwrap(),
        1
    );
}

fn assert_retained_provider_binding_failure(
    timer: &TimerIdentity,
    schedule_requests: u128,
    wakeups_armed: u128,
) {
    let failed = timer_snapshot(timer)
        .expect("snapshot lookup should succeed")
        .expect("retained registration should remain declared");
    assert_eq!(
        failed.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::ControlFailure(TimerControlFailure::ProviderBindingFailed),
        }
    );
    assert_eq!(failed.generation(), None);
    assert_eq!(
        failed
            .observability()
            .counters()
            .schedule_requests()
            .exact_total()
            .unwrap(),
        schedule_requests
    );
    assert_eq!(
        failed
            .observability()
            .counters()
            .wakeups_armed()
            .exact_total()
            .unwrap(),
        wakeups_armed
    );
}

#[test]
fn initialization_is_required_and_idempotent() {
    reset_for_test(10, 7);
    assert!(matches!(timer_inventory(), Err(TimerError::NotInitialized)));

    let first = initialize_runtime().expect("first initialization should succeed");
    set_time(20);
    let second = initialize_runtime().expect("repeated initialization should succeed");
    assert_eq!(first, second);
    assert_eq!(first.canister_version(), 7);
    assert_eq!(first.started_at_ns(), 10);
    let busy = RUNTIME.with(|runtime| {
        let _borrow = runtime.borrow();
        initialize_runtime()
    });
    assert!(matches!(busy, Err(TimerError::RuntimeBusy)));
    let inventory = timer_inventory().expect("initialized inventory should be available");
    assert_eq!(inventory.epoch(), first);
    assert!(inventory.is_empty());
    assert_eq!(inventory.timers(), []);
    assert_eq!(inventory.into_timers(), []);
}

#[test]
fn fresh_inactive_reconciliation_reserves_complete_retained_inventory() {
    let _fixture = setup();
    let once_identity = identity("built-in-once");
    let after_identity = identity("built-in-after");
    let watchdog_identity = identity("built-in-watchdog");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");
    let mut once = None;
    let mut after = None;
    let mut watchdog = None;

    reconcile_once(&mut once, &once_identity, None, |_context| async {
        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
    })
    .expect("fresh inactive once declaration should be retained");
    reconcile_after_completion(
        &mut after,
        &after_identity,
        cadence,
        TimerReconcileState::Inactive,
        |_context| async {
            AfterCompletionRunResult::new(TimerCompletion::no_work(), AfterCompletionDecision::Stop)
        },
    )
    .expect("fresh inactive after-completion declaration should be retained");
    reconcile_watchdog(
        &mut watchdog,
        &watchdog_identity,
        cadence,
        WatchdogReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("fresh inactive watchdog declaration should be retained");

    assert!(once.is_some());
    assert!(after.is_some());
    assert!(watchdog.is_some());
    assert_eq!(timer_count(), 0);
    let inventory = timer_inventory().expect("inventory should be available");
    assert_eq!(inventory.epoch(), TimerEpoch::new(7, 10));
    assert_eq!(inventory.len(), 3);
    let snapshots = inventory.timers();
    assert_eq!(
        snapshots
            .iter()
            .map(TimerSnapshot::policy)
            .collect::<Vec<_>>(),
        vec![
            TimerPolicy::AfterCompletion { cadence },
            TimerPolicy::Once,
            TimerPolicy::Watchdog { cadence },
        ]
    );
    for snapshot in snapshots {
        assert_eq!(snapshot.lifetime(), DeclarationLifetime::Retained);
        assert_eq!(
            snapshot.state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::NeverScheduled,
            }
        );
        assert_eq!(
            snapshot.registration_status(),
            TimerRegistrationStatus::Unregistered
        );
        assert_eq!(snapshot.generation(), None);
    }
}

#[test]
fn fresh_transient_cancellation_expires_every_registration_policy() {
    let _fixture = setup();
    let once_identity = identity("fresh-transient-once");
    let after_identity = identity("fresh-transient-after");
    let watchdog_identity = identity("fresh-transient-watchdog");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");

    let once = register_once(
        once_identity.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(15),
        },
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("once registration should succeed");
    let after = register_after_completion(
        after_identity.clone(),
        cadence,
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_nanos(5)),
        },
        |_context| async {
            AfterCompletionRunResult::new(TimerCompletion::no_work(), AfterCompletionDecision::Stop)
        },
    )
    .expect("after-completion registration should succeed");
    let watchdog = register_watchdog(
        watchdog_identity.clone(),
        cadence,
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_nanos(5)),
        },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");

    once.cancel().expect("once cancellation should succeed");
    after
        .cancel()
        .expect("after-completion cancellation should succeed");
    watchdog
        .cancel()
        .expect("watchdog cancellation should succeed");

    assert_eq!(timer_count(), 0);
    for identity in [&once_identity, &after_identity, &watchdog_identity] {
        assert!(
            timer_snapshot(identity)
                .expect("snapshot lookup should succeed")
                .is_none(),
            "transient cancellation should remove {identity:?}"
        );
    }
    assert!(matches!(
        once.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        after.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        watchdog.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
}

#[test]
fn registration_claims_report_exact_provider_wakeup_ownership() {
    let _fixture = setup();
    let once = register_once(
        identity("liveness-once"),
        TimerStart::Retained { initial: None },
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("once registration should succeed");
    let after = register_after_completion(
        identity("liveness-after"),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| async {
            AfterCompletionRunResult::new(TimerCompletion::no_work(), AfterCompletionDecision::Stop)
        },
    )
    .expect("after-completion registration should succeed");
    let watchdog = register_watchdog(
        identity("liveness-watchdog"),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");

    assert!(!once.has_armed_wakeup().expect("claim should be readable"));
    assert!(!after.has_armed_wakeup().expect("claim should be readable"));
    assert!(
        !watchdog
            .has_armed_wakeup()
            .expect("claim should be readable")
    );

    once.ensure_scheduled(TimerSchedule::At(100))
        .expect("once wake-up should arm");
    after
        .ensure_scheduled()
        .expect("after-completion wake-up should arm");
    watchdog
        .ensure_scheduled()
        .expect("watchdog wake-up should arm");
    assert!(once.has_armed_wakeup().expect("claim should be readable"));
    assert!(after.has_armed_wakeup().expect("claim should be readable"));
    assert!(
        watchdog
            .has_armed_wakeup()
            .expect("claim should be readable")
    );

    once.cancel().expect("once cancellation should succeed");
    after
        .cancel()
        .expect("after-completion cancellation should succeed");
    watchdog
        .cancel()
        .expect("watchdog cancellation should succeed");
    assert!(!once.has_armed_wakeup().expect("claim should be readable"));
    assert!(!after.has_armed_wakeup().expect("claim should be readable"));
    assert!(
        !watchdog
            .has_armed_wakeup()
            .expect("claim should be readable")
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_claim_observes_the_prearmed_successor_not_queued_work() {
    let _fixture = setup();
    let watchdog = register_watchdog(
        identity("liveness-watchdog-successor"),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("watchdog registration should succeed");
    watchdog
        .ensure_scheduled()
        .expect("watchdog wake-up should arm");

    set_time(15);
    assert!(run_next_due(), "scheduler callback should execute");
    assert_eq!(timer_count(), 2, "successor and work should both be queued");
    assert!(
        watchdog
            .has_armed_wakeup()
            .expect("successor ownership should be readable")
    );

    watchdog
        .cancel()
        .expect("cancellation should clear successor and work");
    assert!(
        !watchdog
            .has_armed_wakeup()
            .expect("retained claim should remain readable")
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn immediate_watchdog_reconciliation_arms_one_zero_delay_scheduler() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-reconcile");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = None;
    reconcile_watchdog(
        &mut registration,
        &timer,
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        WatchdogReconcileState::ScheduledImmediately,
        move |_context| {
            callback_calls.set(callback_calls.get().saturating_add(1));
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
        },
    )
    .expect("fresh immediate reconciliation should succeed");

    assert_eq!(timer_count(), 1);
    let scheduled = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(scheduled.next_deadline_ns(), Some(10));
    assert_eq!(
        scheduled.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(scheduled.latest_requested_delay_ns(), Some(0));
    assert_eq!(scheduled.latest_armed_delay_ns(), Some(0));
    let counters = scheduled.observability().counters();
    assert_eq!(counters.schedule_requests().exact_total().unwrap(), 1);
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 1);

    assert!(run_next_due(), "zero-delay scheduler should run later");
    assert_eq!(calls.get(), 0, "the scheduler must not invoke work inline");
    assert_eq!(
        timer_count(),
        2,
        "cadence successor and work should be armed"
    );
    assert!(run_next_due(), "separate work callback should run later");
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 0);
}

#[test]
fn immediate_watchdog_ensure_moves_cadence_earlier_and_repeats_idempotently() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-ensure");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("cadence scheduler should arm");
    assert_eq!(timer_count(), 1);
    registration
        .ensure_scheduled_immediately()
        .expect("cadence scheduler should move to now");
    registration
        .ensure_scheduled_immediately()
        .expect("repeated immediate ensure should coalesce");

    assert_eq!(timer_count(), 1, "replacement must own exactly one handle");
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(10));
    assert_eq!(
        snapshot.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
    let counters = snapshot.observability().counters();
    assert_eq!(counters.schedule_requests().exact_total().unwrap(), 3);
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 2);
    assert_eq!(counters.coalesced().exact_total().unwrap(), 1);

    registration
        .cancel()
        .expect("fixture cleanup should succeed");
    assert_eq!(timer_count(), 0);
}

#[test]
fn immediate_watchdog_ensure_coalesces_an_overdue_cadence_wakeup() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-overdue-cadence");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("cadence scheduler should arm");

    set_time(20);
    registration
        .ensure_scheduled_immediately()
        .expect("overdue cadence scheduler should satisfy immediate demand");

    assert_eq!(timer_count(), 1, "coalescing must not add a provider timer");
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(15));
    assert_eq!(snapshot.scheduling_mode(), TimerSchedulingMode::Watchdog);
    assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(5));
    let counters = snapshot.observability().counters();
    assert_eq!(counters.schedule_requests().exact_total().unwrap(), 2);
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 1);
    assert_eq!(counters.coalesced().exact_total().unwrap(), 1);

    registration
        .cancel()
        .expect("fixture cleanup should succeed");
    assert_eq!(timer_count(), 0);
}

#[test]
fn removed_transient_claim_cannot_report_wakeup_liveness() {
    let _fixture = setup();
    let timer_identity = identity("liveness-transient");
    let timer = register_once(
        timer_identity.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(15),
        },
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("registration should succeed");
    assert!(timer.has_armed_wakeup().expect("claim should be readable"));

    set_time(15);
    assert!(run_next_due());
    assert!(matches!(
        timer.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));

    let replacement = register_once(
        timer_identity,
        TimerStart::Retained { initial: None },
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("replacement registration should succeed");
    replacement
        .ensure_scheduled(TimerSchedule::At(25))
        .expect("replacement wake-up should arm");
    assert!(
        replacement
            .has_armed_wakeup()
            .expect("replacement claim should be readable")
    );
    assert!(matches!(
        timer.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
}

#[test]
fn once_owns_one_provider_handle_and_executes_without_registry_borrow() {
    let _fixture = setup();
    let timer = identity("once");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let callback_identity = timer.clone();
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            advance_instructions(7);
            grow_memory_pages(2, 3);
            let visible = timer_snapshot(&callback_identity)
                .expect("consumer work must not observe a registry borrow")
                .is_some();
            async move {
                assert!(visible);
                OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
            }
        },
    )
    .expect("registration should succeed");

    registration
        .ensure_scheduled(TimerSchedule::After(Duration::from_nanos(5)))
        .expect("ensure should arm provider timer");
    assert_eq!(timer_count(), 1);
    let scheduled = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("snapshot should exist");
    assert_eq!(
        scheduled
            .observability()
            .counters()
            .wakeups_armed()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(scheduled.latest_armed_delay_ns(), Some(5));

    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    assert_eq!(calls.get(), 1);
    let stopped = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained snapshot should exist");
    assert_eq!(
        stopped.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped,
        }
    );
    assert_eq!(
        stopped
            .observability()
            .counters()
            .work_started()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(
        stopped
            .observability()
            .counters()
            .work_completed()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(
        stopped
            .observability()
            .performance()
            .work_instructions()
            .latest(),
        Some(7)
    );
    let memory = stopped.observability().performance().work_memory_pages();
    assert_eq!(memory.samples(), 1);
    let latest = memory.latest().expect("normal work should have one sample");
    assert_eq!(latest.start().wasm_pages(), 1);
    assert_eq!(latest.start().stable_pages(), 0);
    assert_eq!(latest.end().wasm_pages(), 3);
    assert_eq!(latest.end().stable_pages(), 3);
    assert_eq!(memory.maximum_wasm_growth_pages(), Some(2));
    assert_eq!(memory.maximum_stable_growth_pages(), Some(3));
}

#[test]
fn ordinary_callback_borrow_failure_stops_without_invoking_or_measuring_work() {
    for (recurring, lifetime) in [
        (false, DeclarationLifetime::Retained),
        (false, DeclarationLifetime::RemoveWhenStopped),
        (true, DeclarationLifetime::Retained),
        (true, DeclarationLifetime::RemoveWhenStopped),
    ] {
        let _fixture = setup();
        let timer = identity("ordinary-borrow-failure");
        let calls = Rc::new(Cell::new(0));
        let callback_calls = Rc::clone(&calls);
        let callback = if recurring {
            erase_ordinary_callback(
                move |_context| {
                    callback_calls.set(callback_calls.get() + 1);
                    async {
                        AfterCompletionRunResult::new(
                            TimerCompletion::success(1),
                            AfterCompletionDecision::RecurAfterCompletion,
                        )
                    }
                },
                AfterCompletionContext::new,
                OrdinaryRunResult::from,
            )
        } else {
            erase_ordinary_callback(
                move |_context| {
                    callback_calls.set(callback_calls.get() + 1);
                    async { OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop) }
                },
                OnceContext::new,
                OrdinaryRunResult::from,
            )
        };
        let claim = with_registry_mut(|registry| {
            let registered = if recurring {
                registry.register_after_completion_with_callback(
                    timer.clone(),
                    TimerCadence::from_nanos(5).unwrap(),
                    lifetime,
                    Rc::clone(&callback),
                )
            } else {
                registry.register_once_with_callback(timer.clone(), lifetime, Rc::clone(&callback))
            };
            registered.map_err(TimerError::from)
        })
        .unwrap();
        if recurring {
            ensure_recurring_claim(&claim, None).unwrap();
        } else {
            ensure_once_claim(&claim, None, TimerSchedule::At(15)).unwrap();
        }
        assert_eq!(timer_count(), 1);

        // Keep the callback unavailable while dispatch accepts and completes work.
        let _borrow = callback.borrow_mut();
        set_time(15);
        assert!(run_next_due());
        assert_eq!(calls.get(), 0);
        assert_eq!(timer_count(), 0);
        assert!(!run_next_due());

        let snapshot = timer_snapshot(&timer).unwrap();
        if lifetime == DeclarationLifetime::Retained {
            let snapshot = snapshot.unwrap();
            assert_eq!(
                snapshot.state(),
                TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::InvariantFailure,
                }
            );
            let counters = snapshot.observability().counters();
            assert_eq!(counters.work_started().exact_total().unwrap(), 1);
            assert_eq!(counters.work_completed().exact_total().unwrap(), 1);
            assert_eq!(counters.invariant_failure().exact_total().unwrap(), 1);
            let performance = snapshot.observability().performance();
            assert_eq!(performance.work_instructions().aggregate().samples(), 0);
            assert_eq!(performance.work_memory_pages().samples(), 0);
            assert!(!has_armed_wakeup_claim(&claim).unwrap());
        } else {
            assert!(snapshot.is_none());
            assert!(matches!(
                has_armed_wakeup_claim(&claim),
                Err(TimerError::RegistrationExpired)
            ));
        }
    }
}

#[test]
fn after_completion_recurrence_follows_returned_completion_classification() {
    for completion in [
        TimerCompletion::success(3),
        TimerCompletion::no_work(),
        TimerCompletion::retryable_failure(2),
        TimerCompletion::invariant_failure(4),
    ] {
        let _fixture = setup();
        let timer = identity("classified-recurrence");
        let registration = register_after_completion(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            TimerStart::Retained { initial: None },
            move |_| async move {
                AfterCompletionRunResult::new(
                    completion,
                    AfterCompletionDecision::RecurAfterCompletion,
                )
            },
        )
        .unwrap();
        registration.ensure_scheduled().unwrap();
        set_time(15);
        assert!(run_next_due());
        let snapshot = timer_snapshot(&timer).unwrap().unwrap();
        let invariant = completion.outcome() == crate::TimerCompletionOutcome::InvariantFailure;
        assert_eq!(
            snapshot.next_deadline_ns(),
            if invariant { None } else { Some(20) }
        );
        assert_eq!(registration.has_armed_wakeup().unwrap(), !invariant);
        assert_eq!(timer_count(), usize::from(!invariant));
        let outcomes = snapshot.observability().outcomes();
        assert_eq!(
            outcomes.last_outcome(),
            Some(TimerLastOutcome::Completed(completion.outcome()))
        );
        assert_eq!(outcomes.last_work_count(), Some(completion.work_count()));
        assert_eq!(
            outcomes.consecutive_expected_failures(),
            u64::from(completion.outcome() == crate::TimerCompletionOutcome::RetryableFailure)
        );
        assert_eq!(
            snapshot
                .observability()
                .counters()
                .work_completed()
                .exact_total()
                .unwrap(),
            1
        );
        assert_eq!(
            snapshot.latest_directive(),
            Some(if invariant {
                crate::TimerDirectiveSnapshot::Stop
            } else {
                crate::TimerDirectiveSnapshot::RecurAfterCompletion
            })
        );
    }
}

#[test]
fn after_completion_rearms_from_actual_completion_time() {
    let _fixture = setup();
    let timer = identity("after-completion");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_after_completion(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        move |_context| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            async move {
                let directive = if call == 1 {
                    AfterCompletionDecision::RecurAfterCompletion
                } else {
                    AfterCompletionDecision::Stop
                };
                AfterCompletionRunResult::new(TimerCompletion::success(1), directive)
            }
        },
    )
    .expect("registration should succeed");

    registration
        .ensure_scheduled()
        .expect("initial ensure should succeed");
    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    assert!(!run_next_due());

    set_time(20);
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained snapshot should exist");
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .wakeups_armed()
            .exact_total()
            .unwrap(),
        2
    );
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .work_completed()
            .exact_total()
            .unwrap(),
        2
    );
}

#[test]
fn after_completion_context_can_restore_recurrence_after_nested_cancel() {
    let _fixture = setup();
    let timer = identity("after-context");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_after_completion(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        move |context: AfterCompletionContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            async move {
                if call == 1 {
                    context
                        .cancel()
                        .expect("nested cancellation should succeed");
                    context
                        .ensure_scheduled()
                        .expect("later nested ensure should succeed");
                }
                AfterCompletionRunResult::new(
                    TimerCompletion::no_work(),
                    AfterCompletionDecision::Stop,
                )
            }
        },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial ensure should succeed");

    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(20)
    );

    set_time(20);
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn once_context_can_restore_scheduling_after_nested_cancel() {
    let _fixture = setup();
    let timer = identity("nested");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        move |context: OnceContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            async move {
                if call == 1 {
                    context
                        .cancel()
                        .expect("nested cancellation should succeed");
                    context
                        .ensure_scheduled(TimerSchedule::At(30))
                        .expect("later nested ensure should succeed");
                }
                OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
            }
        },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("initial ensure should succeed");

    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 1);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(30)
    );
    set_time(30);
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn retained_once_context_expires_after_its_work_attempt() {
    let _fixture = setup();
    let timer = identity("ordinary-context-expiry");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let retained_context = Rc::new(RefCell::new(None));
    let callback_context = Rc::clone(&retained_context);
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        move |context: OnceContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            let directive = if call == 1 {
                *callback_context.borrow_mut() = Some(context);
                OnceDecision::ContinueImmediately
            } else {
                OnceDecision::Stop
            };
            async move { OnceRunResult::new(TimerCompletion::no_work(), directive) }
        },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("initial ensure should succeed");

    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(
        timer_count(),
        1,
        "completion should arm the next generation"
    );

    let expired = retained_context
        .borrow_mut()
        .take()
        .expect("first callback should retain its context");
    assert_eq!(
        expired.identity(),
        &timer,
        "identity remains inert metadata"
    );
    assert!(matches!(
        expired.cancel(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        expired.ensure_scheduled(TimerSchedule::At(30)),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        expired.reconcile_schedule(None),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(
        timer_count(),
        1,
        "expired context must not clear or replace the next generation"
    );

    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn stale_reused_identity_callback_cannot_change_handles_or_measurements() {
    let _fixture = setup();
    let timer_identity = identity("stale-consume-reuse");
    let old = register_once(
        timer_identity.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(15),
        },
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("old registration should succeed");
    let old_handle = with_registry_mut(|registry| {
        registry
            .take_wakeup_handle(&timer_identity)
            .ok_or(TimerError::OwnershipInvariant)
    })
    .expect("old provider handle should detach");
    let (old_token, old_provider_handle) = old_handle;
    platform::clear_timer(old_provider_handle);
    old.cancel()
        .expect("old transient registration should be removed");

    // Exercise the production recording path with current platform end counters.
    // A removed or reused claim must reject the sample before updating accounting.
    let stale_measurement = CallbackMeasurementStart {
        instructions_before: 0,
        memory_start: MemoryPageExtent::new(1, 0),
    };
    stale_measurement.record(&old_token);
    assert!(timer_snapshot(old_token.identity()).unwrap().is_none());

    let replacement = register_once(
        timer_identity,
        TimerStart::Retained { initial: None },
        |_context| async {
            advance_instructions(7);
            grow_memory_pages(2, 3);
            OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
        },
    )
    .expect("replacement registration should succeed");
    replacement
        .ensure_scheduled(TimerSchedule::At(25))
        .expect("replacement wake-up should arm");
    assert_eq!(timer_count(), 1);

    let before = timer_snapshot(replacement.identity()).unwrap().unwrap();
    let before_performance = before.observability().performance();
    let stale_token = old_token.clone();
    let _stale_delivery = platform::set_timer(Duration::ZERO, async move {
        dispatch_wakeup(WakeupToken::Ordinary(stale_token)).await;
    });
    assert!(run_next_due(), "stale callback delivery should be harmless");
    // Exercise the accounting boundary directly; stale dispatch would return
    // before taking a measurement and cannot reach this ownership check.
    stale_measurement.record(&old_token);
    let after = timer_snapshot(replacement.identity()).unwrap().unwrap();
    let counters = after.observability().counters();
    assert_eq!(after.observability().performance(), before_performance);
    assert_eq!(counters.stale_wakeups().exact_total().unwrap(), 1);
    assert_eq!(counters.work_started().exact_total().unwrap(), 0);
    assert!(
        replacement
            .has_armed_wakeup()
            .expect("replacement claim should retain its handle")
    );
    assert_eq!(timer_count(), 1);

    set_time(25);
    assert!(run_next_due());
    let completed = timer_snapshot(replacement.identity()).unwrap().unwrap();
    let performance = completed.observability().performance();
    assert_eq!(performance.work_instructions().aggregate().samples(), 1);
    assert_eq!(performance.work_instructions().aggregate().total(), 7);
    assert_eq!(performance.work_memory_pages().samples(), 1);
    assert_eq!(
        performance.work_memory_pages().latest(),
        Some(MemoryPageSample::new(
            MemoryPageExtent::new(1, 0),
            MemoryPageExtent::new(3, 3),
        ))
    );
    assert_eq!(
        performance.scheduler_instructions(),
        before_performance.scheduler_instructions()
    );
    assert_eq!(
        performance.scheduler_memory_pages(),
        before_performance.scheduler_memory_pages()
    );
    stale_measurement.record(&old_token);
    let after_stale = timer_snapshot(replacement.identity()).unwrap().unwrap();
    assert_eq!(after_stale.observability().performance(), performance);

    replacement
        .cancel()
        .expect("replacement cleanup should succeed");
    assert_eq!(timer_count(), 0);
}

#[test]
fn replacement_and_cancellation_clear_actual_owned_handles() {
    let _fixture = setup();
    let timer = identity("replace-cancel");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) }
        },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(100))
        .expect("initial ensure should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(50))
        .expect("earlier deadline should replace the handle");
    assert_eq!(timer_count(), 1);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(50)
    );
    let cancelled_generation = timer_snapshot(&timer)
        .unwrap()
        .unwrap()
        .generation()
        .unwrap();

    registration.cancel().expect("cancel should clear handle");
    assert_eq!(timer_count(), 0);
    registration
        .cancel()
        .expect("repeated cancellation should be idempotent");
    set_time(100);
    assert!(!run_next_due());
    assert_eq!(calls.get(), 0);
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained snapshot should exist");
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .cancelled()
            .exact_total()
            .unwrap(),
        1
    );

    registration
        .ensure_scheduled(TimerSchedule::At(150))
        .unwrap();
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().generation(),
        Some(cancelled_generation + 1)
    );
    assert_eq!(timer_count(), 1);
    set_time(150);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 0);
}

#[test]
fn duplicate_registration_and_reconstruction_preserve_live_work_and_release_capacity() {
    let _fixture = setup();
    let timer = identity("duplicate");
    let first_calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&first_calls);
    let registration = register_once(
        timer.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(20),
        },
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) }
        },
    )
    .expect("registration should succeed");
    let before = timer_inventory().unwrap();
    let duplicate = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        |_context| async {
            OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
        },
    );
    assert!(matches!(
        duplicate,
        Err(TimerError::Register(
            RegisterError::IdentityAlreadyRegistered(_)
        ))
    ));
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    assert!(registration.has_armed_wakeup().unwrap());

    let mut empty_slot = None;
    let reconstruction = reconcile_once(&mut empty_slot, &timer, None, |_| async {
        OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
    });
    assert!(matches!(
        reconstruction,
        Err(TimerError::Register(
            RegisterError::IdentityAlreadyRegistered(ref occupied)
        )) if occupied == &timer
    ));
    assert!(empty_slot.is_none());
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    assert!(registration.has_armed_wakeup().unwrap());

    set_time(20);
    assert!(run_next_due());
    assert_eq!(first_calls.get(), 1);
    assert!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .is_none()
    );
    assert!(
        timer_inventory()
            .expect("inventory should succeed")
            .is_empty()
    );
}

#[test]
fn watchdog_scheduler_prearms_successor_before_synchronous_work() {
    let _fixture = setup();
    let timer = identity("watchdog-prearm");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        move |_context| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            assert_eq!(timer_count(), 1, "successor must exist during work");
            let decision = if call == 1 {
                WatchdogDecision::Continue
            } else {
                WatchdogDecision::Stop
            };
            WatchdogRunResult::new(TimerCompletion::success(1), decision)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    assert_eq!(timer_count(), 1);

    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 0, "scheduler must not invoke consumer work");
    assert_eq!(timer_count(), 2, "successor and work must both be owned");
    let dispatched = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert!(matches!(
        dispatched.state(),
        TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::AwaitingWork { .. })
    ));
    assert_eq!(
        dispatched
            .observability()
            .counters()
            .scheduler_started()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(
        dispatched
            .observability()
            .counters()
            .wakeups_armed()
            .exact_total()
            .unwrap(),
        2
    );
    assert_eq!(
        dispatched
            .observability()
            .counters()
            .work_dispatched()
            .exact_total()
            .unwrap(),
        1
    );

    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    set_time(20);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 2);
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_immediate_decision_replaces_successor_without_synchronous_recursion() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-decision");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        move |_context| {
            let call = callback_calls.get().saturating_add(1);
            callback_calls.set(call);
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                if call == 1 {
                    WatchdogDecision::ContinueImmediately
                } else {
                    WatchdogDecision::Stop
                },
            )
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(15);
    assert!(run_next_due(), "scheduler should pre-arm cadence safety");
    assert_eq!(timer_count(), 2);
    assert!(run_next_due(), "first work callback should run");
    assert_eq!(calls.get(), 1);
    assert_eq!(
        timer_count(),
        1,
        "cadence successor must be replaced, not added to"
    );
    let immediate = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(immediate.next_deadline_ns(), Some(15));
    assert_eq!(
        immediate.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(immediate.latest_requested_delay_ns(), Some(0));
    assert_eq!(immediate.latest_armed_delay_ns(), Some(0));
    let counters = immediate.observability().counters();
    assert_eq!(counters.wakeups_armed().exact_total().unwrap(), 3);
    assert_eq!(counters.work_dispatched().exact_total().unwrap(), 1);

    assert!(
        run_next_due(),
        "replacement scheduler should be a later message"
    );
    assert_eq!(
        calls.get(),
        1,
        "scheduler still must not invoke consumer work"
    );
    assert_eq!(timer_count(), 2);
    assert!(run_next_due(), "second separate work callback should run");
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_running_immediate_context_targets_that_attempts_successor() {
    let _fixture = setup();
    let timer = identity("watchdog-context-immediate");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |context| {
            context
                .ensure_scheduled_immediately()
                .expect("running immediate request should succeed");
            context
                .ensure_scheduled()
                .expect("later cadence ensure must not delay immediate demand");
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(timer_count(), 1);
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(15));
    assert_eq!(
        snapshot.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .schedule_requests()
            .exact_total()
            .unwrap(),
        3
    );
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .coalesced()
            .exact_total()
            .unwrap(),
        2
    );

    registration
        .cancel()
        .expect("fixture cleanup should succeed");
}

#[test]
fn watchdog_cancellation_clears_successor_and_queued_work() {
    let _fixture = setup();
    let timer = identity("watchdog-cancel");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    let cancelled_generation = timer_snapshot(&timer)
        .unwrap()
        .unwrap()
        .generation()
        .unwrap();

    registration
        .cancel()
        .expect("cancellation should clear both handles");
    assert_eq!(timer_count(), 0);
    assert!(!run_next_due());
    assert_eq!(calls.get(), 0);
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .cancelled()
            .exact_total()
            .unwrap(),
        1
    );

    registration.ensure_scheduled().unwrap();
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().generation(),
        Some(cancelled_generation + 1)
    );
    assert_eq!(timer_count(), 1);
    set_time(20);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    registration.cancel().unwrap();
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_invariant_failure_overrides_nested_cancellation_and_clears_handles() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        let _fixture = setup();
        let timer = identity("watchdog-cancel-invariant");
        let registration = register_watchdog(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            scheduled_start(lifetime, TimerSchedule::After(Duration::from_nanos(5))),
            |context: WatchdogContext| {
                context.cancel().unwrap();
                WatchdogRunResult::new(
                    TimerCompletion::invariant_failure(1),
                    WatchdogDecision::ContinueImmediately,
                )
            },
        )
        .unwrap();
        set_time(15);
        assert!(run_next_due(), "scheduler should pre-arm its successor");
        assert_eq!(timer_count(), 2);
        assert!(
            run_next_due(),
            "work should complete with invariant failure"
        );
        assert_eq!(
            timer_count(),
            0,
            "terminal failure must clear the successor"
        );

        let snapshot = timer_snapshot(&timer).unwrap();
        if lifetime == DeclarationLifetime::Retained {
            let snapshot = snapshot.unwrap();
            assert_eq!(
                snapshot.state(),
                TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::InvariantFailure,
                }
            );
            let counters = snapshot.observability().counters();
            assert_eq!(counters.work_completed().exact_total().unwrap(), 1);
            assert_eq!(counters.invariant_failure().exact_total().unwrap(), 1);
            assert_eq!(counters.cancelled().exact_total().unwrap(), 0);
        } else {
            assert!(snapshot.is_none());
            assert!(matches!(
                registration.cancel(),
                Err(TimerError::RegistrationExpired)
            ));
        }
    }
}

#[test]
fn watchdog_nested_cancel_then_ensure_retains_committed_successor() {
    let _fixture = setup();
    let timer = identity("watchdog-nested");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        move |context: WatchdogContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            if call == 1 {
                context.cancel().expect("nested cancel should succeed");
                context
                    .ensure_scheduled()
                    .expect("later nested ensure should succeed");
            }
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    assert!(matches!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .map(|snapshot| snapshot.state()),
        Some(TimerRuntimeStateSnapshot::Watchdog(
            WatchdogRuntimeStateSnapshot::Scheduled { .. }
        ))
    ));
}

#[test]
fn retained_watchdog_context_expires_without_clearing_successor() {
    let _fixture = setup();
    let timer = identity("watchdog-context-expiry");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let retained_context = Rc::new(RefCell::new(None));
    let callback_context = Rc::clone(&retained_context);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        move |context: WatchdogContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            if call == 1 {
                *callback_context.borrow_mut() = Some(context);
            }
            let decision = if call == 1 {
                WatchdogDecision::Continue
            } else {
                WatchdogDecision::Stop
            };
            WatchdogRunResult::new(TimerCompletion::no_work(), decision)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1, "committed successor should remain armed");

    let expired = retained_context
        .borrow_mut()
        .take()
        .expect("first work callback should retain its context");
    assert_eq!(
        expired.identity(),
        &timer,
        "identity remains inert metadata"
    );
    assert!(matches!(
        expired.cancel(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        expired.ensure_scheduled(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        expired.ensure_scheduled_immediately(),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(
        timer_count(),
        1,
        "expired context must not clear or duplicate the successor"
    );

    set_time(20);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn abandoned_initial_and_successor_schedulers_reconcile_all_owner_contracts() {
    for successor in [false, true] {
        let expected_deadline = if successor { 20 } else { 15 };
        for desired in [
            WatchdogReconcileState::Scheduled,
            WatchdogReconcileState::ScheduledImmediately,
            WatchdogReconcileState::ScheduledAt(expected_deadline),
        ] {
            let _fixture = setup();
            let timer = identity("abandoned-scheduler");
            let calls = Rc::new(Cell::new(0));
            let callback_calls = Rc::clone(&calls);
            let cadence = TimerCadence::from_nanos(5).unwrap();
            let mut owner = Some(
                register_watchdog(
                    timer.clone(),
                    cadence,
                    TimerStart::Retained {
                        initial: Some(TimerSchedule::At(15)),
                    },
                    move |_| {
                        callback_calls.set(callback_calls.get() + 1);
                        WatchdogRunResult::new(
                            TimerCompletion::success(1),
                            WatchdogDecision::Continue,
                        )
                    },
                )
                .unwrap(),
            );
            if successor {
                set_time(15);
                assert!(run_next_due());
                assert!(run_next_due());
            }
            let before = timer_snapshot(&timer).unwrap().unwrap();
            let deadline = before.next_deadline_ns().unwrap();
            assert_eq!(deadline, expected_deadline);
            let old = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
                .unwrap()
                .unwrap();
            let (old_token, handle) = old;
            bind_provider_handle(&old_token, handle, ProviderBinding::Stage).unwrap();
            let stale = dispatch_wakeup(WakeupToken::Scheduler(old_token));
            set_time(deadline);
            assert!(discard_next_due());
            assert!(!owner.as_ref().unwrap().has_armed_wakeup().unwrap());
            assert_eq!(timer_snapshot(&timer).unwrap().unwrap(), before);
            for _ in 0..2 {
                reconcile_watchdog(&mut owner, &timer, cadence, desired, |_| {
                    unreachable!("existing callback retained")
                })
                .unwrap();
            }
            drop(stale); // old generation cannot mark the replacement abandoned
            assert!(owner.as_ref().unwrap().has_armed_wakeup().unwrap());
            assert_eq!(timer_count(), 1);
            assert!(run_next_due());
            assert!(run_next_due());
            assert_eq!(calls.get(), u64::from(successor) + 1);
            let after = timer_snapshot(&timer).unwrap().unwrap();
            assert_eq!(after.registration_id(), before.registration_id());
            assert_eq!(
                after
                    .observability()
                    .counters()
                    .unacknowledged()
                    .exact_total(),
                Ok(0)
            );
        }
    }
}

#[test]
fn abandoned_successor_repair_keeps_queued_work_and_pending_reconciliation() {
    for discard_work in [false, true] {
        let _fixture = setup();
        let timer = identity("lost-successor-with-pending-work");
        let calls = Rc::new(Cell::new(0));
        let callback_calls = Rc::clone(&calls);
        let cadence = TimerCadence::from_nanos(5).unwrap();
        let mut owner = Some(
            register_watchdog(
                timer.clone(),
                cadence,
                TimerStart::Retained {
                    initial: Some(TimerSchedule::At(10)),
                },
                move |_| {
                    callback_calls.set(callback_calls.get() + 1);
                    WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue)
                },
            )
            .unwrap(),
        );
        assert!(run_next_due()); // successor at 15, work at 10
        if discard_work {
            assert!(discard_next_due());
        }
        assert!(platform::discard_timer_at(15));
        assert!(!owner.as_ref().unwrap().has_armed_wakeup().unwrap());
        reconcile_watchdog(
            &mut owner,
            &timer,
            cadence,
            WatchdogReconcileState::ScheduledAt(100),
            |_| unreachable!(),
        )
        .unwrap();
        assert!(owner.as_ref().unwrap().has_armed_wakeup().unwrap());
        assert_eq!(timer_count(), if discard_work { 1 } else { 2 });
        if discard_work {
            set_time(15);
            assert!(run_next_due());
        }
        assert!(run_next_due());
        assert_eq!(calls.get(), 1);
        let after = timer_snapshot(&timer).unwrap().unwrap();
        assert_eq!(
            after.next_deadline_ns(),
            Some(if discard_work { 20 } else { 100 })
        );
        assert_eq!(
            after
                .observability()
                .counters()
                .unacknowledged()
                .exact_total(),
            Ok(u128::from(discard_work))
        );
        assert_eq!(timer_count(), 1);
    }
}

#[test]
fn watchdog_successor_retires_an_unacknowledged_dispatched_attempt() {
    let _fixture = setup();
    let timer = identity("watchdog-unacknowledged");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    registration
        .reconcile_schedule(Some(TimerSchedule::At(100)))
        .expect("dispatched reconciliation should queue for this attempt");
    assert!(
        discard_next_due(),
        "simulate work without committed completion"
    );
    assert_eq!(timer_count(), 1);

    set_time(20);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .unacknowledged()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(
        snapshot
            .observability()
            .counters()
            .work_started()
            .exact_total()
            .unwrap(),
        0
    );
    assert_eq!(
        snapshot.observability().outcomes().last_outcome(),
        Some(TimerLastOutcome::Unacknowledged)
    );

    assert!(run_next_due());
    let completed = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(
        completed.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped,
        },
        "the retired attempt's reconciliation must not override the successor's Stop"
    );
    assert_eq!(
        completed
            .observability()
            .counters()
            .unacknowledged()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(
        completed
            .observability()
            .counters()
            .work_completed()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn immediate_watchdog_completion_fault_traps_and_leaves_cadence_successor_armed() {
    let _fixture = setup();
    let timer = identity("watchdog-completion-fault");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| {
            grow_memory_pages(2, 3);
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            )
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    inject_watchdog_completion_fault();
    let trapped = catch_unwind(AssertUnwindSafe(run_next_due));
    assert!(trapped.is_err(), "an internal completion failure must trap");
    assert_eq!(
        timer_count(),
        1,
        "the cadence successor was committed by the earlier scheduler message"
    );
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    let counters = snapshot.observability().counters();
    assert_eq!(counters.work_started().exact_total().unwrap(), 1);
    assert_eq!(counters.work_completed().exact_total().unwrap(), 0);
    let performance = snapshot.observability().performance();
    assert_eq!(performance.scheduler_memory_pages().samples(), 1);
    assert_eq!(performance.work_memory_pages().samples(), 0);
    assert_eq!(performance.work_memory_pages().latest(), None);
}

#[test]
fn immediate_watchdog_provider_replacement_failure_traps_for_message_rollback() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-provider-fault");
    let registration = register_watchdog(
        timer,
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| {
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            )
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due(), "scheduler should commit cadence safety");

    inject_provider_install_fault();
    let trapped = catch_unwind(AssertUnwindSafe(run_next_due));
    assert!(
        trapped.is_err(),
        "replacement binding failure must trap the work message"
    );
    // The native provider mock is deliberately not transactional. On the IC,
    // the trap rolls the clear/install and registry transition back together,
    // restoring the cadence successor committed by the scheduler message.
}

#[test]
fn public_immediate_watchdog_replacement_failure_retires_false_scheduled_state() {
    let _fixture = setup();
    let timer = identity("watchdog-public-immediate-provider-fault");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("cadence scheduler should arm");

    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled_immediately(),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(timer_count(), 0);
    assert_retained_provider_binding_failure(&timer, 2, 1);
}

#[test]
fn provider_cleanup_borrow_failure_is_returned_instead_of_discarded() {
    let _fixture = setup();
    let timer = identity("provider-cleanup-fault");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    let failure = RUNTIME.with(|runtime| {
        let _borrow = runtime.borrow();
        clear_entry_provider_handles(&timer)
    });
    assert!(matches!(failure, Err(TimerError::RuntimeBusy)));
    assert_eq!(timer_count(), 1, "failed cleanup must not lose the handle");
    registration
        .cancel()
        .expect("cleanup remains possible after the borrow is released");
    assert_eq!(timer_count(), 0);
    clear_entry_provider_handles(&timer).expect("empty entry cleanup should be harmless");
    registration.unregister().unwrap();
    clear_entry_provider_handles(&timer).expect("missing entry cleanup should be harmless");
    assert_eq!(timer_count(), 0);
}

#[test]
fn rejected_provider_binding_clears_handles_for_unavailable_or_expired_authority() {
    let _fixture = setup();
    let timer = identity("provider-binding-authority");
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("provider wake-up should arm");
    let detached = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
        .expect("registry should be available")
        .expect("provider handle should detach");
    let (token, handle) = detached;

    let rejected = RUNTIME.with(|runtime| {
        let _borrow = runtime.borrow();
        bind_provider_handle(&token, handle, ProviderBinding::Stage)
    });
    assert!(matches!(rejected, Err(TimerError::RuntimeBusy)));
    assert_eq!(timer_count(), 0);

    registration
        .unregister()
        .expect("declaration should be removable after the borrow is released");
    let handle = platform::set_timer(Duration::ZERO, async {});
    assert!(matches!(
        bind_provider_handle(&token, handle, ProviderBinding::Stage),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(timer_count(), 0);

    RUNTIME.with(|runtime| *runtime.borrow_mut() = None);
    let handle = platform::set_timer(Duration::ZERO, async {});
    assert!(matches!(
        bind_provider_handle(&token, handle, ProviderBinding::Stage),
        Err(TimerError::NotInitialized)
    ));
    assert_eq!(timer_count(), 0);
}

#[test]
fn unchanged_watchdog_successor_and_queued_work_stay_bound_without_reinstallation() {
    for abandon_successor in [false, true] {
        let _fixture = setup();
        let timer = identity("provider-unaffected-handles");
        let registration = register_watchdog(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            TimerStart::Retained {
                initial: Some(TimerSchedule::At(15)),
            },
            |_| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
        )
        .unwrap();
        set_time(15);
        assert!(run_next_due());
        assert_eq!(timer_count(), 2);
        if abandon_successor {
            assert!(platform::discard_timer_at(20));
            assert_eq!(timer_count(), 1);
            // Only the replacement scheduler binds. Queued work must remain
            // installed, without consuming another provider admission.
            inject_provider_install_fault_after(1);
            registration.ensure_scheduled().unwrap();
            assert_eq!(timer_count(), 2);
        } else {
            inject_provider_install_fault();
        }
        // Continue retains its prearmed successor without rebinding it.
        assert!(run_next_due());
        assert_eq!(timer_count(), 1);
        assert!(registration.has_armed_wakeup().unwrap());
        let snapshot = timer_snapshot(&timer).unwrap().unwrap();
        assert_eq!(snapshot.next_deadline_ns(), Some(20));
        assert_eq!(
            snapshot
                .observability()
                .counters()
                .work_completed()
                .exact_total(),
            Ok(1)
        );
        // The injected fault remains for the next real binding, proving neither
        // unaffected capability passed through the provider installer.
        assert!(matches!(
            registration.ensure_scheduled_immediately(),
            Err(TimerError::OwnershipInvariant)
        ));
        assert_eq!(timer_count(), 0);
        registration.unregister().unwrap();
    }
}

#[test]
fn terminal_scheduler_failure_clears_queued_work_before_transient_removal() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        let _fixture = setup();
        let timer = identity("terminal-scheduler-cleanup");
        let work = Rc::new(Cell::new(0));
        let observed_work = Rc::clone(&work);
        let registration = register_watchdog(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            scheduled_start(lifetime, TimerSchedule::After(Duration::from_nanos(5))),
            move |_| {
                observed_work.set(observed_work.get() + 1);
                WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue)
            },
        )
        .unwrap();
        set_time(15);
        assert!(run_next_due());
        assert_eq!(timer_count(), 2);

        // Deliver the successor before its deferred work. Consume the fake
        // provider callback exactly as a scheduler entry would, retaining work.
        let wakeup = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
            .unwrap()
            .unwrap();
        let (scheduler, handle) = wakeup;
        platform::clear_timer(handle);
        assert_eq!(timer_count(), 1);
        set_time(u64::MAX);
        dispatch_watchdog_scheduler(&scheduler);

        assert_eq!(timer_count(), 0);
        assert_eq!(work.get(), 0);
        assert!(!run_next_due());
        if lifetime == DeclarationLifetime::Retained {
            assert!(!registration.has_armed_wakeup().unwrap());
            assert_eq!(
                timer_snapshot(&timer).unwrap().unwrap().state(),
                TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::ControlFailure(TimerControlFailure::DeadlineOverflow),
                }
            );
        } else {
            assert!(timer_snapshot(&timer).unwrap().is_none());
            assert!(matches!(
                registration.has_armed_wakeup(),
                Err(TimerError::RegistrationExpired)
            ));
            assert!(
                register_once(timer, TimerStart::Retained { initial: None }, |_| async {
                    OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
                })
                .is_ok()
            );
        }
    }
}

#[test]
fn rejected_and_coalesced_requests_preserve_bound_handles_without_reinstallation() {
    let _fixture = setup();
    let timer = identity("request-handle-preservation");
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        |_| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .unwrap();
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .unwrap();
    let before = timer_snapshot(&timer).unwrap().unwrap();

    // If either request attempts a handle reinstall it consumes this fault.
    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::After(Duration::MAX)),
        Err(TimerError::Schedule(ScheduleError::DelayOutOfRange))
    ));
    assert_eq!(timer_snapshot(&timer).unwrap().unwrap(), before);
    registration
        .ensure_scheduled(TimerSchedule::At(20))
        .unwrap();
    assert!(registration.has_armed_wakeup().unwrap());
    assert_eq!(timer_count(), 1);
    let after = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(after.generation(), before.generation());
    assert_eq!(after.next_deadline_ns(), before.next_deadline_ns());
    assert_eq!(
        after
            .observability()
            .counters()
            .coalesced()
            .exact_total()
            .unwrap(),
        1
    );

    // A real replacement must still encounter the unconsumed installation fault.
    assert!(matches!(
        registration.reconcile_schedule(Some(TimerSchedule::At(25))),
        Err(TimerError::OwnershipInvariant)
    ));
    assert!(!registration.has_armed_wakeup().unwrap());
    assert_eq!(timer_count(), 0);
    registration.unregister().unwrap();
}

#[test]
fn public_provider_install_failure_retires_false_scheduled_state() {
    let _fixture = setup();
    let timer = identity("provider-install-fault");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_once(
        timer.clone(),
        TimerStart::Retained { initial: None },
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) }
        },
    )
    .expect("registration should succeed");

    registration
        .ensure_scheduled(TimerSchedule::At(100))
        .expect("initial provider arm should succeed");
    assert_eq!(timer_count(), 1);
    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(15)),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(
        timer_count(),
        0,
        "failed replacement must clear both old and new provider arms"
    );
    assert_retained_provider_binding_failure(&timer, 2, 1);

    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("retained registration should recover on a later request");
    assert_eq!(timer_count(), 1);
    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 0);
}

#[test]
fn inactive_ordinary_provider_install_failure_retires_false_scheduled_state() {
    for recurring in [false, true] {
        let _fixture = setup();
        let timer = identity("inactive-ordinary-provider-fault");
        // Keep both public control entry points covered; initial registration
        // and replacement failures have distinct lifecycle tests.
        let result = if recurring {
            let registration = register_after_completion(
                timer.clone(),
                TimerCadence::from_nanos(5).unwrap(),
                TimerStart::Retained { initial: None },
                |_| async {
                    AfterCompletionRunResult::new(
                        TimerCompletion::no_work(),
                        AfterCompletionDecision::Stop,
                    )
                },
            )
            .unwrap();
            inject_provider_install_fault();
            registration.ensure_scheduled()
        } else {
            let registration = register_once(
                timer.clone(),
                TimerStart::Retained { initial: None },
                |_| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
            )
            .unwrap();
            inject_provider_install_fault();
            registration.ensure_scheduled(TimerSchedule::At(15))
        };
        assert!(matches!(result, Err(TimerError::OwnershipInvariant)));
        assert_eq!(timer_count(), 0);
        assert_retained_provider_binding_failure(&timer, 1, 0);
    }
}

#[test]
fn watchdog_failed_dispatch_clears_handles_without_confirming_work() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        for failure_stage in [
            "successor-admission",
            "work-admission",
            "successor-binding",
            "work-binding",
        ] {
            let _fixture = setup();
            let timer = identity("provider-watchdog-dispatch-fault");
            let calls = Rc::new(Cell::new(0));
            let callback_calls = Rc::clone(&calls);
            let registration = register_watchdog(
                timer.clone(),
                TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
                scheduled_start(lifetime, TimerSchedule::After(Duration::from_nanos(5))),
                move |_context| {
                    callback_calls.set(callback_calls.get() + 1);
                    WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue)
                },
            )
            .expect("registration should succeed");

            match failure_stage {
                "successor-admission" => {
                    platform::ADMISSION_FAULT.set(Some((0, platform::AdmissionError::Capacity)));
                }
                "work-admission" => {
                    platform::ADMISSION_FAULT
                        .set(Some((1, platform::AdmissionError::IdentityExhausted)));
                }
                "successor-binding" => inject_provider_install_fault(),
                "work-binding" => inject_provider_install_fault_after(1),
                _ => unreachable!("closed failure stages"),
            }
            set_time(15);
            assert!(run_next_due(), "scheduler callback should execute");
            assert_eq!(timer_count(), 0, "failed dispatch must clear every handle");
            assert!(
                !run_next_due(),
                "failed dispatch must not leave queued work"
            );
            assert_eq!(
                calls.get(),
                0,
                "failed dispatch must not invoke consumer work"
            );

            if lifetime == DeclarationLifetime::Retained {
                assert_retained_provider_binding_failure(&timer, 1, 1);
                assert!(!registration.has_armed_wakeup().unwrap());
                let snapshot = timer_snapshot(&timer).unwrap().unwrap();
                let counters = snapshot.observability().counters();
                assert_eq!(counters.scheduler_started().exact_total().unwrap(), 1);
                assert_eq!(counters.work_dispatched().exact_total().unwrap(), 0);
                assert_eq!(counters.work_started().exact_total().unwrap(), 0);
                assert_eq!(counters.work_completed().exact_total().unwrap(), 0);
            } else {
                assert!(timer_snapshot(&timer).unwrap().is_none());
                assert!(matches!(
                    registration.has_armed_wakeup(),
                    Err(TimerError::RegistrationExpired)
                ));
                let replacement =
                    register_once(timer, TimerStart::Retained { initial: None }, |_| async {
                        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
                    })
                    .expect("failed transient dispatch should release the identity");
                replacement.ensure_scheduled(TimerSchedule::At(25)).unwrap();
                assert!(replacement.has_armed_wakeup().unwrap());
                assert!(matches!(
                    registration.has_armed_wakeup(),
                    Err(TimerError::RegistrationExpired)
                ));
                assert_eq!(timer_count(), 1);
                replacement.unregister().unwrap();
                assert_eq!(timer_count(), 0);
            }
        }
    }
}

#[test]
fn stale_watchdog_dispatch_clears_partial_arms_without_confirming_work() {
    let _fixture = setup();
    let registration = register_watchdog(
        identity("stale-dispatch"),
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        |_| panic!("manual effects must not execute consumer work"),
    )
    .unwrap();
    registration.ensure_scheduled().unwrap();
    let handle = with_registry_mut(|registry| {
        Ok(registry
            .take_wakeup_handle(registration.identity())
            .unwrap())
    })
    .unwrap();
    let (scheduler, handle) = handle;
    platform::clear_timer(handle);
    let first = with_registry_mut(|registry| Ok(registry.begin_watchdog_scheduler(&scheduler, 15)))
        .unwrap();
    let RegistryEffect::DispatchWatchdog { dispatch, .. } = first.effect() else {
        panic!("dispatch expected")
    };
    let successor = dispatch.successor().clone();
    let second =
        with_registry_mut(|registry| Ok(registry.begin_watchdog_scheduler(&successor, 20)))
            .unwrap();
    let before = timer_inventory().unwrap();
    assert!(apply_effect(first.effect(), ProviderHandles::default()).is_err());
    assert_eq!(timer_count(), 0);
    assert_eq!(timer_inventory().unwrap(), before);
    apply_effect(second.effect(), ProviderHandles::default()).unwrap();
    assert_eq!(timer_count(), 2);
    let counts = timer_snapshot(registration.identity())
        .unwrap()
        .unwrap()
        .observability()
        .counters();
    assert_eq!(counts.wakeups_armed().exact_total().unwrap(), 2);
    assert_eq!(counts.work_dispatched().exact_total().unwrap(), 1);
}

#[test]
fn remove_on_stop_provider_failure_removes_the_expired_claim() {
    let _fixture = setup();
    let timer = identity("provider-remove-on-stop-fault");
    let registration = register_once(
        timer.clone(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(20),
        },
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("registration should succeed");

    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(15)),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(timer_count(), 0);
    assert!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .is_none(),
        "remove-on-stop failure must not retain a false declaration"
    );
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(20)),
        Err(TimerError::RegistrationExpired)
    ));
}

#[test]
fn overdue_watchdog_coalesces_to_one_dispatch_and_schedules_from_now() {
    let _fixture = setup();
    let timer = identity("watchdog-overdue");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::Retained { initial: None },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(1_000);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(1_005)
    );
    assert!(run_next_due());
    assert_eq!(timer_count(), 1);
    assert!(!run_next_due(), "no historical cadence point may replay");
}

#[test]
fn remove_on_stop_watchdog_clears_successor_before_dropping_entry() {
    let _fixture = setup();
    let timer = identity("watchdog-remove");
    let _registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_nanos(5)),
        },
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    assert!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .is_none()
    );
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered reconstruction and external-demand sequence."
)]
fn reconstruction_and_external_demand_are_synchronous_and_idempotent() {
    let _fixture = setup();
    let timer = identity("reconstructed-watchdog");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");
    let result = Rc::new(Cell::new(WatchdogRunResult::new(
        TimerCompletion::success(1),
        WatchdogDecision::ContinueImmediately,
    )));
    let pages = Rc::new(Cell::new(0_u64));
    let mut registration = None;

    let callback_result = Rc::clone(&result);
    let callback_pages = Rc::clone(&pages);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
        move |_context| {
            advance_instructions(13);
            let result = callback_result.get();
            callback_pages.set(callback_pages.get() + result.completion().work_count());
            result
        },
    )
    .expect("fresh reconstruction should register and schedule");
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
        |_context| {
            WatchdogRunResult::new(
                TimerCompletion::invariant_failure(0),
                WatchdogDecision::Stop,
            )
        },
    )
    .expect("repeated reconstruction should reuse the original callback claim");
    assert_eq!(timer_count(), 1);

    run_watchdog_round(15);
    assert_eq!((pages.get(), timer_count()), (1, 1));

    result.set(WatchdogRunResult::new(
        TimerCompletion::retryable_failure(0),
        WatchdogDecision::Continue,
    ));
    run_watchdog_round(20);
    assert_eq!((pages.get(), timer_count()), (1, 1));
    let retrying = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retryable failure must retain the watchdog");
    assert!(matches!(
        retrying.state(),
        TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled { .. })
    ));
    let retrying_counters = retrying.observability().counters();
    assert_eq!(retrying_counters.work_completed().exact_total().unwrap(), 2);
    assert_eq!(
        retrying_counters.retryable_failure().exact_total().unwrap(),
        1
    );

    result.set(WatchdogRunResult::new(
        TimerCompletion::no_work(),
        WatchdogDecision::Stop,
    ));
    run_watchdog_round(25);
    assert_eq!((pages.get(), timer_count()), (1, 0));
    let stopped = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    assert_lifecycle_measurements(&stopped);

    result.set(WatchdogRunResult::new(
        TimerCompletion::retryable_failure(0),
        WatchdogDecision::Continue,
    ));
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("returned-error commit guard should synchronously restore a wake-up");
    assert_eq!(timer_count(), 1);

    set_time(30);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    result.set(WatchdogRunResult::new(
        TimerCompletion::invariant_failure(0),
        WatchdogDecision::Stop,
    ));
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    let terminal = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    assert_eq!(
        terminal.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::InvariantFailure,
        }
    );
    let terminal_counters = terminal.observability().counters();
    assert_eq!(terminal_counters.work_completed().exact_total().unwrap(), 4);
    assert_eq!(
        terminal_counters.invariant_failure().exact_total().unwrap(),
        1
    );
    assert_eq!(terminal_counters.succeeded().exact_total().unwrap(), 1);
    assert_eq!(terminal_counters.no_work().exact_total().unwrap(), 1);
    assert_eq!(
        terminal_counters.retryable_failure().exact_total().unwrap(),
        1
    );

    result.set(WatchdogRunResult::new(
        TimerCompletion::success(1),
        WatchdogDecision::ContinueImmediately,
    ));
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("retained terminal declaration can be reconstructed from durable demand");
    assert_eq!(timer_count(), 1);
    set_time(35);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("terminal durable authority should cancel successor and queued work");
    assert_eq!(timer_count(), 0);

    let mismatch = reconcile_watchdog(
        &mut registration,
        &timer,
        TimerCadence::from_nanos(6).expect("fixture cadence should be valid"),
        WatchdogReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    );
    assert!(matches!(mismatch, Err(TimerError::ReconciliationConflict)));
}

fn run_watchdog_round(now_ns: u64) {
    set_time(now_ns);
    assert!(run_next_due(), "scheduler message must run at {now_ns}");
    assert!(run_next_due(), "work message must run at {now_ns}");
}

fn assert_lifecycle_measurements(snapshot: &TimerSnapshot) {
    let performance = snapshot.observability().performance();
    for (measurement, samples) in [
        (
            "scheduler instructions",
            performance.scheduler_instructions().aggregate().samples(),
        ),
        (
            "work instructions",
            performance.work_instructions().aggregate().samples(),
        ),
        (
            "scheduler memory",
            performance.scheduler_memory_pages().samples(),
        ),
        ("work memory", performance.work_memory_pages().samples()),
    ] {
        assert_eq!(
            samples, 3,
            "{measurement} after three completed work rounds"
        );
    }
    assert_eq!(performance.scheduler_instructions().latest(), Some(10));
    assert!(
        performance
            .work_instructions()
            .latest()
            .is_some_and(|value| value >= 13)
    );
}

fn assert_reconciliation_conflict_preserves_runtime(
    result: &Result<(), TimerError>,
    before: &TimerInventorySnapshot,
) {
    assert!(matches!(result, Err(TimerError::ReconciliationConflict)));
    assert_eq!(&timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
}

fn assert_original_callback_completes(calls: &Cell<u64>, now_ns: u64) {
    set_time(now_ns);
    assert!(run_next_due());
    assert_eq!(
        calls.get(),
        1,
        "the original callback must remain installed"
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn once_reconciliation_rejects_identity_mismatch_without_disturbing_live_work() {
    let _fixture = setup();
    let timer = identity("reconcile-once-identity");
    let other = identity("reconcile-once-other");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = Some(
        register_once(
            timer.clone(),
            TimerStart::Retained { initial: None },
            move |_| {
                callback_calls.set(callback_calls.get() + 1);
                async { OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop) }
            },
        )
        .unwrap(),
    );
    registration
        .as_ref()
        .unwrap()
        .ensure_scheduled(TimerSchedule::At(20))
        .unwrap();
    let before = timer_inventory().unwrap();

    let rejected = reconcile_once(&mut registration, &other, None, |_| async {
        OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
    });
    assert_reconciliation_conflict_preserves_runtime(&rejected, &before);
    let original = registration.as_ref().unwrap();
    assert_eq!(original.identity(), &timer);
    assert!(original.has_armed_wakeup().unwrap());

    assert_original_callback_completes(&calls, 20);
}

#[test]
fn after_completion_reconciliation_rejects_transient_lifetime_without_cancelling_work() {
    let _fixture = setup();
    let timer = identity("reconcile-after-lifetime");
    let cadence = TimerCadence::from_nanos(5).unwrap();
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = Some(
        register_after_completion(
            timer.clone(),
            cadence,
            TimerStart::RemoveWhenStopped {
                initial: TimerSchedule::After(Duration::from_nanos(5)),
            },
            move |_| {
                callback_calls.set(callback_calls.get() + 1);
                async {
                    AfterCompletionRunResult::new(
                        TimerCompletion::success(1),
                        AfterCompletionDecision::Stop,
                    )
                }
            },
        )
        .unwrap(),
    );
    let before = timer_inventory().unwrap();

    let rejected = reconcile_after_completion(
        &mut registration,
        &timer,
        cadence,
        TimerReconcileState::Inactive,
        |_| async {
            AfterCompletionRunResult::new(
                TimerCompletion::invariant_failure(0),
                AfterCompletionDecision::Stop,
            )
        },
    );
    assert_reconciliation_conflict_preserves_runtime(&rejected, &before);
    let original = registration.as_ref().unwrap();
    assert_eq!(original.identity(), &timer);
    assert!(original.has_armed_wakeup().unwrap());

    assert_original_callback_completes(&calls, 15);
    assert!(timer_snapshot(&timer).unwrap().is_none());
}

#[test]
fn after_completion_reconciliation_rejects_cadence_mismatch_without_disturbing_live_work() {
    let _fixture = setup();
    let timer = identity("reconcile-after-cadence");
    let cadence = TimerCadence::from_nanos(5).unwrap();
    let other_cadence = TimerCadence::from_nanos(6).unwrap();
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = Some(
        register_after_completion(
            timer.clone(),
            cadence,
            TimerStart::Retained { initial: None },
            move |_| {
                callback_calls.set(callback_calls.get() + 1);
                async {
                    AfterCompletionRunResult::new(
                        TimerCompletion::success(1),
                        AfterCompletionDecision::Stop,
                    )
                }
            },
        )
        .unwrap(),
    );
    registration.as_ref().unwrap().ensure_scheduled().unwrap();
    let before = timer_inventory().unwrap();

    let rejected = reconcile_after_completion(
        &mut registration,
        &timer,
        other_cadence,
        TimerReconcileState::Inactive,
        |_| async {
            AfterCompletionRunResult::new(
                TimerCompletion::invariant_failure(0),
                AfterCompletionDecision::Stop,
            )
        },
    );
    assert_reconciliation_conflict_preserves_runtime(&rejected, &before);
    let original = registration.as_ref().unwrap();
    assert_eq!(original.identity(), &timer);
    assert!(original.has_armed_wakeup().unwrap());

    set_time(14);
    assert!(!run_next_due());
    assert_eq!(calls.get(), 0);
    assert_original_callback_completes(&calls, 15);
}

#[test]
fn watchdog_reconciliation_rejects_reused_identity_claim_without_clearing_replacement() {
    let _fixture = setup();
    let timer = identity("reconcile-watchdog-expired");
    let cadence = TimerCadence::from_nanos(5).unwrap();
    let calls = Rc::new(Cell::new(0_u64));
    let expired_calls = Rc::clone(&calls);
    let mut expired = Some(
        register_watchdog(
            timer.clone(),
            cadence,
            TimerStart::RemoveWhenStopped {
                initial: TimerSchedule::After(Duration::from_nanos(5)),
            },
            move |_| {
                expired_calls.set(expired_calls.get() + 1);
                WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
            },
        )
        .unwrap(),
    );
    expired.as_ref().unwrap().cancel().unwrap();
    assert!(timer_snapshot(&timer).unwrap().is_none());
    let replacement_count = Rc::new(Cell::new(0_u64));
    let replacement_calls = Rc::clone(&replacement_count);
    let replacement = register_watchdog(
        timer.clone(),
        cadence,
        TimerStart::Retained { initial: None },
        move |_| {
            replacement_calls.set(replacement_calls.get() + 1);
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
        },
    )
    .unwrap();
    replacement.ensure_scheduled().unwrap();
    let before = timer_inventory().unwrap();

    let rejected = reconcile_watchdog(
        &mut expired,
        &timer,
        cadence,
        WatchdogReconcileState::Inactive,
        |_| {
            WatchdogRunResult::new(
                TimerCompletion::invariant_failure(0),
                WatchdogDecision::Stop,
            )
        },
    );
    assert!(matches!(rejected, Err(TimerError::RegistrationExpired)));
    assert!(matches!(
        expired.as_ref().unwrap().has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    assert!(replacement.has_armed_wakeup().unwrap());

    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 0, "expired work must not execute");
    assert_eq!(
        replacement_count.get(),
        1,
        "replacement work must execute once"
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn after_completion_reconstruction_reuses_its_exact_claim() {
    let _fixture = setup();
    let timer = identity("after-reconstruct");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = None;
    reconcile_after_completion(
        &mut registration,
        &timer,
        cadence,
        TimerReconcileState::Scheduled,
        move |_context| {
            callback_calls.set(callback_calls.get().saturating_add(1));
            async {
                AfterCompletionRunResult::new(
                    TimerCompletion::no_work(),
                    AfterCompletionDecision::Stop,
                )
            }
        },
    )
    .expect("fresh reconstruction should succeed");
    reconcile_after_completion(
        &mut registration,
        &timer,
        cadence,
        TimerReconcileState::Scheduled,
        |_context| async {
            AfterCompletionRunResult::new(
                TimerCompletion::invariant_failure(0),
                AfterCompletionDecision::Stop,
            )
        },
    )
    .expect("repeated reconstruction should coalesce");
    registration
        .as_ref()
        .expect("reconstruction should retain its claim")
        .reconcile_schedule(Some(TimerSchedule::At(25)))
        .expect("ordinary reconciliation should replace a later deadline exactly");
    assert_eq!(timer_count(), 1);
    set_time(25);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
}

#[test]
fn once_reconciliation_owns_one_exact_deadline_and_retains_its_callback() {
    let _fixture = setup();
    let timer = identity("once-reconstruct");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = None;

    reconcile_once(
        &mut registration,
        &timer,
        Some(TimerSchedule::At(20)),
        move |_context| {
            callback_calls.set(callback_calls.get().saturating_add(1));
            async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) }
        },
    )
    .expect("fresh reconstruction should register and arm");
    reconcile_once(
        &mut registration,
        &timer,
        Some(TimerSchedule::At(40)),
        |_context| async {
            OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
        },
    )
    .expect("authoritative reconciliation should move the deadline later");
    assert_eq!(timer_count(), 1);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(40)
    );

    reconcile_once(&mut registration, &timer, None, |_context| async {
        OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
    })
    .expect("inactive reconciliation should clear the exact handle");
    assert_eq!(timer_count(), 0);

    registration
        .as_ref()
        .expect("retained claim should remain available")
        .reconcile_schedule(Some(TimerSchedule::At(50)))
        .expect("retained registration should re-arm");
    set_time(50);
    assert!(run_next_due());
    assert_eq!(
        calls.get(),
        1,
        "reconciliation must retain the first callback"
    );
}

#[test]
fn registration_identity_survives_control_but_changes_on_replacement_and_restart() {
    let fixture = setup();
    let epoch = fixture.epoch;
    let timer = identity("continuity");
    let create = || {
        register_once(
            timer.clone(),
            TimerStart::Retained { initial: None },
            |_| async {
                advance_instructions(7);
                OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
            },
        )
        .unwrap()
    };
    let snapshot = || timer_snapshot(&timer).unwrap().unwrap();
    let first = create();
    let original = snapshot().registration_id();
    assert_eq!(original.epoch(), epoch);
    assert_eq!(original.sequence(), 1);
    for _ in 0..2 {
        first.ensure_scheduled(TimerSchedule::At(10)).unwrap();
        assert!(run_next_due());
    }
    let completed = snapshot().observability().performance();
    assert_eq!(completed.work_instructions().aggregate().samples(), 2);
    assert_eq!(completed.work_instructions().aggregate().total(), 14);
    assert_eq!(
        completed.work_instructions().aggregate().mean(),
        Ok(Some(7))
    );
    assert_eq!(completed.work_instructions().latest(), Some(7));
    assert_eq!(completed.work_instructions().aggregate().maximum(), Some(7));
    assert_eq!(
        completed.scheduler_instructions(),
        crate::MeasurementSummary::EMPTY
    );
    first.cancel().unwrap();
    let before = snapshot();
    assert_eq!(before.registration_id(), original);
    assert_eq!(before.observability().performance(), completed);
    first.unregister().unwrap();
    let replacement = create();
    assert_ne!(snapshot().registration_id(), original);
    assert_eq!(snapshot().registration_id().epoch(), epoch);
    let fresh = snapshot().observability().performance();
    assert_eq!(fresh.work_instructions(), crate::MeasurementSummary::EMPTY);
    assert_eq!(
        fresh.scheduler_instructions(),
        crate::MeasurementSummary::EMPTY
    );
    for _ in 0..3 {
        replacement.ensure_scheduled(TimerSchedule::At(10)).unwrap();
        assert!(run_next_due());
    }
    let regrown = snapshot();
    assert!(
        regrown
            .observability()
            .counters()
            .work_completed()
            .exact_total()
            .unwrap()
            > before
                .observability()
                .counters()
                .work_completed()
                .exact_total()
                .unwrap()
    );
    assert_ne!(regrown.registration_id(), before.registration_id());
    let regrown_work = regrown.observability().performance().work_instructions();
    assert_eq!(regrown_work.aggregate().samples(), 3);
    assert_eq!(regrown_work.aggregate().total(), 21);
    assert_eq!(regrown_work.aggregate().mean(), Ok(Some(7)));
    // Arithmetic alone accepts a reset followed by larger measurements.
    // Different registration identities forbid treating this as one interval.
    let misleading_interval = regrown_work
        .aggregate()
        .interval_since(completed.work_instructions().aggregate())
        .unwrap();
    assert_eq!(misleading_interval.samples(), 1);
    assert_eq!(misleading_interval.total(), 7);
    reset_for_test(20, 8);
    initialize_runtime().unwrap();
    let _reconstructed = create();
    assert_eq!(snapshot().registration_id().sequence(), original.sequence());
    assert_ne!(snapshot().registration_id().epoch(), original.epoch());
    assert_ne!(snapshot().registration_id(), original);
    let reconstructed = snapshot().observability().performance();
    assert_eq!(
        reconstructed.work_instructions(),
        crate::MeasurementSummary::EMPTY
    );
    assert_eq!(
        reconstructed.scheduler_instructions(),
        crate::MeasurementSummary::EMPTY
    );
}

#[test]
fn watchdog_exact_deadline_reconstruction_moves_both_directions_and_sleeps() {
    let _fixture = setup();
    let timer = identity("deadline");
    let mut registration = None;
    reconcile_watchdog(
        &mut registration,
        &timer,
        TimerCadence::from_nanos(5).unwrap(),
        WatchdogReconcileState::ScheduledAt(20),
        |_| WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop),
    )
    .unwrap();
    let registration = registration.unwrap();
    let initial = timer_snapshot(&timer).unwrap().unwrap();
    for deadline in [40, 15, 15] {
        registration
            .reconcile_schedule(Some(TimerSchedule::At(deadline)))
            .unwrap();
        let current = timer_snapshot(&timer).unwrap().unwrap();
        assert_eq!(current.next_deadline_ns(), Some(deadline));
        assert_eq!(current.registration_id(), initial.registration_id());
        assert_eq!(timer_count(), 1);
    }
    let scheduled = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(
        scheduled
            .observability()
            .counters()
            .wakeups_armed()
            .exact_total()
            .unwrap(),
        3
    );
    assert_eq!(
        scheduled
            .observability()
            .counters()
            .coalesced()
            .exact_total()
            .unwrap(),
        1
    );
    assert!(!run_next_due());
    registration.reconcile_schedule(None).unwrap();
    assert_eq!(timer_count(), 0);
    registration
        .reconcile_schedule(Some(TimerSchedule::At(0)))
        .unwrap();
    assert_eq!(
        timer_snapshot(&timer)
            .unwrap()
            .unwrap()
            .latest_armed_delay_ns(),
        Some(0)
    );
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().registration_id(),
        initial.registration_id()
    );
}

#[test]
fn watchdog_deadline_decision_replaces_only_the_prearmed_successor() {
    let _fixture = setup();
    let timer = identity("deadline-result");
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        move |_| {
            callback_calls.set(callback_calls.get() + 1);
            assert_eq!(timer_count(), 1);
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                if callback_calls.get() == 1 {
                    WatchdogDecision::ScheduleAt(100)
                } else {
                    WatchdogDecision::Stop
                },
            )
        },
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    assert!(run_next_due());
    let completed = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(completed.next_deadline_ns(), Some(100));
    assert_eq!(completed.scheduling_mode(), TimerSchedulingMode::Deadline);
    assert_eq!(completed.latest_requested_delay_ns(), None);
    assert_eq!(completed.latest_armed_delay_ns(), Some(90));
    assert_eq!(timer_count(), 1);
    set_time(99);
    assert!(!run_next_due());
    set_time(100);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_dispatched_reconciliation_preserves_recovery_until_completion() {
    let _fixture = setup();
    let timer = identity("dispatched-deadline");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        |_| WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop),
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    registration
        .reconcile_schedule(Some(TimerSchedule::At(100)))
        .unwrap();
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().next_deadline_ns(),
        Some(15)
    );
    assert_eq!(timer_count(), 2);
    assert!(run_next_due());
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().next_deadline_ns(),
        Some(100)
    );
    assert_eq!(timer_count(), 1);
    registration.cancel().unwrap();
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_deadline_context_arbitrates_and_expires() {
    let _fixture = setup();
    let timer = identity("deadline-context");
    let saved = Rc::new(RefCell::new(None));
    let callback_saved = Rc::clone(&saved);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        move |context| {
            context.cancel().unwrap();
            context.ensure_scheduled_immediately().unwrap();
            context
                .reconcile_schedule(Some(TimerSchedule::After(Duration::from_nanos(30))))
                .unwrap();
            context.ensure_scheduled().unwrap();
            *callback_saved.borrow_mut() = Some(context);
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
        },
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    assert!(run_next_due());
    let current = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(current.next_deadline_ns(), Some(40));
    assert_eq!(current.latest_requested_delay_ns(), Some(30));
    assert!(matches!(
        saved
            .borrow()
            .as_ref()
            .unwrap()
            .reconcile_schedule(Some(TimerSchedule::At(60))),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        saved.borrow().as_ref().unwrap().reconcile_schedule(None),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(timer_count(), 1);
}

#[test]
fn invalid_watchdog_deadline_request_preserves_the_live_schedule() {
    let _fixture = setup();
    let timer = identity("invalid-deadline");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        |_| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .unwrap();
    registration.ensure_scheduled().unwrap();
    let before = timer_snapshot(&timer).unwrap();
    for delay in [
        Duration::from_secs(u64::MAX),
        Duration::from_nanos(u64::MAX),
    ] {
        assert!(matches!(
            registration.reconcile_schedule(Some(TimerSchedule::After(delay))),
            Err(TimerError::Schedule(_))
        ));
        assert_eq!(timer_snapshot(&timer).unwrap(), before);
        assert_eq!(timer_count(), 1);
    }
}

#[test]
fn transient_watchdog_deadline_cancellation_clears_both_owned_handles() {
    for dispatched in [false, true] {
        let _fixture = setup();
        let timer = identity("transient-deadline");
        let registration = register_watchdog(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            TimerStart::RemoveWhenStopped {
                initial: TimerSchedule::At(10),
            },
            |_| panic!("cancelled work must not execute"),
        )
        .unwrap();
        if dispatched {
            assert!(run_next_due());
        }
        assert_eq!(timer_count(), if dispatched { 2 } else { 1 });
        registration.reconcile_schedule(None).unwrap();
        assert!(timer_snapshot(&timer).unwrap().is_none());
        assert_eq!(timer_count(), 0);
        assert!(matches!(
            registration.reconcile_schedule(Some(TimerSchedule::At(20))),
            Err(TimerError::RegistrationExpired)
        ));
    }
}

#[test]
fn recovery_retires_an_interrupted_attempts_exact_deadline_proposal() {
    let _fixture = setup();
    let timer = identity("retired-deadline");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        |_| WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue),
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    let before = timer_snapshot(&timer).unwrap().unwrap();
    registration
        .reconcile_schedule(Some(TimerSchedule::At(100)))
        .unwrap();
    assert!(discard_next_due());
    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    let recovered = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(recovered.next_deadline_ns(), Some(20));
    assert_eq!(
        recovered
            .observability()
            .counters()
            .unacknowledged()
            .exact_total()
            .unwrap(),
        1
    );
    assert_eq!(recovered.registration_id(), before.registration_id());
    assert_eq!(timer_count(), 1);
}

#[test]
fn coalesced_watchdog_requests_preserve_paired_handles_without_reinstallation() {
    let _fixture = setup();
    let timer = identity("paired-request-preservation");
    let registration = register_watchdog(
        timer,
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::Retained { initial: None },
        |context| {
            inject_provider_install_fault();
            context.ensure_scheduled().unwrap();
            context.ensure_scheduled_immediately().unwrap();
            // The pending immediate request would replace the successor; Stop
            // via exact cancellation wins and needs no handle installation.
            context.cancel().unwrap();
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
        },
    )
    .unwrap();
    registration.ensure_scheduled().unwrap();
    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    inject_provider_install_fault();
    registration.ensure_scheduled().unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert_eq!(timer_count(), 2);
    // Read/reset the injection so actual work can complete without an unrelated
    // injected failure; a no-op demand must not have consumed it.
    assert!(take_provider_install_fault());
    assert!(run_next_due());
    assert!(take_provider_install_fault());
    assert_eq!(timer_count(), 0);
    registration.unregister().unwrap();
}

struct RemovalCapture {
    timer: TimerIdentity,
    dropped: Rc<Cell<bool>>,
}

impl Drop for RemovalCapture {
    fn drop(&mut self) {
        assert!(timer_snapshot(&self.timer).unwrap().is_none());
        assert_eq!(
            timer_count(),
            0,
            "provider cleanup must precede capture Drop"
        );
        // Reuse the removed identity and capacity from consumer destruction.
        // This also proves nested mutation can run without a registry borrow.
        let replacement = register_once(
            self.timer.clone(),
            TimerStart::Retained { initial: None },
            |_| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
        )
        .unwrap();
        replacement.ensure_scheduled(TimerSchedule::At(20)).unwrap();
        replacement.unregister().unwrap();
        self.dropped.set(true);
    }
}

#[test]
fn cancellation_and_unregistration_release_captures_after_provider_cleanup() {
    for policy in TestPolicy::ALL {
        for (armed, unregister) in [(false, true), (true, true), (true, false)] {
            let _fixture = setup();
            let timer = identity("normal-removal-capture");
            let dropped = Rc::new(Cell::new(false));
            let captured = RemovalCapture {
                timer: timer.clone(),
                dropped: Rc::clone(&dropped),
            };
            let start = if unregister {
                TimerStart::Retained {
                    initial: armed.then_some(TimerSchedule::At(20)),
                }
            } else {
                TimerStart::RemoveWhenStopped {
                    initial: TimerSchedule::At(20),
                }
            };
            let claim = match policy {
                TestPolicy::Once => {
                    register_once(
                        timer,
                        start,
                        stopping_once(move || {
                            std::hint::black_box(&captured);
                        }),
                    )
                    .unwrap()
                    .claim
                }
                TestPolicy::AfterCompletion => {
                    register_after_completion(
                        timer,
                        TimerCadence::from_nanos(5).unwrap(),
                        start,
                        stopping_after_completion(move || {
                            std::hint::black_box(&captured);
                        }),
                    )
                    .unwrap()
                    .claim
                }
                TestPolicy::Watchdog => {
                    register_watchdog(
                        timer,
                        TimerCadence::from_nanos(5).unwrap(),
                        start,
                        move |_| {
                            std::hint::black_box(&captured);
                            WatchdogRunResult::new(
                                TimerCompletion::no_work(),
                                WatchdogDecision::Stop,
                            )
                        },
                    )
                    .unwrap()
                    .claim
                }
            };
            if unregister {
                unregister_claim(&claim).unwrap();
            } else {
                cancel_claim(&claim, None).unwrap();
            }
            assert!(dropped.get());
        }
    }
}

#[test]
fn rejected_registration_releases_its_captures_outside_registry_borrow() {
    struct RejectedCapture(Rc<Cell<bool>>);
    impl Drop for RejectedCapture {
        fn drop(&mut self) {
            assert_eq!(timer_inventory().unwrap().len(), 1);
            self.0.set(true);
        }
    }
    let _fixture = setup();
    let timer = identity("duplicate-capture");
    let original = register_once(
        timer.clone(),
        TimerStart::Retained {
            initial: Some(TimerSchedule::At(20)),
        },
        |_| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .unwrap();
    for policy in TestPolicy::ALL {
        let dropped = Rc::new(Cell::new(false));
        let captured = RejectedCapture(Rc::clone(&dropped));
        let rejected = match policy {
            TestPolicy::Once => register_once(
                timer.clone(),
                TimerStart::Retained {
                    initial: Some(TimerSchedule::At(20)),
                },
                stopping_once(move || {
                    std::hint::black_box(&captured);
                }),
            )
            .map(|_| ()),
            TestPolicy::AfterCompletion => register_after_completion(
                timer.clone(),
                TimerCadence::from_nanos(5).unwrap(),
                TimerStart::Retained {
                    initial: Some(TimerSchedule::At(20)),
                },
                stopping_after_completion(move || {
                    std::hint::black_box(&captured);
                }),
            )
            .map(|_| ()),
            TestPolicy::Watchdog => register_watchdog(
                timer.clone(),
                TimerCadence::from_nanos(5).unwrap(),
                TimerStart::Retained {
                    initial: Some(TimerSchedule::At(20)),
                },
                stopping_watchdog(move || {
                    std::hint::black_box(&captured);
                }),
            )
            .map(|_| ()),
        };
        assert!(matches!(
            rejected,
            Err(TimerError::Register(
                RegisterError::IdentityAlreadyRegistered(_)
            ))
        ));
        assert!(dropped.get());
    }
    original.unregister().unwrap();
}

#[test]
fn terminal_provider_and_scheduler_failures_release_captures_outside_registry_borrow() {
    let _fixture = setup();
    let timer = identity("binding-failure-capture");
    let dropped = Rc::new(Cell::new(false));
    let captured = RemovalCapture {
        timer: timer.clone(),
        dropped: Rc::clone(&dropped),
    };
    let registration = register_once(
        timer,
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(30),
        },
        stopping_once(move || {
            std::hint::black_box(&captured);
        }),
    )
    .unwrap();
    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(20)),
        Err(TimerError::OwnershipInvariant)
    ));
    assert!(dropped.get());

    let timer = identity("scheduler-failure-capture");
    let dropped = Rc::new(Cell::new(false));
    let captured = RemovalCapture {
        timer: timer.clone(),
        dropped: Rc::clone(&dropped),
    };
    let _registration = register_watchdog(
        timer,
        TimerCadence::from_nanos(5).unwrap(),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(u64::MAX),
        },
        stopping_watchdog(move || {
            std::hint::black_box(&captured);
        }),
    )
    .unwrap();
    set_time(u64::MAX);
    assert!(run_next_due());
    assert!(dropped.get());
    assert_eq!(timer_inventory().unwrap().timers(), []);
}

struct CancelContextOnDrop(Rc<RefCell<Option<OnceContext>>>);

impl Drop for CancelContextOnDrop {
    fn drop(&mut self) {
        self.0.borrow().as_ref().unwrap().cancel().unwrap();
    }
}

#[test]
fn repeated_reconciliation_releases_unused_factory_before_applying_control() {
    let _fixture = setup();
    let timer = identity("unused-factory-ordering");
    let gate = SuspendedWork::default();
    let context = Rc::new(RefCell::new(None));
    let callback_context = Rc::clone(&context);
    let callback_gate = gate.clone();
    let mut slot = Some(
        register_once(
            timer.clone(),
            TimerStart::Retained {
                initial: Some(TimerSchedule::At(10)),
            },
            move |context| {
                *callback_context.borrow_mut() = Some(context);
                let gate = callback_gate.clone();
                async move {
                    gate.wait().await;
                    OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
                }
            },
        )
        .unwrap(),
    );
    assert!(run_next_due());
    let captured = CancelContextOnDrop(context);
    reconcile_once(
        &mut slot,
        &timer,
        Some(TimerSchedule::At(20)),
        stopping_once(move || {
            std::hint::black_box(&captured);
        }),
    )
    .unwrap();
    gate.resume();
    assert!(run_next_due());
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().next_deadline_ns(),
        Some(20)
    );
    assert_eq!(timer_count(), 1);
    slot.take().unwrap().unregister().unwrap();
}

enum ReconciliationSlot {
    Once(Option<OnceRegistration>),
    AfterCompletion(Option<AfterCompletionRegistration>),
    Watchdog(Option<WatchdogRegistration>),
}

impl ReconciliationSlot {
    const fn new(policy: TestPolicy) -> Self {
        match policy {
            TestPolicy::Once => Self::Once(None),
            TestPolicy::AfterCompletion => Self::AfterCompletion(None),
            TestPolicy::Watchdog => Self::Watchdog(None),
        }
    }

    fn claim(&self) -> Option<&RegistrationClaim> {
        match self {
            Self::Once(slot) => slot.as_ref().map(|registration| &registration.claim),
            Self::AfterCompletion(slot) => slot.as_ref().map(|registration| &registration.claim),
            Self::Watchdog(slot) => slot.as_ref().map(|registration| &registration.claim),
        }
    }

    fn reconcile(
        &mut self,
        timer: &TimerIdentity,
        cadence: TimerCadence,
        scheduled: bool,
        captured: Option<RemovalCapture>,
        calls: Rc<Cell<u64>>,
    ) -> Result<(), TimerError> {
        let record = move || {
            std::hint::black_box(&captured);
            calls.set(calls.get() + 1);
        };
        match self {
            Self::Once(slot) => reconcile_once(
                slot,
                timer,
                scheduled.then_some(TimerSchedule::After(Duration::from_nanos(
                    cadence.as_nanos(),
                ))),
                stopping_once(record),
            ),
            Self::AfterCompletion(slot) => reconcile_after_completion(
                slot,
                timer,
                cadence,
                if scheduled {
                    TimerReconcileState::Scheduled
                } else {
                    TimerReconcileState::Inactive
                },
                stopping_after_completion(record),
            ),
            Self::Watchdog(slot) => reconcile_watchdog(
                slot,
                timer,
                cadence,
                if scheduled {
                    WatchdogReconcileState::Scheduled
                } else {
                    WatchdogReconcileState::Inactive
                },
                stopping_watchdog(record),
            ),
        }
    }
}

#[test]
fn failed_first_reconciliation_retires_all_policies_before_retry() {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Rejection {
        DeadlineOverflow,
        ProviderBinding,
    }

    for policy in TestPolicy::ALL {
        for rejection in [Rejection::DeadlineOverflow, Rejection::ProviderBinding] {
            let _fixture = setup();
            let timer = identity("first-reconciliation");
            let dropped = Rc::new(Cell::new(false));
            let captured = RemovalCapture {
                timer: timer.clone(),
                dropped: Rc::clone(&dropped),
            };
            let failed_calls = Rc::new(Cell::new(0));
            let mut slot = ReconciliationSlot::new(policy);
            let cadence = TimerCadence::from_nanos(if rejection == Rejection::DeadlineOverflow {
                u64::MAX
            } else {
                5
            })
            .unwrap();
            if rejection == Rejection::ProviderBinding {
                inject_provider_install_fault();
            }
            let result = slot.reconcile(
                &timer,
                cadence,
                true,
                Some(captured),
                Rc::clone(&failed_calls),
            );
            assert!(matches!(
                (rejection, result),
                (
                    Rejection::DeadlineOverflow,
                    Err(TimerError::Schedule(ScheduleError::DeadlineOverflow))
                ) | (
                    Rejection::ProviderBinding,
                    Err(TimerError::OwnershipInvariant)
                )
            ));
            assert!(slot.claim().is_none());
            assert!(dropped.get());
            assert_eq!(timer_inventory().unwrap().timers(), []);
            assert_eq!(timer_count(), 0);
            assert!(!run_next_due());

            let retry_calls = Rc::new(Cell::new(0));
            slot.reconcile(
                &timer,
                TimerCadence::from_nanos(5).unwrap(),
                true,
                None,
                Rc::clone(&retry_calls),
            )
            .unwrap();
            assert!(has_armed_wakeup_claim(slot.claim().unwrap()).unwrap());
            assert_eq!(timer_count(), 1);
            set_time(15);
            while run_next_due() {}
            assert_eq!(retry_calls.get(), 1);
            assert_eq!(failed_calls.get(), 0);
            assert_eq!(timer_count(), 0);
            unregister_claim(slot.claim().unwrap()).unwrap();
        }
    }
}

#[test]
fn first_once_reconciliation_rejects_unencodable_delay_without_retaining_authority() {
    let _fixture = setup();
    let timer = identity("unencodable-reconciliation");
    let dropped = Rc::new(Cell::new(false));
    let captured = RemovalCapture {
        timer: timer.clone(),
        dropped: Rc::clone(&dropped),
    };
    let mut slot = None;
    let result = reconcile_once(
        &mut slot,
        &timer,
        Some(TimerSchedule::After(Duration::MAX)),
        stopping_once(move || {
            std::hint::black_box(&captured);
        }),
    );
    assert!(matches!(
        result,
        Err(TimerError::Schedule(ScheduleError::DelayOutOfRange))
    ));
    assert!(slot.is_none());
    assert!(dropped.get());
    assert_eq!(timer_inventory().unwrap().timers(), []);
    assert_eq!(timer_count(), 0);
}

#[test]
fn later_reconciliation_deadline_error_preserves_existing_inactive_authority() {
    for policy in TestPolicy::ALL {
        let _fixture = setup();
        let timer = identity("later-reconciliation-deadline");
        let cadence = TimerCadence::from_nanos(u64::MAX).unwrap();
        let dropped = Rc::new(Cell::new(false));
        let captured = RemovalCapture {
            timer: timer.clone(),
            dropped: Rc::clone(&dropped),
        };
        let mut slot = ReconciliationSlot::new(policy);
        slot.reconcile(
            &timer,
            cadence,
            false,
            Some(captured),
            Rc::new(Cell::new(0)),
        )
        .unwrap();
        let before = timer_snapshot(&timer).unwrap();
        let result = slot.reconcile(&timer, cadence, true, None, Rc::new(Cell::new(0)));
        assert!(matches!(
            result,
            Err(TimerError::Schedule(ScheduleError::DeadlineOverflow))
        ));
        assert!(slot.claim().is_some());
        assert_eq!(timer_snapshot(&timer).unwrap(), before);
        assert!(!dropped.get());
        assert_eq!(timer_count(), 0);
        unregister_claim(slot.claim().unwrap()).unwrap();
        assert!(dropped.get());
    }
}

#[test]
fn later_reconciliation_provider_error_retains_original_callback_for_retry() {
    for policy in TestPolicy::ALL {
        let _fixture = setup();
        let timer = identity("later-reconciliation-provider");
        let cadence = TimerCadence::from_nanos(5).unwrap();
        let dropped = Rc::new(Cell::new(false));
        let captured = RemovalCapture {
            timer: timer.clone(),
            dropped: Rc::clone(&dropped),
        };
        let original_calls = Rc::new(Cell::new(0));
        let ignored_calls = Rc::new(Cell::new(0));
        let mut slot = ReconciliationSlot::new(policy);
        slot.reconcile(
            &timer,
            cadence,
            false,
            Some(captured),
            Rc::clone(&original_calls),
        )
        .unwrap();
        let original_id = timer_snapshot(&timer).unwrap().unwrap().registration_id();
        inject_provider_install_fault();
        let result = slot.reconcile(&timer, cadence, true, None, Rc::clone(&ignored_calls));
        assert!(matches!(result, Err(TimerError::OwnershipInvariant)));
        assert!(slot.claim().is_some());
        assert!(!dropped.get());
        let failed = timer_snapshot(&timer).unwrap().unwrap();
        assert_eq!(failed.registration_id(), original_id);
        assert_eq!(
            failed.state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::ControlFailure(TimerControlFailure::ProviderBindingFailed),
            }
        );
        assert_eq!(timer_count(), 0);
        slot.reconcile(&timer, cadence, true, None, Rc::clone(&ignored_calls))
            .unwrap();
        set_time(15);
        while run_next_due() {}
        assert_eq!(original_calls.get(), 1);
        assert_eq!(ignored_calls.get(), 0);
        unregister_claim(slot.claim().unwrap()).unwrap();
        assert!(dropped.get());
    }
}

fn initial_registration_with_capture(
    policy: TestPolicy,
    timer: TimerIdentity,
    start: TimerStart,
    captured: RemovalCapture,
) -> Result<RegistrationClaim, TimerError> {
    let work = move || {
        std::hint::black_box(&captured);
    };
    match policy {
        TestPolicy::Once => {
            register_once(timer, start, stopping_once(work)).map(|registration| registration.claim)
        }
        TestPolicy::AfterCompletion => register_after_completion(
            timer,
            TimerCadence::from_nanos(5).unwrap(),
            start,
            stopping_after_completion(work),
        )
        .map(|registration| registration.claim),
        TestPolicy::Watchdog => register_watchdog(
            timer,
            TimerCadence::from_nanos(5).unwrap(),
            start,
            stopping_watchdog(work),
        )
        .map(|registration| registration.claim),
    }
}

#[test]
fn initial_registration_confirms_exact_wakeup_before_returning_authority() {
    for policy in TestPolicy::ALL {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            let _fixture = setup();
            let timer = identity("initial-schedule");
            let dropped = Rc::new(Cell::new(false));
            let captured = RemovalCapture {
                timer: timer.clone(),
                dropped: Rc::clone(&dropped),
            };
            let claim = initial_registration_with_capture(
                policy,
                timer.clone(),
                scheduled_start(lifetime, TimerSchedule::At(20)),
                captured,
            )
            .unwrap();
            assert!(has_armed_wakeup_claim(&claim).unwrap());
            assert_eq!(timer_count(), 1);
            let snapshot = timer_snapshot(&timer).unwrap().unwrap();
            assert_eq!(snapshot.next_deadline_ns(), Some(20));
            assert_eq!(
                snapshot
                    .observability()
                    .counters()
                    .schedule_requests()
                    .exact_total()
                    .unwrap(),
                1
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
            assert!(!run_next_due());
            unregister_claim(&claim).unwrap();
            assert!(dropped.get());
        }
    }
}

#[test]
fn rejected_initial_schedule_allocates_no_claim_and_releases_captures() {
    for policy in TestPolicy::ALL {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            for initial in [
                TimerSchedule::After(Duration::MAX),
                TimerSchedule::After(Duration::from_nanos(u64::MAX)),
            ] {
                let _fixture = setup();
                let timer = identity("rejected-initial-schedule");
                let dropped = Rc::new(Cell::new(false));
                let captured = RemovalCapture {
                    timer: timer.clone(),
                    dropped: Rc::clone(&dropped),
                };
                let result = initial_registration_with_capture(
                    policy,
                    timer,
                    scheduled_start(lifetime, initial),
                    captured,
                );
                match initial {
                    TimerSchedule::After(Duration::MAX) => assert!(matches!(
                        result,
                        Err(TimerError::Schedule(ScheduleError::DelayOutOfRange))
                    )),
                    _ => assert!(matches!(
                        result,
                        Err(TimerError::Schedule(ScheduleError::DeadlineOverflow))
                    )),
                }
                assert!(dropped.get());
                assert_eq!(timer_inventory().unwrap().timers(), []);
                assert_eq!(timer_count(), 0);
            }
        }
    }
}

#[test]
fn initial_provider_failures_retire_all_policies_and_lifetimes_before_capture_drop() {
    for policy in TestPolicy::ALL {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            let _fixture = setup();
            let timer = identity("initial-provider-failure");
            let dropped = Rc::new(Cell::new(false));
            let captured = RemovalCapture {
                timer: timer.clone(),
                dropped: Rc::clone(&dropped),
            };
            inject_provider_install_fault();
            let result = initial_registration_with_capture(
                policy,
                timer,
                scheduled_start(lifetime, TimerSchedule::At(20)),
                captured,
            );
            assert!(matches!(result, Err(TimerError::OwnershipInvariant)));
            assert!(dropped.get());
            assert_eq!(timer_inventory().unwrap().timers(), []);
            assert_eq!(timer_count(), 0);
            assert!(!run_next_due());
        }
    }
}

#[test]
fn backend_admission_failure_retires_initial_claim_before_capture_drop() {
    for policy in TestPolicy::ALL {
        for error in [
            platform::AdmissionError::Capacity,
            platform::AdmissionError::IdentityExhausted,
        ] {
            let _fixture = setup();
            let timer = identity("backend-admission-failure");
            let dropped = Rc::new(Cell::new(false));
            let captured = RemovalCapture {
                timer: timer.clone(),
                dropped: Rc::clone(&dropped),
            };
            platform::ADMISSION_FAULT.set(Some((0, error)));
            let result = initial_registration_with_capture(
                policy,
                timer,
                TimerStart::RemoveWhenStopped {
                    initial: TimerSchedule::At(20),
                },
                captured,
            );
            assert!(matches!(
                (error, result),
                (
                    platform::AdmissionError::Capacity,
                    Err(TimerError::BackendCapacity)
                ) | (
                    platform::AdmissionError::IdentityExhausted,
                    Err(TimerError::BackendIdentityExhausted)
                )
            ));
            assert!(dropped.get());
            assert_eq!(timer_inventory().unwrap().timers(), []);
            assert_eq!(timer_count(), 0);
        }
    }
}

#[test]
fn initial_capacity_rejection_preserves_existing_declarations_and_releases_factory() {
    let _fixture = setup();
    let mut registrations = Vec::new();
    for index in 0..crate::MAX_TIMER_REGISTRATIONS {
        registrations.push(
            register_once(
                identity(&format!("initial-capacity-{index}")),
                TimerStart::Retained { initial: None },
                |_| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
            )
            .unwrap(),
        );
    }
    let before = timer_inventory().unwrap();
    let captured = Rc::new(());
    let weak = Rc::downgrade(&captured);
    let rejected = register_once(
        identity("initial-capacity-rejected"),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::At(20),
        },
        stopping_once(move || {
            std::hint::black_box(&captured);
        }),
    );
    assert!(matches!(
        rejected,
        Err(TimerError::Register(RegisterError::CapacityExceeded { .. }))
    ));
    assert!(weak.upgrade().is_none());
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 0);
    for registration in registrations {
        registration.unregister().unwrap();
    }
}

mod sequences;
