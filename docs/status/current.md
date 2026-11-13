# Current handoff

## Release fixture repair; fresh batch required — 2026-11-13

Frozen `batch-j7ESjtM_`, source `6802fb9bf8343b13945e95bfbd011ed8bef3e9d6`,
stopped at `test-timer-dependencies` before version preparation or delivery.
The full release-tooling and validation-runner fixtures, admission, Clippy and
176 Timers unit tests passed. Preserve the failed attempt and its source-bound
evidence at `target/evidence/deploy-scheduling-20261113.uvS7gR/` and
`target/release-frozen/batch-j7ESjtM_/target/release-validation/validation.2YV46vtq/`.

The dependency fixture still copied the retired root `make/` directory. Replace
that copy with the installed fixture preparer's explicit consumer/dependency/
release Make roots and companion closure. The complete focused dependency
fixture now passes, including its later graph and static-provider refusals.
Other local callers contain no remaining copies of the retired Make/hook/action
directories. No managed payload changed.

This repair changes captured source, so do not resume the failed batch with new
inputs. Stage-level reuse is qualified only for the pure validation fixture;
there is no general admission for the earlier successful stages after this
source change. A fresh batch must retain both full 36-stage plans.

Concurrent normal-lock Metrics 0.30.5 is preserved on unpublished local branch
`deferred/metrics-during-batch-j7ESjtM_`, commit
`e752ab9e59e594ca553037c6d17864cf53337e18`, plus the patch in release evidence.
The fresh release retains the reviewed Metrics 0.30.4 graph; reapply the deferred
lock hunks after final reconciliation without restoring old product versions.

## Authorized patch release preparation — 2026-11-13

Release Timers 0.44.1, then Jobs 0.36.1, through the frozen batch with explicit
GitHub and crates.io delivery. Shared tooling/layout and locked supplier changes
retain both complete 36-stage plans. Focused vendor caller/metadata/recovery
qualification above is available; the release owns the full selected product and
installed-runtime gate, without an extra full CI run first.

The captured graph includes Metrics 0.30.4, Runtime/macros/exact guard 0.22.2,
Types 0.20.2, native Platform 0.32.2, Wire 0.22.1 / Endpoint 0.13.1, Storage
0.74.0, Testkit 0.102.0, Build 0.84.1, Host 0.77.1 and Toko 0.42.5. Registry
freshness review confirms these selected first-party suppliers are current,
stable and non-yanked. Their patch ledgers preserve production contracts;
Runtime retains its exact singleton guard, and Endpoint preserves frame bytes,
identities and rollback. Both locks remained unchanged during review. Serde,
Serde JSON/Test, Tokio, Clap and Cargo Sort selections are current. Qualification
retains Thiserror 2.0.18 for PocketIC 16.1's exact constraint; product uses 2.0.21.
Optional Cargo Sort Derives 0.15.0 is deferred to the next reviewed Toko native
catalog adoption, retaining authenticated 0.14.1 and the selected tool catalog.

Preparation evidence: `target/evidence/deploy-scheduling-20261113.uvS7gR/`.
Source commit, release validation and delivery are pending. Product/private
versions have not changed during preparation; prior deferred refs remain intact.

## Local Toko 0.42.5 vendor migration — 2026-11-13

Both workspace pins/locks and the installed receipt select published Toko
Tooling 0.42.5, archive SHA-256
`071631f382cbdb682ab04242bb8022bea9f1fa237eaa273456433b95ad6f2aec`.
The authenticated installer previewed and moved 105 managed executable/input
files into `vendor/toko-tooling/`, retaining 108 recorded files overall. The
complete selection explicitly includes release and publication components.
Root governance files and Scheduling-owned scripts/configuration retain their
ownership. No managed files or receipt were edited by hand; no old-path wrappers
remain. Historical checksum/source records retain their original paths.

Local Make, workflows/actions, shared Perl imports, fixture closures and release
callers now use the vendor payload. Selected Cargo/IC tools receive the explicit
consumer root; CI/release validation supplies `VALIDATION_REPOSITORY_ROOT` so
checkpoints observe Scheduling inputs. The local hook configuration selects
`vendor/toko-tooling/.githooks`. Common tools, Build's release editor and the
unchanged Testkit CLI are prepared and admitted in the consumer's `.tools`.
Direct LOC examples now state the host-tool PATH requirement. Endpoint/domain
ownership clarification needs no product API change here.

