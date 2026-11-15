use super::*;

#[test]
fn reselected_occurrence_does_not_revive_old_delivery_authority() {
    let mut progress = Progress::new(0);
    let old = progress.select(60).unwrap();
    progress.pause().unwrap();
    assert_eq!(progress.select(60), Err(Error::Paused));
    progress.resume().unwrap();
    let fresh = progress.select(60).unwrap();
    assert_eq!(fresh.occurrence, old.occurrence);
    assert_ne!(fresh, old);
    assert_eq!(progress.accept(old, 60), Err(Error::Stale));
    assert_eq!(progress.accept(fresh, 59), Err(Error::NotDue));
    let context = progress.accept(fresh, 70).unwrap();
    assert_eq!(
        context,
        Context {
            occurrence: fresh.occurrence,
            started_at: 70
        }
    );
    assert_eq!(progress.accept(fresh, 70), Err(Error::Stale));
    assert_eq!(progress.select(120), Err(Error::Busy));
    progress.complete(fresh).unwrap();
    assert_eq!(progress.through(), Some(60));
    assert_eq!(progress.complete(fresh), Err(Error::Stale));
}

#[test]
fn schedule_edit_preserves_running_identity_and_newer_pause() {
    let mut progress = Progress::new(0);
    let old = progress.select(100).unwrap();
    progress.accept(old, 110).unwrap();
    progress.edit(300).unwrap();
    progress.pause().unwrap();
    assert_eq!(progress.select(400), Err(Error::Busy));
    progress.complete(old).unwrap();
    assert_eq!(progress.through(), Some(300));
    assert_eq!(progress.select(400), Err(Error::Paused));
    progress.resume().unwrap();
    let fresh = progress.select(400).unwrap();
    assert_eq!(fresh.occurrence.revision, old.occurrence.revision + 1);
    assert_eq!(progress.block(old), Err(Error::Stale));
    progress.accept(fresh, 400).unwrap();
    progress.complete(fresh).unwrap();
    assert_eq!(progress.through(), Some(400));
}

#[test]
fn queued_edit_invalidates_selection_without_marking_it_settled() {
    let mut progress = Progress::new(0);
    let old = progress.select(100).unwrap();
    progress.edit(10).unwrap();
    assert_eq!(progress.accept(old, 100), Err(Error::Stale));
    let fresh = progress.select(100).unwrap();
    assert_ne!(fresh.occurrence, old.occurrence);
    assert_eq!(progress.through(), Some(10));
}

#[test]
fn uncertainty_blocks_new_occurrences_across_owner_changes() {
    for accepted in [false, true] {
        let mut progress = Progress::new(0);
        let old = progress.select(100).unwrap();
        if accepted {
            progress.accept(old, 100).unwrap();
        }
        progress.block(old).unwrap();
        progress.pause().unwrap();
        progress.edit(300).unwrap();
        progress.resume().unwrap();
        assert_eq!(progress.select(400), Err(Error::RecoveryRequired));
        assert_eq!(progress.complete(old), Err(Error::Stale));
        progress.reconcile(old).unwrap();
        assert_eq!(progress.through(), Some(300));
        let fresh = progress.select(400).unwrap();
        progress.block(fresh).unwrap();
        assert_eq!(progress.reconcile(old), Err(Error::Stale));
        assert_eq!(progress.select(500), Err(Error::RecoveryRequired));
        progress.reconcile(fresh).unwrap();
        assert_eq!(progress.through(), Some(400));
        assert_eq!(progress.select(400), Err(Error::BeforeCursor));
    }
}

#[test]
fn settled_progress_cannot_rewind_with_clock_or_schedule_edit() {
    let mut progress = Progress::new(100);
    assert_eq!(progress.select(99), Err(Error::BeforeCursor));
    assert_eq!(progress.select(100), Err(Error::BeforeCursor));
    let current = progress.select(200).unwrap();
    progress.accept(current, 250).unwrap();
    progress.complete(current).unwrap();
    progress.edit(50).unwrap();
    assert_eq!(progress.through(), Some(200));
    assert_eq!(progress.select(200), Err(Error::BeforeCursor));
    assert!(progress.select(201).is_ok());
}

