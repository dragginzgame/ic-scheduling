# Current handoff

## Published Timers 0.43.4 and Jobs 0.35.4 — 2026-11-12

Both groups were delivered to origin/main and published to crates.io through
completed, reconciled `batch-dZQHGKiw`, captured from adoption commit
`f45902bed6ec13b6dcddb7622a01ab7c8738ecc6`. Do not replay publication.
The normal main checkout contains both release commits. All six private
qualification versions remain unchanged. No sibling repository was changed.

- Timers release commit: `f795aca3ce7cf98cd8eb6fff71531ee55e0a4ced`.
  Annotated `timers-v0.43.4` tag: `5e3a7976d27aacccda602723a96895acb4315392`.
  Archive SHA-256: `e86d41908566ea91a0aa3f3384d81f9ac379a16d9a6e9c1330ea27ed868f6451`.
- Jobs release commit: `5bbc65fb0b258e88751924a6deb85b8c01d77830`.
  Annotated `jobs-v0.35.4` tag: `f58b8fb3601494465b58c66e0a293b188824167a`.
  Archive SHA-256: `75f11b7af3d8549457443488edc59d3559ae89a964ed4d3922250eb798086206`.

Both complete 36-stage release gates passed on Linux/Rust 1.99.1/PocketIC 16.1.0,
including native/Wasm/archive checks, 37 installed watchdog/runtime subjects and
four policy plus two calibration cohorts per gate. The validation-plan fixture
executed for Timers and was admitted for Jobs through qualified exact-input reuse;
other required stages executed. Registry checksums, downloaded/uploaded archive
bytes, source metadata, dependency requirements and remote annotated tags were
independently verified. No fresh macOS or Canic execution is claimed.

Delivery and verification evidence:
`target/evidence/deploy-scheduling-20261112.BXzhJH/` (`release.log`, exit 0).
Durable source, journals and sealed validation:
`.git/release-state/frozen/batch-dZQHGKiw/`.
Installed receipts under `target/release-frozen/batch-dZQHGKiw/`:
`target/tmp/timer-qualification-2789758-0/runtime.json` (Timers) and
`target/tmp/timer-qualification-602382-0/runtime.json` (Jobs).
Cohort receipts are retained in the delivery evidence as `timers-cohorts.json`
and `jobs-cohorts.json`; their original diagnostics are
`target/tmp/timer-qualification-2815269-0/` and
`target/tmp/timer-qualification-621289-0/` in the captured checkout.

## Released Platform, Wire and Toko adoption

The published graph selects Platform Runtime/exact guard 0.21.3 in both locks.
Qualification also selects matching macros, Types 0.19.5, native Platform 0.31.3,
Wire 0.21.4 and Endpoint/macros 0.12.2. Both catalogs and locks select exact
Toko Tooling 0.39.17, archive SHA-256
`aaeab49adb84ea0d66cf86ecc48d826125f9c3ac6eafaf2a362bccfeba7acbfd`.
The installer retained all 60 entrypoints and 108 managed files. Only the shared
baseline changed in the installed payload; common CI/release admission remains
enforced, including the independent qualification workspace.

Fourteen modules with child directories moved into `mod.rs`, retaining identical
Rust bytes and module paths. Current architecture/source links follow the moves;
historical source/hash records retain their original identities. Corrected the
consumer guide's older Wire 0.7 and endpoint-feature descriptions: current Wire
admits redundant varints within width bounds and uses explicit Endpoint budgets.
There are no maintained generated browser clients to regenerate for Wire 0.21.4.

The incoming dirty locks already contained these Platform/Wire versions plus
Host 0.76.5, Storage 0.73.5 and errno 0.3.15. They were preserved; Cargo changed
only Toko's selected entry relative to those starting locks. Timers changes its
packaged source layout and Runtime requirement; both products' normalized
dev-dependency contracts change from Toko 0.39.15 to 0.39.17. Both public groups
therefore received a release; private qualification packages retained their versions.

Focused qualification and supplier review evidence:
`target/evidence/platform-wire-tooling-20261112.zbX1s2/`. Both full locked metadata
observations, graph/provider boundaries, native/Wasm Clippy, 176 Timers native
tests, nine doctests and 82 private native tests passed. The actual inheritance,
dependency-update and tooling-admission callers passed, including 56 refusal
routes. LOC reports retain zero uncategorized lines. All 11 installed Jobs
recovery tests and four focused Timers Wire/Runtime/ownership subjects passed,
along with all four policy and two calibration cohorts. Jobs capture is
`qualification/target/tmp/jobs-recovery.wLGcXu/`; Timers receipts are
`target/tmp/timer-qualification-2955620-0/runtime.json` and
`target/tmp/timer-qualification-2976790-0/cohorts.json`.
These source/graph-bound adoption checks preceded the complete release gates above.

