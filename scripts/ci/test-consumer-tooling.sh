#!/usr/bin/env bash
set -euo pipefail
# With no arguments, retain the complete CI fixture. Local changes can select
# only their affected caller phases, without replaying installer/hosted suites.
phases=("$@")
if [[ $# == 0 ]]; then
    phases=(workspace-inheritance candid-boundary fixture-scratch batch-commands deps-update tooling-admission cli-admission server-commands workflow-selection input-admission artifact-capture ci-admission rust-policy lint-policy group-recipes)
fi
for phase in "${phases[@]}"; do
    case "$phase" in
        workspace-inheritance|candid-boundary|fixture-scratch|batch-commands|deps-update|tooling-admission|cli-admission|server-commands|workflow-selection|input-admission|artifact-capture|ci-admission|rust-policy|lint-policy|group-recipes) ;;
        *) echo "unknown consumer tooling phase: $phase" >&2; exit 2 ;;
    esac
done
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES

root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
export PATH="$root/.tools/host/bin:$PATH"
JOBS_CONSUMER_RUST_VERSION="$(bash "$root/scripts/dev/minimum-rust.sh" --print)"
export JOBS_CONSUMER_RUST_VERSION
# shellcheck source=scripts/ci/fixture-scratch.sh
source "$root/scripts/ci/fixture-scratch.sh"
fixture="$(fixture_create_workspace "$root" "jobs-consumer-tooling.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Consumer tooling fixture retained"' EXIT

# Each phase owns its files and exported environment. Only this outer fixture
# owns cleanup; a failed subshell retains the complete suite for inspection.
suite="$fixture"
prepare_phase() {
    fixture="$suite/$1"
    mkdir -p "$fixture/scripts/dev" "$fixture/scripts/ci" "$fixture/.tools/rust/common/bin"
    bash "$root/vendor/toko-tooling/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture" \
        vendor/toko-tooling/make/consumer.mk vendor/toko-tooling/make/cargo-dependencies.mk \
        vendor/toko-tooling/make/release.mk vendor/toko-tooling/make/cargo-publication.mk \
        vendor/toko-tooling/scripts/ci/fixture-lifecycle.sh
    cp "$root/Makefile" "$fixture/"
    cp -R "$root/ci" "$fixture/"
    cp "$root/scripts/dev/testkit-tool.sh" "$fixture/scripts/dev/"
    cp "$root/scripts/dev/minimum-rust.sh" "$fixture/scripts/dev/"
    cp "$root/vendor/toko-tooling/scripts/ci/read-cargo-workspace-rust-version.sh" "$fixture/vendor/toko-tooling/scripts/ci/"
    cp "$root/Cargo.toml" "$root/rust-toolchain.toml" "$fixture/"
    cp "$root/scripts/ci/test-canister.sh" "$fixture/scripts/ci/"
    cp "$root/Cargo.lock" "$fixture/"
    cp "$root/"{LICENSE.md,CONTRIBUTING.md,README.md} "$fixture/"
    mkdir -p "$fixture/qualification"
    cp "$root/qualification/"{Cargo.toml,Cargo.lock} "$fixture/qualification/"
    cp "$root/scripts/ci/check-qualification-tooling.sh" "$fixture/scripts/ci/"
    export JOBS_CONSUMER_TOOLING_EVENTS="$fixture/events"
    : > "$fixture/events"
}

stub_testkit_selection() {
    export JOBS_CONSUMER_ADMISSION_FAILURE=0
cat > "$fixture/vendor/toko-tooling/scripts/dev/install-rust-tools.sh" <<'INSTALLER'
#!/usr/bin/env bash
set -euo pipefail
[[ "$1" == --consumer && "$2" == "$PWD" ]] || exit 49
shift 2
case "$*" in
    '--package ic-testkit --lockfile qualification/Cargo.lock --bin ic-testkit-server --profile release --features cli')
        printf 'setup\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS" ;;
    '--package ic-testkit --lockfile qualification/Cargo.lock --bin ic-testkit-server --profile release --features cli --check')
        printf 'admission\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"
        echo 'selected CLI admission diagnostic' >&2
        [[ "$JOBS_CONSUMER_ADMISSION_FAILURE" == 0 ]] || exit 37
        printf '%s\n' "$JOBS_CONSUMER_CLI" ;;
    *) exit 49 ;;
esac
INSTALLER
}

stub_target_directory() {
cat > "$fixture/.tools/rust/common/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
printf 'metadata\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"
printf '{"target_directory":"%s"}\n' "$JOBS_CONSUMER_WASM_TARGET"
CARGO
chmod +x "$fixture/.tools/rust/common/bin/cargo"
}

phase_workspace_inheritance() (
    prepare_phase workspace-inheritance
    tar -C "$root" --exclude=target -cf - crates qualification | tar -C "$fixture" -xf -
    mkdir -p "$fixture/releases/jobs" "$fixture/releases/timers"
    cp "$root/releases/jobs/CHANGELOG.md" "$fixture/releases/jobs/"
    cp "$root/releases/timers/CHANGELOG.md" "$fixture/releases/timers/"
    bash "$root/vendor/toko-tooling/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture" vendor/toko-tooling/scripts/ci/check-dependency-pins.sh
    cp "$root/AGENTS.md" "$fixture/"
    git init -q "$fixture"
    git -C "$fixture" add -- AGENTS.md Cargo.lock qualification/Cargo.lock
    make --no-print-directory -C "$fixture" check-pins > "$fixture/valid.log" 2>&1
    for invalid in product-version missing-private-version private-dependency target-dependency published-qualification; do
        cp "$root/crates/ic-timers/Cargo.toml" "$fixture/crates/ic-timers/Cargo.toml"
        cp "$root/qualification/ic-jobs-canister-tests/Cargo.toml" "$fixture/qualification/ic-jobs-canister-tests/Cargo.toml"
        case "$invalid" in
            product-version) perl -pi -e 's/^version = "[^"]+"/version = "0.0.0"/' "$fixture/crates/ic-timers/Cargo.toml" ;;
            missing-private-version) perl -ni -e 'print unless /^version = /' "$fixture/qualification/ic-jobs-canister-tests/Cargo.toml" ;;
            private-dependency) perl -pi -e 's/^toko-tooling.workspace = true/toko-tooling = "=0.42.5"/' "$fixture/qualification/ic-jobs-canister-tests/Cargo.toml" ;;
            target-dependency) printf '\n%s\n%s\n' "[target.'cfg(unix)'.dev-dependencies]" 'serde = "1.0"' >> "$fixture/qualification/ic-jobs-canister-tests/Cargo.toml" ;;
            published-qualification) perl -pi -e 's/^publish = false/publish = ["crates-io"]/' "$fixture/qualification/ic-jobs-canister-tests/Cargo.toml" ;;
        esac
        if make --no-print-directory -C "$fixture" check-pins > "$fixture/$invalid.log" 2>&1; then
            echo "workspace inheritance admitted $invalid" >&2; exit 1
        fi
        [[ -s "$fixture/$invalid.log" ]]
        case "$invalid" in
            private-dependency|target-dependency) grep -F 'package dependencies must inherit workspace.dependencies' "$fixture/$invalid.log" >/dev/null ;;
        esac
    done
    echo 'Actual shared inheritance caller admits independent private versions and refuses stale products, missing versions, independent/target dependencies and publishable qualification'
)

phase_deps_update() (
    local fixture status
    trap - EXIT
    prepare_phase deps-update
    cat > "$fixture/.tools/rust/common/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\0' "$PWD" "$@" >> "$JOBS_CONSUMER_TOOLING_EVENTS"
exit "${JOBS_DEPENDENCY_UPDATE_FAILURE:-0}"
CARGO
    chmod +x "$fixture/.tools/rust/common/bin/cargo"
    PATH="$fixture/.tools/rust/common/bin:$PATH" make --no-print-directory -C "$fixture" deps-update \
        DEPENDENCY_PACKAGE=toko-tooling DEPENDENCY_PRECISE=0.42.5 > "$fixture/update.log" 2>&1
    printf '%s\0' "$fixture" update -v --manifest-path "$fixture/Cargo.toml" \
        --package toko-tooling --precise 0.42.5 \
        "$fixture/qualification" update -v --manifest-path "$fixture/qualification/Cargo.toml" \
        --package toko-tooling --precise 0.42.5 > "$fixture/expected"
    cmp "$fixture/events" "$fixture/expected"
    : > "$fixture/events"
    status=0
    PATH="$fixture/.tools/rust/common/bin:$PATH" JOBS_DEPENDENCY_UPDATE_FAILURE=37 \
        make --no-print-directory -C "$fixture" deps-update > "$fixture/failure.log" 2>&1 || status=$?
    [[ "$status" == 2 ]]
    printf '%s\0' "$fixture" update -v --manifest-path "$fixture/Cargo.toml" > "$fixture/expected"
    cmp "$fixture/events" "$fixture/expected"
    echo 'Actual deps-update caller updates root then qualification with explicit selectors and stops after Cargo failure (substitute Cargo; real locks unchanged)'
)

