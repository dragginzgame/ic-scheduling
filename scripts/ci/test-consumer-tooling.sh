#!/usr/bin/env bash
set -euo pipefail
# With no arguments, retain the complete CI fixture. Local changes can select
# only their affected caller phases, without replaying installer/hosted suites.
phases=("$@")
if [[ $# == 0 ]]; then
    phases=(cli-admission server-commands workflow-selection demo-commands input-admission artifact-capture ci-admission rust-policy)
fi
for phase in "${phases[@]}"; do
    case "$phase" in
        cli-admission|server-commands|workflow-selection|demo-commands|input-admission|artifact-capture|ci-admission|rust-policy) ;;
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
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/jobs-consumer-tooling.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Consumer tooling fixture retained"' EXIT

# Each phase owns its files and exported environment. Only this outer fixture
# owns cleanup; a failed subshell retains the complete suite for inspection.
suite="$fixture"
prepare_phase() {
    fixture="$suite/$1"
    mkdir -p "$fixture/scripts/dev" "$fixture/.tools/rust/common/bin"
    bash "$root/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture" make/consumer.mk
    cp "$root/Makefile" "$fixture/"
    cp -R "$root/ci" "$fixture/"
    cp "$root/scripts/dev/"{install-rust-tools,tool-store,cargo-tool-build,testkit-tool,queue-command}.sh "$fixture/scripts/dev/"
    cp "$root/scripts/dev/minimum-rust.sh" "$fixture/scripts/dev/"
    cp "$root/scripts/ci/read-cargo-workspace-rust-version.sh" "$fixture/scripts/ci/"
    cp "$root/Cargo.toml" "$root/rust-toolchain.toml" "$fixture/"
    cp "$root/scripts/ci/"{verify-file-checksum,test-canister,fixture-lifecycle}.sh "$fixture/scripts/ci/"
    cp "$root/Cargo.lock" "$fixture/Cargo.lock"
    export JOBS_CONSUMER_TOOLING_EVENTS="$fixture/events"
    : > "$fixture/events"
}

stub_testkit_selection() {
    export JOBS_CONSUMER_ADMISSION_FAILURE=0
cat > "$fixture/scripts/dev/install-rust-tools.sh" <<'INSTALLER'
#!/usr/bin/env bash
set -euo pipefail
case "$*" in
    '--package ic-testkit --lockfile Cargo.lock --bin ic-testkit-server --profile release --features cli')
        printf 'setup\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS" ;;
    '--package ic-testkit --lockfile Cargo.lock --bin ic-testkit-server --profile release --features cli --check')
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

phase_cli_admission() (
local fixture
trap - EXIT
prepare_phase cli-admission
# Substitute only compiler identity probes. Any Cargo effect remains forbidden;
# the real installer/parser must refuse the isolated, unprepared source cache.
mkdir -p "$fixture/compiler/bin"
(umask 077; mkdir -p "$fixture/store/cargo-cache")
export SHARED_TOOLING_STORE="$fixture/store" JOBS_CONSUMER_COMPILER="$fixture/compiler"
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
    cat > "$fixture/Cargo.lock" <<LOCK
version = 4

[[package]]
name = "ic-testkit"
version = "$version"
source = "registry+https://github.com/rust-lang/crates.io-index"
LOCK
    cp "$fixture/Cargo.lock" "$fixture/expected.lock"
    for target in testkit-tools-check consumer-server-check test-canister test-canister-optimized demo test-demo test-demo-cli; do
        status=0
        make --no-print-directory -j2 -C "$fixture" "$target" \
            > "$fixture/$version-$target.log" 2>&1 || status=$?
        [[ "$status" == 2 ]] || exit 1
        grep -F "ic-testkit $version" \
            "$fixture/$version-$target.log" > /dev/null
        [[ ! -s "$fixture/events" ]] || exit 1
        cmp "$fixture/Cargo.lock" "$fixture/expected.lock"
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
        cmp "$fixture/Cargo.lock" "$fixture/expected.lock"
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
bash "$root/scripts/ci/extract-workflow-step.sh" "$fixture/recovery-steps.json" testkit \
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

phase_demo_commands() (
local fixture
trap - EXIT
prepare_phase demo-commands
stub_testkit_selection
# Exercise the actual Make recipes and launcher without compilation or servers.
# Spaces in every selected path also qualify the launcher's quoting.
# shellcheck disable=SC2030,SC2031 # Stub inputs intentionally stay inside this phase.
local -x JOBS_CONSUMER_CLI="$fixture/testkit selected" JOBS_CONSUMER_WASM_TARGET="$fixture/target with spaces"
mkdir -p "$JOBS_CONSUMER_WASM_TARGET/debug/examples"
cat > "$JOBS_CONSUMER_CLI" <<'SERVER'
#!/usr/bin/env bash
set -euo pipefail
printf 'server %s\n' "$*" >> "$JOBS_CONSUMER_TOOLING_EVENTS"
case "$*" in
    check) printf '%s\n' "$JOBS_CONSUMER_WASM_TARGET/server binary" ;;
    'run --ttl 900 -- bash scripts/dev/minimum-rust.sh cargo test -p ic-jobs-canister-tests --example queue --features harness --locked --offline -- --ignored --nocapture --test-threads=1')
        [[ "$IC_JOBS_CONSUMER_WASM" == "$JOBS_CONSUMER_WASM_TARGET/wasm32-unknown-unknown/release/ic_jobs_test_consumer.wasm" ]] ;;
    *) exit 49 ;;
