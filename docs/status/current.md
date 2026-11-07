# Current handoff

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

Adoption source: `ee222b54097f677c88906405acb12d5ae19a5673`. The first Timers
capture, `batch-HUM_1Mrp`, failed `test-release-tooling` before product version
preparation or delivery: its patch fixture inherited the pending product 0.41.0
notes and rejected them as a competing version. Fix source
`9e95ca670d84ef59ae0bc293ba01dacbc2d964cd` gives that fixture its own patch
ledgers. Product version selection, changelog conflict refusal and all recovery
assertions remain intact. The corrected source received fresh complete gates;
the planned release versions were retained. Preserve the failed batch and its
retained fixture/logs; do not resume its superseded source as another delivery.

Both complete 37-stage Linux/Rust 1.99.1 release gates passed, each including all
35 installed Timers runtime cases and policy cohorts. Release-tooling fixtures
passed rollback, captured-source isolation, interrupted/uncertain publication,
second-group recovery, completed resume and reconciliation. Their substituted
uploads remain distinct from the actual registry verification above.

Captured gate roots under `target/release-frozen/`:

- Timers: `batch-9DHg5ciE/target/release-validation/validation.wADXTW7H/`, full run
  `logs/validation.grUd7z`; source `9e95ca6`. Runtime/cohort receipts under that
  batch's `target/tmp/timer-qualification-{2982313,3057304}-0/`.
- Jobs: `batch-XkfdZ7UE/target/release-validation/validation.RIkeUrim/`, full run
  `logs/validation.rxNz41`; source `c798e0d`. Runtime/cohort receipts under that
  batch's `target/tmp/timer-qualification-{2728418,2808265}-0/`.
- Final Jobs commit `b2b62bd`: all 11 installed recovery cases, queue workflow and
  CLI cleanup across five exit/interruption modes pass. Recovery bundle:
  `batch-XkfdZ7UE/target/tmp/jobs-recovery.THwSWC/`; normal combined log:
  `target/evidence/deploy-runtime020/jobs-final-installed.log`.

Final released lock SHA-256:
`0a84b6e84261a4c0f0f11e145cb3396dcc747b438a1f9d326ab6d3d9442df1a6`.
Normal raw release logs: `target/evidence/deploy-runtime020/`. Earlier focused
adoption evidence remains under `target/evidence/adoption-runtime020/endpoint-ready/`;
older refusal/404 observations are historical. Hosted Jobs
[run 42397636843](https://github.com/dragginzgame/ic-scheduling/actions/runs/42397636843)
was in progress at delivery. No fresh macOS or cross-release installed migration
result is claimed from the local Linux checks.

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
reapplied, preserving released group versions. It remains uncommitted and
unqualified for a later batch; it was not published in these releases.

Prior batches, supplier branches/stashes, release identities and issue/Canic
history remain preserved and linked in the immutable
[preceding handoff](https://github.com/dragginzgame/ic-scheduling/blob/9e95ca670d84ef59ae0bc293ba01dacbc2d964cd/docs/status/current.md).
No older tag, upload or receipt was rewritten. No sibling source or release changed.
