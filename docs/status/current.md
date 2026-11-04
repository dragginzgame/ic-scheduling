# Current handoff

## Runtime and installed-test cleanup pending — 2026-11-04

Runtime capabilities, registration/reconciliation and callback dispatch now have
private modules; the parent retains shared authority checks and the registry.
The 1,364-line runtime parent is 305 lines; the largest new module is 459 lines.
All 42 public function signatures are preserved. Installed subjects are grouped
under ingress/lifecycle/ordinary/scheduling/watchdog, preserving all 35 cases and
updating all 17 required qualification anchors. Fresh setup is shared at 31 sites;
the overflow case keeps its explicit time advance before installation.

89 native runtime tests, three discovery fixtures, eight compile-fail doctests,
strict owning lint, Rustdoc and probe Wasm checks pass. Both explicitly selected
live subjects (ordinary await/interleaving and Watchdog trap/upgrade) pass under
PocketIC 16.1.0/Rust 1.99.1. Evidence: `target/runtime-cleanup/` and
`target/timer-qualification-1114695-0/runtime.json`. The first live attempt was
socket-blocked before execution; its separate diagnostics remain retained.
Current checks resolve Testkit 0.85.1/native Platform 0.22.0; earlier 0.85.0
proofs below retain their actual graph. No full installed gate, release or push.

## Granular release CI and Testkit refresh pending — 2026-11-04

The shared feedback is addressed locally: release plans name all 37 obligations
individually instead of nesting `ci`. Proven Jobs-only scope requires 19 and marks
18 standalone Timers/unchanged tooling obligations not applicable. Full CI keeps
all 33 common obligations; Timers/shared/unknown/dirty scope keeps all 37. Mixed
lint/docs/archive commands reuse explicit Jobs/Timers leaf recipes. Baselines stay
bound to the admitted source version, and sealing rechecks the exact product
mapping. Required tests cannot be omitted through an edited but consistent plan.

Mapping/rejection/durable-evidence fixtures, actual Make caller/drift fixtures,
all six real lint/docs/archive targets, ShellCheck and snapshot integrity passed.
Product commands in the mapping fixtures are substituted; no new live IC or
macOS qualification is claimed. Both groups' real metadata-editor/projection and
recovery fixtures also passed, with product commands/uploads substituted. Evidence
is in `target/group-ci/`. No full product CI or release was run for this ordinary
change. Jobs 0.34.4 and Timers 0.40.4 have pending tooling/dependency notes;
package versions are unchanged and work remains uncommitted.

The later owner-requested supplier refresh selects **Types 0.18.1** and
**Testkit 0.85.0**. Official checksums match; neither version is yanked. Testkit
0.85.1 was unpublished at observation. Its explicit-server-URL change needs no
Rust caller edits: all three harness/demo constructors already use managed
`PocketIcStartupConfig::build_instance`. Keep compatible Types minimum 0.18.0;
Testkit's root requirement is now 0.85.0. Runtime/macros/guard remain 0.19.1,
native Platform facade 0.21 and server 16.1.0. No unrelated supplier was refreshed.

On this graph, two native harness/demo cases and four API/Wire cases passed;
explicit live Jobs/demo cases stayed ignored. Jobs/Timers probe Wasm checks and
native/Wasm dependency policies passed. Testkit CLI setup/server admission and
one explicitly selected installed Timers trap/upgrade recovery case passed on
Linux/Rust 1.99.1. Evidence: `target/adopt-testkit085/` and
`target/timer-qualification-1035241-0/runtime.json`. This is one live subject, not
a fresh full watchdog/cohort or installed Jobs recovery pass. Earlier CI-fixture
evidence above retains its earlier graph; the final 37-check gate remains pending.

The current real Jobs plan correctly selects all 37: outstanding Timers/supplier
and shared changes since Jobs 0.34.3 prevent Jobs-only scope. The first delivery of
this policy also requires the complete gate; no baseline is advanced for speed.
See [the reviewed mapping](../release-validation-groups.md).

## Timers 0.40.3 delivered — 2026-11-04

Timers **0.40.3** is pushed and published from this normal checkout on `main` to
`origin/main` (`dragginzgame/ic-scheduling`). Source preparation is
`b556199feae4d1ca2235fd20b6fe718a19136a2a`; release commit is
`670cd0d00cf496e9252fb77a3048ffd151d89eb8`, annotated tag `timers-v0.40.3`
(object `d1e08087df6390dc74ab98f51d4c3269facf00de`). The canonical journal is
complete. Separate `make RELEASE_GROUP=timers publish` succeeded from the clean
matching tag after the owner explicitly authorized crates.io publication.
Automatic approval initially rejected the upload; that command did not execute.
The later authorized upload completed. Do not repeat it.

