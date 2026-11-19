//! Standalone Wasm checks of the Saffron UTC adapter, not installed IC behavior.

use ic_jobs::Schedule;

/// Find a UTC occurrence in a fixed qualification scenario.
///
/// Scenarios: daily 09:00, last day, second Monday, leap day, impossible date,
/// and every minute. Zero represents no occurrence in the supplied window.
///
/// # Panics
/// Panics for an unknown scenario or an invalid compiled fixture.
#[expect(
    unsafe_code,
    reason = "the qualification module owns this raw Wasm export"
)]
#[unsafe(no_mangle)]
pub extern "C" fn calendar_next(scenario: u32, after_ns: u64, through_ns: u64) -> u64 {
    let expression = match scenario {
        0 => "0 9 * * *",
        1 => "0 9 L * *",
        2 => "0 9 * * MON#2",
        3 => "0 9 29 FEB *",
        4 => "0 9 31 FEB *",
        5 => "* * * * *",
        _ => panic!("unknown calendar scenario"),
    };

    Schedule::parse(expression)
        .expect("fixture schedule")
        .next_after(after_ns, through_ns)
        .unwrap_or(0)
}
