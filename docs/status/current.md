# Current handoff

## Cleanup and Testkit adoption delivered — 2026-11-08

Timers 0.41.1 and Jobs 0.34.14 are tagged, pushed to `origin/main` and published
to crates.io. Source `b6e62260000f9136f2b6d4ef6192141d1d46d05b` was captured in
supplier-first `batch-3JHG7IaR`; its intent is complete at index 2 and both group
journals are complete. Normal main was fast-forwarded through the guarded shared
reconciliation helper. Do not replay either upload.

- Timers commit: `306bdf7342af9f7fcf8901b5b7573a1e6959c972`;
  annotated `timers-v0.41.1`: `6ae3266752c5e7d0732246438e1d229cfc8f2158`;
  archive SHA-256: `19fdeddff0a6add2fea8475c3226626bf60bf990cfb566939cf3ea88dc9eddba`.
- Jobs commit: `0ba498e484e8d6e8efc98866f21b3eb743a7fd44`;
  annotated `jobs-v0.34.14`: `fe613b57f8e3960ea629936ba6e6697ae852cbb3`;
  archive SHA-256: `e553cb749e2fded602d016ec6a0dbae52c42132f5c9da49602cd775d693312fc`.

Official metadata, downloaded archives, both retained Cargo archives, clean VCS
provenance, every published Rust source file and remote annotated tags agree.
This is library publication with local installed qualification.

The delivered graph selects Testkit 0.93.0, native Platform 0.25.1, Storage 0.68.8,
Runtime/macros/guard 0.20.0, Endpoint/macros 0.8.0, Wire 0.17.12, Types 0.18.2,
Host 0.75.0, Build 0.78.0, Logging 0.24.11 and Metrics 0.29.6. Shared Tooling is
0.31.5, revision `b2026a118e38de4af1555c527af327cb604eacdf` (152 managed files).
Final released lock SHA-256:
`c4b6c10e7fec30a1d1e16f1ffc3cf04a14638056a8f0b433ec05daf4f215a67e`.

## Maintained boundaries and cleanup

Public library APIs and retained application/checkpoint formats are unchanged.
Jobs remains Serde-only, consumer-owned storage/effects use validated Job::restore,
and Memory remains excluded. Timers requires one matching Runtime 0.20 executor
cohort; old 0.19 suppliers remain rejected. See [Canic integration](../canic-integration.md).

Ordinary completion, scheduling observations, stale-delivery accounting and
capture retention now have shared owners. The ten follow-up items consolidate
probe creation, typed owner-slot fixtures, termination state, freezing setup,
checkpoint facts and queue snapshot validation; narrow Jobs Tokio/self-auth to
development and Timers Tokio to current-thread execution; and type scenario
commands. Fresh reconciliation retains its independent empty-slot proof.
Private ordinary observation now uses Wire identity 65 instead of retired identity
22; rebuild private probes and host decoders together. No fallback is retained.

The earlier four-item Timers cleanup reduced production LOC from 4,822 to 4,793.
The ten follow-ups retain 5,465 production lines (including Jobs' 672); test LOC
is 25,022 → 25,056. The normal Jobs harness graph loses seven packages, including
SHA-2 0.11 and Tokio macros. No production dependency was removed; cargo-machete
0.9.2 found no unused direct dependencies. Focused evidence and measurements:
`target/evidence/cleanup-ten-20261108/`.

## Qualification and limitations

Both complete 37-stage Linux/Rust 1.99.1 release gates passed, each including all
35 installed Timers runtime cases and policy cohorts. Tooling fixtures passed
rollback, captured-source isolation, uncertain publication, second-group recovery,
completed resume and reconciliation; fixture uploads were substituted.
Gate roots beneath `target/release-frozen/batch-3JHG7IaR/target/release-validation/`:
Timers `validation.20TvZ4Vr`, Jobs `validation.HMlbHoNf`.
Runtime receipts beneath that checkout's `target/tmp/`:
`timer-qualification-3350910-0/runtime.json` and `timer-qualification-3072354-0/runtime.json`.

The first Timers attempt stopped before metadata preparation at PocketIC download
HTTP 504; its complete failure evidence remains at `validation.qGEpY1Qo`.
An explicit setup retry also got 504. The existing exact authenticated PocketIC
16.1.0 bundle was copied as regular files into the captured checkout and admitted
by Testkit; standard batch resume then reran the complete gate. No failed stage
was relabelled or skipped. Raw logs: `target/evidence/deploy-cleanup-20261108/`.

Final Jobs release source passed all 11 installed recovery cases, queue workflow
and all five CLI cleanup modes. Log: `final-jobs-installed.log` in that evidence
root; captured bundle: `target/release-frozen/batch-3JHG7IaR/target/tmp/jobs-recovery.ok7bv7/`.
Its captured Wasm SHA-256 is
`c4d7101c50d565a910561cc82358ce2f7cd3c37c9ae96bc96e21e13255018796`.

Hosted [Timers CI](https://github.com/dragginzgame/ic-scheduling/actions/runs/42469004832)
failed on all three hosts at host-tool download HTTP 504, including one retry.
Hosted [Jobs CI](https://github.com/dragginzgame/ic-scheduling/actions/runs/42471357280)
failed at the same setup boundary. Neither reached Rust validation; no fresh hosted
Linux/macOS pass is claimed. Retry hosted checks when downloads recover.

## Pending suppliers and custody

A separate normal-checkout update selected Runtime/macros/guard 0.20.1, Storage
0.68.10, Testkit 0.93.1 and Wire 0.17.13 during delivery. It was excluded from the
captured releases, preserved on `deferred/batch-3JHG7IaR-suppliers` at
`da7c549431c6b791d72fb20059458dcc1972ab4e` and stash
`f54126407ea2376497b35c4f916c17d3aab0e1a2`, then reapplied after reconciliation.
Only its supplier hunks remain uncommitted; qualify them before any next release.

Preserve `.git/release-state/frozen/batch-3JHG7IaR/`: durable source/intent, complete
journals/validation receipts and `verification/{timers,jobs}/` with registry,
archives, runtime/cohort, installed Jobs and deferred-update evidence. No batch,
source pin, older tag, stash or build artifact was retired or deleted.
Earlier batches, supplier branches/stashes, prior release identities, issue/Canic
history and superseded qualification remain in the immutable
[preceding handoff](https://github.com/dragginzgame/ic-scheduling/blob/b6e62260000f9136f2b6d4ef6192141d1d46d05b/docs/status/current.md).
No sibling source or release changed.
