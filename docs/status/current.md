# Current handoff

## Timers 0.40.2 delivered — 2026-11-04

Timers **0.40.2** is pushed and published from the normal checkout on `main` to
`origin/main` (`dragginzgame/ic-scheduling`). Source preparation is
`0bc7ddf644dff0c0ce8f5df7cb7e326f1a27f01d`; release commit is
`dc0c2cd59018a02aff68bf06911b35afc4f73b24`, annotated tag `timers-v0.40.2`.
The canonical journal is complete. Separate `make RELEASE_GROUP=timers publish`
succeeded from the clean tagged HEAD. The official checksum matches the retained
archive and its VCS source. Do not repeat the completed upload.

The owner explicitly included the independent patch refresh: Logging 0.24.6,
Metrics 0.29.1, Storage 0.66.2 and Host Artifacts/FS/Process 0.74.3. All six official
registry checksums match the selected lock. Jobs source/library contract stayed
unchanged at **0.34.3**; no Jobs publication or sibling release was performed.
Runtime/macros/guard 0.19, Types 0.18, Testkit 0.84.1 and server 16.1.0 remain selected.
Shared Tooling remains 0.29.4 at `628ccc75d813bfd9bae777bfbaef55838d25b1b2`.

## Qualification and cleanup

The fresh five-stage gate passed on Linux with Rust 1.99.1: all 30 common CI
obligations (671s), selected Testkit CLI/server admission, all 35 installed watchdog
subjects (58s) and the policy cohort (55s). Common CI includes all 178 Timers unit
cases, native/Wasm consumers and both normalized archives. Publication passed final
Clippy and normalized archive compilation at 0.40.2. These qualify the selected
graph and exact source; no new installed Jobs recovery or macOS pass is claimed.
Hosted Linux/macOS CI remains asynchronous. Same-source upgrade cases do not
establish deployed old-Wasm migration, rollback or destructive data retirement.

Ordinary control owns one Ensure/Reconcile selection, pending-command arbitration
and completion planning. Registry entry/provider mechanics and observation
projection live in a private child. Counter fields/defaults/getters have one
declaration; event classification and Metrics arithmetic stay explicit. Public
exports, scheduling rules, callback capture cleanup and provider authority are
preserved. No draft remains for this delivered batch.

Historical matched cleanup measurements use the earlier incoming graph with Host
0.74.2, not the subsequently selected release graph: runtime LOC 10,968→10,923;
registry main-file physical lines 1,858→1,472. The 304 entry/provider and 58 planning
source lines are moves. All 371 source test functions remain. Rust 1.99.1
`timer-probe` used thin LTO, opt-level z, panic abort, symbol stripping and no
post-link optimizer. Raw module/code-section payload bytes were:

| Policy | Module before → after | Code before → after |
| --- | ---: | ---: |
| Once | 120,631 → 120,959 | 105,445 → 105,770 |
| AfterCompletion | 120,644 → 120,980 | 105,419 → 105,744 |
| Watchdog | 124,137 → 124,465 | 108,828 → 109,153 |

Code grew 325 bytes per policy. No speed or instruction-cost improvement is
claimed; those measurements are not relabelled as current-graph qualification.

## Durable custody and current boundaries

`.git/release-state/deployment-timers0402.G6arr0ab/` retains incoming source and
old/new locks, owner-selected scope, supplier registry records, release/publication
logs and statuses, checksum-matched published archive, VCS/dependency inspection,
installed watchdog/cohort diagnostics and a compact historical comparison.
Completed `timers-0.40.2.*` plan/version-2 receipts retain their exact source.
Default retained-state admission passes without cleanup. Compiled comparison
fixtures and development scratch remain removable under workspace `target/`.

Eight members retain explicit matching versions in independent Jobs (three) and
Timers (five) groups; no root workspace version. `ic-jobs-canister-tests::harness`
keeps recovery support explicitly selected and default artifact capture lightweight.
Jobs stays Serde-only with consumer-owned metadata/storage/lifecycle; every restore
uses `Job::restore`. Uncertain effects require explicit disposition. Timers owns
the sole private provider, unsafe FFI confinement and exact Runtime guard.
Storage 0.66 retains admitted 0.64 Cells and blocked attempts/receipts; retired
Memory images are refused without writes/reset. No Memory, Stable Structures or
Testkit Native package is selected. No destructive retirement or consumer migration
is authorized by cleanup. Canic integration remains Canic-owned; the
[0.39 → 0.40 checklist](../timers/adoption/0.40.md) describes consumer qualification.

`ci/release-validation.json` owns 30 common obligations and four standalone Timers
stages. Clean Jobs-only scope uses its annotated current tag; shared/unknown/dirty
changes and Timers keep all five stages. Shared owns bounded proof/evidence
mechanics; Scheduling owns product selection, complete inputs, compiler/host/config
custody and exact metadata projection. Version-2 receipts bind immutable Shared
proof and retained bytes. No stage-proof reuse or historical-receipt conversion is
enabled. See [the group mapping](../release-validation-groups.md).

## Earlier deliveries and protected history

Timers **0.40.1** (`f03d430`) and Jobs **0.34.3** (`0eec50a`) remain pushed/published
with completed journals. Custody: `.git/release-state/deployment-consumer-support.lHbrHngp/`.
The earlier Jobs GitHub push failures were reconciled and delivered by canonical
resume. [The preceding handoff](https://github.com/dragginzgame/ic-scheduling/blob/427dbd98c95c04d19cc0c30a7c23bf899084da04/docs/status/current.md)
retains exact release/qualification details. Timers 0.40.0/0.39.1 and prior Jobs
releases are also complete; never replay those uploads.

Preserve `.git/release-state/deployment-timers0400.IkyEFM1l/`,
`deployment-timers0391.k7IkTIwf/`, `deployment-current.0NCjlIWS/`,
`deployment-timers0381.jnjcs5qg/`, `deployment-upstreams.UjAXfFHz/`,
`deployment-msrv-87.uUHPASl6/` and `deployment-wire-tooling.w6Z6gClk/`, including
journals, receipts, archives and incoming work. Preserve version-1/interrupted
proofs and stashes with their original source; do not pop delivered/old versions.
The [earlier ten simplifications](../redundancy-measurements-2026-11-03.md) retain
their frozen Runtime 0.18 graph. Historical handoffs remain at `1e9821379` and
`af0e67d`; prior 44-case installed Jobs recovery keeps its source at `af87eab`.
