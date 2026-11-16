//! Installed capture custody under full-table control churn; no allocator replacement.
use super::{Call, Observation, OnceDemand, OnceNext, OnceResult, Wait, ic0, self_id};
use ic_platform_runtime::system;
use ic_timers::{Once, Watchdog, WatchdogDemand, WatchdogNext, WatchdogResult};
use std::{cell::RefCell, num::NonZeroU64};

thread_local! {
    static HANDLES: RefCell<(Vec<Once>, Vec<Watchdog>)> = const { RefCell::new((Vec::new(), Vec::new())) };
    // Live retained captures, released rejected captures, live futures, released futures, callbacks.
    static COUNTS: RefCell<[u64; 5]> = const { RefCell::new([0; 5]) };
}
struct Capture {
    bytes: Box<[u8]>,
    rejected: bool,
}
impl Capture {
    fn new(rejected: bool) -> Self {
        if !rejected {
            COUNTS.with_borrow_mut(|counts| counts[0] += 1);
        }
        Self {
            bytes: vec![0x5a; 4096].into_boxed_slice(),
            rejected,
        }
    }
    fn touch(&self) {
        assert_eq!(self.bytes.len(), 4096);
        assert!(self.bytes.iter().all(|&byte| byte == 0x5a));
        COUNTS.with_borrow_mut(|counts| counts[4] += 1);
    }
}
impl Drop for Capture {
    fn drop(&mut self) {
        // Reentry must be safe: rejected user captures cannot drop under a registry borrow.
        ic_timers::initialize().unwrap();
        HANDLES.with_borrow(|handles| {
            for handle in &handles.0 {
                handle.status().unwrap();
            }
        });
        COUNTS.with_borrow_mut(|counts| {
            if self.rejected {
                counts[1] += 1;
            } else {
                counts[0] -= 1;
            }
        });
    }
}
struct Attempt;
impl Attempt {
    fn new() -> Self {
        COUNTS.with_borrow_mut(|counts| counts[2] += 1);
        Self
    }
}
impl Drop for Attempt {
    fn drop(&mut self) {
        COUNTS.with_borrow_mut(|counts| {
            counts[2] -= 1;
            counts[3] += 1;
        });
    }
}
fn result() -> OnceResult {
    OnceResult {
        observation: Observation::NoWork,
        next: OnceNext::Stop,
    }
}

/// Fill all slots with 4 KiB captures and maximum-length identity components.
/// # Panics
/// Traps on admission or capture custody failures.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update retention_setup")]
pub extern "C" fn setup() {
    ic_timers::initialize().unwrap();
    for index in 0..64 {
        let name = format!("{index:064}");
        let capture = Capture::new(false);
        if index % 2 == 0 {
            let handle = ic_timers::bind_once(
                [&"o".repeat(64), &"g".repeat(64), &name],
                ic_timers::Lifetime::Retained,
                move || {
                    capture.touch();
                    async {
                        let _attempt = Attempt::new();
                        drop(crate::expect_reply(
                            Call::new(self_id(), "echo", Wait::Unbounded)
                                .execute()
                                .await,
                        ));
                        result()
                    }
                },
            )
            .unwrap();
            HANDLES.with_borrow_mut(|handles| handles.0.push(handle));
        } else {
            let handle = ic_timers::bind_watchdog(
                [&"o".repeat(64), &"g".repeat(64), &name],
                NonZeroU64::new(1_000_000_000).unwrap(),
                ic_timers::Lifetime::Retained,
                move || {
                    capture.touch();
                    WatchdogResult {
                        observation: Observation::NoWork,
                        next: WatchdogNext::Stop,
                    }
                },
            )
            .unwrap();
            HANDLES.with_borrow_mut(|handles| handles.1.push(handle));
        }
    }
    state();
}

