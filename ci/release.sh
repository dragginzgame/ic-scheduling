#!/usr/bin/env bash
# Consumer metadata and evidence hooks for Toko's standard release runner.
set -euo pipefail
export PATH="$PWD/.tools/host/bin:$PWD/.tools/rust/common/bin:$PATH"
group="${RELEASE_GROUP:-}"
case "$group" in timers|jobs) ;; *) echo 'Select RELEASE_GROUP=timers or jobs' >&2; exit 2 ;; esac
shared="$PWD/vendor/toko-tooling/scripts/ci"
# shellcheck source=vendor/toko-tooling/scripts/ci/release-receipts.sh
source "$shared/release-receipts.sh"
editor() {
  bash ci/metadata-editor.sh "$@"
}
receipt() {
  printf '%s/scheduling-%s-%s.validated' "$(git rev-parse --absolute-git-dir)/release-state" "$group" "${RELEASE_SOURCE:?}"
}
check_receipt() {
  release_receipt_check "$(receipt)" qualification_identity || {
    echo 'Missing or mismatched source-bound qualification' >&2; exit 1;
  }
}
qualification_identity() {
  printf '%s\n' scheduling-qualification-v2
  release_receipt_identity
  bash ci/rust-stage.sh check
}
case "${1:-}" in
  setup)
    if [[ "${RELEASE_SETUP_MODE:-check}" == prepare ]]; then
      cargo fetch --locked
      cargo fetch --manifest-path qualification/Cargo.toml --locked
      make install-tools
      editor > /dev/null
    fi
    make tools-check
    editor --check > /dev/null
    test -x "${POCKET_IC_BIN:?Set POCKET_IC_BIN to the prepared 16.0.0 server}"
    # Entropy source admission belongs to qualify-entropy.
    ;;
  version|files) bash "$shared/cargo-release-group.sh" "$1" "$group" ;;
  preflight)
    bash "$shared/cargo-release-group.sh" check-graph "$group"
    test "$(git rev-parse HEAD)" = "${RELEASE_SOURCE:?}"
    test -z "$(git status --porcelain)"
    awk -v version="${RELEASE_VERSION:?}" -v previous="${RELEASE_PREVIOUS:?}" -v date="${RELEASE_DATE:?}" -f "$shared/finalize-release-changelog.awk" "releases/$group/CHANGELOG.md" > /dev/null
    ;;
  verify)
    test "$(git rev-parse HEAD)" = "${RELEASE_SOURCE:?}"
    test -z "$(git status --porcelain)"
    mkdir -p "$(dirname "$(receipt)")"
    # Retain a previous attempt, then invalidate it before running the gate.
    # A failed retry cannot leave an older success admitted for preparation.
    release_receipt_archive "$(receipt).previous.XXXXXX" "$(receipt)"
    if [[ -f "$(receipt)" ]]; then rm -- "$(receipt)"; fi
    make qualify-release
    # Qualification must not authorize bytes that changed while it ran.
    test "$(git rev-parse HEAD)" = "$RELEASE_SOURCE"
    test -z "$(git status --porcelain)"
    release_receipt_write "$(receipt)" qualification_identity
    ;;
  prepare)
    check_receipt
    CARGO_RELEASE_METADATA_EDITOR="$(editor --check)"
    export CARGO_RELEASE_METADATA_EDITOR CARGO_RELEASE_METADATA_MAX_BYTES=8388608
    bash "$shared/cargo-release-group.sh" prepare "$group"
    ;;
  prepared)
    check_receipt
    bash "$shared/cargo-release-group.sh" check "$group"
    bash ci/qualify.sh archives "$group"
    ;;
  index)
    check_receipt
    bash "$shared/cargo-release-group.sh" check "$group"
    git diff --quiet
    test -z "$(git ls-files --others --exclude-standard)"
    # Staging must contain every changed metadata file and no other source.
    expected="$(mktemp)"
    actual="$(mktemp)"
    bash "$shared/cargo-release-group.sh" files "$group" | tr '\0' '\n' | LC_ALL=C sort > "$expected"
    git diff --cached --name-only | LC_ALL=C sort > "$actual"
    test -s "$actual"
    test -z "$(comm -13 "$expected" "$actual")"
    rm "$expected" "$actual"
    ;;
  committed)
    check_receipt
    test "$(git rev-parse "${RELEASE_COMMIT:?}^")" = "$RELEASE_SOURCE"
    bash "$shared/cargo-release-group.sh" check "$group"
    test -z "$(git status --porcelain)"
    ;;
  publication)
    test -z "$(git status --porcelain)"
    make tools-check check
    bash "$shared/cargo-release-group.sh" check-graph "$group"
    # Published files must be exact prepared source, including its annotated tag.
    version="$(bash "$shared/cargo-release-group.sh" version "$group")"
    bash "$shared/check-release-tag.sh" "$(git rev-parse HEAD)" "$version" "$group"
    if [[ "$group" == jobs ]]; then
      # Timers must already exist in the registry. Prepare the exact standalone
      # Jobs archive and locked registry sources, then verify Wasm offline.
      # This deliberately does not use the unpublished-workspace override.
      cargo package -p ic-jobs --all-features --locked --no-verify
      # Extraction must be outside this Cargo workspace; otherwise Cargo treats
      # the normalized archive manifest as an undeclared workspace member.
      stage="$(mktemp -d "${TMPDIR:-/tmp}/ic-jobs-registry.XXXXXX")"
      printf 'Registry archive verification retained at: %s\n' "$stage"
      tar -xzf "target/package/ic-jobs-$version.crate" -C "$stage"
      manifest="$stage/ic-jobs-$version/Cargo.toml"
      cargo fetch --manifest-path "$manifest" --locked
      cargo check --manifest-path "$manifest" --all-features --target wasm32-unknown-unknown --locked --offline
    fi
    ;;
  *) echo 'Unknown release hook' >&2; exit 2 ;;
esac
