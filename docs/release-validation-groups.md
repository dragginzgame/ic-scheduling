# Scheduling release validation groups

Scheduling adopts the Shared Tooling [group-plan contract](release-group-validation.md).
The versioned policy is [ci/release-validation.json](../ci/release-validation.json);
the existing Make gate exposes its complete roster through
`make release-validation-stages`. The selector refuses disagreement between that
roster, the policy and the Cargo release groups. Patch, minor and major use the
same selection rules. No skip flag or stage-proof reuse is available.

## Stage mapping

| Stage | Jobs-only scope | Timers, shared or uncertain scope | Boundary |
| --- | --- | --- | --- |
| `ci` | Required | Required | Complete repository native/Wasm checks, dependency policies, both archives and combined Jobs/Timers consumer contracts. |
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
inputs also select all stages. Timers always selects all five stages.

The first release containing this adoption and the outstanding cross-group
refactor therefore remains a full-gate release. Narrowing starts with a later
change that the selector can establish is confined to Jobs. This is not a way
to ship the current incompatible dependency proposal or bypass its graph checks.

## Execution and recovery

`make RELEASE_GROUP=jobs release-verify` uses the same shared runner's `--plan`
mode. It retains canonical selection context, plan/policy, complete source input
inventory, per-stage checkpoints, selection/events, logs and timings. Source and
input drift stop subsequent work. Missing, failed or interrupted required stages
cannot be sealed as a successful group execution. Exclusions are explicitly
`not-applicable`, never synthetic passes. Lint ordering stays inside the existing
`ci` stage, before installed qualification.

After a successful release-runner invocation, the execution evidence is copied
to a unique directory beside the group's durable receipts under Git-owned release
state. Its new receipt identity binds every retained file, the source/group,
baseline tag, plan/policy, host/compiler/build settings and completed obligations.
Preparation and subsequent checks re-admit it, together with the existing exact
metadata and unrelated-group preservation receipts. Other source files must still
match the validation checkpoint; intended version metadata is admitted separately.
Deleting development `target/` scratch does not remove the durable proof.
Replacing a validation attempt archives its prior receipt/pointer and preserves
both old and new evidence directories. Old full-gate attempts retain their old
source/adapter contract; this format does not upgrade their receipts.

## Focused qualification

`make test-release-validation` uses the real selector, shared plan parser/runner,
Git baselines, input drift checks and receipt admission. Only build/installed
recipes and the catalog observation are substituted. It checks same-source
Jobs versus full Timers command selection, required/excluded failures, shared
inputs, stale source, missing/unknown groups and baselines, roster drift,
incomplete plans, tampered/missing evidence and recovery after failure. It reports
command counts and elapsed fixture time; that is not a measured production speed-up.

`make test-release-tooling-metadata` and `make test-release-tooling-recovery` cover
the actual Cargo metadata/editor, version transaction, journal and local Git
delivery adapters with product commands and uploads substituted. Existing root
CI still runs the complete release-tooling fixture set. These fixtures establish
selection and custody behavior, not live IC or macOS product qualification.
