# Current handoff

## Timers 0.40.3 delivered — 2026-11-04

Timers **0.40.3** is pushed and published from this normal checkout on `main` to
`origin/main` (`dragginzgame/ic-scheduling`). Source preparation is
`b556199feae4d1ca2235fd20b6fe718a19136a2a`; release commit is
`670cd0d00cf496e9252fb77a3048ffd151d89eb8`, annotated tag `timers-v0.40.3`
(object `d1e08087df6390dc74ab98f51d4c3269facf00de`). The canonical journal is
complete. Separate `make RELEASE_GROUP=timers publish` succeeded from the clean
matching tag after the owner explicitly authorized crates.io publication.
Automatic approval initially rejected the upload; that command did not execute.
The later authorized upload completed. Do not repeat it.

The official registry checksum matches the retained archive and its clean VCS
source at that release commit: `decfcea289e39056a84a13dc228040553f4be61a6775026db03eeaa4fddc660a`.
The archive retains Rust 1.99.1 and only Metrics, Platform Runtime and Thiserror
as normal dependencies. Jobs remains **0.34.3**, without a new publication.
There is no pending draft for this delivered batch and no sibling was released.

## Selected graph and qualification

This release includes Runtime/macros/exact guard **0.19.1**, IC Build **0.74.3**
and Wire/Derive **0.16.2**. All six official registry checksums match the selected
lock, with no yanks. Keep the compatible Runtime 0.19.0 manifest minimum.
Types 0.18.0, native Platform facade 0.21.0, Endpoint 0.6.1, Testkit 0.84.1,
Logging 0.24.6, Metrics 0.29.1, Storage 0.66.2 and Host 0.74.3 remain selected.
Shared Tooling stays **0.29.4** at `628ccc75d813bfd9bae777bfbaef55838d25b1b2`.
The supplier facade 0.22 draft was not adopted.

The fresh five-stage gate passed on Linux with Rust **1.99.1** and PocketIC
**16.1.0**: all 30 common CI obligations (655s), CLI/server admission (9s/10s),
all **35 installed watchdog subjects** (49s) and the policy cohort (46s).
Common CI includes 178 Timers unit tests, eight doctests, Jobs/combined consumers,
native/Wasm checks and both normalized archives. Publication passed final Clippy
and normalized 0.40.3 archive compilation. These qualify the selected graph and
source; no new installed Jobs recovery or macOS pass is claimed. Hosted
Linux/macOS CI remains asynchronous. Same-source upgrade tests do not establish
deployed old-Wasm migration, rollback or destructive data retirement.

## Cleanup and API boundaries

Ordinary stop/cancel requests share control-state handling. Watchdog requests,
cancellation, dispatch and completion transitions live with Watchdog control;
registry claim/lifetime admission, observation ordering and provider cleanup stay
intact. Policy-result construction/terminal normalization and Watchdog callback
adaptation have one implementation. Ordinary probe ownership/reconciliation uses
its existing private registration enum. Public policy types remain distinct.

The capability-method macro was evaluated and discarded: it added nine code lines
and indirection, so those forwarders remain explicit. The final cleanup slice
reduces non-test LOC **10,959→10,947**, retaining all **371** source test functions;
this is not the net measurement of the earlier control-module moves. Generated
Rustdoc preserves all 27 reviewed public signatures/documentation blocks. No
execution-cost or Wasm-size improvement is claimed for this batch.

Metrics recording stays in Timers, with copied observations exposed through the
existing two Metrics type re-exports. Canic's adapter owns reporting DTOs and the
reporting endpoint. Those Rust re-exports create no endpoint or runtime mutation
capability. Jobs stays Serde-only with consumer-owned metadata/storage/lifecycle;
every restore uses `Job::restore` and uncertain effects require disposition.
Timers owns the sole private provider, unsafe FFI confinement and exact Runtime
guard. Storage retains admitted Cells and blocked attempts/receipts; retired
Memory images are refused without writes/reset. No Memory, Stable Structures or
Testkit Native package is selected. Canic integration remains Canic-owned; see
the [0.39 → 0.40 checklist](../timers/adoption/0.40.md).

## Durable custody and release rules

`.git/release-state/deployment-timers0403.XBMtMHVE/` retains incoming source and
old/selected locks, supplier registry observations, cleanup evidence, release and
publication logs/statuses, the checksum-matched published archive, VCS inspection
and installed watchdog/cohort diagnostics. Completed `timers-0.40.3.*` journal and
version-2 receipts retain their exact source. Default retained-state admission
passes without cleanup. Regenerable development scratch remains under `target/`.

Eight members retain explicit matching versions in independent Jobs (three) and
Timers (five) groups; no root workspace version. Root manifests and lock own the
supplier cohort. `ic-jobs-canister-tests::harness` keeps recovery support explicit
and default artifact capture lightweight. The complete 30-obligation roster and
four standalone Timers stages remain owned by `ci/release-validation.json`.
Shared/unknown/dirty scope and Timers retain all five stages; qualified Jobs-only
scope may omit standalone Timers work. No stage-proof reuse or conversion of old
receipts is enabled. See [the group mapping](../release-validation-groups.md).

## Earlier deliveries and protected history

Timers **0.40.2** (`dc0c2cd`), **0.40.1** (`f03d430`) and Jobs **0.34.3** (`0eec50a`)
remain pushed/published. The [previous delivered handoff](https://github.com/dragginzgame/ic-scheduling/blob/af1ff7db20125e29edc82ad9323c317b6cd1de6e/docs/status/current.md)
retains exact 0.40.2 qualification, historical footprint measurements and earlier
delivery identities. Preserve its custody `deployment-timers0402.G6arr0ab/` and
`deployment-consumer-support.lHbrHngp/` under `.git/release-state/`.

Also preserve `deployment-timers0400.IkyEFM1l/`, `deployment-timers0391.k7IkTIwf/`,
`deployment-current.0NCjlIWS/`, `deployment-timers0381.jnjcs5qg/`,
`deployment-upstreams.UjAXfFHz/`, `deployment-msrv-87.uUHPASl6/` and
`deployment-wire-tooling.w6Z6gClk/`, including journals, receipts and archives.
Preserve version-1/interrupted proofs and stashes with their original source;
never pop delivered versions or replay completed uploads. The
[earlier ten simplifications](../redundancy-measurements-2026-11-03.md) retain their
frozen Runtime 0.18 graph; prior 44-case installed Jobs recovery retains `af87eab`.
