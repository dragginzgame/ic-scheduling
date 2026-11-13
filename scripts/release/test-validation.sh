#!/usr/bin/env bash
set -euo pipefail
[[ $# == 0 || ( $# == 1 && "$1" == --scope-only ) ]] || { echo "usage: test-validation.sh [--scope-only]" >&2; exit 2; }
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
export PATH="$root/.tools/host/bin:$PATH"
# shellcheck source=vendor/toko-tooling/scripts/ci/fixture-lifecycle.sh
source "$root/vendor/toko-tooling/scripts/ci/fixture-lifecycle.sh"
mkdir -p "$root/target"
fixture="$(fixture_create "$root/target/group-validation-fixture.XXXXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Group validation fixture retained"' EXIT
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset RELEASE_COMMIT RELEASE_VALIDATION_EVIDENCE
mkdir -p "$fixture/scripts/release" "$fixture/vendor/toko-tooling/scripts/ci" "$fixture/ci" \
    "$fixture/crates/ic-jobs/src" "$fixture/crates/ic-timers/src" \
    "$fixture/qualification/ic-jobs-test-consumer/src" "$fixture/target"
cp "$root/scripts/release/"{validation.pl,ci.sh} "$fixture/scripts/release/"
for name in run-validation-targets.sh check-make-execution.sh verify-file-checksum.sh validation-input-checkpoint.pl release-validation-plan.pl ValidationPlan.pm validation-evidence.pl; do
    cp "$root/vendor/toko-tooling/scripts/ci/$name" "$fixture/vendor/toko-tooling/scripts/ci/"
done
cp "$root/ci/"{cargo-release-groups.json,release-validation.json} "$fixture/ci/"
printf '/target/\n' > "$fixture/.gitignore"
printf 'workspace fixture\n' > "$fixture/Cargo.toml"
printf 'locked fixture\n' > "$fixture/Cargo.lock"
printf 'fn jobs() {}\n' > "$fixture/crates/ic-jobs/src/lib.rs"
printf 'fn consumer() {}\n' > "$fixture/qualification/ic-jobs-test-consumer/src/lib.rs"
printf 'fn timers() {}\n' > "$fixture/crates/ic-timers/src/lib.rs"
cat > "$fixture/scripts/release/group.sh" <<'GROUP'
#!/usr/bin/env bash
set -euo pipefail
case "${RELEASE_GROUP:-}" in jobs) version=0.34.1 ;; timers) version=0.38.1 ;; *) exit 2 ;; esac
# Cargo group admission is qualified separately with the real editor/metadata
# fixtures. This bounded fixture substitutes only that catalog observation.
printf '{"group":"%s","previous":"%s","files":["Cargo.toml","Cargo.lock"]}\n' "$RELEASE_GROUP" "$version"
GROUP
roster="$(perl "$root/scripts/release/validation.pl" roster | tr '\n' ' ')"
printf '.PHONY: %s\n' "$roster" > "$fixture/Makefile"
printf '%s:\n' "$roster" >> "$fixture/Makefile"
cat >> "$fixture/Makefile" <<'MAKE'
	@printf '%s\n' '$@' >> target/events
	@test "$(FAIL_STAGE)" != '$@'
	@if test "$(INTERRUPT_STAGE)" = '$@'; then exit 130; fi
	@if test "$(DRIFT_STAGE)" = '$@'; then printf 'drift\n' >> crates/ic-jobs/src/lib.rs; fi
.PHONY: release-validation-stages ci
release-validation-stages:
	@perl scripts/release/validation.pl release-stages
ci:
	+bash scripts/release/ci.sh