#[test]
fn counters_fault_before_reuse_without_erasing_unresolved_work() {
    let mut progress = Progress::new(0);
    progress.sequence = u64::MAX;
    assert_eq!(progress.select(100), Err(Error::Exhausted));
    assert!(progress.faulted);
    assert_eq!(progress.resume(), Err(Error::Exhausted));

    let mut progress = Progress::new(0);
    let selected = progress.select(100).unwrap();
    progress.accept(selected, 100).unwrap();
    let evidence = progress.active;
    progress.revision = u64::MAX;
    assert_eq!(progress.edit(200), Err(Error::Exhausted));
    assert_eq!(progress.active, evidence);
    assert_eq!(progress.through(), Some(0));
    assert_eq!(progress.complete(selected), Err(Error::Exhausted));
}

#[test]
fn checkpoint_preserves_revision_cursor_and_queued_occurrence() {
    let mut progress = Progress::new(10);
    progress.edit(20).unwrap();
    let selected = progress.select(100).unwrap();
    let checkpoint = progress.checkpoint().unwrap();
    let mut restored = Progress::restore(&checkpoint);
    assert_eq!(restored.checkpoint().unwrap(), checkpoint);
    let queued = restored.active.unwrap().selection;
    assert_eq!(queued.occurrence, selected.occurrence);
    restored.accept(queued, 110).unwrap();
    assert_eq!(restored.checkpoint(), Err(Error::Busy));
    restored.block(queued).unwrap();
    assert_eq!(restored.checkpoint(), Err(Error::RecoveryRequired));
    restored.pause().unwrap();
    assert_eq!(restored.checkpoint(), Err(Error::RecoveryRequired));
    restored.reconcile(queued).unwrap();
    let paused = restored.checkpoint().unwrap();
    assert_eq!(paused.through(), Some(100));
    assert!(paused.paused());
    assert_eq!(paused.selected_at(), None);
    let mut restored = Progress::restore(&paused);
    assert_eq!(restored.select(200), Err(Error::Paused));
    restored.resume().unwrap();
    assert_eq!(restored.select(100), Err(Error::BeforeCursor));
    assert_eq!(
        restored.select(200).unwrap().occurrence.revision,
        selected.occurrence.revision
    );
}

// Explore short command interleavings without adding a property-test dependency.
// Assertions express safety invariants, not a second transition implementation.
#[test]
fn bounded_command_sequences_preserve_progress_and_uncertainty() {
    const ACTIONS: u32 = 11;
    const DEPTH: u32 = 5;
    for trace in 0..ACTIONS.pow(DEPTH) {
        let mut commands = trace;
        let mut progress = Progress::new(0);
        let mut issued = Vec::new();
        for _ in 0..DEPTH {
            let action = commands % ACTIONS;
            commands /= ACTIONS;
            let before = snapshot(&progress);
            let current = progress.active.map(|active| active.selection);
            let result = match action {
                0 | 1 => progress.select(u64::from(action + 1)).map(|selection| {
                    assert!(Some(selection.occurrence.scheduled_at) > before.1);
                    issued.push(selection);
                }),
                2 => current.map_or(Err(Error::Stale), |selection| {
                    progress.accept(selection, 0).map(|_| ())
                }),
                3 => current.map_or(Err(Error::Stale), |selection| {
                    progress.accept(selection, 3).map(|context| {
                        assert!(context.started_at >= context.occurrence.scheduled_at);
                    })
                }),
                4 => current.map_or(Err(Error::Stale), |selection| progress.complete(selection)),
                5 => current.map_or(Err(Error::Stale), |selection| progress.block(selection)),
                6 => current.map_or(Err(Error::Stale), |selection| progress.reconcile(selection)),
                7 => progress.pause(),
                8 => progress.resume(),
                9 => progress.edit(0),
                10 => progress.edit(3),
                _ => unreachable!(),
            };
            assert!(progress.through() >= before.1);
            if result.is_err() {
                assert_eq!(
                    snapshot(&progress),
                    before,
                    "trace {trace}: failed command {action} mutated progress"
                );
            }
            if let Some(active) = before.3
                && matches!(active.phase, Phase::Accepted { .. } | Phase::Blocked)
                && !matches!(action, 4 | 6)
            {
                assert_eq!(
                    progress.active.map(|active| active.selection),
                    Some(active.selection),
                    "command {action} erased unresolved identity"
                );
            }
            match progress.active.map(|active| active.phase) {
                Some(Phase::Accepted { .. }) => assert_eq!(progress.checkpoint(), Err(Error::Busy)),
                Some(Phase::Blocked) => {
                    assert_eq!(progress.checkpoint(), Err(Error::RecoveryRequired))
                }
                _ => {
                    let checkpoint = progress.checkpoint().unwrap();
                    let decoded = crate::checkpoint::Checkpoint::new(
                        checkpoint.version(),
                        checkpoint.revision(),
                        checkpoint.through(),
                        checkpoint.selected_at(),
                        checkpoint.control(),
                        None,
                    )
                    .unwrap();
                    assert_eq!(
                        Progress::restore(&decoded).checkpoint().unwrap(),
                        checkpoint
                    );
                }
            }
            // Invalidated selections remain invalid even when the occurrence time is reused.
            for &stale in &issued {
                if progress
                    .active
                    .is_some_and(|active| active.selection == stale)
                {
                    continue;
                }
                let settled = snapshot(&progress);
                assert_eq!(progress.accept(stale, 3), Err(Error::Stale));
                assert_eq!(progress.complete(stale), Err(Error::Stale));
                assert_eq!(progress.block(stale), Err(Error::Stale));
                assert_eq!(progress.reconcile(stale), Err(Error::Stale));
                assert_eq!(snapshot(&progress), settled);
            }
        }
    }
}

