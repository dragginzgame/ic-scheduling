# Current handoff

## Delivery selected — Timers 0.41.3 and Jobs 0.34.15

The authorized deployment selects one frozen batch in supplier order
`timers,jobs`, with publication enabled to crates.io and direct `origin/main`
delivery. Both public packages have changed packaged tests: Jobs scheduling and
Timers registry fixtures, compared with the verified 0.34.14/0.41.2 registry
archives. Their original manifests and public dependency requirements are
unchanged. Root Runtime/macros/guard 0.20.2, Platform 0.26.1, Testkit 0.93.2 and
Storage 0.69.0 are included for qualification. No other repository is selected.

Planning evidence is under `target/evidence/deploy-root-tooling-20261108/`.
The complete release plans and installed checks have not yet run for this source;
the published releases recorded below keep their original evidence identities.

First capture `batch-TkFUywgM` at
`bfc4f555620259e9530aa37d0ed192cdb11a5e56` stopped at test-release-tooling before
version mutation/tag/push/upload. The publication fixture replaced a snapshot-owned
registry checker, so the new publish-check integrity prerequisite correctly refused
it. The fixture now keeps that checker intact and substitutes HTTP transport in
a phase-local PATH. A focused publication-admission selection exercises both groups
without replaying metadata/recovery setup. Preserve the failed batch and its logs;
release execution must capture the repaired source as a new batch.

## Root-owned Tooling 0.34 migration — 2026-11-08

Local migration explicitly replaces the 0.33 Cargo-dependency scheme. Root
`[workspace.metadata.toko-tooling]` selects published 0.34.0;
`toko-tooling.lock` owns the archive SHA-256
`5ca8ad09ad9359bc4945a4afdaafce301dd5a4b4955cd5c9f5058d7a54ffcf10`.
The new installer matches committed producer release
`44b294db78477ac253f83e02ccbdd09a96df8f73` and the downloaded registry package.
Bootstrap used that checkout with historical `--source` supplied; new payloads
came from authenticated registry archives. Installation, setup and offline tool
admission passed; all 102 entrypoints remain and 165 managed files verify.

Removed both the root catalog entry and `ic-timers-pocketic` dev-dependency.
Cargo metadata contains no tooling package or dependency. No associated machete
exception or selection-only member existed. Keep `ic-timers-pocketic`: its build
and runtime qualification code remains required. All eight workspace members,
default members, group versions and release-group catalogs are unchanged.
Cargo refreshed the application lock only to remove the tooling package/edge;
all application supplier records, including Storage 0.69.0, are preserved.
Application lock SHA-256:
`b0de1eec40975070144d8706c40859334297aa7db3e26b14a567aff7d0fd1637`.

Change the declared tooling version intentionally, then run
`make shared-tooling-update`. `make shared-tooling-install` reinstalls the existing
tooling lock; application Cargo updates no longer select tooling. Keep the root
metadata, dedicated lock and snapshot receipt together in source control.
Old 0.33 dependency mentions below are explicitly historical evidence; the shared
migration guide describes that scheme solely to remove it.

CI checkpoints now include the dedicated lock. Local release prerequisites admit
tooling before Shared's source-directory cleanup; publish-check and the direct
publication script also admit tooling before product effects. Saved batch context,
full product gates and installed-runtime/recovery requirements are unchanged.
Consumer fixture sources carry the complete receipt and tooling lock; routing-only
fixtures substitute admission/cleanup explicitly.

Focused checks passed: root declaration/lock/snapshot admission, application-lock
independence and 18 actual caller rejection routes for absent locks, version drift
and checksum drift; CI input-drift rejection including the tooling lock; batch
routing and group recipes; both membership catalogs; dependency inheritance and
boundaries; release adapters, CI change selection (tooling lock selects full CI),
formatting failure evidence, formatting, shell lint and documentation links.
Consumer rejections substitute Cargo/Git/network effects, not tooling admission.
Evidence: `target/evidence/tooling-root-selection-20261108/`.
No full product/release suite, commit, push or publication was performed.
The previous Storage adoption and fixture cleanup remain local and preserved.

## Storage 0.69 adoption — 2026-11-08

Local adoption selects published `ic-storage` 0.69.0 from the root catalog and
lockfile. Registry archive SHA-256:
`90b93d6a307e67f7a83ea9de7046bd6a26e405957ada63679997d91024053241`.
The targeted update replaces only Storage 0.68.11; all other pending supplier
selections and installed `toko-tooling` 0.33.0 remain unchanged. Locked fetch
passed. Current lock SHA-256:
`b48189ee0410d943bab44cf5b107cf1f6104b5d7d1c8d24e6a0dbb3f9d0cc970`.

The qualification Queue and its corruption/composition fixtures use direct Cell
writes. Removed obsolete collection error arms; typed creation/load refusal,
consumer-owned `Job::restore`, backing custody and commit-before-effect ordering
remain. Storage's ordinary operation failures still trap. Storage 0.68 cell
images need no reset or conversion; existing historical refusal fixtures remain.
No published Jobs/Timers API, package version or new storage policy changed.

