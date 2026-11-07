# Current handoff

## Runtime 0.20 adoption qualified locally — 2026-11-07

Endpoint and Endpoint Macros 0.8.0 are now published and not yanked; official
metadata is retained below. The root catalog/lock select Runtime/macros/exact
guard 0.20.0 and Endpoint/macros 0.8.0, preserving incoming Storage 0.68.7 and
Wire Core 0.17.12. The old execution cohort is gone. Existing ordinary `cdk`
entrypoint attributes already match Runtime's retained macro family; no Rust
implementation change was needed. Jobs stays Serde-only. Stored formats,
owned payloads, explicit request budgets and provider confinement are unchanged.

Version-specific cohort guards and their negative fixtures now require Runtime
0.20 while retaining registry identity, exact guard and dependency exclusions.
Native/Wasm strict lint, graph checks, dependency-refusal fixtures, 178 native
Timers tests, all 11 installed Jobs recovery cases, all 35 installed Timers runtime
cases, policy cohorts and current-source Timers archive verification pass on
Linux/Rust 1.99.1. Logs/metadata/input lock:
`target/evidence/adoption-runtime020/endpoint-ready/`; lock SHA-256
`17e7e9c798234552943eddb088c5b1c8b511941a064898ea2fff811bdd40c663`.
Installed receipts: `target/tmp/jobs-recovery.8iJwvp/`,
`target/tmp/timer-qualification-257607-0/runtime.json` and
`target/tmp/timer-qualification-346423-0/cohorts.json`. The lock stayed unchanged.

Pending notes: Timers 0.41.0 requires consumers to align Runtime/Endpoint suppliers;
Jobs 0.34.13 changes private qualification only. Deployment is now authorized:
release/publish Timers with a minor increment, reconcile it, then capture/release
Jobs with a patch increment. Both use the standard complete release gates;
the current shared batch interface applies one increment per batch. Package
versions remain delivered values until standard preparation. Earlier refusal/404 records under
`target/evidence/adoption-runtime020/` remain historical, not current blockers.
Full release gates and fresh macOS qualification are separate from this adoption.

## Testkit 0.92 adoption delivered — 2026-11-07

Timers 0.40.13 and Jobs 0.34.12 are tagged, pushed to `origin/main` and published
to crates.io through Shared Tooling 0.31.5's captured batch flow. Source adoption:
`417e2d502eeaadc020aff76918f9268672c8bbd2`. Batch `batch-IaXAn_cc` is complete
at index 2. Normal main was fast-forwarded through the guarded shared reconcile
command. Both group journals are complete; do not replay uploads.

- Timers release: `479b92a3e17f64930272da673de59a9ae344ad9f`;
  annotated `timers-v0.40.13`: `ccbe402720ebd786f3136f00db2d2fb0e8da3b84`;
  archive SHA-256: `413d8928c6de95fe8391b57532299b6e2f24c0383953878b048cbf4141f0957d`.
- Jobs release: `84f7b287d9f69f80b128851382ba3ff80de072e4`;
  annotated `jobs-v0.34.12`: `0efedc40c5b083cd1d3da0bdf04764c1583d4b8d`;
  archive SHA-256: `221c34a77914f73358e40f96d24b0e9d797943ae25ada8eb5b854d592a6c3f7f`.

Official registry metadata, downloaded archives, both retained Cargo publication
archives, clean VCS provenance and remote annotated tags agree. This is library
publication with local installed qualification, not production canister deployment.

## Released supplier graph

Testkit 0.92.0 brings Platform 0.24.0's native `CborError` identity. Scheduling has
no direct facade declaration or affected error match to migrate. Native harnesses
and the matching lock-selected CLI adopt the same Testkit generation. Wire 0.17.10
absorbs the previously deferred update; its public codec contracts are unchanged.

