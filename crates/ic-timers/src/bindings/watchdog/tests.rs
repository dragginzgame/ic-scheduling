use super::super::Scheduled;
use super::*;
use crate::engine;
use std::cell::Cell;

#[test]
fn shared_admission_and_reentrant_owner_commands() {
    let bindings = Bindings::new();
    let slot: Rc<RefCell<Option<WatchdogHandle>>> = Rc::new(RefCell::new(None));
    let captured = Rc::clone(&slot);
    let calls = Rc::new(Cell::new(0));
    let counted = Rc::clone(&calls);
    let handle = bindings
        .bind_watchdog(
            ["o", "g", "watch"],
            NonZeroU64::new(10).unwrap(),
            crate::Lifetime::Retained,
            move || {
                counted.set(counted.get() + 1);
                let slot = captured.borrow();
                let handle = slot.as_ref().unwrap();
                assert_eq!(handle.status().unwrap(), (Some(20), true, false));
                handle.set(Demand::Inactive, 10).unwrap();
                (Next::At(0), crate::Observation::Success)
            },
        )
        .unwrap();
    handle.set(Demand::At(10), 0).unwrap();
    *slot.borrow_mut() = Some(handle);
    assert!(matches!(
        bindings.bind(["o", "g", "watch"], crate::Lifetime::Retained, || ready(
            engine::Demand::Inactive
        )),
        Err(Error::DuplicateIdentity)
    ));
    for n in 1..super::super::CAPACITY {
        bindings
            .bind(
                ["o", "g", &n.to_string()],
                crate::Lifetime::Retained,
                || ready(engine::Demand::Inactive),
            )
            .unwrap();
    }
    assert!(matches!(
        bindings.bind_watchdog(
            ["o", "g", "extra"],
            NonZeroU64::new(10).unwrap(),
            crate::Lifetime::Retained,
            || { (Next::Stop, crate::Observation::Success) }
        ),
        Err(Error::Capacity)
    ));
    bindings.advance_watchdogs(10);
    assert_eq!(calls.get(), 0);
    let dispatch = due(&bindings, 10);
    assert!(bindings.run_watchdog(&dispatch).unwrap());
    assert!(!bindings.run_watchdog(&dispatch).unwrap());
    assert_eq!(
        slot.borrow().as_ref().unwrap().status().unwrap(),
        (None, true, false)
    );
    assert!(dispatch.settle(false).unwrap());
    assert_eq!(calls.get(), 1);
}

#[test]
fn authority_and_settlement_are_scoped_to_the_shared_table() {
    let a = Bindings::new();
    let b = Bindings::new();
    let handle = a
        .bind_watchdog(
            ["o", "g", "w"],
            NonZeroU64::new(10).unwrap(),
            crate::Lifetime::Retained,
            || (Next::Continue, crate::Observation::Success),
        )
        .unwrap();
    handle.set(Demand::KeepCadence, 0).unwrap();
    a.advance_watchdogs(10);
    let old = due(&a, 10);
    assert_eq!(b.run_watchdog(&old), Err(Error::StaleAuthority));
    a.advance_watchdogs(100);
    assert!(a.run_watchdog(&old).unwrap());
    assert_eq!(handle.status().unwrap(), (Some(110), true, false));
    assert!(a.select(100).is_none());
    assert!(old.settle(false).unwrap());
    a.advance_watchdogs(110);
    let fresh = due(&a, 110);
    assert!(!old.settle(false).unwrap());
    assert!(a.run_watchdog(&fresh).unwrap());
    drop(a);
    assert_eq!(fresh.settle(false), Err(Error::StaleAuthority));
    assert_eq!(handle.status(), Err(Error::StaleAuthority));
}

#[test]
fn mixed_selection_and_wakes_keep_cadence_separate_from_delivery() {
    let bindings = Bindings::new();
    let once = bindings
        .bind(["o", "g", "once"], crate::Lifetime::Retained, || {
            ready(engine::Demand::Inactive)
        })
        .unwrap();
    once.set(engine::Demand::At(0)).unwrap();
    let watchdog = bindings
        .bind_watchdog(
            ["o", "g", "watch"],
            NonZeroU64::new(10).unwrap(),
            crate::Lifetime::Retained,
            || (Next::Continue, crate::Observation::Success),
        )
        .unwrap();
    watchdog.set(Demand::At(0), 0).unwrap();
    bindings.advance_watchdogs(0);
    assert_eq!(bindings.wake(&std::iter::empty()), (Some(0), Some(10)));
    assert!(matches!(bindings.select(0), Some(Scheduled::Once(_))));
    let Some(Scheduled::Watchdog(dispatch)) = bindings.select(0) else {
        panic!("rotating mixed selection");
    };
    once.set(engine::Demand::Inactive).unwrap();
    assert_eq!(bindings.wake(&std::iter::empty()), (Some(0), Some(10)));
    let submitted = Scheduled::Watchdog(dispatch.clone());
    assert_eq!(
        bindings.wake(&std::iter::once(&submitted)),
        (None, Some(10))
    );
    bindings.advance_watchdogs(100);
    assert_eq!(
        bindings.wake(&std::iter::once(&submitted)),
        (None, Some(110))
    );
    assert!(bindings.run_watchdog(&dispatch).unwrap());
    assert!(dispatch.settle(false).unwrap());
}

fn due(bindings: &Bindings, now: u64) -> Dispatch {
    let Some(Scheduled::Watchdog(dispatch)) = bindings.select(now) else {
        panic!("expected watchdog work");
    };
    dispatch
}

#[test]
fn removal_invalidates_pending_watchdog_and_waits_for_settlement() {
    let bindings = Bindings::new();
    let handle = bindings
        .bind_watchdog(
            ["o", "g", "remove"],
            NonZeroU64::new(10).unwrap(),
            crate::Lifetime::Retained,
            || (Next::Stop, crate::Observation::Success),
        )
        .unwrap();
    handle.set(Demand::At(0), 0).unwrap();
    bindings.advance_watchdogs(0);
    let old = due(&bindings, 0);
    handle.remove().unwrap();
    let replacement = bindings
        .bind_watchdog(
            ["o", "g", "remove"],
            NonZeroU64::new(10).unwrap(),
            crate::Lifetime::RemoveWhenStopped,
            || (Next::Stop, crate::Observation::Success),
        )
        .unwrap();
    replacement.set(Demand::At(0), 0).unwrap();
    assert!(!bindings.run_watchdog(&old).unwrap());
    assert!(!old.settle(false).unwrap());
    bindings.advance_watchdogs(0);
    let dispatch = due(&bindings, 0);
    assert!(bindings.run_watchdog(&dispatch).unwrap());
    assert_eq!(replacement.remove(), Err(Error::Busy));
    assert_eq!(replacement.status(), Ok((None, true, false)));
    assert!(dispatch.settle(false).unwrap());
    assert_eq!(replacement.status(), Err(Error::StaleAuthority));
    assert!(!dispatch.settle(false).unwrap());
}

fn ready(
    demand: crate::engine::Demand,
) -> std::future::Ready<(crate::engine::Demand, crate::Observation)> {
    std::future::ready((demand, crate::Observation::Success))
}
