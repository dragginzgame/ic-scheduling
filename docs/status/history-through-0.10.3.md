# Frozen handoff through 0.10.3

Captured on 2026-10-14 before the deadline/reply cleanup, from delivered Jobs
0.10.3 and the then-pending 0.10.4 extraction working tree. Source identities,
qualification receipts and issue observations below retain their original scope.
Each "latest" label belongs to that historical observation. Follow
[the current handoff](current.md) and owning GitHub issues for current acceptance.

## Source and pending work

Delivered Jobs **0.10.3**: `37937543a58cd1fd54eea8dbd9bb0382f658a194`.
Pending compatible **0.10.4** extracts owned rows directly from consumed,
validated snapshots. Receipt lookup shares one logical-key index; duplicate
acknowledgements return the equal incoming envelope without a payload clone.
Temporary extraction order never reaches stable storage. Both fault endpoints
reconcile the returned dispatch deadline before their unchanged delivery/await/
trap boundaries; the reload-based timer helper is removed.
Delivered 0.10.3 centralizes validated snapshot lookup and pairs management/result
replies with the fresh committed deadline. Candid endpoint values stay unchanged;
management and asynchronous completion reconcile that deadline without reloading
storage. No deadline survives an await: completion reads the current queue before
its own transition/write. Native maintenance filters expiry once per callback,
retaining a separate occurrence commit and partial-progress recovery.
Delivered 0.10.3 also carries the deadline admitted by queue opening directly into
lifecycle reconciliation; no deadline cache or second restore pass is introduced.
Incoming and retained delivery envelopes share bounded field admission while
keeping authorization, uniqueness, exact comparison and error identity local.
The package README/rustdoc owns API examples and timer guidance; the root README
links to that guide and authoritative selections instead of stale version prose.
Delivered **0.10.3** shares delivery-envelope construction without extra
payload clones and filters/truncates the existing native pagination vector.
Idle dispatch now validates the complete snapshot without serialization or stable
writes when no work starts. Manual dispatch reconciles the returned deadline
before spawning delivery, preserving authorization and trap-before-delivery order.
Receipt validation keeps active-attempt/task/ID admission and exact envelope
comparison before completion or reconciliation. Stored records are unchanged.
Delivered 0.10.2 includes toolchain-action adoption for
[Jobs #46](https://github.com/dragginzgame/ic-jobs/issues/46). Both CI/recovery
callers retain their explicit versions and targets; the action now belongs to
the recorded canonical snapshot and rejects malformed selections before setup.
Delivered 0.10.2 also returns the successfully committed dispatch deadline to the
watchdog instead of reloading the whole queue. Snapshot writes validate collection
invariants directly without decoding/restoring their already validated Jobs.
Stored reads still restore every record, preserving restore/entry refusal order;
encoded capacity, payload alignment and commit-before-delivery remain unchanged.

Delivered 0.10.1 delegates
fixture exit injection and retained-evidence assertions to the canonical Shared
helper; its actual fixture roster and archive/command assertions remain local.
Release metadata preparation, rollback and selected-source checks now delegate
to Shared Tooling; Jobs retains direct-delivery admission and exact tool selection.
Recovery fixtures use Testkit's nanosecond clock extension and retain its 0.40
instance owner rather than a bare PocketIC client.
Delivered `Scheduler::start_next` returns
`(index, attempt, running_record)`: update tuple destructuring. The index belongs
only to the supplied batch while its order is unchanged; persist job identity
and the running record as before. Both maintained consumers use that selected
index directly instead of searching again. Stored records and effect-disposition
rules are unchanged; no storage reset is required.

The preceding native fixture cleanup remains included: notification fixtures
keep one validated job-state collection and borrow payloads for commit;
maintenance loads once but commits each occurrence separately. General Memory
contract tests stay with published IC Memory; Jobs retains stable-cell reopen,
blocked-effect and partial-commit recovery coverage.

Local lockfile identities and expected versions share one admission parse.
Disposable release fixtures reject wrong local-row versions at that boundary.
Optimized recovery producers are removed only after successful exact capture;
failed capture retains producer bytes and the frozen original. Qualified bytes,
hashes and optimizer/server logs remain retained.

The incoming Testkit 0.41 manifest/lock update is preserved byte-for-byte.
Focused Rust 1.88 consumer, native/Wasm and harness compilation now qualify the
incoming graph on Linux. This does not qualify Testkit CLI/server setup or live
recovery on that selection.
Own package versions, staging, Git refs, release and publication remain
maintainer-owned. No release or upload ran.

## Shared adoption

Shared Tooling **0.12.7** is pinned to
`9364ffe3f770a6d5d94809f0bf82c33202054bf8`, verified against upstream main and
canonically exported from a clean exact-source clone. The snapshot has 101 files
after adding the canonical Rust-toolchain action entrypoint;
existing roots and tool versions/digests are preserved. Previously prepared tool
selections remain admitted; snapshot and pin checks pass. The public repository
description remains accurate.

AGENTS links to the shared validation cadence: focused checks qualify ordinary
handoff; full suites retain explicit local selection or configured CI. Its
explicit sibling read-only restriction remains in force. The toolchain
action is now snapshot-owned from the delivered revision; producer acceptance
remains separate in [Shared #177](https://github.com/dragginzgame/shared-tooling/issues/177).

## Focused proof

Latest extraction evidence: `target/qualification/receipt-extraction-cleanup/`.
All 22 stable-consumer tests pass on Rust 1.88. Multi-row receipt queries,
duplicate acknowledgements, logical-key evidence lookup and mismatch/authority
refusals preserve stored bytes/order across reopen; backing-memory instrumentation
proves zero writes for those read/idempotent paths. Existing full restoration,
job lookup, corruption and completion/refusal coverage passes. Release Wasm,
capture tests, recovery harness compilation, strict native/Wasm Clippy, separate
core/timers native/Wasm floors, formatting, pins, snapshot and local links pass.
The incoming Cargo selections remain unchanged. No full CI or live recovery ran.

Latest mutation evidence: `target/qualification/mutation-deadline-cleanup/`.
All 21 stable-consumer tests pass on Rust 1.88, including fresh reply deadlines
through create/pause/reschedule/resume/cancel, uncertain completion, later queue
mutation and exact reconciliation followed by reopen. Existing corruption/lookup/
clock/refusal and unchanged-byte cases pass. All 19 native notification/maintenance
cases pass separately with core/timers, including no-work, Skip/CatchUp and partial
commit recovery. Release Wasm, capture tests, harness compilation, strict native/
Wasm Clippy and separate native/Wasm Rust floors pass on the selected Memory 0.38 /
Testkit 0.42 / Timers 0.22.4 graph. Incoming Cargo files are preserved. No full CI,
live PocketIC qualification, release or publication ran.

Latest opening/documentation evidence: `target/qualification/open-docs-cleanup/`.
All 20 stable-consumer tests pass on Rust 1.88, including returned initial/reopened
pending deadlines, unchanged retained bytes and rejection of corrupt inactive
records before lifecycle recovery. Release Wasm, capture tests, harness compilation
and strict native/Wasm Clippy pass. Core/timers Rust 1.88 native/Wasm checks,
warnings-denied rustdoc and doctests for both feature selections pass. Formatting,
snapshot/pin admission and local documentation links pass. The incoming Cargo
files remain unchanged. No live recovery, full CI, release or upload ran.

Latest idle/completion evidence: `target/qualification/idle-completion-cleanup/`.
All 19 stable-consumer tests pass on Rust 1.88. Instrumented backing memory proves
zero writes for empty/future/paused/running/uncertain/completed queues, unchanged
bytes and corruption refusal before idle return; due work still commits metadata
and its local effect. Release Wasm, capture tests, harness compilation, strict
native/Wasm Clippy and separate core/timers native/Wasm floor checks pass. Manual
reconciliation's real IC rollback boundary remains a live qualification obligation.

The completion guard is an isolated additive draft against published Shared 0.13.0
`998065bd2dc8350fb6811c3c3486627cbb48724e`, tracked in
[Shared #192](https://github.com/dragginzgame/shared-tooling/issues/192).
It separates status admission from cleanup; fixture_finish keeps its existing
policy while the proposed live wrapper retains all success/failure evidence.
Completion and existing retention fixtures, plus the proposed Jobs wrapper/tooling
caller, pass on Bash 5 and genuine Bash 3.2, with optimizer/server effects
substituted. Source-following ShellCheck and guide links pass. Complete producer
and caller patches apply-check and remain with the evidence. Active shared payload,
live wrapper and fixture companion roster stay unchanged pending reviewed export;
no sibling edits, producer refs, 0.13 snapshot adoption or live recovery ran.

Latest evidence: `target/qualification/delivery-page-cleanup/`. All 18 stable
consumer tests and 19 native consumer tests with core/timers separately pass on
Rust 1.88. Release Wasm, capture tests, recovery harness compilation, strict
native/Wasm Clippy and core/timers native/Wasm floor checks pass. The incoming
lockfile, including Timers 0.22.4, is preserved; its previously absent archives
were prepared with `cargo fetch --locked` before repeating offline checks.

Host fixture allocation remains pending published availability in
[Host #98](https://github.com/dragginzgame/ic-host-tooling/issues/98).
Its allocator became committed locally during this task, but public main still
selects 0.18.0 and crates.io does not expose 0.18.1. An isolated clone of the
committed candidate qualifies the proposed Jobs test caller: both capture tests
pass on Rust 1.88, retaining manual success cleanup and failure custody. The
registry-only caller patch applies cleanly and requires an explicit 0.18.1 floor
with `test-support` enabled only through the harness development dependency.
The active manifests/capture allocator remain unchanged until reviewed published
availability; no sibling mutation, producer delivery, live gate or release ran.

Latest queue-flow evidence: `target/qualification/queue-flow-cleanup/`. All 18
stable-consumer tests pass, including typed write refusal/unchanged-byte cases,
capacity error precedence, independently frozen snapshots and corruption/effect
blocking. Release Wasm, capture tests, harness compilation and strict native/Wasm
Clippy pass. Rust 1.88 core/timers native and Wasm checks pass separately. No live
recovery or full CI gate ran.

The third cleanup is an isolated Shared Tooling draft for
[Shared #189](https://github.com/dragginzgame/shared-tooling/issues/189), based on
published 0.12.7 `9364ffe3f770a6d5d94809f0bf82c33202054bf8`.
`fixture_create TEMPLATE` returns a physical directory and preserves failure
custody. Allocation and existing retention fixtures pass Bash 5 and genuine Bash
3.2; the four proposed Jobs callers pass disposable formatting, evidence,
consumer-tooling and release checks, with release fixtures passing both shells.
Producer/caller patches are retained with the evidence and apply-check cleanly.
Active snapshot payloads and fixture callers remain unchanged until reviewed
producer delivery/export; sibling working trees and producer refs are untouched.
Native macOS qualification remains with the delivered owner/consumer gates.

Latest action evidence: `target/qualification/toolchain-owner/`. The canonical
offline fixture passes exact arguments for both caller variants, first/second
installation failure propagation and malformed input refusal before effects.
The adopted action matches canonical bytes; Actionlint validates both real
workflows. Snapshot, pins, documentation links and diff checks pass. Native
hosted acceptance of the adopting source remains pending; no real-download or
live recovery check ran.

Latest cleanup evidence: `target/qualification/metadata-owner/`. Disposable
release/publication fixtures pass on Bash 5 and genuine Bash 3.2, including
patch/minor/major preparation, rollback, retained selected-source failures and
resume checks. Both Rust 1.88 native/Wasm feature qualifications pass separately;
19 native consumer scenarios and all 17 stable-consumer tests pass against the
incoming graph. Release Wasm, two capture tests, recovery harness compilation,
strict native/Wasm Clippy, formatting, ShellCheck, snapshot, pin and documentation
checks pass. The initial harness compilation failure exposed Testkit 0.40's
removed builder extension; it is repaired by retaining the published instance
owner. No live recovery ran. Incoming Cargo.toml/Cargo.lock remain byte-identical
to task entry.

The capture-finalization proposal is implemented only as an isolated IC Build
0.3.0 draft, tracked in
[IC Build #26](https://github.com/dragginzgame/ic-build/issues/26).
`CapturedArtifact::retain_to` preserves exact bytes and reports original or
destination custody on rename/cleanup failure. Nine focused staging tests,
strict Clippy and Rust 1.88 rustdoc pass; the proposed Jobs caller passes its two
capture tests in a disposable package. Reviewable upstream and caller patches
are retained in `target/qualification/metadata-owner/`. Active Jobs capture
finalization stays local until the owner API is reviewed and published. Sibling
working trees were untouched; no upstream commit, PR or publication ran.

Latest tooling evidence: `target/qualification/fixture-probes/`. The expanded
entrypoint closure and pins pass. Disposable release/publication fixtures pass
on Bash 5 and genuine Bash 3.2, including function nounset, failed cleanup,
premature exits and contradictory command assertions. Archive round trips stay
consumer-owned. ShellCheck and documentation checks pass. That earlier fixture-probe follow-up
did not change product Rust, Cargo selections or release versions.

Evidence: `target/qualification/shared-0125/` and
`target/qualification/scheduler-index/`. Rust 1.88 core and timers tests pass
separately, including 19 native consumer scenarios, frozen records and transition
histories. Scheduler tests verify selected indices in deadline/tie order and
unchanged refusal behavior. All 17 stable-consumer tests, release Wasm, capture
tests, harness compilation, native/Wasm minimum checks, strict Clippy and
rustdoc pass against the selected graph.

Formatting-hook/evidence, consumer-tooling and disposable release/publication
fixtures pass. Consumer/release fixtures also pass under genuine Bash 3.2,
including child shells. Success retires duplicate optimized producers; injected
capture failure retains them and refuses optimized server dispatch. Wrong local
lock versions refuse without changing the selected graph. Tool, registry,
optimizer/server and complete-release gates are substituted in those fixtures.
ShellCheck, Actionlint, formatting, pins, snapshot integrity and documentation
links pass. Incoming Cargo.toml/Cargo.lock remain byte-identical to task entry.

## Qualification boundaries

Delivered 0.10.3's [CI 38590045221](https://github.com/dragginzgame/ic-jobs/actions/runs/38590045221)
is queued at inspection. Issues #41–#44 remain open pending a complete native
matrix; this dirty 0.10.4 cleanup has no hosted qualification. Jobs has no open PR; IC Build #23 was
closed after the preceding inspection. Unintegrated Shared contributions retain
their owner review/check requirements.

Full `make ci` and live PocketIC recovery were not selected. These local Linux
checks do not establish the required macOS Intel/Apple Silicon matrix or live IC
recovery for this source/graph. The old delivered 0.9.3 Host 0.17.7 failures are
historical evidence, not a qualification of the currently selected graph.

Current issue acceptance remains with
[#41](https://github.com/dragginzgame/ic-jobs/issues/41),
[#42](https://github.com/dragginzgame/ic-jobs/issues/42),
[#43](https://github.com/dragginzgame/ic-jobs/issues/43), and
[#44](https://github.com/dragginzgame/ic-jobs/issues/44).
Last inspected live recovery remains Jobs 0.8.1 source
`f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
[run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820):
84 passes across seven scenarios, four Wasm variants and three hosts.
It does not qualify the pending cut.

See [frozen history](history-through-0.9.1.md), [architecture](../../README.md),
[safety](../../SAFETY.md), [consumer qualification](https://github.com/dragginzgame/ic-scheduling/blob/bb67e6f23ec50a95cf1b58f675cd5a8d8c60f4ff/apps/ic-jobs-test-consumer/README.md),
and [delivery procedure](../releases/README.md).
