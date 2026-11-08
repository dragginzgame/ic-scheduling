use super::*;
use serde::Deserialize;

#[test]
fn aliased_backing_refuses_reopen_until_the_final_handle_drops() {
    let backing = VectorMemory::default();
    let mut queue = Queue::open(MeteredMemory::new(backing.clone()), Some(config()))
        .unwrap()
        .0;
    let id = queue.create(increment(100)).unwrap().value;
    let before = queue.cell.get().as_slice().to_vec();
    // Independent metering wrappers must identify the same physical backing.
    let alias = MeteredMemory::new(backing);
    let refused = || {
        assert!(matches!(
            Queue::open(alias.clone(), None),
            Err(Error::Memory(_))
        ));
        assert_eq!(alias.counts(), Default::default());
    };
    refused();
    drop(queue);
    // A detached checked lease also retains custody after its runtime drops.
    let handle = queue_memory(alias.clone()).unwrap();
    alias.reset();
    refused();
    drop(handle);
    let reopened = Queue::open(alias, None).unwrap().0;
    assert_eq!(reopened.cell.get().as_slice(), before);
    assert_eq!(reopened.inspect(id).unwrap().id, id);
}

#[test]
fn typed_commit_preserves_canonical_snapshot_bytes_and_metadata() {
    let memory = VectorMemory::default();
    let mut queue = Queue::open(
        memory.clone(),
        Some(Init {
            manager: manager(),
            delivery_sender: Some(manager()),
        }),
    )
    .unwrap()
    .0;
    queue.create(increment(100)).unwrap();
    queue
        .create(Create {
            timing: Timing::Once(100),
            task: Task::Deliver(DeliveryPayload {
                destination: manager(),
                message: "paired payload".into(),
            }),
        })
        .unwrap();
    let delivery = queue.dispatch(100).unwrap().value.deliveries.pop().unwrap();
    queue.receive(manager(), delivery.clone()).unwrap();
    queue.finish(&delivery, 101, Outcome::Uncertain).unwrap();
    let later = queue.create(increment(200)).unwrap().value;
    queue.pause(later, 102).unwrap();
    // The raw canonical writer and typed writer must retain the same metadata,
    // payloads, receipt and blocked/terminal/paused records byte for byte.
    let stored: Snapshot = stored_records(queue.cell.get().as_slice()).unwrap();
    let expected = encode(&stored).unwrap();
    let loaded = queue.load().unwrap();
    assert_eq!(queue.commit(loaded, ()).unwrap().next_due_ns, None);
    assert_eq!(queue.cell.get().as_slice(), expected);
    drop(queue);
    let (queue, due) = Queue::open(memory, None).unwrap();
    assert_eq!(due, None);
    assert_eq!(queue.summary().unwrap(), totals(1, 1, None));
    assert_eq!(queue.outstanding(delivery.job).unwrap(), delivery);
    assert_eq!(queue.inspect(later).unwrap().status, Status::Paused);
}

#[test]
fn retired_snapshot_is_refused_without_replacing_the_cell() {
    // Exact old producer bytes remain only as a refusal/preservation fixture.
    for frozen in [
        include_bytes!("../fixtures/snapshot-075.json").as_slice(),
        include_bytes!("../fixtures/snapshot-running-015.json").as_slice(),
        include_bytes!("../fixtures/snapshot-receipt-015.json").as_slice(),
    ] {
        let bytes = frozen.to_vec();
        let backing = VectorMemory::default();
        let mut queue = fresh_queue(backing.clone());
        queue.cell.set(bytes.clone());
        drop(queue);
        for initial in [None, Some(config())] {
            assert!(matches!(
                Queue::open(backing.clone(), initial),
                Err(Error::InvalidStorage)
            ));
            assert_eq!(
                retained_cell(backing.clone(), MAX_BYTES).get().as_slice(),
                bytes
            );
        }
    }
}

// Sparse nonzero runs retain the complete backing, including manager/Cell headers.
#[derive(Deserialize)]
struct FrozenMemory {
    producer: String,
    pages: u64,
    runs: Vec<(u64, String)>,
    snapshot: String,
    #[serde(default)]
    queue_hex: Option<String>,
}