The official registry checksum matches the retained archive and its clean VCS
source at that release commit: `decfcea289e39056a84a13dc228040553f4be61a6775026db03eeaa4fddc660a`.
The archive retains Rust 1.99.1 and only Metrics, Platform Runtime and Thiserror
as normal dependencies. Jobs remains **0.34.3**, without a new publication.
There is no pending draft for this delivered batch and no sibling was released.

## Selected graph and qualification

This release includes Runtime/macros/exact guard **0.19.1**, IC Build **0.74.3**
and Wire/Derive **0.16.2**. All six official registry checksums match the selected
lock, with no yanks. Keep the compatible Runtime 0.19.0 manifest minimum.
Types 0.18.0, native Platform facade 0.21.0, Endpoint 0.6.1, Testkit 0.84.1,
Logging 0.24.6, Metrics 0.29.1, Storage 0.66.2 and Host 0.74.3 remain selected.
Shared Tooling stays **0.29.4** at `628ccc75d813bfd9bae777bfbaef55838d25b1b2`.
The supplier facade 0.22 draft was not adopted.

The fresh five-stage gate passed on Linux with Rust **1.99.1** and PocketIC
**16.1.0**: all 30 common CI obligations (655s), CLI/server admission (9s/10s),
all **35 installed watchdog subjects** (49s) and the policy cohort (46s).
Common CI includes 178 Timers unit tests, eight doctests, Jobs/combined consumers,
native/Wasm checks and both normalized archives. Publication passed final Clippy
and normalized 0.40.3 archive compilation. These qualify the selected graph and
source; no new installed Jobs recovery or macOS pass is claimed. Hosted
Linux/macOS CI remains asynchronous. Same-source upgrade tests do not establish
deployed old-Wasm migration, rollback or destructive data retirement.

## Delivered cleanup and API boundaries

0.40.3 shares ordinary stop/cancel, Watchdog control, callback adaptation and
result normalization. Its final slice was 10,959→10,947 non-test LOC with 371
test functions; the rejected capability macro added indirection. Those are
historical measurements, not totals for the pending module moves above.
Metrics recording stays in Timers; copied type re-exports create no endpoint.
Canic owns reporting. Jobs remains Serde-only with validated `Job::restore` and
explicit uncertain-effect disposition. Timers retains its sole private provider,
unsafe FFI confinement and exact Runtime guard. Storage retains admitted Cells
and blocked attempts/receipts; retired Memory images are refused without writes.
See the [consumer checklist](../timers/adoption/0.40.md).

## Durable custody and release rules

`.git/release-state/deployment-timers0403.XBMtMHVE/` retains incoming source and
old/selected locks, supplier registry observations, cleanup evidence, release and
publication logs/statuses, the checksum-matched published archive, VCS inspection
and installed watchdog/cohort diagnostics. Completed `timers-0.40.3.*` journal and
version-2 receipts retain their exact source. Default retained-state admission
passes without cleanup. Regenerable development scratch remains under `target/`.

Eight members retain explicit matching versions in independent Jobs (three) and
Timers (five) groups; no root workspace version. Root manifests and lock own the
supplier cohort. `ic-jobs-canister-tests::harness` keeps recovery support explicit
and default artifact capture lightweight. `ci/release-validation.json` remains the
sole obligation roster; the pending mapping above replaces the old five-stage
aggregation without reusing proofs or converting historical receipts. Earlier
receipts retain their original source/adapter. See
[the group mapping](../release-validation-groups.md).

## Earlier deliveries and protected history

Timers **0.40.2** (`dc0c2cd`), **0.40.1** (`f03d430`) and Jobs **0.34.3** (`0eec50a`)
remain pushed/published. The [previous delivered handoff](https://github.com/dragginzgame/ic-scheduling/blob/af1ff7db20125e29edc82ad9323c317b6cd1de6e/docs/status/current.md)
retains exact 0.40.2 qualification, historical footprint measurements and earlier
delivery identities. Preserve its custody `deployment-timers0402.G6arr0ab/` and
`deployment-consumer-support.lHbrHngp/` under `.git/release-state/`.

Also preserve `deployment-timers0400.IkyEFM1l/`, `deployment-timers0391.k7IkTIwf/`,
`deployment-current.0NCjlIWS/`, `deployment-timers0381.jnjcs5qg/`,
`deployment-upstreams.UjAXfFHz/`, `deployment-msrv-87.uUHPASl6/` and
`deployment-wire-tooling.w6Z6gClk/`, including journals, receipts and archives.
Preserve version-1/interrupted proofs and stashes with their original source;
never pop delivered versions or replay completed uploads. The
[earlier ten simplifications](../redundancy-measurements-2026-11-03.md) retain their
frozen Runtime 0.18 graph; prior 44-case installed Jobs recovery retains `af87eab`.
