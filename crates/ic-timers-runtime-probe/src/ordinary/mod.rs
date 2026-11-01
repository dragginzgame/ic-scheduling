//! Real await/ingress interleaving and ordinary abandonment qualification.

use ic_timers::{
    AfterCompletionContext, AfterCompletionDecision, AfterCompletionRegistration,
    AfterCompletionRunResult, InactiveReason, OnceContext, OnceDecision, OnceRegistration,
    OnceRunResult, TimerCadence, TimerCompletion, TimerError, TimerIdentity, TimerReconcileState,
    TimerRegistrationStatus, TimerRuntimeStateSnapshot, TimerSchedule, TimerStart,
    WatchdogDecision, WatchdogReconcileState, WatchdogRunResult, reconcile_after_completion,
    reconcile_once, reconcile_watchdog, register_after_completion, register_once,
    register_watchdog, timer_inventory, timer_snapshot,
};
use ic_timers_test_protocol::{
    OrdinaryObservation,
    rpc::{OrdinaryPolicy, ProbeLifetime, ProbePolicy},
};
use std::{
    cell::{Cell, RefCell},
    future::{Ready, ready},
    rc::Rc,
};

enum Registration {
    Once(OnceRegistration),
    AfterCompletion(AfterCompletionRegistration),
}

enum WorkContext {
    Once(OnceContext),
    AfterCompletion(AfterCompletionContext),
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TrapPoint {
    None,
    BeforeAwait,
    AfterAwait,
}

struct CapturedState;

impl Drop for CapturedState {
    fn drop(&mut self) {
        if TRAP_CAPTURE_DROP.get() {
            ic_platform::cdk::api::trap("probe capture drop trap");
        }
        CAPTURE_REGISTRY_ACCESSIBLE.with(|accessible| {
            accessible.set(accessible.get() && timer_inventory().is_ok());
        });
        CAPTURES_DROPPED.with(|count| count.set(count.get().saturating_add(1)));
    }
}

// The failure/removal subjects retain the same capture until the declaration
// retires. Their callbacks complete immediately and never introduce an await.
fn once_callback(captured: CapturedState) -> impl FnMut(OnceContext) -> Ready<OnceRunResult> {
    move |_| {
        std::hint::black_box(&captured);
        ready(OnceRunResult::new(
            TimerCompletion::no_work(),
            OnceDecision::Stop,
        ))
    }
}

fn after_completion_callback(
    captured: CapturedState,
) -> impl FnMut(AfterCompletionContext) -> Ready<AfterCompletionRunResult> {
    move |_| {
        std::hint::black_box(&captured);
        ready(AfterCompletionRunResult::new(
            TimerCompletion::no_work(),
            AfterCompletionDecision::Stop,
        ))
    }
}

fn watchdog_callback(
    captured: CapturedState,
) -> impl FnMut(ic_timers::WatchdogContext) -> WatchdogRunResult {
    move |_| {
        std::hint::black_box(&captured);
        WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
    }
}

thread_local! {
    static TRAP_CAPTURE_DROP: Cell<bool> = const { Cell::new(false) };
    static REGISTRATION: RefCell<Option<Registration>> = const { RefCell::new(None) };
    static GATE_OPEN: Cell<bool> = const { Cell::new(false) };
    static GATE_WAITING: Cell<bool> = const { Cell::new(false) };
    static GATE_REPLIES: Cell<u64> = const { Cell::new(0) };
    static GATE_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
    static COMPLETIONS: Cell<u64> = const { Cell::new(0) };
    static COMPLETED_AT_NS: Cell<Option<u64>> = const { Cell::new(None) };
    static TRAP_POINT: Cell<TrapPoint> = const { Cell::new(TrapPoint::None) };
    static WORK_CONTEXT: RefCell<Option<WorkContext>> = const { RefCell::new(None) };
    static CAPTURES_DROPPED: Cell<u64> = const { Cell::new(0) };
    static CAPTURE_REGISTRY_ACCESSIBLE: Cell<bool> = const { Cell::new(true) };
}

// Deliberately violates the consumer destructor contract to exercise an actual
// dispatch response trap and its IC cleanup callback, without backend hooks.
#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::TrapOrdinaryCaptureDrop as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::TrapOrdinaryCaptureDrop as u64 }>"
)]
fn trap_ordinary_capture_drop(enabled: bool) {
    TRAP_CAPTURE_DROP.set(enabled);
}

