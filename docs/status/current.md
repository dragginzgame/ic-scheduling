# Current handoff

## Jobs 0.34.7 and Timers 0.40.8 delivered — 2026-11-06

Both groups are committed, tagged, pushed to `origin/main` and published to
crates.io. Adoption source: `7336b8603a42731b03fcebce5409a82c88dcb283`;
corrected qualification source: `116026ce14f147ac363c69dc6f771be103db8474`.

- Timers release: `320b179e43961efd989b161e810dd0a58ed3196a`;
  annotated `timers-v0.40.8`: `c7ecf6b3322751d4387823ecc1a52c21cfbe2bd0`;
  crate SHA-256: `c31fa2c5bc8ac5dca1b226f0845557dcc1529db6ebacb17e955ebf6922f99514`.
- Jobs release: `b579300297d0121234857db32df8ac069a4853c0`;
  annotated `jobs-v0.34.7`: `4098dba41d250bc492cdcc7852e45d67f3f9d69c`;
  crate SHA-256: `689221dcc5b44df7d1619df64cc16d5c70f846a5a1e1e0c0945d9622714add3e`.

Official metadata and downloaded registry archives match the local publication
archives and clean release VCS identities. Both remote tag objects and `main`
were verified after publication. Journals are complete; do not replay uploads.

## Delivered adoption and installation boundary

The graph selects Build 0.75.0, Storage 0.68.0, Testkit 0.88.0, Host 0.74.4,
Platform 0.23.2, Wire 0.17.3, Endpoint/macros 0.7.1, Runtime/macros/exact guard
0.19.2, Types 0.18.1, Logging 0.24.8 and Metrics 0.29.3. Incoming `cc` 1.11.1
was retained and qualified in the release graph. Shared Tooling remains reviewed
0.30.4 at `2aa41c7394300302fe802b69b31b855bdc89b671`; 149 payloads verify.

Build/Testkit keep the APIs used here. Public Jobs/Timers contracts and timer ABI
are unchanged. Storage 0.68 replaces checksummed `ICCELL01` Cells with `SCL1`
framing: use fresh qualification backing. Previous Cells are refused during
recovery/creation with zero writes/growth and identical bytes. Preserve old Wasm,
locks, images, blocked attempts and exact receipts until their owner resolves
outstanding effects and separately authorizes destructive retirement. No legacy
reader, converter, reset or replay was added. The queue identity/codec and
`Job::restore` boundary remain unchanged; valid-looking corruption is no longer
covered by a collection checksum. Native mutation panics require discarding the
failed runtime/leases; native execution alone does not prove IC rollback.

## Qualification and corrected Wire check

Both fresh complete 37-stage gates passed on Linux/Rust 1.99.1: 1124 stage-seconds
for Timers and 1160 for Jobs. Each includes native/Wasm, strict lint/docs, supplier/
provider/Candid checks, release/recovery fixtures, both archives, 35 installed
Timers subjects and policy cohorts with PocketIC 16.1.0. No failed gate or prior
stage proof was reused. Publication separately repeated lint and archive checks.
Final Jobs source also passed 11 installed recovery cases, the queue scenario and
five terminal shutdown modes. Hosted CI runs asynchronously; no fresh local
macOS execution or cross-version installed migration/rollback is claimed.

The first Timers gate stopped before version preparation at a test coupled to
Wire's former `wire Malformed` display text. Its 34 passing installed subjects do
not make that failed gate qualification. The repaired test checks native typed
malformation, IC trap and unchanged registered-timer state, preventing an
unrelated missing-registration trap from satisfying the assertion. Focused
Clippy and the exact installed subject passed before fresh full qualification.

## Current delivery custody

Preserve `.git/release-state/deployment-upstreams0408.lbj697GX/`, complete
`timers-0.40.8.plan` / `jobs-0.34.7.plan` and their version-2 receipts. Custody
holds original inputs, failed-gate inputs/logs/installed diagnostics, correction
proof, publication intent/logs, official archives/metadata and completion records.
The failed attempt `validation.JmvYqRI5` and successful attempts
`validation.JvobPuIj` / `validation.JN3ac3CS` remain under
`target/release-validation/`; delivery logs are under
`target/deployment-upstreams0408/`. Final Jobs recovery is retained at
`target/tmp/jobs-recovery.wNtNlE/` and copied into custody. Earlier focused proof
under `target/evidence/adoption-20261106/` retains its original graph, including
the earlier `cc` selection; it is not relabelled as release qualification.

## Earlier deliveries and protected custody

Jobs 0.34.6 / Timers 0.40.7 remain published. Their
[delivered handoff](https://github.com/dragginzgame/ic-scheduling/blob/d4504c0fa657839c0a4125b61d7a96b6d145a502/docs/status/current.md)
retains exact identities, old qualification and earlier immutable handoff links.
Preserve `.git/release-state/` custody `deployment-testkit0407.f2yycLCV/`,
`deployment-cleanup0406.4fc5n5eC/`, `deployment-upstreams0405.xgEN56MD/`,
`deployment-timers0404.OHlQFZ45/`, `deployment-timers0403.XBMtMHVE/`,
`deployment-timers0402.G6arr0ab/`, `deployment-consumer-support.lHbrHngp/`,
`deployment-timers0400.IkyEFM1l/`, `deployment-timers0391.k7IkTIwf/`,
`deployment-current.0NCjlIWS/`, `deployment-timers0381.jnjcs5qg/`,
`deployment-upstreams.UjAXfFHz/`, `deployment-msrv-87.uUHPASl6/` and
`deployment-wire-tooling.w6Z6gClk/`, with journals, receipts and archives. Retain
stash `0f004eda7ab7e02c96edb801416838dff89a4c72` and custody's Wire-only patches
without popping them or restoring old versions. Preserve interrupted/version-1
proofs with their original source. Shared admission retains byte/file/age budgets;
no implicit artifact cleanup is authorized.

## Architectural boundaries

Eight members retain explicit versions across Jobs (three) and Timers (five).
Root manifests/lock own suppliers. Jobs stays Serde-only, consumer-owned and
validated through `Job::restore`; uncertain effects need explicit disposition.
Timers retains its sole private provider, FFI confinement and exact Runtime guard.
Canic owns reporting/application integration; copied Metrics observations create
no endpoint. Storage owns admitted Cells; retired Memory images are refused
without writes. See the [consumer checklist](../timers/adoption/0.40.md).
Sibling repositories remained read-only; no supplier release was performed.
