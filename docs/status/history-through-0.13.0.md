# Historical handoff through 0.13.0

Archived receipts from the handoff before the pending 0.13.1 tooling cleanup.
Pending versions and tracker states below describe their original review time;
use [the current handoff](current.md) for active source and blockers.

# Archived handoff

## Source and pending work

Delivered source: Jobs **0.13.0**, `123b8f0c9a5abcabd9498503d947d45908aa49df`.
The pending compatible release is **0.13.1**: reuse checked restore arithmetic,
one rescheduled deadline and IC Build's reported retained-capture path; share test
restoration and narrow request-only derives, move dispatched fixture payloads and
adopt Shared Tooling 0.15.2.
`start_next` returns `(index, attempt)`. Persist the updated `jobs[index].record()`
with its immutable payload/index before external dispatch. The 0.13 signature cut
preserves stored records, ordering and transition errors.
The adopted Shared Tooling **0.15.2** revision is
`ca98798920188da9a21041ff9fabb76ec18f41ac`, with the dependency graph below.
The canonical exporter supplies 108 snapshot files. Jobs uses shared fixture
allocation, completion admission and assertion probes, preserving its own
cleanup, retained evidence and explicit live gate. Common tool pins are unchanged.
CI, recovery wrappers and focused checks select Rust 1.91.0; development remains
on 1.99.0.
Own package versions, staged work, Git refs, release
and publication remain maintainer-owned; no release or upload ran.

Use `next_due_ns(&jobs)` and `start_next(&mut jobs, now_ns)` instead of constructing
`Scheduler`. Read dispatch intent from the selected job after `start_next` succeeds.
Consumers depend on IC Timers directly for watchdog reconciliation
and completion; Jobs has no runtime Timers dependency. All in-tree callers,
examples, package docs and floor checks use this contract. Job records, ordering,
transition errors and uncertain-effect disposition are unchanged.

The Rust capture tool owns the Jobs optimizer recipe, logs and producer retirement.
It reuses IC Build's published `retain_to` finalization, avoiding a second generic
capture owner. Jobs still selects original/O3/Os/Oz and PocketIC qualification.
Failed optimizer/capture attempts retain producer evidence; successful capture
retires only its producer. Hashing uses the reviewed shared checksum helper.

The current graph selects Memory 0.42.0, Testkit 0.46.0, Timers 0.24.0, Build
0.6.0 and direct Host FS/Process 0.20.1. Concurrent Memory/Host manifest
and lock updates were preserved during the dispatch/write/toolchain cleanup.
Earlier receipts retain their original source and graph scope. Prior API-cut
`before/` and un-suffixed logs retain the Testkit 0.43 graph; its `*-044.log`
evidence records Testkit 0.44 with Timers 0.22.6. Those receipts do not qualify
the current Timers 0.24 graph.

## Current proof and blockers

Dispatch/write/toolchain cleanup evidence is in
`target/qualification/dispatch-write-toolchain-cleanup/`. Notification dispatch
consumes decoded payloads and moves the selected payload only after the stable
intent commits. The fixture Store consumes its one-shot failure control with
Option::take after serialization/size admission, preserving prior bytes on refusal.
All 19 composition tests, 23 consumer tests, five capture tests, harness compilation,
strict Clippy and separate Rust 1.91 core/consumer Wasm checks pass on the current
graph. Final Rust checks requalify the concurrently updated manifest/lock; the
agent did not edit those consumer dependency selections. The first updated Host
capture check lacked cached sources; its failure is retained, followed by locked
fetch and offline qualification. Earlier shell receipts retain their original
graph scope; the helper/caller contract is independently qualified on both shells.

Reviewed Shared main `ca98798920188da9a21041ff9fabb76ec18f41ac` is adopted through
the canonical exporter, including the action's declared Rust preparation helper.
Both actual CI/recovery input sets pass on Bash 5 and genuine Bash 3.2 with
substituted rustup, exact arguments, both installation failures and invalid input
admission. The helper centralizes argument handling; setup still invokes rustup.
Release/publication, formatting evidence and consumer-tooling fixtures pass on
both shells. Shared's retention probes receive Jobs' explicit lowercase root
binding. Release Make adapters, snapshot integrity, formatting, pins and doc links
pass. An initial adapter-check invocation omitted required fixture inputs; its
failed log is retained, and the actual Make target passes. No installation, full
local CI, macOS, live PocketIC, release or publication was selected.

