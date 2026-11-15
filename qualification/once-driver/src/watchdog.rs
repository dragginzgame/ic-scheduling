//! Installed prearm, trap survival, and synchronous owner precedence.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;

async fn state(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> (u64, u8, u8) {
    let bytes = pic
        .query_call(id, sender, "watchdog_state", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 10);
    (
        u64::from_le_bytes(bytes[..8].try_into().unwrap()),
        bytes[8],
        bytes[9],
    )
}
pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for mode in 0..=3 {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 100_000_000_000_000).await;
        pic.install_canister(id, wasm.to_vec(), vec![], None).await;
        assert_eq!(
            pic.update_call(id, sender, "watchdog_setup", vec![mode])
                .await
                .unwrap(),
            [0]
        );
        for _ in 0..30 {
            pic.tick().await;
        }
        let first = state(pic, id, sender).await;
        assert_eq!(first.2, 0, "transport must settle the synchronous ticket");
        if mode == 2 {
            assert_eq!(first, (0, 1, 0));
            pic.advance_time(Duration::from_secs(2)).await;
            for _ in 0..30 {
                pic.tick().await;
            }
            assert_eq!(state(pic, id, sender).await, first);
        } else if mode == 3 {
            assert_ne!(first.0, 0, "owner At defeats callback Stop");
            assert_eq!(first.1, 1);
        } else {
            assert_ne!(first.0, 0, "successor survives work trap or Continue");
            assert_eq!(first.1, u8::from(mode == 0));
            pic.advance_time(Duration::from_secs(2)).await;
            for _ in 0..30 {
                pic.tick().await;
            }
            let second = state(pic, id, sender).await;
            assert!(second.0 > first.0, "next control turn remains live");
            assert_eq!(second.1, if mode == 0 { 2 } else { 0 });
            assert_eq!(second.2, 0);
            pic.update_call(id, sender, "watchdog_mode", vec![4])
                .await
                .unwrap();
            pic.advance_time(Duration::from_secs(2)).await;
            for _ in 0..30 {
                pic.tick().await;
            }
            assert_eq!(state(pic, id, sender).await, (0, second.1 + 1, 0));
        }
    }
    println!("watchdog: committed successor survives traps, cadence continues, owner commands win");
}
