//! Small application-owned consumers using only the public Jobs API.
//!
//! Byte replacement models an atomic commit for fault injection; a host fixture
//! reopens actual stable cells on VectorMemory. These native fixtures do not
//! model IC message rollback, installed upgrades or live timers.

use std::io;

use ic_jobs::{
    Job, JobError, JobId, JobRecord, JobState, MissedRunPolicy, Outcome, RetryPolicy, Schedule,
};
use ic_storage::adapter::StableCell;
use ic_storage::{
    AllocationGrants, GenericAllocationPolicy, MemoryAuthority, MemoryRequest, MemoryRuntime,
    RuntimeOpenError, SealedDeclarationSnapshot, StorageConfig, VectorMemory,
};
use serde::{Deserialize, Serialize};
mod consumer_support;

use consumer_support::{DeliveryFixture, Evidence, Result, Store, assert_failed_commit};

// Minimal local-effect transaction. Scheduling policy remains in Job, and this
// fixture owns neither cache expiry nor a second batch/scheduler implementation.
#[derive(Deserialize, Serialize)]
struct LocalEffect<J = JobRecord> {
    job: J,
    completed: u64,
}

fn local_store() -> Result<Store> {
    Store::new(&LocalEffect {
        job: Job::new(
            JobId(2),
            Schedule::FixedRate {
                first_at_ns: 100,
                every_ns: std::num::NonZeroU64::new(10).unwrap(),
                missed: MissedRunPolicy::CatchUp,
            },
            RetryPolicy::NONE,
        ),
        completed: 0,
    })
}

fn load_local(store: &Store) -> Result<LocalEffect<Job>> {
    let snapshot: LocalEffect = store.read()?;
    Ok(LocalEffect {
        job: Job::restore(snapshot.job)?,
        completed: snapshot.completed,
    })
}

fn complete_local(store: &mut Store, now: u64) -> Result<()> {
    let mut snapshot = load_local(store)?;
    let attempt = snapshot.job.start(now)?;
    snapshot.completed = snapshot
        .completed
        .checked_add(1)
        .ok_or_else(|| io::Error::other("local counter exhausted"))?;
    snapshot.job.finish(attempt, now, Outcome::Success)?;
    store.write(&snapshot)
}

#[test]
fn local_effect_commits_resume_without_repeating_previously_committed_work() -> Result<()> {
    for committed in [0, 1] {
        let mut store = local_store()?;
        store.writes_before_failure = Some(committed);
        for _ in 0..committed {
            complete_local(&mut store, 125)?;
        }
        let before = store.bytes.clone();
        let error = complete_local(&mut store, 125).unwrap_err();
        assert_eq!(
            error.downcast_ref::<io::Error>().unwrap().kind(),
            io::ErrorKind::Other
        );
        assert_eq!(store.bytes, before);
        let mut reopened = Store::from_bytes(store.bytes);
        let snapshot = load_local(&reopened)?;
        assert_eq!(snapshot.completed, committed as u64);
        assert_eq!(snapshot.job.record().occurrence, committed as u64);
        assert_eq!(snapshot.job.record().sequence, committed as u64);
        assert_eq!(
            snapshot.job.next_due_ns(),
            Some(100 + 10 * committed as u64)
        );
        complete_local(&mut reopened, 125)?;
        let snapshot = load_local(&reopened)?;
        assert_eq!(snapshot.completed, committed as u64 + 1);
        assert_eq!(snapshot.job.record().occurrence, committed as u64 + 1);
        assert_eq!(
            snapshot.job.next_due_ns(),
            Some(110 + 10 * committed as u64)
        );
    }
    Ok(())
}

#[test]
fn local_effect_restore_refuses_corruption_before_commit() -> Result<()> {
    let mut store = local_store()?;
    let mut snapshot: LocalEffect = store.read()?;
    snapshot.job.state = JobState::Running;
    store.write(&snapshot)?;
    let before = store.bytes.clone();
    let error = complete_local(&mut store, 100).unwrap_err();
    assert_eq!(
        error.downcast_ref::<JobError>(),
        Some(&JobError::InvalidRecord)
    );
    assert_eq!(store.bytes, before);
    Ok(())
}