Published direct supplier review before capture confirmed Platform/Wire/Toko,
Build 0.83.0, Testkit 0.101.0, Metrics 0.29.11, Host 0.76.5 and Storage 0.73.5.
Direct Serde/JSON/test, Clap and Tokio selections remain current. Qualification
Thiserror 2.0.18 is retained by PocketIC 16.1.0's exact requirement; root selects
2.0.21. Unrelated Generic Array 0.14.9 remains deferred with the existing 0.14.7
selection. Cargo Sort Derives stays at the managed catalog's 0.14.1; review its
previously observed 0.15.0 at the next Toko catalog refresh.

## Pending Metrics 0.29.12

An updater changed both normal-checkout locks to Metrics 0.29.12 during Jobs'
gate. The captured graph and both releases retained qualified Metrics 0.29.11.
The published Timers archive's lock independently confirms that selection.

The deferred change is preserved locally on
`deferred/metrics-during-batch-dZQHGKiw`, commit
`78fea9dff2f9201d5919ed1f504e32c21832187d`. After the standard guarded
fast-forward, only its two Metrics hunks were reapplied. Product versions remain
Timers 0.43.4 and Jobs 0.35.4; no old lockfile was restored. Both locks are dirty
with this pending refresh, and that branch was not pushed. Metrics 0.29.12 has
not been qualified here or included in a release; qualify it before further delivery.

## Previous Timers 0.43.3 and Jobs 0.35.3 — 2026-11-12

Both groups were delivered to origin/main and published to crates.io through
completed, reconciled `batch-StU1JTG0`, captured from
`cf3dc91123b8ccb3e1b02c6c58f481a89cf40242`. Do not replay publication.
The normal main checkout contains both release commits. All six private
qualification versions remain unchanged. No sibling repository was changed.

- Timers release commit: `ef84cb147484cd8b7f12dc0b31437c28710f3f6a`.
  Annotated `timers-v0.43.3` tag: `b76449879cbc0e64c875dac58732ed7f425ba832`.
  Archive SHA-256: `9f68301372c42c18a9a1c8be71a023b4cd6c7db23c9667814e5734f13eb4a60e`.
- Jobs release commit: `4712e52d75eb86e7f434cdb3216d878b7c0c0cb4`.
  Annotated `jobs-v0.35.3` tag: `645070a5102f5320054f8bb90c7fff006c25888d`.
  Archive SHA-256: `094d9a35b457fd1b6be4137973004ae5a385d0d0f325bc8c2a3f56faee879bdf`.

Both complete 36-stage release gates passed on Linux/Rust 1.99.1/PocketIC 16.1.0,
including native/Wasm/archive checks, 37 installed watchdog/runtime subjects,
four policy and two calibration cohorts per gate. The validation-plan fixture
executed for Timers and was admitted for Jobs through qualified exact-input reuse;
other required stages executed. Registry checksums, uploaded archive bytes, source
metadata, required files, origin/main and annotated tags were independently verified.
No fresh macOS or Canic execution is claimed.

Delivery, verification and reconciliation evidence:
`target/evidence/deploy-scheduling-20261112.5MZAKe/` (`release-corrected.log`).
Durable source, journals and sealed validation:
`.git/release-state/frozen/batch-StU1JTG0/`.
Installed receipts under `target/release-frozen/batch-StU1JTG0/`:
`target/tmp/timer-qualification-2803447-0/runtime.json` (Timers) and
`target/tmp/timer-qualification-1066411-0/runtime.json` (Jobs).
Latest cohort receipt: `qualification/target/timer-cohorts.json` there.

## Suppliers qualified for the published batch

That batch selected exact crates.io Toko Tooling 0.39.15, archive SHA-256
`6cc984822e80e52ff7203aadcaf293a13f9fcf33f84175871321cb3a7de703a9`.
Its 60 entrypoints admit 108 managed files. Shared references stay revision-bound;
everything under `docs/` is native Scheduling documentation; see [its index](../README.md).
`make deps-update` runs Cargo sequentially for product and qualification, forwards
package/version selectors and preserves completed lock edits on failure.

