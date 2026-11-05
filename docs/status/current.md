# Current handoff

## Three reviewed cleanups pending — 2026-11-05

Implemented locally from `5e9a415`: Watchdog state mutation is private to its
control owner; registry handle custody and observation ordering stay intact.
Eight fixture callers share one Scheduling scratch helper, delegating allocation
and cleanup to the unchanged Shared payload. Focused qualification uses exact
unique-set comparison and retains Cargo listing order in receipts.

Passed 44 registry, 43 selected runtime and six driver tests; strict native/Wasm
Clippy, scratch admission, actual CI-selection/formatting callers, ShellCheck and
all 147 snapshot payloads. Three explicit installed recovery/cleanup subjects pass:
`target/tmp/timer-qualification-755099-0/runtime.json`. The initial sandbox attempt
failed loopback binding; it is retained separately, without a product-failure claim.
Matched Watchdog module/code payload changed 125,019/109,601 to 124,997/109,578
bytes. This is a footprint result, not an execution-performance claim.
Evidence: `target/cleanup-three/`. Pending notes: Jobs 0.34.5 / Timers 0.40.6.
The maintainer now authorizes direct delivery and crates.io publication of both
patches. Retain Storage 0.67.1 in the locked graph, with its official checksum
verified; both complete 37-stage group gates will qualify the combined release
inputs. The focused cleanup evidence above is not that full release qualification.
Delivery order: Timers, publication, then Jobs and publication. Siblings stay
read-only; completed earlier releases below must not be replayed.

## Jobs 0.34.4 and Timers 0.40.5 delivered — 2026-11-05

The requested supplier adoption and pending Shared Tooling audit are pushed to
`origin/main`; both selected public crates are published on crates.io. Both
release journals are complete. Do not replay either upload.
Source preparation: `df8547b0ca07727270b9513656c17596e9363c68`.

Jobs release: `e94f9ee1af2cbdf8127d0a6d3b939ed8951e4bbe`;
annotated `jobs-v0.34.4` object: `0264c7a6fc1e5a1c2c8d97cacb8cc091aa29072a`.
Timers release: `061de2b6b3035cebeab2642b11d050e502d60d6f`;
annotated `timers-v0.40.5` object: `fec3d2f1725e9d0065e0fd3d7aea3cb2494adf1c`.
Official/local Jobs archive SHA-256:
`59e288773b82eee17264ae82f5b7654613e1a507cc5acddbd08ba0ab2068f46e`.
Official/local Timers archive SHA-256:
`a0388684124526f8143314cf8a2ab669aa9335c333cbeeed7a9ac9623ac25fcf`.
Registry downloads match the uploaded archives and their clean VCS commits.

The root graph selects Logging 0.24.7, Metrics 0.29.2, native Platform 0.22.1,
Storage 0.67.0, Testkit 0.86.0, Wire 0.17.1 and Endpoint/macros 0.7.0.
Types stays 0.18.1; Runtime/macros/exact guard stay 0.19.1. Build/Host stay at
latest-published 0.74.3. Storage, Testkit and Endpoint share one Wire identity;
codec-only callers no longer compile its optional metadata derive. Compatible
minimums are preserved; changed minor-line minima are root-owned. No public
Scheduling API or protocol migration is required, and no backing reset is needed.
Siblings were read-only.

## Qualification and custody

Each group independently passed its complete 37-stage plan on Rust 1.99.1:
strict lint, graph/provider/pin checks, release/recovery fixtures, native and
Wasm consumers, docs and both archives. Each plan passed 35 installed watchdog
subjects and the policy cohort using the selected Testkit and PocketIC 16.1.0.
Publication separately repeated lint and verified each release-version archive.
Jobs' published normal graph remains Serde-only; Timers retains three normal
dependencies. Native tests do not establish IC recovery; the installed receipts
supply their separate named proof. No fresh macOS execution is claimed locally.
Hosted [CI](https://github.com/dragginzgame/ic-scheduling/actions/workflows/ci.yml)
runs asynchronously; the Jobs release push was queued when inspected.

Preserve `.git/release-state/deployment-upstreams0405.xgEN56MD/`, both completed
`jobs-0.34.4.plan` / `timers-0.40.5.plan` journals and their version-2 receipts.
Custody retains incoming inputs, source identity, original group gate logs,
installed diagnostics, registry archives/checksums and per-group completion facts.
Detailed original preparation lives under `target/upstreams-deploy-20261105/`;
group validation attempts are `validation.tyunozsE` and `validation.VxEZWZ5w`.
No check was skipped or reused across groups; no artifact cleanup was performed.

## Shared Tooling audit included

Adopted reviewed 0.30.0 at `b37a882314d22bf7ab393b3fe561809377ffd22d` through
its exporter; all 147 payloads verify. Private source roles are explicit; all
138 audited Rust files are counted, with zero unclassified lines. No singular
test owner needed renaming. Removed all three Cargo lint allowances, documenting
39 timer failure contracts; existing scoped expectations remain checked.
Workspace-owned scratch defaults to `target/tmp`, preserving explicit locations
and refusing invalid raw/resolved paths. Public library contracts are unchanged.
See the [rule audit](../reports/audits/2026/11/05/shared-tooling/01/report.md) and
[source-layout guide](../fleet-tools.md). The audit retains its original 07dce12
scope; `target/cloc-refresh-b37/` records the later committed renderer change.
Uncommitted upstream drafts were excluded. The repository description names
Jobs and Timers; collaborator-only PR admission remains configured.

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
