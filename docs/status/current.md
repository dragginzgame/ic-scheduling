# Current handoff

## Published upstream cohort adopted — 2026-11-01

The working catalog and registry lock now select Memory/Storage 0.62.0,
Metrics 0.25.0, Platform 0.17.0, Runtime/macros/guard 0.17.1, Types 0.16.0,
Testkit/Native 0.78.0, Wire/Derive 0.14.1 and Endpoint 0.4.0. Build/Host
minimums are 0.73.6 and Logging is 0.23.7. All upstream archives match the
official sparse-index checksums. The old Platform 0.16 and registry PocketIC
client are removed; there is one registry Runtime owner and one Wire identity.
No local supplier overrides are used.

The reviewed Shared Tooling snapshot is 0.27.26 at
`1ac8c6d878598c3d32cbd91c234cabea4e416961`, refreshed through its canonical
exporter after an impact review. All 136 payload files pass integrity. The actual
pinned release metadata tool was prepared through the new setup policy and
passes offline admission. Both Jobs and Timers metadata fixtures pass with real
Cargo/editor/Git operations and substituted full gates/setup/upload; these
fixtures do not release anything. Existing grouped formatting, native bootstrap
order and retained evidence boundaries remain in place. No executable store
reclamation was requested or performed.

Timers now consumes the independent Runtime directly; Wasm probes select its
CDK entrypoints and native consumer DTOs select canonical Types. Platform's facade
is qualification-only in the catalog. Jobs remains Serde-only. Timer FFI stays
confined to its private provider; all restore paths retain validated Job::restore,
stored bytes and blocked uncertain-effect behavior.

Focused Linux evidence passes against the complete published graph: strict
native Clippy for the consumer, shared transport and Timers; strict Wasm library
Clippy for the consumer and both probes; separate Rust 1.91 core and native
transport checks; 71 consumer/API/Wire cases and 178 Timers/protocol cases; actual
Rust 1.91 release Wasm builds for the consumer and runtime probe; dependency
graphs, pins, external boundaries, timer provider checks and timer rejection
fixtures. Changed Rust formatting and scoped ShellCheck pass. Live PocketIC
recovery, fresh native macOS qualification and full release CI were not selected.
The final facade role-label move changes no dependency graph or build input
selection; grouped ordering and dependency admission are checked afterward.

Jobs has pending 0.33.5 notes; Timers has pending 0.36.0 notes for coordinated
Runtime/public Metrics identity alignment. Actual package versions remain Jobs
0.33.4 and Timers 0.35.4. The user's existing source changes are preserved. No
commit, push, merge, release or publication was performed in this adoption turn;
other crate delivery remains user-owned.

Incoming backups, exact registry records, final source/lock identities, command
logs, Wasm artifacts and `adoption-receipt.json` are retained under
`.tools/adoption-evidence/ic-scheduling-latest.64UFRFxc/`. Earlier prepared patch
and source-override diagnostics are historical; they are superseded by this
actual registry qualification.

## Platform/Testkit/Wire retry: Endpoint cleared; Testkit release remains — 2026-11-01

The latest follow-up registry read still selects Testkit/Native 0.77.0.
PR #310 remains open at the same head; its runtime CI passes and main CI is in
progress. The working catalog and lock match the prior retry's inputs exactly;
no compilation was repeated for the unchanged failure. Fresh registry records
and a retry receipt are retained under
`.tools/adoption-evidence/ic-scheduling-adoption.kqwXLgxI/`. Completing this
prerequisite requires upstream merge/publication authority, separate from local
Scheduling adoption under the contribution rules.

Platform 0.17.0, Runtime/guard 0.17.1, Types 0.16.0, Testkit/Native 0.77.0
and Wire/Derive 0.14.0 are published. Endpoint 0.4.0 is now published too, with
its archive checksum verified against the official index. The complete registry
candidate resolves with Runtime/macros/guard 0.17.1 and Types 0.16.0. Dependency
graphs, external boundaries, pins and private timer-provider confinement pass.
The Jobs consumer and Timers runtime probe both build for Wasm in release mode
on Rust 1.91. Jobs core separately checks at that floor, and all 175 Timers plus
three protocol cases pass. These are native and Wasm proofs, not installed IC
recovery qualification.

