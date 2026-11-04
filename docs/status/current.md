# Current handoff

## Timers 0.40.4 delivered — 2026-11-04

Timers **0.40.4** is pushed and published from this checkout on `main` to
`origin/main` (`dragginzgame/ic-scheduling`). Source preparation:
`e0f642227343b507705f382df9287050f64201aa`. Release commit:
`30b5b28658612ce00fe793eb0379ba9bb6b5b971`. Annotated `timers-v0.40.4` tag
object: `49142f5e678cca5a4ae7292cd5a500ce7280c10c`. Both release and separate
`make RELEASE_GROUP=timers publish` completed successfully; the journal is complete.
No upload retry or sibling release occurred. Do not republish this version.

The official registry archive matches the local uploaded archive and clean VCS
source at the release commit. SHA-256:
`60f214dcbad01105fcae2f08d5bafd4abd8274d7031dc467e605de6b24da1591`.
It retains Rust 1.99.1 and only Metrics, Platform Runtime and Thiserror as normal
dependencies. Jobs remains published at **0.34.3**; its shared tooling/dependency
notes remain pending under 0.34.4. There is no pending Timers draft for this batch.

## Qualification and selected inputs

The complete 37-stage workspace release plan passed on Linux/Rust 1.99.1 and
PocketIC 16.1.0: all 33 common CI obligations, CLI/server setup, all **35 installed
runtime subjects** and the policy cohort. This includes 178 Timers unit tests,
doctests, Jobs and combined-consumer tests, native/Wasm checks and both normalized
archives. Publication repeated complete Clippy and compiled the 0.40.4 archive.
Installed runtime and cohort stages took 44s/47s; source-bound tooling fixtures
648s and validation-plan fixtures 115s. No fresh macOS or installed Jobs recovery
qualification is claimed. Hosted CI remains asynchronous.

Gate source is the preparation commit above. Its pre-bump lock SHA-256 is
`255ca2482d0f5cfdd1030f63a32bcccaf30158d7dc0c2f10710bc8c5ce5ba029`;
the admitted metadata transaction changes only the selected group's version rows.
Testkit **0.85.1**, native Platform **0.22.0**, Types **0.18.1**, Runtime/macros/
exact guard **0.19.1**, Build/Host **0.74.3**, Wire/Derive **0.16.2**, Metrics
**0.29.1**, Logging **0.24.6** and Storage **0.66.2** are selected. New supplier
checksums match official non-yanked registry records. Testkit's compatible root
minimum is 0.85.0; managed startup supplies the explicit server URL.
Shared Tooling remains **0.29.4**, revision
`628ccc75d813bfd9bae777bfbaef55838d25b1b2`; all 146 payload files verify.

## Delivered cleanup and validation policy

Runtime capabilities, registration/reconciliation and dispatch have private
modules; shared authority checks and the registry static remain in the parent.
The parent is 305 lines, down from 1,364; largest new module: 459 lines.
All 42 public function signatures remain unchanged. All 35 installed cases are
grouped by behavior, with all 17 required qualification anchors updated. Fresh
setup is shared at 31 sites; overflow keeps its pre-install time advance.
Earlier focused evidence stays in `target/runtime-cleanup/`, including the
socket-blocked attempt and separate successful two-subject host qualification.
These structural moves do not establish an execution-cost or Wasm-size gain.

Release plans now name every obligation individually. Proven Jobs-only scope
selects 19 obligations and marks 18 unchanged Timers/tooling obligations not
applicable. Timers, shared, dirty and uncertain inputs keep all 37; full repository
CI keeps all 33 common obligations. No proof reuse or omission through edited
receipts is permitted. The current Jobs plan still broadens because changes since
Jobs 0.34.3 include Timers and shared inputs. See [the mapping](../release-validation-groups.md).

## Custody and boundaries

`.git/release-state/deployment-timers0404.OHlQFZ45/` retains source, graph and
registry/archive verification, release/publication logs, installed diagnostics
and completion evidence. Canonical `timers-0.40.4.plan` and its version-2 receipts
retain exact preparation/gate/source identities; never convert historical proofs.
Also preserve `deployment-timers0403.XBMtMHVE/`, its completed journal/receipts,
checksum-matched archive and installed diagnostics. Regenerable scratch stays in
`target/`; retained-state admission passes without cleanup.

Eight members retain explicit versions across Jobs (three) and Timers (five).
Root manifests/lock own the cohort. Jobs stays Serde-only, consumer-owned and
validated through `Job::restore`; uncertain effects need explicit disposition.
Timers retains its sole private provider, FFI confinement and exact Runtime guard.
Canic owns reporting and application integration; copied Metrics observations
create no endpoint. Storage owns admitted Cells; retired Memory images are refused
without writes. Same-release upgrades do not prove old-Wasm migration or rollback.
See the [consumer checklist](../timers/adoption/0.40.md).

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
