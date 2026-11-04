# Current handoff

## Timers 0.40.0 delivered — 2026-11-04

Timers **0.40.0** is pushed and published to crates.io. Release commit
`01fe9f0c71a43137874578b3126d1058f77e1549` and annotated tag `timers-v0.40.0`
were delivered atomically to `origin/main` from this normal checkout; the journal
is complete. The separate `make RELEASE_GROUP=timers publish` succeeded from the
clean tagged HEAD. Official registry checksum, retained archive and clean VCS
source agree. Do not repeat the completed upload.

The public Metrics dependency is now **0.29**: consumers align direct recorder
types and own report DTOs, codecs, validation and mean/interval arithmetic.
No removed recorder Serde, reporting or reconstruction API is restored. Timer
recording, overflow quality and scheduling behavior are unchanged. Jobs retains
its consumer-owned report/Wire contract and public Serde-only library. Jobs stays
**0.34.2** with **0.34.3** notes pending; no Jobs release or upload occurred.

Source preparation is `d5cf1fa`. Its fresh full five-stage gate passed on Linux
with Rust 1.99.1: all 30 common CI obligations, Testkit CLI/server admission,
all 35 installed watchdog subjects and the policy cohort. This qualifies Metrics
0.29.0, Testkit 0.84.1, Platform's native facade 0.21.0, toml 1.2.0,
toml_edit 0.25.18 and toml_writer 1.2.0 together. Runtime/macros/guard 0.19,
Types 0.18 and server 16.1.0 remain selected. Publication passed final Clippy and
normalized archive compilation at 0.40.0. Hosted Linux/macOS CI is asynchronous;
no new macOS or installed Jobs recovery result is claimed here.

Durable evidence is under
`.git/release-state/deployment-timers0400.IkyEFM1l/`: incoming source/lock,
supplier registry records, release/publication logs and successful statuses,
checksum-matched published archive with VCS/dependency inspection, exact remote
tag objects, watchdog diagnostics and cohort report. Canonical completed plan
and version-2 validation receipts remain at `timers-0.40.0.*` beside that custody.

## Timers 0.39.1 delivered — 2026-11-03

Timers **0.39.1** is released and published to crates.io. Commit
`b2c34a636f05e7ff3abea39141ec412f7f764992` and annotated tag `timers-v0.39.1`
were pushed atomically to `origin/main`; the release journal is complete.
The separate `make RELEASE_GROUP=timers publish` succeeded from that clean tagged
HEAD. The official registry checksum matches the retained archive, whose VCS
metadata identifies the release commit. Do not repeat the completed upload.

Both native harnesses and the Jobs queue example now use Testkit 0.84.0's
maintained `ic_testkit::pocket_ic` client; the separate Platform client dependency
is removed. Jobs' explicit `harness` feature keeps default artifact capture
lightweight. Jobs remains **0.34.2** with **0.34.3** notes pending; no Jobs release
or upload was performed. Source preparation is committed at `720ed0c`.

The fresh standard five-stage gate passed with Rust 1.99.1 on Linux: all 30 common
CI obligations, selected Testkit CLI/server setup, all 35 installed watchdog
subjects and the policy cohort. Publication passed final Clippy and normalized
archive verification at 0.39.1. Build 0.74.2, Wire/Derive 0.16.1 and Endpoint
0.6.1 were retained and qualified with this graph. Runtime 0.19, Types 0.18,
Metrics 0.28, Storage 0.66 and server 16.1.0 remain selected. Hosted Linux/macOS
CI runs asynchronously; no new macOS result or Jobs installed recovery is claimed.

Durable evidence is under
`.git/release-state/deployment-timers0391.k7IkTIwf/`: incoming source and lock,
release/publication logs and successful statuses, official registry readbacks,
checksum-matched archive and VCS/dependency inspection, exact remote tag objects,
watchdog diagnostics and cohort report. Canonical completed journal and version-2
validation receipts remain at `timers-0.39.1.*` beside that custody directory.
Earlier evidence below retains its original source and Testkit 0.83 graph.

## Previous delivery — 2026-11-03

Both Scheduling release groups are pushed and published to crates.io:

- Timers **0.39.0**, commit `0eae01ec5a0c9288f3f736e0f63b21691f1408a2`,
  annotated tag `timers-v0.39.0`.
- Jobs **0.34.2**, commit `5066ac8a6c9010155ffe1b23d38794b6349f6286`,
  annotated tag `jobs-v0.34.2`.

The normal checkout is `/home/adam/projects/ic-scheduling` on `main`, delivered
straight to `origin/main` (`dragginzgame/ic-scheduling`) after the owner's direct
push request. Each standard release made its own atomic branch/tag push and
completed its journal. Each separate `make publish` admitted the clean tagged
HEAD and uploaded only its selected public crate. Official registry records and
retained archive checksums agree. Do not replay those completed uploads.
Publication was admitted at each release HEAD before this handoff.