Focused checks pass: snapshot/pins/membership, dependency inventory, release Make
adapters, affected consumer phases (including 56 tooling refusal routes and CI
input drift), formatting hook/evidence, workflow classification and Actionlint,
release-status refusal, validation-plan/recovery/exact-input reuse, and both
groups' real metadata preparation/rollback/private-version preservation cases.
Product checks and uploads are substituted in those release fixtures. The four
artifact CLI tests and one runner CLI test pass on Rust 1.100 without source-only
tooling markers. All 603 documentation links pass. Repository and qualification
LOC callers report zero unclassified lines. Earlier failed fixture attempts are
retained in the evidence; the corrected callers passed their affected reruns.

Evidence: `target/evidence/tooling-refresh-20261113.C06MVz/`. Only Toko changed
relative to the incoming locks; the separate Metrics 0.30.3, Wire 0.22.1 and
Endpoint/macros 0.13.1 selections remain pending supplier qualification. Product
versions remain Jobs 0.36.0 / Timers 0.44.0; patch notes are pending. This adoption
has no full CI/live/macOS/Canic qualification, commit, push, publication or sibling
edit. Preserve all earlier release and deferred-ref evidence.

## Timers 0.44.0 and Jobs 0.36.0 published and reconciled — 2026-11-13

Frozen `batch-OpbeLrxp` completed GitHub delivery and crates.io publication from
source `fe7c7d4dbdccd5d03c413ca7a3b7754b2a41b5ea`. The normal `main` checkout
was reconciled through the guarded batch command. Both annotated remote tags and
registry index entries were independently verified; downloaded crate bytes match
the actual upload archives and registry checksums. Both declare Rust 1.100.0.

- Timers 0.44.0: commit `ba8d055a22d11edcc1d155b0690382351744ea0e`, tag object
  `32aa270b23cc1bc61f7d21b5e114a6b4afefe948`, archive SHA-256
  `0db52eb4fab46b8dfab283687ce457235a8445f2a8fe928b7e13648ea49a09a1`.
- Jobs 0.36.0: commit `91db54b340472a34f9f2c20af4039296ad79269a`, tag object
  `d020d18bc7e23cc7d770da14d8c22b2ca0a182c3`, archive SHA-256
  `3f8689dcefb01760fcce0df6ecf531e13a51c747612142a9322b5ff01d97d8f7`.

Both complete 36-stage Linux/Rust 1.100 plans passed. Each ran 37 installed
runtime subjects and all four policy/two calibration cohorts. Timers runtime
receipt: `target/release-frozen/batch-OpbeLrxp/target/tmp/timer-qualification-2388049-0/runtime.json`;
Jobs gate runtime receipt: the same batch's `target/tmp/timer-qualification-2022254-0/runtime.json`.
Cohort diagnostics are `timer-qualification-2523014-0` and
`timer-qualification-2062979-0` under that batch's `target/tmp/`. Only the pure
validation-runner fixture reused exact-input evidence in Jobs; other obligations
executed. Fresh selected-source lint and upload-archive verification also passed.
This is not fresh macOS, Canic or hosted-CI evidence.

Evidence: `target/evidence/deploy-scheduling-20261113.9RHB0R/`; release exit 0.
Observed preparation began 15:20:03 UTC; release ran 15:25:31–16:24:52 UTC.
Recorded validation stages total 1,839 seconds for Timers and 1,517 for Jobs,
including 9/8 seconds of archive-verification stages. Release-tooling fixtures
dominated at 1,198/1,193 seconds; the validation-runner fixture took 222/10 seconds
(executed/reused). No finer upload/network phase timing is claimed.

During validation, another writer refreshed normal-checkout Metrics to 0.30.3
and qualification Wire to 0.22.1. Frozen delivery retained the qualified Metrics
0.30.2 / Wire 0.22.0 graph. Preserve the unpublished local branch
`deferred/dependencies-during-batch-OpbeLrxp`, commit
`a097f945bc69136573420b086018725a79a8d0e7`, and its patch in release evidence.
Reapply only its lock hunks after reconciliation, retaining released product
versions; these supplier refreshes remain pending qualification and publication.
Keep all prior batch, stash and deferred-ref recovery evidence.

