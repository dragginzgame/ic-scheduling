# Scheduling release validation groups

Current local migration: only `ic-jobs` and `ic-timers` belong to product groups.
The six private packages live in the independent `qualification/` workspace;
its literal private versions remain unchanged by product releases. The existing
stage roster still maps group changes to the same native, Wasm, installed-runtime
and policy-cohort obligations through explicit qualification manifest selections.

Toko Tooling 0.39.15 supplies shared multi-lock rendering, admission and
all-or-rollback preparation. `ci/cargo-release-groups.json` opts in through
`qualification_workspace: "qualification"`; its admitted plan owns both
qualification manifests/locks alongside the selected product files. Remove the
former shared-metadata lock declaration and the temporary consumer release guard;
there is no consumer copy of the rendering machinery. The focused metadata caller
covers both groups, frozen file/receipt inventories, private-version and supplier
preservation, and failures in product and qualification lock rendering.
Completed/failed historical batches retain their source and custody; do not
reinterpret their journals under this new policy. Ordinary release gates,
installed-runtime/cohort requirements and explicit group/publication selection
remain mandatory. This adoption does not execute a product release.

The pin-check caller uses the shared `--cargo-inheritance` validator with each
workspace's actual version owner. Product literals and explicitly declared private
qualification versions are admitted by the complete shared group plan. Dependency
inheritance is checked in both roots, including normal/dev/build/target tables.
The former consumer inheritance adapter is removed; no source inventory is filtered
and no findings are suppressed.
The consumer caller fixture refuses stale product versions, missing private
versions, independent/target dependency selections and publishable qualification.
No fake qualification release group or private version bump is introduced.

Scheduling adopts the Shared Tooling [group-plan contract](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/docs/releases/release-group-validation.md).
The versioned policy is [ci/release-validation.json](../../ci/release-validation.json);
the sole ordered roster contains 32 common CI obligations and four standalone
Timers obligations. `make release-validation-stages` emits all 36; normal `make ci`
executes the 32 common obligations once. Both project that declaration through
Shared's roster and Cargo group-name admission. Every release stage now names an
individual check, including explicit not-applicable decisions. There is no nested
aggregate `ci` stage. Patch, minor and major use the same selection rules. No skip
flag is available. One qualified fixture may admit exact-input stage evidence as
described below.

## Stage mapping

| Stage | Jobs-only scope | Timers, shared or uncertain scope | Boundary |
| --- | --- | --- | --- |
| Source/tool/group admission, pins, links, declaration and graph policies, release command admission, formatting | Required | Required | Preserve source custody, compiler/tool selection, Jobs purity and combined consumer runtime singleton checks. |
| `clippy-jobs`, `test-jobs`, `test-consumers`, `check-consumer`, `check-wasm`, `docs-jobs`, `package-jobs` | Required | Required | Jobs model, native harness and real Jobs/Timers consumer, Wasm, documentation and Jobs archive. |
| `test-consumer-tooling`, `test-recovery-evidence` | Required | Required | Consumer tool/launcher and artifact-capture inputs may change inside Jobs-owned source paths. |
| `clippy-timers`, `check-timer-provider`, `check-timer-dependencies`, `check-timer-probes`, `test-timers`, `test-timer-dependencies`, `docs-timers`, `package-timers` | Not applicable | Required | Standalone Timers sources/probes and dependency selections are unchanged in admitted Jobs-only scope. |
| Release/validation/formatting/CI-selection/status/hook self-tests | Not applicable | Required | `test-release-tooling`, `test-release-validation`, `test-formatting-evidence`, `test-ci-change-selection`, `test-release-status`, `formatting-hook-check`: their scripts and configuration cannot change in Jobs-only scope. |
| `install-testkit-tools` | Not applicable | Required | Prepare the lock-selected client/server tooling for standalone Timers qualification. |
| `install-consumer-server` | Not applicable | Required | Admit the selected installed server before running Timers probes. |
| `test-timer-watchdog` | Not applicable | Required | Timers provider, executor, traps, cancellation and recovery. |
| `test-timer-cohorts` | Not applicable | Required | Timers policy/cohort artifact and installed measurement qualification. |

