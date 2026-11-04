//! Installed ingress qualification subjects.

use super::*;

#[test]
fn competing_global_timer_writes_trap_and_roll_back_timer_control() {
    let (owned, canister_id) = installed_probe();
    let pic = owned.instance();
    let caller = ExternalPrincipal::anonymous();
    let now = pic
        .get_time()
        .expect("read IC time")
        .as_nanos_since_unix_epoch();
    let foreign = now + 100_000_000_000;
    let set_foreign = |timestamp| {
        update_wire::<u64, _>(
            pic,
            canister_id,
            caller,
            Method::FixtureGlobalTimerSet,
            (timestamp,),
        )
        .unwrap()
    };
    let assert_conflict = |method: Method, args: Vec<u8>| {
        let rejected = canister_rejection(
            pic.update_call(canister_id, caller, method.name(), args)
                .unwrap_err(),
        );
        assert!(
            rejected
                .reject_message
                .contains("competing global timer owner"),
            "{rejected:?}"
        );
    };
    use ic_timers_test_protocol::rpc::REQUEST_LIMITS;
    use ic_wire::native::encode_request;

    // First acquisition refuses an already occupied global timer. Trapping
    // rolls back registration, queue publication and the system timer write.
    assert_eq!(set_foreign(foreign), 0);
    assert_conflict(
        Method::Start,
        encode_request(Method::Start as u64, &(), REQUEST_LIMITS).unwrap(),
    );
    assert!(!snapshot(pic, canister_id).registered);
    assert_eq!(set_foreign(0), foreign);
    assert!(update_bool(pic, canister_id, Method::Start));
    let armed = snapshot(pic, canister_id);
    let deadline = armed.next_deadline_ns.unwrap();

    assert_eq!(set_foreign(foreign), deadline);
    assert_conflict(
        Method::ScheduleAt,
        encode_request(Method::ScheduleAt as u64, &(deadline + 1,), REQUEST_LIMITS).unwrap(),
    );
    assert_eq!(snapshot(pic, canister_id), armed);
    assert_eq!(set_foreign(deadline), foreign);

    assert_eq!(set_foreign(0), deadline);
    assert_conflict(
        Method::Stop,
        encode_request(Method::Stop as u64, &(), REQUEST_LIMITS).unwrap(),
    );
    assert_eq!(snapshot(pic, canister_id), armed);
    assert_eq!(set_foreign(deadline), 0);

    // Restore the legitimate state solely within this adversarial fixture.
    // Normal rearm, actual firing (prior timestamp zero) and cancellation work.
    update_wire::<(), _>(
        pic,
        canister_id,
        caller,
        Method::ScheduleAt,
        (deadline + 1,),
    )
    .unwrap();
    pic.advance_time(Duration::from_nanos(1_000_000_001))
        .expect("advance IC time");
    let completed = wait_for_probe(
        pic,
        32,
        |pic| snapshot(pic, canister_id),
        |s| s.completed_work == 1,
    );
    assert_eq!(completed.scheduler_started.total, 1);
    update_unit(pic, canister_id, Method::Stop);
    assert_eq!(snapshot(pic, canister_id).next_deadline_ns, None);
    advance_and_tick(pic, Duration::from_secs(2), 16).expect("advance and tick PocketIC");
    assert_eq!(snapshot(pic, canister_id).completed_work, 1);
}

