//! Completion-relative recurrence uses the installed Once provider.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for mode in [6, 7, 8, 1, 2] {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 10_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        pic.update_call(id, sender, "arm", vec![mode])
            .await
            .unwrap();
        pic.advance_time(Duration::from_secs(2)).await;
        if mode == 6 {
            let mut running = false;
            let mut observed = Vec::new();
            for _ in 0..30 {
                pic.tick().await;
                observed = pic.query_call(id, sender, "state", vec![]).await.unwrap();
                if observed == [2, 1, 0] {
                    running = true;
                    break;
                }
            }
            assert!(
                running,
                "observe the callback awaiting self-calls: {observed:?}"
            );
            pic.advance_time(Duration::from_secs(20)).await;
        }
        for _ in 0..400 {
            pic.tick().await;
        }
        let expected = match mode {
            6 => [1, 1, 1],
            7 | 8 => [0, 1, 1],
            1 => [3, 0, 0],
            2 => [3, 1, 0],
            _ => unreachable!(),
        };
        assert_eq!(
            pic.query_call(id, sender, "state", vec![]).await.unwrap(),
            expected
        );
        if mode == 6 {
            let times = pic
                .query_call(id, sender, "completion_state", vec![])
                .await
                .unwrap();
            let completed = u64::from_le_bytes(times[..8].try_into().unwrap());
            let deadline = u64::from_le_bytes(times[8..].try_into().unwrap());
            assert_eq!(deadline, completed + 10_000_000_000);
            pic.advance_time(Duration::from_secs(9)).await;
            for _ in 0..20 {
                pic.tick().await;
            }
            assert_eq!(
                pic.query_call(id, sender, "state", vec![]).await.unwrap(),
                [1, 1, 1]
            );
            pic.advance_time(Duration::from_secs(2)).await;
            for _ in 0..30 {
                pic.tick().await;
            }
            assert_eq!(
                pic.query_call(id, sender, "state", vec![]).await.unwrap(),
                [0, 2, 2]
            );
        } else {
            pic.advance_time(Duration::from_secs(60)).await;
            for _ in 0..30 {
                pic.tick().await;
            }
            assert_eq!(
                pic.query_call(id, sender, "state", vec![]).await.unwrap(),
                expected
            );
        }
    }
    println!(
        "after-completion: delayed await anchors successor at completion; stop, owner override, overflow and trap barriers pass"
    );
}
