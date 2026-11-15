//! Host-owned unresolved records across real upgrades; no automatic replay.
use super::{Call, Cell, RefCell, WaitPolicy, ic0, self_id};
use ic_jobs::{
    Attempt, CatchUpProgress, Checkpoint, Context, ControlState, Job, Outcome, Policy,
    RecoveryDisposition, RetryPolicy, Schedule, Status, UnresolvedPhase,
};
use std::num::NonZeroU64;
const SECOND: u64 = 1_000_000_000;
thread_local! {
    static JOB: RefCell<Option<Job>> = const { RefCell::new(None) };
    static MODE: Cell<u8> = const { Cell::new(0) };
    static RUNS: Cell<u64> = const { Cell::new(0) };
    static EDITED: Cell<u64> = const { Cell::new(0) };
    static UPGRADED: Cell<bool> = const { Cell::new(false) };
    static EXPORTED_PHASE: Cell<u64> = const { Cell::new(2) };
}
fn schedule() -> Schedule {
    let edited = EDITED.with(Cell::get);
    if edited != 0 {
        Schedule::once(edited)
    } else if MODE.with(Cell::get) == 6 {
        Schedule::fixed_rate(0, NonZeroU64::new(SECOND).unwrap())
    } else {
        Schedule::once(0)
    }
}
fn policy() -> Policy {
    if MODE.with(Cell::get) == 6 {
        Policy::CatchUp(ic_jobs::CatchUpLimit::new(u64::MAX, 2).unwrap())
    } else {
        Policy::SkipLate {
            lateness_ns: u64::MAX,
        }
    }
}
fn retries() -> RetryPolicy {
    RetryPolicy::new(3, 10 * SECOND, 15 * SECOND).unwrap()
}
fn work(context: Context) -> impl std::future::Future<Output = Outcome> {
    let mode = MODE.with(Cell::get);
    let upgraded = UPGRADED.with(Cell::get);
    if !upgraded && (mode == 0 || (mode == 3 && context.attempt == 2)) {
        ic0::trap(b"unresolved factory");
    }
    async move {
        RUNS.with(|runs| runs.set(runs.get() + 1));
        for _ in 0..if !upgraded && matches!(mode, 2 | 5) {
            100
        } else {
            1
        } {
            Call::new(self_id(), "jobs_unresolved_echo", WaitPolicy::Unbounded)
                .execute()
                .await
                .unwrap();
        }
        if !upgraded && matches!(mode, 1 | 4 | 6) {
            ic0::trap(b"unresolved after await");
        }
        if !upgraded && mode == 3 {
            Outcome::RetryableFailure
        } else {
            Outcome::Success
        }
    }
}
/// Bind the unresolved-record fixture.
/// # Panics
/// Traps on invalid mode or admission failure.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_unresolved_setup")]
pub extern "C" fn setup() {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut mode = [0];
    ic0::msg_arg_data_copy(&mut mode, 0);
    assert!(mode[0] <= 6);
    MODE.with(|value| value.set(mode[0]));
    ic_timers::initialize().unwrap();
    let job = ic_jobs::bind(
        ["unresolved", "jobs", "work"],
        schedule(),
        policy(),
        retries(),
        work,
    )
    .unwrap();
    assert_eq!(job.unresolved_checkpoint(), Err(ic_jobs::Error::Unsettled));
    JOB.with_borrow_mut(|slot| *slot = Some(job));
    ic0::msg_reply();
}
/// Commit an owner edit during the old attempt's actual await.
/// # Panics
/// Traps on control failure.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_unresolved_echo")]
pub extern "C" fn echo() {
    if !UPGRADED.with(Cell::get)
        && matches!(MODE.with(Cell::get), 4 | 5)
        && EDITED.with(Cell::get) == 0
    {
        let at = ic0::time() + 100 * SECOND;
        JOB.with_borrow(|slot| slot.as_ref().unwrap().edit(Schedule::once(at)).unwrap());
        EDITED.with(|value| value.set(at));
    }
    ic0::msg_reply();
}
/// Return status identity, run count, exported phase, owner revision and batch count.
/// # Panics
/// Traps on provider faults.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query jobs_unresolved_state")]
pub extern "C" fn state() {
    JOB.with_borrow(|slot| {
        let job = slot.as_ref().unwrap();
        let (phase, revision, at, attempt) = match job.status().unwrap() {
            Status::Paused => (0, 0, 0, 0),
            Status::Idle => (1, 0, 0, 0),
            Status::Armed {
                revision,
                scheduled_at,
                attempt,
                ..
            } => (2, revision, scheduled_at, attempt),
            Status::Running {
                revision,
                scheduled_at,
                attempt,
            } => (3, revision, scheduled_at, attempt),
            Status::RecoveryRequired {
                revision,
                scheduled_at,
                attempt,
                ..
            } => (4, revision, scheduled_at, attempt),
            _ => panic!("unexpected unresolved fixture state"),
        };
        let progress = if let Ok(record) = job.unresolved_checkpoint() {
            record
        } else {
            job.checkpoint().unwrap()
        };
        for value in [
            phase,
            revision,
            at,
            u64::from(attempt),
            RUNS.with(Cell::get),
            EXPORTED_PHASE.with(Cell::get),
            progress.revision(),
            progress
                .catch_up()
                .map_or(0, |batch| batch.pending().len() as u64),
        ] {
            ic0::msg_reply_data_append(&value.to_le_bytes());
        }
        let mut result_bytes = Vec::new();
        super::outcomes::append(progress.last_outcome(), &mut result_bytes);
        ic0::msg_reply_data_append(&result_bytes);
    });
    ic0::msg_reply();
}
/// Exercise the restoration barrier or explicitly reconcile it.
/// # Panics
/// Traps if controls bypass uncertainty or reject valid authority.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_unresolved_control")]
pub extern "C" fn control() {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut command = [0];
    ic0::msg_arg_data_copy(&mut command, 0);
    JOB.with_borrow(|slot| {
        let job = slot.as_ref().unwrap();
        if command[0] == 3 {
            job.resume().unwrap();
            return;
        }
        let Status::RecoveryRequired { recovery, .. } = job.status().unwrap() else {
            panic!("expected restored barrier")
        };
        if command[0] == 0 {
            job.pause().unwrap();
            job.resume().unwrap();
            assert!(matches!(
                job.status().unwrap(),
                Status::RecoveryRequired { .. }
            ));
            assert_eq!(job.cancel(), Err(ic_jobs::Error::Unsettled));
            assert_eq!(job.checkpoint(), Err(ic_jobs::Error::Unsettled));
        } else {
            assert!(command[0] <= 2);
            let Status::RecoveryRequired {
                recovery: stale, ..
            } = job.status().unwrap()
            else {
                panic!("expected barrier")
            };
            if EDITED.with(Cell::get) != 0 {
                assert_eq!(
                    job.recover(recovery, RecoveryDisposition::RetrySafe),
                    Err(ic_jobs::Error::RetryUnavailable)
                );
                let Status::RecoveryRequired { recovery, .. } = job.status().unwrap() else {
                    panic!("lost barrier")
                };
                job.recover(recovery, RecoveryDisposition::Retire).unwrap();
            } else {
                job.recover(
                    recovery,
                    if command[0] == 1 {
                        RecoveryDisposition::Retire
                    } else {
                        RecoveryDisposition::RetrySafe
                    },
                )
                .unwrap();
            }
            assert_eq!(
                job.recover(stale, RecoveryDisposition::Retire),
                Err(ic_jobs::Error::StaleRecovery)
            );
            assert!(matches!(job.status().unwrap(), Status::Paused));
        }
    });
    ic0::msg_reply();
}
pub(super) fn pre_upgrade() -> bool {
    JOB.with_borrow(|slot| {
        let Some(job) = slot else {
            return false;
        };
        let record = job.unresolved_checkpoint().unwrap();
        let progress = &record;
        let attempt = record.unresolved().unwrap().0;
        let pending = progress
            .catch_up()
            .map_or(&[][..], CatchUpProgress::pending);
        assert!(pending.len() <= 2);
        let mut bytes = b"UNRESOL3".to_vec();
        for value in [
            u64::from(MODE.with(Cell::get)),
            RUNS.with(Cell::get),
            EDITED.with(Cell::get),
            u64::from(progress.version()),
            progress.revision(),
            u64::from(progress.through().is_some()),
            progress.through().unwrap_or(0),
            attempt.revision(),
            attempt.scheduled_at(),
            u64::from(attempt.number()),
            u64::from(record.unresolved().unwrap().1 == UnresolvedPhase::Uncertain),
            progress.catch_up().map_or(0, CatchUpProgress::through),
            pending.len() as u64,
            pending.first().copied().unwrap_or(0),
            pending.get(1).copied().unwrap_or(0),
        ] {
            bytes.extend(value.to_le_bytes());
        }
        super::outcomes::append(progress.last_outcome(), &mut bytes);
        if ic0::stable64_size() == 0 {
            assert_eq!(ic0::stable64_grow(1), 0);
        }
        ic0::stable64_write(&bytes, 0);
        true
    })
}
pub(super) fn post_upgrade() -> bool {
    let mut bytes = [0; 168];
    ic0::stable64_read(&mut bytes, 0);
    if &bytes[..8] != b"UNRESOL3" {
        return false;
    }
    let v: [u64; 15] = std::array::from_fn(|i| {
        u64::from_le_bytes(bytes[8 + i * 8..16 + i * 8].try_into().unwrap())
    });
    assert!(v[0] <= 6 && v[5] <= 1 && v[10] <= 1 && v[12] <= 2);
    MODE.with(|value| value.set(u8::try_from(v[0]).unwrap()));
    RUNS.with(|value| value.set(v[1]));
    EDITED.with(|value| value.set(v[2]));
    UPGRADED.with(|value| value.set(true));
    EXPORTED_PHASE.with(|value| value.set(v[10]));
    let batch = if v[12] == 0 {
        None
    } else {
        Some(CatchUpProgress::new(&v[13..13 + usize::try_from(v[12]).unwrap()], v[11]).unwrap())
    };
    let progress = Checkpoint::new(
        u8::try_from(v[3]).unwrap(),
        v[4],
        (v[5] == 1).then_some(v[6]),
        None,
        ControlState::Paused,
        batch,
        None,
    )
    .unwrap();
    let progress = super::outcomes::restore(progress, &bytes[128..]);
    let record = progress
        .with_unresolved(
            Attempt::new(v[7], v[8], u32::try_from(v[9]).unwrap()).unwrap(),
            if v[10] == 0 {
                UnresolvedPhase::Running
            } else {
                UnresolvedPhase::Uncertain
            },
        )
        .unwrap();
    ic_timers::initialize().unwrap();
    assert!(matches!(
        ic_jobs::restore(
            ["unresolved", "jobs", "work"],
            schedule(),
            policy(),
            retries(),
            record.clone(),
            work,
        ),
        Err(ic_jobs::Error::InvalidCheckpoint)
    ));
    let job = ic_jobs::restore_unresolved(
        ["unresolved", "jobs", "work"],
        schedule(),
        policy(),
        retries(),
        record,
        work,
    )
    .unwrap();
    assert!(matches!(
        job.status().unwrap(),
        Status::RecoveryRequired { .. }
    ));
    JOB.with_borrow_mut(|slot| *slot = Some(job));
    true
}
