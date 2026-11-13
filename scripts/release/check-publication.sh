#!/usr/bin/env bash
set -euo pipefail

# IC Scheduling read-only publication admission. Dependencies: Bash 3.2, Git, jq/yq,
# curl and the reviewed version/tag/registry/Make-execution helpers.
# Effects: remote-tag/registry observations; no Cargo upload or Git writes.
[[ $# == 1 && "$1" =~ ^[A-Za-z0-9._][A-Za-z0-9._-]*$ ]] || {
    echo 'usage: check-publication.sh REMOTE' >&2; exit 2;
}
remote="$1"
root="${BASH_SOURCE[0]}"
case "$root$PWD" in *$'\n'*|*$'\r'*) echo 'publication directory contains CR/LF' >&2; exit 2 ;; esac
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
bash "$root/scripts/release/group.sh" validate
case "$root" in *$'\n'*|*$'\r'*) echo 'resolved publication directory contains CR/LF' >&2; exit 2 ;; esac
physical_cwd="$(cd -P -- . && printf '%s/.' "$PWD")"
case "$physical_cwd" in *$'\n'*|*$'\r'*) echo 'resolved publication directory contains CR/LF' >&2; exit 2 ;; esac
fail() { echo "publication refused: $*" >&2; exit 1; }
bash "$root/vendor/toko-tooling/scripts/ci/check-make-execution.sh"

# Publication allows no metadata exceptions; retain the helper's full diagnostics.
bash "$root/vendor/toko-tooling/scripts/ci/check-release-source.sh" || fail 'source admission failed before registry observation or upload'
commit="$(git rev-parse --verify HEAD)" || fail 'publication requires a committed release'
version="$(bash "$root/scripts/release/group.sh" version)"
packages=("$(bash "$root/scripts/release/group.sh" package)")
bash "$root/vendor/toko-tooling/scripts/ci/check-release-tag.sh" "$commit" "$version"
tag="refs/tags/$RELEASE_GROUP-v$version"
tag_object="$(git rev-parse --verify "$tag")"
destination="$(git remote get-url --push --all "$remote" && printf '.')" || fail 'cannot inspect push destination'
destination="${destination%.}"
destination="${destination%$'\n'}"
[[ -n "$destination" && "$destination" != *$'\n'* && "$destination" != *$'\r'* ]] || fail 'select exactly one push destination'
observed="$(git ls-remote --exit-code -- "$destination" "$tag")" || fail 'cannot verify the pushed release tag'
[[ "$observed" == "$tag_object"$'\t'"$tag" ]] || fail 'remote release tag differs from the local annotated tag'

# Unknown registry state cannot authorize a new upload. Duplicate versions are
# immutable at crates.io; an interrupted upload is observed again on invocation.
for package in "${packages[@]}"; do
registry_status=0
bash "$root/vendor/toko-tooling/scripts/ci/check-crates-io-version.sh" "$package" "$version" || registry_status=$?
case "$registry_status" in
    0) fail "$package $version already exists on crates.io; reconcile any earlier upload" ;;
    1) ;;
    *) fail 'crates.io version observation is unavailable; resolve it before retrying' ;;
esac
done

# Recheck local source after network observations, before dispatching Cargo.
current_commit="$(git rev-parse --verify HEAD)"
current_tag="$(git rev-parse --verify "$tag")"
[[ "$current_commit" == "$commit" && "$current_tag" == "$tag_object" ]] || fail 'release identity changed during preflight'
bash "$root/vendor/toko-tooling/scripts/ci/check-release-source.sh" || fail 'source admission failed after publication preflight; upload has not started'
