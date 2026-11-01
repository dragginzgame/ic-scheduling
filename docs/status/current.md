# Current handoff

## Platform 0.16 delivery in progress — 2026-11-01

The requested update now resolves with published Platform 0.16.0, Endpoint 0.3.0,
Memory/Storage 0.60.6, Testkit/Native 0.76.0 and Wire/Derive 0.13.3. There is one
registry Platform facade, macro and matching executor guard; canonical Types
remains 0.15.1. The local graph policy and README now select the same generation.
Existing incoming lock selections were preserved, including Build/Host 0.73.4,
Logging 0.23.5 and Metrics 0.24.4. These are the actual focused-check inputs.
No sibling source, public API, timer behavior or stored representation changed.

Full locked metadata, native/Wasm dependency/provider policies, affected-package
Clippy on Rust 1.91, all four native size-probe profiles, 175 Timers unit tests,
and 26 consumer composition/stored-Wire/API cases pass. Both the Jobs consumer
and Timers runtime probe link successfully as optimized Wasm on Rust 1.91.
The dependency rejection
fixtures pass against Platform 0.16. Managed Testkit CLI 0.76 setup/offline admission
and the existing PocketIC 16.1.0 server check pass. Cargo sorting, local links,
ShellCheck and whitespace checks cover the changed files. These are Linux native
and Wasm compile checks, not fresh installed-runtime or macOS qualification.

Evidence is `/tmp/ic-scheduling-platform-retry.5hinanvm/`: registry observations,
incoming file backups/diff, prepared lock, metadata, original linked Wasm and
focused command logs.
Earlier blocked attempts remain `/tmp/ic-scheduling-platform016.90riv96s/` and
`/tmp/ic-scheduling-platform-retry.rnr3b32a/`; do not relabel their failed evidence.
Compatible pending notes are Timers 0.35.3 and Jobs 0.33.3 because the changed
Platform contracts stay private to these packages. Cargo package versions remain
Timers 0.35.2 / Jobs 0.33.2. The user has now authorized delivery of both groups, including separate crates.io
publication. The selected patch releases are Timers 0.35.3 then Jobs 0.33.3,
using the standard direct origin/main workflow and each complete release gate.
Delivery checkout/evidence: `/tmp/ic-scheduling-platform-delivery.e3mlrmqt/`.
The original checkout and exact candidate backups are preserved during delivery.

## Timers 0.35.2 delivered — 2026-11-01

