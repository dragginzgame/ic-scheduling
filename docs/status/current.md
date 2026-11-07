# Current handoff

## Storage/Testkit adoption and release preparation — 2026-11-07

The requested delivery selects Timers 0.40.12 followed by Jobs 0.34.11, with
crates.io publication through Shared's captured batch flow to `origin/main`.
Package versions remain at their delivered base until the standard runner bumps
each group. No new release or publication has happened yet.

The root catalog/lock select Storage 0.68.5, Testkit 0.91.0, Host Artifacts/FS/Process
0.75.0 and Build 0.78.0. Testkit now shares the Host 0.75 error identities; the old
Host 0.74 copies are gone. Runtime/macros/exact guard 0.19.5, Types 0.18.2,
Platform 0.23.6, Wire 0.17.8, Endpoint/macros 0.7.3, Logging 0.24.11 and Metrics
0.29.6 are retained. Jobs stays Serde-only; public APIs and stored formats are
unchanged. Shared Tooling's latest committed revision remains 0.31.5,
`b2026a118e38de4af1555c527af327cb604eacdf` (152 managed files).

The cleanup narrows notification fixtures to validated composition/recovery;
the actual Queue owns management/auth/pagination. Timers shares capacity, capture
and stopping fixtures, preserving typed controls, classifications, destruction
ordering and separate Watchdog messages. Four Cargo-owned retired-feature checks
were removed; provider/dependency guards and distinct recovery fixtures remain.
Jobs now shares one integration target with all 48 cases/frozen records. Installed
tests combine 27 adjacent advance/tick pairs with unchanged budgets and observations.
Shared cloc: tests 25,608 → 25,009 (599 fewer), library/app unchanged at 5,494.

Before release capture, Storage's 49 native Queue and 14 composition tests pass;
the Testkit 0.91 recovery consumer compiles. Testkit's selected CLI is prepared.
Current adoption/preparation evidence: `target/evidence/deploy-storage0685-testkit091/`.
Earlier cleanup and Host checks remain historical at `target/evidence/cleanup-shared0315/`,
`cleanup-fixtures/`, `cleanup-progress/` and `adoption-host075/`. Their older lock
identities are superseded by this release candidate; they are not release gates.

## Jobs 0.34.10 and Timers 0.40.11 delivered — 2026-11-07

Both groups are tagged, pushed to `origin/main` and published to crates.io through
Shared Tooling 0.31.1's captured batch flow. Source adoption:
`c6eb119e89c80e8866768da279e69a0fce499e8c`. Batch `batch-vLPyIAUg` is complete
at index 2. Both standard release journals are complete; do not replay uploads.

- Timers release: `ea121275b86c5dc1bf88f6174118daa5ec214938`;
  annotated `timers-v0.40.11`: `89e6d1867f47a2753b5e0904a18d3536fd85283c`;
  archive SHA-256: `2dbebf8c69cb379be40c7783047b107d51e4a8721d736a7ae54a694d914b8c42`.
- Jobs release: `3b74e30425df968ae524a8b2dbc028199532bd34`;
  annotated `jobs-v0.34.10`: `820e7697fdd21bafbaee7ac6f4fc932bf7b0d4ba`;
  archive SHA-256: `dd36beb40f81c744eb16cb85fe2987d3c700d5927e4a7204ef993fa59460c253`.

Official registry metadata, downloaded archives, both retained Cargo publication
archives, clean VCS provenance and remote annotated tags agree. Main was reconciled
through the guarded shared command. This is library delivery, with local PocketIC
qualification; it does not establish production application deployment.

## Released supplier graph and Testkit migration

The delivered graph used Build 0.77.0 and Testkit 0.90.0. Runtime/macros/exact
guard 0.19.5, Types 0.18.2, Platform 0.23.5, Storage 0.68.3, Wire 0.17.6,
Endpoint/macros 0.7.2, Logging 0.24.11, Metrics 0.29.6 and Host 0.74.4 are retained.
Shared's reviewed snapshot is 0.31.1,
`5efbda2ab81ad84e30f490f3137d8ef083c303d2` (152 managed files).

All three native PocketIC constructors use explicit positive 60-second request
budgets. Startup deadlines, server limits and explicit instance teardown remain
separate. No attachment/listing callers or retired unbounded settings remain in
maintained callers. Build's removed forwarding getters were unused. Public Jobs
and Timers APIs, owned Wire bytes and retained formats are unchanged; Jobs stays
Serde-only and the retired Memory package remains excluded.

