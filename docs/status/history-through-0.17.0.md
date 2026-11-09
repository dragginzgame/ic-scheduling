# Historical handoff through 0.17.0

Archived verbatim from the handoff at delivered Jobs 0.17.0, before the ownership
cleanup on 2026-10-18. Pending versions, proposed helper availability and tracker
states below describe their original review time. They are historical evidence;
use [the current handoff](current.md) for the maintained boundary and validation.

# Current handoff

## Source and pending work

Delivered Jobs **0.16.2**: `41d9e31ff92de1703976d642b28d20ff08bae8aa`.
Pending **0.17.0** hard-cuts management inspection replies to
`Status::Pending { due_ns }` and removes the separate `View::due_ns` field.
Rebuild the unpublished qualification canister and harness together. Stored
JobRecords and delivery receipts keep their formats; no storage reset or
conversion is required for this response-only cut. No compatibility reader or
fallback is added. The public status/deadline pairing is structural; projection
derives it directly from validated JobState.
Retry configuration wraps one private field representation; construction and
decoding keep the same numeric constraints and canonical named serialization.
The capture CLI prints one success receipt after either retention path succeeds,
preserving optimizer status and failed-producer/log custody.
Native notification enqueue/cancel now share commit-before-reply projection,
keeping stored ordering and the complete queue's deadline before extracting the
response entry. Destination identity filtering is shared while evidence queries
still scan for exact payloads and delivery still checks the first retained match.
The consumer gate selects both harness targets in one Cargo invocation; live
recovery scenarios retain their ignored default.

The earlier compatible cleanup reuses the already-derived active-attempt timing evidence
through restoration. Queue summaries use the existing reply/deadline envelope
and remain read-only; the canister's public summary fields are unchanged.
Delivery projection uses the admitted queue identity bound, retaining active
attempt, payload and exact-envelope checks. That cleanup preserves the core API
and stored/wire formats; the management response cut above is separate.
Error precedence and uncertain-effect blocking remain unchanged.
The incoming Testkit/Candid 0.52 manifest and lock selection is preserved;
its adoption is separate from these three local cleanups.

Delivered **0.16.2** shares watchdog reconciliation before returning committed
consumer replies. Management still authorizes first, delivery completion
keeps its existing authority path, and reconciliation failure traps in the same
message segment. The callback's result-driven watchdog path remains separate.
Consumer view projection is infallible after stored identity admission; malformed
IDs still refuse before inspection or paging. Native composition assertions reuse
one decoded snapshot per unchanged observation phase, retaining fresh reads after
mutations and independent physical Cell reopening. This compatible cleanup leaves
core source, public/wire/storage contracts and own package versions unchanged.
The incoming Timers 0.27.1/Metrics 0.13.3 lock selection is preserved; Testkit's
independent Metrics 0.12 selection remains alongside it.
The selected Memory 0.48 graph removes the infallible fixture-cell writer.
It uses bounded `try_set` for initialization, normal commits and
the deliberate corruption endpoint, propagating write refusals before success.
All native fixture writers explicitly require successful bounded writes.
The existing 24 KiB capacity check still precedes collection admission; record
restoration, refused-byte preservation and unknown-effect blocking remain.
Memory 0.48 and Build 0.15 manifest/lock selections are preserved unchanged.

