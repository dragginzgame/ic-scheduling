# Current handoff

## Dependencies and Shared Tooling adopted locally — 2026-11-06

The pending graph selects Wire 0.17.4, Endpoint/macros 0.7.2, Runtime/macros/exact
guard 0.19.3 and Types 0.18.2. Incoming Platform 0.23.3, Testkit 0.89.1,
Logging 0.24.9 and Metrics 0.29.4 patches are retained for release qualification;
they were present before the release source commit. Testkit's compatible
requirement is updated at the root; other compatible
minima are retained. Incoming `flate2` 1.1.11 is preserved in this graph.

Testkit no longer sets up file logging or a global subscriber. These harnesses
do not rely on that behavior; no subscriber or replacement sink was added.
Runtime's task-removal optimization preserves cancellation order. Public library
contracts, timer ABI, Wire bytes and storage layouts remain unchanged.

Earlier Linux/Rust 1.99.1 checks passed against Platform 0.23.2, Testkit 0.89.0,
Logging 0.24.8 and Metrics 0.29.3: affected native and Wasm Clippy,
seven Wire contract/storage tests, dependency/provider admission, five exact
installed timer subjects (cancellation, continuation trap, watchdog successor,
trap/upgrade and malformed ingress), and two combined Jobs recovery subjects
(management upgrade/delivery and watchdog batch trap rollback). PocketIC 16.1.0
uses the prepared Testkit 0.89 CLI. The lock stayed unchanged during these checks.
Inputs, registry metadata, captured Jobs Wasm and logs are at
`target/evidence/adoption-wire-platform-testkit/`; the focused timer receipt is
`target/tmp/timer-qualification-2565731-0/runtime.json`. These checks do not
replace full CI, release gates, macOS or cross-version installed qualification.

Shared Tooling 0.30.7 is reviewed at committed revision
`22a853e26c05ec323f06c57fa316c08b808f252b`: 152 managed payloads. Dirty supplier
work is excluded. Common admission enables the shared CBOR source and Cargo
checks with empty Candid/CBOR source allowances. Platform owns external protocols;
owned DTOs remain Wire-only. The actual Make caller fixture proves source refusal
before Cargo and aliased optional target declarations despite local permission.
The selected CBOR fixture and ShellCheck passed in the 0.30.6 batch; evidence is
`target/evidence/shared0306/`. Current export evidence is `target/evidence/shared0307/`.
Frozen releases and stage reuse remain disabled; see the local
[adoption status](../release-validation-groups.md#shared-tooling-0307-adoption-status).
The 0.30.7 integrity check, actual Make release routing fixture and edited-document
links passed. The 37-stage release roster and pending Cargo manifest/lock are
byte-identical to their pre-refresh copies. No product rerun was needed.

All eight members use this root manifest and lock. No browser roots, independent
Cargo roots or maintained generated Rust entrypoints were found. Lexical checks
still require review of macro expansion, generated strings and arbitrary aliases.
Historical CBOR images remain refusal fixtures; no retained bytes were migrated.
CI/release rosters are unchanged. Pending notes are Jobs 0.34.8 and Timers 0.40.9;
package versions remain delivered 0.34.7 / 0.40.8. Publication is authorized.
The first gate on `fff0dae` was intentionally interrupted before preparation to
correct stale patch notes; it is not a completed qualification. Retain attempt
`validation.iYZ0pmJ6` and custody `deployment-shared0307.kV0zoH90/`.

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
was retained and qualified in the release graph. Delivered Shared Tooling was reviewed
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
