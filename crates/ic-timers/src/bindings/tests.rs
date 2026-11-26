use super::*;
use std::{
    cell::Cell,
    future::pending,
    task::{Context, Poll, Waker},
};

struct ReenterOnDrop {
    handle: Rc<Handle>,
    dropped: Rc<Cell<bool>>,
}
impl Drop for ReenterOnDrop {
    fn drop(&mut self) {
        self.handle.set(Demand::Inactive).unwrap();
        self.dropped.set(true);
    }
}

#[test]
fn binding_retains_original_and_rejected_captures_drop_outside_borrow() {
    let runtime = Bindings::new();
    let calls = Rc::new(Cell::new(0));
    let counted = Rc::clone(&calls);
    let handle = Rc::new(
        runtime
            .bind(
                ["owner", "group", "name"],
                crate::Lifetime::Retained,
                move || {
                    counted.set(counted.get() + 1);
                    ready(Demand::Inactive)
                },
            )
            .unwrap(),
    );
    let dropped = Rc::new(Cell::new(false));
    let guard = ReenterOnDrop {
        handle: Rc::clone(&handle),
        dropped: Rc::clone(&dropped),
    };
    let rejected = runtime.bind(
        ["owner", "group", "name"],
        crate::Lifetime::Retained,
        move || {
            let _keep = &guard;
            ready(Demand::Inactive)
        },
    );
    assert!(matches!(rejected, Err(Error::DuplicateIdentity)));
    assert!(dropped.get());
    handle.set(Demand::At(0)).unwrap();
    drop(handle);
    assert_eq!(calls.get(), 0);
    let dispatch = due(&runtime, 0).unwrap();
    let (accepted, mut future) = runtime.start(&dispatch).unwrap().unwrap();
    assert_eq!(calls.get(), 1);
    assert!(runtime.start(&dispatch).unwrap().is_none());
    assert_eq!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Ready((Demand::Inactive, crate::Observation::Success))
    );
    assert!(
        accepted
            .complete(Demand::Inactive, crate::Observation::Success)
            .unwrap()
    );
}

#[test]
fn consumer_factory_poll_and_drop_can_reenter() {
    let runtime = Bindings::new();
    let slot: Rc<RefCell<Option<Rc<Handle>>>> = Rc::new(RefCell::new(None));
    let captured = Rc::clone(&slot);
    let dropped = Rc::new(Cell::new(false));
    let drop_flag = Rc::clone(&dropped);
    let handle = Rc::new(
        runtime
            .bind(["o", "g", "n"], crate::Lifetime::Retained, move || {
                let handle = Rc::clone(captured.borrow().as_ref().unwrap());
                handle.set(Demand::At(10)).unwrap();
                let guard = ReenterOnDrop {
                    handle: Rc::clone(&handle),
                    dropped: Rc::clone(&drop_flag),
                };
                async move {
                    handle.set(Demand::At(20)).unwrap();
                    pending::<()>().await;
                    drop(guard);
                    (Demand::Inactive, crate::Observation::Success)
                }
            })
            .unwrap(),
    );
    *slot.borrow_mut() = Some(Rc::clone(&handle));
    handle.set(Demand::At(0)).unwrap();
    let dispatch = due(&runtime, 0).unwrap();
    let (accepted, mut future) = runtime.start(&dispatch).unwrap().unwrap();
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert!(due(&runtime, u64::MAX).is_none());
    drop(future);
    assert!(dropped.get());
    assert!(matches!(handle.state().unwrap(), State::Running { .. }));
    // Native fixture supplies termination explicitly; Drop itself did not.
    assert!(accepted.abandon().unwrap());
    let recovery = handle.recovery().unwrap().unwrap();
    handle.recover(&recovery, Demand::Inactive).unwrap();
    assert_eq!(handle.state().unwrap(), State::Inactive);
}

