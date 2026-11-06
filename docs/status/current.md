# Current handoff

## Jobs 0.34.9 and Timers 0.40.10 delivered — 2026-11-06

Both groups are tagged, pushed to `origin/main` and published to crates.io through
Shared Tooling 0.31.0's captured batch flow. Delivery source:
`6456b5a498933fde3b9e0cad4d3322d08cd0be76`. The normal checkout was explicitly
reconciled after publication; deferred supplier edits were reapplied as dependency
hunks, preserving the released workspace versions.

- Timers commit: `83a1d1940d7f5e51c56f5e45473d33333e6744a8`;
  annotated `timers-v0.40.10`: `06c20cf79b1366c7fb4f144badb3b37b96ed23b0`;
  crate SHA-256: `4e8ebf1d7ed1afd69bd328a334a7ba4b901b4d48866ddba07be963de0b368810`.
- Jobs commit: `6b5bbc5b283f070682677e1fb5ad6d7a37ff395d`;
  annotated `jobs-v0.34.9`: `971c59431f56d67af7d2ac526af9826b6cc7b803`;
  crate SHA-256: `139022544a16a839e7b80c7e0416780e8620d392e6159c8a49efbd2b1a7f9cf0`.

Official registry metadata, downloaded archives and local publication archives
agree, including clean release VCS identities and remote annotated tag objects.
Batch `batch-hCRiwOcg` is complete at index 2; both standard journals are complete.
Do not replay uploads. No production canister deployment is implied.

## Delivered source and supplier graph

Released Shared revision `74313941af2fce4399bec0e6909ca6229673c3f8` exports 152
managed payloads. Local callers use the 0.31 release settings and renamed runner;
retired settings refuse. Schema-2 intents record publication commits. Shared owns
batch orchestration and lifecycle; consumer setup admits saved tools offline.
See the [adoption status](../release-validation-groups.md#shared-tooling-0310-adoption-status).

The released lock selects Runtime/macros/exact guard 0.19.4, Types 0.18.2,
Platform 0.23.4, Wire 0.17.5, Endpoint/macros 0.7.2, Storage 0.68.1,
Testkit 0.89.3, Logging 0.24.10 and Metrics 0.29.5. Build 0.75.0 and Host 0.74.4
remain selected. Root dependency minima and the single workspace lock retain
authority. Public library APIs, timer ABI, owned Wire bytes and storage formats
are unchanged. No supplier repository was edited or released.

## Current checkout: macOS tooling repair

Hosted CI found a Bash 3.2 parse error in Shared 0.31.0's checksum helper before
host-tool setup on both macOS architectures. The [failed run](https://github.com/dragginzgame/ic-scheduling/actions/runs/42279479219)
records that failure; its Linux job passed. Adopted committed Shared 0.31.1 at
`5efbda2ab81ad84e30f490f3137d8ef083c303d2` through the exporter, preserving 152
managed payloads. The fix extracts the NUL reader outside quoted substitution,
retaining rejection and backend status. Consumer Linux caller checks pass;
fresh hosted macOS qualification remains asynchronous. This Git-only tooling
repair changes no published library archive. Package versions remain delivered;
pending notes are Jobs 0.34.10 / Timers 0.40.11.

## Qualification and recovery

Both complete 37-stage gates passed on Linux/Rust 1.99.1. Each includes native
and Wasm checks, strict lint/docs, dependency/provider/source boundaries, tooling
fixtures, both archives, all 35 installed Timers subjects and policy cohorts with
PocketIC 16.1.0. Stage reuse remains disabled. Publication separately admitted
the final source/tag, repeated Clippy and verified the final-version archive.

Relative to `target/release-frozen/batch-hCRiwOcg/`, successful evidence is:

- Timers: `target/release-validation/validation.QF2TBmEv/`, run
  `logs/validation.abDfFY`, source `6456b5a`; runtime receipt
  `target/tmp/timer-qualification-1914668-0/runtime.json`.
- Jobs: `target/release-validation/validation.SKSyqJHu/`, run
  `logs/validation.Uh4mME`, source `83a1d19`; runtime receipt
  `target/tmp/timer-qualification-1142929-0/runtime.json`.
- Final Jobs source `6b5bbc5` separately passed all 11 installed recovery cases,
  the queue scenario and five terminal shutdown modes. Recovery artifacts are
  `target/tmp/jobs-recovery.RiTgkl/`; the transcript is
  `target/deployment-shared031/jobs-live-qualification.log` in the normal checkout.

An initial batch, `batch-qIKZnu6X` on source `341ac6f`, stopped in consumer-tooling
before version preparation, tags, pushes or uploads. The corrected source isolates
fixtures from inherited batch selections and supplies their new CI inputs. That
attempt's artifacts and failed logs remain preserved; its gate was not reused.

After Jobs' successful gate, system package updates changed the metadata tool's
native-input fingerprint. Explicit setup re-admitted IC Build 0.75.0 from the saved
Jobs source lock. Exact batch recovery completed the saved preparation plan and
publication without repeating Timers or replacing Jobs' successful evidence.
Hosted CI runs asynchronously; no fresh local macOS or cross-version installed
migration/rollback qualification is claimed.

## Custody and deferred work

Preserve `.git/release-state/frozen/batch-hCRiwOcg/`, its `repository.git`, complete
group plans/receipts and `verification/` records. Its managed checkout and outputs
remain under `target/release-frozen/batch-hCRiwOcg/`; no retirement or cleanup was
performed. Preserve the failed `batch-qIKZnu6X` store and checkout as well.
Normal delivery, verification, tool-recovery and reconciliation logs are under
`target/deployment-shared031/`. Earlier adoption evidence stays under
`target/evidence/shared031/` and `target/evidence/frozen-hooks/`.

The normal lock still has deferred Logging 0.24.11, Metrics 0.29.6, Platform
0.23.5 and Storage 0.68.2 updates. They are outside both releases and remain
unqualified. Preserve branch `deferred/shared031-6456b5a`, stash
`96492f00b7f0e99d13898109e413bd0b6acb412f` and the deployment directory's
`deferred-final-Cargo.lock.patch`. Do not restore a whole old lock over current
workspace versions. Optional supplier patches alone do not require republishing.

Earlier Jobs 0.34.8 / Timers 0.40.9 and their custody remain immutable. The
[previous handoff](https://github.com/dragginzgame/ic-scheduling/blob/6456b5a498933fde3b9e0cad4d3322d08cd0be76/docs/status/current.md)
retains exact identities, earlier interrupted attempts and older custody links.
Preserve `.git/release-state/deployment-shared0307.kV0zoH90/`, all earlier custody,
stash `d7d1c504df0be4cba608f68eda56aa4196575df8` and stash
`0f004eda7ab7e02c96edb801416838dff89a4c72`. Retained-state byte/file/age limits
still apply; this delivery grants no implicit artifact deletion.

## Architectural and retained-data boundaries

Jobs stays Serde-only and consumer-owned; every restore uses `Job::restore`.
Uncertain effects remain blocked until explicit owner disposition. Timers retains
its sole private provider, unsafe FFI confinement and exact Runtime guard. Canic
owns reporting/application integration; copied Metrics observations add no endpoint.
Storage owns checked allocation and Cells; the retired Memory crate stays absent.
Storage 0.68's prior `ICCELL01` images remain refused without writes or growth:
preserve old Wasm, locks, images, blocked attempts and receipts until their owner
resolves effects and authorizes retirement. No conversion, reset or replay was added.
Native mutation failures require discarding runtime/leases; native execution alone
does not prove IC rollback. See the [consumer checklist](../timers/adoption/0.40.md).
