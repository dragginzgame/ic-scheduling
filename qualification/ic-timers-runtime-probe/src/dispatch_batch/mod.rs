//! Maximum-capacity dispatch through real queue pressure and suspended callbacks.

use ic_timers::{
    MAX_TIMER_REGISTRATIONS, OnceDecision, OnceRegistration, OnceRunResult, TimerCompletion,
    TimerIdentity, TimerSchedule, TimerStart, register_once, timer_inventory,
};
use ic_timers_test_protocol::DispatchBatchObservation;
use std::{
    cell::{Cell, RefCell},
    time::Duration,
};

const COUNTERS: usize = MAX_TIMER_REGISTRATIONS + 1;

thread_local! {
    static REGISTRATIONS: RefCell<Vec<Option<OnceRegistration>>> = const { RefCell::new(Vec::new()) };
    static STARTS: RefCell<[u64; COUNTERS]> = const { RefCell::new([0; COUNTERS]) };
    static COMPLETIONS: RefCell<[u64; COUNTERS]> = const { RefCell::new([0; COUNTERS]) };
    static ACTIVE: Cell<u64> = const { Cell::new(0) };
    static PEAK_ACTIVE: Cell<u64> = const { Cell::new(0) };
    static RELEASED: Cell<bool> = const { Cell::new(false) };
}

fn install(index: usize, counter: usize) -> OnceRegistration {
    register_once(
        TimerIdentity::try_new("ic-timers", "dispatch-batch", format!("timer-{index:02}"))
            .expect("bounded batch identity"),
        TimerStart::Retained {
            initial: Some(TimerSchedule::After(Duration::ZERO)),
        },
        move |_| async move {
            STARTS.with_borrow_mut(|counts| counts[counter] += 1);
            ACTIVE.set(ACTIVE.get() + 1);
            PEAK_ACTIVE.set(PEAK_ACTIVE.get().max(ACTIVE.get()));
            loop {
                // A real call keeps each protected task's IC context alive.
                if super::codec::call_gate(ic_timers_test_protocol::rpc::Method::DispatchBatchGate)
                    .await
                    .expect("batch gate exchange")
                {
                    break;
                }
            }
            COMPLETIONS.with_borrow_mut(|counts| counts[counter] += 1);
            ACTIVE.set(ACTIVE.get() - 1);
            OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
        },
    )
    .expect("batch registration")
}

ic_timers_test_protocol::probe_endpoints! {
    #[endpoint(update, StartDispatchBatch)]
    fn start_dispatch_batch() {
        assert!(REGISTRATIONS.with_borrow(Vec::is_empty), "fresh batch");
        super::transport::start_pressure();
        REGISTRATIONS.with_borrow_mut(|slots| {
            for index in 0..MAX_TIMER_REGISTRATIONS {
                slots.push(Some(install(index, index)));
            }
        });
    }

    #[endpoint(update, ChangePendingDispatchBatch)]
    fn change_pending_dispatch_batch() {
        REGISTRATIONS.with_borrow_mut(|slots| {
            slots[0]
                .as_ref()
                .expect("first registration")
                .cancel()
                .expect("cancel pending timer");
            slots[1]
                .take()
                .expect("second registration")
                .unregister()
                .expect("remove pending timer");
            slots[1] = Some(install(1, MAX_TIMER_REGISTRATIONS));
        });
    }

    #[endpoint(update, RearmCancelledDispatchBatchTimer)]
    fn rearm_cancelled_dispatch_batch_timer() {
        REGISTRATIONS.with_borrow(|slots| {
            slots[0]
                .as_ref()
                .expect("cancelled registration retained")
                .ensure_scheduled(TimerSchedule::After(Duration::ZERO))
                .expect("rearm after batch drains");
        });
    }

    #[endpoint(update, ReleaseDispatchBatch)]
    fn release_dispatch_batch() {
        RELEASED.set(true);
    }

    #[endpoint(update, DispatchBatchGate)]
    fn dispatch_batch_gate() -> bool {
        RELEASED.get()
    }

    #[endpoint(query, DispatchBatchObservation)]
    fn dispatch_batch_observation() -> DispatchBatchObservation {
        DispatchBatchObservation {
            starts: STARTS.with_borrow(|counts| counts.to_vec()),
            completions: COMPLETIONS.with_borrow(|counts| counts.to_vec()),
            active: ACTIVE.get(),
            peak_active: PEAK_ACTIVE.get(),
            inventory_len: timer_inventory().expect("batch inventory").len() as u64,
        }
    }
}
