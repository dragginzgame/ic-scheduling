#!/usr/bin/env bash
set -euo pipefail
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES

root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
export PATH="$root/.tools/host/bin:$PATH"
# shellcheck source=scripts/ci/fixture-scratch.sh
source "$root/scripts/ci/fixture-scratch.sh"
fixture="$(fixture_create_workspace "$root" "scheduling-ci-selection.XXXXXX")"
fixture_complete=false
finish() {
    local status=$?
    trap - EXIT
    fixture_completion_status "$status" "$fixture_complete" || status=$?
    printf 'CI selection evidence retained: %s\n' "$fixture" >&2
    exit "$status"
}
trap finish EXIT

yq -o json '.' "$root/.github/workflows/ci.yml" > "$fixture/workflow.json"
jq '.jobs."change-scope".steps' "$fixture/workflow.json" > "$fixture/classify-steps.json"
jq '.jobs.validate.steps' "$fixture/workflow.json" > "$fixture/validate-steps.json"
for selection in 'classify classify' 'classification-admission admission' 'documentation docs' 'validate native'; do
    read -r step_id script_name <<< "$selection"
    step_file=validate
    [[ "$step_id" != classify ]] || step_file=classify
    bash "$root/vendor/toko-tooling/scripts/ci/extract-workflow-step.sh" "$fixture/$step_file-steps.json" "$step_id" > "$fixture/$script_name.sh"
done

