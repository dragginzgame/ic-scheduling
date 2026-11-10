# Current handoff

## Published Timers 0.43.0 and Jobs 0.35.0 — 2026-11-10

Both groups were delivered to origin/main and published to crates.io through
completed, reconciled `batch-vqHkyqbM`, captured from
`4660ad69b6f727624043c0865f87f04ab9b0626d`. Do not replay publication.
All six private qualification versions remain unchanged. No sibling repository
was edited or released; Canic adoption remains separately qualified.

- Timers release commit: `166d4e989d7d4cc8cb035d19dd5e9a20671e504b`.
  Annotated `timers-v0.43.0` tag: `997f07d0a7e39ce46275863b12015c46248342f0`.
  Archive SHA-256: `530217c8594fe6542d19df41e631c72cbbfd765d9b1a031b874686a1ccb95603`.
- Jobs release commit: `c6cb00c36dc41c8221d70e5973fd6ae5226dbbf9`.
  Annotated `jobs-v0.35.0` tag: `a41da6277cb1fc50f266645af64282684f62133e`.
  Archive SHA-256: `7617a9a870fa55ede837253578da38a22f48e81bdfb4069b429e498eb726290b`.

Both complete 36-stage release gates passed on Linux/Rust 1.99.1 with PocketIC
16.1.0, including native/Wasm/archive checks, 36 installed watchdog/runtime
subjects, four policy and two calibration cohorts per gate. The validation-plan
fixture executed for Timers and was admitted for Jobs through the qualified
exact-input reuse contract. Other required stages executed. Registry archives
match upload bytes and registry checksums and identify their release commits;
origin/main and both annotated tags were independently observed.
No macOS execution or new complete Jobs installed-recovery pass is claimed.

Delivery and registry evidence:
`target/evidence/deploy-scheduling-20261110.FHcFsx/`.
Durable batch, source, journals and sealed validation:
`.git/release-state/frozen/batch-vqHkyqbM/`.
Installed runtime receipts under `target/release-frozen/batch-vqHkyqbM/`:
`target/tmp/timer-qualification-680373-0/runtime.json` (Timers) and
`target/tmp/timer-qualification-105532-0/runtime.json` (Jobs).
Latest cohort receipt: `qualification/target/timer-cohorts.json` there.

## Current tooling and workspace ownership

Both catalogs/locks select exact crates.io Toko Tooling 0.39.7, archive SHA-256
`fd795f6ef878496ac27a990a9208b87fa786a9148c18580338ef8ac79df501d7`.
Its complete 59-entrypoint selection admits 106 managed files. The exporter
retired 66 authenticated reference copies; consumer links retain immutable
upstream revisions. Everything under `docs/` is native Scheduling documentation;
use [its index](../README.md). The root baseline remains `TOKO-TOOLING.md`.

The product workspace contains only Jobs and Timers. Six private packages live
under the independent qualification catalog/lock and belong to no release group.
Standard `qualification_workspace` release preparation owns both workspaces'
product references/locks while preserving private versions and other suppliers.
The shared inheritance checker replaces the consumer adapter; rejection tests
retain stale/missing versions, independent/target dependencies and privacy cases.
Shared grouped releases are explicitly enabled with `TOKO_TOOLING_RELEASE := 1`.
See [release operations](../releases/README.md) and
[validation groups](../releases/validation-groups.md).

## Deferred work and graph boundaries

The normal product Cargo.lock retains only an uncommitted Metrics 0.29.11 refresh.
It is not part of either publication; both release locks select Metrics 0.29.9.
After standard batch reconciliation, targeted Cargo update reapplied only this
version/checksum against the new product versions. No related adoption or release
is claimed. Original pending graph/source custody remains in stash
`19beca78079c6437b91eb436d789521fc86ac760`; preserve the older deferred graph stash
`7f1fe032b88665507192692dcea2278aeec713a6` too. Do not apply whole old locks over
released product versions or drop protected custody.

Runtime/macros/exact guard remain 0.21.0, Types 0.19, Wire 0.19 / Endpoint 0.10,
Storage 0.71.1 and Testkit 0.96. Qualification uses raw Testkit transport with
current Wire framing; Testkit's optional adapter selects the older Wire family.
Jobs stays Serde-only, with consumer-owned storage and metadata. Timers retains
its private FFI provider, singleton guard and runtime/cohort gates. Native tests
do not establish installed IC recovery. Do not reintroduce ic-memory.

Storage 0.71 requires fresh backing or explicitly authorized whole-installation
retirement after preserving binary/lock/backup, blocked attempts and receipts.
Frozen 0.70 images retain Running/Uncertain deliveries and receipts; physical
creation/recovery refuse with zero writes/growth. Payload restore remains blocked
through Job::restore. No converter, automatic reset or effect disposition exists.
See the [fixture guide](../../crates/ic-jobs/tests/fixtures/README.md) and
[consumer guide](../../qualification/ic-jobs-test-consumer/README.md).

## Earlier evidence and recovery custody

The [previous handoff](https://github.com/dragginzgame/ic-scheduling/blob/4660ad69b6f727624043c0865f87f04ab9b0626d/docs/status/current.md)
retains earlier release identities, qualification limits and evidence locations.
Timers 0.42.0 completed in `batch-zgh_z6sJ`; its Runtime 0.21 adoption and all ten
explicit Jobs installed-recovery subjects retain their original source/graph in
`target/evidence/runtime21-release-20261109T171837Z/`. Issue #92 was closed then.
Preserve failed `batch-1wgxqWCb`, `batch-ACww0dNn` and `batch-XVdlf0HL`, completed
`batch-ZNhgLJjb` and `batch-6xiXTmGx`, and all older recovery records. Do not relabel
historical evidence, replay publication, or retire protected batches, tags,
stashes, branches, archives or artifacts as part of normal delivery.
