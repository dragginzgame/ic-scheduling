# Current handoff

## Delivered: Timers 0.41.4 — 2026-11-08

Timers 0.41.4 is tagged, pushed to `origin/main` and published to crates.io via
frozen `batch-6xiXTmGx`. Intent is complete at index 1. Guarded reconciliation
fast-forwarded the clean normal checkout; no deferred work needed reapplication.
Do not replay publication. All five Timers members are 0.41.4; all three Jobs
members remain 0.34.15.

- Captured source: `b8a1d6e517a803ceb23643ffacb3464605c46794`.
- Release commit: `432244ca7b30dde502280ce65620c49e502661db`.
- Annotated `timers-v0.41.4` object: `625b2ac864c8a1ba696d7be2b1420f8d573ff0e6`.
- Published archive SHA-256:
  `d661ce75a37884058c9ef35fb9cf4df4aa4ef220b2f6d910bdcb337b6228568e`.

Registry metadata reports the version unyanked. Downloaded bytes equal the retained
upload and identify the clean release commit. All 60 packaged Rust files and the
original manifest match tagged source. Public requirements/features are unchanged.
Preview comparison with the previously verified archives found changed Timers
tests/lock, while Jobs was byte-identical except VCS provenance, including its
normalized manifest and packaged lock. Jobs' maintenance ships through Git only;
its 0.34.16 notes remain pending. No Jobs tag or upload was selected.

## Qualified changes and graph

The interactive queue demo, dedicated tests, launcher, Make targets and obsolete
tooling fixture are removed. Artifact capture, native management transport and
installed Jobs recovery remain. Cargo drops ctrlc and its four unused macOS
backend packages. Native fixture setup is shared; runtime assertions retain actual
provider/public-result boundaries and owner-local semantic matrices remain.
All 89 native runtime tests and independent generated/suspended oracles remain.

Three private next-work setters become one typed request (fresh method ID 66),
reducing the method roster from 64 to 62. Rebuild probe and host together. Message
order, one-time consumption, checkpoint bytes and public library APIs are unchanged.
Rust LOC is 30,517, down 842 from the captured cleanup baseline: tests -306 and
examples -536; libraries remain 5,465, tooling 907 and unclassified zero.

Captured suppliers: Storage 0.69.1, Testkit 0.94.0, Build 0.80.0, Host
Artifacts/FS/Process 0.76.0, Platform 0.27.0, Logging 0.24.11, Metrics 0.29.7,
Types 0.18.2, Runtime/macros/exact guard 0.20.2, Wire 0.17.14 and Endpoint 0.8.1.
Metrics 0.29.7 reached the lock before source capture. Its archive checksum and
clean VCS identity were verified; all Rust source files are identical to 0.29.6.
The full release gate qualifies this captured graph; earlier focused results
remain tied to their earlier lock. Group version projection preserved every
registry supplier record.

Captured lock SHA-256:
`e6176cd50fbbed8fd29bab2c28cbccaa7290e9816eec6d02702b881fd8b6c018`.
Delivered lock SHA-256:
`87a8abda20f7bebfa8f6e711a495aa5874715431b5d30580f1c640fd5ee5d669`.

## Evidence and limits

All 36 required workspace-scope stages passed on Linux/Rust 1.99.1, including
strict lint, native/consumer/Wasm/docs/archive checks and the complete installed
Timers runtime/cohort roster with PocketIC 16.1.0. All 35 runtime cases passed.
The full tooling fixture took 1,396s; the validation-plan fixture executed in
225s. Summed per-stage time was 2,041s. This is not fresh installed Jobs recovery
or macOS execution evidence.

Durable verification is under
`.git/release-state/frozen/batch-6xiXTmGx/verification/`: registry metadata, matching
upload/download archives, source verification, plan/timings, runtime/cohort receipts,
publication-impact and Metrics review, remote identities, completed intent and log.
Original sealed evidence remains in the batch's `repository.git/release-state`.
Raw delivery/reconcile logs: `target/evidence/deploy-cleanup-20261108/`.
Earlier focused cleanup/adoption evidence remains at its original owner.

Hosted [CI run 42515800213](https://github.com/dragginzgame/ic-scheduling/actions/runs/42515800213)
was in progress when observed. The handoff push may supersede it; pending or
cancelled work is not a pass. No wait solely for the optional host matrix is needed.

## Tooling and preserved custody

Root metadata and `toko-tooling.lock` select Tooling 0.34.0 independently of Cargo;
102 entrypoints and 165 managed files remain admitted. Use
`make shared-tooling-update` after changing the declaration and
`make shared-tooling-install` to reinstall the dedicated lock. Preserve tooling
admission before release cleanup/publication effects.

Prior delivery, failed-batch and supplier-deferral identities remain in the
[captured handoff](https://github.com/dragginzgame/ic-scheduling/blob/b8a1d6e517a803ceb23643ffacb3464605c46794/docs/status/current.md).
Preserve batch-OGberYFh, failed batch-TkFUywgM and all earlier batches, tags,
source pins, branches/stashes, archives and recovery artifacts. Nothing was retired.
Jobs stays Serde-only; every restore uses Job::restore and uncertain effects stay
blocked. Timers retains its sole private IC provider, unsafe confinement, exact
guard and runtime/cohort gates. ic-memory remains excluded. Siblings remain read-only.