## Authorized release preparation — 2026-11-13

The selected batch is Timers 0.44.0 followed by Jobs 0.36.0, including GitHub
delivery to `origin/main` and crates.io publication. Both groups require their
complete 36-stage plans because compiler, tooling and shared inputs changed.
The common frozen batch owns validation and publication; no separate full CI
run is added before it. Minor versions account for Rust 1.100 and the public
Metrics generation change. Private package versions remain unchanged.

Release freshness review confirms the selected Runtime 0.22.1, Types 0.20.1,
Platform 0.32.1, Wire 0.22.0 / Endpoint 0.13.0, Storage 0.74.0, Testkit 0.102.0,
Build 0.84.1, Host 0.77.1, Metrics 0.30.2 and Toko 0.41.1 are published current
suppliers. Metrics 0.30.2 and Host 0.77.1 were already in the qualified graph;
their patch ledgers concern tooling/policy and preserve production contracts.
Cargo freshness probes did not change either lock. Qualification retains
Thiserror 2.0.18 for PocketIC 16.1's exact constraint; the product selects 2.0.21.
Managed native tools retain the authenticated Toko catalog. Optional Cargo Sort
Derives 0.15.0 and native catalog refreshes are deferred to catalog maintenance;
the installed Cloc invocation counts selected source files rather than archives.

Preparation evidence and timing: `target/evidence/deploy-scheduling-20261113.9RHB0R/`.
Formatting passes; release validation, delivery and publication are pending.

## Local Wire/Platform/Build adoption; executor blocker resolved — 2026-11-13

Both roots select Runtime 0.22.1 and its exact guard; qualification's entrypoint
macros also select 0.22.1. Qualification selects Wire 0.22.0, Endpoint/macros
0.13.0, Types 0.20.1, native
Platform 0.32.1 and Build 0.84.1. Endpoint 0.13's Runtime 0.22 support removes
the prior executor-adoption blocker. Old Runtime 0.21 and Types 0.19 records are
gone from the selected locks; Storage and Timers share one Runtime/guard/IC0
backend. Native DTOs and Testkit share one Types generation, while native harness
graphs remain executor-free and the facade stays outside deployed graphs.

The graph policy and negative fixtures admit the new 0.22 owner and preserve
exact guard/source matching. The temporary two-Runtime/two-Types counts are
removed; composed checks require one identity each. Storage 0.74, Testkit 0.102,
Host 0.77, Metrics 0.30 and Toko 0.41.1 are retained. Unrelated selected package
versions/checksums are unchanged. Wire frame bytes, method IDs, budgets, storage
formats and blocked/uncertain-effect behavior retain their existing contracts.

Evidence: `target/evidence/wire-platform-build-20261113.Px76dw/`. Strict native/
Wasm Clippy, affected Rustdoc, 258 native tests (including five CLI tests),
dependency rejection fixtures and tooling/pin/group/boundary admission pass.
All 11 installed Jobs recovery tests pass; capture:
`qualification/target/tmp/jobs-recovery.OBwu7q/`. All 37 installed Timers runtime
subjects pass; receipt: `target/tmp/timer-qualification-1640490-0/runtime.json`.
Four policy and two calibration cohorts pass; diagnostics:
`target/tmp/timer-qualification-1709791-0/`, with the report copied into the
adoption evidence. The runtime receipt binds the final qualification lock
`f67e3cab375449a62a4ada91425afed7b0b9fdf1f029191b5b9346bf55561ff2`.
Package inputs stayed fixed during installed qualification.

The Build 0.84.1 editor is prepared. Both groups' focused metadata preparation,
rollback, private-version and foreign-supplier preservation fixtures pass with
real Cargo/editor/Git receipts and substituted product/upload effects.
Owned current integration docs and pending ledgers reflect the aligned cohort;
historical records remain source-bound. These are Linux/Rust 1.100 checks, not
full CI/release or fresh macOS/Canic qualification. Product/private versions are
unchanged; no commit, push, publication or sibling edit occurred.

## Local Toko 0.41.1 refresh — 2026-11-13