// Expand only the bounded, checked-in historical backing fixtures. This raw
// construction precedes all Storage custody and is never a runtime reader.
fn frozen_backing(frozen: &FrozenMemory, maximum_pages: u64) -> VectorMemory {
    assert!(frozen.pages <= maximum_pages);
    let backing = VectorMemory::default();
    let mut bytes = backing.borrow_mut();
    bytes.resize(frozen.pages as usize * 65_536, 0);
    let mut previous_end = 0;
    for (offset, run) in &frozen.runs {
        assert_eq!(run.len() % 2, 0);
        let start = usize::try_from(*offset).unwrap();
        let end = start.checked_add(run.len() / 2).unwrap();
        assert!(start >= previous_end && end <= bytes.len());
        for (byte, hex) in bytes[start..end]
            .iter_mut()
            .zip(run.as_bytes().as_chunks::<2>().0.iter())
        {
            *byte = u8::from_str_radix(std::str::from_utf8(hex).unwrap(), 16).unwrap();
        }
        previous_end = end;
    }
    drop(bytes);
    backing
}

fn assert_frozen_memory_refused(frozen: &FrozenMemory) {
    let backing = frozen_backing(frozen, 3);
    let memory = MeteredMemory::new(backing);
    let before = memory.backing().borrow().clone();
    for initial in [None, Some(config())] {
        assert!(matches!(
            Queue::open(memory.clone(), initial),
            Err(Error::Memory(_))
        ));
        assert!(!memory.counts().overflowed);
        assert_eq!(memory.counts().writes, 0);
        assert_eq!(memory.size(), frozen.pages);
        assert_eq!(*memory.backing().borrow(), before);
    }
}

#[test]
fn prior_memory_ledgers_are_refused_without_writes_or_reset() {
    for (encoded, producer) in [
        (
            include_str!("../fixtures/stable-image-075.json"),
            "060fb9d226389bab702132ba23f42bd0b887a690",
        ),
        (
            include_str!("../fixtures/stable-image-memory040.json"),
            "ic-memory 0.40.0",
        ),
    ] {
        let frozen: FrozenMemory = serde_json::from_str(encoded).unwrap();
        assert_eq!(frozen.producer, producer);
        // Both layers are retired. The outer Memory ledger must refuse before
        // payload admission, publishing an allocation or overwriting old bytes.
        assert!(matches!(
            decode(frozen.snapshot.as_bytes()),
            Err(Error::InvalidStorage)
        ));
        assert_frozen_memory_refused(&frozen);
    }
}

#[test]
fn memory_wire_cut_refuses_cbor_without_releasing_blocked_work() {
    let frozen: FrozenMemory =
        serde_json::from_str(include_str!("../fixtures/stable-image-memory0552.json")).unwrap();
    assert_eq!(
        frozen.producer,
        "cdb48cc9d1c3281e2b898bcfe4a19e2a673b4560 / ic-memory 0.55.2"
    );
    // Frozen producer metadata still establishes the unresolved obligation.
    // Neither retired JSON nor the outer CBOR ledger is a current reader path.
    let snapshot = serde_json::from_str::<Snapshot>(&frozen.snapshot)
        .unwrap()
        .admit()
        .unwrap();
    assert_eq!(snapshot.jobs.len(), 1);
    assert_eq!(snapshot.jobs[0].job.record().state, JobState::Running);
    assert_eq!(snapshot.next_due_ns(), None);
    assert!(matches!(
        decode(frozen.snapshot.as_bytes()),
        Err(Error::InvalidStorage)
    ));
    assert_frozen_memory_refused(&frozen);
}

#[test]
fn json_queue_cut_preserves_retired_backing_and_outstanding_effect_evidence() {
    let frozen: FrozenMemory =
        serde_json::from_str(include_str!("../fixtures/stable-image-json01749.json")).unwrap();
    assert_eq!(
        frozen.producer,
        "26108182368b3ef9687f195feaf2ce6c38021dfc / ic-memory 0.56.0 / incoming 2026-10-30 lock"
    );
    // Only this frozen-fixture assertion reads the old format. The old producer
    // cold-opened both outstanding attempts blocked before capturing these bytes.
    let snapshot = serde_json::from_str::<Snapshot>(&frozen.snapshot)
        .unwrap()
        .admit()
        .unwrap();
    assert_eq!(
        snapshot.jobs[0].job.record().state,
        JobState::Uncertain { started_ns: 10 }
    );
    assert_eq!(snapshot.jobs[1].job.record().state, JobState::Running);
    assert_eq!(snapshot.receipts.len(), 2);
    for entry in &snapshot.jobs {
        assert_eq!(
            entry.delivery().unwrap(),
            snapshot.receipts[entry.job.record().id.0 as usize - 1]
        );
    }
    assert_eq!(snapshot.next_due_ns(), None);
    assert!(matches!(
        decode(frozen.snapshot.as_bytes()),
        Err(Error::InvalidStorage)
    ));

    assert_frozen_memory_refused(&frozen);
}