fn snapshot(progress: &Progress) -> (u64, Option<u64>, ControlState, Option<Active>, u64, bool) {
    (
        progress.revision,
        progress.through,
        progress.control,
        progress.active,
        progress.sequence,
        progress.faulted,
    )
}

#[test]
fn zero_occurrence_round_trips_before_acceptance_and_retires_once() {
    let mut progress = Progress::unstarted();
    assert_eq!(progress.through(), None);
    let first = progress.select(0).unwrap();
    let checkpoint = progress.checkpoint().unwrap();
    assert_eq!(checkpoint.selected_at(), Some(0));
    let mut restored = Progress::restore(&checkpoint);
    let selected = restored.active.unwrap().selection;
    restored.accept(selected, 100).unwrap();
    assert_eq!(restored.checkpoint(), Err(Error::Busy));
    restored.block(selected).unwrap();
    assert_eq!(restored.checkpoint(), Err(Error::RecoveryRequired));
    restored.reconcile(selected).unwrap();
    assert_eq!(restored.through(), Some(0));
    assert_eq!(restored.select(0), Err(Error::BeforeCursor));
    progress.accept(first, 0).unwrap();
    progress.complete(first).unwrap();
    assert_eq!(progress.through(), Some(0));
    assert_eq!(progress.select(0), Err(Error::BeforeCursor));
}

#[test]
fn cancellation_is_terminal_and_refuses_unsettled_work_without_mutation() {
    for phase in 0..3 {
        let mut progress = Progress::unstarted();
        let selected = progress.select(0).unwrap();
        if phase > 0 {
            progress.accept(selected, 0).unwrap();
        }
        if phase == 2 {
            progress.block(selected).unwrap();
        }
        let before = snapshot(&progress);
        if phase > 0 {
            assert_eq!(
                progress.cancel(),
                Err(if phase == 1 {
                    Error::Busy
                } else {
                    Error::RecoveryRequired
                })
            );
            assert_eq!(snapshot(&progress), before);
        } else {
            progress.cancel().unwrap();
            let checkpoint = progress.checkpoint().unwrap();
            assert_eq!(checkpoint.control(), ControlState::Cancelled);
            assert_eq!(checkpoint.selected_at(), None);
            assert_eq!(checkpoint.through(), None);
            let mut restored = Progress::restore(&checkpoint);
            for result in [
                restored.resume(),
                restored.pause(),
                restored.edit(10),
                restored.select(0).map(|_| ()),
                restored.accept(selected, 0).map(|_| ()),
                restored.complete(selected),
                restored.reconcile(selected),
            ] {
                assert_eq!(result, Err(Error::Cancelled));
            }
            assert_eq!(restored.checkpoint().unwrap(), checkpoint);
        }
    }
    let mut paused = Progress::new(10);
    paused.pause().unwrap();
    paused.cancel().unwrap();
    assert_eq!(
        paused.checkpoint().unwrap().control(),
        ControlState::Cancelled
    );
}
