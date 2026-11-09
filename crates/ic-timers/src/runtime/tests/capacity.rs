use super::*;

fn assert_same_registrations(before: &TimerInventorySnapshot) {
    let after = timer_inventory().unwrap();
    assert_eq!(after.len(), crate::MAX_TIMER_REGISTRATIONS);
    assert_eq!(
        after
            .timers()
            .iter()
            .map(TimerSnapshot::registration_id)
            .collect::<Vec<_>>(),
        before
            .timers()
            .iter()
            .map(TimerSnapshot::registration_id)
            .collect::<Vec<_>>()
    );
}

// Share the capacity invariant; each caller supplies its real typed controls and
// policy-specific delivery steps, including Watchdog's separate scheduler message.
fn assert_full_capacity_rearming(
    calls: &Cell<u64>,
    ensure: impl Fn(u64),
    cancel: impl Fn(),
    reconcile: impl Fn(u64),
    complete: impl Fn(u64),
) {
    let _fillers = fill_registry();
    let before = timer_inventory().unwrap();
    for (round, deadline) in [(1, 20), (2, 30)] {
        ensure(deadline);
        ensure(deadline);
        assert_eq!(timer_count(), 1, "repeated ensure must coalesce");
        cancel();
        assert_eq!(timer_count(), 0);
        assert_eq!(calls.get(), round - 1, "cancelled work must not execute");
        assert_same_registrations(&before);

        reconcile(deadline);
        assert_eq!(timer_count(), 1);
        set_time(deadline);
        complete(round);
        assert_eq!(
            calls.get(),
            round,
            "the original callback must execute once"
        );
        assert_eq!(timer_count(), 0, "Stop must clear remaining wakeups");
        assert!(!run_next_due());
        assert_same_registrations(&before);
    }
}

#[test]
fn retained_once_rearms_at_full_registry_capacity() {
    let _fixture = setup();
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let registration = retained_once(identity("retained-once"), move || {
        callback_calls.set(callback_calls.get() + 1);
    });
    assert_full_capacity_rearming(
        &calls,
        |deadline| {
            registration
                .ensure_scheduled(TimerSchedule::At(deadline))
                .unwrap();
            assert!(registration.has_armed_wakeup().unwrap());
        },
        || registration.cancel().unwrap(),
        |deadline| {
            registration
                .reconcile_schedule(Some(TimerSchedule::At(deadline)))
                .unwrap();
        },
        |_| assert!(run_next_due()),
    );
}

#[test]
fn retained_after_completion_rearms_at_full_registry_capacity() {
    let _fixture = setup();
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let registration = register_after_completion(
        identity("retained-after-completion"),
        test_cadence(5),
        TimerStart::Retained { initial: None },
        stopping_after_completion(move || callback_calls.set(callback_calls.get() + 1)),
    )
    .unwrap();
    assert_full_capacity_rearming(
        &calls,
        |_| {
            registration.ensure_scheduled().unwrap();
            assert!(registration.has_armed_wakeup().unwrap());
        },
        || registration.cancel().unwrap(),
        |deadline| {
            registration
                .reconcile_schedule(Some(TimerSchedule::At(deadline)))
                .unwrap();
        },
        |_| assert!(run_next_due()),
    );
}

#[test]
fn retained_watchdog_rearms_at_full_registry_capacity() {
    let _fixture = setup();
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let registration = retained_watchdog(identity("retained-watchdog"), move || {
        callback_calls.set(callback_calls.get() + 1);
    });
    assert_full_capacity_rearming(
        &calls,
        |_| {
            registration.ensure_scheduled().unwrap();
            assert!(registration.has_armed_wakeup().unwrap());
        },
        || registration.cancel().unwrap(),
        |deadline| {
            registration
                .reconcile_schedule(Some(TimerSchedule::At(deadline)))
                .unwrap();
        },
        |round| {
            assert!(run_next_due(), "scheduler must pre-arm the successor");
            assert_eq!(calls.get(), round - 1, "scheduler must not invoke work");
            assert_eq!(timer_count(), 2, "successor and queued work stay distinct");
            assert!(run_next_due(), "queued work must execute");
        },
    );
}
