# Current handoff

## Upstream adoption and delivery in progress — 2026-11-05

The maintainer authorized current supplier adoption and deployment in this repo.
Registry inventory and resolution-only graph admission are retained under
`target/upstreams-deploy-20261105/`. The selected graph is Logging 0.24.7, Metrics
0.29.2, Platform 0.22.1, Storage 0.67.0, Testkit 0.86.0, Wire 0.17.1 and Endpoint
0.7.0; Types 0.18.1, Runtime/macros/exact guard 0.19.1 and Build/Host 0.74.3 stay.
The full Clippy gate passes, including the native and Wasm combined consumer.
This is not yet installed-runtime qualification or publication. Delivery plan:
Timers 0.40.5, then Jobs 0.34.4, each with the complete group release gate and
separate crates.io upload. No sibling edits or supplier releases are authorized.

## Shared Tooling 0.30.0 standards audit pending — 2026-11-05

The maintainer requested the new Shared Tooling, cloc/test-directory rules,
allow/expect review and a repository-wide best-practices audit. Source began clean
at `0c3f310a17a1a0ea03f9c6e030dbb9559a448332`. The audit source is included in the authorized delivery above; its original
focused evidence below remains attributable to the audited inputs.

Adopted reviewed **0.30.0**, revision
`b37a882314d22bf7ab393b3fe561809377ffd22d`, through the canonical exporter;
all 147 payloads verify independently. Private fixture/tool package roles are
explicit in all six manifests. No singular test owner needs renaming. Actual
cloc accounts for all 138 tracked Rust files, with zero unclassified lines;
5,473 lines belong to public libraries. This is accounting, not code removal.

Removed all three Cargo lint allowances: two obsolete choices and the missing-
errors allowance, replaced by 39 public timer error sections. There were no
source-level allows. Existing scoped expectations remain; native/test and Wasm
Clippy enforce them. Native harness errors and artifact-verifier panic docs are
also explicit. Public scheduling, Wire/storage bytes and dependencies are unchanged.

Repo-owned fixture/qualification scratch defaults to `target/tmp`; explicit
TMPDIR/RUNNER_TEMP locations remain caller-owned. Invalid raw/resolved driver
paths are refused before qualification-run allocation; no global environment is
changed. Shared allocators/Host FS retain cleanup and failure custody. Preserve
historical outside-target attempts; no artifact cleanup was authorized.

See the [rule-by-rule audit](../reports/audits/2026/11/05/shared-tooling/01/report.md)
and [source-layout guide](../fleet-tools.md). The same repository's GitHub
description now names Jobs and Timers; collaborator-only PR admission was already set.

## Focused qualification and limits

Strict Timers native/test/Wasm Clippy, driver Clippy, Rustdoc, 11 driver tests,
four capture tests, queue-client compilation, actual cloc, provider/dependency
checks, ShellCheck, formatting/hook fixtures and selected caller/evidence/CI
fixtures pass. Both groups' real metadata/editor/rollback fixtures pass with
product checks/uploads substituted. Scratch refusal checks use the actual worker
without starting PocketIC. Evidence: `target/shared-tooling030/`.
Initial formatting failures and the invalidated running-driver attempt remain
retained; the complete affected checks were rerun after freezing source edits.
The later committed cloc refresh removes duplicate workspace totals and shortens
labels. Its JSON source hashes/counts/roles and both Make table modes pass unchanged;
`target/cloc-refresh-b37/` retains that proof. The original audit retains 07dce12.
Further uncommitted upstream cloc edits are not part of this reproducible snapshot.
No full CI, installed recovery or fresh macOS qualification is claimed. Hosted
run 41884732991 at the starting commit selected documentation-only checks.

Pending compatible notes: **Jobs 0.34.4**, **Timers 0.40.5**. Package versions
remain Jobs 0.34.3/Timers 0.40.4 before release preparation. The audit itself did
not change Cargo.lock; the subsequent authorized adoption above does. Delivery keeps
explicit group selection and the complete applicable gate. No new reuse/skip
policy or public product mode is introduced.

## Timers 0.40.4 delivered — 2026-11-04

Published release: `30b5b28658612ce00fe793eb0379ba9bb6b5b971`; source preparation:
`e0f642227343b507705f382df9287050f64201aa`. Annotated `timers-v0.40.4` object:
`49142f5e678cca5a4ae7292cd5a500ce7280c10c`. The GitHub push and separate crates.io
upload completed; journal complete. Do not republish. Official/local archive
SHA-256: `60f214dcbad01105fcae2f08d5bafd4abd8274d7031dc467e605de6b24da1591`.
Its clean VCS source, Rust 1.99.1 and three normal dependencies were verified.
Jobs 0.34.3 was not republished. The [delivered handoff](https://github.com/dragginzgame/ic-scheduling/blob/0c3f310a17a1a0ea03f9c6e030dbb9559a448332/docs/status/current.md)
retains graph identities, the 37-stage gate, 35 installed subjects/cohort,
structural measurements and qualification limits for that exact release.

## Custody and boundaries

Preserve `.git/release-state/deployment-timers0404.OHlQFZ45/`, the completed
`timers-0.40.4.plan` and version-2 receipts; also preserve
`deployment-timers0403.XBMtMHVE/` and its completed journal/receipts. Original
source, dependency, archive and installed identities must not be relabelled.
Shared release admission enforces retained byte/file/age budgets without cleanup.

Eight members retain explicit versions across Jobs (three) and Timers (five).
Root manifests/lock own the cohort. Jobs stays Serde-only, consumer-owned and
validated through `Job::restore`; uncertain effects need explicit disposition.
Timers retains its sole private provider, FFI confinement and exact Runtime guard.
Canic owns reporting and application integration; copied Metrics observations
create no endpoint. Storage owns admitted Cells; retired Memory images are refused
without writes. Same-release upgrades do not prove old-Wasm migration or rollback.
See the [consumer checklist](../timers/adoption/0.40.md).

## Earlier deliveries and protected history

Timers **0.40.2** (`dc0c2cd`), **0.40.1** (`f03d430`) and Jobs **0.34.3** (`0eec50a`)
remain pushed/published. The [previous delivered handoff](https://github.com/dragginzgame/ic-scheduling/blob/af1ff7db20125e29edc82ad9323c317b6cd1de6e/docs/status/current.md)
retains exact 0.40.2 qualification, historical footprint measurements and earlier
delivery identities. Preserve its custody `deployment-timers0402.G6arr0ab/` and
`deployment-consumer-support.lHbrHngp/` under `.git/release-state/`.

Also preserve `deployment-timers0400.IkyEFM1l/`, `deployment-timers0391.k7IkTIwf/`,
`deployment-current.0NCjlIWS/`, `deployment-timers0381.jnjcs5qg/`,
`deployment-upstreams.UjAXfFHz/`, `deployment-msrv-87.uUHPASl6/` and
`deployment-wire-tooling.w6Z6gClk/`, including journals, receipts and archives.
Preserve version-1/interrupted proofs and stashes with their original source;
never pop delivered versions or replay completed uploads. The
[earlier ten simplifications](../redundancy-measurements-2026-11-03.md) retain their
frozen Runtime 0.18 graph; prior 44-case installed Jobs recovery retains `af87eab`.