#[test]
fn authority_is_scoped_to_runtime_and_declaration() {
    let first = Bindings::new();
    let second = Bindings::new();
    let a = first
        .bind(["o", "g", "a"], crate::Lifetime::Retained, || {
            ready(Demand::Inactive)
        })
        .unwrap();
    let b = first
        .bind(["o", "g", "b"], crate::Lifetime::Retained, || {
            ready(Demand::Inactive)
        })
        .unwrap();
    a.set(Demand::At(0)).unwrap();
    b.set(Demand::At(0)).unwrap();
    let ticket = due(&first, 0).unwrap();
    assert_eq!(
        second.rejected(&ticket, trap().outcome(), 0),
        Err(Error::StaleAuthority)
    );
    assert!(matches!(second.start(&ticket), Err(Error::StaleAuthority)));
    let (accepted, future) = first.start(&ticket).unwrap().unwrap();
    assert!(accepted.is_running().unwrap());
    drop(future);
    assert!(accepted.abandon().unwrap());
    assert!(!accepted.is_running().unwrap());
    let key = a.recovery().unwrap().unwrap();
    assert_eq!(b.recover(&key, Demand::At(0)), Err(Error::StaleAuthority));
    drop(first);
    assert_eq!(a.set(Demand::At(0)), Err(Error::StaleAuthority));
    assert_eq!(
        accepted.complete(Demand::Inactive, crate::Observation::Success),
        Err(Error::StaleAuthority)
    );
}

#[test]
fn bounded_admission_and_rotating_selection() {
    let runtime = Bindings::new();
    for invalid in ["".to_owned(), "é".repeat(33)] {
        assert!(matches!(
            runtime.bind([&invalid, "g", "n"], crate::Lifetime::Retained, || ready(
                Demand::Inactive
            )),
            Err(Error::InvalidIdentity)
        ));
    }
    for index in 0..CAPACITY {
        let handle = runtime
            .bind(
                ["o", "g", &index.to_string()],
                crate::Lifetime::Retained,
                || ready(Demand::Inactive),
            )
            .unwrap();
        handle.set(Demand::At(0)).unwrap();
    }
    assert!(matches!(
        runtime.bind(["o", "g", "overflow"], crate::Lifetime::Retained, || ready(
            Demand::Inactive
        )),
        Err(Error::Capacity)
    ));
    for expected in 0..CAPACITY {
        assert_eq!(due(&runtime, 0).unwrap().slot, expected);
    }
    assert_eq!(due(&runtime, 0).unwrap().slot, 0);
}

#[test]
fn submitted_ticket_does_not_hide_new_due_work_or_a_replacement() {
    let runtime = Bindings::new();
    let first = runtime
        .bind(["o", "g", "first"], crate::Lifetime::Retained, || {
            ready(Demand::Inactive)
        })
        .unwrap();
    let second = runtime
        .bind(["o", "g", "second"], crate::Lifetime::Retained, || {
            ready(Demand::Inactive)
        })
        .unwrap();
    first.set(Demand::At(0)).unwrap();
    second.set(Demand::At(10)).unwrap();
    let submitted = due(&runtime, 0).unwrap();
    assert_eq!(
        runtime
            .wake(&std::iter::once(&Scheduled::Once(submitted.clone())))
            .0,
        Some(10)
    );
    first.set(Demand::Inactive).unwrap();
    first.set(Demand::At(0)).unwrap();
    assert_eq!(
        runtime
            .wake(&std::iter::once(&Scheduled::Once(submitted.clone())))
            .0,
        Some(0)
    );
    let replacement = due(&runtime, 0).unwrap();
    assert!(!replacement.same(&submitted));
    assert_eq!(
        runtime.rejected(&submitted, trap().outcome(), 0).unwrap(),
        None
    );
    second.set(Demand::Inactive).unwrap();
    assert_eq!(
        runtime
            .wake(&std::iter::once(&Scheduled::Once(replacement)))
            .0,
        None
    );
    // A ticket from another runtime cannot mask this runtime's deadline.
    let other = Bindings::new();
    other
        .bind(["o", "g", "other"], crate::Lifetime::Retained, || {
            ready(Demand::Inactive)
        })
        .unwrap()
        .set(Demand::At(0))
        .unwrap();
    assert_eq!(
        runtime
            .wake(&std::iter::once(&Scheduled::Once(due(&other, 0).unwrap())))
            .0,
        Some(0)
    );
    let active = due(&runtime, 0).unwrap();
    assert_eq!(
        runtime.rejected(&active, trap().outcome(), 0).unwrap(),
        Some(Rejected::Abandoned)
    );
    assert_eq!(
        runtime.rejected(&active, trap().outcome(), 0).unwrap(),
        None
    );
    assert!(first.recovery().unwrap().is_some());
}

