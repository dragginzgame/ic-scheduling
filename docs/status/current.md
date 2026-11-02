# Current handoff

## Delivered upstream adoption — 2026-11-02

Both release groups were delivered to `origin/main` with annotated tags and
published to crates.io under the authorized Scheduling deployment:

| Group | Version | Release commit | Tag |
| --- | --- | --- | --- |
| Timers | 0.38.0 | `97a55b612a559b7f2f66e15964e463d8b6dcbbe6` | `timers-v0.38.0` |
| Jobs | 0.34.1 | `db1a35afd3ad076d8f8eab4f43b5d277e2f16ed2` | `jobs-v0.34.1` |

The source adoption is `8d6453f`. Both downloaded registry archives match the
non-yanked sparse-index checksums, exact release source commits, original
manifests and Rust 1.99.1 metadata. No sibling repository was changed or released.
Hosted release CI was still pending/running at the recorded observation; it is
asynchronous and is not claimed as fresh macOS qualification.

The delivered lock selects Runtime/guard 0.18.1, Types 0.17.0, Wire/Derive 0.15.0,
Endpoint 0.5.0, Storage 0.65.0, Metrics 0.27.1, Logging 0.24.2 and Testkit 0.80.1.
Build 0.74.0, Host Artifacts/FS/Process 0.74.1 and native Platform PocketIC 0.18.0
remain selected. Manifest minima allow the Logging and Metrics patch selections.
No ic-memory, ic-stable-structures or ic-testkit-native package remains in the lock.
Only one Runtime/guard, canonical Types and Metrics identity is selected.

Timers 0.38 is a minor compatibility boundary for its public Metrics identity
and Runtime supplier cohort. Consumers must adopt those suppliers together.
Jobs' public Serde model is unchanged; its patch release carries the qualified
consumer/tooling adoption. Jobs retains consumer-owned storage/lifecycle authority;
Timers retains the sole private timer provider, exact guard and confined FFI.
The Rust floor and pinned compiler remain 1.99.1 for all nine owned members.

## Qualification and storage boundary

Each group independently passed the complete local Linux release gate: full CI,
Clippy, provider/dependency/graph guards, tooling and recovery fixtures, native
core/consumer tests, Wasm checks, documentation and both public package archives.
Each also passed all 35 installed PocketIC watchdog cases and the policy-cohort
comparison. Publication repeated clean-source, exact remote-tag and full Clippy
admission. Shared's optional group-validation narrowing/reuse was not activated.
These results do not establish fresh macOS execution or the separately selected
live Jobs recovery/rollback matrix.

The qualification consumer uses Cells, not Storage 0.65's changed map format.
A real 0.64 installation was frozen with Running and Uncertain attempts plus
exact destination receipts. Native tests reopen that complete image on
0.65/Wire 0.15/Types 0.17 without writes, growth or redispatch. Storage owns native
IO metering through test-support; the local bridge is removed. Stored JQW payloads,
bounded Cell loading, validated Job::restore and refusal ordering remain intact.
Empty recovery never initializes replacement state.

The older 0.63/Memory physical image stays refused without writes/reset, while
its unchanged Wire payload preserves the blocked Jobs. Earlier JSON/CBOR images
retain provenance and refusal coverage. Retiring incompatible installations still
requires owner disposition of outstanding effects and explicit authorization.
This library release resets, migrates or deploys no consumer installation.
Native Cell continuity does not establish compatibility of old map images.

Shared snapshot 0.27.28 at `8c7fae6605c69a930e3b1f14e1ff5c4c9dafbcb7` was exported
from a clean committed source. All 142 payloads verify. Entrypoints and tool pins
are retained. Standard tool admission reused qualified cached selections, including
Testkit 0.80 and the Build 0.74 metadata editor. PocketIC 16.1.0 was explicitly
installed in the isolated checkout for the release runtime qualifications.

## Evidence, checkout and next actions

`.git/release-state/deployment-upstreams.UjAXfFHz/` retains the incoming tree,
source patch/hashes, exact source checkout pointer, both release logs and journals,
validation/preparation receipts, full runtime diagnostics/cohort receipts, registry
archives/checksum receipts and hosted-CI observations. The isolated delivery source
remains under `target/upstream-release-source.jyvBPzEt/`; do not clean it or replay
completed release increments/uploads. `target/current-all-upstream-adoption`
retains the preceding focused adoption evidence and old-reader capture.

The ordinary checkout was fast-forwarded after confirming no concurrent edits.
Its complete pre-deployment adoption remains in a named preservation stash,
identified by `preserved-adoption-stash.txt` in the deployment evidence directory.
Do not pop that stash: its source and old package metadata are already delivered.
Earlier release journals, evidence and preservation stashes also remain intact.

Next work belongs to consumers: adopt Timers 0.38 with the aligned supplier
cohort, and explicitly qualify their installed recovery/rollback paths before
changing any persisted deployment. Library publication is complete. No application
production deployment target is owned by this repository.

[Issue #87](https://github.com/dragginzgame/ic-scheduling/issues/87) remains complete.
Its original delivery evidence is retained at
`.git/release-state/deployment-msrv-87.uUHPASl6/`; the pre-adoption handoff is
`cc78b6258cacdacb44872a9b66e430609108e349:docs/status/current.md`.
Earlier Wire/tooling evidence remains at
`.git/release-state/deployment-wire-tooling.w6Z6gClk/`.