Proven Jobs-only releases require **18** obligations and explicitly omit **18**.
Every new, unclassified obligation is required by default. Full `ci` remains
complete, including every tooling self-test and both libraries. Its mixed lint,
docs and archive targets now have separate Jobs/Timers recipes, also reused by
the existing `clippy`, `docs-check` and `package-all` aggregate commands. No Cargo
package or installed acceptance is removed from full CI. The hosted Linux,
macOS Apple Silicon and Intel matrix and explicit Jobs recovery workflow remain.
Native CI does not establish installed Jobs recovery or consumer rollback.

## Scope and baseline

The source is the release runner's admitted `RELEASE_SOURCE`. A clean Jobs source
may select group scope only against its current version's **annotated**
`jobs-vX.Y.Z` release tag, on an ancestor of that source. That trusted release
boundary is selected from the admitted Cargo group catalog; callers cannot supply
an arbitrary baseline. Missing/lightweight/divergent baselines select all stages.
No previous whole-gate proof is relabelled. The bounded fixture below retains
its original execution and receives fresh source/group admission.

All changes since that baseline are reviewed by the selector, not merely the
latest commit. The closed Jobs mapping covers model/consumer/harness source,
tests/examples and the Jobs ledger. Standalone Timers code/probes have no Jobs
dependency; unchanged manifests and lock keep that closure fixed. The selected
Jobs obligations still qualify the changed combined consumer. Any manifest,
dependency lock,
compiler, tooling/configuration, generator outside those owned source trees,
Timers input, shared file or unknown path selects workspace scope. Uncommitted
inputs also select all stages. Timers always requires the full 36-stage roster.

The first release containing this mapping remains a full-gate release because
the selector/Make/policy changes are shared inputs. Current outstanding Timers
and supplier changes since Jobs 0.34.3 also force workspace scope. Narrowing starts
only after an admitted Jobs release baseline contains those changes; no baseline
is advanced automatically to obtain omissions. Dependency graph admission remains
mandatory.

## Publication lint

`publish-check` and actual publication use the same consumer lint adapter.
An annotated tag must identify clean HEAD before narrowing. Shared's public
release-plan reader admits the matching group/version journal; the existing
metadata adapter re-admits its sealed validation evidence, source/compiler/host
configuration and exact prepared metadata. The admitted Jobs-only plan selects
`clippy-jobs`, including its combined consumer. Timers and workspace plans use
complete `clippy`; missing, stale or invalid proof also retains complete fresh lint.
Group selection alone does not narrow lint, and no prior lint PASS is reused.

Lint failures stop both callers before Cargo publication. Initially clean source
must remain clean at the same HEAD after lint, including dry runs. An initially
dirty dry run retains its existing `--allow-dirty` behavior with complete lint.
Actual upload still requires its clean source, matching pushed annotated tag and
fresh registry/archive admission. This does not change the release stage roster.

## Execution and recovery

`make RELEASE_GROUP=jobs release-verify` uses the same shared runner's `--plan`
mode. It retains canonical selection context, plan/policy, complete source input
inventory, per-stage checkpoints, selection/events, logs and timings. Source and
input drift stop subsequent work. Missing, failed or interrupted required stages
cannot be sealed as a successful group execution. Exclusions are explicitly
`not-applicable`, never synthetic passes. The original order is preserved:
snapshot/tool admission, lint, metadata and fixtures, native/Wasm/package checks,
then explicitly installed qualification. Every obligation has its own source/input
checkpoint and retained event/log. Full CI runs all 32 common obligations once,
without looping over groups or repeating their shared work.

At seal time Scheduling invokes Shared's
[`validation-evidence.pl verify`](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/docs/releases/group-validation-adoption.md) on the original
plan, generated release policy, context and exact run directory. Shared admits
selection, ordered events, timings, stage checkpoints and logs. `shared-proof.json` retains that canonical
result. The local adapter does not parse runner TSV or interpret event state.
Shared also owns annotated-baseline observation and bounded JSON/path/hash reads;
Scheduling retains its closed Jobs ownership mapping, host/compiler/build context
and complete source inventory.

Before sealing or admitting retained evidence, Scheduling recomputes the expected
scope and exact required/omitted rows from that source's Cargo group version,
baseline observation and saved dirty classification. A self-consistent edited plan
cannot omit Jobs/consumer/graph checks or replace the mapped baseline. Metadata
projection keeps the baseline bound to the pre-bump source, not the live version.

The version-3 Scheduling receipt binds the shared proof and every retained byte.
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
and version-2 receipts remain with their original source/adapter; they are never converted into
version-3 proofs or used to authorize the changed plan. No upload or new release
is performed by this adoption.

