# Current handoff

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
