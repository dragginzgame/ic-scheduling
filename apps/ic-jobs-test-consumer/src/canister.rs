use std::{
    cell::{Cell, RefCell},
    time::Duration,
};

use ic_jobs::Outcome;
use ic_platform_runtime::cdk;
use ic_storage::DefaultMemoryImpl;
use ic_timers::{
    TimerCadence, TimerIdentity, WatchdogReconcileState, WatchdogRegistration, WatchdogRunResult,
    reconcile_watchdog,
};
use ic_wire::native;
use serde::de::DeserializeOwned;

use crate::{
    api::{Create, Delivery, Error, Init, Summary, View},
    delivery_wire::{
        Method as DeliveryMethod, RECEIPT_ID, RECEIVE_ID, REQUEST_LIMITS, RESPONSE_LIMITS,
        decode_reply,
    },
    management_wire::{self as control, Method},
    queue::{Committed, DispatchBatch, Queue, QueueTotals},
};

thread_local! {
    static QUEUE: RefCell<Option<Queue<DefaultMemoryImpl>>> = const { RefCell::new(None) };
    static WATCHDOG: RefCell<Option<WatchdogRegistration>> = const { RefCell::new(None) };
    // Fault control belongs only to this unpublished qualification canister.
    static TRAP_WATCHDOG_BATCH: Cell<bool> = const { Cell::new(false) };
}

fn required<T>(result: Result<T, impl std::fmt::Debug>) -> T {
    result.unwrap_or_else(|error| cdk::api::trap(format!("consumer boundary failed: {error:?}")))
}

fn with_queue<T>(
    operation: impl FnOnce(&mut Queue<DefaultMemoryImpl>) -> Result<T, Error>,
) -> Result<T, Error> {
    QUEUE.with(|queue| operation(queue.borrow_mut().as_mut().ok_or(Error::InvalidStorage)?))
}

fn with_authorized_queue<T>(
    operation: impl FnOnce(&mut Queue<DefaultMemoryImpl>) -> Result<T, Error>,
) -> Result<T, Error> {
    with_queue(|queue| {
        queue.authorize(cdk::api::msg_caller())?;
        operation(queue)
    })
}

fn update_queue<T>(
    operation: impl FnOnce(&mut Queue<DefaultMemoryImpl>) -> Result<Committed<T>, Error>,
) -> Result<T, Error> {
    Ok(reconcile_committed(with_authorized_queue(operation)?))
}

fn reconcile_committed<T>(committed: Committed<T>) -> T {
    // Reconciliation failure traps the same message segment as the stable write.
    reconcile_at(committed.next_due_ns);
    committed.value
}

fn reconcile_at(due: Option<u64>) {
    WATCHDOG.with(|watchdog| {
        required(reconcile_watchdog(
            &mut watchdog.borrow_mut(),
            &required(TimerIdentity::try_new(
                "ic-jobs-test-consumer",
                "queue",
                "dispatch",
            )),
            required(TimerCadence::new(Duration::from_secs(1))),
            due.map_or(
                WatchdogReconcileState::Inactive,
                WatchdogReconcileState::ScheduledAt,
            ),
            |_| wake(),
        ))
    });
}

fn admit_control_input() -> Result<(), String> {
    if ic_platform_runtime::ic0::msg_arg_data_size() > control::REQUEST_LIMITS.bytes {
        return Err("management request exceeds Wire frame limit".into());
    }
    Ok(())
}

fn decode_control<T: DeserializeOwned, const ID: u64>(bytes: Vec<u8>) -> T {
    ic_wire_endpoint::decode_request(&bytes, ID, control::REQUEST_LIMITS)
}

fn recover(initial: Option<Init>) {
    let (queue, due) = required(Queue::open(DefaultMemoryImpl::default(), initial));
    // Open/restore the entire bounded snapshot before registering any callback.
    QUEUE.with(|slot| *slot.borrow_mut() = Some(queue));
    required(ic_timers::initialize_runtime());
    reconcile_at(due);
}

#[cdk::init]
fn init() {
    // Lifecycle macros have no guards. Admit length before copying init bytes,
    // then use the same bounded codec as the argument-bearing RPC hooks.
    required(admit_control_input());
    let config = decode_control::<Init, { Method::Init as u64 }>(cdk::api::msg_arg_data());
    recover(Some(config));
}

#[cdk::post_upgrade]
fn post_upgrade() {
    recover(None);
}

#[ic_wire_endpoint::update(
    id = Method::Create as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn create((input,): (Create,)) -> Result<u64, Error> {
    update_queue(|queue| queue.create(input))
}

#[ic_wire_endpoint::query(
    id = Method::Inspect as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn inspect((id,): (u64,)) -> Result<View, Error> {
    with_authorized_queue(|queue| queue.inspect(id))
}