fn identity() -> TimerIdentity {
    TimerIdentity::try_new("ic-timers", "await-probe", "ordinary").expect("fixed identity")
}

#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::StartOrdinary as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::StartOrdinary as u64 }>"
)]
fn start_ordinary(policy: OrdinaryPolicy, lifetime: ProbeLifetime) {
    let initial = TimerSchedule::At(ic_platform::cdk::api::time());
    let start = if lifetime == ProbeLifetime::RemoveWhenStopped {
        TimerStart::RemoveWhenStopped { initial }
    } else {
        TimerStart::Retained {
            initial: Some(initial),
        }
    };
    REGISTRATION.with_borrow_mut(|slot| {
        assert!(slot.is_none(), "one probe registration");
        let captured = Rc::new(CapturedState);
        let registration = if policy == OrdinaryPolicy::AfterCompletion {
            Registration::AfterCompletion(
                register_after_completion(
                    identity(),
                    TimerCadence::from_nanos(super::CADENCE_NS).expect("fixed cadence"),
                    start,
                    move |context| {
                        WORK_CONTEXT.with_borrow_mut(|slot| {
                            *slot = Some(WorkContext::AfterCompletion(context))
                        });
                        let captured = Rc::clone(&captured);
                        async move {
                            let completion = work().await;
                            std::hint::black_box(captured);
                            AfterCompletionRunResult::new(
                                completion,
                                AfterCompletionDecision::RecurAfterCompletion,
                            )
                        }
                    },
                )
                .expect("register after-completion"),
            )
        } else {
            Registration::Once(
                register_once(identity(), start, move |context| {
                    WORK_CONTEXT.with_borrow_mut(|slot| *slot = Some(WorkContext::Once(context)));
                    let captured = Rc::clone(&captured);
                    async move {
                        let completion = work().await;
                        std::hint::black_box(captured);
                        OnceRunResult::new(completion, OnceDecision::Stop)
                    }
                })
                .expect("register Once"),
            )
        };
        *slot = Some(registration);
    });
}

async fn work() -> TimerCompletion {
    if TRAP_POINT.with(Cell::get) == TrapPoint::BeforeAwait {
        ic_platform::cdk::api::trap("ordinary probe trap before await");
    }
    GATE_WAITING.with(|waiting| waiting.set(true));
    loop {
        // Every closed-gate reply is followed by another real call await. A bare
        // Pending future cannot keep the IC call context alive, and a protected
        // CDK task cannot resume in the separate ingress that opens the gate.
        let result =
            super::codec::call_gate(ic_timers_test_protocol::rpc::Method::OrdinaryGate).await;
        let open = match result {
            Ok(open) => open,
            Err(error) => {
                GATE_WAITING.with(|waiting| waiting.set(false));
                GATE_ERROR.with_borrow_mut(|slot| *slot = Some(error));
                return TimerCompletion::invariant_failure(0);
            }
        };
        GATE_REPLIES.with(|count| count.set(count.get().saturating_add(1)));
        if open {
            break;
        }
    }
    if TRAP_POINT.with(Cell::get) == TrapPoint::AfterAwait {
        ic_platform::cdk::api::trap("ordinary probe trap after await");
    }
    GATE_WAITING.with(|waiting| waiting.set(false));
    COMPLETIONS.with(|count| count.set(count.get() + 1));
    COMPLETED_AT_NS.with(|time| time.set(Some(ic_platform::cdk::api::time())));
    TimerCompletion::success(1)
}

#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::OrdinaryGate as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::OrdinaryGate as u64 }>"
)]
fn ordinary_gate() -> bool {
    assert_eq!(
        ic_platform::cdk::api::msg_caller(),
        ic_platform::cdk::api::canister_self(),
        "self-call gate"
    );
    GATE_OPEN.with(Cell::get)
}