#[test]
fn host_inspection_refuses_timer_ingress_but_foreign_calls_still_authenticate() {
    let (owned, canister_id) = installed_probe();
    let pic = owned.instance();
    let relay_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, Method::Start));
    let before = snapshot(pic, canister_id);
    let balance = pic
        .cycle_balance(canister_id)
        .expect("read canister cycles");
    // The first pending dispatch ID is valid: admission must not rely on a
    // malformed payload or a nonexistent task to prevent early execution.
    let payload = 1_u64.to_le_bytes().to_vec();
    let denied = canister_rejection(
        pic.update_call(
            canister_id,
            ExternalPrincipal::anonymous(),
            TIMER_EXECUTOR_METHOD,
            payload.clone(),
        )
        .unwrap_err(),
    );
    assert_eq!(denied.error_code, ErrorCode::CanisterCalledTrap);
    assert!(
        denied
            .reject_message
            .contains("probe ingress: timer dispatch is internal")
    );
    assert_eq!(snapshot(pic, canister_id), before);
    assert_eq!(
        pic.cycle_balance(canister_id)
            .expect("read canister cycles"),
        balance
    );

    // The host's existing payload policy still runs for non-timer methods.
    let denied = canister_rejection(
        pic.update_call(
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::Start.name(),
            vec![0; ic_timers_test_protocol::rpc::REQUEST_LIMITS.bytes + 1],
        )
        .unwrap_err(),
    );
    assert_eq!(denied.error_code, ErrorCode::CanisterCalledTrap);
    assert!(
        denied
            .reject_message
            .contains("probe ingress: host payload limit")
    );
    assert_eq!(snapshot(pic, canister_id), before);

    let balance = pic
        .cycle_balance(canister_id)
        .expect("read canister cycles");
    let rejected: String = update_wire(
        pic,
        relay_id,
        ExternalPrincipal::anonymous(),
        Method::FixtureForeignDispatch,
        (canister_id.as_slice().to_vec(), payload),
    )
    .unwrap();
    assert!(rejected.contains("self-call required"), "{rejected}");
    assert_eq!(snapshot(pic, canister_id), before);
    let foreign_cost = balance
        - pic
            .cycle_balance(canister_id)
            .expect("read canister cycles");
    assert!(
        foreign_cost > 0,
        "endpoint authentication still costs target cycles"
    );
    println!("foreign_dispatch_authentication_cycles={foreign_cost}");

    pic.advance_time(Duration::from_secs(1))
        .expect("advance IC time");
    let completed = wait_for_probe(
        pic,
        32,
        |pic| snapshot(pic, canister_id),
        |s| s.completed_work == 1,
    );
    assert_eq!(completed.scheduler_started.total, 1);
    assert_eq!(completed.work_started.total, 1);
}

