//! Explicit live PocketIC tests. Ordinary CI compiles these without launching a server.

use ic_jobs_canister_tests::{
    harness,
    wire::{WireCallError, WireCallSpec, query_wire},
};
use ic_jobs_test_consumer::{
    api::{Create, Delivery, DeliveryPayload, Error, Init, Status, Summary, Task, Timing, View},
    delivery_wire::{RECEIPT_ID, RECEIVE_ID, REQUEST_LIMITS, RESPONSE_LIMITS},
    management_wire::{self as control, Method},
};
use ic_platform_types::Principal;
use ic_testkit::pic::{
    PocketIcInstance, PocketIcStartupConfig, RetryPolicy, advance_and_tick, tick_until,
};
use ic_testkit::pocket_ic::{
    CanisterId, ErrorCode, PocketIcBuilder, RejectResponse, nonblocking::PocketIc,
};
use ic_wire::native;
use serde::{Serialize, de::DeserializeOwned};
use std::{convert::Infallible, path::PathBuf, task::Poll, time::Duration};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

struct Fixture {
    pic: PocketIcInstance,
    wasm: Vec<u8>,
    manager: Principal,
    sender: Principal,
}

impl Fixture {
    async fn now_ns(&self) -> u64 {
        self.pic
            .instance()
            .get_time()
            .await
            .as_nanos_since_unix_epoch()
    }

    async fn close(self) {
        self.pic.close().await;
    }

    async fn install(
        pic: &PocketIc,
        wasm: &[u8],
        manager: Principal,
        delivery_sender: Option<Principal>,
    ) -> Principal {
        let canister = ic_testkit::pic::create_and_install(
            pic,
            harness::install_spec(
                wasm.to_vec(),
                &Init {
                    manager,
                    delivery_sender,
                },
            )
            .unwrap(),
        )
        .await;
        Principal::from_slice(canister.as_slice())
    }

    async fn install_receiver(&self) -> Principal {
        Self::install(
            self.pic.instance(),
            &self.wasm,
            self.manager,
            Some(self.sender),
        )
        .await
    }

    async fn new() -> Self {
        let path = PathBuf::from(
            std::env::var_os("IC_JOBS_CONSUMER_WASM")
                .expect("build-consumer must select IC_JOBS_CONSUMER_WASM"),
        );
        let wasm = std::fs::read(path).expect("read selected consumer Wasm");
        let config = PocketIcStartupConfig::from_env(Duration::from_secs(30))
            .expect("use ic-testkit-server run after explicit setup/check");
        let pic = config
            .build_instance(
                PocketIcBuilder::new()
                    .with_max_request_time_ms(Some(REQUEST_TIMEOUT.as_millis().try_into().unwrap()))
                    .with_application_subnet(),
            )
            .await
            .expect("bounded PocketIC startup");
        let manager = Principal::self_authenticating(b"jobs-manager");
        let sender = Self::install(pic.instance(), &wasm, manager, None).await;
        Self {
            pic,
            wasm,
            manager,
            sender,
        }
    }

