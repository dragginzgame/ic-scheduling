//! Actual unresolved upgrades retain identity without admitting automatic replay.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;
async fn state(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> [u64; 13] {
    let bytes = pic
        .query_call(id, sender, "jobs_unresolved_state", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 104);
    std::array::from_fn(|i| u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap()))
}

async fn settle(pic: &PocketIc, seconds: u64) {
    pic.advance_time(Duration::from_secs(seconds)).await;
    for _ in 0..50 {
        pic.tick().await;
    }
}

async fn control(pic: &PocketIc, id: CanisterId, sender: CanisterId, command: u8) {
    pic.update_call(id, sender, "jobs_unresolved_control", vec![command])
        .await
        .unwrap();
}

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for mode in 0..=6 {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 100_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        pic.update_call(id, sender, "jobs_unresolved_setup", vec![mode])
            .await
            .unwrap();
        pic.advance_time(Duration::from_secs(2)).await;
        if matches!(mode, 2 | 5) {
            for _ in 0..30 {
                pic.tick().await;
                let observed = state(pic, id, sender).await;
                if observed[0] == 3 && (mode != 5 || observed[6] == 2) {
                    break;
                }
            }
        } else {
            settle(pic, 0).await;
            if mode == 3 {
                settle(pic, 11).await;
            }
        }
        let before = state(pic, id, sender).await;
        assert_eq!(before[0], if matches!(mode, 2 | 5) { 3 } else { 4 });
        assert_eq!(before[1..4], [1, 0, if mode == 3 { 2 } else { 1 }]);
        pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
            .await
            .unwrap();
        let restored = state(pic, id, sender).await;
        assert_eq!(restored[0], 4);
        assert_eq!(restored[1..5], before[1..5]);
        assert_eq!(restored[5], u64::from(before[0] == 4));
        assert_eq!(restored[6..8], before[6..8]);
        assert_eq!(
            restored[8..],
            before[8..],
            "latest outcome survives unresolved upgrade"
        );
        control(pic, id, sender, 0).await;
        settle(pic, 60).await;
        assert_eq!(
            state(pic, id, sender).await,
            restored,
            "time and resume must not replay restored work"
        );
        // A second upgrade remains blocked and now records uncertainty.
        pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
            .await
            .unwrap();
        let repeated = state(pic, id, sender).await;
        assert_eq!(repeated[0..5], restored[0..5]);
        assert_eq!(repeated[5], 1);
        control(pic, id, sender, if mode == 1 { 1 } else { 2 }).await;
        let paused = state(pic, id, sender).await;
        assert_eq!(paused[0], 0);
        assert_eq!(paused[8..11], before[1..4]);
        assert_eq!(paused[12], if matches!(mode, 1 | 4 | 5) { 5 } else { 6 });
        settle(pic, 20).await;
        assert_eq!(state(pic, id, sender).await, paused);
        control(pic, id, sender, 3).await;
        settle(pic, 0).await;
        let finished = state(pic, id, sender).await;
        if matches!(mode, 1 | 4 | 5) {
            assert_eq!(
                finished[4], before[4],
                "retirement does not replay old work"
            );
            if mode != 1 {
                assert_eq!(finished[6], 2, "owner edit survives reconstruction");
            }
        } else {
            assert_eq!(finished[4], before[4] + if mode == 6 { 2 } else { 1 });
            assert_eq!(finished[7], 0);
            assert_eq!(
                finished[12], 2,
                "normal success supersedes reconciliation observation"
            );
        }
        pic.stop_canister(id, None).await.unwrap();
    }
    println!(
        "jobs unresolved: running/factory/post-await upgrades, repeat reconstruction, retry budget, owner edits and catch-up custody pass"
    );
}
