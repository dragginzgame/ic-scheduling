//! Installed idle schedules, ordinary outcomes and host-restored numeric limits.
use super::{RefCell, ic0};
use ic_jobs::{Checkpoint, Error, Job, Outcome, Schedule, Status};

thread_local! {
    static JOBS: RefCell<Vec<Job>> = const { RefCell::new(Vec::new()) };
    static COUNTS: RefCell<[u8; 5]> = const { RefCell::new([0; 5]) };
    static TIMES: RefCell<[u64; 3]> = const { RefCell::new([0; 3]) };
}
fn minute() -> Schedule {
    Schedule::parse("* * * * *").unwrap()
}

/// Bind ordinary-outcome jobs, an impossible rule and two boundary checkpoints.
/// # Panics
/// Traps if boundary progress is admitted unsafely or status/control is wrong.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_boundaries_setup")]
pub extern "C" fn setup() {
    ic_timers::initialize().unwrap();
    for index in 0..3 {
        let schedule = if index == 2 {
            Schedule::parse("0 0 31 FEB *").unwrap()
        } else {
            minute()
        };
        let job = ic_jobs::bind(
            ["jobs", "boundaries", &index.to_string()],
            schedule,
            ic_jobs::Policy::SkipLate {
                lateness_ns: u64::MAX,
            },
            move |context| async move {
                COUNTS.with(|counts| counts.borrow_mut()[index] += 1);
                TIMES.with(|times| times.borrow_mut()[index] = context.scheduled_at);
                match index {
                    0 => Outcome::NoWork,
                    1 => Outcome::ExpectedFailure,
                    _ => Outcome::Success,
                }
            },
        )
        .unwrap();
        JOBS.with(|jobs| jobs.borrow_mut().push(job));
    }
    let exhausted = ic_jobs::restore(
        ["jobs", "boundaries", "revision"],
        minute(),
        ic_jobs::Policy::SkipLate {
            lateness_ns: u64::MAX,
        },
        Checkpoint::new(
            4,
            u64::MAX,
            Some(ic0::time()),
            None,
            ic_jobs::ControlState::Active,
            None,
        )
        .unwrap(),
        |_| async {
            COUNTS.with(|counts| counts.borrow_mut()[3] += 1);
            Outcome::Success
        },
    )
    .unwrap();
    assert!(matches!(exhausted.status().unwrap(), Status::Armed { .. }));
    assert_eq!(exhausted.edit(minute()), Err(Error::Faulted));
    assert!(matches!(exhausted.status().unwrap(), Status::Faulted));
    assert_eq!(exhausted.checkpoint(), Err(Error::Faulted));
    assert_eq!(exhausted.resume(), Err(Error::Faulted));
    let end = ic_jobs::restore(
        ["jobs", "boundaries", "end"],
        minute(),
        ic_jobs::Policy::SkipLate {
            lateness_ns: u64::MAX,
        },
        Checkpoint::new(
            4,
            1,
            Some(u64::MAX),
            None,
            ic_jobs::ControlState::Active,
            None,
        )
        .unwrap(),
        |_| async {
            COUNTS.with(|counts| counts.borrow_mut()[4] += 1);
            Outcome::Success
        },
    )
    .unwrap();
    end.pause().unwrap();
    end.resume().unwrap();
    assert!(matches!(end.status().unwrap(), Status::Idle));
    assert_eq!(end.checkpoint().unwrap().through(), Some(u64::MAX));
    JOBS.with(|jobs| jobs.borrow_mut().extend([exhausted, end]));
    ic0::msg_reply();
}

/// Switch the initially impossible rule to every minute, then back to impossible.
/// # Panics
/// Traps on malformed input or unexpected schedule/revision state.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_boundaries_edit")]
pub extern "C" fn edit() {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut action = [0];
    ic0::msg_arg_data_copy(&mut action, 0);
    JOBS.with(|jobs| {
        let jobs = jobs.borrow();
        match action[0] {
            0 => {
                assert!(matches!(jobs[2].status().unwrap(), Status::Idle));
                jobs[2].edit(minute()).unwrap();
                assert!(matches!(
                    jobs[2].status().unwrap(),
                    Status::Armed { revision: 2, .. }
                ));
            }
            1 => {
                jobs[2]
                    .edit(Schedule::parse("0 0 31 FEB *").unwrap())
                    .unwrap();
                assert!(matches!(jobs[2].status().unwrap(), Status::Idle));
                let checkpoint = jobs[2].checkpoint().unwrap();
                assert_eq!(checkpoint.revision(), 3);
                assert_eq!(checkpoint.selected_at(), None);
            }
            _ => ic0::trap(b"invalid boundary action"),
        }
    });
    ic0::msg_reply();
}

/// Five statuses, five callback counts and three scheduled timestamps.
/// # Panics
/// Traps if a public status query fails.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query jobs_boundaries_state")]
pub extern "C" fn state() {
    JOBS.with(|jobs| {
        for job in jobs.borrow().iter() {
            let phase = match job.status().unwrap() {
                Status::Paused => 0,
                Status::Idle => 1,
                Status::Armed { .. } => 2,
                Status::Running { .. } => 3,
                Status::RecoveryRequired { .. } => 4,
                Status::Cancelled => ic0::trap(b"unexpected cancelled fixture"),
                Status::Faulted => 5,
            };
            ic0::msg_reply_data_append(&[phase]);
        }
    });
    COUNTS.with(|counts| ic0::msg_reply_data_append(&*counts.borrow()));
    TIMES.with(|times| {
        for time in times.borrow().iter() {
            ic0::msg_reply_data_append(&time.to_le_bytes());
        }
    });
    ic0::msg_reply();
}