MAKE
cd "$fixture"
git init -q --template= -b main
git config user.name 'Scheduling validation fixture'
git config user.email 'fixture@example.invalid'
git config core.hooksPath /dev/null
git add -- .; git commit -qm 'qualified fixture baseline'
baseline="$(git rev-parse HEAD)"
git tag -a jobs-v0.34.1 -m 'Jobs fixture baseline'
git tag -a timers-v0.38.1 -m 'Timers fixture baseline'
export RELEASE_GROUP=jobs
export RELEASE_SOURCE="$baseline"
attempt=''
start_case() {
    git checkout -q --detach "$baseline"
    printf 'fn changed_jobs() {}\n' >> crates/ic-jobs/src/lib.rs
    git add -- crates/ic-jobs/src/lib.rs; git commit -qm 'Jobs-owned change'
    RELEASE_SOURCE="$(git rev-parse HEAD)"
    export RELEASE_SOURCE
    : > target/events
}
select_case() {
    attempt="$(mktemp -d "$fixture/target/validation.XXXXXXXX")"
    perl scripts/release/validation.pl select "$attempt"
}
run_case() {
    local status=0
    select_case || return
    VALIDATION_REPOSITORY_ROOT="$PWD" VALIDATION_LOG_DIR="$attempt/logs" bash vendor/toko-tooling/scripts/ci/run-validation-targets.sh \
        --plan "${attempt#"$fixture/"}/plan.json" > "$attempt/console.log" 2>&1 || status=$?
    [[ "$status" == 0 ]] || return "$status"
    perl scripts/release/validation.pl seal "$attempt"
}
refuse() {
    if "$@" > target/refusal.log 2>&1; then
        printf 'Unexpected admission: %s\n' "$*" >&2; exit 1
    fi
}
scope_is() { [[ "$(jq -r .scope "$attempt/plan.json")" == "$1" ]]; }
full_events() { perl scripts/release/validation.pl roster > target/expected; cmp target/events target/expected; }
start_case
printf '// relocated consumer change\n' >> qualification/ic-jobs-test-consumer/src/lib.rs
git add -- qualification/ic-jobs-test-consumer/src/lib.rs
git commit -qm 'Relocated consumer-owned change'
RELEASE_SOURCE="$(git rev-parse HEAD)"
export RELEASE_SOURCE
select_case; scope_is group
if [[ "${1:-}" == --scope-only ]]; then
    start_case
    printf 'independent qualification lock change\n' > qualification/Cargo.lock
    git add -- qualification/Cargo.lock; git commit -qm 'Qualification graph change'
    RELEASE_SOURCE="$(git rev-parse HEAD)"; export RELEASE_SOURCE
    select_case; scope_is workspace
    jq -e 'all(.stages[]; .required)' "$attempt/plan.json" > /dev/null
    echo 'Moved Jobs qualification source retains Jobs/consumer scope; qualification lock changes retain the complete gate (selection only, no product effects)'
    fixture_complete=true
    exit 0
fi
start_case
started="$SECONDS"
FAIL_STAGE=test-timer-watchdog run_case
narrow_seconds="$((SECONDS - started))"
scope_is group
# Independent product expectation: selected Jobs keeps real consumer/tooling and
# graph boundaries, without standalone Timers or unchanged release self-tests.
cat > target/expected <<'JOBS'
toko-tooling-check
validation-tools-check
check-release-groups
clippy-jobs
check-doc-links
check-pins
check-dependency-boundaries
check-dependency-graphs
check-release-commands
test-consumer-tooling
test-recovery-evidence
fmt-check
test-jobs
test-consumers
check-consumer
check-wasm
docs-jobs
package-jobs
JOBS
cmp target/events target/expected
cp target/expected target/jobs-expected
jq -e 'all(.stages[]; .target != "ci")' "$attempt/plan.json" > /dev/null
[[ "$(jq '[.stages[] | select(.required == false)] | length' "$attempt/plan.json")" == 18 ]]
for excluded in test-timers clippy-timers package-timers test-release-tooling test-release-validation; do
    : > target/events
    FAIL_STAGE="$excluded" run_case
    scope_is group
    cmp target/events target/jobs-expected