Focused checks passed on Rust 1.99.1: 41 Queue, 13 composition and three stored
Wire tests, plus strict native/all-target and Wasm consumer Clippy. This is native
and compilation evidence, not fresh installed IC recovery or release acceptance.
Evidence is in `target/evidence/storage-adoption-20261108/`.
The prior tooling migration and fixture cleanup remain local and preserved.
No commit, push or publication was performed.

## Native fixture cleanup — 2026-11-08

Local cleanup reduces counted Rust LOC from 31,587 to 31,364 (-223), all in
tests. Tests are 24,240 LOC; library code remains 5,465, examples 752 and tooling
907. Unclassified remains zero. The comparison uses Shared Tooling 0.33's same
LOC classification before and after this batch, including its existing migration
changes and the unchanged pending supplier lock.

Queue scenarios share fresh setup and expected-value constructors; durable reads,
dispatch effects, memory metering and reopen boundaries remain explicit. Four
registry scenarios share policy registration. Jobs' malformed-record cases share
one named table. Composition cases share interrupted/lost-reply reconstruction and
due/future corruption admission, retaining every semantic case.

The notification/maintenance composition harness remains deliberately separate:
its failed-commit injection, retained rejection evidence and two-component Storage
bootstrap are not supplied by the canister Queue. No new test framework, feature,
dependency or public API was introduced. Production source and group versions
are unchanged; both ledgers retain pending private-maintenance notes.

Focused native checks passed: 41 Queue tests, 13 composition tests, 43 registry
tests and 25 Jobs scheduling tests. Strict Clippy passed for the consumer and
Timers test targets and the Jobs integration target. Evidence and before/after
LOC reports are in `target/evidence/cleanup-fixtures-20261108/`.
These checks use the pending supplier graph; they do not establish installed IC
recovery, macOS qualification or full release readiness. No commit, push or
publication was performed.

## Historical 0.33 Cargo-dependency migration — superseded by 0.34

The following records the earlier bootstrap and its evidence. Root-owned tooling
selection replaces it; no Cargo tooling dependency or related update instruction
remains active.

Local, uncommitted adoption selects crates.io `toko-tooling = "0.33"` in the
root dependency catalog, inherited only as a dev-dependency by the existing
private `ic-timers-pocketic` member. Cargo selected 0.33.0 with archive SHA-256
`4bd31469e2819833bcf120144d250c1b917b570682f7dcfdf02da3d219162e81`.
No package or release-group versions changed.

Resolved Cargo metadata and `cargo fetch --locked` passed. The installer was
located through locked offline Cargo metadata and executed from that downloaded
registry package, with `/home/adam/projects/toko-tooling` supplied only for
historical Git ownership proof. All 102 recorded entrypoints were retained;
163 managed files now match the registry receipt. Installation, tool setup and
offline tool admission passed. Existing supplier lock selections were preserved.

Focused checks passed: snapshot integrity, formatting, documentation links,
release-command adapters, dependency pins/boundaries, both release-group
catalogs, actual CI workflow selection and formatting failure evidence. Consumer
batch commands, CI input-drift rejection and group recipes passed with external
tool/product effects substituted. The drift fixture now substitutes the new
tooling admission entrypoint instead of the retired direct verifier call.
The local LOC caller reports zero unclassified lines. Evidence is under
`target/evidence/tooling-registry-migration/`.

This maintenance batch has not been committed, pushed or released. Product
release qualification, installed-runtime coverage, group selection and saved
batch recovery requirements remain unchanged. The delivery below describes the
previously published graph, not qualification of the pending supplier refresh.

## Timers 0.41.2 delivered — 2026-11-08

Timers 0.41.2 is tagged, pushed to origin/main and published to crates.io through
Shared's frozen `batch-3bIH1nRr`. Intent is complete at index 1; the normal branch
was reconciled through the guarded helper. Do not replay the upload.

- Captured source: `ef8c275ea1aa7f1acb874c643fcb737f0df6a399`.
- Release commit: `54984adc58e5e3ee0161b057210f538ee14f7168`.
- Annotated `timers-v0.41.2` object: `4e5bfd4e07401fe4106a3be3453b332b44df5c62`.
- Archive SHA-256: `1db8ff240ea8f7524cc623389375ea7ad479a3c6d664be56ef9e01e35c9e3d8c`.

Registry metadata and downloaded bytes match the actual upload, clean tagged
VCS identity and all 59 packaged Rust source files. The version is not yanked.
Timers and its four private members are 0.41.2. The root index's stale Jobs
private-package count is corrected to two.

Jobs stays published at 0.34.14. Comparison against its verified published archive
found identical payload and normalized manifest except VCS provenance; no Jobs
release was selected. Its private-maintenance notes remain pending at 0.34.15.
Jobs release commit: `0ba498e484e8d6e8efc98866f21b3eb743a7fd44`;
tag object: `fe613b57f8e3960ea629936ba6e6697ae852cbb3`;
archive: `e553cb749e2fded602d016ec6a0dbae52c42132f5c9da49602cd775d693312fc`.

