//! Installed checks of the product Once provider.
use ic_testkit::pocket_ic::PocketIcBuilder;
use std::time::Duration;
fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run());
}
async fn run() {
    let wasm = std::fs::read(std::env::args().nth(1).expect("probe wasm path")).unwrap();
    let server = std::env::var_os("POCKET_IC_BIN").expect("prepared PocketIC 16.0.0 server");
    let pic = PocketIcBuilder::new()
        .with_server_binary(server.into())
        .with_application_subnet()
        .build_async()
        .await;
    let external = pic.create_canister().await;
    for (mode, expected) in [
        (0, vec![0, 1, 1]),
        (1, vec![3, 0, 0]),
        (2, vec![3, 1, 0]),
        (3, vec![0, 1, 1]),
        (4, vec![0, 2, 2]),
        (5, vec![4, 1, 0]),
    ] {
        let id = pic.create_canister().await;
        pic.add_cycles(id, 10_000_000_000_000).await;
        pic.install_canister(id, wasm.clone(), vec![], None).await;
        assert_eq!(
            pic.update_call(id, external, "arm", vec![mode])
                .await
                .unwrap(),
            [1, 0]
        );
        pic.advance_time(Duration::from_secs(2)).await;
        for _ in 0..30 {
            pic.tick().await;
        }
        let state = pic.query_call(id, external, "state", vec![]).await.unwrap();
        println!("mode={mode} state={state:?}");
        assert_eq!(state, expected);
        assert!(
            pic.update_call(id, external, "__ic_scheduling_once", vec![0; 16])
                .await
                .is_err()
        );
        if mode == 5 {
            // arm initializes first; a terminal provider fault rejects there.
            assert!(pic.update_call(id, external, "arm", vec![0]).await.is_err());
            for _ in 0..20 {
                pic.tick().await;
            }
            assert_eq!(
                pic.query_call(id, external, "state", vec![]).await.unwrap(),
                [4, 1, 0]
            );
        }
        if mode == 1 || mode == 2 {
            assert_eq!(
                pic.update_call(id, external, "arm", vec![0]).await.unwrap()[0],
                0
            );
            pic.update_call(id, external, "recover", vec![])
                .await
                .unwrap();
            assert_eq!(
                pic.query_call(id, external, "state", vec![]).await.unwrap()[0],
                0
            );
            assert_eq!(
                pic.update_call(id, external, "arm", vec![0]).await.unwrap()[0],
                1
            );
            pic.advance_time(Duration::from_secs(2)).await;
            for _ in 0..30 {
                pic.tick().await;
            }
            assert_eq!(
                pic.query_call(id, external, "state", vec![]).await.unwrap()[0],
                0
            );
        }
    }
    pressure::check(&pic, &wasm, external).await;
    lifecycle::check(&pic, &wasm, external).await;
    refusal::check(&pic, &wasm, external).await;
    held_upgrade::check(&pic, &wasm, external).await;
    watchdog::check(&pic, &wasm, external).await;
    pic.drop().await;
    println!("Once installed lifecycle, authentication and pressure checks passed");
}

mod pressure;

mod lifecycle;

mod refusal;

mod held_upgrade;

mod watchdog;
