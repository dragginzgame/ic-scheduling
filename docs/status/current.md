# Current handoff

## Timers 0.40.3 delivery in progress — 2026-11-04

The requested Platform refresh selects Runtime/macros/exact guard **0.19.1** in
the root lock. Official registry metadata confirms all three checksums and no
yanks. Keep the compatible Runtime 0.19.0 manifest minimum: this supplier patch
moves tests without changing execution behavior. Types remains 0.18.0 and the
native facade remains 0.21.0; the supplier's facade 0.22 draft is not adopted.
Shared Tooling remains unchanged. Timers **0.40.3** has pending dependency/cleanup
notes; package versions remain the delivered versions below.

Focused Linux Rust 1.99.1 checks passed: 178 Timers library tests, Wasm checks for
the Jobs consumer and Timers runtime probe, and the Timers host/Wasm dependency
policy (one matching registry Runtime/guard cohort). Scratch evidence is in
`target/platform0191/`. Ordinary stop/cancel handling now shares one control
transition. Watchdog requests, cancellation, dispatch and completion effects live
with Watchdog control; registry claim/lifetime admission, observation ordering and
provider cleanup stay intact. The 178 Timers tests and strict package Clippy pass
for the cleanup (`target/cleanup0403/`); final Clippy also admits the narrowed
private helper visibility. Registry main shrank 1,472→1,326 physical lines, mostly
by moving transitions; no total-code or performance reduction is claimed.

The follow-on four-candidate review retained three changes: shared policy-result
construction/normalization, one Watchdog callback adapter and ordinary-probe enum
ownership/reconciliation helpers. The capability-method macro was evaluated and
discarded: it added nine code lines and indirection, so those forwarders remain
explicit. This slice reduces non-test LOC 10,959→10,947, preserving all 371 source
test functions. All 178 Timers unit tests, eight doctests, strict library/probe
Clippy and the runtime-probe Wasm check pass. Generated Rustdoc preserves all 27
reviewed public method signatures/documentation blocks. Focused evidence and the
comparison are in `target/cleanup-four/`; no new installed IC claim is made.

The owner requested deployment after cleanup. The selected Timers patch includes
the current IC Build 0.74.3 and Wire/Derive 0.16.2 lock refreshes alongside Platform
and cleanup. The complete five-stage gate must qualify this exact graph before
release/push and separate crates.io publication. Jobs stays 0.34.3; siblings remain
read-only. No fresh installed or release pass is claimed yet. Compact custody:
`.git/release-state/deployment-timers0403.XBMtMHVE/`. The preceding 0.40.2 handoff
commit `af1ff7d` reached `origin/main` after a transient GitHub rejection.

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

The [delivered cleanup handoff](https://github.com/dragginzgame/ic-scheduling/blob/af1ff7db20125e29edc82ad9323c317b6cd1de6e/docs/status/current.md)
retains the matched measurements on the earlier Host 0.74.2 graph: runtime LOC
10,968→10,923, with 325 more code-section bytes per policy. These are historical
measurements, not qualification of the pending graph or a performance gain.

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