Both workspace pins/locks and the installed receipt select published Toko Tooling
0.41.1, archive SHA-256
`93c16a4886a3f48275cf6518a99d98f3f189ae493ed8bf2af21344820c4ab2a0`.
The installer retained 60 entrypoints and 108 files, updating only the baseline,
dependency-list helper and tooling-selection reader. `make deps-list` now labels
the product workspace `(root)` and the private workspace `qualification/`.

The release permits an exact source-only tooling marker for affected binary-only
tests. Both current binary-owning packages passed without it (four artifact CLI
tests and one supervised-runner CLI test), so their existing declarations remain.
No dummy imports, new library targets or unused-dependency allowances were added.

Installed selection, pin/group admission, read-only group versions and focused
inheritance/dependency-update/tooling callers pass, including 56 refusal routes.
Only Toko changed relative to the incoming locks; the supplier adoption and
DropGuard cleanup below are preserved. Evidence:
`target/evidence/tooling-refresh-20261113.8IPLlv/`.
No full CI/live qualification, commit, push or publication was performed.

## Local supplier adoption and Rust 1.100 cleanup — 2026-11-13

Qualification now selects Testkit 0.102.0, Storage 0.74.0, Build 0.84.0 and the
three used Host packages at 0.77.0. Native Testkit selects Platform 0.32.0 /
Types 0.20.0 independently of deployed DTO/entrypoint Types 0.19.5. Storage's
Runtime 0.22.0 enables IC0 only; Timers and Wire Endpoint still use Runtime
0.21.3 with its sole exact guard. Both use one IC0 backend. The product root
selection is unchanged; no supplier was forced outside its published range.

This resolves the earlier Storage/Testkit adoption blockers by checking actual
capabilities rather than requiring one generation throughout the lock. Full
Timers executor/CDK adoption of Runtime 0.22 remains blocked by the latest
published Wire Endpoint 0.12.2's `^0.21.0` requirement. The direct Types 0.19
selection remains necessary at that deployed typed boundary. Graph admission
checks the composed consumer, Storage's IC0-only leaf and executor-free native
harness; the existing dependency rejection fixture passes.

Rust 1.100's `std::mem::DropGuard` replaces the private `WakeupDelivery` type,
its manual destructor and completion flag. Construction still precedes the
first poll; dismissal occurs only on normal completion. The abandonment body,
borrow-safe capture cleanup, uncertain-effect policy and private FFI are retained.
Other named fault/resource guards retain their domain invariants; no allocator
or never-type rewrite was justified by the reviewed code.

Evidence: `target/evidence/supplier-rust-20261113.AqkMfQ/`. Strict native/Wasm
Clippy, affected Rustdoc, 176 Timers unit tests and 77 private tests pass. Installed
Jobs recovery passed 11 tests; 37 Timers runtime subjects passed with receipt
`target/tmp/timer-qualification-3185218-0/runtime.json`. The subsequent cohort
test executed successfully, but its collector refused because the library README
was edited during execution. Preserve the refused attempt at
`target/tmp/timer-qualification-3290841-0/`; it is not an admitted cohort report.
Its source-sensitive collector has no qualified post-processing recovery.
Only the affected cohorts were rerun after finishing the package README edits;
the fresh four-policy/two-calibration report passed and was published at
`qualification/target/timer-cohorts.json`, with diagnostics under
`target/tmp/timer-qualification-3700887-0/`. The earlier successful runtime and
Jobs evidence retains its original captured inputs; it was not rewritten.
Both groups' real metadata/editor rollback and private/supplier preservation
fixtures pass with Build 0.84. Tooling, pin/group/boundary admission and
documentation links pass. No full repository/release or new macOS/Canic gate is
claimed from these Linux checks.

The updated release editor and Testkit CLI/server were explicitly prepared.
Product/private versions remain unchanged; Jobs 0.36.0 and Timers 0.44.0 notes
describe the pending batch. No commit, push, publication or sibling edit occurred.

## Local Toko 0.41.0 adoption and warning-mode qualification — 2026-11-13

