#!/usr/bin/env bash
set -euo pipefail

root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
cd "$root"
[[ "${RELEASE_BATCH_ACTIVE:-}" == 1 ]] || { echo 'candidate setup requires frozen hook context' >&2; exit 2; }
case "${RELEASE_SETUP_MODE:-}" in prepare|check) ;; *) echo 'select frozen setup prepare or check' >&2; exit 2 ;; esac
bash scripts/release/group.sh validate
# shellcheck source=scripts/release/source-lock.sh
source "$root/scripts/release/source-lock.sh"
if [[ "$RELEASE_SETUP_MODE" == prepare ]]; then
    if ! make --no-print-directory host-tools-check; then make --no-print-directory install-host-tools; fi
    bash scripts/dev/minimum-rust.sh --check
    bash scripts/dev/minimum-rust.sh cargo fetch --locked
    bash scripts/dev/minimum-rust.sh cargo fetch --manifest-path qualification/Cargo.toml --locked
    if ! make --no-print-directory tools-check; then make --no-print-directory install-tools; fi
    if ! make --no-print-directory release-tools-check; then make --no-print-directory install-release-tools; fi
    make --no-print-directory install-hooks
else
    # Never fetch or install against partially prepared working metadata.
    make --no-print-directory tools-check
    [[ "$(git config --get core.hooksPath)" == vendor/toko-tooling/.githooks ]] || { echo 'saved candidate hooks are not admitted' >&2; exit 1; }
    select_release_source_lock "$root" "${RELEASE_SOURCE:?saved group source required}"
    make --no-print-directory release-tools-check "RELEASE_METADATA_LOCKFILE=$RELEASE_METADATA_LOCKFILE"
fi
