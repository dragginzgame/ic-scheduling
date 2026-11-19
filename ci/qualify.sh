#!/usr/bin/env bash
# Repository-owned obligations; orchestration and retained logs belong to Toko.
set -euo pipefail
export PATH="$PWD/.tools/host/bin:$PWD/.tools/rust/common/bin:$PATH"
case "${1:-}" in
  rust)
    rust_scope="${2:-workspace}"
    case "$rust_scope" in
      workspace) products=(--workspace); drivers=(-p once-driver -p executor-driver); probes=(-p once-probe -p executor-probe -p calendar-probe) ;;
      jobs) products=(-p ic-jobs); drivers=(-p once-driver); probes=(-p once-probe -p calendar-probe) ;;
      *) echo 'select workspace or jobs Rust scope' >&2; exit 2 ;;
    esac
    for features in --all-features --no-default-features; do
      cargo clippy "${products[@]}" --all-targets "$features" --locked -- -D warnings
      cargo clippy "${products[@]}" --target wasm32-unknown-unknown "$features" --locked -- -D warnings
      # Jobs without calendar/execution exports no scheduler or test cases.
      # Its disabled-feature build is covered by both Clippy commands above.
      if [[ "$rust_scope:$features" != jobs:--no-default-features ]]; then
        bash vendor/toko-tooling/scripts/ci/run-nonempty-cargo-test.sh "${products[@]}" "$features" --locked
      fi
    done
    RUSTDOCFLAGS='-D warnings' cargo doc "${products[@]}" --all-features --no-deps --target wasm32-unknown-unknown --locked
    cargo clippy --manifest-path qualification/Cargo.toml "${drivers[@]}" --all-targets --locked -- -D warnings
    cargo clippy --manifest-path qualification/Cargo.toml "${probes[@]}" --target wasm32-unknown-unknown --locked -- -D warnings
    if [[ "$rust_scope" == workspace ]]; then
    for features in --all-features --no-default-features; do
      cargo clippy --manifest-path qualification/Cargo.toml -p metrics-probe --target wasm32-unknown-unknown "$features" --locked -- -D warnings
    done
    fi
    ;;
  entropy)
    stage="$(node qualification/entropy-integration/prepare.mjs "${ENTROPY_SOURCE:?Set ENTROPY_SOURCE to the pinned clean Entropy checkout}")"
    build="$PWD/qualification/target/entropy-integration-build"
    installed="$PWD/qualification/target/entropy-installed-build"
    bash vendor/toko-tooling/scripts/ci/run-nonempty-cargo-test.sh --manifest-path "$stage/Cargo.toml" --target-dir "$build" --workspace --all-features
    bash vendor/toko-tooling/scripts/ci/run-nonempty-cargo-test.sh --manifest-path "$stage/Cargo.toml" --target-dir "$build" -p ic-entropy-ic --no-default-features --locked
    for features in --all-features --no-default-features; do
      cargo clippy --manifest-path "$stage/Cargo.toml" --target-dir "$build" -p ic-entropy-ic "$features" --all-targets --locked -- -D warnings
      cargo clippy --manifest-path "$stage/Cargo.toml" --target-dir "$build" -p ic-entropy-ic "$features" --target wasm32-unknown-unknown --locked -- -D warnings
    done
    cargo build --manifest-path "$stage/qualification/Cargo.toml" --target-dir "$installed" -p entropy-probe --target wasm32-unknown-unknown --release
    ENTROPY_PROBE_WASM="$installed/wasm32-unknown-unknown/release/entropy_probe.wasm" bash vendor/toko-tooling/scripts/ci/run-nonempty-cargo-test.sh --manifest-path "$stage/qualification/Cargo.toml" --target-dir "$installed" -p entropy-probe --test installed --locked -- --nocapture
    ;;
  archives)
    # Timers' archive also qualifies its reverse consumer, Jobs. Jobs-only
    # final admission uses the already published Timers supplier.
    archive_scope="${2:-workspace}"
    case "$archive_scope" in
      workspace|timers) packages=(--workspace) ;;
      jobs) packages=(-p ic-jobs) ;;
      *) echo 'select workspace, timers or jobs archive scope' >&2; exit 2 ;;
    esac
    cargo package "${packages[@]}" --all-features --allow-dirty --locked
    metadata="$(cargo metadata --no-deps --locked --format-version 1)"
    timers="$(jq -r '.packages[] | select(.name == "ic-timers") | .version' <<< "$metadata")"
    jobs="$(jq -r '.packages[] | select(.name == "ic-jobs") | .version' <<< "$metadata")"
    # Cargo 1.100's multi-package temporary registry fails Wasm verification
    # with "no hash listed" for the unpublished supplier. Compile the extracted
    # archives together, with the sole override pointing to that exact archive.
    # Registry-only consumer verification follows the supplier's publication.
    if [[ "$archive_scope" == jobs ]]; then
      cargo check --manifest-path "target/package/ic-jobs-$jobs/Cargo.toml" --all-features --target wasm32-unknown-unknown --locked
    else
      cargo check --manifest-path "target/package/ic-timers-$timers/Cargo.toml" --all-features --target wasm32-unknown-unknown --locked
      cargo check --manifest-path "target/package/ic-jobs-$jobs/Cargo.toml" --all-features --target wasm32-unknown-unknown --config "patch.crates-io.ic-timers.path='$PWD/target/package/ic-timers-$timers'"
    fi
    ;;
  *) echo 'select rust, entropy or archives' >&2; exit 2 ;;
esac