#[ic_wire_endpoint::query(
    id = Method::List as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn list((after, limit): (Option<u64>, u8)) -> Result<Vec<View>, Error> {
    with_authorized_queue(|queue| queue.page(after, limit))
}

#[ic_wire_endpoint::update(
    id = Method::Cancel as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn cancel((id,): (u64,)) -> Result<(), Error> {
    update_queue(|queue| queue.cancel(id, cdk::api::time()))
}

#[ic_wire_endpoint::update(
    id = Method::Pause as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn pause((id,): (u64,)) -> Result<(), Error> {
    update_queue(|queue| queue.pause(id, cdk::api::time()))
}

#[ic_wire_endpoint::update(
    id = Method::Resume as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn resume((id,): (u64,)) -> Result<(), Error> {
    update_queue(|queue| queue.resume(id, cdk::api::time()))
}

#[ic_wire_endpoint::update(
    id = Method::Reschedule as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn reschedule((id, timing): (u64, crate::api::Timing)) -> Result<(), Error> {
    update_queue(|queue| queue.reschedule(id, timing, cdk::api::time()))
}

#[ic_wire_endpoint::query(
    id = Method::Summary as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn summary() -> Result<Summary, Error> {
    let Committed {
        value: QueueTotals { counter, receipts },
        next_due_ns,
    } = with_authorized_queue(|queue| queue.summary())?;
    let armed = WATCHDOG.with(|watchdog| {
        watchdog
            .borrow()
            .as_ref()
            .is_some_and(|registration| required(registration.has_armed_wakeup()))
    });
    Ok(Summary {
        next_due_ns,
        counter,
        receipts,
        armed,
    })
}

#[ic_wire_endpoint::query(
    id = Method::JobLogs as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn job_logs((request,): (crate::api::JobLogRequest,)) -> Result<crate::api::JobLogPage, Error> {
    with_authorized_queue(|queue| queue.job_logs(request, cdk::api::canister_version()))
}

fn complete_delivery(delivery: &Delivery, outcome: Outcome) -> Result<(), Error> {
    let committed = with_queue(|queue| queue.finish(delivery, cdk::api::time(), outcome))?;
    reconcile_committed(committed);
    Ok(())
}

#[ic_wire_endpoint::query(
    id = Method::JobMetrics as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn job_metrics() -> Result<crate::api::JobMetrics, Error> {
    with_authorized_queue(|queue| Ok(queue.job_metrics(cdk::api::canister_version())))
}

#[ic_wire_endpoint::update(
    id = Method::ResetJobMetrics as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn reset_job_metrics() -> Result<crate::api::JobMetrics, Error> {
    with_authorized_queue(|queue| Ok(queue.reset_job_metrics(cdk::api::canister_version())))
}

async fn delivery_call(
    delivery: &Delivery,
    method: DeliveryMethod,
) -> Result<cdk::call::Response, cdk::call::CallFailed> {
    let bytes = required(native::encode_request(
        method.id(),
        delivery,
        REQUEST_LIMITS,
    ));
    cdk::call::Call::unbounded_wait(delivery.payload.destination, method.name())
        .with_raw_args(&bytes)
        .await
}

async fn delivery_confirmed(delivery: &Delivery) -> bool {
    let method = DeliveryMethod::Receive;
    let response = delivery_call(delivery, method).await;
    decode_reply::<Delivery, _>(method.id(), response).is_some_and(|receipt| receipt == *delivery)
}

async fn deliver(delivery: Delivery) {
    let confirmed = delivery_confirmed(&delivery).await;
    // Transport rejection or codec failure cannot establish that no effect ran.
    required(complete_delivery(
        &delivery,
        if confirmed {
            Outcome::Success
        } else {
            Outcome::Uncertain
        },
    ));
}

fn launch_deliveries(deliveries: Vec<Delivery>) {
    for delivery in deliveries {
        cdk::futures::spawn(deliver(delivery));
    }
}

fn wake() -> WatchdogRunResult {
    let result = required(with_queue(|queue| queue.dispatch(cdk::api::time())));
    let DispatchBatch { deliveries, work } = result.value;
    if work > 0 && TRAP_WATCHDOG_BATCH.with(Cell::get) {
        // A stable write in this segment is not independently committed. No
        // external delivery escapes before this deliberate rollback boundary.
        cdk::api::trap("fixture: watchdog trap after stable batch write");
    }
    launch_deliveries(deliveries);
    // Work counts handled attempts, not confirmed external effects.
    WatchdogRunResult::completed_batch(work, result.next_due_ns)
}

#[ic_wire_endpoint::update(
    id = Method::DispatchDue as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn dispatch_due() -> Result<(), Error> {
    launch_deliveries(dispatch_reconciled()?);
    Ok(())
}

