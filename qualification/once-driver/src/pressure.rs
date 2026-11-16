//! Full-table admission and held-flight pressure, without advancing held-call time.
use ic_testkit::{
    held::{Catalog, Fixture, Limits, Reply},
    pocket_ic::{CanisterId, nonblocking::PocketIc},
};

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let target = pic.create_canister().await;
    pic.add_cycles(target, 100_000_000_000_000).await;
    let methods = vec![
        Reply {
            method: "hold",
            bytes: b"ok"
        };
        12
    ];
    let catalog = Catalog::new(
        &methods,
        Limits {
            slots: 12,
            reply_bytes: 24,
            name_bytes: 48,
        },
    )
    .unwrap();
    let mut held = Fixture::new(
        pic,
        target.as_slice().try_into().unwrap(),
        &catalog,
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
    pic.update_call(id, sender, "pressure_setup", target.as_slice().to_vec())
        .await
        .unwrap();
    for _ in 0..100 {
        pic.tick().await;
        if held.pending(pic).await.unwrap().len() == 8 {
            break;
        }
    }
    let pending = held.pending(pic).await.unwrap();
    assert_eq!(pending.len(), 8, "fill the provisional flight bound");
    let counts = pic
        .query_call(id, sender, "pressure_counts", vec![])
        .await
        .unwrap();
    assert_eq!(&counts[..8], &[1; 8]);
    assert!(counts[8..].iter().all(|&count| count == 0));
    pic.update_call(id, sender, "pressure_cancel", vec![])
        .await
        .unwrap();
    for _ in 0..20 {
        pic.tick().await;
    }
    assert_eq!(held.pending(pic).await.unwrap().len(), 8);
    // Reverse release makes completion order independent of dispatch order.
    for request in pending.iter().rev() {
        held.release(pic, request).await.unwrap();
    }
    for _ in 0..60 {
        pic.tick().await;
    }
    let counts = pic
        .query_call(id, sender, "pressure_counts", vec![])
        .await
        .unwrap();
    assert_eq!(&counts[..8], &[1; 8]);
    assert_eq!(&counts[64..72], &[1; 8]);
    assert!(
        counts[8..64]
            .iter()
            .chain(&counts[72..])
            .all(|&count| count == 0)
    );
    pic.update_call(id, sender, "pressure_rearm", vec![])
        .await
        .unwrap();
    for _ in 0..100 {
        pic.tick().await;
        if held.pending(pic).await.unwrap().len() == 4 {
            break;
        }
    }
    let pending = held.pending(pic).await.unwrap();
    assert_eq!(pending.len(), 4);
    for request in &pending {
        held.release(pic, request).await.unwrap();
    }
    for _ in 0..60 {
        pic.tick().await;
    }
    let counts = pic
        .query_call(id, sender, "pressure_counts", vec![])
        .await
        .unwrap();
    assert_eq!(&counts[..12], &[1; 12]);
    assert_eq!(&counts[64..76], &[1; 12]);
    assert!(
        counts[12..64]
            .iter()
            .chain(&counts[76..])
            .all(|&count| count == 0)
    );
    pic.update_call(id, sender, "pressure_interfere", vec![])
        .await
        .unwrap();
    for _ in 0..20 {
        pic.tick().await;
    }
    assert_eq!(
        pic.query_call(id, sender, "pressure_counts", vec![])
            .await
            .unwrap(),
        counts
    );
    println!(
        "pressure: 64 admitted, 65th refused, 8 held flights, 12 single local starts; foreign timer write faults visibly"
    );
}
