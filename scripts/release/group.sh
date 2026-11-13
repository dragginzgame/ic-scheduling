#!/usr/bin/env bash
set -euo pipefail

# Explicit consumer selection; Shared owns policy/metadata admission.
case "${RELEASE_GROUP:-}" in jobs|timers) ;; *) echo 'select RELEASE_GROUP=jobs or timers explicitly' >&2; exit 2 ;; esac
if [[ -n "${PACKAGE:-}" && "$PACKAGE" != "ic-$RELEASE_GROUP" ]]; then
    echo 'PACKAGE must belong to the explicitly selected RELEASE_GROUP' >&2; exit 2
fi
export RELEASE_GROUP
operation="${1:?select plan, version, files or package}"
case "$operation" in
    validate) ;;
    plan|version|files) exec bash vendor/toko-tooling/scripts/ci/cargo-release-group.sh "$operation" "$RELEASE_GROUP" ;;
    package)
        plan="$(bash vendor/toko-tooling/scripts/ci/cargo-release-group.sh plan "$RELEASE_GROUP")"
        printf '%s\n' "$plan" | jq -er .primary ;;
    *) exit 2 ;;
esac