fn due(bindings: &Bindings, now: u64) -> Option<Dispatch> {
    match bindings.select(now) {
        Some(Scheduled::Once(dispatch)) => Some(dispatch),
        None => None,
        Some(Scheduled::Watchdog(_)) => panic!("Once-only fixture selected watchdog work"),
    }
}

#[test]
fn removal_reuses_capacity_without_reviving_old_handles_or_deliveries() {
    let runtime = Bindings::new();
    let old = runtime
        .bind(["o", "g", "reuse"], crate::Lifetime::Retained, || {
            ready(Demand::Inactive)
        })
        .unwrap();
    old.set(Demand::At(0)).unwrap();
    let dispatch = due(&runtime, 0).unwrap();
    old.remove().unwrap();
    let replacement = runtime
        .bind(["o", "g", "reuse"], crate::Lifetime::Retained, || {
            ready(Demand::Inactive)
        })
        .unwrap();
    replacement.set(Demand::At(0)).unwrap();
    assert_eq!(old.slot, replacement.slot);
    assert_ne!(old.incarnation, replacement.incarnation);
    assert_eq!(old.set(Demand::Inactive), Err(Error::StaleAuthority));
    assert_eq!(old.remove(), Err(Error::StaleAuthority));
    assert!(runtime.start(&dispatch).unwrap().is_none());
    assert_eq!(replacement.state(), Ok(State::Armed(0)));
    replacement.remove().unwrap();
    for _ in 0..CAPACITY * 3 {
        runtime
            .bind(["o", "g", "reuse"], crate::Lifetime::Retained, || {
                ready(Demand::Inactive)
            })
            .unwrap()
            .remove()
            .unwrap();
    }
    assert_eq!(runtime.0.borrow().entries.len(), 1);
}

#[test]
fn removal_refuses_accepted_and_uncertain_work_and_expires_recovery() {
    let runtime = Bindings::new();
    let handle = runtime
        .bind(["o", "g", "n"], crate::Lifetime::Retained, || {
            ready(Demand::Inactive)
        })
        .unwrap();
    handle.set(Demand::At(0)).unwrap();
    let (accepted, _) = runtime.start(&due(&runtime, 0).unwrap()).unwrap().unwrap();
    assert_eq!(handle.remove(), Err(Error::Busy));
    accepted.abandon().unwrap();
    assert_eq!(
        handle.remove(),
        Err(Error::Control(ControlError::RecoveryRequired))
    );
    let recovery = handle.recovery().unwrap().unwrap();
    handle.recover(&recovery, Demand::Inactive).unwrap();
    handle.remove().unwrap();
    let replacement = runtime
        .bind(["o", "g", "n"], crate::Lifetime::Retained, || {
            ready(Demand::Inactive)
        })
        .unwrap();
    assert_eq!(
        replacement.recover(&recovery, Demand::At(1)),
        Err(Error::StaleAuthority)
    );
    assert!(
        !accepted
            .complete(Demand::At(99), crate::Observation::Success)
            .unwrap()
    );
    assert_eq!(replacement.state(), Ok(State::Inactive));
}

#[test]
fn transient_stops_release_captures_outside_borrow_and_respect_owner_precedence() {
    let runtime = Bindings::new();
    let target = Rc::new(
        runtime
            .bind(["o", "g", "target"], crate::Lifetime::Retained, || {
                ready(Demand::Inactive)
            })
            .unwrap(),
    );
    let dropped = Rc::new(Cell::new(false));
    let guard = ReenterOnDrop {
        handle: target,
        dropped: Rc::clone(&dropped),
    };
    let handle = runtime
        .bind(
            ["o", "g", "transient"],
            crate::Lifetime::RemoveWhenStopped,
            move || {
                let _keep = &guard;
                ready(Demand::Inactive)
            },
        )
        .unwrap();
    handle.set(Demand::At(0)).unwrap();
    let (accepted, _) = runtime.start(&due(&runtime, 0).unwrap()).unwrap().unwrap();
    handle.set(Demand::At(50)).unwrap();
    accepted
        .complete(Demand::Inactive, crate::Observation::Success)
        .unwrap();
    assert_eq!(handle.state(), Ok(State::Armed(50)));
    assert!(!dropped.get());
    handle.set(Demand::Inactive).unwrap();
    assert!(dropped.get());
    assert_eq!(handle.state(), Err(Error::StaleAuthority));
    let handle = runtime
        .bind(
            ["o", "g", "transient"],
            crate::Lifetime::RemoveWhenStopped,
            || ready(Demand::Inactive),
        )
        .unwrap();
    handle.set(Demand::At(0)).unwrap();
    let (accepted, _) = runtime.start(&due(&runtime, 0).unwrap()).unwrap().unwrap();
    accepted.abandon().unwrap();
    handle.set(Demand::Inactive).unwrap();
    assert!(
        handle.recovery().unwrap().is_some(),
        "stopping cannot erase uncertainty"
    );
    handle
        .recover(&handle.recovery().unwrap().unwrap(), Demand::Inactive)
        .unwrap();
    assert_eq!(handle.state(), Err(Error::StaleAuthority));
}

