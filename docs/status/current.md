# Current handoff

## Storage/Testkit adoption delivered — 2026-11-07

Timers 0.40.12 and Jobs 0.34.11 are tagged, pushed to `origin/main` and published
to crates.io through Shared Tooling 0.31.5's captured batch flow. Source adoption:
`43dc4f4e77eec07bfcdf8d19d1b6d7823fc3ecb1`. Batch `batch-ArjuBucS` is complete
at index 2; normal main was reconciled with its final release commit through the
shared guarded command. Do not replay either upload.

- Timers release: `e91c45c4ea14c945440ea373bf86a9b6c0781e53`;
  annotated `timers-v0.40.12`: `a257f6ce87a651fdde10b644400a6e0592e52af5`;
  archive SHA-256: `1dadb4d2894f4a2190f21ad78a9de758ce429aa85877413454706537731e40b6`.
- Jobs release: `6844f3a9b371c11981ae2aea702bbf9b7e5fd1a6`;
  annotated `jobs-v0.34.11`: `4f1283182b78632ad09d204401b523fa18b34dae`;
  archive SHA-256: `1664281cadb460bfa184b19a0770c91799b1d410ad01b303c26305b560105c42`.

Official registry metadata, downloaded archives, both retained Cargo publication
archives, clean VCS provenance and remote annotated tags agree. This is library
publication with local PocketIC qualification, not production canister deployment.

## Released graph and cleanup

Storage 0.68.5 and Testkit 0.91.0 align with Host Artifacts/FS/Process 0.75.0 and
Build 0.78.0; the older Host 0.74 copies are gone. Retained suppliers:
Runtime/macros/exact guard 0.19.5, Types 0.18.2, Platform 0.23.6, Wire 0.17.8,
Endpoint/macros 0.7.3, Logging 0.24.11 and Metrics 0.29.6. Shared Tooling is
0.31.5, revision `b2026a118e38de4af1555c527af327cb604eacdf` (152 managed files).
Jobs remains Serde-only. Public APIs and stored formats are unchanged; the retired
Memory package remains excluded. Explicit request budgets and server custody remain.

Notification fixtures now focus on committed intent, receipts and validated
recovery; actual Queue tests retain management/auth/pagination coverage. Timers
shares capacity, capture and stopping fixtures while retaining typed controls,
classifications, destruction ordering and separate Watchdog messages. Four
Cargo-owned retired-feature rejection cases were removed; provider/dependency
guards and distinct recovery boundaries remain. Jobs uses one integration target
with all 48 cases and frozen records. Installed tests combine 27 adjacent
advance/tick pairs with unchanged durations, budgets and observations. Shared cloc:
tests 25,608 → 25,009 (599 fewer); library/app unchanged at 5,494.

## Qualification and custody

Both complete 37-stage Linux/Rust 1.99.1 release gates passed. Each includes all
35 installed Timers runtime cases and policy cohorts. Real release-tooling
fixtures also passed source isolation, rollback, uncertain-publication recovery,
second-group recovery, completed resume and reconciliation; substituted fixture
uploads are distinct from actual publication proof.

Captured evidence under `target/release-frozen/batch-ArjuBucS/target/`:

- Timers gate: `release-validation/validation.euwjk5fy/`, actual logs
  `logs/validation.Nd6X9I`; source `43dc4f4`; runtime receipt
  `tmp/timer-qualification-1808472-0/runtime.json`, cohorts `1887817-0`.
- Jobs gate: `release-validation/validation.RcYfUyp3/`, actual logs
  `logs/validation.iGdzPK`; source `e91c45c`; runtime receipt
  `tmp/timer-qualification-834726-0/runtime.json`, cohorts `927072-0`.
- Against final Jobs commit `6844f3a`, all 11 installed recovery cases passed;
  receipt bundle `tmp/jobs-recovery.8MIndN/`. Queue workflow and process-level CLI
  cleanup covering five exit/interruption modes passed on retry.

Final released lock SHA-256:
`c72a1402442c0cfaeeae4422b8c5103b56fc7d2f7e48c7085916160254f2e55b`.
Normal raw logs and adoption inputs: `target/evidence/deploy-storage0685-testkit091/`.
The first installed command passed recovery but stopped at queue admission because
a competing process held the shared tool-store installation lock. Its original
log is retained; `jobs-final-queue-retry.log` covers only the remaining checks.

Preserve `.git/release-state/frozen/batch-ArjuBucS/`: durable Git store, complete
intent, group journals/receipts and `verification/{jobs,timers}/` with official
registry metadata, archives, provenance, remote refs and runtime/cohort receipts.
No batch retirement or deletion was performed. Hosted Jobs
[run 42374466527](https://github.com/dragginzgame/ic-scheduling/actions/runs/42374466527)
was in progress at delivery; local Linux passes establish no fresh macOS result
or cross-release installed migration proof.

## Deferred normal-checkout work and prior custody

A concurrent update selected Wire 0.17.9 in the normal lock during delivery. The
captured graph stayed at 0.17.8. Before guarded reconciliation, the later update
was preserved on branch `deferred/batch-ArjuBucS-suppliers`, stash
`6d9e52644b4d90a59c676dfe4dd4608ab02baec3`, plus
`target/evidence/deploy-storage0685-testkit091/deferred-normal-lock.patch`.
Only that supplier patch was reapplied, preserving released group versions.
It remains uncommitted and unqualified for a later batch; do not replace the
current lock wholesale with an old one.

Earlier custody is unchanged, including completed `batch-vLPyIAUg` and
`batch-hCRiwOcg`, failed `batch-qIKZnu6X`, their deferred supplier branches/stashes
and all older release evidence. Full historical identities and recovery references
are in the immutable
[preceding handoff](https://github.com/dragginzgame/ic-scheduling/blob/43dc4f4e77eec07bfcdf8d19d1b6d7823fc3ecb1/docs/status/current.md).
No older tag, upload or receipt was rewritten. Issue #89 remains closed. Canic
issues #735–737 and their local fixes were reviewed read-only; no sibling source
or release was changed here.