The requested issue fix and deployment are complete.
[PR #85](https://github.com/dragginzgame/ic-scheduling/pull/85) is merged and
[issue #84](https://github.com/dragginzgame/ic-scheduling/issues/84) is closed.
Absent ordinary demand now settles as Stopped/idle without cancellation accounting,
retaining identity and observations for later rearming. Explicit cancellation,
existing inactive reasons, unregister dominance and invariant failure priority
remain intact. There is no public API, persistence or dependency change.

[IC Timers 0.35.2](https://crates.io/crates/ic-timers/0.35.2) is published.
The annotated `timers-v0.35.2` tag identifies release commit
`3b4e0f88cbf9e4387d35f33d6ce234e9cb7fee95`; tag object is
`7479f0ae2d20fcb63d6e035d3b7776701bbe12e7`. Standard direct delivery atomically
pushed main and tag, followed by the separate authorized crates.io command.
Fresh registry readback confirms the version is not yanked. The downloaded crate,
local upload archives and registry agree on checksum
`d6e84f746bfc3990dc299ac430de9fcefdb534d06d58faef8612211e9d1b7c91`;
embedded VCS identity matches the release commit. Do not repeat the upload.
Jobs remains 0.33.2; the unrelated working dependency updates were excluded.

The complete local release gate passed on merged source
`c020552ae411016c8ddec2aa31d5e2e69d8adf2d`: combined CI, Rust 1.91/native/Wasm,
package and tooling checks, all 44 live watchdog cases and the policy-cohort gate.
The standard metadata transaction binds the prepared versioned payload separately.
Local evidence is Linux x86_64. On the updated PR head `f56c306`,
[hosted CI](https://github.com/dragginzgame/ic-scheduling/actions/runs/41293842362)
has passed Linux; both macOS job conclusions were still pending at readback.
Do not infer native macOS execution from local evidence.

Delivery checkout and evidence are `/tmp/ic-scheduling-timers0352.AO0pyy/`:
`source/.git/release-state/timers-0.35.2.plan` is complete; `release-patch.log`,
`publish.log`, `publication-receipt.json`, `registry-after/` and the original
`.crate` retain effect evidence. `retained-release-evidence/` preserves original
Wasm/build stamps, runtime/server diagnostics, cohort report and receipts with
checksums. Qualification/released locks and `released-source.tar` retain source
identities. The regression's original qualification is separately retained at
`/tmp/ic-scheduling-issue84.PARjAP/`, including failures against unchanged code.

## Checkout and remaining work

The normal checkout combines this adoption with pre-existing Cargo.lock edits.
Preserve the original lock snapshot and unstaged work; the completed 0.35.2
delivery excluded those edits and selected no Jobs release.
Synchronization to `5e0a072` completed, preserving the dependency diff exactly
while advancing the five Timers lock entries to 0.35.2. The 0.35.2 evidence
directory retains `root-before-sync/` and `checkout-sync-receipt.json` with
backups and verification; the index remains empty.
Both selected patch deliveries are pending; do not claim publication until
registry and archive/VCS readback succeeds for each group.

Canic adoption is separate consumer work. The previously qualified Jobs integration
is [Canic PR #722](https://github.com/dragginzgame/canic/pull/722), source `4759bec`,
from Canic 0.114.0. It owns the queue, storage, authorization and exact receipts;
all restores use Job::restore. Its installed recovery case and five native tests
passed. Evidence is `/tmp/ic-scheduling-improvements.VvCLTy/`, including original
Wasm, source/lock identity and `canic-qualification-receipt.json`. No Canic merge,
release or adoption is selected. Sibling working checkouts remain read-only.

## Workspace and recovery custody

Published groups are Jobs 0.33.2 and Timers 0.35.2. Jobs 0.33.3 draft stays open;
Timers 0.35.2 notes are finalized and the compatible 0.35.3 draft is open. All nine packages keep independent group
versions, root catalogs, complete alias rosters and group ledgers; no shared root
version. Shared 0.27.23 at `619d194875369e0f72b45477b66adb430acbf6b7` remains the
intact canonical 132-file snapshot. Committed supplier selections remain those from `6810b1d`; the working graph
above has focused adoption checks but has not acquired release qualification.

Prior Timers 0.35.1 evidence is `/tmp/ic-scheduling-timers0351.7OO3LK/`, including
its complete journal, registry/archive/VCS identity and retained runtime evidence.
Prior Timers 0.35.0 custody is `/tmp/ic-scheduling-timers035.602bpJ/`; published
Jobs 0.33.2 retains its complete journal and registry/archive/VCS evidence.
Completed improvement details and historical source/test identities remain in the
[pre-delivery handoff](https://github.com/dragginzgame/ic-scheduling/blob/a74126c3e2f70055d5a357496329b36c7f419119/docs/status/current.md).
Do not replay delivered effects, overwrite receipts or discard original artifacts.

Recovery artifact retention and read-only `make release-status` shipped in
`77fc661`; the release-status fixture's inherited group/commit inputs were fixed
in `2f536d5`. Status remains informational; actual effects use standard gates.
Older Observability/CI evidence is `/tmp/ic-scheduling-observability-ci.fLZWMG/`;
live Jobs recovery stays `/tmp/jobs-recovery.2IcNgn/` with its original inputs.

Require explicit RELEASE_GROUP for release/resume/publication. Direct delivery
uses origin/main; publication stays separate. Preserve exact source, dependencies,
policy, payload and receipts on recovery. Jobs stays Serde-only, consumer-owned
and independent of Timers/runtime/storage. Running/Uncertain effects stay blocked
until exact receipts or explicit owner disposition; preserve bounded queue bytes,
attempts, immutable payloads, rollback and allocation/queue custody. Logging stays
heap-only at 32 KiB; Metrics reset neither clears logs nor reuses IDs. Timers owns
the sole private provider with confined FFI and one exact Platform guard. Rust
floor is 1.91. Native execution never establishes IC recovery. Sibling Timers
source is provenance, not a build input; merge custody is
`/tmp/ic-scheduling-merge.7PRSi3Nq/`. The older Canic checkout
`/tmp/canic-jobs-adapter.dPFVZ7FD/source` is unavailable; do not reuse its claims.
