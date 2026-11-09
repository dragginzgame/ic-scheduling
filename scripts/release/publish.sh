#!/usr/bin/env bash
set -euo pipefail

# Consumer publication policy. Dependencies: Bash 3.2, Git, Make, Cargo/Clippy, jq/yq,
# curl and the local publication-admission checker.
# Effects: package verification and both published library uploads to crates.io; no Git writes.
[[ $# == 1 && "$1" =~ ^[A-Za-z0-9._-]+$ ]] || {
    echo 'usage: publish.sh REMOTE' >&2; exit 2;
}
remote="$1"
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
cd "$root"
bash "$root/scripts/release/group.sh" validate
make --no-print-directory -C "$root" shared-tooling-check
# Capture identity before admission so network observations and linting must
# authorize the same release. Source admission still precedes metadata reads.
bash "$root/scripts/ci/check-make-execution.sh"
bash "$root/scripts/ci/check-release-source.sh"
commit="$(git rev-parse --verify HEAD)"
version="$(bash "$root/scripts/release/group.sh" version)"
package="$(bash "$root/scripts/release/group.sh" package)"
tag="refs/tags/$RELEASE_GROUP-v$version"
tag_object="$(git rev-parse --verify "$tag")"
bash "$root/scripts/release/check-publication.sh" "$remote"
bash "$root/scripts/release/publication-lint.sh"
current_commit="$(git rev-parse --verify HEAD)"
current_version="$(bash "$root/scripts/release/group.sh" version)"
current_tag="$(git rev-parse --verify "$tag")"
[[ "$current_commit" == "$commit" && "$current_version" == "$version" && "$current_tag" == "$tag_object" ]] || {
    echo 'publication refused: release identity changed during admission or lint; upload has not started' >&2
    exit 1
}
bash "$root/scripts/ci/check-release-source.sh"
cargo publish -p "$package" --all-features --locked --registry crates-io
