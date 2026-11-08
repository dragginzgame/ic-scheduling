//! Small application-owned consumers using only the public Jobs API.
//!
//! Byte replacement models an atomic commit for fault injection; a host fixture
//! reopens actual stable cells on VectorMemory. These native fixtures do not
//! model IC message rollback, installed upgrades or live timers.

use std::io;

use ic_jobs::{Job, JobError, JobId, JobState, MissedRunPolicy, Outcome, RetryPolicy, Schedule};
use ic_storage::adapter::StableCell;
use ic_storage::{
    AllocationGrants, GenericAllocationPolicy, MemoryAuthority, MemoryRequest, MemoryRuntime,
    RuntimeOpenError, SealedDeclarationSnapshot, StorageConfig, VectorMemory,
};
mod consumer_support;

use consumer_support::{
    Destination, Maintenance, MaintenanceSnapshot, Notification, NotificationSnapshot,
    Notifications, Result, Wake,
};

#[test]
fn host_pool_cells_reopen_committed_jobs_without_replaying_blocked_effects() -> Result<()> {
    let requests = [
        Notifications::memory_request()?,
        Maintenance::memory_request()?,
    ];
    let declarations = SealedDeclarationSnapshot::new(requests.to_vec())?;
    let grants = AllocationGrants::new(vec![
        MemoryAuthority::new("test-notifications", "test.notifications.")?,
        MemoryAuthority::new("test-maintenance", "test.maintenance.")?,
    ])?;
    let config = StorageConfig::default();
    for uncertain in [false, true] {
        let backing = VectorMemory::default();
        let mut sender = Notifications::new()?;
        sender.enqueue(
            JobId(3),
            200,
            Notification {
                recipient: "bob".into(),
                body: "Later".into(),
            },
        )?;
        let delivery = sender.dispatch_due(100)?.delivery.unwrap();
        if uncertain {
            sender.finish(delivery.attempt, 101, Outcome::Uncertain)?;
        }
        let mut maintenance = Maintenance::new(MissedRunPolicy::Skip)?;
        maintenance.wake(125)?;
        let ids;
        {
            let mut host = MemoryRuntime::new_with_config(backing.clone(), config)?;
            host.bootstrap(&declarations, &grants, &GenericAllocationPolicy)?;
            ids = [
                host.allocation_id(requests[0].stable_key()).unwrap(),
                host.allocation_id(requests[1].stable_key()).unwrap(),
            ];
            assert_ne!(ids[0], ids[1]);
            for (request, bytes) in requests
                .iter()
                .zip([&sender.store.bytes, &maintenance.store.bytes])
            {
                // The host opens only after commitment. Components verify their
                // requirements without bootstrapping or selecting physical IDs.
                host.verify_authority(
                    &SealedDeclarationSnapshot::new(vec![request.clone()])?,
                    request.authority(),
                )?;
                StableCell::create(host.open_memory(request.stable_key())?, bytes.clone())?;
            }
        }
        drop(sender);
        drop(maintenance);
        // A foreign claim cannot commit or open either store, nor change bytes.
        let before = backing.borrow().clone();
        let foreign = SealedDeclarationSnapshot::new(vec![MemoryRequest::new(
            "test-maintenance",
            requests[0].stable_key().as_str(),
        )?])?;
        {
            let mut rejected = MemoryRuntime::new_with_config(backing.clone(), config)?;
            assert!(
                rejected
                    .bootstrap(&foreign, &grants, &GenericAllocationPolicy)
                    .is_err()
            );
            assert!(matches!(
                rejected.open_memory(requests[0].stable_key()),
                Err(RuntimeOpenError::NotBootstrapped)
            ));
        }
        assert_eq!(*backing.borrow(), before);
        // Discard runtime, cells and consumer state. Reopen retained cells from
        // the same backing, including the original payload and blocked attempt.
        for reopen in 0..2 {
            let mut host = MemoryRuntime::new_with_config(backing.clone(), config)?;
            host.bootstrap(&declarations, &grants, &GenericAllocationPolicy)?;
            assert_eq!(
                host.allocation_id(requests[0].stable_key()).unwrap(),
                ids[0]
            );
            assert_eq!(
                host.allocation_id(requests[1].stable_key()).unwrap(),
                ids[1]
            );
            let mut queue = StableCell::<Vec<u8>, _>::load(
                host.open_memory(requests[0].stable_key())?,
                24 * 1024,
            )?;
            let state = StableCell::<Vec<u8>, _>::load(
                host.open_memory(requests[1].stable_key())?,
                24 * 1024,
            )?;
            let mut restored = Notifications::from_bytes(queue.get().clone());
            let restored_maintenance = Maintenance::from_bytes(state.get().clone());
            assert_eq!(restored.outstanding()?, vec![delivery.clone()]);
            assert_eq!(restored.dispatch_due(150)?.delivery, None);
            assert!(!restored.reconcile(delivery.attempt, &Destination::default(), 150)?);
            let maintenance_snapshot = restored_maintenance.load()?;
            assert_eq!(maintenance_snapshot.job.next_due_ns(), Some(130));
            assert_eq!(maintenance_snapshot.completed_runs, 1);
            assert_eq!(
                restored.next_due_ns()?,
                if reopen == 0 { Some(200) } else { None }
            );
            if reopen == 0 {
                restored.cancel(JobId(3), 150)?;
                queue.set(restored.store.bytes);
                // Unrelated consumer bytes remain intact after the queue commit.
                assert_eq!(state.get(), &restored_maintenance.store.bytes);
            }
        }
    }
    Ok(())
}

