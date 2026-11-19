//! Completion-relative deadlines survive real awaits, owner commands and upgrades.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;
async fn state(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> [u64; 6] {
    let bytes = pic
        .query_call(id, sender, "jobs_completion_state", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 48);
    std::array::from_fn(|i| u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap()))
}

async fn ticks(pic: &PocketIc, count: usize) {
    for _ in 0..count {
        pic.tick().await;
    }
}

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for mode in 0..=10 {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 100_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        pic.update_call(id, sender, "jobs_completion_setup", vec![mode])
            .await
            .unwrap();
        pic.advance_time(Duration::from_secs(2)).await;
        if !matches!(mode, 3 | 6 | 8..=10) {
            let mut running = false;
            for _ in 0..30 {
                pic.tick().await;
                if state(pic, id, sender).await[0] == 3 {
                    running = true;
                    break;
                }
            }
            assert!(running, "mode {mode} accepted a real await");
            pic.advance_time(Duration::from_secs(20)).await;
        }
        ticks(pic, 400).await;
        let before = state(pic, id, sender).await;
        match mode {
            0 | 1 | 7 => {
                assert_eq!(before[0], if mode == 1 { 0 } else { 2 });
                assert_eq!(before[1], 1);
                assert!(before[2] > 0);
                assert_eq!(before[4], before[2]);
                if mode != 1 {
                    assert_eq!(before[3], before[2] + 10_000_000_000);
                }
                pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
                    .await
                    .unwrap();
                assert_eq!(&state(pic, id, sender).await[..5], &before[..5]);
                if mode == 1 {
                    pic.advance_time(Duration::from_secs(20)).await;
                    ticks(pic, 30).await;
                    assert_eq!(state(pic, id, sender).await[1], 1);
                    pic.update_call(id, sender, "jobs_completion_resume", vec![])
                        .await
                        .unwrap();
                } else {
                    pic.advance_time(Duration::from_secs(9)).await;
                    ticks(pic, 30).await;
                    assert_eq!(state(pic, id, sender).await[1], 1);
                    pic.advance_time(Duration::from_secs(2)).await;
                }
                ticks(pic, 80).await;
                let second = state(pic, id, sender).await;
                assert_eq!(second[1], 2);
                assert_eq!(second[3], second[2] + 10_000_000_000);
                pic.advance_time(Duration::from_secs(3600)).await;
                ticks(pic, 80).await;
                assert_eq!(state(pic, id, sender).await[1], 3, "no invented backlog");
            }
            2 => {
                assert_eq!(before[0..2], [2, 1]);
                assert_eq!(before[3], before[5]);
                assert_eq!(
                    before[4],
                    before[5] - 100_000_000_000,
                    "old completion cannot advance new definition"
                );
            }
            3 | 4 => {
                assert_eq!(before[0], 4);
                assert_eq!(before[1], u64::from(mode == 4));
                assert_eq!(before[2], 0);
                assert!(
                    pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
                        .await
                        .is_err()
                );
                pic.advance_time(Duration::from_secs(60)).await;
                ticks(pic, 40).await;
                assert_eq!(state(pic, id, sender).await, before);
                pic.update_call(id, sender, "jobs_completion_resume", vec![])
                    .await
                    .unwrap();
                ticks(pic, 30).await;
                assert_eq!(state(pic, id, sender).await[1], before[1]);
                pic.advance_time(Duration::from_secs(11)).await;
                ticks(pic, 400).await;
                assert_eq!(state(pic, id, sender).await[1], before[1] + 1);
            }
            5 => {
                assert_eq!(before[0..2], [1, 1]);
            }
            6 => {
                assert_eq!(before[0..3], [2, 0, 0]);
                assert_eq!(before[3], before[4] + 10_000_000_000);
            }
            8..=10 => {
                assert_eq!(before, [5, 0, 0, 0, 0, 0]);
                pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
                    .await
                    .unwrap();
                pic.advance_time(Duration::from_secs(3600)).await;
                ticks(pic, 80).await;
                assert_eq!(
                    state(pic, id, sender).await,
                    before,
                    "cancelled work stays cancelled across upgrade"
                );
            }
            _ => unreachable!(),
        }
        pic.stop_canister(id, None).await.unwrap();
    }
    println!(
        "jobs completion: awaits, pause/upgrade, edits, overflow, recovery and terminal cancellation pass"
    );
}
