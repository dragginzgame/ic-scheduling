use ic_timers::{
    MAX_TIMER_REGISTRATIONS, MemoryPageSummary, TimerCadence, TimerCompletion, TimerIdentity,
    TimerLastOutcome, TimerSchedule, TimerSchedulingMode, TimerStart, WatchdogDecision,
    WatchdogRegistration, WatchdogRunResult, initialize_runtime, register_watchdog,
    timer_inventory, timer_snapshot,
};
use ic_timers_test_protocol::{
    CounterObservation, ProbeMemorySummary, ProbeSnapshot, RuntimeCheckpoint,
};
use std::cell::{Cell, RefCell};

const CADENCE_NS: u64 = 1_000_000_000;

mod checkpoint;
mod churn;
mod codec;
mod dispatch_batch;
mod mixed;
mod ordinary;
mod scheduler_pressure;
mod scheduling_trace;
mod transport;
mod watchdog;

// One host-owned hook composes timer exclusion with its own payload policy.
// Inter-canister calls bypass this hook and must still authenticate at dispatch.
#[ic_platform_runtime::cdk::inspect_message]
fn inspect_message() {
    if ic_timers::is_timer_dispatch_method(&ic_platform_runtime::cdk::api::msg_method_name()) {
        ic_platform_runtime::cdk::api::trap("probe ingress: timer dispatch is internal");
    }
    if ic_platform_runtime::ic0::msg_arg_data_size()
        > ic_timers_test_protocol::rpc::REQUEST_LIMITS.bytes
    {
        ic_platform_runtime::cdk::api::trap("probe ingress: host payload limit");
    }
    ic_platform_runtime::cdk::api::accept_message();
}

thread_local! {
    static REGISTRATION: RefCell<Option<WatchdogRegistration>> = const { RefCell::new(None) };
    static SECONDARY_REGISTRATION: RefCell<Option<WatchdogRegistration>> = const { RefCell::new(None) };
    static NEXT_EXTRA_NAME: Cell<u64> = const { Cell::new(0) };
    static COMPLETED_WORK: Cell<u64> = const { Cell::new(0) };
    static SECONDARY_COMPLETED_WORK: Cell<u64> = const { Cell::new(0) };
    static TRAP_WINDOW: Cell<Option<(u64, u64)>> = const { Cell::new(None) };
    static EXHAUST_WINDOW: Cell<Option<(u64, u64)>> = const { Cell::new(None) };
    static NEXT_WORK_DECISION: Cell<WatchdogDecision> = const { Cell::new(WatchdogDecision::Continue) };
    static DESIRED_SCHEDULED: Cell<bool> = const { Cell::new(false) };
    static SECONDARY_DESIRED_SCHEDULED: Cell<bool> = const { Cell::new(false) };
    static POST_UPGRADE_RECONSTRUCTED: Cell<bool> = const { Cell::new(false) };
}

#[ic_platform_runtime::cdk::init]
fn init() {
    if initialize_runtime().is_err() {
        ic_platform_runtime::cdk::api::trap("ic-timers runtime initialization failed");
    }
}

#[ic_platform_runtime::cdk::pre_upgrade]
fn pre_upgrade() {
    let state = RuntimeCheckpoint {
        desired_scheduled: DESIRED_SCHEDULED.with(Cell::get),
        secondary_desired_scheduled: SECONDARY_DESIRED_SCHEDULED.with(Cell::get),
        completed_work: COMPLETED_WORK.with(Cell::get),
        secondary_completed_work: SECONDARY_COMPLETED_WORK.with(Cell::get),
    };
    checkpoint::save(&state);
}

