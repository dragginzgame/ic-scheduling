//! Actual upgrades and authenticated negative dispatch cases.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;

fn envelope(epoch: u64, sequence: u64) -> Vec<u8> {
    let mut bytes = epoch.to_le_bytes().to_vec();
    bytes.extend_from_slice(&sequence.to_le_bytes());
    bytes
}
fn epoch(state: &[u8]) -> u64 {
    u64::from_le_bytes(state[..8].try_into().unwrap())
}

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let id = pic.create_canister().await;
    pic.add_cycles(id, 10_000_000_000_000).await;
    pic.install_canister(id, wasm.to_vec(), vec![1], None).await;
    let before = pic
        .query_call(id, sender, "lifecycle_state", vec![])
        .await
        .unwrap();
    assert_eq!(before[16], 0);
    // The caller is genuinely self through the relay, so these reach decoding.
    for bytes in [vec![], vec![0; 15], vec![0; 17]] {
        assert_eq!(
            pic.update_call(id, sender, "wire_relay", bytes)
                .await
                .unwrap(),
            [0]
        );
    }
    for bytes in [
        envelope(epoch(&before), 0),
        envelope(epoch(&before), u64::MAX),
        envelope(epoch(&before) + 1, 1),
    ] {
        assert_eq!(
            pic.update_call(id, sender, "wire_relay", bytes)
                .await
                .unwrap(),
            [1]
        );
    }
    assert_eq!(
        pic.query_call(id, sender, "lifecycle_state", vec![])
            .await
            .unwrap(),
        before
    );
    pic.advance_time(Duration::from_secs(2)).await;
    pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
        .await
        .unwrap();
    let after = pic
        .query_call(id, sender, "lifecycle_state", vec![])
        .await
        .unwrap();
    assert_ne!(epoch(&before), epoch(&after));
    assert_eq!(
        &after[8..],
        &before[8..],
        "retain absolute demand without a run"
    );
    assert_eq!(
        pic.update_call(id, sender, "wire_relay", envelope(epoch(&before), 1))
            .await
            .unwrap(),
        [1]
    );
    for _ in 0..10 {
        pic.tick().await;
    }
    assert_eq!(
        pic.query_call(id, sender, "lifecycle_state", vec![])
            .await
            .unwrap()[16],
        0
    );
    pic.advance_time(Duration::from_secs(11)).await;
    for _ in 0..30 {
        pic.tick().await;
    }
    assert_eq!(
        pic.query_call(id, sender, "lifecycle_state", vec![])
            .await
            .unwrap()[16],
        1
    );
    // Replayed settled flight, current epoch: a no-op, never another callback.
    assert_eq!(
        pic.update_call(id, sender, "wire_relay", envelope(epoch(&after), 1))
            .await
            .unwrap(),
        [1]
    );
    for _ in 0..10 {
        pic.tick().await;
    }
    assert_eq!(
        pic.query_call(id, sender, "lifecycle_state", vec![])
            .await
            .unwrap()[16],
        1
    );
    println!("lifecycle: malformed/stale envelopes inert; future deadline rebinds across upgrade");
}
