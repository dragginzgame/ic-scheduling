//! Message-prefix instruction samples for actual dispatch and async resumption.
use super::{
    Call, Cell, Observation, Once, OnceDemand, OnceNext, OnceResult, RefCell, WaitPolicy, ic0,
    self_id,
};
thread_local! {
    static HANDLES: RefCell<Vec<Once>> = const { RefCell::new(Vec::new()) };
    static SAMPLES: RefCell<[[u64; 4]; 64]> = const { RefCell::new([[0; 4]; 64]) };
    static FINISHED: Cell<u8> = const { Cell::new(0) };
}

/// Bind and arm a bounded batch. Samples use per-message counter 0, never deltas
/// across awaits. The first raw argument is declaration count (1, 8 or 64).
/// # Panics
/// Traps on invalid setup or failed scheduler operations.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update dispatch_costs_setup")]
pub extern "C" fn setup() {
    assert_eq!(ic0::msg_arg_data_size(), 1);
    let mut args = [0];
    ic0::msg_arg_data_copy(&mut args, 0);
    assert!(matches!(args[0], 1 | 8 | 64));
    ic_timers::initialize().unwrap();
    for index in 0..usize::from(args[0]) {
        let handle =
            ic_timers::bind_once(["dispatch-cost", "once", &index.to_string()], move || {
                let factory_prefix = ic0::performance_counter(0);
                SAMPLES.with(|samples| samples.borrow_mut()[index][0] = factory_prefix);
                async move {
                    let poll_prefix = ic0::performance_counter(0);
                    SAMPLES.with(|samples| samples.borrow_mut()[index][1] = poll_prefix);
                    let response =
                        Call::new(self_id(), "dispatch_costs_echo", WaitPolicy::Unbounded)
                            .execute()
                            .await;
                    let resume_prefix = ic0::performance_counter(0);
                    let echo = response.unwrap().into_bytes();
                    assert_eq!(echo.len(), 8);
                    let echo_prefix = u64::from_le_bytes(echo.try_into().unwrap());
                    SAMPLES.with(|samples| samples.borrow_mut()[index][3] = echo_prefix);
                    SAMPLES.with(|samples| samples.borrow_mut()[index][2] = resume_prefix);
                    FINISHED.with(|finished| finished.set(finished.get() + 1));
                    OnceResult {
                        observation: Observation::Success,
                        next: OnceNext::Stop,
                    }
                }
            })
            .unwrap();
        handle.set(OnceDemand::At(0)).unwrap();
        HANDLES.with(|handles| handles.borrow_mut().push(handle));
    }
    ic0::msg_reply_data_append(&[FINISHED.with(Cell::get)]);
    ic0::msg_reply();
}

/// A separate real message to force suspension and later resumption.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update dispatch_costs_echo")]
pub extern "C" fn echo() {
    let prefix = ic0::performance_counter(0);
    ic0::msg_reply_data_append(&prefix.to_le_bytes());
    ic0::msg_reply();
}

/// Return completed/inactive counts, then four u64 LE samples per declaration.
/// # Panics
/// Traps on unexpected capability errors.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query dispatch_costs_state")]
pub extern "C" fn state() {
    HANDLES.with(|handles| {
        let handles = handles.borrow();
        let inactive = handles
            .iter()
            .filter(|handle| matches!(handle.status().unwrap(), ic_timers::OnceStatus::Inactive))
            .count();
        ic0::msg_reply_data_append(&[FINISHED.with(Cell::get), inactive as u8]);
        SAMPLES.with(|samples| {
            for row in samples.borrow().iter().take(handles.len()) {
                for value in row {
                    ic0::msg_reply_data_append(&value.to_le_bytes());
                }
            }
        });
    });
    ic0::msg_reply();
}

/// Separate raw-entry/context/task baselines, without importing scheduler state.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update executor_costs_baseline")]
pub extern "C" fn executor_baseline() {
    let entry = ic0::performance_counter(0);
    ic_platform_runtime::executor::in_tracking_executor_context(|| {
        let context = ic0::performance_counter(0);
        let _task = ic_platform_runtime::executor::spawn_protected(async move {
            let poll = ic0::performance_counter(0);
            for value in [entry, context, poll] {
                ic0::msg_reply_data_append(&value.to_le_bytes());
            }
            ic0::msg_reply();
        });
    });
}
