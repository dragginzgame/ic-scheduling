# Current handoff

## Timers 0.35.2 delivery in progress — 2026-11-01

The user requested fixing the sole open Scheduling issue,
[#84](https://github.com/dragginzgame/ic-scheduling/issues/84). The working tree
contains the reviewed correction from existing
[PR #85](https://github.com/dragginzgame/ic-scheduling/pull/85), plus direct IC
qualification and the required private fixture changes. Absent ordinary demand
settles as Stopped/idle without cancellation accounting. It retains identity and
observations; positive demand rearms the same claim. Explicit cancel stays
Cancelled, existing inactive reasons remain intact, unregister remains dominant,
and invariant failures retain priority. No public API or dependency changes.

The old proposal's note belonged to 0.35.1 before that release was delivered.
The completed local batch instead has compatible 0.35.2 draft notes. All package
versions remain Timers 0.35.1 / Jobs 0.33.2; published notes and tags are unchanged.
The user subsequently requested deployment after reviewing PR #85. This selects
updating and integrating that PR, then the standard Timers-only 0.35.2 release
and separate crates.io publication when enforced checks permit. The complete
candidate is incorporated into the existing PR without rewriting its history.
Delivery runs in `/tmp/ic-scheduling-timers0352.AO0pyy/source`, preserving the
normal checkout's dependency edits. Canic #545 needs the supplying published
version; no publication is claimed until registry/archive readback succeeds.

Evidence is `/tmp/ic-scheduling-issue84.PARjAP/`. The two original regression
cases fail against unchanged producer code. On the released dependency graph,
all 175 Timers library tests, strict core/probe package Clippy and Rust 1.91 Wasm
checks pass. The canonical focused IC driver passes three exact subjects: demand
removal versus cancellation, ordinary await/control interleaving and continuation
trap retirement. The private probe now has distinct clearing/cancel operations
and reports stopped/cancelled disposition and cancellation counts. The new case
also covers retained identity, expired contexts and later rearming; it belongs
to the required watchdog roster. Provider confinement, formatting, changed-doc
links and diff checks pass. No full local workspace/release suite is claimed.

Qualification used the isolated `source/` checkout and committed dependency lock
from `a74126c`, preserving the normal checkout's independently changing Cargo.lock.
The failed working-graph compile diagnostic is retained separately and is not
qualification of those dependency edits. Source snapshots, original IC Wasm,
server logs, lock identity and command evidence remain outside target. The
original working candidate remains recoverable while the isolated delivery
branch is committed; keep the unrelated dependency lock untouched.

## Timers 0.35.1 delivered — 2026-11-01

The user's latest-Timers deployment request is complete. Only `ic-timers` 0.35.1
was uploaded to [crates.io](https://crates.io/crates/ic-timers/0.35.1). Jobs remains
0.33.2. The annotated
[`timers-v0.35.1`](https://github.com/dragginzgame/ic-scheduling/releases/tag/timers-v0.35.1)
tag identifies release commit `ff5932665475bae87d28322a7849943f2c865ae1`; its tag object
is `119df226e0e7fdd2d166252c6f720b5fe13cb064`. Standard direct delivery atomically
pushed the release and tag to origin/main; the later handoff commit is a descendant.

The complete local release gate passed: combined CI, Rust-floor/native/Wasm and
package checks, all 43 live watchdog cases, and the policy-cohort promotion gate.
This is Linux x86_64 evidence, not a claim that native macOS CI ran locally.
Qualification source is `2f536d585911ba0a8ef0cedcf78ee12fe4ac68b3`; the standard
metadata transaction separately binds the final versioned payload. The first
gate found the readiness fixture inheriting the caller's RELEASE_GROUP; its
fixture-owned group/commit inputs are fixed and the complete gate was rerun.

The separate `make RELEASE_GROUP=timers publish` succeeded. Fresh registry
metadata confirms 0.35.1 is not yanked. The downloaded crate matches the local
uploaded archive and registry checksum
`a4e2d0ee65121e3b42389d9f85326bf94510f254b01d34ef6c9c080e9ae7b1f0`;
its embedded VCS identity matches the release commit. Do not repeat the upload.

Release work and original evidence remain in `/tmp/ic-scheduling-timers0351.7OO3LK/`.
The isolated checkout is `source/`; its complete journal is
`source/.git/release-state/timers-0.35.1.plan`. Keep that checkout for exact-attempt
reconciliation. `release-patch.log` retains the initial failed gate,
`release-patch-fixed.log` the passing complete release, and `publish.log` the
successful upload. `publication-receipt.json`, `registry-after/` and the original
`.crate` bind registry/archive/VCS identity. `retained-release-evidence/` contains
both runtime diagnostics, original Wasm, server output, the cohort report and
release receipts with checksums. `qualification-source-Cargo.lock`,
`released-Cargo.lock` and `released-source.tar` preserve their separate identities.

The normal checkout's unrelated dependency-lock edits were excluded from the
release and remain pending/unqualified. Its saved pre-delivery working lock is
`preserved-working-Cargo.lock` in the evidence directory. Synchronizing this
checkout must preserve those edits; neither a dependency update nor Jobs release
was selected by this deployment.

## Prior improvements

The user authorized three improvements: repair recovery artifact custody, add a
read-only release-readiness command and qualify a concrete Canic-owned Jobs queue.
All three are implemented and qualified. The first two are delivered in `77fc661`.
Canic source is committed and pushed at
[`4759bec`](https://github.com/dragginzgame/canic/commit/4759beca60e0923cf91a16991541ab03c119d1ac),
branch `codex/jobs-application-qualification`, from delivered Canic 0.114.0 at
`e753785953e6805461ac337868c32163f65c3209`. The isolated clone is
`/tmp/ic-scheduling-improvements.VvCLTy/canic/`; normal sibling checkouts stay read-only.
The user explicitly approved opening the prepared Canic review request after the
initial automatic-review refusal. [Canic PR #722](https://github.com/dragginzgame/canic/pull/722)
is open against main at that exact source. Merge authority is not selected.
The improvement batch selected no release or package upload; the subsequent
Timers delivery is recorded above.

Unrelated supplier updates are arriving in the working Cargo.lock. Leave them
unstaged and untouched. Source-qualified root supplier selections remain those
committed at `6810b1d`; temporary observation of an unprepared working graph does
not qualify newer selections.

## Implemented root improvements

Recovery upload patterns now select the driver's `timer-qualification-*` directories
and the promoted `target/timer-cohorts.json` receipt, alongside original consumer
Wasm and logs. A fixture expands the actual YAML upload paths against runtime,
server, original-Wasm and cohort artifacts. Upload remains selected after failure.

`make release-status` reports group current/latest published/proposed versions,
exact-commit CI, local/remote annotated tag observations and blocked inputs.
`RELEASE_GROUP=jobs|timers` narrows it; `STATUS_OFFLINE=1` makes local observations
only, and `STATUS_JSON=1` emits structured output. It installs nothing and never
writes source, refs, remote state or release journals. Network/cache failures
remain unknown/blocked. Failed working metadata admission can project versions
from committed HEAD only, explicitly labelled. It is informational: the standard
release/publication owners still admit every real effect and run complete gates.

Focused evidence is `/tmp/ic-scheduling-improvements.VvCLTy/`: real consumer
Make/workflow routing with substituted tool effects, actual upload-selection
expansion, and readiness cases with real Git and substituted registry/CI/group
metadata. Readiness covers publication gaps, offline/unknown observations,
unprepared working metadata, dirty/occupied sources, invalid groups and preservation
of local/remote refs. Actual public-registry/CI/tag readback also succeeded.
Pinned workflow/shell lint, Perl syntax, formatting and local link checks pass.
No new product compilation, full local CI or live IC recovery was needed for
these root tooling-only edits. Hosted CI runs asynchronously against delivered source.

## Canic qualification completed

The existing managed lifecycle probe now uses published IC Jobs 0.33.2. Its
application owns eight immutable one-shot jobs, bounded Wire storage, memory
grants, owner/Fleet authorization, exact destination receipts and lifecycle
reconstruction through the existing Timers 0.35.0 runtime. Every retained record
passes Job::restore. Retained Running attempts become Uncertain; only exact
receipts resolve them. This is private application qualification, not a public
framework mount API or replacement of Canic's paid-operation lease protocol.
[#502](https://github.com/dragginzgame/canic/issues/502) remains a separate assessment.

Five native queue tests, probe/harness Clippy, the integration's Rust 1.91 check,
governed inventory/worker partition and the exact installed PocketIC case pass.
The installed case exercises an actual intent-writing trap before submission,
same-release wakeups, a lost reply after a committed destination effect,
uncertainty without redelivery, stale/duplicate receipt disposition, forged caller
refusal and corrupt restore without reset. Native cases cover immutable payload
mismatch, independent post-await work, fresh installation IDs and full-capacity
frames/receipts. The native Wire producer admits the same installed Wasm; all
seven endpoint identities and direction-specific budgets match. Formatting,
changed-document links and diff checks pass. Full Canic CI/release validation was
not run locally; no Canic merge, version transaction or publication is selected.

The [Canic qualification guide](https://github.com/dragginzgame/canic/blob/4759beca60e0923cf91a16991541ab03c119d1ac/docs/features/jobs-qualification.md)
records the scope and exact governed selector. Source archive, original Wasm,
source/lock identity, build stamp, successful server output, schema and command
logs are retained outside target at `/tmp/ic-scheduling-improvements.VvCLTy/`.
Machine-readable custody is `canic-qualification-receipt.json`; original runtime
files and their checksums are in `canic-runtime-evidence/`. Installed Wasm SHA-256
is `d44298e9531dff37116a6518a0ebf729530b5a1219f1c80d46ae2057e6455bfd`.
Initial fixture failures and their logs are retained separately; passing evidence
belongs to `canic-installed-retained.log`, `canic-native-complete.log`,
`canic-final-checks-qualified.log` and `canic-contracts.log`.

The older private checkout `/tmp/canic-jobs-adapter.dPFVZ7FD/source` referenced by
prior handoffs is unavailable. No prior results or source are reused from it;
retained historical commit `fc14bbe1baa3c0fdc09b0550e25a1d3fdcd9d2d1` remains provenance.

## Workspace and recovery custody

Published group versions are Jobs 0.33.2 and Timers 0.35.1. The Jobs 0.33.3
draft remains open; Timers 0.35.1 notes are finalized and 0.35.2 is the compatible
issue #84 draft. All nine packages retain independent group versions, root catalogs,
full alias rosters and group ledgers; no root VERSION or shared workspace version.
Shared 0.27.23 at `619d194875369e0f72b45477b66adb430acbf6b7` remains an intact canonical
132-file snapshot. Jobs remains Serde-only, independent of platform/storage/runtime.

Prior Observability/CI selection evidence is `/tmp/ic-scheduling-observability-ci.fLZWMG/`:
243 native tests, Rust 1.91 native/Wasm Clippy, both complete consumer release
fixtures and 18 workflow-selection cases. #83 is delivered and closed. Code and
release gates remain complete; prose checks keep all three stable host job names
and explicitly do not claim native qualification. Prior live consumer evidence
stays `/tmp/jobs-recovery.2IcNgn/` with its original source/lock/tool selection.

Published Jobs `jobs-v0.33.2` / `4a76520d2265ab598113f00c4fbde0111268ffe2` and
Timers `timers-v0.35.1` / `ff5932665475bae87d28322a7849943f2c865ae1` retain complete
journals and registry/archive/VCS evidence. Prior Timers 0.35.0 custody remains
`/tmp/ic-scheduling-timers035.602bpJ/`; never replay effects or rewrite artifacts.
Earlier evidence is linked from the
[prior handoff](https://github.com/dragginzgame/ic-scheduling/blob/6810b1d4ab55ab2a0da8dc301f29cbdda84d9c6a/docs/status/current.md).

Require explicit RELEASE_GROUP for release/resume/publication; direct delivery
remains origin/main and publication is separate. Preserve saved source, policy,
payload and receipts on recovery. All restores use Job::restore. Running/Uncertain
effects stay blocked until exact receipts or explicit owner disposition. Preserve
bounded JQW queue bytes, attempts, immutable payloads, rollback and allocation
ledger/queue custody. Logging stays heap-only at 32 KiB; Metrics reset neither
clears logs nor reuses IDs. Timers keeps the sole private provider, confined FFI
and one exact Platform guard. Rust floor remains 1.91. Native execution alone
never qualifies IC recovery. Sibling Timers source is provenance, not a build
input; migration custody stays `/tmp/ic-scheduling-merge.7PRSi3Nq/`.
