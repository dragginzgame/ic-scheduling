//! Full-table retained callback custody and repeated settled execution.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};

fn decode(bytes: &[u8]) -> [u64; 6] {
    assert_eq!(bytes.len(), 48);
    std::array::from_fn(|index| {
        u64::from_le_bytes(bytes[index * 8..index * 8 + 8].try_into().unwrap())
    })
}
pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let id = pic.create_canister().await;
    pic.add_cycles(id, 100_000_000_000_000).await;
    pic.install_canister(id, wasm.to_vec(), vec![], None).await;
    let initial = decode(
        &pic.update_call(id, sender, "retention_setup", vec![])
            .await
            .unwrap(),
    );
    assert_eq!(&initial[..5], &[64, 0, 0, 0, 0]);
    let mut pages = vec![initial[5]];
    for batch in 1..=10 {
        let state = decode(
            &pic.update_call(id, sender, "retention_churn", vec![])
                .await
                .unwrap(),
        );
        assert_eq!(&state[..5], &[64, batch * 48, 0, 0, 0]);
        pages.push(state[5]);
    }
    for batch in 1..=3 {
        pic.update_call(id, sender, "retention_run", vec![])
            .await
            .unwrap();
        for _ in 0..120 {
            pic.tick().await;
        }
        let state = decode(
            &pic.query_call(id, sender, "retention_state", vec![])
                .await
                .unwrap(),
        );
        assert_eq!(&state[..5], &[64, 480, 0, batch * 32, batch * 64]);
        pages.push(state[5]);
    }
    for _ in 0..30 {
        pic.tick().await;
    }
    let final_state = decode(
        &pic.query_call(id, sender, "retention_state", vec![])
            .await
            .unwrap(),
    );
    assert_eq!(&final_state[..5], &[64, 480, 0, 96, 192]);
    println!(
        "retention: captures=64 payload_bytes=262144 rejected_drops=480 future_drops=96 callbacks=192 pages={pages:?}"
    );
}
