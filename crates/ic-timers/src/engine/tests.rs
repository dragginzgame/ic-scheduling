use super::*;

fn start(state: &mut OnceState, at: u64) -> Attempt {
    state.set(Demand::At(at)).unwrap();
    let ticket = state.due(at).unwrap().unwrap();

    state.accept(ticket).unwrap().unwrap()
}

#[test]
fn owner_decision_overrides_callback_without_overlap() {
    for (owner, returned, expected) in [
        (Demand::At(20), Demand::Inactive, State::Armed(20)),
        (Demand::Inactive, Demand::At(30), State::Inactive),
        (Demand::At(10), Demand::Inactive, State::Armed(10)),
    ] {
        let mut state = OnceState::new();
        let attempt = start(&mut state, 10);
        state.set(owner).unwrap();
        state.set(owner).unwrap();
        assert_eq!(state.due(u64::MAX).unwrap(), None);
        assert!(state.complete(attempt, returned));
        assert_eq!(state.state(), expected);
    }
}

#[test]
fn latest_owner_wins_and_unoverridden_result_is_applied() {
    let mut state = OnceState::new();
    let attempt = start(&mut state, 0);
    state.set(Demand::At(20)).unwrap();
    state.set(Demand::Inactive).unwrap();
    state.set(Demand::At(40)).unwrap();
    assert!(state.complete(attempt, Demand::At(30)));
    assert_eq!(state.state(), State::Armed(40));
    let ticket = state.due(40).unwrap().unwrap();
    let attempt = state.accept(ticket).unwrap().unwrap();
    assert!(state.complete(attempt, Demand::Inactive));
    assert_eq!(state.state(), State::Inactive);
}

#[test]
fn same_deadline_preserves_ticket_but_cancel_and_rearm_do_not() {
    let mut state = OnceState::new();
    state.set(Demand::At(7)).unwrap();
    assert_eq!(state.due(6).unwrap(), None);
    let old = state.due(7).unwrap().unwrap();
    state.set(Demand::At(7)).unwrap();
    assert_eq!(state.due(100).unwrap(), Some(old));
    assert_eq!(state.ticket_counter, 1);
    state.set(Demand::Inactive).unwrap();
    state.set(Demand::At(7)).unwrap();
    let current = state.due(100).unwrap().unwrap();
    assert_ne!(old, current);
    assert_eq!(state.accept(old).unwrap(), None);
    let attempt = state.accept(current).unwrap().unwrap();
    assert_eq!(state.accept(current).unwrap(), None);
    assert!(state.complete(attempt, Demand::At(0)));
    assert!(!state.complete(attempt, Demand::Inactive));
    assert_eq!(state.state(), State::Armed(0));
}

#[test]
fn replacement_defeats_unaccepted_ticket() {
    let mut state = OnceState::new();
    state.set(Demand::At(0)).unwrap();
    let old = state.due(0).unwrap().unwrap();
    state.set(Demand::At(50)).unwrap();
    assert_eq!(state.accept(old).unwrap(), None);
    assert_eq!(state.due(49).unwrap(), None);
    assert!(state.due(50).unwrap().is_some());
}

#[test]
fn abandonment_discards_demand_and_requires_current_recovery() {
    let mut state = OnceState::new();
    let first = start(&mut state, 0);
    state.set(Demand::At(10)).unwrap();
    assert!(state.abandon(first));
    let State::RecoveryRequired(token) = state.state() else {
        panic!("recovery required")
    };
    assert_eq!(
        state.set(Demand::At(20)),
        Err(ControlError::RecoveryRequired)
    );
    state.set(Demand::Inactive).unwrap();
    assert_eq!(state.state(), State::RecoveryRequired(token));
    assert_eq!(state.due(u64::MAX).unwrap(), None);
    assert!(!state.complete(first, Demand::At(0)));
    state.recover(token, Demand::Inactive).unwrap();
    assert_eq!(
        state.recover(token, Demand::At(0)),
        Err(ControlError::StaleRecovery)
    );
    let second = start(&mut state, 0);
    assert!(!state.abandon(first));
    assert!(state.abandon(second));
    let blocked = state.state();
    assert_eq!(
        state.recover(token, Demand::At(0)),
        Err(ControlError::StaleRecovery)
    );
    assert_eq!(state.state(), blocked);
    let State::RecoveryRequired(current) = blocked else {
        panic!("recovery required")
    };
    state.recover(current, Demand::At(30)).unwrap();
    assert_eq!(state.state(), State::Armed(30));
}

#[test]
fn counter_exhaustion_faults_before_authority_reuse() {
    let mut state = OnceState::new();
    state.ticket_counter = u64::MAX;
    state.set(Demand::At(0)).unwrap();
    assert_eq!(state.due(0), Err(ControlError::Exhausted));
    assert_eq!(state.state(), State::Faulted);
    assert_eq!(state.set(Demand::Inactive), Err(ControlError::Exhausted));
    assert_eq!(state.due(0), Err(ControlError::Exhausted));

    let mut state = OnceState::new();
    state.attempt_counter = u64::MAX;
    state.set(Demand::At(0)).unwrap();
    let ticket = state.due(0).unwrap().unwrap();
    assert_eq!(state.accept(ticket), Err(ControlError::Exhausted));
    assert_eq!(state.state(), State::Faulted);
    assert_eq!(state.accept(ticket), Ok(None));
}
