//! Host upgrade refusal preserves a live attempt until its real reply settles.
use ic_testkit::{
    pic::fixtures::{RawReplyLimits, RawReplyMethod, ReplyMode, held::HeldReplyFixture},
    pocket_ic::{CanisterId, ErrorCode, nonblocking::PocketIc},
};
use std::time::Duration;

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
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
    pic.install_canister(id, wasm.to_vec(), vec![1], None).await;
    pic.update_call(id, sender, "lifecycle_hold", target.as_slice().to_vec())
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
    let running = pic
        .query_call(id, sender, "lifecycle_state", vec![])
        .await
        .unwrap();
    assert_eq!(running.len(), 18);
    assert_eq!(&running[16..], &[1, 2]);
    let epoch = u64::from_le_bytes(running[..8].try_into().unwrap());
    // First accepted flight is sequence 1. Neither a duplicate nor a different
    // incarnation with that sequence may create another accepted future.
    for version in [epoch, epoch + 1] {
        let mut envelope = version.to_le_bytes().to_vec();
        envelope.extend_from_slice(&1_u64.to_le_bytes());
        assert_eq!(
            pic.update_call(id, sender, "wire_relay", envelope)
                .await
                .unwrap(),
            [1]
        );
    }
    let rejection = pic
        .upgrade_canister(id, wasm.to_vec(), vec![], None)
        .await
        .unwrap_err();
    assert_eq!(rejection.error_code, ErrorCode::CanisterCalledTrap);
    assert_eq!(
        pic.query_call(id, sender, "lifecycle_state", vec![])
            .await
            .unwrap(),
        running
    );
    assert_eq!(held.pending(pic).await.unwrap().len(), 1);
    held.release(pic, &pending[0]).await.unwrap();
    for _ in 0..30 {
        pic.tick().await;
    }
    let settled = pic
        .query_call(id, sender, "lifecycle_state", vec![])
        .await
        .unwrap();
    assert_eq!(&settled[16..], &[1, 0]);
    pic.update_call(id, sender, "lifecycle_rearm", vec![])
        .await
        .unwrap();
    let armed = pic
        .query_call(id, sender, "lifecycle_state", vec![])
        .await
        .unwrap();
    pic.upgrade_canister(id, wasm.to_vec(), vec![], None)
        .await
        .unwrap();
    let restored = pic
        .query_call(id, sender, "lifecycle_state", vec![])
        .await
        .unwrap();
    assert_ne!(&restored[..8], &armed[..8]);
    assert_eq!(&restored[8..16], &armed[8..16]);
    assert_eq!(&restored[16..], &[0, 1]);
    pic.advance_time(Duration::from_secs(11)).await;
    for _ in 0..30 {
        pic.tick().await;
    }
    assert_eq!(
        &pic.query_call(id, sender, "lifecycle_state", vec![])
            .await
            .unwrap()[16..],
        &[1, 0]
    );
    println!(
        "held upgrade: live duplicates inert, host refuses upgrade, original attempt settles, fresh demand reconstructs"
    );
}