done
for required in test-jobs test-consumers check-dependency-graphs package-jobs; do
    : > target/events
    FAIL_STAGE="$required" refuse run_case
    refuse perl scripts/release/validation.pl seal "$attempt"
    [[ ! -e "$attempt/execution.json" ]]
done
: > target/events
run_case
perl scripts/release/validation.pl check "$attempt"
good="$attempt"
: > target/events
make --silent ci > target/ci.log 2>&1
perl scripts/release/validation.pl ci-stages > target/expected; cmp target/events target/expected
: > target/events
started="$SECONDS"
RELEASE_GROUP=timers run_case
full_seconds="$((SECONDS - started))"
scope_is workspace; full_events
# Required/excluded decisions are product policy, not authority supplied by a
# self-consistent editable plan. Shared verifies execution; Scheduling checks mapping.
start_case
select_case
jq -cS '(.stages[] | select(.target == "test-jobs")) |= (.required = false | .reason = "tampered omission")' "$attempt/plan.json" > target/tampered
mv target/tampered "$attempt/plan.json"
VALIDATION_REPOSITORY_ROOT="$PWD" VALIDATION_LOG_DIR="$attempt/logs" bash vendor/toko-tooling/scripts/ci/run-validation-targets.sh --plan "${attempt#"$fixture/"}/plan.json" > "$attempt/console.log" 2>&1
refuse perl scripts/release/validation.pl seal "$attempt"
grep -F 'validation selection differs' target/refusal.log > /dev/null
[[ ! -e "$attempt/execution.json" ]]
# Restore the same source selected by the retained good execution for custody checks.
RELEASE_SOURCE="$(jq -r .source "$good/context.json")"
export RELEASE_SOURCE
git checkout -q --detach "$RELEASE_SOURCE"
refuse env RUSTFLAGS='--cfg changed' perl scripts/release/validation.pl check "$good"
refuse env RUSTC_WRAPPER=changed-compiler-wrapper perl scripts/release/validation.pl check "$good"
log="$(find "$good/logs" -name 0.log | head -n 1)"
cp "$log" target/original.log; printf 'tamper\n' >> "$log"
refuse perl scripts/release/validation.pl check "$good"
cp target/original.log "$log"
printf 'changed\n' >> crates/ic-jobs/src/lib.rs
refuse perl scripts/release/validation.pl check "$good"
git checkout -- crates/ic-jobs/src/lib.rs
cp "$good/execution.json" target/execution.json
rm "$good/execution.json"
refuse perl scripts/release/validation.pl check "$good"
cp target/execution.json "$good/execution.json"
# Historical consumer receipts stay with their original adapter, never upgrade.
jq -cS '.schema = 1' target/execution.json > "$good/execution.json"
cp "$good/execution.json" target/historical-execution.json
refuse perl scripts/release/validation.pl check "$good"
cmp "$good/execution.json" target/historical-execution.json
cp target/execution.json "$good/execution.json"
mkdir -p .git/release-state/fixture.receipts
cp -R "$good" .git/release-state/fixture.receipts/validation.retained
printf 'validation.retained\n' > .git/release-state/fixture.receipts/validation-evidence
rm -rf "$good"
perl scripts/release/validation.pl identity .git/release-state/fixture.receipts > target/identity
grep -Fx scheduling-group-validation-v3 target/identity > /dev/null
RELEASE_GROUP=timers refuse perl scripts/release/validation.pl identity .git/release-state/fixture.receipts
start_case
FAIL_STAGE=validation-tools-check refuse run_case
refuse perl scripts/release/validation.pl seal "$attempt"
[[ ! -e "$attempt/execution.json" ]]
printf 'toko-tooling-check\nvalidation-tools-check\n' > target/expected; cmp target/events target/expected
start_case
INTERRUPT_STAGE=validation-tools-check refuse run_case
refuse perl scripts/release/validation.pl seal "$attempt"
[[ ! -e "$attempt/execution.json" ]]
start_case
printf 'fn changed_timer() {}\n' >> crates/ic-timers/src/lib.rs
git add -- crates/ic-timers/src/lib.rs; git commit -qm 'Outstanding Timers change'
RELEASE_SOURCE="$(git rev-parse HEAD)"
export RELEASE_SOURCE
FAIL_STAGE=test-timer-watchdog refuse run_case
refuse perl scripts/release/validation.pl seal "$attempt"
[[ ! -e "$attempt/execution.json" ]]
: > target/events
run_case
scope_is workspace; full_events
: > target/events
RELEASE_GROUP=timers run_case
scope_is workspace; full_events
for path in Cargo.toml Cargo.lock Makefile ci/cargo-release-groups.json scripts/release/ci.sh unknown-input; do
    start_case
    if [[ "$path" == ci/cargo-release-groups.json ]]; then
        printf '\n' >> "$path"
    elif [[ "$path" == Makefile ]]; then
        printf '\n# changed recipe\n' >> "$path"
    else printf 'changed\n' >> "$path"; fi
    git add -- "$path"; git commit -qm 'Shared or uncertain input'
    RELEASE_SOURCE="$(git rev-parse HEAD)"
    export RELEASE_SOURCE
    select_case; scope_is workspace
