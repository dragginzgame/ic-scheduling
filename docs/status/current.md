# Current handoff

## Pending cleanup, Build and Shared adoption — 2026-11-08

Shared Tooling 0.31.11 is adopted at reviewed commit
`6c529e6940a97226cca826726491e03173b08cd2` (158 managed files), including the exact-input
runner/sealer. Snapshot integrity and independent selected-source comparison pass.
Build 0.79.0 remains root-selected for private artifact tools and the release editor.
No removed Build batch API was in use here.

The [complexity/duplication audit](../reports/audits/2026/11/08/complexity-duplication/01/report.md)
is immutable. Its three findings and the subsequent two cleanup items are addressed:

- The roster has 32 common CI plus four installed obligations; proven Jobs-only
  plans require 18 and omit 18. The duplicate core `check` stage is removed;
  standalone `make check`, Wasm, default-feature, archive and installed proofs remain.
- One bounded fixture uses Shared's exact-input contract on qualified Linux
  configurations. Each group admits fresh source/selection/receipts and retains
  the original execution/proof/logs. Version-3 receipts contain durable copies.
  Other stages, including release-tooling, still execute. Custom configuration,
  parallel Make and unqualified hosts execute ordinarily. The reuse regression
  now uses four full-plan attempts and five direct-stage cases, replacing nine
  full-plan attempts while retaining cross-group, failed-stage and recovery checks.
- Private helpers share 14 Jobs clock chains, 44 awaited close chains, 71 Timers
  advancement chains, eight Timers clock conversions and seven ordinary setup
  calls. Timing arguments/order, observation points and teardown remain intact.
  Native fixtures also share 62 present-snapshot lookups, 60 valid-cadence
  constructions and local immutable counter views. Fresh LOC: production 5,465,
  tests 24,463, examples 752, tooling 907, total Rust 31,587, unclassified zero.
  The native cleanup removes 146 test LOC; cumulative cleanup removes 593.
  The proof adapter adds non-Rust tooling.

The earlier pending CI changes remain: one independent redacted full-history
Gitleaks job, authenticated download-cache setup, hash admission, Cargo-owned
formatting and crate/module LOC reports. No live scan/cache-transport pass is claimed.
Delivery is authorized for this workspace. Archive comparison against verified
published archives selects Timers 0.41.2: its native test payload and packaged
Runtime/Guard lock change. Jobs 0.34.14 has identical payload/normalized manifest
apart from VCS provenance; Jobs notes remain pending at 0.34.15, with no release.
The shared frozen flow will qualify and publish Timers to origin/main/crates.io.
A later normal-tree refresh to compatible Build 0.79.1 / Storage 0.68.11 is retained
for the captured gate; earlier focused evidence below does not qualify that graph.
Publication-impact evidence: target/evidence/deploy-maintenance-20261108/.
No public API or retained format changed; delivery has not completed yet.
First capture `batch-9PFRX4TP` stopped at the consumer formatting-evidence fixture
before metadata/tag/push/publication. Its Cargo stub lacked mandatory workspace
metadata and still expected Rustfmt `--all`. The consumer fixture now admits its
one explicit member and checks member-scoped formatting; focused failure/order/
archive-retention proof and ShellCheck pass. Preserve the failed batch and logs
under target/evidence/deploy-maintenance-20261108/; delivery restarts from the fix.

## Focused qualification and evidence

Latest native-only cleanup passes strict all-target Timers Clippy and all 89
`runtime::tests` cases, including generated command sequences, provider failures
and capture teardown. Error/absence checks remain at their original boundaries;
helpers retain call-site panic locations. A later concurrent lock refresh was
preserved; native checks used unchanged SHA-256
`87589aa489c70e521eaf6fa70fbc8fd03718b2caa1f4171b631745db836efabe`.
Logs, LOC and the selected lock: `target/tmp/cleanup-native-fixtures/`.
No public/Wasm source changed; installed proofs below retain their own graphs.

Earlier Linux/Rust 1.99.1 qualification: A concurrent lock refresh to Wire 0.17.14 was preserved;
Clippy and installed qualification used stable lock SHA-256
`9e96a99044f70340c5959f06f74610c15d0eacaa0e9b8a4e13a15b4eeee79838`.
The graph retains Build 0.79.0, Runtime/macros/guard 0.20.1, Storage 0.68.10,
Testkit 0.93.1 and Endpoint/macros 0.8.1. Strict all-target Timers-driver Clippy,
all 35 installed runtime subjects and the policy cohorts/calibrations pass with
PocketIC 16.1.0. Receipts: `target/tmp/timer-qualification-3679316-0/runtime.json`,
`target/tmp/timer-qualification-3788107-0/`, and retained
`target/tmp/cleanup-followup/cohorts.json` (also latest `target/timer-cohorts.json`).

