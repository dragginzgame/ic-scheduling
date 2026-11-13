#!/usr/bin/env bash
set -euo pipefail

# Jobs selects the inputs; the canonical checker owns formatting/preservation
# assertions. Every Git mutation occurs in its disposable copies.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-scratch.sh
source "$root/scripts/ci/fixture-scratch.sh"
fixture="$(fixture_create_workspace "$root" "jobs-formatting.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Jobs formatting input retained"' EXIT

# Only reorder two adjacent dependencies; keep names, versions and features.
perl "$root/vendor/toko-tooling/scripts/ci/prepare-cargo-formatting-fixture.pl" \
    "$root/Cargo.toml" workspace.dependencies serde_json serde_test > "$fixture/unsorted.toml"
bash "$root/vendor/toko-tooling/scripts/ci/check-formatting-hooks.sh" "$root" \
    crates/ic-jobs/src/job.rs Cargo.toml "$fixture/unsorted.toml" \
    --rust-sources \
    $(cat "$root/ci/workspace-manifests.txt") qualification/Cargo.toml qualification/Cargo.lock \
    Cargo.lock vendor/toko-tooling/make/consumer.mk vendor/toko-tooling/make/cargo-dependencies.mk vendor/toko-tooling/make/tools.mk vendor/toko-tooling/make/rust-format.mk vendor/toko-tooling/make/release.mk vendor/toko-tooling/make/execution.mk \
    vendor/toko-tooling/ci/tool-versions.env vendor/toko-tooling/scripts/ci/check-format-tools.sh vendor/toko-tooling/scripts/ci/run-formatting.sh \
    vendor/toko-tooling/scripts/ci/run-rust-formatting.pl
fixture_complete=true
