//! Actual low-cycle self-call refusal followed by funding and delivery.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let id = pic.create_canister().await;
    pic.add_cycles(id, 10_000_000_000_000).await;
    pic.install_canister(id, wasm.to_vec(), vec![], None).await;
    let setup = pic
        .update_call(id, sender, "refusal_setup", vec![])
        .await
        .unwrap();
    assert_eq!(setup.len(), 24);
    let initial = u64::from_le_bytes(setup[..8].try_into().unwrap());
    for _ in 0..10 {
        pic.tick().await;
    }
    let deferred = pic
        .update_call(id, sender, "refusal_state", vec![])
        .await
        .unwrap();
    assert_eq!(deferred.len(), 10);
    assert_eq!(&deferred[8..], &[1, 0]);
    let retry = u64::from_le_bytes(deferred[..8].try_into().unwrap());
    assert!(
        retry >= initial + 1_000_000_000,
        "timer control must have run and backed off"
    );
    pic.advance_time(Duration::from_secs(2)).await;
    for _ in 0..10 {
        pic.tick().await;
    }
    let repeated = pic
        .update_call(id, sender, "refusal_state", vec![])
        .await
        .unwrap();
    assert_eq!(repeated.len(), 10);
    assert_eq!(&repeated[8..], &[1, 0]);
    let later = u64::from_le_bytes(repeated[..8].try_into().unwrap());
    assert!(
        later >= retry + 1_000_000_000,
        "another refusal must retain a later opportunity"
    );
    pic.add_cycles(id, 10_000_000_000_000).await;
    for _ in 0..10 {
        pic.tick().await;
    }
    assert_eq!(
        pic.update_call(id, sender, "refusal_state", vec![])
            .await
            .unwrap(),
        repeated,
        "funding must not execute work before the retained retry deadline"
    );
    pic.advance_time(Duration::from_secs(2)).await;
    for _ in 0..30 {
        pic.tick().await;
    }
    let completed = pic
        .update_call(id, sender, "refusal_state", vec![])
        .await
        .unwrap();
    assert_eq!(completed.len(), 10);
    assert_eq!(&completed[..8], &[0; 8]);
    assert_eq!(&completed[8..], &[0, 1]);
    let cost = u128::from_le_bytes(setup[8..].try_into().unwrap());
    println!(
        "refusal: quoted cost {cost} cycles; repeated deferral without acceptance, then one funded completion"
    );
}

async fn watchdog_state(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> (u64, u8, u8) {
    let bytes = pic
        .query_call(id, sender, "refusal_watch_state", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 10);
    (
        u64::from_le_bytes(bytes[..8].try_into().unwrap()),
        bytes[8],
        bytes[9],
    )
}

pub(super) async fn watchdog(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let id = pic.create_canister().await;
    pic.add_cycles(id, 10_000_000_000_000).await;
    pic.install_canister(id, wasm.to_vec(), vec![], None).await;
    let setup = pic
        .update_call(id, sender, "refusal_setup", vec![1])
        .await
        .unwrap();
    assert_eq!(setup.len(), 24);
    let initial = u64::from_le_bytes(setup[..8].try_into().unwrap());
    for _ in 0..30 {
        pic.tick().await;
    }
    let first = watchdog_state(pic, id, sender).await;
    assert!(first.0 > initial);
    assert_eq!((first.1, first.2), (1, 0));
    pic.advance_time(Duration::from_millis(100)).await;
    for _ in 0..30 {
        pic.tick().await;
    }
    let later = watchdog_state(pic, id, sender).await;
    assert!(later.0 > first.0);
    assert_eq!((later.1, later.2), (1, 0));
    pic.add_cycles(id, 10_000_000_000_000).await;
    for _ in 0..100 {
        pic.tick().await;
    }
    let funded = watchdog_state(pic, id, sender).await;
    assert!(
        funded.0 > later.0,
        "cadence control continues during delivery backoff"
    );
    assert!(
        funded.0 < initial + 1_000_000_000,
        "observe before the retry floor"
    );
    assert_eq!(
        (funded.1, funded.2),
        (1, 0),
        "funding and cadence wakes must not bypass delivery backoff"
    );
    pic.advance_time(Duration::from_secs(2)).await;
    for _ in 0..30 {
        pic.tick().await;
    }
    assert_eq!(watchdog_state(pic, id, sender).await, (0, 0, 1));
    println!(
        "watchdog refusal: short cadence advances without early delivery, then one funded completion"
    );
}
