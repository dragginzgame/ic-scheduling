# Current handoff

## Jobs 0.34.8 and Timers 0.40.9 delivered — 2026-11-06

Both groups are committed, tagged, pushed to `origin/main` and published to
crates.io. Initial adoption source: `fff0daeaf2131b1ba4f66922c9128bcfb8669219`;
corrected release-note source: `16f53e59320165181822c933a54b0c7e25c8d962`.

- Timers release: `988c2238ab4d64e7efcd55eb6ed56d50ec74189d`;
  annotated `timers-v0.40.9`: `040eb68054f398b3976c8b3bc2accb2f2e80cae8`;
  crate SHA-256: `741e4de93574f2aa0af7ccad454e782a65e9ad2eea8086fd227ac7ac6ed45951`.
- Jobs release: `f2403cbc5895288e7f611029401554990e66f8f8`;
  annotated `jobs-v0.34.8`: `b1c3dfb4d759b11f71b139dfda65d313c76fdade`;
  crate SHA-256: `67b7920e2e52d0a4df3dfabb70e12c8819ad3a8a512bb27f1d42c72d6c604e60`.

Official registry metadata, downloaded archives and local publication archives
agree, including clean release VCS identities. Both remote annotated tag objects
and release `main` were verified. Journals are complete; do not replay uploads.

## Delivered graph and tooling

The qualified graph selects Wire 0.17.4, Endpoint/macros 0.7.2, Runtime/macros/exact
guard 0.19.3, Types 0.18.2, Platform 0.23.3, Testkit 0.89.1, Logging 0.24.9 and
Metrics 0.29.4. Build 0.75.0, Storage 0.68.0, Host 0.74.4, `cc` 1.11.1 and
`flate2` 1.1.11 remain selected. Compatible root minima are preserved, with
Testkit raised to 0.89. No supplier repository was changed or released.

Shared Tooling 0.30.7 is reviewed at committed revision
`22a853e26c05ec323f06c57fa316c08b808f252b`: 152 managed payloads. Common Rust
source/Cargo admission enforces Platform ownership of CBOR with empty Candid/CBOR
source allowances. Owned DTOs remain Wire-only. All eight members share the root
manifest/lock; there are no browser or independent Cargo roots or maintained
generated Rust entrypoints. Macro expansion, generated strings and arbitrary
aliases still need review. Historical CBOR images remain refusal fixtures.

