# Current handoff

## Published Timers 0.42.0 / Runtime 0.21 — 2026-11-09

Timers 0.42.0 was delivered to origin/main and published to crates.io through
completed, reconciled `batch-zgh_z6sJ`, captured from `f6c6637`. Do not replay
publication. [Issue #92](https://github.com/dragginzgame/ic-scheduling/issues/92)
is closed. Its required executor family changes from 0.20 to 0.21; composed
consumers must update their Runtime/Endpoint stack. Canic adoption remains
separately qualified; sibling repositories were not edited or released.
Jobs stays 0.34.16, with private maintenance notes Unreleased 0.34.17.
All six qualification versions remain unchanged.

- Release commit: `a3f04e0b1f82286e1b723d278fb10aa8841789ef`.
- Annotated `timers-v0.42.0` tag object: `53875bd22bba8a98d4b0c5172c1594ff7b145fe0`.
- Registry/upload archive SHA-256: `01158550931928401fe8f4a71ce7c6d6ab4e51fb9cbb8e2458491fe1eaec728f`.

All 36 selected release stages passed on Linux/Rust 1.99.1 with PocketIC 16.1.0:
36 installed Watchdog/runtime subjects, four policy and two calibration cohorts,
native/Wasm/archive checks and the consumer tooling fixtures. The exact-input
validation fixture was admitted through its standard reuse contract; other
required stages executed. The published archive matches the uploaded bytes and
registry checksum, records the release commit and requires Runtime 0.21.0.
A fresh registry-only Cargo probe resolves published Timers 0.42.0 with exact
Runtime 0.21.0 and executor/IC0/runtime features, selecting one matching guard.
Its metadata and result are under `published-0.42.0/composition-probe/` in the
evidence directory. This is resolution proof, not Canic execution.
No macOS pass is claimed.

Evidence: `target/evidence/runtime21-release-20261109T171837Z/`;
durable batch/journals/sealed qualification:
`.git/release-state/frozen/batch-zgh_z6sJ/`. Installed runtime receipt:
`target/release-frozen/batch-zgh_z6sJ/target/tmp/timer-qualification-3134713-0/runtime.json`;
cohort receipt: `target/release-frozen/batch-zgh_z6sJ/qualification/target/timer-cohorts.json`.

Preserve failed `batch-1wgxqWCb` and `batch-ACww0dNn`. Their fixture failures
were corrected without relaxing product gates: clean only disposable qualification
Cargo cache before retirement, and explicitly substitute membership admission in
batch-routing/CI-drift fixtures while keeping all 56 real admission routes.
The completed batch also retains its initial ENOSPC failure at the final cohort
build. Capacity recovered without repository cleanup; a focused cohort retry and
then standard saved-batch recovery passed. No failed run was sealed as a pass.

Both owning locks now select Runtime/macros/exact guard 0.21.0; qualification
selects Types 0.19, Wire 0.19 / Endpoint 0.10 and Storage 0.71.1. This now resolves
with one Runtime, guard, Types and Wire identity. The temporary principal
byte projections are removed; endpoint identities, stored fixtures and codec bytes are unchanged.
Do not reintroduce ic-memory or bypass singleton/source/provider gates.

Testkit 0.96 remains current. Its optional Wire adapter selects Wire 0.18, so
qualification uses its raw transport with current Wire framing instead. The
existing Jobs native test package exports one harness-gated transport shared by
Jobs and Timers tests; default artifact capture remains lightweight. Request,
transport and reply failures retain exact context, budgets and no-retry behavior.
No extra package or production/framework dependency is added to either library.

Toko Tooling 0.38.2 remains exact in both workspaces: 105 roots/169 managed files.
The product root has only Jobs/Timers; six private packages live under the
independent qualification/ catalog/lock. Standard `qualification_workspace`
release preparation owns both manifests/locks and preserves private versions and
unrelated suppliers. The actual two-group metadata/receipt/rollback caller passed
in `target/evidence/tooling-runtime-20261109T170141Z/`; product gates/setup/uploads
were substituted, not product qualification.

The pin blocker is fixed at the consumer caller: retain the common declaration,
workflow and tracked-lock validator, then call the unchanged shared inheritance
function with product-group and private qualification version ownership. No source
inventory filtering, suppressed findings, fake private release group or version
bump is used. The caller fixture admits both workspaces and rejects stale/missing
versions, independent and target dependencies, and publishable qualification.
All dependency-table inheritance and complete product-catalog checks remain.

Current focused checks: native Jobs/DTO/storage/recovery tests, native harness
and Wasm Clippy, pin/member/graph/provider/group admission, and Runtime composition
plus typed request/transport/reply error live subjects pass. All 10 explicit Jobs
installed recovery subjects pass on the captured current-stack Wasm. Main adoption evidence:
`target/evidence/runtime21-release-20261109T171837Z/`; initial focused Timers receipt:
`target/tmp/timer-qualification-1483809-0/runtime.json`. The completed release's
fresh Watchdog/policy-cohort gate is recorded above; earlier Runtime 0.20 evidence
retains its original graph and does not substitute for it.

The release includes the prior pending work: issue #90/#91 fixes,
qualified storage backing hard cut, canonical native Types/Testkit adoption and
Tooling/layout migration. Previous evidence remains in
`target/evidence/platform-placement-20261109T162754Z/`,
`target/evidence/platform-testkit-20261109/`,
`target/evidence/platform-types-20261109/`,
`target/evidence/issues-90-91-20261109/` and
`target/evidence/storage-tooling-20261109/`. Do not relabel historical graphs.

Storage 0.71 uses fresh backing or explicitly authorized whole-installation
retirement after preserving binary/lock/backup, blocked attempts and receipts.
No converter, automatic reset or effect disposition is added. Frozen 0.70 images
retain Running/Uncertain deliveries and receipts; creation/recovery refuse with
zero writes/growth, while payload restore stays blocked through Job::restore.
See [the fixture guide](../../crates/ic-jobs/tests/fixtures/README.md) and
[consumer guide](../../qualification/ic-jobs-test-consumer/README.md).

A normal Cargo.lock refresh to Metrics 0.29.10 remains uncommitted and deferred;
the release used 0.29.9. After standard reconciliation, targeted Cargo update
reapplied only that version/checksum, preserving Timers 0.42.0 and other suppliers.
Its original bytes remain in stash `7f1fe032b88665507192692dcea2278aeec713a6`
and `issue92-retirement/deferred-metrics-Cargo.lock` beneath the evidence directory.
Do not drop that stash or treat this refresh as released. No protected prior
batch/artifact custody was retired.

## Published baseline and recovery custody

Timers 0.41.5 and Jobs 0.34.16 were delivered/published through completed,
reconciled `batch-ZNhgLJjb`; do not replay publication. Captured source:
`627cf7432d459f9623ecc7727b5b9541863b6b3d`.

- Timers commit: `c62a6aa21d018574b6de697aea7dc7bc37972852`;
  tag object: `2074bdf8b70417097d46e4162accf45de31dcb22`;
  archive: `9b0d4b7043e9d4bd65eb85c99d133160e9fe6c3975142caef3cac57393b657c8`.
- Jobs commit/delivered main: `bb67e6f23ec50a95cf1b58f675cd5a8d8c60f4ff`;
  tag object: `ee8ba2e10b7c4a5c5309eb441fdfa6daeb653e4d`;
  archive: `c7a9980fc680629aa063184a066570d6a4c7fe6ed95c5406da5205e43ad0d0fa`.

Both 36-stage Linux/Rust 1.99.1 gates, 35 installed Watchdog subjects and four
policy/two calibration cohorts passed with PocketIC 16.1.0. Upload bytes matched
registry archives and tagged sources. No complete fresh Jobs live or macOS pass
was claimed. Evidence: `target/evidence/deploy-scheduling-20261109/`; durable
custody: `.git/release-state/frozen/batch-ZNhgLJjb/`. Preserve failed batch-XVdlf0HL,
completed batch-6xiXTmGx and all older custody. Do not reinterpret old journals
under the new membership/lock policy. No batches, tags, stashes, branches,
archives or protected artifacts were retired. Jobs stays Serde-only; uncertain
effects remain blocked. Timers retains its private FFI/provider, exact guard and
runtime/cohort gates. Storage stays checked; ic-memory remains excluded.
