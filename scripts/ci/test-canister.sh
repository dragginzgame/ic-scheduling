#!/usr/bin/env bash
set -euo pipefail

[[ $# == 0 || ( $# == 1 && "$1" == --optimized ) ]] || {
    echo 'usage: test-canister.sh [--optimized]' >&2; exit 2;
}
complete=false
status=0
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source scripts/ci/fixture-lifecycle.sh
trap 'status=$?; fixture_completion_status "$status" "$complete" || status=$?; exit "$status"' EXIT

# Caller explicitly selects this live gate. The canonical Cargo-tool checker and
# Testkit own executable selection and PocketIC admission/lifecycle respectively.
cli="$(bash scripts/dev/testkit-tool.sh --check)"
target="$(bash scripts/dev/minimum-rust.sh cargo metadata --manifest-path qualification/Cargo.toml --format-version 1 --no-deps --locked --offline | jq -er .target_directory)"
input="$target/wasm32-unknown-unknown/release/ic_jobs_test_consumer.wasm"
[[ -f "$input" && ! -L "$input" ]] || exit 1
capture="$target/debug/jobs-capture-artifact"
[[ -f "$capture" && ! -L "$capture" && -x "$capture" ]] || exit 1
variants=(original)
if [[ $# == 1 ]]; then
    bin="$(bash scripts/dev/install-ic-tools.sh --tool wasm-opt --check)"
    variants+=(O3 Os Oz)
fi
# Explicit CI/operator evidence roots remain overrides; local scratch is cleanable.
evidence_parent="${RUNNER_TEMP:-${TMPDIR:-$target/tmp}}"
case "$evidence_parent" in *$'\r'*|*$'\n'*) echo 'evidence directory contains CR/LF' >&2; exit 2 ;; esac
[[ -n "${RUNNER_TEMP:-${TMPDIR:-}}" ]] || mkdir -p "$evidence_parent"
evidence="$(fixture_create "$evidence_parent/jobs-recovery.XXXXXX")"
printf 'Live recovery evidence: %s\n' "$evidence"
"$capture" "$input" "$evidence/ic_jobs_test_consumer.wasm"
cp qualification/Cargo.lock "$evidence/Cargo.lock"
cp Cargo.lock "$evidence/product-Cargo.lock"
cp .toko-tooling.snapshot "$evidence/"
cp "${cli%/bin/*}/selection.json" "$evidence/testkit-selection.json"
{
    git rev-parse HEAD
    uname -sm
    bash scripts/dev/minimum-rust.sh rustc --version --verbose
    bash scripts/ci/verify-file-checksum.sh --print sha256 scripts/ci/test-canister.sh
    bash scripts/ci/verify-file-checksum.sh --print sha256 "$capture"
} > "$evidence/source.txt"
if [[ $# == 1 ]]; then
    cp .tools/ic/pins.tsv .tools/ic/files.sha256 "$evidence/"
    "$bin/wasm-opt" --version > "$evidence/optimizer.txt"
    bash scripts/ci/verify-file-checksum.sh --print sha256 "$bin/wasm-opt" >> "$evidence/optimizer.txt"
fi
for variant in "${variants[@]}"; do
    mkdir "$evidence/$variant"
    if [[ "$variant" == original ]]; then
        export IC_JOBS_CONSUMER_WASM="$evidence/ic_jobs_test_consumer.wasm"
    else
        export IC_JOBS_CONSUMER_WASM="$evidence/$variant/ic_jobs_test_consumer.wasm"
        # The Rust tool preserves the exact optimizer arguments, producer/log
        # custody and capture-before-cleanup order. It enables no extra features.
        "$capture" "$evidence/ic_jobs_test_consumer.wasm" "$IC_JOBS_CONSUMER_WASM" \
            --optimize "$bin/wasm-opt" "-$variant"
    fi
    bash scripts/ci/verify-file-checksum.sh --print sha256 "$IC_JOBS_CONSUMER_WASM" \
        > "$evidence/$variant/consumer-wasm.sha256"
    printf 'Qualifying %s: %s\n' "$variant" "$(cat "$evidence/$variant/consumer-wasm.sha256")"
    # Testkit's run command checks the prepared server bundle; it never downloads.
    "$cli" run --ttl 900 --server-stdout "$evidence/$variant/server.stdout" \
        --server-stderr "$evidence/$variant/server.stderr" \
        -- bash scripts/dev/minimum-rust.sh cargo test --manifest-path qualification/Cargo.toml -p ic-jobs-canister-tests \
        --test recovery --features harness --locked --offline -- --ignored --test-threads=1 --nocapture \
        2>&1 | tee "$evidence/$variant/test.log"
done
complete=true