Frozen releases and stage reuse remain disabled pending their separate consumer
qualification; see the [local adoption status](../release-validation-groups.md#shared-tooling-0307-adoption-status).
This delivery used the normal checkout. No extra release directory was created.
Testkit's removed automatic logging is unused here. Library APIs, timer ABI,
Wire bytes and stored formats are unchanged by this batch.

## Qualification

Both fresh complete 37-stage gates passed on Linux/Rust 1.99.1: 1125 summed
stage-seconds for Timers and 1107 for Jobs. Each includes native/Wasm, strict
lint/docs, dependency/provider/source boundaries, release/recovery fixtures, both
archives, all 35 installed Timers subjects and policy cohorts with PocketIC 16.1.0.
Publication separately repeated Clippy and final-version archive verification.
Final Jobs source also passed all 11 installed recovery cases, the queue scenario
and five terminal shutdown modes. Hosted CI runs asynchronously; no fresh local
macOS execution or cross-version installed migration/rollback is claimed.

The successful attempts are `target/release-validation/validation.r4HTtY9O/`
(Timers, source `16f53e5`, run `validation.W22zPS`) and
`target/release-validation/validation.VwfMn0aV/` (Jobs, source `988c223`, run
`validation.DJmxzu`). Installed timer receipts remain in
`target/tmp/timer-qualification-3596438-0/` and
`target/tmp/timer-qualification-1674980-0/`. Final Jobs recovery is retained at
`target/tmp/jobs-recovery.yxnGZS/` and copied into delivery custody.

## Interrupted attempts and deferred updates

The first Timers attempt, `validation.iYZ0pmJ6` on `fff0dae`, was intentionally
stopped before preparation to correct stale supplier patch notes. Its passing
stages and 35 installed subjects do not make it a completed qualification.
The first Jobs attempt, `validation.oVRZEL5H` on `988c223`, was stopped before
preparation after another process updated the root lock. Neither attempt was
reused; original inputs/logs and interruption records remain in custody.

The deferred Metrics 0.29.5 / Platform 0.23.4 refresh was restored as dependency
hunks after both publications, preserving Jobs 0.34.8 / Timers 0.40.9 workspace
versions. Further updater changes have since appeared in the working lockfile.
These pending refreshes are outside both releases and their qualification; the
tagged locks and verified registry artifacts define the delivered graph. Preserve stash
`d7d1c504df0be4cba608f68eda56aa4196575df8`, custody's deferred lock snapshots and
`deferred-final-Cargo.lock.patch`. Do not pop a whole older lock over release
metadata. Other writers must stay coordinated during future normal releases.

## Delivery custody and earlier releases

Preserve `.git/release-state/deployment-shared0307.kV0zoH90/`, complete
`timers-0.40.9.plan` / `jobs-0.34.8.plan` and their version-2 receipts. Custody
retains inputs, interrupted evidence, deferred refresh/stash identity, publication
intents/logs, official archives/metadata, completion records and installed recovery
proof. Delivery logs are under `target/deployment-shared0307/`; earlier adoption
proof under `target/evidence/shared0306/`, `shared0307/` and
`adoption-wire-platform-testkit/` keeps its original inputs and meaning.

Jobs 0.34.7 / Timers 0.40.8 remain published. Their
[delivered handoff](https://github.com/dragginzgame/ic-scheduling/blob/63aec658fc7f2c68a8b83be6e2a8d400971f7547/docs/status/current.md)
retains exact identities, the corrected Wire check, installation boundaries and
older immutable handoff links. Preserve `.git/release-state/` custody
`deployment-upstreams0408.lbj697GX/`, `deployment-testkit0407.f2yycLCV/`,
`deployment-cleanup0406.4fc5n5eC/`, `deployment-upstreams0405.xgEN56MD/`,
`deployment-timers0404.OHlQFZ45/`, `deployment-timers0403.XBMtMHVE/`,
`deployment-timers0402.G6arr0ab/`, `deployment-consumer-support.lHbrHngp/`,
`deployment-timers0400.IkyEFM1l/`, `deployment-timers0391.k7IkTIwf/`,
`deployment-current.0NCjlIWS/`, `deployment-timers0381.jnjcs5qg/`,
`deployment-upstreams.UjAXfFHz/`, `deployment-msrv-87.uUHPASl6/` and
`deployment-wire-tooling.w6Z6gClk/`, with journals, receipts and archives. Retain
stash `0f004eda7ab7e02c96edb801416838dff89a4c72` and custody's Wire-only patches.
Preserve interrupted/version-1 proofs with their original source. Retained-state
byte/file/age budgets still apply; no implicit artifact cleanup is authorized.

## Architectural and retained-data boundaries

Jobs stays Serde-only and consumer-owned; every restore uses `Job::restore`.
Uncertain effects remain blocked until explicit owner disposition. Timers retains
its sole private provider, unsafe FFI confinement and exact Runtime guard. Canic
owns reporting/application integration; copied Metrics observations add no endpoint.
Storage owns checked allocation and Cells; the retired Memory crate stays absent.
Storage 0.68's prior `ICCELL01` images remain refused without writes or growth:
preserve old Wasm, locks, images, blocked attempts and receipts until their owner
resolves effects and authorizes retirement. No legacy reader, conversion, reset
or replay was added. Native mutation failures require discarding runtime/leases;
native execution alone does not prove IC rollback. See the
[consumer checklist](../timers/adoption/0.40.md).