/// Exercise deadline replacement and rejected capture destruction without running work.
/// # Panics
/// Traps on unexpected acceptance, reentrant drop failure or control failure.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update retention_churn")]
pub extern "C" fn churn() {
    let future = system::time().checked_add(60_000_000_000).unwrap();
    for round in 0..16 {
        HANDLES.with_borrow(|handles| {
            for handle in &handles.0 {
                handle.set(OnceDemand::At(future + round)).unwrap();
                handle.set(OnceDemand::At(future + round)).unwrap();
                handle.set(OnceDemand::Inactive).unwrap();
            }
            for handle in &handles.1 {
                handle.set(WatchdogDemand::At(future + round)).unwrap();
                handle.set(WatchdogDemand::KeepCadence).unwrap();
                handle.set(WatchdogDemand::Inactive).unwrap();
            }
        });
        for (identity, expected) in [
            (
                ["o".repeat(64), "g".repeat(64), format!("{:064}", 0)],
                ic_timers::Error::DuplicateIdentity,
            ),
            (
                ["extra".into(), "extra".into(), "extra".into()],
                ic_timers::Error::Capacity,
            ),
            (
                [String::new(), "extra".into(), "extra".into()],
                ic_timers::Error::InvalidIdentity,
            ),
        ] {
            let capture = Capture::new(true);
            let binding = ic_timers::bind_once(
                identity.each_ref().map(String::as_str),
                ic_timers::Lifetime::Retained,
                move || {
                    capture.touch();
                    async { result() }
                },
            );
            assert!(matches!(binding, Err(error) if error == expected));
            let capture = Capture::new(true);
            let binding = ic_timers::bind_watchdog(
                identity.each_ref().map(String::as_str),
                NonZeroU64::new(1_000_000_000).unwrap(),
                ic_timers::Lifetime::Retained,
                move || {
                    capture.touch();
                    WatchdogResult {
                        observation: Observation::NoWork,
                        next: WatchdogNext::Stop,
                    }
                },
            );
            assert!(matches!(binding, Err(error) if error == expected));
        }
    }
    state();
}

/// Execute every retained callback once, with real awaits for Once declarations.
/// # Panics
/// Traps on control failure.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update retention_run")]
pub extern "C" fn run() {
    HANDLES.with_borrow(|handles| {
        for handle in &handles.0 {
            handle.set(OnceDemand::At(0)).unwrap();
        }
        for handle in &handles.1 {
            handle.set(WatchdogDemand::At(0)).unwrap();
        }
    });
    ic0::msg_reply();
}

/// Drop inactive handles (0), or arm all declarations one second ahead first (1).
/// # Panics
/// Traps if dropping handles frees capacity or permits identity replacement.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update retention_drop_handles")]
pub extern "C" fn drop_handles() {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut mode = [0];
    ic0::msg_arg_data_copy(&mut mode, 0);
    assert!(mode[0] <= 1);
    // Drop outside the fixture borrow too, so a regression can run capture Drop.
    let handles = HANDLES.with_borrow_mut(std::mem::take);
    if mode[0] == 1 {
        let at = system::time().checked_add(1_000_000_000).unwrap();
        for handle in &handles.0 {
            handle.set(OnceDemand::At(at)).unwrap();
        }
        for handle in &handles.1 {
            handle.set(WatchdogDemand::At(at)).unwrap();
        }
    }
    drop(handles);
    for (identity, expected) in [
        (
            ["o".repeat(64), "g".repeat(64), format!("{:064}", 0)],
            ic_timers::Error::DuplicateIdentity,
        ),
        (
            ["o".repeat(64), "g".repeat(64), format!("{:064}", 1)],
            ic_timers::Error::DuplicateIdentity,
        ),
        (
            ["new".into(), "new".into(), "new".into()],
            ic_timers::Error::Capacity,
        ),
    ] {
        assert!(matches!(
            ic_timers::bind_once(identity.each_ref().map(String::as_str), ic_timers::Lifetime::Retained, || async { result() }),
            Err(error) if error == expected
        ));
    }
    state();
}

/// Five custody counters and coarse linear-memory pages (not live allocated bytes).
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query retention_state")]
pub extern "C" fn state() {
    COUNTS.with_borrow(|counts| {
        for count in counts {
            ic0::msg_reply_data_append(&count.to_le_bytes());
        }
    });
    ic0::msg_reply_data_append(&(std::arch::wasm32::memory_size::<0>() as u64).to_le_bytes());
    ic0::msg_reply();
}
