//! Fixed-rate admission is anchored to its definition, not bind time.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId, one_shot: bool) {
    for mode in 0..=if one_shot { 7 } else { 5 } {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 100_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        let first = pic
            .update_call(
                id,
                sender,
                "jobs_initial_setup",
                vec![mode, u8::from(one_shot)],
            )
            .await
            .unwrap();
        let first = u64::from_le_bytes(first.try_into().unwrap());
        pic.advance_time(Duration::from_secs(2)).await;
        for _ in 0..80 {
            pic.tick().await;
        }
        let expected = match mode {
            0 | 5 | 7 => vec![0],
            1 | 3 if one_shot => vec![first],
            1 => vec![0, 10_000_000_000, 20_000_000_000],
            3 => vec![first, first + 10_000_000_000],
            _ => vec![],
        };
        let bytes = pic
            .query_call(id, sender, "jobs_initial_state", vec![])
            .await
            .unwrap();
        let expected: Vec<_> = expected.into_iter().flat_map(u64::to_le_bytes).collect();
        assert_eq!(bytes, expected, "mode {mode}");
        for _ in 0..30 {
            pic.tick().await;
        }
        assert_eq!(
            pic.query_call(id, sender, "jobs_initial_state", vec![])
                .await
                .unwrap(),
            expected
        );
        if mode == 4 {
            pic.advance_time(Duration::from_secs(10)).await;
            for _ in 0..40 {
                pic.tick().await;
            }
            assert_eq!(
                pic.query_call(id, sender, "jobs_initial_state", vec![])
                    .await
                    .unwrap(),
                first.to_le_bytes()
            );
        }
        if one_shot {
            pic.advance_time(Duration::from_secs(3600)).await;
            for _ in 0..40 {
                pic.tick().await;
            }
            let expected = if mode == 4 {
                first.to_le_bytes().to_vec()
            } else {
                expected
            };
            assert_eq!(
                pic.query_call(id, sender, "jobs_initial_state", vec![])
                    .await
                    .unwrap(),
                expected
            );
            if matches!(mode, 6 | 7) {
                pic.update_call(id, sender, "jobs_initial_recover", vec![])
                    .await
                    .unwrap();
                pic.advance_time(Duration::from_secs(3600)).await;
                for _ in 0..40 {
                    pic.tick().await;
                }
                assert_eq!(
                    pic.query_call(id, sender, "jobs_initial_state", vec![])
                        .await
                        .unwrap(),
                    expected
                );
            }
        }
        pic.stop_canister(id, None).await.unwrap();
    }
    println!(
        "jobs initial: one_shot={one_shot}; zero, overdue and future deadlines, catch-up, skip-late and unstarted reconstruction pass"
    );
}