#[test]
fn transient_completion_removes_once_and_incarnations_never_wrap() {
    let runtime = Bindings::new();
    let handle = runtime
        .bind(
            ["o", "g", "once"],
            crate::Lifetime::RemoveWhenStopped,
            || ready(Demand::Inactive),
        )
        .unwrap();
    handle.set(Demand::At(0)).unwrap();
    let (accepted, _) = runtime.start(&due(&runtime, 0).unwrap()).unwrap().unwrap();
    assert!(
        accepted
            .complete(Demand::Inactive, crate::Observation::Success)
            .unwrap()
    );
    assert!(
        !accepted
            .complete(Demand::At(1), crate::Observation::Success)
            .unwrap()
    );
    assert_eq!(handle.state(), Err(Error::StaleAuthority));
    runtime.0.borrow_mut().next_incarnation = u64::MAX;
    assert!(matches!(
        runtime.bind(["o", "g", "new"], crate::Lifetime::Retained, || ready(
            Demand::Inactive
        )),
        Err(Error::Control(ControlError::Exhausted))
    ));
    assert!(runtime.0.borrow().entries[0].is_none());
}

fn ready(
    demand: crate::engine::Demand,
) -> std::future::Ready<(crate::engine::Demand, crate::Observation)> {
    std::future::ready((demand, crate::Observation::Success))
}

#[cfg(feature = "metrics")]
#[test]
fn inventory_is_sorted_inert_and_scoped_to_the_declaration_incarnation() {
    let runtime = Bindings::new();
    let once = runtime
        .bind(["z", "g", "once"], crate::Lifetime::Retained, || {
            ready(Demand::Inactive)
        })
        .unwrap();
    let watch = runtime
        .bind_watchdog(
            ["a", "g", "watch"],
            std::num::NonZeroU64::new(10).unwrap(),
            crate::Lifetime::Retained,
            || (crate::watchdog::Next::Continue, crate::Observation::Success),
        )
        .unwrap();
    watch.set(crate::watchdog::Demand::At(20), 0).unwrap();
    let original = runtime.inventory();
    assert_eq!(original.len(), 2);
    assert_eq!(original[0].identity, ["a", "g", "watch"]);
    assert_eq!(original[0].deadline, Some(20));
    assert_eq!(original[0].kind, crate::TimerKind::Watchdog);
    assert_eq!(original[1].kind, crate::TimerKind::Once);
    for outcome in [
        crate::Observation::ExpectedFailure,
        crate::Observation::ExpectedFailure,
        crate::Observation::Success,
    ] {
        once.set(Demand::At(0)).unwrap();
        let (accepted, _) = runtime.start(&due(&runtime, 0).unwrap()).unwrap().unwrap();
        assert_eq!(runtime.inventory()[1].phase, crate::TimerPhase::Running);
        accepted.complete(Demand::Inactive, outcome).unwrap();
        let measurements = runtime.inventory()[1].measurements;
        if outcome == crate::Observation::Success {
            assert_eq!(
                measurements.consecutive_expected_failures.exact_total(),
                Ok(0)
            );
        } else {
            assert_eq!(
                measurements.consecutive_expected_failures,
                measurements.completed
            );
        }
    }
    let snapshot = runtime.inventory();
    assert_eq!(snapshot[1].measurements.accepted.exact_total(), Ok(3));
    assert_eq!(snapshot[1].measurements.completed.exact_total(), Ok(3));
    assert_eq!(
        runtime.inventory(),
        snapshot,
        "reading never changes eligibility or observations"
    );
    once.remove().unwrap();
    let replacement = runtime
        .bind(["z", "g", "once"], crate::Lifetime::Retained, || {
            ready(Demand::Inactive)
        })
        .unwrap();
    let replaced = runtime.inventory();
    assert_ne!(replaced[1].incarnation, original[1].incarnation);
    assert_eq!(
        replaced[1].measurements,
        crate::TimerMeasurements::default()
    );
    replacement.remove().unwrap();
    watch.remove().unwrap();
    assert!(runtime.inventory().is_empty());
}

