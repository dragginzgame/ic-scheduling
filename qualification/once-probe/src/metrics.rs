//! Host-owned encoding of aggregate diagnostics; no product exporter callbacks.
use super::ic0;

/// Six u128 totals followed by their six overflow flags.
/// # Panics
/// Traps if the provider has not been initialized.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_query timer_metrics")]
pub extern "C" fn metrics() {
    let metrics = ic_timers::metrics().unwrap();
    let counters = [
        metrics.accepted,
        metrics.completed,
        metrics.abandoned,
        metrics.expected_failures,
        metrics.deferred_submissions,
        metrics.provider_faults,
    ];
    for counter in counters {
        ic0::msg_reply_data_append(&counter.total().to_le_bytes());
    }
    for counter in counters {
        ic0::msg_reply_data_append(&[u8::from(counter.total_overflowed())]);
    }
    ic0::msg_reply();
}

/// Exercise both ordinary outcome variants through both timer kinds, with
/// reentrant metrics reads and initialization that must not reset counters.
/// # Panics
/// Traps on failed admission or incorrect outcome accounting.
#[expect(unsafe_code, reason = "qualification owns this endpoint")]
#[unsafe(export_name = "canister_update metrics_setup")]
pub extern "C" fn setup() {
    ic_timers::initialize().unwrap();
    for index in 0..2 {
        let once = ic_timers::bind_once(
            ["metrics", "once", &index.to_string()],
            move || async move {
                let before = ic_timers::metrics().unwrap();
                ic_timers::initialize().unwrap();
                assert_eq!(before, ic_timers::metrics().unwrap());
                super::OnceResult {
                    observation: if index == 0 {
                        super::Observation::NoWork
                    } else {
                        super::Observation::ExpectedFailure
                    },
                    next: super::OnceNext::Stop,
                }
            },
        )
        .unwrap();
        once.set(super::OnceDemand::At(0)).unwrap();
        let watchdog = ic_timers::bind_watchdog(
            ["metrics", "watchdog", &index.to_string()],
            std::num::NonZeroU64::new(1_000_000_000).unwrap(),
            move || {
                let _read = ic_timers::metrics().unwrap();
                ic_timers::WatchdogResult {
                    observation: if index == 0 {
                        super::Observation::NoWork
                    } else {
                        super::Observation::ExpectedFailure
                    },
                    next: ic_timers::WatchdogNext::Stop,
                }
            },
        )
        .unwrap();
        watchdog.set(ic_timers::WatchdogDemand::At(0)).unwrap();
    }
    ic0::msg_reply();
}
