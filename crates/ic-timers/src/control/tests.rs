use super::*;

impl TimerControl {
    pub(crate) const fn seed_generation_for_test(&mut self, generation: u64) {
        assert!(matches!(
            self.registration,
            TimerRegistration::Inactive { .. }
        ));
        self.generation = generation;
    }
}

fn arm(control: &mut TimerControl, deadline_ns: u64) -> u64 {
    assert_eq!(control.schedule(deadline_ns), Ok(Some(WakeupArm::Initial)));
    control.generation()
}

#[test]
fn duplicate_and_later_schedules_keep_one_earliest_handle() {
    let mut control = TimerControl::default();
    assert_eq!(arm(&mut control, 100), 1);
    assert_eq!(control.schedule(100), Ok(None));
    assert_eq!(control.schedule(200), Ok(None));
    assert_eq!(control.generation(), 1);
    assert_eq!(
        control.registration,
        TimerRegistration::Scheduled { deadline_ns: 100 }
    );
}

#[test]
fn earlier_schedule_replaces_and_invalidates_old_generation() {
    let mut control = TimerControl::default();
    let old_generation = arm(&mut control, 100);
    assert_eq!(control.schedule(50), Ok(Some(WakeupArm::Replacement)));
    assert_eq!(control.generation(), 2);
    assert_eq!(
        control.registration,
        TimerRegistration::Scheduled { deadline_ns: 50 }
    );
    assert!(!control.begin(old_generation));
    assert!(control.begin(2));
}

#[test]
fn authoritative_reconciliation_can_move_scheduled_deadline_later() {
    let mut control = TimerControl::default();
    let old_generation = arm(&mut control, 100);
    assert_eq!(control.reconcile(200), Ok(Some(WakeupArm::Replacement)));
    assert_eq!(control.generation(), 2);
    assert_eq!(
        control.registration,
        TimerRegistration::Scheduled { deadline_ns: 200 }
    );
    assert!(!control.begin(old_generation));
    assert!(control.begin(2));
}

#[test]
fn running_requests_leave_completion_selection_to_the_registry() {
    let mut control = TimerControl::default();
    let generation = arm(&mut control, 100);
    assert!(control.begin(generation));
    let TimerRegistration::Running { pending } = &mut control.registration else {
        panic!("accepted fixture should be running");
    };
    *pending = Some(OrdinaryPending::Unregister);
    assert_eq!(control.reconcile(300), Ok(None));
    assert_eq!(control.schedule(90), Ok(None));
    assert_eq!(
        control.registration,
        TimerRegistration::Running {
            pending: Some(OrdinaryPending::Unregister),
        }
    );
    assert_eq!(control.generation(), generation);
}

#[test]
fn completion_arms_the_supplied_deadline() {
    for deadline_ns in [90, 300] {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        assert_eq!(control.arm_deadline(deadline_ns), Ok(()));
        assert_eq!(
            control.registration,
            TimerRegistration::Scheduled { deadline_ns }
        );
        assert_eq!(control.generation(), 2);
    }
}

#[test]
fn completion_without_a_successor_stops_running_work() {
    let mut control = TimerControl::default();
    let generation = arm(&mut control, 100);
    assert!(control.begin(generation));
    assert!(!control.terminate(InactiveReason::Stopped));
    assert_eq!(
        control.registration,
        TimerRegistration::Inactive {
            reason: InactiveReason::Stopped,
        }
    );
}

#[test]
fn stopping_invalidates_delivery_without_allocating_and_rearm_uses_next_generation() {
    let mut control = TimerControl::default();
    let generation = arm(&mut control, 100);
    assert!(control.terminate(InactiveReason::Cancelled));
    assert!(!control.begin(generation));
    assert_eq!(control.generation(), generation);
    assert_eq!(
        control.registration,
        TimerRegistration::Inactive {
            reason: InactiveReason::Cancelled,
        }
    );
    assert!(!control.terminate(InactiveReason::Cancelled));
    assert_eq!(control.generation(), generation);
    assert_eq!(
        control.registration,
        TimerRegistration::Inactive {
            reason: InactiveReason::Cancelled,
        }
    );
    assert_eq!(arm(&mut control, 200), generation + 1);
    assert!(!control.begin(generation));
    assert!(control.begin(generation + 1));
}

#[test]
fn exhausted_generation_rejects_successors_without_preventing_stop() {
    let mut control = TimerControl::default();
    control.seed_generation_for_test(u64::MAX - 1);
    assert_eq!(arm(&mut control, 100), u64::MAX);

    assert_eq!(
        control.schedule(50),
        Err(TimerControlFailure::GenerationExhausted)
    );
    assert_eq!(
        control.registration,
        TimerRegistration::Scheduled { deadline_ns: 100 }
    );
    assert_eq!(control.generation(), u64::MAX);

    assert!(control.begin(u64::MAX));
    let TimerRegistration::Running { pending } = &mut control.registration else {
        panic!("accepted fixture should be running");
    };
    *pending = Some(OrdinaryPending::Cancel);
    assert_eq!(
        control.arm_deadline(200),
        Err(TimerControlFailure::GenerationExhausted)
    );
    assert_eq!(
        control.registration,
        TimerRegistration::Running {
            pending: Some(OrdinaryPending::Cancel),
        }
    );
    assert_eq!(control.generation(), u64::MAX);
    assert!(!control.terminate(InactiveReason::Cancelled));
    assert_eq!(control.generation(), u64::MAX);
    assert_eq!(
        control.registration,
        TimerRegistration::Inactive {
            reason: InactiveReason::Cancelled,
        }
    );
}