<a id="shared-tooling-0307-adoption-status"></a>
<a id="shared-tooling-0309-adoption-status"></a>

## Shared release batch contract

The installed tooling is crates.io `toko-tooling` 0.39.15, selected by one exact
Cargo dependency pin in each workspace, inherited by its members. Both normal
Cargo locks select the same archive checksum and are admitted against one snapshot.
Its installed archive checksum is recorded in `.toko-tooling.snapshot`.
The batch interface introduced at 0.31.0
remains in use; its receipt schema is independent of the Scheduling group
validation receipt. Historical release evidence remains source-bound.

Shared Tooling 0.31.0 owns the automatic single-group default; the local overlay
does not duplicate it or permit an empty-roster fallback to live delivery.
Release starts use the shared [batch runner](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/docs/releases/release-batches.md), selecting
the explicit `RELEASE_GROUP` by default or a complete `RELEASE_GROUPS`
roster. Publication is selected once with `RELEASE_PUBLISH_TARGET=publish`.
The consumer setup hook prepares only missing captured tools, installs the actual
format hook, and admits saved tools offline during recovery. Ordinary preflight
re-admits frozen setup instead of reinstalling it. Receipt reading admits the
current Git directory's exact group/version receipt subtree even when that Git
directory is outside the captured worktree; arbitrary external paths and symlinks
remain refused. Shared owns batch orchestration, saved context and lifecycle
commands; no local batch orchestration wrappers are added.

The 0.31 hard cut uses `RELEASE_GROUPS`, `RELEASE_PUBLISH_TARGET`,
`RELEASE_SETUP_TARGET`, `RELEASE_SETUP_MODE` and `RELEASE_BATCH_ACTIVE`; retired
settings refuse. New batches use schema 2 and save their publication commit.
Pre-0.31 batches must finish, reconcile and retire with their original tooling;
their storage paths and source pins are preserved. No active batch needed migration
at this adoption. This does not change the separate group validation receipt schema.

`make test-release-tooling-frozen` exercises both real group adapters, setup,
format hooks, metadata/editor, Git journals/local remotes, native/Wasm lint and
selected archive verification. Broad product stages and registry effects are
substituted. It covers concurrent normal-lock changes, tagged-release recovery,
uncertain publication, clean second-group interruption, completed resume without
republishing, dirty reconciliation refusal and explicit retirement preserving tags.
`make test-release-tooling-frozen-installed` additionally selects focused live
Timers ingress/trap/upgrade and Jobs management/recovery from the captured source.
Ordinary CI runs the native fixture; live execution remains explicitly selected.
These fixtures do not replace the actual release plan's complete product gates.

Jobs' mapping includes isolated model, consumer and harness implementation paths;
Timers remains full-gate-only. Archive recipes select their actual package with
`-p`; shared or uncertain inputs retain both archives and complete obligations.
The first `clippy-jobs` command covers the former core `check` stage's identical
packages, targets, features, profile and compiler. That duplicate roster entry is
removed; standalone `make check` remains available. Wasm, default-feature,
installed and archive obligations remain distinct.

### Bounded fixture reuse

On qualified Linux configurations, `test-release-validation` uses Shared's
exact-input runner and sealer. The closed input roster is
[ci/validation-fixture-inputs.txt](../../ci/validation-fixture-inputs.txt). It covers
the fixture, selector and evidence helpers, its own regression test, tool bytes,
Perl/native loader closure, compiler identity, file modes and bounded environment.
The fixture creates its own dummy manifests, lock and catalog; it does not read
product Cargo metadata or build product artifacts. A release version bump alone
therefore need not invalidate this one fixture's proof. All other required
stages, including `test-release-tooling`, continue to execute.

Each group still selects and executes its complete plan. The adapter records
`executed`, `reused`, or `ordinary` with a fallback reason. Reuse retains the
original source/group, proof and logs, while admitting the current source/group.
The version-3 group receipt binds self-contained copies of those original bytes;
recovery does not depend on the regenerable `target/validation-fixture-cache`.
Missing or corrupt proofs require execution; changed inputs cannot reuse them.

Custom process/compiler/Git configuration, parallel Make, nonstandard external
tool paths and unqualified hosts use ordinary execution. macOS reuse is not
qualified. Manual `make test-release-validation` also executes ordinarily.
This is stage evidence reuse, not a cached whole release or a product-test skip.

