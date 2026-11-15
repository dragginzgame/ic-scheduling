//! Host-owned future-deadline reconstruction and raw self-dispatch qualification.
use super::{
    Call, Cell, Observation, Once, OnceDemand, OnceNext, OnceResult, OnceStatus, RefCell,
    WaitPolicy, ic0, self_id,
};
use ic_platform_runtime::executor;

thread_local! {
    static HANDLE: RefCell<Option<Once>> = const { RefCell::new(None) };
    static DEADLINE: Cell<u64> = const { Cell::new(0) };
    static EPOCH: Cell<u64> = const { Cell::new(0) };
    static RUNS: Cell<u8> = const { Cell::new(0) };
    static TARGET: Cell<Option<super::Principal>> = const { Cell::new(None) };
}
fn bind(deadline: u64) {
    ic_timers::initialize().unwrap();
    EPOCH.with(|epoch| epoch.set(ic0::canister_version()));
    DEADLINE.with(|value| value.set(deadline));
    let handle = ic_timers::bind_once(["lifecycle", "once", "future"], || async {
        RUNS.with(|count| count.set(count.get() + 1));
        if let Some(target) = TARGET.with(Cell::get) {
            let response = Call::new(target, "hold", WaitPolicy::Unbounded)
                .execute()
                .await;
            assert!(response.is_ok());
        }
        OnceResult {
            observation: Observation::Success,
            next: OnceNext::Stop,
        }
    })
    .unwrap();
    handle.set(OnceDemand::At(deadline)).unwrap();
    HANDLE.with(|slot| *slot.borrow_mut() = Some(handle));
}

/// Opt-in fixture installation argument 1 binds a deadline ten seconds ahead.
/// Empty arguments leave the other fixture workloads unaffected.
/// # Panics
/// Traps on malformed setup or failed provider admission.
#[expect(unsafe_code, reason = "qualification owns this lifecycle hook")]
#[unsafe(export_name = "canister_init")]
pub extern "C" fn init() {
    if ic0::msg_arg_data_size() == 0 {
        return;
    }
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut args = [0];
    ic0::msg_arg_data_copy(&mut args, 0);
    assert_eq!(args, [1]);
    bind(ic0::time().checked_add(10_000_000_000).unwrap());
}

/// This host fixture persists a Jobs checkpoint or an Armed Once declaration.
/// It deliberately does not claim to reconcile accepted work across upgrades.
/// # Panics
/// Refuses unresolved Jobs work, non-Armed Once state or failed stable writes.
#[expect(unsafe_code, reason = "qualification owns this lifecycle hook")]
#[unsafe(export_name = "canister_pre_upgrade")]
pub extern "C" fn pre_upgrade() {
    if super::jobs_completion::pre_upgrade() {
        return;
    }
    if super::jobs_catch_up::pre_upgrade() {
        return;
    }
    if super::jobs::pre_upgrade() {
        return;
    }
    let deadline = HANDLE.with(
        |slot| match slot.borrow().as_ref().unwrap().status().unwrap() {
            OnceStatus::Armed(at) => at,
            _ => ic0::trap(b"host requires armed future for upgrade"),
        },
    );
    if ic0::stable64_size() == 0 {
        assert_eq!(ic0::stable64_grow(1), 0);
    }
    let mut bytes = [0; 16];
    bytes[..8].copy_from_slice(b"ONCEDEMO");
    bytes[8..].copy_from_slice(&deadline.to_le_bytes());
    ic0::stable64_write(&bytes, 0);
}

/// Rebind fresh volatile capabilities from host-owned persisted demand.
/// # Panics
/// Traps if the fixture's stable record is invalid or provider admission fails.
#[expect(unsafe_code, reason = "qualification owns this lifecycle hook")]
#[unsafe(export_name = "canister_post_upgrade")]
pub extern "C" fn post_upgrade() {
    if super::jobs_completion::post_upgrade() {
        return;
    }
    if super::jobs_catch_up::post_upgrade() {
        return;
    }
    if super::jobs::post_upgrade() {
        return;
    }
    let mut bytes = [0; 16];
    ic0::stable64_read(&mut bytes, 0);
    assert_eq!(&bytes[..8], b"ONCEDEMO");
    bind(u64::from_le_bytes(bytes[8..].try_into().unwrap()));
}

/// Epoch and absolute deadline (u64 LE), then committed run count.
/// The last byte is status: 0 inactive, 1 armed, 2 running, 3 recovery, 4 fault.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query lifecycle_state")]
pub extern "C" fn state() {
    ic0::msg_reply_data_append(&EPOCH.with(Cell::get).to_le_bytes());
    ic0::msg_reply_data_append(&DEADLINE.with(Cell::get).to_le_bytes());
    ic0::msg_reply_data_append(&[RUNS.with(Cell::get)]);
    let status = HANDLE.with(
        |slot| match slot.borrow().as_ref().unwrap().status().unwrap() {
            OnceStatus::Inactive => 0,
            OnceStatus::Armed(_) => 1,
            OnceStatus::Running => 2,
            OnceStatus::RecoveryRequired(_) => 3,
            OnceStatus::Faulted => 4,
        },
    );
    ic0::msg_reply_data_append(&[status]);
    ic0::msg_reply();
}

/// Set a held-reply target and arm the existing lifecycle declaration immediately.
/// # Panics
/// Traps on malformed setup or absent/blocked declaration.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update lifecycle_hold")]
pub extern "C" fn hold() {
    let len = ic0::msg_arg_data_size();
    assert!(len > 0 && len <= 29);
    let mut bytes = [0; 29];
    ic0::msg_arg_data_copy(&mut bytes[..len], 0);
    TARGET.with(|target| target.set(Some(super::Principal::from_slice(&bytes[..len]))));
    HANDLE.with(|slot| {
        slot.borrow()
            .as_ref()
            .unwrap()
            .set(OnceDemand::At(0))
            .unwrap()
    });
    ic0::msg_reply();
}

/// After settlement, establish fresh future demand eligible for host reconstruction.
/// # Panics
/// Traps unless the held attempt has completed and control succeeds.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update lifecycle_rearm")]
pub extern "C" fn rearm() {
    HANDLE.with(|slot| {
        let slot = slot.borrow();
        let handle = slot.as_ref().unwrap();
        assert!(matches!(handle.status().unwrap(), OnceStatus::Inactive));
        let deadline = ic0::time().checked_add(10_000_000_000).unwrap();
        handle.set(OnceDemand::At(deadline)).unwrap();
        DEADLINE.with(|value| value.set(deadline));
    });
    TARGET.with(|target| target.set(None));
    ic0::msg_reply();
}

/// Qualification-only self-call relay to exercise authenticated envelope parsing.
/// Replies with 1 on acceptance/no-op and 0 on rejection. No production bypass.
/// # Panics
/// Refuses relay payloads over 17 bytes.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update wire_relay")]
pub extern "C" fn relay() {
    let size = ic0::msg_arg_data_size();
    assert!(size <= 17);
    let mut bytes = vec![0; size];
    ic0::msg_arg_data_copy(&mut bytes, 0);
    executor::in_tracking_executor_context(|| {
        let _task = executor::spawn_protected(async move {
            let response = Call::new(self_id(), "__ic_scheduling_once", WaitPolicy::Unbounded)
                .take_raw_args(bytes)
                .execute()
                .await;
            ic0::msg_reply_data_append(&[u8::from(response.is_ok())]);
            ic0::msg_reply();
        });
    });
}