#[test]
fn host_pool_cells_reopen_committed_jobs_without_replaying_blocked_effects() -> Result<()> {
    let requests = [
        DeliveryFixture::memory_request()?,
        MemoryRequest::new("test-local-effect", "test.local.effect.v1")?,
    ];
    let declarations = SealedDeclarationSnapshot::new(requests.to_vec())?;
    let grants = AllocationGrants::new(vec![
        MemoryAuthority::new("test-notifications", "test.notifications.")?,
        MemoryAuthority::new("test-local-effect", "test.local.")?,
    ])?;
    let config = StorageConfig::default();
    for uncertain in [false, true] {
        let backing = VectorMemory::default();
        let mut sender = DeliveryFixture::new()?;
        let delivery = sender.dispatch_due(100)?.unwrap();
        if uncertain {
            sender.finish(delivery.attempt, 101, Outcome::Uncertain)?;
        }
        let mut local = local_store()?;
        complete_local(&mut local, 125)?;
        let ids;
        {
            let mut host = MemoryRuntime::new_with_config(backing.clone(), config)?;
            host.bootstrap(&declarations, &grants, &GenericAllocationPolicy)?;
            ids = [
                host.allocation_id(requests[0].stable_key()).unwrap(),
                host.allocation_id(requests[1].stable_key()).unwrap(),
            ];
            assert_ne!(ids[0], ids[1]);
            for (request, bytes) in requests.iter().zip([&sender.store.bytes, &local.bytes]) {
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
        drop(local);
        // A foreign claim cannot commit or open either store, nor change bytes.
        let before = backing.borrow().clone();
        let foreign = SealedDeclarationSnapshot::new(vec![MemoryRequest::new(
            "test-local-effect",
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
            let queue = StableCell::<Vec<u8>, _>::load(
                host.open_memory(requests[0].stable_key())?,
                24 * 1024,
            )?;
            let mut state = StableCell::<Vec<u8>, _>::load(
                host.open_memory(requests[1].stable_key())?,
                24 * 1024,
            )?;
            let mut restored = DeliveryFixture::from_bytes(queue.get().clone());
            let mut local = Store::from_bytes(state.get().clone());
            assert_eq!(
                restored.load()?.job.active_attempt(),
                Some(delivery.attempt)
            );
            assert_eq!(restored.load()?.payload, delivery.payload);
            assert_eq!(restored.dispatch_due(150)?, None);
            assert!(!restored.reconcile(delivery.attempt, &Evidence::default(), 150)?);
            let mut local_snapshot = load_local(&local)?;
            assert_eq!(
                local_snapshot.job.next_due_ns(),
                (reopen == 0).then_some(110)
            );
            assert_eq!(local_snapshot.completed, 1);
            assert_eq!(restored.load()?.job.next_due_ns(), None);
            if reopen == 0 {
                local_snapshot.job.cancel(150)?;
                local.write(&local_snapshot)?;
                state.set(local.bytes);
                assert_eq!(queue.get(), &restored.store.bytes);
            }
        }
    }
    Ok(())
}

#[test]
fn notification_commit_failure_never_exposes_delivery() -> Result<()> {
    let mut sender = DeliveryFixture::new()?;
    let before = sender.store.bytes.clone();
    let snapshot = sender.load()?;
    sender.store.write(&snapshot)?;
    assert_eq!(
        sender.store.bytes, before,
        "direct serialization preserves stored bytes"
    );

    let before = sender.store.bytes.clone();
    let idle = sender.dispatch_due(99)?;
    assert_eq!(idle, None);
    assert_eq!(sender.load()?.job.next_due_ns(), Some(100));
    assert_failed_commit!(sender.store, sender.dispatch_due(100));
    assert_eq!(sender.store.bytes, before);
    assert!(sender.load()?.job.active_attempt().is_none());
    assert_eq!(sender.load()?.job.next_due_ns(), Some(100));

    let dispatched = sender.dispatch_due(100)?;
    let delivery = dispatched.unwrap();
    assert_eq!(sender.load()?.job.active_attempt(), Some(delivery.attempt));
    assert_eq!(sender.load()?.job.next_due_ns(), None);
    Ok(())
}

#[test]
fn interrupted_delivery_and_failed_commits_reconcile_without_redelivery() -> Result<()> {
    for (delivered, failed_result_commit) in [(false, false), (true, false), (true, true)] {
        let mut sender = DeliveryFixture::new()?;
        let mut destination = Evidence::default();
        let delivery = sender.dispatch_due(100)?.unwrap();
        if delivered {
            destination
                .receipts
                .push((delivery.attempt.execution, delivery.payload.clone()));
        }
        if failed_result_commit {
            assert_failed_commit!(
                sender.store,
                sender.finish(delivery.attempt, 101, Outcome::Success)
            );
        }

        // Only committed bytes survive reconstruction; the destination is separate.
        let mut restored = DeliveryFixture::from_bytes(sender.store.bytes);
        assert_eq!(
            restored.load()?.job.active_attempt(),
            Some(delivery.attempt)
        );
        assert_eq!(restored.load()?.payload, delivery.payload);
        assert_eq!(restored.dispatch_due(200)?, None);
        restored.finish(delivery.attempt, 200, Outcome::Uncertain)?;
        assert_eq!(restored.dispatch_due(201)?, None);
        assert!(!restored.reconcile(delivery.attempt, &Evidence::default(), 201)?);
        assert_eq!(
            restored.load()?.job.record().state,
            JobState::Uncertain { started_ns: 100 }
        );
        if !delivered {
            assert_eq!(restored.dispatch_due(1000)?, None);
            continue;
        }
        assert_failed_commit!(
            restored.store,
            restored.reconcile(delivery.attempt, &destination, 202)
        );
        assert_eq!(restored.dispatch_due(202)?, None);
        assert!(restored.reconcile(delivery.attempt, &destination, 202)?);
        assert_eq!(restored.load()?.job.record().state, JobState::Completed);
        assert_eq!(restored.dispatch_due(300)?, None);
        assert_eq!(destination.receipts.len(), 1);
    }
    Ok(())
}

#[test]
fn lost_rejection_reply_uses_exact_evidence_and_retains_retry_budget() -> Result<()> {
    for second_succeeds in [false, true] {
        let mut sender = DeliveryFixture::new()?;
        let first = sender.dispatch_due(100)?.unwrap();
        let mut destination = Evidence {
            rejections: vec![(first.attempt, first.payload.clone())],
            ..Evidence::default()
        };
        let mut restored = DeliveryFixture::from_bytes(sender.store.bytes);
        assert_eq!(restored.dispatch_due(101)?, None);
        restored.finish(first.attempt, 101, Outcome::Uncertain)?;

        let before = restored.store.bytes.clone();
        assert!(!restored.reconcile(first.attempt, &Evidence::default(), 102)?);
        assert_eq!(restored.store.bytes, before);
        assert_failed_commit!(
            restored.store,
            restored.reconcile(first.attempt, &destination, 102)
        );
        assert_eq!(restored.dispatch_due(102)?, None);
        assert!(restored.reconcile(first.attempt, &destination, 102)?);
        assert_eq!(restored.load()?.job.next_due_ns(), Some(112));
        assert_eq!(restored.dispatch_due(111)?, None);
        let second = restored.dispatch_due(112)?.unwrap();
        assert_eq!(second.attempt.execution, first.attempt.execution);
        assert_ne!(second.attempt.sequence, first.attempt.sequence);
        assert_eq!(second.payload, first.payload);
        if second_succeeds {
            destination
                .receipts
                .push((second.attempt.execution, second.payload.clone()));
        } else {
            destination
                .rejections
                .push((second.attempt, second.payload.clone()));
        }
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
            restored.load()?.job.record().state,
            if second_succeeds {
                JobState::Completed
            } else {
                JobState::Failed
            }
        );
        assert_eq!(restored.load()?.job.next_due_ns(), None);
        assert_eq!(restored.dispatch_due(1000)?, None);
        assert!(restored.load()?.job.active_attempt().is_none());
        assert_eq!(destination.receipts.len(), usize::from(second_succeeds));
    }
    Ok(())
}

#[test]
fn reconciliation_requires_matching_payload_and_prefers_success_evidence() -> Result<()> {
    let mut sender = DeliveryFixture::new()?;
    let delivery = sender.dispatch_due(100)?.unwrap();
    sender.finish(delivery.attempt, 101, Outcome::Uncertain)?;
    let different = "another effect".to_owned();
    let mut destination = Evidence {
        receipts: vec![(delivery.attempt.execution, different.clone())],
        rejections: vec![(delivery.attempt, different)],
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
    assert_eq!(sender.load()?.job.record().state, JobState::Completed);
    Ok(())
}

#[test]
fn failed_management_commits_preserve_committed_deadlines() -> Result<()> {
    for cancel in [false, true] {
        let mut sender = DeliveryFixture::new()?;
        let mutate = |sender: &mut DeliveryFixture| {
            sender.transition(|job| {
                if cancel {
                    job.cancel(0)
                } else {
                    job.reschedule(Schedule::Once { at_ns: 80 }, 0)
                }
            })
        };
        assert_failed_commit!(sender.store, mutate(&mut sender));
        assert!(sender.load()?.job.active_attempt().is_none());
        assert_eq!(sender.load()?.job.next_due_ns(), Some(100));
        mutate(&mut sender)?;
        assert_eq!(
            sender.load()?.job.next_due_ns(),
            if cancel { None } else { Some(80) }
        );
    }
    Ok(())
}
