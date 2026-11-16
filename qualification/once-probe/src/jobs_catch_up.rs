//! Bounded catch-up execution and host-owned batch persistence.
use super::{Call, Cell, RefCell, Wait, ic0, self_id};
use ic_jobs::{
    CatchUpLimit, CatchUpProgress, Checkpoint, Context, Job, Outcome, Policy, Schedule, Status,
};
thread_local! {
    static JOB: RefCell<Option<Job>> = const { RefCell::new(None) };
    static MODE: Cell<u8> = const { Cell::new(0) };
    static LOG: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    static FINISHED: Cell<u8> = const { Cell::new(0) };
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
}
fn policy(mode: u8) -> Policy {
    Policy::CatchUp(
        CatchUpLimit::new(
            if matches!(mode, 1 | 7) {
                120_000_000_000
            } else {
                u64::MAX
            },
            match mode {
                6 => 1,
                8 => 64,
                _ => 3,
            },
        )
        .unwrap(),
    )
}
fn arg() -> u8 {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut value = [0];
    ic0::msg_arg_data_copy(&mut value, 0);
    value[0]
}
fn work(context: Context) -> impl std::future::Future<Output = Outcome> {
    let mode = MODE.with(Cell::get);
    if mode == 3 || (mode == 9 && LOG.with(|log| log.borrow().len()) == 1) {
        ic0::trap(b"catch-up factory trap");
    }
    async move {
        assert!(
            !ACTIVE.with(|active| active.replace(true)),
            "overlapping occurrence"
        );
        LOG.with(|log| {
            let mut log = log.borrow_mut();
            assert!(log.len() < 96);
            log.push(context.scheduled_at);
        });
        drop(crate::expect_reply(
            Call::new(self_id(), "catch_up_echo", Wait::Unbounded)
                .execute()
                .await,
        ));
        let mode = MODE.with(Cell::get);
        if mode == 4 || (mode == 10 && LOG.with(|log| log.borrow().len()) == 2) {
            ic0::trap(b"catch-up post-await trap");
        }
        FINISHED.with(|count| count.set(count.get() + 1));
        ACTIVE.with(|active| active.set(false));
        Outcome::Success
    }
}
/// Bind a selected catch-up scenario.
/// # Panics
/// Traps on invalid setup or admission failure.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update catch_up_setup")]
pub extern "C" fn setup() {
    let mode = arg();
    assert!(mode <= 10);
    MODE.with(|value| value.set(mode));
    ic_timers::initialize().unwrap();
    let job = ic_jobs::bind(
        ["catch-up", "probe", "work"],
        Schedule::parse("* * * * *").unwrap(),
        policy(mode),
        ic_jobs::RetryPolicy::NONE,
        work,
    )
    .unwrap();
    JOB.with(|slot| *slot.borrow_mut() = Some(job));
    ic0::msg_reply();
}
/// Apply owner controls during the first accepted occurrence.
/// # Panics
/// Traps on unexpected checkpoint/control behavior.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update catch_up_echo")]
pub extern "C" fn echo() {
    JOB.with(|slot| {
        let slot = slot.borrow();
        let job = slot.as_ref().unwrap();
        assert_eq!(job.checkpoint(), Err(ic_jobs::Error::Unsettled));
        assert!(ACTIVE.with(Cell::get));
        if LOG.with(|log| log.borrow().len()) == 1 {
            match MODE.with(Cell::get) {
                2 | 7 => job.pause().unwrap(),
                5 => job.edit(Schedule::parse("*/2 * * * *").unwrap()).unwrap(),
                _ => {}
            }
        }
    });
    ic0::msg_reply();
}
/// Resume paused work, or reconcile uncertainty and resume future work.
/// # Panics
/// Traps on malformed controls or absent recovery authority.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update catch_up_control")]
pub extern "C" fn control() {
    let action = arg();
    JOB.with(|slot| {
        let slot = slot.borrow();
        let job = slot.as_ref().unwrap();
        match action {
            0 => job.resume().unwrap(),
            1 => {
                let Status::RecoveryRequired { recovery, .. } = job.status().unwrap() else {
                    ic0::trap(b"expected recovery");
                };
                job.recover(recovery, ic_jobs::RecoveryDisposition::Retire)
                    .unwrap();
                ACTIVE.with(|active| active.set(false));
                MODE.with(|value| value.set(0));
                job.resume().unwrap();
            }
            _ => ic0::trap(b"invalid control"),
        }
    });
    ic0::msg_reply();
}
/// Phase, finished count, log length, selected time, then the committed start log.
/// # Panics
/// Traps on missing job or status errors.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query catch_up_state")]
pub extern "C" fn state() {
    let (phase, at) = JOB.with(
        |slot| match slot.borrow().as_ref().unwrap().status().unwrap() {
            Status::Paused => (0, 0),
            Status::Idle => (1, 0),
            Status::Armed { scheduled_at, .. } => (2, scheduled_at),
            Status::Running { scheduled_at, .. } => (3, scheduled_at),
            Status::RecoveryRequired { scheduled_at, .. } => (4, scheduled_at),
            Status::Cancelled => ic0::trap(b"unexpected cancelled fixture"),
            Status::Faulted => (5, 0),
        },
    );
    LOG.with(|log| {
        let log = log.borrow();
        ic0::msg_reply_data_append(&[
            phase,
            FINISHED.with(Cell::get),
            u8::try_from(log.len()).unwrap(),
        ]);
        ic0::msg_reply_data_append(&at.to_le_bytes());
        for at in log.iter() {
            ic0::msg_reply_data_append(&at.to_le_bytes());
        }
    });
    ic0::msg_reply();
}

