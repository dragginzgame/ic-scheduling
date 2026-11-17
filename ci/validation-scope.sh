#!/usr/bin/env bash
# Product selection only; baseline observation belongs to Toko Tooling.
set -euo pipefail
case "${RELEASE_GROUP:-}" in
  timers) echo workspace; exit 0 ;;
  jobs) ;;
  *) echo 'select timers or jobs' >&2; exit 2 ;;
esac
baseline="$(perl vendor/toko-tooling/scripts/ci/validation-evidence.pl baseline "$PWD" \
  "${RELEASE_SOURCE:?}" "refs/tags/jobs-v${RELEASE_PREVIOUS:?}")"
# Only Jobs Rust implementation/tests and its ledger have a narrow mapping.
# Manifests, locks, generators, qualification, tooling and unknown paths broaden.
if jq -e '.available and (.changes | length > 0) and
  all(.changes[]; (startswith("crates/ic-jobs/src/") and endswith(".rs")) or
    . == "releases/jobs/CHANGELOG.md")' <<< "$baseline" > /dev/null; then
  echo jobs
else
  echo 'Full validation: missing baseline or shared/unqualified changes.' >&2
  echo workspace
fi
