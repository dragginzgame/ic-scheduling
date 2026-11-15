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
    assert_eq!(progress.through(), 60);
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
    assert_eq!(progress.through(), 300);
    assert_eq!(progress.select(400), Err(Error::Paused));
    progress.resume().unwrap();
    let fresh = progress.select(400).unwrap();
    assert_eq!(fresh.occurrence.revision, old.occurrence.revision + 1);
    assert_eq!(progress.block(old), Err(Error::Stale));
    progress.accept(fresh, 400).unwrap();
    progress.complete(fresh).unwrap();
    assert_eq!(progress.through(), 400);
}

#[test]
fn queued_edit_invalidates_selection_without_marking_it_settled() {
    let mut progress = Progress::new(0);
    let old = progress.select(100).unwrap();
    progress.edit(10).unwrap();
    assert_eq!(progress.accept(old, 100), Err(Error::Stale));
    let fresh = progress.select(100).unwrap();
    assert_ne!(fresh.occurrence, old.occurrence);
    assert_eq!(progress.through(), 10);
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
        assert_eq!(progress.through(), 300);
        let fresh = progress.select(400).unwrap();
        progress.block(fresh).unwrap();
        assert_eq!(progress.reconcile(old), Err(Error::Stale));
        assert_eq!(progress.select(500), Err(Error::RecoveryRequired));
        progress.reconcile(fresh).unwrap();
        assert_eq!(progress.through(), 400);
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
    assert_eq!(progress.through(), 200);
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
    assert_eq!(progress.through(), 0);
    assert_eq!(progress.complete(selected), Err(Error::Exhausted));
}
