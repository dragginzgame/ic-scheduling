//! Completion-relative Jobs, using real calls and host-owned settled upgrades.
use super::{Call, Cell, RefCell, WaitPolicy, ic0, self_id};
use ic_jobs::{Checkpoint, Context, ControlState, Job, Outcome, Policy, Schedule, Status};
use std::num::NonZeroU64;
thread_local! {
    static JOB: RefCell<Option<Job>> = const { RefCell::new(None) };
    static MODE: Cell<u8> = const { Cell::new(0) };
    static RUNS: Cell<u64> = const { Cell::new(0) };
    static FINISHED: Cell<u64> = const { Cell::new(0) };
    static EDITED: Cell<u64> = const { Cell::new(0) };
}
fn schedule() -> Schedule {
    if MODE.with(Cell::get) == 10 {
        return Schedule::fixed_rate(0, NonZeroU64::new(10_000_000_000).unwrap());
    }
    Schedule::after_completion(
        0,
        NonZeroU64::new(if MODE.with(Cell::get) == 5 {
            u64::MAX
        } else {
            10_000_000_000
        })
        .unwrap(),
    )
}
fn policy() -> Policy {
    match MODE.with(Cell::get) {
        7 | 10 => Policy::CatchUp(ic_jobs::CatchUpLimit::new(u64::MAX, 3).unwrap()),
        6 => Policy::SkipLate { lateness_ns: 0 },
        _ => Policy::SkipLate {
            lateness_ns: u64::MAX,
        },
    }
}
fn work(context: Context) -> impl std::future::Future<Output = Outcome> {
    if MODE.with(Cell::get) == 3 {
        ic0::trap(b"completion factory trap");
    }
    async move {
        assert!(context.started_at >= context.scheduled_at);
        let run = RUNS.with(|runs| {
            runs.set(runs.get() + 1);
            runs.get()
        });
        for _ in 0..if run == 1 { 100 } else { 1 } {
            Call::new(self_id(), "jobs_completion_echo", WaitPolicy::Unbounded)
                .execute()
                .await
                .unwrap();
        }
        if MODE.with(Cell::get) == 4 {
            ic0::trap(b"completion post-await trap");
        }
        FINISHED.with(|finished| finished.set(ic0::time()));
        if run == 1 {
            Outcome::ExpectedFailure
        } else {
            Outcome::NoWork
        }
    }
}
/// Bind the completion-relative fixture.
/// # Panics
/// Traps on invalid mode or failed admission.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_completion_setup")]
pub extern "C" fn setup() {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut mode = [0];
    ic0::msg_arg_data_copy(&mut mode, 0);
    assert!(mode[0] <= 10);
    MODE.with(|value| value.set(mode[0]));
    ic_timers::initialize().unwrap();
    let job = if mode[0] == 10 {
        let pending = ic_jobs::CatchUpProgress::new(&[0, 10_000_000_000], 20_000_000_000).unwrap();
        ic_jobs::restore(
            ["completion", "jobs", "work"],
            schedule(),
            policy(),
            ic_jobs::RetryPolicy::NONE,
            Checkpoint::new(7, 1, None, None, ControlState::Paused, Some(pending), None).unwrap(),
            work,
        )
    } else {
        ic_jobs::bind(
            ["completion", "jobs", "work"],
            schedule(),
            policy(),
            ic_jobs::RetryPolicy::NONE,
            work,
        )
    }
    .unwrap();
    if mode[0] >= 8 {
        if mode[0] == 9 {
            job.pause().unwrap();
        }
        job.cancel().unwrap();
        check_cancelled(&job);
    }
    JOB.with_borrow_mut(|slot| *slot = Some(job));
    ic0::msg_reply();
}
/// Apply a newer owner command while work awaits.
/// # Panics
/// Traps on unexpected control failure.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_completion_echo")]
pub extern "C" fn echo() {
    if RUNS.with(Cell::get) == 1 && EDITED.with(Cell::get) == 0 {
        JOB.with_borrow(|slot| {
            let job = slot.as_ref().unwrap();
            assert_eq!(job.cancel(), Err(ic_jobs::Error::Unsettled));
            match MODE.with(Cell::get) {
                1 => {
                    job.pause().unwrap();
                    EDITED.with(|value| value.set(1));
                }
                2 => {
                    let at = ic0::time() + 100_000_000_000;
                    job.edit(Schedule::once(at)).unwrap();
                    EDITED.with(|value| value.set(at));
                }
                _ => {}
            }
        });
    }
    ic0::msg_reply();
}
/// Read phase, count, completion time, selected deadline, cursor and edit deadline.
/// # Panics
/// Traps on provider faults or malformed progress.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query jobs_completion_state")]
pub extern "C" fn state() {
    JOB.with_borrow(|slot| {
        let job = slot.as_ref().unwrap();
        let (phase, at) = match job.status().unwrap() {
            Status::Paused => (0, 0),
            Status::Idle => (1, 0),
            Status::Armed { scheduled_at, .. } => (2, scheduled_at),
            Status::Running { scheduled_at, .. } => (3, scheduled_at),
            Status::RecoveryRequired { scheduled_at, .. } => (4, scheduled_at),
            Status::Cancelled => (5, 0),
            Status::Faulted => ic0::trap(b"faulted completion job"),
        };
        let through = match job.checkpoint() {
            Ok(checkpoint) => checkpoint.through().unwrap_or(0),
            Err(ic_jobs::Error::Unsettled) => 0,
            Err(_) => ic0::trap(b"invalid completion checkpoint"),
        };
        for value in [
            phase,
            RUNS.with(Cell::get),
            FINISHED.with(Cell::get),
            at,
            through,
            EDITED.with(Cell::get),
        ] {
            ic0::msg_reply_data_append(&value.to_le_bytes());
        }
    });
    ic0::msg_reply();
}
/// Resume paused work, or explicitly reconcile uncertainty before resuming.
/// # Panics
/// Traps on failed recovery/control.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_completion_resume")]
pub extern "C" fn resume() {
    JOB.with_borrow(|slot| {
        let job = slot.as_ref().unwrap();
        if let Status::RecoveryRequired { recovery, .. } = job.status().unwrap() {
            assert_eq!(job.cancel(), Err(ic_jobs::Error::Unsettled));
            job.pause().unwrap();
            job.resume().unwrap();
            assert!(matches!(
                job.status().unwrap(),
                Status::RecoveryRequired { .. }
            ));
            job.recover(recovery, ic_jobs::RecoveryDisposition::Retire)
                .unwrap();
            MODE.with(|mode| mode.set(0));
        }
        job.resume().unwrap();
        let checkpoint = job.checkpoint().unwrap();
        assert_eq!(
            checkpoint.selected_at(),
            checkpoint.through().unwrap().checked_add(10_000_000_000)
        );
    });
    ic0::msg_reply();
}