phase_tooling_admission() (
local fixture
trap - EXIT
prepare_phase tooling-admission
cp "$root/.toko-tooling.snapshot" "$fixture/"
# Admit the actual member manifests and receipt, without compiling product code.
tar -C "$root" --exclude=target -cf - crates qualification | tar -C "$fixture" -xf -
while IFS=$'\t' read -r kind _ _ path; do
    [[ "$kind" == file ]] || continue
    [[ "$path" != */* ]] || mkdir -p "$fixture/${path%/*}"
    cp -p "$root/$path" "$fixture/$path"
done < "$root/.toko-tooling.snapshot"
mkdir -p "$fixture/scripts/release"
cp "$root/scripts/release/"{group,publish,candidate-setup,metadata}.sh "$fixture/scripts/release/"
make --no-print-directory -C "$fixture" toko-tooling-check > "$fixture/admitted.log" 2>&1
# Cargo metadata is a required read for member admission. Refuse compilation,
# resolution, Git and network effects through the actual consumer entrypoints.
local -x JOBS_CONSUMER_REAL_CARGO
JOBS_CONSUMER_REAL_CARGO="$(command -v cargo)"
cat > "$fixture/.tools/rust/common/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == metadata && " $* " == *' --locked '* && " $* " == *' --offline '* && " $* " == *' --no-deps '* ]]; then
    exec "$JOBS_CONSUMER_REAL_CARGO" "$@"
fi
printf 'cargo %s\n' "$*" >> "$JOBS_CONSUMER_TOOLING_EVENTS"
exit 49
CARGO
chmod +x "$fixture/.tools/rust/common/bin/cargo"
for tool in git curl rustc; do
    cat > "$fixture/.tools/rust/common/bin/$tool" <<'EFFECT'
#!/usr/bin/env bash
printf '%s\n' "${0##*/} $*" >> "$JOBS_CONSUMER_TOOLING_EVENTS"
exit 49
EFFECT
    chmod +x "$fixture/.tools/rust/common/bin/$tool"
done
PATH="$fixture/.tools/rust/common/bin:$PATH" make --no-print-directory -C "$fixture" toko-tooling-check > "$fixture/admitted-with-effect-guards.log" 2>&1
[[ ! -s "$fixture/events" ]]
for invalid in missing-lock version-drift checksum-drift missing-member independent-member missing-product-member qualification-version-drift qualification-checksum-drift; do
    cp "$root/"{Cargo.toml,Cargo.lock} "$fixture/"
    cp "$root/qualification/"{Cargo.toml,Cargo.lock} "$fixture/qualification/"
    cp "$root/crates/ic-jobs/Cargo.toml" "$fixture/crates/ic-jobs/Cargo.toml"
    cp "$root/qualification/ic-timers-size-probe/Cargo.toml" "$fixture/qualification/ic-timers-size-probe/Cargo.toml"
    case "$invalid" in
        missing-product-member)
            perl -0pi -e 's/^toko-tooling\.workspace = true\n//m' "$fixture/crates/ic-jobs/Cargo.toml" ;;
        qualification-version-drift)
            perl -0pi -e 's/(toko-tooling = ")=[^"]+/${1}=0.0.0/' "$fixture/qualification/Cargo.toml" ;;
        qualification-checksum-drift)
            perl -0pi -e 's/(name = "toko-tooling"\nversion = "[^"]+"\nsource = "[^"]+"\nchecksum = )"[^"]+"/${1}"0000000000000000000000000000000000000000000000000000000000000000"/' "$fixture/qualification/Cargo.lock" ;;
        missing-lock) rm "$fixture/Cargo.lock" ;;
        version-drift)
            perl -0pi -e 's/(toko-tooling = ")=[^"]+/${1}=0.0.0/' "$fixture/Cargo.toml" ;;
        checksum-drift)
            perl -0pi -e 's/(name = "toko-tooling"\nversion = "[^"]+"\nsource = "[^"]+"\nchecksum = )"[^"]+"/${1}"0000000000000000000000000000000000000000000000000000000000000000"/' "$fixture/Cargo.lock" ;;
        missing-member)
            perl -0pi -e 's/^toko-tooling\.workspace = true\n//m' "$fixture/qualification/ic-timers-size-probe/Cargo.toml" ;;
        independent-member)
            perl -0pi -e 's/toko-tooling\.workspace = true/toko-tooling = "=0.42.5"/' "$fixture/qualification/ic-timers-size-probe/Cargo.toml" ;;
    esac
    for target in ci release-patch release-preflight publish publish-check clean-empty-source-dirs; do
        status=0
        PATH="$fixture/.tools/rust/common/bin:$PATH" RELEASE_GROUP="jobs" make --no-print-directory -C "$fixture" "$target" \
            > "$fixture/$invalid-$target.log" 2>&1 || status=$?
        [[ "$status" == 2 && ! -s "$fixture/events" ]] || exit 1
        grep -E 'toko-tooling-check|check-qualification-tooling' "$fixture/$invalid-$target.log" > /dev/null
    done
    status=0
    PATH="$fixture/.tools/rust/common/bin:$PATH" RELEASE_GROUP="jobs" bash "$fixture/scripts/release/publish.sh" origin \
        > "$fixture/$invalid-direct-publish.log" 2>&1 || status=$?
    [[ "$status" != 0 && ! -s "$fixture/events" ]] || exit 1
    grep -E 'toko-tooling-check|check-qualification-tooling' "$fixture/$invalid-direct-publish.log" > /dev/null
done
echo 'Cargo-owned Tooling admission rejects lock/pin/receipt drift and non-default member omissions/overrides before compiler, Git or network effects (56 actual caller routes across product and qualification)'
)

phase_cli_admission() (
local fixture
trap - EXIT
prepare_phase cli-admission
# Substitute only compiler identity probes. Any Cargo effect remains forbidden;
# the real installer/parser must refuse the isolated, unprepared source cache.
mkdir -p "$fixture/compiler/bin"
(umask 077; mkdir -p "$fixture/store/cargo-cache")
export TOKO_TOOLING_STORE="$fixture/store" JOBS_CONSUMER_COMPILER="$fixture/compiler"
export CARGO_NET_OFFLINE=true
cat > "$fixture/.tools/rust/common/bin/rustc" <<'RUSTC'
#!/usr/bin/env bash
case "$*" in
    '--print sysroot') printf '%s\n' "$JOBS_CONSUMER_COMPILER" ;;
    -vV) printf 'rustc fixture\nhost: fixture-host\nrelease: %s\n' "$JOBS_CONSUMER_RUST_VERSION" ;;
    *) printf 'unexpected rustc dispatch\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"; exit 49 ;;
esac
RUSTC
cat > "$fixture/.tools/rust/common/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
if [[ "$*" == -vV ]]; then printf 'cargo fixture\n'; exit 0; fi
printf '%s\n' "$*" >> "$JOBS_CONSUMER_TOOLING_EVENTS"
echo 'unexpected Cargo dispatch before CLI admission' >&2
exit 49
CARGO
chmod +x "$fixture/.tools/rust/common/bin/"{cargo,rustc}
cp "$fixture/.tools/rust/common/bin/"{cargo,rustc} "$fixture/compiler/bin/"

# Use the real TOML reader, shared installer and actual Jobs Makefile/wrapper.
# Both selections are deliberately unprepared; neither may fall back to a pin
# or dispatch compilation/installation after offline admission fails.
for version in 0.0.1 0.0.2; do
    cat > "$fixture/qualification/Cargo.lock" <<LOCK
