//! Paired feature comparison, not whole-async-lifetime instruction attribution.
use ic_testkit::pocket_ic::nonblocking::PocketIc;

pub(super) async fn check(pic: &PocketIc, disabled: &[u8], enabled: &[u8]) {
    let sender = pic.create_canister().await;
    println!(
        "metrics Wasm: disabled={} enabled={} delta={}",
        disabled.len(),
        enabled.len(),
        enabled.len() as i128 - disabled.len() as i128
    );
    let mut samples = [[[0_u64; 4]; 3]; 2];
    for (feature, wasm) in [disabled, enabled].into_iter().enumerate() {
        for sample in &mut samples[feature] {
            let id = pic.create_canister().await;
            pic.add_cycles(id, 100_000_000_000_000).await;
            pic.install_canister(id, wasm.to_vec(), vec![], None).await;
            pic.update_call(id, sender, "setup", vec![]).await.unwrap();
            for _ in 0..40 {
                pic.tick().await;
            }
            let bytes = pic.query_call(id, sender, "state", vec![]).await.unwrap();
            assert_eq!(bytes.len(), 137);
            assert_eq!(&bytes[..3], &[1, 1, 1]);
            assert_eq!(&bytes[131..], &[0; 6]);
            for (index, value) in sample.iter_mut().enumerate() {
                *value =
                    u64::from_le_bytes(bytes[3 + index * 8..11 + index * 8].try_into().unwrap());
                assert!(*value > 0);
            }
            let metrics: [u128; 6] = std::array::from_fn(|index| {
                u128::from_le_bytes(bytes[35 + index * 16..51 + index * 16].try_into().unwrap())
            });
            assert_eq!(
                metrics,
                if feature == 0 {
                    [0; 6]
                } else {
                    [2, 2, 0, 1, 0, 0]
                }
            );
            for _ in 0..20 {
                pic.tick().await;
            }
            assert_eq!(
                pic.query_call(id, sender, "state", vec![]).await.unwrap(),
                bytes
            );
        }
    }

    for (index, name) in [
        "setup",
        "once_factory_prefix",
        "once_resume_prefix",
        "watchdog_prefix",
    ]
    .into_iter()
    .enumerate()
    {
        let mut off = samples[0].map(|sample| sample[index]);
        off.sort_unstable();
        let mut on = samples[1].map(|sample| sample[index]);
        on.sort_unstable();
        println!(
            "metrics cost: point={name} disabled={off:?} enabled={on:?} median_delta={}",
            i128::from(on[1]) - i128::from(off[1])
        );
    }
}
