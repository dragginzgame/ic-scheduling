#!/usr/bin/env bash
set -euo pipefail
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
export PATH="$root/.tools/host/bin:$PATH"
# Contract cases own their configuration, including when the outer fixture was
# deliberately executed ordinarily for custom flags or parallel Make.
if [[ $# == 0 ]]; then
    environment=("PATH=$PATH" "HOME=${HOME:?}")
    for key in CARGO_HOME RUSTUP_HOME RUSTUP_TOOLCHAIN; do
        if declare -p "$key" > /dev/null 2>&1; then environment+=("$key=${!key}"); fi
    done
    exec env -i "${environment[@]}" bash "$0" --isolated
fi
[[ $# == 1 && "$1" == --isolated ]] || exit 2
source "$root/scripts/ci/fixture-scratch.sh"
fixture="$(fixture_create_workspace "$root" 'validation-fixture-reuse.XXXXXX')"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Fixture reuse qualification retained"' EXIT
mkdir -p "$fixture/repo/target" "$fixture/repo/.tools/host/bin"
while IFS= read -r path; do
    mkdir -p "$fixture/repo/$(dirname "$path")"
    cp "$root/$path" "$fixture/repo/$path"
done < "$root/ci/validation-fixture-inputs.txt"
# Observe a real native tool; a byte-only change below must prevent reuse.
cp "$(command -v jq)" "$fixture/repo/.tools/host/bin/jq"
cp "$(command -v yq)" "$fixture/repo/.tools/host/bin/yq"
cd "$fixture/repo"
printf '/target/\n/.tools/\n' > .gitignore
printf 'fixture source\n' > Cargo.toml
printf 'fixture lock\n' > Cargo.lock
cat > scripts/release/group.sh <<'GROUP'
#!/usr/bin/env bash
printf '{"group":"%s","previous":"1.0.0","files":["Cargo.toml","Cargo.lock"]}\n' "$RELEASE_GROUP"
GROUP
# Substitute only the expensive fixture body; wrapper, observer, native tools,
# exact-input runner, group plan, evidence verifier and recovery are real.
cat > scripts/release/test-validation.sh <<'BODY'
#!/usr/bin/env bash
set -euo pipefail
printf 'execution\n' >> target/fixture-executions
[[ ! -f target/fail-fixture ]] || exit 23
BODY
roster="$(jq -r '.stages[]' ci/release-validation.json | tr '\n' ' ')"
printf '.PHONY: %s\n' "$roster" > Makefile
for stage in $roster; do
    if [[ "$stage" == test-release-validation ]]; then
        printf '%s:\n\t+perl scripts/release/validation-fixture.pl\n' "$stage" >> Makefile
    else printf '%s:\n\t@:\n' "$stage" >> Makefile; fi
done
git init -q --template= -b main
git config user.name 'Scheduling reuse fixture'
git config user.email 'fixture@example.invalid'
git config core.hooksPath /dev/null
git config commit.gpgsign false
git add -- .gitignore Cargo.toml Cargo.lock rust-toolchain.toml Makefile ci scripts
git commit -qm 'Fixture inputs'
git tag -a jobs-v1.0.0 -m baseline
git tag -a timers-v1.0.0 -m baseline
attempt=''
select_attempt() {
    export RELEASE_GROUP="$1" RELEASE_SOURCE
    RELEASE_SOURCE="$(git rev-parse HEAD)"
    attempt="$(mktemp -d target/validation.XXXXXXXX)"
    perl scripts/release/validation.pl select "$attempt" || return
}
run_plan() {
    select_attempt "$1" || return
    SCHEDULING_VALIDATION_ATTEMPT="$attempt" VALIDATION_LOG_DIR="$attempt/logs" \
        bash scripts/ci/run-validation-targets.sh --plan "$attempt/plan.json" > "$attempt/console.log" 2>&1 || return
    perl scripts/release/validation.pl seal "$attempt" || return
    perl scripts/release/validation.pl check "$attempt" || return
}
# Invalidation and fallback belong to the stage adapter. Full plans above/below
# retain cross-group, failed-required-stage and durable group recovery coverage.
run_stage() {
    select_attempt "$1" || return
    SCHEDULING_VALIDATION_ATTEMPT="$attempt" make --no-print-directory test-release-validation \
        > "$attempt/console.log" 2>&1 || return
}
refuse() { if "$@" > target/refusal.log 2>&1; then echo "unexpected admission: $*" >&2; exit 1; fi; }
run_plan timers
first="$attempt"
if [[ "$(uname -s)" != Linux ]]; then
    jq -e '.mode == "ordinary"' "$attempt/fixture-stage/decision.json"
    fixture_complete=true
    exit 0
fi
jq -e '.mode == "executed"' "$first/fixture-stage/decision.json" > /dev/null
printf 'new unrelated source\n' > unrelated.txt
git add -- unrelated.txt; git commit -qm 'New source requiring a full Jobs plan'
run_plan jobs
second="$attempt"
jq -e '.mode == "reused" and .group == "jobs" and .seal.origin.group == "timers"' "$second/fixture-stage/decision.json" > /dev/null
[[ "$(wc -l < target/fixture-executions)" == 1 ]]
# Missing/tampered retained bytes cannot satisfy group recovery.
cp "$second/fixture-stage/decision.json" target/decision.json
rm "$second/fixture-stage/decision.json"
refuse perl scripts/release/validation.pl check "$second"
cp target/decision.json "$second/fixture-stage/decision.json"
cp "$second/fixture-stage/original-0" target/original-0
printf 'tamper\n' >> "$second/fixture-stage/original-0"
refuse perl scripts/release/validation.pl check "$second"
cp target/original-0 "$second/fixture-stage/original-0"
# Corrupt original cache logs force a real execution, not a reused PASS.
original="$(jq -r '.seal.origin.attempt' "$second/fixture-stage/decision.json")"
printf 'tamper\n' >> "$original/command.stdout"
run_stage timers
jq -e '.mode == "executed"' "$attempt/fixture-stage/decision.json" > /dev/null
[[ "$(wc -l < target/fixture-executions)" == 2 ]]
# Source and native tool bytes invalidate their exact-input proofs.
printf '# changed fixture input\n' >> scripts/release/test-validation.sh
run_stage timers
jq -e '.mode == "executed"' "$attempt/fixture-stage/decision.json" > /dev/null
printf x >> .tools/host/bin/jq
run_stage timers
jq -e '.mode == "executed"' "$attempt/fixture-stage/decision.json" > /dev/null
[[ "$(wc -l < target/fixture-executions)" == 4 ]]
# User configuration falls back visibly, without resetting it to obtain reuse.
RUSTFLAGS='--cfg fixture_custom' run_stage timers
jq -e '.mode == "ordinary" and (.reason|contains("configuration"))' "$attempt/fixture-stage/decision.json" > /dev/null
MAKEFLAGS=-j2 run_stage timers
jq -e '.mode == "ordinary" and (.reason|contains("parallel Make"))' "$attempt/fixture-stage/decision.json" > /dev/null
# Failed fresh work cannot be sealed or become a future execution proof.
printf '# force a new context\n' >> scripts/release/test-validation.sh
touch target/fail-fixture
refuse run_plan timers
[[ ! -e "$attempt/execution.json" && ! -e "$attempt/fixture-stage/decision.json" ]]
rm target/fail-fixture
run_plan timers
jq -e '.mode == "executed"' "$attempt/fixture-stage/decision.json" > /dev/null
# Recovery uses the group's self-contained copies after scratch cache deletion.
rm -rf target/validation-fixture-cache
perl scripts/release/validation.pl check "$attempt"
printf 'Fixture reuse passed (four full-plan attempts; five direct-stage cases): cross-group admission, exact inputs/tools, configuration fallback, corrupt/missing evidence, failed execution and cache-independent recovery. Product commands substituted.\n'
fixture_complete=true
