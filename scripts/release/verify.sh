#!/usr/bin/env bash
set -euo pipefail
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
cd "$root"
bash scripts/release/group.sh validate
write_receipt=false
[[ -z "${RELEASE_SOURCE:-}" ]] || write_receipt=true
export RELEASE_SOURCE="${RELEASE_SOURCE:-$(git rev-parse HEAD)}"
mkdir -p target/release-validation
attempt="$(mktemp -d "$root/target/release-validation/validation.XXXXXXXX")"
printf 'Release validation selection and evidence: %s\n' "$attempt"
perl scripts/release/validation.pl select "$attempt"
CARGO_NET_OFFLINE=true VALIDATION_LOG_DIR="$attempt/logs" \
    SCHEDULING_VALIDATION_ATTEMPT="${attempt#"$root/"}" \
    VALIDATION_FAILURE_LOG_DIR="$(git rev-parse --git-path release-state)/validation-failures" \
    bash scripts/ci/run-validation-targets.sh --plan "${attempt#"$root/"}/plan.json"
perl scripts/release/validation.pl seal "$attempt"
if [[ "$write_receipt" == true ]]; then
    state="$(git rev-parse --git-path release-state)/$RELEASE_GROUP-${RELEASE_VERSION:?}.receipts"
    [[ ! -L "$state" ]] || exit 1
    mkdir -p "$state"
    retained="$(mktemp -d "$state/validation.XXXXXXXX")"
    cp -R "$attempt/." "$retained/"
    export RELEASE_VALIDATION_EVIDENCE="${retained##*/}"
    bash scripts/release/metadata.sh validated
fi
