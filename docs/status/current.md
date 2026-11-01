# Current handoff

## Current scope and suppliers

Adopt Build/Host Artifacts/Fs/Process 0.73.3, Testkit/Native 0.75.2 and its required
Wire/Derive 0.13.2, then push under the standing source-delivery instruction.
No own package bump, release or upload. Jobs stays 0.33.2 and Timers 0.35.0;
compatible pending notes are Jobs 0.33.3 and Timers 0.35.1.

The incoming lock already contained these patches plus separate Logging 0.23.3
and Metrics 0.24.1 edits. Those two remain user-owned and unstaged. Original bytes,
scoped lock, source diffs and checks are at
`/tmp/ic-scheduling-build-host-testkit.KztxX1/`. Qualification and commit use the
scoped Logging 0.23.2 / Metrics 0.24.0 graph, restoring the original working lock
byte-for-byte afterward. Do not qualify those unrelated patches by implication.

Other selected suppliers remain Platform/guard/macros 0.15.3, Types 0.15.1,
Memory/Storage 0.60.3 and Endpoint 0.2.1. One Metrics 0.24 identity serves Timers
and Testkit. Metrics defaults stay disabled. Jobs remains Serde-only and has no
runtime dependency on Timers, Memory, Platform or a database.

All nine members remain in one root workspace with explicit matching versions
within independent Jobs/Timers groups. The root catalog and
`ci/cargo-release-groups.json` own complete member/alias rosters and ledgers;
there is no shared root version. A local catalog entry adds no edge until used.
Shared Tooling remains 0.27.22 at `f4f431627290e18f0ccc1e5f8525d16b197cf7c3`
(131 canonical payloads). No snapshot payloads changed and no artifacts were cleaned.

## Current qualification

Focused Rust 1.91 native/Wasm Clippy and 83 native tests pass: consumer 71,
artifact capture 4, build coordinator 5 and protocol 3. The separate real CLI
shutdown test remains explicitly ignored in this native selection.
Both actual group metadata/rollback fixtures pass with the managed Build 0.73.3
editor; Cargo/editor/Git effects are real, full gates/setup/uploads are substitutes.
The managed Testkit 0.75.2 CLI and existing PocketIC 16.1.0 bundle are admitted;
consumer tooling fixtures pass with their stated substituted compiler/server effects.

All 11 explicit live consumer recovery tests pass on the scoped graph. Frozen
original-Wasm evidence is `/tmp/jobs-recovery.2IcNgn/`, retaining the selected lock,
tool identity and candidate manifest/source patch against source base
`408fcbb30d2abc3ab24bc7bee99f74f193fa983c`. This covers the changed Testkit startup
reader and actual Build artifact acquisition while preserving endpoint refusals,
traps, upgrades, authorization and uncertain-effect handling. The published Host,
Native and Wire Rust sources match their prior versions; Build cache acquisition
and Testkit startup have reviewed internal changes. Stored queue bytes are unchanged.

Snapshot integrity, pins, release-group admission, dependency boundaries and all
fourteen graph contexts pass. Formatting, local links and diff checks pass.
No new full local CI, Timers watchdog/cohort run or native macOS qualification is
claimed for this update. Before adoption, all three hosted CI jobs passed at
`408fcbb` in [run 41231445179](https://github.com/dragginzgame/ic-scheduling/actions/runs/41231445179).
New hosted qualification runs asynchronously and must bind the exact candidate.

## Situation audit and next work

Inspection used `audits/ci-release-integrity.md` at the reviewed snapshot above,
scoped to repository workflows, release/publication admission, managed tools,
release custody and supplier status. It is not a new whole-product correctness,
security or performance audit. Detailed observations are retained with adoption
checks in `audit-scope.txt` outside the workspace; findings remain below/in GitHub.

No new delivery blocker was found. CI uses read-only tokens, immutable action
identities and three native hosts; recovery is explicitly manual. Releases retain
full combined CI/live gates, source/prepared receipts, explicit group selection,
annotated tags and atomic exact branch/tag delivery. Publication separately admits
clean source, the exact remote tag and known registry absence before Clippy/upload.
GitHub reports collaborator-only PR creation. Existing direct-main delivery policy
is unchanged; both current release journals are complete and no lock was occupied.

Two LOW-priority follow-ups remain: adopt/qualify the separate Logging 0.23.3 and
Metrics 0.24.1 patches; then review committed Shared 0.27.23 at
`619d194875369e0f72b45477b66adb430acbf6b7` through the exporter. The sole open issue is
[#83 — Documentation-only CI selection](https://github.com/dragginzgame/ic-scheduling/issues/83),
which needs an exact reviewed prose allowlist and actual workflow selection fixtures.
Unknown/code/policy paths and ambiguous history must retain full native CI.
No open PRs were observed. No new library publication is required just for tooling.

Canic's read-only current handoff now reports Timers 0.35.0 adoption, single runtime
identities and the former blocker resolved; its own 0.114 source release is selected
there. That is downstream evidence, not a new Canic qualification or write here.

## Release and recovery custody

Registry releases remain Jobs `jobs-v0.33.2` /
`4a76520d2265ab598113f00c4fbde0111268ffe2` and Timers `timers-v0.35.0` /
`e6bace3fa4d6a8fb6719e51b967aea3ae952cf80`. Their journals are complete; the prior
Timers 0.34.0 journal is retained. Never replay effects or rewrite registry artifacts.
Timers 0.35 evidence is `/tmp/ic-scheduling-timers035.602bpJ/`: complete CI, 43 live
watchdog tests, policy cohorts and exact registry/archive/VCS readback. A fresh
registry consumer compiled both public measurement types with one Metrics 0.24.1.
Earlier release/consumer evidence remains linked from the
[prior handoff](https://github.com/dragginzgame/ic-scheduling/blob/408fcbb30d2abc3ab24bc7bee99f74f193fa983c/docs/status/current.md).

Require explicit `RELEASE_GROUP=jobs|timers` for release/resume/publication.
Direct delivery remains `origin`/`main`; publication is separate. Preserve saved
source, policy, payloads and receipts when reconciling a specific old attempt.
Consumers own metadata, storage, handler selection and lifecycle reconstruction.
Restore only through `Job::restore`; Running/Uncertain effects remain blocked
until exact receipts or explicit owner disposition. Preserve bounded `JQW\x01`
queue bytes, retry identities, rollback, binaries/backups and attempt custody.
Never clear the allocation ledger independently of the queue. Logging stays
heap-only at 32 KiB; Metrics reset neither clears logs nor reuses IDs.

Timers retains the sole private IC provider, confined unsafe FFI and one exact
Platform guard. Rust floor remains 1.91 for core and consumer integration.
Native tests alone do not prove IC recovery; live receipts qualify only their inputs.
Jobs provenance is `50fac19a55624c2110778e21ba5bb15ef8bf469a`; imported Timers source
`651efce370bd6aec6290b74118eb709488062d0a` remains provenance, not a build input.
Migration custody stays `/tmp/ic-scheduling-merge.7PRSi3Nq/`. The private Canic
adapter remains `/tmp/canic-jobs-adapter.dPFVZ7FD/source`, branch
`codex/jobs-application-adapter`, commit `fc14bbe1baa3c0fdc09b0550e25a1d3fdcd9d2d1`;
it still needs its own integration qualification. No shipped Canic mount API is claimed.