#[ic_platform_runtime::cdk::post_upgrade]
fn post_upgrade() {
    let state = checkpoint::load();
    DESIRED_SCHEDULED.with(|desired| desired.set(state.desired_scheduled));
    SECONDARY_DESIRED_SCHEDULED.with(|desired| desired.set(state.secondary_desired_scheduled));
    COMPLETED_WORK.with(|completed| completed.set(state.completed_work));
    SECONDARY_COMPLETED_WORK.with(|completed| completed.set(state.secondary_completed_work));
    if initialize_runtime().is_err() {
        ic_platform_runtime::cdk::api::trap("post-upgrade runtime initialization failed");
    }
    if state.desired_scheduled && !install_watchdog() {
        ic_platform_runtime::cdk::api::trap("post-upgrade watchdog reconstruction failed");
    }
    if state.secondary_desired_scheduled && !install_secondary_watchdog() {
        ic_platform_runtime::cdk::api::trap(
            "post-upgrade secondary watchdog reconstruction failed",
        );
    }
    let reconstructed = timer_snapshot(&probe_identity())
        .ok()
        .flatten()
        .and_then(|snapshot| snapshot.next_deadline_ns())
        .is_some();
    POST_UPGRADE_RECONSTRUCTED.with(|observed| observed.set(reconstructed));
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::Start as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::Start as u64 }>"
)]
fn start() -> bool {
    DESIRED_SCHEDULED.with(|desired| desired.set(true));
    install_watchdog()
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::StartImmediately as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::StartImmediately as u64 }>"
)]
fn start_immediately() -> bool {
    DESIRED_SCHEDULED.with(|desired| desired.set(true));
    install_watchdog_with_initial_schedule(true)
}

fn install_watchdog() -> bool {
    install_watchdog_with_initial_schedule(false)
}

fn install_watchdog_with_initial_schedule(immediate: bool) -> bool {
    REGISTRATION.with_borrow_mut(|slot| {
        watchdog::install(
            slot,
            probe_identity(),
            if immediate { 0 } else { CADENCE_NS },
            |context| {
                let now_ns = ic_platform_runtime::cdk::api::time();
                let should_trap = watchdog::in_window(TRAP_WINDOW.get(), now_ns);
                let should_exhaust = watchdog::in_window(EXHAUST_WINDOW.get(), now_ns);
                if should_trap || should_exhaust {
                    if context
                        .reconcile_schedule(Some(TimerSchedule::At(now_ns + 60 * CADENCE_NS)))
                        .is_err()
                    {
                        ic_platform_runtime::cdk::api::trap("deadline proposal failed");
                    }
                    if should_trap {
                        ic_platform_runtime::cdk::api::trap("intentional watchdog work trap");
                    }
                    watchdog::exhaust_message_instructions();
                }
                COMPLETED_WORK.with(|completed| completed.set(completed.get().saturating_add(1)));
                let decision = take_work_decision();
                WatchdogRunResult::new(TimerCompletion::success(1), decision)
            },
        )
    })
}

// The latest control request replaces the previous proposal. Consume it only
// after the failure window, so a trapped callback leaves the proposal pending.
fn take_work_decision() -> WatchdogDecision {
    let decision =
        NEXT_WORK_DECISION.with(|requested| requested.replace(WatchdogDecision::Continue));
    if decision == WatchdogDecision::Stop {
        DESIRED_SCHEDULED.with(|desired| desired.set(false));
    }
    decision
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ContinueImmediatelyOnNextWork as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::ContinueImmediatelyOnNextWork as u64 }>"
)]
fn continue_immediately_on_next_work() {
    NEXT_WORK_DECISION.with(|requested| requested.set(WatchdogDecision::ContinueImmediately));
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ScheduleAt as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::ScheduleAt as u64 }>"
)]
fn schedule_at(deadline_ns: u64) {
    REGISTRATION.with_borrow(|slot| {
        let Some(registration) = slot else {
            ic_platform_runtime::cdk::api::trap("missing registration")
        };
        if registration
            .reconcile_schedule(Some(TimerSchedule::At(deadline_ns)))
            .is_err()
        {
            ic_platform_runtime::cdk::api::trap("deadline reconciliation failed");
        }
    });
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ScheduleAtOnNextWork as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::ScheduleAtOnNextWork as u64 }>"
)]
fn schedule_at_on_next_work(deadline_ns: u64) {
    NEXT_WORK_DECISION.with(|requested| requested.set(WatchdogDecision::ScheduleAt(deadline_ns)));
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ReplaceRegistration as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::ReplaceRegistration as u64 }>"
)]
fn replace_registration() {
    REGISTRATION.with_borrow_mut(|slot| {
        if let Some(registration) = slot.take()
            && registration.unregister().is_err()
        {
            ic_platform_runtime::cdk::api::trap("unregister failed");
        }
    });
    if !install_watchdog() {
        ic_platform_runtime::cdk::api::trap("replacement failed");
    }
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::StartSecondary as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::StartSecondary as u64 }>"
)]
fn start_secondary() -> bool {
    SECONDARY_DESIRED_SCHEDULED.with(|desired| desired.set(true));
    install_secondary_watchdog()
}

