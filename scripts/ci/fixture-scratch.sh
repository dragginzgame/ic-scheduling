#!/usr/bin/env bash
# Shared companions: scripts/ci/fixture-lifecycle.sh
# Scheduling owns workspace defaults; Shared owns allocation and cleanup.
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "${BASH_SOURCE[0]%/*}/fixture-lifecycle.sh"

fixture_create_workspace() {
    [[ $# == 2 ]] || return 2
    local root="$1" template="$2" parent
    case "$root" in ''|*$'\r'*|*$'\n'*) return 2 ;; esac
    case "$template" in ''|-*|*/*|*$'\r'*|*$'\n'*) return 2 ;; esac
    parent="${TMPDIR:-$root/target/tmp}"
    case "$parent" in *$'\r'*|*$'\n'*) echo 'scratch directory contains CR/LF' >&2; return 2 ;; esac
    [[ -n "${TMPDIR:-}" ]] || mkdir -p "$parent" || return
    fixture_create "$parent/$template"
}