#[test]
fn storage_064_refuses_the_previous_wire_installation_without_releasing_effects() {
    let frozen: FrozenMemory =
        serde_json::from_str(include_str!("../fixtures/stable-image-storage063.json")).unwrap();
    assert!(
        frozen
            .producer
            .contains("ic-memory 0.63.0 / ic-storage 0.63.0")
    );
    let hex = frozen.queue_hex.as_ref().unwrap();
    assert_eq!(hex.len() % 2, 0);
    let bytes: Vec<_> = hex
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect();
    // The application Wire schema still admits the exact old payload and keeps
    // both outstanding effects blocked. Its retired physical image cannot open.
    let snapshot = decode(&bytes).unwrap();
    assert_eq!(
        serde_json::to_value(&snapshot).unwrap(),
        serde_json::from_str::<serde_json::Value>(&frozen.snapshot).unwrap()
    );
    assert_eq!(
        snapshot.jobs[0].job.record().state,
        JobState::Uncertain { started_ns: 100 }
    );
    assert_eq!(snapshot.jobs[1].job.record().state, JobState::Running);
    assert_eq!(snapshot.next_due_ns(), None);
    assert_eq!(snapshot.receipts.len(), 2);
    for (entry, receipt) in snapshot.jobs.iter().zip(&snapshot.receipts) {
        assert_eq!(entry.delivery().unwrap(), *receipt);
    }
    assert_frozen_memory_refused(&frozen);
}

#[test]
fn storage_068_refuses_previous_cells_without_releasing_effects_or_writing() {
    let frozen: FrozenMemory =
        serde_json::from_str(include_str!("../fixtures/stable-image-storage064.json")).unwrap();
    assert!(
        frozen
            .producer
            .contains("ic-storage 0.64.0 / Runtime 0.17.1")
    );
    let expected = serde_json::from_str::<Snapshot>(&frozen.snapshot)
        .unwrap()
        .admit()
        .unwrap();
    assert_eq!(expected.jobs.len(), 2);
    assert_eq!(expected.receipts.len(), 2);
    assert_eq!(
        expected.jobs[0].job.record().state,
        JobState::Uncertain { started_ns: 100 }
    );
    assert_eq!(expected.jobs[1].job.record().state, JobState::Running);
    assert_eq!(expected.next_due_ns(), None);
    for entry in &expected.jobs {
        let delivery = entry.delivery().unwrap();
        assert!(expected.receipts.contains(&delivery));
    }
    // The frozen metadata retains the outstanding obligations. Storage 0.68
    // refuses this prior Cell framing; no production reader or reset is added.
    let backing = frozen_backing(&frozen, 4);
    let before = backing.borrow().clone();
    let memory = MeteredMemory::new(backing.clone());
    for initial in [None, Some(config())] {
        assert!(matches!(
            Queue::open(memory.clone(), initial),
            Err(Error::InvalidStorage)
        ));
        assert_eq!(memory.counts().writes, 0);
        assert_eq!(memory.counts().grows, 0);
        assert!(!memory.counts().overflowed);
        assert_eq!(*backing.borrow(), before);
    }
}