fn install_secondary_watchdog() -> bool {
    SECONDARY_REGISTRATION.with_borrow_mut(|slot| {
        watchdog::install(slot, secondary_identity(), CADENCE_NS, |_context| {
            SECONDARY_COMPLETED_WORK.with(|completed| {
                completed.set(completed.get().saturating_add(1));
            });
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue)
        })
    })
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ArmTrap as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::ArmTrap as u64 }>"
)]
fn arm_trap() -> u64 {
    let window = failure_window();
    TRAP_WINDOW.with(|target| target.set(Some(window)));
    window.0
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::ArmExhaustion as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::ArmExhaustion as u64 }>"
)]
fn arm_exhaustion() -> u64 {
    let window = failure_window();
    EXHAUST_WINDOW.with(|target| target.set(Some(window)));
    window.0
}

fn failure_window() -> (u64, u64) {
    let start = ic_platform_runtime::cdk::api::time()
        .checked_add(CADENCE_NS)
        .unwrap_or_else(|| ic_platform_runtime::cdk::api::trap("failure deadline overflow"));
    let end = start
        .checked_add(CADENCE_NS)
        .unwrap_or_else(|| ic_platform_runtime::cdk::api::trap("failure window overflow"));
    (start, end)
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::StopOnNextWork as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::StopOnNextWork as u64 }>"
)]
fn stop_on_next_work() {
    NEXT_WORK_DECISION.with(|requested| requested.set(WatchdogDecision::Stop));
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::Stop as u64 }>",
    encode_with = "crate::codec::encode_unit::<{ ic_timers_test_protocol::rpc::Method::Stop as u64 }>"
)]
fn stop() {
    DESIRED_SCHEDULED.with(|desired| desired.set(false));
    REGISTRATION.with_borrow(|slot| {
        if let Some(registration) = slot.as_ref()
            && registration.cancel().is_err()
        {
            ic_platform_runtime::cdk::api::trap("watchdog cancellation failed");
        }
    });
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::EnsureFromReturnedError as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::EnsureFromReturnedError as u64 }>"
)]
fn ensure_from_returned_error() -> bool {
    DESIRED_SCHEDULED.with(|desired| desired.set(true));
    REGISTRATION.with_borrow(|slot| {
        let Some(registration) = slot.as_ref() else {
            ic_platform_runtime::cdk::api::trap("watchdog registration is absent");
        };
        if registration.ensure_scheduled().is_err() {
            ic_platform_runtime::cdk::api::trap("commit-window ensure failed");
        }
    });
    false
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::BurnToBelowTimerCallCost as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::BurnToBelowTimerCallCost as u64 }>"
)]
fn burn_to_below_timer_call_cost() -> (u128, u128) {
    let method_len = "<ic-timers internal> dispatch".len() as u64;
    let call_cost = ic_platform_runtime::cdk::api::cost_call(method_len, 8);
    let liquid = ic_platform_runtime::ic0::canister_liquid_cycle_balance128();
    let target = call_cost.saturating_sub(1);
    let burned = ic_platform_runtime::cdk::api::cycles_burn(liquid.saturating_sub(target));
    (burned, call_cost)
}

