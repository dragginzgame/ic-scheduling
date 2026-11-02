//! Explicit live PocketIC tests. Ordinary CI compiles these without launching a server.

use ic_jobs_canister_test_support::wire;
use ic_jobs_test_consumer::{
    api::{Create, Delivery, DeliveryPayload, Error, Init, Status, Summary, Task, Timing, View},
    delivery_wire::{RECEIPT_ID, RECEIVE_ID, REQUEST_LIMITS, RESPONSE_LIMITS},
    management_wire::{self as control, Method},
};
use ic_platform_pocket_ic::{ErrorCode, PocketIc, PocketIcBuilder, RejectResponse};
use ic_platform_types::principal::ExternalPrincipal;
use ic_testkit::pic::{
    CanisterDiagnosticsRequest, CanisterInstallError, CanisterInstallExt, CanisterLogRenderLimits,
    InstallSpec, PocketIcCallFailure, PocketIcDiagnosticsExt, PocketIcInstance,
    PocketIcStartupConfig, RetryPolicy, WireCallError, advance_and_tick, query_wire, tick_until,
};
use ic_wire::native;
use serde::{Serialize, de::DeserializeOwned};
use std::{convert::Infallible, path::PathBuf, task::Poll, time::Duration};

struct Fixture {
    pic: PocketIcInstance<PocketIc>,
    wasm: Vec<u8>,
    manager: ExternalPrincipal,
    sender: ExternalPrincipal,
}

impl Fixture {
    fn install(
        pic: &PocketIc,
        wasm: &[u8],
        manager: ExternalPrincipal,
        delivery_sender: Option<ExternalPrincipal>,
    ) -> ExternalPrincipal {
        pic.try_create_and_install(InstallSpec {
            wasm: wasm.to_vec(),
            init_bytes: wire::init_bytes(&Init {
                manager: manager.into(),
                delivery_sender: delivery_sender.map(Into::into),
            })
            .unwrap(),
            cycles: 10_000_000_000_000,
            install_sender: None,
            label: Some("ic-jobs-test-consumer".into()),
        })
        .unwrap_or_else(|error| {
            if let CanisterInstallError::InstallCode { canister_id, .. } = &error {
                // Diagnostics are subordinate to the original install failure,
                // including a failed stderr write or collection panic.
                let _ = pic.write_canister_diagnostics(
                    CanisterDiagnosticsRequest {
                        canister_id: *canister_id,
                        status_sender: ExternalPrincipal::anonymous(),
                        log_sender: ExternalPrincipal::anonymous(),
                        log_limits: CanisterLogRenderLimits::default(),
                    },
                    "install Jobs consumer",
                    &mut std::io::stderr().lock(),
                );
            }
            panic!("install Jobs consumer: {error:?}");
        })
    }

    fn install_receiver(&self) -> ExternalPrincipal {
        Self::install(
            self.pic.instance(),
            &self.wasm,
            self.manager,
            Some(self.sender),
        )
    }

    fn new() -> Self {
        let path = PathBuf::from(
            std::env::var_os("IC_JOBS_CONSUMER_WASM")
                .expect("build-consumer must select IC_JOBS_CONSUMER_WASM"),
        );
        let wasm = std::fs::read(path).expect("read selected consumer Wasm");
        let config = PocketIcStartupConfig::from_env(Duration::from_secs(30))
            .expect("use ic-testkit-server run after explicit setup/check");
        let pic = config
            .build_instance(PocketIcBuilder::new().with_application_subnet())
            .expect("bounded PocketIC startup");
        let manager = ExternalPrincipal::self_authenticating(b"jobs-manager");
        let sender = Self::install(pic.instance(), &wasm, manager, None);
        Self {
            pic,
            wasm,
            manager,
            sender,
        }
    }

