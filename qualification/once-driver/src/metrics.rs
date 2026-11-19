//! Installed event accounting at committed lifecycle boundaries.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};

pub(super) async fn read(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> [u128; 6] {
    let bytes = pic
        .query_call(id, sender, "timer_metrics", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 102);
    assert_eq!(&bytes[96..], &[0; 6]);

    std::array::from_fn(|index| {
        u128::from_le_bytes(bytes[index * 16..index * 16 + 16].try_into().unwrap())
    })
}

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let id = pic.create_canister().await;
    pic.add_cycles(id, 100_000_000_000_000).await;
    pic.install_canister(id, wasm.to_vec(), vec![], None).await;
    pic.update_call(id, sender, "metrics_setup", vec![])
        .await
        .unwrap();
    for _ in 0..40 {
        pic.tick().await;
    }
    assert_eq!(read(pic, id, sender).await, [4, 4, 0, 2, 0, 0]);
    for _ in 0..20 {
        pic.tick().await;
    }
    assert_eq!(read(pic, id, sender).await, [4, 4, 0, 2, 0, 0]);
    assert!(
        pic.query_call(id, sender, "inventory_check", vec![])
            .await
            .unwrap()
            .is_empty()
    );
    println!("metrics: ordinary outcomes, reentrant reads and initialization preservation pass");
}
