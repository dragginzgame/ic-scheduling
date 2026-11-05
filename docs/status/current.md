# Current handoff

## Jobs 0.34.5 and Timers 0.40.6 delivered — 2026-11-05

The reviewed cleanup is committed, tagged, pushed to `origin/main` and published
for both groups on crates.io. Both release journals are complete; do not replay
uploads. Source preparation: `88ff77f42fa3d74c5dd0ae18a84af8351dd4bbc2`.
Jobs release: `e8980445af860c99f817d2db010eeeb73dde7260`;
annotated `jobs-v0.34.5`: `741835a77d6ea96c54e532dd891b12b73649f48f`.
Timers release: `c8cced2932f2e2b33e6afbbeff10c2f5f206c294`;
annotated `timers-v0.40.6`: `871fd2ae77a36d5d9b2f4de211637006758db2f3`.
Official/local archive SHA-256, Jobs:
`64869c5933816650d0b1da73b3a77ca2a707d71f96aae5849c1ee38a4fb18dd7`;
Timers: `3c6bd9f59618daa56f0021a6587827196dad3a9d915d07979722b0fc0fd5dc67`.
Both downloaded archives match the uploads and their clean VCS release commits.

The released graph retains Logging 0.24.7, Metrics 0.29.2, Platform 0.22.1,
Storage 0.67.1, Testkit 0.86.0, Wire 0.17.1 and Endpoint/macros 0.7.0.
Types stays 0.18.1; Runtime/macros/exact guard stay 0.19.1; Build/Host stay 0.74.3.
Root minima and all eight grouped member identities retain their owners.

## Qualification and preserved pending work

Both complete 37-stage gates passed independently on Rust 1.99.1: lint, graph,
provider/pin checks, release/recovery fixtures, native/Wasm consumers, strict docs
and both archives. Each passed 35 installed runtime subjects and the policy cohort
with PocketIC 16.1.0. Publication separately repeated lint and archive verification.
Jobs remains Serde-only; Timers retains its three normal dependencies. No fresh
macOS execution is claimed locally; hosted CI runs asynchronously.

Preserve `.git/release-state/deployment-cleanup0406.4fc5n5eC/`, completed
`timers-0.40.6.plan` / `jobs-0.34.5.plan` and all version-2 receipts. Custody holds
original inputs, group logs, installed diagnostics, official archives/checksums,
source/tag identities and completion records. Attempts: `validation.v9g365AV`
(Timers) and `validation.z7wdeMUo` (Jobs), under `target/release-validation/`.
Preparation and logs also remain under `target/deployment-cleanup0406/`.

The first Timers gate correctly refused an in-flight Wire 0.17.1 → 0.17.2 lock
change. Its failed evidence is retained; it was not counted as qualification.
After the maintainer paused other writes, both entire gates ran on stable inputs.
Wire 0.17.2 was kept separate and restored as the sole local `Cargo.lock` diff
after both uploads, without reverting released package versions. It remains
unqualified pending work and is absent from the released graph. Preserve stash
`0f004eda7ab7e02c96edb801416838dff89a4c72` and custody's Wire-only patches; do not pop old stashes or replay tags.
Other writes may resume after the completed handoff.

## Delivered cleanup

Watchdog state mutation belongs to its private control owner. The registry keeps
claim admission, provider-handle custody, observations and capture cleanup order.
Eight fixture callers share Scheduling's scratch policy while the unchanged
Shared allocator owns allocation/cleanup. Focused subject matching uses exact
unique sets and preserves Cargo listing order in receipts. Public APIs, protocol,
persistence and scheduling/recovery semantics are unchanged; no new product mode.

Original focused proof: 93 native cases, three installed subjects, native/Wasm
Clippy, scratch/companion refusal and actual caller checks under `target/cleanup-three/`.
Its matched Watchdog probe shrank by 22 module bytes and 23 code-payload bytes;
that source-bound footprint observation is not an execution-performance claim.
The initial sandbox loopback refusal is retained separately from the passing
host run at `target/tmp/timer-qualification-755099-0/runtime.json`.

## Previous delivery and Shared audit

Jobs 0.34.4 / Timers 0.40.5 remain published. Their [delivered handoff](https://github.com/dragginzgame/ic-scheduling/blob/5e9a41509d40e4537e49c61b6d1430f84e4f329e/docs/status/current.md)
retains exact source/tag/checksum, supplier and qualification identities, plus the
Shared audit. Preserve `deployment-upstreams0405.xgEN56MD/` under
`.git/release-state/` and its completed journals/receipts.
Shared 0.30.0 stays at `b37a882314d22bf7ab393b3fe561809377ffd22d`; all 147
payloads verify. The [rule audit](../reports/audits/2026/11/05/shared-tooling/01/report.md)
retains its original 07dce12 scope; [source roles](../fleet-tools.md) and the later
committed cloc renderer remain documented. No sibling source/release was changed.

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
