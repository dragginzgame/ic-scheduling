//! Installed retries preserve budgets and effect identity across awaits and upgrades.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;
const SECOND: u64 = 1_000_000_000;
async fn state(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> [u64; 17] {
    let bytes = pic
        .query_call(id, sender, "jobs_retry_state", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 136);

    std::array::from_fn(|i| u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap()))
}

async fn settle(pic: &PocketIc, seconds: u64) {
    pic.advance_time(Duration::from_secs(seconds)).await;

    for _ in 0..50 {
        pic.tick().await;
    }
}

async fn control(pic: &PocketIc, id: CanisterId, sender: CanisterId, command: u8) {
    pic.update_call(id, sender, "jobs_retry_control", vec![command])
        .await
        .unwrap();
}

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for mode in 0..=10 {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 100_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        pic.update_call(id, sender, "jobs_retry_setup", vec![mode])
            .await
            .unwrap();
        settle(pic, 2).await;
        let first = state(pic, id, sender).await;
        assert_eq!(first[5..9], [1, 1, 0, 1], "mode {mode} initial identity");
        assert_eq!(
            first[12..],
            [1, 0, 1, first[9], if mode == 9 { 3 } else { 4 }]
        );
        if mode == 9 {
            assert_eq!(first[0], 1, "ordinary expected failure settles");
            settle(pic, 60).await;
            assert_eq!(state(pic, id, sender).await, first);
            pic.stop_canister(id, None).await.unwrap();
            continue;
        }
        assert_eq!(first[10], 1);
        if mode != 10 {
            assert_eq!(first[0..4], [2, 1, 0, 2]);
            assert_eq!(first[4], first[9] + 10 * SECOND);
        } else {
            assert_eq!(first[0], 0);
        }
        if mode == 8 {
            assert_eq!(first[11], 2, "retry does not consume catch-up quota");
        }
        if mode == 6 {
            control(pic, id, sender, 2).await;
            settle(pic, 60).await;
            let cancelled = state(pic, id, sender).await;
            assert_eq!((cancelled[0], cancelled[5], cancelled[10]), (5, 1, 0));
            pic.stop_canister(id, None).await.unwrap();
            continue;
        }
        if mode == 2 || mode == 10 {
            if mode == 2 {
                control(pic, id, sender, 0).await;
                let paused = state(pic, id, sender).await;
                pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
                    .await
                    .unwrap();
                assert_eq!(state(pic, id, sender).await, paused);
            }
            settle(pic, 30).await;
            assert_eq!(state(pic, id, sender).await[5], 1);
            control(pic, id, sender, 1).await;
            settle(pic, 0).await;
        } else {
            if mode == 0 {
                pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
                    .await
                    .unwrap();
                assert_eq!(state(pic, id, sender).await, first);
            }
            settle(pic, 9).await;
            assert_eq!(state(pic, id, sender).await[5], 1, "no early retry");
            settle(pic, 2).await;
        }
        let second = state(pic, id, sender).await;
        if mode == 3 || mode == 4 {
            assert_eq!(second[0..4], [4, 1, 0, 2], "uncertain attempt identity");
            assert_eq!(second[5], if mode == 3 { 1 } else { 2 });
            assert!(
                pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
                    .await
                    .is_err()
            );
            control(pic, id, sender, 0).await;
            control(pic, id, sender, 1).await;
            settle(pic, 60).await;
            assert_eq!(
                state(pic, id, sender).await,
                second,
                "time and resume cannot retry uncertainty"
            );
            control(pic, id, sender, 3).await;
            assert_eq!(state(pic, id, sender).await[0], 0);
            control(pic, id, sender, 1).await;
            settle(pic, 60).await;
            assert_eq!(
                state(pic, id, sender).await[5],
                second[5],
                "recovery retires without replay"
            );
        } else if mode == 5 {
            assert_eq!(second[0..2], [2, 2]);
            assert_eq!(second[3], 1, "new definition starts with its own budget");
            assert!(second[2] > second[9]);
            assert_eq!(
                second[5..9],
                [2, 1, 0, 2],
                "old completion retains accepted identity"
            );
            assert_eq!(second[10], 0);
        } else if mode == 1 {
            assert_eq!(second[0], 1);
            assert_eq!(second[5..9], [2, 1, 0, 2]);
            settle(pic, 60).await;
            assert_eq!(state(pic, id, sender).await, second, "success ends retries");
        } else {
            assert_eq!(second[0..4], [2, 1, 0, 3]);
            assert_eq!(second[5..9], [2, 1, 0, 2]);
            assert_eq!(second[4], second[9] + 15 * SECOND, "backoff cap");
            assert_eq!(second[10], 2);
            settle(pic, 14).await;
            assert_eq!(state(pic, id, sender).await[5], 2);
            settle(pic, 2).await;
            let third = state(pic, id, sender).await;
            match mode {
                7 => {
                    assert_eq!(third[0..2], [2, 1]);
                    assert_eq!(
                        third[2],
                        third[9] + 100 * SECOND,
                        "completion delay starts at final settlement"
                    );
                    assert_eq!(third[5..9], [3, 1, 0, 3]);
                    assert_eq!(third[10], 0);
                }
                8 => {
                    assert_eq!(third[0..4], [2, 1, SECOND, 2]);
                    assert_eq!(third[5..9], [4, 1, SECOND, 1]);
                    assert_eq!(third[11], 1, "only settlement consumes batch quota");
                    settle(pic, 11).await;
                    settle(pic, 16).await;
                    let batch_done = state(pic, id, sender).await;
                    assert_eq!(batch_done[5], 6, "two occurrences, three attempts each");
                    assert_eq!(batch_done[11], 0);
                }
                _ => {
                    assert_eq!(third[0], 1);
                    assert_eq!(third[5..9], [3, 1, 0, 3]);
                    assert_eq!(third[10], 0);
                    settle(pic, 60).await;
                    assert_eq!(state(pic, id, sender).await, third, "budget cannot restart");
                }
            }
        }
        pic.stop_canister(id, None).await.unwrap();
    }
    println!(
        "jobs retry: capped budget, stable identity, pause/upgrades, owner edits, cancellation, settlement and trap recovery pass"
    );
}

