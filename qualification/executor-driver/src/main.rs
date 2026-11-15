//! Installed PocketIC checks for the selected Platform Runtime's trap contract.

use pocket_ic::PocketIcBuilder;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("usage: executor-driver PROBE.wasm");
    let wasm = std::fs::read(path).expect("read probe Wasm");
    let server = std::env::var_os("POCKET_IC_BIN")
        .expect("set POCKET_IC_BIN to the prepared PocketIC 16.0.0 server");
    let pic = PocketIcBuilder::new()
        .with_server_binary(server.into())
        .with_application_subnet()
        .build();
    let canister = pic.create_canister();
    pic.add_cycles(canister, 10_000_000_000_000);
    pic.install_canister(canister, wasm, vec![], None);
    let external = pic.create_canister();
    let sender = external;
    for (mode, expected_reply, expected_events) in [
        (0, 1, [1, 1, 1, 1, 0]),
        (1, 2, [1, 0, 0, 0, 0]),
        (2, 2, [1, 1, 0, 0, 1]),
    ] {
        let result = pic.update_call(canister, sender, "launch", vec![mode]);
        let events = pic
            .query_call(canister, sender, "events", vec![])
            .expect("events");
        println!("scenario={mode} result={result:?} events={events:?}");
        assert_eq!(result.expect("launch settles"), vec![expected_reply]);
        assert_eq!(events, expected_events);
    }
    assert!(
        pic.update_call(canister, external, "work", vec![0])
            .is_err()
    );
    assert!(pic.update_call(canister, external, "echo", vec![]).is_err());
    assert!(
        pic.update_call(canister, sender, "launch", vec![0, 0])
            .is_err()
    );
    assert_eq!(
        pic.query_call(canister, sender, "events", vec![])
            .expect("events after rejected admission"),
        vec![1, 1, 0, 0, 1]
    );
    println!("executor probe: 3 lifecycle and 3 admission checks passed");
}