fn check_cancelled(job: &Job) {
    assert!(matches!(job.status().unwrap(), Status::Cancelled));
    job.cancel().unwrap();
    assert_eq!(job.resume(), Err(ic_jobs::Error::Cancelled));
    assert_eq!(job.pause(), Err(ic_jobs::Error::Cancelled));
    assert_eq!(
        job.edit(Schedule::once(u64::MAX)),
        Err(ic_jobs::Error::Cancelled)
    );
    let checkpoint = job.checkpoint().unwrap();
    assert_eq!(checkpoint.control(), ControlState::Cancelled);
    assert_eq!(checkpoint.selected_at(), None);
    assert_eq!(checkpoint.catch_up(), None);
}

pub(super) fn pre_upgrade() -> bool {
    JOB.with_borrow(|slot| {
        let Some(job) = slot else {
            return false;
        };
        let checkpoint = job
            .checkpoint()
            .expect("host refuses unresolved completion job");
        assert!(
            checkpoint.retry().is_none(),
            "fixture definition disables retries"
        );
        let mode = MODE.with(Cell::get);
        assert!(matches!(mode, 0 | 1 | 7..=10));
        let mut bytes = b"JOBSCMP4".to_vec();
        for value in [
            u64::from(mode),
            RUNS.with(Cell::get),
            FINISHED.with(Cell::get),
            checkpoint.through().unwrap_or(0),
            checkpoint.selected_at().unwrap_or(0),
            match checkpoint.control() {
                ControlState::Active => 0,
                ControlState::Paused => 1,
                ControlState::Cancelled => 2,
            },
            u64::from(checkpoint.through().is_some()),
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
    let mut bytes = [0; 104];
    ic0::stable64_read(&mut bytes, 0);
    if &bytes[..8] != b"JOBSCMP4" {
        return false;
    }
    let values: [u64; 7] = std::array::from_fn(|i| {
        u64::from_le_bytes(bytes[8 + i * 8..16 + i * 8].try_into().unwrap())
    });
    assert!(matches!(values[0], 0 | 1 | 7..=10) && values[5] <= 2 && values[6] <= 1);
    MODE.with(|mode| mode.set(u8::try_from(values[0]).unwrap()));
    RUNS.with(|runs| runs.set(values[1]));
    FINISHED.with(|finished| finished.set(values[2]));
    let control = match values[5] {
        0 => ControlState::Active,
        1 => ControlState::Paused,
        _ => ControlState::Cancelled,
    };
    if control != ControlState::Active {
        assert_eq!(values[4], 0);
    }
    if values[6] == 0 {
        assert_eq!(values[3], 0);
    }
    let checkpoint = Checkpoint::new(
        7,
        1,
        (values[6] == 1).then_some(values[3]),
        (control == ControlState::Active).then_some(values[4]),
        control,
        None,
        None,
    )
    .unwrap();
    let checkpoint = super::outcomes::restore(checkpoint, &bytes[64..]);
    ic_timers::initialize().unwrap();
    let job = ic_jobs::restore(
        ["completion", "jobs", "work"],
        schedule(),
        policy(),
        ic_jobs::RetryPolicy::NONE,
        checkpoint,
        work,
    )
    .unwrap();
    if control == ControlState::Cancelled {
        check_cancelled(&job);
    }
    JOB.with_borrow_mut(|slot| *slot = Some(job));
    true
}
