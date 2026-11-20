//! Jobs selection, owner precedence, skip-late behavior and Once recovery mapping.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;

pub(super) async fn read_metrics(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> [u128; 7] {
    let bytes = pic
        .query_call(id, sender, "jobs_metrics", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 112);

    std::array::from_fn(|index| {
        u128::from_le_bytes(bytes[index * 16..(index + 1) * 16].try_into().unwrap())
    })
}

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
    for mode in 0..=6 {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 100_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        pic.update_call(id, sender, "jobs_setup", vec![mode])
            .await
            .unwrap();
        let (initial, selected) = state(pic, id, sender).await;
        assert_eq!(initial, [2, 0, 0]);
        assert_eq!(selected[3], 1);
        assert_eq!(read_metrics(pic, id, sender).await, [0; 7]);

        minute(pic).await;
        let (observed, context) = state(pic, id, sender).await;
        let expected_metrics = match mode {
            1 => [0; 7],
            2 => [1, 0, 0, 0, 0, 0, 0],
            5 => [0, 0, 1, 0, 0, 0, 0],
            _ => [1, 0, 0, 0, 1, 0, 0],
        };
        assert_eq!(read_metrics(pic, id, sender).await, expected_metrics);
        assert_eq!(read_metrics(pic, id, sender).await, expected_metrics);

        match mode {
            0 => {
                assert_eq!(observed, [2, 1, 1]);
                assert!(context[4] > selected[4]);
            }
            1 | 2 => {
                assert_eq!(observed, [4, u8::from(mode == 2), 0]);
                assert_eq!(&context[3..], &selected[3..]);
                let rejection = pic
                    .upgrade_canister(id, wasm.to_vec(), vec![], None)
                    .await
                    .unwrap_err();
                assert_eq!(
                    rejection.error_code,
                    ic_testkit::pocket_ic::ErrorCode::CanisterCalledTrap
                );
                assert_eq!(state(pic, id, sender).await, (observed, context));
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
            4 | 6 => {
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
        if matches!(mode, 0 | 2 | 3 | 4 | 6) {
            assert_eq!(context[1], selected[4]);
            assert!(context[2] >= context[1]);
        }
    }
    println!(
        "jobs: occurrence context, skip-late, edit/pause precedence, recovery isolation and reentrant future destruction pass"
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

pub(super) async fn lifecycle(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let id = pic.create_canister().await;
    pic.add_cycles(id, 100_000_000_000_000).await;
    pic.install_canister(id, wasm.to_vec(), vec![], None).await;
    pic.update_call(id, sender, "jobs_invalid_restore", vec![])
        .await
        .unwrap();
    pic.update_call(id, sender, "jobs_setup", vec![0])
        .await
        .unwrap();
    pic.update_call(id, sender, "jobs_control", vec![2])
        .await
        .unwrap();
    let before = state(pic, id, sender).await;
    assert_eq!(before.1[3], 2);
    pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
        .await
        .unwrap();
    assert_eq!(
        state(pic, id, sender).await,
        before,
        "preserve selected occurrence and revision"
    );
    minute(pic).await;
    let completed = state(pic, id, sender).await;
    assert_eq!(completed.0, [2, 1, 1]);
    assert_eq!(&completed.1[..2], &[2, before.1[4]]);
    pic.update_call(id, sender, "jobs_control", vec![3])
        .await
        .unwrap();
    pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
        .await
        .unwrap();
    assert_eq!(state(pic, id, sender).await.0, [0, 0, 0]);
    assert_eq!(
        super::metrics::read(pic, id, sender).await,
        [0; 6],
        "normal upgrade resets volatile diagnostics"
    );
    assert_eq!(read_metrics(pic, id, sender).await, [0; 7]);
    minute(pic).await;
    assert_eq!(state(pic, id, sender).await.0, [0, 0, 0]);
    pic.update_call(id, sender, "jobs_control", vec![0])
        .await
        .unwrap();
    let resumed = state(pic, id, sender).await;
    assert_eq!(resumed.1[3], 2);
    assert!(
        resumed.1[4] > completed.1[1],
        "settled occurrence must not replay"
    );
    minute(pic).await;
    assert_eq!(state(pic, id, sender).await.0, [2, 1, 1]);
    println!(
        "jobs lifecycle: armed/paused progress reconstructs, revisions survive, unresolved upgrades refuse"
    );
}

pub(super) async fn downtime(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for mode in [0, 5] {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 100_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        pic.update_call(id, sender, "jobs_setup", vec![mode])
            .await
            .unwrap();
        let before = state(pic, id, sender).await;
        assert_eq!(before.0, [2, 0, 0]);
        pic.stop_canister(id, None).await.unwrap();
        pic.advance_time(Duration::from_secs(3 * 24 * 60 * 60))
            .await;
        pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
            .await
            .unwrap();
        pic.start_canister(id, None).await.unwrap();
        for _ in 0..40 {
            pic.tick().await;
        }
        let after = state(pic, id, sender).await;
        assert_eq!(after.0, if mode == 0 { [2, 1, 1] } else { [2, 0, 0] });
        assert_eq!(after.1[3], before.1[3]);
        let now = pic.get_time().await.as_nanos_since_unix_epoch();
        assert!(
            after.1[4] > now,
            "successor is in the future after downtime"
        );
        assert!(after.1[4] - now <= 60_000_000_000);
        if mode == 0 {
            assert_eq!(&after.1[..2], &[before.1[3], before.1[4]]);
            assert!(after.1[2] > before.1[4] + 2 * 24 * 60 * 60 * 1_000_000_000);
            assert!(after.1[4] > after.1[2]);
        } else {
            assert_eq!(&after.1[..3], &[0; 3]);
        }
        for _ in 0..40 {
            pic.tick().await;
        }
        assert_eq!(state(pic, id, sender).await, after, "no catch-up burst");
    }
    println!(
        "jobs downtime: overdue selection executes once or skips under restored lateness policy"
    );
}
