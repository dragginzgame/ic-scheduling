# Current handoff

## Cleanup and Testkit adoption pending — 2026-11-08

The current graph selects Testkit 0.93.0, native Platform 0.25.1 and Storage
0.68.8, retaining Runtime/macros/guard 0.20.0, Endpoint/macros 0.8.0, Wire 0.17.12
and Types 0.18.2. These supplier selections were preserved during cleanup.
Pending notes remain Jobs 0.34.14 and Timers 0.41.1; package versions retain
published values. Public library APIs and retained application/checkpoint formats
are unchanged. Jobs remains Serde-only; Memory remains excluded.

The earlier Timers cleanup centralizes ordinary completion state, scheduling
observations, stale-delivery accounting and capture retention for all six adapters.
The ten follow-up items are addressed: shared probe creation and native typed
owner-slot fixtures, enum-based ordinary termination, common freezing preparation,
checkpoint facts held together, snapshot-owned queue validation, development-only
Jobs Tokio/self-authentication, current-thread Timers Tokio, and typed scenario
commands. Fresh reconciliation keeps its independent empty-slot proof; shared
capture creation does not erase that admission boundary.

The private ordinary observation uses Wire method identity 65, replacing retired
identity 22's four-boolean layout. Rebuild private probes and host decoders
together; no fallback is retained. Checkpoint field order and bytes are unchanged.
Queue reads still restore every JobRecord through Job::restore, and observations
and external effects still follow durable commit.

Focused Linux/Rust 1.99.1 evidence is under
`target/evidence/cleanup-ten-20261108/`:

- Native: 178 Timers, one runtime-probe and four protocol tests; 49 consumer
  tests; the queue parser and installed workflow both pass.
- Strict all-target Clippy passes for the four touched Timers packages and both
  Jobs consumer/harness packages. Formatting, diff, provider confinement and the
  lightweight artifact-capture normal/build graph checks pass.
- All 11 selected Timers installed subjects pass: capture admission/removal,
  ordinary command matrices, both ordinary freezing policies, both Watchdog
  freezing positions and trap/upgrade reconstruction. Runtime receipt:
  `target/tmp/timer-qualification-2521560-0/runtime.json`.
- Jobs Wasm build and artifact capture pass. Three installed recovery subjects
  pass: management/upgrade, invalid restore/controller snapshot preservation and
  lost-result reconciliation without redelivery. The queue example passes against
  the captured `jobs-consumer.wasm` in the evidence directory.
- Initial sandboxed PocketIC execution failed before scenarios at localhost bind;
  the same selected subjects passed with local-network permission. Failed logs
  remain retained alongside the successful run.

Production LOC stays 5,465 (Timers 4,793; Jobs 672). Test LOC is 25,022 → 25,056:
shared scenarios remove repetition, while typed cases and the new Wire boundary
proof add lines. The normal Jobs harness graph loses seven packages, including
SHA-2 0.11 and Tokio macros; Timers qualification drops rt-multi-thread selection.
No production dependency was removed. Cargo-machete 0.9.2 reports no unused direct
dependencies. Raw cloc and feature-tree comparisons are in the evidence directory.
Cargo.lock stayed at SHA-256
`7f2a5319369c9e4a73415917706711de7d20dbb0f9d571bec3bcab63b2e622b7`.
No staging, commit, push, publication, full CI or fresh macOS gate was run.

Earlier Testkit adoption evidence remains at `target/evidence/adoption-testkit093/`,
Jobs bundle `target/tmp/jobs-recovery.aR2P3E/` and timer receipt
`target/tmp/timer-qualification-3374892-0/runtime.json`. It covered all 11 installed
Jobs recovery cases, queue workflow, five CLI cleanup modes and three selected
Timers subjects against native Platform 0.25.0 and lock SHA-256
`2037968d813a8c61f4692b575cfdd950878d730b7b176a2472bb98a39d5a2bb4`;
those broader older checks are not fresh qualification of the current graph.

## Runtime 0.20 adoption delivered — 2026-11-07

Timers 0.41.0 and Jobs 0.34.13 are tagged, pushed to `origin/main` and published
to crates.io. Their different increments used separate captured batches, in
supplier-first order: Timers minor `batch-9DHg5ciE`, then Jobs patch
`batch-XkfdZ7UE`. Both intents are complete at index 1 and both group journals
are complete. Normal main was reconciled through the guarded shared helper after
each publication. Do not replay either upload.

- Timers commit: `c798e0db954a364637021891ddc918a690e32339`;
  annotated `timers-v0.41.0`: `bf34b57d66b33f70a547380f906950e4ab55fcdd`;
  archive SHA-256: `2bfa9d76bea3111771873ab3df2ed2aca99b9a9d8b0e6182f01d09ea29917d1a`.
