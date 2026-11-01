# Current handoff

## Current scope and suppliers

Complete the authorized Logging 0.23.3 / Metrics 0.24.1 adoption, Shared Tooling
0.27.23 update and documentation-only CI selection for
[#83 — Adopt conservative documentation-only CI selection](https://github.com/dragginzgame/ic-scheduling/issues/83).
Push source under the standing instruction and reconcile the issue with evidence.
No own package bump, release or upload. Jobs stays 0.33.2 and Timers 0.35.0;
compatible pending notes remain Jobs 0.33.3 and Timers 0.35.1.

The formerly separate dirty lock patches are now explicitly authorized and
qualified. Original inputs, source comparisons, canonical impact/refresh logs,
commands and results are at `/tmp/ic-scheduling-observability-ci.fLZWMG/`.
Their published Rust source trees match Logging 0.23.2 and Metrics 0.24.0 exactly.
No unrelated lock edits are included. Metrics defaults stay disabled; logs remain
heap-only and bounded. One Metrics 0.24 identity serves Timers, consumers and Testkit.

Other suppliers remain Build/Host Artifacts/Fs/Process 0.73.3, Testkit/Native
0.75.2, Wire/Derive 0.13.2, Platform/guard/macros 0.15.3, Types 0.15.1,
Memory/Storage 0.60.3 and Endpoint 0.2.1. Jobs stays Serde-only and has no runtime
dependency on Timers, Memory, Platform or a database.

Shared 0.27.23 is canonically exported from clean committed revision
`619d194875369e0f72b45477b66adb430acbf6b7` (132 payloads, including the classifier).
Dirty sibling edits were never exported. No snapshot payload was hand-edited.
All nine members retain explicit matching versions within independent Jobs/Timers
groups. Root catalogs/alias rosters and group ledgers remain the version owners;
there is no shared root version or new `VERSION` file. Shared's pending VERSION
automation does not apply to these Cargo groups. No artifacts were cleaned.

## Qualification and CI selection

Rust 1.91 native/Wasm Clippy and all 243 selected native tests pass: 172 Timers
and 71 consumer cases. The managed Testkit 0.75.2 CLI is prepared on this graph;
Build 0.73.3 metadata-editor admission passes. Both actual group release adapters
pass their complete consumer fixtures, with real Cargo/editor/Git effects and
substituted full gates, setup and uploads. No real release effects occurred.

The 18 actual workflow-selection cases pass through the real Git/classifier,
workflow run bodies and admission scripts; native setup/Make effects are substitutes.
They cover all six allowed prose paths, code/mixed/policy/workflow/packaged changes,
missing history, tags, divergent pushes, tested PR merges, bad whitespace and
failed/invalid classifiers. The modified consumer tooling fixtures pass, including
concurrent policy edits stopping actual Make/CI before product Cargo effects.
Actionlint 1.7.12, ShellCheck 0.11.0, snapshot/pins/group/graph boundaries, formatting,
local links and diff checks pass. Shell lint follows the shared fixture helper.

CI classifies the exact tested `github.sha` with complete history and PR base or
push before identity. The consumer allowlist has six exact prose paths. It excludes
package READMEs, governance/baseline, release ledgers/contracts, snapshot companions,
source/tooling/policy and unknown paths. Policy bytes must agree at both commits.
Unknown or unavailable comparisons select full checks; classification-job failure
fails all dependent validation jobs before tool effects. No workflow path filters.

All three existing `validate` job names remain on Linux, macOS ARM and macOS Intel.
Docs branches perform host-tool admission, snapshot integrity, local links and
changed-whitespace checks, with a summary stating that native qualification was
not selected. Code/tooling changes keep full native CI; releases always keep the
complete combined gates. The classifier, workflow and policy are guarded inputs.
New hosted CI runs asynchronously; no current macOS or full local CI pass is claimed.

No new live IC qualification ran for these source-identical Observability patches.
Prior Build/Testkit qualification is `/tmp/ic-scheduling-build-host-testkit.KztxX1/`:
83 native cases, group metadata fixtures and all 11 live consumer recovery tests.
Frozen original-Wasm evidence is `/tmp/jobs-recovery.2IcNgn/`, bound to its selected
lock, managed tools and candidate patch against `408fcbb`. Those live inputs used
Logging 0.23.2 / Metrics 0.24.0; do not relabel them as a new run on this graph.

## Delivery and release custody

After source push and issue reconciliation, no further work is selected in this
batch. Review asynchronous hosted results; a future library release remains a
separate explicit operation. Both pending groups are compatible patch batches.
Canic's prior read-only handoff recorded Timers 0.35.0 adoption and the previous
publication blocker resolved; no Canic changes or fresh qualification occurred here.

Published releases remain Jobs `jobs-v0.33.2` /
`4a76520d2265ab598113f00c4fbde0111268ffe2` and Timers `timers-v0.35.0` /
`e6bace3fa4d6a8fb6719e51b967aea3ae952cf80`. Their journals and prior Timers 0.34.0
journal are complete. Never replay effects or rewrite registry artifacts.
Timers release evidence stays `/tmp/ic-scheduling-timers035.602bpJ/`: full CI,
43 live watchdog cases, policy cohorts and exact registry/archive/VCS readback.
Earlier evidence and the prior situation audit remain linked from the
[previous handoff](https://github.com/dragginzgame/ic-scheduling/blob/9da81e542ec2060cee6bf8994448ef125cbe6a1b/docs/status/current.md).

Require explicit `RELEASE_GROUP=jobs|timers` for release/resume/publication.
Direct delivery remains `origin`/`main`; publication is separate. Preserve saved
source, policy, payloads and receipts when reconciling a specific old attempt.
Consumers own metadata, storage, handler selection and lifecycle reconstruction.
Restore only through `Job::restore`; Running/Uncertain effects remain blocked
until exact receipts or explicit owner disposition. Preserve bounded `JQW\x01`
queue bytes, retry identities, rollback, binaries/backups and attempt custody.
Never clear allocation ledger independently of queue. Logging stays heap-only at
32 KiB; Metrics reset neither clears logs nor reuses IDs. Timers retains the sole
private provider, confined unsafe FFI and one exact Platform guard. Rust floor
remains 1.91. Native tests alone do not establish IC recovery.

Jobs provenance is `50fac19a55624c2110778e21ba5bb15ef8bf469a`; imported Timers source
`651efce370bd6aec6290b74118eb709488062d0a` remains provenance, not a build input.
Migration custody stays `/tmp/ic-scheduling-merge.7PRSi3Nq/`. The private Canic
adapter stays `/tmp/canic-jobs-adapter.dPFVZ7FD/source`, branch
`codex/jobs-application-adapter`, commit `fc14bbe1baa3c0fdc09b0550e25a1d3fdcd9d2d1`;
it needs separate integration qualification. No shipped Canic mount API is claimed.