pub(super) fn pre_upgrade() -> bool {
    JOB.with(|slot| {
        let slot = slot.borrow();
        let Some(job) = slot.as_ref() else {
            return false;
        };
        let checkpoint = job
            .checkpoint()
            .unwrap_or_else(|_| ic0::trap(b"host refuses unsettled catch-up"));
        assert!(
            checkpoint.retry().is_none(),
            "fixture definition disables retries"
        );
        assert!(matches!(
            checkpoint.control(),
            ic_jobs::ControlState::Active | ic_jobs::ControlState::Paused
        ));
        let mode = MODE.with(Cell::get);
        assert!(matches!(mode, 0 | 2 | 7));
        let pending = checkpoint
            .catch_up()
            .map_or(&[][..], CatchUpProgress::pending);
        let mut bytes = b"CATCHUP5".to_vec();
        bytes.extend([
            mode,
            u8::from(checkpoint.paused()),
            FINISHED.with(Cell::get),
            u8::from(checkpoint.selected_at().is_some()),
            u8::try_from(pending.len()).unwrap(),
        ]);
        for value in [
            checkpoint.revision(),
            checkpoint
                .through()
                .expect("cron fixture has settled cursor"),
            checkpoint.selected_at().unwrap_or(0),
            checkpoint.catch_up().map_or(0, CatchUpProgress::through),
        ] {
            bytes.extend(value.to_le_bytes());
        }
        for at in pending {
            bytes.extend(at.to_le_bytes());
        }
        LOG.with(|log| {
            let log = log.borrow();
            bytes.push(u8::try_from(log.len()).unwrap());
            for at in log.iter() {
                bytes.extend(at.to_le_bytes());
            }
        });
        super::outcomes::append(checkpoint.last_outcome(), &mut bytes);
        assert!(bytes.len() <= 1024);
        bytes.resize(1024, 0);
        if ic0::stable64_size() == 0 {
            assert_eq!(ic0::stable64_grow(1), Some(0));
        }
        ic0::stable64_write(0, &bytes);
        true
    })
}
pub(super) fn post_upgrade() -> bool {
    let mut bytes = [0; 1024];
    ic0::stable64_read(0, &mut bytes);
    if &bytes[..8] != b"CATCHUP5" {
        return false;
    }
    assert!(matches!(bytes[8], 0 | 2 | 7) && bytes[9] <= 1 && bytes[11] <= 1 && bytes[12] <= 64);
    MODE.with(|value| value.set(bytes[8]));
    FINISHED.with(|value| value.set(bytes[10]));
    let mut offset = 13;
    fn read(bytes: &[u8], offset: &mut usize) -> u64 {
        let value = u64::from_le_bytes(bytes[*offset..*offset + 8].try_into().unwrap());
        *offset += 8;
        value
    }
    let revision = read(&bytes, &mut offset);
    let through = read(&bytes, &mut offset);
    let selected = read(&bytes, &mut offset);
    let end = read(&bytes, &mut offset);
    let pending: Vec<_> = (0..bytes[12]).map(|_| read(&bytes, &mut offset)).collect();
    let batch = if pending.is_empty() {
        assert_eq!(end, 0);
        None
    } else {
        Some(CatchUpProgress::new(&pending, end).unwrap())
    };
    let count = bytes[offset];
    offset += 1;
    assert!(count <= 32);
    LOG.with(|log| *log.borrow_mut() = (0..count).map(|_| read(&bytes, &mut offset)).collect());
    let checkpoint = Checkpoint::new(
        7,
        revision,
        Some(through),
        (bytes[11] != 0).then_some(selected),
        if bytes[9] != 0 {
            ic_jobs::ControlState::Paused
        } else {
            ic_jobs::ControlState::Active
        },
        batch,
        None,
    )
    .unwrap();
    let checkpoint = super::outcomes::restore(checkpoint, &bytes[offset..offset + 40]);
    ic_timers::initialize().unwrap();
    let job = ic_jobs::restore(
        ["catch-up", "probe", "work"],
        Schedule::parse("* * * * *").unwrap(),
        policy(bytes[8]),
        ic_jobs::RetryPolicy::NONE,
        checkpoint,
        work,
    )
    .unwrap();
    JOB.with(|slot| *slot.borrow_mut() = Some(job));
    true
}
