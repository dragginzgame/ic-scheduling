//! Explicit retry-safe outcomes, stable effect identity and pending retry upgrades.
use super::{Call, Cell, RefCell, WaitPolicy, ic0, self_id};
use ic_jobs::{
    Checkpoint, Context, ControlState, Job, Outcome, Policy, RetryPolicy, RetryProgress, Schedule,
    Status,
};
use std::num::NonZeroU64;
const SECOND: u64 = 1_000_000_000;
thread_local! {
    static JOB: RefCell<Option<Job>> = const { RefCell::new(None) };
    static MODE: Cell<u8> = const { Cell::new(0) };
    static RUNS: Cell<u64> = const { Cell::new(0) };
    static LAST: Cell<[u64; 4]> = const { Cell::new([0; 4]) };
}
fn schedule() -> Schedule {
    match MODE.with(Cell::get) {
        7 => Schedule::after_completion(0, NonZeroU64::new(100 * SECOND).unwrap()),
        8 | 17 => Schedule::fixed_rate(0, NonZeroU64::new(SECOND).unwrap()),
        _ => Schedule::once(0),
    }
}
fn policy() -> Policy {
    if matches!(MODE.with(Cell::get), 8 | 17) {
        Policy::CatchUp(ic_jobs::CatchUpLimit::new(u64::MAX, 2).unwrap())
    } else {
        Policy::SkipLate {
            // First delivery is within tolerance; later retry delays exceed it.
            lateness_ns: if MODE.with(Cell::get) == 7 {
                ic0::time() + 5 * SECOND
            } else {
                u64::MAX
            },
        }
    }
}
fn retry_policy() -> RetryPolicy {
    match MODE.with(Cell::get) {
        15 => RetryPolicy::new(3, u64::MAX, u64::MAX).unwrap(),
        16 => RetryPolicy::NONE,
        _ => RetryPolicy::new(3, 10 * SECOND, 15 * SECOND).unwrap(),
    }
}
fn work(context: Context) -> impl std::future::Future<Output = Outcome> {
    if (matches!(MODE.with(Cell::get), 3 | 11) && context.attempt == 2)
        || (matches!(MODE.with(Cell::get), 15 | 16) && context.attempt == 1)
    {
        ic0::trap(b"retry factory trap");
    }
    async move {
        RUNS.with(|value| value.set(value.get() + 1));
        LAST.with(|value| {
            value.set([
                context.revision,
                context.scheduled_at,
                u64::from(context.attempt),
                0,
            ])
        });
        Call::new(self_id(), "jobs_retry_echo", WaitPolicy::Unbounded)
            .execute()
            .await
            .unwrap();
        if (matches!(MODE.with(Cell::get), 4 | 12 | 14) && context.attempt == 2)
            || (MODE.with(Cell::get) == 13 && context.attempt == 3)
            || (MODE.with(Cell::get) == 17 && context.attempt == 2 && context.scheduled_at == 0)
        {
            ic0::trap(b"retry post-await trap");
        }
        LAST.with(|value| {
            let mut last = value.get();
            last[3] = ic0::time();
            value.set(last);
        });
        match MODE.with(Cell::get) {
            1 if context.attempt == 2 => Outcome::Success,
            9 => Outcome::ExpectedFailure,
            _ => Outcome::RetryableFailure,
        }
    }
}
/// Admit one retry fixture.
/// # Panics
/// Traps on invalid input or failed admission.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_retry_setup")]
pub extern "C" fn setup() {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut mode = [0];
    ic0::msg_arg_data_copy(&mut mode, 0);
    assert!(mode[0] <= 17);
    MODE.with(|value| value.set(mode[0]));
    ic_timers::initialize().unwrap();
    let job = ic_jobs::bind(
        ["retry", "jobs", "work"],
        schedule(),
        policy(),
        retry_policy(),
        work,
    )
    .unwrap();
    JOB.with_borrow_mut(|slot| *slot = Some(job));
    ic0::msg_reply();
}
/// Exercise owner precedence during an actual await.
/// # Panics
/// Traps on failed control or lost attempt identity.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_retry_echo")]
pub extern "C" fn echo() {
    JOB.with_borrow(|slot| {
        let job = slot.as_ref().unwrap();
        assert_eq!(job.checkpoint(), Err(ic_jobs::Error::Unsettled));
        assert_eq!(job.cancel(), Err(ic_jobs::Error::Unsettled));
        let attempt = LAST.with(Cell::get)[2];
        if MODE.with(Cell::get) == 5 && attempt == 2 {
            job.edit(Schedule::once(ic0::time() + 100 * SECOND))
                .unwrap();
            assert!(matches!(
                job.status().unwrap(),
                Status::Running {
                    attempt: 2,
                    revision: 1,
                    scheduled_at: 0
                }
            ));
        }
        if MODE.with(Cell::get) == 10 && attempt == 1 {
            job.pause().unwrap();
        }
    });
    ic0::msg_reply();
}
/// Pause, resume, cancel or explicitly reconcile the current occurrence.
/// # Panics
/// Traps on invalid commands or control failure.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_retry_control")]
pub extern "C" fn control() {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut command = [0];
    ic0::msg_arg_data_copy(&mut command, 0);
    JOB.with_borrow(|slot| {
        let job = slot.as_ref().unwrap();
        match command[0] {
            0 => job.pause().unwrap(),
            1 => job.resume().unwrap(),
            2 => job.cancel().unwrap(),
            3 => {
                let Status::RecoveryRequired {
                    recovery,
                    attempt: 2,
                    ..
                } = job.status().unwrap()
                else {
                    panic!("expected uncertain retry")
                };
                job.recover(recovery, ic_jobs::RecoveryDisposition::Retire)
                    .unwrap();
                assert!(job.checkpoint().unwrap().retry().is_none());
            }
            4 | 6 => {
                let Status::RecoveryRequired { recovery, attempt, .. } = job.status().unwrap() else { panic!("expected recovery") };
                let Status::RecoveryRequired { recovery: stale, .. } = job.status().unwrap() else { panic!("expected recovery") };
                let result = job.recover(recovery, ic_jobs::RecoveryDisposition::RetrySafe);
                if command[0] == 6 {
                    assert_eq!(result, Err(ic_jobs::Error::RetryUnavailable));
                    assert!(matches!(job.status().unwrap(), Status::RecoveryRequired { attempt: current, .. } if current == attempt));
                    // Rejection does not consume the live authority or settle the attempt.
                    job.recover(stale, ic_jobs::RecoveryDisposition::Retire).unwrap();
                    assert!(job.checkpoint().unwrap().retry().is_none());
                } else {
                    result.unwrap();
                    assert_eq!(job.recover(stale, ic_jobs::RecoveryDisposition::RetrySafe), Err(ic_jobs::Error::StaleRecovery));
                    let checkpoint = job.checkpoint().unwrap();
                    assert_eq!(checkpoint.retry().unwrap().failed_attempts(), attempt);
                    assert_eq!(checkpoint.retry().unwrap().failed_at(), ic0::time());
                    assert_eq!(checkpoint.retry().unwrap().retry_at(), ic0::time() + 15 * SECOND);
                }
                assert!(matches!(job.status().unwrap(), Status::Paused));
            }
            5 => job.edit(Schedule::once(ic0::time() + 100 * SECOND)).unwrap(),
            _ => panic!("invalid command"),
        }
    });
    ic0::msg_reply();
}
/// Return phase, identity, attempt, readiness, runs, last callback and pending count.
/// # Panics
/// Traps on terminal provider faults.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query jobs_retry_state")]
pub extern "C" fn state() {
    JOB.with_borrow(|slot| {
        let job = slot.as_ref().unwrap();
        let (phase, revision, scheduled, attempt, ready) = match job.status().unwrap() {
            Status::Paused => (0, 0, 0, 0, 0),
            Status::Idle => (1, 0, 0, 0, 0),
            Status::Armed {
                revision,
                scheduled_at,
                attempt,
                ready_at,
            } => (2, revision, scheduled_at, attempt, ready_at),
            Status::Running {
                revision,
                scheduled_at,
                attempt,
            } => (3, revision, scheduled_at, attempt, 0),
            Status::RecoveryRequired {
                revision,
                scheduled_at,
                attempt,
                ..
            } => (4, revision, scheduled_at, attempt, 0),
            Status::Cancelled => (5, 0, 0, 0, 0),
            Status::Faulted => panic!("faulted retry"),
        };
        let checkpoint = job.checkpoint().ok();
        let pending = checkpoint.as_ref().and_then(Checkpoint::retry);
        let last = LAST.with(Cell::get);
        for value in [
            phase,
            revision,
            scheduled,
            u64::from(attempt),
            ready,
            RUNS.with(Cell::get),
            last[0],
            last[1],
            last[2],
            last[3],
            pending.map_or(0, |retry| u64::from(retry.failed_attempts())),
            checkpoint
                .as_ref()
                .and_then(Checkpoint::catch_up)
                .map_or(0, |batch| batch.pending().len() as u64),
        ] {
            ic0::msg_reply_data_append(&value.to_le_bytes());
        }
        let mut result_bytes = Vec::new();
        super::outcomes::append(
            checkpoint.as_ref().and_then(Checkpoint::last_outcome),
            &mut result_bytes,
        );
        ic0::msg_reply_data_append(&result_bytes);
    });
    ic0::msg_reply();
}
pub(super) fn pre_upgrade() -> bool {
    JOB.with_borrow(|slot| {
        let Some(job) = slot else {
            return false;
        };
        let checkpoint = job.checkpoint().expect("host refuses unresolved retry");
        assert!(matches!(MODE.with(Cell::get), 0 | 2 | 11 | 12));
        assert_eq!(checkpoint.through(), None);
        assert_eq!(checkpoint.catch_up(), None);
        let retry = checkpoint
            .retry()
            .expect("fixture upgrades pending retries");
        let last = LAST.with(Cell::get);
        let mut bytes = b"JOBRETR2".to_vec();
        for value in [
            u64::from(checkpoint.version()),
            u64::from(MODE.with(Cell::get)),
            RUNS.with(Cell::get),
            last[0],
            last[1],
            last[2],
            last[3],
            checkpoint.revision(),
            u64::from(checkpoint.paused()),
            retry.scheduled_at(),
            u64::from(retry.failed_attempts()),
            retry.failed_at(),
            retry.retry_at(),
        ] {
            bytes.extend(value.to_le_bytes());
        }
        super::outcomes::append(checkpoint.last_outcome(), &mut bytes);
        if ic0::stable64_size() == 0 {
            assert_eq!(ic0::stable64_grow(1), 0);
        }
        ic0::stable64_write(&bytes, 0);
        true
    })
}
pub(super) fn post_upgrade() -> bool {
    let mut bytes = [0; 152];
    ic0::stable64_read(&mut bytes, 0);
    if &bytes[..8] != b"JOBRETR2" {
        return false;
    }
    let v: [u64; 13] = std::array::from_fn(|i| {
        u64::from_le_bytes(bytes[8 + i * 8..16 + i * 8].try_into().unwrap())
    });
    assert!(matches!(v[1], 0 | 2 | 11 | 12) && v[8] <= 1);
    MODE.with(|value| value.set(u8::try_from(v[1]).unwrap()));
    RUNS.with(|value| value.set(v[2]));
    LAST.with(|value| value.set([v[3], v[4], v[5], v[6]]));
    let checkpoint = Checkpoint::new(
        u8::try_from(v[0]).unwrap(),
        v[7],
        None,
        (v[8] == 0).then_some(v[9]),
        if v[8] == 0 {
            ControlState::Active
        } else {
            ControlState::Paused
        },
        None,
        Some(RetryProgress::new(v[9], u32::try_from(v[10]).unwrap(), v[11], v[12]).unwrap()),
    )
    .unwrap();
    let checkpoint = super::outcomes::restore(checkpoint, &bytes[112..]);
    ic_timers::initialize().unwrap();
    for invalid in [
        RetryPolicy::NONE,
        RetryPolicy::new(3, SECOND, SECOND).unwrap(),
    ] {
        assert!(matches!(
            ic_jobs::restore(
                ["invalid", "retry", "work"],
                schedule(),
                policy(),
                invalid,
                checkpoint.clone(),
                work
            ),
            Err(ic_jobs::Error::InvalidCheckpoint)
        ));
    }
    let job = ic_jobs::restore(
        ["retry", "jobs", "work"],
        schedule(),
        policy(),
        retry_policy(),
        checkpoint,
        work,
    )
    .unwrap();
    JOB.with_borrow_mut(|slot| *slot = Some(job));
    true
}