    fn update<T: DeserializeOwned, A: Serialize>(
        &self,
        method: Method,
        args: A,
    ) -> Result<T, Error> {
        self.update_as(self.manager, method, args)
            .expect("PocketIC Wire call")
    }
    fn update_as<T: DeserializeOwned, A: Serialize>(
        &self,
        caller: ExternalPrincipal,
        method: Method,
        args: A,
    ) -> Result<T, RejectResponse> {
        match self.client(self.sender, caller).update(method, &args) {
            Ok(value) => Ok(value),
            Err(WireCallError::Call { source, .. }) => match *source {
                PocketIcCallFailure::Rejected(response) => Err(response),
                other => panic!("PocketIC Wire call failed: {other:?}"),
            },
            Err(other) => panic!("PocketIC Wire contract failed: {other:?}"),
        }
    }
    fn query<T: DeserializeOwned, A: Serialize>(
        &self,
        id: ExternalPrincipal,
        method: Method,
        args: A,
    ) -> Result<T, Error> {
        self.client(id, self.manager)
            .query(method, &args)
            .expect("PocketIC Wire call")
    }
    fn client(&self, canister: ExternalPrincipal, caller: ExternalPrincipal) -> wire::Client<'_> {
        wire::Client {
            pic: self.pic.instance(),
            canister,
            caller,
        }
    }
    fn stopped<T>(&self, canister: ExternalPrincipal, operation: impl FnOnce(&PocketIc) -> T) -> T {
        let pic = self.pic.instance();
        pic.stop_canister(canister, None).unwrap();
        let result = operation(pic);
        pic.start_canister(canister, None).unwrap();
        result
    }
    fn snapshot(&self, canister: ExternalPrincipal) -> Vec<u8> {
        self.stopped(canister, |pic| {
            pic.take_canister_snapshot(canister, None, None).unwrap().id
        })
    }
    fn restore_snapshot(&self, canister: ExternalPrincipal, snapshot: Vec<u8>) {
        self.stopped(canister, |pic| {
            pic.load_canister_snapshot(canister, None, snapshot)
                .unwrap()
        });
    }
    fn view(&self, id: u64) -> View {
        self.query(self.sender, Method::Inspect, (id,)).unwrap()
    }
    fn summary(&self, id: ExternalPrincipal) -> Summary {
        self.query(id, Method::Summary, ()).unwrap()
    }
    fn upgrade(&self, id: ExternalPrincipal) -> Result<(), RejectResponse> {
        self.pic
            .instance()
            .upgrade_canister(id, self.wasm.clone(), Vec::new(), None)
    }
    fn until(&self, predicate: impl Fn() -> bool) {
        tick_until(self.pic.instance(), 30, Duration::ZERO, |_| {
            if predicate() {
                Poll::Ready(Ok::<_, Infallible>(()))
            } else {
                Poll::Pending
            }
        })
        .expect("bounded canister progress did not complete");
    }
}

fn assert_canister_trap(error: RejectResponse) {
    assert_eq!(
        error.error_code,
        ErrorCode::CanisterCalledTrap,
        "expected canister trap: {error:?}"
    );
}

fn increment(at: u64, amount: u64) -> Create {
    Create {
        timing: Timing::Once(at),
        task: Task::Increment(amount),
    }
}