Retained suppliers: Storage 0.68.5, Host Artifacts/FS/Process 0.75.0, Build 0.78.0,
Runtime/macros/exact guard 0.19.5, Types 0.18.2, Endpoint/macros 0.7.3, Logging
0.24.11 and Metrics 0.29.6. Shared Tooling remains 0.31.5, revision
`b2026a118e38de4af1555c527af327cb604eacdf` (152 managed files). Jobs stays
Serde-only. Public Jobs/Timers APIs, stored formats, owned Wire payloads, explicit
request budgets and server cleanup are unchanged. Memory remains excluded.

## Qualification and evidence

Both complete 37-stage Linux/Rust 1.99.1 release gates passed, each including all
35 installed Timers runtime cases and policy cohorts. Release-tooling fixtures
passed rollback, captured-source isolation, uncertain-publication recovery,
second-group recovery, completed resume and reconciliation. Their substituted
uploads remain distinct from the actual registry proof above.

Captured evidence under `target/release-frozen/batch-IaXAn_cc/target/`:

- Timers gate: `release-validation/validation.uhOiIwT9/`, actual full run
  `logs/validation.Tj4D1B`; source `417e2d5`; runtime receipt
  `tmp/timer-qualification-416898-0/runtime.json`, cohorts `497888-0`.
- Jobs gate: `release-validation/validation.u99Gavfk/`, actual full run
  `logs/validation.9U7QH6`; source `479b92a`; runtime receipt
  `tmp/timer-qualification-3659947-0/runtime.json`, cohorts `3703997-0`.
- Against final Jobs commit `84f7b28`, all 11 installed recovery cases, the queue
  workflow and CLI cleanup covering five exit/interruption modes pass. Recovery
  bundle: `tmp/jobs-recovery.T0dP4Z/`; combined log in normal
  `target/evidence/deploy-testkit092/jobs-final-installed.log`.

Final released lock SHA-256:
`1f6598129ca0da177ff823ffbe24efe609a79805d8985c2b8e00ae57a001aa67`.
Pre-release focused checks remain historical at
`target/evidence/adoption-testkit092-wire01710/` and the earlier Wire 0.17.9 graph
at `target/evidence/adoption-testkit092/`; neither replaces the full release gates.

Hosted Jobs [run 42384135905](https://github.com/dragginzgame/ic-scheduling/actions/runs/42384135905)
was in progress at delivery. Local Linux passes establish no fresh hosted/macOS
result or cross-release installed migration proof.

## Recovery custody and normal checkout

Preserve `.git/release-state/frozen/batch-IaXAn_cc/`: durable Git store, complete
intent, group journals/receipts and `verification/{jobs,timers}/` with official
registry metadata, all three matching archives, VCS provenance, remote tag refs,
runtime/cohort receipts and final installed Jobs proof. Managed delivery source
and build artifacts remain under `target/release-frozen/batch-IaXAn_cc/`.
Normal raw logs and preparation records: `target/evidence/deploy-testkit092/`.
No batch retirement or deletion was performed.

At that delivery the normal lock matched the released graph; the pending adoption
above now changes its suppliers. No deferred changes occurred during that batch.
The older Wire preservation branch
`deferred/batch-ArjuBucS-suppliers` and stash
`6d9e52644b4d90a59c676dfe4dd4608ab02baec3` remain recovery history; do not restore
their whole older lock over released group versions.

Prior batches `batch-ArjuBucS`, `batch-vLPyIAUg`, `batch-hCRiwOcg` and failed
`batch-qIKZnu6X`, their journals, evidence, branches and stashes remain preserved.
Full prior release identities and custody are linked in the immutable
[preceding handoff](https://github.com/dragginzgame/ic-scheduling/blob/417e2d502eeaadc020aff76918f9268672c8bbd2/docs/status/current.md).
No older tag, upload or receipt was rewritten. Issue #89 remains closed. Canic
issues #735–737 and their local fixes were reviewed read-only; no sibling source
or release was changed here.