esac
SERVER
cat > "$fixture/.tools/rust/common/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf 'cargo %s\n' "$*" >> "$JOBS_CONSUMER_TOOLING_EVENTS"
case "$*" in
    "+${JOBS_CONSUMER_RUST_VERSION} metadata --format-version 1 --no-deps --locked --offline")
        printf '{"target_directory":"%s"}\n' "$JOBS_CONSUMER_WASM_TARGET" ;;
    "+${JOBS_CONSUMER_RUST_VERSION} build -p ic-jobs-test-consumer --release --target wasm32-unknown-unknown --locked --offline"|\
    "+${JOBS_CONSUMER_RUST_VERSION} build -p ic-jobs-canister-tests --example queue --features harness --locked --offline"|\
    "+${JOBS_CONSUMER_RUST_VERSION} test -p ic-jobs-canister-tests --example queue --features harness --locked --offline --no-run") ;;
    "+${JOBS_CONSUMER_RUST_VERSION} test -p ic-jobs-canister-tests --test queue_cli --features harness --locked --offline -- --ignored --nocapture")
        [[ "$IC_JOBS_QUEUE_SERVER" == "$JOBS_CONSUMER_WASM_TARGET/server binary" &&
           "$IC_JOBS_QUEUE_BIN" == "$JOBS_CONSUMER_WASM_TARGET/debug/examples/queue" &&
           "$IC_JOBS_CONSUMER_WASM" == "$JOBS_CONSUMER_WASM_TARGET/wasm32-unknown-unknown/release/ic_jobs_test_consumer.wasm" ]] ;;
    *) exit 49 ;;
esac
CARGO
cat > "$JOBS_CONSUMER_WASM_TARGET/debug/examples/queue" <<'QUEUE'
#!/usr/bin/env bash
set -euo pipefail
[[ $# == 4 && "$1" == --server && "$2" == "$JOBS_CONSUMER_WASM_TARGET/server binary" &&
   "$3" == --wasm && "$4" == "$JOBS_CONSUMER_WASM_TARGET/wasm32-unknown-unknown/release/ic_jobs_test_consumer.wasm" ]]
