use super::*;
use std::{
    cell::Cell,
    future::{pending, ready},
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
            .bind(["owner", "group", "name"], move || {
                counted.set(counted.get() + 1);
                ready(Demand::Inactive)
            })
            .unwrap(),
    );
    let dropped = Rc::new(Cell::new(false));
    let guard = ReenterOnDrop {
        handle: Rc::clone(&handle),
        dropped: Rc::clone(&dropped),
    };
    let rejected = runtime.bind(["owner", "group", "name"], move || {
        let _keep = &guard;
        ready(Demand::Inactive)
    });
    assert!(matches!(rejected, Err(Error::DuplicateIdentity)));
    assert!(dropped.get());
    handle.set(Demand::At(0)).unwrap();
    drop(handle);
    assert_eq!(calls.get(), 0);
    let dispatch = runtime.due(0).unwrap();
    let (accepted, mut future) = runtime.start(&dispatch).unwrap().unwrap();
    assert_eq!(calls.get(), 1);
    assert!(runtime.start(&dispatch).unwrap().is_none());
    assert_eq!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop())),
        Poll::Ready(Demand::Inactive)
    );
    assert!(accepted.complete(Demand::Inactive).unwrap());
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
            .bind(["o", "g", "n"], move || {
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
                    Demand::Inactive
                }
            })
            .unwrap(),
    );
    *slot.borrow_mut() = Some(Rc::clone(&handle));
    handle.set(Demand::At(0)).unwrap();
    let dispatch = runtime.due(0).unwrap();
    let (accepted, mut future) = runtime.start(&dispatch).unwrap().unwrap();
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert!(runtime.due(u64::MAX).is_none());
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
        .bind(["o", "g", "a"], || ready(Demand::Inactive))
        .unwrap();
    let b = first
        .bind(["o", "g", "b"], || ready(Demand::Inactive))
        .unwrap();
    a.set(Demand::At(0)).unwrap();
    b.set(Demand::At(0)).unwrap();
    let ticket = first.due(0).unwrap();
    assert!(matches!(second.start(&ticket), Err(Error::StaleAuthority)));
    let (accepted, future) = first.start(&ticket).unwrap().unwrap();
    drop(future);
    assert!(accepted.abandon().unwrap());
    let key = a.recovery().unwrap().unwrap();
    assert_eq!(b.recover(&key, Demand::At(0)), Err(Error::StaleAuthority));
    drop(first);
    assert_eq!(a.set(Demand::At(0)), Err(Error::StaleAuthority));
    assert_eq!(
        accepted.complete(Demand::Inactive),
        Err(Error::StaleAuthority)
    );
}

#[test]
fn bounded_admission_and_rotating_selection() {
    let runtime = Bindings::new();
    for invalid in ["".to_owned(), "é".repeat(33)] {
        assert!(matches!(
            runtime.bind([&invalid, "g", "n"], || ready(Demand::Inactive)),
            Err(Error::InvalidIdentity)
        ));
    }
    for index in 0..CAPACITY {
        let handle = runtime
            .bind(["o", "g", &index.to_string()], || ready(Demand::Inactive))
            .unwrap();
        handle.set(Demand::At(0)).unwrap();
    }
    assert!(matches!(
        runtime.bind(["o", "g", "overflow"], || ready(Demand::Inactive)),
        Err(Error::Capacity)
    ));
    for expected in 0..CAPACITY {
        assert_eq!(runtime.due(0).unwrap().slot, expected);
    }
    assert_eq!(runtime.due(0).unwrap().slot, 0);
}

#[test]
fn submitted_ticket_does_not_hide_new_due_work_or_a_replacement() {
    let runtime = Bindings::new();
    let first = runtime
        .bind(["o", "g", "first"], || ready(Demand::Inactive))
        .unwrap();
    let second = runtime
        .bind(["o", "g", "second"], || ready(Demand::Inactive))
        .unwrap();
    first.set(Demand::At(0)).unwrap();
    second.set(Demand::At(10)).unwrap();
    let submitted = runtime.due(0).unwrap();
    assert_eq!(runtime.deadline(&[&submitted]), Some(10));
    first.set(Demand::Inactive).unwrap();
    first.set(Demand::At(0)).unwrap();
    assert_eq!(runtime.deadline(&[&submitted]), Some(0));
    let replacement = runtime.due(0).unwrap();
    second.set(Demand::Inactive).unwrap();
    assert_eq!(runtime.deadline(&[&replacement]), None);
    // A ticket from another runtime cannot mask this runtime's deadline.
    let other = Bindings::new();
    other
        .bind(["o", "g", "other"], || ready(Demand::Inactive))
        .unwrap()
        .set(Demand::At(0))
        .unwrap();
    assert_eq!(runtime.deadline(&[&other.due(0).unwrap()]), Some(0));
}