#[test]
fn reply_limits_admit_the_largest_consumer_page() {
    let views = (0..8)
        .map(|id| View {
            id,
            task: Task::Deliver(DeliveryPayload {
                destination: ExternalPrincipal::self_authenticating(b"receiver").into(),
                message: "x".repeat(256),
            }),
            status: Status::Pending { due_ns: u64::MAX },
            sequence: u64::MAX,
            occurrence: u64::MAX,
        })
        .collect::<Vec<_>>();
    let id = Method::List as u64;
    let bytes =
        native::encode_response(id, &Ok::<_, Error>(&views), control::RESPONSE_LIMITS).unwrap();
    let decoded: Result<Vec<View>, Error> =
        native::decode_response(id, &bytes, control::RESPONSE_LIMITS).unwrap();
    assert_eq!(decoded.unwrap(), views);
    let old = include_bytes!("fixtures/pending-page-0162.didbin");
    assert!(
        native::decode_response::<Result<Vec<View>, Error>>(id, old, control::RESPONSE_LIMITS)
            .is_err()
    );
    let oversized = vec![0; control::RESPONSE_LIMITS.bytes + 1];
    assert_eq!(
        native::decode_response::<()>(id, &oversized, control::RESPONSE_LIMITS),
        Err(ic_wire::Error::Limit)
    );
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn management_wire_refuses_retired_and_invalid_unit_requests() {
    let fixture = Fixture::new();
    let before = fixture.summary(fixture.sender);
    for init_bytes in [
        include_bytes!(
            "../../../apps/ic-jobs-test-consumer/tests/fixtures/api-pre-platform.didbin"
        )
        .to_vec(),
        vec![0; control::REQUEST_LIMITS.bytes + 1],
    ] {
        let error = fixture
            .pic
            .instance()
            .try_create_and_install(InstallSpec {
                wasm: fixture.wasm.clone(),
                init_bytes,
                cycles: 10_000_000_000_000,
                install_sender: None,
                label: Some("invalid Jobs init".into()),
            })
            .unwrap_err();
        assert!(matches!(error, CanisterInstallError::InstallCode { .. }));
    }
    let id = Method::DispatchDue as u64;
    let valid = native::encode_request(id, &(), control::REQUEST_LIMITS).unwrap();
    let mut trailing = valid;
    trailing.push(0);
    for bytes in [
        include_bytes!(
            "../../../apps/ic-jobs-test-consumer/tests/fixtures/api-pre-platform.didbin"
        )
        .to_vec(),
        Vec::new(),
        native::encode_request(Method::Summary as u64, &(), control::REQUEST_LIMITS).unwrap(),
        native::encode_response(id, &(), control::RESPONSE_LIMITS).unwrap(),
        trailing,
    ] {
        assert_canister_trap(
            fixture
                .pic
                .instance()
                .update_call(
                    fixture.sender,
                    fixture.manager,
                    Method::DispatchDue.name(),
                    bytes,
                )
                .unwrap_err(),
        );
        assert_eq!(fixture.summary(fixture.sender), before);
    }
    let error = fixture
        .pic
        .instance()
        .update_call(
            fixture.sender,
            fixture.manager,
            Method::DispatchDue.name(),
            vec![0; control::REQUEST_LIMITS.bytes + 1],
        )
        .unwrap_err();
    assert_canister_trap(error);
    assert_eq!(fixture.summary(fixture.sender), before);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn delivery_wire_endpoints_refuse_candid_and_preserve_authorized_receipts() {
    let fixture = Fixture::new();
    let receiver = fixture.install_receiver();
    let delivery = Delivery {
        job: 99,
        occurrence: 0,
        sequence: 1,
        payload: DeliveryPayload {
            destination: receiver.into(),
            message: "x".repeat(256),
        },
    };
    for (method, id, other) in [
        ("receive_delivery", RECEIVE_ID, RECEIPT_ID),
        ("delivery_receipt", RECEIPT_ID, RECEIVE_ID),
    ] {
        for bytes in [
            include_bytes!(
                "../../../apps/ic-jobs-test-consumer/tests/fixtures/api-pre-platform.didbin"
            )
            .to_vec(),
            native::encode_request(other, &delivery, REQUEST_LIMITS).unwrap(),
            native::encode_response(id, &delivery, REQUEST_LIMITS).unwrap(),
        ] {
            let error = fixture
                .pic
                .instance()
                .update_call(receiver, fixture.sender, method, bytes)
                .unwrap_err();
            assert_eq!(error.error_code, ErrorCode::CanisterCalledTrap);
            assert_eq!(fixture.summary(receiver).receipts, 0);
        }
        let error = fixture
            .pic
            .instance()
            .update_call(
                receiver,
                fixture.sender,
                method,
                vec![0; REQUEST_LIMITS.bytes + 1],
            )
            .unwrap_err();
        assert_canister_trap(error);
        assert_eq!(fixture.summary(receiver).receipts, 0);
    }
    let request = native::encode_request(RECEIVE_ID, &delivery, REQUEST_LIMITS).unwrap();
    let bytes = fixture
        .pic
        .instance()
        .update_call(
            receiver,
            ExternalPrincipal::anonymous(),
            "receive_delivery",
            request.clone(),
        )
        .unwrap();
    assert_eq!(
        native::decode_response::<Result<Delivery, Error>>(RECEIVE_ID, &bytes, RESPONSE_LIMITS)
            .unwrap(),
        Err(Error::Unauthorized)
    );
    assert_eq!(fixture.summary(receiver).receipts, 0);
    for _ in 0..2 {
        let bytes = fixture
            .pic
            .instance()
            .update_call(
                receiver,
                fixture.sender,
                "receive_delivery",
                request.clone(),
            )
            .unwrap();
        assert_eq!(
            native::decode_response::<Result<Delivery, Error>>(RECEIVE_ID, &bytes, RESPONSE_LIMITS)
                .unwrap(),
            Ok(delivery.clone())
        );
        assert_eq!(fixture.summary(receiver).receipts, 1);
    }
    fixture.upgrade(receiver).unwrap();
    let request = native::encode_request(RECEIPT_ID, &delivery, REQUEST_LIMITS).unwrap();
    let bytes = fixture
        .pic
        .instance()
        .update_call(receiver, fixture.sender, "delivery_receipt", request)
        .unwrap();
    assert_eq!(
        native::decode_response::<Result<Option<Delivery>, Error>>(
            RECEIPT_ID,
            &bytes,
            RESPONSE_LIMITS
        )
        .unwrap(),
        Ok(Some(delivery))
    );
    assert_eq!(fixture.summary(receiver).receipts, 1);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn management_upgrade_and_real_watchdog_delivery() {
    let fixture = Fixture::new();
    let receiver = fixture.install_receiver();
    let now = fixture
        .pic
        .instance()
        .get_time()
        .as_nanos_since_unix_epoch();
    let unauthorized: Result<u64, Error> = fixture
        .update_as(
            ExternalPrincipal::anonymous(),
            Method::Create,
            (increment(now + 20_000_000_000, 5),),
        )
        .expect("PocketIC Wire call");
    assert_eq!(unauthorized, Err(Error::Unauthorized));
    let first: u64 = fixture
        .update(Method::Create, (increment(now + 20_000_000_000, 5),))
        .unwrap();
    let second: u64 = fixture
        .update(Method::Create, (increment(now + 30_000_000_000, 7),))
        .unwrap();
    let before = fixture.view(second);
    assert!(fixture.summary(fixture.sender).armed);
    fixture.upgrade(fixture.sender).unwrap();
    assert_eq!(fixture.view(second), before);
    assert_eq!(
        fixture.summary(fixture.sender).next_due_ns,
        Some(now + 20_000_000_000)
    );
    fixture.update::<(), _>(Method::Cancel, (first,)).unwrap();
    assert_eq!(
        fixture.summary(fixture.sender).next_due_ns,
        Some(now + 30_000_000_000)
    );
    let page: Vec<View> = fixture
        .query(fixture.sender, Method::List, (Some(first), 1_u8))
        .unwrap();
    assert_eq!(page, vec![before]);
    fixture.pic.instance().advance_time(Duration::from_secs(40));
    fixture.until(|| fixture.view(second).status == Status::Completed);
    assert_eq!(fixture.view(first).status, Status::Cancelled);
    assert_eq!(fixture.summary(fixture.sender).counter, 7);
    assert!(!fixture.summary(fixture.sender).armed);
    let third: u64 = fixture
        .update(
            Method::Create,
            (Create {
                timing: Timing::Once(
                    fixture
                        .pic
                        .instance()
                        .get_time()
                        .as_nanos_since_unix_epoch()
                        + 10_000_000_000,
                ),
                task: Task::Deliver(DeliveryPayload {
                    destination: receiver.into(),
                    message: "timer delivery".into(),
                }),
            },),
        )
        .unwrap();
    fixture.upgrade(fixture.sender).unwrap();
    fixture.pic.instance().advance_time(Duration::from_secs(15));
    fixture.until(|| fixture.view(third).status == Status::Completed);
    assert_eq!(fixture.summary(receiver).receipts, 1);
    fixture.upgrade(fixture.sender).unwrap();
    fixture.upgrade(receiver).unwrap();
    advance_and_tick(fixture.pic.instance(), Duration::ZERO, 3);
    assert_eq!(fixture.summary(receiver).receipts, 1);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn synchronous_trap_rolls_back_storage_identity_and_timer_changes() {
    let fixture = Fixture::new();
    let id: u64 = fixture
        .update(
            Method::Create,
            (increment(
                fixture
                    .pic
                    .instance()
                    .get_time()
                    .as_nanos_since_unix_epoch()
                    + 60_000_000_000,
                3,
            ),),
        )
        .unwrap();
    let before = fixture.summary(fixture.sender);
    let failed = fixture.update_as::<(), _>(
        fixture.manager,
        Method::TrapAfterCreate,
        (increment(
            fixture
                .pic
                .instance()
                .get_time()
                .as_nanos_since_unix_epoch()
                + 30_000_000_000,
            9,
        ),),
    );
    assert_canister_trap(failed.unwrap_err());
    assert_eq!(fixture.summary(fixture.sender), before);
    let page: Vec<View> = fixture
        .query(fixture.sender, Method::List, (None::<u64>, 8_u8))
        .unwrap();
    assert_eq!(page.len(), 1);
    fixture.upgrade(fixture.sender).unwrap();
    assert_eq!(fixture.summary(fixture.sender), before);
    let next: u64 = fixture
        .update(
            Method::Create,
            (increment(
                fixture
                    .pic
                    .instance()
                    .get_time()
                    .as_nanos_since_unix_epoch()
                    + 120_000_000_000,
                4,
            ),),
        )
        .unwrap();
    assert_eq!(next, id + 1);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn watchdog_batch_trap_rolls_back_then_recovers_without_repeating_local_effects() {
    let fixture = Fixture::new();
    let due = fixture
        .pic
        .instance()
        .get_time()
        .as_nanos_since_unix_epoch()
        + 10_000_000_000;
    let healthy: u64 = fixture
        .update(Method::Create, (increment(due, 1),))
        .unwrap();
    let failed: u64 = fixture
        .update(Method::Create, (increment(due, u64::MAX),))
        .unwrap();
    let later_due = due + 30_000_000_000;
    let later: u64 = fixture
        .update(Method::Create, (increment(later_due, 4),))
        .unwrap();
    let healthy_before = fixture.view(healthy);
    let failed_before = fixture.view(failed);
    fixture
        .update::<(), _>(Method::TrapWatchdogBatch, (true,))
        .unwrap();
    fixture.pic.instance().advance_time(Duration::from_secs(10));
    // Observe a real trap, not merely a dispatched callback awaiting execution.
    let marker = b"fixture: watchdog trap after stable batch write";
    fixture.until(|| {
        fixture
            .pic
            .instance()
            .fetch_canister_logs(fixture.sender, ExternalPrincipal::anonymous())
            .unwrap()
            .iter()
            .any(|record| {
                record
                    .content
                    .windows(marker.len())
                    .any(|part| part == marker)
            })
    });
    assert_eq!(fixture.view(healthy), healthy_before);
    assert_eq!(fixture.view(failed), failed_before);
    let rolled_back = fixture.summary(fixture.sender);
    assert_eq!(rolled_back.counter, 0);
    let logs: ic_jobs_test_consumer::api::JobLogPage = fixture
        .query(
            fixture.sender,
            Method::JobLogs,
            ic_jobs_test_consumer::api::JobLogRequest {
                before: None,
                min_level: None,
                limit: 8,
            },
        )
        .unwrap();
    assert!(logs.entries.is_empty());
    let metrics: ic_jobs_test_consumer::api::JobMetrics = fixture
        .query(fixture.sender, Method::JobMetrics, ())
        .unwrap();
    assert_eq!(metrics.dispatched.total, 0);
    assert_eq!(metrics.succeeded.total, 0);
    assert_eq!(metrics.permanent_failures.total, 0);
    assert_eq!(rolled_back.next_due_ns, Some(due));
    assert!(rolled_back.armed);

    // This control does not rearm the timer: recovery must use its survivor.
    fixture
        .update::<(), _>(Method::TrapWatchdogBatch, (false,))
        .unwrap();
    fixture.pic.instance().advance_time(Duration::from_secs(2));
    fixture.until(|| fixture.view(healthy).status == Status::Completed);
    assert_eq!(fixture.view(failed).status, Status::Failed);
    assert_eq!(fixture.view(healthy).sequence, 1);
    assert_eq!(fixture.view(failed).sequence, 1);
    let recovered = fixture.summary(fixture.sender);
    assert_eq!(recovered.counter, 1);
    assert_eq!(recovered.next_due_ns, Some(later_due));
    assert!(recovered.armed);
    assert_eq!(
        fixture.view(later).status,
        Status::Pending { due_ns: later_due }
    );

    fixture.pic.instance().advance_time(Duration::from_secs(40));
    fixture.until(|| fixture.view(later).status == Status::Completed);
    assert_eq!(fixture.summary(fixture.sender).counter, 5);
    assert!(!fixture.summary(fixture.sender).armed);
    fixture.upgrade(fixture.sender).unwrap();
    advance_and_tick(fixture.pic.instance(), Duration::from_secs(3), 12);
    assert_eq!(fixture.summary(fixture.sender).counter, 5);
    assert_eq!(fixture.view(failed).status, Status::Failed);
    assert!(!fixture.summary(fixture.sender).armed);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn lost_result_keeps_intent_blocked_and_receipt_resolution_does_not_redeliver() {
    let fixture = Fixture::new();
    let receiver = fixture.install_receiver();
    let input = Create {
        timing: Timing::Once(0),
        task: Task::Deliver(DeliveryPayload {
            destination: receiver.into(),
            message: "retained external effect".into(),
        }),
    };
    let failed = fixture.update_as::<Result<(), Error>, _>(
        fixture.manager,
        Method::DispatchTrapResult,
        (input,),
    );
    assert_canister_trap(failed.unwrap_err());
    let page: Vec<View> = fixture
        .query(fixture.sender, Method::List, (None::<u64>, 8_u8))
        .unwrap();
    let intent = page.into_iter().next().unwrap();
    let id = intent.id;
    assert_eq!(intent.status, Status::Running);
    assert_eq!(fixture.summary(receiver).receipts, 1);
    fixture.upgrade(fixture.sender).unwrap();
    fixture.upgrade(receiver).unwrap();
    fixture.update::<(), _>(Method::DispatchDue, ()).unwrap();
    // Both queues now retain Wire: same-contract upgrade must preserve the
    // complete attempt identity and immutable payload, not just Running state.
    assert_eq!(fixture.view(id), intent);
    assert_eq!(fixture.summary(receiver).receipts, 1);
    fixture
        .update::<(), _>(Method::MarkUncertain, (id,))
        .unwrap();
    let uncertain = fixture.view(id);
    assert_eq!(uncertain.status, Status::Uncertain);
    fixture.upgrade(fixture.sender).unwrap();
    assert_eq!(fixture.view(id), uncertain);
    assert!(
        fixture
            .update::<bool, _>(Method::ReconcileDelivery, (id,))
            .unwrap()
    );
    assert_eq!(fixture.view(id).status, Status::Completed);
    fixture.update::<(), _>(Method::DispatchDue, ()).unwrap();
    assert_eq!(fixture.summary(receiver).receipts, 1);
    let absent: u64 = fixture
        .update(
            Method::CreateBlocked,
            (Create {
                timing: Timing::Once(0),
                task: Task::Deliver(DeliveryPayload {
                    destination: receiver.into(),
                    message: "never sent".into(),
                }),
            },),
        )
        .unwrap();
    fixture.upgrade(fixture.sender).unwrap();
    assert!(
        !fixture
            .update::<bool, _>(Method::ReconcileDelivery, (absent,))
            .unwrap()
    );
    assert_eq!(fixture.view(absent).status, Status::Running);
    assert!(fixture.update::<(), _>(Method::Cancel, (absent,)).is_err());
    for state in [Status::Running, Status::Uncertain] {
        if state == Status::Uncertain {
            fixture
                .update::<(), _>(Method::MarkUncertain, (absent,))
                .unwrap();
        }
        let before = fixture.view(absent);
        assert_eq!(before.status, state);
        for method in [Method::Pause, Method::Resume] {
            assert!(fixture.update::<(), _>(method, (absent,)).is_err());
        }
        assert!(
            fixture
                .update::<(), _>(
                    Method::Reschedule,
                    (
                        absent,
                        Timing::Once(
                            fixture
                                .pic
                                .instance()
                                .get_time()
                                .as_nanos_since_unix_epoch()
                        )
                    )
                )
                .is_err()
        );
        assert_eq!(fixture.view(absent), before);
    }
    assert_eq!(fixture.summary(receiver).receipts, 1);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn invalid_restore_refuses_upgrade_and_preserves_the_controller_snapshot() {
    let fixture = Fixture::new();
    let id: u64 = fixture
        .update(
            Method::Create,
            (increment(
                fixture
                    .pic
                    .instance()
                    .get_time()
                    .as_nanos_since_unix_epoch()
                    + 3_600_000_000_000,
                1,
            ),),
        )
        .unwrap();
    // The controller's independent snapshot remains the explicit recovery owner.
    let backup = fixture.snapshot(fixture.sender);
    fixture
        .update::<(), _>(Method::CorruptRecord, (id,))
        .unwrap();
    // The fixture truncates a current-format Wire snapshot. Upgrade must refuse
    // without replacing the independent controller snapshot used for recovery.
    assert!(matches!(
        fixture
            .query::<View, _>(fixture.sender, Method::Inspect, (id,))
            .unwrap_err(),
        Error::InvalidStorage
    ));
    let rejected = fixture.upgrade(fixture.sender).unwrap_err();
    assert_eq!(rejected.error_code, ErrorCode::CanisterCalledTrap);
    fixture.restore_snapshot(fixture.sender, backup);
    // Snapshot loading consumes installation resources too. Testkit owns the
    // bounded rate-limit retry; every other rejection must fail immediately.
    // The job's one-hour deadline exceeds the maximum thirty-minute cooldown.
    fixture
        .pic
        .instance()
        .retry_install_code(
            RetryPolicy {
                max_attempts: std::num::NonZeroUsize::new(4).unwrap(),
                cooldown: Duration::from_secs(600),
            },
            || fixture.upgrade(fixture.sender),
        )
        .unwrap();
    assert!(matches!(fixture.view(id).status, Status::Pending { .. }));
    assert_eq!(fixture.summary(fixture.sender).counter, 0);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn overdue_fixed_rate_work_reconstructs_catch_up_and_skip_policies() {
    let fixture = Fixture::new();
    let first_ns = fixture
        .pic
        .instance()
        .get_time()
        .as_nanos_since_unix_epoch()
        + 10_000_000_000;
    let mut ids = vec![];
    for catch_up in [false, true] {
        let id: u64 = fixture
            .update(
                Method::Create,
                (Create {
                    timing: Timing::FixedRate {
                        first_ns,
                        every_ns: 1_000_000_000,
                        catch_up,
                    },
                    task: Task::Increment(1),
                },),
            )
            .unwrap();
        ids.push(id);
        fixture.update::<(), _>(Method::Pause, (id,)).unwrap();
    }
    fixture.upgrade(fixture.sender).unwrap();
    fixture.pic.instance().advance_time(Duration::from_secs(13));
    for id in &ids {
        assert_eq!(fixture.view(*id).status, Status::Paused);
        assert_eq!(fixture.view(*id).occurrence, 0);
    }
    assert_eq!(fixture.summary(fixture.sender).counter, 0);
    assert!(!fixture.summary(fixture.sender).armed);
    fixture.upgrade(fixture.sender).unwrap();
    for id in &ids {
        fixture.update::<(), _>(Method::Resume, (*id,)).unwrap();
    }
    fixture.until(|| fixture.view(ids[0]).occurrence == 1 && fixture.view(ids[1]).occurrence >= 1);
    let Status::Pending { due_ns } = fixture.view(ids[0]).status else {
        panic!("overdue occurrence must remain pending");
    };
    assert!(
        due_ns
            > fixture
                .pic
                .instance()
                .get_time()
                .as_nanos_since_unix_epoch()
    );
    for id in ids {
        fixture.update::<(), _>(Method::Cancel, (id,)).unwrap();
    }
    assert!(!fixture.summary(fixture.sender).armed);
    let before = fixture.summary(fixture.sender).counter;
    advance_and_tick(fixture.pic.instance(), Duration::from_secs(30), 3);
    assert_eq!(fixture.summary(fixture.sender).counter, before);
}

#[test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
fn durable_management_reconstructs_wakeups_and_traps_roll_back_changes() {
    let fixture = Fixture::new();
    let id: u64 = fixture
        .update(
            Method::Create,
            (increment(
                fixture
                    .pic
                    .instance()
                    .get_time()
                    .as_nanos_since_unix_epoch()
                    + 30_000_000_000,
                5,
            ),),
        )
        .unwrap();
    for method in [Method::Pause, Method::Resume] {
        let denied: Result<(), Error> = fixture
            .update_as(ExternalPrincipal::anonymous(), method, (id,))
            .expect("PocketIC Wire call");
        assert_eq!(denied, Err(Error::Unauthorized));
    }
    let denied: Result<(), Error> = fixture
        .update_as(
            ExternalPrincipal::anonymous(),
            Method::Reschedule,
            (id, Timing::Once(0)),
        )
        .expect("PocketIC Wire call");
    assert_eq!(denied, Err(Error::Unauthorized));
    fixture.update::<(), _>(Method::Pause, (id,)).unwrap();
    assert_eq!(fixture.view(id).status, Status::Paused);
    assert_eq!(fixture.summary(fixture.sender).next_due_ns, None);
    assert!(!fixture.summary(fixture.sender).armed);
    fixture.upgrade(fixture.sender).unwrap();
    advance_and_tick(fixture.pic.instance(), Duration::from_secs(40), 3);
    assert_eq!(fixture.summary(fixture.sender).counter, 0);
    let later = fixture
        .pic
        .instance()
        .get_time()
        .as_nanos_since_unix_epoch()
        + 100_000_000_000;
    fixture
        .update::<(), _>(Method::Reschedule, (id, Timing::Once(later)))
        .unwrap();
    let paused = fixture.view(id);
    assert_eq!(paused.status, Status::Paused);
    assert_eq!((paused.sequence, paused.occurrence), (0, 0));
    fixture.upgrade(fixture.sender).unwrap();
    assert_eq!(fixture.view(id), paused);
    fixture.update::<(), _>(Method::Resume, (id,)).unwrap();
    assert_eq!(fixture.summary(fixture.sender).next_due_ns, Some(later));
    assert!(fixture.summary(fixture.sender).armed);
    let due = fixture
        .pic
        .instance()
        .get_time()
        .as_nanos_since_unix_epoch()
        + 15_000_000_000;
    fixture
        .update::<(), _>(Method::Reschedule, (id, Timing::Once(due)))
        .unwrap();
    let before = fixture.view(id);
    let summary = fixture.summary(fixture.sender);
    let failed = fixture.update_as::<(), _>(
        fixture.manager,
        Method::TrapAfterReschedule,
        (
            id,
            Timing::Once(
                fixture
                    .pic
                    .instance()
                    .get_time()
                    .as_nanos_since_unix_epoch()
                    + 5_000_000_000,
            ),
        ),
    );
    assert_canister_trap(failed.unwrap_err());
    assert_eq!(fixture.view(id), before);
    assert_eq!(fixture.summary(fixture.sender), summary);
    let failed = fixture.update_as::<(), _>(fixture.manager, Method::TrapAfterPause, (id,));
    assert_canister_trap(failed.unwrap_err());
    assert_eq!(fixture.view(id), before);
    assert_eq!(fixture.summary(fixture.sender), summary);
    fixture.upgrade(fixture.sender).unwrap();
    fixture.pic.instance().advance_time(Duration::from_secs(20));
    fixture.until(|| fixture.view(id).status == Status::Completed);
    assert_eq!(fixture.summary(fixture.sender).counter, 5);
    assert_eq!(fixture.view(id).sequence, 1);
    assert!(!fixture.summary(fixture.sender).armed);
}

#[test]
#[ignore = "explicit local canister qualification: make test-canister"]
fn job_metrics_are_authorized_resettable_and_restart_on_upgrade() {
    use ic_jobs_test_consumer::api::JobMetrics;
    let fixture = Fixture::new();
    let initial: JobMetrics = fixture
        .query(fixture.sender, Method::JobMetrics, ())
        .unwrap();
    assert_eq!(initial.dispatched.total, 0);
    assert_eq!(initial.reset_sequence, Some(0));
    let unauthorized: Result<JobMetrics, Error> = query_wire(
        fixture.pic.instance(),
        fixture.sender,
        ExternalPrincipal::anonymous(),
        wire::control(Method::JobMetrics),
        &(),
    )
    .unwrap();
    assert_eq!(unauthorized, Err(Error::Unauthorized));
    let unauthorized: Result<JobMetrics, Error> = fixture
        .update_as(ExternalPrincipal::anonymous(), Method::ResetJobMetrics, ())
        .unwrap();
    assert_eq!(unauthorized, Err(Error::Unauthorized));
    let id: u64 = fixture.update(Method::Create, (increment(0, 1),)).unwrap();
    fixture.update::<(), _>(Method::DispatchDue, ()).unwrap();
    let completed: JobMetrics = fixture
        .query(fixture.sender, Method::JobMetrics, ())
        .unwrap();
    assert_eq!(completed.dispatched.total, 1);
    assert_eq!(completed.succeeded.total, 1);
    let job = fixture.view(id);
    let reset: JobMetrics = fixture.update(Method::ResetJobMetrics, ()).unwrap();
    assert_eq!(reset.dispatched.total, 0);
    assert_eq!(reset.reset_sequence, Some(1));
    assert_eq!(reset.canister_version, completed.canister_version);
    assert_eq!(fixture.view(id), job);
    fixture.upgrade(fixture.sender).unwrap();
    let reopened: JobMetrics = fixture
        .query(fixture.sender, Method::JobMetrics, ())
        .unwrap();
    assert!(reopened.canister_version > completed.canister_version);
    assert_eq!(reopened.dispatched.total, 0);
    assert_eq!(reopened.reset_sequence, Some(0));
    assert_eq!(fixture.view(id), job);
}

#[test]
#[ignore = "explicit local canister qualification: make test-canister"]
fn job_logs_are_manager_only_and_restart_without_changing_jobs() {
    use ic_jobs_test_consumer::api::{JobLogPage, JobLogRequest, JobMetrics};
    let fixture = Fixture::new();
    let request = JobLogRequest {
        before: None,
        min_level: None,
        limit: 8,
    };
    let initial: JobLogPage = fixture
        .query(fixture.sender, Method::JobLogs, request)
        .unwrap();
    assert!(initial.entries.is_empty());
    let unauthorized: Result<JobLogPage, Error> = query_wire(
        fixture.pic.instance(),
        fixture.sender,
        ExternalPrincipal::anonymous(),
        wire::control(Method::JobLogs),
        &request,
    )
    .unwrap();
    assert_eq!(unauthorized, Err(Error::Unauthorized));
    let id: u64 = fixture.update(Method::Create, (increment(0, 1),)).unwrap();
    fixture.update::<(), _>(Method::DispatchDue, ()).unwrap();
    let completed: JobLogPage = fixture
        .query(fixture.sender, Method::JobLogs, request)
        .unwrap();
    assert_eq!(completed.entries.len(), 2);
    assert_eq!(completed.entries[0].topic, "outcome");
    assert_eq!(completed.entries[1].topic, "dispatch");
    let job = fixture.view(id);
    fixture
        .update::<JobMetrics, _>(Method::ResetJobMetrics, ())
        .unwrap();
    let after_reset: JobLogPage = fixture
        .query(fixture.sender, Method::JobLogs, request)
        .unwrap();
    assert_eq!(after_reset.entries, completed.entries);
    fixture.upgrade(fixture.sender).unwrap();
    let reopened: JobLogPage = fixture
        .query(fixture.sender, Method::JobLogs, request)
        .unwrap();
    assert!(reopened.entries.is_empty());
    assert_eq!(reopened.omitted.total, 0);
    assert!(reopened.canister_version > completed.canister_version);
    assert_eq!(fixture.view(id), job);
}
