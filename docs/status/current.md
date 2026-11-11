# Current handoff

## Published Timers 0.43.1 and Jobs 0.35.1 — 2026-11-11

Both groups were delivered to origin/main and published to crates.io through
completed, reconciled `batch-t4HsIlVh`, captured from
`5bb381437a2d128f70f068a0928b80e1ef98495c`. Do not replay publication.
The normal checkout contains both release commits, with no deferred supplier edits.
All six private qualification versions remain unchanged. No sibling repository
was edited or released; Canic adoption remains separately qualified.

- Timers release commit: `38a8b6f64c924a517965ac1cab20ec28f29cced2`.
  Annotated `timers-v0.43.1` tag: `abb679d68631bef9e5a9cd524d1c8e028165e0e4`.
  Archive SHA-256: `1a039360710679965194cb7aa2955898607cc88334504b59824ccbd425fdbc86`.
- Jobs release commit: `17bf5ab5fa7b79c7d3d94448bf1c1aa7dc44394e`.
  Annotated `jobs-v0.35.1` tag: `68b8eba0e08f7cb14dfca3e7ecbfd14ab64accf2`.
  Archive SHA-256: `a7772955bcba396daa8a3d2f313e05595892746c3c73632e823f7e800d94793f`.

Both complete 36-stage release gates passed on Linux/Rust 1.99.1 with PocketIC
16.1.0, including native/Wasm/archive checks, 36 installed watchdog/runtime
subjects, four policy and two calibration cohorts per gate. The validation-plan
fixture executed for Timers and was admitted for Jobs through the qualified
exact-input reuse contract. Other required stages executed. Registry archives
match upload bytes and registry checksums and identify their release commits;
source metadata, required files, origin/main and both tags were independently verified.
No macOS execution or new complete Jobs installed-recovery pass is claimed.

Delivery, registry verification and reconciliation evidence:
`target/evidence/deploy-patches-20261111/`.
Durable batch, source, journals and sealed validation:
`.git/release-state/frozen/batch-t4HsIlVh/`.
Installed runtime receipts under `target/release-frozen/batch-t4HsIlVh/`:
`target/tmp/timer-qualification-3107941-0/runtime.json` (Timers) and
`target/tmp/timer-qualification-1106739-0/runtime.json` (Jobs).
Latest cohort receipt: `qualification/target/timer-cohorts.json` there.

## Tooling, cleanup and workspace ownership

Both catalogs/locks select exact crates.io Toko Tooling 0.39.9, archive SHA-256
`1dd7bdb5bbb9bb8076dd35d8dc179c9e49196d9945c8e585990e40091b468b48`.
Its 59-entrypoint selection admits 106 managed files. Shared references remain
at immutable upstream revisions. Everything under `docs/` is native Scheduling
documentation; use [its index](../README.md).

Both locked fetches, standard installation, snapshot/pin/group/link admission,
workspace inheritance, dependency-boundary callers and all 56 tooling drift-refusal
routes pass. Native-only PocketIC dev dependencies are admitted; production/Wasm
restrictions and CBOR refusals remain. Both published groups include this refresh.

Six private qualification packages remain outside the Jobs/Timers product groups.
Their independent catalog/lock uses standard `qualification_workspace` preparation,
preserving private versions and suppliers. `TOKO_TOOLING_RELEASE := 1` enables releases.
See [release operations](../releases/README.md) and [validation groups](../releases/validation-groups.md).

Test cleanup reduces Rust LOC from 29,011 to 28,762 (249 fewer lines), retaining
all scenarios, 126 exact counter expectations and 31 receipt-corruption cases.
Production logic and its 5,444 app/library lines are unchanged by the cleanup.
Focused evidence: `target/evidence/test-cleanup-20261111/` and
`target/evidence/qualification-cleanup-20261111/`. Both full release gates above
qualify the resulting source and refreshed graph.

## Supplier and persistence boundaries

Both workspaces now select Metrics 0.29.11 and Runtime/exact guard 0.21.1;
qualification also selects matching macros and the required Types 0.19.2.
Runtime owns the single transitive registry ic0 1.2.0 backend. Native/Wasm graphs
and rejection fixtures pass, including direct SDK aliases in both workspaces.
Common direct-dependency bans, the protocol/coordinator exclusions and Timers'
exact runtime guard remain enforced. Wire 0.19 / Endpoint 0.10, Storage 0.71.1
and Testkit 0.96 selections are retained. Qualification uses raw Testkit transport
with current Wire framing; Testkit's optional adapter selects the older Wire family.

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

The [previous handoff](https://github.com/dragginzgame/ic-scheduling/blob/5bb381437a2d128f70f068a0928b80e1ef98495c/docs/status/current.md)
retains earlier release identities, qualification limits and evidence locations.
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