#[test]
fn notification_commit_failure_never_exposes_delivery() -> Result<()> {
    let mut sender = Notifications::new()?;
    let before = sender.store.bytes.clone();
    let jobs = sender.load()?;
    sender.commit(&jobs)?;
    assert_eq!(
        sender.store.bytes, before,
        "direct serialization preserves stored bytes"
    );

    let before = sender.store.bytes.clone();
    let idle = sender.dispatch_due(99)?;
    assert_eq!(idle.delivery, None);
    assert_eq!(idle.next_due_ns, Some(100));
    sender.store.writes_before_failure = Some(0);
    assert!(sender.dispatch_due(100).is_err());
    assert_eq!(sender.store.bytes, before);
    assert!(sender.outstanding()?.is_empty());
    assert_eq!(sender.next_due_ns()?, Some(100));

    let dispatched = sender.dispatch_due(100)?;
    assert_eq!(dispatched.next_due_ns, None);
    let delivery = dispatched.delivery.unwrap();
    assert_eq!(sender.outstanding()?, vec![delivery]);
    assert_eq!(sender.next_due_ns()?, None);
    Ok(())
}

#[test]
fn interrupted_delivery_and_failed_commits_reconcile_without_redelivery() -> Result<()> {
    for (delivered, failed_result_commit) in [(false, false), (true, false), (true, true)] {
        let mut sender = Notifications::new()?;
        let mut destination = Destination::default();
        let delivery = sender.dispatch_due(100)?.delivery.unwrap();
        if delivered {
            assert!(destination.deliver(&delivery));
        }
        if failed_result_commit {
            sender.store.writes_before_failure = Some(0);
            assert!(
                sender
                    .finish(delivery.attempt, 101, Outcome::Success)
                    .is_err()
            );
        }

        // Only committed bytes survive reconstruction; the destination is separate.
        let mut restored = Notifications::from_bytes(sender.store.bytes);
        assert_eq!(restored.outstanding()?, vec![delivery.clone()]);
        assert_eq!(restored.dispatch_due(200)?.delivery, None);
        restored.finish(delivery.attempt, 200, Outcome::Uncertain)?;
        assert_eq!(restored.dispatch_due(201)?.delivery, None);
        assert!(!restored.reconcile(delivery.attempt, &Destination::default(), 201)?);
        assert_eq!(
            restored.load()?[0].job.record().state,
            JobState::Uncertain { started_ns: 100 }
        );
        if !delivered {
            assert_eq!(restored.dispatch_due(1000)?.delivery, None);
            assert_eq!(destination.calls, 0);
            continue;
        }
        let unresolved = restored.store.bytes.clone();
        restored.store.writes_before_failure = Some(0);
        assert!(
            restored
                .reconcile(delivery.attempt, &destination, 202)
                .is_err()
        );
        assert_eq!(restored.store.bytes, unresolved);
        assert_eq!(restored.dispatch_due(202)?.delivery, None);
        assert!(restored.reconcile(delivery.attempt, &destination, 202)?);
        assert_eq!(restored.load()?[0].job.record().state, JobState::Completed);
        assert_eq!(restored.dispatch_due(300)?.delivery, None);
        assert_eq!(destination.calls, 1);
        assert_eq!(destination.receipts.len(), 1);
    }
    Ok(())
}

