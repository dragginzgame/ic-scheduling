//! Installed catch-up policy, pause/upgrade custody and trap isolation.
use ic_testkit::pocket_ic::{CanisterId, ErrorCode, nonblocking::PocketIc};
use std::time::Duration;
const MINUTE: u64 = 60_000_000_000;
async fn ticks(pic: &PocketIc) {
    for _ in 0..80 {
        pic.tick().await;
    }
}

async fn state(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> ([u8; 2], u64, Vec<u64>) {
    let bytes = pic
        .query_call(id, sender, "catch_up_state", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 11 + usize::from(bytes[2]) * 8);
    (
        bytes[..2].try_into().unwrap(),
        u64::from_le_bytes(bytes[3..11].try_into().unwrap()),
        bytes[11..]
            .as_chunks::<8>()
            .0
            .iter()
            .map(|value| u64::from_le_bytes(*value))
            .collect(),
    )
}

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for mode in 0..=10 {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 100_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        pic.update_call(id, sender, "catch_up_setup", vec![mode])
            .await
            .unwrap();
        let initial = state(pic, id, sender).await;
        assert_eq!(initial.0, [2, 0]);
        pic.advance_time(Duration::from_secs(if mode == 8 { 7200 } else { 600 }))
            .await;
        ticks(pic).await;
        if mode == 8 {
            for step in 0..1000 {
                pic.tick().await;
                if step % 20 == 0 && state(pic, id, sender).await.0 == [2, 64] {
                    break;
                }
            }
        }
        let observed = state(pic, id, sender).await;
        match mode {
            0 => {
                assert_eq!(observed.0, [2, 3]);
                assert_eq!(
                    observed.2,
                    [initial.1, initial.1 + MINUTE, initial.1 + 2 * MINUTE]
                );
            }
            1 => {
                assert_eq!(observed.0, [2, 2]);
                assert_eq!(observed.2, [observed.1 - 2 * MINUTE, observed.1 - MINUTE]);
            }
            2 | 7 => {
                assert_eq!(observed.0, [0, 1]);
                assert_eq!(observed.2.len(), 1);
                let first = observed.2[0];
                if mode == 2 {
                    assert_eq!(first, initial.1);
                }
                let count = if mode == 7 { 2 } else { 3 };
                pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
                    .await
                    .unwrap();
                assert_eq!(state(pic, id, sender).await, observed);
                pic.advance_time(Duration::from_secs(600)).await;
                ticks(pic).await;
                assert_eq!(state(pic, id, sender).await, observed);
                pic.update_call(id, sender, "catch_up_control", vec![0])
                    .await
                    .unwrap();
                ticks(pic).await;
                let resumed = state(pic, id, sender).await;
                assert_eq!(resumed.0, [2, count]);
                let expected: Vec<_> = (0..count)
                    .map(|index| first + u64::from(index) * MINUTE)
                    .collect();
                assert_eq!(resumed.2, expected);
                assert!(resumed.1 > pic.get_time().await.as_nanos_since_unix_epoch());
                pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
                    .await
                    .unwrap();
                assert_eq!(state(pic, id, sender).await, resumed);
            }
            3 | 4 | 9 | 10 => {
                let finished = u8::from(mode >= 9);
                let started = usize::from(finished) + usize::from(matches!(mode, 4 | 10));
                assert_eq!(observed.0, [4, finished]);
                assert_eq!(
                    observed.1,
                    initial.1 + u64::from(finished) * MINUTE,
                    "trap retains committed occurrence identity"
                );
                let expected: Vec<_> = (0..started)
                    .map(|index| initial.1 + u64::try_from(index).unwrap() * MINUTE)
                    .collect();
                assert_eq!(observed.2, expected);
                let rejection = pic
                    .upgrade_canister(id, wasm.to_vec(), vec![], None)
                    .await
                    .unwrap_err();
                assert_eq!(rejection.error_code, ErrorCode::CanisterCalledTrap);
                pic.update_call(id, sender, "catch_up_control", vec![0])
                    .await
                    .unwrap();
                pic.advance_time(Duration::from_secs(61)).await;
                ticks(pic).await;
                assert_eq!(state(pic, id, sender).await, observed);
                pic.update_call(id, sender, "catch_up_control", vec![1])
                    .await
                    .unwrap();
                let recovered = state(pic, id, sender).await;
                assert_eq!(recovered.0, [2, finished]);
                assert!(recovered.1 > pic.get_time().await.as_nanos_since_unix_epoch());
                assert_eq!(recovered.2, observed.2);
                pic.advance_time(Duration::from_secs(61)).await;
                ticks(pic).await;
                let later = state(pic, id, sender).await;
                assert_eq!(later.0, [2, finished + 1]);
                assert_eq!(later.2.last(), Some(&recovered.1));
                assert_eq!(later.2.len(), observed.2.len() + 1);
            }
            5 | 6 => {
                assert_eq!(observed.0, [2, 1]);
                assert_eq!(observed.2, [initial.1]);
                if mode == 5 {
                    assert_eq!(observed.1 % (2 * MINUTE), 0);
                }
            }
            8 => {
                assert_eq!(observed.0, [2, 64]);
                let expected: Vec<_> = (0..64).map(|index| initial.1 + index * MINUTE).collect();
                assert_eq!(observed.2, expected);
            }
            _ => unreachable!(),
        }
        let final_state = state(pic, id, sender).await;
        assert!(final_state.1 > pic.get_time().await.as_nanos_since_unix_epoch());
        ticks(pic).await;
        assert_eq!(
            state(pic, id, sender).await,
            final_state,
            "batch does not replenish its quota"
        );
        pic.stop_canister(id, None).await.unwrap();
    }
    println!(
        "jobs catch-up: full 64-count batch, paused upgrade, schedule edit and first/later trap recovery pass"
    );
}
