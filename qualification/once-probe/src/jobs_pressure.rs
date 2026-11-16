//! Shared admission and independent Jobs failure custody under held-flight pressure.
use super::{Call, Principal, RefCell, Wait, ic0};
use ic_jobs::{Job, Outcome, Schedule, Status};
use ic_platform_runtime::system;

thread_local! {
    static JOBS: RefCell<Vec<Job>> = const { RefCell::new(Vec::new()) };
    static COUNTS: RefCell<([u8; 64], [u8; 64])> = const { RefCell::new(([0; 64], [0; 64])) };
    static CONTEXTS: RefCell<[(u64, u64); 64]> = const { RefCell::new([(0, 0); 64]) };
}

/// Fill the shared table with twelve Jobs and 52 inactive Once declarations.
/// # Panics
/// Traps on malformed target input or unexpected admission results.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_pressure_setup")]
pub extern "C" fn setup() {
    let len = ic0::msg_arg_data_size();
    assert!(len > 0 && len <= 29);
    let mut bytes = [0; 29];
    ic0::msg_arg_data_copy(&mut bytes[..len], 0);
    let target = Principal::from_slice(&bytes[..len]);
    ic_timers::initialize().unwrap();
    for index in 0..12 {
        let job = ic_jobs::bind(
            ["jobs", "pressure", &index.to_string()],
            Schedule::parse("* * * * *").unwrap(),
            ic_jobs::Policy::SkipLate {
                lateness_ns: u64::MAX,
            },
            ic_jobs::RetryPolicy::NONE,
            move |context| async move {
                CONTEXTS.with(|values| {
                    values.borrow_mut()[index] = (context.scheduled_at, context.started_at);
                });
                let first = COUNTS.with(|counts| {
                    let mut counts = counts.borrow_mut();
                    counts.0[index] += 1;
                    counts.0[index] == 1
                });
                if index < 8 && first {
                    let reply = Call::new(target, "hold", Wait::Unbounded).execute().await;
                    drop(crate::expect_reply(reply));
                    if index == 0 {
                        ic0::trap(b"one pressured job abandons after await");
                    }
                }
                COUNTS.with(|counts| counts.borrow_mut().1[index] += 1);
                Outcome::Success
            },
        )
        .unwrap();
        JOBS.with(|jobs| jobs.borrow_mut().push(job));
    }
    for index in 0..52 {
        let _once = ic_timers::bind_once(
            ["jobs", "filler", &index.to_string()],
            ic_timers::Lifetime::Retained,
            || async {
                super::OnceResult {
                    observation: super::Observation::NoWork,
                    next: super::OnceNext::Stop,
                }
            },
        )
        .unwrap();
    }
    assert!(matches!(
        ic_jobs::bind(
            ["jobs", "pressure", "overflow"],
            Schedule::parse("* * * * *").unwrap(),
            ic_jobs::Policy::SkipLate {
                lateness_ns: u64::MAX
            },
            ic_jobs::RetryPolicy::NONE,
            |_| async { Outcome::Success }
        ),
        Err(ic_jobs::Error::Timer(ic_timers::Error::Capacity))
    ));
    ic0::msg_reply();
}

/// Pause or resume the ninth job, which initially waits outside eight held flights.
/// # Panics
/// Traps on malformed input or failed job control.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_pressure_control")]
pub extern "C" fn control() {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut action = [0];
    ic0::msg_arg_data_copy(&mut action, 0);
    JOBS.with(|jobs| match action[0] {
        0 => jobs.borrow()[8].pause().unwrap(),
        1 => jobs.borrow()[8].resume().unwrap(),
        _ => ic0::trap(b"invalid pressure action"),
    });
    ic0::msg_reply();
}

/// One record per job: phase/start/finish followed by scheduled/start timestamps.
/// # Panics
/// Traps on failed status projection.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query jobs_pressure_state")]
pub extern "C" fn state() {
    JOBS.with(|jobs| {
        for (index, job) in jobs.borrow().iter().enumerate() {
            let phase = match job.status().unwrap() {
                Status::Paused => 0,
                Status::Idle => 1,
                Status::Armed { .. } => 2,
                Status::Running { .. } => 3,
                Status::RecoveryRequired { .. } => 4,
                Status::Cancelled => ic0::trap(b"unexpected cancelled fixture"),
                Status::Faulted => 5,
            };
            COUNTS.with(|counts| {
                let counts = counts.borrow();
                ic0::msg_reply_data_append(&[phase, counts.0[index], counts.1[index]]);
            });
            CONTEXTS.with(|values| {
                let (scheduled, started) = values.borrow()[index];
                ic0::msg_reply_data_append(&scheduled.to_le_bytes());
                ic0::msg_reply_data_append(&started.to_le_bytes());
            });
        }
    });
    ic0::msg_reply();
}

/// Fill all shared declarations with three-occurrence catch-up jobs.
/// # Panics
/// Traps on admission failure, overlapping attempts or duplicate/out-of-order work.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_bulk_setup")]
pub extern "C" fn bulk_setup() {
    ic_timers::initialize().unwrap();
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut mode = [0];
    ic0::msg_arg_data_copy(&mut mode, 0);
    let schedule = match mode[0] {
        0 => Schedule::parse("* * * * *").unwrap(),
        1 => Schedule::fixed_rate(
            system::time().checked_add(5_000_000_000).unwrap(),
            std::num::NonZeroU64::new(10_000_000_000).unwrap(),
        ),
        _ => ic0::trap(b"invalid schedule mode"),
    };
    for index in 0..64 {
        let job = ic_jobs::bind(
            ["jobs", "bulk", &index.to_string()],
            schedule.clone(),
            ic_jobs::Policy::CatchUp(ic_jobs::CatchUpLimit::new(u64::MAX, 3).unwrap()),
            ic_jobs::RetryPolicy::NONE,
            move |context| async move {
                COUNTS.with_borrow_mut(|counts| {
                    assert_eq!(counts.0[index], counts.1[index], "overlapping job attempts");
                    assert!(counts.0[index] < 3, "catch-up quota replenished");
                    counts.0[index] += 1;
                });
                CONTEXTS.with_borrow_mut(|values| {
                    assert!(context.scheduled_at > values[index].0);
                    values[index] = (context.scheduled_at, context.started_at);
                });
                drop(crate::expect_reply(
                    Call::new(super::self_id(), "echo", Wait::Unbounded)
                        .execute()
                        .await,
                ));
                COUNTS.with_borrow_mut(|counts| counts.1[index] += 1);
                Outcome::Success
            },
        )
        .unwrap();
        let Status::Armed { scheduled_at, .. } = job.status().unwrap() else {
            ic0::trap(b"expected initial occurrence");
        };
        ic0::msg_reply_data_append(&scheduled_at.to_le_bytes());
        JOBS.with_borrow_mut(|jobs| jobs.push(job));
    }
    assert!(matches!(
        ic_jobs::bind(
            ["jobs", "bulk", "overflow"],
            Schedule::parse("* * * * *").unwrap(),
            ic_jobs::Policy::CatchUp(ic_jobs::CatchUpLimit::new(u64::MAX, 3).unwrap()),
            ic_jobs::RetryPolicy::NONE,
            |_| async { Outcome::Success }
        ),
        Err(ic_jobs::Error::Timer(ic_timers::Error::Capacity))
    ));
    ic0::msg_reply();
}
