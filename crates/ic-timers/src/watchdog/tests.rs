use super::*;

fn armed() -> WatchdogState {
    let mut state = WatchdogState::new(NonZeroU64::new(10).unwrap());
    state.set(Demand::KeepCadence, 0).unwrap();
    state
}

#[test]
fn delayed_delivery_coalesces_without_replacing_ticket() {
    let mut state = armed();
    state.advance(9).unwrap();
    assert_eq!(state.pending(), None);
    state.advance(10).unwrap();
    let ticket = state.pending().unwrap();
    assert_eq!(state.deadline(), Some(20));
    for now in [20, 100, 1000] {
        state.advance(now).unwrap();
        assert_eq!(state.pending(), Some(ticket));
        assert_eq!(state.deadline(), Some(now + 10));
    }
    assert!(state.accept(ticket));
    assert!(!state.accept(ticket));
    assert!(state.complete(ticket, Next::Continue));
    assert_eq!(state.deadline(), Some(1010));
    state.advance(1010).unwrap();
    assert_eq!(
        state.pending(),
        None,
        "response still owns the outstanding ticket"
    );
    assert!(state.outstanding());
    assert!(state.settle(ticket));
    assert_eq!(state.deadline(), Some(1020));
    state.advance(1020).unwrap();
    let next = state.pending().unwrap();
    assert_ne!(ticket, next);
    assert!(!state.settle(ticket));
    assert_eq!(state.pending(), Some(next));
}

#[test]
fn work_trap_preserves_successor_and_does_not_retry_immediately() {
    for accepted in [false, true] {
        let mut state = armed();
        state.advance(10).unwrap();
        let ticket = state.pending().unwrap();
        if accepted {
            assert!(state.accept(ticket));
        }
        // Pending represents a rolled-back work message; Running represents a
        // confirmed terminated callback. Transport must prove either disposition.
        assert!(state.settle(ticket));
        assert_eq!(state.deadline(), Some(20));
        state.advance(19).unwrap();
        assert_eq!(state.pending(), None);
        state.advance(20).unwrap();
        assert_ne!(state.pending(), Some(ticket));
        assert!(state.pending().is_some());
    }
}

#[test]
fn owner_commands_override_even_when_deadline_is_unchanged() {
    for (demand, returned, expected) in [
        (Demand::KeepCadence, Next::Stop, Some(20)),
        (Demand::At(20), Next::Stop, Some(20)),
        (Demand::Inactive, Next::At(0), None),
        (Demand::At(30), Next::At(40), Some(30)),
    ] {
        let mut state = armed();
        state.advance(10).unwrap();
        let ticket = state.pending().unwrap();
        assert!(state.accept(ticket));
        state.set(demand, 10).unwrap();
        state.set(demand, 10).unwrap();
        assert!(state.complete(ticket, returned));
        assert_eq!(state.deadline(), expected);
        assert!(state.outstanding());
        assert!(state.settle(ticket));
    }
}

#[test]
fn cancelled_pending_work_and_stale_results_cannot_control_new_demand() {
    let mut state = armed();
    state.advance(10).unwrap();
    let old = state.pending().unwrap();
    state.set(Demand::Inactive, 10).unwrap();
    state.set(Demand::At(10), 10).unwrap();
    assert!(!state.accept(old));
    state.advance(10).unwrap();
    let fresh = state.pending().unwrap();
    assert_ne!(old, fresh);
    assert!(!state.complete(old, Next::Stop));
    assert!(!state.settle(old));
    assert!(state.accept(fresh));
    assert!(state.complete(fresh, Next::At(50)));
    assert_eq!(state.deadline(), Some(50));
    assert!(!state.complete(fresh, Next::Stop));
    assert!(state.settle(fresh));
    state.advance(50).unwrap();
    let last = state.pending().unwrap();
    assert!(state.accept(last));
    assert!(state.complete(last, Next::Stop));
    assert_eq!(state.deadline(), None);
}

#[test]
fn equal_pending_demand_retains_ticket_but_replacement_invalidates_it() {
    let mut state = armed();
    state.advance(10).unwrap();
    let ticket = state.pending().unwrap();
    state.set(Demand::KeepCadence, 11).unwrap();
    state.set(Demand::At(20), 11).unwrap();
    assert_eq!(state.pending(), Some(ticket));
    state.set(Demand::At(40), 11).unwrap();
    assert_eq!(state.pending(), None);
    assert!(!state.accept(ticket));
    state.advance(39).unwrap();
    assert_eq!(state.pending(), None);
    state.advance(40).unwrap();
    assert!(state.pending().is_some());
}

#[test]
fn overflow_faults_before_deadline_or_ticket_wraps() {
    let mut state = armed();
    assert_eq!(state.advance(u64::MAX), Err(Exhausted));
    assert!(state.faulted());
    assert_eq!(state.deadline(), None);
    assert!(!state.outstanding());
    assert_eq!(state.set(Demand::At(0), 0), Err(Exhausted));
    assert_eq!(state.advance(0), Err(Exhausted));

    let mut state = WatchdogState::new(NonZeroU64::new(10).unwrap());
    assert_eq!(state.set(Demand::KeepCadence, u64::MAX), Err(Exhausted));
    assert!(state.faulted());

    let mut state = armed();
    state.sequence = u64::MAX;
    assert_eq!(state.advance(10), Err(Exhausted));
    assert!(state.faulted());
    assert_eq!(state.pending(), None);
}