[#89](https://github.com/dragginzgame/ic-scheduling/issues/89) is closed as completed
with delivered-source, registry/archive and actual consumer qualification evidence.
Both finalized group ledgers link the issue. Active README/architecture guidance
names Testkit 0.90 and Endpoint 0.7; dated adoption records retain their original
source and dependency scope.

## Qualification and evidence

Both complete 37-stage Linux/Rust 1.99.1 release gates passed. Each includes all
35 installed Timers runtime cases and policy cohorts. Original focused adoption
checks passed strict selected-consumer lint, 18 native cases, six installed cases,
Wasm and dependency/guard admission. Both real metadata fixtures passed, with
product gates and uploads substituted; those fixture passes are distinct from the
complete actual release gates.

Against final Jobs release `3b74e304`, all 11 installed recovery cases, the queue
workflow and the process-level CLI cleanup test covering five exit/interruption
modes pass. The final released lock SHA-256 is
`7f3908c7b0d42332de1feac3cbf86e26dfd90cd5f6f96ea29efa291c4dd92d77`.

Captured validation roots under `target/release-frozen/batch-vLPyIAUg/target/`:

- Timers: `release-validation/validation.cSObfZ4p/`, actual run
  `logs/validation.Llj8g5`; source `c6eb119`;
  runtime receipt `tmp/timer-qualification-2858500-0/runtime.json`.
- Jobs: `release-validation/validation.PaFAHov6/`, actual run
  `logs/validation.OPuQOz`; source `ea12127`;
  runtime receipt `tmp/timer-qualification-2352605-0/runtime.json`.
- Final Jobs installed evidence: `tmp/jobs-recovery.F8VTjs/`, also copied into
  durable verification custody below. Queue and CLI logs share the normal
  `target/deployment-build077-testkit090/jobs-final-installed.log`.

At delivery, hosted Jobs [run 42363044232](https://github.com/dragginzgame/ic-scheduling/actions/runs/42363044232)
was in progress; the preceding Timers run was cancelled. These historical statuses
establish no fresh hosted/macOS or cross-release installed migration pass.

## Normal checkout and deferred supplier updates

A concurrent updater selected Platform 0.23.6 and Storage 0.68.4 in the normal
lock during delivery. The captured release graph and publication archives remained
unchanged. The later refresh was preserved, main was fast-forwarded through
`make release-batch-reconcile BATCH=batch-vLPyIAUg`, and only those supplier hunks
were reapplied. Released package versions were retained. The refresh remains
uncommitted and unqualified for a later batch; it was not uploaded in these releases.

Preservation: branch `deferred/batch-vLPyIAUg-suppliers`, stash
`927ee1cd6ee829caeee92e5a0950932d829b2c3d`, and raw patch
`target/deployment-build077-testkit090/deferred-suppliers.patch`.
Do not restore a whole old lock over the current group versions.

## Recovery custody

Preserve `.git/release-state/frozen/batch-vLPyIAUg/`: its durable `repository.git`,
complete intent, group journals and source-bound validation receipts, plus
`verification/{jobs,timers}/` with official metadata, archives, provenance, remote
refs, runtime/cohort receipts and final installed Jobs proof. Its managed checkout
and build artifacts remain at `target/release-frozen/batch-vLPyIAUg/`. No retirement
or cleanup was performed. Normal raw logs are under
`target/deployment-build077-testkit090/`; initial adoption logs/input snapshots
remain under `target/evidence/adoption-build077-testkit090/`.

Earlier custody is unchanged: completed `batch-hCRiwOcg`, failed `batch-qIKZnu6X`,
branch `deferred/shared031-6456b5a`, stash
`96492f00b7f0e99d13898109e413bd0b6acb412f`, and the older release/stash records
remain preserved. Their exact versions, hashes, recovery paths and limitations
are in the immutable
[preceding handoff](https://github.com/dragginzgame/ic-scheduling/blob/c6eb119e89c80e8866768da279e69a0fce499e8c/docs/status/current.md).
No older upload, release tag or receipt was rewritten.

Canic owns follow-up issues
[#735](https://github.com/dragginzgame/canic/issues/735),
[#736](https://github.com/dragginzgame/canic/issues/736) and
[#737](https://github.com/dragginzgame/canic/issues/737). Their local fixes and
retained tests were reviewed read-only; no Canic source was changed here.
