#!/usr/bin/env bash

# Consumer qualification: real setup, metadata/editor, Git journals, archives and
# publication admission. Broad product stages and registry effects are replaced;
# focused installed checks below exercise the captured consumer sources directly.
qualify_frozen() (
    set -euo pipefail
    local suite="$1" normal="$1/frozen-normal" bin="$1/frozen-bin"
    local base state batch work delivered group version commit ledger candidate
    local original_lock status before installed_proof=''
    unset RELEASE_BATCH_ACTIVE RELEASE_SOURCE RELEASE_VERSION RELEASE_COMMIT
    unset RELEASE_PREPARATION_SOURCE RELEASE_GROUP PACKAGE
    cp -R "$suite/source" "$normal"
    if [[ -n "${FROZEN_TEST_LOCKFILE:-}" ]]; then cp "$FROZEN_TEST_LOCKFILE" "$normal/Cargo.lock"; fi
    mkdir -p "$bin" "$suite/frozen-publications"
    export FROZEN_TEST_NORMAL="$normal" FROZEN_TEST_SUITE="$suite"
    export FROZEN_TEST_REAL_MAKE="$GROUP_TEST_REAL_MAKE" FROZEN_TEST_REAL_CARGO="$GROUP_TEST_REAL_CARGO"
    FROZEN_TEST_REAL_CURL="$(command -v curl)"
    export FROZEN_TEST_REAL_CURL
    export RELEASE_MAKE="$bin/make"
    export PATH="$bin:$PATH"
    cat > "$bin/make" <<'MAKE'
#!/usr/bin/env bash
set -euo pipefail
for arg in "$@"; do
    case "$arg" in RELEASE_*=*) export "$arg" ;; esac
done
for arg in "$@"; do
    case "$arg" in
        release-verify)
            [[ "$PWD" != "$FROZEN_TEST_NORMAL" ]]
            yq -p toml -o json Cargo.lock | jq -S '[.package[] | select(.source != null)]' > "$FROZEN_TEST_SUITE/observed-graph.json"
            cmp "$FROZEN_TEST_SUITE/observed-graph.json" "$FROZEN_TEST_SUITE/frozen-graph.json"
            if [[ ! -f "$FROZEN_TEST_SUITE/normal-updated" ]]; then
                printf 'concurrent normal lock write in progress\n' > "$FROZEN_TEST_NORMAL/Cargo.lock"
                touch "$FROZEN_TEST_SUITE/normal-updated"
            fi
            if [[ "$RELEASE_GROUP" == jobs && -f "$FROZEN_TEST_SUITE/fail-jobs-gate" ]]; then
                rm "$FROZEN_TEST_SUITE/fail-jobs-gate"
                exit 37
            fi
            exec bash scripts/release/verify.sh ;;
        release-push-check)
            "$FROZEN_TEST_REAL_MAKE" "$@"
            if [[ "$RELEASE_GROUP" == timers && -f "$FROZEN_TEST_SUITE/fail-timers-push" ]]; then
                rm "$FROZEN_TEST_SUITE/fail-timers-push"
                echo 'Fixture: interrupted after tagged release, before push' >&2
                exit 37
            fi
            exit 0 ;;
        toko-tooling-check|clippy-jobs|clippy-timers|package-jobs|package-timers)
            exec "$FROZEN_TEST_REAL_MAKE" "$@" ;;
    esac
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
exec "$FROZEN_TEST_REAL_MAKE" "$@"
MAKE
    cat > "$bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" != publish ]]; then exec "$FROZEN_TEST_REAL_CARGO" "$@"; fi
[[ "$*" == "publish -p ic-$RELEASE_GROUP --all-features --locked --registry crates-io" ]]
[[ "$(git rev-parse HEAD)" == "${RELEASE_COMMIT:?saved delivered identity}" ]]
[[ "${RELEASE_REMOTE:?}" == selected && "${RELEASE_BRANCH:?}" == delivery ]]
[[ ! -e "$FROZEN_TEST_SUITE/frozen-publications/$RELEASE_GROUP.json" ]]
"$FROZEN_TEST_REAL_CARGO" package -p "ic-$RELEASE_GROUP" --all-features --locked --offline
archive="target/package/tmp-crate/ic-$RELEASE_GROUP-$RELEASE_VERSION.crate"
[[ -f "$archive" ]] || archive="target/package/ic-$RELEASE_GROUP-$RELEASE_VERSION.crate"
tar -xOf "$archive" "ic-$RELEASE_GROUP-$RELEASE_VERSION/.cargo_vcs_info.json" |
    jq -e --arg commit "$RELEASE_COMMIT" '.git.sha1 == $commit and ((.git.dirty // false) == false)'
