# Current handoff

## Jobs 0.34.6 and Timers 0.40.7 delivered — 2026-11-05

Both groups are committed, tagged, pushed to `origin/main` and published to
crates.io. Source preparation: `744394559cf49d08a76c746f2920389a855f30f3`.

- Timers release: `faf15491c727b63f64cf577c2d82837be1890d4f`;
  annotated `timers-v0.40.7`: `88368267a1038647990818ebe30dc89bd737bbdd`;
  crate SHA-256: `29c6e2bcb0e294273e14f9835fd9a0f0ba1b44f8f38a24054877166dfc0efa27`.
- Jobs release: `c19b41d8652deb4152b9350d5df21f48e718c083`;
  annotated `jobs-v0.34.6`: `f76a911616bad2ee7c64569d4032621afe38f435`;
  crate SHA-256: `9b11ad590c158b68c1035114c99affa9aa5577860cddfe7ed860a90117103ee1`.

Official registry metadata and downloaded archives match the local publication
archives and clean release VCS identities. Both remote tag objects and `main`
were verified after publication. Journals are complete; do not replay uploads.

## Delivered changes and qualification

The graph selects Testkit 0.87.0, native Platform 0.23.0, Runtime/macros/exact
guard 0.19.2, Types 0.18.1, Logging 0.24.8, Metrics 0.29.3, Storage 0.67.1,
Build/Host 0.74.4, Wire 0.17.2 and Endpoint/macros 0.7.1. Testkit resolves the
former Platform blocker. Native callers own async runtimes, await transport and
progress, and close instances before servers stop. The capture binary keeps
Tokio/Testkit outside its default normal graph. No automatic update replay was
added; public library contracts, timer ABI and restore admission are unchanged.

Shared Tooling 0.30.4 is committed at
`2aa41c7394300302fe802b69b31b855bdc89b671`; all 149 payloads verify. Its lexical
Candid check runs once before common Cargo boundary admission, with no allowances.
The [source/evidence guide](../fleet-tools.md#documentation-and-experimental-evidence)
records compact-topic retention; no historical/recovery artifacts were deleted.

Both independent 37-stage gates passed on Linux/Rust 1.99.1: 1125 stage-seconds
for Timers and 1119 for Jobs. Each includes native/Wasm, strict lint/docs, provider
and graph policy, release/recovery fixtures, both archives, 35 installed Timers
subjects and policy cohorts with PocketIC 16.1.0. No stage proof was reused.
Publication separately repeated lint and archive verification. Final Jobs source
also passed 11 installed recovery cases, the queue scenario and five terminal
shutdown modes. Hosted CI runs asynchronously; no fresh local macOS execution is
claimed. Same-release upgrades do not prove old-Wasm migration or rollback.

## Current delivery custody

Preserve `.git/release-state/deployment-testkit0407.f2yycLCV/`, complete
`timers-0.40.7.plan` / `jobs-0.34.6.plan` and their version-2 receipts. Custody
holds original inputs, publication intent/logs, official archives/metadata,
source/tag identities and installed receipts. Initial offline admission refused
an uncached dependency before preparation; locked fetching resolved it, with the
original refusal retained. No failed observation is counted as qualification.

Attempts `validation.JsfV9f9Y` and `validation.adU7Qux3` remain under
`target/release-validation/`; delivery logs are under
`target/deployment-testkit0407/`. Final Jobs recovery is retained at
`target/tmp/jobs-recovery.ZAHsRJ/` and copied into custody. Earlier focused
adoption evidence under `target/evidence/testkit-adoption/`,
`target/evidence/shared0304/` and `target/platform-adoption-20261105/` retains its
original source/dependency scope; it is not relabelled as release qualification.

## Earlier deliveries and protected custody

Jobs 0.34.5 / Timers 0.40.6 remain published. Their
[delivered handoff](https://github.com/dragginzgame/ic-scheduling/blob/9414cad9667822ee26ac02d6f41cf639bc2affd4/docs/status/current.md)
retains identities and qualification. Preserve
`deployment-cleanup0406.4fc5n5eC/` under `.git/release-state/`, its completed
journals and receipts. That delivery's failed Wire-drift attempt remains a
failure. Wire 0.17.2 is now in the new published graph above; retain stash
`0f004eda7ab7e02c96edb801416838dff89a4c72` and custody's Wire-only patches without
popping them or restoring old versions. Other writes resumed after that delivery.

The [0.34.4/0.40.5 handoff](https://github.com/dragginzgame/ic-scheduling/blob/5e9a41509d40e4537e49c61b6d1430f84e4f329e/docs/status/current.md),
[Timers 0.40.4 handoff](https://github.com/dragginzgame/ic-scheduling/blob/0c3f310a17a1a0ea03f9c6e030dbb9559a448332/docs/status/current.md)
and [0.40.2/earlier handoff](https://github.com/dragginzgame/ic-scheduling/blob/af1ff7db20125e29edc82ad9323c317b6cd1de6e/docs/status/current.md)
retain older delivery identities, measurements, audits and qualification limits.

Preserve `.git/release-state/` custody `deployment-upstreams0405.xgEN56MD/`,
`deployment-timers0404.OHlQFZ45/`, `deployment-timers0403.XBMtMHVE/`,
`deployment-timers0402.G6arr0ab/`, `deployment-consumer-support.lHbrHngp/`,
`deployment-timers0400.IkyEFM1l/`, `deployment-timers0391.k7IkTIwf/`,
`deployment-current.0NCjlIWS/`, `deployment-timers0381.jnjcs5qg/`,
`deployment-upstreams.UjAXfFHz/`, `deployment-msrv-87.uUHPASl6/` and
`deployment-wire-tooling.w6Z6gClk/`, including journals, receipts and archives.
Preserve version-1/interrupted proofs and stashes with their original source.
Shared admission enforces retained byte/file/age budgets without implicit cleanup.

## Architectural boundaries

Eight members retain explicit versions across Jobs (three) and Timers (five).
Root manifests/lock own suppliers. Jobs stays Serde-only, consumer-owned and
validated through `Job::restore`; uncertain effects need explicit disposition.
Timers retains its sole private provider, FFI confinement and exact Runtime guard.
Canic owns reporting/application integration; copied Metrics observations create
no endpoint. Storage owns admitted Cells; retired Memory images are refused
without writes. See the [consumer checklist](../timers/adoption/0.40.md).
Sibling repositories remained read-only; no supplier release was performed.