## Focused qualification

`make test-release-validation` uses the real selector, shared parser/runner/verifier,
Git baselines, input drift checks and receipt admission. Only build/installed
recipes and the catalog observation are substituted. It checks same-source
normal CI versus Jobs/full Timers command selection, required/excluded failures,
new roster stages, shared inputs, stale source, missing/unknown groups and baselines, roster drift,
incomplete plans, self-consistent unauthorized omissions, tampered/missing evidence
and recovery after failure. Reuse-contract checks retain full-plan cross-group,
failed-required-stage and durable recovery attempts; input invalidation and
configuration fallback dispatch the fixture stage directly. Its independent expected Jobs roster keeps consumer
boundaries required and proves newly added obligations remain required even after
a new policy baseline is admitted. It reports
command counts and elapsed fixture time; that is not a measured production speed-up.

`bash scripts/ci/test-consumer-tooling.sh group-recipes ci-admission` exercises the
actual mixed/group Make recipes and normal CI drift admission. Cargo/compiler
effects are substituted: Jobs recipes select only Jobs roots while retaining the
Wasm consumer; complete aggregates retain both libraries and stop on lint failure.
Run the six affected native recipe targets to qualify their real compiler, docs
and archive calls; a full CI or installed replay is not part of ordinary edits.

`make test-release-tooling-metadata` and `make test-release-tooling-recovery` cover
the actual Cargo metadata/editor, version transaction, journal and local Git
delivery adapters with product commands and uploads substituted. Existing root
CI still runs the complete release-tooling fixture set. These fixtures establish
selection and custody behavior, not live IC or macOS product qualification.

## Delivered qualification

Timers **0.40.3** passed the complete five-stage plan on source `b556199` with
Runtime/macros/guard 0.19.1, Build 0.74.3 and Wire/Derive 0.16.2. All 30 common
obligations (655s), CLI/server admission, 35 installed watchdog subjects and the
policy cohort passed on Linux/Rust 1.99.1. Metadata projection admitted release
`670cd0d`; Git delivery and explicitly authorized separate crates.io publication
completed. No fresh installed Jobs recovery or macOS execution is claimed.

Timers **0.40.2** passed the complete five-stage plan on source `0bc7ddf` with
the selected Logging 0.24.6, Metrics 0.29.1, Storage 0.66.2 and Host 0.74.3 patches.
All 30 common obligations, CLI/server admission, 35 installed watchdog subjects
and the policy cohort passed on Linux/Rust 1.99.1. Metadata projection admitted
release `dc0c2cd`; its journal and separate publication completed. This qualifies
the private control/entry/counter cleanup with that graph, without claiming fresh
installed Jobs recovery or macOS execution. The handoff owns retained custody.

Timers **0.40.1** and Jobs **0.34.3** each passed a fresh complete five-stage plan
on Linux with Rust 1.99.1 and Shared Tooling 0.29.4: all 30 common obligations,
selected CLI/server admission, all 35 installed watchdog subjects and the policy
cohort. Timers source `65b792f` projected to release `f03d430`; Jobs source
`f03d430` projected to release `0eec50a`. Jobs retained workspace scope because
the reviewed inputs included shared and Timers changes. Both journals completed
and separate publication succeeded. These runs qualify the current retained-state
guard and actual consumer adapter; they do not establish fresh installed Jobs
recovery or macOS execution. The handoff links their durable custody.

Timers 0.39.1 exercised the complete five-stage plan against source `720ed0c`
with Testkit 0.84 and Rust 1.99.1. Full CI, CLI/server admission, all 35 installed
watchdog subjects and the policy cohort passed on Linux. The adapter admitted
the exact metadata projection to release commit `b2c34a6`; its journal completed
and the separately authorized publication succeeded. This establishes a real
installed execution in addition to the substituted-command fixtures above.
Jobs installed recovery and hosted macOS acceptance retain their separate scopes.
The [current handoff](../status/current.md) records durable release evidence.

Timers 0.40.0 ran a fresh complete plan against source `d5cf1fa`, qualifying
Metrics 0.29, Testkit 0.84.1, Platform's native facade 0.21 and the selected TOML
tooling together. All common CI obligations and installed watchdog/cohort stages
passed on Linux with Rust 1.99.1. Exact metadata projection admitted release
commit `01fe9f0`; the journal completed and separate publication was verified
against the official registry/archive checksum. No earlier stage proof was reused.
