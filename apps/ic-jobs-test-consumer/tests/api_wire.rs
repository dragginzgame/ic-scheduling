//! Current management Wire contract and refusal of frozen pre-cut Candid bytes.
use ic_jobs_test_consumer::{
    api::{Create, Delivery, DeliveryPayload, Init, Task, Timing},
    delivery_wire,
    management_wire::{Method, REQUEST_LIMITS, RESPONSE_LIMITS},
};
use ic_platform_runtime::Principal;
use ic_wire::native;

#[test]
fn principal_adoption_preserves_wire_frames_and_stored_json() {
    #[derive(serde::Deserialize)]
    struct Fixture {
        bytes: Vec<u8>,
        init_frame: Vec<u8>,
        init_json: serde_json::Value,
        delivery_frame: Vec<u8>,
        delivery_json: serde_json::Value,
    }
    // Frozen independent external-Principal producer; see fixtures/README.md.
    let fixtures: [Fixture; 30] =
        serde_json::from_slice(include_bytes!("fixtures/principal-platform076.json")).unwrap();
    for (length, fixture) in fixtures.into_iter().enumerate() {
        assert_eq!(fixture.bytes.len(), length);
        let owned = Principal::from_slice(&fixture.bytes);
        let init = Init {
            manager: owned,
            delivery_sender: Some(owned),
        };
        assert_eq!(
            native::encode_request(Method::Init as u64, &init, REQUEST_LIMITS).unwrap(),
            fixture.init_frame
        );
        assert_eq!(
            native::decode_request::<Init>(
                Method::Init as u64,
                &fixture.init_frame,
                REQUEST_LIMITS
            )
            .unwrap(),
            init
        );
        assert_eq!(
            serde_json::from_value::<Init>(fixture.init_json.clone()).unwrap(),
            init
        );
        assert_eq!(serde_json::to_value(&init).unwrap(), fixture.init_json);

        let delivery = Delivery {
            job: u64::MAX,
            occurrence: u64::MAX,
            sequence: u64::MAX,
            payload: DeliveryPayload {
                destination: owned,
                message: "x".repeat(256),
            },
        };
        assert_eq!(
            native::encode_request(
                delivery_wire::RECEIVE_ID,
                &delivery,
                delivery_wire::REQUEST_LIMITS,
            )
            .unwrap(),
            fixture.delivery_frame
        );
        assert_eq!(
            native::decode_request::<Delivery>(
                delivery_wire::RECEIVE_ID,
                &fixture.delivery_frame,
                delivery_wire::REQUEST_LIMITS,
            )
            .unwrap(),
            delivery
        );
        assert_eq!(
            serde_json::to_value(&delivery).unwrap(),
            fixture.delivery_json
        );
        assert_eq!(
            serde_json::from_value::<Delivery>(fixture.delivery_json.clone()).unwrap(),
            delivery
        );
    }
}

#[test]
fn management_contract_admits_current_requests_and_refuses_retired_candid() {
    let init = Init {
        manager: Principal::anonymous(),
        delivery_sender: None,
    };
    let bytes = native::encode_request(Method::Init as u64, &init, REQUEST_LIMITS).unwrap();
    assert_eq!(
        native::decode_request::<Init>(Method::Init as u64, &bytes, REQUEST_LIMITS).unwrap(),
        init
    );
    assert!(native::decode_request::<Init>(Method::Create as u64, &bytes, REQUEST_LIMITS).is_err());
    assert!(
        native::decode_request::<Init>(
            Method::Init as u64,
            include_bytes!("fixtures/api-pre-platform.didbin"),
            REQUEST_LIMITS
        )
        .is_err()
    );

    let create = Create {
        timing: Timing::Once(100),
        task: Task::Increment(7),
    };
    // Keep the singleton argument tuple in both the typed endpoint and harness:
    // its frame bytes and traversal budgets match the original Platform hook.
    let bytes = native::encode_request(Method::Create as u64, &(create,), REQUEST_LIMITS).unwrap();
    let (decoded,): (Create,) =
        native::decode_request(Method::Create as u64, &bytes, REQUEST_LIMITS).unwrap();
    assert!(matches!(decoded.timing, Timing::Once(100)));
    assert_eq!(decoded.task, Task::Increment(7));

    let bytes =
        native::encode_request(Method::List as u64, &(Some(4_u64), 8_u8), REQUEST_LIMITS).unwrap();
    assert_eq!(
        native::decode_request::<(Option<u64>, u8)>(Method::List as u64, &bytes, REQUEST_LIMITS)
            .unwrap(),
        (Some(4), 8)
    );
    let bytes = native::encode_request(Method::DispatchDue as u64, &(), REQUEST_LIMITS).unwrap();
    assert!(
        native::decode_request::<()>(Method::DispatchDue as u64, &bytes, REQUEST_LIMITS).is_ok()
    );
    let mut trailing = bytes;
    trailing.push(0);
    assert!(
        native::decode_request::<()>(Method::DispatchDue as u64, &trailing, REQUEST_LIMITS)
            .is_err()
    );
    let response =
        native::encode_response(Method::DispatchDue as u64, &(), RESPONSE_LIMITS).unwrap();
    assert!(
        native::decode_request::<()>(Method::DispatchDue as u64, &response, REQUEST_LIMITS)
            .is_err()
    );
}