    async fn update<T: DeserializeOwned, A: Serialize>(
        &self,
        method: Method,
        args: A,
    ) -> Result<T, Error> {
        self.update_as(self.manager, method, args)
            .await
            .expect("PocketIC Wire call")
    }
    async fn update_as<T: DeserializeOwned, A: Serialize>(
        &self,
        caller: Principal,
        method: Method,
        args: A,
    ) -> Result<T, RejectResponse> {
        match self.client(self.sender, caller).update(method, &args).await {
            Ok(value) => Ok(value),
            Err(WireCallError::Call { source, .. }) => Err(*source),
            Err(other) => panic!("PocketIC Wire contract failed: {other:?}"),
        }
    }
    async fn query<T: DeserializeOwned, A: Serialize>(
        &self,
        id: Principal,
        method: Method,
        args: A,
    ) -> Result<T, Error> {
        self.client(id, self.manager)
            .query(method, &args)
            .await
            .expect("PocketIC Wire call")
    }
    fn client(&self, canister: Principal, caller: Principal) -> harness::Client<'_> {
        harness::Client {
            pic: self.pic.instance(),
            canister,
            caller,
        }
    }
    // Raw frames deliberately bypass typed request admission in rejection tests.
    async fn raw_update(
        &self,
        canister: Principal,
        caller: Principal,
        method: &str,
        bytes: Vec<u8>,
    ) -> Result<Vec<u8>, RejectResponse> {
        self.pic
            .instance()
            .update_call(
                native_principal(canister),
                native_principal(caller),
                method,
                bytes,
            )
            .await
    }

    async fn delivery<T: DeserializeOwned>(
        &self,
        canister: Principal,
        caller: Principal,
        (method, method_id): (&'static str, u64),
        delivery: &Delivery,
    ) -> T {
        ic_jobs_canister_tests::wire::update_wire(
            self.pic.instance(),
            canister,
            caller,
            WireCallSpec {
                method,
                method_id,
                request_limits: REQUEST_LIMITS,
                response_limits: RESPONSE_LIMITS,
            },
            delivery,
        )
        .await
        .expect("PocketIC delivery Wire call")
    }

    async fn stopped<T>(
        &self,
        canister: Principal,
        operation: impl AsyncFnOnce(&PocketIc) -> T,
    ) -> T {
        let pic = self.pic.instance();
        pic.stop_canister(native_principal(canister), None)
            .await
            .unwrap();
        let result = operation(pic).await;
        pic.start_canister(native_principal(canister), None)
            .await
            .unwrap();
        result
    }
    async fn snapshot(&self, canister: Principal) -> Vec<u8> {
        self.stopped(canister, async |pic| {
            pic.take_canister_snapshot(native_principal(canister), None, None)
                .await
                .unwrap()
                .id
        })
        .await
    }
    async fn restore_snapshot(&self, canister: Principal, snapshot: Vec<u8>) {
        self.stopped(canister, async |pic| {
            pic.load_canister_snapshot(native_principal(canister), None, snapshot)
                .await
                .unwrap()
        })
        .await;
    }
    async fn view(&self, id: u64) -> View {
        self.query(self.sender, Method::Inspect, (id,))
            .await
            .unwrap()
    }
    async fn summary(&self, id: Principal) -> Summary {
        self.query(id, Method::Summary, ()).await.unwrap()
    }
    async fn upgrade(&self, id: Principal) -> Result<(), RejectResponse> {
        self.pic
            .instance()
            .upgrade_canister(native_principal(id), self.wasm.clone(), Vec::new(), None)
            .await
    }
    async fn until(&self, predicate: impl AsyncFn() -> bool) {
        tick_until(self.pic.instance(), 30, Duration::ZERO, async |_| {
            if predicate().await {
                Poll::Ready(Ok::<_, Infallible>(()))
            } else {
                Poll::Pending
            }
        })
        .await
        .expect("bounded canister progress did not complete");
    }
}

fn native_principal(principal: Principal) -> CanisterId {
    CanisterId::from_slice(principal.as_slice())
}