Both workspace pins/locks and the installed receipt select Toko Tooling 0.41.0,
archive SHA-256 `4aaafc699324611501e051132e971288c16392a3ece605e9909e88e0c7e80a2f`.
The installer retained 60 entrypoints and 108 files, changing the baseline and
Cargo tool-build helper. Installed `make deps-list` reports all eight packages.
Only Toko changed relative to the incoming locks; no supplier upgrade was added.

Both roots and Timers' equivalent local table deny the new Cargo dependency,
metadata, inheritance and unknown-lint checks, plus invalid Rustdoc tables.
Ordinary Clippy recipes use `CARGO_BUILD_WARNINGS=deny`; the existing consumer
fixture proves warning refusal for fresh and reused artifacts and checks actual
recipe selection. Plain JSON rejection and explicit Rustdoc denial remain.
Product/private native and maintained Wasm/feature Clippy and Rustdoc pass without
warnings. Tooling/pin/group admission, inheritance/update/refusal callers and both
focused publication-admission fixtures pass. Publication fixtures use real
tooling/Git/tag admission with substitute transport/uploads.

The changed installer policy required readmission of the same locked IC Build
0.83.0 editor and Testkit 0.101.0 CLI; both explicit setups reused admitted cache
artifacts and their checks pass. The [deployment notes](../releases/validation-groups.md#deployment-sequencing-and-recovery-limits)
explain captured-lock setup, independent native suppliers and optional upgrade
probes versus actual required-qualification failures. No release gate was removed.

Evidence: `target/evidence/tooling-0410-20261113.cgXk9H/`. These are local Linux
checks on Rust 1.100.0, not fresh macOS, full CI or installed-runtime qualification.
No product/private versions, commits, pushes or publications changed. Earlier
pending compiler, Metrics and manifest cleanup work remains in the checkout.

## Local Toko 0.40.2 and clean Cargo/Clippy manifests — 2026-11-13

Both workspace pins, locks and the installed receipt select published Toko
Tooling 0.40.2, archive SHA-256
`7881b793f4754148fd9b24705a7772243076cd1a298f0e64c757c087ba153d4b`.
The root installer preserved 60 entrypoints and 108 files, updating the baseline,
dependency Make module/listing helper and group parser. Only Toko changed in the
incoming locks; Metrics 0.30 and the working supplier graph remain selected.

Removed unused product aliases, three private probe aliases, qualification's
unused Thiserror declaration and unused description/repository workspace fields.
The product group dependency-alias rosters are empty; both member rosters and all
eight packages remain intact. Real qualification product/source dependencies
remain. `make deps-list` reports all package versions and group ownership offline;
owned instructions and the metadata fixture now use those actual boundaries.

Strict Clippy passes without compiler or Cargo manifest warnings for products,
private qualification, all four size-probe modes and maintained Wasm selections.
Tooling, pin/group admission and focused inheritance, dependency-update,
tooling-refusal and group-recipe callers pass. Evidence:
`target/evidence/tooling-clippy-20261113.MdR0di/`.
Both groups' focused metadata fixture passed with the real Cargo/editor/Git
and plan receipts, including product/qualification rollback, private/supplier
preservation and refusal to recreate unused aliases. Product suites, setup and
uploads were substituted in that fixture. Documentation links also passed.

Cargo also observed Testkit 0.102.0 during this scoped Toko refresh; defer that
optional adoption to the next Platform/Storage maintenance batch, which must
recheck current Wire/Endpoint requirements. The failed optional upgrade probes
below do not block release of the existing qualified selection. No full CI/live
runtime suite, product version change, commit, push or publication was performed.

## Local Metrics 0.30 adoption; Platform/Storage upgrade blocked — 2026-11-13

Both root catalogs and locks select published Metrics 0.30.0, archive SHA-256
`aa71da71d785f922a85cf412be796c8b7946c3dfb35461eda3274aed334d34c0`.
The existing Rust 1.100.0 floor meets its requirement. Pending Timers 0.44 exposes
Metrics 0.30 recorder identities; published Timers 0.43 exposes Metrics 0.29.
The public README distinguishes those identities and current integration choices.
Jobs remains Serde-only; its Metrics dependency belongs to private qualification.

Published Platform Runtime 0.22.0, Types 0.20.0 and facade 0.32.0 were reviewed.
The requested retry also reviewed newly published Storage 0.74.0: it requires
Rust 1.100.0 and Runtime 0.22 for stable IO, with unchanged APIs/stored formats.
Storage's previous Runtime requirement is therefore aligned in the new release.
Runtime 0.22 resolved in the product root, but actual combined qualification
resolution still failed: current Wire Endpoint 0.12.2 requires Runtime `^0.21.0`,
whose exact executor guard conflicts with 0.22.0's singleton guard.
Testkit 0.101.0 retains Types `^0.19.3` and facade
`^0.31.2`, and Wire 0.21.5's identity support retains Types `^0.19.0`.
These are the latest stable non-yanked supplier releases observed through Cargo's
refreshed registry index. The separate crates.io HTTP API returned 403; the
published Cargo records and resolver diagnostics are retained in the evidence.

The attempted Platform/Storage changes were removed without overriding guards, patching
suppliers or breaking qualification. Runtime/exact guard 0.21.3 and Types 0.19.5
remain selected, with Storage 0.73.6; the native transitive facade remains 0.31.3. Revisit Platform/Storage
when the affected suppliers publish compatible Runtime/identity requirements.
This is an unresolved adoption request, not qualification of Platform 0.22 or
Storage 0.74. Retry evidence: `target/evidence/platform-storage-20261113.GRO32R/`.
Both manifests and locks were compared byte-for-byte with the incoming state;
locked metadata and tooling/pin admission passed. No runtime suite was repeated
for these unchanged inputs. Earlier Metrics evidence below remains applicable.

Evidence: `target/evidence/platform-metrics-20261113.p4kmbC/`. Strict Timers and
consumer native/Wasm lint passed, as did all 176 Timers unit tests, four consumer
observability tests and five report tests. The focused installed watchdog
trap/recovery/upgrade subject passed with real instruction and memory recording;
receipt: `target/tmp/timer-qualification-4163092-0/runtime.json`. It binds the
new lock, Rust 1.100.0 and exact Wasm bytes on Linux. It is not a complete
watchdog/cohort release gate or new macOS/Canic qualification.

Both locked fetches, tooling/pin checks and native/Wasm dependency graph admission
passed. Only Metrics changed relative to the incoming locks; pending supplier,
compiler and tooling work is preserved. No product versions, sibling sources,
commits, remote refs or publications changed.

## Local Toko 0.40.1 deployment-guidance adoption — 2026-11-13

Both catalogs and Cargo locks now select published Toko Tooling 0.40.1,
archive SHA-256 `fb884b05e2e5bbca2388dd0b757833fb5ad37e91b3ced09f6a37b454c7fe9f90`.
The root installer preserved all 60 entrypoints and 108 managed files; only the
baseline payload changed. All non-Toko lock selections are unchanged. This
supersedes the 0.40.0 tooling selection below and preserves its Rust 1.100.0 work.

The patch adds deployment sequencing and failure-specific recovery guidance;
it supplies no new executable recovery mechanism. The [local adapter review](../releases/validation-groups.md#deployment-sequencing-and-recovery-limits)
records one selected gate per group, fresh publication admission and the current
post-processing recovery limit. Prior full release-tooling fixtures took 1,159
and 1,133 seconds across the two groups. Only the validation-plan fixture has
qualified stage reuse; do not infer arbitrary runtime recovery from the new rule.

Evidence: `target/evidence/tooling-0401-20261113.m1tInu/`. Both locked fetches and
metadata graphs, installed snapshot/member admission, pin checks, documentation
links and focused dependency-update/tooling-admission callers passed, including
56 refusal routes before compiler, Git or network effects. No executable payload
or product code changed in this refresh; the previous compiler/product evidence
remains applicable. No full gate, commit, push or publication was performed.

## Local Toko 0.40.0 and Rust 1.100.0 migration — 2026-11-13

The fix for [#93](https://github.com/dragginzgame/ic-scheduling/issues/93) is present
in the normal checkout; the issue is closed with the local proof and host limits.
Both owning roots and the development/release pin select
Rust 1.100.0. Explicit setup prepared rustfmt, Clippy and Wasm; actual identities
are rustc `0ef1bc386059596dada8fb65b50425aab2ea637a` and Cargo
`3acc79c0c841844675bc53a605922dbfc441e750` on x86_64 Linux.
Hosted CI/recovery selectors derive the same pin; older mocked policy vectors
remain deliberate normalization/refusal inputs, not selected real compilers.

Both catalogs and locks select crates.io Toko Tooling 0.40.0, archive SHA-256
`9c1758c3c04e2754d97fd38d29177d73fc03d3d414d654f689d8b75cbc7ed918`.
The root installer preserved all 60 entrypoints and 108 managed files. Only the
baseline changed in the payload; no snapshot-owned file was patched manually.
The prior unpublished 0.39.18 adoption is superseded; its evidence remains at
`target/evidence/tooling-adoption-20261112.hdn7tt/`.

Seven packages inherit their owning root's common Rust/Clippy/Rustdoc minimum.
Timers mirrors it in the documented local table so only its private FFI can use
the existing unsafe expectation. Safe packages still forbid unsafe code; roots
retain missing-docs denial. Timers retains reviewed pedantic/individual checks,
with warning denial in blocking commands. Blanket nursery and obsolete line-count
and IC-local Send expectations are removed. Existing protocol/report fields and
macro-generated interfaces are documented; no missing-docs exemption was added.
Owned queue commits, timing conversion and lifecycle frames retain narrow
ownership expectations. Jobs value APIs now expose their must-use contract.
Record layouts, callback semantics and blocked/uncertain effects are unchanged.

Evidence: `target/evidence/tooling-rust-20261113.ElypLV/`. Strict native/Wasm Clippy
and Rustdoc passed, including every size-probe feature. All 224 product tests,
15 doctests and 82 private/native consumer tests passed; both local product
archives verified with the new compiler. The actual Make/hosted selector and
admission callers passed, including 56 tooling refusal routes. A real disposable
consumer verifies every member's effective table, safe-code admission and rejected
ignored Results/unsafe declarations. Private LOC retains zero uncategorized lines.

Installed Jobs recovery passed all 11 tests; capture:
`qualification/target/tmp/jobs-recovery.yOUl8V/`. All 37 installed Timers runtime
subjects passed with receipt `target/tmp/timer-qualification-172770-0/runtime.json`.
Four policy and two calibration cohorts passed; diagnostics:
`target/tmp/timer-qualification-248486-0/`, with the cohort receipt retained in the
adoption evidence. These checks qualify this selected local graph on Linux.
Fresh macOS Intel/Apple Silicon execution is unavailable here; selector checks
do not establish native macOS behavior. No full repository/release gate or new
Canic execution is claimed.

Both locks retain every incoming supplier selection byte-for-byte except Toko:
Metrics 0.29.12, qualification Storage 0.73.6/Wire 0.21.5 and the prior Runtime,
Types, Host, Build and Testkit selections. Current native/installed checks apply
to that graph; no unrelated supplier refresh was performed. Registry freshness
observations and deferred newer versions are retained in `supplier-freshness.txt`
in the adoption evidence. Keep the exact PocketIC Thiserror constraint and managed
tool selections until their own maintenance; compiler/Cargo catalog warnings do
not authorize deleting required group/catalog entries.

Product versions remain Jobs 0.35.4 and Timers 0.43.4; all private versions stay
unchanged. The compiler-support cut selects pending Jobs 0.36.0 and Timers 0.44.0
notes. This migration is uncommitted, unpushed and unpublished. No sibling source
or release was changed; existing frozen batches and deferred refs remain protected.

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

## Deferred Metrics after the previous delivery

An updater changed both normal-checkout locks to Metrics 0.29.12 during Jobs'
gate. The captured graph and both releases retained qualified Metrics 0.29.11.
The published Timers archive's lock independently confirms that selection.

The deferred change is preserved locally on
`deferred/metrics-during-batch-dZQHGKiw`, commit
`78fea9dff2f9201d5919ed1f504e32c21832187d`. After the standard guarded
fast-forward, only its two Metrics hunks were reapplied. Product versions remain
Timers 0.43.4 and Jobs 0.35.4; no old lockfile was restored. Both locks are dirty
with this pending refresh, and that branch was not pushed. At that delivery,
Metrics 0.29.12 was not qualified or released. The newer compiler/consumer
qualification above now covers the selected local graph; it is still unpublished.

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
