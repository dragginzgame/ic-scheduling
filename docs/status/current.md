# Current handoff

## Published Timers 0.43.2 and Jobs 0.35.2 — 2026-11-12

Both groups were delivered to origin/main and published to crates.io through
completed, reconciled `batch-LYs1wQ6P`, captured from
`7fd064d5d4444f83615299f7ae4251566b4608d0`. Do not replay publication.
Both release commits are reconciled into the normal main checkout.
All six private qualification versions remain unchanged. No sibling repository
was edited or released; Canic adoption remains separately qualified.

- Timers release commit: `be653c74f9a6e5fae71f9102b559921565af25ea`.
  Annotated `timers-v0.43.2` tag: `032fff2c71f818833ea290ce567e02920ce51081`.
  Archive SHA-256: `e941ec1d73cc48b903bdf809f8da3902f4a6e998d8ab5e198aca38003756a2a9`.
- Jobs release commit: `23e817f981d8004a76ede73f9f71f36cba2ed152`.
  Annotated `jobs-v0.35.2` tag: `933dc508f6fae83ad3456f8df19082ca1526d3d7`.
  Archive SHA-256: `18a459a58063aa3cb2758dcf1d0037db2a50dd961780ad890af5a81df590c9ab`.

Both complete 36-stage release gates passed on Linux/Rust 1.99.1 with PocketIC
16.1.0, including native/Wasm/archive checks, 37 installed watchdog/runtime
subjects, four policy and two calibration cohorts per gate. The validation-plan
fixture executed for Timers and was admitted for Jobs through the qualified
exact-input reuse contract. Other required stages executed. Registry archives
match upload bytes and registry checksums and identify their release commits;
source metadata, required files, origin/main and both tags were independently verified.
No macOS execution is claimed. The ten installed Jobs recovery tests separately
passed on the pre-release source/graph before version preparation; their capture
is `qualification/target/tmp/jobs-recovery.ylbrb4/`.

Delivery, registry verification and reconciliation evidence:
`target/evidence/deploy-retry-20261112.iuLa37/`.
Durable batch, source, journals and sealed validation:
`.git/release-state/frozen/batch-LYs1wQ6P/`.
Installed runtime receipts under `target/release-frozen/batch-LYs1wQ6P/`:
`target/tmp/timer-qualification-2027386-0/runtime.json` (Timers) and
`target/tmp/timer-qualification-3213211-0/runtime.json` (Jobs).
Latest cohort receipt: `qualification/target/timer-cohorts.json` there.

## Cached baseline qualification