fn assert_canister_trap(error: &RejectResponse) {
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
                destination: Principal::self_authenticating(b"receiver"),
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

#[tokio::test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
async fn management_wire_refuses_retired_and_invalid_unit_requests() {
    let fixture = Fixture::new().await;
    let before = fixture.summary(fixture.sender).await;
    for init_bytes in [
        include_bytes!("../../ic-jobs-test-consumer/tests/fixtures/api-pre-platform.didbin")
            .to_vec(),
        vec![0; control::REQUEST_LIMITS.bytes + 1],
    ] {
        let pic = fixture.pic.instance();
        let canister = pic.create_canister().await;
        pic.add_cycles(canister, 10_000_000_000_000).await;
        // Reinstall a fresh empty canister to observe the original init rejection
        // rather than upstream install_canister's panic-on-rejection convenience.
        let error = pic
            .reinstall_canister(canister, fixture.wasm.clone(), init_bytes, None)
            .await
            .unwrap_err();
        assert_canister_trap(&error);
    }
    let id = Method::DispatchDue as u64;
    let mut trailing = native::encode_request(id, &(), control::REQUEST_LIMITS).unwrap();
    trailing.push(0);
    for bytes in [
        include_bytes!("../../ic-jobs-test-consumer/tests/fixtures/api-pre-platform.didbin")
            .to_vec(),
        Vec::new(),
        native::encode_request(Method::Summary as u64, &(), control::REQUEST_LIMITS).unwrap(),
        native::encode_response(id, &(), control::RESPONSE_LIMITS).unwrap(),
        trailing,
        vec![0; control::REQUEST_LIMITS.bytes + 1],
    ] {
        assert_canister_trap(
            &fixture
                .raw_update(
                    fixture.sender,
                    fixture.manager,
                    Method::DispatchDue.name(),
                    bytes,
                )
                .await
                .unwrap_err(),
        );
        assert_eq!(fixture.summary(fixture.sender).await, before);
    }

    fixture.close().await;
}

#[tokio::test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
async fn delivery_wire_endpoints_refuse_candid_and_preserve_authorized_receipts() {
    let fixture = Fixture::new().await;
    let receiver = fixture.install_receiver().await;
    let delivery = Delivery {
        job: 99,
        occurrence: 0,
        sequence: 1,
        payload: DeliveryPayload {
            destination: receiver,
            message: "x".repeat(256),
        },
    };
    let receive = ("receive_delivery", RECEIVE_ID);
    let receipt = ("delivery_receipt", RECEIPT_ID);
    for (method, id, other) in [
        (receive.0, receive.1, receipt.1),
        (receipt.0, receipt.1, receive.1),
    ] {
        for bytes in [
            include_bytes!("../../ic-jobs-test-consumer/tests/fixtures/api-pre-platform.didbin")
                .to_vec(),
            native::encode_request(other, &delivery, REQUEST_LIMITS).unwrap(),
            native::encode_response(id, &delivery, REQUEST_LIMITS).unwrap(),
            vec![0; REQUEST_LIMITS.bytes + 1],
        ] {
            assert_canister_trap(
                &fixture
                    .raw_update(receiver, fixture.sender, method, bytes)
                    .await
                    .unwrap_err(),
            );
            assert_eq!(fixture.summary(receiver).await.receipts, 0);
        }
    }
    assert_eq!(
        fixture
            .delivery::<Result<Delivery, Error>>(
                receiver,
                Principal::anonymous(),
                receive,
                &delivery,
            )
            .await,
        Err(Error::Unauthorized)
    );
    assert_eq!(fixture.summary(receiver).await.receipts, 0);
    for _ in 0..2 {
        assert_eq!(
            fixture
                .delivery::<Result<Delivery, Error>>(receiver, fixture.sender, receive, &delivery,)
                .await,
            Ok(delivery.clone())
        );
        assert_eq!(fixture.summary(receiver).await.receipts, 1);
    }
    fixture.upgrade(receiver).await.unwrap();
    assert_eq!(
        fixture
            .delivery::<Result<Option<Delivery>, Error>>(
                receiver,
                fixture.sender,
                receipt,
                &delivery,
            )
            .await,
        Ok(Some(delivery))
    );
    assert_eq!(fixture.summary(receiver).await.receipts, 1);

    fixture.close().await;
}

#[tokio::test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
async fn management_upgrade_and_real_watchdog_delivery() {
    let fixture = Fixture::new().await;
    let receiver = fixture.install_receiver().await;
    let now = fixture.now_ns().await;
    let unauthorized: Result<u64, Error> = fixture
        .update_as(
            Principal::anonymous(),
            Method::Create,
            (increment(now + 20_000_000_000, 5),),
        )
        .await
        .expect("PocketIC Wire call");
    assert_eq!(unauthorized, Err(Error::Unauthorized));
    let first: u64 = fixture
        .update(Method::Create, (increment(now + 20_000_000_000, 5),))
        .await
        .unwrap();
    let second: u64 = fixture
        .update(Method::Create, (increment(now + 30_000_000_000, 7),))
        .await
        .unwrap();
    let before = fixture.view(second).await;
    assert!(fixture.summary(fixture.sender).await.armed);
    fixture.upgrade(fixture.sender).await.unwrap();
    assert_eq!(fixture.view(second).await, before);
    assert_eq!(
        fixture.summary(fixture.sender).await.next_due_ns,
        Some(now + 20_000_000_000)
    );
    fixture
        .update::<(), _>(Method::Cancel, (first,))
        .await
        .unwrap();
    assert_eq!(
        fixture.summary(fixture.sender).await.next_due_ns,
        Some(now + 30_000_000_000)
    );
    let page: Vec<View> = fixture
        .query(fixture.sender, Method::List, (Some(first), 1_u8))
        .await
        .unwrap();
    assert_eq!(page, vec![before]);
    fixture
        .pic
        .instance()
        .advance_time(Duration::from_secs(40))
        .await;
    fixture
        .until(async || fixture.view(second).await.status == Status::Completed)
        .await;
    assert_eq!(fixture.view(first).await.status, Status::Cancelled);
    assert_eq!(fixture.summary(fixture.sender).await.counter, 7);
    assert!(!fixture.summary(fixture.sender).await.armed);
    let third: u64 = fixture
        .update(
            Method::Create,
            (Create {
                timing: Timing::Once(fixture.now_ns().await + 10_000_000_000),
                task: Task::Deliver(DeliveryPayload {
                    destination: receiver,
                    message: "timer delivery".into(),
                }),
            },),
        )
        .await
        .unwrap();
    fixture.upgrade(fixture.sender).await.unwrap();
    fixture
        .pic
        .instance()
        .advance_time(Duration::from_secs(15))
        .await;
    fixture
        .until(async || fixture.view(third).await.status == Status::Completed)
        .await;
    assert_eq!(fixture.summary(receiver).await.receipts, 1);
    fixture.upgrade(fixture.sender).await.unwrap();
    fixture.upgrade(receiver).await.unwrap();
    advance_and_tick(fixture.pic.instance(), Duration::ZERO, 3).await;
    assert_eq!(fixture.summary(receiver).await.receipts, 1);

    fixture.close().await;
}

#[tokio::test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
async fn synchronous_trap_rolls_back_storage_identity_and_timer_changes() {
    let fixture = Fixture::new().await;
    let id: u64 = fixture
        .update(
            Method::Create,
            (increment(fixture.now_ns().await + 60_000_000_000, 3),),
        )
        .await
        .unwrap();
    let before = fixture.summary(fixture.sender).await;
    let failed = fixture
        .update_as::<(), _>(
            fixture.manager,
            Method::TrapAfterCreate,
            (increment(fixture.now_ns().await + 30_000_000_000, 9),),
        )
        .await;
    assert_canister_trap(&failed.unwrap_err());
    assert_eq!(fixture.summary(fixture.sender).await, before);
    let page: Vec<View> = fixture
        .query(fixture.sender, Method::List, (None::<u64>, 8_u8))
        .await
        .unwrap();
    assert_eq!(page.len(), 1);
    fixture.upgrade(fixture.sender).await.unwrap();
    assert_eq!(fixture.summary(fixture.sender).await, before);
    let next: u64 = fixture
        .update(
            Method::Create,
            (increment(fixture.now_ns().await + 120_000_000_000, 4),),
        )
        .await
        .unwrap();
    assert_eq!(next, id + 1);

    fixture.close().await;
}

#[tokio::test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
async fn watchdog_batch_trap_rolls_back_then_recovers_without_repeating_local_effects() {
    let fixture = Fixture::new().await;
    let due = fixture.now_ns().await + 10_000_000_000;
    let healthy: u64 = fixture
        .update(Method::Create, (increment(due, 1),))
        .await
        .unwrap();
    let failed: u64 = fixture
        .update(Method::Create, (increment(due, u64::MAX),))
        .await
        .unwrap();
    let later_due = due + 30_000_000_000;
    let later: u64 = fixture
        .update(Method::Create, (increment(later_due, 4),))
        .await
        .unwrap();
    let healthy_before = fixture.view(healthy).await;
    let failed_before = fixture.view(failed).await;
    fixture
        .update::<(), _>(Method::TrapWatchdogBatch, (true,))
        .await
        .unwrap();
    fixture
        .pic
        .instance()
        .advance_time(Duration::from_secs(10))
        .await;
    // Observe a real trap, not merely a dispatched callback awaiting execution.
    let marker = b"fixture: watchdog trap after stable batch write";
    fixture
        .until(async || {
            fixture
                .pic
                .instance()
                .fetch_canister_logs(native_principal(fixture.sender), CanisterId::anonymous())
                .await
                .unwrap()
                .iter()
                .any(|record| {
                    record
                        .content
                        .windows(marker.len())
                        .any(|part| part == marker)
                })
        })
        .await;
    assert_eq!(fixture.view(healthy).await, healthy_before);
    assert_eq!(fixture.view(failed).await, failed_before);
    let rolled_back = fixture.summary(fixture.sender).await;
    assert_eq!(rolled_back.counter, 0);
    let metrics: ic_jobs_test_consumer::api::JobMetrics = fixture
        .query(fixture.sender, Method::JobMetrics, ())
        .await
        .unwrap();
    assert_eq!(metrics.dispatched.total, 0);
    assert_eq!(metrics.succeeded.total, 0);
    assert_eq!(metrics.permanent_failures.total, 0);
    assert_eq!(rolled_back.next_due_ns, Some(due));
    assert!(rolled_back.armed);

    // This control does not rearm the timer: recovery must use its survivor.
    fixture
        .update::<(), _>(Method::TrapWatchdogBatch, (false,))
        .await
        .unwrap();
    fixture
        .pic
        .instance()
        .advance_time(Duration::from_secs(2))
        .await;
    fixture
        .until(async || fixture.view(healthy).await.status == Status::Completed)
        .await;
    assert_eq!(fixture.view(failed).await.status, Status::Failed);
    assert_eq!(fixture.view(healthy).await.sequence, 1);
    assert_eq!(fixture.view(failed).await.sequence, 1);
    let recovered = fixture.summary(fixture.sender).await;
    assert_eq!(recovered.counter, 1);
    assert_eq!(recovered.next_due_ns, Some(later_due));
    assert!(recovered.armed);
    assert_eq!(
        fixture.view(later).await.status,
        Status::Pending { due_ns: later_due }
    );

    fixture
        .pic
        .instance()
        .advance_time(Duration::from_secs(40))
        .await;
    fixture
        .until(async || fixture.view(later).await.status == Status::Completed)
        .await;
    assert_eq!(fixture.summary(fixture.sender).await.counter, 5);
    assert!(!fixture.summary(fixture.sender).await.armed);
    fixture.upgrade(fixture.sender).await.unwrap();
    advance_and_tick(fixture.pic.instance(), Duration::from_secs(3), 12).await;
    assert_eq!(fixture.summary(fixture.sender).await.counter, 5);
    assert_eq!(fixture.view(failed).await.status, Status::Failed);
    assert!(!fixture.summary(fixture.sender).await.armed);

    fixture.close().await;
}

#[tokio::test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
async fn lost_result_keeps_intent_blocked_and_receipt_resolution_does_not_redeliver() {
    let fixture = Fixture::new().await;
    let receiver = fixture.install_receiver().await;
    let input = Create {
        timing: Timing::Once(0),
        task: Task::Deliver(DeliveryPayload {
            destination: receiver,
            message: "retained external effect".into(),
        }),
    };
    let failed = fixture
        .update_as::<Result<(), Error>, _>(fixture.manager, Method::DispatchTrapResult, (input,))
        .await;
    assert_canister_trap(&failed.unwrap_err());
    let page: Vec<View> = fixture
        .query(fixture.sender, Method::List, (None::<u64>, 8_u8))
        .await
        .unwrap();
    let intent = page.into_iter().next().unwrap();
    let id = intent.id;
    assert_eq!(intent.status, Status::Running);
    assert_eq!(fixture.summary(receiver).await.receipts, 1);
    fixture.upgrade(fixture.sender).await.unwrap();
    fixture.upgrade(receiver).await.unwrap();
    fixture
        .update::<(), _>(Method::DispatchDue, ())
        .await
        .unwrap();
    // Both queues now retain Wire: same-contract upgrade must preserve the
    // complete attempt identity and immutable payload, not just Running state.
    assert_eq!(fixture.view(id).await, intent);
    assert_eq!(fixture.summary(receiver).await.receipts, 1);
    fixture
        .update::<(), _>(Method::MarkUncertain, (id,))
        .await
        .unwrap();
    let uncertain = fixture.view(id).await;
    assert_eq!(uncertain.status, Status::Uncertain);
    fixture.upgrade(fixture.sender).await.unwrap();
    assert_eq!(fixture.view(id).await, uncertain);
    assert!(
        fixture
            .update::<bool, _>(Method::ReconcileDelivery, (id,))
            .await
            .unwrap()
    );
    assert_eq!(fixture.view(id).await.status, Status::Completed);
    fixture
        .update::<(), _>(Method::DispatchDue, ())
        .await
        .unwrap();
    assert_eq!(fixture.summary(receiver).await.receipts, 1);
    let absent: u64 = fixture
        .update(
            Method::CreateBlocked,
            (Create {
                timing: Timing::Once(0),
                task: Task::Deliver(DeliveryPayload {
                    destination: receiver,
                    message: "never sent".into(),
                }),
            },),
        )
        .await
        .unwrap();
    fixture.upgrade(fixture.sender).await.unwrap();
    assert!(
        !fixture
            .update::<bool, _>(Method::ReconcileDelivery, (absent,))
            .await
            .unwrap()
    );
    assert_eq!(fixture.view(absent).await.status, Status::Running);
    assert!(
        fixture
            .update::<(), _>(Method::Cancel, (absent,))
            .await
            .is_err()
    );
    for state in [Status::Running, Status::Uncertain] {
        if state == Status::Uncertain {
            fixture
                .update::<(), _>(Method::MarkUncertain, (absent,))
                .await
                .unwrap();
        }
        let before = fixture.view(absent).await;
        assert_eq!(before.status, state);
        for method in [Method::Pause, Method::Resume] {
            assert!(fixture.update::<(), _>(method, (absent,)).await.is_err());
        }
        assert!(
            fixture
                .update::<(), _>(
                    Method::Reschedule,
                    (absent, Timing::Once(fixture.now_ns().await))
                )
                .await
                .is_err()
        );
        assert_eq!(fixture.view(absent).await, before);
    }
    assert_eq!(fixture.summary(receiver).await.receipts, 1);

    fixture.close().await;
}

#[tokio::test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
async fn retired_storage_backing_refuses_upgrade_without_changing_stable_bytes() {
    let image: serde_json::Value = serde_json::from_str(include_str!(
        "../../ic-jobs-test-consumer/src/queue/fixtures/stable-image-storage071.json"
    ))
    .unwrap();
    assert_eq!(image["pages"], 23);
    let mut bytes = vec![0; 23 * 65_536];
    let runs: Vec<(usize, String)> = serde_json::from_value(image["runs"].clone()).unwrap();
    for (offset, run) in runs {
        for (index, pair) in run.as_bytes().as_chunks::<2>().0.iter().enumerate() {
            bytes[offset + index] =
                u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap();
        }
    }
    let fixture = Fixture::new().await;
    let pic = fixture.pic.instance();
    let canister = native_principal(fixture.sender);
    pic.stop_canister(canister, None).await.unwrap();
    // Seed the exact prior-format backing in a stopped qualification canister.
    // This tests new post-upgrade refusal, not an old application's execution.
    pic.set_stable_memory(
        canister,
        bytes.clone(),
        ic_testkit::pocket_ic::common::rest::BlobCompression::NoCompression,
    )
    .await;
    assert_eq!(pic.get_stable_memory(canister).await, bytes);
    assert_canister_trap(&fixture.upgrade(fixture.sender).await.unwrap_err());
    assert_eq!(pic.get_stable_memory(canister).await, bytes);
    fixture.close().await;
}

#[tokio::test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
async fn invalid_restore_refuses_upgrade_and_preserves_the_controller_snapshot() {
    let fixture = Fixture::new().await;
    let id: u64 = fixture
        .update(
            Method::Create,
            (increment(fixture.now_ns().await + 3_600_000_000_000, 1),),
        )
        .await
        .unwrap();
    // The controller's independent snapshot remains the explicit recovery owner.
    let backup = fixture.snapshot(fixture.sender).await;
    fixture
        .update::<(), _>(Method::CorruptRecord, (id,))
        .await
        .unwrap();
    // The fixture truncates a current-format Wire snapshot. Upgrade must refuse
    // without replacing the independent controller snapshot used for recovery.
    assert!(matches!(
        fixture
            .query::<View, _>(fixture.sender, Method::Inspect, (id,))
            .await
            .unwrap_err(),
        Error::InvalidStorage
    ));
    let rejected = fixture.upgrade(fixture.sender).await.unwrap_err();
    assert_canister_trap(&rejected);
    fixture.restore_snapshot(fixture.sender, backup).await;
    // Snapshot loading consumes installation resources too. Testkit owns the
    // bounded rate-limit retry; every other rejection must fail immediately.
    // The job's one-hour deadline exceeds the maximum thirty-minute cooldown.
    ic_testkit::pic::retry_install_code(
        fixture.pic.instance(),
        RetryPolicy {
            max_attempts: std::num::NonZeroUsize::new(4).unwrap(),
            cooldown: Duration::from_secs(600),
        },
        async || fixture.upgrade(fixture.sender).await,
    )
    .await
    .unwrap();
    assert!(matches!(
        fixture.view(id).await.status,
        Status::Pending { .. }
    ));
    assert_eq!(fixture.summary(fixture.sender).await.counter, 0);

    fixture.close().await;
}

#[tokio::test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
async fn overdue_fixed_rate_work_reconstructs_catch_up_and_skip_policies() {
    let fixture = Fixture::new().await;
    let first_ns = fixture.now_ns().await + 10_000_000_000;
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
            .await
            .unwrap();
        ids.push(id);
        fixture.update::<(), _>(Method::Pause, (id,)).await.unwrap();
    }
    fixture.upgrade(fixture.sender).await.unwrap();
    fixture
        .pic
        .instance()
        .advance_time(Duration::from_secs(13))
        .await;
    for id in &ids {
        assert_eq!(fixture.view(*id).await.status, Status::Paused);
        assert_eq!(fixture.view(*id).await.occurrence, 0);
    }
    assert_eq!(fixture.summary(fixture.sender).await.counter, 0);
    assert!(!fixture.summary(fixture.sender).await.armed);
    fixture.upgrade(fixture.sender).await.unwrap();
    for id in &ids {
        fixture
            .update::<(), _>(Method::Resume, (*id,))
            .await
            .unwrap();
    }
    fixture
        .until(async || {
            fixture.view(ids[0]).await.occurrence == 1 && fixture.view(ids[1]).await.occurrence >= 1
        })
        .await;
    let Status::Pending { due_ns } = fixture.view(ids[0]).await.status else {
        panic!("overdue occurrence must remain pending");
    };
    assert!(due_ns > fixture.now_ns().await);
    for id in ids {
        fixture
            .update::<(), _>(Method::Cancel, (id,))
            .await
            .unwrap();
    }
    assert!(!fixture.summary(fixture.sender).await.armed);
    let before = fixture.summary(fixture.sender).await.counter;
    advance_and_tick(fixture.pic.instance(), Duration::from_secs(30), 3).await;
    assert_eq!(fixture.summary(fixture.sender).await.counter, before);

    fixture.close().await;
}

