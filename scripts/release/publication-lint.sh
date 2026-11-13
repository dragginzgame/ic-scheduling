#!/usr/bin/env bash
set -euo pipefail

# Consumer lint selection; Shared admits the journal and existing metadata/proof
# adapters admit the saved scope. Missing proof retains complete fresh lint.
[[ $# == 0 ]] || exit 2
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
cd "$root"
bash scripts/release/group.sh validate
commit="$(git rev-parse --verify HEAD)"
clean=false
if bash vendor/toko-tooling/scripts/ci/check-release-source.sh; then clean=true; fi
select_target() (
    # Observe current metadata independently of inherited release hook context.
    unset RELEASE_COMMIT
    version="$(bash scripts/release/group.sh version)" || exit
    tag="refs/tags/$RELEASE_GROUP-v$version"
    [[ "$(git cat-file -t "$tag")" == tag && "$(git rev-parse "$tag^{commit}")" == "$commit" ]] || exit 1
    plan="$(perl vendor/toko-tooling/scripts/ci/release-plan.pl \
        --plan "$(git rev-parse --git-path "release-state/$RELEASE_GROUP-$version.plan")" \
        --group "$RELEASE_GROUP" --version "$version")" || exit
    export RELEASE_SOURCE RELEASE_PREVIOUS RELEASE_VERSION RELEASE_DATE RELEASE_REMOTE RELEASE_BRANCH RELEASE_DELIVERY RELEASE_COMMIT
    RELEASE_SOURCE="$(printf '%s\n' "$plan" | jq -er .source)" || exit
    RELEASE_PREVIOUS="$(printf '%s\n' "$plan" | jq -er .previous)" || exit
    RELEASE_VERSION="$version"
    RELEASE_DATE="$(printf '%s\n' "$plan" | jq -er .date)" || exit
    RELEASE_REMOTE="$(printf '%s\n' "$plan" | jq -er .remote)" || exit
    RELEASE_BRANCH="$(printf '%s\n' "$plan" | jq -er .branch)" || exit
    RELEASE_DELIVERY="$(printf '%s\n' "$plan" | jq -er .delivery)" || exit
    RELEASE_COMMIT="$commit"
    git merge-base --is-ancestor "$RELEASE_SOURCE" "$commit" || exit
    bash scripts/release/metadata.sh publication-lint
)
target=clippy
if [[ "$clean" == true ]] && selected="$(select_target)"; then
    case "$selected" in clippy|clippy-jobs) target="$selected" ;; *) echo 'invalid publication lint target' >&2; exit 1 ;; esac
else
    echo 'Publication scope unavailable; retaining complete fresh lint.' >&2
fi
"${RELEASE_MAKE:-make}" --no-print-directory -C "$root" "$target"
if [[ "$clean" == true ]]; then
    [[ "$(git rev-parse --verify HEAD)" == "$commit" ]] || { echo 'publication source changed during lint' >&2; exit 1; }
    bash vendor/toko-tooling/scripts/ci/check-release-source.sh
fi
