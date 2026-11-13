#!/usr/bin/env bash
# shellcheck disable=SC2030,SC2031 # Each qualification phase owns its subshell environment; only asserted files cross phases.
set -euo pipefail
fixture_mode="${1:-all}"
case "$fixture_mode" in all|retention|metadata|delivery|recovery|publication|publication-admission|frozen) ;; *) exit 2 ;; esac
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
export PATH="$root/.tools/host/bin:$PATH"
# shellcheck source=scripts/ci/fixture-scratch.sh
source "$root/scripts/ci/fixture-scratch.sh"
fixture_suite="$(fixture_create_workspace "$root" "scheduling-release-groups.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture_suite" "Release-group fixture retained"' EXIT
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset RELEASE_COMMIT RELEASE_VERSION RELEASE_PREVIOUS RELEASE_DATE RELEASE_PREPARATION_SOURCE RELEASE_GROUP PACKAGE
export GROUP_TEST_REAL_MAKE GROUP_TEST_REAL_EDITOR GROUP_TEST_REAL_CARGO
GROUP_TEST_REAL_MAKE="$(command -v make)"
GROUP_TEST_REAL_CARGO="$(command -v cargo)"
GROUP_TEST_REAL_EDITOR="${CARGO_RELEASE_METADATA_EDITOR:-}"
if [[ -z "$GROUP_TEST_REAL_EDITOR" && "$fixture_mode" != frozen && "$fixture_mode" != publication-admission ]]; then
    GROUP_TEST_REAL_EDITOR="$(make --silent --no-print-directory -C "$root" release-tools-check)"
fi
mkdir "$fixture_suite/source" "$fixture_suite/bin"
cp "$root"/{Cargo.toml,Cargo.lock,CHANGELOG.md,README.md,LICENSE.md,CONTRIBUTING.md,Makefile,.gitignore,.toko-tooling.snapshot,rust-toolchain.toml,TOKO-TOOLING.md,AGENTS.md} "$fixture_suite/source/"
tar -C "$root" --exclude=target -cf - crates qualification scripts vendor ci docs releases .github | tar -C "$fixture_suite/source" -xf -
if [[ "$fixture_mode" == frozen ]]; then
    # shellcheck source=scripts/release/test-frozen.sh
    source "$root/scripts/release/test-frozen.sh"
    qualify_frozen "$fixture_suite"
    fixture_complete=true
    exit 0
fi
cat > "$fixture_suite/bin/editor" <<'EDITOR'
#!/usr/bin/env bash
set -euo pipefail
if [[ ( "${GROUP_TEST_EDITOR_FAILURE:-}" == manifest && "$*" == *group-manifest* ) ||
      ( "${GROUP_TEST_EDITOR_FAILURE:-}" == lock && "$*" == *' lock '* ) ||
      ( "${GROUP_TEST_EDITOR_FAILURE:-}" == qualification-lock && "$*" == *' lock '* && "$*" == *'/qualification/Cargo.lock '* ) ]]; then
    echo 'partial invalid candidate'
    exit 37
fi
exec "$GROUP_TEST_REAL_EDITOR" "$@"
EDITOR
cat > "$fixture_suite/bin/make" <<'MAKE'
#!/usr/bin/env bash
set -euo pipefail
for arg in "$@"; do
    case "$arg" in
        clippy|clippy-jobs|clippy-timers)
            printf '%s\n' "$arg" >> "$GROUP_TEST_LINTS"
            [[ "${GROUP_TEST_LINT_FAILURE:-0}" == 0 ]] || exit 37
            "$GROUP_TEST_REAL_MAKE" "$@"
            if [[ "${GROUP_TEST_LINT_DRIFT:-0}" == 1 ]]; then
                printf '// drift during lint\n' >> crates/ic-jobs/src/lib.rs
            fi
            exit 0 ;;
        RELEASE_SOURCE=*|RELEASE_PREVIOUS=*|RELEASE_VERSION=*|RELEASE_DATE=*|RELEASE_REMOTE=*|RELEASE_BRANCH=*|RELEASE_COMMIT=*|RELEASE_GROUP=*|RELEASE_PREPARATION_SOURCE=*) export "$arg" ;;
    esac