fn rejection(code: u32, message: &str) -> ic_platform_runtime::call::Completion {
    ic_platform_runtime::call::Completion::from_rejection(
        code.try_into().unwrap(),
        message.into(),
        None,
    )
}

fn trap() -> ic_platform_runtime::call::Completion {
    rejection(5, "IC0503: callback trapped")
}

#[test]
fn replica_refusal_defers_only_the_current_once_declaration() {
    for (code, text) in [
        (2, "IC0207: out of cycles"),
        (5, "IC0508: stopped"),
        (5, "IC0509: stopping"),
    ] {
        let runtime = Bindings::new();
        let bind = |name| {
            runtime
                .bind(["o", "g", name], crate::Lifetime::Retained, || {
                    ready(Demand::Inactive)
                })
                .unwrap()
        };
        let victim = bind("victim");
        victim.set(Demand::At(10)).unwrap();
        let submitted = due(&runtime, 10).unwrap();
        let response = rejection(code, text);
        assert_eq!(
            runtime
                .rejected(&submitted, response.outcome(), 11)
                .unwrap(),
            Some(Rejected::Deferred)
        );
        assert_eq!(
            runtime
                .rejected(&submitted, response.outcome(), 12)
                .unwrap(),
            None
        );
        assert_eq!(victim.state().unwrap(), State::Armed(10));
        let retry = 60_000_000_011;
        assert_eq!(runtime.wake(&std::iter::empty()).0, Some(retry));
        let other = bind("other");
        other.set(Demand::At(12)).unwrap();
        let healthy = due(&runtime, 12).unwrap();
        assert!(runtime.start(&healthy).unwrap().is_some());
        assert!(due(&runtime, retry - 1).is_none());
        let next = due(&runtime, retry).unwrap();
        assert!(!next.same(&submitted));
        assert!(runtime.start(&submitted).unwrap().is_none());
        let (accepted, _future) = runtime.start(&next).unwrap().unwrap();
        assert!(runtime.start(&next).unwrap().is_none());
        assert!(
            accepted
                .complete(Demand::Inactive, crate::Observation::Success)
                .unwrap()
        );

        victim.set(Demand::At(retry)).unwrap();
        let cancelled = due(&runtime, retry).unwrap();
        assert_eq!(
            runtime
                .rejected(&cancelled, response.outcome(), retry)
                .unwrap(),
            Some(Rejected::Deferred)
        );
        victim.set(Demand::Inactive).unwrap();
        assert!(due(&runtime, u64::MAX).is_none());
        victim.set(Demand::At(retry + 1)).unwrap();
        let fresh = due(&runtime, retry + 1).unwrap();
        assert!(runtime.start(&fresh).unwrap().is_some());
    }
}

#[test]
fn ambiguous_rejection_requires_recovery() {
    for (code, text) in [
        (5, "IC0509x: stopping"),
        (5, "trap: IC0509: stopping"),
        (2, "unknown"),
        (4, "IC0509: stopping"),
        (6, "IC0207: unknown"),
    ] {
        let runtime = Bindings::new();
        let handle = runtime
            .bind(["o", "g", "n"], crate::Lifetime::Retained, || {
                ready(Demand::Inactive)
            })
            .unwrap();
        handle.set(Demand::At(0)).unwrap();
        let dispatch = due(&runtime, 0).unwrap();
        assert_eq!(
            runtime
                .rejected(&dispatch, rejection(code, text).outcome(), 0)
                .unwrap(),
            Some(Rejected::Abandoned)
        );
        assert!(handle.recovery().unwrap().is_some());
    }
}