#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ReleaseOrdinaryWork as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::ReleaseOrdinaryWork as u64 }>"
)]
fn release_ordinary_work() {
    GATE_OPEN.with(|open| open.set(true));
}

#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::TrapOrdinaryWork as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::TrapOrdinaryWork as u64 }>"
)]
fn trap_ordinary_work(before_await: bool) {
    TRAP_POINT.with(|point| {
        point.set(if before_await {
            TrapPoint::BeforeAwait
        } else {
            TrapPoint::AfterAwait
        })
    });
}

#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ClearOrdinaryTrap as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::ClearOrdinaryTrap as u64 }>"
)]
fn clear_ordinary_trap() {
    TRAP_POINT.with(|point| point.set(TrapPoint::None));
}

#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ExpiredOrdinaryContextRejected as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::ExpiredOrdinaryContextRejected as u64 }>"
)]
fn expired_ordinary_context_rejected() -> bool {
    WORK_CONTEXT.with_borrow(|slot| {
        let result = match slot.as_ref().expect("committed work context") {
            WorkContext::Once(context) => context.cancel(),
            WorkContext::AfterCompletion(context) => context.cancel(),
        };
        matches!(result, Err(TimerError::RegistrationExpired))
    })
}

#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::DiscardExpiredOrdinaryClaim as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::DiscardExpiredOrdinaryClaim as u64 }>"
)]
fn discard_expired_ordinary_claim() {
    REGISTRATION.with_borrow_mut(|slot| {
        let result = match slot.as_ref().expect("expired registration") {
            Registration::Once(timer) => timer.has_armed_wakeup(),
            Registration::AfterCompletion(timer) => timer.has_armed_wakeup(),
        };
        assert!(matches!(result, Err(TimerError::RegistrationExpired)));
        *slot = None;
    });
}

fn reconcile(registration: &Registration, desired: Option<TimerSchedule>) {
    let result = match registration {
        Registration::Once(timer) => timer.reconcile_schedule(desired),
        Registration::AfterCompletion(timer) => timer.reconcile_schedule(desired),
    };
    result.expect("ordinary reconciliation");
}

#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ReconcileOrdinaryAt as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::ReconcileOrdinaryAt as u64 }>"
)]
fn reconcile_ordinary_at(deadline_ns: u64) {
    REGISTRATION.with_borrow(|slot| {
        reconcile(
            slot.as_ref().expect("registered"),
            Some(TimerSchedule::At(deadline_ns)),
        )
    });
}

#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::CancelOrdinary as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::CancelOrdinary as u64 }>"
)]
fn cancel_ordinary() {
    REGISTRATION
        .with_borrow(|slot| match slot.as_ref().expect("registered") {
            Registration::Once(timer) => timer.cancel(),
            Registration::AfterCompletion(timer) => timer.cancel(),
        })
        .expect("explicit ordinary cancellation");
}

#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ReconcileOrdinaryInactive as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::ReconcileOrdinaryInactive as u64 }>"
)]
fn reconcile_ordinary_inactive() {
    REGISTRATION.with_borrow(|slot| reconcile(slot.as_ref().expect("registered"), None));
}

#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::UnregisterOrdinary as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::UnregisterOrdinary as u64 }>"
)]
fn unregister_ordinary() {
    let registration = REGISTRATION
        .with_borrow_mut(Option::take)
        .expect("registered");
    match registration {
        Registration::Once(timer) => timer.unregister(),
        Registration::AfterCompletion(timer) => timer.unregister(),
    }
    .expect("ordinary unregister");
}

