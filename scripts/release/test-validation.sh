#!/usr/bin/env bash
set -euo pipefail
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
export PATH="$root/.tools/host/bin:$PATH"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
mkdir -p "$root/target"
fixture="$(fixture_create "$root/target/group-validation-fixture.XXXXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Group validation fixture retained"' EXIT
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset RELEASE_COMMIT RELEASE_VALIDATION_EVIDENCE
mkdir -p "$fixture/scripts/release" "$fixture/scripts/ci" "$fixture/ci" \
    "$fixture/crates/ic-jobs/src" "$fixture/crates/ic-timers/src" "$fixture/target"
cp "$root/scripts/release/validation.pl" "$fixture/scripts/release/"
for name in run-validation-targets.sh check-make-execution.sh verify-file-checksum.sh validation-input-checkpoint.pl release-validation-plan.pl ValidationPlan.pm; do
    cp "$root/scripts/ci/$name" "$fixture/scripts/ci/"
done
cp "$root/ci/"{cargo-release-groups.json,release-validation.json} "$fixture/ci/"
printf '/target/\n' > "$fixture/.gitignore"
printf 'workspace fixture\n' > "$fixture/Cargo.toml"
printf 'locked fixture\n' > "$fixture/Cargo.lock"
printf 'fn jobs() {}\n' > "$fixture/crates/ic-jobs/src/lib.rs"
printf 'fn timers() {}\n' > "$fixture/crates/ic-timers/src/lib.rs"
cat > "$fixture/scripts/release/group.sh" <<'GROUP'
#!/usr/bin/env bash
set -euo pipefail
case "${RELEASE_GROUP:-}" in jobs) version=0.34.1 ;; timers) version=0.38.1 ;; *) exit 2 ;; esac
# Cargo group admission is qualified separately with the real editor/metadata
# fixtures. This bounded fixture substitutes only that catalog observation.
printf '{"group":"%s","previous":"%s","files":["Cargo.toml","Cargo.lock"]}\n' "$RELEASE_GROUP" "$version"
GROUP
roster="$(make --silent --no-print-directory -C "$root" release-validation-stages | tr '\n' ' ')"
printf 'override RELEASE_VALIDATION_STAGES := %s\n' "$roster" > "$fixture/Makefile"
cat >> "$fixture/Makefile" <<'MAKE'
.PHONY: release-validation-stages $(RELEASE_VALIDATION_STAGES) nested-check
release-validation-stages:
	@printf '%s\n' $(RELEASE_VALIDATION_STAGES)
$(RELEASE_VALIDATION_STAGES):
	@printf '%s\n' '$@' >> target/events
	@test "$(FAIL_STAGE)" != '$@'
	@if test "$(INTERRUPT_STAGE)" = '$@'; then exit 130; fi
	@if test "$(DRIFT_STAGE)" = '$@'; then printf 'drift\n' >> crates/ic-jobs/src/lib.rs; fi
	@if test '$@' = ci; then bash scripts/ci/run-validation-targets.sh --fail-fast nested-check; fi
nested-check:
	@true
MAKE
cd "$fixture"
git init -q -b main
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
    VALIDATION_LOG_DIR="$attempt/logs" bash scripts/ci/run-validation-targets.sh \
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
full_events() { make --silent release-validation-stages > target/expected; cmp target/events target/expected; }
start_case
started="$SECONDS"
FAIL_STAGE=test-timer-watchdog run_case
narrow_seconds="$((SECONDS - started))"
scope_is group
[[ "$(cat target/events)" == ci ]]
perl scripts/release/validation.pl check "$attempt"
good="$attempt"
: > target/events
started="$SECONDS"
RELEASE_GROUP=timers run_case
full_seconds="$((SECONDS - started))"
scope_is workspace; full_events
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
mkdir -p .git/release-state/fixture.receipts
cp -R "$good" .git/release-state/fixture.receipts/validation.retained
printf 'validation.retained\n' > .git/release-state/fixture.receipts/validation-evidence
rm -rf "$good"
perl scripts/release/validation.pl identity .git/release-state/fixture.receipts > target/identity
grep -Fx scheduling-group-validation-v1 target/identity > /dev/null
RELEASE_GROUP=timers refuse perl scripts/release/validation.pl identity .git/release-state/fixture.receipts
start_case
FAIL_STAGE=ci refuse run_case
refuse perl scripts/release/validation.pl seal "$attempt"
[[ ! -e "$attempt/execution.json" ]]
[[ "$(cat target/events)" == ci ]]
start_case
INTERRUPT_STAGE=ci refuse run_case
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
for path in Cargo.toml Cargo.lock Makefile ci/cargo-release-groups.json unknown-input; do
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
start_case
select_case
jq -cS 'del(.stages[-1])' "$attempt/plan.json" > target/incomplete
mv target/incomplete "$attempt/plan.json"
refuse env VALIDATION_LOG_DIR="$attempt/logs" bash scripts/ci/run-validation-targets.sh --plan "${attempt#"$fixture/"}/plan.json"
[[ ! -s target/events ]]
start_case
printf '\noverride RELEASE_VALIDATION_STAGES += new-check\n' >> Makefile
git add -- Makefile; git commit -qm 'Unmapped full-gate stage'
RELEASE_SOURCE="$(git rev-parse HEAD)"
export RELEASE_SOURCE
refuse select_case
start_case
jq -cS '.groups += ["unknown"]' ci/release-validation.json > target/new-policy
mv target/new-policy ci/release-validation.json
refuse select_case
git checkout -- ci/release-validation.json
start_case
select_case
printf 'new source\n' >> crates/ic-jobs/src/lib.rs
git add -- crates/ic-jobs/src/lib.rs; git commit -qm 'Moved source'
refuse env VALIDATION_LOG_DIR="$attempt/logs" bash scripts/ci/run-validation-targets.sh --plan "${attempt#"$fixture/"}/plan.json"
[[ ! -s target/events ]]
start_case
DRIFT_STAGE=ci refuse run_case
refuse perl scripts/release/validation.pl seal "$attempt"
# A new attempt after restoring the input succeeds; failed attempts remain.
git checkout -- crates/ic-jobs/src/lib.rs
: > target/events
run_case
perl scripts/release/validation.pl check "$attempt"
printf 'Group plans passed: Jobs-only=1 stage; Timers/shared=5; real runner, checkpoints and evidence; build/installed commands substituted.\n'
printf 'Same-source substituted-command qualification: Jobs %ss; complete Timers gate %ss. No production-time saving is claimed.\n' "$narrow_seconds" "$full_seconds"
fixture_complete=true
