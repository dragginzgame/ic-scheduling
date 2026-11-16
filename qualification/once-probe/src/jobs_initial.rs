//! Initial elapsed deadlines, including epoch zero and unstarted reconstruction.
use super::{Cell, RefCell, ic0};
use ic_jobs::{Checkpoint, Job, Outcome, Policy, Schedule, Status};
use ic_platform_runtime::system;
use std::num::NonZeroU64;

thread_local! {
    static JOB: RefCell<Option<Job>> = const { RefCell::new(None) };
    static LOG: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    static MODE: Cell<u8> = const { Cell::new(0) };
    static ONE_SHOT: Cell<bool> = const { Cell::new(false) };
}

/// Bind or reconstruct the original first deadline without treating zero as settled.
/// # Panics
/// Traps on malformed mode or incorrect admission.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_initial_setup")]
pub extern "C" fn setup() {
    assert_eq!(ic0::msg_arg_data_size(), 2);
    let mut mode = [0; 2];
    ic0::msg_arg_data_copy(&mut mode, 0);
    assert!(mode[1] <= 1);
    let one_shot = mode[1] == 1;
    ONE_SHOT.with(|value| value.set(one_shot));
    let mode = mode[0];
    assert!(mode <= 5 || (one_shot && mode <= 7));
    MODE.with(|value| value.set(mode));
    let first = match mode {
        3 => system::time() - 20_000_000_000,
        4 => system::time() + 10_000_000_000,
        _ => 0,
    };
    let every = if matches!(mode, 0 | 5) {
        u64::MAX
    } else {
        10_000_000_000
    };
    let schedule = if one_shot {
        Schedule::once(first)
    } else {
        Schedule::fixed_rate(first, NonZeroU64::new(every).unwrap())
    };
    let policy = match mode {
        1 | 3 => Policy::CatchUp(
            ic_jobs::CatchUpLimit::new(u64::MAX, if mode == 1 { 3 } else { 2 }).unwrap(),
        ),
        2 => Policy::SkipLate { lateness_ns: 0 },
        _ => Policy::SkipLate {
            lateness_ns: u64::MAX,
        },
    };
    ic_timers::initialize().unwrap();
    let work = |context: ic_jobs::Context| {
        if MODE.with(Cell::get) == 6 {
            ic0::trap(b"one-shot factory trap");
        }
        async move {
            LOG.with_borrow_mut(|log| log.push(context.scheduled_at));
            assert!(context.started_at >= context.scheduled_at);
            if MODE.with(Cell::get) == 7 {
                drop(crate::expect_reply(
                    super::Call::new(super::self_id(), "echo", super::Wait::Unbounded)
                        .execute()
                        .await,
                ));
                ic0::trap(b"one-shot post-await trap");
            }
            Outcome::Success
        }
    };
    let job = if mode == 5 {
        ic_jobs::restore(
            ["initial", "fixed", "job"],
            schedule,
            policy,
            ic_jobs::RetryPolicy::NONE,
            Checkpoint::new(
                7,
                1,
                None,
                Some(0),
                ic_jobs::ControlState::Active,
                None,
                None,
            )
            .unwrap(),
            work,
        )
    } else {
        ic_jobs::bind(
            ["initial", "fixed", "job"],
            schedule,
            policy,
            ic_jobs::RetryPolicy::NONE,
            work,
        )
    }
    .unwrap();
    assert!(
        matches!(job.status().unwrap(), Status::Armed { scheduled_at, .. } if scheduled_at == first)
    );
    let checkpoint = job.checkpoint().unwrap();
    assert_eq!(checkpoint.through(), None);
    assert_eq!(checkpoint.selected_at(), Some(first));
    JOB.with_borrow_mut(|slot| *slot = Some(job));
    ic0::msg_reply_data_append(&first.to_le_bytes());
    ic0::msg_reply();
}

/// Read executed occurrence identities and verify the resulting checkpoint.
/// # Panics
/// Traps on faulted or unbound state.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query jobs_initial_state")]
pub extern "C" fn state() {
    JOB.with_borrow(|slot| {
        let job = slot.as_ref().unwrap();
        assert!(!matches!(job.status().unwrap(), Status::Faulted));
        LOG.with_borrow(|log| {
            if matches!(MODE.with(Cell::get), 6 | 7) {
                assert!(matches!(
                    job.status().unwrap(),
                    Status::RecoveryRequired { .. }
                ));
                assert_eq!(job.checkpoint(), Err(ic_jobs::Error::Unsettled));
            } else if ONE_SHOT.with(Cell::get) && (!log.is_empty() || MODE.with(Cell::get) == 2) {
                assert!(matches!(job.status().unwrap(), Status::Idle));
            }
            if !log.is_empty() && matches!(MODE.with(Cell::get), 0 | 5) {
                assert_eq!(job.checkpoint().unwrap().through(), Some(0));
            }
            for at in log {
                ic0::msg_reply_data_append(&at.to_le_bytes());
            }
        });
    });
    ic0::msg_reply();
}

/// Explicitly retire the uncertain one-shot; resuming cannot create another occurrence.
/// # Panics
/// Traps unless uncertainty is retained across owner pause/resume and recovery succeeds.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_initial_recover")]
pub extern "C" fn recover() {
    JOB.with_borrow(|slot| {
        let job = slot.as_ref().unwrap();
        job.pause().unwrap();
        job.resume().unwrap();
        let Status::RecoveryRequired {
            scheduled_at: 0,
            recovery,
            ..
        } = job.status().unwrap()
        else {
            ic0::trap(b"lost one-shot uncertainty");
        };
        job.recover(recovery, ic_jobs::RecoveryDisposition::Retire)
            .unwrap();
        job.resume().unwrap();
        assert!(matches!(job.status().unwrap(), Status::Idle));
        assert!(job.checkpoint().unwrap().through().is_some());
    });
    MODE.with(|mode| mode.set(8));
    ic0::msg_reply();
}