## Qualification and changes

All 36 required release stages passed on Linux/Rust 1.99.1, including early
strict lint, native/consumer/Wasm/docs checks, both archives, all 35 installed
Timers runtime cases and policy cohorts/calibrations using PocketIC 16.1.0.
Metadata projection, source/tag/registry admission and post-bump Cargo archive
verification passed. Native checks do not establish installed Jobs recovery;
no fresh Jobs live recovery is claimed for this graph.

Captured supplier graph: Build 0.79.1, Storage 0.68.11, Runtime/macros/guard
0.20.1, Testkit 0.93.1, Wire 0.17.14 and Endpoint/macros 0.8.1.
Source lock SHA-256:
`3d87f45f81241e1098bcc374cc33f7e5f4f4440bc28a0ec73c763820a4dc8ff1`.
Delivered lock after group metadata projection:
`f32198714a6efdf2f4d45ab36b6c23b0c7e5fd89e7104ed747c88419a50591eb`.

Shared Tooling 0.31.11 is pinned at
`6c529e6940a97226cca826726491e03173b08cd2` (158 managed files). Delivery includes
Cargo-owned formatting/hash admission/LOC reports, one independent full-history
redacted CI secret scan, authenticated download-cache wiring, the flat group
roster and bounded Linux fixture reuse with fresh per-group custody.
There are 32 common CI obligations plus four installed release obligations;
proven Jobs-only scope requires 18 and omits 18. Other stages still execute.

The [structural audit](../reports/audits/2026/11/08/complexity-duplication/01/report.md)
remains immutable. Its three findings and the follow-up test cleanup are addressed.
Rust tests decreased by 593 LOC to 24,463; production stays 5,465, examples 752,
tooling 907, total 31,587, unclassified zero. Timing/observations/teardown and all
scenarios remain; snapshot/cadence preconditions and counter projections are shared.
The reuse regression retains four full-plan attempts and five direct-stage cases.
Its earlier 206s execute/6s reuse measurements are single-stage observations,
not a whole-release benchmark. The largest tooling stage still runs normally.

First capture `batch-9PFRX4TP` stopped before version/tag/push/publication because
the formatting-evidence fixture lacked required workspace metadata and expected
Rustfmt --all. The consumer fixture now checks an explicit Cargo-owned member;
focused rejection/order/archive retention and the complete restarted gate pass.
Preserve that failed batch and its original failure logs.

## Hosted CI and evidence custody

Maintenance source ef8c275 passed hosted Linux and the actual full-history secret
scan in [run 42482714452](https://github.com/dragginzgame/ic-scheduling/actions/runs/42482714452).
Its macOS jobs were cancelled by the newer release push. Release/handoff host
matrices run asynchronously; no fresh macOS code pass is claimed here.

Durable current evidence is under
`.git/release-state/frozen/batch-3bIH1nRr/verification/timers/`: registry response,
metadata, upload/download archives, source verification, runtime/cohort receipts,
remote identities, timing table and complete release log. Group journals and
version-3 validation receipts remain in the batch's repository.git/release-state.
Raw logs: target/evidence/deploy-maintenance-20261108/ (both attempts and fixture
fix), plus earlier target/tmp/cleanup-{three,followup,native-fixtures}/.
No source pin, batch, tag, stash or artifact was retired.

## Deferred supplier update — not part of this release

Normal-checkout updates during validation were excluded from the frozen graph:
Platform 0.26.1, Runtime/macros/guard 0.20.2 and Testkit 0.93.2. They remain pending
and unqualified; do not attribute the release gate to them.
Preserved branch: `deferred/batch-3bIH1nRr-suppliers`, commit
`c08c92c20ea413b6f9813d53ec93ba2e63ddb52a`;
retained stash: `032c4538c774b72e4f32996da5c498e49a3e77e9`.
Pre-reconciliation lock: `1e8bbedaa8419b6f7bf4d0f8c763d37cac632889996d4898c9681d7c35d11a19`.
Reapplied lock: `aa20141b6053368ae1ab2505e4f77c559e272ef190face980b2f35c77338a254`.
All eight local lock/catalog/member versions were checked: Timers remains 0.41.2
and Jobs 0.34.14. The working lock is intentionally dirty; nothing restored old
group versions. Reconciliation/deferral identities live beside verification/.

Earlier release custody, supplier branches/stashes and Canic/issue history remain
in the [preceding handoff](https://github.com/dragginzgame/ic-scheduling/blob/ef8c275ea1aa7f1acb874c643fcb737f0df6a399/docs/status/current.md),
including batch-3JHG7IaR and its completed Jobs/Timers publications. Preserve them.
Jobs stays Serde-only and consumer-owned; every restore uses Job::restore and
uncertain effects remain blocked. Timers retains its sole private IC provider,
unsafe confinement, exact executor guard and runtime/policy gates. Memory remains
excluded; sibling source and releases were unchanged.