#[ic_platform::cdk::query(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::OrdinaryObservation as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::OrdinaryObservation as u64 }>"
)]
fn ordinary_observation() -> OrdinaryObservation {
    let snapshot = timer_snapshot(&identity()).expect("ordinary snapshot");
    let armed = REGISTRATION.with_borrow(|slot| {
        let result = match slot.as_ref() {
            Some(Registration::Once(timer)) => timer.has_armed_wakeup(),
            Some(Registration::AfterCompletion(timer)) => timer.has_armed_wakeup(),
            None => return false,
        };
        match result {
            Ok(armed) => armed,
            Err(TimerError::RegistrationExpired) => false,
            Err(error) => panic!("ordinary ownership observation failed: {error}"),
        }
    });
    OrdinaryObservation {
        registration_id: snapshot.as_ref().map(|value| {
            let id = value.registration_id();
            (
                id.epoch().canister_version(),
                id.epoch().started_at_ns(),
                id.sequence(),
            )
        }),
        declared: snapshot.is_some(),
        running: snapshot
            .as_ref()
            .is_some_and(|value| value.registration_status() == TimerRegistrationStatus::Running),
        waiting: GATE_WAITING.with(Cell::get),
        gate_replies: GATE_REPLIES.with(Cell::get),
        gate_error: GATE_ERROR.with_borrow(Clone::clone),
        completed: COMPLETIONS.with(Cell::get),
        completed_at_ns: COMPLETED_AT_NS.with(Cell::get),
        next_deadline_ns: snapshot.as_ref().and_then(|value| value.next_deadline_ns()),
        work_started: snapshot
            .as_ref()
            .map(|value| {
                super::counter_observation(value.observability().counters().work_started())
            })
            .unwrap_or_default(),
        work_completed: snapshot
            .as_ref()
            .map(|value| {
                super::counter_observation(value.observability().counters().work_completed())
            })
            .unwrap_or_default(),
        abandoned: snapshot.as_ref().is_some_and(|value| {
            value.state()
                == TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::Abandoned,
                }
        }),
        stopped: snapshot.as_ref().is_some_and(|value| {
            value.state()
                == TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::Stopped,
                }
        }),
        cancelled: snapshot.as_ref().is_some_and(|value| {
            value.state()
                == TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::Cancelled,
                }
        }),
        cancellations: snapshot
            .as_ref()
            .map(|value| super::counter_observation(value.observability().counters().cancelled()))
            .unwrap_or_default(),
        invariant_failure: snapshot.as_ref().is_some_and(|value| {
            value.state()
                == TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::InvariantFailure,
                }
        }),
        armed,
        unacknowledged: snapshot
            .as_ref()
            .map(|value| {
                super::counter_observation(value.observability().counters().unacknowledged())
            })
            .unwrap_or_default(),
        instruction_samples: snapshot.as_ref().map_or(0, |value| {
            value
                .observability()
                .performance()
                .work_instructions()
                .aggregate()
                .samples()
        }),
        captures_dropped: CAPTURES_DROPPED.with(Cell::get),
        capture_registry_accessible: CAPTURE_REGISTRY_ACCESSIBLE.with(Cell::get),
        inventory_len: timer_inventory()
            .expect("ordinary inventory")
            .timers()
            .len() as u64,
    }
}

/// Remove armed authority in one message before any consumer work can start.
#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::RemoveArmedCallback as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::RemoveArmedCallback as u64 }>"
)]
fn remove_armed_callback(policy: ProbePolicy) {
    let deadline = ic_platform::cdk::api::time()
        .checked_add(super::CADENCE_NS)
        .expect("fixture deadline");
    let captured = CapturedState;
    match policy {
        ProbePolicy::Once => {
            let timer = register_once(
                identity(),
                TimerStart::Retained {
                    initial: Some(TimerSchedule::At(deadline)),
                },
                once_callback(captured),
            )
            .expect("register Once");
            timer.unregister().expect("unregister Once");
        }
        ProbePolicy::AfterCompletion => {
            let timer = register_after_completion(
                identity(),
                TimerCadence::from_nanos(super::CADENCE_NS).expect("cadence"),
                TimerStart::RemoveWhenStopped {
                    initial: TimerSchedule::At(deadline),
                },
                after_completion_callback(captured),
            )
            .expect("register after-completion");
            timer.cancel().expect("cancel transient");
        }
        ProbePolicy::Watchdog => {
            let timer = register_watchdog(
                identity(),
                TimerCadence::from_nanos(super::CADENCE_NS).expect("cadence"),
                TimerStart::Retained {
                    initial: Some(TimerSchedule::At(deadline)),
                },
                watchdog_callback(captured),
            )
            .expect("register Watchdog");
            timer.unregister().expect("unregister Watchdog");
        }
    }
}