version = 4

[[package]]
name = "ic-testkit"
version = "$version"
source = "registry+https://github.com/rust-lang/crates.io-index"
LOCK
    cp "$fixture/qualification/Cargo.lock" "$fixture/expected.lock"
    for target in testkit-tools-check consumer-server-check test-canister test-canister-optimized; do
        status=0
        make --no-print-directory -j2 -C "$fixture" "$target" \
            > "$fixture/$version-$target.log" 2>&1 || status=$?
        [[ "$status" == 2 ]] || exit 1
        grep -F "ic-testkit $version" \
            "$fixture/$version-$target.log" > /dev/null
        [[ ! -s "$fixture/events" ]] || exit 1
        cmp "$fixture/qualification/Cargo.lock" "$fixture/expected.lock"
    done
    for mode in original optimized; do
        args=()
        [[ "$mode" != optimized ]] || args=(--optimized)
        status=0
        (cd "$fixture" && PATH="$fixture/.tools/rust/common/bin:$PATH" \
            "$BASH" scripts/ci/test-canister.sh ${args[@]+"${args[@]}"}) \
            > "$fixture/$version-wrapper-$mode.log" 2>&1 || status=$?
        [[ "$status" == 1 ]] || exit 1
        grep -F "ic-testkit $version" \
            "$fixture/$version-wrapper-$mode.log" > /dev/null
        [[ ! -s "$fixture/events" ]] || exit 1
        cmp "$fixture/qualification/Cargo.lock" "$fixture/expected.lock"
    done
done
echo 'Lock-selected consumer CLI refusal precedes Cargo/PocketIC effects (real caller/parser; substitute compiler identity; no installation)'
)

phase_server_commands() (
local fixture
trap - EXIT
prepare_phase server-commands
stub_testkit_selection
# shellcheck disable=SC2030,SC2031 # Stub inputs intentionally stay inside this phase.
local -x JOBS_CONSUMER_CLI="$fixture/server"
cat > "$fixture/server" <<'SERVER'
#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ( "$1" == setup || "$1" == check ) ]] || exit 49
printf '%s\n' "$1" >> "$JOBS_CONSUMER_TOOLING_EVENTS"
SERVER
chmod +x "$fixture/server"
for command in setup check; do
    target=consumer-server-check
    [[ "$command" != setup ]] || target=install-consumer-server
    : > "$fixture/events"
    make --no-print-directory -j2 -C "$fixture" "$target" > "$fixture/server-$command.log" 2>&1
    printf 'admission\n%s\n' "$command" > "$fixture/expected-events"
    cmp "$fixture/events" "$fixture/expected-events"
    : > "$fixture/events"
    status=0
    JOBS_CONSUMER_ADMISSION_FAILURE=1 make --no-print-directory -j2 -C "$fixture" "$target" \
        > "$fixture/server-$command-refused.log" 2>&1 || status=$?
    [[ "$status" == 2 && "$(cat "$fixture/events")" == admission ]] || exit 1
done
echo 'Server commands admit the selected CLI once and refuse before server effects (substitute installer/server)'
)

phase_workflow_selection() (
local fixture
trap - EXIT
prepare_phase workflow-selection
stub_testkit_selection
# Execute the hosted selection step against the actual Make adapter. Its checked
# stdout is the executable path; diagnostics must stay out of that selection.
yq -o json '.jobs.recovery.steps' "$root/.github/workflows/recovery.yml" > "$fixture/recovery-steps.json"
bash "$root/vendor/toko-tooling/scripts/ci/extract-workflow-step.sh" "$fixture/recovery-steps.json" testkit \
    > "$fixture/testkit-step.sh"
# shellcheck disable=SC2030,SC2031 # Stub inputs intentionally stay inside this phase.
local -x JOBS_CONSUMER_CLI="$fixture/testkit selected/bin/ic-testkit-server"
mkdir -p "${JOBS_CONSUMER_CLI%/bin/*}/bin"
printf '{"selected":"fixture"}\n' > "${JOBS_CONSUMER_CLI%/bin/*}/selection.json"
for outcome in accepted refused; do
    mkdir -p "$fixture/$outcome-workflow/jobs-recovery"
    : > "$fixture/events"
    failure=0; expected=0
    if [[ "$outcome" == refused ]]; then failure=1; expected=2; fi
    status=0
    (cd "$fixture" && RUNNER_TEMP="$fixture/$outcome-workflow" \
        JOBS_CONSUMER_ADMISSION_FAILURE="$failure" \
        "$BASH" --noprofile --norc -e -o pipefail "$fixture/testkit-step.sh") \
        > "$fixture/workflow-$outcome.log" 2>&1 || status=$?
    [[ "$status" == "$expected" ]] || exit 1
    printf 'setup\nadmission\n' > "$fixture/expected-events"
    cmp "$fixture/events" "$fixture/expected-events"
    grep -F 'selected CLI admission diagnostic' "$fixture/$outcome-workflow/tools-testkit-check-stderr.log" > /dev/null
    grep -F 'selected CLI admission diagnostic' "$fixture/workflow-$outcome.log" > /dev/null
    if [[ "$outcome" == accepted ]]; then
        [[ "$(cat "$fixture/$outcome-workflow/tools-testkit-check.log")" == "$JOBS_CONSUMER_CLI" ]] || exit 1
        cmp "${JOBS_CONSUMER_CLI%/bin/*}/selection.json" \
            "$fixture/$outcome-workflow/jobs-recovery/testkit-selection.json"
    else
        [[ ! -e "$fixture/$outcome-workflow/jobs-recovery/testkit-selection.json" ]] || exit 1
    fi
done
echo 'Hosted CLI selection admits once, retains diagnostics and refuses before receipt copying (actual workflow/Make; substitute installer)'
)

phase_input_admission() (
local fixture
trap - EXIT
prepare_phase input-admission
stub_testkit_selection
# Isolate both file-admission stages after CLI admission. Substitute only the
# selected CLI and Cargo metadata; invalid inputs must create no attempt or
# start a server. Capture admission requires the preceding Wasm input to be valid.
mkdir -p "$fixture/evidence" "$fixture/wasm/wasm32-unknown-unknown/release" "$fixture/wasm/debug"
stub_target_directory
# shellcheck disable=SC2030,SC2031 # Stub inputs intentionally stay inside this phase.
local -x JOBS_CONSUMER_CLI="$fixture/server" JOBS_CONSUMER_WASM_TARGET="$fixture/wasm"
cat > "$fixture/server" <<'SERVER'
#!/usr/bin/env bash
printf 'unexpected server dispatch\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"
exit 49
SERVER
chmod +x "$fixture/server"
printf 'retained Wasm input\n' > "$fixture/input"
input="$fixture/wasm/wasm32-unknown-unknown/release/ic_jobs_test_consumer.wasm"
capture="$fixture/wasm/debug/jobs-capture-artifact"
for stage in wasm capture; do
    invalid_input="$input"; link_target="$fixture/input"
    if [[ "$stage" == capture ]]; then
        cp "$fixture/input" "$input"
        invalid_input="$capture"; link_target="$fixture/server"
    fi
    for kind in absent symlink directory; do
        [[ "$kind" != symlink ]] || ln -s "$link_target" "$invalid_input"
        [[ "$kind" != directory ]] || mkdir "$invalid_input"
        : > "$fixture/events"
        status=0
        (cd "$fixture" && PATH="$fixture/.tools/rust/common/bin:$PATH" RUNNER_TEMP="$fixture/evidence" \
            "$BASH" scripts/ci/test-canister.sh) > "$fixture/$stage-$kind.log" 2>&1 || status=$?
        [[ "$status" == 1 && "$(cat "$fixture/events")" == $'admission\nmetadata' ]] || exit 1
        [[ -z "$(ls -A "$fixture/evidence")" && "$(cat "$fixture/input")" == 'retained Wasm input' ]] || exit 1
        if [[ "$stage" == capture ]]; then cmp "$fixture/input" "$input"; fi
        if [[ "$kind" == symlink ]]; then rm "$invalid_input"
        elif [[ "$kind" == directory ]]; then rmdir "$invalid_input"; fi
    done
done
echo 'Invalid Wasm/capture inputs refuse before attempt/server dispatch (real wrapper; substitute metadata/CLI)'
)

