#!/usr/bin/env bash
set -euo pipefail
# Scheduling selects common obligations; Shared owns dispatch and checkpoints.
stages=()
selected="$(perl scripts/release/validation.pl ci-stages)"
while IFS= read -r stage; do stages+=("$stage"); done <<< "$selected"
VALIDATION_REPOSITORY_ROOT="$PWD" exec bash vendor/toko-tooling/scripts/ci/run-validation-targets.sh --fail-fast "$@" "${stages[@]}"
