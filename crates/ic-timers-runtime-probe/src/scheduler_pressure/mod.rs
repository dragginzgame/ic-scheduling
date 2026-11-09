//! Real instruction pressure ahead of the Watchdog's first self-dispatch.
use ic_timers::{
    OnceDecision, OnceRegistration, OnceRunResult, TimerCadence, TimerCompletion, TimerIdentity,
    TimerSchedule, TimerStart, WatchdogReconcileState, reconcile_watchdog, register_once,
};
use ic_timers_test_protocol::rpc::SchedulerReconcileMode;
use std::{
    cell::{Cell, RefCell},
    time::Duration,
};

// Each timer consumes more than one execution slice without trapping. The test
// requires an observed completed predecessor and a still-unstarted scheduler;
// elapsed time or these budgets alone are never treated as abandonment proof.
const PRESSURE_TIMERS: usize = 8;
const PRESSURE_INSTRUCTIONS: u64 = 3_000_000_000;

thread_local! {
    static PRESSURE: RefCell<Vec<OnceRegistration>> = const { RefCell::new(Vec::new()) };
    static COMPLETED: Cell<u64> = const { Cell::new(0) };
}

ic_timers_test_protocol::probe_endpoints! {
    #[endpoint(update, StartSchedulerPressure)]
    fn start_scheduler_pressure((successor,): (bool,)) {
        let delay = if successor {
            let deadline = ic_timers::timer_snapshot(&super::probe_identity())
                .unwrap()
                .unwrap()
                .next_deadline_ns()
                .unwrap();
            Duration::from_nanos(
                deadline
                    .checked_sub(ic_platform_runtime::ic0::time())
                    .unwrap()
                    - 1,
            )
        } else {
            Duration::from_secs(1)
        };
        PRESSURE.with_borrow_mut(|registrations| {
            assert!(registrations.is_empty());
            for index in 0..PRESSURE_TIMERS {
                registrations.push(
                    register_once(
                        TimerIdentity::try_new("fixture", "round-pressure", format!("{index}"))
                            .unwrap(),
                        TimerStart::Retained {
                            initial: Some(TimerSchedule::After(delay)),
                        },
                        |_| async {
                            let end = ic_platform_runtime::ic0::performance_counter(0)
                                + PRESSURE_INSTRUCTIONS;
                            while ic_platform_runtime::ic0::performance_counter(0) < end {
                                std::hint::black_box(());
                            }
                            COMPLETED.set(COMPLETED.get() + 1);
                            OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
                        },
                    )
                    .unwrap(),
                );
            }
        });
        if !successor {
            assert!(super::start());
        }
    }

    #[endpoint(query, SchedulerPressureCompleted)]
    fn scheduler_pressure_completed() -> u64 {
        COMPLETED.get()
    }

    #[endpoint(update, ReconcileSchedulerPressure)]
    fn reconcile_scheduler_pressure((mode, deadline): (SchedulerReconcileMode, u64)) -> bool {
        let desired = match mode {
            SchedulerReconcileMode::Scheduled => WatchdogReconcileState::Scheduled,
            SchedulerReconcileMode::ScheduledImmediately => {
                WatchdogReconcileState::ScheduledImmediately
            }
            SchedulerReconcileMode::ScheduledAt => WatchdogReconcileState::ScheduledAt(deadline),
        };
        super::REGISTRATION.with_borrow_mut(|slot| {
            reconcile_watchdog(
                slot,
                &super::probe_identity(),
                TimerCadence::from_nanos(super::CADENCE_NS).unwrap(),
                desired,
                |_| panic!("retained callback must not be replaced"),
            )
            .is_ok()
        })
    }
}