#[test]
fn known_rejection_retries_same_payload_and_rejects_old_reply() -> Result<()> {
    let mut sender = Notifications::new()?;
    let mut destination = Destination {
        reject_next: true,
        ..Destination::default()
    };
    let first = sender.dispatch_due(100)?.delivery.unwrap();
    assert!(!destination.deliver(&first));
    assert!(destination.receipts.is_empty());
    sender.finish(first.attempt, 101, Outcome::RetryableFailure)?;
    assert_eq!(sender.next_due_ns()?, Some(111));
    assert_eq!(sender.dispatch_due(110)?.delivery, None);
    let second = sender.dispatch_due(111)?.delivery.unwrap();
    assert_eq!(first.attempt.execution, second.attempt.execution);
    assert_ne!(first.attempt.sequence, second.attempt.sequence);
    assert_eq!(first.payload, second.payload);

    let before = sender.store.bytes.clone();
    let stale = sender
        .finish(first.attempt, 112, Outcome::Success)
        .unwrap_err();
    assert_eq!(
        stale.downcast_ref::<JobError>(),
        Some(&JobError::StaleAttempt)
    );
    assert_eq!(sender.store.bytes, before);
    assert!(destination.deliver(&second));
    destination.reject_next = true;
    assert!(destination.deliver(&second)); // Destination-side duplicate suppression.
    sender.finish(second.attempt, 112, Outcome::Success)?;
    assert_eq!(destination.receipts.len(), 1);
    assert_eq!(sender.next_due_ns()?, None);
    Ok(())
}

#[test]
fn maintenance_reconstruction_bounds_catch_up_and_rebuilds_deadlines() -> Result<()> {
    let maintenance = Maintenance::new(MissedRunPolicy::CatchUp)?;
    let mut restored = Maintenance::from_bytes(maintenance.store.bytes);
    let early = restored.wake(99)?;
    assert_eq!(
        early,
        Wake {
            completed: 0,
            next_due_ns: Some(100)
        }
    );
    let first = restored.wake(135)?;
    assert_eq!(
        first,
        Wake {
            completed: 2,
            next_due_ns: Some(120)
        }
    );
    let snapshot = restored.load()?;
    assert_eq!(snapshot.completed_runs, 2);
    assert_eq!(snapshot.expires_ns, vec![200]);
    let second = restored.wake(135)?;
    assert_eq!(
        second,
        Wake {
            completed: 2,
            next_due_ns: Some(140)
        }
    );
    assert_eq!(restored.load()?.completed_runs, 4);
    {
        use ic_timers::{TimerCompletionOutcome, WatchdogDecision};
        assert_eq!(
            early.timer_result().completion().outcome(),
            TimerCompletionOutcome::NoWork
        );
        assert_eq!(first.timer_result().completion().work_count(), 2);
        assert_eq!(
            first.timer_result().completion().outcome(),
            TimerCompletionOutcome::Success
        );
        assert_eq!(
            early.timer_result().decision(),
            WatchdogDecision::ScheduleAt(100)
        );
        assert_eq!(
            first.timer_result().decision(),
            WatchdogDecision::ScheduleAt(120)
        );
        assert_eq!(
            second.timer_result().decision(),
            WatchdogDecision::ScheduleAt(140)
        );
    }
    restored.cancel(136)?;
    let stopped = restored.wake(200)?;
    assert_eq!(
        stopped,
        Wake {
            completed: 0,
            next_due_ns: None
        }
    );
    assert_eq!(
        stopped.timer_result().decision(),
        ic_timers::WatchdogDecision::Stop
    );
    Ok(())
}