#[tokio::test]
#[ignore = "explicit PocketIC qualification; run make test-canister"]
async fn durable_management_reconstructs_wakeups_and_traps_roll_back_changes() {
    let fixture = Fixture::new().await;
    let id: u64 = fixture
        .update(
            Method::Create,
            (increment(fixture.now_ns().await + 30_000_000_000, 5),),
        )
        .await
        .unwrap();
    for method in [Method::Pause, Method::Resume] {
        let denied: Result<(), Error> = fixture
            .update_as(Principal::anonymous(), method, (id,))
            .await
            .expect("PocketIC Wire call");
        assert_eq!(denied, Err(Error::Unauthorized));
    }
    let denied: Result<(), Error> = fixture
        .update_as(
            Principal::anonymous(),
            Method::Reschedule,
            (id, Timing::Once(0)),
        )
        .await
        .expect("PocketIC Wire call");
    assert_eq!(denied, Err(Error::Unauthorized));
    fixture.update::<(), _>(Method::Pause, (id,)).await.unwrap();
    assert_eq!(fixture.view(id).await.status, Status::Paused);
    assert_eq!(fixture.summary(fixture.sender).await.next_due_ns, None);
    assert!(!fixture.summary(fixture.sender).await.armed);
    fixture.upgrade(fixture.sender).await.unwrap();
    advance_and_tick(fixture.pic.instance(), Duration::from_secs(40), 3).await;
    assert_eq!(fixture.summary(fixture.sender).await.counter, 0);
    let later = fixture.now_ns().await + 100_000_000_000;
    fixture
        .update::<(), _>(Method::Reschedule, (id, Timing::Once(later)))
        .await
        .unwrap();
    let paused = fixture.view(id).await;
    assert_eq!(paused.status, Status::Paused);
    assert_eq!((paused.sequence, paused.occurrence), (0, 0));
    fixture.upgrade(fixture.sender).await.unwrap();
    assert_eq!(fixture.view(id).await, paused);
    fixture
        .update::<(), _>(Method::Resume, (id,))
        .await
        .unwrap();
    assert_eq!(
        fixture.summary(fixture.sender).await.next_due_ns,
        Some(later)
    );
    assert!(fixture.summary(fixture.sender).await.armed);
    let due = fixture.now_ns().await + 15_000_000_000;
    fixture
        .update::<(), _>(Method::Reschedule, (id, Timing::Once(due)))
        .await
        .unwrap();
    let before = fixture.view(id).await;
    let summary = fixture.summary(fixture.sender).await;
    let failed = fixture
        .update_as::<(), _>(
            fixture.manager,
            Method::TrapAfterReschedule,
            (id, Timing::Once(fixture.now_ns().await + 5_000_000_000)),
        )
        .await;
    assert_canister_trap(&failed.unwrap_err());
    assert_eq!(fixture.view(id).await, before);
    assert_eq!(fixture.summary(fixture.sender).await, summary);
    let failed = fixture
        .update_as::<(), _>(fixture.manager, Method::TrapAfterPause, (id,))
        .await;
    assert_canister_trap(&failed.unwrap_err());
    assert_eq!(fixture.view(id).await, before);
    assert_eq!(fixture.summary(fixture.sender).await, summary);
    fixture.upgrade(fixture.sender).await.unwrap();
    fixture
        .pic
        .instance()
        .advance_time(Duration::from_secs(20))
        .await;
    fixture
        .until(async || fixture.view(id).await.status == Status::Completed)
        .await;
    assert_eq!(fixture.summary(fixture.sender).await.counter, 5);
    assert_eq!(fixture.view(id).await.sequence, 1);
    assert!(!fixture.summary(fixture.sender).await.armed);

    fixture.close().await;
}