checksum="$(bash vendor/toko-tooling/scripts/ci/verify-file-checksum.sh --print sha256 "$archive")"
jq -n --arg group "$RELEASE_GROUP" --arg version "$RELEASE_VERSION" --arg commit "$RELEASE_COMMIT" --arg checksum "$checksum" \
    '{group:$group,version:$version,commit:$commit,archive_sha256:$checksum}' > "$FROZEN_TEST_SUITE/frozen-publications/$RELEASE_GROUP.json"
printf '%s\n' "$RELEASE_GROUP" >> "$FROZEN_TEST_SUITE/frozen-uploads"
if [[ "$RELEASE_GROUP" == timers && -f "$FROZEN_TEST_SUITE/fail-timers-publication" ]]; then
    rm "$FROZEN_TEST_SUITE/fail-timers-publication"
    exit 37
fi
CARGO
    cat > "$bin/curl" <<'CURL'
#!/usr/bin/env bash
set -euo pipefail
for arg in "$@"; do
    case "$arg" in
        https://crates.io/api/v1/crates/ic-jobs/*|https://crates.io/api/v1/crates/ic-timers/*)
            printf '404'
            exit 0 ;;
    esac
done
exec "$FROZEN_TEST_REAL_CURL" "$@"
CURL
    chmod +x "$bin/make" "$bin/cargo" "$bin/curl"
    # Only the registry observation/upload is simulated. Real source/tag/remote
    # admission, native/Wasm lint and selected archive verification stay active.
    cd "$normal"
    # This fixture deliberately releases patches. Its ledgers must not inherit
    # an unrelated product minor/major draft from the source being qualified.
    for group in timers jobs; do
        version="$(RELEASE_GROUP="$group" bash scripts/release/group.sh version)"
        ledger="$(RELEASE_GROUP="$group" bash scripts/release/group.sh plan | jq -er .changelog)"
        candidate="$(bash vendor/toko-tooling/scripts/ci/next-release-version.sh "$version" patch)"
        printf '# Fixture ledger\n\n## [%s] - Unreleased\n\n### 🧪 Testing\n\n- Captured patch delivery.\n\n## [%s] - 2026-10-30\n\n### 🧪 Testing\n\n- Qualified fixture baseline.\n' \
            "$candidate" "$version" > "$ledger"
    done
    git init -q -b delivery
    git config user.name 'Scheduling frozen fixture'
    git config user.email 'fixture@example.invalid'
    git config core.hooksPath /dev/null
    git add -- .
    git commit -qm 'Scheduling frozen consumer fixture'
    base="$(git rev-parse HEAD)"
    original_lock="$(git hash-object Cargo.lock)"
    yq -p toml -o json Cargo.lock | jq -S '[.package[] | select(.source != null)]' > "$suite/frozen-graph.json"
    for group in timers jobs; do
        version="$(RELEASE_GROUP="$group" bash scripts/release/group.sh version)"
        git tag -a "$group-v$version" -m 'Fixture release boundary'
    done
    git init -q --bare "$suite/frozen-remote.git"
    git remote add selected "$suite/frozen-remote.git"
    git push -q selected delivery
    # Invalid local selections must fail before creating batch state.
    for group in 'jobs,jobs' 'jobs,unknown' 'jobs,' ',jobs' 'jobs,,timers' 'jobs timers'; do
        if "$GROUP_TEST_REAL_MAKE" --no-print-directory release-patch "RELEASE_GROUPS=$group" > "$suite/frozen-refusal.log" 2>&1; then exit 1; fi
        [[ ! -e .git/release-state/frozen ]]
    done
    if "$GROUP_TEST_REAL_MAKE" --no-print-directory release-patch RELEASE_GROUP=jobs RELEASE_GROUPS= > "$suite/frozen-no-live-fallback.log" 2>&1; then exit 1; fi
    [[ ! -e .git/release-state/frozen ]]
    touch "$suite/fail-timers-push" "$suite/fail-timers-publication" "$suite/fail-jobs-gate"
    status=0
    "$GROUP_TEST_REAL_MAKE" --no-print-directory release-patch RELEASE_GROUPS=timers,jobs \
        RELEASE_PUBLISH_TARGET=publish RELEASE_REMOTE=selected RELEASE_BRANCH=delivery > "$suite/frozen-first.log" 2>&1 || status=$?
    [[ "$status" != 0 ]]
    state="$(printf '%s\n' "$normal"/.git/release-state/frozen/batch-*/intent.json)"
    [[ -f "$state" && "$(jq -r .phase "$state")" == releasing && "$(jq -r .index "$state")" == 0 ]]
    [[ "$(jq -r .schema "$state")" == 2 ]]
    batch="${state%/intent.json}"; batch="${batch##*/}"
    work="$normal/target/release-frozen/$batch"
    [[ ! -e "$suite/fail-timers-push" ]]
    version="$(jq -r .candidate "$state")"
    [[ -f "$(git -C "$work" rev-parse --git-path "release-state/timers-$version.plan")" ]]
    [[ "$(git rev-parse HEAD)" == "$base" ]]
    [[ "$(cat Cargo.lock)" == 'concurrent normal lock write in progress' ]]
    [[ "$(git --git-dir="$suite/frozen-remote.git" rev-parse delivery)" == "$base" ]]
    [[ "$(git -C "$work" config --get core.hooksPath)" == vendor/toko-tooling/.githooks ]]
    (
        cd "$work"
        export RELEASE_GROUP=timers RELEASE_VERSION="$version"
        RELEASE_SOURCE="$(jq -r .group_source "$state")"
        export RELEASE_SOURCE
        local receipts
        receipts="$(git rev-parse --git-path "release-state/timers-$version.receipts")"
        perl scripts/release/validation.pl identity "$receipts" > "$suite/frozen-receipt-identity"
        cp -R "$receipts" "$suite/foreign-receipts"
        if perl scripts/release/validation.pl identity "$suite/foreign-receipts" > "$suite/foreign-receipt.log" 2>&1; then exit 1; fi
        mv "$receipts/validation-evidence" "$suite/receipt-pointer"
        ln -s "$suite/receipt-pointer" "$receipts/validation-evidence"
        if perl scripts/release/validation.pl identity "$receipts" > "$suite/symlink-receipt.log" 2>&1; then exit 1; fi
        rm "$receipts/validation-evidence"
        mv "$suite/receipt-pointer" "$receipts/validation-evidence"
        perl scripts/release/validation.pl identity "$receipts" > "$suite/frozen-receipt-after-refusals"
        cmp "$suite/frozen-receipt-identity" "$suite/frozen-receipt-after-refusals"
    )
    # Preserve tagged/committed recovery, then simulate an uncertain registry
    # response after recording one validated archive. No automatic upload retry.
    status=0
    "$GROUP_TEST_REAL_MAKE" --no-print-directory release-batch-resume "BATCH=$batch" > "$suite/frozen-publish-interrupt.log" 2>&1 || status=$?
    [[ "$status" != 0 && "$(jq -r .phase "$state")" == publishing ]]
    [[ "$(jq -r .release_commit "$state")" == "$(git -C "$work" rev-parse HEAD)" ]]
    [[ "$(cat "$suite/frozen-uploads")" == timers ]]
    before="$(cat "$suite/frozen-uploads")"
    if "$GROUP_TEST_REAL_MAKE" --no-print-directory release-batch-resume "BATCH=$batch" > "$suite/frozen-uncertain.log" 2>&1; then exit 1; fi
    [[ "$(cat "$suite/frozen-uploads")" == "$before" ]]
    status=0
    "$GROUP_TEST_REAL_MAKE" --no-print-directory release-batch-resume "BATCH=$batch" PUBLICATION_RESULT=published > "$suite/frozen-jobs-interrupt.log" 2>&1 || status=$?
    [[ "$status" != 0 && "$(jq -r .index "$state")" == 1 && "$(jq -r .phase "$state")" == releasing ]]
    "$GROUP_TEST_REAL_MAKE" --no-print-directory release-batch-resume "BATCH=$batch" > "$suite/frozen-complete.log" 2>&1
    [[ "$(jq -r .phase "$state")" == complete ]]
    [[ "$(cat "$suite/frozen-uploads")" == $'timers\njobs' ]]
    "$GROUP_TEST_REAL_MAKE" --no-print-directory release-batch-resume "BATCH=$batch" > "$suite/frozen-complete-resume.log" 2>&1
    [[ "$(cat "$suite/frozen-uploads")" == $'timers\njobs' ]]
    [[ "$(git rev-parse HEAD)" == "$base" && "$(cat Cargo.lock)" == 'concurrent normal lock write in progress' ]]
    delivered="$(jq -r .commit "$state")"
    [[ "$(git --git-dir="$suite/frozen-remote.git" rev-parse delivery)" == "$delivered" ]]
    # Run focused live subjects from the delivered candidate, using its own tools,
    # source paths, compiler and artifacts rather than the normal checkout.
    if [[ "${FROZEN_TEST_INSTALLED:-0}" == 1 ]]; then
        installed_proof=' and selected installed Timers/Jobs subjects'
        (
        cd "$work"
        "$GROUP_TEST_REAL_MAKE" --no-print-directory install-testkit-tools install-consumer-server
        local cli server
        cli="$(bash scripts/dev/testkit-tool.sh --check)"; server="$("$cli" check)"
        POCKET_IC_BIN="$server" CARGO_NET_OFFLINE=true bash scripts/dev/minimum-rust.sh cargo run --manifest-path qualification/Cargo.toml -p ic-timers-pocketic --locked --offline -- focused \
            tests::lifecycle::shared_watchdog_survives_trap_and_upgrade \
            tests::ingress::typed_entrypoints_reject_invalid_wire_before_mutating_timer_state
        "$GROUP_TEST_REAL_MAKE" --no-print-directory build-consumer
        local target
        target="$("$FROZEN_TEST_REAL_CARGO" metadata --manifest-path qualification/Cargo.toml --no-deps --locked --offline --format-version 1 | jq -er .target_directory)"
        IC_JOBS_CONSUMER_WASM="$target/wasm32-unknown-unknown/release/ic_jobs_test_consumer.wasm" \
            "$cli" run --ttl 900 -- bash scripts/dev/minimum-rust.sh cargo test --manifest-path qualification/Cargo.toml -p ic-jobs-canister-tests --test recovery --features harness --locked --offline -- \
            --ignored --exact management_upgrade_and_real_watchdog_delivery --nocapture
    ) > "$suite/frozen-installed.log" 2>&1; fi
    # Reconciliation preserves dirty normal work. Explicit fixture-only cleanup
    # then permits fast-forward/retirement and proves release refs survive it.
    if "$GROUP_TEST_REAL_MAKE" --no-print-directory release-batch-reconcile "BATCH=$batch" > "$suite/frozen-dirty-reconcile.log" 2>&1; then exit 1; fi
    [[ "$(cat Cargo.lock)" == 'concurrent normal lock write in progress' ]]
    git restore -- Cargo.lock
    [[ "$(git hash-object Cargo.lock)" == "$original_lock" ]]
    "$GROUP_TEST_REAL_MAKE" --no-print-directory release-batch-reconcile "BATCH=$batch" > "$suite/frozen-reconcile.log" 2>&1
    [[ "$(git rev-parse HEAD)" == "$delivered" ]]
    # Shared retirement preserves ignored paths outside its root build/tool trees.
    # These qualification artifacts belong only to this disposable test. Ask
    # Cargo to clean its explicit cache; never relax production retirement or
    # remove a real release's build/evidence directories as a side effect.
    if [[ -d "$work/qualification/target" ]]; then
        [[ ! -L "$work/qualification/target" ]]
        "$FROZEN_TEST_REAL_CARGO" clean --manifest-path "$work/qualification/Cargo.toml" \
            --target-dir "$work/qualification/target" > "$suite/frozen-qualification-clean.log" 2>&1
    fi
    "$GROUP_TEST_REAL_MAKE" --no-print-directory release-batch-retire "BATCH=$batch" > "$suite/frozen-retire.log" 2>&1
    [[ ! -e "$work" && ! -e "${state%/intent.json}" ]]
    for group in timers jobs; do
        version="$(jq -r .version "$suite/frozen-publications/$group.json")"
        commit="$(jq -r .commit "$suite/frozen-publications/$group.json")"
        [[ "$(git rev-parse "$group-v$version^{commit}")" == "$commit" ]]
        cat "$suite/frozen-publications/$group.json"
    done
    echo "Frozen consumer hooks passed: real setup/format hooks, metadata/editor/journals, native/Wasm lint, verified archives$installed_proof; concurrent normal lock isolation, interrupted release/uncertain publication/second-group recovery, completed resume, reconciliation and retirement (broad product stages and uploads substituted)"
)