#[test]
fn local_maintenance_effect_rolls_back_with_failed_commit_and_skip_avoids_backlog() -> Result<()> {
    let mut maintenance = Maintenance::new(MissedRunPolicy::Skip)?;
    let before = maintenance.store.bytes.clone();
    maintenance.store.writes_before_failure = Some(0);
    assert!(maintenance.wake(135).is_err());
    assert_eq!(maintenance.store.bytes, before);
    let snapshot = maintenance.load()?;
    assert_eq!(snapshot.completed_runs, 0);
    assert_eq!(snapshot.expires_ns, vec![90, 100, 125, 200]);
    assert_eq!(
        maintenance.wake(135)?,
        Wake {
            completed: 1,
            next_due_ns: Some(140)
        }
    );
    assert_eq!(maintenance.load()?.expires_ns, vec![200]);
    Ok(())
}

#[test]
fn maintenance_rejects_corrupt_records_before_effects() -> Result<()> {
    let mut maintenance = Maintenance::new(MissedRunPolicy::Skip)?;
    let mut snapshot: MaintenanceSnapshot = maintenance.store.read()?;
    snapshot.job.state = JobState::Running;
    maintenance.store.write(&snapshot)?;
    let before = maintenance.store.bytes.clone();
    let error = maintenance.wake(100).unwrap_err();
    assert_eq!(
        error.downcast_ref::<JobError>(),
        Some(&JobError::InvalidRecord)
    );
    assert_eq!(maintenance.store.bytes, before);
    Ok(())
}

#[test]
fn shared_notification_wakeup_preserves_order_and_progress_past_unresolved_work() -> Result<()> {
    let mut sender = Notifications::new()?;
    for (id, at_ns) in [(2, 90), (3, 95), (4, 90)] {
        sender.enqueue(
            JobId(id),
            at_ns,
            Notification {
                recipient: format!("recipient-{id}"),
                body: format!("report-{id}"),
            },
        )?;
    }
    assert_eq!(sender.next_due_ns()?, Some(90));
    assert_eq!(sender.dispatch_due(89)?.delivery, None);
    let first = sender.dispatch_due(90)?.delivery.unwrap();
    assert_eq!(first.attempt.execution.job, JobId(2));

    // Reconstruct the whole bounded queue with job 2's intent still unresolved.
    let mut restored = Notifications::from_bytes(sender.store.bytes);
    let mut destination = Destination::default();
    let tied = restored.dispatch_due(90)?.delivery.unwrap();
    assert_eq!(tied.attempt.execution.job, JobId(4));
    restored.finish(first.attempt, 91, Outcome::Uncertain)?;
    assert!(destination.deliver(&tied));
    restored.finish(tied.attempt, 91, Outcome::Success)?;
    assert_eq!(restored.next_due_ns()?, Some(95));

    let before = restored.store.bytes.clone();
    assert!(restored.cancel(JobId(2), 91).is_err());
    assert_eq!(restored.store.bytes, before);
    restored.cancel(JobId(3), 91)?;
    assert_eq!(restored.next_due_ns()?, Some(100));
    assert_eq!(restored.dispatch_due(99)?.delivery, None);
    let last = restored.dispatch_due(100)?.delivery.unwrap();
    assert_eq!(last.attempt.execution.job, JobId(1));
    assert_eq!(last.payload.recipient, "alice");
    assert!(destination.deliver(&last));
    restored.finish(last.attempt, 101, Outcome::Success)?;
    assert_eq!(restored.next_due_ns()?, None);
    assert_eq!(restored.dispatch_due(1000)?.delivery, None);
    assert_eq!(restored.outstanding()?, vec![first.clone()]);
    assert!(!restored.reconcile(first.attempt, &destination, 1000)?);
    assert_eq!(destination.calls, 2);
    assert_eq!(destination.receipts.len(), 2);
    Ok(())
}

