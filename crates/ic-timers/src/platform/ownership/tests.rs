use super::*;

#[test]
fn first_arm_rearm_cancel_and_stale_callbacks_accept_legitimate_state() {
    let mut owner = TimerOwnership::new();
    assert!(owner.record_replacement(0, 100, 1));
    assert!(owner.record_replacement(100, 200, 2));
    assert!(owner.record_replacement(200, 0, 3));
    assert!(owner.record_replacement(0, 0, 201)); // stale callback after cancellation
    assert!(owner.record_replacement(0, 300, 201));
    assert!(owner.record_replacement(300, 400, 202)); // stale transport callback
}

#[test]
fn firing_reset_is_valid_only_at_or_after_the_expected_deadline() {
    let mut owner = TimerOwnership::new();
    assert!(owner.record_replacement(0, 100, 1));
    assert!(!owner.record_replacement(0, 200, 99));
    assert_eq!(owner.expected, 100);
    assert!(owner.record_replacement(0, 200, 100));
    assert!(owner.record_replacement(0, 300, 250));
}

#[test]
fn foreign_first_arm_overwrite_and_early_clear_are_detected_without_committing() {
    let mut owner = TimerOwnership::new();
    assert!(!owner.record_replacement(100, 200, 1));
    assert_eq!(owner.expected, 0);
    assert!(owner.record_replacement(0, 100, 1));
    for (prior, now) in [(101, 2), (101, 200), (0, 2)] {
        assert!(!owner.record_replacement(prior, 300, now));
        assert_eq!(owner.expected, 100);
    }
    assert!(owner.record_replacement(100, 0, 3));
}

#[test]
fn equal_writes_and_overdue_clears_are_observationally_indistinguishable() {
    let mut owner = TimerOwnership::new();
    assert!(owner.record_replacement(0, 100, 1));
    assert!(owner.record_replacement(100, 200, 2));
    assert!(owner.record_replacement(0, 300, 201));
}