#[test]
fn job_metrics_wire_keeps_wide_totals_quality_and_distinct_reset_identity() {
    use ic_jobs_test_consumer::api::{CounterMetric, Error, JobMetrics};
    let exact = CounterMetric {
        total: u128::MAX,
        overflowed: false,
    };
    let clipped = CounterMetric {
        total: u128::MAX,
        overflowed: true,
    };
    let report = JobMetrics {
        canister_version: 7,
        reset_sequence: None,
        dispatched: exact,
        retry_dispatches: clipped,
        succeeded: exact,
        retryable_failures: exact,
        permanent_failures: exact,
        uncertain: exact,
        reconciliations: exact,
    };
    let frame = native::encode_response(
        Method::JobMetrics as u64,
        &Ok::<_, Error>(report),
        RESPONSE_LIMITS,
    )
    .unwrap();
    assert_eq!(
        native::decode_response::<Result<JobMetrics, Error>>(
            Method::JobMetrics as u64,
            &frame,
            RESPONSE_LIMITS
        )
        .unwrap(),
        Ok(report)
    );
    let reset =
        native::encode_request(Method::ResetJobMetrics as u64, &(), REQUEST_LIMITS).unwrap();
    assert!(
        native::decode_request::<()>(Method::ResetJobMetrics as u64, &reset, REQUEST_LIMITS)
            .is_ok()
    );
    assert!(
        native::decode_request::<()>(Method::JobMetrics as u64, &reset, REQUEST_LIMITS).is_err()
    );
}

#[test]
fn job_logs_wire_admits_the_largest_page_and_binds_its_method() {
    use ic_jobs_test_consumer::api::{
        CounterMetric, Error, JobLogEntry, JobLogLevel, JobLogPage, JobLogRequest,
    };
    let request = JobLogRequest {
        before: Some(u64::MAX),
        min_level: Some(JobLogLevel::Warn),
        limit: 8,
    };
    let bytes = native::encode_request(Method::JobLogs as u64, &request, REQUEST_LIMITS).unwrap();
    assert_eq!(
        native::decode_request::<JobLogRequest>(Method::JobLogs as u64, &bytes, REQUEST_LIMITS)
            .unwrap(),
        request
    );
    assert!(
        native::decode_request::<JobLogRequest>(Method::JobMetrics as u64, &bytes, REQUEST_LIMITS)
            .is_err()
    );
    let page = JobLogPage {
        canister_version: u64::MAX,
        entries: vec![
            JobLogEntry {
                sequence: u64::MAX,
                created_at_secs: u64::MAX,
                level: JobLogLevel::Error,
                topic: "reconciliation".into(),
                message: "x".repeat(192),
                truncated: true,
            };
            8
        ],
        scanned: 32,
        next_before: Some(u64::MAX),
        omitted: CounterMetric {
            total: u128::MAX,
            overflowed: true,
        },
    };
    let bytes = native::encode_response(
        Method::JobLogs as u64,
        &Ok::<_, Error>(&page),
        RESPONSE_LIMITS,
    )
    .unwrap();
    assert_eq!(
        native::decode_response::<Result<JobLogPage, Error>>(
            Method::JobLogs as u64,
            &bytes,
            RESPONSE_LIMITS
        )
        .unwrap(),
        Ok(page)
    );
    assert!(
        native::decode_response::<Result<JobLogPage, Error>>(
            Method::JobLogs as u64,
            &bytes,
            ic_wire::Limits {
                bytes: bytes.len() - 1,
                ..RESPONSE_LIMITS
            }
        )
        .is_err()
    );
}