pub(super) async fn recovery(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for mode in 11..=17 {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 100_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        pic.update_call(id, sender, "jobs_retry_setup", vec![mode])
            .await
            .unwrap();
        settle(pic, 2).await;
        if mode < 15 || mode == 17 {
            settle(pic, 11).await;
        }
        if mode == 13 {
            settle(pic, 16).await;
        }
        let uncertain = state(pic, id, sender).await;
        let attempt = match mode {
            13 => 3,
            15 | 16 => 1,
            _ => 2,
        };
        assert_eq!(uncertain[0..4], [4, 1, 0, attempt]);
        settle(pic, 60).await;
        assert_eq!(state(pic, id, sender).await, uncertain);
        if mode == 14 {
            control(pic, id, sender, 5).await;
        }
        if (13..=16).contains(&mode) {
            // Exhausted budget, replaced definition, deadline overflow or retries disabled.
            control(pic, id, sender, 6).await;
            let retired = state(pic, id, sender).await;
            assert_eq!((retired[0], retired[5], retired[10]), (0, uncertain[5], 0));
        } else {
            control(pic, id, sender, 4).await;
            let paused = state(pic, id, sender).await;
            assert_eq!(paused[12..15], [1, 0, 2]);
            assert_eq!(paused[16], 6);
            assert_eq!((paused[0], paused[5], paused[10]), (0, uncertain[5], 2));
            if mode != 17 {
                pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
                    .await
                    .unwrap();
                assert_eq!(
                    state(pic, id, sender).await,
                    paused,
                    "reconciled retry survives upgrade"
                );
            } else {
                assert_eq!(
                    paused[11], 2,
                    "reconciliation preserves committed batch quota"
                );
            }
            // Authorizing a retry is separate from resuming it.
            settle(pic, 30).await;
            assert_eq!(state(pic, id, sender).await, paused);
            control(pic, id, sender, 1).await;
            settle(pic, 0).await;
            let done = state(pic, id, sender).await;
            if mode == 17 {
                assert_eq!(done[0..4], [2, 1, SECOND, 2]);
                assert_eq!((done[5], done[11]), (4, 1));
                pic.stop_canister(id, None).await.unwrap();
                continue;
            }
            assert_eq!(done[0], 1);
            assert_eq!(done[5], uncertain[5] + 1);
            assert_eq!(
                done[6..9],
                [1, 0, 3],
                "reconciliation consumes the uncertain attempt's budget"
            );
            settle(pic, 60).await;
            assert_eq!(state(pic, id, sender).await, done);
        }
        pic.stop_canister(id, None).await.unwrap();
    }
    println!(
        "jobs recovery: explicit retry safety, paused upgrades, exact authority and unavailable-retry refusals pass"
    );
}
