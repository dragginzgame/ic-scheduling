#!/usr/bin/env bash
set -euo pipefail
export PATH="$PWD/.tools/host/bin:$PWD/.tools/rust/common/bin:$PATH"
case "${1:-}" in
  --workspace) scope=workspace ;;
  '') scope="$(bash ci/validation-scope.sh)" ;;
  *) echo 'usage: qualify-release.sh [--workspace]' >&2; exit 2 ;;
esac
mkdir -p target/release-validation
groups="$(mktemp target/release-validation/groups.XXXXXX)"
trap 'rm -f -- "$groups"' EXIT
jq -cS '[.groups[].name]' ci/cargo-release-groups.json > "$groups"
roster="$(perl vendor/toko-tooling/scripts/ci/validation-evidence.pl roster "$PWD" ci/release-validation.json "$groups")"
targets=()
while IFS= read -r target; do
  if [[ "$scope" == jobs ]]; then
    case "$target" in check-formatting-hooks|check-executor|check-metrics|qualify-entropy) continue ;; esac
  fi
  targets+=("$target")
done <<< "$roster"
case "$scope" in
  jobs)
    echo 'Jobs implementation: native/Wasm Jobs, calendar, complete shared Once integration and Jobs archives.'
    echo 'Unchanged independent executor, Timers metrics-cost and Entropy journeys are not applicable.'
    ;;
  workspace)
    echo 'Full workspace validation; the Rust stage admits only exact final-candidate evidence.'
    ;;
  *) echo 'invalid validation scope' >&2; exit 2 ;;
esac
QUALIFICATION_SCOPE="$scope" VALIDATION_REPOSITORY_ROOT="$PWD" bash vendor/toko-tooling/scripts/ci/run-validation-targets.sh --fail-fast "${targets[@]}"