printf 'queue\n' >> "$JOBS_CONSUMER_TOOLING_EVENTS"
QUEUE
chmod +x "$JOBS_CONSUMER_CLI" "$fixture/.tools/rust/common/bin/cargo" "$JOBS_CONSUMER_WASM_TARGET/debug/examples/queue"
for command in demo test-demo test-demo-cli; do
    : > "$fixture/events"
    # Bind stubs in Make's incoming environment as well as its recipe exports.
    # This isolates direct and recursive recipes on macOS GNU Make 3.81 too.
    status=0
    PATH="$fixture/.tools/rust/common/bin:$PATH" \
        make --no-print-directory -j2 -C "$fixture" "$command" > "$fixture/$command.log" 2>&1 || status=$?
    if [[ "$status" != 0 ]]; then
        cat "$fixture/$command.log" >&2
        exit "$status"
    fi
    {
        printf 'admission\nserver check\ncargo +%s build -p ic-jobs-test-consumer --release --target wasm32-unknown-unknown --locked --offline\n' "$JOBS_CONSUMER_RUST_VERSION"
        if [[ "$command" == test-demo ]]; then
            printf 'cargo +%s test -p ic-jobs-canister-tests --example queue --features harness --locked --offline --no-run\n' "$JOBS_CONSUMER_RUST_VERSION"
        else
            printf 'cargo +%s build -p ic-jobs-canister-tests --example queue --features harness --locked --offline\n' "$JOBS_CONSUMER_RUST_VERSION"
        fi
        printf 'admission\ncargo +%s metadata --format-version 1 --no-deps --locked --offline\n' "$JOBS_CONSUMER_RUST_VERSION"
        case "$command" in
            demo) printf 'server check\nqueue\n' ;;
            test-demo) printf 'server run --ttl 900 -- bash scripts/dev/minimum-rust.sh cargo test -p ic-jobs-canister-tests --example queue --features harness --locked --offline -- --ignored --nocapture --test-threads=1\n' ;;
            test-demo-cli) printf 'server check\ncargo +%s test -p ic-jobs-canister-tests --test queue_cli --features harness --locked --offline -- --ignored --nocapture\n' "$JOBS_CONSUMER_RUST_VERSION" ;;
        esac
    } > "$fixture/expected-events"
    cmp "$fixture/events" "$fixture/expected-events"
    : > "$fixture/events"
    status=0
    (cd "$fixture" && PATH="$fixture/.tools/rust/common/bin:$PATH" JOBS_CONSUMER_ADMISSION_FAILURE=1 \
        "$BASH" scripts/dev/queue-command.sh "$command") > "$fixture/$command-refused.log" 2>&1 || status=$?
    [[ "$status" == 37 && "$(cat "$fixture/events")" == admission ]] || exit 1
done
echo 'Demo commands preserve build/admission order, arguments and separate server lifetimes (actual Make/launcher; substitute Cargo/server/client)'
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
target="$(cargo "+$JOBS_CONSUMER_RUST_VERSION" metadata --format-version 1 --no-deps --locked --offline | jq -er .target_directory)"
cp "$target/debug/jobs-capture-artifact" "$capture"
stub_target_directory
# The real recovery wrapper must consume one frozen original even if the live
# producer is replaced during qualification. Only transforms/server effects are
# substituted; flags, paths, hashes, logs and all dispatch plumbing are real.
mkdir -p "$fixture/selected/bin" "$fixture/ic-bin" "$fixture/.tools/ic"
cp "$root/.tools/ic/"{pins.tsv,files.sha256} "$fixture/.tools/ic/"
cp "$root/.shared-tooling.snapshot" "$fixture/"
printf '{}\n' > "$fixture/selected/selection.json"
# shellcheck disable=SC2030,SC2031 # Stub inputs intentionally stay inside this phase.
local -x JOBS_CONSUMER_CLI="$fixture/selected/bin/ic-testkit-server"
export JOBS_CONSUMER_IC_BIN="$fixture/ic-bin" JOBS_CONSUMER_FROZEN="$fixture/input"
export JOBS_CONSUMER_PRODUCER="$input" JOBS_CONSUMER_CAPTURED="$fixture/consumed"
cat > "$fixture/scripts/dev/install-ic-tools.sh" <<'INSTALLER'
#!/usr/bin/env bash
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
[[ "$*" == 'bash scripts/dev/minimum-rust.sh cargo test -p ic-jobs-canister-tests --test recovery --features harness --locked --offline -- --ignored --test-threads=1 --nocapture' ]] || exit 49
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
  actual="$(bash "$root/scripts/ci/verify-file-checksum.sh" --print sha256 "$JOBS_CONSUMER_CAPTURED/$variant.wasm")"
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
# These local release inputs are part of Make's common checkpoint inventory.
mkdir -p "$fixture/scripts/release"
cp "$root/scripts/release/"{validation.pl,verify.sh,ci.sh} "$fixture/scripts/release/"
cp "$root/scripts/ci/"{ValidationPlan.pm,validation-evidence.pl} "$fixture/scripts/ci/"
# Run Jobs' actual CI route only through its first admission stage. Substitute
# tool effects and mutate a selected input there; the logger must block all
# subsequent product checks without compilation, installation or server effects.
cp "$root/"{Cargo.toml,.shared-tooling.snapshot,rust-toolchain.toml} "$fixture/"
for member in crates/ic-jobs apps/ic-jobs-test-consumer crates/ic-jobs-canister-tests crates/ic-timers crates/ic-timers-pocketic crates/ic-timers-runtime-probe crates/ic-timers-size-probe crates/ic-timers-test-protocol; do
    mkdir -p "$fixture/$member"
    cp "$root/$member/Cargo.toml" "$fixture/$member/"