docs_condition="needs.change-scope.outputs.scope == 'docs'"
native_condition="needs.change-scope.outputs.scope != 'docs'"
# Assert the real workflow retains identities/history, failure admission, every
# native host, and the same full gate. Only the two supported scope predicates
# are interpreted below; changing workflow routing requires fixture review.
jq -e --arg docs "$docs_condition" --arg native "$native_condition" \
    --arg head '${{ github.sha }}' --arg guard '${{ !cancelled() }}' '
    .jobs as $jobs |
    $jobs."change-scope".steps[0].with."fetch-depth" == 0 and
    $jobs."change-scope".steps[1].env.CHANGE_HEAD == $head and
    ($jobs."change-scope".steps[1].env.CHANGE_BASE | contains("github.event.pull_request.base.sha")) and
    ($jobs."change-scope".steps[1].env.CHANGE_BASE | contains("github.event.before")) and
    ($jobs."change-scope".steps[1].env.CHANGE_REF | contains("github.base_ref")) and
    $jobs.validate.needs == "change-scope" and $jobs.validate.if == $guard and
    $jobs.validate.strategy.matrix.os == ["ubuntu-24.04", "macos-15", "macos-15-intel"] and
    $jobs.validate.strategy."fail-fast" == false and
    $jobs.validate.steps[1].id == "classification-admission" and
    $jobs.validate.steps[1].env.CLASSIFICATION_RESULT == "${{ needs.change-scope.result }}" and
    all($jobs.validate.steps[];
        if .id == "documentation" or .id == "save-doc-downloads" then .if == $docs
        elif .id == "classification-admission" or (.uses // "" | startswith("actions/checkout@"))
            or .id == "restore-downloads" or .run == "make install-host-tools" then has("if") | not
        elif .if == "failure()" then .uses == "./vendor/toko-tooling/.github/actions/retain-failure-evidence"
        else .if == $native end) and
    ((.on.pull_request // {}) | has("paths") | not) and
    ((.on.push // {}) | has("paths") | not)
' "$fixture/workflow.json" > "$fixture/workflow-contract.txt"

# The repository scan runs even for docs and failed classification, once outside
# native/recovery matrices. Exercise the real scan step's arguments and failure.
jq -e '
    .jobs.secrets |
    ."runs-on" == "ubuntu-24.04" and
    (has("strategy") or has("needs") or has("if") | not) and
    .steps[0].with."fetch-depth" == 0 and
    ([.steps[] | select(.id == "scan-secrets")] | length == 1) and
    all(.steps[]; has("if") | not)
' "$fixture/workflow.json" > "$fixture/secrets-contract.txt"
jq '.jobs.secrets.steps' "$fixture/workflow.json" > "$fixture/secrets-steps.json"
bash "$root/vendor/toko-tooling/scripts/ci/extract-workflow-step.sh" "$fixture/secrets-steps.json" scan-secrets > "$fixture/scan.sh"
cat > "$fixture/scanner" <<'SCANNER'
#!/usr/bin/env bash
[[ "$*" == 'git --redact=100 --no-banner --no-color --log-level error' ]] || exit 91
exit "$SCANNER_RESULT"
SCANNER
chmod +x "$fixture/scanner"
for result in 0 23; do
    status=0
    GITLEAKS_BIN="$fixture/scanner" SCANNER_RESULT="$result" bash -e -o pipefail "$fixture/scan.sh" || status=$?
    [[ "$status" == "$result" ]]
done

# Cache restoration precedes setup; each scope saves admitted downloads before
# its product checks. The explicitly dispatched recovery workflow shares setup,
# but does not schedule a second repository scan.
yq -o json '.' "$root/.github/workflows/recovery.yml" > "$fixture/recovery.json"
jq -e --slurpfile recovery "$fixture/recovery.json" '
    def position($id): map(.id) | index($id);
    def cache_steps:
        all(.[] | select(.id == "restore-downloads" or .id == "save-downloads" or .id == "save-doc-downloads");
            .uses == "./vendor/toko-tooling/.github/actions/cache-tool-downloads" and
            (if .id == "restore-downloads" then (.with.mode // "restore") == "restore"
             else .with.mode == "save" end));
    .jobs.validate.steps as $steps |
    ($steps | cache_steps) and
    ([$steps[].id] | contains(["restore-downloads", "host-tools", "save-doc-downloads", "documentation", "selected-tools", "save-downloads", "validate"])) and
    ($steps | position("restore-downloads")) < ($steps | position("host-tools")) and
    ($steps | position("host-tools")) < ($steps | position("save-doc-downloads")) and
    ($steps | position("save-doc-downloads")) < ($steps | position("documentation")) and
    ($steps | position("selected-tools")) < ($steps | position("save-downloads")) and
    ($steps | position("save-downloads")) < ($steps | position("validate")) and
    ($recovery[0].jobs.recovery.steps as $r |
        ($r | cache_steps) and
        ([$r[].id] | contains(["restore-downloads", "host-tools", "selected-tools", "save-downloads", "live"])) and
        ($r | position("restore-downloads")) < ($r | position("host-tools")) and
        ($r | position("selected-tools")) < ($r | position("save-downloads")) and
        ($r | position("save-downloads")) < ($r | position("live"))) and
    ([., $recovery[0]] | [.. | objects | select(.id? == "scan-secrets")] | length == 1)
' "$fixture/workflow.json" > "$fixture/cache-order.txt"

mkdir -p "$fixture/repo/ci" "$fixture/repo/scripts/ci" "$fixture/bin"
cp "$root/ci/docs-only-paths.txt" "$fixture/repo/ci/"
bash "$root/vendor/toko-tooling/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture/repo" \
    vendor/toko-tooling/scripts/ci/classify-ci-change.pl
while IFS= read -r path; do
    case "$path" in ''|'#'*) continue ;; esac
    # Admission review guard: prose must stay outside executable/packaged inputs
    # and snapshot companions, regardless of the classifier's Markdown grammar.
    [[ "$path" != AGENTS.md && "$path" != TOKO-TOOLING.md && "$path" != releases/* && "$path" != crates/* && "$path" != apps/* ]]
    awk '/Shared companions:/ { exit 1 }' "$root/$path"
    awk -F '\t' -v path="$path" '$1 == "file" && $4 == path { found=1 } END { exit found ? 1 : 0 }' "$root/.toko-tooling.snapshot"
    mkdir -p "$(dirname "$fixture/repo/$path")"
    printf '# Prose fixture\n' > "$fixture/repo/$path"
done < "$root/ci/docs-only-paths.txt"
cat > "$fixture/bin/make" <<'MAKE'
#!/usr/bin/env bash
printf '%s\n' "$*" >> "$CI_SELECTION_EVENTS"
MAKE
chmod +x "$fixture/bin/make"
export CI_SELECTION_EVENTS="$fixture/events"
export GITHUB_STEP_SUMMARY="$fixture/summary"
export PATH="$fixture/bin:$PATH"
cd "$fixture/repo"
git init -q
git config user.name 'CI selection fixture'
git config user.email 'fixture@example.invalid'
git config core.hooksPath /dev/null
git config commit.gpgsign false
git add -- ci vendor README.md docs
git commit -qm 'Baseline policy and prose'
base="$(git rev-parse HEAD)"

run_case() {
    local label="$1" expected="$2" comparison_base="$3" tested_head="$4"
    local event="${5:-push}" ref="${6:-refs/heads/main}" status=0 scope selected
    : > "$fixture/$label.outputs"
    : > "$CI_SELECTION_EVENTS"
    CHANGE_BASE="$comparison_base" CHANGE_HEAD="$tested_head" CHANGE_EVENT="$event" CHANGE_REF="$ref" \
        GITHUB_OUTPUT="$fixture/$label.outputs" bash -e -o pipefail "$fixture/classify.sh" \
        > "$fixture/$label.log" 2>&1 || status=$?
    if [[ "$expected" == failure ]]; then
        [[ "$status" != 0 && ! -s "$fixture/$label.outputs" ]]
        status=0
        CLASSIFICATION_RESULT=failure bash -e -o pipefail "$fixture/admission.sh" || status=$?
        [[ "$status" != 0 && ! -s "$CI_SELECTION_EVENTS" ]]
    else
        [[ "$status" == 0 ]]
        scope="$(sed -n 's/^scope=//p' "$fixture/$label.outputs")"
        [[ "$scope" == "$expected" ]]
        CLASSIFICATION_RESULT=success bash -e -o pipefail "$fixture/admission.sh"
        selected="$(jq -er --arg scope "$scope" --arg docs "$docs_condition" --arg native "$native_condition" '
            [.[] | select(.id == "documentation" or .id == "validate") |
                select((.if == $docs and $scope == "docs") or (.if == $native and $scope != "docs"))] |
            if length == 1 then .[0].id else error("ambiguous validation branch") end
        ' "$fixture/validate-steps.json")"
        if [[ "$selected" == documentation ]]; then
            bash -e -o pipefail "$fixture/docs.sh"
            printf 'host-tools-check toko-tooling-check check-doc-links\n' > "$fixture/expected"
        else
            TMPDIR="$fixture" bash -e -o pipefail "$fixture/native.sh"
            printf 'ci\n' > "$fixture/expected"
        fi
        cmp "$fixture/expected" "$CI_SELECTION_EVENTS"
    fi
    printf 'PASS %s (%s)\n' "$label" "$expected"
}

while IFS= read -r path; do
    case "$path" in ''|'#'*) continue ;; esac
    git checkout -q --detach "$base"
    printf '\nReviewed prose.\n' >> "$path"
    git add -- "$path"
    git commit -qm 'Edit allowed prose'
    run_case "docs-$(printf '%s' "$path" | tr '/.' '__')" docs "$base" "$(git rev-parse HEAD)"
done < ci/docs-only-paths.txt
docs_head="$(git rev-parse HEAD)"

for kind in code mixed policy workflow packaged manifest lock private-manifest; do
    git checkout -q --detach "$base"
    case "$kind" in
        code|mixed) printf 'fn changed() {}\n' > code.rs ;;
        policy) printf '\n# Policy review changes require full CI.\n' >> ci/docs-only-paths.txt ;;
        workflow) mkdir -p .github/workflows; cp "$root/.github/workflows/ci.yml" .github/workflows/ci.yml ;;
        packaged) mkdir -p crates/ic-jobs; printf '# Embedded library docs\n' > crates/ic-jobs/README.md ;;
        manifest) cp "$root/Cargo.toml" Cargo.toml ;;
        lock) cp "$root/Cargo.lock" Cargo.lock ;;
        private-manifest)
            mkdir -p qualification/ic-timers-size-probe
            cp "$root/qualification/ic-timers-size-probe/Cargo.toml" qualification/ic-timers-size-probe/Cargo.toml ;;
    esac
    [[ "$kind" != mixed ]] || printf '\nProse alongside code.\n' >> README.md
    git add -- .
    git commit -qm "Change $kind inputs"
    run_case "$kind" full "$base" "$(git rev-parse HEAD)"
    [[ "$kind" != code ]] || code_head="$(git rev-parse HEAD)"
done

git checkout -q --detach "$docs_head"
run_case missing-history full 0000000000000000000000000000000000000000 "$docs_head"
run_case tag-ref full "$base" "$docs_head" push refs/tags/timers-v0.35.0
run_case divergent-push full "$code_head" "$docs_head"
git merge -q --no-ff "$code_head" -m 'Tested PR merge'
run_case tested-pr-merge docs "$code_head" "$(git rev-parse HEAD)" pull_request

git checkout -q --detach "$base"
printf 'Trailing whitespace. \n' >> README.md
git add -- README.md
git commit -qm 'Invalid prose whitespace'
run_case invalid-doc-whitespace failure "$base" "$(git rev-parse HEAD)"
for kind in failure invalid-output; do
    git checkout -q --detach "$base"
    if [[ "$kind" == failure ]]; then
        printf 'die "classifier execution failed\\n";\n' > vendor/toko-tooling/scripts/ci/classify-ci-change.pl
    else
        printf 'print "unknown\\n";\n' > vendor/toko-tooling/scripts/ci/classify-ci-change.pl
    fi
    git add -- vendor/toko-tooling/scripts/ci/classify-ci-change.pl
    git commit -qm 'Failed classification producer'
    run_case "classifier-$kind" failure "$base" "$(git rev-parse HEAD)"
done
fixture_complete=true
echo 'Actual workflow selection passed; Git/classifier/scripts are real, native setup and Make effects are substituted.'
