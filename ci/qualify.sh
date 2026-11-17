#!/usr/bin/env bash
# Repository-owned obligations; orchestration and retained logs belong to Toko.
set -euo pipefail
export PATH="$PWD/.tools/host/bin:$PWD/.tools/rust/common/bin:$PATH"
case "${1:-}" in
  rust)
    for features in --all-features --no-default-features; do
      cargo clippy --workspace --all-targets "$features" --locked --offline -- -D warnings
      cargo clippy --workspace --target wasm32-unknown-unknown "$features" --locked --offline -- -D warnings
      bash vendor/toko-tooling/scripts/ci/run-nonempty-cargo-test.sh --workspace "$features" --locked --offline
    done
    RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps --target wasm32-unknown-unknown --locked --offline
    cargo clippy --manifest-path qualification/Cargo.toml -p once-driver -p executor-driver --all-targets --locked --offline -- -D warnings
    cargo clippy --manifest-path qualification/Cargo.toml -p once-probe -p executor-probe -p calendar-probe --target wasm32-unknown-unknown --locked --offline -- -D warnings
    for features in --all-features --no-default-features; do
      cargo clippy --manifest-path qualification/Cargo.toml -p metrics-probe --target wasm32-unknown-unknown "$features" --locked --offline -- -D warnings
    done
    ;;
  entropy)
    stage="$(node qualification/entropy-integration/prepare.mjs "${ENTROPY_SOURCE:?Set ENTROPY_SOURCE to the pinned clean Entropy checkout}")"
    build="$PWD/qualification/target/entropy-integration-build"
    installed="$PWD/qualification/target/entropy-installed-build"
    bash vendor/toko-tooling/scripts/ci/run-nonempty-cargo-test.sh --manifest-path "$stage/Cargo.toml" --target-dir "$build" --workspace --all-features --offline
    bash vendor/toko-tooling/scripts/ci/run-nonempty-cargo-test.sh --manifest-path "$stage/Cargo.toml" --target-dir "$build" -p ic-entropy-ic --no-default-features --locked --offline
    for features in --all-features --no-default-features; do
      cargo clippy --manifest-path "$stage/Cargo.toml" --target-dir "$build" -p ic-entropy-ic "$features" --all-targets --locked --offline -- -D warnings
      cargo clippy --manifest-path "$stage/Cargo.toml" --target-dir "$build" -p ic-entropy-ic "$features" --target wasm32-unknown-unknown --locked --offline -- -D warnings
    done
    cargo build --manifest-path "$stage/qualification/Cargo.toml" --target-dir "$installed" -p entropy-probe --target wasm32-unknown-unknown --release --offline
    ENTROPY_PROBE_WASM="$installed/wasm32-unknown-unknown/release/entropy_probe.wasm" bash vendor/toko-tooling/scripts/ci/run-nonempty-cargo-test.sh --manifest-path "$stage/qualification/Cargo.toml" --target-dir "$installed" -p entropy-probe --test installed --locked --offline -- --nocapture
    ;;
  archives)
    cargo package --workspace --all-features --allow-dirty --locked --offline
    metadata="$(cargo metadata --no-deps --locked --offline --format-version 1)"
    timers="$(jq -r '.packages[] | select(.name == "ic-timers") | .version' <<< "$metadata")"
    jobs="$(jq -r '.packages[] | select(.name == "ic-jobs") | .version' <<< "$metadata")"
    # Cargo 1.100's multi-package temporary registry fails Wasm verification
    # with "no hash listed" for the unpublished supplier. Compile the extracted
    # archives together, with the sole override pointing to that exact archive.
    # Registry-only consumer verification follows the supplier's publication.
    cargo check --manifest-path "target/package/ic-timers-$timers/Cargo.toml" --all-features --target wasm32-unknown-unknown --locked --offline
    cargo check --manifest-path "target/package/ic-jobs-$jobs/Cargo.toml" --all-features --target wasm32-unknown-unknown --offline --config "patch.crates-io.ic-timers.path='$PWD/target/package/ic-timers-$timers'"
    ;;
  *) echo 'select rust, entropy or archives' >&2; exit 2 ;;
esac