done
for arg in "$@"; do
    case "$arg" in
        release-verify)
            printf '%s\n' "${RELEASE_GROUP:?}" >> "$GROUP_TEST_EVENTS"
            bash scripts/release/verify.sh
            exit 0 ;;
        ci) exit 0 ;;
        install-tools|tools-check|install-release-tools) exit 0 ;;
        release-tools-check) printf '%s\n' "$GROUP_TEST_EDITOR"; exit 0 ;;
        release-push-check)
            if [[ "${GROUP_TEST_PUSH_FAILURE:-0}" == 1 ]]; then exit 37; fi ;;
    esac
    # Product obligations come from the same policy as CI and release selection.
    if jq -e --arg target "$arg" '.stages | index($target) != null' ci/release-validation.json > /dev/null; then
        if [[ "$arg" == test-release-validation && -n "${SCHEDULING_VALIDATION_ATTEMPT:-}" ]]; then
            mkdir -p "$SCHEDULING_VALIDATION_ATTEMPT/fixture-stage"
            jq -cnS --arg source "$RELEASE_SOURCE" --arg group "$RELEASE_GROUP" \
                '{schema:1,source:$source,group:$group,mode:"ordinary",reason:"fixture substitutes product commands"}' \
                > "$SCHEDULING_VALIDATION_ATTEMPT/fixture-stage/decision.json"
        fi
        exit 0
    fi
done
exec "$GROUP_TEST_REAL_MAKE" "$@"
MAKE
cat > "$fixture_suite/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == clippy ]]; then exit 0; fi
if [[ "${1:-}" == publish || "${1:-}" == package ]]; then
    printf '%s\n' "$*" >> "$GROUP_TEST_UPLOADS"
    exit 0
fi
exec "$GROUP_TEST_REAL_CARGO" "$@"
CARGO
chmod +x "$fixture_suite/bin/"*
export PATH="$fixture_suite/bin:$PATH" GROUP_TEST_EDITOR="$fixture_suite/bin/editor"
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
# Phase functions run in subshells: fault injection and local fixture exports
# cannot become inputs to the next phase. Only their admitted filesystem effects
# (metadata, journals, local Git refs and logs) cross a phase boundary.
check_metadata_rollback() (
    local suite="$1" selected="$2" previous="$3" candidate="$4" base="$5"
    local receipts status failure
    bash scripts/release/verify.sh > "$suite/$selected-validation.log" 2>&1
    receipts=".git/release-state/$selected-$candidate.receipts"
    cp "$receipts/validated" "$suite/$selected-valid-receipt"
    printf 'tampered\n' >> "$receipts/validated"
    if bash scripts/release/metadata.sh prepare > "$suite/$selected-tampered-receipt.log" 2>&1; then exit 1; fi
    git diff --exit-code HEAD -- > /dev/null
    cp "$suite/$selected-valid-receipt" "$receipts/validated"
    for failure in manifest lock qualification-lock; do
        status=0
        GROUP_TEST_EDITOR_FAILURE="$failure" RELEASE_PREVIOUS="$previous" RELEASE_VERSION="$candidate" RELEASE_DATE=2026-10-31 \
            RELEASE_PREPARATION_SOURCE="$base" bash scripts/release/metadata.sh prepare > "$suite/$selected-rollback-$failure.log" 2>&1 || status=$?
        [[ "$status" != 0 ]] || exit 1
        git diff --exit-code HEAD -- > /dev/null
    done
)

interrupt_release() (
    local suite="$1" selected="$2" candidate="$3"
    local receipts status
    status=0
    GROUP_TEST_PUSH_FAILURE=1 make release-patch RELEASE_MAKE="$suite/bin/make" > "$suite/$selected-release.log" 2>&1 || status=$?
    [[ "$status" != 0 ]] || exit 1
    [[ "$(cat "$GROUP_TEST_EVENTS")" == "$selected" ]]
    [[ -f ".git/release-state/$selected-$candidate.plan" ]]
    receipts=".git/release-state/$selected-$candidate.receipts"
    [[ -f "$receipts/validated" && -f "$receipts/prepared" ]]
    grep -Fx "$selected" ".git/release-state/$selected-$candidate.plan" > /dev/null
    cmp "$suite/$selected-files" ".git/release-state/$selected-$candidate.plan.files"
)