fn dispatch_reconciled() -> Result<Vec<Delivery>, Error> {
    Ok(update_queue(|queue| queue.dispatch(cdk::api::time()))?.deliveries)
}

#[ic_wire_endpoint::update(
    id = RECEIVE_ID,
    request = REQUEST_LIMITS,
    response = RESPONSE_LIMITS
)]
fn receive_delivery((delivery,): (Delivery,)) -> Result<Delivery, Error> {
    if delivery.payload.destination != cdk::api::canister_self() {
        return Err(Error::ReceiptMismatch);
    }
    with_queue(|queue| queue.receive(cdk::api::msg_caller(), delivery))
}

#[ic_wire_endpoint::update(
    id = RECEIPT_ID,
    request = REQUEST_LIMITS,
    response = RESPONSE_LIMITS
)]
fn delivery_receipt((delivery,): (Delivery,)) -> Result<Option<Delivery>, Error> {
    with_queue(|queue| queue.receipt(cdk::api::msg_caller(), &delivery))
}

#[ic_wire_endpoint::update(
    id = Method::ReconcileDelivery as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
async fn reconcile_delivery((id,): (u64,)) -> Result<bool, Error> {
    let delivery = with_authorized_queue(|queue| queue.outstanding(id))?;
    let method = DeliveryMethod::Receipt;
    let response = delivery_call(&delivery, method).await;
    let receipt = decode_reply::<Option<Delivery>, _>(method.id(), response).flatten();
    let Some(receipt) = receipt else {
        return Ok(false);
    };
    if receipt != delivery {
        return Err(Error::ReceiptMismatch);
    }
    complete_delivery(&delivery, Outcome::Success)?;
    Ok(true)
}

// These explicitly named fault controls exist only in this unpublished test
// canister. They use real IC traps; there is no production cfg(test) substitute.
#[ic_wire_endpoint::update(
    id = Method::TrapWatchdogBatch as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn fixture_trap_watchdog_batch((enabled,): (bool,)) -> Result<(), Error> {
    with_authorized_queue(|_| Ok(()))?;
    TRAP_WATCHDOG_BATCH.with(|fault| fault.set(enabled));
    Ok(())
}

#[ic_wire_endpoint::update(
    id = Method::TrapAfterCreate as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn fixture_trap_after_create((input,): (Create,)) {
    required(create((input,)));
    cdk::api::trap("fixture: trap after stable write and watchdog reconciliation");
}

#[ic_wire_endpoint::update(
    id = Method::TrapAfterPause as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn fixture_trap_after_pause((id,): (u64,)) {
    required(pause((id,)));
    cdk::api::trap("fixture: trap after paused write and watchdog reconciliation");
}

#[ic_wire_endpoint::update(
    id = Method::TrapAfterReschedule as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn fixture_trap_after_reschedule((id, timing): (u64, crate::api::Timing)) {
    required(reschedule((id, timing)));
    cdk::api::trap("fixture: trap after rescheduled write and watchdog reconciliation");
}

#[ic_wire_endpoint::update(
    id = Method::CorruptRecord as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn fixture_corrupt_record((id,): (u64,)) -> Result<(), Error> {
    with_authorized_queue(|queue| queue.corrupt_record(id))
}

#[ic_wire_endpoint::update(
    id = Method::MarkUncertain as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn fixture_mark_uncertain((id,): (u64,)) -> Result<(), Error> {
    let delivery = with_authorized_queue(|queue| queue.outstanding(id))?;
    complete_delivery(&delivery, Outcome::Uncertain)?;
    Ok(())
}

#[ic_wire_endpoint::update(
    id = Method::DispatchTrapResult as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
async fn fixture_dispatch_trap_result((input,): (Create,)) -> Result<(), Error> {
    create((input,))?;
    let deliveries = dispatch_reconciled()?;
    // Intent commits at the real inter-canister call boundary. The receipt lives
    // at the destination; a later sender trap cannot roll back that earlier call.
    if let Some(delivery) = deliveries.into_iter().next() {
        if !delivery_confirmed(&delivery).await {
            cdk::api::trap("fixture delivery was not confirmed");
        }
        cdk::api::trap("fixture: lost sender result after confirmed external effect");
    }
    Ok(())
}

#[ic_wire_endpoint::update(
    id = Method::CreateBlocked as u64,
    request = control::REQUEST_LIMITS,
    response = control::RESPONSE_LIMITS
)]
fn fixture_create_blocked((input,): (Create,)) -> Result<u64, Error> {
    let id = create((input,))?;
    // Simulate interruption after committed intent but before dispatch. This
    // endpoint never sends an envelope or claims an absent receipt is retry safe.
    dispatch_reconciled()?;
    Ok(id)
}
