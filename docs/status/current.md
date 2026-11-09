# Current handoff

## Runtime 0.21 adoption and authorized Timers release — 2026-11-09

The user authorized issue #92, Runtime 0.21 adoption and deployment/publication in this
repository. Origin/main equals the current base bb67e6f; siblings remain read-only.
Target Timers 0.42.0: its required executor family changes from 0.20 to 0.21, so
composed consumers must update their Runtime/Endpoint stack. Jobs' public library
contract is unchanged and remains 0.34.16; its private maintenance notes remain
Unreleased 0.34.17. Qualification versions stay at their existing literals.
Implementation source 1f2b9ed adopted the complete matching family. Frozen batch
`batch-1wgxqWCb` failed before preparation/delivery at `test-release-tooling`: its
disposable nested fixture retired after reconciliation, but generic retirement
correctly refused ignored `qualification/target` output. The consumer fixture now
cleans only that explicit Cargo cache before retirement; the retained completed
fixture passed focused cleanup/retirement with both release tags preserved.
Evidence: `target/evidence/runtime21-release-20261109T171837Z/issue92-retirement/`.
The fresh `batch-ACww0dNn` on fixed source `10718c7` passed the full frozen
release fixture, including reconciliation and retirement, then stopped at
`test-consumer-tooling`. Batch-routing and CI-drift fixtures had retained their
snapshot substitution but unexpectedly invoked real qualification membership
admission against incomplete fixtures. They now explicitly substitute both
admission checks; dedicated tooling admission retains all real 56 caller routes
and rejection tests. The complete consumer-tooling suite passes after the fix.
Evidence: `target/evidence/runtime21-release-20261109T171837Z/consumer-all-fixed.log`.
Keep both failed real batches/source/evidence intact. No product version, tag,
push or upload occurred. Capture a new fixed source and rerun the standard full flow:
`make release-minor RELEASE_GROUP=timers RELEASE_PUBLISH_TARGET=publish` after
committing the complete implementation and obtaining a clean source checkout.
Preserve any resulting batch/context and use its explicit BATCH for recovery.

Both owning locks now select Runtime/macros/exact guard 0.21.0; qualification
selects Types 0.19, Wire 0.19 / Endpoint 0.10 and Storage 0.71.1. This now resolves
with one Runtime, guard, Types and Wire identity. Remove the temporary principal
byte projections; endpoint identities, stored fixtures and codec bytes are unchanged.
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
`target/tmp/timer-qualification-1483809-0/runtime.json`. The release still requires
its full selected gate and fresh Watchdog/policy-cohort qualification on captured
source; earlier Runtime 0.20 or old native passes do not substitute for it.

All prior pending work is preserved for authorized delivery: issue #90/#91 fixes,
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

A concurrent normal Cargo.lock refresh selects Metrics 0.29.10 instead of the
recorded 0.29.9 product graph. Preserve it separately; it belongs to the next
candidate and does not alter the Runtime 0.21 release inputs.

This fixed implementation is being captured as the authorized clean release source.
The deployment request authorizes its scoped implementation commit, standard
release commits/tags, origin/main delivery and Timers publication.
No sibling release or editing is authorized. No protected prior batch/artifact
custody has been retired.

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