resume_release() (
    local suite="$1" selected="$2" other_group="$3" candidate="$4" base="$5"
    local path before
    # Resume must not accept another group's saved intent.
    before="$(git rev-parse HEAD)"
    cp ".git/release-state/$selected-$candidate.plan" ".git/release-state/$other_group-$candidate.plan"
    if make release-resume RELEASE_GROUP="$other_group" VERSION="$candidate" RELEASE_MAKE="$suite/bin/make" > "$suite/$selected-wrong-resume.log" 2>&1; then exit 1; fi
    [[ "$(git rev-parse HEAD)" == "$before" ]]
    git diff --quiet HEAD --
    cmp ".git/release-state/$selected-$candidate.plan" ".git/release-state/$other_group-$candidate.plan"
    rm ".git/release-state/$other_group-$candidate.plan"
    make release-resume VERSION="$candidate" RELEASE_MAKE="$suite/bin/make" > "$suite/$selected-resume.log" 2>&1
    [[ "$(cat "$GROUP_TEST_EVENTS")" == "$selected" ]]
    git show --format= --name-only HEAD | LC_ALL=C sort > "$suite/$selected-committed"
    # The plan admits the shared root manifest even when this group
    # has no requirement to rewrite there. Only changed plan entries
    # belong in the commit; unchanged admitted files remain receipt-bound.
    : > "$suite/$selected-changed"
    while IFS= read -r -d '' path; do
        if [[ "$(git rev-parse "$base:$path")" != "$(git rev-parse "HEAD:$path")" ]]; then
            printf '%s\n' "$path" >> "$suite/$selected-changed"
        fi
    done < "$suite/$selected-files"
    LC_ALL=C sort "$suite/$selected-changed" > "$suite/$selected-expected"
    cmp "$suite/$selected-committed" "$suite/$selected-expected"
    [[ "$(git log -1 --format=%s)" == "Release $selected $candidate" ]]
    bash vendor/toko-tooling/scripts/ci/check-release-tag.sh "$(git rev-parse HEAD)" "$candidate" "$selected"
    [[ "$(git --git-dir="$suite/$selected-remote.git" rev-parse main)" == "$(git rev-parse HEAD)" ]]
)

check_publication() (
    local suite="$1" selected="$2" other_group="$3" candidate="$4" package="$5"
    # Substitute transport, keeping the real checker and snapshot bytes intact.
    local registry_bin="$suite/$selected-registry-bin"
    mkdir -p "$registry_bin"
    cat > "$registry_bin/curl" <<'REGISTRY'
#!/usr/bin/env bash
[[ "${@: -1}" == "https://crates.io/api/v1/crates/ic-$RELEASE_GROUP/$RELEASE_VERSION" ]] || exit 49
case "${GROUP_TEST_REGISTRY_STATUS:-1}" in
    0) printf 200 ;; 1) printf 404 ;; 2) printf 503 ;; *) exit 49 ;;