#[test]
fn notification_queue_rejects_reused_identities_and_excess_work() -> Result<()> {
    let mut sender = Notifications::new()?;
    let payload = sender.load()?[0].payload.clone();
    let before = sender.store.bytes.clone();
    for (id, body) in [(1, "duplicate".into()), (2, "x".repeat(4097))] {
        assert!(
            sender
                .enqueue(
                    JobId(id),
                    200,
                    Notification {
                        recipient: "bob".into(),
                        body,
                    }
                )
                .is_err()
        );
        assert_eq!(sender.store.bytes, before);
        assert_eq!(sender.next_due_ns()?, Some(100));
    }
    for id in 2..=4 {
        sender.enqueue(JobId(id), 200, payload.clone())?;
    }
    let before = sender.store.bytes.clone();
    assert!(sender.enqueue(JobId(5), 200, payload.clone()).is_err());
    assert_eq!(sender.store.bytes, before);

    let mut snapshots: Vec<NotificationSnapshot> = sender.store.read()?;
    snapshots.push(NotificationSnapshot {
        job: Job::new(JobId(5), Schedule::Once { at_ns: 200 }, RetryPolicy::NONE).record(),
        payload,
    });
    sender.store.write(&snapshots)?;
    let before = sender.store.bytes.clone();
    assert!(sender.dispatch_due(100).is_err());
    assert_eq!(sender.store.bytes, before);
    snapshots.pop();

    // Reject duplicate retained identities independently at the restore boundary.
    snapshots[3].job = snapshots[0].job;
    sender.store.write(&snapshots)?;
    let before = sender.store.bytes.clone();
    assert!(sender.dispatch_due(100).is_err());
    assert_eq!(sender.store.bytes, before);

    // Both due and future corruption block the whole queue before effects.
    snapshots[3].job =
        Job::new(JobId(4), Schedule::Once { at_ns: 200 }, RetryPolicy::NONE).record();
    for index in [0, 3] {
        let valid = snapshots[index].job;
        snapshots[index].job.state = JobState::Completed;
        sender.store.write(&snapshots)?;
        let before = sender.store.bytes.clone();
        let error = sender.dispatch_due(100).unwrap_err();
        assert_eq!(
            error.downcast_ref::<JobError>(),
            Some(&JobError::InvalidRecord)
        );
        assert_eq!(sender.store.bytes, before);
        snapshots[index].job = valid;
    }
    Ok(())
}

#[test]
fn lost_rejection_reply_uses_exact_evidence_and_retains_retry_budget() -> Result<()> {
    for second_succeeds in [false, true] {
        let mut sender = Notifications::new()?;
        let mut destination = Destination {
            reject_next: true,
            ..Destination::default()
        };
        let first = sender.dispatch_due(100)?.delivery.unwrap();
        assert!(!destination.deliver(&first));
        // The receiver retains its rejection even if an old delivery is repeated.
        assert!(!destination.deliver(&first));
        let mut restored = Notifications::from_bytes(sender.store.bytes);
        assert_eq!(restored.dispatch_due(101)?.delivery, None);
        restored.finish(first.attempt, 101, Outcome::Uncertain)?;

        let before = restored.store.bytes.clone();
        assert!(!restored.reconcile(first.attempt, &Destination::default(), 102)?);
        assert_eq!(restored.store.bytes, before);
        restored.store.writes_before_failure = Some(0);
        assert!(
            restored
                .reconcile(first.attempt, &destination, 102)
                .is_err()
        );
        assert_eq!(restored.store.bytes, before);
        assert_eq!(restored.dispatch_due(102)?.delivery, None);
        assert!(restored.reconcile(first.attempt, &destination, 102)?);
        assert_eq!(restored.next_due_ns()?, Some(112));
        assert_eq!(restored.dispatch_due(111)?.delivery, None);
        let second = restored.dispatch_due(112)?.delivery.unwrap();
        assert_eq!(second.attempt.execution, first.attempt.execution);
        assert_ne!(second.attempt.sequence, first.attempt.sequence);
        assert_eq!(second.payload, first.payload);
        destination.reject_next = !second_succeeds;
        assert_eq!(destination.deliver(&second), second_succeeds);
        restored.finish(second.attempt, 113, Outcome::Uncertain)?;

        let before = restored.store.bytes.clone();
        let error = restored
            .reconcile(first.attempt, &destination, 114)
            .unwrap_err();
        assert_eq!(
            error.downcast_ref::<JobError>(),
            Some(&JobError::StaleAttempt)
        );
        assert_eq!(restored.store.bytes, before);
        assert!(restored.reconcile(second.attempt, &destination, 114)?);
        assert_eq!(
            restored.load()?[0].job.record().state,
            if second_succeeds {
                JobState::Completed
            } else {
                JobState::Failed
            }
        );
        assert_eq!(restored.next_due_ns()?, None);
        assert_eq!(restored.dispatch_due(1000)?.delivery, None);
        assert!(restored.outstanding()?.is_empty());
        assert_eq!(destination.receipts.len(), usize::from(second_succeeds));
    }
    Ok(())
}

