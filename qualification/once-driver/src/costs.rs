//! Repeated installed instruction samples for isolated synchronous control calls.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};

const LABELS: [&str; 14] = [
    "counter_pair",
    "initialize",
    "bind_once_first",
    "bind_watchdog_second",
    "bind_last",
    "once_arm",
    "once_replace",
    "once_equal",
    "once_inactive",
    "watchdog_start",
    "watchdog_keep",
    "watchdog_replace",
    "watchdog_inactive",
    "module_memory_pages",
];
pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for count in [3, 64] {
        let mut samples = Vec::new();
        for _ in 0..3 {
            let id = pic.create_canister().await;
            pic.add_cycles(id, 100_000_000_000_000).await;
            pic.install_canister(id, wasm.to_vec(), vec![], None).await;
            let bytes = pic
                .update_call(id, sender, "control_costs", vec![count])
                .await
                .unwrap();
            assert_eq!(bytes.len(), LABELS.len() * 8);
            let values: Vec<_> = bytes
                .as_chunks::<8>()
                .0
                .iter()
                .map(|chunk| u64::from_le_bytes(*chunk))
                .collect();
            assert!(values.iter().all(|&value| value > 0));
            samples.push(values);
        }
        for (index, label) in LABELS.iter().enumerate() {
            let minimum = samples.iter().map(|sample| sample[index]).min().unwrap();
            let maximum = samples.iter().map(|sample| sample[index]).max().unwrap();
            println!(
                "control_cost declarations={count} operation={label} min={minimum} max={maximum} samples=3"
            );
        }
    }
}
