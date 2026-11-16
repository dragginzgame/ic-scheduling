//! Installed dispatch/resume prefix samples; no claim of full-message cost.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
const LABELS: [&str; 4] = [
    "factory_entry",
    "first_poll",
    "resume_after_echo",
    "empty_echo_entry",
];

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    for count in [1_u8, 8, 64] {
        let mut samples: [Vec<u64>; 4] = std::array::from_fn(|_| Vec::new());
        for _ in 0..3 {
            let id = pic.create_canister().await;
            pic.add_cycles(id, 100_000_000_000_000).await;
            pic.install_canister(id, wasm.to_vec(), vec![], None).await;
            baseline(pic, id, sender, count, "cold").await;
            assert_eq!(
                pic.update_call(id, sender, "dispatch_costs_setup", vec![count])
                    .await
                    .unwrap(),
                [0]
            );
            let mut state = Vec::new();
            for _ in 0..200 {
                pic.tick().await;
                state = pic
                    .query_call(id, sender, "dispatch_costs_state", vec![])
                    .await
                    .unwrap();
                assert_eq!(state.len(), 2 + usize::from(count) * 32);
                if state[..2] == [count, count] {
                    break;
                }
            }
            assert_eq!(
                &state[..2],
                &[count, count],
                "every admitted callback must finish and stop"
            );
            for row in state[2..].as_chunks::<32>().0 {
                let values: [u64; 4] = std::array::from_fn(|index| {
                    u64::from_le_bytes(row[index * 8..index * 8 + 8].try_into().unwrap())
                });
                assert!(values.iter().all(|&value| value > 0));
                assert!(
                    values[1] >= values[0],
                    "first poll follows its factory in the same message"
                );
                for (index, value) in values.into_iter().enumerate() {
                    samples[index].push(value);
                }
            }
            for _ in 0..10 {
                pic.tick().await;
            }
            assert_eq!(
                pic.query_call(id, sender, "dispatch_costs_state", vec![])
                    .await
                    .unwrap(),
                state
            );
            baseline(pic, id, sender, count, "warm").await;
        }
        for (label, values) in LABELS.iter().zip(samples) {
            println!(
                "dispatch_prefix declarations={count} point={label} min={} max={} samples={}",
                values.iter().min().unwrap(),
                values.iter().max().unwrap(),
                values.len()
            );
        }
    }
}

async fn baseline(pic: &PocketIc, id: CanisterId, sender: CanisterId, count: u8, phase: &str) {
    let baseline = pic
        .update_call(id, sender, "executor_costs_baseline", vec![])
        .await
        .unwrap();
    assert_eq!(baseline.len(), 24);
    let baseline: [u64; 3] = std::array::from_fn(|index| {
        u64::from_le_bytes(baseline[index * 8..index * 8 + 8].try_into().unwrap())
    });
    println!(
        "executor_baseline declarations={count} phase={phase} raw={} before_task={} root_poll={}",
        baseline[0], baseline[1], baseline[2]
    );
}
