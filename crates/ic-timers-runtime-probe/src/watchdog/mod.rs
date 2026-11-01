use ic_timers::{
    TimerCadence, TimerIdentity, TimerSchedule, TimerStart, WatchdogContext, WatchdogRegistration,
    WatchdogRunResult, register_watchdog,
};
use std::time::Duration;

pub(super) fn install(
    slot: &mut Option<WatchdogRegistration>,
    identity: TimerIdentity,
    initial_delay_ns: u64,
    callback: impl FnMut(WatchdogContext) -> WatchdogRunResult + 'static,
) -> bool {
    if slot.is_some() {
        return false;
    }
    let cadence = TimerCadence::from_nanos(super::CADENCE_NS)
        .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("probe cadence is invalid"));
    let registration = register_watchdog(
        identity,
        cadence,
        TimerStart::Retained {
            initial: Some(TimerSchedule::After(Duration::from_nanos(initial_delay_ns))),
        },
        callback,
    )
    .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("watchdog registration failed"));
    *slot = Some(registration);
    true
}

#[inline(never)]
pub(super) fn exhaust_message_instructions() -> ! {
    loop {
        std::hint::black_box(ic_platform_runtime::ic0::performance_counter(0));
    }
}

pub(super) fn in_window(window: Option<(u64, u64)>, now_ns: u64) -> bool {
    window.is_some_and(|(start, end)| now_ns >= start && now_ns < end)
}
