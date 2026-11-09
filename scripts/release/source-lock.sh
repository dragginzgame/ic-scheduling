#!/usr/bin/env bash

# Select tools from admitted source, including during partially prepared recovery.
# The caller owns its process and this helper's one private EXIT-cleaned directory.
select_release_source_lock() {
    local root="$1" source="$2" parent
    [[ "$source" =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ && "$(git cat-file -t "$source")" == commit ]] || return 1
    parent="${TMPDIR:-$root/target/tmp}"
    case "$parent" in *$'\r'*|*$'\n'*) echo 'scratch directory contains CR/LF' >&2; return 2 ;; esac
    [[ -n "${TMPDIR:-}" ]] || mkdir -p "$parent"
    release_lock_selection="$(mktemp -d "$parent/jobs-release-selection.XXXXXX")"
    trap 'rm -rf -- "$release_lock_selection"' EXIT
    git show "$source:qualification/Cargo.lock" > "$release_lock_selection/Cargo.lock" || return
    export RELEASE_METADATA_LOCKFILE="$release_lock_selection/Cargo.lock"
}
