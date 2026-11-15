//! Installed prearm, trap survival, and synchronous owner precedence.
use ic_testkit::pic::fixtures::{
    RawReplyLimits, RawReplyMethod, ReplyMode, held::HeldReplyFixture,
};
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

pub(super) async fn admission(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let target = pic.create_canister().await;
    pic.add_cycles(target, 100_000_000_000_000).await;
    let mut held = HeldReplyFixture::new(
        pic,
        target,
        &[RawReplyMethod {
            name: "hold",
            mode: ReplyMode::Update,
            reply: b"ok",
        }],
        RawReplyLimits {
            methods: 1,
            reply_bytes: 2,
            method_name_bytes: 4,
        },
        50_000_000_000,
        100_000,
    )
    .await
    .unwrap();
    pic.install_canister(target, held.wasm().to_vec(), vec![], None)
        .await;
    let id = pic.create_canister().await;
    pic.add_cycles(id, 100_000_000_000_000).await;
    pic.install_canister(id, wasm.to_vec(), vec![], None).await;
    pic.update_call(id, sender, "watchdog_target", target.as_slice().to_vec())
        .await
        .unwrap();
    pic.update_call(id, sender, "watchdog_setup", vec![5])
        .await
        .unwrap();
    for _ in 0..100 {
        pic.tick().await;
        if held.pending(pic).await.unwrap().len() == 1 {
            break;
        }
    }
    let pending = held.pending(pic).await.unwrap();
    assert_eq!(pending.len(), 1);
    for _ in 0..10 {
        pic.update_call(id, sender, "watchdog_poke", vec![])
            .await
            .unwrap();
        for _ in 0..20 {
            pic.tick().await;
        }
    }
    let observed = state(pic, id, sender).await;
    assert_eq!(
        observed.1, 11,
        "watchdog callbacks continue while consumer request is live"
    );
    assert_eq!(
        observed.2, 0,
        "watchdog flight settles independently of consumer async work"
    );
    assert_eq!(
        pic.query_call(id, sender, "watchdog_requests", vec![])
            .await
            .unwrap(),
        [1, 0, 1]
    );
    assert_eq!(held.pending(pic).await.unwrap().len(), 1);
    pic.update_call(id, sender, "watchdog_mode", vec![4])
        .await
        .unwrap();
    held.release(pic, &pending[0]).await.unwrap();
    for _ in 0..30 {
        pic.tick().await;
    }
    assert_eq!(
        pic.query_call(id, sender, "watchdog_requests", vec![])
            .await
            .unwrap(),
        [1, 1, 0]
    );
    pic.update_call(id, sender, "watchdog_poke", vec![])
        .await
        .unwrap();
    for _ in 0..30 {
        pic.tick().await;
    }
    assert_eq!(state(pic, id, sender).await, (0, 12, 0));
    println!(
        "watchdog admission: 11 opportunities, one held physical request, independent settlement"
    );
}

pub(super) async fn pressure(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let target = pic.create_canister().await;
    pic.add_cycles(target, 100_000_000_000_000).await;
    let methods = vec![
        RawReplyMethod {
            name: "hold",
            mode: ReplyMode::Update,
            reply: b"ok"
        };
        8
    ];
    let mut held = HeldReplyFixture::new(
        pic,
        target,
        &methods,
        RawReplyLimits {
            methods: 8,
            reply_bytes: 16,
            method_name_bytes: 32,
        },
        50_000_000_000,
        100_000,
    )
    .await
    .unwrap();
    pic.install_canister(target, held.wasm().to_vec(), vec![], None)
        .await;
    let id = pic.create_canister().await;
    pic.add_cycles(id, 100_000_000_000_000).await;
    pic.install_canister(id, wasm.to_vec(), vec![], None).await;
    pic.update_call(id, sender, "watchdog_target", target.as_slice().to_vec())
        .await
        .unwrap();
    pic.update_call(id, sender, "watchdog_block", vec![])
        .await
        .unwrap();
    for _ in 0..100 {
        pic.tick().await;
        if held.pending(pic).await.unwrap().len() == 8 {
            break;
        }
    }
    let pending = held.pending(pic).await.unwrap();
    assert_eq!(pending.len(), 8);
    assert_eq!(
        pic.update_call(id, sender, "watchdog_setup", vec![6])
            .await
            .unwrap(),
        [0]
    );
    for _ in 0..30 {
        pic.tick().await;
    }
    let first = state(pic, id, sender).await;
    assert_eq!((first.1, first.2), (0, 1));
    for _ in 0..100 {
        pic.tick().await;
    }
    let later = state(pic, id, sender).await;
    assert!(
        later.0 >= first.0 + 30,
        "cadence must advance through multiple periods with full flights"
    );
    assert_eq!((later.1, later.2), (0, 1));
    assert_eq!(held.pending(pic).await.unwrap().len(), 8);
    // Changing callback behavior does not replace the pending ticket. Releasing
    // one flight must allow that original opportunity through the shared limit.
    pic.update_call(id, sender, "watchdog_mode", vec![4])
        .await
        .unwrap();
    held.release(pic, &pending[0]).await.unwrap();
    for _ in 0..60 {
        pic.tick().await;
    }
    assert_eq!(state(pic, id, sender).await, (0, 1, 0));
    assert_eq!(held.pending(pic).await.unwrap().len(), 7);
    for request in pending.iter().skip(1) {
        held.release(pic, request).await.unwrap();
    }
    for _ in 0..60 {
        pic.tick().await;
    }
    assert_eq!(
        pic.query_call(id, sender, "watchdog_blocker_finishes", vec![])
            .await
            .unwrap(),
        [8]
    );
    assert_eq!(state(pic, id, sender).await, (0, 1, 0));
    println!(
        "watchdog pressure: eight flights held, cadence advances, one release admits one coalesced callback"
    );
}