#[test]
fn reconciliation_requires_matching_payload_and_prefers_success_evidence() -> Result<()> {
    let mut sender = Notifications::new()?;
    let delivery = sender.dispatch_due(100)?.delivery.unwrap();
    sender.finish(delivery.attempt, 101, Outcome::Uncertain)?;
    let different = Notification {
        body: "another effect".into(),
        ..delivery.payload.clone()
    };
    let mut destination = Destination {
        receipts: vec![(delivery.attempt.execution, different.clone())],
        rejections: vec![(delivery.attempt, different)],
        ..Destination::default()
    };
    let before = sender.store.bytes.clone();
    assert!(!sender.reconcile(delivery.attempt, &destination, 102)?);
    assert_eq!(sender.store.bytes, before);
    destination
        .receipts
        .push((delivery.attempt.execution, delivery.payload.clone()));
    destination
        .rejections
        .push((delivery.attempt, delivery.payload.clone()));
    assert!(sender.reconcile(delivery.attempt, &destination, 102)?);
    assert_eq!(sender.load()?[0].job.record().state, JobState::Completed);
    assert_eq!(destination.calls, 0);
    // Evidence queries may find a later exact match; delivery must still refuse
    // an identity whose first retained payload contradicts the request.
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            destination.deliver(&delivery)
        }))
        .is_err()
    );
    Ok(())
}

#[test]
fn maintenance_resumes_after_partial_batch_commit_without_repeating_work() -> Result<()> {
    let mut maintenance = Maintenance::new(MissedRunPolicy::CatchUp)?;
    maintenance.store.writes_before_failure = Some(1);
    let error = maintenance.wake(135).unwrap_err();
    assert_eq!(
        error.downcast_ref::<io::Error>().unwrap().kind(),
        io::ErrorKind::Other
    );
    let snapshot = maintenance.load()?;
    assert_eq!(snapshot.completed_runs, 1);
    assert_eq!(snapshot.expires_ns, vec![200]);
    assert_eq!(snapshot.job.record().occurrence, 1);
    assert_eq!(snapshot.job.record().sequence, 1);
    assert_eq!(snapshot.job.next_due_ns(), Some(110));

    let mut restored = Maintenance::from_bytes(maintenance.store.bytes);
    assert_eq!(
        restored.wake(135)?,
        Wake {
            completed: 2,
            next_due_ns: Some(130)
        }
    );
    let snapshot = restored.load()?;
    assert_eq!(snapshot.completed_runs, 3);
    assert_eq!(snapshot.job.record().occurrence, 3);
    assert_eq!(snapshot.job.record().sequence, 3);
    assert_eq!(
        restored.wake(135)?,
        Wake {
            completed: 1,
            next_due_ns: Some(140)
        }
    );
    let snapshot = restored.load()?;
    assert_eq!(snapshot.completed_runs, 4);
    assert_eq!(snapshot.expires_ns, vec![200]);
    Ok(())
}

#[test]
fn failed_queue_mutations_preserve_committed_deadlines() -> Result<()> {
    let payload = || Notification {
        recipient: "bob".into(),
        body: "later".into(),
    };
    for cancel in [false, true] {
        let mut sender = Notifications::new()?;
        let before = sender.store.bytes.clone();
        let mutate = |sender: &mut Notifications| {
            if cancel {
                sender.cancel(JobId(1), 100)
            } else {
                sender.enqueue(JobId(3), 80, payload())
            }
        };
        sender.store.writes_before_failure = Some(0);
        let error = mutate(&mut sender).unwrap_err();
        assert_eq!(
            error.downcast_ref::<io::Error>().unwrap().kind(),
            io::ErrorKind::Other
        );
        assert_eq!(sender.store.bytes, before);
        assert!(sender.outstanding()?.is_empty());
        assert_eq!(sender.next_due_ns()?, Some(100));
        mutate(&mut sender)?;
        assert_eq!(sender.next_due_ns()?, if cancel { None } else { Some(80) });
    }
    Ok(())
}
