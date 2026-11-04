# Scheduling release validation groups

Scheduling adopts the Shared Tooling [group-plan contract](release-group-validation.md).
The versioned policy is [ci/release-validation.json](../ci/release-validation.json);
the sole ordered roster contains the 30 common CI obligations and four standalone
Timers obligations. `make release-validation-stages` and normal `make ci` project
that declaration through Shared's roster admission, including Cargo group-name
consistency. Make maintains no second list. Release plans generate a five-stage
policy containing aggregate `ci` plus the four standalone stages; `ci` consumes
the same 30 common obligations. This preserves the existing full-source
checkpoint cadence without maintaining a second roster. Patch, minor and major
use the same selection rules. No skip flag or stage-proof reuse is available.

## Stage mapping

| Stage | Jobs-only scope | Timers, shared or uncertain scope | Boundary |
| --- | --- | --- | --- |
| Common policy stages | Required | Required | Complete repository native/Wasm checks, dependency policies, both archives and combined Jobs/Timers consumer contracts. |
| `install-testkit-tools` | Not applicable | Required | Prepare the lock-selected client/server tooling for standalone Timers qualification. |
| `install-consumer-server` | Not applicable | Required | Admit the selected installed server before running Timers probes. |
| `test-timer-watchdog` | Not applicable | Required | Timers provider, executor, traps, cancellation and recovery. |
| `test-timer-cohorts` | Not applicable | Required | Timers policy/cohort artifact and installed measurement qualification. |

`ci` remains deliberately conservative and complete: this adoption removes
unrelated standalone Timers installed work from Jobs releases, without narrowing
repository CI or weakening its native/Wasm integration checks. The hosted Linux,
macOS Apple Silicon and Intel matrix and explicit Jobs recovery workflow remain.
Native CI does not establish installed Jobs recovery or consumer rollback.

## Scope and baseline

The source is the release runner's admitted `RELEASE_SOURCE`. A clean Jobs source
may select group scope only against its current version's **annotated**
`jobs-vX.Y.Z` release tag, on an ancestor of that source. That trusted release
boundary is selected from the admitted Cargo group catalog; callers cannot supply
an arbitrary baseline. Missing/lightweight/divergent baselines select all stages.
No previous execution proof is relabelled or reused.

All changes since that baseline are reviewed by the selector, not merely the
latest commit. The closed Jobs mapping covers model/consumer/harness source,
tests/examples and the Jobs ledger. Standalone Timers code/probes have no Jobs
dependency; unchanged manifests and lock keep that closure fixed. Complete CI
still qualifies the changed combined consumer. Any manifest, dependency lock,
compiler, tooling/configuration, generator outside those owned source trees,
Timers input, shared file or unknown path selects workspace scope. Uncommitted
inputs also select all stages. Timers always requires `ci` and all four
standalone stages.

The first release containing this adoption and the outstanding cross-group
refactor therefore remains a full-gate release. Narrowing starts with a later
change that the selector can establish is confined to Jobs. Dependency graph
admission remains mandatory.

## Execution and recovery

`make RELEASE_GROUP=jobs release-verify` uses the same shared runner's `--plan`
mode. It retains canonical selection context, plan/policy, complete source input
inventory, per-stage checkpoints, selection/events, logs and timings. Source and
input drift stop subsequent work. Missing, failed or interrupted required stages
cannot be sealed as a successful group execution. Exclusions are explicitly
`not-applicable`, never synthetic passes. The original order is preserved:
snapshot/tool admission, lint, metadata and fixtures, native/Wasm/package checks,
then explicitly installed qualification.
The snapshot check was previously a CI prerequisite; it is now also recorded as
a stage. Full CI runs its 30 common obligations once, without looping over groups.

At seal time Scheduling invokes Shared's
[`validation-evidence.pl verify`](group-validation-adoption.md) on the original
plan, generated release policy, context and exact run directory. Shared admits
selection, ordered events, timings, stage checkpoints and logs. `shared-proof.json` retains that canonical
result. The local adapter no longer parses runner TSV or interprets event state.
Shared also owns annotated-baseline observation and bounded JSON/path/hash reads;
Scheduling retains its closed Jobs ownership mapping, host/compiler/build context
and complete source inventory.

The version-2 Scheduling receipt binds the shared proof and every retained byte.
A successful release-runner invocation copies it beside the group's Git-owned
journal. Recovery checks that immutable evidence, current source/configuration,
baseline identity and all inherited input hashes. Generated plan/policy/context
paths are mapped to their retained copies; their bytes and the original proof stay
unchanged. Only the selected group's metadata files may differ, under the
existing exact metadata/prepared receipts and original source/index admission.
The shared verifier does not grant version projection or relocation: those remain
the existing consumer custody boundary. No stages are replayed or reused during
receipt admission.

Deleting original `target/` scratch therefore preserves the durable evidence.
Failed attempts cannot seal, missing or changed evidence refuses, and replacing a
validation receipt archives its prior pointer and receipt. Historical version-1
receipts remain with their original source/adapter; they are never converted into
version-2 proofs or used to authorize the changed plan. No upload or new release
is performed by this adoption.

## Focused qualification

`make test-release-validation` uses the real selector, shared parser/runner/verifier,
Git baselines, input drift checks and receipt admission. Only build/installed
recipes and the catalog observation are substituted. It checks same-source
normal CI versus Jobs/full Timers command selection, required/excluded failures,
new roster stages, shared inputs, stale source, missing/unknown groups and baselines, roster drift,
incomplete plans, tampered/missing evidence and recovery after failure. It reports
command counts and elapsed fixture time; that is not a measured production speed-up.

`make test-release-tooling-metadata` and `make test-release-tooling-recovery` cover
the actual Cargo metadata/editor, version transaction, journal and local Git
delivery adapters with product commands and uploads substituted. Existing root
CI still runs the complete release-tooling fixture set. These fixtures establish
selection and custody behavior, not live IC or macOS product qualification.

## Delivered qualification

Timers 0.39.1 exercised the complete five-stage plan against source `720ed0c`
with Testkit 0.84 and Rust 1.99.1. Full CI, CLI/server admission, all 35 installed
watchdog subjects and the policy cohort passed on Linux. The adapter admitted
the exact metadata projection to release commit `b2c34a6`; its journal completed
and the separately authorized publication succeeded. This establishes a real
installed execution in addition to the substituted-command fixtures above.
Jobs installed recovery and hosted macOS acceptance retain their separate scopes.
The [current handoff](status/current.md) records durable release evidence.

Timers 0.40.0 ran a fresh complete plan against source `d5cf1fa`, qualifying
Metrics 0.29, Testkit 0.84.1, Platform's native facade 0.21 and the selected TOML
tooling together. All common CI obligations and installed watchdog/cohort stages
passed on Linux with Rust 1.99.1. Exact metadata projection admitted release
commit `01fe9f0`; the journal completed and separate publication was verified
against the official registry/archive checksum. No earlier stage proof was reused.