#[test]
fn typed_entrypoints_reject_invalid_wire_before_mutating_timer_state() {
    use ic_timers_test_protocol::rpc::{REQUEST_LIMITS, RESPONSE_LIMITS};
    use ic_wire::native::{decode_response, encode_request, encode_response};
    let (owned, canister_id) = installed_probe();
    let pic = owned.instance();
    let id = Method::StartOrdinary as u64;
    let valid = encode_request(
        id,
        &(OrdinaryPolicy::Once, ProbeLifetime::Retained),
        REQUEST_LIMITS,
    )
    .unwrap();
    let mut truncated = valid.clone();
    truncated.pop();
    for arguments in [
        b"not wire".to_vec(),
        encode_request(id, &(2_u8, ProbeLifetime::Retained), REQUEST_LIMITS).unwrap(),
        encode_request(id, &(OrdinaryPolicy::Once, 2_u8), REQUEST_LIMITS).unwrap(),
        encode_request(id, &(false,), REQUEST_LIMITS).unwrap(),
        encode_request(id, &(false, false, 0_u8), REQUEST_LIMITS).unwrap(),
        encode_request(
            Method::TrapOrdinaryWork as u64,
            &(false, false),
            REQUEST_LIMITS,
        )
        .unwrap(),
        encode_response(id, &(false, false), RESPONSE_LIMITS).unwrap(),
        vec![0; REQUEST_LIMITS.bytes + 1],
        truncated,
    ] {
        canister_rejection(
            pic.update_call(
                canister_id,
                ExternalPrincipal::anonymous(),
                Method::StartOrdinary.name(),
                arguments,
            )
            .expect_err("invalid Wire arguments must reject before registration"),
        );
        assert!(!ordinary_observation(pic, canister_id).declared);
        assert_eq!(snapshot(pic, canister_id).scheduler_started.total, 0);
    }
    // Wire 0.7 rejects nonminimal varints during decoding, before the endpoint
    // can reach its missing-registration trap or reconcile a timer deadline.
    let mut nonminimal =
        encode_request(Method::ScheduleAt as u64, &(0_u64,), REQUEST_LIMITS).unwrap();
    assert_eq!(nonminimal.pop(), Some(0));
    nonminimal.extend_from_slice(&[0x80, 0]);
    let rejection = canister_rejection(
        pic.update_call(
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::ScheduleAt.name(),
            nonminimal,
        )
        .unwrap_err(),
    );
    assert_eq!(rejection.error_code, ErrorCode::CanisterCalledTrap);
    assert!(
        rejection.reject_message.contains("wire Malformed"),
        "{rejection:?}"
    );
    assert_eq!(snapshot(pic, canister_id).scheduler_started.total, 0);
    // Exercise our protocol-to-Testkit adapter at its three failure boundaries.
    // Raw malformed frames above remain direct calls so no client normalizes them.
    let caller = ExternalPrincipal::anonymous();
    let error = update_wire::<(), _>(
        pic,
        canister_id,
        caller,
        Method::StartOrdinary,
        vec![0_u8; REQUEST_LIMITS.bytes + 1],
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ic_testkit::pic::WireCallError::Encode { .. }
    ));
    let error = update_wire::<(), _>(
        pic,
        canister_id,
        caller,
        Method::StartOrdinary,
        (2_u8, false),
    )
    .unwrap_err();
    let ic_testkit::pic::WireCallError::Call { context, source } = error else {
        panic!("expected probe rejection, got {error}");
    };
    assert_eq!(context.operation, "update_call");
    assert_eq!(context.canister_id, canister_id);
    assert_eq!(context.caller, caller);
    assert_eq!(context.method, Method::StartOrdinary.name());
    assert_eq!(context.method_id, id);
    assert!(matches!(
        source.as_ref(),
        ic_testkit::pocket_ic::ClientError::Canister(_)
    ));
    let error = query_wire::<bool, _>(pic, canister_id, caller, Method::OrdinaryObservation, ())
        .unwrap_err();
    let ic_testkit::pic::WireCallError::Decode { context, .. } = error else {
        panic!("expected response shape rejection, got {error}");
    };
    assert_eq!(context.operation, "query_call");
    assert_eq!(context.method_id, Method::OrdinaryObservation as u64);
    assert!(!ordinary_observation(pic, canister_id).declared);
    let reply = pic
        .update_call(
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::StartOrdinary.name(),
            valid,
        )
        .expect("valid arguments still work after rejected input");
    decode_response::<()>(id, &reply, RESPONSE_LIMITS).expect("method-bound unit reply");
    // Responses need Deserialize only, including through our native adapter.
    #[derive(Debug, serde::Deserialize)]
    #[serde(transparent)]
    struct ResponseOnly(OrdinaryObservation);
    let observed =
        query_wire::<ResponseOnly, _>(pic, canister_id, caller, Method::OrdinaryObservation, ())
            .unwrap();
    assert!(observed.0.declared);
}

#[test]
fn backend_rejects_malformed_self_calls_and_ignores_unknown_deliveries() {
    let (owned, canister_id) = installed_probe();
    let pic = owned.instance();
    assert!(update_bool(pic, canister_id, Method::Start));
    let before = snapshot(pic, canister_id);
    for payload in [vec![], vec![0_u8; 7], vec![0_u8; 9]] {
        let accepted: bool = update_wire(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::FixtureDispatchPayload,
            (payload,),
        )
        .unwrap();
        assert!(!accepted);
    }
    for _ in 0..2 {
        let accepted: bool = update_wire(
            pic,
            canister_id,
            ExternalPrincipal::anonymous(),
            Method::FixtureDispatchPayload,
            (0_u64.to_le_bytes().to_vec(),),
        )
        .unwrap();
        assert!(accepted);
    }
    assert_eq!(snapshot(pic, canister_id), before);
}