done
start_case
printf 'dirty\n' >> crates/ic-jobs/src/lib.rs
select_case; scope_is workspace
git checkout -- crates/ic-jobs/src/lib.rs
RELEASE_GROUP=unknown refuse perl scripts/release/validation.pl select target
start_case
git tag -d jobs-v0.34.1 > /dev/null
select_case; scope_is workspace
git tag jobs-v0.34.1 "$baseline"
select_case; scope_is workspace
git tag -d jobs-v0.34.1 > /dev/null
foreign="$(printf 'unrelated baseline\n' | git commit-tree "$(git rev-parse "$baseline^{tree}")")"
git tag -a jobs-v0.34.1 "$foreign" -m unrelated
select_case; scope_is workspace
git tag -d jobs-v0.34.1 > /dev/null
git tag -a jobs-v0.34.1 "$baseline" -m restored
# Shared admits declared inputs; Scheduling must require the whole source inventory.
start_case
select_case
jq -cS '.inputs -= ["crates/ic-jobs/src/lib.rs"]' "$attempt/plan.json" > target/incomplete
mv target/incomplete "$attempt/plan.json"
VALIDATION_REPOSITORY_ROOT="$PWD" VALIDATION_LOG_DIR="$attempt/logs" bash vendor/toko-tooling/scripts/ci/run-validation-targets.sh --plan "${attempt#"$fixture/"}/plan.json" > "$attempt/console.log" 2>&1
refuse perl scripts/release/validation.pl seal "$attempt"
grep -F 'missing source inventory input' target/refusal.log > /dev/null
[[ ! -e "$attempt/execution.json" ]]
# A self-consistent generated policy cannot omit a declared Timers obligation.
start_case
select_case
jq -cS '.stages -= ["test-timer-watchdog"]' "$attempt/release-policy.json" > target/incomplete
mv target/incomplete "$attempt/release-policy.json"
jq -cS 'del(.stages[] | select(.target == "test-timer-watchdog"))' "$attempt/plan.json" > target/incomplete
mv target/incomplete "$attempt/plan.json"
VALIDATION_REPOSITORY_ROOT="$PWD" VALIDATION_LOG_DIR="$attempt/logs" bash vendor/toko-tooling/scripts/ci/run-validation-targets.sh --plan "${attempt#"$fixture/"}/plan.json" > "$attempt/console.log" 2>&1
refuse perl scripts/release/validation.pl seal "$attempt"
grep -F 'release policy differs' target/refusal.log > /dev/null
[[ ! -e "$attempt/execution.json" ]]
start_case
select_case
jq -cS 'del(.stages[-1])' "$attempt/plan.json" > target/incomplete
mv target/incomplete "$attempt/plan.json"
refuse env VALIDATION_REPOSITORY_ROOT="$PWD" VALIDATION_LOG_DIR="$attempt/logs" bash vendor/toko-tooling/scripts/ci/run-validation-targets.sh --plan "${attempt#"$fixture/"}/plan.json"
[[ ! -s target/events ]]
# A new common obligation reaches both normal CI and selected Jobs plans.
start_case
jq -cS '.stages |= (index("install-testkit-tools") as $i | .[:$i] + ["new-check"] + .[$i:])' ci/release-validation.json > target/new-policy
mv target/new-policy ci/release-validation.json
printf '\nnew-check:\n\t@printf "new-check\\n" >> target/events\n' >> Makefile
git add -- ci/release-validation.json Makefile; git commit -qm 'New common obligation'
RELEASE_SOURCE="$(git rev-parse HEAD)"
export RELEASE_SOURCE
run_case; scope_is workspace; full_events
: > target/events
make --silent ci > target/ci.log 2>&1
perl scripts/release/validation.pl ci-stages > target/expected; cmp target/events target/expected
# Even with an admitted baseline containing the new policy, an unclassified
# obligation is required in a later genuinely Jobs-only plan.
git tag -d jobs-v0.34.1 > /dev/null
git tag -a jobs-v0.34.1 "$RELEASE_SOURCE" -m 'New policy qualified baseline'
printf 'fn another_jobs_change() {}\n' >> crates/ic-jobs/src/lib.rs
git add -- crates/ic-jobs/src/lib.rs; git commit -qm 'Jobs change after new obligation'
RELEASE_SOURCE="$(git rev-parse HEAD)"
export RELEASE_SOURCE
: > target/events
run_case; scope_is group
grep -Fx new-check target/events > /dev/null
git tag -d jobs-v0.34.1 > /dev/null
git tag -a jobs-v0.34.1 "$baseline" -m 'Restored original policy baseline'
# Projection cannot silently reorder a new common obligation after installed work.
start_case
jq -cS '.stages += ["late-common"]' ci/release-validation.json > target/new-policy
mv target/new-policy ci/release-validation.json
refuse select_case
git checkout -- ci/release-validation.json
start_case
jq -cS '.groups += ["unknown"]' ci/release-validation.json > target/new-policy
mv target/new-policy ci/release-validation.json
refuse select_case
git checkout -- ci/release-validation.json
start_case
select_case
printf 'new source\n' >> crates/ic-jobs/src/lib.rs
git add -- crates/ic-jobs/src/lib.rs; git commit -qm 'Moved source'
refuse env VALIDATION_REPOSITORY_ROOT="$PWD" VALIDATION_LOG_DIR="$attempt/logs" bash vendor/toko-tooling/scripts/ci/run-validation-targets.sh --plan "${attempt#"$fixture/"}/plan.json"
[[ ! -s target/events ]]
start_case
DRIFT_STAGE=validation-tools-check refuse run_case
refuse perl scripts/release/validation.pl seal "$attempt"
# A new attempt after restoring the input succeeds; failed attempts remain.
git checkout -- crates/ic-jobs/src/lib.rs
: > target/events
run_case
perl scripts/release/validation.pl check "$attempt"
printf 'Group plans passed: 18 Jobs obligations; 36 complete release obligations; unchanged product coverage; exact mapping, shared verifier and durable recovery; product commands substituted.\n'
printf 'Same-source substituted-command qualification: Jobs %ss; complete Timers gate %ss. No production-time saving is claimed.\n' "$narrow_seconds" "$full_seconds"
fixture_complete=true
# Qualify this consumer contract inside the one bounded fixture execution.
bash "$root/scripts/release/test-validation-reuse.sh"
