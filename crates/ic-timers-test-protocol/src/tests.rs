use super::rpc::{Method, REQUEST_LIMITS, RESPONSE_LIMITS, WorkDecision};
use super::{CohortObservation, CounterObservation, ProbeMemorySummary, ProbeSnapshot};
use ic_wire::native::{decode_request, decode_response, encode_request, encode_response};

#[test]
fn next_work_decisions_use_one_typed_method_boundary() {
    let id = Method::SetWorkDecision as u64;
    for decision in [
        WorkDecision::ContinueImmediately,
        WorkDecision::ScheduleAt(u64::MAX),
        WorkDecision::Stop,
    ] {
        let frame = encode_request(id, &(decision,), REQUEST_LIMITS).unwrap();
        assert_eq!(
            decode_request::<(WorkDecision,)>(id, &frame, REQUEST_LIMITS),
            Ok((decision,))
        );
    }
    let invalid = encode_request(id, &(3_u8,), REQUEST_LIMITS).unwrap();
    assert!(decode_request::<(WorkDecision,)>(id, &invalid, REQUEST_LIMITS).is_err());
}

#[test]
fn borrowed_cohort_label_decodes_into_owned_text_with_wide_totals() {
    let observed = CohortObservation {
        cohort: "watchdog",
        scheduler_instruction_total: u128::from(u64::MAX) + 1,
        work_instruction_total: u128::MAX,
        ..CohortObservation::default()
    };
    let encoded = encode_response(Method::Observe as u64, &observed, RESPONSE_LIMITS)
        .expect("encode borrowed cohort observation");
    let decoded = decode_response::<CohortObservation<String>>(
        Method::Observe as u64,
        &encoded,
        RESPONSE_LIMITS,
    )
    .expect("decode owned cohort observation");
    assert_eq!(decoded.cohort, "watchdog");
    assert_eq!(
        decoded.scheduler_instruction_total,
        u128::from(u64::MAX) + 1
    );
    assert_eq!(decoded.work_instruction_total, u128::MAX);
}

#[test]
fn snapshot_round_trip_preserves_wide_totals_and_optional_memory() {
    let observed = ProbeSnapshot {
        registration_id: Some((7, 11, 13)),
        registered: true,
        schedule_requests: CounterObservation {
            total: u128::from(u64::MAX) + 1,
            total_overflowed: false,
        },
        work_started: CounterObservation {
            total: u128::MAX,
            total_overflowed: false,
        },
        work_completed: CounterObservation {
            total: u128::MAX,
            total_overflowed: true,
        },
        next_deadline_ns: Some(u64::MAX),
        scheduler_instruction_total: u128::from(u64::MAX) + 1,
        work_instruction_total: u128::MAX,
        scheduler_memory: Some(ProbeMemorySummary {
            samples: 1,
            latest_wasm_start_pages: 2,
            latest_wasm_end_pages: 3,
            latest_stable_start_pages: 5,
            latest_stable_end_pages: 8,
            maximum_wasm_growth_pages: 1,
            maximum_stable_growth_pages: 3,
        }),
        work_memory: Some(ProbeMemorySummary {
            samples: 2,
            latest_wasm_start_pages: 3,
            latest_wasm_end_pages: 5,
            latest_stable_start_pages: 8,
            latest_stable_end_pages: 13,
            maximum_wasm_growth_pages: 2,
            maximum_stable_growth_pages: 5,
        }),
        ..ProbeSnapshot::default()
    };
    let encoded = encode_response(Method::Snapshot as u64, &observed, RESPONSE_LIMITS)
        .expect("encode wide probe snapshot");
    assert_eq!(
        decode_response::<ProbeSnapshot>(Method::Snapshot as u64, &encoded, RESPONSE_LIMITS)
            .unwrap(),
        observed
    );
}

#[test]
fn checkpoint_round_trip_preserves_flags_and_wide_values() {
    // Current checkpoints retain both flags and wide values through one format.
    let previous = (true, false, u64::MAX, 7_u64);
    let state = super::RuntimeCheckpoint {
        desired_scheduled: true,
        secondary_desired_scheduled: false,
        completed_work: u64::MAX,
        secondary_completed_work: 7,
    };
    let checkpoint = native_checkpoint(&state);
    assert_eq!(checkpoint, native_checkpoint(&previous));
    assert_eq!(
        ic_wire::native::decode_request::<super::RuntimeCheckpoint>(
            super::rpc::CHECKPOINT_ID,
            &native_checkpoint(&previous),
            REQUEST_LIMITS,
        )
        .unwrap(),
        state
    );
}

fn native_checkpoint(state: &impl serde::Serialize) -> Vec<u8> {
    ic_wire::native::encode_request(super::rpc::CHECKPOINT_ID, state, REQUEST_LIMITS).unwrap()
}

#[test]
fn ordinary_termination_round_trip_uses_the_current_method_identity() {
    use super::{OrdinaryObservation, OrdinaryTermination};
    for termination in [
        None,
        Some(OrdinaryTermination::Abandoned),
        Some(OrdinaryTermination::Stopped),
        Some(OrdinaryTermination::Cancelled),
        Some(OrdinaryTermination::InvariantFailure),
    ] {
        let observed = OrdinaryObservation {
            termination,
            ..OrdinaryObservation::default()
        };
        let encoded = encode_response(
            Method::OrdinaryObservation as u64,
            &observed,
            RESPONSE_LIMITS,
        )
        .expect("encode enum termination");
        let decoded: OrdinaryObservation = decode_response(
            Method::OrdinaryObservation as u64,
            &encoded,
            RESPONSE_LIMITS,
        )
        .expect("decode enum termination");
        assert_eq!(decoded.termination, termination);
        // A retained caller of the four-boolean contract cannot interpret this layout.
        assert!(decode_response::<OrdinaryObservation>(22, &encoded, RESPONSE_LIMITS).is_err());
    }
}