done
cp "$root/scripts/ci/"{run-validation-targets,check-format-tools}.sh "$fixture/scripts/ci/"
cp "$root/scripts/ci/classify-ci-change.pl" "$fixture/scripts/ci/"
mkdir -p "$fixture/.github/workflows"
cp "$root/.github/workflows/"{ci,recovery}.yml "$fixture/.github/workflows/"
cat > "$fixture/scripts/ci/verify-shared-tooling-snapshot.sh" <<'VERIFY'
#!/usr/bin/env bash
exit 0
VERIFY
for toolset in host ic rust; do
    cat > "$fixture/scripts/dev/install-$toolset-tools.sh" <<'ADMISSION'
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
. "$root/ci/tool-versions.env"
export JOBS_CONSUMER_SORT_VERSION="$SHARED_TOOLING_CARGO_SORT_VERSION"
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
# Jobs Make/CI wiring proof: changing the lock, CLI selection or demo launcher
# during admission stops checks before product Cargo effects.
for input in Cargo.lock scripts/dev/testkit-tool.sh scripts/dev/queue-command.sh ci/docs-only-paths.txt; do
    cp "$root/Cargo.lock" "$fixture/Cargo.lock"
    cp "$root/scripts/dev/testkit-tool.sh" "$fixture/scripts/dev/testkit-tool.sh"
    cp "$root/scripts/dev/queue-command.sh" "$fixture/scripts/dev/queue-command.sh"
    cp "$root/ci/docs-only-paths.txt" "$fixture/ci/docs-only-paths.txt"
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
echo 'Changed lock, CLI selection, demo launcher or docs policy stops Jobs CI after admission (actual Make route; substitute tool effects; no product validation)'
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
    printf '+%s build -p ic-jobs-test-consumer --release --target wasm32-unknown-unknown --locked --offline\n' \
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
    bash "$root/scripts/ci/extract-workflow-step.sh" "$fixture/steps.json" rust-policy > "$fixture/selection.sh"
    : > "$fixture/outputs"
    (cd "$fixture" && GITHUB_OUTPUT="$fixture/outputs" bash -e -o pipefail selection.sh)
    printf 'toolchain=1.92.0\n' > "$fixture/expected"
    cmp "$fixture/expected" "$fixture/outputs"
done

# The actual hosted setup prepares components and Wasm exactly once.
cp "$root/scripts/dev/prepare-rust-toolchain.sh" "$fixture/scripts/dev/"
cat > "$fixture/.tools/rust/common/bin/rustup" <<'RUSTUP'
#!/usr/bin/env bash
printf '%s\n' "$*" >> "$JOBS_CONSUMER_TOOLING_EVENTS"
RUSTUP
chmod +x "$fixture/.tools/rust/common/bin/rustup"
for workflow in ci recovery; do
    job=validate
    [[ "$workflow" != recovery ]] || job=recovery
    yq -o json '.' "$root/.github/workflows/$workflow.yml" | jq --arg job "$job" '.jobs[$job].steps' > "$fixture/steps.json"
    bash "$root/scripts/ci/extract-workflow-step.sh" "$fixture/steps.json" rust-setup > "$fixture/setup.sh"
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

for phase in "${phases[@]}"; do
    "phase_${phase//-/_}"
done
fixture_complete=true