esac
REGISTRY
    chmod +x "$registry_bin/curl"
    export PATH="$registry_bin:$PATH"
    if GROUP_TEST_REGISTRY_STATUS=2 bash scripts/release/check-publication.sh origin > "$suite/$selected-unknown-registry.log" 2>&1; then exit 1; fi
    if RELEASE_GROUP="$other_group" bash scripts/release/check-publication.sh origin > "$suite/$selected-wrong-publication.log" 2>&1; then exit 1; fi
    touch unexpected-source
    if bash scripts/release/check-publication.sh origin > "$suite/$selected-dirty-publication.log" 2>&1; then exit 1; fi
    rm unexpected-source
    bash scripts/release/check-publication.sh origin
    : > "$GROUP_TEST_LINTS"
    bash scripts/release/publish.sh origin
    [[ "$(cat "$GROUP_TEST_LINTS")" == "$GROUP_TEST_EXPECTED_LINT" ]]
    [[ "$(cat "$GROUP_TEST_UPLOADS")" == "publish -p $package --all-features --locked --registry crates-io" ]]
    : > "$GROUP_TEST_UPLOADS"
    : > "$GROUP_TEST_LINTS"
    make publish-check > "$suite/$selected-publish-check.log" 2>&1
    [[ "$(cat "$GROUP_TEST_LINTS")" == "$GROUP_TEST_EXPECTED_LINT" ]]
    [[ "$(cat "$GROUP_TEST_UPLOADS")" == "publish -p $package --all-features --locked --registry crates-io --allow-dirty --dry-run" ]]
    # Both actual entrypoints stop before Cargo publication on failed lint or
    # clean-source drift. The dirty dry-run contract still retains full lint.
    local command fault
    for command in publish publish-check; do
        for fault in GROUP_TEST_LINT_FAILURE GROUP_TEST_LINT_DRIFT; do
            : > "$GROUP_TEST_UPLOADS"
            if env "$fault=1" "$GROUP_TEST_REAL_MAKE" --no-print-directory "$command" > "$suite/$selected-$command-$fault.log" 2>&1; then exit 1; fi
            [[ ! -s "$GROUP_TEST_UPLOADS" ]]
            git checkout -- crates/ic-jobs/src/lib.rs
        done
    done
    # Missing/stale evidence may only broaden; never trust a scope field without
    # its sealed source/configuration and exact prepared metadata.
    local state journal proof
    state=".git/release-state/$selected-$candidate.receipts"
    journal=".git/release-state/$selected-$candidate.plan"
    for proof in "$journal" "$state/validated" "$state/prepared" "$state/validation-evidence"; do
        [[ -f "$proof" ]] || continue
        cp "$proof" "$suite/saved-proof"
        if [[ "$proof" == "$journal" ]]; then rm "$proof"; else printf 'tampered\n' >> "$proof"; fi
        : > "$GROUP_TEST_LINTS"; : > "$GROUP_TEST_UPLOADS"
        make publish-check > "$suite/$selected-fallback-$(basename "$proof").log" 2>&1
        [[ "$(cat "$GROUP_TEST_LINTS")" == clippy ]]
        cp "$suite/saved-proof" "$proof"
    done
    touch unexpected-source
    : > "$GROUP_TEST_LINTS"; : > "$GROUP_TEST_UPLOADS"
    make publish-check > "$suite/$selected-dirty-dry-run.log" 2>&1
    [[ "$(cat "$GROUP_TEST_LINTS")" == clippy ]]
    rm unexpected-source
)

