# Current handoff

## Complete local upstream adoption — 2026-11-02

The root selects Runtime/guard 0.18.1, Types 0.17.0, Wire/Derive 0.15.0,
Endpoint 0.5.0, Storage 0.65.0, Metrics 0.27.0 and Testkit 0.80.1. Build 0.74.0,
Host Artifacts/FS/Process 0.74.1, Logging 0.24.1 and native Platform PocketIC
0.18.0 are current. Official registry checksums and source identities were
verified for the new cohort. Previous published-supplier blockers are resolved.

Both native harnesses retain the independent PocketIC client and explicit Types
conversions. The deployed graph has one Runtime 0.18, matching guard and canonical
Types identity. Source and feature checks reject competing or substituted owners.
Jobs remains Serde-only; Timers retains the sole private timer provider and FFI.
The Rust floor and pinned compiler stay 1.99.1 for all nine owned members.

Storage now supplies native IO metering through test-support. The old bridge,
Testkit Native dependency and unused direct Platform facade selection are removed.
No ic-memory, ic-stable-structures or ic-testkit-native package remains in the lock.
Only the Metrics 0.27 identity remains in the working graph.

## Retained data and consumer actions

The qualification consumer uses Cells, not Storage 0.65's changed map format.
Before changing its reader, a real 0.64 installation was frozen with Running and
Uncertain attempts plus exact destination receipts. New native tests reopen that
complete image on 0.65/Wire 0.15/Types 0.17 without writes, growth or redispatch.
Stored JQW payloads, bounded Cell loading, validated Job::restore and refusal
ordering remain intact. Empty recovery never initializes replacement state.

The older 0.63/Memory physical image stays refused without writes/reset, while
its unchanged Wire payload preserves the blocked Jobs. Earlier JSON/CBOR images
retain provenance and refusal coverage. Retiring incompatible installations still
requires owner disposition of outstanding effects and explicit authorization.
Adoption resets, migrates or installs nothing. Native Cell continuity does not
establish installed IC recovery/rollback or compatibility of old map images.

Release delivery is now authorized for this repository only: Timers 0.38.0,
then Jobs 0.34.1, followed by each selected crates.io publication. Preparation
uses an isolated main checkout. The frozen working lock also includes Logging
0.24.2 and Metrics 0.27.1 patch selections; the complete release gates will
qualify those exact inputs before version preparation. No result is claimed yet.
The earlier ordinary checkout and concurrent edits remain preserved.

## Shared Tooling and qualification

Shared snapshot 0.27.28 at `8c7fae6605c69a930e3b1f14e1ff5c4c9dafbcb7` was refreshed
through the canonical exporter from a clean source. All 142 payloads verify.
Entrypoints and tool pins are retained. The new group-validation capability is
not opted into: repository CI and both groups keep their existing complete gates.
Common tool admission required standard re-preparation for the current kernel
identity; admitted cached selections were reused. Testkit CLI 0.80 and the Build
0.74 metadata editor pass admission. No PocketIC server was installed or started.

All 269 selected tests pass: 75 consumer, 175 Timers, 3 protocol and 16 native
helper cases. Two explicitly live tests remain ignored. Strict native and Wasm
Clippy, release Wasm builds and all four size-probe feature checks pass. Independent
core/consumer compiler checks, dependency/provider/pin guards, consumer/dependency
fixtures, formatting hooks and both groups' metadata fixtures pass. Formatting,
ShellCheck and documentation links pass. Full CI, fresh macOS and installed IC
qualification were not selected; a release retains those applicable gates.

`target/current-all-upstream-adoption` identifies source/registry archives,
incoming-state snapshots, old-reader capture, qualification/tool logs and receipts.
Both physical images and their provenance remain in the consumer's queue fixtures;
the canonical fixture guide explains their different compatibility boundaries.
Earlier adoption/rejected-probe evidence remains under the previous target pointers.
Existing work and unresolved release-recovery evidence below remain preserved.

## Delivered baseline and recovery

[Issue #87](https://github.com/dragginzgame/ic-scheduling/issues/87) is completed.
The compiler-policy implementation is `a2a6cdd423843d8f7ce5f639c8aeac7711c953c6`.
Both groups were pushed to `origin/main` with annotated tags and published,
non-yanked crates.io versions before this local adoption:

| Group | Version | Release commit | Tag |
| --- | --- | --- | --- |
| Jobs | 0.34.0 | `41391e23c3116fe60901331cd95ac78ec2f11460` | `jobs-v0.34.0` |
| Timers | 0.37.0 | `1389d8e5e55ae3a8efbe6a49d66fc9164055be81` | `timers-v0.37.0` |

Their downloaded registry archives match index checksums and release source
identities, including Rust 1.99.1 metadata. Each release separately passed full
local Linux CI, 35 installed watchdog cases and the policy-cohort comparison.
Those results do not qualify this newer dependency graph or establish fresh
macOS execution or the separate live Jobs recovery matrix.

`.git/release-state/deployment-msrv-87.uUHPASl6/` retains the delivery receipt,
all attempts, registry verification, journals and source-bound runtime archives.
The isolated delivery checkout remains under `target/msrv-87/delivery/` with its
private tool store. Standard private-store setup resolved shared-store contention;
no live lock was removed or validation bypassed. Do not replay completed increments,
uploads or the preserved pre-delivery stash, which includes already delivered work.

The pre-adoption handoff is retained at
`cc78b6258cacdacb44872a9b66e430609108e349:docs/status/current.md`; earlier context
is at `a2a6cdd423843d8f7ce5f639c8aeac7711c953c6:docs/status/current.md`.
Historical Rust 1.91 evidence remains unchanged. Prior Wire/tooling delivery
evidence remains at `.git/release-state/deployment-wire-tooling.w6Z6gClk/`.