[#88 — Consolidate Jobs/Timers plan evidence](https://github.com/dragginzgame/ic-scheduling/issues/88)
is closed by source commit `7816c9aa2bc81cf414a90fdade60d8c2652cf23d` on public
main. Earlier source preparation is retained in `643f9b8` and `af87eab`.
No sibling source, supplier release or alternate delivery checkout was changed.

## Previous release qualification and evidence

Both fresh standard release gates passed with the new adapter: full repository
CI, selected tool/server admission, all 35 installed Timers watchdog subjects
and policy cohorts. Full CI records 30 common obligations, including snapshot
admission, native/Wasm lint and tests, dependency/provider policies, consumer
contracts, documentation and both normalized package archives. Publication also
passed final Clippy and normalized archive verification at each release version.

The 44 Jobs installed recovery cases across original/O3/Os/Oz Wasm passed at
`af87eab`. Product source and supplier selections stayed unchanged through #88;
its tooling-only changes passed mapping, real CI input-drift, metadata rollback,
interrupted journal recovery and publication-admission fixtures. Those fixtures
use the actual Shared runner/verifier and Cargo editor/Git; product/setup/uploads
are substituted. Canonical release metadata projection was admitted separately.

Local runtime evidence is Linux/PocketIC 16.1.0. Hosted CI runs asynchronously on
Linux, macOS Apple Silicon and Intel; consult the current GitHub run rather than
claiming a pending matrix passed. Same-source install/upgrades do not establish
deployed old-Wasm migration, rollback or destructive retirement of retained data.

Durable evidence is under
`.git/release-state/deployment-current.0NCjlIWS/`: source preservation, #88 patch
and focused results, complete release/publication logs, registry responses,
checksum-matched published archives, installed diagnostics and cohort reports.
Actual completed plans/file rosters and version-2 receipts remain beside it at
`timers-0.39.0.*` and `jobs-0.34.2.*`. Historical version-1 receipts and interrupted
qualification remain preserved with their original source/adapter.
The owner selected Logging 0.24.5 after an unexpected lock change; both old/new
locks and interrupted evidence are retained. Its verified Rust source matches
0.24.4. Development `target/` evidence remains removable by normal cargo clean.

## Current workspace and validation ownership

There are eight workspace members, with explicit matching versions per group
and no root workspace version. Jobs owns three members; Timers owns five.
`ic-jobs-canister-tests::harness` replaces the old support crate behind the
explicit `harness` feature. Recovery/queue/process targets and launchers select
it; the default artifact-capture graph remains unchanged and lightweight.

Shared Tooling is 0.28.2 at `5cf67f81c50c56ac17389ae71a3a87d46e86e8a2`
(145 canonical payloads). Local supplier selections are Logging 0.24.5, Testkit
0.84 with its owned PocketIC client, Metrics 0.29, Runtime/macros/guard 0.19, Types 0.18,
Storage 0.66, Wire/Derive 0.16 and Endpoint 0.6. Rust 1.99.1 remains pinned.
Published Timers 0.40 exposes Metrics 0.29 identities and Runtime 0.19.
Downstream consumers must align direct Metrics types
and their Runtime/Endpoint suppliers before adoption.
No retired Memory, Stable Structures or Testkit Native crate is in the graph.

`ci/release-validation.json` is the sole complete roster: 30 common obligations
and four standalone Timers stages. Normal CI and Jobs-only releases execute the
same complete common CI. Generated release plans retain aggregate `ci` plus the
four standalone stages. Clean Jobs-only scope uses its annotated current tag;
shared/unknown/dirty changes and Timers retain all five. See
[the consumer mapping](../release-validation-groups.md).

Shared owns baseline/roster observation, bounded JSON/path/hash mechanics and
run-evidence verification. Scheduling owns product selection, complete input
coverage, host/compiler/configuration custody and exact metadata projection.
Version-2 receipts bind the immutable Shared proof and all retained bytes, admit
Git-owned relocation, and reject missing inputs or incomplete generated policy.
No stage-proof reuse or historical-receipt conversion is enabled.

## Preserved constraints and history

Jobs remains Serde-only with consumer-owned metadata, storage and lifecycle;
every restore uses `Job::restore`. Timers retains the sole private provider and
unsafe FFI confinement, exact Runtime guard and explicit installed qualification.
Retained 0.64 Cells preserve blocked attempts/receipts under Storage 0.66. Retired
Memory images remain refused without writes/reset; uncertain effects stay blocked
until explicit disposition. No consumer migration or destructive retirement is
authorized without owner disposition of outstanding effects.

The ten simplifications and their
[measurements](../redundancy-measurements-2026-11-03.md) remain in source. Those
comparisons use the older frozen Runtime 0.18 graph, not the current cohort.
Preserve `.git/release-state/deployment-timers0381.jnjcs5qg/`,
`deployment-upstreams.UjAXfFHz/`, `deployment-msrv-87.uUHPASl6/` and
`deployment-wire-tooling.w6Z6gClk/`, including journals, receipts, archives and
incoming work. Do not pop preservation stashes containing delivered source/old
versions. Historical detail remains at `1e9821379:docs/status/current.md` and
`af0e67d:docs/status/current.md`.
