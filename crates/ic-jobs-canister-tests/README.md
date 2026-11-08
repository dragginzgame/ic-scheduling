# Jobs artifact capture and recovery tests

The default package builds `jobs-capture-artifact` without Tokio, PocketIC,
Testkit, Wire or the qualification-consumer library. It retains exact Wasm
artifacts for the consumer-owned recovery evidence flow.

Enable `--features harness` for the native management transport and installed
recovery tests. The harness borrows its caller's async runtime and enables only
external principal conversion. Recovery tests own their Tokio runtime, await
network operations and explicitly close each instance before its server stops.
Managed server custody contains failed tests; no update retry or replay is added.

From the repository root:

```bash
# Native capture tests and recovery-harness compilation; no server is launched.
bash scripts/dev/minimum-rust.sh cargo test -p ic-jobs-canister-tests --bin jobs-capture-artifact --test recovery --features harness --locked --offline

# Explicit installed recovery qualification after preparing the selected server.
make install-testkit-tools install-consumer-server
make test-canister
```

The [qualification consumer](../../apps/ic-jobs-test-consumer/README.md) owns
storage, management, dispatch and its Timers watchdog. Its installed recovery
cases retain their explicit PocketIC gate and source-bound evidence.