#[tokio::test]
#[ignore = "explicit local canister qualification: make test-canister"]
async fn job_metrics_are_authorized_resettable_and_restart_on_upgrade() {
    use ic_jobs_test_consumer::api::JobMetrics;
    let fixture = Fixture::new().await;
    let initial: JobMetrics = fixture
        .query(fixture.sender, Method::JobMetrics, ())
        .await
        .unwrap();
    assert_eq!(initial.dispatched.total, 0);
    assert_eq!(initial.reset_sequence, Some(0));
    let unauthorized: Result<JobMetrics, Error> = query_wire(
        fixture.pic.instance(),
        fixture.sender,
        Principal::anonymous(),
        harness::control(Method::JobMetrics),
        &(),
    )
    .await
    .unwrap();
    assert_eq!(unauthorized, Err(Error::Unauthorized));
    let unauthorized: Result<JobMetrics, Error> = fixture
        .update_as(Principal::anonymous(), Method::ResetJobMetrics, ())
        .await
        .unwrap();
    assert_eq!(unauthorized, Err(Error::Unauthorized));
    let id: u64 = fixture
        .update(Method::Create, (increment(0, 1),))
        .await
        .unwrap();
    fixture
        .update::<(), _>(Method::DispatchDue, ())
        .await
        .unwrap();
    let completed: JobMetrics = fixture
        .query(fixture.sender, Method::JobMetrics, ())
        .await
        .unwrap();
    assert_eq!(completed.dispatched.total, 1);
    assert_eq!(completed.succeeded.total, 1);
    let job = fixture.view(id).await;
    let reset: JobMetrics = fixture.update(Method::ResetJobMetrics, ()).await.unwrap();
    assert_eq!(reset.dispatched.total, 0);
    assert_eq!(reset.reset_sequence, Some(1));
    assert_eq!(reset.canister_version, completed.canister_version);
    assert_eq!(fixture.view(id).await, job);
    fixture.upgrade(fixture.sender).await.unwrap();
    let reopened: JobMetrics = fixture
        .query(fixture.sender, Method::JobMetrics, ())
        .await
        .unwrap();
    assert!(reopened.canister_version > completed.canister_version);
    assert_eq!(reopened.dispatched.total, 0);
    assert_eq!(reopened.reset_sequence, Some(0));
    assert_eq!(fixture.view(id).await, job);

    fixture.close().await;
}
