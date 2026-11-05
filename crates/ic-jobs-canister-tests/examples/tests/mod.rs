use super::*;

#[test]
fn durations_and_commands_reject_invalid_or_unbounded_input() {
    assert!(matches!(
        Prompt::try_parse_from(["logs", "--before", "8"])
            .unwrap()
            .command,
        QueueCommand::Logs { before: Some(8) }
    ));
    assert!(Prompt::try_parse_from(["logs", "--before", "-1"]).is_err());
    assert!(matches!(
        Prompt::try_parse_from(["metrics"]).unwrap().command,
        QueueCommand::Metrics
    ));
    assert!(matches!(
        Prompt::try_parse_from(["reset-metrics"]).unwrap().command,
        QueueCommand::ResetMetrics
    ));
    for (input, seconds) in [
        ("5s", 5),
        ("2m", 120),
        ("3h", 10800),
        ("4d", 345600),
        ("2w", 1209600),
    ] {
        assert_eq!(duration(input).unwrap(), Duration::from_secs(seconds));
        assert_eq!(span(seconds * 1_000_000_000), input);
    }
    assert_eq!(span(0), "0s");
    assert_eq!(span(1), "1s");
    assert_eq!(
        span((604800 + 86400 + 3600 + 60 + 1) * 1_000_000_000),
        "1w 1d 1h 1m 1s"
    );
    for input in ["", "5", "1.5m", "-1s", "1m30s", "999999999999999999w"] {
        assert!(duration(input).is_err());
    }
    let QueueCommand::Add {
        name,
        after,
        every,
        catch_up,
    } = Prompt::try_parse_from("add report --after 0s --every 2w --catch-up".split_whitespace())
        .unwrap()
        .command
    else {
        panic!("expected typed add command");
    };
    assert_eq!(name, "report");
    assert_eq!(after, Duration::ZERO);
    assert_eq!(every, Some(Duration::from_secs(1_209_600)));
    assert!(catch_up);
    assert!(matches!(
        Prompt::try_parse_from(["exit"]).unwrap().command,
        QueueCommand::Quit
    ));
    for input in [
        "add x --after 1s --every 0s",
        "add x --after 1s --catch-up",
        "watch 0s",
        "pause nope",
    ] {
        assert!(Prompt::try_parse_from(input.split_whitespace()).is_err());
    }
}

#[tokio::test]
#[ignore = "explicit local canister demo: make test-demo"]
async fn canister_session_queues_seconds_minutes_weeks_and_manages_recurrence() {
    let wasm = std::env::var_os("IC_JOBS_CONSUMER_WASM").expect("make test-demo selects the Wasm");
    let config = PocketIcStartupConfig::from_env(Duration::from_secs(30)).unwrap();
    let mut demo = Demo::new(Path::new(&wasm), config).await.unwrap();
    let run = async |demo: &mut Demo, line: &str| {
        let args = Prompt::try_parse_from(line.split_whitespace()).unwrap();
        assert!(demo.execute(&args.command).await.unwrap());
    };
    run(&mut demo, "sample").await;
    assert!(
        demo.views()
            .await
            .unwrap()
            .iter()
            .all(|view| matches!(view.status, Status::Pending { .. }))
    );
    // Two watch steps exercise carrying the previous display's views.
    run(&mut demo, "watch 2s").await;
    run(&mut demo, "advance 3s").await;
    assert_eq!(demo.views().await.unwrap()[0].status, Status::Completed);
    assert!(matches!(
        demo.views().await.unwrap()[1].status,
        Status::Pending { .. }
    ));
    run(&mut demo, "advance 55s").await;
    assert_eq!(demo.views().await.unwrap()[1].status, Status::Completed);
    run(&mut demo, "advance 1w").await;
    assert_eq!(demo.views().await.unwrap()[2].sequence, 1);
    run(&mut demo, "pause 3").await;
    run(&mut demo, "advance 2w").await;
    assert_eq!(demo.views().await.unwrap()[2].status, Status::Paused);
    assert_eq!(demo.views().await.unwrap()[2].sequence, 1);
    run(&mut demo, "resume 3").await;
    run(&mut demo, "advance 1s").await;
    assert_eq!(demo.views().await.unwrap()[2].sequence, 2);
    run(&mut demo, "cancel 3").await;
    run(&mut demo, "advance 1w").await;
    assert_eq!(demo.views().await.unwrap()[2].status, Status::Cancelled);
    let summary: Summary = demo.query(Method::Summary, &()).await.unwrap();
    assert_eq!(summary.counter, 4);
    assert!(!summary.armed);
    demo.pic
        .close()
        .await
        .expect("delete demo instance before its borrowed server stops");
}