Qualification selected Build 0.83.0, Storage 0.73.4, Wire 0.21.2 and Endpoint/macros
0.12.1. Both workspace locks selected Runtime/exact guard 0.21.2; qualification also
selected matching macros, Types 0.19.4, Host 0.76.4 and Testkit 0.101.0.
Metrics remains 0.29.11. Application DTOs use canonical Types; PocketIC native
identities are converted explicitly. Transport panics propagate without replay.

Pre-release adoption checks passed: native/Wasm Clippy (including all four probe
feature selections), 82 native tests, 11 installed Jobs recovery tests and four
focused Timers subjects. These were source/graph-bound pre-version-preparation
runs, separate from the complete release gates above. Evidence:
`target/evidence/qualification-suppliers-20261112.5Wta8m/`; Jobs capture:
`qualification/target/tmp/jobs-recovery.T0IYu1/`; focused runtime receipt:
`target/tmp/timer-qualification-3841871-0/runtime.json`.
Tooling caller evidence: `target/evidence/tooling-adoption-20261112.FD7Fjq/`.
Build metadata preparation/rollback fixtures passed for both groups using the
real selected editor and substituted product gates/uploads.

Build uses per-label shared Cargo targets and explicit exact-cache pruning with
the existing seven-day/512-MiB limits. Retained records protect exact artifacts;
complete manifest/lock bytes determine freshness. Six private packages retain
their independent catalog/lock and versions outside all product release groups.
See [release operations](../releases/README.md) and [validation groups](../releases/validation-groups.md).

## Runtime and persistence boundaries

The cached-baseline regression passes unchanged and both mutated restore/callback
cycles; genuine competing owners still trap. The production guard is intact.
[Timers #87](https://github.com/dragginzgame/ic-timers/issues/87) owns this regression;
[Tooling #263](https://github.com/dragginzgame/ic-tooling/issues/263)/Canic own the
separate exact consumer sequence. Earlier failed 0.100.2 evidence remains in
`target/evidence/timer-baseline-87-20261112.FT14fd/`; do not relabel it.

Storage 0.73 requires fresh whole backing or explicitly authorized retirement of
the old installation after preserving binaries, locks, backups, blocked attempts
and receipts. Frozen 0.70/0.71 payloads retain Running/Uncertain deliveries and
exact receipts; native recovery/creation refuse their physical images with zero
writes/growth. Installed post-upgrade refusal preserves every seeded stable byte.
That test seeds a stopped current fixture; it does not execute an old application.
There is no converter, automatic reset or implicit effect disposition.
See the [consumer guide](../../qualification/ic-jobs-test-consumer/README.md).

Wire accepts redundant varints within width bounds; over-width input still refuses.
Encoder bytes, identities and budgets remain unchanged. Preserve original frames
for retries/hashes; every Job restoration still passes through `Job::restore`.
Jobs stays Serde-only and consumer-owned storage/metadata remain outside the
library. Timers retains its sole private provider, exact guard and cohort gates.
Do not reintroduce ic-memory.

Registry review before capture confirmed the adopted suppliers. Optional
qualification Thiserror 2.0.21 remains deferred to the next targeted maintenance;
its lock retains 2.0.18. Managed Cargo Sort Derives 0.14.1 awaits the next Toko
tool-catalog refresh for 0.15.0. Revisit these at maintenance or next release review.

## Earlier evidence and recovery custody

The [previous handoff](https://github.com/dragginzgame/ic-scheduling/blob/cf3dc91123b8ccb3e1b02c6c58f481a89cf40242/docs/status/current.md)
retains earlier release identities, qualification limits and evidence locations.
Preserve completed `batch-LYs1wQ6P` (Timers 0.43.2 / Jobs 0.35.2), `batch-t4HsIlVh`,
`batch-vqHkyqbM`, `batch-zgh_z6sJ`, `batch-ZNhgLJjb` and `batch-6xiXTmGx`.
Preserve failed `batch-1wgxqWCb`, `batch-ACww0dNn`, `batch-XVdlf0HL` and all older
recovery records. Interrupted `batch-BZtlH7k8` stopped before metadata/delivery
to correct Runtime prose; source `6ca263e`, no release commit, push or publication.

Preserve stashes `4adfda0e74ffee32281b835735a25128b7648ceb`,
`19beca78079c6437b91eb436d789521fc86ac760` and
`7f1fe032b88665507192692dcea2278aeec713a6`. Never restore whole older locks over
released versions, replay publication or implicitly retire protected custody.