The release incorporates the existing publication-owner contribution from
[PR #67](https://github.com/dragginzgame/ic-jobs/pull/67). Jobs now owns the complete
`ic-jobs` publication policy under `scripts/release/`; Shared retains canonical
source, version, tag, registry and Make observations. The publisher uses the local
policy without changing clean-source, pushed-tag, registry-absence or uncertainty
admission. Its local fixture is selected by `make test-release-tooling`, including
exit/cleanup and explicit-assertion failure retention. Snapshot bytes stay intact;
retirement of the unused Shared policy entry needs a reviewed export.
The older contribution branch is closed as incorporated, without a merge or
branch deletion. The cleanup is compatible with the current Jobs contract.
The same 0.16.1 batch shares Pending/Paused admission between cancellation and
rescheduling, preserving clock-first errors and rescheduling's attempted-work
refusal. Native fixture snapshot reads/writes share one byte ceiling, and
notification load/enqueue share one capacity. Page, receipt and rejection limits
remain independent. The release adapter keeps representative admission refusals,
exact Cargo dispatch, upload-status propagation and reconciliation after upload
failure; the local publication fixture owns the broader policy matrix.
The release also selects Build 0.14 and direct Host FS/Process 0.26. Earlier
focused capture/optimizer checks retain their recorded source scope; current
delivered acceptance uses the native CI receipt below.

Delivered **0.16.0** removes the duplicate Running timestamp: `JobState::Running`
is a unit variant and its start time is `JobRecord::updated_ns`. Entering
Uncertain copies the original dispatch time before replacing the transition
time; subsequent unresolved reconciliation preserves that start. Restoration,
retry history admission, error precedence and effect blocking remain.
The qualification consumer uses one `DeliveryPayload` for its retained tasks
and delivery/receipt envelopes. Payload bounds still apply at each storage/wire
boundary; exact-envelope comparison and unknown-effect refusal remain.
The maintainer selected these hard cuts under the earlier no-preservation
instruction. Retire affected prior installations under their own authority and
create fresh qualification state; this source change performs no reset,
conversion or deployment. Timestamp-bearing Running and flat receipt shapes
refuse. Current-format unresolved attempts remain blocked until disposition.
Core callers must update Running constructors/patterns; consumer callers use
`Task::Deliver(payload)` and `delivery.payload`, rebuilding both peers and harness.
The hard cut was delivered in 0.16.0; 0.16.1 preserves that contract.

The candidate carries the earlier published Build 0.13 capture-helper adoption,
receipt lookup and notification paging/dispatch cleanup. Native management replies
now use explicit fields, retaining authorization, commit and deadline ordering.
Incoming Timers 0.27, other dependency selections and capture edits are preserved.

Host's status helper and Testkit's panic-safe reporter are qualified as source
candidates only. Their current selected registry packages do not export those
helpers; active Jobs callers keep their local implementations until publication.
The prepared callers and their source-bound checks are described below. Follow
[#52](https://github.com/dragginzgame/ic-jobs/issues/52),
[#63](https://github.com/dragginzgame/ic-jobs/issues/63) and
[#64](https://github.com/dragginzgame/ic-jobs/issues/64) at their GitHub owners.

Delivered **0.15.0** hard-cuts the latest result to
`JobRecord::last_outcome: Option<RecordedOutcome>`, pairing `outcome` and `at_ns`.
The separate timestamp field and mismatched-presence admission check are removed.
The same batch restores stored consumer entries directly with their payloads,
removing the temporary job vector and positional reassembly.
Validated `Job` now serializes transparently through the canonical `JobRecord`
writer, without direct deserialization. Native notifications persist typed entries
directly and no longer need a borrowed-payload projection type parameter.
Maintenance keeps one validated job with its cache entries/counter rather than a
validated job plus a manually synchronized raw record. Reads still pass through
`Job::restore`; bounds admission and per-occurrence commit ordering remain.
Completed/Failed restoration shares its common timing/attempt checks while
retaining Once-only success, permanent failure and exhausted-retry admission.
Pending/Paused and Cancelled restoration share retained retry evidence admission;
only Pending/Paused must match the retained retry's deadline. Cancellation keeps
its independent management timestamp, and Running retry checks remain separate.
Notification management responses consume the record and payload together at one
projection boundary, after commit and full-queue deadline projection where needed.
The native notification destination owns recorded-result lookup, including
payload matching and success-before-rejection evidence precedence. The sender
still classifies those facts and explicitly resolves its retained job. Missing
evidence remains blocked. The persistence example and shared roundtrip helper
serialize Jobs directly, while reads still decode JobRecords and use `Job::restore`.
Valid-state consumer fixtures share public transition setup with explicit target
states and original timestamps; malformed-record fixtures remain independent.
Each consumer reuses its existing identity lookup for extraction/duplicate checks.
Owned extraction changes only the consumed snapshot; persisted ordering is
unchanged. Page and single-job inspection project the complete owned entry,
keeping payload and validated job together through the response boundary.
These cleanups preserve the current 0.15 JSON and transition behavior.
The delivered 0.14.1 adopted Memory 0.45's bounded fixture-cell API in the
unpublished consumer and the standalone Candid decoder for delivery replies.
Memory owns fixture allocation/bootstrap and fresh/retained Cell admission;
Jobs keeps its 24 KiB limit, stable key/configuration, snapshot codec, collection
checks and every `Job::restore` boundary. One payload header is checked before
retained bytes are loaded. Refused cell bytes are preserved; allocation metadata
may change before a later cell refusal. No reset, conversion or old reader is added.

The stored and validated queue states share `Snapshot<J>` rather than two metadata
structs. Consuming admission maps each record and payload into one validated entry;
typed writes serialize Jobs directly and borrow the same header/entry/receipt
checks without building an intermediate raw snapshot or identity-mapped output.
Every decoded record still enters `Job::restore` before that entry's metadata
checks, after header admission and before receipt checks. Encoded capacity still
precedes collection errors on writes. The entry-admission cleanup preserves
error precedence, payload ownership and transition behavior; the stored-record
cut is separate.

The maintainer explicitly selected the hard cut and retirement of earlier
installations without preserving their jobs. Earlier JSON is refused, including
the old separate timestamp field; there is no dual reader, converter or fallback.
Consumers must create fresh state under their own installation authority.
This implementation executes no storage reset or deployment. Refused cell bytes
are not replaced; Memory bootstrap may still update allocation bookkeeping.
Current-format Running/Uncertain records
still restore as blocked and require the same explicit effect disposition.
The former 0.7.5 positive record corpus is removed. A hand-authored current
uncertain record qualifies exact JSON and blocked reconstruction; frozen old
consumer JSON/physical images remain only as refusal evidence.

Delivery decoding uses an 8 KiB wire ceiling, 4 KiB header, 64 type entries and
finite decoding/skipping quotas, independently from the host management limits.
Unreadable replies stay distinct from an authoritative absent receipt and keep
uncertain effects blocked. The application still compares the exact envelope.
Jobs' actual CI fixture keeps one selected-lock drift case through its Make route;
the pinned Shared producer owns generic checkpoint/output and refusal matrices.
The 0.15 record API and JSON change; transition and error ordering remain unchanged
for current-format records. The decoder's work limits and uncertainty policy remain.
Shared Tooling is **0.23.0** at
`dabc59d944492bebcb43dd92ba58e462fb100597` (114 payloads), from canonical export
of delivered 0.23.0 plus the focused hook-fixture fix in
[Shared PR #281](https://github.com/dragginzgame/shared-tooling/pull/281).
The fix admits the production formatter's `--ignore-unknown` argument in its
prepared test substitute; production formatter behavior is unchanged.
Jobs explicitly selects IC setup/checks after common host/Rust tools and retains
the IC executable PATH for optimized qualification. ICP CLI is now 1.7.0.
Both local Make fixtures use declared companion closure preparation. The unused
shared publication policy and superseded fixture copier are removed by the
canonical export; Jobs' local publication policy remains selected.
Retained-failure assertions and locked release preflight keep their existing owners.
Release preparation now uses Shared's metadata renderer directly. Jobs no longer
requires cargo-edit, a separate release-tool catalog or local setup/check targets.
Common tool admission and metadata refusal/rollback boundaries remain in place.

The selected graph uses Memory 0.48.0, incoming Testkit/Candid 0.52.0, Timers
0.27.1 with Metrics 0.13.3, Testkit's separate Metrics 0.12.0, Build 0.15.0 and
incoming direct Host FS/Process 0.27.1. That selection already matches the Host
repository's latest delivered source; no dependency versions change in this batch.
Rust floor is 1.91.0; development is 1.99.0.
The cleanup preserves those incoming registry selections. Memory 0.46's narrowed
exports supply backing IO types directly, without a full substrate re-export;
its non-cloneable runtime memory lease stays with one cell owner. Only native
fixture tests name the already-selected `ic-stable-structures` package directly,
for raw Cell construction/corruption. That development dependency adds one lock
edge without changing selected package versions or published Jobs dependencies.
The standalone Candid decoder remains a direct dependency of the unpublished
consumer. Memory fixture support is enabled only there. Incoming lock updates
are preserved.
Own versions,
index, refs, release and publication remain maintainer-owned. Memory requests
follow its key-only API. Its incompatible outer ledger requires explicit owner
disposition before deployment of retained earlier installations; no reset or
migration is executed by this source update. Recovery replies now
have finite wire, header, type-table and decoding/skipping work limits. The
maximum-page and oversize-refusal proof runs natively without starting PocketIC.

`start_next` returns `(index, attempt)`. Commit the selected job record with its
immutable payload/index before external dispatch. Indexes refer to iterator order.
All restore paths retain
`Job::restore`; ordering, transition errors and uncertain-effect disposition are
unchanged. Consumers own storage and depend on Timers directly. Jobs has no
runtime Timers dependency or lifecycle exports. The capture tool owns Jobs'
optimizer recipe and uses IC Build's retained path; Jobs selects the live gate.

## Qualification

Shared 0.23 adoption is qualified locally on Linux against the incoming pending
0.17.0 tree on delivered Jobs 0.16.2; retained source inputs and the receipt are in
`target/qualification/shared-tooling-0230/`. Setup and offline tool admission,
snapshot integrity, pins, release-command adapters, consumer and formatting
evidence fixtures, real-Git release/publication fixtures and Jobs' hook-adoption
check pass. The upstream hook fixture initially failed with 0.23.0's stale
Prettier substitute; that failed fixture remains at `/tmp/git-hooks-test.DK06Ex`.
The focused correction passes at its upstream owner before canonical export.
Three capture tests and the native bounded-reply test pass on Rust 1.91 with the
existing Host 0.27.1 selection; seven live recovery scenarios remain ignored.
Runtime source, Cargo metadata and lock selection are unchanged by this adoption.
The new stable-memory governance was reviewed against the unpublished fixture
consumer boundary; this batch adds no production store or raw-memory path.
Native macOS, full local CI and live recovery were not selected. The contributor
PR is separate from Jobs' primary index/refs; no Jobs release or upload ran.

Native notification flow evidence is in
`target/qualification/notification-flow-0170/`, relative to the incoming pending
0.17.0 tree on delivered Jobs 0.16.2
`41d9e31ff92de1703976d642b28d20ff08bae8aa`, with its Testkit/Candid 0.52 metadata.
All 19 composition tests pass on Rust 1.91, retaining commit refusal, persisted
ordering, complete-queue deadlines, authorization and unresolved-effect blocking.
The payload-evidence case now also proves delivery refuses a conflicting first
payload even when reconciliation can find a later exact match; success evidence
still precedes rejection evidence. Focused composition Clippy passes.
The merged harness command passes three capture tests and one native bounded
reply test on Rust 1.91; seven live scenarios stay ignored. The actual consumer
tooling fixture passes with substituted compiler/optimizer/server effects and
the updated Makefile. Formatting, changed-document links and diff checks pass.
The application/core runtime source and incoming dependency edits are unchanged;
their earlier Wasm/core qualifications retain their own source scope. Own package
versions and primary index/refs are preserved. No full local CI, live PocketIC,
native macOS, release, publication, deployment or reset ran for this cleanup.

View/retry/capture cleanup evidence is in
`target/qualification/view-policy-capture-0170/`, relative to the incoming pending
0.16.3 tree on delivered Jobs 0.16.2
`41d9e31ff92de1703976d642b28d20ff08bae8aa`, including Testkit/Candid 0.52 metadata.
All 48 core tests and six doctests, 26 consumer tests, 19 composition tests, three
capture tests and the harness's native reply test pass on Rust 1.91. Retry JSON,
named Serde tokens and invalid-number admission remain qualified unchanged.
The old pending-page reply was encoded and decoded with the original API before
the cut; the current bounded decoder refuses the frozen 222-byte response. The
current largest pending page round-trips within the same finite decoder limits.
All seven live recovery scenarios compile but remain ignored.
Separate Rust 1.91 core Wasm checking and consumer Wasm release build, strict
native all-target/Wasm Clippy and warning-denied core Rust 1.91 rustdoc pass.
The actual consumer-tooling wrapper passes with substituted optimizer/server
effects, including one CLI success receipt per retained variant and none for
failed optimized capture. Source-following ShellCheck, formatting, documentation
links and diff checks pass. The initial harness compile caught a pattern binding
inside an assertion macro; the binding is now outside that assertion and the
corrected harness passes. The original failed log is retained.
Pending notes move the entire unreleased candidate to 0.17.0; package versions,
incoming manifest/lock bytes and primary index/refs stay unchanged. No full local
CI, live PocketIC, native macOS, release, publication, deployment or reset ran.

Restoration/summary/delivery cleanup evidence is in
`target/qualification/restore-summary-delivery-0163/`, relative to delivered Jobs
0.16.2 `41d9e31ff92de1703976d642b28d20ff08bae8aa` plus the incoming Testkit/Candid
0.52 manifest/lock selection. All 48 core tests and six doctests, 26 consumer tests
and 19 native composition tests pass on Rust 1.91. Separate core and consumer
Rust 1.91 Wasm checks and strict native all-target/Wasm Clippy pass. The extended
identity case preserves a near-u64::MAX ID through dispatch and outstanding
delivery projection; malformed stored identities refuse before views or delivery
escape. Receipt-query coverage also observes zero writes after reading a summary,
while retaining independent physical Cell reopening checks.
The first consumer compile caught a moved-backing error in the added write-count
assertion; the assertion now runs before reopening transfers the backing. The
corrected consumer checks pass, with the initial failure retained in the evidence.
Changed-source formatting, documentation links and diff checks pass. Incoming
manifest/lock bytes, own package versions and primary index/refs are unchanged.
No full local CI, live PocketIC, native macOS, harness qualification, release,
publication, deployment or reset was selected. Earlier receipts retain their
original source and graph scope.

Memory 0.48 write repair evidence is in
`target/qualification/memory048-writes-0162/`, relative to candidate commit
`8ec2a1cbd12cec4ebd7fd25d05a378f152bc3d8d` with its selected registry Memory
0.48.0/Build 0.15.0 graph. The supplied release log records eight missing `set`
calls; the previous projection receipt below used Memory 0.47 and does not
qualify this newer selection. All eight callers now use fallible bounded writes.
All 26 consumer tests and 19 composition tests pass on Rust 1.91, retaining
capacity/error precedence, malformed/current/retired record admission and byte
preservation. The separate consumer Rust 1.91 Wasm check and strict native
all-target/Wasm Clippy pass. Three capture tests and the harness's native bounded
reply test pass on Rust 1.91; all seven live recovery scenarios remain ignored.
Changed-source formatting, documentation links and diff checks pass. Manifest,
lock, own package versions and primary index/refs are unchanged. No full local
CI, live PocketIC, native macOS, release, publication, deployment or reset ran.
Core source is unchanged; its earlier floor evidence retains its source scope.

Consumer projection/reconciliation evidence is in
`target/qualification/consumer-projection-0162/`, relative to delivered Jobs
0.16.1 `31f5961e70b0033dfcfc7bae068bcd43c08d36c8` plus the incoming handoff and
Timers 0.27.1/Metrics 0.13.3 lock selection. All 26 consumer tests and 19 native
composition tests pass on Rust 1.91. The new boundary case preserves a valid ID
near u64::MAX exactly and refuses unallocated/oversized stored IDs before both
view entrypoints without replacing stored bytes. Existing coverage retains
authorization, commit/error ordering, payload matching and blocked uncertainty.
The separate Rust 1.91 consumer Wasm check and strict native all-target/Wasm
Clippy pass; the Wasm checks compile the actual canister reconciliation callers.
The first offline run stopped at uncached selected Metrics 0.13.3. A locked fetch
prepared the selected sources, then the focused checks passed offline; the lock
file is unchanged. Changed-source formatting, documentation links and diff checks
pass. Core source is unchanged and its earlier floor evidence retains its scope.
No full local CI, live PocketIC, native macOS, harness qualification, release,
publication, installation reset or core test rerun was selected for this cleanup.

Delivered 0.16.1 native acceptance is bound to
`31f5961e70b0033dfcfc7bae068bcd43c08d36c8` and
[CI run 39034706253](https://github.com/dragginzgame/ic-jobs/actions/runs/39034706253).
Linux (`ubuntu-24.04`) and Apple Silicon (`macos-15`) pass; Intel macOS
(`macos-15-intel`) is still executing validation at this inspection. The seven
implementation/adoption issues retain this latest delivered-source gate. Their
brief closure against the previous green release was corrected by reopening
them against 0.16.1. No workflow was dispatched or rerun. Native CI does not
establish live PocketIC recovery. Earlier focused receipts retain their source
and graph identities.

Management/fixture cleanup evidence is in
`target/qualification/management-fixture-0161/`, relative to delivered Jobs
0.16.0 `5b73312a5d34f01f8f09d4381e53723f21c3a5d6` plus the incoming publication-owner
work and Build 0.14/Host 0.26 manifest/lock selections. All 48 core tests and six
doctests, 25 consumer tests and 19 native composition tests pass on Rust 1.91.
Separate core and consumer Rust 1.91 Wasm checks and strict native all-target and
consumer Wasm Clippy pass. Existing coverage retains management error precedence,
attempted/unresolved refusal, queue bounds, failed writes and committed bytes.
The release adapter and publication policy fixtures pass, including exact upload
status and post-failure registry reconciliation; release gates and registry/upload
effects are substituted in disposable repositories. Source-following ShellCheck,
snapshot integrity, formatting, changed-document links and diff checks pass.
Rust 1.91 rustfmt was unavailable; the configured development Rust 1.99 formatter
was used. No full local CI, live PocketIC, native macOS, harness qualification,
release, publication or reset was selected for this cleanup.

Publication-owner evidence is in `target/qualification/publication-owner-0161/`,
against delivered Jobs 0.16.0 `5b73312a5d34f01f8f09d4381e53723f21c3a5d6` plus
incoming Build 0.14 metadata. The implementation comes from existing PR #67,
head `f6b75c9c4286e10f06fb95956994301331960d1e`; its retention probes additionally
select the new local fixture. The Linux publication ordering/race/failed-observer
and real-Git eligibility fixture passes, along with the actual complete release/
publication adapter fixture and source-following ShellCheck. Registry/upload and
release validation effects are substituted inside disposable repositories.

The first retention-selection run refused the fixture's compact root/case-arm
layout. The local fixture now uses the standard anchored root line and a
standalone status assertion; the repeated focused fixture passes. Shared runtime
bytes are unchanged. The actual Build capture tests pass on Rust 1.91; formatting,
changed-document links, snapshot integrity and diff checks pass. No full local CI,
live PocketIC, native macOS run, release, publication or reset is selected.

The earlier delivered 0.16.0
[CI run](https://github.com/dragginzgame/ic-jobs/actions/runs/39031920784) completed
successfully on Linux, macOS Intel and Apple Silicon. That exact-source proof
remains historical; routine acceptance now selects delivered 0.16.1 above.
PRs #55 and #65 are closed as source-incorporated/superseded by delivered registry
Testkit 0.51 callers, not merged or newly live-qualified. PR #67 is incorporated
in delivered 0.16.1 and closed; its older branch is retained.

Running/payload hard-cut evidence is in `target/qualification/running-payload-cut/`,
relative to Jobs 0.15.0 `71761e0c8ce3c6b2a4ce0226b80691892aa57f63` plus the
incoming Build/receipt/page/dispatch cleanup and selected Timers 0.27 graph.
All 48 core tests and six doctests pass on Rust 1.91, including bounded public
transition histories, Running's exact unit JSON shape, chronology refusal and
preservation of dispatch time through delayed uncertainty and reconciliation.
All 25 consumer and 19 composition tests pass, retaining admission/error order,
failed writes, independent Cell reopening, complete-queue bounds and blocked
unresolved effects. The recovery target compiles and its native reply case passes;
all seven live scenarios remain ignored.

The frozen 0.15 Running record, Running snapshot, flat receipt snapshot and flat
Candid reply were captured using the incoming serializers in an isolated copy.
Its original queue decoder admits both snapshots and its Candid decoder admits
the reply. Current code refuses the retired shapes, preserving refused Cell
payloads through both recovery and fresh-install refusal. They are negative
fixtures only; no old reader, converter or reset is added. The unchanged current
Uncertain JSON remains exact positive blocked-restoration evidence.

Separate core and consumer Rust 1.91 Wasm checks, the consumer Wasm build,
strict native all-target and Wasm Clippy, warning-denied Rust 1.91 rustdoc,
formatting, documentation links and diff checks pass. Incoming manifests/lock,
shared snapshot, capture edits, own versions and primary index/refs are preserved.
The numbered notes move the complete candidate to 0.16.0; no release, publication,
deployment, installation reset, full CI, live PocketIC or native macOS ran.
Earlier source qualifications below retain their original shapes and identities.

Receipt/page/dispatch cleanup evidence is in
`target/qualification/receipt-page-dispatch-cleanup/`, relative to Jobs 0.15.0
`71761e0c8ce3c6b2a4ce0226b80691892aa57f63` plus the incoming Build cleanup and
Timers 0.27 manifest/lock selection. All 25 consumer and 19 composition tests pass
on Rust 1.91. Existing cases preserve logical receipt lookup with exact-envelope
checks, zero-write queries/duplicate acknowledgements, out-of-order paging,
full-queue admission/deadline projection, failed commits and uncertain effects.
Dispatch assertions now also inspect both idle and committed reply deadlines;
independent stored reads and real Cell reopening proofs remain.

The consumer's separate Rust 1.91 Wasm check, strict native all-target and Wasm
Clippy, changed-source formatting, documentation links and diff checks pass.
Core source is unchanged; earlier core-floor evidence retains its source scope.
This batch preserves incoming manifests/lock, capture edits, snapshot, own
versions and primary index/refs. No full CI, live PocketIC, native macOS,
release, publication, deployment or reset ran.

Current Build/Host/reporter evidence is in
`target/qualification/build-host-report-cleanup/`, against Jobs 0.15.0
`71761e0c8ce3c6b2a4ce0226b80691892aa57f63` plus the retained Build caller diff.
On the actual registry graph, all three optimizer/capture tests pass on Rust
1.91, together with the actual consumer-tooling wrapper fixture (optimizer and
server effects substituted), recovery-target compilation and its native bounded
reply case. Strict harness Clippy and Rust formatting pass. Seven live recovery
scenarios remain ignored; no server is started by this qualification.

[Testkit PR #117](https://github.com/dragginzgame/ic-testkit/pull/117) is refreshed
without rewriting history at `5036ff00005badba183b599fb6ec8a092ea0a558`, merging
released 0.51 main `fca91d828e123bc6a5fc49bc703fb30981990bcc`. Its added reporter
case now uses public-field requests and both pending notes select 0.51.1;
package versions are unchanged. All seven diagnostic tests pass on Rust 1.91,
including exact independent senders, collection/writer panic containment,
ordinary writer errors and reports containing failed diagnostic calls. Strict
library/test Clippy, warning-denied Rust 1.91 rustdoc, the Wasm library floor,
formatting and changed-document local links pass.

The status module from
[Host PR #172](https://github.com/dragginzgame/ic-host-tooling/pull/172), source
`2de898dd5ddad41582acc676e607a44bc4778b45`, is qualified byte-identically on
published Host 0.25.1 with only that module/export added in an isolated clone.
Both real exit/signal and unclassified-status tests pass on Rust 1.88, with strict
Process Clippy and formatting. The owning integration branch is untouched.

Combined prepared Jobs callers pass three capture tests, native recovery reply
coverage with all seven live cases ignored, and strict all-target harness Clippy.
All 308 package names/versions remain identical; only the five explicit Host
Artifacts/FS/Process and Testkit/Candid source/checksum rows change in that
isolated lock. Both caller patches apply to the current working tree. The first
candidate build exposed duplicate Host artifact source identities; the corrected
qualification unifies that source and passes. Active Cargo manifests/lock, shared
snapshot, own versions, primary index and refs are unchanged. No full CI, live
PocketIC, native macOS qualification, release, publication, deployment or reset
ran. Delivered/native acceptance remains with the linked adoption issues.

Earlier receipts below retain their original source, graph and pending-version
scope. They do not override the current registry adoption or publication facts.

The Testkit 0.51 recovery-harness repair is qualified in
`target/qualification/testkit-051-recovery-fix/`, against clean incoming source
`56d7a6f9769fd5cac5fd14d490281d8b79aff513` and its selected registry graph. The
harness now uses public-field install/diagnostic/retry records and the current
explicit-caller Candid methods. Install sender/label/cycles, independent diagnostic
senders/default log bounds, reply decoding limits and four-attempt/ten-minute
retry settings remain unchanged. No compatibility adapters are added.

The complete recovery target compiles on actual Rust 1.91 and its bounded-reply
test passes; all seven live scenarios remain explicitly ignored. Strict Clippy
for that target passes without the unused-trait warning. Earlier consumer/capture
passes remain recorded in the supplied failure log; this repair does not relabel
them as new executions. Versions, dependency selections, shared snapshot and
primary index/refs are unchanged. No full CI, live PocketIC, native macOS,
release or publication ran. Earlier qualifications retain their source scope.

Entry/image cleanup evidence is in `target/qualification/entry-image-cleanup/`,
relative to the incoming pending 0.15 tree on `cc9587c`. The active registry caller
passes all 25 consumer and 19 composition tests on actual Rust 1.91, the separate
consumer Wasm floor check and strict native/Wasm consumer Clippy. Shared identity
lookups retain duplicate-before-restore and complete-restore-before-lookup order;
owned inspection retains state mapping, checked IDs, payloads and durable order.
Core runtime source is unchanged; its previous separate floor/Wasm evidence
retains its source scope.

[Memory PR #121 — Share bounded sparse memory image fixture loading](https://github.com/dragginzgame/ic-memory/pull/121)
proposes the fixture loader at `1b80f9c55129a5fb55ffc7839943f39368627089`, based on
published Memory 0.46.0 `578a25a15575c1e2c54439c41fc1866a847e006a`. Five producer
tests pass on Rust 1.91, with strict library/test Clippy, Rust 1.91 Wasm,
warning-denied Rust 1.91 rustdoc, formatting and changed-document links. The helper
imports only fresh zero-filled fixture backing under an explicit page ceiling;
it checks runs before allocation and never recovers authority or converts formats.

The isolated Jobs caller passes 25 consumer and 19 composition tests plus an
independent exact comparison reproducing both complete frozen backings with the
previous expansion. Its Rust 1.91 Wasm check and strict Clippy pass. Package
names/versions are unchanged; only Memory's source/checksum lock row differs in
that qualification copy. Producer/caller evidence includes the initial corrected
link-command and qualification-file documentation refusals.

The prepared `memory-adoption.patch` applies to the active tree. Jobs retains the
frozen images, provenance, schema/payload refusal and zero-write/full-backing
assertions; observation begins after fresh image import. Active registry adoption
and removal of the local expansion await reviewed publication under
[#66 — Adopt Memory sparse fixture-image loader after publication](https://github.com/dragginzgame/ic-jobs/issues/66).
There is no active path/Git dependency or copied upstream fallback. Root versions,
dependency selections, snapshot and primary index/refs are unchanged. No full CI,
live PocketIC, native macOS, reset, deployment, release or publication ran.

Evidence/fixture cleanup is qualified in
`target/qualification/evidence-fixture-cleanup/`, against the incoming pending
0.15 tree on `cc9587c`. All 47 core, 25 consumer and 19 composition tests pass on
actual Rust 1.91. The persistence example runs on Rust 1.91 and strict native
core/consumer Clippy passes. Existing cases retain success-over-rejection
precedence, payload matching, stale-attempt rejection, absent-evidence blocking,
failed commits, completion error order and idle zero-write admission. The shared
fixture builder asserts its actual target state; both callers retain their
original transition times. Formatting, documentation links and diff checks pass.
Independent writer-equivalence and corrupt-record
coverage are unchanged. This batch changes native fixtures and example/test
serialization callers; runtime and Wasm source is unchanged, so the prior
separate core/consumer Wasm qualification below retains its scope.
Own versions, dependency selections, shared snapshot and primary index/refs are
unchanged. No full CI, live PocketIC, native macOS run, reset, deployment, release
or publication ran.

Queue/retry cleanup evidence is in `target/qualification/queue-retry-cleanup/`,
relative to the incoming pending 0.15 tree on `cc9587c` and its incoming Memory
0.46.0/Timers 0.26.1/Build 0.12.1 lock selection. All 47 core, 25 consumer and 19
composition tests pass on actual Rust 1.91. Exact typed/raw snapshot bytes retain
metadata, payloads, receipts and blocked/terminal/paused records through reopening.
Collection/capacity refusal order, record restoration priority, failed commits,
deadline projection and notification record/payload pairing remain covered.
Retained retry corruption refuses across Pending/Paused/Cancelled; later
management timestamps and cancellation before the retry deadline remain valid.

Separate core/consumer Rust 1.91 Wasm checks, strict native/Wasm Clippy, recovery
harness compilation, warning-denied Rust 1.91 rustdoc and six doctests pass.
Formatting, documentation links and diff checks pass.
The first consumer attempts exposed the incoming Memory re-export and lease
changes; their failures remain retained alongside the corrected passes. The final
lock differs from the incoming selection only by the consumer's explicit raw Cell
development edge. Own versions, shared snapshot and primary index/refs remain
unchanged. No full CI, live PocketIC, native macOS qualification, storage reset,
deployment, release or publication ran. Earlier evidence retains its source scope.

Serialization/maintenance evidence is in
`target/qualification/serialization-maintenance-cleanup/`, relative to the incoming
pending 0.15 tree on `cc9587c`. All 46 core, 24 consumer and 19 composition tests
pass on actual Rust 1.91. The bounded transition-history explorer compares direct
Job serialization with its canonical record writer across every state; the exact
hand-authored JSON fixture still matches. Terminal corruption cases retain result
time, completion-kind and retry-budget refusal. Native fixtures retain exact
notification bytes, corrupt-record refusal, failed writes and partial-batch
reconstruction without repeating committed work.

Separate core/consumer Rust 1.91 Wasm checks, strict native/Wasm Clippy,
recovery-harness compilation, warning-denied Rust 1.91 rustdoc and six doctests
pass. The new compile-fail example fails specifically because Job does not
implement Deserialize, preserving record admission through `Job::restore`.
Formatting, documentation links and diff checks pass. The selected dependency
graph, own versions, shared snapshot and primary index/refs remain unchanged.
No full CI, live PocketIC, native macOS qualification, reset, deployment, release
or publication ran. Earlier qualifications below retain their source scope.

Paired-outcome cut evidence is in `target/qualification/paired-outcome-cut/`,
relative to delivered 0.14.1 `cc9587c` plus the incoming entry-admission cleanup.
The maintainer's explicit hard-cut/no-preservation answer supplies the current
retained-installation disposition. All 45 core tests, 24 consumer and 19
composition tests pass on actual Rust 1.91, including bounded transition-history
restoration, exact current-format JSON, incomplete/retired-shape refusal,
timing/counter corruption, competing admission errors, cell preservation and
unchanged zero-write refusal of incompatible outer ledgers. Running/Uncertain
states remain blocked until the existing explicit result disposition.

Separate core and consumer Rust 1.91 Wasm checks, strict native/Wasm Clippy,
recovery-target compilation, warning-denied Rust 1.91 rustdoc and all five
doctests pass. Formatting, documentation links and diff checks pass. The isolated
Build caller is refreshed with the new core/consumer Rust sources; its three
optimizer tests, recovery compilation and strict harness Clippy pass. Only
Build's source/checksum lock row differs there; the actual registry graph remains
unchanged and [#64 — Adopt Build post-processing custody after publication](https://github.com/dragginzgame/ic-jobs/issues/64)
retains its producer-publication gate. Earlier source qualifications below retain
their original format/source scope. Own versions, Cargo selections, the shared
snapshot and primary index/refs are unchanged. No full CI, live PocketIC, native
macOS qualification, storage reset, deployment, release or publication ran.

Entry/post-processing evidence is in
`target/qualification/outcome-entry-transform-cleanup/`, based on delivered
0.14.1 `cc9587c87812ef123918a64ad61d4ee729258921`. All 24 consumer and 19 composition
tests pass on Rust 1.91. The new competing-error cases preserve header-before-
restore, restore-before-entry-metadata, earlier-entry-before-later-record and
restore-before-receipt refusal order. Separate core/consumer Rust 1.91 Wasm checks,
strict native/Wasm consumer Clippy, formatting, links and diff checks pass.

[Build PR #53 — Share retained artifact custody for caller-owned post-processing](https://github.com/dragginzgame/ic-build/pull/53)
delivers the proposed helper at `5cd31f7`, based on published Build 0.12.0
`2598a417a96834d3139751adf549126add666c4b`. All 18 producer staging tests pass
on Rust 1.91, with strict library/test Clippy and warning-denied Rust 1.91 rustdoc.
The isolated Jobs caller passes three actual optimizer cases, recovery-target
compilation, strict harness Clippy and the actual consumer-tooling fixture with
optimizer/server effects substituted. All package names/versions are unchanged;
only Build's source/checksum lock row differs for source qualification. The
prepared caller patch applies to the active tree, but registry adoption remains
gated by [#64 — Adopt Build post-processing custody after publication](https://github.com/dragginzgame/ic-jobs/issues/64).
The active registry graph and local optimizer wrapper remain unchanged until
reviewed publication. No full suite, live PocketIC, native macOS run, release,
publication, reset, own-version change or primary index/ref mutation ran.

Snapshot/report/status evidence is in
`target/qualification/snapshot-report-status-cleanup/`, relative to this turn's
incoming pending 0.14.1 tree at base `242445f`. All 23 consumer and 19 composition
tests, separate Rust 1.91 core/consumer Wasm checks and strict native/Wasm consumer
Clippy pass for the active snapshot cleanup. Exact frozen JSON, incompatible-ledger
refusals, failed writes, receipt matching and uncertainty remain covered.

[Testkit PR #117](https://github.com/dragginzgame/ic-testkit/pull/117) delivers the
explicit diagnostic reporter at `7db1adc94158be0bc6116f9989a775237fe0eba8`, based on
current main `34eb5305342d86e0b55e0526379309f1c21fd54e`. It takes the exact request,
context and output writer; collection/formatting/output panics return as secondary
IO errors with their captured cause, and ordinary writer errors remain unchanged.
Seven focused diagnostic tests pass on Rust 1.91, with strict Clippy, rustdoc,
Rust 1.91 Wasm library checking, formatting and documentation links. The isolated
PR includes compatible 0.50.1 notes, without changing package versions. Earlier
formatter/test-style and preparation failures are retained alongside final passes.

Host PR #133 is closed as superseded by
[Host integration PR #172](https://github.com/dragginzgame/ic-host-tooling/pull/172),
source `85ef8b5c9f6a2e6213ab72f227e895098a7b39c8`. Its exact status module passes
two Rust 1.88 tests and strict Process Clippy on published Host 0.24.0 base
`f7ecf24557a6bbec6a6dc23ee9fb3e2052bf0251` with only its module/export added in a
separate checkout. The helper is not in the selected published Host package.
No duplicate contribution or sibling-checkout edit is made; current qualification
and the prepared caller are attached to the existing integration PR.

The combined isolated Jobs caller passes 23 consumer and 19 composition tests,
three capture tests, recovery-target compilation and strict native harness/Wasm
consumer Clippy. All package names/versions remain identical; only the five
explicit producer-source rows differ. Both caller patches pass apply-check
against the real working tree. The candidate is source qualification, not active
registry adoption: Jobs retains local diagnostic containment and numeric-status
projection until reviewed producer publication. These gates belong to
[#63 — Adopt panic-safe diagnostic reporter](https://github.com/dragginzgame/ic-jobs/issues/63)
and [#52 — Adopt shared child-status projection](https://github.com/dragginzgame/ic-jobs/issues/52).
No active dependency, package version, primary index/ref, live PocketIC, full CI,
release, publication, installation reset or native macOS qualification changed.

Fixture/decoder/runner cleanup evidence is in
`target/qualification/fixture-decoder-runner-cleanup/`, based on delivered 0.14.0
and the incoming lock selection. All 23 consumer tests and 19 composition tests
pass on Rust 1.91. They retain exact frozen Jobs JSON, unchanged incompatible
ledger backing with zero writes, restore/refusal order, failed commits, blocked
effects and receipt evidence. The new oversized-cell case bypasses the bounded
writer only to corrupt its fixture, then proves exact Cell header/payload
preservation for recovery and attempted installation. It makes no whole-backing
or blanket zero-write claim. The maximum 256-byte receipt and absent receipt are
admitted, while oversized replies remain unreadable rather than absent evidence.

Separate Rust 1.91 core/consumer Wasm checks, recovery-harness compilation and
strict native/Wasm consumer Clippy pass. Normal consumer Wasm includes
`ic-testkit-candid` and excludes PocketIC, full Testkit and Host. The only lock
change from the incoming graph is that consumer dependency edge; package versions
and other package rows remain unchanged. Formatting, pins, ShellCheck, shell
syntax, documentation links and diff checks pass.

Shared's exact pinned validation-target-runner fixture passes from a separate
Git checkout at `989752490b392f7f4c542c7652dcb84e75ecf363`; its runner matches the
active Jobs payload byte for byte. The first archive-only preparation could not
serve its nested clone fixture; that failure and its disposable fixture remain
retained. The corrected checkout passes generic mutation/deletion/redirect,
observation failure, checkpoint and timing coverage. The narrowed actual Jobs
consumer-tooling fixture passes Bash 5 with optimizer/server effects substituted;
it retains actual selected-lock admission-stop, original capture and variant/
failure evidence. No Shared source or snapshot is changed. Linux Bash 5 does not
establish Bash 3.2 or native macOS qualification.

[Jobs #50](https://github.com/dragginzgame/ic-jobs/issues/50) now tracks the actual
registry-based caller adoption, replacing its unpublished-source preparation.
Delivery and the applicable native matrix remain pending. No full CI, live
PocketIC, release, publication, installation reset or primary index/ref mutation ran.

Paired-entry cleanup evidence is in
`target/qualification/paired-entry-cleanup/`, based on local source `2e7a3f7`
with the incoming dependency updates preserved. All 44 core integration tests,
22 consumer and 19 composition tests pass on Rust 1.91, including exact frozen
Jobs JSON, incompatible-ledger zero-write refusal, full-snapshot validation,
failed commits and completion-refusal ordering. The scheduler's existing ordering
case now projects jobs from paired payload entries. Alignment state and its
synthetic mismatch test are removed because the loaded shape cannot split them.
The delivery projection constructs one full expected envelope for dispatch,
inspection and completion; comparison may copy the bounded 256-byte message,
while transitions and durable-write order remain unchanged.

The three remaining capture tests, native maximum-page/oversize-reply test,
separate Rust 1.91 core/consumer Wasm checks, strict native/Wasm Clippy, rustdoc
and all five doctests pass on the selected graph. The recovery harness compiles;
its seven live scenarios remain ignored. `check-consumer` runs the native reply
proof without selecting live execution. The actual consumer-tooling fixture
passes Bash 5 with substituted optimizer/server effects, including original
capture, producer replacement, exact variant flags and retained failure evidence.
Generic capture/retention behavior is already owned by Build's staging tests.
Initial checks exposed Memory/Testkit API fallout and concurrent graph changes;
failed receipts remain alongside passing current-graph logs. Earlier sections
below retain their historical source/graph scope and pending-version names.
No full CI, live PocketIC, native macOS, reset, release or publication ran.

Memory #105 caller evidence is in
`target/qualification/memory105-current-adoption/`. The exact producer head
`5ce93dc8903642f4a0725ceafc75a78a07bd5494` is qualified only in a separate consumer
copy, based on delivered Jobs 0.13.3 plus the pending install-diagnostics edit.
Its 24 consumer and 19 composition tests, separate Rust 1.91 core/consumer Wasm
checks, recovery compilation and strict native/Wasm consumer Clippy pass.
Only Memory's source changes in that qualification lock; all package versions
and other lock rows match the active graph. Normal consumer Wasm excludes
Testkit, PocketIC and Host. The complete caller patch passes apply-check against
the active tree and is attached to [Jobs #50](https://github.com/dragginzgame/ic-jobs/issues/50).

The candidate uses existing 24 KiB bounds for distinct fresh/retained admission.
Oversize refusal preserves the cell header and payload, while documented
allocation-bookkeeping effects remain possible. Earlier zero-write/whole-backing
assumptions failed and are retained as such; the corrected proof scopes exact
preservation to the admitted cell. The frozen incompatible-ledger cases still
prove zero writes and unchanged backing separately. Temporary preparation-path
failures are retained; final source and caller copies live under qualification.
The active Rust/registry graph and the pending 0.13.4 fix remain untouched.
No actual adoption, full CI, live recovery, native macOS Jobs run or release ran.

Recovery install-diagnostic evidence for pending 0.13.4 is in
`target/qualification/recovery-install-diagnostics-0134/`. The actual recovery
target compiles on Rust 1.91 against locked registry Testkit 0.49.0; strict recovery
Clippy, formatting, documentation links and diff checks pass. This qualifies API
integration, not live installation failure or installed recovery. No full CI,
PocketIC execution, release or publication ran for this pending change.

Delivered 0.13.3 acceptance is
[CI run 38923932920](https://github.com/dragginzgame/ic-jobs/actions/runs/38923932920),
source `684cf173af893c56f87ac462e830de960807240b`. Linux and macOS Apple Silicon
pass; macOS Intel is still running at this review. Jobs #54/#58/#59/#60/#61 remain open for that
complete matrix. Source-incorporated PRs #51 and #62 are closed without merging;
PR #55 remains open until this newer local integration is delivered. Earlier
receipts below retain their original source and pending/delivery observations.

Issue-reduction/adoption evidence is in
`target/qualification/issue-reduction-shared-adoption/`. Jobs
[#53 — Shared Cargo dependency-order fixture preparation](https://github.com/dragginzgame/ic-jobs/issues/53)
meets delivered/native acceptance at 0.13.2 `c497c0f`, with all three lanes of
[CI run 38888314944](https://github.com/dragginzgame/ic-jobs/actions/runs/38888314944)
passing. That result does not qualify the current dirty source.

The existing PR #62 caller selection is applied locally before refresh; actual
release/publication fixtures pass Bash 5 and genuine Bash 3.2 at both the old
pin and Shared 0.16.0. Canonical preview/application preserve every retained root
and retire only the three selected operational payloads. The first previews
refused the isolated clone's local/suffixed remote; selecting the exact canonical
remote corrected provenance before any export. Snapshot integrity verifies all
111 payloads. No active snapshot payload is hand-edited.

Actual release, consumer and formatting-evidence fixtures pass both shells on
the new snapshot, including selected Make paths with spaces, failed fetch/tool
admission before gate/intent and exact source/index custody. Formatting/hook
qualification, Bash 3.2's actual formatting caller, pins, release adapters,
source-following ShellCheck, documentation links and diff checks pass. Release,
registry/upload, optimizer and server effects remain substituted in disposable
fixtures. Shared's delivered source has its own
[three-host CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38906056937).
At that qualification, Jobs #54/#58/#59/#60/#61 retained delivered/native
acceptance; PR #62's green matrix covered its own earlier source.
No Rust source, incoming Cargo selections, own version or real index/ref changes
are part of this tooling batch. No full CI, live recovery, release, publication,
service installation or native macOS execution ran locally. Earlier qualification
sections below retain their original source/snapshot claims as historical evidence.

Scheduler/installer/update evidence is in
`target/qualification/schedules-installer-update-cleanup/`. All 25 scheduler/job,
ten management and one bounded history-explorer tests pass on Rust 1.91. The
recurring constructor is separately importable; expected deadlines, history
exploration order and operation-specific codec/restore diagnostics are retained.
One caller-explicit update boundary serves ordinary recovery calls and all seven
authorization/trap sites, preserving the outer structured Candid error.
Rust 1.91 recovery-target compilation and strict core/recovery Clippy pass.

One exact setup/admission substitute serves server, workflow and recovery-wrapper
qualification after the real-parser refusal cases. Input-refusal assertions now
require CLI admission before metadata access. Actual consumer-tooling fixtures
pass Bash 5 and genuine Bash 3.2 with optimizer/server effects substituted;
ShellCheck, formatting, diff and documentation links pass. The incoming Build
0.11 / Host 0.23.1 / Metrics 0.11.7 selection is retained; the capture tool is
rebuilt on Rust 1.91 and both consumer-tooling shells pass again against that
binary. The graph, snapshot, own versions and root index/refs are preserved. No full CI, live PocketIC,
release, publication or native macOS qualification ran.

Extraction/typed-management/collector evidence is in
`target/qualification/extraction-typed-management-collector/`. All 23 consumer
and 19 composition tests pass on Rust 1.91, including authorization before
corrupt decoding, per-operation refusal preservation, failed commits, returned
deadlines, exact receipts and blocked-effect recovery. Typed fixture operations
retain the same bounds, policies and stored JSON; no published API changes.
Strict native/Wasm consumer Clippy and separate Rust 1.91 core/consumer Wasm
checks pass. The initial Clippy callback-signature failure is retained; its
correction uses a named test-only function-pointer type. Formatting uses the
prepared development rustfmt; the minimum toolchain has no rustfmt component.

The exact pinned Shared producer's eight refused-store/compaction combinations
pass separately on Bash 5 and genuine Bash 3.2. The adopted collector matches
that pin byte for byte. Its generic regression already retains independent local
files, selection diagnostics and incomplete-collection refusal; no upstream
source change is required. Jobs removes its duplicate refused-store scenario
and keeps its actual CI gate/collector ordering and exact archive round trip for
formatting logs and retained inner fixtures. That caller passes both shells;
ShellCheck, formatting, diff and documentation-link checks pass. This supersedes
the earlier local collector roster below without relabelling earlier receipts.
The incoming graph, snapshot, own versions and root index/refs are preserved.
No full CI, live PocketIC, release, publication or native macOS qualification ran.

Upstream helper refresh evidence is in
`target/qualification/upstream-helper-handoffs/`. Registry inspection finds no
native Testkit package entry; published Timers 0.25.2 and Host Process 0.23.1
lack the proposed APIs. Active Jobs Rust, manifests/lock and snapshot are unchanged.
The apply-checked source patches select unpublished helpers only in a separate copy.
Its 23 consumer, 19 composition and four capture tests, recovery compilation,
separate Rust 1.91 core/consumer Wasm checks, consumer Wasm test compilation,
strict native/Wasm Clippy and actual tooling fixtures on Bash 5/genuine Bash 3.2
pass. Normal consumer Wasm dependencies exclude Testkit/PocketIC. The counter
replacement covers all four constructions and rejects overflow at all five
observations; deadline/work policy, trap-before-delivery and optimizer custody stay local.

Testkit PR #97 remains stacked on #88 at
`741e2e8e89cd1442e29dd3a8e1ff235260806bf0`; all four IO contracts, two allocator
tests, actual Rust 1.88 native/Wasm, strict Clippy and Rustdoc pass unchanged.
Host PR #133 now includes released 0.23.1, head
`68929d021232dcda14863a92af2839bf2012ee17`, with pending 0.23.2 notes and restored
published history. Both actual Rust 1.88 status tests, Clippy and Rustdoc pass.
Timers #64 has a refreshed, uncommitted patch on released 0.25.2
`19ed7612afd12f68ce2fe44ff7d96be926c9f290`, with pending 0.25.3 notes. Its nine
work/deadline combinations, actual Rust 1.91 Wasm, Clippy and Rustdoc pass;
its AGENTS.md still reserves contribution commits to the maintainer.
Jobs #52/#56/#57 retain reviewed publication, actual registry qualification and
delivered/native acceptance. No full CI, native macOS, live PocketIC, release or
publication ran. Sibling working trees and root index/refs remain unchanged.

Ticks/traps/schedules evidence is in
`target/qualification/ticks-traps-schedules-cleanup/`. Recovery reuses Testkit's
advance-then-tick helper with unchanged durations and tick counts. One recurring
schedule constructor serves five core test sites, retaining all policy cases,
one-shot management cases, zero deadlines, maximum intervals and fifty-cycle history.
All 25 Jobs and 10 management tests pass on Rust 1.91, as do recovery-target
compilation, focused core/recovery Clippy and the capture-tool build. The incoming
Testkit 0.49 manifest/lock selection is preserved; these receipts bind that graph.

Four direct EXIT traps retain their original labels and completion/failure policy.
Actual release/retention, formatting-evidence, consumer-tooling and formatting/hook
fixtures pass Bash 5 and genuine Bash 3.2. Exit probes retain nounset, premature,
failed completion and cleanup-refusal evidence; release gates and registry/upload
effects remain substituted in disposable repositories. Source-following ShellCheck,
formatting and documentation links pass. The snapshot, own versions, manifest/lock
selections and root index/refs remain unchanged. No full CI, live PocketIC, release,
publication or native macOS qualification ran.

Jobserver/corruption/policy evidence is in
`target/qualification/jobserver-corruption-policy-cleanup/`. Canonical preview and
export add only the pipe-jobserver probe at the existing reviewed revision; all
110 earlier payloads retain their exact bytes and executable modes. Formatting
now uses its descriptor identity, pipe and direction checks. Actual formatting/
workflow/archive fixtures pass Bash 5 and genuine Bash 3.2; ShellCheck, Perl
syntax, snapshot integrity and dependency pins pass.

The shared corrupted-queue assertion path preserves exact dispatch/reopen errors
and bytes, and checks summary/inspection rejection for all six corruption cases.
One valid retry-policy table retains getters, JSON and named-token contracts.
All four configuration tests and 23 consumer tests pass on Rust 1.91; focused
core/consumer Clippy, formatting and documentation links pass. The incoming lock
already selected Timers 0.25.2 / Metrics 0.11.6; an initial missing-cache refusal
is retained, followed by locked source preparation and passing offline checks.
Manifest/lock selections, own versions and the root index/refs remain unchanged.
No full CI, live PocketIC, release, publication or native macOS qualification ran.

Refusal/source cleanup evidence is in
`target/qualification/refusal-source-cleanup/`. One completion-case table retains
all six receipt/lookup/clock cases across both payloads and seven states. The live
recovery source shares management refusals across Running and Uncertain, retaining
cancellation and receipt-count checks and checking unchanged views in both states.
All 23 consumer tests, Rust 1.91 recovery-target compilation, focused consumer/
recovery Clippy and formatting pass. The live test was compiled, not executed.

[Shared PR #242](https://github.com/dragginzgame/shared-tooling/pull/242),
`75e9c9c12c9dacba602b62d1a89e5a4ee2a9a209`, adds exact lock/index preservation
after literal allowance and exact index/destination preservation after rename
refusal to the existing producer fixture. It passes Bash 5 and genuine Bash 3.2;
the production checker matches the active Jobs pin byte for byte. Jobs deletes
its duplicate direct-checker cases and retains actual release/metadata-adapter
dirty-source and failed-observation refusals before later effects. Actual Jobs
release/publication fixtures pass both shells with gate/registry/upload effects
substituted; ShellCheck and documentation links pass at both owners. Upstream
review remains with the PR. The incoming graph, snapshot and root index/refs are
unchanged. No full CI, live PocketIC, release, publication or native macOS
qualification ran; the Shared contribution used a separate clone/topic branch.

Capture/admission/target cleanup evidence is in
`target/qualification/capture-admission-target-cleanup/`. The combined capture
failure test proves producer/log retention, then exact retained bytes after
explicit producer removal. One admission table covers absent, symlink and
directory inputs at both ordered stages; capture admission receives valid Wasm.
All four capture tests, Rust 1.91 recovery-target compilation, strict capture
Clippy and the capture-tool build pass. The actual consumer-tooling fixture
passes Bash 5 and genuine Bash 3.2 with optimizer/server effects substituted;
ShellCheck, formatting and documentation links pass. Make still runs capture
tests before compiling only the recovery target with `--no-run`. The incoming
graph, active snapshot, own versions and index/refs are preserved. No full CI,
live PocketIC, release, publication or native macOS qualification ran.

Delivery/upgrade/formatting cleanup evidence is in
`target/qualification/delivery-upgrade-format-owner/`. Completion now performs
its active-attempt/task checks directly, retaining error precedence and exact
receipt comparisons. One result-returning fixture upgrade path serves success,
expected rejection and Testkit's existing selective retry. All 23 consumer tests,
Rust 1.91 recovery-harness compilation, separate core/consumer Wasm checks and
focused native/Wasm Clippy pass. Incoming Memory 0.44 / Build 0.10 edits arrived
during qualification and are preserved; `current-graph/` binds rerun receipts
to the unchanged manifest/lock across those checks. Earlier receipts retain
their original scope.

[Shared PR #240](https://github.com/dragginzgame/shared-tooling/pull/240),
`06e82e8a9cce6a7107735df4f824cf4dc6fcfe59`, owns standalone formatting success
and exact earlier-log preservation. Producer formatting and actual Jobs
workflow/archive fixtures pass Bash 5 and genuine Bash 3.2; ShellCheck,
formatting and documentation links pass. The wrapper matches the active pin
byte for byte; the transfer changes tests only and requires no runtime snapshot
adoption. Jobs retains actual Make, workflow ordering, archive round trips,
refused-store evidence and incomplete-collection checks. The active snapshot,
own versions and root index/refs remain unchanged. No full CI, live PocketIC,
release, publication or native macOS qualification ran.

Store/Make/publication cleanup evidence is in
`target/qualification/store-make-publication-cleanup/`. One raw-byte constructor
reconstructs native stores and clears volatile fault injection; consumer loads
retain bounded decoding and Job::restore. All 19 composition tests on Rust 1.91,
focused strict Clippy and formatting pass. The release fixture now uses the
copied production Makefile, substituting the validation runner instead of
maintaining another set of release recipes. Tool admission, source/metadata
refusals, recovery and exact publication arguments remain qualified locally.
Actual Jobs release/publication fixtures pass Bash 5 and genuine Bash 3.2;
source-following ShellCheck and documentation links pass.

[Shared PR #238](https://github.com/dragginzgame/shared-tooling/pull/238),
`5703391e88747e74c3eb5c863224a433943620a2`, transfers the generic publication
source/tag/destination and registry matrix into its existing producer fixture.
Both Bash 5 and genuine Bash 3.2 pass; source-following ShellCheck and documentation
links pass. All six publication runtime helpers match the active Jobs pin byte
for byte, recorded alongside the exact producer base/head. No runtime helper
or snapshot adoption is part of this test-only transfer; upstream review remains
with that PR. Actual Jobs adapter refusals and failed/uncertain upload checks
remain here. Focused fixture effects use disposable Git repositories and
substituted validation/registry/upload effects; no full CI, live recovery,
release, publication or native macOS execution was selected.

View/snapshot/probe cleanup evidence is in
`target/qualification/view-snapshot-cleanup-probes/`. Inspection/list replies now
derive their checked identity from the validated job at one view boundary; the
composition snapshot no longer derives unused cloning. All 23 consumer and
19 composition tests, separate Rust 1.91 core/consumer Wasm checks, native/Wasm
consumer Clippy and formatting pass on the preserved incoming graph.

[Shared PR #237](https://github.com/dragginzgame/shared-tooling/pull/237) adds
cleanup refusal to the standard exit-probe roster. Producer callers pass Bash 5
and genuine Bash 3.2. A canonical prospective 111-payload export at
`cd2a673353275d09be8223420b88e5a165a84534` passes integrity and the actual Jobs
release/publication fixtures on both shells with external effects substituted.
The apply-checked Jobs deletion is retained under `adoption/`; active Shared
0.15.2 still needs its explicit cleanup case. Reviewed export and delivered/native
acceptance remain with [Jobs #60](https://github.com/dragginzgame/ic-jobs/issues/60).
No active snapshot, root index/ref, own version, live recovery, release or
publication changed. The Shared contribution used its isolated PR workflow;
no sibling working checkout was edited. Linux Bash 3.2 is not macOS proof.

Capture/admission/roster cleanup evidence is in
`target/qualification/capture-admission-roster-cleanup/`. Recovery-only Candid,
Testkit, consumer and Serde edges are development dependencies; the capture
binary's normal graph excludes Testkit/PocketIC and the consumer package. Its
Rust 1.91 build and five tests pass, as does recovery-harness compilation on the
incoming Build 0.9 / Testkit 0.48 graph. Cargo discovery executes all 45 current
core integration tests without a manual target roster. The actual hosted Testkit
selection step uses the Make adapter once, keeping its checked path separate
from retained diagnostics; success/refusal and the remaining consumer-tooling
fixtures pass Bash 5 and genuine Bash 3.2 with tool/server effects substituted.
ShellCheck, manifest formatting and documentation links pass. Incoming root
manifest/lock selections and the shared snapshot are preserved. No live recovery
or full CI ran; native macOS acceptance remains separate.

[Delivered 0.13.0 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38753013217)
passes Linux, macOS Apple Silicon and macOS Intel at
`123b8f0c9a5abcabd9498503d947d45908aa49df`. This supplies delivered/native acceptance for shared fixture allocation
and completion admission (#47/#48); it does not qualify later edits.

Current-graph focused Rust receipts are in
`target/qualification/dispatch-write-toolchain-cleanup/`: 19 composition tests,
23 consumer tests, five capture tests, harness compilation, strict Clippy and
separate Rust 1.91 core/consumer Wasm checks pass. Actual CI/recovery toolchain
inputs and release/evidence/consumer fixtures pass Bash 5 and genuine Bash 3.2.
Source snapshots bind receipts to their graph; earlier graphs remain historical.

Current tooling cleanup receipts are in
`target/qualification/release-frontend-handoff-cleanup/`. Canonical export preview
confirmed that `scripts/dev/format-frontend.sh` remains required by the adopted
`test-git-hooks.sh` frontend cases. It is retained with its explicit snapshot root;
removal requires splitting the canonical hook fixture first, not deleting a
snapshot payload. No runtime frontend formatting flow is enabled in Jobs.
Release/publication and consumer-tooling fixtures pass Bash 5 and genuine Bash
3.2 with external effects substituted. Hook qualification, snapshot integrity,
pins, release adapters, source-following ShellCheck and documentation links pass.
The first hook run caught a deleted-catalog fixture input; its failure is retained
and the corrected run passes. No Rust source changed in this tooling batch.

Dispatch/delivery/formatting cleanup evidence is in
`target/qualification/dispatch-delivery-format-cleanup/`, based on delivered
0.13.1. All 23 consumer tests pass, including idle zero-write, unchanged bytes,
exact receipts and refusal ordering. Separate Rust 1.91 core/consumer Wasm checks,
strict native/Wasm consumer Clippy and recovery-harness compilation pass. Actual
formatting/hook checks pass with prepared formatters; the Jobs caller also passes
on genuine Bash 3.2. Snapshot integrity, pins, release adapters, ShellCheck, Perl
syntax and documentation links pass. Existing test expectations follow the same
deadline/payload results; no new test flow, public API or storage layout is added.
The old graph-bound receipts above remain scoped to their recorded inputs.

Release/memory/watchdog evidence is in
`target/qualification/release-memory-watchdog-cleanup/`. Release preflight no
longer probes cargo-sort help after common-tool admission; actual release and
publication substitutes pass Bash 5 and genuine Bash 3.2, with ShellCheck passing.

The prospective memory owner is Testkit PR #97, not a second Memory API; Memory
#101 records that accepted decision. Exact Testkit head
`741e2e8e89cd1442e29dd3a8e1ff235260806bf0` passes all four memory IO contracts and
Rust 1.88 Wasm compilation. The Timers completed-batch proposal is refreshed
against delivered 0.25.0 `11e73bff33676e6d829baa502fb943ffa312b988`; its independent
work/deadline regression, Rust 1.91 Wasm and strict Clippy pass. Timers' explicit
maintainer-owned contribution-commit rule keeps that change as a tested patch
posted on its existing issue, without a contribution commit or PR.

Combined isolated Jobs adoption passes 19 composition tests, 23 consumer tests,
Rust 1.91 Wasm test compilation and strict consumer/composition Clippy. Counts
retain shared reset/backing and reject overflow; frozen image checks stay local.
The production consumer Wasm tree excludes Testkit/PocketIC. Qualification-only
manifests/lock and apply-checked source patches are under `adoption/`; the only
lock changes are the local Timers edge and optional native Testkit package.
These receipts qualify unpublished source, not active registry adoption. The
existing local counter/projection remains until reviewed delivery/publication;
[Jobs #56](https://github.com/dragginzgame/ic-jobs/issues/56) and
[Jobs #57](https://github.com/dragginzgame/ic-jobs/issues/57) own that acceptance.
No active path override, own version, primary index/ref, sibling working-tree, live
recovery, release or publication mutation ran. Incoming Host 0.21 selections
were preserved. The direct crates.io availability probe returned HTTP 403;
it establishes no package availability or absence.

Composition/restore/release cleanup evidence is in
`target/qualification/composition-restore-release-cleanup/`. The unchanged
19-scenario native composition harness and ordinary support modules now belong
to `apps/job-consumer/tests/`; core no longer has Memory/Timers dev-dependencies.
`test-consumers` and the separate composition MSRV check select that package;
`check-consumer` selects its library tests to avoid replaying composition.
The lock changes only the core local dependency row; package identities and all
external selections match this batch's incoming lock. Memory 0.43 / Build 0.8
catalog updates were incoming and are preserved.

Restore reuses its occurrence offset only after admitting origin bounds; checked
multiplication/addition and InvalidRecord refusals remain. All 45 core tests,
five doctests, 19 composition and 23 native consumer tests pass. Separate core/
composition Rust 1.91 checks, core/consumer Wasm and strict all-target Clippy pass.
Jobs retains eleven release/format/publication mode/control cases with unchanged
source/index/fetch/gate/state assertions. The exact pinned Shared producer's
exhaustive release and formatting matrices pass separately. Both actual Jobs
release fixtures pass Bash 5 and genuine Bash 3.2; ShellCheck, formatting,
snapshot integrity, pins, release adapters and documentation checks pass.
Earlier receipts and prospective caller patches retain their original graph and
paths; the watchdog caller patch is refreshed for the composition relocation.

Fixture/workflow/derive cleanup evidence is in
`target/qualification/fixture-workflow-derive-cleanup/`. The actual CI gate has a
stable `validate` ID; its run body and both collector bodies use the canonical
extractor. Jobs retains workflow ordering, temp-root, archive and failure-status
assertions. Active evidence fixtures pass Bash 5 and genuine Bash 3.2. All 23
consumer tests, Rust 1.91 consumer Wasm, recovery-harness compilation and strict
consumer/harness Clippy pass after removing five unused Clone derives. Candid
fields and stored codecs are unchanged. Release-retention fixtures, source-following
ShellCheck, formatting, snapshot, pins, release adapters and doc links pass.

Make-fixture bootstrap remains prospective: Shared PR #227 at
`aa401e37e1d87abd4329fd4c4986b5d88e7c6a4e` is still open. Both affected Jobs
fixtures pass both shells in an isolated copy using that helper and the current
common Make source; producer preparation/refusal tests also pass both shells.
The apply-checked caller patch and exact helper are retained under `adoption/`.
This does not qualify or adopt the complete proposed Shared revision. Active
copy lists await canonical export from reviewed source and actual requalification,
tracked in [Jobs #58](https://github.com/dragginzgame/ic-jobs/issues/58).

Release-owner convergence evidence is in
`target/qualification/release-owner-convergence/`. One installer substitute now
covers common leaf routing; Jobs retains late parallel admission and setup/check
refusals before gate effects. Adapter mismatch and missing-version preparation
checks reuse the existing patch-release repository. Shared PR #231's exhaustive
leaf fixture and PR #232's selected-source metadata fixture pass both shells;
the relevant production helpers match the active pin. Generic cache tests remain
active while the manual preparation block remains.

[Shared PR #233](https://github.com/dragginzgame/shared-tooling/pull/233),
`c00f3602bcdc193230acd2c54bd5a39680e23d08`, fixes shared preflight's lost
RELEASE_MAKE selection. Both producer shells pass selected paths with spaces and
failure propagation. Its prospective Jobs adapter and narrower cache fixture
are qualified through a canonical 112-payload export in isolation; the active
110-payload snapshot and preparation adapter remain unchanged pending reviewed
export, tracked in [Jobs #59](https://github.com/dragginzgame/ic-jobs/issues/59).
Earlier export/setup refusals are retained separately from corrected
qualification. No root index/ref, package version, sibling checkout, release or
publication mutation was performed; the scoped Shared contribution used its
isolated PR workflow.

Full local CI and live PocketIC were not selected for this batch. Linux Bash 3.2
execution does not establish native macOS acceptance. The last recorded live
recovery evidence is Jobs 0.8.1, source
`f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
[run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820):
84 passes across seven scenarios, four Wasm variants and three hosts. Current IC
recovery, upgrades and timer delivery still require explicitly selected live proof.
Retained pre-cut Memory installations still require explicit owner disposition;
no reset, migration or deployment ran.

## Upstream follow-up

GitHub owns live tracker status. The remaining source adoption proposals are
[Host child-status projection](https://github.com/dragginzgame/ic-jobs/issues/52),
[Testkit memory observation](https://github.com/dragginzgame/ic-jobs/issues/56),
[Timers completed-batch projection](https://github.com/dragginzgame/ic-jobs/issues/57),
[Testkit panic-safe diagnostics](https://github.com/dragginzgame/ic-jobs/issues/63)
and [Memory sparse fixture loading](https://github.com/dragginzgame/ic-jobs/issues/66).
Their linked upstream proposals remain unintegrated; the selected registry
packages do not export the proposed helpers. Keep actual callers until reviewed
publication and registry qualification; no Git/path substitution is active.
Prepared patches and their earlier source qualification remain evidence, not
proof of publication or current native/live acceptance.

Source-incorporated preparation PRs #51/#55/#62/#65/#67 are closed. The local
publication policy is delivered; unused Shared snapshot bytes retire only
through a reviewed canonical export. See the current native receipt above for
Jobs' delivered-source gate rather than treating older running observations as
current acceptance.

Earlier source-bound receipts and original tracker descriptions are preserved in
[history through 0.13.0](history-through-0.13.0.md). See also the
[package contract](../../crates/ic-jobs/README.md), [safety contract](../../SAFETY.md),
[consumer qualification](https://github.com/dragginzgame/ic-scheduling/blob/bb67e6f23ec50a95cf1b58f675cd5a8d8c60f4ff/apps/ic-jobs-test-consumer/README.md) and
[delivery procedure](../releasing.md).
