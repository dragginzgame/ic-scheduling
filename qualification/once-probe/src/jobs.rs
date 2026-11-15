//! Actual Jobs-to-Once execution; host persistence remains outside this fixture.
use super::{Call, Cell, RefCell, WaitPolicy, ic0, self_id};
use ic_jobs::{Context, Job, Outcome, Schedule, Status};
thread_local! {
    static JOB: RefCell<Option<Job>> = const { RefCell::new(None) };
    static MODE: Cell<u8> = const { Cell::new(0) };
    static STARTS: Cell<u8> = const { Cell::new(0) };
    static FINISHES: Cell<u8> = const { Cell::new(0) };
    static CONTEXT: Cell<[u64; 3]> = const { Cell::new([0; 3]) };
}
fn mode() -> u8 {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut bytes = [0];
    ic0::msg_arg_data_copy(&mut bytes, 0);
    bytes[0]
}
/// Bind an every-minute job; mode selects a concrete callback/failure behavior.
/// # Panics
/// Traps on malformed fixture setup or failed binding.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_setup")]
pub extern "C" fn setup() {
    let mode = mode();
    MODE.with(|value| value.set(mode));
    ic_timers::initialize().unwrap();
    let job = ic_jobs::bind(
        ["jobs", "probe", "work"],
        Schedule::parse("* * * * *").unwrap(),
        if mode == 5 { 0 } else { u64::MAX },
        |context: Context| {
            if MODE.with(Cell::get) == 1 {
                ic0::trap(b"job factory trap");
            }
            CONTEXT.with(|value| {
                value.set([context.revision, context.scheduled_at, context.started_at])
            });
            async {
                STARTS.with(|value| value.set(value.get() + 1));
                let reply = Call::new(self_id(), "jobs_echo", WaitPolicy::Unbounded)
                    .execute()
                    .await;
                assert!(reply.is_ok());
                if MODE.with(Cell::get) == 2 {
                    ic0::trap(b"job post-await trap");
                }
                FINISHES.with(|value| value.set(value.get() + 1));
                Outcome::Success
            }
        },
    )
    .unwrap();
    JOB.with(|slot| *slot.borrow_mut() = Some(job));
    ic0::msg_reply();
}
/// Owner changes execute in a distinct message while the original job awaits.
/// # Panics
/// Traps if no job is running or owner control fails.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_echo")]
pub extern "C" fn echo() {
    JOB.with(|slot| {
        let slot = slot.borrow();
        let job = slot.as_ref().unwrap();
        assert!(matches!(job.status().unwrap(), Status::Running { .. }));
        match MODE.with(Cell::get) {
            3 => {
                job.pause().unwrap();
                job.resume().unwrap();
                job.edit(Schedule::parse("*/2 * * * *").unwrap()).unwrap();
            }
            4 => {
                job.pause().unwrap();
            }
            _ => {}
        }
    });
    ic0::msg_reply();
}
/// State bytes: phase/start/finish, then context revision/time/start and current
/// selected revision/time as five little-endian u64 values.
/// # Panics
/// Traps on absent setup or failed status projection.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query jobs_state")]
pub extern "C" fn state() {
    let (phase, revision, at) =
        JOB.with(
            |slot| match slot.borrow().as_ref().unwrap().status().unwrap() {
                Status::Paused => (0, 0, 0),
                Status::Idle => (1, 0, 0),
                Status::Armed {
                    revision,
                    scheduled_at,
                } => (2, revision, scheduled_at),
                Status::Running {
                    revision,
                    scheduled_at,
                } => (3, revision, scheduled_at),
                Status::RecoveryRequired {
                    revision,
                    scheduled_at,
                    ..
                } => (4, revision, scheduled_at),
                Status::Faulted => (5, 0, 0),
            },
        );
    ic0::msg_reply_data_append(&[phase, STARTS.with(Cell::get), FINISHES.with(Cell::get)]);
    for value in CONTEXT.with(Cell::get).into_iter().chain([revision, at]) {
        ic0::msg_reply_data_append(&value.to_le_bytes());
    }
    ic0::msg_reply();
}
/// Try owner changes without reconciliation, or explicitly reconcile then resume.
/// # Panics
/// Traps on malformed input or unexpected recovery state.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_control")]
pub extern "C" fn control() {
    let action = mode();
    JOB.with(|slot| {
        let slot = slot.borrow();
        let job = slot.as_ref().unwrap();
        match action {
            0 => {
                job.pause().unwrap();
                job.resume().unwrap();
            }
            1 => {
                let Status::RecoveryRequired { recovery, .. } = job.status().unwrap() else {
                    ic0::trap(b"expected recovery");
                };
                let Status::RecoveryRequired {
                    recovery: stale, ..
                } = job.status().unwrap()
                else {
                    ic0::trap(b"expected recovery");
                };
                job.recover(recovery).unwrap();
                assert!(matches!(job.status().unwrap(), Status::Paused));
                assert_eq!(job.recover(stale), Err(ic_jobs::Error::StaleRecovery));
                MODE.with(|value| value.set(0));
                job.resume().unwrap();
            }
            _ => ic0::trap(b"invalid action"),
        }
    });
    ic0::msg_reply();
}

/// Measure supplier parse/search separately for ordinary, sparse and impossible rules.
/// # Panics
/// Traps if a fixed fixture rule is rejected or instruction accounting regresses.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_calendar_costs")]
pub extern "C" fn calendar_costs() {
    for (rule, after) in [
        ("* * * * *", 0),
        ("0 9 29 FEB *", 4_076_006_400_000_000_000),
        ("0 9 31 * *", 1_775_001_600_000_000_000),
        ("0 9 31 FEB *", 0),
        ("* * * * *", u64::MAX / 60_000_000_000 * 60_000_000_000),
    ] {
        let before = ic0::performance_counter(0);
        let schedule = Schedule::parse(rule).unwrap();
        let parsed = ic0::performance_counter(0);
        let next = schedule.next_after(after, u64::MAX);
        let searched = ic0::performance_counter(0);
        for value in [
            parsed.checked_sub(before).unwrap(),
            searched.checked_sub(parsed).unwrap(),
            next.unwrap_or(0),
        ] {
            ic0::msg_reply_data_append(&value.to_le_bytes());
        }
    }
    ic0::msg_reply();
}