/// Measure bounded catch-up previews independently of execution state.
/// # Panics
/// Traps if fixed fixture inputs are rejected or instruction accounting regresses.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_catch_up_plans")]
pub extern "C" fn catch_up_plans() {
    for (rule, now, age, count) in [
        ("* * * * *", u64::MAX, u64::MAX, 64),
        ("* * * * *", 600_000_000_000, 300_000_000_000, 3),
        ("0 0 31 FEB *", u64::MAX, u64::MAX, 64),
    ] {
        let schedule = Schedule::parse(rule).unwrap();
        let limit = ic_jobs::CatchUpLimit::new(age, count).unwrap();
        let before = ic0::performance_counter(0);
        let plan = schedule.catch_up(0, now, limit);
        let cost = ic0::performance_counter(0).checked_sub(before).unwrap();
        ic0::msg_reply_data_append(&[
            u8::try_from(plan.occurrences().len()).unwrap(),
            u8::from(plan.has_excess()),
        ]);
        for value in [
            plan.occurrences().first().copied().unwrap_or(0),
            plan.occurrences().last().copied().unwrap_or(0),
            plan.through(),
            cost,
        ] {
            ic0::msg_reply_data_append(&value.to_le_bytes());
        }
    }
    ic0::msg_reply();
}

thread_local! {
    static AHEAD: RefCell<Vec<Job>> = const { RefCell::new(Vec::new()) };
    static AHEAD_RUNS: RefCell<[[u64; 3]; 2]> = const { RefCell::new([[0; 3]; 2]) };
}

/// Restore settled progress ten minutes ahead of current time under both policies.
/// # Panics
/// Traps on restore failure or unexpected progress.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_ahead_setup")]
pub extern "C" fn ahead_setup() {
    ic_timers::initialize().unwrap();
    let minute_ns = 60_000_000_000;
    let through = (ic0::time() / minute_ns + 10) * minute_ns;
    for (index, policy) in [
        ic_jobs::Policy::SkipLate {
            lateness_ns: u64::MAX,
        },
        ic_jobs::Policy::CatchUp(ic_jobs::CatchUpLimit::new(u64::MAX, 3).unwrap()),
    ]
    .into_iter()
    .enumerate()
    {
        let job = ic_jobs::restore(
            ["jobs", "ahead", &index.to_string()],
            minute(),
            policy,
            Checkpoint::new(
                4,
                7,
                Some(through),
                None,
                ic_jobs::ControlState::Active,
                None,
            )
            .unwrap(),
            move |context| async move {
                AHEAD_RUNS.with_borrow_mut(|runs| {
                    runs[index][0] += 1;
                    runs[index][1] = context.scheduled_at;
                    runs[index][2] = context.revision;
                });
                Outcome::Success
            },
        )
        .unwrap();
        AHEAD.with_borrow_mut(|jobs| jobs.push(job));
    }
    ahead_state();
}

/// Edit while paused before the restored cursor, then resume without rewinding it.
/// # Panics
/// Traps on control failure or cursor/selection regression.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_ahead_control")]
pub extern "C" fn ahead_control() {
    AHEAD.with_borrow(|jobs| {
        for job in jobs {
            let before = job.checkpoint().unwrap();
            assert!(Some(ic0::time()) < before.through());
            job.pause().unwrap();
            job.edit(minute()).unwrap();
            let paused = job.checkpoint().unwrap();
            assert!(paused.paused());
            assert_eq!(paused.through(), before.through());
            assert_eq!(paused.selected_at(), None);
            job.resume().unwrap();
            let after = job.checkpoint().unwrap();
            assert_eq!(after.through(), before.through());
            assert_eq!(after.selected_at(), before.selected_at());
            assert_eq!(after.revision(), before.revision() + 1);
        }
    });
    ahead_state();
}

/// Two rows of cursor, selection, revision, run count, last occurrence and revision.
/// # Panics
/// Traps if progress is unsettled or no future occurrence is armed.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query jobs_ahead_state")]
pub extern "C" fn ahead_state() {
    AHEAD.with_borrow(|jobs| {
        for (index, job) in jobs.iter().enumerate() {
            assert!(matches!(job.status().unwrap(), Status::Armed { .. }));
            let checkpoint = job.checkpoint().unwrap();
            for value in [
                checkpoint
                    .through()
                    .expect("cron fixture has settled cursor"),
                checkpoint.selected_at().unwrap(),
                checkpoint.revision(),
            ] {
                ic0::msg_reply_data_append(&value.to_le_bytes());
            }
            AHEAD_RUNS.with_borrow(|runs| {
                for value in runs[index] {
                    ic0::msg_reply_data_append(&value.to_le_bytes());
                }
            });
        }
    });
    ic0::msg_reply();
}
