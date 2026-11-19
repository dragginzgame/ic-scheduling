//! Transient removal and replacement safety in installed IC messages.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;
async fn state(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> [u64; 4] {
    let bytes = pic
        .query_call(id, sender, "lifetime_state", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 32);

    std::array::from_fn(|i| u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap()))
}

async fn settle(pic: &PocketIc, seconds: u64) {
    pic.advance_time(Duration::from_secs(seconds)).await;

    for _ in 0..60 {
        pic.tick().await;
    }
}

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for mode in 0..=7 {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 100_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        pic.update_call(id, sender, "lifetime_setup", vec![mode])
            .await
            .unwrap();
        settle(pic, 2).await;
        let first = state(pic, id, sender).await;
        match mode {
            0 | 4 => assert_eq!(first, [5, 1, 1, 0]),
            1 | 2 => {
                assert_eq!(first, [4, u64::from(mode == 2), 0, 0]);
                settle(pic, 60).await;
                assert_eq!(state(pic, id, sender).await, first);
                pic.update_call(id, sender, "lifetime_recover", vec![])
                    .await
                    .unwrap();
                assert_eq!(state(pic, id, sender).await, [5, first[1], 1, 0]);
            }
            3 => {
                assert_eq!(first, [2, 1, 0, 0]);
                settle(pic, 11).await;
                assert_eq!(state(pic, id, sender).await, [5, 2, 1, 0]);
            }
            5 | 6 => {
                assert_eq!(
                    first,
                    [5, 0, 1, 2],
                    "removed delivery cannot invoke victim or replacement"
                );
                settle(pic, 11).await;
                assert_eq!(state(pic, id, sender).await, [5, 1, 1, 5]);
            }
            7 => {
                assert!(
                    matches!(first[0], 4 | 6),
                    "cleanup trap must retain uncertainty or fault closed"
                );
                assert_eq!(first[1..3], [1, 0]);
                settle(pic, 60).await;
                assert_eq!(
                    state(pic, id, sender).await,
                    first,
                    "cleanup trap must not replay work"
                );
            }
            _ => unreachable!(),
        }
        pic.stop_canister(id, None).await.unwrap();
    }
    println!(
        "timer lifetimes: transient settlement, trapped custody, reentrant capture release and stale queued replacement pass"
    );
}