/// A rejected initial request must not leave callback authority in the IC heap.
#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::RejectInitialSchedule as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::RejectInitialSchedule as u64 }>"
)]
fn reject_initial_schedule(policy: ProbePolicy, lifetime: ProbeLifetime) -> (bool, u64, bool) {
    let before = timer_inventory().expect("initial inventory");
    let dropped_before = CAPTURES_DROPPED.with(Cell::get);
    let initial = TimerSchedule::After(std::time::Duration::MAX);
    let start = if lifetime == ProbeLifetime::RemoveWhenStopped {
        TimerStart::RemoveWhenStopped { initial }
    } else {
        TimerStart::Retained {
            initial: Some(initial),
        }
    };
    let captured = CapturedState;
    let result = match policy {
        ProbePolicy::Once => register_once(identity(), start, once_callback(captured)).map(|_| ()),
        ProbePolicy::AfterCompletion => register_after_completion(
            identity(),
            TimerCadence::from_nanos(super::CADENCE_NS).expect("cadence"),
            start,
            after_completion_callback(captured),
        )
        .map(|_| ()),
        ProbePolicy::Watchdog => register_watchdog(
            identity(),
            TimerCadence::from_nanos(super::CADENCE_NS).expect("cadence"),
            start,
            watchdog_callback(captured),
        )
        .map(|_| ()),
    };
    let rejected = matches!(
        result,
        Err(TimerError::Schedule(
            ic_timers::ScheduleError::DelayOutOfRange
        ))
    );
    assert!(rejected, "typed initial rejection");
    (
        timer_inventory().expect("rejected inventory") == before,
        CAPTURES_DROPPED.with(Cell::get) - dropped_before,
        CAPTURE_REGISTRY_ACCESSIBLE.with(Cell::get),
    )
}

/// A rejected first lifecycle reconciliation must leave its owner slot empty.
#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::RejectInitialReconciliation as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::RejectInitialReconciliation as u64 }>"
)]
fn reject_initial_reconciliation(policy: ProbePolicy) -> (bool, bool, u64, bool) {
    let before = timer_inventory().expect("initial inventory");
    let dropped_before = CAPTURES_DROPPED.with(Cell::get);
    let captured = CapturedState;
    let cadence = TimerCadence::from_nanos(u64::MAX).expect("positive cadence");
    let (result, empty) = match policy {
        ProbePolicy::Once => {
            let mut slot = None;
            let result = reconcile_once(
                &mut slot,
                &identity(),
                Some(TimerSchedule::After(std::time::Duration::MAX)),
                once_callback(captured),
            );
            (result, slot.is_none())
        }
        ProbePolicy::AfterCompletion => {
            let mut slot = None;
            let result = reconcile_after_completion(
                &mut slot,
                &identity(),
                cadence,
                TimerReconcileState::Scheduled,
                after_completion_callback(captured),
            );
            (result, slot.is_none())
        }
        ProbePolicy::Watchdog => {
            let mut slot = None;
            let result = reconcile_watchdog(
                &mut slot,
                &identity(),
                cadence,
                WatchdogReconcileState::Scheduled,
                watchdog_callback(captured),
            );
            (result, slot.is_none())
        }
    };
    let rejected = matches!(
        (policy, result),
        (
            ProbePolicy::Once,
            Err(TimerError::Schedule(
                ic_timers::ScheduleError::DelayOutOfRange
            ))
        ) | (
            ProbePolicy::AfterCompletion | ProbePolicy::Watchdog,
            Err(TimerError::Schedule(
                ic_timers::ScheduleError::DeadlineOverflow
            ))
        )
    );
    (
        empty && rejected,
        timer_inventory().expect("rejected inventory") == before,
        CAPTURES_DROPPED.with(Cell::get) - dropped_before,
        CAPTURE_REGISTRY_ACCESSIBLE.with(Cell::get),
    )
}