#[test]
fn bounded_wire_queue_preserves_full_history_payloads_receipts_and_schedule_shapes() {
    let backing = VectorMemory::default();
    let mut queue = fresh_queue(backing.clone());
    let mut snapshot = queue.load().unwrap();
    snapshot.next_id = u64::MAX;
    snapshot.counter = u64::MAX;
    let states = [
        JobState::Pending { due_ns: 100 },
        JobState::Paused { due_ns: 100 },
        JobState::Running,
        JobState::Uncertain { started_ns: 100 },
        JobState::Completed,
        JobState::Failed,
        JobState::Cancelled,
    ];
    for index in 0..MAX_JOBS {
        let id = u64::MAX - MAX_JOBS as u64 + index as u64;
        let schedule = if index < states.len() {
            Schedule::Once { at_ns: 100 }
        } else {
            match index % 3 {
                0 => Schedule::FixedRate {
                    first_at_ns: 100,
                    every_ns: NonZeroU64::new(10).unwrap(),
                    missed: MissedRunPolicy::Skip,
                },
                1 => Schedule::FixedRate {
                    first_at_ns: 100,
                    every_ns: NonZeroU64::new(10).unwrap(),
                    missed: MissedRunPolicy::CatchUp,
                },
                _ => Schedule::AfterCompletion {
                    first_at_ns: 100,
                    every_ns: NonZeroU64::new(10).unwrap(),
                },
            }
        };
        let mut job = Job::new(
            JobId(u128::from(id)),
            schedule,
            RetryPolicy::new(3, 5, 20).unwrap(),
        );
        if let Some(state) = states.get(index) {
            set_job_state(&mut job, *state, 100);
        }
        let payload = DeliveryPayload {
            destination: manager(),
            message: "x".repeat(MAX_MESSAGE),
        };
        snapshot.jobs.push(Entry {
            job,
            task: Task::Deliver(payload.clone()),
        });
        snapshot.receipts.push(Delivery {
            job: id,
            occurrence: u64::MAX,
            sequence: u64::MAX,
            payload,
        });
    }
    let expected = encode(&snapshot).unwrap();
    assert!(expected.len() <= MAX_BYTES);
    stored_records(&expected).unwrap();
    assert_eq!(queue.commit(snapshot, ()).unwrap().next_due_ns, Some(100));
    assert_eq!(queue.cell.get().as_slice(), expected);
    assert_eq!(queue.create(increment(200)), Err(Error::Capacity));
    drop(queue);
    let (queue, due) = Queue::open(backing, None).unwrap();
    assert_eq!(due, Some(100));
    let loaded = queue.load().unwrap();
    assert_eq!(loaded.jobs.len(), MAX_JOBS);
    assert_eq!(loaded.receipts.len(), MAX_JOBS);
    assert_eq!(loaded.counter, u64::MAX);
    assert_eq!(encode(&loaded).unwrap(), expected);
}

#[test]
fn wire_queue_refuses_wrong_identity_malformed_trailing_and_exhausted_budgets() {
    let mut queue = fresh_queue(VectorMemory::default());
    queue.create(increment(100)).unwrap();
    let original = queue.cell.get().as_slice().to_vec();
    let mut wrong_identity = original.clone();
    wrong_identity[0] ^= 1;
    let mut trailing = original.clone();
    trailing.push(0);
    for bytes in [
        vec![],
        wrong_identity,
        original[..original.len() - 1].to_vec(),
        trailing,
        vec![0; MAX_BYTES + 1],
    ] {
        assert!(matches!(decode(&bytes), Err(Error::InvalidStorage)));
        if bytes.len() <= MAX_BYTES {
            let memory = VectorMemory::default();
            let mut damaged = fresh_queue(memory.clone());
            damaged.cell.set(bytes.clone());
            assert_eq!(damaged.dispatch(100), Err(Error::InvalidStorage));
            assert_eq!(damaged.cell.get().as_slice(), bytes);
            drop(damaged);
            assert!(matches!(
                Queue::open(memory.clone(), None),
                Err(Error::InvalidStorage)
            ));
            assert_eq!(retained_cell(memory, MAX_BYTES).get().as_slice(), bytes);
        }
    }
    let payload = original.strip_prefix(FORMAT).unwrap();
    for limits in [
        Limits {
            bytes: payload.len() - 1,
            ..LIMITS
        },
        Limits {
            allocation: LIMITS.bytes - 1,
            ..LIMITS
        },
        Limits { nodes: 1, ..LIMITS },
        Limits { depth: 0, ..LIMITS },
    ] {
        assert!(matches!(
            native::decode::<Snapshot>(payload, limits),
            Err(ic_wire::Error::Limit)
        ));
        assert_eq!(
            native::encode(&queue.load().unwrap(), limits),
            Err(ic_wire::Error::Limit)
        );
    }
    assert_eq!(queue.cell.get().as_slice(), original);
}
