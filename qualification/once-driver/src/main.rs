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
    let mut args = std::env::args().skip(1);
    let first = args.next().expect("probe wasm path or --metrics-cost");
    let paired = first == "--metrics-cost";
    let wasm = std::fs::read(if paired {
        args.next().expect("disabled probe path")
    } else {
        first
    })
    .unwrap();
    let enabled = paired.then(|| std::fs::read(args.next().expect("enabled probe path")).unwrap());
    let focused = args.next();
    assert!(focused.as_deref().is_none_or(|arg| matches!(
        arg,
        "--after-completion"
            | "--jobs-initial"
            | "--jobs-one-shot"
            | "--jobs-completion"
            | "--jobs-retry"
            | "--jobs-unresolved"
            | "--lifetimes"
            | "--diagnostics"
    )));
    let server = std::env::var_os("POCKET_IC_BIN").expect("prepared PocketIC 16.0.0 server");
    let pic = PocketIcBuilder::new()
        .with_server_binary(server.into())
        .with_application_subnet()
        .build_async()
        .await;
    if let Some(enabled) = enabled {
        metrics_cost::check(&pic, &wasm, &enabled).await;
        pic.drop().await;
        return;
    }
    let external = pic.create_canister().await;
    if focused.is_some() {
        if focused.as_deref() == Some("--diagnostics") {
            metrics::check(&pic, &wasm, external).await;
        } else if focused.as_deref() == Some("--lifetimes") {
            lifetimes::check(&pic, &wasm, external).await;
        } else if focused.as_deref() == Some("--jobs-unresolved") {
            jobs_unresolved::check(&pic, &wasm, external).await;
        } else if focused.as_deref() == Some("--jobs-retry") {
            jobs_retry::check(&pic, &wasm, external).await;
            jobs_retry::recovery(&pic, &wasm, external).await;
        } else if focused.as_deref() == Some("--jobs-completion") {
            jobs_completion::check(&pic, &wasm, external).await;
        } else if focused.as_deref() == Some("--jobs-one-shot") {
            jobs_initial::check(&pic, &wasm, external, true).await;
        } else if focused.as_deref() == Some("--jobs-initial") {
            jobs_initial::check(&pic, &wasm, external, false).await;
            jobs::lifecycle(&pic, &wasm, external).await;
            jobs::downtime(&pic, &wasm, external).await;
            jobs_catch_up::check(&pic, &wasm, external).await;
        } else {
            after_completion::check(&pic, &wasm, external).await;
        }
        pic.drop().await;
        return;
    }
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
        let metrics_expected = match mode {
            0 | 3 => [1, 1, 0, 0, 0, 0],
            1 | 2 => [1, 0, 1, 0, 0, 0],
            4 => [2, 2, 0, 0, 0, 0],
            5 => [1, 0, 0, 0, 0, 1],
            _ => unreachable!(),
        };
        assert_eq!(
            metrics::read(&pic, id, external).await,
            metrics_expected,
            "Once metrics mode {mode}"
        );
        assert!(
            pic.update_call(id, external, "__ic_scheduling_once", vec![0; 16])
                .await
                .is_err()
        );
        assert_eq!(
            metrics::read(&pic, id, external).await,
            metrics_expected,
            "external rejection is not acceptance"
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
    after_completion::check(&pic, &wasm, external).await;
    metrics::check(&pic, &wasm, external).await;
    lifecycle::check(&pic, &wasm, external).await;
    refusal::check(&pic, &wasm, external).await;
    refusal::watchdog(&pic, &wasm, external).await;
    held_upgrade::check(&pic, &wasm, external).await;
    watchdog::check(&pic, &wasm, external).await;
    watchdog::admission(&pic, &wasm, external).await;
    watchdog::pressure(&pic, &wasm, external).await;
    costs::check(&pic, &wasm, external).await;
    retention::check(&pic, &wasm, external).await;
    dispatch_costs::check(&pic, &wasm, external).await;
    jobs::calendar(&pic, &wasm, external).await;
    jobs::check(&pic, &wasm, external).await;
    jobs::lifecycle(&pic, &wasm, external).await;
    jobs::downtime(&pic, &wasm, external).await;
    jobs_pressure::check(&pic, &wasm, external).await;
    jobs_boundaries::check(&pic, &wasm, external).await;
    jobs_initial::check(&pic, &wasm, external, false).await;
    jobs_initial::check(&pic, &wasm, external, true).await;
    lifetimes::check(&pic, &wasm, external).await;
    jobs_unresolved::check(&pic, &wasm, external).await;
    jobs_retry::check(&pic, &wasm, external).await;
    jobs_retry::recovery(&pic, &wasm, external).await;
    jobs_completion::check(&pic, &wasm, external).await;
    jobs_boundaries::ahead(&pic, &wasm, external).await;
    jobs_catch_up::check(&pic, &wasm, external).await;
    pic.drop().await;
    println!("Once installed lifecycle, authentication and pressure checks passed");
}

mod after_completion;
mod pressure;

mod metrics;

mod metrics_cost;

mod lifecycle;

mod refusal;

mod held_upgrade;

mod watchdog;

mod costs;
mod retention;

mod dispatch_costs;

mod jobs;

mod jobs_pressure;

mod jobs_boundaries;
mod jobs_completion;
mod jobs_initial;
mod jobs_retry;
mod jobs_unresolved;
mod lifetimes;

mod jobs_catch_up;
