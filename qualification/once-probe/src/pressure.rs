//! Qualification-only many-declaration workload; holds real calls in Testkit.
use super::{
    Call, Observation, Once, OnceDemand, OnceNext, OnceResult, Principal, RefCell, Wait, ic0,
};
use ic_platform_runtime::system;

thread_local! {
    static HANDLES: RefCell<Vec<Once>> = const { RefCell::new(Vec::new()) };
    static COUNTS: RefCell<([u8; 64], [u8; 64])> = const { RefCell::new(([0;64], [0;64])) };
}

/// Bind the full declaration table, arm twelve calls to a held-reply canister.
/// # Panics
/// Traps on invalid fixture setup or an unexpected provider admission result.
#[expect(unsafe_code, reason = "qualification owns this entrypoint")]
#[unsafe(export_name = "canister_update pressure_setup")]
pub extern "C" fn setup() {
    let len = ic0::msg_arg_data_size();
    assert!(len <= 29 && len > 0);
    let mut bytes = [0; 29];
    ic0::msg_arg_data_copy(&mut bytes[..len], 0);
    let target = Principal::from_slice(&bytes[..len]);
    ic_timers::initialize().unwrap();
    for index in 0..64 {
        let handle = ic_timers::bind_once(
            ["pressure", "once", &index.to_string()],
            ic_timers::Lifetime::Retained,
            move || async move {
                COUNTS.with(|counts| counts.borrow_mut().0[index] += 1);
                let response = Call::new(target, "hold", Wait::Unbounded).execute().await;
                drop(crate::expect_reply(response));
                COUNTS.with(|counts| counts.borrow_mut().1[index] += 1);
                OnceResult {
                    observation: Observation::Success,
                    next: OnceNext::Stop,
                }
            },
        )
        .unwrap();
        if index < 12 {
            handle.set(OnceDemand::At(0)).unwrap();
        }
        HANDLES.with(|handles| handles.borrow_mut().push(handle));
    }
    assert!(matches!(
        ic_timers::bind_once(
            ["pressure", "once", "overflow"],
            ic_timers::Lifetime::Retained,
            || async {
                OnceResult {
                    observation: Observation::NoWork,
                    next: OnceNext::Stop,
                }
            }
        ),
        Err(ic_timers::Error::Capacity)
    ));
    ic0::msg_reply();
}

/// Repeated equal demand followed by Inactive must not start concurrent futures.
/// Cancel the four declarations still waiting for flight capacity as well.
/// # Panics
/// Traps if the driver calls this before setup or control fails.
#[expect(unsafe_code, reason = "qualification owns this entrypoint")]
#[unsafe(export_name = "canister_update pressure_cancel")]
pub extern "C" fn cancel() {
    HANDLES.with(|handles| {
        for handle in handles.borrow().iter().take(12) {
            for _ in 0..3 {
                handle.set(OnceDemand::At(0)).unwrap();
            }
            handle.set(OnceDemand::Inactive).unwrap();
        }
    });
    ic0::msg_reply();
}

/// Rearm the four unaccepted declarations after capacity is released.
/// # Panics
/// Traps if setup or provider control failed.
#[expect(unsafe_code, reason = "qualification owns this entrypoint")]
#[unsafe(export_name = "canister_update pressure_rearm")]
pub extern "C" fn rearm() {
    HANDLES.with(|handles| {
        for handle in handles.borrow().iter().skip(8).take(4) {
            handle.set(OnceDemand::At(0)).unwrap();
        }
    });
    ic0::msg_reply();
}

/// Return per-declaration start and completion counts for independent assertions.
#[expect(unsafe_code, reason = "qualification owns this entrypoint")]
#[unsafe(export_name = "canister_query pressure_counts")]
pub extern "C" fn counts() {
    COUNTS.with(|counts| {
        let counts = counts.borrow();
        ic0::msg_reply_data_append(&counts.0);
        ic0::msg_reply_data_append(&counts.1);
    });
    ic0::msg_reply();
}

/// Deliberately violate timer ownership, then assert a terminal control fault.
/// # Panics
/// Traps if setup is absent or the provider fails to expose the foreign write.
#[expect(unsafe_code, reason = "qualification owns this entrypoint")]
#[unsafe(export_name = "canister_update pressure_interfere")]
pub extern "C" fn interfere() {
    system::global_timer_set(system::time().checked_add(1_000_000_000).unwrap());
    HANDLES.with(|handles| {
        let handles = handles.borrow();
        assert_eq!(
            handles[0].set(OnceDemand::At(0)),
            Err(ic_timers::Error::Faulted)
        );
        for handle in handles.iter() {
            assert!(matches!(
                handle.status().unwrap(),
                super::OnceStatus::Faulted
            ));
        }
        assert_eq!(
            handles[0].set(OnceDemand::Inactive),
            Err(ic_timers::Error::Faulted)
        );
    });
    ic0::msg_reply();
}
