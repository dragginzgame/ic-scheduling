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

Both catalogs/locks locally select exact crates.io Toko Tooling 0.39.9, archive SHA-256
`1dd7bdb5bbb9bb8076dd35d8dc179c9e49196d9945c8e585990e40091b468b48`.
Its complete 59-entrypoint selection admits 106 managed files. Shared references
remain at immutable upstream revisions. Everything under `docs/` is native
Scheduling documentation; use [its index](../README.md).

The 0.39.9 refresh is included in the pending patch delivery. Both locked fetches, standard tool installation,
snapshot/pin/group/link admission, workspace inheritance, both dependency-boundary
callers and all 56 tooling drift-refusal routes pass. Native-only PocketIC dev
dependencies are admitted; production/Wasm restrictions and CBOR refusals remain.
The published releases above retain their original 0.39.7 source and graph.

Six private qualification packages remain outside the Jobs/Timers product groups.
Their independent catalog/lock uses standard `qualification_workspace` preparation,
preserving private versions and suppliers. `TOKO_TOOLING_RELEASE := 1` enables releases.
See [release operations](../releases/README.md) and [validation groups](../releases/validation-groups.md).

Local test cleanup reduces Rust LOC from 29,011 to 28,762 across two passes.
The first retains 126 counter expectations and all 176 Timers / 41 queue tests;
evidence: `target/evidence/test-cleanup-20261111/`. The second retains all test
names and 31 receipt-corruption cases; five report tests, eight composition tests,
and the two changed installed watchdog subjects pass, with Clippy/format checks.
Production logic is unchanged. Evidence: `target/evidence/qualification-cleanup-20261111/`;
live receipt: `target/tmp/timer-qualification-79894-0/runtime.json` (focused, not a full gate).

## Active patch delivery and graph boundaries

Jobs 0.35.1 and Timers 0.43.1 are authorized for frozen-batch release and
publication, Timers first. Both catalogs/locks now select Metrics 0.29.11 and
Runtime/exact guard 0.21.1; qualification also selects matching macros and the
required Types 0.19.2. Runtime owns the single transitive ic0 1.2.0 backend.
Focused native/Wasm graphs and actual rejection fixtures pass, including direct
SDK aliases in both workspaces. Full product and installed-runtime qualification
must pass on the captured source before delivery; earlier live receipts above
do not qualify the refreshed graph. Evidence: `target/evidence/deploy-patches-20261111/`.

Tracked pre-adoption work and both original locks are preserved in stash
`4adfda0e74ffee32281b835735a25128b7648ceb`. Original pending graph/source custody remains in stash
`19beca78079c6437b91eb436d789521fc86ac760`; preserve the older deferred graph stash
`7f1fe032b88665507192692dcea2278aeec713a6` too. Do not apply whole old locks over
released product versions or drop protected custody.

The delivered baseline above is Runtime/macros/exact guard 0.21.0. Current selections retain Wire 0.19 / Endpoint 0.10,
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
