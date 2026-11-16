//! Installed removal, transient custody and stale queued-delivery checks.
use super::{Call, Cell, RefCell, Wait, ic0, self_id};
use ic_platform_runtime::system;
use ic_timers::{
    Lifetime, Once, OnceDemand, OnceNext, OnceResult, OnceStatus, Watchdog, WatchdogDemand,
    WatchdogNext, WatchdogResult,
};
use std::num::NonZeroU64;
thread_local! {
    static MODE: Cell<u8> = const { Cell::new(0) };
    static RUNS: Cell<u64> = const { Cell::new(0) };
    static DROPS: Cell<u64> = const { Cell::new(0) };
    static ONCE: RefCell<Option<Once>> = const { RefCell::new(None) };
    static WATCHDOG: RefCell<Option<Watchdog>> = const { RefCell::new(None) };
    static REPLACEMENT: RefCell<Option<Once>> = const { RefCell::new(None) };
    static OWNER: RefCell<Option<Once>> = const { RefCell::new(None) };
}
struct Capture;
impl Drop for Capture {
    fn drop(&mut self) {
        if MODE.with(Cell::get) == 7 {
            ic0::trap(b"transient capture cleanup trap");
        }
        // Capture destruction can reenter binding/removal without a table borrow.
        let temporary = ic_timers::bind_once(
            ["lifetime", "drop", "reenter"],
            Lifetime::Retained,
            || async { stopped() },
        )
        .unwrap();
        temporary.remove().unwrap();
        DROPS.with(|value| value.set(value.get() + 1));
    }
}
fn stopped() -> OnceResult {
    OnceResult {
        observation: ic_timers::Observation::Success,
        next: OnceNext::Stop,
    }
}
fn replace_victim() {
    if MODE.with(Cell::get) == 5 {
        ONCE.with_borrow(|slot| slot.as_ref().unwrap().remove().unwrap());
    } else {
        WATCHDOG.with_borrow(|slot| slot.as_ref().unwrap().remove().unwrap());
    }
    let replacement = ic_timers::bind_once(
        ["lifetime", "victim", "old"],
        Lifetime::RemoveWhenStopped,
        || async {
            RUNS.with(|value| value.set(value.get() + 1));
            stopped()
        },
    )
    .unwrap();
    replacement
        .set(OnceDemand::At(system::time() + 10_000_000_000))
        .unwrap();
    REPLACEMENT.with_borrow_mut(|slot| *slot = Some(replacement));
}
/// Bind transient work or a remover before a queued victim.
/// # Panics
/// Traps on invalid mode or violated admission/control invariants.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update lifetime_setup")]
pub extern "C" fn setup() {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut mode = [0];
    ic0::msg_arg_data_copy(&mut mode, 0);
    assert!(mode[0] <= 7);
    MODE.with(|value| value.set(mode[0]));
    ic_timers::initialize().unwrap();
    if matches!(mode[0], 5 | 6) {
        let owner =
            ic_timers::bind_once(["lifetime", "remover", "first"], Lifetime::Retained, || {
                replace_victim();
                async { stopped() }
            })
            .unwrap();
        owner.set(OnceDemand::At(0)).unwrap();
        OWNER.with_borrow_mut(|slot| *slot = Some(owner));
    }
    let capture = Capture;
    if matches!(mode[0], 4 | 6) {
        let job = ic_timers::bind_watchdog(
            ["lifetime", "victim", "old"],
            NonZeroU64::new(10_000_000_000).unwrap(),
            Lifetime::RemoveWhenStopped,
            move || {
                let _keep = &capture;
                assert_eq!(
                    MODE.with(Cell::get),
                    4,
                    "removed queued watchdog must not run"
                );
                RUNS.with(|value| value.set(value.get() + 1));
                WATCHDOG.with_borrow(|slot| {
                    assert_eq!(slot.as_ref().unwrap().remove(), Err(ic_timers::Error::Busy))
                });
                WatchdogResult {
                    observation: ic_timers::Observation::Success,
                    next: WatchdogNext::Stop,
                }
            },
        )
        .unwrap();
        job.set(WatchdogDemand::At(0)).unwrap();
        WATCHDOG.with_borrow_mut(|slot| *slot = Some(job));
    } else {
        let job = ic_timers::bind_once(
            ["lifetime", "victim", "old"],
            Lifetime::RemoveWhenStopped,
            move || {
                let _keep = &capture;
                assert_ne!(MODE.with(Cell::get), 5, "removed queued Once must not run");
                if MODE.with(Cell::get) == 1 {
                    ic0::trap(b"transient factory trap");
                }
                async {
                    RUNS.with(|value| value.set(value.get() + 1));
                    drop(crate::expect_reply(
                        Call::new(self_id(), "lifetime_echo", Wait::Unbounded)
                            .execute()
                            .await,
                    ));
                    if MODE.with(Cell::get) == 2 {
                        ic0::trap(b"transient post-await trap");
                    }
                    stopped()
                }
            },
        )
        .unwrap();
        job.set(OnceDemand::At(0)).unwrap();
        ONCE.with_borrow_mut(|slot| *slot = Some(job));
    }
    ic0::msg_reply();
}
/// Check accepted-work removal refusal and newer-owner precedence.
/// # Panics
/// Traps if removal bypasses custody or an owner update fails.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update lifetime_echo")]
pub extern "C" fn echo() {
    ONCE.with_borrow(|slot| {
        let job = slot.as_ref().unwrap();
        assert_eq!(job.remove(), Err(ic_timers::Error::Busy));
        if MODE.with(Cell::get) == 3 && RUNS.with(Cell::get) == 1 {
            job.set(OnceDemand::At(system::time() + 10_000_000_000))
                .unwrap();
        }
    });
    ic0::msg_reply();
}
/// Return phase, callback count, released captures and replacement state.
/// # Panics
/// Traps on unexpected provider failures or authority revival.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query lifetime_state")]
pub extern "C" fn state() {
    let phase = if matches!(MODE.with(Cell::get), 4 | 6) {
        WATCHDOG.with_borrow(|slot| match slot.as_ref().unwrap().status() {
            Err(ic_timers::Error::StaleAuthority) => 5,
            Ok(_) => 2,
            Err(_) => panic!("watchdog provider fault"),
        })
    } else {
        ONCE.with_borrow(|slot| match slot.as_ref().unwrap().status() {
            Ok(OnceStatus::Armed(_)) => 2,
            Ok(OnceStatus::Running) => 3,
            Ok(OnceStatus::RecoveryRequired(_)) => 4,
            Ok(OnceStatus::Faulted) => 6,
            Err(ic_timers::Error::StaleAuthority) => 5,
            _ => panic!("unexpected transient state"),
        })
    };
    let replacement = REPLACEMENT.with_borrow(|slot| {
        slot.as_ref().map_or(0, |job| match job.status() {
            Ok(OnceStatus::Armed(_)) => 2,
            Err(ic_timers::Error::StaleAuthority) => 5,
            _ => panic!("replacement fault"),
        })
    });
    for value in [
        phase,
        RUNS.with(Cell::get),
        DROPS.with(Cell::get),
        replacement,
    ] {
        ic0::msg_reply_data_append(&value.to_le_bytes());
    }
    ic0::msg_reply();
}
/// Explicitly reconcile trapped transient work before removal.
/// # Panics
/// Traps if uncertainty can be removed or stale authority remains usable.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update lifetime_recover")]
pub extern "C" fn recover() {
    ONCE.with_borrow(|slot| {
        let job = slot.as_ref().unwrap();
        let OnceStatus::RecoveryRequired(token) = job.status().unwrap() else {
            panic!("expected recovery")
        };
        assert_eq!(job.remove(), Err(ic_timers::Error::RecoveryRequired));
        job.recover(token, OnceDemand::Inactive).unwrap();
        assert!(matches!(
            job.status(),
            Err(ic_timers::Error::StaleAuthority)
        ));
        assert_eq!(
            job.set(OnceDemand::At(0)),
            Err(ic_timers::Error::StaleAuthority)
        );
    });
    ic0::msg_reply();
}