# Exercise public refusal before any tool, gate, journal or metadata effect.
check_refusals() (
    local suite="$1"
    cp -R "$suite/source" "$suite/refusals"
    cd "$suite/refusals"
    export GROUP_TEST_EVENTS="$suite/refusal-events" GROUP_TEST_UPLOADS="$suite/refusal-uploads"
    : > "$GROUP_TEST_EVENTS"; : > "$GROUP_TEST_UPLOADS"
    git init -q -b main
    git config user.name 'Scheduling fixture'; git config user.email 'fixture@example.invalid'
    git config core.hooksPath /dev/null
    git add -- .; git commit -qm baseline
    refuse_group() {
        local selection="$1" status=0
        shift
        case "$selection" in
            missing) env -u RELEASE_GROUP "$@" > "$suite/refusal.log" 2>&1 || status=$? ;;
            empty) RELEASE_GROUP='' "$@" > "$suite/refusal.log" 2>&1 || status=$? ;;
            unknown) RELEASE_GROUP=unknown "$@" > "$suite/refusal.log" 2>&1 || status=$? ;;
        esac
        [[ "$status" != 0 && ! -s "$GROUP_TEST_EVENTS" && ! -s "$GROUP_TEST_UPLOADS" ]] || exit 1
        grep -F RELEASE_GROUP "$suite/refusal.log" > /dev/null
        [[ ! -e .git/release-state && -z "$(git status --porcelain)" ]] || exit 1
    }
    for selection in missing empty unknown; do
        for target in release-patch release-minor release-major release-resume publish publish-check package; do
            refuse_group "$selection" "$GROUP_TEST_REAL_MAKE" --no-print-directory "$target"
        done
        refuse_group "$selection" bash scripts/release/metadata.sh preflight
        refuse_group "$selection" bash scripts/release/publish.sh origin
        refuse_group "$selection" bash scripts/release/check-publication.sh origin
    done
    for target in publish publish-check; do
        if RELEASE_GROUP=jobs PACKAGE=ic-timers "$GROUP_TEST_REAL_MAKE" "$target" > "$suite/subset-refusal.log" 2>&1; then exit 1; fi
        grep -F 'PACKAGE must belong' "$suite/subset-refusal.log" > /dev/null
    done
    [[ ! -e .git/release-state && -z "$(git status --porcelain)" ]] || exit 1
    echo 'Explicit group selection refuses missing, empty and unknown Make/CLI requests before effects'
)
qualify_group() (
    local suite="$1" selected="$2" mode="$3"
    local repo="$suite/$selected"
    local ledger other_ledger package other previous candidate other_group other_version
    local base plan
    cp -R "$suite/source" "$repo"
    cd "$repo"
    # These fixtures exercise the inner standard runner. The frozen fixture
    # separately qualifies outer batch capture and candidate hooks.
    export RELEASE_BATCH_ACTIVE=1 RELEASE_GROUP="$selected" RELEASE_MAKE="$suite/bin/make"
    export GROUP_TEST_EVENTS="$suite/$selected-gates" GROUP_TEST_UPLOADS="$suite/$selected-uploads"
    export GROUP_TEST_LINTS="$suite/$selected-lints" GROUP_TEST_EXPECTED_LINT=clippy
    [[ "$selected" != jobs || "$mode" == publication-admission ]] || GROUP_TEST_EXPECTED_LINT=clippy-jobs
    : > "$GROUP_TEST_EVENTS"; : > "$GROUP_TEST_UPLOADS"
    ledger=releases/jobs/CHANGELOG.md; other_ledger=releases/timers/CHANGELOG.md; package=ic-jobs; other=ic-timers
    if [[ "$selected" == timers ]]; then
        ledger=releases/timers/CHANGELOG.md; other_ledger=releases/jobs/CHANGELOG.md; package=ic-timers; other=ic-jobs
    fi
    previous="$(bash scripts/release/group.sh version)"
    candidate="$(bash vendor/toko-tooling/scripts/ci/next-release-version.sh "$previous" patch)"
    other_group=timers; [[ "$selected" != timers ]] || other_group='jobs'
    other_version="$(RELEASE_GROUP="$other_group" bash scripts/release/group.sh version)"
    printf '# Fixture ledger\n\n## [%s] - 2026-10-30\n\n### 🩹 Fixed\n\n- Qualified fixture baseline.\n' "$previous" > "$ledger"
    git init -q -b main
    git config user.name 'Scheduling fixture'; git config user.email 'fixture@example.invalid'
    git config core.hooksPath /dev/null
    git add -- .; git commit -qm baseline
    git tag -a "$selected-v$previous" -m 'Qualified fixture release boundary'
    if [[ "$mode" == publication-admission ]]; then
        export RELEASE_VERSION="$previous"
        git init -q --bare "$suite/$selected-remote.git"
        git remote add origin "$suite/$selected-remote.git"
        git push -q origin main "refs/tags/$selected-v$previous"
        check_publication "$suite" "$selected" "$other_group" "$previous" "$package"
        echo "$selected publication caller passed (real tooling/Git/tag admission; substitute transport and Cargo uploads)"
        exit 0
    fi
    printf '# Fixture ledger\n\n## [%s] - Unreleased\n\n### 🩹 Fixed\n\n- Fixture group preparation.\n' "$candidate" > "$ledger"
    git add -- "$ledger"; git commit -qm 'Pending group notes'
    base="$(git rev-parse HEAD)"
    export RELEASE_SOURCE="$base" RELEASE_PREVIOUS="$previous" RELEASE_VERSION="$candidate"
    export RELEASE_DATE=2026-10-31 RELEASE_REMOTE=origin RELEASE_BRANCH=main
    git init -q --bare "$suite/$selected-remote.git"
    git remote add origin "$suite/$selected-remote.git"
    git push -q origin main
    plan="$(bash scripts/release/group.sh plan)"
    [[ "$(printf '%s\n' "$plan" | jq -r .primary)" == "$package" ]]
    [[ "$(make -s release-version)" == "$previous" ]]
    printf '%s\n' "$plan" > "$suite/$selected-plan.json"
    bash scripts/release/group.sh files > "$suite/$selected-files"
    cp "$other_ledger" "$suite/$selected-other-ledger"
    cp CHANGELOG.md "$suite/$selected-index"
    cp releases/history.md "$suite/$selected-history"
    cargo metadata --locked --offline --format-version 1 > "$suite/$selected-before.json"
    cargo metadata --manifest-path qualification/Cargo.toml --locked --offline --format-version 1 > "$suite/$selected-qualification-before.json"
    yq -p toml -o json qualification/Cargo.lock | jq -S '[.package[] | select(.source != null)]' > "$suite/$selected-suppliers-before.json"
    case "$mode" in
        all|metadata|retention)
            check_metadata_rollback "$suite" "$selected" "$previous" "$candidate" "$base" ;;
    esac
    case "$mode" in
        metadata|retention)
            RELEASE_PREPARATION_SOURCE="$base" bash scripts/release/metadata.sh prepare > "$suite/$selected-prepare.log" 2>&1
            bash scripts/release/metadata.sh check ;;
        all|delivery|recovery|publication)
            interrupt_release "$suite" "$selected" "$candidate"
            resume_release "$suite" "$selected" "$other_group" "$candidate" "$base"
            check_publication "$suite" "$selected" "$other_group" "$candidate" "$package" ;;
    esac
    local validation_scope=workspace evidence
    [[ "$selected" != jobs ]] || validation_scope=group
    evidence="$(cat ".git/release-state/$selected-$candidate.receipts/validation-evidence")"
    [[ "$(jq -r .scope ".git/release-state/$selected-$candidate.receipts/$evidence/plan.json")" == "$validation_scope" ]]
    cmp "$other_ledger" "$suite/$selected-other-ledger"
    cmp CHANGELOG.md "$suite/$selected-index"
    cmp releases/history.md "$suite/$selected-history"
    cargo metadata --locked --offline --format-version 1 > "$suite/$selected-after.json"
    cargo metadata --manifest-path qualification/Cargo.toml --locked --offline --format-version 1 > "$suite/$selected-qualification-after.json"
    jq -n -e --slurpfile before "$suite/$selected-qualification-before.json" --slurpfile after "$suite/$selected-qualification-after.json" --arg selected "$package" --arg candidate "$candidate" '
        def private($m): [$m.packages[] | select(.id as $id | $m.workspace_members | index($id)) | {name,version}] | sort_by(.name);
        private($before[0]) == private($after[0]) and
        any($after[0].packages[]; .name == $selected and .source == null and .version == $candidate)
    ' >/dev/null
    yq -p toml -o json qualification/Cargo.lock | jq -S '[.package[] | select(.source != null)]' > "$suite/$selected-suppliers-after.json"
    cmp "$suite/$selected-suppliers-before.json" "$suite/$selected-suppliers-after.json"
    grep -Fx 'qualification/Cargo.toml' "$suite/$selected-files" >/dev/null
    grep -Fx 'qualification/Cargo.lock' "$suite/$selected-files" >/dev/null
    jq -e --arg other "$other" --arg version "$other_version" '.packages[] | select(.name == $other and .source == null) | .version == $version' "$suite/$selected-after.json" > /dev/null
    # Release membership comes from policy/metadata; preparation must not restore
    # display-only dependency aliases removed under the installed baseline.
    yq -p toml -o json Cargo.toml | jq -e '.workspace.dependencies | (has("ic-jobs") or has("ic-timers")) | not' >/dev/null
    if [[ "$selected" == timers ]]; then
        [[ "$(yq -p toml -o yaml -r '.package.version' crates/ic-jobs/Cargo.toml)" == "$other_version" ]]
        # The unchanged standalone Jobs consumer still builds against Jobs
        # its retained version after the independent Timers upgrade.
        CARGO_TARGET_DIR="$suite/jobs-consumer-target" bash scripts/dev/minimum-rust.sh cargo check \
            -p ic-jobs --example persisted_job --locked --offline
    fi
    printf '%s group %s passed (scope=%s; real Cargo metadata/editor/Git/plan receipts; product checks, setup and uploads substituted)\n' "$selected" "$mode" "$validation_scope"
)

if [[ "$fixture_mode" != publication-admission ]]; then check_refusals "$fixture_suite"; fi
for selected in jobs timers; do
    qualify_group "$fixture_suite" "$selected" "$fixture_mode"
done
if [[ "$fixture_mode" == all ]]; then
    # shellcheck source=scripts/release/test-frozen.sh
    source "$root/scripts/release/test-frozen.sh"
    qualify_frozen "$fixture_suite"
fi
fixture_complete=true
