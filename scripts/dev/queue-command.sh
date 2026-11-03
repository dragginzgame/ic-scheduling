#!/usr/bin/env bash
set -euo pipefail

# Make owns compilation and server admission before these launch commands.
# Testkit owns setup/admission and the distinct borrowed/spawned server lifetimes.
[[ $# == 1 ]] || { echo 'usage: queue-command.sh demo|test-demo|test-demo-cli' >&2; exit 2; }
case "$1" in demo|test-demo|test-demo-cli) ;; *) echo 'unknown queue command' >&2; exit 2 ;; esac
cli="$(bash scripts/dev/testkit-tool.sh --check)"
target="$(bash scripts/dev/minimum-rust.sh cargo metadata --format-version 1 --no-deps --locked --offline | jq -er .target_directory)"
wasm="$target/wasm32-unknown-unknown/release/ic_jobs_test_consumer.wasm"
binary="$target/debug/examples/queue"
case "$1" in
    demo)
        server="$("$cli" check)"
        exec "$binary" --server "$server" --wasm "$wasm" ;;
    test-demo)
        export IC_JOBS_CONSUMER_WASM="$wasm"
        exec "$cli" run --ttl 900 -- bash scripts/dev/minimum-rust.sh cargo test -p ic-jobs-canister-tests \
            --example queue --features harness --locked --offline -- --ignored --nocapture --test-threads=1 ;;
    test-demo-cli)
        IC_JOBS_QUEUE_SERVER="$("$cli" check)"
        export IC_JOBS_QUEUE_SERVER IC_JOBS_CONSUMER_WASM="$wasm" IC_JOBS_QUEUE_BIN="$binary"
        exec bash scripts/dev/minimum-rust.sh cargo test -p ic-jobs-canister-tests --test queue_cli \
            --features harness --locked --offline -- --ignored --nocapture ;;
esac
