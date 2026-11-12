#!/usr/bin/env bash
set -euo pipefail
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES

# Execute the selected wrapper and collector; the hosted upload is not invoked.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-scratch.sh
source "$root/scripts/ci/fixture-scratch.sh"
fixture="$(fixture_create_workspace "$root" "jobs-formatting-evidence.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Formatting evidence fixture retained"' EXIT
mkdir "$fixture/temp" "$fixture/repository" "$fixture/unpacked"
(umask 077; mkdir "$fixture/store")
yq -o json '.' "$root/.github/workflows/ci.yml" > "$fixture/workflow.json"
yq -o json '.' "$root/.github/actions/retain-failure-evidence/action.yml" > "$fixture/collector.json"
jq '.jobs.validate.steps' "$fixture/workflow.json" > "$fixture/workflow-steps.json"
jq '.runs.steps' "$fixture/collector.json" > "$fixture/collector-steps.json"
jq -e '
    .jobs.validate.steps as $steps |
    [$steps | to_entries[] | select(.value.id == "validate")] as $gates |
    [$steps | to_entries[] | select(.value.uses == "./.github/actions/retain-failure-evidence")] as $collectors |
    ($gates | length) == 1 and ($collectors | length) == 1 and
    $gates[0].value["continue-on-error"] != true and
    $collectors[0].key > $gates[0].key and $collectors[0].value.if == "failure()"
' "$fixture/workflow.json" > /dev/null

# Execute the actual workflow gate with a failed fixture. Its temporary root
# must be one the actual collector archives, and the child status must survive.
bash "$root/scripts/ci/extract-workflow-step.sh" "$fixture/workflow-steps.json" validate \
    > "$fixture/gate.sh"
temporary_selection="$(jq -er '.[] | select(.id == "validate") | .env.TMPDIR' "$fixture/workflow-steps.json")"
# Substitute the literal GitHub expression, not a shell variable.
# shellcheck disable=SC2016
gate_temp="${temporary_selection//'${{ runner.temp }}'/$fixture/temp}"
mkdir "$fixture/bin"
cat > "$fixture/bin/make" <<'MAKE'
#!/usr/bin/env bash
set -euo pipefail
[[ "$*" == ci ]] || exit 45
attempt="$(mktemp -d "$TMPDIR/jobs-release-tooling.XXXXXX")"
printf 'retained-inner-fixture-failure\n' > "$attempt/failure.log"
exit 37
MAKE
chmod +x "$fixture/bin/make"
status=0
PATH="$fixture/bin:$PATH" TMPDIR="$gate_temp" bash "$fixture/gate.sh" \
    > "$fixture/gate-output" 2>&1 || status=$?
[[ "$status" == 37 ]] || exit 1
attempts=("$gate_temp/"*)
[[ ${#attempts[@]} == 1 && -f "${attempts[0]}/failure.log" ]] || exit 1

# Shared owns generic driver/reuse and standalone wrapper proofs.
# Jobs retains actual workflow, Make and collector integration below.

# Exercise Jobs' actual Makefile and selected includes, substituting only Cargo
# in this disposable checkout. A sorter failure must stop the later rustfmt call.
# Force every pass for this caller/evidence proof; metadata/reuse cases stay Shared-owned.
consumer="$fixture/jobs"
mkdir -p "$consumer/.tools/rust/common/bin" "$consumer/src"
printf '[package]\nname = "formatting-fixture"\nversion = "0.0.0"\nedition = "2024"\n' > "$consumer/Cargo.toml"
printf 'pub fn fixture() {}\n' > "$consumer/src/lib.rs"
bash "$root/scripts/ci/prepare-script-fixture.sh" "$root" "$consumer" make/consumer.mk make/cargo-dependencies.mk
cp "$root/Makefile" "$consumer/"
cp -R "$root/ci" "$consumer/"
cp "$root/scripts/ci/check-format-tools.sh" "$consumer/scripts/ci/"
export JOBSERVER_PROBE="$root/scripts/ci/check-pipe-jobserver.pl"
cat > "$consumer/.tools/rust/common/bin/formatting-fixture-cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
[[ "$CARGO_NET_OFFLINE" == true && "$RUSTUP_AUTO_INSTALL" == 0 ]] || exit 1
perl "$JOBSERVER_PROBE"
case "$*" in
    'metadata --locked --offline --no-deps --format-version 1')
        perl -MJSON::PP -MCwd=getcwd -MEncode=decode -e '
            $root=decode("UTF-8",getcwd());
            print encode_json({workspace_root=>$root,workspace_members=>["fixture"],
                packages=>[{id=>"fixture",name=>"formatting-fixture",manifest_path=>"$root/Cargo.toml",
                    targets=>[{src_path=>"$root/src/lib.rs"}]}],target_directory=>"$root/target"});'
        exit 0 ;;
    'sort --version') echo "cargo-sort ${FORMAT_EVIDENCE_VERSION:?}"; exit 0 ;;
    'fmt --version') exit 0 ;;
