#!/usr/bin/env bash
set -euo pipefail

# Consumer adapter. Dependencies: Cargo, jq, yq, Git, tar, awk and Unix utilities.
operation="${1:-}"
[[ $# -eq 1 ]] || exit 2
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
bash "$root/scripts/release/group.sh" validate
bash "$root/scripts/ci/check-release-delivery.sh" \
    "$root/ci/release-delivery.txt" "${RELEASE_DELIVERY-direct}"
if [[ "$operation" == version ]]; then
    exec bash "$root/scripts/release/group.sh" version
fi
group_plan="$(RELEASE_COMMIT="${RELEASE_COMMIT:-${RELEASE_PREPARATION_SOURCE:-}}" bash "$root/scripts/release/group.sh" plan)"
primary="$(printf '%s\n' "$group_plan" | jq -er .primary)"
changelog="$(printf '%s\n' "$group_plan" | jq -er .changelog)"
release_files=()
while IFS= read -r path; do release_files+=("$path"); done < <(printf '%s\n' "$group_plan" | jq -r '.files[]')
read_version() { bash "$root/scripts/release/group.sh" version; }
# Shared owns receipt encoding/comparison; the runner owns exclusive release
# state custody. No receipt is written for ordinary version/plan observations.
# shellcheck source=scripts/ci/release-receipts.sh
source "$root/scripts/ci/release-receipts.sh"
# shellcheck source=scripts/release/source-lock.sh
source "$root/scripts/release/source-lock.sh"
validation_identity() {
    release_receipt_identity || return
    perl "$root/scripts/release/validation.pl" identity "$receipt_state"
}
[[ "${RELEASE_VERSION:-}" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]] || exit 2
receipt_state="$(git rev-parse --git-path release-state)/$RELEASE_GROUP-$RELEASE_VERSION.receipts"
[[ ! -L "$receipt_state" ]] || exit 1
admit_files() {
    local arguments=() path
    for path in "${release_files[@]}"; do arguments+=(--allow "$path"); done
    bash "$root/scripts/ci/check-release-source.sh" "${arguments[@]}"
}
# The runner binds the selected attempt before preparation. A present plan
# selects its admitted source lock without granting setup authority here.
select_release_tools() {
    local plan source
    export RELEASE_METADATA_LOCKFILE="$root/Cargo.lock"
    prepare_tools=1
    plan="$(git rev-parse --git-path "release-state/$RELEASE_GROUP-${RELEASE_VERSION:?}.plan")"
    if [[ -e "$plan" || -L "$plan" ]]; then
        prepare_tools=0
        source="${RELEASE_PREPARATION_SOURCE:?select the admitted preparation source}"
        select_release_source_lock "$root" "$source"
    fi
}
admit_editor() {
    # A failed check must stop even if it printed a plausible executable path.
    CARGO_RELEASE_METADATA_EDITOR="$("${RELEASE_MAKE:-make}" --silent --no-print-directory \
        -C "$root" release-tools-check "RELEASE_METADATA_LOCKFILE=$RELEASE_METADATA_LOCKFILE")" || return $?
    export CARGO_RELEASE_METADATA_EDITOR CARGO_RELEASE_METADATA_MAX_BYTES=1048576
}
case "$operation" in
    validated)
        mkdir -p "$receipt_state"
        [[ "${RELEASE_VALIDATION_EVIDENCE:-}" =~ ^validation\.[A-Za-z0-9]+$ ]] || exit 1
        perl "$root/scripts/release/validation.pl" check "$receipt_state/$RELEASE_VALIDATION_EVIDENCE"
        admit_files
        release_receipt_archive "$receipt_state/archive.XXXXXX" "$receipt_state/validated" "$receipt_state/prepared" "$receipt_state/validation-evidence"
        release_receipt_write "$receipt_state/validation-evidence" printf '%s\n' "$RELEASE_VALIDATION_EVIDENCE"
        release_receipt_write "$receipt_state/validated" validation_identity
        ;;
    preflight)
        current_version="$(read_version)"
        [[ "$current_version" == "${RELEASE_PREVIOUS:?}" ]] || exit 1
        admit_files
        for path in "${release_files[@]}"; do
            [[ -f "$path" && ! -L "$path" ]] || exit 1
        done
        # Refuse conflicting pending notes before a gate or preparation intent.
        awk -v version="${RELEASE_VERSION:?}" -v previous="$RELEASE_PREVIOUS" -v date="${RELEASE_DATE:?}" \
            -f scripts/ci/finalize-release-changelog.awk "$changelog" > /dev/null
        select_release_tools
        if [[ "$prepare_tools" == 0 ]]; then
            # Recovery uses the saved lock without resolving live partial metadata
            # or installing tools. Prepared/committed checks need no editor.
            admit_editor
            exit 0
        fi
        # Jobs admits source/candidate first; Shared owns locked preparation,
        # selected Make execution and unchanged tool-catalog admission.
        cache_prepare=1
        # Frozen setup already prepared this captured source. Fresh preflight
        # re-admits it offline; exact-version recovery keeps the saved-lock path.
        [[ "${RELEASE_BATCH_ACTIVE:-}" != 1 ]] || cache_prepare=0
        bash "$root/scripts/ci/cargo-release-preflight.sh" "$root" "$cache_prepare" \
            --pin "$root/ci/tool-versions.env" --pin "$root/ci/ic-tools.tsv" \
            --check tools-check --prepare install-tools --prepare tools-check \
            --prepare install-release-tools || {
            status=$?
            echo 'release dependency/tool preparation failed; selected metadata is unchanged' >&2
            echo 'prepare the graph with cargo fetch --locked and tools with make install-tools and make install-release-tools; explicit offline settings still apply' >&2
            exit "$status"
        }
        admit_editor
        ;;
    prepare)
        current_version="$(read_version)"
        [[ "$current_version" == "${RELEASE_PREVIOUS:?}" ]] || exit 1
        release_receipt_check "$receipt_state/validated" validation_identity
        select_release_tools
        admit_editor
        bash "$root/scripts/ci/cargo-release-metadata.sh" prepare "$primary"
        release_receipt_write "$receipt_state/prepared" release_receipt_payload '' "${release_files[@]}"
        ;;
    check|commit-check)
        release_receipt_check "$receipt_state/validated" validation_identity
        # Shared owns selected-tree metadata checks and retained failure custody.
        bash "$root/scripts/ci/cargo-release-metadata.sh" check "$primary" --manifest-order-grouped
        release_receipt_check "$receipt_state/prepared" release_receipt_payload "${RELEASE_COMMIT:-}" "${release_files[@]}"
        admit_files
        if [[ "$operation" == commit-check ]]; then
            [[ -z "${RELEASE_COMMIT:-}" ]] || exit 1
            git diff --quiet -- "${release_files[@]}"
        fi
        ;;
    *) echo 'usage: metadata.sh version|validated|preflight|prepare|check|commit-check' >&2; exit 2 ;;
esac
