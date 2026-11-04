# Current handoff

## Scheduling deployment in progress — 2026-11-04

The owner authorized deployment of the completed local batch. Delivery scope is
this normal checkout on `main` to `origin/main` (`dragginzgame/ic-scheduling`),
Timers **0.40.1** first and Jobs **0.34.3** second, with separate crates.io uploads.
Both groups require fresh complete five-stage plans because shared tooling and
Timers inputs changed since their annotated baselines. Do not claim earlier
focused checks as complete release qualification. No sibling source/release is
included. Preserve active attempts and reconcile remote/registry state on retry.

Incoming source, untracked files and lock are preserved in
`.git/release-state/deployment-consumer-support.lHbrHngp/`; canonical journals and
version-2 receipts will bind each group's actual release source. Source preparation
and release/publication are in progress; neither new version is delivered yet.

The batch adopts Shared Tooling **0.29.4** at
`628ccc75d813bfd9bae777bfbaef55838d25b1b2` (146 canonical payloads), including
retained-state admission before/around release adapters. Actual retained state
passes the default limits without cleanup; both substituted-product delivery
fixtures pass. Fourteen inline test/provider bodies move to ordinary test files;
167 cases are split into 22 behavior modules with shared fixtures retained.
Production authority, visibility and scheduling semantics are preserved. The
classifier fixture now copies its companion closure. Wasm probes use `std::arch`.

Three new native cases protect retained Once/AfterCompletion/Watchdog owners at
full registry capacity, preserving identity, original callbacks, coalescing and
separate watchdog messages. Public docs list all six creation/reconciliation
capabilities. The [0.39 → 0.40 checklist](../timers/adoption/0.40.md) covers Metrics,
graph alignment and consumer qualification. Canic's guard/funding/retry work stays
consumer-owned. All three cases, Clippy, rustdoc, formatting, links and package
roster checks pass; earlier affected tests retained 241 passes/one existing ignored
live case, and the behavior split passed 176 selected cases. Focused evidence is
under `target/shared-tooling-adoption/`, `target/test-suite-split/`,
`target/shared-tooling-0294/` and `target/timers-consumer-support/`. Earlier Wasm,
policy, selector and plan/evidence fixtures retain their named source scope.
No new installed IC or macOS pass is claimed before the fresh release gates.

## Delivered releases and retained evidence

Timers **0.40.0** is pushed and published: release commit
`01fe9f0c71a43137874578b3126d1058f77e1549`, annotated tag `timers-v0.40.0`.
Its journal is complete; official registry checksum, retained archive and clean VCS
source agree. Source preparation is `d5cf1fa`; delivery handoff is `7e2c0cd`.
The fresh Linux/Rust 1.99.1 gate passed all 30 common CI obligations, Testkit
CLI/server admission, all 35 installed watchdog cases and the policy cohort.
Publication passed final Clippy and normalized archive verification at 0.40.0.
Custody: `.git/release-state/deployment-timers0400.IkyEFM1l/` and completed
`timers-0.40.0.*` plans/version-2 receipts. No new macOS or Jobs recovery is claimed.

Timers **0.39.1** was pushed/published at
`b2c34a636f05e7ff3abea39141ec412f7f764992`, tag `timers-v0.39.1`; its full gate,
publication, checksum and VCS inspection passed with its original supplier graph.
Custody: `.git/release-state/deployment-timers0391.k7IkTIwf/` and `timers-0.39.1.*`.
Earlier Timers **0.39.0** (`0eae01ec5a0c9288f3f736e0f63b21691f1408a2`) and Jobs
**0.34.2** (`5066ac8a6c9010155ffe1b23d38794b6349f6286`) were also pushed/published
with completed journals. Do not replay any completed uploads.

[#88](https://github.com/dragginzgame/ic-scheduling/issues/88) is closed by public
source `7816c9aa2bc81cf414a90fdade60d8c2652cf23d`. Both fresh release gates passed;
44 installed Jobs recovery cases across original/O3/Os/Oz passed at `af87eab`.
Actual Shared runner/verifier and Cargo editor/Git fixtures qualified the adapter;
product/setup/uploads were substituted. Custody:
`.git/release-state/deployment-current.0NCjlIWS/`, `timers-0.39.0.*`, `jobs-0.34.2.*`.
The owner selected Logging 0.24.5 after concurrent lock drift; both graphs and
interrupted evidence remain preserved, and Rust source matches 0.24.4.

[The previous handoff](https://github.com/dragginzgame/ic-scheduling/blob/7e2c0cd28619233a3f663e18bc088f28ee35c0dd/docs/status/current.md)
retains detailed release provenance. Consult current hosted CI rather than claiming
an asynchronous Linux/macOS matrix passed. Same-source install/upgrades do not
establish deployed old-Wasm migration, rollback or destructive data retirement.

## Workspace, qualification and preserved constraints

Eight members use explicit versions in independent Jobs (three members) and Timers
(five members) groups; no root version. `ic-jobs-canister-tests::harness` owns recovery
support behind explicit selection; default artifact capture stays lightweight.
Current suppliers: Logging 0.24.5, Testkit 0.84.1 and its owned PocketIC client,
Metrics 0.29, Runtime/macros/guard 0.19, Types 0.18, Storage 0.66, Wire/Derive 0.16,
Endpoint 0.6; Rust 1.99.1 remains pinned. No retired Memory, Stable Structures or
Testkit Native package is selected. Timers 0.40 exposes Metrics 0.29 and Runtime
0.19 identities; consumers must align their direct Metrics and Runtime/Endpoint.

`ci/release-validation.json` owns 30 common obligations and four standalone Timers
stages. CI and Jobs-only releases keep complete common CI. Clean Jobs-only scope
uses its annotated current tag; shared/unknown/dirty changes and Timers retain all
five stages. See [the consumer mapping](../release-validation-groups.md).
Shared owns bounded proof/evidence mechanics; Scheduling owns product selection,
complete inputs, host/compiler/configuration custody and exact metadata projection.
Version-2 receipts bind immutable Shared proof and retained bytes, admit Git-owned
relocation and reject incomplete policy/inputs. No stage-proof reuse or historical
receipt conversion is enabled. Preserve version-1/interrupted evidence as recorded.

Jobs stays Serde-only with consumer-owned metadata/storage/lifecycle and validated
`Job::restore`. Timers keeps the sole private provider, unsafe FFI confinement,
exact Runtime guard and explicit installed qualification. Retained 0.64 Cells
preserve blocked attempts/receipts under Storage 0.66; retired Memory images are
refused without writes/reset. Uncertain effects require explicit disposition;
no destructive retirement or consumer migration is authorized by this cleanup.

The [ten simplifications and measurements](../redundancy-measurements-2026-11-03.md)
use their frozen Runtime 0.18 graph. Preserve `.git/release-state/` custody including
`deployment-timers0381.jnjcs5qg/`, `deployment-upstreams.UjAXfFHz/`,
`deployment-msrv-87.uUHPASl6/` and `deployment-wire-tooling.w6Z6gClk/`, journals,
receipts, archives and incoming work. Do not pop preservation stashes containing
delivered source/old versions. Historical handoffs remain at `1e9821379` and
`af0e67d`. Workspace `target/` evidence stays removable by normal cargo clean.