Published Testkit 0.101.0 corrects the cached snapshot defect tracked by
[Timers #87](https://github.com/dragginzgame/ic-timers/issues/87) and
[Tooling #263](https://github.com/dragginzgame/ic-tooling/issues/263). The mandatory,
unignored same-instance regression passes unchanged restoration and both mutated
restore/rearm/callback cycles in both complete gates. The competing-owner
negative control still passes; production guard code is intact. Canic's exact
two-case integration remains separately owned and was not executed here.

The initial paired passing receipt is `target/tmp/timer-qualification-704322-0/runtime.json`.
Earlier 0.100.2 failures remain in
`target/evidence/timer-baseline-87-20261112.FT14fd/`; they are not passes.

## Tooling, cleanup and workspace ownership

Both catalogs/locks locally select exact crates.io Toko Tooling 0.39.15, archive SHA-256
`6cc984822e80e52ff7203aadcaf293a13f9fcf33f84175871321cb3a7de703a9`.
Its 60-entrypoint selection admits 108 managed files. Shared references remain
at immutable upstream revisions. Everything under `docs/` is native Scheduling
documentation; use [its index](../README.md).

The tooling refresh is local and uncommitted; published groups retain 0.39.12.
The installer preserved all previous files and added the standard dependency
update Make/script pair. `make deps-update` runs Cargo sequentially in the product
and qualification workspaces, preserving completed changes on failure. Explicit
package/version selectors are forwarded; no broad dependency update was run here.
The updater caller, tooling/member admission and CI drift fixtures use substituted
effects; they do not qualify changed supplier runtime behavior.
Snapshot/pin/group checks, release-command callers, formatting evidence and local
documentation links pass. Pending compatible notes are Jobs 0.35.3 and Timers
0.43.3; product versions remain unchanged and nothing was committed or delivered.

Local tooling evidence: `target/evidence/tooling-adoption-20261112.FD7Fjq/`.

Qualification now selects published Build 0.83.0, Storage 0.73.4, Wire 0.21.2 and
Endpoint/macros 0.12.1, preserving the prior Runtime/macros/guard 0.21.2 refresh
and external lock edits. Its Build metadata CLI is installed and admitted; real
Jobs/Timers metadata preparation and rollback fixtures pass with substituted
product gates/uploads. No release effects were performed.
Native/Wasm Clippy, dependency policies and 82 native tests pass. Eleven installed
Jobs tests pass, including old-image upgrade rejection with identical stable bytes;
four focused Timers subjects pass for Wire, Runtime composition, cached restoration
and competing-owner rejection. Linux/Rust 1.99.1/PocketIC 16.1.0 only; no fresh
full release gate, cohort comparison, macOS or Canic execution is claimed.
Evidence: `target/evidence/qualification-suppliers-20261112.5Wta8m/`;
Jobs capture: `qualification/target/tmp/jobs-recovery.T0IYu1/`; focused runtime
receipt: `target/tmp/timer-qualification-3841871-0/runtime.json`.
Product source/versions and its Runtime 0.21.1 graph are unchanged by this adoption.

Six private qualification packages remain outside the Jobs/Timers product groups.
Their independent catalog/lock uses standard `qualification_workspace` preparation,
preserving private versions and suppliers. `TOKO_TOOLING_RELEASE := 1` enables releases.
See [release operations](../releases/README.md) and [validation groups](../releases/validation-groups.md).

## Supplier and persistence boundaries

The published workspaces select Metrics 0.29.11 and Runtime/exact guard 0.21.1;
published qualification also selects matching macros and Types 0.19.4.
Runtime owns the single transitive registry ic0 1.2.0 backend. Native/Wasm graphs
and rejection fixtures pass, including direct SDK aliases in both workspaces.
Common direct-dependency bans, the protocol/coordinator exclusions and Timers'
exact runtime guard remain enforced. Published qualification selects Testkit 0.101.0,
required Host 0.76.4 patches, native Platform 0.31.2 and upstream PocketIC through Testkit's
native re-export. Application DTOs retain canonical Types; native identity
conversion is explicit. Rejections remain typed, transport panics propagate and
awaited deletion is upstream best effort. No failed update is replayed.
Its Wire 0.19 / Endpoint 0.10 and Storage 0.71.1 are retained. The consumer-owned
Wire harness uses raw transport, without enabling Testkit's Wire 0.21 adapter.

The 2026-11-12 registry review and requested qualification adoption complete the
earlier Build/Storage/Wire deferrals. Optional product Runtime 0.21.2 and
qualification Thiserror 2.0.21 remain deferred to the next product/qualification
supplier maintenance batch respectively; qualification retains Thiserror 2.0.18.
Cargo Sort Derives 0.15.0 awaits the next Toko tool-catalog refresh; the reviewed
managed pin remains 0.14.1. Other reviewed direct supplier locks match observed
stable releases. Revisit deferrals at those boundaries or the next release review.

Jobs stays Serde-only, with consumer-owned storage and metadata. Timers retains
its private FFI provider, singleton guard and runtime/cohort gates. Native tests
do not establish installed IC recovery. Do not reintroduce ic-memory.

Current qualification Storage 0.73 requires fresh backing or explicitly authorized whole-installation
retirement after preserving binary/lock/backup, blocked attempts and receipts.
Frozen 0.70 and 0.71 images retain Running/Uncertain deliveries and receipts; physical
creation/recovery refuse with zero writes/growth. Payload restore remains blocked
through Job::restore. No converter, automatic reset or effect disposition exists.
See the [fixture guide](../../crates/ic-jobs/tests/fixtures/README.md) and
[consumer guide](../../qualification/ic-jobs-test-consumer/README.md).

## Earlier evidence and recovery custody

The [previous handoff](https://github.com/dragginzgame/ic-scheduling/blob/0ef8d81c2e87766cf6823d95c7480a72b5a5ad83/docs/status/current.md)
retains earlier release identities, qualification limits and evidence locations.
Preserve completed `batch-t4HsIlVh` (Timers 0.43.1 / Jobs 0.35.1), its original
graph and `target/evidence/deploy-patches-20261111/` delivery evidence.
Preserve completed `batch-vqHkyqbM` (Timers 0.43.0 / Jobs 0.35.0) and its original graph.
Timers 0.42.0 completed in `batch-zgh_z6sJ`; its Runtime 0.21 adoption and all ten
explicit Jobs installed-recovery subjects retain their original source/graph in
`target/evidence/runtime21-release-20261109T171837Z/`. Issue #92 was closed then.
Preserve failed `batch-1wgxqWCb`, `batch-ACww0dNn` and `batch-XVdlf0HL`, completed
`batch-ZNhgLJjb` and `batch-6xiXTmGx`, and all older recovery records.

Tracked pre-adoption work and original locks remain in stash
`4adfda0e74ffee32281b835735a25128b7648ceb`; preserve stashes
`19beca78079c6437b91eb436d789521fc86ac760` and
`7f1fe032b88665507192692dcea2278aeec713a6` too. Do not apply whole old locks over
released product versions, relabel historical evidence, replay publication or
retire protected batches, tags, stashes, branches, archives or artifacts implicitly.