#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::FillInventory as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::FillInventory as u64 }>"
)]
fn fill_inventory() -> u64 {
    let current = timer_inventory()
        .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("inventory lookup failed"))
        .len();
    let cadence = TimerCadence::from_nanos(CADENCE_NS)
        .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("extra probe cadence is invalid"));
    for _ in current..MAX_TIMER_REGISTRATIONS {
        let index = NEXT_EXTRA_NAME.with(Cell::get);
        let name = format!("extra-{index:02}");
        let identity =
            TimerIdentity::try_new("ic-timers", "runtime-probe", name).unwrap_or_else(|_| {
                ic_platform_runtime::cdk::api::trap("extra probe identity is invalid")
            });
        // Retained declarations belong to the registry; these inert extras
        // need no caller-held control capabilities after registration.
        let _ = register_watchdog(
            identity,
            cadence,
            TimerStart::Retained { initial: None },
            |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
        )
        .unwrap_or_else(|_| {
            ic_platform_runtime::cdk::api::trap("extra watchdog registration failed")
        });
        NEXT_EXTRA_NAME.with(|next| next.set(index + 1));
    }
    MAX_TIMER_REGISTRATIONS as u64
}

#[ic_platform_runtime::cdk::query(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::InventoryMeasurement as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::InventoryMeasurement as u64 }>"
)]
fn inventory_measurement() -> (u64, u64, bool) {
    let before = ic_platform_runtime::ic0::performance_counter(1);
    let inventory = timer_inventory()
        .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("inventory snapshot failed"));
    let instructions = ic_platform_runtime::ic0::performance_counter(1).saturating_sub(before);
    let ordered = inventory
        .timers()
        .windows(2)
        .all(|pair| pair[0].identity() < pair[1].identity());
    (inventory.len() as u64, instructions, ordered)
}

#[ic_platform_runtime::cdk::query(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::Snapshot as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::Snapshot as u64 }>"
)]
fn snapshot() -> ProbeSnapshot {
    let registered = REGISTRATION.with_borrow(Option::is_some);
    let completed_work = COMPLETED_WORK.with(Cell::get);
    let snapshot = timer_snapshot(&probe_identity())
        .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("snapshot lookup failed"));
    let observed = ProbeSnapshot {
        registered,
        completed_work,
        post_upgrade_reconstructed: POST_UPGRADE_RECONSTRUCTED.with(Cell::get),
        secondary_registered: SECONDARY_REGISTRATION.with_borrow(Option::is_some),
        secondary_completed_work: SECONDARY_COMPLETED_WORK.with(Cell::get),
        ..ProbeSnapshot::default()
    };
    let Some(snapshot) = snapshot else {
        return observed;
    };
    let counters = snapshot.observability().counters();
    let performance = snapshot.observability().performance();
    let scheduler_memory = performance.scheduler_memory_pages();
    let work_memory = performance.work_memory_pages();
    ProbeSnapshot {
        registration_id: Some((
            snapshot.registration_id().epoch().canister_version(),
            snapshot.registration_id().epoch().started_at_ns(),
            snapshot.registration_id().sequence(),
        )),
        next_deadline_ns: snapshot.next_deadline_ns(),
        immediate_scheduling: snapshot.scheduling_mode() == TimerSchedulingMode::Continuation,
        latest_requested_delay_ns: snapshot.latest_requested_delay_ns(),
        latest_armed_delay_ns: snapshot.latest_armed_delay_ns(),
        schedule_requests: counter_observation(counters.schedule_requests()),
        scheduler_started: counter_observation(counters.scheduler_started()),
        wakeups_armed: counter_observation(counters.wakeups_armed()),
        work_dispatched: counter_observation(counters.work_dispatched()),
        work_started: counter_observation(counters.work_started()),
        work_completed: counter_observation(counters.work_completed()),
        coalesced: counter_observation(counters.coalesced()),
        unacknowledged: counter_observation(counters.unacknowledged()),
        last_unacknowledged: snapshot.observability().outcomes().last_outcome()
            == Some(TimerLastOutcome::Unacknowledged),
        scheduler_instruction_samples: performance.scheduler_instructions().aggregate().samples(),
        scheduler_instruction_total: performance.scheduler_instructions().aggregate().total(),
        work_instruction_samples: performance.work_instructions().aggregate().samples(),
        work_instruction_total: performance.work_instructions().aggregate().total(),
        scheduler_memory: probe_memory_summary(scheduler_memory),
        work_memory: probe_memory_summary(work_memory),
        ..observed
    }
}

