//! Real cycle pressure, confined to an installed qualification canister.
use super::{Cell, Observation, Once, OnceDemand, OnceNext, OnceResult, OnceStatus, RefCell, ic0};
use ic_platform_runtime::cdk::api;

thread_local! {
    static HANDLE: RefCell<Option<Once>> = const { RefCell::new(None) };
    static STARTS: Cell<u8> = const { Cell::new(0) };
}

/// Arm work, then reduce liquid cycles below the self-call's quoted cost.
/// # Panics
/// Traps if fixture admission or cycle reduction fails.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update refusal_setup")]
pub extern "C" fn setup() {
    ic_timers::initialize().unwrap();
    let handle = ic_timers::bind_once(["refusal", "once", "work"], || async {
        STARTS.with(|starts| starts.set(starts.get() + 1));
        OnceResult {
            observation: Observation::Success,
            next: OnceNext::Stop,
        }
    })
    .unwrap();
    handle.set(OnceDemand::At(0)).unwrap();
    HANDLE.with(|slot| *slot.borrow_mut() = Some(handle));
    let timer = ic0::global_timer_set(0);
    ic0::global_timer_set(timer);
    let cost = ic0::cost_call("__ic_scheduling_once".len() as u64, 16);
    let balance = ic0::canister_liquid_cycle_balance128();
    let burn = balance.checked_sub(cost / 2).unwrap();
    assert_eq!(api::cycles_burn(burn), burn);
    ic0::msg_reply_data_append(&timer.to_le_bytes());
    ic0::msg_reply_data_append(&cost.to_le_bytes());
    ic0::msg_reply();
}

/// Observe the timer without changing its value, plus status and callback starts.
/// # Panics
/// Traps if called before setup.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update refusal_state")]
pub extern "C" fn state() {
    // The setter is unavailable in queries. Restore the exact value so the
    // provider's ownership observation remains valid.
    let timer = ic0::global_timer_set(0);
    ic0::global_timer_set(timer);
    let status = HANDLE.with(
        |slot| match slot.borrow().as_ref().unwrap().status().unwrap() {
            OnceStatus::Armed(_) => 1,
            OnceStatus::Inactive => 0,
            _ => 2,
        },
    );
    ic0::msg_reply_data_append(&timer.to_le_bytes());
    ic0::msg_reply_data_append(&[status, STARTS.with(Cell::get)]);
    ic0::msg_reply();
}