The upstream guide's old no-helper sentence is corrected in
[Shared PR #223](https://github.com/dragginzgame/shared-tooling/pull/223); actual
companion export and action behavior already work in the adopted main revision.
Operational maintenance retirement remains pending PR #220 / Jobs #54. This
refresh retains those three operational files until that change is reviewed.

Test/request/maintenance cleanup evidence is in
`target/qualification/test-request-maintenance-cleanup/`. Job and management
integration tests share one JSON round-trip helper, preserving `Job::restore`.
Candid-only Timing/Create requests retain CandidType/Deserialize without unused
Serde writers; stored Init/Task/Delivery serialization is unchanged. All 35 job/
management and 23 consumer tests, Rust 1.91 consumer Wasm, recovery-harness
compilation, strict Clippy, formatting, pins and documentation checks pass.

[Shared PR #220](https://github.com/dragginzgame/shared-tooling/pull/220),
`ac28a238066d009b008eac7f2c26112636a8995f`, separates the three optional operational
maintenance roots from governance selection. Default guidance links to the
canonical host-owned files; existing roots remain until explicitly retired.
Export/opt-in/modified-retirement refusal/clean-retirement tests pass on Bash 5
and genuine Bash 3.2. The initial synthetic-history fixture failures are retained;
retirement now uses a real disposable source commit. A committed export into an
isolated Jobs copy removes the three operational files and passes integrity and
documentation links with 106 payloads. That earlier isolated result is not active retirement;
[Jobs #54](https://github.com/dragginzgame/ic-jobs/issues/54) tracks active adoption.
The exact selection arguments and candidate manifest are retained with the logs.
This is distribution evidence, not qualification of the broader Shared 0.15.2
consumer delta. No full local CI, macOS, live PocketIC, schedule, agent, merge,
release or publication was selected.

Restore/deadline/retention cleanup evidence is in
`target/qualification/restore-deadline-retention-cleanup/`, based on delivered
0.13.0 `123b8f0c9a5abcabd9498503d947d45908aa49df` plus the incoming lock update.
Restore carries checked prior-dispatch arithmetic into retry admission; the same
bounds and InvalidRecord refusals remain. Rescheduling derives one shared deadline.
The failed-capture test reads IC Build's typed retained path after removing the
producer, avoiding staging-directory discovery. Core/composition tests and all
five capture tests pass. Separate Rust 1.91 core/consumer Wasm checks, strict
Clippy, formatting, pins and documentation links pass. No full CI, macOS, live
PocketIC, release, publication, own-version, index or ref mutation ran.

Scheduler-result cut evidence is in `target/qualification/scheduler-result-cut/`.
Before files retain incoming dirty work separately from this batch. Every maintained
caller, example and package guide uses the two-element result; persistence still
crosses the validated restore boundary. Existing scheduler tests retain ordering,
retry/recurrence, blocked-effect and unchanged-on-error coverage. Notification
pagination filters before projection while retaining sorted identity order.
Server setup/check performs one CLI admission inside its recipe before effects;
substituted success/refusal checks cover both targets. Core/composition and all
23 consumer tests, Rust 1.91 core/consumer Wasm checks, example execution, rustdoc,
recovery-harness compilation and strict Clippy pass. The consumer-tooling fixture
passes on Bash 5 and genuine Bash 3.2. No full CI, macOS, live PocketIC, release,
publication, own-version mutation, staging or Git ref changes ran.

Notification/install/delivery cleanup evidence is in
`target/qualification/notification-install-delivery-cleanup/`. Notification rows
are consumed once, retaining duplicate-ID-before-restore ordering and aligned
payloads. Testkit owns create/fund/install mechanics; sender installation precedes
receiver setup with the same cycles, init identity and default management sender.
One private helper sends and confirms exact delivery receipts. Production failures
remain uncertain; the lost-result fixture traps after confirmed external effects
and also refuses unconfirmed replies. No queue borrow crosses an await.
All 19 composition tests and 23 consumer tests pass with the incoming Testkit 0.46
and Timers 0.24 graph. Rust 1.91 consumer Wasm, recovery-harness compilation,
strict Clippy, formatting, pins and documentation links pass. No full CI, live
PocketIC, macOS qualification, release or publication ran.

Restore/ID/trap cleanup evidence is in
`target/qualification/restore-id-trap-cleanup/`, based on delivered 0.12.1
`ead141625b4a4b3b701a47d429e48bbef075ec6f`. Fixed-rate restore computes interval
and occurrence offsets once and checks the selected missed-run policy directly.
Snapshot ordering retains IDs as u128 while preserving next-ID bounds and restore
ordering. Recovery scenarios share one trap assertion with complete error detail.
Core/composition and all 23 consumer tests pass, along with separate Rust 1.91
core/consumer Wasm checks, recovery-harness compilation, strict Clippy, formatting,
pins and documentation links. No full CI, live PocketIC, release or publication ran.

Memory 0.41 adoption evidence is in `target/qualification/memory041/`. All 23
consumer tests pass, including refusal of frozen registry Memory 0.40 and old
0.7.5 physical images with zero writes, unchanged pages and identical bytes.
Fresh-current-memory reopening of the frozen Jobs JSON still passes. The existing
Jobs core/composition suite, native Clippy and Rust 1.91 consumer Wasm check pass.
The isolated frozen-image producer and lock are retained with checksums; its first
attempt used an invalid feature name and the corrected attempt is retained too.
No deployment, data disposition or reset was performed. Retained 0.40 installations
need explicit owner disposition before use with the incompatible 0.41 ledger.

The incoming Testkit 0.45 graph exposed removed Candid panic helpers and retry
construction in the recovery harness. Calls now use result-returning Candid APIs
with explicit expectations and the nonzero retry constructor. Harness compilation
and strict Clippy pass after that repair; the initial failure log is retained.
These are compilation checks, not live PocketIC evidence. The fallible Memory
fixture helper remains unpublished in PR #116 on its older 0.40 base; do not apply
its earlier caller patch to the current graph without requalification.

Authorization/view cleanup is in `target/qualification/authorization-view-watchdog/`.
Manager-only operations authorize within queue access; borrows end before timer
reconciliation and awaits. Receipt authentication remains independent. Inspection
uses its matched ID for infallible view projection; pagination keeps checked ID
conversion at its boundary. All 23 consumer tests, strict Clippy, Rust 1.91 Wasm,
formatting, pin and documentation checks pass.

[Timers #64](https://github.com/dragginzgame/ic-timers/issues/64) carries the
implemented `WatchdogRunResult::completed_batch` patch on published 0.23.2
(`227c51e085`). Its named boundary regression, Rust 1.88 Wasm and strict Clippy
pass. Isolated Jobs callers pass 19 composition tests and Rust 1.91 Wasm checks.
The upstream patch, apply-checked caller patches and isolated manifests/lock are
retained under `authorization-view-watchdog/adoption/`. Timers' AGENTS.md reserves
contribution commits to the maintainer, so no upstream commit or PR was made.
The active Jobs dependency and both timer callers await reviewed upstream delivery.
No full CI, macOS qualification, live recovery or release ran.

The admitted-row cleanup removes redundant ID projection in `Queue::finish` and
an unreachable task-vector bounds check in `LoadedSnapshot::take_entry`.
Decoded vector alignment, commit validation, restore admission and receipt checks
remain authoritative. All 23 consumer tests, strict Clippy and Rust 1.91 Wasm
checks pass; formatting, pins and documentation links pass. Evidence and before
files are in `target/qualification/admitted-row-cleanup/`.

[Shared Tooling PR #213](https://github.com/dragginzgame/shared-tooling/pull/213)
provides the shared dependency-order fixture generator at `ffd972e`. Its focused
boundary tests pass on Bash 5 and genuine Bash 3.2, along with ShellCheck, Perl
syntax and documentation links. A minimal canonical export passes integrity
verification. Jobs and published Host 0.18.6 caller patches pass their actual
formatting/hook/perturbation checkers on both shells in isolated copies. Patches,
logs and producer proof are retained under `admitted-row-cleanup/adoption/`.
Initial setup failures remain retained separately. Actual consumer snapshots and
local AWK callers await reviewed shared adoption; tracking and apply-ready patches
are in [Jobs #53](https://github.com/dragginzgame/ic-jobs/issues/53) and
[Host #134](https://github.com/dragginzgame/ic-host-tooling/issues/134).
This work ran no full CI, macOS qualification, live recovery, merge or release.

Shared/Host adoption evidence is in `target/qualification/shared015-host0186/`.
Snapshot/pin/format/tool admission, release/publication substitutes, actual
formatting/evidence/consumer fixtures and Make release adapters pass. All four
changed fixture callers pass on Bash 5 and genuine Bash 3.2. The actual recovery
wrapper preserves evidence across completed, failed, premature-zero and nounset
exits on both shells; canonical allocation/completion boundary fixtures also pass.
Rust 1.91 checks pass 23 stable-consumer tests, five capture tests, release Wasm,
recovery-harness compilation, native/Wasm floors and strict native/Wasm Clippy.
The incoming Host lock selection is unchanged by this task. Source hashes and
before files bind these receipts; full local CI, macOS and live recovery remain
unqualified for this working tree.

Identity/retry/fixture cleanup evidence is in
`target/qualification/identity-retry-fixture/`. Rust 1.91 passes 45 core tests,
19 native composition scenarios, 23 stable-consumer tests, five capture tests,
native/Wasm floor checks, release Wasm and recovery-harness compilation.
Native/Wasm Clippy, formatting, pins and documentation links pass. This source
selects Timers 0.23.1; before files and final hashes bind the receipt to its graph.
Only Host's development-dependency edge was added; locked versions are unchanged
within this cleanup. No full CI or live PocketIC qualification ran.

Rust 1.91 evidence is in `target/qualification/msrv191/`, with before files,
source hashes and compiler identity. Core native/Wasm floor checks, 45 core
tests, 19 native composition scenarios, 23 stable-consumer tests, five capture
tests, release Wasm, recovery-harness compilation and native/Wasm Clippy pass.
The recovery-wrapper fixture passes with substituted optimizer/server effects;
no PocketIC qualification was selected. Formatting, pins, documentation links
and the unchanged shared snapshot also pass. Earlier Rust 1.88 receipts retain
their original source and graph scope.

Cleanup evidence is in `target/qualification/fixture-owner-cleanup/`.
`make check-consumer` passed all 23 native consumer tests, five capture tests,
release Wasm, recovery-harness compilation and native/Wasm Clippy on the earlier
Build 0.3.3 graph, which selected Host FS 0.18.4.

Recovery allocation now uses the shared physical-path helper while retaining
successful and failed evidence. Bash 5 and genuine 3.2 wrapper fixtures pass;
evidence is in `target/qualification/recovery-owner-cleanup/`.

The current fallible Memory proposal is
[PR #116](https://github.com/dragginzgame/ic-memory/pull/116), `46f5cdd`, directly
on published 0.40.0. It supersedes draft #111 without inheriting its unpublished
bounded constructors. [Host PR #133](https://github.com/dragginzgame/ic-host-tooling/pull/133),
`929ee1b`, supplies numeric child-status projection while retaining caller-owned
fallback values. Consumer adoption is tracked in
[#50 — Fallible Memory fixture admission](https://github.com/dragginzgame/ic-jobs/issues/50),
[#52 — Shared child-status projection](https://github.com/dragginzgame/ic-jobs/issues/52)
and [Testkit #102](https://github.com/dragginzgame/ic-testkit/issues/102).

Apply-checked Jobs/Memory, Jobs/Host and Testkit/Host patches and exact isolated
manifests/locks are retained under `recovery-owner-cleanup/adoption/`. Combined
candidate checks pass all 23 consumer tests, five capture tests, release Wasm,
harness compilation, native/Wasm Clippy and Rust 1.91 floors. Frozen-image refusal
remains write-free. The actual Testkit CLI compiles and passes strict Clippy with
the proposed Host helper. Producer feature/envelope tests, floors, Clippy and
Rustdoc pass separately. The retained initial Memory lint failure is corrected.
These path overrides qualify unpublished source only; actual Jobs retains registry
Memory/Host and its current setup until reviewed publication. No full local CI,
live recovery, source merge, release or publication ran. Earlier `96b5a1b` receipts
remain historical evidence, not qualification of the new proposal.

Earlier API-cut evidence: `target/qualification/011-cut/`, including the before
tree, selected source hashes and logs. Its focused checks cover 45 core tests, 19 native consumer
scenarios, five capture tests, Rust 1.88 core/consumer native and Wasm floors,
release Wasm, strict Clippy and rustdoc. The substituted recovery wrapper passes
on Bash 5 and genuine Bash 3.2, including optimizer arguments, frozen originals,
failed final capture and retained logs. These checks do not execute PocketIC.
The prepared Binaryen executable also produced all three optimized variants from
the real consumer Wasm; final artifact/tool hashes are under `binaryen-044/`.
The original `binaryen/` evidence remains bound to its earlier graph.

The earlier Memory/Testkit failures are repaired in
`target/qualification/memory039-admission/`. All 23 consumer tests, five capture
tests, release Wasm, recovery harness compilation and native/Wasm Clippy pass via
`make check-consumer`. The frozen two-slot image is now qualified for refusal on
Memory 0.39, with zero stable writes and unchanged pages during both recovery and
installation requests. The separate frozen Jobs JSON still reopens on current
memory and retains blocked effects. No image was rewritten and no reset was run.
Retained pre-cut installations still need explicit owner disposition.

The recovery harness reads native PocketIC nanoseconds directly and uses Testkit's
public rejection field. These are compilation checks; no live test was selected.
The initial failed attempt and prior failing logs remain with their source scope.
At that earlier source, native consumer tests and Rust 1.88 native/Wasm floor
checks also passed; the two frozen inputs, Cargo manifests and lockfile retain
their exact hashes.

Prior focused evidence remains in `target/qualification/consumer-layout-cleanup/`,
`target/qualification/dispatch-lookup-cleanup/`,
`target/qualification/active-delivery-cleanup/`,
`target/qualification/deadline-reply-cleanup/` and
`target/qualification/receipt-extraction-cleanup/`, with its original source scope.
Older proof is recorded in [history through 0.10.3](history-through-0.10.3.md) and
[history through 0.9.1](history-through-0.9.1.md).

## Acceptance boundaries

Full `make ci` and live PocketIC recovery were not selected. Local Linux execution
and artifact transformation do not establish macOS Intel/Apple Silicon acceptance,
real IC rollback, installed upgrades or timer delivery for this source/graph.
The last recorded live evidence is Jobs 0.8.1, source
`f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
[run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820):
84 passes across seven scenarios, four Wasm variants and three hosts.

Delivered 0.11.0 native acceptance is verified at
[CI run 38702118931](https://github.com/dragginzgame/ic-jobs/actions/runs/38702118931),
source `7e6c643c50dfe3f67252e51537f44391d92db2c7`: Linux, macOS Apple Silicon
and macOS Intel all pass. The maintained implementations and regression coverage
support closing the older native/adoption acceptance issues. This run does not
qualify the pending 0.12 tree or establish live recovery.
Remaining compatible shared adoption is tracked in
[#47 — Fixture allocation](https://github.com/dragginzgame/ic-jobs/issues/47) and
[#48 — Completion admission](https://github.com/dragginzgame/ic-jobs/issues/48).
Both helpers are adopted in the pending 0.12.1 tree through Shared Tooling 0.15.0;
their delivered/native acceptance remains with the linked issues.
Memory adoption remains at [#50](https://github.com/dragginzgame/ic-jobs/issues/50).
The delivered timestamp change supersedes PR #49; draft
[PR #51](https://github.com/dragginzgame/ic-jobs/pull/51) awaits published Testkit
0.45 before its constructor change can be integrated.
The snapshot-owned assertion probe replaces Jobs' local rewrite block.
GitHub owns current issue/PR status. No workflow was dispatched during this review.

See the [package migration guide](../../crates/ic-jobs/README.md#011-api-cut),
[safety contract](../../SAFETY.md), [consumer qualification](https://github.com/dragginzgame/ic-scheduling/blob/bb67e6f23ec50a95cf1b58f675cd5a8d8c60f4ff/apps/ic-jobs-test-consumer/README.md)
and [delivery procedure](../releasing.md).
