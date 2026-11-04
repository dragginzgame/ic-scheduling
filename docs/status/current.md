# Current handoff

## Timers 0.40.1 and Jobs 0.34.3 delivered — 2026-11-04

Both groups are pushed and published from this normal checkout on `main` to
`origin/main` (`dragginzgame/ic-scheduling`). Source preparation is `65b792f`.

- Timers **0.40.1**: release commit `f03d4304fa2c48e54919af49417fa1859afdc41e`,
  annotated tag `timers-v0.40.1`.
- Jobs **0.34.3**: release commit `0eec50a96fbf0a9133c1737614bbb365b113111a`,
  annotated tag `jobs-v0.34.3`.

Each canonical journal is complete. Each separate `make publish` succeeded from
its clean tagged HEAD. Official registry checksums match retained archives and
VCS release commits. Jobs' archive keeps Serde as its only production dependency.
Do not replay these completed uploads. Canic and supplier repositories stayed
read-only; no application deployment or upstream release was included.

Both fresh five-stage gates passed on Linux with Rust 1.99.1: all 30 common CI
obligations, Testkit CLI/server admission, all 35 installed watchdog subjects and
the policy cohort. Common CI took 575s for Timers and 595s for Jobs and included
178 Timers unit cases, native/Wasm checks and both normalized archives. These
are gate timings, not performance comparisons. Publication passed final Clippy
and archive compilation at each release version. No new installed Jobs recovery
or macOS pass is claimed; hosted CI remains asynchronous.

GitHub returned Internal Server Error for the first Jobs atomic push and its
first resume. Remote reconciliation confirmed the preceding Timers branch and
absent Jobs tag; a second canonical resume delivered the original Jobs commit/tag.
The rejected attempts and their logs remain preserved; no history/tag rewrite or
non-atomic fallback was used.

Custody: `.git/release-state/deployment-consumer-support.lHbrHngp/` retains incoming
work, successful/failing command statuses, registry observations, checksum-matched
archives, VCS/dependency inspection and separate installed diagnostics/cohort reports
for both groups. Completed `timers-0.40.1.*` and `jobs-0.34.3.*` journals/version-2
receipts retain their exact sources. Default retained-state admission passes.

The batch adopts Shared Tooling **0.29.4** at
`628ccc75d813bfd9bae777bfbaef55838d25b1b2` (146 payloads), including release retention
admission. Inline tests/provider support move to ordinary files; runtime, registry
and queue suites use behavior modules. Three new native cases protect retained
owner cancel/rearm and original callbacks at full registry capacity across all
policies, including separate watchdog scheduler/work messages. Public docs list
all six creation/reconciliation capabilities; the
[0.39 → 0.40 checklist](../timers/adoption/0.40.md) covers Metrics, graph alignment
and consumer qualification. Scheduling authority and production visibility stay
unchanged. The classifier fixture copies its companion closure; standard-library
Wasm probes use `std::arch`. No draft remains for this delivered batch.

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
