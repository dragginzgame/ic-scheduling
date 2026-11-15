//! Jobs selection, owner precedence, skip-late behavior and Once recovery mapping.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;
async fn state(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> ([u8; 3], [u64; 5]) {
    let bytes = pic
        .query_call(id, sender, "jobs_state", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 43);
    (
        bytes[..3].try_into().unwrap(),
        std::array::from_fn(|index| {
            u64::from_le_bytes(bytes[3 + index * 8..11 + index * 8].try_into().unwrap())
        }),
    )
}
async fn minute(pic: &PocketIc) {
    pic.advance_time(Duration::from_secs(61)).await;
    for _ in 0..40 {
        pic.tick().await;
    }
}
pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for mode in 0..=5 {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 100_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        pic.update_call(id, sender, "jobs_setup", vec![mode])
            .await
            .unwrap();
        let (initial, selected) = state(pic, id, sender).await;
        assert_eq!(initial, [2, 0, 0]);
        assert_eq!(selected[3], 1);
        minute(pic).await;
        let (observed, context) = state(pic, id, sender).await;
        match mode {
            0 => {
                assert_eq!(observed, [2, 1, 1]);
                assert!(context[4] > selected[4]);
            }
            1 | 2 => {
                assert_eq!(observed, [4, u8::from(mode == 2), 0]);
                assert_eq!(&context[3..], &selected[3..]);
                pic.update_call(id, sender, "jobs_control", vec![0])
                    .await
                    .unwrap();
                minute(pic).await;
                assert_eq!(
                    state(pic, id, sender).await,
                    (observed, context),
                    "pause/resume cannot replay uncertain work"
                );
                pic.update_call(id, sender, "jobs_control", vec![1])
                    .await
                    .unwrap();
                let (_, next) = state(pic, id, sender).await;
                assert!(next[4] > selected[4]);
                minute(pic).await;
                assert_eq!(state(pic, id, sender).await.0, [2, observed[1] + 1, 1]);
            }
            3 => {
                assert_eq!(observed, [2, 1, 1]);
                assert_eq!(context[0], 1, "running work keeps old revision");
                assert_eq!(context[3], 2, "new schedule controls its successor");
                assert_eq!(context[4] % 120_000_000_000, 0);
            }
            4 => {
                assert_eq!(observed, [0, 1, 1]);
                minute(pic).await;
                assert_eq!(state(pic, id, sender).await.0, observed);
            }
            5 => {
                assert_eq!(observed, [2, 0, 0]);
                assert!(
                    context[4] > selected[4],
                    "late occurrence skips directly to a future one"
                );
            }
            _ => unreachable!(),
        }
        if matches!(mode, 0 | 2 | 3 | 4) {
            assert_eq!(context[1], selected[4]);
            assert!(context[2] >= context[1]);
        }
    }
    println!(
        "jobs: occurrence context, skip-late, edit/pause precedence, initial/post-await recovery pass"
    );
}

pub(super) async fn calendar(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let id = pic.create_canister().await;
    pic.add_cycles(id, 100_000_000_000_000).await;
    pic.install_canister(id, wasm.to_vec(), vec![], None).await;
    let bytes = pic
        .update_call(id, sender, "jobs_calendar_costs", vec![])
        .await
        .unwrap();
    let expected = [
        60_000_000_000,
        4_233_718_800_000_000_000,
        1_780_218_000_000_000_000,
        0,
        0,
    ];
    assert_eq!(bytes.len(), expected.len() * 24);
    for (index, row) in bytes.as_chunks::<24>().0.iter().enumerate() {
        let values: [u64; 3] = std::array::from_fn(|field| {
            u64::from_le_bytes(row[field * 8..field * 8 + 8].try_into().unwrap())
        });
        assert_eq!(values[2], expected[index]);
        assert!(values[0] > 0 && values[1] > 0);
        println!(
            "jobs_calendar scenario={index} parse={} search={} next={}",
            values[0], values[1], values[2]
        );
    }
}
