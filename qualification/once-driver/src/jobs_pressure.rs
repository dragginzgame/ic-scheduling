//! Jobs share transport capacity without sharing occurrence or recovery authority.
use ic_testkit::{
    pic::fixtures::{RawReplyLimits, RawReplyMethod, ReplyMode, held::HeldReplyFixture},
    pocket_ic::{CanisterId, nonblocking::PocketIc},
};
use std::time::Duration;

async fn state(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> [[u8; 19]; 12] {
    let bytes = pic
        .query_call(id, sender, "jobs_pressure_state", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 12 * 19);
    std::array::from_fn(|index| bytes[index * 19..(index + 1) * 19].try_into().unwrap())
}

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let target = pic.create_canister().await;
    pic.add_cycles(target, 100_000_000_000_000).await;
    let methods = vec![
        RawReplyMethod {
            name: "hold",
            mode: ReplyMode::Update,
            reply: b"ok",
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
    pic.update_call(
        id,
        sender,
        "jobs_pressure_setup",
        target.as_slice().to_vec(),
    )
    .await
    .unwrap();
    // Advance only before opening HTTP barriers; ticks suffice while calls are held.
    pic.advance_time(Duration::from_secs(61)).await;
    for _ in 0..100 {
        pic.tick().await;
        if held.pending(pic).await.unwrap().len() == 8 {
            break;
        }
    }
    let pending = held.pending(pic).await.unwrap();
    assert_eq!(pending.len(), 8);
    let blocked = state(pic, id, sender).await;
    for row in &blocked[..8] {
        assert_eq!(&row[..3], &[3, 1, 0]);
        assert_eq!(&row[3..11], &blocked[0][3..11]);
    }
    for row in &blocked[8..] {
        assert_eq!(&row[..3], &[2, 0, 0]);
    }
    pic.update_call(id, sender, "jobs_pressure_control", vec![0])
        .await
        .unwrap();
    for _ in 0..20 {
        pic.tick().await;
    }
    assert_eq!(held.pending(pic).await.unwrap().len(), 8);
    let paused = state(pic, id, sender).await;
    assert_eq!(&paused[..8], &blocked[..8], "no overlapping starts");
    assert_eq!(&paused[8][..3], &[0, 0, 0]);
    for request in pending.iter().rev() {
        held.release(pic, request).await.unwrap();
    }
    for _ in 0..80 {
        pic.tick().await;
    }
    assert!(held.pending(pic).await.unwrap().is_empty());
    let settled = state(pic, id, sender).await;
    assert_eq!(&settled[0][..3], &[4, 1, 0]);
    assert_eq!(&settled[8][..3], &[0, 0, 0]);
    for index in (1..8).chain(9..12) {
        assert_eq!(&settled[index][..3], &[2, 1, 1]);
        assert_eq!(&settled[index][3..11], &blocked[0][3..11]);
        let scheduled = u64::from_le_bytes(settled[index][3..11].try_into().unwrap());
        let started = u64::from_le_bytes(settled[index][11..19].try_into().unwrap());
        assert!(started >= scheduled);
    }
    pic.update_call(id, sender, "jobs_pressure_control", vec![1])
        .await
        .unwrap();
    pic.advance_time(Duration::from_secs(61)).await;
    for _ in 0..80 {
        pic.tick().await;
    }
    let next = state(pic, id, sender).await;
    assert_eq!(next[0], settled[0], "uncertain occurrence never replays");
    assert_eq!(&next[8][..3], &[2, 1, 1]);
    for index in (1..8).chain(9..12) {
        assert_eq!(&next[index][..3], &[2, 2, 2]);
        let old = u64::from_le_bytes(settled[index][3..11].try_into().unwrap());
        let new = u64::from_le_bytes(next[index][3..11].try_into().unwrap());
        assert!(new > old);
    }
    println!(
        "jobs pressure: shared 64-declaration limit, eight held flights, queued pause, isolated recovery and healthy progress pass"
    );
}
