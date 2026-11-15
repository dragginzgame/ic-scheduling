//! Ordinary outcomes, impossible schedules and restored numeric boundary states.
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use std::time::Duration;

async fn state(pic: &PocketIc, id: CanisterId, sender: CanisterId) -> ([u8; 10], [u64; 3]) {
    let bytes = pic
        .query_call(id, sender, "jobs_boundaries_state", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 34);
    (
        bytes[..10].try_into().unwrap(),
        std::array::from_fn(|index| {
            u64::from_le_bytes(bytes[10 + index * 8..18 + index * 8].try_into().unwrap())
        }),
    )
}
async fn minute(pic: &PocketIc) {
    pic.advance_time(Duration::from_secs(61)).await;
    for _ in 0..40 {
        pic.tick().await;
    }
}

pub(super) async fn check(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let id = pic.create_canister().await;
    pic.add_cycles(id, 100_000_000_000_000).await;
    pic.install_canister(id, wasm.to_vec(), vec![], None).await;
    plans(pic, id, sender).await;
    pic.update_call(id, sender, "jobs_boundaries_setup", vec![])
        .await
        .unwrap();
    assert_eq!(
        state(pic, id, sender).await.0,
        [2, 2, 1, 5, 1, 0, 0, 0, 0, 0]
    );
    minute(pic).await;
    let first = state(pic, id, sender).await;
    assert_eq!(first.0, [2, 2, 1, 5, 1, 1, 1, 0, 0, 0]);
    assert!(first.1[0] > 0);
    assert_eq!(first.1[0], first.1[1]);
    assert_eq!(first.1[2], 0);
    for _ in 0..40 {
        pic.tick().await;
    }
    assert_eq!(state(pic, id, sender).await, first, "no immediate retry");
    pic.update_call(id, sender, "jobs_boundaries_edit", vec![0])
        .await
        .unwrap();
    minute(pic).await;
    let second = state(pic, id, sender).await;
    assert_eq!(second.0, [2, 2, 2, 5, 1, 2, 2, 1, 0, 0]);
    assert!(second.1[0] > first.1[0]);
    assert_eq!(second.1, [second.1[0]; 3]);
    pic.update_call(id, sender, "jobs_boundaries_edit", vec![1])
        .await
        .unwrap();
    minute(pic).await;
    let third = state(pic, id, sender).await;
    assert_eq!(third.0, [2, 2, 1, 5, 1, 3, 3, 1, 0, 0]);
    assert_eq!(
        third.1[2], second.1[2],
        "impossible edit removes queued work"
    );
    assert!(third.1[0] > second.1[0]);
    assert_eq!(third.1[0], third.1[1]);
    println!(
        "jobs boundaries: ordinary outcomes recur, impossible schedules idle, exhausted progress never invokes work"
    );
}

async fn plans(pic: &PocketIc, id: CanisterId, sender: CanisterId) {
    let bytes = pic
        .update_call(id, sender, "jobs_catch_up_plans", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 3 * 34);
    for (index, (row, expected)) in bytes
        .as_chunks::<34>()
        .0
        .iter()
        .zip([
            ([64, 1], [60_000_000_000, 3_840_000_000_000, u64::MAX]),
            ([3, 1], [300_000_000_000, 420_000_000_000, 600_000_000_000]),
            ([0, 0], [0, 0, u64::MAX]),
        ])
        .enumerate()
    {
        assert_eq!(&row[..2], &expected.0);
        let values: [u64; 4] = std::array::from_fn(|field| {
            u64::from_le_bytes(row[2 + field * 8..10 + field * 8].try_into().unwrap())
        });
        assert_eq!(&values[..3], &expected.1);
        assert!(values[3] > 0);
        println!(
            "jobs catch-up plan: case={index} count={} instructions={}",
            row[0], values[3]
        );
    }
}

fn ahead_rows(bytes: &[u8]) -> [[u64; 6]; 2] {
    assert_eq!(bytes.len(), 96);
    std::array::from_fn(|row| {
        std::array::from_fn(|column| {
            let offset = row * 48 + column * 8;
            u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
        })
    })
}

pub(super) async fn ahead(pic: &PocketIc, wasm: &[u8], sender: CanisterId) {
    let minute = 60_000_000_000;
    let id = pic.create_canister().await;
    pic.add_cycles(id, 100_000_000_000_000).await;
    pic.install_canister(id, wasm.to_vec(), vec![], None).await;
    let initial = ahead_rows(
        &pic.update_call(id, sender, "jobs_ahead_setup", vec![])
            .await
            .unwrap(),
    );
    let cursor = initial[0][0];
    let selected = cursor + minute;
    assert_eq!(initial, [[cursor, selected, 7, 0, 0, 0]; 2]);
    for revision in [8, 9] {
        let rows = ahead_rows(
            &pic.update_call(id, sender, "jobs_ahead_control", vec![])
                .await
                .unwrap(),
        );
        assert_eq!(rows, [[cursor, selected, revision, 0, 0, 0]; 2]);
    }
    // Cross nine ordinary cron boundaries without replaying anything through the cursor.
    for step in 0..=10 {
        let target = cursor - (10 - step) * minute + 1_000_000_000;
        let now = pic.get_time().await.as_nanos_since_unix_epoch();
        if target > now {
            pic.advance_time(Duration::from_nanos(target - now)).await;
        }
        for _ in 0..20 {
            pic.tick().await;
        }
        let rows = ahead_rows(
            &pic.query_call(id, sender, "jobs_ahead_state", vec![])
                .await
                .unwrap(),
        );
        assert_eq!(rows, [[cursor, selected, 9, 0, 0, 0]; 2]);
    }
    for occurrence in 0..2 {
        let scheduled = selected + occurrence * minute;
        let target = scheduled + 1_000_000_000;
        let now = pic.get_time().await.as_nanos_since_unix_epoch();
        pic.advance_time(Duration::from_nanos(target.checked_sub(now).unwrap()))
            .await;
        for _ in 0..40 {
            pic.tick().await;
        }
        let rows = ahead_rows(
            &pic.query_call(id, sender, "jobs_ahead_state", vec![])
                .await
                .unwrap(),
        );
        for row in rows {
            // Catch-up retires through its batch endpoint; SkipLate settles the occurrence.
            assert!(row[0] >= scheduled && row[0] < scheduled + minute);
            assert_eq!(
                &row[1..],
                &[scheduled + minute, 9, occurrence + 1, scheduled, 9]
            );
        }
    }
    println!(
        "jobs ahead cursor: both policies preserve future progress through pause/edit/resume, skip settled boundaries and resume once per new occurrence"
    );
}