Published Testkit 0.77 still exposes Core 0.13 `Limits` in `WireCallSpec`.
The full registry graph's native shared transport fails E0308 when projecting
the consumer's Core 0.14 limits. This is the sole remaining known adoption
blocker. [Tooling PR #310](https://github.com/dragginzgame/ic-tooling/pull/310)
aligns Testkit with Wire 0.14 and prepares breaking 0.78.0 notes, leaving actual
package versions unchanged. It is open at
`8a520d66aab2384060a108b2f3dd8a6a082a5667`; no merge or publication was run.

The upstream fix previously passed strict all-target/all-feature Clippy for Testkit, Dev
and the private probe, nine typed transport tests on Rust 1.91, native/Wasm graph
policies, and actual Rust 1.91 Wasm probe compilation. Scheduling's shared native
transport now also checks on Rust 1.91 against that source candidate with the
complete dependency roster, including Endpoint 0.4; the registry lock is restored
afterward. The earlier 71-case native consumer/API/Wire proof retains its original
source override and Endpoint omission. Neither diagnostic establishes published
Testkit compatibility or installed IC recovery. Final registry checks still
select Testkit/Native 0.77.0.

Durable inputs, registry records, failed and passing logs, source archives,
applicable candidate patch and `adoption-retry-receipt.json` are retained under
`.tools/adoption-evidence/ic-scheduling-adopt-now.chfqQmKc/`. Earlier source-fix
and native test evidence remains under `ic-scheduling-cohort.4gI3D2f1` in that
same evidence parent. Its forward patch selects Testkit/Native 0.78.0, pending
upstream release, and Endpoint 0.4.0. Apply it with the historical status hunk
excluded after the aligned Testkit release is published, then qualify the actual
registry transport. Jobs and Timers release metadata remain
unchanged. The working catalog, lock and earlier Memory/Storage source changes
are preserved; only this handoff changes in the ordinary checkout for this retry.

## Platform retry: native client cleared; Wire release still pending — 2026-11-01

The retry uses published Platform 0.17, Runtime/guard 0.17.1, Types 0.16,
Memory/Storage 0.61.1 and Testkit/Native 0.77. The old Testkit client mismatch is
resolved: strict native consumer/transport Clippy and Rust 1.91 transport
compilation pass in the isolated native diagnostic. All 71 consumer/API/Wire and
178 Timers/protocol cases pass (249 total). This diagnostic omits Endpoint only
for the native proof; its manifest and lock are restored afterward. It does not
qualify the final complete graph or installed IC runtime.

The complete currently published graph still fails exact guard admission through
Endpoint 0.3.0. [Wire PR #41](https://github.com/dragginzgame/ic-wire/pull/41) is
merged, but its ledgers select unpublished Core 0.14 and Endpoint 0.4. Fresh
index checks confirm latest Core 0.13.5 / Endpoint 0.3.0. The forward candidate
now declares the coordinated versions and stops explicitly because ^0.14.0
cannot be found. Publication is the remaining known prerequisite; both #86 and
[Tooling #307 — Adopt Platform 0.17's owned PocketIC client in Testkit](https://github.com/dragginzgame/ic-tooling/issues/307)
record the cleared client failure and remaining graph limit.

Retained source, applicable coordinated patch, actual diagnostic lock/metadata,
249-case logs and resolver evidence: `/home/adam/projects/ic-scheduling/.tools/adoption-evidence/platform-runtime-retry.2ZEWTXxO`.
The patch is `coordinated-platform-adoption.patch`, checked against the complete
incoming source; `adoption-retry-receipt.json` separates the native proof from
the unqualified forward cohort. The earlier external temporary candidate was no
longer available, so this retry was reconstructed from public PR #86 and the
preserved incoming source. Evidence is retained inside the ignored tools tree,
outside build targets, as well as at `/tmp/ic-scheduling-platform-retry.XAEhVa09/`.

The incoming lock already selected Memory/Storage 0.61.1 and both facade 0.16/0.17
identities, with the old guard 0.16. It is preserved byte for byte, together with
the direct Platform 0.16 catalog and all earlier source edits. Older qualification
receipts retain their earlier locks. No partial runtime migration, package version
change, commit, push, merge or release was applied. After Core/Endpoint registry
publication, apply/reconcile the prepared candidate and qualify its full graph,
native/Wasm consumers and explicit installed recovery obligations.

## Platform 0.17 adoption prepared; suppliers still block it — 2026-11-01

Platform 0.17.0, Runtime/macros/guard 0.17.1 and Types 0.16.0 are published;
archive checksums and release source identities were independently verified.
The full registry candidate still fails singleton admission: Endpoint 0.3.0
selects the old embedded executor. [Wire PR #41](https://github.com/dragginzgame/ic-wire/pull/41)
owns its migration. Omitting Endpoint only in a temporary resolver diagnostic
admits the new guard, confirming that boundary; the manifest and lock are restored.

A native transport check of that diagnostic exposes a second concrete blocker:
Testkit 0.76.2 expects registry `pocket_ic::PocketIc`, while Platform 0.17 supplies
its owned client. The typed Wire query/update helpers fail E0308. This is tracked
in [Tooling #307 — Adopt Platform 0.17's owned PocketIC client in Testkit](https://github.com/dragginzgame/ic-tooling/issues/307).

The source candidate combines PR #86 with the existing Memory/Storage/Wire work.
Its native DTO fix uses a normal canonical Types 0.16 dependency, retaining the
CDK dependency only on Wasm. Four frozen API/Wire cases and strict Clippy pass on
Rust 1.91 in the separately locked native DTO projection. Provider confinement,
script lint/syntax, document links and patch consistency also pass. These proofs
do not qualify the blocked full graph or installed runtime. PR #86 now records
the two supplier blockers and native fix evidence.

Candidate, applicable delta patch, exact failed resolver/client logs, successful
native proof and incoming backups: `/tmp/ic-scheduling-platform-adopt.mwvJJFYc/`.
The patch is `runtime-adoption-delta.patch`; `adoption-attempt-receipt.json`
records the limited qualification and restoration. The candidate's eventual
Timers minor is 0.36.0 because all executor suppliers must migrate together.
This does not change the actual pending 0.35.5 Wire entry in the current checkout.

The working catalog and lock retain Platform 0.16.0 and the earlier successful
Memory/Storage/Wire selections. Their bytes are unchanged by this attempt. No
partial migration, local supplier override, commit, push or release was applied.
After compatible Endpoint/Testkit publication, refresh the full candidate's
registry lock and qualify the affected native, Wasm and IC runtime boundaries.

## Wire 0.13.5 adoption — 2026-11-01

The root catalog now requires Wire 0.13.5; the incoming lock already selected
Wire/Derive 0.13.5 and is preserved byte for byte. Endpoint 0.3.0 remains the
latest published endpoint selection. Both new core archives were verified
against the official Cargo sparse index. Published source is commit
`a8195d42683f90810a9afd615e4f27e3e5a3c744`; its writer-inlining change preserves
codec bytes, budgets, request identities and error acceptance. No consumer API
migration or protocol regeneration is required here.

Focused qualification passes on Rust 1.91: 45 queue, 19 composition, four API Wire,
three stored Wire and three Timers protocol/checkpoint cases (74 total). Native
Clippy covers the consumer, shared Jobs transport, Timers protocol and runtime
probe; Wasm Clippy covers both canisters, with warnings denied. Actual Wasm
builds pass for the Jobs consumer at release profile and Timers runtime probe at
its timer-probe profile. Dependency graphs, IC boundaries and pin/inheritance
checks pass, retaining the Serde-only Jobs core and sole private timer provider.
No live IC suite ran, and no downstream size/cost improvement is claimed.

Evidence and copies of both qualified Wasms: `/tmp/ic-scheduling-wire.TYxeGMAV/`.
The earlier Memory/Storage 0.61.0 migration and unrelated selections remain
intact. The initial targeted update named the older 0.13.4 lock identity and
refused because 0.13.5 was already selected; no lock mutation was needed.
The separate successful locked fetch and qualification logs retain their scope.

Pending Jobs 0.33.5 notes now include Wire adoption. Timers has a new pending
0.35.5 entry for its private codecs. These are compatible at the published
library boundaries; group package versions remain 0.33.4 and 0.35.4. Changes are
local and uncommitted; no release, push or package publication was selected.

## Memory/Storage 0.61 adoption — 2026-11-01

The root catalog and lock now select published Memory and Storage 0.61.0.
Published archive checksums were verified against the official Cargo sparse
index; the exact release's migration guide was reviewed. Only those two package
records changed from this task's incoming lock. Existing Build/Host 0.73.5,
Testkit/Native 0.76.1, Metrics 0.24.5 and Wire 0.13.4 selections are preserved.
Platform and its runtime guard remain the single matching 0.16.0 identities.

The private Jobs consumer and fixtures use nonzero one-page bucket setup and
validated allocation keys. Fresh cells use `StableCell::create`; reconstruction
uses fallible retained-only `StableCell::load`, preserving the committed queue,
allocation identity and blocked-effect policy. Every Job restore still crosses
`Job::restore`. Public Jobs stays Serde-only, and Timers' provider is unchanged.

Memory-adoption qualification passed on its Wire 0.13.4 lock: 45 native queue cases, 19 composition
cases and three stored Wire cases on Rust 1.91; separate core 1.91 compilation;
consumer 1.91 Wasm compilation; native/Wasm consumer Clippy with warnings denied;
Jobs/consumer and Timers dependency graphs, IC boundaries and pin/inheritance
checks. The existing frozen-memory and corruption/recovery cases are retained.
These native/build checks do not supply a fresh installed IC recovery receipt.

Evidence and incoming backups: `/tmp/ic-scheduling-memory-storage.udQgKScf/`.
Only Jobs has a new pending ledger entry, 0.33.5, because the changed dependencies
serve its private qualification packages. This is compatible at Jobs' published
boundary despite the upstream API hard cut. Group package versions remain Jobs
0.33.4 and Timers 0.35.4; no release, commit or push is selected by this adoption.
The earlier local handoff changes and unrelated lock selections are preserved.

## Canic CI repairs submitted — 2026-11-01

[Canic PR #722](https://github.com/dragginzgame/canic/pull/722) is updated to
`59633d9ce4ad863d897807dbe6aa89f36779fcf9`. Its two timer roster omissions are
fixed, current main's native export/metrics fixture fixes are incorporated, and
shared CI repairs from [PR #726](https://github.com/dragginzgame/canic/pull/726)
are included so its workflow can qualify the combined queue source. PR #726 is
`4b333aae13165c0f49d66ce9667ff60264561724`; review/integrate it first.

The shared fixes address #450/#551: physical recovery-fixture paths, bounded
TLS/download retries, command-owned active PocketIC server lifetime and embedded
peer reproducibility with or without Rust sources. The regenerated peer is
byte-identical to the original hosted rebuild (`58fea38e…`). All focused native,
Wasm reproduction, runner, lint and verification checks pass on their recorded
sources; combined #722 additionally passes all 15 timer inventory cases and its
canonical peer verifier. Its original installed Jobs receipt stays bound to
`4759bec`; no fresh installed result is claimed for the combined head.

Both PRs remain open for hosted CI/review. Issues #450/#551 retain required native
macOS and long governed PocketIC acceptance. No Canic main merge or release was
performed. The normal Canic checkout and its ongoing adoption were left alone.
Sources/evidence remain `/tmp/canic-timer-ci-repair.Ioj4mAWX/`,
`/tmp/ccfix.Smld3Y/` and `/tmp/ic-scheduling-improvements.VvCLTy/`.
The Scheduling release is unchanged; later lockfile edits remain uncommitted.
The concurrent lock now also selects Metrics 0.24.5. This additional selection
was preserved without qualification or release as part of the Canic repair.

## Complexity batch delivered — 2026-11-01

All ten reviewed complexity improvements are delivered on origin/main. Both
canonical group ledgers are finalized and both crates are published:
[IC Timers 0.35.4](https://crates.io/crates/ic-timers/0.35.4) and
[IC Jobs 0.33.4](https://crates.io/crates/ic-jobs/0.33.4). No pending release remains
from this batch. Do not repeat uploads, rewrite tags or replay completed effects.
No sibling repository or Canic integration was changed.

The batch names ordinary completion outcomes and provider binding phases,
resolves watchdog schedules before mutation, unifies cohort selection and typed
measurements, admits whole build identity and names byte-compatible checkpoint
fields. It separates installed subjects from host-only utility tests and isolates
release-fixture phases. Jobs uses typed Wire RPC attributes with explicit IDs,
argument tuples and budgets. Oversized private requests now trap instead of
returning the former guard rejection; state-preservation assertions remain.
Published library APIs, queue formats, restoration and uncertain-effect policy
remain intact. Cargo dependencies and the reviewed tooling snapshot are unchanged.

## Release identities and registry verification

Preparation source is `54028ab5a7e41539348b0975e7a27573026a13ab`.
Timers tag `timers-v0.35.4` identifies `14917d09e7a55c16eda1b7d739f8213ac9d4209e`;
its annotated object is `aec700646fa316e7e7cf88c81b094f417d38cfaf`.
Jobs tag `jobs-v0.33.4` identifies `96f2d94e5af16ffb31c270d6b94a400caf917423`;
its annotated object is `9b1fdfe89c044a2ef6fc3d50ddb221e4ba223302`.
Timers was released/published first, followed by Jobs against Timers 0.35.4.
Each standard release atomically pushed main and its exact tag, followed by the
separate authorized publication command. Fresh GitHub readback confirms both tags.

Fresh registry metadata confirms both versions are present and not yanked.
Downloaded packages match the retained Cargo upload archives and embedded release
VCS identity. Timers SHA-256 is `ed9bf73e5cab8dff71c5b0b922cd61e587aa7212993e7c71929d8c060f61bf27`;
Jobs SHA-256 is `0285cc4debb0076968538599e96224febea2382b75fe2ed3454c8fd96b69596a`.
Cargo retained uploads under `target/package/tmp-crate/` and `tmp-registry/`;
verified independent copies are saved outside target in the delivery evidence.

## Qualification and custody

Both complete standard release gates passed: strict Clippy, full combined CI,
Rust 1.91/native/Wasm, formatting, packaging, graph/tooling/recovery fixtures,
all 35 installed timer runtime subjects and the policy-cohort gate. CI took
378 seconds for Timers and 279 seconds for Jobs. Source-bound qualification is
on the preparation commit and Timers release commit respectively; the standard
metadata receipts bind each final versioned payload.
Jobs' final installed recovery passes all 11 subjects on original, O3, Os and Oz
Wasm, including rollback, restore refusal, lost-result intent and receipts.
These are Linux x86_64 and installed IC observations. Hosted three-host CI runs
asynchronously; this handoff does not claim completed native macOS qualification.

Delivery checkout/evidence: `/tmp/ic-scheduling-complexity-delivery.y314l_zn/`.
Keep `source/` and its complete `.git/release-state/` journals. Per-group
`*-release-evidence/` retains copied journals, final locks/source archives,
runtime/cohort receipts, original Wasms/build stamps and server diagnostics.
`preparation-source.tar`, `jobs-preparation-source.tar` and their matching locks
retain the qualified inputs before each group's version preparation.
Per-group `*-publication-receipt.json`, `*-registry-after/` and uploaded/downloaded
`.crate` copies bind public payload identity. Publication was performed once
per group; later verification only observed those completed effects.

`jobs-live-qualification/` retains all four original Jobs Wasm variants, capture
records, optimizer identity and server/test logs. Original live evidence remains
`/tmp/jobs-recovery.HPXjXO/`. Original timer qualification directories are recorded
in each group's `identity.json` and remain alongside their copied diagnostics.
The metadata transaction cleans successful scratch backups under its existing
contract; Git, retained preparation archives and locks preserve their original bytes.

`incoming-source.tar`, `incoming-manifest.json` and `incoming.diff` preserve the
initial dirty checkout. The synchronization result is recorded in
`checkout-sync-receipt.json`; the retained Git stash backs up the incorporated
batch and concurrent edits; do not replay it over the synchronized checkout. Development
proof remains `/tmp/ic-scheduling-complexity-rest.7bSQcO3E/` and
`/tmp/ic-scheduling-simplification.ut9fdf13/`; it does not replace release receipts.

## Concurrent local work

At release synchronization, the original checkout had received later lockfile
updates while release qualification was running: Build/Host 0.73.5, Memory/Storage 0.60.7 and Testkit/Native 0.76.1.
They were outside the admitted release source. At that point, exactly those eight registry
version/checksum updates were preserved as uncommitted Cargo.lock edits on top of
the released group versions. The newer adoption above supersedes those selections
where named. They have not been qualified or published in this
batch. `concurrent-Cargo.lock`, `concurrent-lock-changes.json` and the rebased lock
in the delivery evidence retain their custody; the sync receipt records the
resulting dirty lockfile and backup stash. Keep this later work for its own
qualification rather than treating the published release receipts as its proof.

## Retained boundaries and older delivery

Keep nine catalog/group members, explicit RELEASE_GROUP, origin/main direct
delivery and separately authorized upload. Jobs remains Serde-only and consumer
owned. All restores use Job::restore; Running/Uncertain effects stay blocked
until exact evidence or owner disposition. Preserve queue, payload and attempt
custody. Logging stays heap-only at 32 KiB; Metrics reset does not clear logs or
reuse IDs. Timers retains the sole private provider, confined FFI and matching
Platform guard. Rust floor is 1.91; native execution alone does not qualify IC
recovery. The Shared 0.27.23 snapshot remains intact; siblings stay read-only.

Earlier Platform 0.16 delivery custody remains
`/tmp/ic-scheduling-platform-delivery.e3mlrmqt/` and the
[committed prior handoff](https://github.com/dragginzgame/ic-scheduling/blob/af71b4b903f3cdc231afad5a56857f3c3692c9da/docs/status/current.md).
Separate Canic [PR #722](https://github.com/dragginzgame/canic/pull/722) retains its
original `4759bec` installed proof at `/tmp/ic-scheduling-improvements.VvCLTy/`.
Its subsequent branch repairs and PR #726 are recorded above; no Canic release is selected.