esac
printf '%s\n' "$*" >> "$FORMAT_EVIDENCE_EVENTS"
case "$*" in
    'sort --workspace --grouped --check')
        echo retained-sorter-diff
        if [[ "${FORMAT_EVIDENCE_FAIL:-}" == sort ]]; then
            echo sorter-error >&2; exit 23
        fi ;;
    'fmt --package formatting-fixture -- --check') echo successful-rustfmt-detail ;;
    *) exit 45 ;;
esac
CARGO
chmod +x "$consumer/.tools/rust/common/bin/formatting-fixture-cargo"
# The pin catalog is selected from the resolved consumer root at runtime.
# shellcheck disable=SC1091
. "$root/ci/tool-versions.env"
export FORMAT_EVIDENCE_VERSION="$TOKO_TOOLING_CARGO_SORT_VERSION"
export FORMAT_EVIDENCE_EVENTS="$fixture/events"
: > "$FORMAT_EVIDENCE_EVENTS"
status=0
parallel=(-j2)
if make --help | grep -q -- --jobserver-style; then parallel+=(--jobserver-style=pipe); fi
RUNNER_TEMP="$fixture/temp" FORMAT_EVIDENCE_FAIL=sort \
    make --no-print-directory "${parallel[@]}" -C "$consumer" fmt-check FORMAT_CARGO=formatting-fixture-cargo FORMAT_RUST_ARGS=--no-cache \
    > "$fixture/make-failure" 2>&1 || status=$?
[[ "$status" == 2 ]] || exit 1
printf 'sort --workspace --grouped --check\n' > "$fixture/expected-events"
cmp "$fixture/expected-events" "$FORMAT_EVIDENCE_EVENTS"
grep -E '^Manifest sorting \[\.\]\.\.\. FAILED \(exit 23, [0-9]+[.][0-9]+s\)$' "$fixture/make-failure" > /dev/null
logs=("$fixture/temp"/formatting.*)
[[ ${#logs[@]} == 1 ]] || exit 1
grep -Flx sorter-error "${logs[@]}" > "$fixture/sorter-logs"
[[ "$(wc -l < "$fixture/sorter-logs" | tr -d ' ')" == 1 ]] || exit 1
sorter_log="$(cat "$fixture/sorter-logs")"
grep -Fx "Details: $sorter_log" "$fixture/make-failure" > /dev/null
grep -Fx retained-sorter-diff "$sorter_log" > /dev/null
: > "$FORMAT_EVIDENCE_EVENTS"
RUNNER_TEMP="$fixture/temp" make --no-print-directory "${parallel[@]}" -C "$consumer" \
    fmt-check FORMAT_CARGO=formatting-fixture-cargo FORMAT_RUST_ARGS=--no-cache > "$fixture/make-success"
printf 'sort --workspace --grouped --check\nfmt --package formatting-fixture -- --check\n' > "$fixture/expected-events"
cmp "$fixture/expected-events" "$FORMAT_EVIDENCE_EVENTS"
printf 'Manifest sorting [.]... ok\nRust formatting [.]... ok\n' > "$fixture/expected"
sed -E 's/ \([0-9]+[.][0-9]+s\)$//' "$fixture/make-success" > "$fixture/normalized-success"
cmp "$fixture/expected" "$fixture/normalized-success"

bash "$root/scripts/ci/extract-workflow-step.sh" "$fixture/collector-steps.json" archive \
    > "$fixture/collect.sh"
TOKO_TOOLING_STORE="$fixture/store" \
    EVIDENCE_TEMP_ROOT="$fixture/temp" EVIDENCE_REPOSITORY_ROOT="$fixture/repository" \
    EVIDENCE_ACTION_ROOT="$root/.github/actions/retain-failure-evidence" \
    RUNNER_TEMP="$fixture/temp" GITHUB_OUTPUT="$fixture/archive-output" \
    bash "$fixture/collect.sh" > "$fixture/collection.log" 2>&1
archive="$(sed -n 's/^path=//p' "$fixture/archive-output")"
[[ -f "$archive" ]] || exit 1
tar -xzf "$archive" -C "$fixture/unpacked"
for log in "${logs[@]}"; do cmp "$log" "$fixture/unpacked/${log##*/}"; done
cmp "${attempts[0]}/failure.log" \
    "$fixture/unpacked/portable-fixtures/${attempts[0]##*/}/failure.log"

# Shared test-tool-evidence.sh owns refused-store collection, retained local
# diagnostics and incomplete-collection refusal. The Jobs caller/archive proof
# above retains its actual workflow ordering and formatting/fixture logs.
echo 'Formatting failure propagation and local archive retention passed (no hosted upload)'
fixture_complete=true