- Jobs commit: `b2b62bd7bc61c1112f3b6b19c86db60680f2f6f9`;
  annotated `jobs-v0.34.13`: `efed1ad69b9446b14f9323ba60507a8d656166e2`;
  archive SHA-256: `fbcba1318f763e0ff62c9239d20cdabc2a0c560181b63ac19770b61cad58e579`.

Official registry metadata, downloaded archives, both retained Cargo upload
archives, clean VCS provenance and remote annotated tag objects/peeled commits
agree. This is library publication with local installed qualification.

## Consumer migration and supplier graph

Timers 0.41 requires Runtime 0.20 and its matching macros/exact executor guard.
Consumers must align all execution suppliers and Wire Endpoint 0.8; mixing the
old 0.19 executor cohort remains rejected. Scheduling already used Runtime's
retained ordinary `cdk` macro family. Jobs remains Serde-only. Timer APIs, stored
formats, owned Wire payloads, request budgets and private provider confinement
are unchanged; no client regeneration or data migration is introduced.

The released graph selects Runtime/macros/guard 0.20.0, Endpoint/macros 0.8.0,
Wire Core 0.17.12, Storage 0.68.7, Testkit 0.92.0, Types 0.18.2, native Platform
0.24.0, Host Artifacts/FS/Process 0.75.0, Build 0.78.0, Logging 0.24.11 and Metrics
0.29.6. Shared Tooling remains 0.31.5, revision
`b2026a118e38de4af1555c527af327cb604eacdf` (152 managed files). Memory is excluded.

## Fixture correction and release qualification

Adoption source `ee222b54097f677c88906405acb12d5ae19a5673` failed in captured
`batch-HUM_1Mrp` before product preparation/delivery: a patch fixture inherited
the product minor draft. Fix source `9e95ca670d84ef59ae0bc293ba01dacbc2d964cd`
gives the fixture its own ledgers, preserving product conflict refusal and all
recovery assertions. Preserve the failed batch; do not resume its superseded
source as another delivery. Corrected source received fresh complete gates.

Both complete 37-stage Linux/Rust 1.99.1 release gates passed, each including all
35 installed Timers runtime cases and policy cohorts. Release-tooling fixtures
passed rollback, captured-source isolation, interrupted/uncertain publication,
second-group recovery, completed resume and reconciliation. Their substituted
uploads remain distinct from the actual registry verification above.

Full gate/runtime receipt paths and final Jobs installed evidence remain in the
immutable preceding handoff linked below and the durable verification custody.
Final Jobs commit `b2b62bd` passed all 11 recovery cases, queue and CLI cleanup.

Final released lock SHA-256:
`0a84b6e84261a4c0f0f11e145cb3396dcc747b438a1f9d326ab6d3d9442df1a6`.
Normal raw release logs: `target/evidence/deploy-runtime020/`. Earlier adoption
and refusal/404 observations remain historical. Hosted CI observations are in the
preceding handoff; local Linux checks establish no fresh macOS or cross-release
installed migration result.

## Custody and deferred Storage update

Preserve `.git/release-state/frozen/{batch-9DHg5ciE,batch-XkfdZ7UE}/`: durable Git
stores, complete intents, group journals/receipts and `verification/{timers,jobs}/`
with registry metadata, three matching archives, VCS provenance, remote refs,
runtime/cohort receipts and final installed Jobs proof. Managed delivery source
and build outputs remain under their `target/release-frozen/` batch paths.
No retirement or deletion was performed, including failed `batch-HUM_1Mrp`.

A concurrent update selected Storage 0.68.8 in the normal lock during Jobs delivery.
The captured release remained at 0.68.7. The later edit was preserved on branch
`deferred/batch-XkfdZ7UE-suppliers`, stash
`c6a9dd954bd2e0aafd1b8e743e7dcec24343eaad`, and raw patch
`target/evidence/deploy-runtime020/deferred-storage.patch` (also retained in Jobs
verification custody). After guarded reconciliation only that supplier hunk was
reapplied, preserving released group versions. It remains uncommitted in the
pending Testkit graph above and was not published in those releases.

Prior batches, supplier branches/stashes, release identities and issue/Canic
history remain preserved and linked in the immutable
[preceding handoff](https://github.com/dragginzgame/ic-scheduling/blob/61f9d2295fd6ef1fdf61dd252b9be56bb41aa460/docs/status/current.md).
No older tag, upload or receipt was rewritten. No sibling source or release changed.