fn probe_memory_summary(summary: MemoryPageSummary) -> Option<ProbeMemorySummary> {
    let latest = summary.latest()?;
    Some(ProbeMemorySummary {
        samples: summary.samples(),
        latest_wasm_start_pages: latest.start().wasm_pages(),
        latest_wasm_end_pages: latest.end().wasm_pages(),
        latest_stable_start_pages: latest.start().stable_pages(),
        latest_stable_end_pages: latest.end().stable_pages(),
        maximum_wasm_growth_pages: summary.maximum_wasm_growth_pages()?,
        maximum_stable_growth_pages: summary.maximum_stable_growth_pages()?,
    })
}

fn probe_identity() -> TimerIdentity {
    TimerIdentity::try_new("ic-timers", "runtime-probe", "watchdog")
        .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("probe identity is invalid"))
}

fn secondary_identity() -> TimerIdentity {
    TimerIdentity::try_new("ic-timers", "runtime-probe", "secondary")
        .unwrap_or_else(|_| ic_platform_runtime::cdk::api::trap("secondary identity is invalid"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_work_request_replaces_previous_proposals_and_is_consumed_once() {
        DESIRED_SCHEDULED.with(|desired| desired.set(true));
        stop_on_next_work();
        continue_immediately_on_next_work();
        schedule_at_on_next_work(42);
        assert_eq!(take_work_decision(), WatchdogDecision::ScheduleAt(42));
        assert!(DESIRED_SCHEDULED.with(Cell::get));
        assert_eq!(take_work_decision(), WatchdogDecision::Continue);

        schedule_at_on_next_work(99);
        continue_immediately_on_next_work();
        assert_eq!(take_work_decision(), WatchdogDecision::ContinueImmediately);
        assert_eq!(take_work_decision(), WatchdogDecision::Continue);

        continue_immediately_on_next_work();
        stop_on_next_work();
        assert_eq!(take_work_decision(), WatchdogDecision::Stop);
        assert!(!DESIRED_SCHEDULED.with(Cell::get));
        assert_eq!(take_work_decision(), WatchdogDecision::Continue);
    }
}

// Unpublished fixture: exercise the authenticated backend payload boundary.
#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::FixtureDispatchPayload as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::FixtureDispatchPayload as u64 }>"
)]
async fn fixture_dispatch_payload(payload: Vec<u8>) -> bool {
    ic_platform_runtime::cdk::call::Call::unbounded_wait(
        ic_platform_runtime::cdk::api::canister_self(),
        "<ic-timers internal> dispatch",
    )
    .take_raw_args(payload)
    .await
    .is_ok()
}

// A real foreign-canister caller bypasses the target's ingress inspection.
#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::FixtureForeignDispatch as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::FixtureForeignDispatch as u64 }>"
)]
async fn fixture_foreign_dispatch(target: Vec<u8>, payload: Vec<u8>) -> String {
    let target = ic_platform_runtime::Principal::from_slice(&target);
    match ic_platform_runtime::cdk::call::Call::unbounded_wait(
        target,
        "<ic-timers internal> dispatch",
    )
    .take_raw_args(payload)
    .await
    {
        Ok(_) => String::new(),
        Err(error) => error.to_string(),
    }
}

// Deliberately violate ownership, without linking a second timer provider.
#[ic_platform_runtime::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode::<_, { ic_timers_test_protocol::rpc::Method::FixtureGlobalTimerSet as u64 }>",
    encode_with = "crate::codec::encode::<_, { ic_timers_test_protocol::rpc::Method::FixtureGlobalTimerSet as u64 }>"
)]
fn fixture_global_timer_set(timestamp: u64) -> u64 {
    ic_platform_runtime::ic0::global_timer_set(timestamp)
}
fn counter_observation(counter: ic_timers::MeasurementCounter) -> CounterObservation {
    CounterObservation {
        total: counter.total(),
        total_overflowed: counter.total_overflowed(),
    }
}
