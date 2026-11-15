//! Actual Jobs-to-Once execution and host-owned checkpoint reconstruction.
use super::{Call, Cell, RefCell, WaitPolicy, ic0, self_id};
use ic_jobs::{Context, Job, Outcome, Schedule, Status};
use std::{
    future::Future,
    pin::Pin,
    task::{Context as TaskContext, Poll},
};
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
        ic_jobs::Policy::SkipLate {
            lateness_ns: if mode == 5 { 0 } else { u64::MAX },
        },
        work,
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
        assert_eq!(job.checkpoint(), Err(ic_jobs::Error::Unsettled));
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
                reject_foreign_recovery(job);
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
            2 => {
                job.edit(Schedule::parse("* * * * *").unwrap()).unwrap();
            }
            3 => {
                job.pause().unwrap();
            }
            _ => ic0::trap(b"invalid action"),
        }
    });
    ic0::msg_reply();
}

// A token obtained from one uncertain job cannot change another job's progress.
// Exercise both policies and both armed/paused target states before real recovery.
fn reject_foreign_recovery(source: &Job) {
    for (index, policy) in [
        ic_jobs::Policy::SkipLate {
            lateness_ns: u64::MAX,
        },
        ic_jobs::Policy::CatchUp(ic_jobs::CatchUpLimit::new(u64::MAX, 3).unwrap()),
    ]
    .into_iter()
    .enumerate()
    {
        let target = ic_jobs::bind(
            ["jobs", "foreign-recovery", &index.to_string()],
            Schedule::parse("* * * * *").unwrap(),
            policy,
            |_| async { ic0::trap(b"foreign recovery target must remain paused") },
        )
        .unwrap();
        for paused in [false, true] {
            if paused {
                target.pause().unwrap();
            }
            let before = target.checkpoint().unwrap();
            let Status::RecoveryRequired {
                revision,
                scheduled_at,
                recovery,
            } = source.status().unwrap()
            else {
                ic0::trap(b"source must remain uncertain");
            };
            assert_eq!(target.recover(recovery), Err(ic_jobs::Error::StaleRecovery));
            assert_eq!(target.checkpoint().unwrap(), before);
            assert!(
                matches!(source.status().unwrap(), Status::RecoveryRequired {
                revision: current_revision, scheduled_at: current_at, ..
            } if current_revision == revision && current_at == scheduled_at)
            );
            assert_eq!(source.checkpoint(), Err(ic_jobs::Error::Unsettled));
        }
    }
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

fn work(context: Context) -> impl std::future::Future<Output = Outcome> {
    if MODE.with(Cell::get) == 1 {
        ic0::trap(b"job factory trap");
    }
    CONTEXT.with(|value| value.set([context.revision, context.scheduled_at, context.started_at]));
    let future = async {
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
    };
    ReentrantDrop {
        inner: Box::pin(future),
        enabled: MODE.with(Cell::get) == 6,
    }
}

// Exercise destruction of the returned future itself, rather than only a local
// guard inside its async body. The adapter must hold no job-state borrow here.
struct ReentrantDrop<F> {
    inner: Pin<Box<F>>,
    enabled: bool,
}
impl<F: Future> Future for ReentrantDrop<F> {
    type Output = F::Output;
    fn poll(mut self: Pin<&mut Self>, context: &mut TaskContext<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(context)
    }
}
impl<F> Drop for ReentrantDrop<F> {
    fn drop(&mut self) {
        if self.enabled {
            JOB.with_borrow(|slot| {
                let job = slot.as_ref().unwrap();
                assert!(matches!(job.status().unwrap(), Status::Running { .. }));
                assert_eq!(job.checkpoint(), Err(ic_jobs::Error::Unsettled));
                job.edit(Schedule::parse("* * * * *").unwrap()).unwrap();
                job.pause().unwrap();
            });
        }
    }
}

pub(super) fn pre_upgrade() -> bool {
    JOB.with(|slot| {
        let slot = slot.borrow();
        let Some(job) = slot.as_ref() else {
            return false;
        };
        let checkpoint = job.checkpoint().expect("host refuses unresolved job");
        // This fixed host schema owns the every-minute rule; mode records lateness.
        let mode = MODE.with(Cell::get);
        assert!(matches!(mode, 0 | 5), "unsupported fixture definition");
        let mut bytes = [0; 35];
        bytes[..8].copy_from_slice(b"JOBDATA4");
        bytes[8] = checkpoint.version();
        bytes[9..17].copy_from_slice(&checkpoint.revision().to_le_bytes());
        bytes[17..25].copy_from_slice(
            &checkpoint
                .through()
                .expect("cron fixture has settled cursor")
                .to_le_bytes(),
        );
        bytes[25..33].copy_from_slice(&checkpoint.selected_at().unwrap_or(0).to_le_bytes());
        bytes[33] =
            u8::from(checkpoint.selected_at().is_some()) | (u8::from(checkpoint.paused()) << 1);
        bytes[34] = mode;
        if ic0::stable64_size() == 0 {
            assert_eq!(ic0::stable64_grow(1), 0);
        }
        ic0::stable64_write(&bytes, 0);
        true
    })
}
pub(super) fn post_upgrade() -> bool {
    let mut bytes = [0; 35];
    ic0::stable64_read(&mut bytes, 0);
    if &bytes[..8] != b"JOBDATA4" {
        return false;
    }
    assert!(bytes[33] <= 3);
    assert!(matches!(bytes[34], 0 | 5));
    MODE.with(|value| value.set(bytes[34]));
    let selected = u64::from_le_bytes(bytes[25..33].try_into().unwrap());
    if bytes[33] & 1 == 0 {
        assert_eq!(selected, 0);
    }
    let checkpoint = ic_jobs::Checkpoint::new(
        bytes[8],
        u64::from_le_bytes(bytes[9..17].try_into().unwrap()),
        Some(u64::from_le_bytes(bytes[17..25].try_into().unwrap())),
        (bytes[33] & 1 != 0).then_some(selected),
        bytes[33] & 2 != 0,
        None,
    )
    .unwrap();
    ic_timers::initialize().unwrap();
    let job = ic_jobs::restore(
        ["jobs", "probe", "work"],
        Schedule::parse("* * * * *").unwrap(),
        ic_jobs::Policy::SkipLate {
            lateness_ns: if bytes[34] == 5 { 0 } else { u64::MAX },
        },
        checkpoint,
        work,
    )
    .unwrap();
    JOB.with(|slot| *slot.borrow_mut() = Some(job));
    true
}

/// A structurally valid but non-calendar occurrence must fail before admission.
/// # Panics
/// Traps if invalid restoration consumes an identity or succeeds.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update jobs_invalid_restore")]
pub extern "C" fn invalid_restore() {
    ic_timers::initialize().unwrap();
    let checkpoint = ic_jobs::Checkpoint::new(3, 1, Some(0), Some(1), false, None).unwrap();
    assert!(matches!(
        ic_jobs::restore(
            ["invalid", "job", "restore"],
            Schedule::parse("* * * * *").unwrap(),
            ic_jobs::Policy::SkipLate { lateness_ns: 0 },
            checkpoint,
            work
        ),
        Err(ic_jobs::Error::InvalidCheckpoint)
    ));
    let _handle = ic_timers::bind_once(["invalid", "job", "restore"], || async {
        super::OnceResult {
            observation: super::Observation::NoWork,
            next: super::OnceNext::Stop,
        }
    })
    .unwrap();
    for (name, pending, policy) in [
        (
            "calendar",
            [60_000_000_000, 61_000_000_000],
            ic_jobs::Policy::CatchUp(ic_jobs::CatchUpLimit::new(u64::MAX, 3).unwrap()),
        ),
        (
            "policy",
            [60_000_000_000, 120_000_000_000],
            ic_jobs::Policy::SkipLate {
                lateness_ns: u64::MAX,
            },
        ),
        (
            "count",
            [60_000_000_000, 120_000_000_000],
            ic_jobs::Policy::CatchUp(ic_jobs::CatchUpLimit::new(u64::MAX, 1).unwrap()),
        ),
    ] {
        let checkpoint = ic_jobs::Checkpoint::new(
            3,
            1,
            Some(0),
            Some(pending[0]),
            false,
            Some(ic_jobs::CatchUpProgress::new(&pending, 180_000_000_000).unwrap()),
        )
        .unwrap();
        assert!(matches!(
            ic_jobs::restore(
                ["invalid", "batch", name],
                Schedule::parse("* * * * *").unwrap(),
                policy,
                checkpoint,
                work
            ),
            Err(ic_jobs::Error::InvalidCheckpoint)
        ));
        let _handle = ic_timers::bind_once(["invalid", "batch", name], || async {
            super::OnceResult {
                observation: super::Observation::NoWork,
                next: super::OnceNext::Stop,
            }
        })
        .unwrap();
    }
    ic0::msg_reply();
}