phase_artifact_capture() (
local fixture
trap - EXIT
prepare_phase artifact-capture
stub_testkit_selection
mkdir -p "$fixture/wasm/wasm32-unknown-unknown/release" "$fixture/wasm/debug"
input="$fixture/wasm/wasm32-unknown-unknown/release/ic_jobs_test_consumer.wasm"
capture="$fixture/wasm/debug/jobs-capture-artifact"
printf 'retained Wasm input\n' > "$fixture/input"
# shellcheck disable=SC2030,SC2031 # Stub inputs intentionally stay inside this phase.
local -x JOBS_CONSUMER_WASM_TARGET="$fixture/wasm"
target="$(cargo "+$JOBS_CONSUMER_RUST_VERSION" metadata --manifest-path qualification/Cargo.toml --format-version 1 --no-deps --locked --offline | jq -er .target_directory)"
cp "$target/debug/jobs-capture-artifact" "$capture"
stub_target_directory
# The real recovery wrapper must consume one frozen original even if the live
# producer is replaced during qualification. Only transforms/server effects are
# substituted; flags, paths, hashes, logs and all dispatch plumbing are real.
mkdir -p "$fixture/selected/bin" "$fixture/ic-bin" "$fixture/.tools/ic"
cp "$root/.tools/ic/"{pins.tsv,files.sha256} "$fixture/.tools/ic/"
cp "$root/.toko-tooling.snapshot" "$fixture/"
printf '{}\n' > "$fixture/selected/selection.json"
# shellcheck disable=SC2030,SC2031 # Stub inputs intentionally stay inside this phase.
local -x JOBS_CONSUMER_CLI="$fixture/selected/bin/ic-testkit-server"
export JOBS_CONSUMER_IC_BIN="$fixture/ic-bin" JOBS_CONSUMER_FROZEN="$fixture/input"
export JOBS_CONSUMER_PRODUCER="$input" JOBS_CONSUMER_CAPTURED="$fixture/consumed"
cat > "$fixture/vendor/toko-tooling/scripts/dev/install-ic-tools.sh" <<'INSTALLER'
#!/usr/bin/env bash
[[ "$1" == --consumer && "$2" == "$PWD" ]] || exit 49
shift 2
[[ "$*" == '--tool wasm-opt --check' ]] || exit 49
printf '%s\n' "$JOBS_CONSUMER_IC_BIN"
INSTALLER
cat > "$fixture/.tools/rust/common/bin/git" <<'GIT'
#!/usr/bin/env bash
[[ "$*" == 'rev-parse HEAD' ]] || exit 49
printf '%040d\n' 1
GIT
cat > "$fixture/.tools/rust/common/bin/rustc" <<'RUSTC'
#!/usr/bin/env bash
[[ "$*" == "+${JOBS_CONSUMER_RUST_VERSION} --version --verbose" ]] || exit 49
echo 'rustc fixture'
RUSTC
cat > "$fixture/ic-bin/wasm-opt" <<'OPTIMIZER'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == --version ]]; then echo 'wasm-opt fixture'; exit 0; fi
[[ $# == 4 && "$3" == -o && ( "$2" == -O3 || "$2" == -Os || "$2" == -Oz ) ]] || exit 49
cmp "$1" "$JOBS_CONSUMER_FROZEN"
cat "$1" > "$4"
printf '%s\n' "$2" >> "$4"
OPTIMIZER
cat > "$JOBS_CONSUMER_CLI" <<'SERVER'
#!/usr/bin/env bash
set -euo pipefail
[[ "$1" == run && "$2" == --ttl && "$3" == 900 && "$4" == --server-stdout && "$6" == --server-stderr && "$8" == -- ]] || exit 49
variant="${5%/server.stdout}"; variant="${variant##*/}"
printf 'server stdout\n' > "$5"
printf 'server stderr\n' > "$7"
shift 8
[[ "$*" == 'bash scripts/dev/minimum-rust.sh cargo test --manifest-path qualification/Cargo.toml -p ic-jobs-canister-tests --test recovery --features harness --locked --offline -- --ignored --test-threads=1 --nocapture' ]] || exit 49
printf '%s\n' "$IC_JOBS_CONSUMER_WASM" >> "$JOBS_CONSUMER_CAPTURED/paths"
cp "$IC_JOBS_CONSUMER_WASM" "$JOBS_CONSUMER_CAPTURED/$variant.wasm"
printf 'replaced mutable producer\n' > "$JOBS_CONSUMER_PRODUCER"
echo 'substitute qualification passed'
SERVER
chmod +x "$fixture/.tools/rust/common/bin/"{git,rustc} "$fixture/ic-bin/wasm-opt" "$JOBS_CONSUMER_CLI"
for mode in original optimized; do
 mkdir "$fixture/$mode-evidence"
 mkdir -p "$JOBS_CONSUMER_CAPTURED"
 : > "$JOBS_CONSUMER_CAPTURED/paths"
 cp "$fixture/input" "$input"
 args=(); variants=(original)
 if [[ "$mode" == optimized ]]; then args=(--optimized); variants+=(O3 Os Oz); fi
 (cd "$fixture" && PATH="$fixture/.tools/rust/common/bin:$PATH" RUNNER_TEMP="$fixture/$mode-evidence" \
  "$BASH" scripts/ci/test-canister.sh ${args[@]+"${args[@]}"}) > "$fixture/$mode-wrapper.log" 2>&1
 [[ "$(grep -c '^Retained capture: ' "$fixture/$mode-wrapper.log")" == "${#variants[@]}" ]] || exit 1
 evidence=("$fixture/$mode-evidence/"*)
 [[ ${#evidence[@]} == 1 && "$(head -n 1 "$JOBS_CONSUMER_CAPTURED/paths")" == "${evidence[0]}/ic_jobs_test_consumer.wasm" ]] || exit 1
 cmp "$fixture/input" "$JOBS_CONSUMER_CAPTURED/original.wasm"
 for variant in "${variants[@]}"; do
  if [[ "$variant" != original ]]; then
   [[ ! -e "${evidence[0]}/$variant/producer.wasm" ]] || exit 1
   { cat "$fixture/input"; printf '%s\n' "-$variant"; } > "$fixture/expected.wasm"
   cmp "$fixture/expected.wasm" "$JOBS_CONSUMER_CAPTURED/$variant.wasm"
  fi
  actual="$(bash "$root/vendor/toko-tooling/scripts/ci/verify-file-checksum.sh" --print sha256 "$JOBS_CONSUMER_CAPTURED/$variant.wasm")"
  [[ "$(cat "${evidence[0]}/$variant/consumer-wasm.sha256")" == "$actual" && -s "${evidence[0]}/$variant/test.log" && -s "${evidence[0]}/$variant/server.stdout" && -s "${evidence[0]}/$variant/server.stderr" ]] || exit 1
 done
 [[ "$(cat "$input")" == 'replaced mutable producer' ]] || exit 1
done
# Successful capture retires producers; failed optimized capture must retain
# both its producer bytes and the original frozen input, before server dispatch.
mv "$capture" "$fixture/real-capture"
export JOBS_CONSUMER_REAL_CAPTURE="$fixture/real-capture"
cat > "$capture" <<'CAPTURE'
#!/usr/bin/env bash
set -euo pipefail
# Force final capture publication to fail after the real optimizer has run.
# The binary must retain both its producer and IC Build's exact staging copy.
if [[ "${3:-}" == --optimize ]]; then mkdir "$2"; fi
exec "$JOBS_CONSUMER_REAL_CAPTURE" "$@"
CAPTURE
chmod +x "$capture"
mkdir "$fixture/failed-capture-evidence"
cp "$fixture/input" "$input"
status=0
(cd "$fixture" && PATH="$fixture/.tools/rust/common/bin:$PATH" RUNNER_TEMP="$fixture/failed-capture-evidence" \
 "$BASH" scripts/ci/test-canister.sh --optimized) > "$fixture/failed-capture.log" 2>&1 || status=$?
[[ "$status" == 1 ]] || exit 1
[[ "$(grep -c '^Retained capture: ' "$fixture/failed-capture.log")" == 1 ]] || exit 1
evidence=("$fixture/failed-capture-evidence/"*)
[[ ${#evidence[@]} == 1 && -f "${evidence[0]}/O3/producer.wasm" && ! -e "${evidence[0]}/O3/server.stdout" ]] || exit 1
cmp "$fixture/input" "${evidence[0]}/ic_jobs_test_consumer.wasm"
{ cat "$fixture/input"; printf '%s\n' '-O3'; } > "$fixture/expected.wasm"
cmp "$fixture/expected.wasm" "${evidence[0]}/O3/producer.wasm"
echo 'Recovery consumes one frozen original with preserved variants/hashes/logs after producer replacement (substitute optimizer/server; no live gate)'
)

phase_ci_admission() (
local fixture
trap - EXIT
prepare_phase ci-admission
bash "$root/vendor/toko-tooling/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture" \
    vendor/toko-tooling/scripts/ci/check-cargo-dependency-boundaries.sh
# These local release inputs are part of Make's common checkpoint inventory.
mkdir -p "$fixture/scripts/release"
cp "$root/scripts/release/"{validation.pl,verify.sh,ci.sh,candidate-setup.sh,source-lock.sh,metadata.sh,publish.sh,publication-lint.sh} "$fixture/scripts/release/"
cp "$root/vendor/toko-tooling/scripts/ci/"{ValidationPlan.pm,validation-evidence.pl} "$fixture/vendor/toko-tooling/scripts/ci/"
# Run Jobs' actual CI route only through its first admission stage. Substitute
# tool effects and mutate a selected input there; the logger must block all
# subsequent product checks without compilation, installation or server effects.
cp "$root/"{Cargo.toml,.toko-tooling.snapshot,rust-toolchain.toml} "$fixture/"
for member in crates/ic-jobs qualification/ic-jobs-test-consumer qualification/ic-jobs-canister-tests crates/ic-timers qualification/ic-timers-pocketic qualification/ic-timers-runtime-probe qualification/ic-timers-size-probe qualification/ic-timers-test-protocol; do
    mkdir -p "$fixture/$member"
    cp "$root/$member/Cargo.toml" "$fixture/$member/"
    if [[ -f "$root/$member/README.md" ]]; then cp "$root/$member/README.md" "$fixture/$member/"; fi
done
cp "$root/vendor/toko-tooling/scripts/ci/"{run-validation-targets,check-format-tools}.sh "$fixture/vendor/toko-tooling/scripts/ci/"
cp "$root/vendor/toko-tooling/scripts/ci/classify-ci-change.pl" "$fixture/vendor/toko-tooling/scripts/ci/"
mkdir -p "$fixture/.github/workflows"
cp "$root/.github/workflows/"{ci,recovery}.yml "$fixture/.github/workflows/"
cat > "$fixture/vendor/toko-tooling/scripts/dev/toko-tooling.sh" <<'VERIFY'
#!/usr/bin/env bash
# Snapshot admission is substituted here; this phase owns later input drift.
[[ "$*" == "check --consumer $PWD --manifest .toko-tooling.snapshot" ]]
VERIFY
cat > "$fixture/scripts/ci/check-qualification-tooling.sh" <<'VERIFY'
#!/usr/bin/env bash
# Membership admission is covered by tooling-admission, before this drift proof.
exit 0
VERIFY
for toolset in host ic rust; do
    cat > "$fixture/vendor/toko-tooling/scripts/dev/install-$toolset-tools.sh" <<'ADMISSION'
#!/usr/bin/env bash
set -euo pipefail
[[ "$1" == --consumer && "${@: -1}" == --check ]] || exit 49
case "${0##*/}" in
    install-host-tools.sh)
        printf 'admission\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"
        printf '\n# concurrent selected tool input edit\n' >> "$PWD/$JOBS_CONSUMER_MUTATION_INPUT" ;;

esac
ADMISSION
done
# shellcheck disable=SC1091
. "$root/vendor/toko-tooling/ci/tool-versions.env"
export JOBS_CONSUMER_SORT_VERSION="$TOKO_TOOLING_CARGO_SORT_VERSION"
cat > "$fixture/.tools/rust/common/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
case "$*" in
    'sort --version') printf 'cargo-sort %s\n' "$JOBS_CONSUMER_SORT_VERSION" ;;
    'fmt --version') exit 0 ;;
    *) printf 'unexpected Cargo effect\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"; exit 49 ;;
esac
CARGO
chmod +x "$fixture/.tools/rust/common/bin/cargo"
# Shared's pinned test-validation-target-runner.sh owns input mutation/deletion,
# observation failures, checkpoint files and timing format. Retain this actual
# Jobs Make/CI wiring proof: changing the Cargo manifest/lock, CLI selection or docs policy
# during admission stops checks before product Cargo effects.
for input in Cargo.toml Cargo.lock LICENSE.md crates/ic-timers/README.md scripts/dev/testkit-tool.sh ci/docs-only-paths.txt scripts/release/publication-lint.sh; do
    cp "$root/Cargo.lock" "$fixture/Cargo.lock"
    cp "$root/Cargo.toml" "$fixture/Cargo.toml"
    cp "$root/"{LICENSE.md,CONTRIBUTING.md,README.md} "$fixture/"
    cp "$root/crates/ic-timers/README.md" "$fixture/crates/ic-timers/"
    cp "$root/scripts/dev/testkit-tool.sh" "$fixture/scripts/dev/testkit-tool.sh"
    cp "$root/ci/docs-only-paths.txt" "$fixture/ci/docs-only-paths.txt"
    cp "$root/scripts/release/publication-lint.sh" "$fixture/scripts/release/publication-lint.sh"
    : > "$fixture/events"
    status=0
    JOBS_CONSUMER_MUTATION_INPUT="$input" VALIDATION_LOG_DIR="$fixture/drift-logs-${input##*/}" \
        VALIDATION_FAILURE_LOG_DIR="$fixture/drift-failures-${input##*/}" \
        make --no-print-directory -j2 -C "$fixture" ci \
        > "$fixture/ci-drift-${input##*/}.log" 2>&1 || status=$?
    [[ "$status" == 2 && "$(cat "$fixture/events")" == admission ]] || exit 1
    # Nested release runners omit console failure details. The retained target
    # log owns the diagnostic at every nesting depth; keep it inside the fixture.
    grep -F "validation input changed: $input" \
        "$fixture/drift-failures-${input##*/}/latest.log" > /dev/null
done
echo 'Changed Cargo manifest/lock, package metadata, CLI selection, docs or publication policy stops Jobs CI after admission (actual Make route; substitute tool effects; no product validation)'
)

phase_rust_policy() (
local fixture
trap - EXIT
prepare_phase rust-policy
cp "$root/rust-toolchain.toml" "$fixture/"
cat > "$fixture/.tools/rust/common/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
printf '%s\n' "$*" >> "$JOBS_CONSUMER_TOOLING_EVENTS"
CARGO
chmod +x "$fixture/.tools/rust/common/bin/cargo"
for selection in 1.92 1.92.3; do
    POLICY_FLOOR="$selection" perl -pi -e \
        's/^rust-version = "[^"]*"/rust-version = "$ENV{POLICY_FLOOR}"/' "$fixture/Cargo.toml"
    expected="$selection"
    [[ "$expected" == *.*.* ]] || expected="$expected.0"
    POLICY_PIN="$expected" perl -pi -e 's/^channel = "[^"]*"/channel = "$ENV{POLICY_PIN}"/' "$fixture/rust-toolchain.toml"
    : > "$fixture/events"
    make --no-print-directory -C "$fixture" build-consumer > "$fixture/$selection.log" 2>&1
    printf '+%s build --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --release --target wasm32-unknown-unknown --locked --offline\n' \
        "$expected" > "$fixture/expected"
    cmp "$fixture/expected" "$fixture/events"
done

# Exercise both actual hosted selection steps without installing any compiler.
POLICY_FLOOR=1.92 perl -pi -e \
    's/^rust-version = "[^"]*"/rust-version = "$ENV{POLICY_FLOOR}"/' "$fixture/Cargo.toml"
perl -pi -e 's/^channel = "[^"]*"/channel = "1.92.0"/' "$fixture/rust-toolchain.toml"
for workflow in ci recovery; do
    job=validate
    [[ "$workflow" != recovery ]] || job=recovery
    yq -o json '.' "$root/.github/workflows/$workflow.yml" | jq --arg job "$job" '.jobs[$job].steps' > "$fixture/steps.json"
    bash "$root/vendor/toko-tooling/scripts/ci/extract-workflow-step.sh" "$fixture/steps.json" rust-policy > "$fixture/selection.sh"
    : > "$fixture/outputs"
    (cd "$fixture" && GITHUB_OUTPUT="$fixture/outputs" bash -e -o pipefail selection.sh)
    printf 'toolchain=1.92.0\n' > "$fixture/expected"
    cmp "$fixture/expected" "$fixture/outputs"
done

# The actual hosted setup prepares components and Wasm exactly once.
cp "$root/vendor/toko-tooling/scripts/dev/prepare-rust-toolchain.sh" "$fixture/vendor/toko-tooling/scripts/dev/"
cat > "$fixture/.tools/rust/common/bin/rustup" <<'RUSTUP'
#!/usr/bin/env bash
printf '%s\n' "$*" >> "$JOBS_CONSUMER_TOOLING_EVENTS"
RUSTUP
chmod +x "$fixture/.tools/rust/common/bin/rustup"
for workflow in ci recovery; do
    job=validate
    [[ "$workflow" != recovery ]] || job=recovery
    yq -o json '.' "$root/.github/workflows/$workflow.yml" | jq --arg job "$job" '.jobs[$job].steps' > "$fixture/steps.json"
    bash "$root/vendor/toko-tooling/scripts/ci/extract-workflow-step.sh" "$fixture/steps.json" rust-setup > "$fixture/setup.sh"
    : > "$fixture/events"
    (cd "$fixture" && PATH="$fixture/.tools/rust/common/bin:$PATH" bash -e -o pipefail setup.sh)
    printf 'toolchain install 1.92.0 --profile minimal --component rustfmt --component clippy --target wasm32-unknown-unknown\n' > "$fixture/expected"
    cmp "$fixture/expected" "$fixture/events"
done

# Enforce actual compiler identities, including a correct rustc with wrong Cargo.
export JOBS_CONSUMER_POLICY_VERSION=1.92.0
for compiler in rustc cargo; do
    cat > "$fixture/.tools/rust/common/bin/$compiler" <<'COMPILER'
#!/usr/bin/env bash
[[ "$*" == '--version --verbose' ]] || exit 49
name="${0##*/}"
version="$JOBS_CONSUMER_POLICY_VERSION"
[[ "${JOBS_CONSUMER_WRONG_COMPILER:-}" != "$name" ]] || version=0.0.1
printf '%s fixture\nrelease: %s\n' "$name" "$version"
COMPILER
    chmod +x "$fixture/.tools/rust/common/bin/$compiler"
done
(cd "$fixture" && PATH="$fixture/.tools/rust/common/bin:$PATH" make --no-print-directory rust-policy-check) > "$fixture/identities.log"
for compiler in rustc cargo; do
    status=0
    (cd "$fixture" && PATH="$fixture/.tools/rust/common/bin:$PATH" JOBS_CONSUMER_WRONG_COMPILER="$compiler" \
        make --no-print-directory clippy) > "$fixture/wrong-$compiler.log" 2>&1 || status=$?
    [[ "$status" != 0 ]] || exit 1
    grep -F "Rust policy mismatch: $compiler reports" "$fixture/wrong-$compiler.log" > /dev/null
done

# Divergent or moving development pins refuse before any Cargo dispatch.
for pin in 1.98.0 stable; do
    POLICY_PIN="$pin" perl -pi -e 's/^channel = "[^"]*"/channel = "$ENV{POLICY_PIN}"/' "$fixture/rust-toolchain.toml"
    : > "$fixture/events"
    status=0
    make --no-print-directory -C "$fixture" build-consumer > "$fixture/pin-$pin.log" 2>&1 || status=$?
    [[ "$status" != 0 && ! -s "$fixture/events" ]] || exit 1
done

# A malformed policy cannot fall back or dispatch a plausible Cargo command.
perl -pi -e 's/^rust-version = "[^"]*"/rust-version = "invalid"/' "$fixture/Cargo.toml"
: > "$fixture/events"
status=0
make --no-print-directory -C "$fixture" build-consumer > "$fixture/invalid.log" 2>&1 || status=$?
[[ "$status" != 0 && ! -s "$fixture/events" ]] || exit 1
echo 'Unified compiler policy reaches Make/hosted setup once; drift and wrong Cargo/rustc refuse (substitute compiler only)'
)

phase_lint_policy() (
local fixture workspace member package selection lint_manifest diagnostic
fixture="$suite/lint-policy"
mkdir -p "$fixture/consumer/src"
cp "$root/qualification/ic-jobs-test-consumer/tests/fixtures/rust-policy/consumer.toml" "$fixture/consumer/Cargo.toml"
cp "$root/qualification/ic-jobs-test-consumer/tests/fixtures/rust-policy/src/lib.rs" "$fixture/consumer/src/lib.rs"
for workspace in "$root" "$root/qualification"; do
    yq -p toml -o json '.' "$workspace/Cargo.toml" > "$fixture/workspace.json"
    cargo "+$JOBS_CONSUMER_RUST_VERSION" metadata --manifest-path "$workspace/Cargo.toml" \
        --locked --offline --no-deps --format-version 1 > "$fixture/metadata.json"
    while IFS= read -r member; do
        yq -p toml -o json '.' "$member" > "$fixture/member.json"
        package="$(jq -r .package.name "$fixture/member.json")"
        jq -e --slurpfile workspace "$fixture/workspace.json" '
            def level: if type == "object" then .level else . end;
            .package.name as $name |
            (if .lints.workspace == true then $workspace[0].workspace.lints
             elif $name == "ic-timers" then .lints
             else error("member must inherit workspace lints") end) |
            select([.rust.dead_code, .rust.unused_must_use,
                    .rust.unfulfilled_lint_expectations, .rust.unsafe_op_in_unsafe_fn]
                   | all(level == "deny")) |
            select((.rust.missing_docs | level) == "deny" or (.rust.missing_docs | level) == "warn") |
            select((.rust.unsafe_code | level) == (if $name == "ic-timers" then "deny" else "forbid" end)) |
            select([.cargo.unused_dependencies, .cargo.unused_workspace_dependencies,
                    .cargo.unused_workspace_package_fields, .cargo.missing_lints_inheritance,
                    .cargo.unknown_lints] | all(level == "deny")) |
            select([.rustdoc.broken_intra_doc_links, .rustdoc.invalid_markdown_table]
                   | all(level == "deny")) |
            select(.clippy.all == {level:"warn", priority:-2}) |
            select(.clippy.correctness == {level:"deny", priority:-1}) |
            select([.clippy.redundant_clone, .clippy.needless_pass_by_value,
                    .clippy.wildcard_imports, .clippy.missing_panics_doc,
                    .clippy.must_use_candidate] | all(level == "warn")) |
            select((.clippy.too_many_lines | level) == "allow") |
            select(.clippy.nursery == null and .clippy.restriction == null and .clippy.future_not_send == null)
        ' "$fixture/member.json" > "$fixture/effective.json"
        lint_manifest="$member"
        if jq -e '.lints.workspace == true' "$fixture/member.json" > /dev/null; then
            lint_manifest="$workspace/Cargo.toml"
        fi
        {
            printf '[workspace]\nmembers = ["consumer"]\nresolver = "3"\n\n'
            jq -r '.workspace.package | "[workspace.package]\nedition = \(.edition | tojson)\nrust-version = \(."rust-version" | tojson)\n"' "$fixture/workspace.json"
            # Copy actual tables; yq intentionally has no general TOML encoder.
            awk '/^\[/ {emit = ($0 ~ /^\[(workspace\.)?lints\.(cargo|rust|rustdoc|clippy)\]$/)}
                 emit {sub(/^\[lints\./, "[workspace.lints."); print}' "$lint_manifest"
        } > "$fixture/Cargo.toml"
        cargo "+$JOBS_CONSUMER_RUST_VERSION" check --manifest-path "$fixture/Cargo.toml" \
            --offline --features valid > "$fixture/$package-valid.log" 2>&1
        for selection in must-use unsafe; do
            if cargo "+$JOBS_CONSUMER_RUST_VERSION" check --manifest-path "$fixture/Cargo.toml" \
                --locked --offline --features "$selection" --message-format=json \
                > "$fixture/$package-$selection.jsonl" 2> "$fixture/$package-$selection.stderr"; then exit 1; fi
            case "$selection" in
                must-use) diagnostic=unused_must_use ;;
                unsafe) diagnostic=unsafe_code ;;
            esac
            jq -se --arg code "$diagnostic" 'any(.[]; .reason == "compiler-message" and .message.level == "error" and .message.code.code == $code)' \
                "$fixture/$package-$selection.jsonl" > /dev/null
        done
    done < <(jq -r '.workspace_members as $members | .packages[] | select(.id as $id | $members | index($id)) | .manifest_path' "$fixture/metadata.json")
done
# Qualify the warning mode used by the actual Make recipes without changing
# compiler flags. These records distinguish reused artifacts from a new compile.
CARGO_TARGET_DIR="$fixture/target" CARGO_BUILD_WARNINGS=warn \
    cargo "+$JOBS_CONSUMER_RUST_VERSION" clippy --manifest-path "$fixture/Cargo.toml" \
    --locked --offline --features warning --message-format=json-render-diagnostics \
    > "$fixture/warning-permitted.jsonl" 2> "$fixture/warning-permitted.stderr"
for selection in warm fresh; do
    warning_target="$fixture/target"
    [[ "$selection" != fresh ]] || warning_target="$fixture/fresh-warning-target"
    if CARGO_TARGET_DIR="$warning_target" CARGO_BUILD_WARNINGS=deny \
        cargo "+$JOBS_CONSUMER_RUST_VERSION" clippy --manifest-path "$fixture/Cargo.toml" \
        --locked --offline --features warning --message-format=json-render-diagnostics \
        > "$fixture/warning-$selection.jsonl" 2> "$fixture/warning-$selection.stderr"; then exit 1; fi
    # Rendered mode sends diagnostics to stderr and artifact records to stdout.
    grep -F 'warning: unused variable: `unused_value`' "$fixture/warning-$selection.stderr" > /dev/null
done
jq -se 'any(.[]; .reason == "compiler-artifact" and .fresh == true)' "$fixture/warning-warm.jsonl" > /dev/null
echo 'Every actual member admits safe documented code and rejects ignored Results/unsafe declarations; Cargo warning admission rejects fresh and reused Clippy diagnostics (real compiler)'
)

phase_group_recipes() (
local fixture
trap - EXIT
prepare_phase group-recipes
# Exercise actual aggregate and group recipes; only Cargo/compiler effects are
# replaced. No product sources, package uploads or installed runtime are involved.
export GROUP_CI_RECIPE_EVENTS="$fixture/events"
cat > "$fixture/.tools/rust/common/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$GROUP_CI_RECIPE_EVENTS"
if [[ "$1" == clippy ]]; then [[ "${CARGO_BUILD_WARNINGS:-}" == deny ]] || exit 47; fi
if [[ "${GROUP_CI_RECIPE_FAIL:-}" == timers && "$*" == 'clippy -p ic-timers '* ]]; then exit 31; fi
CARGO
chmod +x "$fixture/.tools/rust/common/bin/cargo"
cat > "$fixture/scripts/dev/minimum-rust.sh" <<'POLICY'
#!/usr/bin/env bash
set -euo pipefail
[[ "$1" != --check ]] || exit 0
[[ "$1" == cargo ]] || exit 2
shift
exec cargo "$@"
POLICY
cat > "$fixture/expected-jobs" <<'JOBS'
clippy -p ic-jobs --all-targets --all-features --locked --offline
clippy --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer -p ic-jobs-canister-tests --all-targets --features ic-jobs-canister-tests/harness --locked --offline
clippy --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --target wasm32-unknown-unknown --locked --offline
doc -p ic-jobs --all-features --locked --offline --no-deps
test -p ic-jobs --doc --all-features --locked --offline
package -p ic-jobs --all-features --locked --offline --allow-dirty
JOBS
make --no-print-directory -C "$fixture" clippy-jobs docs-jobs package-jobs > "$fixture/jobs.log" 2>&1
cmp "$fixture/events" "$fixture/expected-jobs"
cat > "$fixture/expected-full" <<'FULL'
clippy -p ic-jobs --all-targets --all-features --locked --offline
clippy --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer -p ic-jobs-canister-tests --all-targets --features ic-jobs-canister-tests/harness --locked --offline
clippy --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --target wasm32-unknown-unknown --locked --offline
clippy -p ic-timers --all-targets --locked --offline
doc -p ic-jobs --all-features --locked --offline --no-deps
test -p ic-jobs --doc --all-features --locked --offline
doc -p ic-timers --all-features --locked --offline --no-deps
package -p ic-jobs --all-features --locked --offline --allow-dirty
package -p ic-timers --all-features --locked --offline --allow-dirty
FULL
: > "$fixture/events"
make --no-print-directory -C "$fixture" clippy docs-check package-all > "$fixture/full.log" 2>&1
cmp "$fixture/events" "$fixture/expected-full"
: > "$fixture/events"
if GROUP_CI_RECIPE_FAIL=timers make --no-print-directory -C "$fixture" clippy docs-check package-all > "$fixture/failure.log" 2>&1; then exit 1; fi
head -n 4 "$fixture/expected-full" > "$fixture/expected-failure"
cmp "$fixture/events" "$fixture/expected-failure"
echo 'Actual Jobs recipes preserve consumer/Wasm lint, docs and archive checks; full aggregates retain both libraries and stop on lint failure (substitute Cargo/compiler effects)'
)

phase_batch_commands() (
    local fixture operation setting
    trap - EXIT
    prepare_phase batch-commands
    # The fixture selects its own groups/publication, including when CI itself
    # runs inside a multi-group publication batch.
    unset RELEASE_GROUP RELEASE_BATCH_ACTIVE RELEASE_GROUPS RELEASE_PUBLISH_TARGET
    # Exercise the actual consumer Make includes; lifecycle effects stay with
    # Shared's producer suite. This is routing proof, not frozen-hook qualification.
    cat > "$fixture/vendor/toko-tooling/scripts/dev/toko-tooling.sh" <<'ADMISSION'
#!/usr/bin/env bash
[[ "${1:-}" == check ]]
ADMISSION
    cat > "$fixture/scripts/ci/check-qualification-tooling.sh" <<'ADMISSION'
#!/usr/bin/env bash
# This phase owns batch routing; tooling-admission tests real member admission.
exit 0
ADMISSION
    cat > "$fixture/vendor/toko-tooling/scripts/dev/clean-empty-source-dirs.pl" <<'CLEANUP'
print "Cleanup substituted for batch routing qualification.\n";
CLEANUP
    cat > "$fixture/vendor/toko-tooling/scripts/ci/run-release-batch.pl" <<'RUNNER'
use strict;
use warnings;
open my $events, '>>', $ENV{JOBS_CONSUMER_TOOLING_EVENTS} or die $!;
print {$events} join("\0", @ARGV), "\0" or die $!;
close $events or die $!;
exit($ENV{BATCH_COMMAND_RESULT} // 0);
RUNNER
    for operation in status resume reconcile retire; do
        : > "$fixture/events"
        if make --no-print-directory -C "$fixture" "release-batch-$operation" BATCH= > "$fixture/missing-$operation.log" 2>&1; then exit 1; fi
        [[ ! -s "$fixture/events" ]]
        make --no-print-directory -C "$fixture" "release-batch-$operation" BATCH=batch-review_42 RELEASE_DELIVERY=direct PUBLICATION_RESULT= > "$fixture/$operation.log" 2>&1
        printf '%s\0' --root "$fixture" "--$operation" batch-review_42 --delivery direct > "$fixture/expected"
        cmp "$fixture/events" "$fixture/expected"
    done
    : > "$fixture/events"
    make --no-print-directory -C "$fixture" release-batch-resume BATCH=batch-review_42 PUBLICATION_RESULT=published RELEASE_DELIVERY=direct > "$fixture/publication.log" 2>&1
    printf '%s\0' --root "$fixture" --resume batch-review_42 --delivery direct --publication-result published > "$fixture/expected"
    cmp "$fixture/events" "$fixture/expected"
    : > "$fixture/events"
    if BATCH_COMMAND_RESULT=37 make --no-print-directory -C "$fixture" release-batch-status BATCH=batch-review_42 RELEASE_DELIVERY=direct PUBLICATION_RESULT= > "$fixture/failure.log" 2>&1; then exit 1; fi
    printf '%s\0' --root "$fixture" --status batch-review_42 --delivery direct > "$fixture/expected"
    cmp "$fixture/events" "$fixture/expected"
    : > "$fixture/events"
    if make --no-print-directory -C "$fixture" release-patch release-batch-status RELEASE_GROUP=jobs BATCH=batch-review_42 > "$fixture/conflict.log" 2>&1; then exit 1; fi
    [[ ! -s "$fixture/events" ]]
    if make --no-print-directory -C "$fixture" release-batch-resume BATCH=batch-review_42 RELEASE_DELIVERY=pr > "$fixture/policy.log" 2>&1; then exit 1; fi
    [[ ! -s "$fixture/events" ]]
    make --no-print-directory -C "$fixture" release-patch RELEASE_GROUP=jobs RELEASE_REMOTE=origin RELEASE_BRANCH=main RELEASE_DELIVERY=direct RELEASE_PUBLISH_TARGET= > "$fixture/single-group.log" 2>&1
    printf '%s\0' --root "$fixture" --groups jobs --kind patch --remote origin --branch main --delivery direct --setup-target release-candidate-setup > "$fixture/expected"
    cmp "$fixture/events" "$fixture/expected"
    : > "$fixture/events"
    make --no-print-directory -C "$fixture" release-patch RELEASE_GROUP=jobs PACKAGE=ic-jobs RELEASE_REMOTE=origin RELEASE_BRANCH=main RELEASE_DELIVERY=direct RELEASE_PUBLISH_TARGET= > "$fixture/single-package.log" 2>&1
    cmp "$fixture/events" "$fixture/expected"
    : > "$fixture/events"
    if make --no-print-directory -C "$fixture" release-patch RELEASE_GROUPS=timers,jobs PACKAGE=ic-jobs > "$fixture/package-subset.log" 2>&1; then exit 1; fi
    [[ ! -s "$fixture/events" ]]
    if make --no-print-directory -C "$fixture" release-patch RELEASE_GROUP=jobs RELEASE_GROUPS= > "$fixture/no-live-fallback.log" 2>&1; then exit 1; fi
    [[ ! -s "$fixture/events" ]]
    for setting in RELEASE_FROZEN_GROUPS RELEASE_FROZEN_PUBLISH_TARGET RELEASE_FROZEN_SETUP_TARGET RELEASE_FROZEN_SETUP_MODE FROZEN_RELEASE_ACTIVE; do
        if make --no-print-directory -C "$fixture" release-patch RELEASE_GROUP=jobs "$setting=" > "$fixture/retired-setting.log" 2>&1; then exit 1; fi
        [[ ! -s "$fixture/events" ]]
        rg -F 'Retired release settings:' "$fixture/retired-setting.log" > /dev/null
    done
    echo 'Actual consumer batch Make commands require selection, preserve saved-batch/publication arguments and failures, and refuse conflicting targets or unsupported delivery before dispatch (substitute runner)'
)

phase_candid_boundary() (
    prepare_phase candid-boundary
    bash "$root/vendor/toko-tooling/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture" vendor/toko-tooling/scripts/ci/check-candid-boundary.pl
    mkdir -p "$fixture/crates/gate/src" "$fixture/crates/probe/src" "$fixture/qualification/probe/src"
    printf 'pub struct Qualification;\n' > "$fixture/qualification/probe/src/lib.rs"
    # Parser inputs only: these sample declarations are never compiled/injected.
    printf 'pub struct Owned;\nuse ic_platform::certification::cbor;\n' > "$fixture/crates/gate/src/lib.rs"
    printf 'pub struct Probe;\n' > "$fixture/crates/probe/src/lib.rs"
    cat > "$fixture/vendor/toko-tooling/scripts/ci/check-cargo-dependency-boundaries.sh" <<'BOUNDARY'
#!/usr/bin/env bash
printf 'dependency admission\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"
BOUNDARY
    make --no-print-directory -C "$fixture" check-dependency-boundaries > "$fixture/valid.log" 2>&1
    [[ "$(cat "$fixture/events")" == $'dependency admission\ndependency admission' ]]
    for source in crates/probe/src/lib.rs crates/gate/src/lib.rs; do
        for input in '#[derive(CandidType)] pub struct Owned;' 'pub use serde_cbor as packet_codec;'; do
            : > "$fixture/events"
            printf '%s\n' "$input" > "$fixture/$source"
            if make --no-print-directory -C "$fixture" check-dependency-boundaries > "$fixture/rejected.log" 2>&1; then exit 1; fi
            [[ ! -s "$fixture/events" ]]
        done
        printf 'pub struct Owned;\n' > "$fixture/$source"
    done
    # Exercise the actual Cargo checker through Make, including renamed,
    # optional target-specific declarations and the independent common policy.
    bash "$root/vendor/toko-tooling/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture" \
        vendor/toko-tooling/scripts/ci/check-cargo-dependency-boundaries.sh
    cat > "$fixture/Cargo.toml" <<'MANIFEST'
[package]
name = "boundary-probe"
version = "0.0.0"
edition = "2024"
[workspace]
[lib]
path = "crates/gate/src/lib.rs"
[target.'cfg(target_os = "none")'.dependencies]
packet = { package = "serde_cbor", version = "0.11", optional = true }
MANIFEST
    printf '[{"dependency":"serde_cbor","allowed_packages":["boundary-probe"]}]\n' > "$fixture/ci/cargo-dependency-boundaries.json"
    if make --no-print-directory -C "$fixture" check-dependency-boundaries > "$fixture/cargo-rejected.log" 2>&1; then exit 1; fi
    grep -F 'boundary-probe -> serde_cbor' "$fixture/cargo-rejected.log" > /dev/null
    echo 'Actual common boundary covers both first-party packages’ Candid and CBOR source refusals before Cargo, then rejects a real aliased/optional/target CBOR declaration despite local permission'
)

phase_fixture_scratch() (
    fixture="$suite/fixture-scratch"
    mkdir -p "$fixture/override"
    # Exercise the real caller's transitive companion closure in a fresh shell.
    copied="$fixture/copied"
    mkdir "$copied"
    bash "$root/vendor/toko-tooling/scripts/ci/prepare-script-fixture.sh" "$root" "$copied" scripts/ci/test-timer-dependencies.sh
    bash -c 'set -e; unset TMPDIR; source "$1/scripts/ci/fixture-scratch.sh"; fixture_create_workspace "$1" copied.XXXXXX' fixture "$copied" > "$fixture/copied.log"
    workspace="$fixture/workspace with spaces"
    created="$(unset TMPDIR; fixture_create_workspace "$workspace" 'default.XXXXXX')"
    [[ "$created" == "$workspace/target/tmp/"* && -d "$created" ]]
    # Caller-selected relative parents stay authoritative and are not created.
    unused="$fixture/unused workspace"
    created="$(cd "$fixture"; TMPDIR=override fixture_create_workspace "$unused" 'override.XXXXXX')"
    [[ "$created" == "$fixture/override/"* && -d "$created" && ! -e "$unused" ]]
    missing="$fixture/missing"
    if TMPDIR="$missing" fixture_create_workspace "$unused" 'missing.XXXXXX' > "$fixture/missing.log" 2>&1; then exit 1; fi
    [[ ! -e "$missing" && ! -e "$unused" ]]
    for invalid in "$fixture/invalid"$'\r' "$fixture/invalid"$'\n'; do
        status=0
        TMPDIR="$invalid" fixture_create_workspace "$unused" 'invalid.XXXXXX' > "$fixture/invalid.log" 2>&1 || status=$?
        [[ "$status" == 2 && ! -e "$invalid" && ! -e "$unused" ]]
    done
    echo 'Workspace fixture defaults, spaces, caller overrides and pre-allocation CR/LF refusal passed'
)

for phase in "${phases[@]}"; do
    "phase_${phase//-/_}"
done
fixture_complete=true