The actual reduced validation fixture passes through Make/Shared, including
invalidation/fallback cases and full-plan cross-group, failure and durable recovery.
Timers stage admission took 206.408s (200.228s execution); fresh Jobs admission
reused it in 6.083s with the same original execution proof SHA-256:
`b78d4d4a0d185164acde55ec3b0986076cf81f31fa9776394c986c43ad2683ef`.
The preceding larger fixture took 271.016s / 6.443s. These are single Linux
stage observations, not a controlled benchmark or complete release receipts.
Syntax, ShellCheck, formatting, links and diff checks pass. Logs, LOC and stage
measurements: `target/tmp/cleanup-followup/`; prior logs remain
`target/tmp/cleanup-three/`, including `final-stage-timing.jsonl` and both original
stage admissions. No prior proof is rewritten or relabelled.

Earlier cleanup qualified all 11 Jobs recovery cases and 35 Timers subjects on
Wire 0.17.13 / lock
`670684d27c1dd4262af10c9ff9462e78d34107da80a9185ec9516e80843b1fea`.
Jobs receipt: `target/tmp/jobs-recovery.cljg2Q/`; Timers receipt:
`target/tmp/timer-qualification-1992037-0/runtime.json`. Both group metadata/editor/
rollback fixtures passed with product gates/publication substituted. Those
assertions are unchanged by this follow-up. Prior Build checks on lock 47fbb591
remain at `target/tmp/adopt-build-shared/` (16 native tests, boundaries, pins,
Actionlint, CI routing and one installed trap/upgrade subject).
No fresh Jobs recovery on Wire 0.17.14, full CI, macOS or release gate is claimed.

## Delivered baseline and recovery custody

Timers 0.41.1 and Jobs 0.34.14 were tagged, pushed to `origin/main` and published
to crates.io from cleanup source `b6e62260000f9136f2b6d4ef6192141d1d46d05b` in
supplier-first `batch-3JHG7IaR`. Its intent is complete at index 2; both group
journals are complete. Normal main was reconciled through the guarded helper.
Do not replay either upload.

- Timers commit: `306bdf7342af9f7fcf8901b5b7573a1e6959c972`;
  annotated tag object: `6ae3266752c5e7d0732246438e1d229cfc8f2158`;
  archive SHA-256: `19fdeddff0a6add2fea8475c3226626bf60bf990cfb566939cf3ea88dc9eddba`.
- Jobs commit: `0ba498e484e8d6e8efc98866f21b3eb743a7fd44`;
  annotated tag object: `fe613b57f8e3960ea629936ba6e6697ae852cbb3`;
  archive SHA-256: `e553cb749e2fded602d016ec6a0dbae52c42132f5c9da49602cd775d693312fc`.

That delivered graph used Build 0.78, Shared 0.31.5, Testkit 0.93.0, Runtime 0.20.0,
Storage 0.68.8 and Wire 0.17.12. Both complete 37-stage release gates passed,
including 35 installed Timers cases and policy cohorts; final Jobs source passed
11 recovery cases, queue and five CLI cleanup modes. Registry/archive/source/tag
identities matched. These proofs do not qualify the current pending graph.
First-attempt PocketIC HTTP 504 failure and successful full-gate recovery remain
retained. Hosted Timers CI (including one retry) and Jobs CI failed on all hosts at
host-tool HTTP 504 before Rust checks; no later hosted pass is claimed here.

A concurrent supplier refresh remains preserved and reapplied: Runtime/macros/guard
0.20.1, Storage 0.68.10, Testkit 0.93.1 and Wire 0.17.13. Original custody is branch
`deferred/batch-3JHG7IaR-suppliers` at `da7c549431c6b791d72fb20059458dcc1972ab4e`
and stash `f54126407ea2376497b35c4f916c17d3aab0e1a2`. It was not in the releases.

Preserve `.git/release-state/frozen/batch-3JHG7IaR/`, including complete intent,
journals/receipts, durable source and `verification/{timers,jobs}/`. Raw release
logs remain at `target/evidence/deploy-cleanup-20261108/`; managed source/builds
remain under the batch's `target/release-frozen/` path. No batch, pin, tag, stash or
artifact was retired. Cleanup measurements, private Wire identity 65 migration,
older batches and issue/Canic history remain in the immutable
[preceding handoff](https://github.com/dragginzgame/ic-scheduling/blob/354dd5143d7c2d848096d836301fd349a68e9ad8/docs/status/current.md).
Jobs remains Serde-only and consumer-owned; restores use Job::restore. Timers keeps
its sole private provider and matching executor guard. Memory stays excluded.
No sibling source or release changed.
