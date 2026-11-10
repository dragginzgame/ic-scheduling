# Current handoff

## Current preparation — 0.9.1 consumer flow cleanup

Additional borrowed-consumer cleanup is complete. Completion validates the exact
retained state, task and envelope fields without cloning an Entry/Delivery;
owned envelopes remain the output of outstanding inspection. Removed unused
Entry/Snapshot Clone derivations. Notification cancellation now admits one
snapshot, applies Job::cancel, commits, then projects its reply/deadline from
that same state. Its direct fixture callers now use the authorized management
entry; the superseded private cancellation path is removed. Receipt/rejection
lookup borrows payloads and keeps success-first evidence selection.

Focused `make test-consumers check-consumer` passes: 21 consumer tests each with
core/timers on Rust 1.88, all 16 stable-consumer tests, release Wasm, harness
compilation and strict native/Wasm Clippy. New evidence cases prove mismatched
payloads retain blocked intent and a matching success receipt wins over rejection
evidence without sending a delivery. Existing unknown-job, clock, corruption,
receipt mismatch, commit failure and unchanged-byte checks remain passing.
Evidence: `target/qualification/borrowed-consumer-cleanup/`. This is local
qualification of the pending 0.9.1 batch, not hosted or live acceptance.

Issue acceptance: #39/#40 are closed after inspection of actual 0.8.7 CI logs
for `0951f3083c961204bd6b40da97251d0812798031`, run 38395094597: complete gates
pass Linux and both macOS architectures. #41/#43 are delivered in 0.9.0;
Linux passes run 38421834204, with macOS acceptance pending at inspection.
#42 remains open: Shared #165 and Build #14 are integrated producer drafts,
with committed/published delivery and canonical Jobs adoption still pending.

The test consumer now has one synchronous management-update boundary for create,
cancel, pause, resume and reschedule. It authorizes before invoking the queue
operation, drops the queue borrow, then reconciles only after success. A
reconciliation trap remains in the same message segment as the stable mutation;
queries, delivery dispatch and asynchronous reconciliation retain their own
boundaries.

Delivery completion still admits the complete snapshot and checks the exact
retained envelope before clock/transition checks. It now uses the restored job's
canonical active attempt instead of reconstructing its identity. Expanded
completion coverage rejects altered occurrence, sequence and destination across
all retained states/tasks, preserving refusal precedence and committed bytes.
Deadline summaries fold over restored jobs without collecting a mutable batch;
full snapshot admission, blocked/terminal exclusion and earliest-deadline
semantics remain unchanged.

Focused `make check-consumer` passes on the current locked graph: all 16 native
consumer tests on Rust 1.88, release Wasm, harness compilation, and warnings-denied
native/Wasm Clippy. Inputs include Timers 0.22.1, Memory 0.36.3, Metrics 0.9.0,
Testkit 0.37.0 and Host 0.17.4. Evidence and input hashes are retained under
`target/qualification/consumer-cleanup-091/`. Formatting and documentation links
pass. Source review establishes the synchronous authorization/reconciliation
ordering; native tests and compilation do not establish live IC trap rollback.
No full CI or live PocketIC gate was selected. Library API, storage shape,
manifest/lock selections, release metadata and real index/refs remain unchanged.
Pending notes are 0.9.1; delivered source at task start was
`70feddfa188fc625b6ec83d4291c0757281c69f9` (0.9.0).

## Earlier preparation — 0.9.0 dispatch/artifact/fixture cleanup

Implemented [#43](https://github.com/dragginzgame/ic-jobs/issues/43): Pending
admission guarantees attempts below the immutable nonzero retry budget, so
start increments that counter directly. Checked u64 sequence advancement,
clock/state/due ordering and atomic mutation remain unchanged. Every restore
still uses the validated boundary; no public error variant was removed.

Independent Rust **1.88** original/cleaned probes match across **76,044** inputs,
including all 354 frozen records and budget/counter/time/history extremes.
There are **15,724** admitted start operations: **1,551** successes and **14,173**
refusals, with explicit unchanged-record assertions for every failed start.
The remaining **60,320** raw records refuse restore identically. Exact successful
records, error identity and all serialized post-operation records match.

Also implemented the additional [#42](https://github.com/dragginzgame/ic-jobs/issues/42)
audit reductions. Original recovery now consumes the initial frozen artifact
without a second copy; original/ still retains logs/hash records. Optimized
variants derive from that same frozen input. The fixture endpoint calls create
once, retaining create's authorization before any mutation or await.

Core/timers tests pass separately on Rust **1.88**, including both 20-test
consumer suites. All 16 stable-consumer tests, native/Wasm minimum checks,
release Wasm, harness compilation, warnings-denied native/Wasm Clippy and docs
pass. Actual recovery-wrapper fixtures pass on Bash 5 and genuine 3.2.57,
substituting optimizer/server effects: producer replacement cannot change the
frozen original/variants, and paths, hashes, logs and input/graph refusals remain
correct. ShellCheck and formatting/link/pin/snapshot checks pass.

Evidence: `target/qualification/start-capture-cleanup/`, with original source,
corpus, both probe outputs, failed fixture attempts and final focused logs.
The #42 proposed IC Build caller was rebased to retain the same single original;
its earlier capture experiments remain bound to their actual inputs. No live
PocketIC/trap execution or full local gate was selected. #43 remains open for
delivery/final-source native acceptance; #42 also retains its upstream adoption
obligations. Pending notes stay **0.9.0**; own metadata remains **0.8.7**. Incoming
Cargo selections, real index/refs and snapshot are preserved.

## Earlier preparation — three cleanup owners; upstream delivery pending

[#42](https://github.com/dragginzgame/ic-jobs/issues/42) records the three
requested cleanups. Locally removed duplicate standalone formatter assertions;
Shared owns those tests. Jobs retains actual Make/workflow/collector proof,
complete failure logs, incomplete collection and successful-log cleanup checks.
Updated the disposable publication fixture to use ordinary spaces; CR/LF
checkout success is outside the reviewed baseline. Focused Jobs fixtures and
genuine Bash 3.2 formatter/evidence qualification pass.

Prepared read-only publication admission in isolated Shared source at
`/tmp/jobs-extractions/shared-tooling`, based on committed 0.10.2
`626fc2bd3125195520fb870b92abfec11b01454d`, under
[Shared #165](https://github.com/dragginzgame/shared-tooling/issues/165).
The helper retains source/version/annotated-tag/remote/registry admission and
post-observation identity/source checks; upload remains consumer-owned.
Focused Bash 5/3.2, ShellCheck and the complete portable suite pass. The proposed
thin Jobs caller passes the existing disposable publication/release fixtures.

Prepared explicit captured-evidence retention in isolated IC Build source at
`/tmp/jobs-extractions/ic-build`, based on published 0.2.2
`e9be97fa356c14cb5431b879e304a0f8cc5077c3`, under
[Build #14](https://github.com/dragginzgame/ic-build/issues/14).
Ordinary Drop/close still clean temporary captures; explicit retention transfers
cleanup to the caller. Rust 1.88 staging/all-package tests, docs and documented
check gate pass. Proposed Jobs capture preserves the actual consumer Wasm and
all three O3/Os/Oz producer outputs byte-for-byte, with no live execution.

Evidence: `target/qualification/cleanup-three/`, including proposed callers,
producer/consumer fixture logs, retained failure attempts and patch artifacts.
Sibling dirty files remain untouched. Neither upstream draft is committed,
released or published. Jobs' active publication/recovery implementations remain
until canonical Shared delivery/export and published IC Build dependency/caller
admission permit their replacement. Recipe flags, filenames, evidence and
Testkit/live policy remain Jobs-owned; no competing engine is introduced.

Pending Jobs notes remain **0.9.0** alongside #41. Own metadata, incoming
manifest, real index and refs are unchanged. Incoming Host lock selections
advanced from 0.17.2 to 0.17.3 during this task and were not overwritten.
Earlier Rust proofs retain their original graph; they do not qualify that
independent new selection. Producer native macOS and
final-source Jobs acceptance remain pending; no release or live gate was run.

## Earlier preparation — Jobs 0.9.0 Timers/Metrics identity cut

Implemented [#41](https://github.com/dragginzgame/ic-jobs/issues/41) on delivered
**0.8.7** `0951f3083c961204bd6b40da97251d0812798031`. Incoming manifest/lock
already select published **Timers 0.22.0 / Metrics 0.9.0**. Official checksums,
non-yanked status and packaged Timers VCS source match public
`ffe175b2ae1b38719ec649286468b96a08a2f111`. Timers stays Rust **1.88**;
Metrics arithmetic stays Rust 1.85. Incoming Host 0.17.2 and other lock rows
are preserved exactly.

The Jobs optional adapter exposes Timers-owned types, so this changes its public
package/summary identity and requires **0.9.0**. Root and packaged READMEs require
direct Timers 0.22 alignment and Metrics 0.9 or the Timers summary re-export,
including overflow accessor/error migration. Job record shape, storage keys,
consumer bootstrap, restore admission and blocked effects remain unchanged;
no retained-data reset or compatibility reader is required. No product Rust
adaptation was needed. Job measurements remain a consumer-owned concern;
callback completion does not establish a job's external effect outcome.

Core/timers tests pass separately on Rust **1.88**, including both 20-test
consumer suites. All 16 stable-consumer tests (including frozen records/images),
native/Wasm minimum checks, release Wasm, harness compilation without live
execution, warnings-denied native/Wasm Clippy and docs/doctests pass. Canonical
lock-selected Testkit 0.37 CLI refresh and admission pass; older builds remain
retained and PocketIC was not provisioned/launched. Evidence:
`target/qualification/timers-022-adoption/`, with starting bytes and focused logs.

Own metadata remains **0.8.7**; pending notes are **0.9.0**. Incoming selections,
real index/refs and Shared 0.10 snapshot are preserved. No full local gate,
release or new live qualification was selected. #41 remains open for delivery
and configured final-source all-host native acceptance.

Delivered 0.8.7's [CI 38395094597](https://github.com/dragginzgame/ic-jobs/actions/runs/38395094597)
passes Linux with both macOS jobs running at inspection. #39/#40 are delivered
but remain open for their all-host acceptance; that run does not qualify the
pending Timers 0.22/Metrics 0.9 graph or live recovery.

## Earlier preparation — compatible 0.8.7 Shared 0.10 adoption

Implemented [#40](https://github.com/dragginzgame/ic-jobs/issues/40) from public
Shared **0.10.0** `ed9c26f015691859f67b50b512a94709dcdc0c6c` through a clean,
exact-source canonical preview/export. All **88 roots/files** are retained;
14 payloads update with no additions/retirement. Complete host/Cargo and IC
catalog records, including all digests, match the starting selections.
Both catalogs were already immutable root-layout snapshot payloads; no
consumer collision, redundant default copy or catalog exception exists.

Make, standalone installers, formatting, freshness and evidence defaults now
use the executing reviewed snapshot; Host and Rust overrides are independent.
Reviewed direct caller/fixture/CI inputs already select these same root paths.
Compiler/MSRV, product tools, cargo-edit, Testkit/PocketIC and application
dependencies retain their owners. No tool pin upgrades or installation deletion.
The same adoption retains release-lock failure status and LOC CR/LF path refusal.

Canonical `make install-tools` and complete offline admission pass. Jobs
formatting, snapshot/pin/link/release-adapter checks, disposable release and
publication fixtures, formatting/evidence/consumer fixtures and hooks pass.
Producer command/formatter/freshness/host/IC fixtures pass on Bash 5 and genuine
3.2.57, including child shells and actual evidence-action assertions using
synthetic owned installations. Snapshot export and prepared-cloc fixtures pass.
Evidence: `target/qualification/shared-010-adoption/`, including exact source,
preview/export, catalog comparison, actual setup/admission, fixture logs,
qualified-input hashes and preservation proof. Rust source and the incoming
Cargo graph are unchanged; the earlier #39 focused Rust results remain bound
to those same inputs. Incoming Testkit 0.37/Host 0.17 CLI adoption is separate.

Delivered **0.8.6** `e5bce9bf5304d7b3fd1700ceb005fa0c0e471b0d` passes
[CI 38379305759](https://github.com/dragginzgame/ic-jobs/actions/runs/38379305759)
on Linux and both macOS architectures, with actual complete gate steps
inspected. #34–#38 are accepted/closed and linked producer owners notified.
Those native results do not qualify this Shared 0.10 source or live recovery.

Pending notes remain **0.8.7** alongside #39; own metadata remains **0.8.6**.
The prepared restore cleanup, incoming manifest/lock, real index and refs are
preserved. #39/#40 remain open for delivery and configured final-source all-host
acceptance. No full local gate, release or new live workflow was selected.
Producer 0.10's Intel macOS acceptance was still queued at inspection.

## Earlier preparation — compatible 0.8.7 restore-state cleanup

Implemented [#39](https://github.com/dragginzgame/ic-jobs/issues/39) within
Job::restore: common attempt/start timestamp admission runs first, followed
by one selection of Running or Uncertain history rules. Prior-outcome
expectations are calculated only for Running. The removed Uncertain timestamp
predicate follows from retained `started_ns <= updated_ns` and
`last_outcome_ns == Some(updated_ns)` checks. Every other admission rule,
public API, stored record shape, typed refusal and blocked effect is preserved.

Rust **1.88** original/cleaned comparison produces identical serialized records
and typed restore refusals for **47,514** inputs: **790** accepted and **46,724**
rejected, including all **354** frozen valid records. The corpus varies active
states, history outcomes, timestamps and counter extremes across 18 distinct
active-history seeds. Independent source/probe builds and both observation files
are retained in `target/qualification/restore-state-cleanup/`.

Core/timers tests pass separately on Rust **1.88**, including both 20-test
consumer suites. All 16 stable-consumer tests, native/Wasm minimum checks,
release Wasm, harness compilation without execution, warnings-denied
native/Wasm Clippy, docs/doctests and formatting pass. These checks use the
incoming **Testkit 0.37 / Host 0.17** selection; they do not claim completed CLI
adoption or hosted/live acceptance of that separate dependency change.

Pending notes are **0.8.7**; own metadata remains **0.8.6**. Incoming manifest,
lock, prepared Shared snapshot, real index and refs are preserved. No new public
surface, helper framework or permanent implementation-shaped test was added.
No full local CI, release or new live qualification was selected. #39 remains
open for delivery and configured final-source all-host native acceptance.

## Earlier delivery — 0.8.6; native acceptance pending

Verified public/local main **0.8.6** at
`e5bce9bf5304d7b3fd1700ceb005fa0c0e471b0d`, with Shared **0.9.15**
`a9a8ea5ddea0db279d8c139ef5a951474c50f742` and the prepared Testkit/Host graph.
The checkout was clean at delivery inspection.
[Configured CI 38379305759](https://github.com/dragginzgame/ic-jobs/actions/runs/38379305759)
has Linux running and both macOS jobs queued. The newer delivery cancelled
0.8.5's run; neither cancelled run supplies complete native acceptance.
#34–#38 remain open for final-source all-host acceptance. Focused and live
qualification remain bound to their original tested inputs; no new full local
gate or live workflow was selected.

Further review found no actionable code defect or obsolete compatibility path.
No new implementation or release draft was added. Earlier preparation notes
below are historical; delivery replaces their pending version/source status.

## Earlier preparation — compatible 0.8.6 tooling/Host adoption

Adopted public Shared **0.9.15** at
`a9a8ea5ddea0db279d8c139ef5a951474c50f742` under
[#37](https://github.com/dragginzgame/ic-jobs/issues/37). Canonical clean-source
preview/export retains all 88 roots/files, updating three payloads without
retirement. Maintenance path admission rejects literal/resolved CR/LF before
CLI probes or state writes. Failed-attempt log recovery guidance is adopted;
Jobs does not select the optional gh-ci command. Tool policy/pins/installers
are unchanged, so common/release tool reinstallation is unnecessary.

[#38](https://github.com/dragginzgame/ic-jobs/issues/38) qualifies incoming
**Testkit 0.36.0 / Host 0.16.0**. Official Cargo publication/source identities
match the selected packages. Latest published Testkit requires Host ^0.16;
Host 0.17 is published but cannot replace that owned graph. Jobs adds no direct
Host dependency, bridge or compatibility path. Incoming **Memory 0.36.1 /
Metrics 0.8.4 / Timers 0.21.4** selections are preserved exactly.

Core and optional timers pass separately on Rust **1.88**, including both
20-test consumer suites. All 16 stable-consumer tests, native/Wasm minimum
checks, release Wasm, harness compilation without execution, warnings-denied
native/Wasm Clippy and docs/doctests pass. Canonical Testkit CLI refresh and
admission pass, retaining older builds without provisioning/launching PocketIC.
Shared maintenance fixtures pass under Bash 5 and genuine 3.2.57 including
child shells. Tool/formatter admission, formatting, snapshot/pin/link/adapter
checks and consumer-tooling fixtures pass.

Evidence: `target/qualification/shared-host-adoption/`, with starting bytes,
index/refs, canonical preview/export and focused logs. Pending notes are
**0.8.6**; own metadata remains **0.8.5**. No full local CI, release or new live
qualification was selected. #37/#38 remain open for maintainer delivery and
configured final-source Linux/Intel/Apple native acceptance. Delivered 0.8.5's
CI remains queued at inspection; #34/#35/#36 remain open at their source.

## Earlier delivery — 0.8.5; cleanup inspection complete

Delivered **0.8.5** at `4ebc1efea3160291f535c89df9db19202ccd08e4`
includes the Memory 0.36 consumer adaptation, Shared 0.9.13 adoption and
one-shot restore simplification. The checkout was clean at inspection.
[Configured native CI 38375977226](https://github.com/dragginzgame/ic-jobs/actions/runs/38375977226)
is queued; #34/#35/#36 remain open for final-source Linux and both macOS
architectures. The earlier 0.8.4 run was cancelled and supplies no complete
native acceptance. Existing focused evidence remains bound to its actual inputs;
no new live recovery qualification was selected.

Further review of core scheduling/configuration, restore admission, timers,
stable consumer management/receipts, canister dispatch and recovery helpers
found no additional concrete cleanup worth implementing. Similar flows retain
independent authorization, corruption admission, timer completion or uncertain
receipt semantics. No new compatibility path or obsolete declaration was found.
No product source or dependency changes, new tests or release draft were made.
Earlier preparation notes below are historical.

## Earlier preparation — compatible 0.8.5 restore cleanup

Implemented [#36](https://github.com/dragginzgame/ic-jobs/issues/36), removing
only the one-shot deadline predicate already implied by common first-occurrence
admission. The independent one-shot occurrence-origin guard and every other
restore/state/history rule remain intact. Public APIs, record encoding, refusal
identity/order and blocked effects are unchanged. This extends the same pending
**0.8.5** notes alongside the Shared adoption below.

Rust **1.88** baseline/cleaned comparison observes identical results for all
**41,154** inputs: all 354 frozen valid records plus one-shot deadline,
occurrence/origin combinations and counter/timestamp extremes. Exact successful
record serialization and typed restoration refusals match; original/fixed
observations and source inputs remain separate. No new implementation-shaped
guard test was added for this implied condition.

Focused checks pass: core/timers separately on Rust **1.88**, both 20-test
consumer suites, all 16 stable-consumer tests, native/Wasm minimum checks,
release Wasm, harness compilation without live execution, warnings-denied
native/Wasm Clippy, docs/doctests, formatting, snapshot/pins/links and whitespace.
Evidence: `target/qualification/once-restore-cleanup.dhboOj/`, including
input corpus, original/fixed observations, command logs and qualified hashes.
Own metadata stays **0.8.4**; the lock, prepared Shared snapshot, index and refs
are unchanged. #36 remains open for delivery and final-source configured native
acceptance. Full local CI and live recovery were not selected; delivered 0.8.4's
native gate is still queued at inspection and does not qualify this dirty source.

## Earlier Shared preparation — compatible 0.8.5

Adopted reviewed committed/public Shared **0.9.13**
`35cad65677ff989b5fb24a88d947fda37d2366e8` under
[#35](https://github.com/dragginzgame/ic-jobs/issues/35). A clean isolated checkout
excludes the sibling's uncommitted 0.9.14 work. Canonical preview/export retains
all **88 roots/files**, with eight updates and no additions or retirement.
AGENTS identifies the exact source; README records the required tool refresh.

Shared lock acquisition preserves filesystem diagnostics/status and refuses
existing lock paths without deleting another owner's lock. Common/selected
Cargo source acquisition retains its original failure status before build or
activation. Locked Cargo dependency-source preparation is now explicitly
authorized during development/validation, preserving selections and explicit
offline settings. This does not activate implicit fetching, executable setup,
runtime networking or release effects. Upstream CI branch-selection command
changes remain outside Jobs' selected executable set.

Canonical common/release Rust-tool refresh and actual complete offline tool
admission pass; prior generations remain retained. Producer host/IC/Rust
fixtures pass on Linux under Bash **5** and genuine **3.2.57**, including child
shells. Jobs release/publication, formatting/evidence, consumer refusal/input
drift and hook fixtures pass with their documented substitute effects. Formatter,
snapshot/pins/docs/release adapters, ShellCheck and whitespace checks pass.
No product Rust source, dependencies, pins, own metadata, real index or refs
changed; no full local gate or live PocketIC qualification was selected.

Pending compatible notes are **0.8.5**; package metadata remains **0.8.4**.
Local/public delivered 0.8.4 is
`555b4e3c6ba7fa869fbda9deda27a83b2f6cc110`; its queued native gate and #34
remain source-bound to that Memory adoption. Delivered 0.8.3 now passes
[all-host native CI 38365726174](https://github.com/dragginzgame/ic-jobs/actions/runs/38365726174)
at `c165c29d941b02d0da02e1ad5ad51cc33f07066a`, completing #31/#32/#33.
Neither run qualifies this pending Shared snapshot. Producer 0.9.13 CI is still
running at inspection; #35 retains delivery and final-source native acceptance.

Evidence: `target/qualification/shared-0913-adoption.YI0uef/` retains clean source,
canonical previews/export, starting identities, fixtures/setup/admission logs,
static checks and preservation proof. Earlier preparation notes below are
historical.

## Earlier preparation — compatible 0.8.4 Memory adoption

Implemented [#34](https://github.com/dragginzgame/ic-jobs/issues/34) using the
incoming published Memory **0.36.0** selection. Every declaration caller now
supplies an owned request vector; collision refusal uses the current declaration
error. Existing host composition, permanent keys, configuration, authority
checks, runtime persistence, codecs and Job::restore boundaries are preserved.
Memory remains a dev/unpublished-consumer dependency; Jobs gains no storage
owner, global registration, compatibility path or public API cut.

Core/timers tests pass separately on Rust **1.88**, including both **20-test**
consumer suites. All **16** stable-consumer tests pass, including frozen old
snapshots and complete stable-memory image reopening, retained allocation IDs
and identical Jobs cell bytes, blocked Running/Uncertain effects, duplicate-key
and foreign-authority refusal, and unchanged bytes on rejected management.
Native/Wasm minimum checks, release Wasm, harness compilation without execution,
warnings-denied native/Wasm Clippy, docs/doctests, formatting, snapshot/pins/links
and whitespace checks pass. No new tests mirror these mechanical API changes;
existing boundary and retained-data coverage supplies their proof.

Pending notes are **0.8.4**; own metadata remains **0.8.3**. Incoming Memory 0.36
and Metrics 0.8.2 selections, the real index and refs remain intact. Evidence:
`target/qualification/memory-036-adoption.z8ySpH/`, including original
inputs, fixed command logs and qualified hashes. #34 remains open for delivery
and configured final-source native acceptance; no full local gate or live
recovery was selected, and older hosted evidence remains at its original graph.

Upstream Memory's Linux native job fails only the inspected expected-diagnostic
fixture: both outputs still reject forbidden committed-capability construction
with E0599, but the stored compiler-help snippet differs. Reported at
[Memory #62](https://github.com/dragginzgame/ic-memory/issues/62#issuecomment-6141500964).
Its complete native acceptance is not claimed. Jobs' older delivered 0.8.3 CI
passes Linux with macOS running; it does not qualify this pending Memory graph.

## Earlier inspection — Memory 0.36 adoption required

Reviewed published **IC Memory 0.36.0** at committed/public source
`834fb71b09d84655dc64bd89b6bfb4c0c9b35c9d`. The official Cargo index reports
non-yanked and Rust **1.88.0**, with checksum matching the incoming Jobs lock;
the cached package VCS identity matches the commit. Incoming manifest/lock edits
already select Memory 0.36 and Metrics 0.8.2; both were preserved.

Memory now owns explicit request vectors and runtime ledger persistence, removing
constructor/global registration. Jobs already composes its host requests/pool
explicitly, but needs owned vectors and the current declaration-error name.
Focused Rust **1.88** checks reproduce the Wasm consumer's vector type mismatch
and seven integration-test errors at the old declaration calls/error import.
[#34](https://github.com/dragginzgame/ic-jobs/issues/34) owns the narrow consumer
adaptation and retained-image/reopen qualification. Upstream reports unchanged
stable keys/IDs/ledger encoding; Jobs acceptance is not yet established.

Evidence: `target/qualification/memory-036-review.0GcO3O/` retains compiler failures,
official index metadata and preservation hashes. No source repair, own version
change, full gate or live execution was performed. Upstream 0.36 native CI is
queued. This is a compatible Jobs patch candidate **0.8.4** when implemented;
Memory remains a dev/unpublished-consumer dependency, with no Jobs public API cut.

## Earlier delivery — 0.8.3 acceptance pending

Verified clean delivered **0.8.3** at local/public main
`c165c29d941b02d0da02e1ad5ad51cc33f07066a`. Local/public annotated tag object
`2492de7f68491d9bd6b265ee100b2727444d0a98` agrees and peels to that commit.
The release retains the Shared **0.9.10** snapshot and both prepared cleanups.
[CI 38365726174](https://github.com/dragginzgame/ic-jobs/actions/runs/38365726174)
is queued at inspection; #31/#32/#33 remain open for delivered-source native
acceptance. No unimplemented defect or competing open PR was found in the
current issue review; pending qualification alone does not require another fix.

The actual delivered graph selects **Host 0.15.2 / Memory 0.35.19 / Metrics 0.8.1 /
Testkit 0.35.1 / Timers 0.21.2**. Earlier focused and live results remain bound
to their original inputs; they do not qualify this changed lock or 0.8.3 source.
No new full local gate, Rust build or live workflow was selected by this review.

Delivered **0.8.2** now passes its configured native gate on Linux and both macOS
architectures at `f969d24681312ee659106c494f12c0e2cc94d5b5`,
[run 38358974128](https://github.com/dragginzgame/ic-jobs/actions/runs/38358974128).
That completes #30's named-codec acceptance. Existing v0.8.1 recovery evidence
remains at its immutable source; no later live qualification is inferred.

## Earlier preparation — compatible 0.8.3 cleanup

Implemented [#32](https://github.com/dragginzgame/ic-jobs/issues/32) and
[#33](https://github.com/dragginzgame/ic-jobs/issues/33) on delivered 0.8.2,
preserving the incoming Shared 0.9.10 snapshot and Host 0.15.2 lock edit.
Restore admission drops only predicates implied by the existing aggregate
counter, schedule-anchor and timestamp checks. Receipt completion loads the
whole snapshot once, shares its selected-entry projection with inspection,
then restores/transitions the selected job and validates the committed bytes.
Only the selected entry is cloned for projection; unrelated payloads need no
second decode. Public APIs, JSON/stable bytes, error order and blocked effects
remain unchanged; there is no compatibility reader or storage-policy change.

Compared baseline and cleaned restoration on **25,554** frozen/adversarial
records using Rust **1.88**: all observations match exactly (**3,615** admitted,
**21,585** aggregate refusals, **354** decode refusals). The corpus includes all
354 frozen valid old records. Two receipt refusal tests pass against the
pre-cleanup consumer and cleaned source, covering all seven states, wrong task,
unknown identity, receipt/clock precedence, unrelated corruption and unchanged
stable bytes. The stable consumer passes all **16** tests.

Focused checks pass: core/timers separately on Rust **1.88**, native/Wasm minimum
checks, both native consumers, release Wasm, harness compilation without live
execution, warnings-denied native/Wasm Clippy, rustdoc/doctests, formatting,
snapshot/pins/links and whitespace. Evidence stays at
`target/qualification/cleanup-restore-receipt.RFLvnJ/`, including baseline/fixed
observations, input corpus, original bytes and per-command logs. The initial
disposable comparison probe hit the documentation lint; its failed log is retained
and the documented probe passes. No production failure was suppressed.

Compatible pending notes are **0.8.3**; package metadata stays **0.8.2**.
Issues #32/#33 remain open for delivery and configured final-source native
acceptance. Full local CI, release gates and new live recovery were not selected;
older hosted results do not qualify this pending source. Real index/refs and
incoming lock selections remain untouched.

## Earlier adoption — Shared 0.9.10

Adopted reviewed committed/public Shared **0.9.10**
`7e55adecf6f0b196c050f104b37f8c69b6bf434e` under
[#31](https://github.com/dragginzgame/ic-jobs/issues/31). Exported through the
canonical owner from a clean isolated checkout, excluding dirty sibling work.
The snapshot retains all prior entrypoints and adds the mandatory
`rules/change-reporting.md`: **88 roots/files**, six payload updates, one
addition and no retirement. AGENTS records that exact source; README identifies
the adoption and preserves preparation guidance for older build policies.

Focused checks pass: snapshot bytes/modes, **244** documentation references
across **51** documents, dependency declarations/inheritance and standard release
Make adapters with substituted effects. The build policy, installers and pins
are unchanged; prepared tools need no rebuild. The upstream CI cache and Rust
fixture executables remain outside Jobs' selection. Jobs workflows, Rust **1.88**,
API/storage/restore policy and release authority are unchanged.

This governance/documentation-only adoption adds no behavior release note or
own version change. The incoming Host **0.15.2** lockfile edit remains intact and
requires separate runtime qualification. No local full gate, Rust build, live
dispatch, staging/commit/ref mutation, release or publication was performed.
Delivery and configured native acceptance on Linux and both macOS architectures
remain with #31; old 0.8.1 native/live evidence stays bound to its source.
Delivered 0.8.2's native CI now passes Linux/Apple Silicon, with Intel running at
inspection; #30 remains open. Upstream 0.9.10 Linux/lint pass, with macOS running.

Evidence: `target/qualification/shared-0910-adoption.I5P0y0/` retains starting dirty input bytes,
real index/ref/version/lock identities, clean canonical source, preview/export
and focused checks. Prior inspection and acceptance notes below are historical.

## Earlier inspection — Shared 0.9.10 and delivered acceptance

Verified delivered Jobs **0.8.2** at local/public main
`f969d24681312ee659106c494f12c0e2cc94d5b5`. Its configured native run
[38358974128](https://github.com/dragginzgame/ic-jobs/actions/runs/38358974128)
is still running at inspection; #30 remains open for final-source native
acceptance. The preparation notes below are historical. No new live dispatch
or rerun was selected for 0.8.2.

Reviewed clean committed/public Shared **0.9.10**,
`7e55adecf6f0b196c050f104b37f8c69b6bf434e`, against Jobs' recorded **0.9.7**.
The canonical preview updates six selected payloads without retiring files.
Preserving old entrypoints omits the newly referenced mandatory reporting rule:
an isolated export reproduces five broken documentation links. Explicitly add
`rules/change-reporting.md` during adoption; the isolated canonical export then
verifies **88 files** and all affected documentation links. The real Jobs
snapshot remains unchanged. The common Cargo build policy, installers and pins
are byte-identical, so this adoption would not itself require tool rebuilding.

Shared 0.9.8 adds exact-identity native CI caching and fixture timing at its
owner; Jobs would need a separate deliberate workflow/helper selection to use
that cache. Shared 0.9.9 fixes the standalone Rust fixture's invoking-directory
isolation and replaces fixture commits with borrowed history. These owner-only
executable changes are outside Jobs' current selection. Shared 0.9.10 adds
reporting rules separating semantic deletions, moves/renames and physical churn.
Its exact native run is still in progress; 0.9.9's native run succeeded.

Delivered **0.8.1** now has complete native acceptance on all three hosts at
[38288606583](https://github.com/dragginzgame/ic-jobs/actions/runs/38288606583)
and live acceptance at
[38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820),
both bound to `f9c0c0eb4e8363565be63f9434a0cf8b9940294b`. Downloaded the exact
three recovery artifacts and verified upload digests, safe archive paths,
committed manifests/lock/toolchain/snapshot/pins, wrapper and build-policy hashes,
Testkit 0.35.0 selection, each host's four Wasm hashes and all **84** live results.
Every scenario passes with zero failed/ignored tests. Native acceptance resolves
#26/#29; native plus live acceptance resolves #28. #27's earlier 0.9.4-only
qualification is superseded by accepted 0.9.7, not relabelled as a successful
historical run. These results do not qualify the subsequent 0.8.2 codec change.

Current inspection evidence, exact archives, source-bound reports, canonical
previews and isolated exports stay under
`target/qualification/shared-0910-review/`. No full local gate, source repair,
real snapshot adoption, release or publication was performed.

An incoming lockfile edit appeared during inspection, advancing Host artifacts,
fs and process from 0.15.1 to **0.15.2**. It was preserved without reselection.
The committed-source native/live results above do not qualify this dirty graph;
its dependency review and focused qualification remain separate work.

## Earlier 0.8.2 preparation — named retry codec agreement

Reproduced and repaired
[#30](https://github.com/dragginzgame/ic-jobs/issues/30). Released 0.8.1's retry
writer emits the public Serde structure name `RetryPolicy`, but its checked
private projection requested `RetryPolicyFields` while decoding. JSON ignores
names, so existing frozen JSON proofs did not detect that named-structure formats
could not round-trip the emitted policy. A disposable canonical `serde_test`
probe fails on released source with those two conflicting names, using Rust
**1.88**, Serde **1.0.229** and `serde_test` **1.0.177**.

The private projection now retains the existing public wire name while calling
the same checked constructor. No public API, numeric field/layout, JSON bytes,
retry/state/error policy, restore boundary, storage key or retained installation
changes. One dev-only canonical test dependency covers named-token writer/reader
agreement for NONE-equivalent, ordinary and maximum numeric values. All existing
package selections are preserved; only `serde_test` 1.0.177 is added. Compatible
pending notes are **0.8.2**; own metadata stays **0.8.1**, with no release effects.

Focused Linux checks pass: core and optional timers separately on Rust **1.88**,
actual Make job route, both consumers, native/Wasm minimum checks, **14** stable
consumer tests, release Wasm, harness compilation without execution, warnings-
denied native/Wasm Clippy, docs/doctests, formatter, pins/links and snapshot.
All **354** frozen old records still re-encode exactly and the old complete memory
image reopens. Reproduction/fixed inputs and lock are kept separately at
`target/qualification/retry-codec.y0y7g97c/`, including probe source/lock,
released policy bytes, baseline failure, fixed checks, dependency-preservation
proof and qualified hashes. No compatibility reader, unchecked constructor,
new schema/reset, own version/index/ref mutation or full local gate was added.
#30 remains open for delivery and final-source native acceptance.

## Delivered 0.8.1 — Linux live evidence inspected

The explicitly selected
[recovery run 38289182820](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820)
passes Linux in **10 minutes 5 seconds**. Inspected all **28** live results:
seven source-declared scenarios for each original/O3/Os/Oz Wasm. Verified the
exact uploaded artifact **11765006978**, digest
`6790dd49d0a413fc06bf331f9aafeb7b44c9992163626bbd57fc1bf14bf3e828`,
all consumer Wasm hashes, source `f9c0c0eb4e8363565be63f9434a0cf8b9940294b`,
v0.8.1 tag/run/attempt, committed Cargo files/lock/toolchain/snapshot, wrapper
hash, Testkit **0.35.0** selection and the actual Shared build-policy digest.
The archive reports `live-outcome=success`; every scenario passes with zero
failed/ignored tests. Artifact extraction validates paths and rejects symlinks.

Apple Silicon recovery is now running and Intel remains queued at the latest
observation. The separate native CI passes Linux; both macOS jobs remain queued.
No macOS acceptance or all-host completion is claimed. Native/live observations
remain at their actual source/graph; this v0.8.1 run does not qualify pending #30.
Linux evidence/report, raw job log and exact archive are retained under
`target/qualification/delivered-081.ifzmznb8/`. No rerun or second dispatch was
performed. Continue inspecting the existing run's macOS evidence when available;
#26/#27/#28/#29 remain open for their outstanding source-bound acceptance.

## Delivered 0.8.1 — current qualification

Verified local and public annotated **v0.8.1** at
`f9c0c0eb4e8363565be63f9434a0cf8b9940294b`, tag object
`f45ea24c5b94e6d73de624e73ca4e6da2bd28388`; public main matches.
The maintainer delivered the reviewed Shared **0.9.7** snapshot at
`4d10a06d50e44d302f00d5b7b706cac5f9b0127c`. Package metadata and dated notes
now select **0.8.1**. Compared with delivered 0.8.0, all library/application/
harness sources, frozen retained-byte fixtures, runtime tool pins and recovery
workflow/wrapper are unchanged. External package selections are identical;
only the three own lock versions advance.

The exact delivered graph is **Host 0.15.1 / Memory 0.35.19 / Metrics 0.8.1 /
Testkit 0.35.0 / Timers 0.21.0**. Correction: earlier current-graph prose listed
Metrics 0.8.0; both committed 0.8.0 and 0.8.1 lockfiles select **0.8.1**.
Their lock hashes/source identities remain authoritative. This corrects version
prose, not dependency selection or a claim of newly recovered historical logs.

[Native 0.8.1 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38288606583)
is running on Linux; both macOS jobs are queued at inspection. The 0.8.0 run was
superseded under the mandatory latest-ref policy and cannot establish complete
native acceptance. Native qualification for the delivered configuration/counter
changes and Shared adoption still follows the retained latest source.

The maintainer explicitly selected **v0.8.1 recovery**. Dispatched the existing
[Canister recovery workflow](https://github.com/dragginzgame/ic-jobs/actions/runs/38289182820)
once, bound to the verified tag/commit. It qualifies seven scenarios across
original/O3/Os/Oz Wasm on Linux and Intel/Apple Silicon macOS, with explicit
Testkit/CLI/server setup and evidence retention. The hosted run is queued at
initial observation; dispatch is not live acceptance. Prior v0.7.1 evidence
stays at its original source/graph. No workflow rerun, second dispatch, release
or publication was performed by this continuation.

Evidence: `target/qualification/delivered-081.ifzmznb8/` retains public tag/main,
workflow/native job observations, dispatch request time and selected run identity,
committed graph comparison, watch output and preservation. Read-only watch and
artifact inspection are selected for this run. The following sections retain
preparation observations from before 0.8.1 delivery, with their historical own
versions and statuses. No additional code change or compiler/test gate was
selected merely for this delivery inspection.

## Delivered 0.8.1 — Shared 0.9.7 preparation evidence

Adopted reviewed committed/public Shared **0.9.7**
`4d10a06d50e44d302f00d5b7b706cac5f9b0127c` through the canonical clean-source
preview/export. All **87 roots/files** remain; four payloads update, with no
addition or retirement. AGENTS records the exact source. Baseline bytes,
common pins, library/storage API, Rust **1.88**, direct delivery and the
application lock remain unchanged. This compatible contributor-tooling patch
uses one undated **0.8.1** entry; own package versions remain **0.8.0**.
[#29](https://github.com/dragginzgame/ic-jobs/issues/29) owns delivery/native
acceptance, separately from the earlier Shared 0.9.4 issue.

The Cargo-tool owner now rejects non-boolean `net.offline` before installation
or admission, preserving explicit environment overrides and Cargo config-file
precedence. Refreshed actual common/release Rust selections with the new helper
policy digest; previous physical builds and the old common set remain retained.
Actual tool admission passes, with pins unchanged. The selected Testkit CLI is
prepared separately before live qualification; this batch installs neither that
CLI nor a PocketIC server. Artifact discovery/PR fixes are outside Jobs' selected
upload-only/direct adapters; no unused helpers, policy flags or workflow changes
are introduced. The public repository description still matches the library.

Focused Linux tooling checks pass: canonical Cargo-tool fixture (synthetic source/
compiler/build effects, real parsing/identity/retention/refusal), actual common/
release/format admission, snapshot/pins/Make adapters, disposable release and
publication recovery, formatting/evidence, consumer-tool refusal, real formatting
hooks, formatter and links. Malformed string/number/array/table offline values
refuse before store/build/selection effects; valid booleans, empty config, config
precedence, environment overrides and prior-selection retention pass. Repeat
canonical preview is **87 KEEP**. ShellCheck passes with required source files
resolved. Initial failures remain: the synthetic compiler refused the Jobs
working directory; a neutral fixture directory passes, and the initial ShellCheck
call omitted external source following. No vendored source was patched to make
those test invocations pass. Native macOS/Bash 3.2 acceptance remains pending.

Focused package qualification also passes on the exact delivered graph: Rust
**1.88** core and optional timers tests separately, actual Make job route, both
application consumers, native/Wasm minimum checks, **14** stable-consumer tests,
release Wasm, native harness compilation without execution, warnings-denied
native/Wasm Clippy and rustdoc/doctests. Inputs/fixture bytes are checkpointed.
Graph: **Host 0.15.1 / Memory 0.35.19 / Metrics 0.8.0 / Testkit 0.35.0 /
Timers 0.21.0**. The job implementation, all retained byte fixtures, dependency
selections and release metadata are preserved; older graph evidence stays bound
to its old source. Earlier ignored evidence directories are unavailable in this
session; new logs are not presented as recovered historical artifacts.

The maintainer delivered **0.8.0** at
`820d65e9eefd29e775857175aa72972b762a0caa`, including checked configuration,
Timers 0.21 and Testkit 0.35. Updated #26/#27/#28 to that delivery and their
remaining hosted qualification, preserving historical proposals/evidence.
[Latest Jobs CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38285177233)
passes Linux; both macOS jobs remain queued at inspection.
[Exact Shared CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38283181603)
is queued. The older 0.7.5 run passed Linux/Intel but Apple Silicon was cancelled
when superseded. That is the mandatory latest-source workflow/ref policy;
[#3](https://github.com/dragginzgame/ic-jobs/issues/3) records an earlier policy,
not current cancellation authority. Cancelled checks do not qualify their source.
No concurrency change, workflow rerun/dispatch or new live execution was made.

Evidence: `target/qualification/shared-097.ayrf6evg/`: source-bound public/CI/
issue observations, before/lock identities, canonical export/previews, actual
setup/admission and retained-tool paths, upstream fixture and failed invocations,
per-target tooling/package logs, qualified lock/input checkpoints and preservation.
Final-source Linux/Intel/Apple Silicon acceptance for #29 still needs delivery;
#26/#27/#28 remain open for their retained latest native result. New live evidence
remains unselected. No full local CI, own package bump, staging, commit, ref change,
release, push, publication or sibling edit was performed.

## Earlier upstream review — delivered with 0.8.0

Verified current public tagged Testkit **0.35.0** at
`ad5bdfa5221ec25743fbc56e028fb958f2a6a558` and Timers **0.21.0** at
`a6b518722438d2f7e2dccd7ccfef07debc35b5b3`. Both registry records are
non-yanked and declare Rust **1.88.0**. Incoming Cargo.toml/Cargo.lock already
select those releases; preserved them without dependency reselection. The
selected graph now contains **Host 0.15.1 / Memory 0.35.19 / Metrics 0.8.0 /
Testkit 0.35.0 / Timers 0.21.0**. Earlier qualification below remains bound to
its original graph and does not qualify these new inputs.

Testkit's generic artifact/Cargo/Wasm APIs moved to IC Build and its unused
Host-tools re-export is gone. Jobs' harness uses neither surface; its PocketIC
imports and selected server CLI contract remain supported, so no IC Build or
direct Host-tools dependency is needed. Timers' reconciliation signatures stay
supported, but first creation now publishes the consumer slot only after the
requested state succeeds; failure cleans up new authority/captures and leaves
an empty slot for retry's supplied callback. Existing authority remains on later
failure. The public measurement summary selects Metrics **0.8**. Aligned both
package/root guides and pending **0.8.0** notes with the incoming family and
semantics; the checked Jobs configuration/storage contract is unchanged.

Focused current-graph Linux checks pass after explicit `cargo fetch --locked`:
Rust **1.88** core/optional timers native/Wasm checks, both application-consumer
suites, all **14** stable-consumer tests (including release-produced memory
reopen), release consumer Wasm, native recovery harness compilation only and
warnings-denied native/Wasm consumer/harness Clippy. The common runner checkpoints
all five selected manifest/lock/toolchain/snapshot/Make inputs. All runtime/test
source bytes were preserved through the check; published source and registry
observations are separately retained. New live execution and final-source
macOS qualification remain pending. No source changes or new tests were made
for this upstream review.

Shared's public main is **0.9.7** at
`4d10a06d50e44d302f00d5b7b706cac5f9b0127c`. A clean committed-source canonical
preview keeps the **87-file** selection, with four updates and no additions or
retirement: release/setup/host documentation and the Cargo-tool build helper.
Its stricter boolean `net.offline` admission changes the build-policy digest.
Actual adoption would require explicit Rust-tool setup to select the new policy;
old builds remain retained. PR identity hardening is outside Jobs' selected
direct-release adapter. Artifact-visibility discovery is relevant to upload
readback workflows; Jobs only uploads evidence and has no such download call.
No extra helper or PR policy is needed here. The real snapshot remains reviewed
**0.9.4**; this was a preview, not adoption or tool installation.

Evidence: `target/qualification/upstream-review.xejqfsaq/`: incoming lock and
preservation hashes, public main/tag and registry responses, isolated committed
Shared source, canonical preview and focused logs/checkpoints. Initial preview
refused the isolated clone's local remote; the isolated remote was set to the
manifest's public identity and repeat preview passed. Real HEAD/index/refs, own
**0.7.5** versions and sibling files remain unchanged. No complete local gate,
release/publication, live workflow dispatch or executable installation was run.

## Delivered 0.8.0 — configuration preparation and original Timers 0.20 evidence

The maintainer explicitly selected implementation of
[#28](https://github.com/dragginzgame/ic-jobs/issues/28). Recurring intervals now
use standard `NonZeroU64`; absolute deadlines remain u64, including zero.
Retry configuration is immutable and checked once by `RetryPolicy::new`;
deserialization invokes that same constructor. Accessors expose its numeric
values, and `NONE` retains the previous no-retry policy. `Job::new` is infallible
because its inputs are already checked. Update interval/retry literals, remove
constructor result handling and update exhaustive error matches as documented
in the root/package guides. Pending **0.8.0** is required by this source API cut
and the exposed Timers dependency family.

Adopts registry **Timers 0.20.0**, matching non-yanked registry checksum
`bab1d6a188038e5e70dd447f3062df9e83b1702bc1f014ac2687e6cd00b4e30f`
and tagged source `01031c912058089756aed87fafa4918476c12bd4`. Jobs delegates to
the unchanged reconciliation helper; direct timer registration, storage,
payloads, handler dispatch, clocks and lifecycle remain consumer-owned. Final
applications must align all direct Timers dependencies to one 0.20 identity.
Incoming **Memory 0.35.18 / Testkit 0.34.7** selections were preserved, alongside
**Host 0.14.10 / Metrics 0.7.3**. Explicit locked fetch prepared that exact graph.
Own workspace versions remain **0.7.5**; Shared **0.9.4** and its 87-file snapshot
remain unchanged.

Valid retained record layout and the stable consumer key/codec are unchanged.
Malformed local configuration is refused during decoding; every decoded
aggregate still passes `Job::restore` for history/timing/counter checks. Runtime
errors for representable inputs, failed-transition immutability and blocked
Running/Uncertain effects remain. The raw consumer API rejects zero intervals
with typed `InvalidInput` without commit, after storage/identity admission.
Qualification-only corruption mutates encoded bytes; it does not provide an
unchecked configuration constructor. The live recovery expectation now uses
the typed `InvalidStorage` decode failure.

Independent archived **0.7.5** source
`060fb9d226389bab702132ba23f42bd0b887a690` produced **354** frozen mixed-history
records before implementation. Every record restores and re-encodes exactly.
The stable consumer reopens its frozen snapshot and complete old stable-memory
image, including allocator/cell headers, without installation/reset. Existing
Jobs bytes, effect identities, payloads and blocked effects remain retained while
healthy work proceeds. An initial assertion that the whole backing image stays
unchanged was too strong: independently rerunning the old producer confirms
MemoryRuntime already republishes its ledger on reopen (592 backing-byte changes
with Jobs cell unchanged). The corrected proof admits that established owner
behavior while requiring retained allocation/cell data. Sparse hex runs encode
all old-image bytes losslessly; they are fixture data, not a product codec.

Focused Linux qualification passes: Rust **1.88** core and optional timers
separately, both native/Wasm minimum checks, actual Make job route, both application
consumer suites, **14** stable-consumer tests, release Wasm, native harness
compilation without execution, warnings-denied native/Wasm Clippy, rustdoc and
five doctests (including two compile-fail checks), formatting, links, declaration/
snapshot and release-Make checks. Final consumer coverage was requalified after
adding the old-memory image; unchanged core inputs retain their earlier passing
evidence. Checkpoints cover manifests/lock/toolchain/snapshot/Make/Rust sources,
compiled package README and frozen fixture bytes.

Evidence: `target/qualification/jobs-080.i92pbmwe/`: archived producer/generator
source and isolated target, producer logs, original record/snapshot/image data,
failed test attempts, `qualified-Cargo.lock`, input checkpoints, both focused
runner directories and preservation records. Initial failures were test
expectations: blocked `start` retains `NotPending`, and memory reopen republishes
its ledger. No failed production behavior was hidden or old evidence relabelled.

Delivery and exact final-source native Linux/Intel/Apple Silicon acceptance for
#28 remain pending. New live PocketIC evidence remains unselected; the existing
v0.7.1 observations stay bound to their old source/graph. No full local CI,
workflow dispatch, package bump, staging, commit, ref update, release, push,
publication or sibling edit was performed.

## Delivered 0.7.5 — current acceptance observation

The maintainer delivered **0.7.5** at
`060fb9d226389bab702132ba23f42bd0b887a690`; own package metadata and dated release
notes now match. [CI run 38271757772](https://github.com/dragginzgame/ic-jobs/actions/runs/38271757772)
has successful Linux qualification; Apple Silicon and Intel remain queued at
inspection. [#26](https://github.com/dragginzgame/ic-jobs/issues/26) and
[#27](https://github.com/dragginzgame/ic-jobs/issues/27) remain open for their exact
all-host acceptance. The following sections retain the preparation evidence
from before delivery; their then-current versions/graphs are historical.

## Delivered 0.7.5 — preparation evidence and Shared 0.9.4

The compatible batch contains the completed
[#26](https://github.com/dragginzgame/ic-jobs/issues/26) counter-validation repair
and reviewed Shared **0.9.4** adoption under
[#27](https://github.com/dragginzgame/ic-jobs/issues/27). Canonical clean-source
preview/export selects committed/public
`4532b66c1b43bab331978523ee97d29fe11a2f50`, updates nine payloads and retains
all **87 roots/files** without additions or retirement. Baseline bytes, common
tool pins, direct-release policy and library/storage/Rust **1.88** remain.
AGENTS and the one undated **0.7.5** entry align. The proposed Timers 0.20 / Jobs
0.8 configuration cut remains separate from this compatible batch.

Shared now consolidates host/IC installation cleanup, releasing shared locks
even if checkout cleanup fails and retaining the original installation status.
Asset downloads retry transient errors within fixed limits; permanent errors
and checksum failures remain terminal. Checksum manifests reject NUL before
hashing payloads; installers reject supplied/resolved CR/LF consumer paths
before setup/configuration. Snapshot and release identities require exactly
40 or 64 hex characters. Source owners:
[Shared #138](https://github.com/dragginzgame/shared-tooling/issues/138) and
[Shared #133](https://github.com/dragginzgame/shared-tooling/issues/133).

Focused actual Jobs admission, pins/snapshot/release adapters, disposable
release/publication recovery, formatting hook/evidence retention, consumer
CLI/Wasm refusal and input drift checks pass. Exact-source upstream host/IC/
Rust installers and store cleanup, checksum/path/snapshot, release-runner and
download fixtures pass under Bash 5 and genuine **Bash 3.2.57**, including
child shells. Download tests exercise actual curl against a loopback server
with HTTP transport substituted: transient recovery, retry exhaustion,
permanent failure and checksum refusal are verified. No real tool installation
or release/publication is dispatched. Both actual repeat previews show 87 KEEP.
ShellCheck passes with the optional PR source resolved from the exact reviewed
checkout for analysis; earlier source-path failures stay retained. This does not
vendor that helper or select PR release policy.

Fresh Rust **1.88** core and optional timers tests pass separately. The actual
Make job-test route, native/Wasm minimum checks, stable consumer tests/release
Wasm, harness compilation without execution, warnings-denied native/Wasm
Clippy, rustdoc/doctests and formatting pass. Fixed input checkpoints cover all
28 selected manifests/lock/toolchain/snapshot/Make/tool pins/Rust files. Qualified
graph: **Host 0.14.10 / Memory 0.35.17 / Metrics 0.7.3 / Testkit 0.34.6 /
Timers 0.19.3**, including incoming aws-lc-rs **1.18.2**, aws-lc-sys **0.46.0**
and Syn **3.0.9**. Cache preparation uses `cargo fetch --locked`; incoming
selection is preserved. Earlier graph evidence stays separately attributed.

Evidence: `target/qualification/shared-094.lcrxn9pu/`, including before inputs,
canonical preview/export, per-command logs/results, both shell previews,
`qualified-Cargo.lock`, `qualified-inputs.json` and preservation. Original
isolated counter reproductions/fixed checks and delivered 0.7.4 all-host
acceptance remain at `target/qualification/counter-restore.t2x62tuy/`.
Own package versions stay **0.7.4**; real HEAD/index/refs and pending Rust
changes are preserved. No full local CI, real release/publication, new live
PocketIC qualification or workflow dispatch is selected.

[#26](https://github.com/dragginzgame/ic-jobs/issues/26) and
[#27](https://github.com/dragginzgame/ic-jobs/issues/27) remain open for delivery
and exact final-source native acceptance. Older #21/#23/#24/#25 are completed
at their recorded native/live sources. Exact Shared
[CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38269332848)
is queued at inspection; Linux/Bash fixtures do not establish native macOS.
User delivery selects the configured complete gate; new live execution remains
separate. No automatic release or publication is authorized by finishing notes.

## Delivered 0.7.5 — restore dispatch-count validation

Reproduced and repaired [#26](https://github.com/dragginzgame/ic-jobs/issues/26):
`Job::restore` admitted a first occurrence claiming more dispatches than allocated
attempts, and an extra dispatch beyond a completed occurrence's retry budget after
rescheduling. It now checks the maximum dispatch count implied by the total
occurrence history and immutable retry budget, using u128 arithmetic for large
valid records. Schedule rebasing does not reset dispatch history. The existing
`InvalidRecord` error, configuration/error precedence, public transitions,
stored fields and blocked Running/Uncertain effects remain unchanged.

Coverage rejects contradictory counts in every attempted state and in paused/
rescheduled work, while retaining full-budget successful histories and maximum
counter exhaustion. Existing exhaustion fixtures now include a feasible prior
occurrence count instead of an impossible one-shot history. The stable consumer
regression proves corrupt Running/Uncertain counts block the whole queue before
healthy work dispatches, and preserve committed bytes through refused reads,
dispatch and reopen. No new scheduling/storage policy or consumer lifecycle
ownership is introduced.

Fresh qualification passes on the fixed incoming graph: **Host 0.14.9 / Memory
0.35.17 / Metrics 0.7.3 / Testkit 0.34.5 / Timers 0.19.3**. Core and optional
timers tests pass separately on Rust **1.88**, also using an independent fresh
build directory. The actual Make test route, native/Wasm minimum checks, stable
consumer tests and release Wasm, native harness compilation, warnings-denied
native/Wasm Clippy, rustdoc/doctests and formatting pass. The shared runner
checkpoints all workspace manifests, lock, toolchain, snapshot, Make/tool pins
and Rust sources; final hashes agree with all 28 selected inputs.

Evidence: `target/qualification/counter-restore.t2x62tuy/`. Its archived delivered
baseline at `6f3006a08a5b111433620c7147ce809fc2fbe9c3` uses the new regressions and
same selected graph; both cases fail with unexpected `Ok(Job)` in a fresh
isolated target. The fixed source passes separately. An initial reproduction
sharing the normal target unexpectedly passed; retained as inconclusive rather
than treated as baseline proof. Earlier temporary logs became unavailable after
the session restart, so this batch recreates the proofs instead of relabeling
those missing artifacts. Preserve `isolated-baseline-*`, `isolated-fixed-*`,
`focused-validation.log`, `qualified-Cargo.lock`, `qualified-inputs.json` and
preservation records. Incoming lock updates are preserved; own package version
stays **0.7.4**, compatible pending notes **0.7.5**, with no API/format/MSRV break.
No full local gate, release/publication, new PocketIC execution or dispatch.

## Delivered 0.7.4 — all-host acceptance complete

[CI run 38247839718](https://github.com/dragginzgame/ic-jobs/actions/runs/38247839718)
succeeds on Linux, Apple Silicon and Intel macOS at exact delivered source
`6f3006a08a5b111433620c7147ce809fc2fbe9c3`. Inspected all job/step outcomes and
actual logs for the complete `make ci` gate, selected 87-file Shared **0.9.0**
revision `aec7a149f581a42b9531fc747b46024fc0314fcf`, and native Rust 1.88
identities. Intel completes its whole job in **27 minutes 38 seconds** under the
delivered 45-minute budget. Report: `native-074-acceptance.json` and associated
raw logs in the same evidence directory. Empty per-job CLI log views were
recovered through the direct job-log API; acceptance uses the actual contents.

This completes [#21](https://github.com/dragginzgame/ic-jobs/issues/21) and
[#25](https://github.com/dragginzgame/ic-jobs/issues/25). It also supplies the
remaining native acceptance for [#23](https://github.com/dragginzgame/ic-jobs/issues/23)
and [#24](https://github.com/dragginzgame/ic-jobs/issues/24), alongside their
previously inspected 84 v0.7.1 live results. Git comparison confirms the core,
queue and canister implementation is identical between that live source and
delivered 0.7.4. Closed those four accepted issues with their distinct source/
graph evidence. Native 0.7.4 success does not qualify a new live dependency graph
or pending #26; that fix remains open for delivery/native acceptance.

## Earlier Shared 0.9.0 adoption — delivered with Jobs 0.7.4

Canonically adopted committed/public Shared **0.9.0**
`aec7a149f581a42b9531fc747b46024fc0314fcf` from a clean isolated checkout.
The committed VERSION is 0.9.0. Three payloads update: consuming guidance,
snapshot verifier and workspace LOC helper. All **87 roots/payloads** remain;
none are added or retired. Baseline bytes, tool pins and consumer Make recipes
are unchanged; AGENTS records the exact reviewed source. Refresh and integrity
verification now use the same syntax reader, also used by the central fleet
reporter. NUL bytes refuse before record splitting; malformed input emits no
admitted records. `--read-manifest FILE` establishes syntax only, with payload
integrity, source attribution and prior ownership still separate. Existing v1
snapshots require no conversion. Owner:
[Shared #133](https://github.com/dragginzgame/shared-tooling/issues/133).

Focused Jobs checks pass: actual common/release/format tool admission, snapshot,
pins, release Make adapters, disposable release/publication recovery,
formatting/evidence retention, consumer CLI/Wasm refusal and validation-input
drift, real formatting-hook preservation and formatting. No cache preparation
or tool installation is needed for this batch. Exact-source upstream snapshot
admission/export/conflict/retry, workspace LOC and manifest-reader/tooling LOC
integration fixtures pass under Bash 5 and genuine **Bash 3.2.57**, including
child shells. Both actual repeat previews retain all 87 payloads. ShellCheck,
documentation links and whitespace pass.

The adopted `cloc.sh --json` runs against Jobs in both shells and produces
identical structured results. Numeric totals match the existing `make cloc`
human projection: 1,473 runtime lines, 3,549 test lines, 72 test functions.
Fleet reporting stays central; no sibling reporter, fixture or consumer Make
flag is added. Evidence: `/tmp/ic-jobs-shared-090-m2hkqgiz/` (before/preservation
identities, export/preview logs, per-command results and LOC agreement report).

Jobs **0.7.3** is now delivered at
`44a28d79ae8a5247a1b3cbd44a7f81fc6ffd4157`; its finalized notes are preserved.
This compatible contributor-only batch starts pending **0.7.4** notes. Library
API/runtime/storage, Rust **1.88**, own package versions, Cargo.lock, native
45-minute budget, transition coverage, Make route, HEAD/index/refs remain
unchanged. Earlier Rust/live qualification remains bound to its saved inputs.
No new package Rust build/tests, full local CI, live qualification, dispatch,
release/publication or sibling edit is selected by this adoption.

Delivered Jobs [native CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38243360179)
and exact-source Shared [CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38246537047)
are queued at inspection. Local Linux Bash execution does not qualify native
macOS. [#21](https://github.com/dragginzgame/ic-jobs/issues/21) stays open for
delivery and final-source native acceptance; the delivered 45-minute repair
under [#25](https://github.com/dragginzgame/ic-jobs/issues/25) still awaits its
source-bound native result. The previously inspected 84 v0.7.1 live results
remain separate from this tooling source.

## Earlier Shared 0.8.3 adoption — delivered with Jobs 0.7.3

Canonically adopted committed Shared **0.8.3** at
`8cca7e67b03ce6e701b6e7dcec2e9753eb98337e` from a clean isolated source.
Preview/export updates nine shared payloads and adds the Cargo tool freshness
guide: **87 files**, all 86 previous roots retained, none retired. The baseline
bytes and tool pins are unchanged; AGENTS records the reviewed source. This
adopts formatter admission against the selected Rust catalog, refusal of
redirected observed-build evidence, complete Make installer/LOC companions and
quiet successful PCRE2 checks. Canonical refresh now replaces each payload
atomically, preserving old bytes on an interrupted copy and rechecking local
edits/index before replacement. Source owners:
[Shared #135](https://github.com/dragginzgame/shared-tooling/issues/135),
[Shared #136](https://github.com/dragginzgame/shared-tooling/issues/136),
[Shared #131](https://github.com/dragginzgame/shared-tooling/issues/131).

Focused Jobs checks pass: actual common/release/format tool admission, snapshot,
pins, Make release adapters, formatting/evidence retention, consumer CLI/Wasm
refusal and validation-input drift, disposable release/publication recovery,
real formatting-hook preservation and formatting. ShellCheck passes for the
changed installers. Exact-source upstream Rust/host installer, Make consumer,
snapshot interruption/conflict/retry and Cargo freshness fixtures pass under
Bash 5 and genuine **Bash 3.2.57**, including child shells. The freshness fixture
uses registry substitutes; this batch does not repeat the preceding real online
17-tool inspection or install tools. Both actual repeat previews show 87 KEEP.

The first Jobs release fixture refused offline cargo-edit metadata because the
incoming **IC Memory 0.35.16** crate was absent from the cache. Retained at
`/tmp/jobs-release-tooling.1tnPj9/`; preparation with `cargo fetch --locked`
downloaded that selected crate, preserved the lock and allowed the retry to pass.
No source repair or dependency reselection was needed. Fixture gates,
installations, registry observations and upload effects are substitutes in
local disposable repositories; this is not a real release or publication.
Evidence: `/tmp/ic-jobs-shared-083-va8ublbw/` (before/preservation records,
preview/export logs, per-command results and both attempts).

The incoming graph is **Host 0.14.7 / Memory 0.35.16 / Metrics 0.7.2 /
Testkit 0.34.4 / Timers 0.19.1**; it is preserved byte-for-byte. Earlier Rust
qualification below remains bound to its saved graph. This tooling-only batch
runs no new Rust package build/tests, full local CI, live PocketIC or hosted
dispatch. Jobs' published API, storage, own 0.7.2 versions and Rust **1.88** floor
are unchanged; compatible notes remain **0.7.3**. Pending transition coverage,
45-minute native workflow budget and Make test route are preserved, along with
the real index/HEAD. No shared function/method/type is removed.

Exact-source Shared [CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38238988308)
is queued at inspection; Linux Bash checks do not establish native macOS
acceptance. Jobs [#21](https://github.com/dragginzgame/ic-jobs/issues/21) stays open
for delivery and final-source native qualification. Delivered 0.7.2 native Linux
and Apple Silicon pass; Intel's 30-minute timeout is tracked by
[#25](https://github.com/dragginzgame/ic-jobs/issues/25), whose pending budget fix
still awaits delivery/acceptance. The previously verified 84 v0.7.1 live results
remain bound to their exact source and graph.

## Earlier Shared 0.8.2 inspection — before 0.8.3 adoption

Verified committed/public Shared **0.8.2** at
`aaf75a37717357dd62f4b0b15001eaa3ba1629cf`. It adds the read-only online Cargo
tool freshness checker and its task; committed installers, pins and baseline
are unchanged from 0.8.1. The isolated canonical preview updates three task/
catalog files. Adding `tasks/cargo-tool-versions.md` as an entrypoint completes
that catalog's new guide link; preview then shows one addition, with all 86
previous payloads retained. No export is applied by this inspection.

Ran the exact-source checker against Jobs, its common selection receipts and
user-level Cargo receipts. All **17 reported tools** are current with
non-yanked stable releases; the three common pins match their selected versions.
This is version freshness, not executable byte/build admission, and does not
inventory consumer-specific selected profiles such as the Testkit CLI. Registry
responses/statuses and JSON report are retained. No pins or installations change.
Evidence: `/tmp/ic-jobs-shared-082-review-qxzv26el/`.

Shared's working tree separately contains uncommitted formatting catalog
selection ([#135](https://github.com/dragginzgame/shared-tooling/issues/135)),
observed-evidence path admission ([#136](https://github.com/dragginzgame/shared-tooling/issues/136)),
companion-closure, installer-output and payload replacement fixes. They are
excluded from the reviewed commit. Recommend the next adoption use a committed
revision including the relevant installer fixes rather than importing mutable
bytes. Exact-source Shared 0.8.2 CI is queued at inspection.

Jobs' snapshot stays **0.8.1**; pending **0.7.3** changes and index/refs are
preserved. The incoming lock now selects Host **0.14.7**, Metrics **0.7.2** and
Testkit **0.34.4**, plus Memory **0.35.15** / Timers **0.19.1**. Preserved without
relabeling the preceding Host 0.14.6/Metrics 0.7.1/Testkit 0.34.3 qualification.
Delivered 0.7.2 native CI now passes Linux and Apple Silicon; Intel's observed
30-minute timeout remains the native blocker tracked by #25. The pending
45-minute repair awaits delivery/acceptance. No compilation, tests, tool
installation, full gate, workflow dispatch, release/publication or sibling edit
is performed in this inspection. Only this handoff is updated.

## Pending 0.7.3 — durable-record coverage and native CI budget

Added bounded mixed-transition qualification for the existing persistence
contract: every successful producer record serializes and restores with the
same record, deadline and active attempt; rejected operations retain the entire
record. Histories include all public job states, pause/resume/reschedule, all
outcomes, retries, backward clocks and maximum deadlines. The test runs through
`make test-jobs`, so the existing CI/release gate selects it. No new runtime,
storage policy, API, lifecycle ownership or compatibility layer is introduced.

Diagnosed the actual 0.7.2 Intel native cancellation: GitHub annotates the job as
exceeding **30 minutes**. Its log continues compiling consumer Clippy dependencies
after successfully building the Wasm consumer and Rust 1.88 recovery harness.
Raised native CI's finite job budget to **45 minutes**, matching recovery. All
three hosts, setup/admission, complete gates and permissions remain. Actionlint
passes; delivery and exact-source native completion under the new budget remain
pending. Owner: [#25](https://github.com/dragginzgame/ic-jobs/issues/25).
Evidence: `intel-native-annotations.json`, `intel-native-job.json`,
`intel-native.log`, `workflow-qualification.json` in this batch directory.

Focused Linux qualification passes on a fixed input checkpoint with incoming
Host **0.14.6**, Memory **0.35.15**, Timers **0.19.1**, Testkit **0.34.3** and
Metrics **0.7.1**. Rust **1.88** core/timers tests pass separately, including the
new history coverage; native/Wasm library and stable-consumer checks, harness
compilation, warnings-denied Clippy/rustdoc and actual Make test routing pass.
The consumer CLI/Wasm refusal and graph-drift guard fixture passes. Snapshot,
pins, Make adapters, links, formatting and whitespace pass. Host advanced during
the initial checks; that mixed-input evidence is retained separately. Repeated
focused checks use the shared validation runner's input checkpoints and saved
before/after graph hashes. Qualified lock/input identities are retained in
`qualified-Cargo.lock`, `qualified-inputs.json` and `stable-checks.json`.
Evidence: `/tmp/ic-jobs-improvements-49xrn201/`.

The previously authorized [v0.7.1 recovery](https://github.com/dragginzgame/ic-jobs/actions/runs/38225596987)
now passes all three hosts. Apple Silicon's uploaded evidence verifies exact
commit/ref/run, tagged manifests/lock/snapshot, Testkit 0.34.1 native selection,
runner checksum, actual Wasm hashes, server logs and seven outcomes per variant.
Together with the prior Linux/Intel reports, this establishes **84 inspected
live results** for `c44312a3e76e08b4f908f1f02ce3374169658212`.
Reports: `apple-071-recovery-acceptance.json` and
`all-hosts-071-recovery-acceptance.json`; artifact: `recovery-071-apple/`.
It qualifies that tested source/graph, not the pending 0.7.3 graph.

#23/#24's selected live obligation is complete. Their remaining native acceptance
and #21's final-source tooling acceptance remain open: 0.7.2 native Linux passes,
Intel timed out and Apple Silicon is queued. #25 addresses the observed blocker.
The library implementation remains unchanged; pending notes stay compatible
**0.7.3**. Own versions/index/refs and external dependency edits are preserved.
No full local CI/release gate, release/publication, new live dispatch or sibling
edit is performed. New Testkit 0.34.3 executable preparation remains separate
from harness compilation and is required before selecting a new live graph.

## Pending 0.7.3 — Shared Tooling 0.8.1

Canonically adopted committed **0.8.1** at
`c97f9f9c7b7c706fffdcfc5925cf58759b58093f`, verified as public main, from a
clean isolated checkout. All **86 roots/payloads** remain. Four shared files
update: strict snapshot-field admission and exact before/observed build-identity
retention on installer environment drift, plus their guides. No path is added or
retired; AGENTS records the reviewed revision. Source owners:
[Shared #133](https://github.com/dragginzgame/shared-tooling/issues/133) and
[Shared #134](https://github.com/dragginzgame/shared-tooling/issues/134).

Focused Linux checks pass: actual common/release tool admission, snapshot/repeat
preview, pins, Make adapters, formatting/hook preservation, consumer CLI/Wasm
refusal and input-drift blocking, formatting-evidence and release/publication
fixtures with substituted release effects. Exact-source upstream Rust installer
and malformed snapshot/export/inventory fixtures pass under Bash 5 and genuine
**Bash 3.2.57**, including child shells. The initial inventory attempts lacked
cloc on PATH; failures are retained. Reruns use Jobs' admitted pinned host bundle
(cloc 2.10) and pass. ShellCheck, links, formatting and whitespace pass.
Evidence: `/tmp/ic-jobs-shared-081-bzkcvgb5/`.

The incoming lock selects Memory **0.35.15**, Timers **0.19.1** and Testkit
**0.34.3**; preserved byte-for-byte. Earlier native/Rust evidence remains bound
to its older released graph; this tooling adoption does not qualify these
independent edits or prepare the new Testkit CLI. Jobs Rust, manifests, tool pins,
Make recipes, real index/refs and Rust **1.88** floor remain unchanged. Shared-only
changes select compatible **0.7.3** notes; no release/version mutation, full local
CI, new live dispatch or sibling edit is performed.

The already authorized v0.7.1 Intel recovery now passes. Its uploaded artifact is
verified against exact commit/ref/run, tagged graph/snapshot, native Testkit 0.34.1
receipt/check log, runner checksum, actual Wasm hashes, server logs and seven
outcomes per variant: **28 Intel live passes**, joining Linux for **56 inspected
v0.7.1 results**. Report: `intel-071-recovery-acceptance.json`; artifact directory:
`recovery-071-intel/`. Apple Silicon remains running; #23/#24 stay open. These
results do not qualify the dirty 0.7.3 graph. Delivered 0.7.2 native CI passes
Linux, with Intel running and Apple Silicon queued at inspection. #21 awaits
this adoption's final delivered-source native acceptance.

## Delivered 0.7.2 — issue qualification

Verified **0.7.2** at `17ff4fbe9d9e7a7c8587a356cd9ba644e7a18fe9`, matching
public main and its annotated tag. The checkout was clean. #21's Shared 0.8.0
adoption, #23's durable management and #24's restore repair are implemented and
delivered; remaining work is source-bound hosted acceptance.

Focused Linux checks pass on this exact released graph: Rust **1.88.0** core and
timers separately (20 composition, 23/24 job, ten management and two doctest
cases), both Wasm library configurations, ten stable-consumer tests, release Wasm
build and harness compilation. Warnings-denied core/consumer native/Wasm Clippy,
rustdoc, actual common/release/Testkit tool admission, snapshot and pins pass.
The initial Testkit refusal retained evidence; canonical setup/check selects
**0.34.2** for the current native environment. Only the receipt's native build
fingerprint differs from its previous selection. Older installations remain.
No PocketIC setup/launch or new recovery dispatch was selected.
Exact inputs/logs: `/tmp/ic-jobs-072-issues-ff_gwi1b/`.

[0.7.2 native CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38231105752)
is running Linux and Intel; Apple Silicon is queued. Superseded 0.7.1 native CI was
cancelled before all-host acceptance. The already authorized
[v0.7.1 recovery](https://github.com/dragginzgame/ic-jobs/actions/runs/38225596987)
has **28 verified Linux live results**; both macOS recovery jobs are running.
No new uploaded artifact was available at inspection. Jobs/consumer/harness Rust,
runner and workflow bytes match 0.7.1; lock/snapshot/version inputs differ, so its
live evidence cannot be relabeled as testing the 0.7.2 graph. Inspect remaining
host evidence under the existing authorization. #21/#23/#24 remain open.

No further implementation defect is identified by this scoped issue review.
No Rust source, package version, lock, index/ref, release/publication, full local
CI or sibling edit is performed. Core/timers retain Rust **1.88**. Routine
qualification adds no pending release entry; the dated 0.7.2 ledger is preserved.

## Shared Tooling 0.8.0 adoption — delivered in 0.7.2

Canonically adopted committed Shared **0.8.0** at
`045a6f22a9de497bfb3af90cb2571957384806f2`, verified as public main, from a
clean isolated source. Committed VERSION now correctly records **0.8.0**;
the earlier version-annotation blocker is resolved. No public v0.8.0 tag was
observed; this is exact committed-source adoption, not tag/publication evidence.

The exporter automatically retires verified obsolete owned payloads outside the
selected entrypoints/companion closure; `--retire-file` is removed. Jobs has no
caller of that flag. All **86 recorded roots/payloads** remain selected. Preview
and apply update only two shared guides plus manifest provenance; no paths are
added or retired. AGENTS records the new revision. Baseline, all shared executable
bytes, Make recipes, tool pins, Rust sources/manifests and incoming lock are
unchanged from the preceding qualified tooling batch.

Actual snapshot, repeat preview, formatting, links and whitespace checks pass.
Exact-source upstream distribution and automatic-retirement fixtures pass with
both Linux Bash 5 and genuine **Bash 3.2.57**, including child shells. Actual
consumer verification/repeat preview also pass under Bash 3.2. Prior Jobs
hook/consumer/release fixture evidence remains bound to its unchanged executable
inputs; no unnecessary product rebuild is run for this documentation/provenance
refresh. Evidence and preserved-input hashes: `/tmp/ic-jobs-shared-080-uy3bob3n/`.
Notes remain compatible **0.7.2**; core/timers retain Rust **1.88**. No package
version, real index/ref, release/publication, full local CI or sibling edit.
[Shared's exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38229670802)
is queued at inspection. #21 awaits final delivered-source native acceptance.

The already authorized [v0.7.1 recovery](https://github.com/dragginzgame/ic-jobs/actions/runs/38225596987)
still has **28 verified Linux live passes**. Intel is now running; Apple Silicon
remains queued. Inspect their uploaded evidence before claiming acceptance or
closing #23/#24; that run does not qualify this dirty graph.

## Previous entrypoint adoption — superseded by Shared 0.8.0

Adopted the committed Shared 0.7 entrypoint implementation at
`8e8cc21265272e0234326c53a8c7bf9e7d8d77df` from a clean isolated source.
Its commit title/changelog select 0.7.0, but committed `VERSION` remains **0.6.6**
and no public v0.7.0 tag was observed. The canonical snapshot preserves that
annotation and exact revision. Upstream has uncommitted VERSION, retirement and
guide changes; they are excluded. Do not claim a finalized 0.7.0 release or
adopt mutable sibling bytes. See [Shared #131](https://github.com/dragginzgame/shared-tooling/issues/131).

The initial migration explicitly retains all **86 previous payloads as roots**,
as the adoption guide permits. Five shared payloads update; none are added or
retired. Subsequent refresh uses the recorded roots and derives companions;
repeat preview reports only KEEP. AGENTS records the reviewed revision. Future
root narrowing needs caller and other-manifest ownership review. Shared files
remain canonical; the exporter/integration fixtures stay upstream-only.

Focused Linux checks pass: actual snapshot/repeat preview, links, formatting,
pins, release adapters, consumer CLI/Wasm refusal and graph-drift blocking,
formatting/evidence and hook preservation, and release/publication recovery with
real disposable Git and substituted gate/registry/upload effects. The upstream
exact-source distribution/retirement fixtures and actual consumer verification/
repeat preview also pass with **Bash 3.2.57**, including Bash child commands.
Prepared/admitted the existing selected cargo-edit **0.13.13** through the canonical
installer after the first fixture preflight refused its missing selection.
Existing tool installations and failed-attempt evidence are retained.

Corrected the remaining root-guide direct Timers alignment to **0.19**. The
incoming lock now selects Host **0.14.5**; it is preserved. Earlier Rust/native
qualification below used Host 0.14.4 and does not qualify this later graph.
No Rust source, package versions, tool pins, real index/refs, API, storage or
Rust **1.88** floor changes. Compatible notes remain **0.7.2**. No complete local
CI/release gate, release/publication, new live dispatch or sibling edit runs.
Evidence: `/tmp/ic-jobs-shared-070-3gwkbl45/`.

The already authorized v0.7.1 recovery still has Linux's **28 verified live
passes**, with both macOS jobs queued at the latest observation. Its source/
graph is separate from this pending tooling adoption; keep #23/#24 open.

## Pending 0.7.2 — Shared 0.6.6 and current graph qualification

Jobs **0.7.1** is delivered at `c44312a3e76e08b4f908f1f02ce3374169658212`,
matching public main and annotated `v0.7.1`. It delivered the 0.6.5 adoption;
the 0.6.6 preview remained unapplied. Canonically adopted committed Shared
**0.6.6** `a15eb9d406d287697df9bc8fa96db8cda6721a71` from an isolated clean
source after public-main verification. All **86 selections** remain; only the
tool catalog, local setup and hook rules change. AGENTS records the new revision.
Shared files match the canonical export.

Prepared and admitted the selected common Rust tools through the canonical
installer, including `cargo-sort-derives` **0.14.1**. Its actual stdin probe
preserves both literal `$crate` paths while sorting them. Jobs has no affected
macros or derive-sort invocation to change; no workaround markers are removed.
Existing installations are retained. Contributor tool preparation does not
change Jobs' core/timers Rust **1.88** floor.

The incoming lock selects Host **0.14.4**, Memory **0.35.14**, Metrics **0.7.1**
and Testkit **0.34.2**, with Timers **0.19.0** already released in 0.7.1.
The lock is preserved byte-for-byte. Published Host, Memory and Metrics Rust
sources and normalized dependencies match the preceding patches. Selected
archives match lock checksums; unpacked Rust/manifests match those archives.
Testkit changes only its native server CLI's early line-break store-path refusal
and coverage; its library interface remains unchanged. Prepared/admitted its
exact 0.34.2 CLI separately. An actual invalid-path offline check refuses before
creating the store. PocketIC setup/live execution is not selected by that check.

The delivered 0.7.1 manifest also changed the exposed Timers family from **0.18
to 0.19**, omitted from its release notes. This crosses the adapter's public
type identity and required a pre-1.0 minor release; the already delivered tag,
version and dated history are preserved. Both current adapter guides are
corrected from the stale 0.17 claim to **0.19**, and new notes state the required
direct-consumer alignment. This pending batch is compatible relative to the
actual 0.7.1 contract, so notes select **0.7.2** without version changes.

Focused Linux qualification passes on the exact incoming graph: Rust **1.88.0**
core/timers separately (20 composition cases, 23/24 job cases, ten management
cases and two doctests), both Wasm feature configurations, ten native stable
consumer cases, release Wasm consumer build and harness compilation. Core and
consumer native/Wasm Clippy, warnings-denied rustdoc, actual offline full tool
admission, formatting/hook preservation, CLI/Wasm refusal, graph-drift blocking,
release adapters and release/publication recovery fixtures pass. Snapshot, pins,
documentation links and whitespace pass. No full local CI/release gate, package
publication, current-graph live recovery, Rust implementation/index/ref/version
change or sibling edit is performed. Evidence: `/tmp/ic-jobs-shared-066-7bmatz22/`, including
the qualified lock, input hashes, source/archive review and prepared CLI evidence.

The previously selected [v0.6.0 recovery](https://github.com/dragginzgame/ic-jobs/actions/runs/38153167824)
now passes all three hosts. Intel's uploaded artifact is verified against exact
commit/ref, root lock/snapshot, Testkit selection, runner checksum, actual Wasm
hashes, server logs and seven outcomes per variant. Combined with the earlier
Linux/Apple evidence, this establishes **84 verified live passes** for the tested
0.6.0 source/graph. Intel evidence/report is in this batch's `recovery-intel/`
and `intel-recovery-acceptance.json`; prior Linux and Apple reports remain at
their recorded locations. It does not qualify the later restore repair or 0.7.1
Timers/Metrics graph. Keep #23/#24 open for remaining current-source acceptance.

[Delivered 0.7.1 native CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38222886565)
passes Linux with both macOS hosts queued. Prior 0.7.0 and 0.6.1 native runs were
cancelled before all-host acceptance. Shared 0.6.6 CI passes Linux/lint, with both
macOS hosts running. #21 follows this new adoption's final delivered source.
The maintainer explicitly selected **v0.7.1 recovery**. The reviewed workflow
checksum matches the tagged source; no existing run for that tag was found.
[Run 38225596987](https://github.com/dragginzgame/ic-jobs/actions/runs/38225596987)
is dispatched at exact commit `c44312a3e76e08b4f908f1f02ce3374169658212`.
Linux now passes. Its uploaded artifact is inspected: exact commit/ref/run,
tagged lock/snapshot, Testkit **0.34.1** selection, runner checksum, all four
actual Wasm hashes, server logs and seven outcomes per variant agree:
**28 verified current-source Linux live results**. Report:
`linux-071-recovery-acceptance.json`; evidence: `recovery-071-linux/`. Both macOS
jobs remain queued. This qualifies v0.7.1's tested source/graph, independently
of this dirty 0.7.2 graph. Read-only watch output is retained in
`recovery-071-watch.log` (exec session 72446); inspect uploaded artifacts before
claiming any completed live result or closing #23/#24. This authorization
persists for that run's inspection; do not request dispatch approval again.

## Shared 0.6.6 and Host 0.14.3 inspection

Verified public Shared main at **0.6.6**
`a15eb9d406d287697df9bc8fa96db8cda6721a71` and Host **0.14.3** main/annotated tag
at `543b137458feba0f6dfd3e458b680fb2be3d9e6d`. Shared now selects
`cargo-sort-derives` **0.14.1**, repairing literal `$crate` replacement for direct
and conditional derives. Its contributor build requires Rust 1.88 or newer;
Jobs' core/timers floor is unchanged. Jobs' formatter does not run derive sorting,
and no affected macros or workaround markers are found in its crates/consumer.
No Jobs Rust edit or workaround retirement is needed.

The canonical read-only preview from an isolated clean source passes against
the incoming uncommitted 0.6.5 adoption: three existing selections would change
(`ci/tool-versions.env`, local setup and hook rules), with no new companions or
selection removals. This inspection does not export or prepare the new CLI.
The snapshot remains Shared **0.6.5**, with **86 selections**; adopting 0.6.6
must explicitly prepare/check the new common tool selection before qualification.
Existing tools and receipts are retained.

The incoming lock already selects all four Host crates **0.14.3**. Published Rust
trees and normalized manifests match 0.14.2 apart from package versions; declared
floors remain **1.88**. All selected archive checksums match the lock, and unpacked
Rust/manifests match those archives. Locked offline metadata admits the graph.
Host stays native-only through Testkit, with no direct Jobs dependency or adapter
change needed. The lock is preserved byte-for-byte. Pending compatible **0.7.1**
notes include this existing selection; package versions remain **0.7.0**.

[Shared exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38220477204)
and [Host exact-source CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38219424683)
are queued at inspection. Source/metadata review does not establish new builds or
native/live qualification. No tests, compilation, lint, full gate, tool install,
workflow dispatch, publication, index/ref mutation or sibling edit is performed
by this inspection. Evidence: `/tmp/ic-jobs-upstream-066-34q14fyp/`.
Only pending notes and this handoff are updated; all incoming adoption work is
preserved.

## Pending 0.7.1 — Shared Tooling 0.6.5

Jobs **0.7.0** is delivered at `09497fc523874bf673224c3b87925d05cc2a0311`,
matching public main and its annotated tag. The incoming checkout was clean.
The accepted Shared update now adopts committed **0.6.5** at
`42f2091768b5f71c98a684d3dab563f408ca2dd9`, verified as public main, from an
isolated clean source. The canonical exporter preserves 84 selections, adds
`make/consumer.mk` and `docs/adoption/make-interface.md`, and updates 14 existing files:
**86 selections**. AGENTS records the exact reviewed revision; shared copies
are unmodified after export.

Jobs uses the common Make entrypoint for setup, formatting, hooks, snapshot
verification and releases. Equivalent local hook/snapshot recipes and separate
includes are replaced; the default goal, cargo-edit extension, direct delivery,
product gates and metadata/publication adapters remain consumer-owned. Fixture
exports include the new Make input. The new shared hook passes prepared-tool
selection context scoped to its exact exported worktree; the Rust installer
admits it only for checks against exported pins/compiler.

CI and release verification checkpoint the workspace manifests, lock, toolchain,
snapshot, Makefile and tool catalogs through the shared validation runner.
Nested gates in this checkout retain the original checkpoint. Changed, missing
or unreadable inputs block later stages and retain identities/logs. The actual
Jobs CI route is qualified in disposable changed/missing-lock cases: substitute
tool admission completes, the first stage fails its checkpoint, and no later
product validation is dispatched. No full product CI or PocketIC runs in these
cases. Existing independent fixtures qualify shared nesting and failure status.

Release recovery fixtures now keep one recipe per target, retain actual shared
ordered setup and release routing, and substitute installer/gate/registry/upload
effects. Product metadata transforms and local Git/bare-remote recovery remain
real. Successful logs must contain no recipe-override warnings. Linux focused
qualification passes: actual offline complete tool admission (no installation),
formatting and hook preservation/installation in disposable copies, consumer CLI
and Wasm refusal, changed/missing-input blocking, release Make adapters and the
release/publication recovery fixture. ShellCheck with sourced companions,
snapshot, declarations, links and whitespace also pass. Retained earlier
attempts include the missing fixture delivery/version adapters, a missing sort
pin in the new drift fixture, and an incomplete ShellCheck invocation; corrected
checks pass. Evidence: `/tmp/ic-jobs-shared-064-5adcvig8/`.

Pending notes select compatible **0.7.1** with
[#21](https://github.com/dragginzgame/ic-jobs/issues/21) and Shared
[#130](https://github.com/dragginzgame/shared-tooling/issues/130),
[#129](https://github.com/dragginzgame/shared-tooling/issues/129) and
[#128](https://github.com/dragginzgame/shared-tooling/issues/128). No Jobs API,
runtime, stored format, compiler floor or dependency change is made. Core/timers
retain Rust **1.88**; package versions and lock remain **0.7.0**. Full CI/release
gates, native macOS and new live qualification for this adoption are not selected.
[Delivered 0.7.0 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38217166313)
passes Linux; Intel is running and Apple Silicon queued. This qualifies its
0.6.3 snapshot, not the pending 0.6.5 adoption. Keep #21 open for final-source
acceptance. No real index/ref/version change, commit, release/publication,
hosted dispatch, installation reset or sibling edit is performed.

## Upstream inspection — incoming Timers 0.18 selects Jobs 0.7.0

The incoming manifest already selects **Timers 0.18**, and the incoming lock
selects Timers **0.18.0**, all four Host crates **0.14.2**, Candid/derive **0.10.38**
and unicode-ident **1.0.27**. A concurrent maintainer commit
`d7c000a091cd8f6a901ace1f86f77a29bfc0dbc0` commits the repair and advances Memory
to **0.35.13**; it is preserved. Package versions remain **0.6.1**.
Pending notes now select **0.7.0**
because the optional Jobs adapter exposes Timers' nominal public types: consumers
must select Timers 0.18 together. This replaces the earlier compatible 0.6.2
draft classification for the complete batch. No stored JobRecord change or
installation reset is needed.

Public main/tag verification finds Host **0.14.2** at
`fed8724e79862d8273d01823619fe648c981c019` and Timers **0.18.0** at
`52bc4bad399122be040cf155d9b4563b5509e009`. Published Host Rust sources and
normalized manifests match 0.14.1 apart from package versions; Host stays
native-only through Testkit. Timers retains Rust **1.88**, but direct register
functions add an explicit initial schedule and scheduled-creation cleanup.
Jobs and its consumer use the unchanged `reconcile_watchdog` boundary, so no
caller rewrite or new timer provider is required. Selected archive checksums
match the lock. Upstream CI on both exact delivered commits passes.

Shared public main is **0.6.4** at
`6fc8eb9a94f83fb0b9d0a26377d4232508cdbe2d`, with successful exact-source CI.
Its common Make interface owns setup, formatting, hooks, snapshot checking and
releases while retaining consumer inputs. The canonical read-only preview
passes: ten current selections would update, and `make/consumer.mk` plus
`docs/adoption/make-interface.md` would be added (86 selections). Adoption must replace
Jobs' separate includes and equivalent local hook/snapshot recipes, propagate
fixture inputs and qualify actual routing. This inspection does **not** export
or claim adoption; the reviewed snapshot remains Shared **0.6.3**, with 84 files.

Focused qualification is repeated after the concurrent Memory change and passes
on the final selected graph: Rust **1.88.0** core and
timers independently (20 composition, 23/24 job, ten management and two doctest
cases), both Wasm feature configurations, ten native stable-consumer tests,
release Wasm consumer build, recovery-harness compilation, warnings-denied
core/consumer Clippy and rustdoc. These are Linux results; full CI/release gates,
new macOS qualification and live recovery for this graph are not selected.
Evidence: `/tmp/ic-jobs-upstream-1012-17x8ucse/`, including the qualified lock
and pre/post input identities. Earlier logs retain the Memory 0.35.12 graph.
The incoming manifest/lock, product source and prior work are preserved; no agent
index/ref or sibling mutation is performed.
Only pending notes and this handoff are updated; no install, dispatch or release
effect is performed.

## Pending 0.6.2 — retained timing validation

The accepted management/restore review reproduced three corrupt-record admissions
on released 0.6.1 product source, tracked in
[#24](https://github.com/dragginzgame/ic-jobs/issues/24). Restoration now checks
that Running retries respect their retained completion and preceding backoff,
that cancelled retries retain a possible completion/backoff deadline, and that
successor deadlines still match retained success after cancellation or first
dispatch. The canonical schedule calculation remains the deadline owner.
Uncertain records still block dispatch until explicit disposition; public
transition errors and valid records retain their contracts.

The stable consumer's new corruption cases refuse the complete queue before
effects, and failed reopening preserves the stored queue payload. Repeated
rescheduling, pause/resume, cancellation before retry eligibility, representable
maximum deadlines and exhausted identity counters remain covered. The next
notes select compatible **0.6.2**: no API, stored layout, dependency or Rust-floor
change. Package manifests and lock stay **0.6.1**.

Focused Linux checks pass on the current locked graph: Rust **1.88.0** core and
timers separately (20 composition cases, 23/24 job cases, ten management cases,
two doctests), separate core/timers Wasm checks, ten native stable-consumer tests,
release Wasm consumer build and recovery-harness compilation. Core and consumer
native/Wasm Clippy with warnings denied, rustdoc, formatting, links, declarations,
snapshot and whitespace checks also pass. Retained failed attempts include the
released corruption reproductions, a corrected test expectation for the existing
NotPending error, a corrected non-Copy fixture setup and an overly broad backing
memory comparison replaced by queue-payload preservation. Evidence:
`/tmp/ic-jobs-062-restore-c55aezci/`. Full CI/release gates and new live execution
remain unselected; #24 stays open for delivery and source-bound acceptance.

The already selected [v0.6.0 recovery](https://github.com/dragginzgame/ic-jobs/actions/runs/38153167824)
now passes Linux and Apple Silicon. Apple artifact source/ref, lock/snapshot,
Testkit selection, runner checksum, four actual Wasm hashes, server logs and all
seven scenarios are inspected: **28 results per host, 56 verified live results**.
Apple evidence is in this batch's `recovery-apple/` and
`apple-recovery-acceptance.json`; prior Linux evidence remains at the location
below. Intel recovery is running. This qualifies the tested **0.6.0** source and
graph; it does not qualify this pending restore repair or 0.6.1's graph.
[0.6.1 native CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38155976004)
still passes Linux with both macOS hosts queued. #21 and #23 remain open.
No new dispatch, commit, release, publication, reset or sibling edit is performed;
the incoming handoff, real index and refs are preserved.

## Delivered 0.6.1 — finish hosted acceptance

Jobs **0.6.1** is delivered at `4cc7fdb75867ca4c5d55a2364efaa2c8cab79478`,
matching public main and the annotated `v0.6.1` target. The checkout was clean at
inspection. Product Rust sources, consumer, recovery harness and workflows match
0.6.0; release metadata, lock and prior qualification notes changed. The snapshot
remains Shared **0.6.3** `4fd979d13c32adb3cb2f51f213ebfc4f98339b3d`.

The released graph selects Memory **0.35.12**, Timers **0.17.14**, Metrics **0.6.6**,
Testkit **0.34.0**, Host **0.14.1** and Syn **3.0.8**. The prior Timers/Metrics
source reviews are delivered. Memory's normalized manifest and floor remain
unchanged; its only changed Rust file adds tests to the private `repo_tool`
example, which Jobs does not compile as a dependency. Jobs' core/timers floor
remains Rust **1.88**. No new library adapter or scheduling policy is needed.

[0.6.1 native CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38155976004)
passes Linux; Intel and Apple Silicon remain queued. The preceding 0.6.0 native
run passed Linux but was cancelled before macOS acceptance. Keep
[#21](https://github.com/dragginzgame/ic-jobs/issues/21) open for the current
source's all-host tooling gate. The CI concurrency policy replaces queued/running
checks on each newer main push; allowing this source's macOS jobs to finish avoids
repeatedly superseding its acceptance.

[Selected v0.6.0 recovery](https://github.com/dragginzgame/ic-jobs/actions/runs/38153167824)
passes Linux. Its uploaded artifact is inspected: exact commit/ref, released
lock and snapshot, Testkit 0.34.0 selection, recovery-runner checksum, all four
Wasm hashes and server logs match the selected execution. All seven scenarios
pass for original, `-O3`, `-Os` and `-Oz`: **28 live Linux results**, including
management upgrades, wakeup reconstruction, trap rollback and blocked effects.
Intel and Apple Silicon recovery remain queued. This evidence is bound to
`7d23a7919890db6d7b22d491b84372576fbf545f` and its 0.6.0 graph; it is not relabelled
as 0.6.1 live qualification. Keep
[#23](https://github.com/dragginzgame/ic-jobs/issues/23) open for its remaining
hosted acceptance. No new recovery dispatch or local compilation is selected by
this inspection; no current failing CI job is observed.

The next useful work is completing those hosted gates and inspecting their
remaining evidence before closing issues. There are no other open issues or PRs
at this inspection. Evidence: `/tmp/ic-jobs-061-inspection-8ic7k1tg/`, including
retained Linux artifacts and `linux-recovery-acceptance.json`. Index, refs,
manifest versions and lock are preserved; no release/publication, installation
reset or sibling file change is performed.

## 0.6.0 delivery and earlier qualification observations

Jobs **0.6.0** is delivered at `7d23a7919890db6d7b22d491b84372576fbf545f`,
matching public main and the annotated `v0.6.0` target. Durable management and
its seven-scenario recovery harness are delivered. The snapshot remains Shared
**0.6.3** `4fd979d13c32adb3cb2f51f213ebfc4f98339b3d`, with all 84 selections.

[Exact-source native CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38152475233)
passes Linux; Intel and Apple Silicon remain queued. The preceding
[0.5.13 run](https://github.com/dragginzgame/ic-jobs/actions/runs/38149217014)
passed Linux but both macOS jobs were cancelled. Keep
[#21](https://github.com/dragginzgame/ic-jobs/issues/21) open for final-source
three-host tooling acceptance and
[#23](https://github.com/dragginzgame/ic-jobs/issues/23) open for native and live
management acceptance. No failing current CI job is observed; queued/running
checks do not establish a completed gate.

The released graph now selects Memory **0.35.11**, Timers **0.17.13**, Testkit
**0.34.0**, Host **0.14.1** and Metrics **0.6.5**. The focused preparation evidence
below uses the earlier 0.5.13 graph and is not relabelled as qualification of
these delivered dependencies. The existing recovery workflow is delivered and
can qualify `v0.6.0` after explicit selection: seven live scenarios, original plus
three optimized Wasm variants, on all three hosts. It prepares the exact locked
Testkit CLI/server and records source, graph, artifacts and outcomes. Earlier
v0.5.9 recovery cannot qualify this new stored contract or graph. The maintainer
selected the described v0.6.0 workflow by asking to keep going.
[Recovery run 38153167824](https://github.com/dragginzgame/ic-jobs/actions/runs/38153167824)
is dispatched on the exact annotated tag: Linux is preparing tools, with both
macOS hosts queued. No new live result is claimed before its evidence is inspected.

The incoming dirty lock initially selected Metrics **0.6.5 -> 0.6.6**; a
concurrent external update also selects Timers **0.17.13 -> 0.17.14**. Both are
preserved. Source review verifies identical published Rust files and normalized
manifest dependencies/features against their preceding patches. Floors remain
Metrics 1.85 and Timers 1.88. Both cached archives match their selected registry
checksums and unpacked file bytes. No Jobs adapter or runtime change is required;
numbered compatible notes select **0.6.1** without changing package versions. This dirty graph is separate from v0.6.0 live/native
qualification; it has not received new build, full-CI or live qualification.
Evidence: `/tmp/ic-jobs-060-recovery-gerzjl79/` (dispatch/progress/Metrics review),
`/tmp/ic-jobs-060-delivery-uzl5p5z4/` (delivery). No local tool install,
index/ref/version change, release/publication, installation reset or sibling file
change is performed.

## 0.6.0 implementation — earlier focused qualification

[#23](https://github.com/dragginzgame/ic-jobs/issues/23) now has core and consumer
implementation. `Job::pause` retains Pending deadlines and retry evidence while
excluding dispatch. `resume` retains the original deadline and Skip/CatchUp
policy; overdue catch-up remains consumer-bounded. `reschedule` replaces intent
from an unstarted occurrence forward, retaining job/payload/effect identity,
sequence and retry policy. Paused work stays paused; cancellation accepts it.
Attempted occurrences refuse rescheduling, and Running/Uncertain effects remain
blocked until existing explicit disposition. Errors preserve records and committed
bytes, with the existing transition precedence retained.

The current JobRecord has an explicit `schedule_origin` and independent
`last_outcome_ns`: management time cannot rewrite retry or recurrence evidence.
All reconstruction still crosses `Job::restore`. The canister consumer adds
manager-authenticated pause/resume/reschedule, commits the complete validated
queue before watchdog reconciliation, and uses the distinct
`test.jobs.consumer.management.v1` key. The maintainer explicitly confirmed only
disposable 0.5 test installations exist and approved their retirement/reset.
Create fresh 0.6 fixtures; no old reader, migration, automatic reset or old-key
reuse is implemented. No deployed installation was reset by this work.

The maintainer explicitly selected focused Rust qualification for this batch,
overriding the earlier no-tests/builds/lint exception only for these focused
checks. Linux qualification passes on Rust **1.88.0** independently for core and
optional timers: 20 consumer-composition tests, 23 core / 24 timer-enabled job
tests, and six new management tests in both configurations. Core and optional
timers also pass the separate Wasm floor checks. `make check-consumer` passes
nine native stable-consumer tests, the Rust 1.88 release Wasm build, recovery
harness compilation and warnings-denied native/Wasm consumer Clippy. Core Clippy,
warnings-denied rustdoc and both doctests pass on development Rust 1.99.0.
The first consumer attempt failed because the new key omitted Memory's required
`.vN` suffix; the original log is retained and the corrected distinct key passes.
Evidence: `/tmp/ic-jobs-060-427glpry/`.

The harness now has seven ignored live scenarios, adding authenticated management,
paused/rescheduled upgrade retention, wakeup stop/rearm, overdue resume policies,
management trap rollback and blocked external-work refusal. Compilation does not
qualify those live effects. Full local CI/release gates, native macOS and new live
PocketIC remain unselected/pending; prior v0.5.9 recovery does not qualify the new
record or harness. Keep #23 open for delivered-source native/live acceptance.
Package versions remain **0.5.13**; only numbered pending notes select **0.6.0**.
The real index, refs, manifests and lockfile are preserved. No commit, release,
publication, hosted dispatch, sibling edit or installation reset was performed.

## Delivered 0.5.13 — Shared 0.6.3 acceptance remaining

Jobs **0.5.13** is delivered at
`5fa4c500dc6a40a15fd27260dfed1d581cd054eb`, matching public main and the annotated
`v0.5.13` target. The 84-file snapshot adopts committed Shared **0.6.3**
`4fd979d13c32adb3cb2f51f213ebfc4f98339b3d` canonically. Normal direct release
commands can archive superseded preparation from a clean descendant commit and
select the increment from current metadata with fresh validation. Exact resume
and later effect reconciliation retain their existing contracts. Local guidance
records temporary metadata backup's abrupt-kill limits; there is no durable
metadata journal or consumer fork. Adoption evidence remains at
`/tmp/ic-jobs-shared-063-bpd6yvgr/`.

[Exact-source Jobs CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38149217014)
passes Linux; Intel and Apple Silicon are queued at inspection. Keep
[#21](https://github.com/dragginzgame/ic-jobs/issues/21) open for both remaining
hosts on this final delivered adoption. [Shared 0.6.3 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38145666246)
passes lint/security, Linux and Intel; Apple Silicon is running. Linked producer
tracking is [Shared #126](https://github.com/dragginzgame/shared-tooling/issues/126).
These results bind the delivered source, not the dirty 0.6 management batch.

The delivered lock selects Memory 0.35.10, Timers 0.17.12, Testkit 0.33.8 and Host
0.13.5 (Metrics 0.6.4). Published Testkit Rust sources match 0.33.7; Timers changes
only its own private test provider, not the dependency compiled into Jobs. The
separately selected Testkit 0.33.8 CLI remains unprepared at its last offline check.
Run explicit `make install-testkit-tools` and `make testkit-tools-check` before
selecting new live recovery; no CLI/server install is selected by focused harness
compilation. Graph-review evidence remains at `/tmp/ic-jobs-0513-iue1y6cz/`.
The older v0.5.9 live evidence stays bound to its original graph/source.

## Delivered 0.5.12 — three-host native acceptance

Jobs **0.5.12** at `08cf421e9dd5dd3ae12d446ef8a4dfe5baed4a8b` now
[passes all three native hosts](https://github.com/dragginzgame/ic-jobs/actions/runs/38144542439).
Its full logs include release/formatting/consumer fixtures and separate core/timers
Rust 1.88 native/Wasm checks. This qualifies its Shared 0.6.2 adoption, not the
subsequently delivered 0.6.3 snapshot or current management changes. #21 retains
final 0.5.13-source acceptance. Logs are retained with current evidence at
`/tmp/ic-jobs-060-427glpry/jobs-0512-ci.log`.

## Pending 0.5.12 — Shared 0.6.2 and selected upstream patches

Canonically adopted committed Shared **0.6.2** at
`7f1ca16b0dfa98021d2671017ecb7de767409b0e` from the clean isolated source.
The exporter retains all 84 selections and updates the release runner, release
guide and local setup guide. Source conflicts report the exact saved/current
commits and plan identity. Recovery remains a guarded maintainer procedure;
ordinary adoption does not retire release plans or authorize release effects.
Audit/Machete guidance now distinguishes their actual invocation contracts;
Jobs has no such consumer target to change. Adoption remains with
[#21](https://github.com/dragginzgame/ic-jobs/issues/21), with source changes in
[Shared #126](https://github.com/dragginzgame/shared-tooling/issues/126) and
[Shared #123](https://github.com/dragginzgame/shared-tooling/issues/123).

The incoming lock already selects Memory **0.35.10**, Timers **0.17.11** and
Host **0.13.5**, alongside Testkit 0.33.7 and Metrics 0.6.4. It is preserved
byte-for-byte. Locked fetching prepares the selected source cache; locked offline
metadata admits the graph. Published Memory, Timers and all four Host Rust trees
match their preceding selected patches. Their declared floors remain 1.88;
Metrics remains 1.85. No Jobs API, runtime, storage or MSRV change is needed.
Snapshot integrity, pin declarations, 222 local references, Bash parsing and
whitespace checks pass. Real offline `make tools-check` also passes the complete
selected tool set, including cargo-edit admission; no installation or tool build
is dispatched. These checks do not qualify core/timers builds or native
and live behavior. Tests, builds, lint, complete CI and live execution remain
excluded by the recorded local exception. Evidence:
`/tmp/ic-jobs-0512-hwreyei3/`.

Jobs **0.5.11** is delivered at
`628c181f082475fba3e6d55a9b28b860f63d3d81`, matching public main and annotated
`v0.5.11`. Its [native CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38143101651)
is running Linux with both macOS hosts queued. The preceding 0.5.10 run was
cancelled before all-host acceptance. Exact-source
[Shared 0.6.2 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38143010876)
passes lint/security; portable Linux/Apple Silicon are running and Intel queued.
#21 remains open for final-source native acceptance. Earlier v0.5.9 recovery
qualifies only its tested source/graph/snapshot, not this pending batch. Package
versions stay 0.5.11; the real index and refs are untouched. No release,
publication, new workflow dispatch or sibling edit is performed.

## Pending 0.5.11 — Shared 0.6.1 installer failure handling

Canonically adopted committed Shared **0.6.1** at
`731909525038c58d6f80b498e80f8630b11c685d`, verified as current public main.
The clean isolated source/exporter preserves all 84 selections and updates only
the Rust installer, Cargo-build helper and store companion. The installer now
propagates lock/validation/activation failures explicitly even where Bash disables
errexit, preserves the original failure status and retains failed attempts.
Activation requires acquired store and checkout locks. No Jobs-owned installer,
vendored patch, API/storage/runtime or Rust-floor change is added. Pending notes
stay compatible **0.5.11**, with adoption/acceptance in
[#21](https://github.com/dragginzgame/ic-jobs/issues/21) and source ownership in
[Shared #120](https://github.com/dragginzgame/shared-tooling/issues/120).

Snapshot integrity, dependency declarations, documentation links, Bash parsing
and whitespace checks pass. Offline `make tools-check` passes host/IC admission,
then refuses unprepared shared cargo-sort (status 2). The changed Cargo-build
policy is part of exact selection identity; old installations are retained and
never used as a fallback. Explicit `make install-tools` and `make tools-check`
are required before qualification, with separately selected Testkit setup/check
before live recovery. Cargo-tool preparation builds remain excluded by the
recorded local exception unless selected explicitly. No new tests, lint, full
gate or live execution is performed. Exact-source
[Shared CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38141614488)
passes lint/security; Linux portable regression is running, both macOS hosts
queued. Evidence: `/tmp/ic-jobs-0510-live-d0nt184y/`.

The incoming lock is preserved byte-for-byte. It advances Host 0.13.3 to 0.13.4,
Memory 0.35.8 to 0.35.9 and Metrics 0.6.3 to 0.6.4. A later incoming edit also
advances Testkit 0.33.6 to 0.33.7; current SHA-256:
`5137f879026b9cdc353da93786316f414a33377fd0ce7ae1b93aa409e0da87e4`.
Both incoming selections are retained separately in the evidence directory.
Six cached published Rust trees match the preceding versions. Host FS adds a
private regular-lock-file opener and factors existing descriptor admission;
Jobs does not call that new API. Advertised dependency floors remain at or below
1.88, but this incoming graph requires its own qualification. Package versions,
real index and refs are unchanged; no commit, release/publication or new hosted
dispatch is performed.

## Delivered 0.5.10 — native acceptance remaining

Jobs **0.5.10** is delivered at
`7dfe1a38a200d3ede2f86f25c93992abf2d5f284`, matching public main and annotated
`v0.5.10`. The delivered Shared 0.6.0 adoption includes the repaired release
fixture preflight and accurate consumer memory/recovery guidance. Documentation
acceptance completes [#22](https://github.com/dragginzgame/ic-jobs/issues/22).
[Native CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38140477458)
passes Linux; Intel is running, Apple Silicon queued. Source-bound all-host
adoption acceptance remains with #21; the pending 0.6.1 snapshot and dirty graph
are separate inputs.

The explicitly authorized
[v0.5.9 recovery](https://github.com/dragginzgame/ic-jobs/actions/runs/38134824424)
now passes all three hosts. Both remaining macOS archives are inspected alongside
the earlier Linux archive against exact source
`b289e443d4955ffe17d9d97a232eb83432340395`: source/lock/toolchain/snapshot match,
all six maintained scenarios pass for original/Binaryen 133 O3/Os/Oz (**72 live
results**), actual Wasm checksums and genuine PocketIC logs verify. This completes
[#20](https://github.com/dragginzgame/ic-jobs/issues/20). The
[Intel archive](https://github.com/dragginzgame/ic-jobs/actions/runs/38134824424/artifacts/11701927162)
SHA-256 is `08279f8e4c5f224ace6dbf122a5c9bd83483971e62716560eaa47e7ba6a44d47`;
the [Apple Silicon archive](https://github.com/dragginzgame/ic-jobs/actions/runs/38134824424/artifacts/11702183331)
SHA-256 is `e6bab29bb44fdc0a9c2a0eb633089144b5783caf232f0ebf3979803577f13bab`.
Both match GitHub digests. Executable payloads are not uploaded; identities remain
admission receipts. Hidden attempt snapshots are omitted; workflow-level copies
match exact released source. Combined review: `all-hosts-review.json` in the
current evidence directory. Maintained Rust trees match 0.5.10, but these results
qualify the tested v0.5.9 graph/snapshot only. No redispatch or local live gate is
performed.

## Earlier 0.5.10 preparation — Shared 0.6.0 and consumer guidance

The maintainer's CI attempt at prepared `48472b4` reached the release fixture,
then failed because its substituted Rust installer admitted only cargo-edit
checks, omitting the common `install-tools` preflight call. The fixture now
admits only that declared consumer/catalog/preflight argument shape, records it
before installation and retains refusal of unexpected dispatches. Its ordered
setup assertion includes the preflight. The reported failed attempt remains at
`/tmp/jobs-release-tooling.2NF9Hb`; the original CI log is preserved.

The maintainer explicitly authorized **only `make test-release-tooling`** for
this repair, overriding the earlier local exception for that focused target.
It passes on Linux with the actual local Git fixtures and substituted complete
gate, registry and upload effects; prerequisites admit the prepared cargo-edit
and format tools. Shell parsing, snapshot/pin/link and whitespace checks pass.
Evidence: `/tmp/ic-jobs-release-preflight-ron28ihb/`. This is a fixture-only
compatible addition to 0.5.10 under
[#21](https://github.com/dragginzgame/ic-jobs/issues/21). Full CI/native acceptance
of the final source remains required. No real-index, package-version, lock or
Git-ref changes, actual release/publication or new live gate are performed.

### Earlier snapshot preparation and admission

Canonically adopted committed Shared **0.6.0** at
`75180980a0c4e4252d8a915aaecfaefd92bf9a4e` from an isolated clean source clone.
The exporter retains all 83 selections, updates 12 existing files and adds the
required Cargo-build companion: 84 selected files. The baseline, standard Cargo
build engine and failure action/selector advance together. Jobs' pinned
cargo-edit setup/check and metadata execution now use the shared owner's returned
checked path; previous installations remain retained. The release fixture covers
consumer routing with substituted admission, and the CLI-refusal fixture copies
all companions and isolates its store/compiler identity probes.

The canonical [Shared #121](https://github.com/dragginzgame/shared-tooling/issues/121)
repair archives admitted local evidence and selector diagnostics on refused store
admission, uploads it, then fails incomplete collection. Jobs' existing action
fixture now covers the local archive/final-failure sequence with substituted hosted
upload. New fixture execution remains pending under
[#21](https://github.com/dragginzgame/ic-jobs/issues/21).

Before qualification, explicitly run `make install-tools` and `make tools-check`;
prepare the separately selected CLI with `make install-testkit-tools` and
`make testkit-tools-check` before recovery qualification. Those setup commands can
build tools and are not run under the recorded local no-tests/builds/lint/full-CI/
live exception. Snapshot integrity, pin declarations, documentation links, Bash
parsing and whitespace checks pass; these do not establish execution. Tool
admission, affected fixtures and all three native host gates remain required for
the final source. Real offline `make tools-check` passes host/IC admission then
refuses the unprepared shared cargo-sort selection (status 2). Separate
`make release-tools-check` and `make testkit-tools-check` also refuse missing
shared cargo-edit 0.13.13 and lock-selected Testkit 0.33.6 (status 2); neither
falls back to an older installation or builds a tool. Check logs are retained.
Exact-source
[Shared CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38136593268)
passes lint/security and Linux portable regression; both native macOS gates are
running at inspection. Evidence: `/tmp/ic-jobs-0510-issues.yc882gx7/`.

An incoming Cargo.lock edit during this batch is preserved separately as
`late-incoming.lock` in that evidence directory. It selects Host 0.13.2, Memory
0.35.8, Metrics 0.6.3, Testkit 0.33.6 and Timers 0.17.9; SHA-256:
`a47f4a00ea2b6b4af393c6418db8eb5e1ed54b013503a0290e8afd090976c285`.
Seven cached published Rust trees match their preceding versions; the new Timers
source is absent from the cache at inspection. This graph requires separate
qualification, including the maintained core/timers Rust 1.88 checks. Delivered
v0.5.9 native/live results qualify its earlier graph and snapshot.

The consumer guide now names the actual workspace Memory 0.35 family and leaves
the patch selection with Cargo.lock. Both consumer guides distinguish completed
v0.5.6 five-scenario acceptance from the current watchdog scenario's source-bound
qualification. This documentation-only compatible patch is tracked in
[#22](https://github.com/dragginzgame/ic-jobs/issues/22). The complete pending batch
remains compatible **0.5.10**: Jobs API, runtime, stored formats and declared Rust
floor are unchanged. Package versions and the real index/refs are unchanged;
no commit, release, publication or new hosted dispatch is performed. Incoming handoff work is
preserved. Evidence: `/tmp/ic-jobs-0510-guide.yvnjiftg/`.

## Delivered 0.5.9 — acceptance remaining

Jobs **0.5.9** is delivered at
`b289e443d4955ffe17d9d97a232eb83432340395`, matching public main and the annotated
`v0.5.9` tag. The checkout was clean at inspection. Complete local tool admission
passes, including the formerly missing common Rust selection. A later IC
admission refusal is retained and does not reproduce on the diagnostic recheck;
its cause is not established. Evidence: `/tmp/ic-jobs-059-acceptance.aoug83g5/`.
No installer or build is run by this inspection.

[Current native CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38133877061)
passes Linux, native macOS Intel and Apple Silicon at this exact source. Each
host reports the affected local archive-retention fixture passed, completing
[#18](https://github.com/dragginzgame/ic-jobs/issues/18). This accepts the older
Shared 0.5.0 adoption; new-source preparation/acceptance remains with
[#21](https://github.com/dragginzgame/ic-jobs/issues/21).

The maintainer explicitly selected the existing recovery workflow on `v0.5.9`.
[Run 38134824424](https://github.com/dragginzgame/ic-jobs/actions/runs/38134824424)
is dispatched at the exact delivered commit, covering the sixth watchdog
scenario under [#20](https://github.com/dragginzgame/ic-jobs/issues/20). Acceptance
requires inspecting uploaded evidence separately for all three hosts and all
original/O3/Os/Oz variants. Old five-scenario v0.5.6 results do not qualify these
inputs. This selection authorizes that hosted gate; local tests, builds, lint
and full CI remain excluded by the recorded exception. No additional release
effect or sibling source edit is performed.

Linux recovery passes, with its uploaded archive inspected against exact
v0.5.9 source/lock/toolchain/snapshot. All six maintained scenarios pass for
original/Binaryen 133 O3/Os/Oz: **24 live results**, actual Wasm checksums and real
PocketIC server logs verified. The
[Linux archive](https://github.com/dragginzgame/ic-jobs/actions/runs/38134824424/artifacts/11700832504)
SHA-256 matches GitHub's digest:
`33a0ca96ae7894295a6ea98380d3617143869cb56cd6d768a3ffb4a16c2bc7e0`.
Testkit 0.33.5/tool identities are admission receipts; executable payloads are
not uploaded. The hidden attempt snapshot is omitted by upload; its workflow-level
copy exactly matches released source. Intel recovery is running and Apple Silicon
queued at inspection. Review: `linux-review.json` in the current evidence directory.
Keep #20 open until both macOS archives are inspected; this does not qualify the
pending snapshot or incoming graph.

## Earlier 0.5.9 preparation — Shared 0.5.0 and consumer boundary work

Canonically adopted Shared **0.5.0** at
`a306180ce534227fcb98b882fba9a526d5d747e0` from an isolated clean source clone.
The preview preserved consumer files and the real index, and identified one
required companion, `scripts/dev/tool-store.sh`. Explicit export retains all
82 prior selections and adds that helper: 83 files, with nine existing files
updated from the prepared 0.4.1 snapshot. Baseline, host/IC installers, evidence
selector and storage documentation advance together. The existing failure
action delegates to that selector, so no Jobs-specific storage manager is added.
Tracking: [#21](https://github.com/dragginzgame/ic-jobs/issues/21), with upstream
storage ownership in [Shared #120](https://github.com/dragginzgame/shared-tooling/issues/120).

Real `make install-host-tools` and `make install-ic-tools` reuse verified bundles
in the default per-user store. Checkout-local `.tools/host/bin` and
`.tools/ic/bin` remain the command paths; repository references bind each exact
bundle generation. Previous physical local bundles remain byte-identical.
`make tools-check` passes host and IC admission, then refuses the missing common
Rust set with status 2. Preparing `.tools/rust/common/bin` requires Cargo builds
and remains pending under the recorded maintainer validation exception; no
Rust installer, tests, builds, lint, full gate or live qualification is run.
When that preparation is selected, run `make install-rust-tools` followed by
`make tools-check`. Selected Testkit CLI paths and PocketIC ownership remain
unchanged. Use the same `SHARED_TOOLING_STORE` setting for setup, checking and
collection if overriding the documented default.

Evidence: `/tmp/ic-jobs-shared-050.d1d7kbng/`, including preview/export logs,
real setup/check logs, prior-bundle hashes and source/index preservation records.
Snapshot integrity, dependency declarations, document links, Bash parsing and
whitespace checks pass separately from execution qualification. No native
macOS, hook/archive regression or new six-scenario recovery acceptance is
claimed for this working tree. The earlier 0.4.1 preview/adoption evidence remains
at `/tmp/ic-jobs-shared-041.mvQ7qn/`; the previously reported Shared-only retention
fixture mismatch is corrected in 0.5.0 source, without claiming a passing hosted
gate. Exact-source [Shared CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38132790945)
remains pending at inspection.

This contributor-tooling adoption stays compatible for Jobs: pending release
remains **0.5.9**, together with the already applied consumer boundary work in
[#20](https://github.com/dragginzgame/ic-jobs/issues/20). Library API, runtime,
stored formats and Rust 1.88 floor are unchanged. Incoming consumer/harness
source and Cargo.lock are preserved. Package versions, real index and Git refs
are unchanged; no commit, push, release, publication, hosted dispatch or sibling
source edit is performed. Required consumer qualification and delivery remain
with the owning issues; prior v0.5.6 recovery results do not qualify new inputs.

## Delivered 0.5.8 — composition fix and hosted recovery acceptance

Jobs **0.5.8** is delivered at
`06f359debe22afe0c2bd0d3017e0a26e0d7f4e3a`, matching public main and annotated
`v0.5.8`. Both README guides match the qualified composition repair in
[#19](https://github.com/dragginzgame/ic-jobs/issues/19), now accepted. Delivered
dependency selections match the preceding locally qualified graph except local
package versions. That evidence remains bound to its tested source; fresh
[0.5.8 Linux CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38127816171)
also passes. Native Apple Silicon CI is running, Intel queued. Both queued
macOS jobs in the previous 0.5.7 run were cancelled by this delivery; native
tooling acceptance remains in [#18](https://github.com/dragginzgame/ic-jobs/issues/18).

The explicitly authorized [v0.5.6 hosted recovery run](https://github.com/dragginzgame/ic-jobs/actions/runs/38123372599)
passes all three hosts: Linux, native macOS Intel and Apple Silicon. All three
uploaded archives are inspected against exact source
`e47bda833e91fe717d415dae9374d28b22fbba3a` and its graph/snapshot/tool records;
all 60 maintained live results and actual Wasm checksums verify, with real
PocketIC server logs. This completes consumer/recovery acceptance in
[#11](https://github.com/dragginzgame/ic-jobs/issues/11). The final
[Intel archive](https://github.com/dragginzgame/ic-jobs/actions/runs/38123372599/artifacts/11698965883)
SHA-256 matches GitHub's digest:
`04644fa518c14cdcb207a38936f4371082af9bac1fb4b58598ba864f32a9e6c7`.
Executable payloads are not uploaded; their identities are admission receipts.
Review evidence: `/tmp/ic-jobs-058-acceptance.dRGBRq/`, including
`hosted-all-hosts-review.json`. Core, consumer and harness Rust source remains
unchanged since the hosted tag; its older graph is not relabelled as 0.5.8.

This acceptance update changes only the handoff. No new dependency selection,
full local gate, live execution, hosted dispatch, sibling edit, commit, push,
release or publication is performed.

## Earlier preparation for delivered 0.5.8

## Pending 0.5.9 — Jobs/Timers failure boundary

Released Jobs **0.5.8** is `06f359debe22afe0c2bd0d3017e0a26e0d7f4e3a`.
The maintainer authorized the boundary-audit improvements from the Timers session.
The unpublished consumer now records a known no-effect counter overflow as the
exact job's PermanentFailure, retaining unrelated work and the shared two-attempt
batch bound. Native coverage checks both failure orders, another overdue job,
terminal recurrence and reconstruction. Empty watchdog batches report NoWork;
handled attempts remain distinct from confirmed external effects.

The existing live harness adds a sixth scenario: actual watchdog work writes a
stable batch then traps, the host observes the real trap log and rolled-back
records/effects, and clearing the fault without rearming requires the surviving
successor to progress. It also checks the next global deadline and terminal
state after upgrade. Original/Binaryen O3/Os/Oz runs select this case through the
existing harness, without another server, provider or qualification path.
Prior five-scenario evidence does not qualify these new inputs.

Both maintained guides and the safety boundary explain that a callback batch
limit is not a backlog/cycle or fairness guarantee. Skip/CatchUp semantics and
earliest-deadline ordering are unchanged. The compatible pending **0.5.9** changes
the unpublished consumer and qualification/guidance only; library APIs, JobRecord
fields, storage formats, dependency selections and package versions remain
unchanged. No Timers runtime change is needed.

Tracking: [#20](https://github.com/dragginzgame/ic-jobs/issues/20), including
qualification of the new watchdog scenario. The completed five-case hosted
acceptance in [#11](https://github.com/dragginzgame/ic-jobs/issues/11) does not
qualify this changed harness.
Tests, builds, lint and live qualification were not run by this Timers session;
the maintainer's validation exception remains in force. Source/diff and document
checks do not establish execution. No commit, release or publication was performed.

## Pending 0.5.8 — timer composition and current graph qualification

Jobs **0.5.7** is delivered at
`06ef135f8c2167438cce3c9dc9ab321e12cac653`, matching public main and annotated
`v0.5.7`. The incoming lock advances only Timers 0.17.7 to **0.17.8** from that
release, preserving Memory 0.35.7, Metrics 0.6.1, Testkit 0.33.3 and all four
Host 0.13.0 crates. Lock SHA-256:
`2e8905fbdd43663b294aa978f5be32059d812cfa1a80b0360535b9246723cdf6`.
Published Timers 0.17.8 changes tests/docs only; Memory 0.35.7, Metrics 0.6.1
and Testkit 0.33.3 Rust trees match their preceding published versions. Host
0.13.0 removes a filesystem convenience function that Jobs does not call;
the other three Host Rust trees match 0.12.10. Dependencies remain within the
advertised Rust 1.88 floor, qualified separately for core/timers native/Wasm.

Both maintained README guides now align direct timer owners with the actual
Timers 0.17 workspace dependency and recommend its canonical
`MeasurementSummary` re-export, removing stale Timers 0.16 / Metrics 0.3
composition guidance. See [#19](https://github.com/dragginzgame/ic-jobs/issues/19).
This selects compatible pending **0.5.8**; Job APIs, retained records, restore
validation, scheduling and consumer authority are unchanged. Package versions
remain 0.5.7. Evidence: `/tmp/ic-jobs-issues-057.RgKMfc/`. The offline CLI check
refuses missing lock-selected Testkit 0.33.3; explicit setup then prepares it.

Fresh full `make ci` passes, including actual tooling/hook fixtures, separate
Rust 1.88 core/timers native/Wasm checks, consumers, Clippy, docs and packaging.
All five maintained live scenarios pass on each original/Binaryen 133 O3/Os/Oz
variant under parallel genuine Linux Bash 3.2.57/Make 3.81, without jobserver
warnings. Frozen source/lock hashes match after both gates; final handoff
documentation is checked separately. Live attempt `/tmp/jobs-recovery.nfnNMp/`
retains actual Wasm/tool/server evidence; original Wasm SHA-256 is
`ba7c64253062abd9e05bf95b67ddaa9a0aac565024f9ca422801e9a4a5a08a69`.
Normal core/canister Wasm graphs contain no Host crates. The selected CLI's
published installation graph also uses Host 0.13.0, with its own retained
installation log and selection receipt. The README repair awaits delivery.

The authorized [v0.5.6 recovery run](https://github.com/dragginzgame/ic-jobs/actions/runs/38123372599)
now passes Linux and native Apple Silicon, with both uploaded archives inspected.
The [Apple Silicon archive](https://github.com/dragginzgame/ic-jobs/actions/runs/38123372599/artifacts/11696709293)
matches the exact tagged source/graph/snapshot, tool admission records, all four
actual Wasm checksums and all 20 maintained live results, including real PocketIC
logs. Archive SHA-256 matches GitHub's digest:
`eca37eeab697990311e1e0d2c5ca0162c43770afc62acf68dbb55270fc0031bd`.
Review: `hosted-apple-silicon-review.json` in the evidence directory above.
Executable payloads are not uploaded; their identities remain admission receipts.
Intel recovery is still queued; acceptance remains open. The hosted v0.5.6
graph/evidence remains separate from the current local graph.
The earlier v0.5.6 native Intel CI was cancelled by delivery of v0.5.7.
Acceptance in [#18](https://github.com/dragginzgame/ic-jobs/issues/18) now awaits
the [v0.5.7 native CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38125188373),
which passes Linux with both macOS jobs queued at inspection. Recovery
acceptance remains in [#11](https://github.com/dragginzgame/ic-jobs/issues/11).
No new hosted dispatch, sibling source edit, commit, push, release or publication
is performed by this local batch.

## Earlier preparation for delivered 0.5.7

## Pending 0.5.7 — Testkit preparation and recovery qualification

Jobs **0.5.6** is delivered at
`e47bda833e91fe717d415dae9374d28b22fbba3a`, matching public main. Its
[native CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38121992741)
passes Linux and native Apple Silicon; Intel remains queued. Delivery of the
fixture-evidence repair in [#18](https://github.com/dragginzgame/ic-jobs/issues/18)
is complete, with Intel acceptance still pending.

The local qualification used Memory 0.35.6, Metrics 0.5.6, Timers 0.17.6,
Testkit 0.33.2 and four Host 0.12.10 crates. SHA-256:
`a416f6cd6b3bf1a68975207ad9494da12e33dfc5e33138cd7a6e6a8fac7d4e34`.
Only Testkit changes from the delivered lock. Published Testkit 0.33.2 Rust
source matches 0.33.1 and retains Rust 1.88. The actual offline check correctly
refuses the missing 0.33.2 CLI, then explicit setup prepares and admits the
selected CLI and existing Testkit-owned PocketIC bundle. Earlier CLI slots and
evidence are preserved. The compatible graph/tool selection chooses pending
0.5.7, retaining Job APIs, consumer authority and stored-record contracts.

Evidence: `/tmp/ic-jobs-issues-056.15LnzE/`. Fresh full `make ci` on that graph
passes, including separate Rust 1.88 core/timers native/Wasm checks, the consumer/
harness, Clippy, docs and packaging. All five live recovery scenarios pass on
each original/Binaryen 133 `-O3`, `-Os`, `-Oz` variant under parallel genuine
Linux Bash 3.2.57/Make 3.81 without jobserver warnings. Attempt
`/tmp/jobs-recovery.x5QDFi/` retains exact variant/server/tool artifacts; original
Wasm SHA-256 is `46e408d1e14029cb478e9e1d030cc6c0935868f7455ee44c69a3b57749842b89`.
Frozen source hashes match after both gates; final handoff checks follow separately.
Normal core/canister Wasm graphs contain no Host crates. The selected CLI's own
published installation graph uses Host 0.12.9, independently of Jobs' 0.12.10 graph.

A later incoming lock edit during hosted monitoring selects Metrics 0.6.0 and
Timers 0.17.7, retaining the other selections. It is preserved separately as
`late-incoming.lock` in the evidence directory; SHA-256
`8ffa757375434bdec2a98137b81f88bebf7ba38c2687f4ff23cd82244cd36096`.
The CI/live results above do not qualify this later graph. Its cache preparation,
source review and affected qualification remain required before delivery.

The maintainer explicitly authorized hosted recovery on annotated `v0.5.6`.
Tag identity is verified against delivered `e47bda833e91fe717d415dae9374d28b22fbba3a`.
[Run 38123372599](https://github.com/dragginzgame/ic-jobs/actions/runs/38123372599)
passes Linux; both macOS jobs remain queued. The Linux archive
([artifact 11696247627](https://github.com/dragginzgame/ic-jobs/actions/runs/38123372599/artifacts/11696247627))
is inspected: exact tagged source, lock and snapshot match; all five maintained
scenarios pass on each original/O3/Os/Oz artifact, with recomputed Wasm checksums
and real PocketIC server logs. Archive SHA-256 matches GitHub's digest:
`ff1589010173536cb7104847b2793d58f57c5b6bb1a3ac6192ac5543ffc4bac0`.
Original hosted Wasm SHA-256:
`013bc01255287fe62672f7fb88e018071f8411cd66a3ab678af083265bea9665`.
The uploaded Testkit CLI/Binaryen records are admission receipts; executable
payloads are not uploaded. Its released graph selects Testkit 0.33.1; the local
0.33.2 evidence is separate. Review: `hosted-linux-review.json` in the evidence
directory above.
Hosted optimized recovery acceptance remains in
[#11](https://github.com/dragginzgame/ic-jobs/issues/11) until actual native
execution and uploaded artifact inspection complete. Neither queued jobs nor
local Linux execution qualify the macOS recovery gate.
Package versions remain 0.5.6. No sibling source edit, commit, push, release or
publication is performed by this local batch.

## Earlier preparation for delivered 0.5.6

## Pending 0.5.6 — Shared 0.3.11 metadata correction

Canonically adopted Shared **0.3.11** at
`db143ae9ae140eb66826c413360723d23c078f66`, preserving the 82-file selection.
Committed VERSION and snapshot annotation now agree; the owning version report
[Shared #115](https://github.com/dragginzgame/shared-tooling/issues/115) is closed.
Only snapshot identity/version metadata changes among selected shared inputs;
all vendored file bytes and modes match the previously qualified revision.
The other upstream change isolates diagnostics in its unselected regression
fixture, leaving the installed validation runner unchanged.

Pending Jobs stays compatible **0.5.6**. The incoming root lock, including Host
0.12.10 and newly selected Memory 0.35.6, is preserved byte-for-byte. Its SHA-256
is `b1f470fde4ccbc1673573a2deef986eaff04c5beb26346be45c961d930d7dc9f`.
Memory 0.35.6 is the only dependency change since the prior qualified graph;
the previous full CI/live passes used 0.35.5 and are not reassigned to it.
This Shared metadata refresh needs focused integrity/documentation checks;
snapshot integrity, local links, pin declarations, release-adapter checks and
diff checks pass. The upstream isolated-diagnostic regression passes on current
Bash and genuine Linux Bash 3.2; it uses substitute Make targets. All 82 vendored
hashes and executable modes match the prior revision. New-source
delivery/native acceptance remains in [#18](https://github.com/dragginzgame/ic-jobs/issues/18),
hosted recovery acceptance in [#11](https://github.com/dragginzgame/ic-jobs/issues/11).
Evidence: `/tmp/ic-jobs-shared-0311.5OZpsp/`.
No sibling edit, dependency change, full gate, live run, commit, push, release,
publication or hosted dispatch occurs in this metadata refresh.

## Earlier pending 0.5.6 qualification at the .10-labelled Shared revision

## Pending 0.5.6 — latest Shared revision and Host 0.12.10

The compatible 0.5.6 batch continues on delivered Jobs 0.5.5
`9b3abf4fdeb8438de218cba7543660c7eb908386`. Shared's reviewed commit labelled
0.3.10 is `37af29b2f8fb4532744b600c5f85ce716a8b57cf`, adopted through the
canonical exporter with the same 82-file selection. Its committed VERSION is
still 0.3.9, so the canonical snapshot correctly retains that annotation; the
owning metadata report is [Shared #115](https://github.com/dragginzgame/shared-tooling/issues/115).
No annotation was patched locally. The release-command checker now handles
relative/symlinked TMPDIR and anchors nested Make probes. The prior Jobs checker
fails the relative-root reproduction; the new actual Jobs checker and canonical
release fixture pass on current Bash and genuine Linux Bash 3.2.57/Make 3.81.
Jobs has no dollar-prefixed derive macros requiring the new protection guidance.

Selected all four published Host crates **0.12.10**, reviewed at released source
`5074de1792f920112c6539629605a6624f573863`. Their published Rust trees match
0.12.9 and retain Rust 1.88. Only the four Host records change from the incoming
lock; unrelated dependency edges are preserved and admitted by locked Cargo
metadata. Cargo's intermediate generated lock remains retained separately.
Memory 0.35.5, Timers 0.17.6, Metrics 0.5.5, Testkit 0.33.1 and serde_json 1.0.152
retain their selections. Selected lock SHA-256 is
`d4879e3d2eb28712ec54595d1a225d651048846550571ad38a3e411d7928c981`.

Evidence: `/tmp/ic-jobs-shared-0310.Obw28A/`. Focused relative/aliased temporary
path qualification and fresh full `make ci` pass, including separate Rust 1.88
core/timers native/Wasm checks, the host harness, docs and packaging. All five
live recovery scenarios pass afresh for each original/Binaryen 133 optimized
`-O3`, `-Os`, `-Oz` variant under parallel Bash 3.2/Make 3.81 without jobserver
warnings. Source hashes match after CI/live, and normal core/canister Wasm
graphs contain no Host crates. Attempt `/tmp/jobs-recovery.bO51Fc/` retains the
exact variants/server/tool evidence; original Wasm SHA-256 is
`62fed594a02659266d7bdec8d47bd2cfcd1fe591de6d5cb674ac9001c54df93c`.
Final handoff documentation checks follow separately.

Host .10 [CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38120032068)
now passes Linux, both macOS architectures and MSRV. Shared's exact-source
[CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38120869060)
passes Linux/lint while both macOS jobs remain running; earlier passes are not
reassigned. The prior 0.3.9 qualification below
remains bound to its source and graph. Jobs' fixture delivery/native acceptance
remains in [#18](https://github.com/dragginzgame/ic-jobs/issues/18), hosted optimized
recovery acceptance in [#11](https://github.com/dragginzgame/ic-jobs/issues/11).
No package version mutation, sibling source edit, commit, push, release,
publication or hosted dispatch occurred.

## Earlier pending 0.5.6 qualification at Shared 0.3.9 / Host 0.12.9

## Pending 0.5.6 — Shared setup/diagnostics and native fixture evidence

Delivered Jobs **0.5.5** is `9b3abf4fdeb8438de218cba7543660c7eb908386`.
Its [complete CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38072923082)
passes Linux and native macOS Intel/Apple Silicon. This closes the delivered
assertion/hook acceptance in [#16](https://github.com/dragginzgame/ic-jobs/issues/16)
and [#17](https://github.com/dragginzgame/ic-jobs/issues/17).

Pending compatible **0.5.6** adopts reviewed Shared **0.3.9** at
`1ed2ca523186641e0e0cdf77e3cb10814c22a430` through the canonical exporter,
preserving the 82-file selection. Rust-tool probes/installation now share the
selected consumer's Cargo configuration; retained validation diagnostics use
the same classifier as live output. Native CI selects the shared collector's
existing `portable-fixtures` directory as TMPDIR. The maintained evidence test
executes the actual workflow command with substituted failing Make, preserves
its status 37 and verifies its inner fixture log in the actual local archive.
Delivery/native acceptance of that repair remains in
[#18](https://github.com/dragginzgame/ic-jobs/issues/18).

The incoming lock selects Memory **0.35.5**, Timers **0.17.6**, Metrics **0.5.5**,
Testkit **0.33.1**, four Host crates **0.12.9** and serde_json **1.0.152**;
SHA-256 `7db925f4c9de0a9ab812856b88230bc817b1be24e09daff3a7f1ca82d9903cc0`.
It is preserved. Published Host/Timers Rust trees match their prior releases,
and Host retains Rust 1.88. Host's macOS portability fix is delivered at
`8cc85c53cdd59701a7352f566425b3faf195a37b`; its
[CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38073002651)
passes all native hosts and MSRV. Jobs' count checks already normalize padded
counts; no equivalent source repair is needed. Host remains outside normal
core/canister Wasm graphs. Job/storage/effect contracts retain consumer ownership.

Evidence: `/tmp/ic-jobs-shared-039.cncWCY/`. Current-shell Shared installer/
diagnostic fixtures and Jobs' actual workflow/archive regression pass, as do
genuine Linux Bash 3.2.57 checks and the Jobs evidence fixture under Make 3.81.
Outside-consumer admission reuses the actual selected Testkit 0.33.1 CLI.
Fresh `make ci` passes, including separate Rust 1.88 core/timers native/Wasm,
consumer/harness checks, Clippy, docs and packaging. All five live recovery
scenarios pass on each original/Binaryen 133 `-O3`, `-Os`, `-Oz` variant under
parallel Bash 3.2/Make 3.81 without jobserver warnings. Frozen source hashes match
after both gates; final handoff checks follow separately.

`live-bash32.log` retains the sandboxed PocketIC loopback-bind failure and its
attempt `jobs-recovery.HjWpQA/`. The authorized local-server rerun passes in
`live-local-server.log`, retaining attempt `/tmp/jobs-recovery.XbFAEv/` and exact
variant/server/tool artifacts. Original Wasm SHA-256 is
`62fed594a02659266d7bdec8d47bd2cfcd1fe591de6d5cb674ac9001c54df93c`.
This is Linux live/shell evidence; native macOS acceptance of the new source
remains separate. Earlier passing evidence below is bound to its own graph.
Hosted optimized recovery acceptance remains in
[#11](https://github.com/dragginzgame/ic-jobs/issues/11); no recovery workflow runs
exist at inspection. Package versions remain 0.5.5. No sibling source edit,
commit, push, release, publication or hosted dispatch occurred.

## Historical preparation for 0.5.5

Jobs **0.5.4** is delivered at `d5e5aaf6e8450072b90357c3ea3eba3913bc24d7`,
matching public main and annotated `v0.5.4`. Its actual released graph includes
Memory 0.35.4, Metrics 0.5.5, Testkit 0.33.1 and Host 0.12.7; earlier local
qualification below remains bound to its original selections. Exact-source
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38068172788) passes Linux;
Intel macOS was cancelled and Apple Silicon remains queued.

Pending compatible **0.5.5** adds `make formatting-hook-check` to full CI on all
three native hosts. The canonical hook fixture tests failed Git observations,
partial staging, formatter failures and preservation; the Jobs adapter selects
its actual Rust/manifest/Make inputs and an unsorted copy of the same manifest.
All Git changes occur in disposable copies. Shared **0.3.8** at
`67285b28a98b7c4211ad32de726709d4e87edea4` is adopted canonically; the snapshot
now has 82 files, adding the two hook checkers and their declared frontend
fixture companion. README references now reflect the current allocator, snapshot
and canister qualification.

The incoming root lock is preserved: Memory **0.35.4**, Timers **0.17.5**,
Metrics **0.5.5**, Testkit **0.33.1**, Host **0.12.8**, serde_json **1.0.152**;
SHA-256 `0f87aadb5d67f4bdd30b9785c6c6d2b9e6b638420a3617a18232cc0dda1c846c`.
Host's published Rust trees match 0.12.7; Timers' match 0.17.4. Reviewed Memory's
policy documentation and Testkit's added invalid-destination tests separately.
Host remains outside normal core/canister Wasm graphs. Job APIs, stored records
and consumer effect/storage authority retain their contracts.

The first CI attempt stopped after a concurrent lock change selected uncached
Timers 0.17.5; that failed run and its earlier input hashes remain retained.
Fresh cache preparation, cold lock-selected Testkit 0.33.1 CLI setup and server
admission pass. Offline admission correctly refused the missing 0.33.1 CLI
before setup, despite the existing 0.33.0 installation.
Actual hook qualification passes on current Bash/Make and genuine Linux Bash
3.2.57/Make 3.81. Fresh full `make ci` passes, including separate Rust 1.88 core/
timers native/Wasm checks and packaging. All five live scenarios pass on each
original and Binaryen 133 `-O3`, `-Os`, `-Oz` output under parallel Bash 3.2/
Make 3.81 without jobserver warnings. Source hashes match after CI/live.

Evidence lives under `/tmp/ic-jobs-hook-qualification.EnxLxR/`; its
`graph-timers-0175/` contains the passing CI/live logs, frozen graph, CLI receipt
and source hashes. Attempt `jobs-recovery.XlhOGX/` retains exact variant/server
artifacts; original Wasm SHA-256 is
`5b5f1ac1fd336d9d11b5c0b2dedf8ca5d6e0d2c7210fee22893bd848ee48a6b6`.
Final documentation checks/hash binding follow separately. New native Jobs
acceptance remains in [#17](https://github.com/dragginzgame/ic-jobs/issues/17);
hosted optimized recovery acceptance remains in [#11](https://github.com/dragginzgame/ic-jobs/issues/11).
Host 0.12.8's Apple Silicon job passes both hook checks, then fails its own
fixture-retention test; the [owning report](https://github.com/dragginzgame/ic-host-tooling/issues/57#issuecomment-6100282418)
records the log and retained artifact. No sibling repair was attempted. Package
metadata remains 0.5.4; this work is local, with no commit, push, release,
publication or hosted dispatch.

## Earlier pending 0.5.4 qualification at Shared 0.3.7

Pending compatible **0.5.4** now adopts committed Shared **0.3.7** at
`34e5ad7aac3599306c9572bb547f2239d09df1a3`, through its canonical exporter with
the existing 79-file selection. Shared rejects failed Cargo-tool activation
assertions explicitly on Bash 3.2. Jobs' three tooling fixtures, release metadata
adapter and live Wasm-file admission use explicit mandatory failures too.
Contradicting an observed status now stops the fixture and retains evidence;
release version mismatches stop before cache/setup or metadata mutation.
Existing setup, release and recovery command contracts remain compatible.

The incoming root selection is preserved byte-for-byte: Memory **0.35.3**, Timers
**0.17.4**, Metrics **0.5.4**, Testkit **0.33.0** and all four Host crates
**0.12.6**. Lock SHA-256 is
`3923304ae00bfe855ce268d25ec035b23783d66877b24c919a62c159c30055d9`.
Reviewed public Host release `5f356effea97fbc31dfcca5b9f1b2834f35325a9`;
the four published Rust source trees match 0.12.5 and retain Rust 1.88.0.
Host remains transitive through the host harness and absent from both normal
core/canister Wasm graphs. Job records, effect disposition and consumer storage
authority retain their established contracts.

Both validation workflows keep only the latest run per workflow/ref, preserving
all three native hosts. These incoming workflow changes satisfy Shared's new
policy; cancelled hosts do not qualify their source. Delivered Jobs **0.5.3**
at `d4468fb423c97550c74a2e87a02f8b3e70361ef5` now has passing
[CI on all three hosts](https://github.com/dragginzgame/ic-jobs/actions/runs/38059981517).
That evidence belongs to the delivered graph, not this local repair.

Actual Jobs fixtures pass on current Bash/Make and genuine Linux Bash 3.2.57/
Make 3.81. They cover failed status assertions with retained evidence, original
nonzero statuses, version refusal before effects and invalid Wasm admission
before attempt/server dispatch. Canonical exact-source installer fixtures pass
on both profiles with substitute Cargo. Real selected Testkit CLI/server setup
and admission reuse the prepared 0.33.0/16.1.0 installation. Full local `make ci`
passes, including separate Rust 1.88 core/timers native/Wasm checks and packaging.
All five live scenarios pass afresh on each original and Binaryen 133 `-O3`,
`-Os`, `-Oz` output under parallel Bash 3.2/Make 3.81 without jobserver warnings.

Evidence: `/tmp/ic-jobs-shared-037.FdcYDN/`, including the incoming graph/patch,
snapshot refresh, before reproductions, Jobs/producer fixture logs, package
source review, CLI admission, `ci.log`, `live-bash32.log` and source hashes.
Live attempt `jobs-recovery.YPdQlo/` retains exact variant/server/tool artifacts;
original Wasm SHA-256 remains
`5101fe9b88e51d030ec52a3b3fa818625598a8257c6d8c6389aaf45d5e38707d`.
Source hashes match after CI/live. Final handoff checks/hashes follow separately.
Earlier evidence below remains bound to its original inputs. Local adoption and
assertion repair are recorded in [#17](https://github.com/dragginzgame/ic-jobs/issues/17)
and [#16](https://github.com/dragginzgame/ic-jobs/issues/16); native macOS acceptance
of the new source and hosted optimized recovery acceptance remain pending.
Package metadata remains 0.5.3; no commit, push, release, publication, sibling edit
or hosted dispatch occurred.

## Earlier pending 0.5.4 qualification at Shared 0.3.6

Public Jobs **0.5.3** is at `d4468fb423c97550c74a2e87a02f8b3e70361ef5`.
Pending compatible **0.5.4** adopts Shared **0.3.6** at
`0604bfd730ec7ec288cd2cfdad217a0d42bf256b` through the canonical exporter,
retaining the same 79-file selection. Testkit setup, offline admission and the
live/manual recovery callers now select the CLI directly from `Cargo.lock`;
the duplicate environment-file pin is removed. Existing Make targets retain
their contracts. Rerun `make install-testkit-tools` after a locked Testkit change.
Shared owns parsing, strict admission, installation and selection receipts;
Testkit still owns PocketIC provisioning and lifecycle.

The incoming manifest/lock is preserved: Memory **0.35.3**, Timers **0.17.4**,
Metrics **0.5.4**, Testkit **0.33.0** and four Host crates **0.12.5**. Lock SHA-256
is `2eb1cc33741697393e9b49eb6fe9c93aafc477e8901a089db385a93c2e0cbe8b`.
Host remains transitive in the host harness, outside the Jobs/canister graph.
Testkit's scoped transport-reset classification does not change the direct
PocketIC harness or its typed trap assertions. Jobs APIs, stored records,
consumer authority, uncertain-effect blocking and Rust 1.88 remain unchanged.

Real cold CLI setup and CLI/server admission pass; repeated CLI setup/admission
reuse the installation with Cargo installation and downloads disabled. The old
0.32.2 slot and receipt still verify. Actual Jobs caller fixtures reject changed,
unprepared lock selections before Cargo/PocketIC dispatch on both shell profiles.
Canonical hook/installer fixtures and Jobs' actual hook/formatting admission pass
on current Bash/Make and genuine Linux Bash 3.2.57/Make 3.81. These producer
fixtures use substitute build tools; actual installation and live results are
recorded separately.

Full local `make ci` passes, including separate Rust 1.88 core/timers native/Wasm
checks and packaging. All five live recovery scenarios pass on each original and
Binaryen 133 `-O3`, `-Os`, `-Oz` output under parallel Bash 3.2/Make 3.81, without
jobserver warnings. Source hashes match after CI and live execution. Evidence is
retained under `/tmp/ic-jobs-shared-036.bgUa2J/`: `ci.log`, `live-bash32.log`,
`qualified-source.sha256`, CLI selection, incoming graph and focused fixture logs.
Attempt `jobs-recovery.AXXHen/` retains actual Wasm/hashes and per-variant logs;
original Wasm SHA-256 is
`5101fe9b88e51d030ec52a3b3fa818625598a8257c6d8c6389aaf45d5e38707d`.
Final documentation changes are checked and hashed separately after qualification.

[#17](https://github.com/dragginzgame/ic-jobs/issues/17) records this local repair;
[#11](https://github.com/dragginzgame/ic-jobs/issues/11) retains hosted recovery
and native macOS acceptance. Earlier evidence below is not relabelled. Historical
complete-toolset acceptance in [#15](https://github.com/dragginzgame/ic-jobs/issues/15)
is closed against the three passing native hosts at Jobs 0.5.0. The newer
[0.5.3 CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38059981517)
passes Linux; both macOS jobs are running. The 0.5.2 macOS jobs were cancelled,
so [#16](https://github.com/dragginzgame/ic-jobs/issues/16) retains native-host
acceptance. Package metadata remains 0.5.3; this batch is local and uncommitted.

## Earlier 0.5.3 qualification before delivery

Pending compatible **0.5.3** aligns the selected Testkit CLI to the incoming
**0.32.2** harness library; rerun `make install-testkit-tools`. The existing
Shared 0.3.5 snapshot refresh remains local, with all 79 selected file hashes
identical to delivered Shared 0.3.4. Public Jobs main remains released 0.5.2 at
`6516eba4f54f69c50a1c1bcab38fa65354ba8b46`; its exact-source CI now passes
Linux, while both macOS jobs remain queued. The Shared 0.3.5 snapshot refresh
and CLI changes remain uncommitted here.

The incoming lock is preserved: Memory **0.35.3**, Timers **0.17.3**, Metrics
**0.5.3**, Testkit **0.32.2** and four Host crates **0.12.4**, SHA-256
`f0929622a8977029d94d31aed7ffc48d44e7781b57ee9a7115c768a1b9850fac`.
Reviewed published producer commits: Testkit
`9e697b6363441e3fa96477adae5b07820461c3c5`, Memory
`b1fd17cf905fee32675f165f774da2def55b2059`, Timers
`4cc3c64e6b773d73edb66f6bdf2d91cce5e68790`, Host
`5400f159474cebac1ec7ae7c8763abfd258bde03`. Their packaged Rust source trees
match the prior patch versions byte-for-byte; tooling/qualification changed.
Jobs acquires no direct Host dependency or new lifecycle/storage authority.
Job APIs, stored records, scheduling policy and Rust 1.88 remain unchanged.

Locked cache preparation and selected CLI/server admission pass. Full local
`make ci` passes, including separate Rust 1.88 core/timers native/Wasm checks,
native consumers, linked Wasm/harness builds, strict Clippy, docs and packaging.
All five live recovery scenarios pass for the original and Binaryen 133 `-O3`,
`-Os`, `-Oz` Wasm outputs, using genuine Linux Bash 3.2.57/Make 3.81 with parallel
Make and no jobserver warnings. Source hashes match after CI and live execution.

Evidence is retained under `/tmp/ic-jobs-testkit-0322.qotFQz/`: `ci.log`,
`live-bash32.log`, `metadata.json`, `source-review.txt`, selected CLI receipt,
incoming lock and source hashes. Attempt `jobs-recovery.gYgMoY/` retains actual
Wasm/hashes and per-variant optimizer/test/server logs; original Wasm SHA-256 is
`19f44d2f8c83427dff68b3aee7a859c18e701067c5e27df0742cd3d56d00a057`.
Final handoff checks/hash binding follow qualification. Earlier graph evidence
below remains separately bound. Native macOS and hosted recovery/artifact
acceptance remain pending in #11, #15 and #16. No commit, push, release,
publication, sibling edit or hosted dispatch occurred; package metadata remains
0.5.2.

## Earlier Shared 0.3.5 snapshot-only qualification

IC Jobs **0.5.2** is pushed at
`6516eba4f54f69c50a1c1bcab38fa65354ba8b46`, matching public main. Its
[exact-source CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38056524883)
is running on Linux; both macOS jobs are queued. The updated optimized recovery
workflow is delivered, but hosted execution/upload remain unqualified.

The maintainer-requested Shared Tooling **0.3.5** adoption selects committed
`a744d7f1990b9e1451ef45cd6d495de00a141cd3` through the canonical exporter from
clean detached `/tmp/ic-jobs-shared-035.bAP8SC/shared`. All 79 selected file
hashes/modes match the prior 0.3.4 snapshot; only its revision/version and the
local AGENTS reference change. Newer dirty upstream hook edits are excluded.
Binaryen 133 and every executable/dependency selection remain unchanged, so
this adoption needs no reinstall, product changes or governance-only changelog
entry. Package metadata remains 0.5.2.

Shared 0.3.5 isolates its own validation-runner fixture from inherited parent
logs, failure logs and GitHub summaries
([shared #105](https://github.com/dragginzgame/shared-tooling/issues/105)). Jobs
does not vendor that fixture or the producer portable suite. The exact committed
fixture passes from the detached checkout with all three parent destinations
selected, preserving sentinel contents and directory membership, on current
Bash/Make and genuine Linux Bash 3.2.57/Make 3.81. Jobs snapshot integrity,
complete offline tool admission, format-tool admission, pins, documentation links
and substitute release-adapter checks pass; Cargo.lock is unchanged.

Evidence is retained under `/tmp/ic-jobs-shared-035.bAP8SC/`: `refresh.log`,
`parent-current.log`, `parent-bash32.log`, retained parent directories/test logs
and `jobs-focused.log`. Shared 0.3.5
[exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38056455993)
passes lint/security; Linux regression is running and both macOS jobs are queued.
No new full Jobs CI or live tests were run for this metadata-only refresh;
the prior qualification below remains bound to its actual inputs. Native macOS
and hosted recovery acceptance remain pending. No commit, push, release,
publication, sibling edit or hosted dispatch occurred in this adoption.

## Earlier 0.5.2 implementation and Shared 0.3.4 qualification

Pending compatible **0.5.2** now adopts Shared Tooling **0.3.4** at
`169d77b8440568c5200eede971625126181f7bb2`. The canonical exporter refreshed
the same 79 selected files from a clean detached checkout under
`/tmp/ic-jobs-shared-034.Gx5D82/shared`. The common pin matrix selects Binaryen
133; explicit `make install-ic-tools` and offline admission pass. Repeated setup
with downloads disabled reuses the selected set. The previous `ic-set.a1yAq4`
Binaryen 132 bundle's original pins/receipt remain identical and its bytes verify.

`make test-canister-optimized` qualifies the original consumer Wasm and independent
`-O3`, `-Os`, `-Oz` outputs through the existing five live recovery tests. The
manual recovery workflow selects this target on all three native hosts. Inputs,
outputs, hashes, optimizer identity/logs and each variant's test/server logs are
retained, with failures stopping before later variants. Normal `build-consumer`
and the original-only `test-canister` remain available. No production optimizer
policy, Job API, storage authority or Rust 1.88 floor changes are introduced.

The incoming graph is preserved, including its newer Metrics **0.5.3** selection:
Memory **0.35.2**, Timers **0.17.2**, Testkit **0.32.1** and four Host crates
**0.12.3**. The incoming lock SHA-256 remains
`541050654749edfbf664ced622d6f386769c0342fe39eeb2d7a264c4d12c3253`.
Full local `make ci` passes. Final affected shell/workflow/docs checks pass.
All five live scenarios pass for all four variants on Linux under current
Bash/Make and genuine Bash 3.2.57/Make 3.81, with parallel Make and no jobserver
warnings. An incomplete wrapper returns failure on both shells.

Evidence lives under `/tmp/ic-jobs-shared-034.Gx5D82/`: `ci.log`, `metadata.json`,
`live-current.log`, `live-bash32.log`, source hashes and tool receipts. Each live
attempt retains its actual Wasm and per-variant results. The Bash 3.2 attempt is
`jobs-recovery.AwYGvw/`; original Wasm SHA-256 is
`58c59e5d46c15eed68e92a4199d88a9dfe1eb39b299142c9150a3e805ca8127f`.
Source hashes match after CI and live runs. The preliminary feature probe
succeeded without extra feature flags; its initially asserted refusal was
incorrect and is retained with an explanatory summary. Final optimization uses
the input's declared features. Previous graph evidence below is not relabelled.

Shared 0.3.4 [exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38054347275)
passes Linux and lint/security; both macOS jobs are queued. Delivered Jobs 0.5.1
CI also passes Linux with both macOS jobs queued. Native macOS optimized execution
and hosted recovery upload remain pending. No commit, push, release, publication
or hosted dispatch occurred; package metadata remains 0.5.1.

## Earlier pending 0.5.2 qualification at Shared 0.3.3

IC Jobs **0.5.1** is pushed at
`cfbe3ed6eccf6370f0d1b92abd34d928f6febd26`, matching public main. Its
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38051154299) passes Linux;
both macOS jobs remain queued at the latest observation. The manual recovery
workflow is delivered. Registry publication was not independently checked.

Pending compatible **0.5.2** aligns the selected Testkit CLI with the maintainer's
updated **0.32.1** harness library. Rerun `make install-testkit-tools` before live
qualification. Reviewed Testkit commit `ff5da12c27bf0835586892ada8e443117229290b`
and published registry sources. Rust source is unchanged from 0.32.0; the Host 0.12 and
PocketIC 16.1.0 contracts remain selected. Jobs has no direct Host dependency.
Its public API, JobRecord, consumer storage, scheduling policy and Rust 1.88 floor
remain unchanged; no consumer API rewrite or stored-data reset is required.

The same patch adopts committed Shared Tooling **0.3.3** at
`d63f0cfaba8ab2961d6012064adbf051c1898bc1`, exporting 79 selected files from clean
detached `/tmp/ic-jobs-shared-033.hHIiH9/shared`. The added README-freshness task is
advisory; no schedule or new CI gate is activated. Newer dirty Binaryen changes
are excluded and executable pins stay unchanged. The exporter initially refused
a mismatched source-remote suffix before mutation; that refusal and the corrected
export are retained in `refresh.log` and `refresh-final.log` under that directory.

Shared's setup preflight now admits the complete platform/pins and available Rust
toolchain before downloads. Its Make includes and Jobs-owned Cargo, selected-tool
and metadata recipes preserve jobserver descriptors under the existing execution
guard. The production validation runner, both Jobs fixtures and both metadata
cleanup paths require explicit completion before successful exit/cleanup.
Actual released Jobs Bash 3.2 reproductions returned zero for incomplete work:
fixture nounset also deleted evidence, and absent RELEASE_VERSION falsely passed
prepare/selected-commit checks. Corrected checks return failure, preserve original
metadata and retain snapshots; original nonzero statuses remain unchanged.
[#16](https://github.com/dragginzgame/ic-jobs/issues/16) records these findings.

Actual Jobs release/publication substitute fixtures and parallel formatting
descriptor checks pass on current Bash/Make and genuine Linux Bash 3.2.57/Make
3.81. Canonical command, host-tool and validation-runner fixtures pass on both
profiles from that exact source. Real parallel Cargo/selected-tool probes no
longer emit closed-jobserver warnings. Logs and before/after exit probes are under
`/tmp/ic-jobs-shared-033.hHIiH9/`. Producer
[exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38052409053)
passes Linux and lint/security; both macOS jobs remain queued.

The incoming catalog/lock changes are preserved: Memory **0.35.2**, Timers
**0.17.2**, Metrics **0.5.2**, Testkit **0.32.1** and all four Host crates **0.12.3**.
Locked cache preparation and selected CLI/server admission pass. The lockfile
remains byte-for-byte equal to the incoming selection, with SHA-256
`833011b09c294fd687da7e4f36cbcb42b9a2ed13af0e783a5e9247603fc8224e`.

Full local `make ci` passes on this graph, including separate Rust 1.88 core and
timers native/Wasm checks, native consumers, linked consumer Wasm and host harness,
strict Clippy, docs and packaging. All five live Linux recovery scenarios pass
through the selected 0.32.1 CLI under Bash 3.2/Make 3.81 with parallel Make and no
jobserver warnings. Evidence is retained under
`/tmp/ic-jobs-shared-033.hHIiH9/graph-0321/`: `ci.log`, `live-bash32.log`,
`metadata.json`, the Testkit selection receipt and `qualified-source.sha256` bind
the execution inputs. Source hashes match after both CI and live execution.
The attempt `jobs-recovery.mGfY08/` retains the tested Wasm and server logs; its
Wasm SHA-256 is
`c3c4842cb4deb31c2af71fe723e3989385508c6a9436b8363541889a7ab31856`.
Final source hashes and the patch additionally bind this handoff update; the
selected new task is also retained separately as `readme-freshness.md`.

Earlier Testkit 0.32.0 qualification remains separately bound in
`/tmp/ic-jobs-testkit-032.8VWsoX/`, and the first Shared 0.3.3 gate/live runs are
retained at `/tmp/ic-jobs-shared-033.hHIiH9/{ci.log,live-bash32.log}`. The lockfile
advanced concurrently before final handoff, detected by the source binding check.
The new CLI pin, cache preparation, complete CI and live gate were then qualified
afresh under `graph-0321/`; earlier evidence is not relabelled as this graph.

[#11](https://github.com/dragginzgame/ic-jobs/issues/11) remains open for hosted
recovery execution/artifact acceptance and native host qualification;
[#15](https://github.com/dragginzgame/ic-jobs/issues/15) awaits delivered macOS CI.
The new [#16](https://github.com/dragginzgame/ic-jobs/issues/16) correction also
awaits delivery and native host qualification.
These Linux results do not establish either macOS host or hosted upload behavior.
No sibling edits, commit, release, push, publication or hosted dispatch occurred.
Package metadata remains 0.5.1.

## Earlier 0.5.1 workflow evidence before delivery

IC Jobs **0.5.0** is pushed at
`b4b7fb6f0e0b8221126019ff29f0eb332c91eb3e`, matching public main. The annotated
`v0.5.0` object `09a3eab5c3e4ce8ffc90e58835b1f4f934f01aa9` matches GitHub;
the retained release plan is complete. Its
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38048491093) passes Linux;
both macOS jobs remain queued at the latest observation. Shared Tooling's
[producer CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38044218125)
now passes on all three hosts and lint/security. Registry publication was not
independently checked.

Pending compatible **0.5.1** adds the manually triggered
[Canister recovery workflow](../../.github/workflows/recovery.yml), running the
existing five PocketIC scenarios on Linux, Apple Silicon and Intel macOS. It
uses the reviewed common tooling, selected Testkit CLI and Testkit-owned server
setup/admission. A fresh job compiles the Wasm and host harness before starting
the 15-minute server lifetime. The test wrapper retains a Wasm copy/hash and
server stdout/stderr in a new attempt directory; tests read that retained copy.
Each workflow host uploads source/graph/tool selections, test outcome/output,
the tested Wasm and server logs for 14 days on success or failure. Failed setup
also uses the canonical failure collector. Push/PR CI remains compile-only for
the live harness. See the [consumer instructions](https://github.com/dragginzgame/ic-scheduling/blob/bb67e6f23ec50a95cf1b58f675cd5a8d8c60f4ff/apps/ic-jobs-test-consumer/README.md)
for manual execution after delivery to `main`.

The maintainer's delivered catalog selects Memory **0.35.0**, Timers **0.17.0**
and Testkit **0.31.0**. Those selections are preserved; no dependency update was
performed in this batch. The unchanged lock SHA-256 is
`5d1f6362eccaa041f8d61298b29c9b1b7461741dfae17646554bb19644fc1d33`.
Jobs runtime source, JobRecord and Rust 1.88 floor are unchanged. The workflow
adds no library lifecycle, storage or Host dependency ownership.

Full local `make ci` passes at this delivered graph, including separate Rust 1.88
core/timers native/Wasm checks, consumer native tests, linked Wasm/harness
compilation, strict Clippy, docs and packaging. All five live Linux tests pass
with the new log capture; after binding tests directly to the retained copy, the
final wrapper passes again on genuine Linux Bash 3.2.57/Make 3.81. Workflow lint,
ShellCheck, snapshot, pin, formatting and documentation checks pass. Logs are
retained in `/tmp/ic-jobs-recovery-workflow.k03mBT/` (`ci.log`, `live.log`,
`live-final-bash32.log`, `metadata.log`); `final-source.sha256` and
`tracked.patch` plus `recovery.yml` bind the final working tree. The final live
attempt is `jobs-recovery.036j71/` in that directory; its Wasm SHA-256 is
`810f093b5d097a04894fbae2b7b1a53c92fab4f3df25698c89bfdf7b91c56d00`.
The successful server emitted startup stdout and empty stderr, both retained.

Hosted recovery execution and artifact upload remain unqualified until the
workflow is delivered and manually dispatched; local passes do not establish
macOS live behavior or the hosted upload. [#11](https://github.com/dragginzgame/ic-jobs/issues/11)
owns recovery qualification, and [#15](https://github.com/dragginzgame/ic-jobs/issues/15)
still awaits delivered native macOS CI. No commit, push, release, publication
or hosted workflow dispatch occurred in this batch.

## Earlier 0.5.0 adoption evidence before delivery

IC Jobs **0.4.7** is pushed at
`b7833ab0f32008f787167e5ca0b4f84aeb2b4694`, matching public main. The annotated
`v0.4.7` object `73330d87bc4c8ba68807372f123213b57aa445eb` matches GitHub and
the retained release plan is complete. The maintainer reports it live; registry
publication was not independently checked. Its
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38045779398) failed on
Linux because consumer Clippy uses Rust 1.99's Wasm target, which the workflow
prepared only for Rust 1.88. The local workflow now prepares both targets.
Both released macOS jobs remain queued at the latest observation.

Pending **0.5.0** adopts committed Shared Tooling **0.3.0** at
`88a73139a0f083344c41a6f6f4b5c3a8aca7dc1d` through its canonical exporter from
clean detached `/tmp/ic-jobs-qualification.l4gwGK/shared` (78 selected files).
Newer dirty sibling edits were excluded. Common setup/check now orders the full
host, five IC and three Rust toolsets before selected cargo-edit. CI and admitted
release preflight use the same complete path; source/candidate admission and
locked fetch still precede release setup. Previous installations and saved-release
reconciliation are retained. Ordinary checks install nothing; optional Testkit
CLI/server setup remains separate. This is a breaking tooling contract and
requires a minor release; package metadata remains 0.4.7.
[#15](https://github.com/dragginzgame/ic-jobs/issues/15) tracks delivery.

The Testkit CLI now matches the selected 0.31.0 library. Memory 0.34.1, Timers
0.16.7 and the incoming lock selection are preserved, with lock SHA-256
`8f3c8e0964cbad6428dfab246fbef259e67ed5367623dd6da4d370747c83ecc2`.
The public Jobs API, JobRecord format and Rust 1.88 floor are unchanged;
Host/Testkit dependencies remain outside the canister graph.

All five **live Linux PocketIC 16.1.0** scenarios now pass: authenticated task
management and real timer delivery across upgrades, synchronous storage/identity/
timer rollback, lost sender results with independent receipt disposition and no
duplicate delivery, invalid-record upgrade refusal with controller-snapshot
recovery, and overdue CatchUp/Skip reconstruction. Rejection assertions require
the actual canister-trap error. Snapshot recovery uses Testkit's bounded retry
for installation rate limits only; its job deadline exceeds the maximum cooldown.
Two earlier attempts hit the snapshot recovery installation limit and remain
retained separately. The passing log is
`/tmp/ic-jobs-qualification.l4gwGK/live-owner-retry.log`; failed logs are
`live.log` and `live-corrected.log` in the same directory. Exact live-source
hashes are in `live-source-final.sha256`. The linked Rust 1.88 Wasm SHA-256 is
`c6bfe0357a2ad1ec3f413f59a9dbdc9cc29d3e872ad5396f5761c0fe6da70ded`.
[#11's qualification comment](https://github.com/dragginzgame/ic-jobs/issues/11#issuecomment-6096833997)
records the evidence and remaining delivery requirement.

The full local `make ci` passes in `ci.log`, including separate Rust 1.88
core/timers native/Wasm checks, native consumer tests, linked Wasm and host harness
compilation, strict Clippy, docs and packaging. Subsequent fixture-only changes
were rechecked with Bash 5/Make 4.3 and genuine Linux Bash 3.2.57/Make 3.81 in
`release-fixtures-final.log` and `release-fixtures-bash32.log`. They exercise
ordered setup/check, reuse, failures, release reconciliation and refusal before
parallel builds for every missing common tool set. Canonical host/routing fixtures
pass on both shell profiles. ShellCheck, workflow lint, formatting, snapshot,
pin and documentation checks pass. Repeated real setup/check preserves executable
bytes/inodes and receipts (`reuse.log`, `reuse-before.sha256` and inode records).
All these logs are under `/tmp/ic-jobs-qualification.l4gwGK/`; `qualified.patch`
and `qualified-source.sha256` bind the full CI invocation before the final fixture
checks and handoff update, while `final-source.sha256` records the final tree.

Producer [exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38044218125)
passes Linux, Apple Silicon and lint/security; Intel remains in progress at the
latest observation. Jobs delivery and native macOS qualification of this working
tree remain pending. Linux live evidence does not establish those host results or
final Canic/IcyDB composition. No sibling edits, commit, release, push or
publication occurred. Both owning issues remain open for source-bound delivery.

## Earlier 0.4.7 implementation evidence before delivery

IC Jobs **0.4.6** is pushed at
`33463211e61b007777aeac0699bd5ea2b01820a9`, matching public main. The annotated
`v0.4.6` object `a557c0f8738be6a6b82d3e4f1ae5dee06643e386` matches GitHub,
and the retained release plan is complete. Its exact-source
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38042280637) passes Linux,
Apple Silicon and Intel macOS. This completes the delivered 0.2.13 adoption
qualification for [#14](https://github.com/dragginzgame/ic-jobs/issues/14).
Registry publication was not independently checked.

Pending compatible **0.4.7** adds the unpublished
[consumer canister](https://github.com/dragginzgame/ic-scheduling/blob/bb67e6f23ec50a95cf1b58f675cd5a8d8c60f4ff/apps/ic-jobs-test-consumer/README.md) and a separate Testkit
host harness for [#11](https://github.com/dragginzgame/ic-jobs/issues/11).
The public Jobs API, stored JobRecord and Rust 1.88 floor remain unchanged.
The new consumer owns its bounded stable snapshot, authenticated management,
counter/delivery payloads, receipt authority and lifecycle exports. The one
watchdog is reconstructed after all retained records pass `Job::restore`.
It commits Running intent before external dispatch; absent receipts retain
blocked work, while exact retained receipts permit explicit disposition.

The selected graph contains Memory 0.34.1, Timers 0.16.7 and Testkit 0.30.0.
Memory 0.34.1 was already selected in the incoming lock; concurrent selection
updates to Timers 0.16.7 and the Testkit 0.30 catalog are retained. Public Testkit
0.30.0 is `6ac161b8ed689012bf8b0ce926946f94ea9f407d`; Timers 0.16.7 is
`999d9b5c3a84ec5abd729ca72b8f259abbb060e1`. Published selected API sources
were reviewed directly. Host/Testkit dependencies stay in the separate host
package; default workspace commands still select the Jobs library.
PocketIC 16.1.0 requires exact thiserror 2.0.18, so the expanded graph necessarily
replaces the previous compatible 2.0.21 selection. A precise attempt to retain
2.0.21 was refused by that upstream constraint; it made no lock mutation.
Other released registry identities remain selected alongside the required new
host dependencies. The final lock SHA-256 is
`73045efaeecaa8bb4800eb9fd6cf83d9f68bc58b77f995eceb37f09bdaa9e72d`.

Five native stable-cell consumer tests pass at Rust 1.88, and the initial focused
native/Wasm/harness compile-only and strict Clippy checks pass. An initial
compile found undocumented Candid fields and an unused import; those were fixed.
Evidence is retained at `/tmp/ic-jobs-consumer-native.log` and
`/tmp/ic-jobs-consumer-check.log`. Final `make check-consumer` passes in
`/tmp/ic-jobs-consumer-check-final.log`: five native tests, a linked Rust 1.88
release Wasm, host harness compilation and warnings-denied native/Wasm Clippy.
The Wasm SHA-256 is
`bea14e94468abb8426bd68cc93a84d5af420b9145e4d755a9f04d1ec353b3820`;
Candid extraction succeeds at `/tmp/ic-jobs-consumer.did`.
The original twenty consumer tests pass separately for core/timers, all 24 job
tests pass, and Rust 1.88 core/timers native/Wasm plus Jobs Clippy pass in
`/tmp/ic-jobs-consumer-integration.log`. That invocation then found committed
metadata export omitted app members, retaining its failed fixture at
`/tmp/jobs-release-tooling.EIJYOK` and selected metadata at
`/tmp/jobs-committed-metadata.dDcXLw`. Exact-commit archive now supplies all
workspace members without a second directory roster. Corrected real local Git
release/publication substitute fixtures pass with Bash 5/GNU Make 4.3 and genuine
Linux Bash 3.2.57/GNU Make 3.81 in `/tmp/ic-jobs-consumer-final-tools.log` and
`/tmp/ic-jobs-consumer-release-bash32.log`. Snapshot, pins, formatting, ShellCheck,
documentation links and diff checks pass. The selected Testkit CLI is not yet
installed: its offline admission correctly refuses, without setup effects, in
`/tmp/ic-jobs-consumer-cli-admission.log`.

The full `make ci` gate remains unrun and explicitly selected under the
maintainer's supplied overlay. Five live harness tests are implemented but
not executed: management/timer upgrade recovery, synchronous write/timer trap
rollback, lost sender-result/receipt disposition, invalid restore with controller
snapshot recovery, and overdue policies. Live PocketIC and native macOS evidence
for this working tree remain pending. No server installation, live gate, commit,
release, push or publication has been performed during this batch.

Shared 0.3.0's complete common toolset remains separate adoption work in
[#15](https://github.com/dragginzgame/ic-jobs/issues/15), awaiting reviewed committed
producer source at the earlier observation; no dirty upstream files were adopted.

## Earlier 0.4.6 adoption evidence before delivery

IC Jobs package version **0.4.5** is pushed at
`c253b3aab8ce8c52ff93b69e1ae14f5dd86b74c9`. Public main matches local HEAD;
the annotated `v0.4.5` object `063d7d0d7682a64513bc754ff9831261d68b1b9e`
matches GitHub's tag reference, and the retained release plan is complete.
The maintainer reports it live; registry publication was not independently checked.
Release-source [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38038868582)
has passed Linux (`114174945921`), Apple Silicon (`114174945776`) and Intel
(`114174945917`) at the exact released source. The configured failure
collector was skipped on the successful Linux gate, so this does not establish
hosted failure-log upload. The released lock changes only the local Jobs version
row from 0.4.4, with SHA-256
`036531dc76445e1c562787a5c77dd8436b6b930d0cd4d460f4c3d1075c90c482`.

Pending compatible **0.4.6** implements the requested upstream adoption. Shared
Tooling **0.2.13** at `5864f468d39f8f9d1bd26fca1afe0e20f25f1b5e`, matching
public main, is adopted through the canonical exporter from clean detached
`/tmp/ic-jobs-adoption.4pCu8K/shared` (78 files). Source/candidate admission and
locked fetch precede selected cargo-edit setup/check. Versioned tool roots retain
older installations; valid selections are checked and reused without replacement,
and invalid existing roots refuse rather than being silently repaired. Ordinary
CI checks tools before dependent dispatch, including under parallel Make.
This adopts the full-delivery-suite baseline; release and live IC execution
retain separate authorization. Jobs has no selected Testkit CLI caller.
[#14](https://github.com/dragginzgame/ic-jobs/issues/14) owns consumer delivery.
Shared's [exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38039035514)
passes Linux portable regression, both native macOS hosts and lint/security.

IC Memory **0.34.0** is delivered at
`958080df899ebfa7bb9c2d4c93bb8664bd23575d`, verified as public main and reviewed
from `/tmp/ic-jobs-adoption.4pCu8K/memory`, excluding dirty sibling files. The
maintainer's incoming catalog already selects 0.34; its entire locked selection
is preserved, including Timers 0.16.5 and Metrics 0.3.7, at SHA-256
`b6343c8d97d217ae33c6b0718a075d1f99c58658970268c66df4b0d004a279ee`.
Consumers use the replacement request-only declaration and key-open APIs directly.
One native host owns the shared pool, grants namespaces and bootstraps before
component verification/open. Real stable cells on VectorMemory persist existing
snapshot bytes; cold reopens retain IDs, payloads, deadlines and blocked Running/
Uncertain attempts. A pending cancellation survives another reopen. Foreign
claims fail without opening capability or changing retained backing bytes.
Fault-injection byte stores remain separate native test models, not fallback
readers for another allocation contract. No Job format or installation is reset.
Memory stays a development dependency; normal core/timers trees have no Memory.
The Jobs test graph has one Memory/stable-structures (0.7.2)/Timers identity.
This is native consumer persistence evidence, not installed Jobs canister upgrade,
timer reconstruction, IC rollback or final Canic/IcyDB composition; those remain
with [#11](https://github.com/dragginzgame/ic-jobs/issues/11) and
[Memory #44](https://github.com/dragginzgame/ic-memory/issues/44).

All 20 consumer tests pass separately at Rust 1.88 for core and timers.
The complete `make ci` passes on final code, including native/Wasm Rust 1.88
checks for both feature selections, strict Clippy, doctests and verified packaging.
Current metadata is still 0.4.5; packaging uploads nothing. Maintained actual
release/admission/recovery/publication substitute fixtures pass on Linux Bash 5.2/
GNU Make 4.3 and genuine Bash 3.2.57/GNU Make 3.81. They cover selected-root
absence/invalidity/reuse, refusal before parallel builds, admitted setup/check
ordering and failure, offline/network cache failures, dirty-source refusal and
saved-release resume without reinstallation. Canonical Rust-tool installer
fixtures also pass on both Linux shell profiles. Selected cargo-edit 0.13.13 was
installed into its new root; corrected offline reuse preserves executable inode,
bytes and lockfile. Snapshot, pin, formatting, ShellCheck, workflow, docs and diff
checks pass. Logs are retained under `/tmp/ic-jobs-adoption.4pCu8K/`.
An initial exporter invocation refused its temporary clone's local remote; the
remote identity was corrected before adoption. The first reuse probe revealed
Cargo replacing a byte-identical executable, so setup now explicitly checks and
reuses valid existing roots; the earlier log remains separate. Initial lint
source annotations were corrected before final lint. No sibling edits, commit,
push, release, publication, hook activation or live qualification occurred.
Native macOS results above bind released 0.4.5, not this working-tree adoption.
The patch classification follows unchanged Jobs public API, record and Rust floor:
the Memory hard cut affects private test consumers only.

The preceding **0.4.4** was pushed at
`7470aebe60e10f84a5dd105eb96ee144865ac509`;
the annotated `v0.4.4` object `07743ef1f17f86fcddc930f27796b165bf28cd4f`
matches GitHub's tag reference, and the retained release plan is complete.
Release-source [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/38037161776)
has passed Linux (`114169891050`); Apple Silicon (`114169890892`) and Intel
(`114169891111`) were queued at the latest observation. These results qualify
0.4.4 only. No crates.io observation was made for this release during continuation.

Delivered **0.4.5** adopts committed Shared Tooling **0.2.12** at
`a8ba9b461b831846eacf64452e6ddcd2acd000f1`, verified as public main, through
the canonical exporter from `/tmp/ic-jobs-shared-0212.gyuP1R/source`.
The clean detached source excludes sibling worktree edits; the selected snapshot
contains 78 files. Canonical admission now checks independently retained Make
invocation evidence and refuses overwritten `MFLAGS`, allowing removal of Jobs'
temporary `_jobs_make_execution_flags` parser. This follows the delivered repair
in [Shared #30](https://github.com/dragginzgame/shared-tooling/issues/30).
The subsequent committed 0.2.12 repair rejects LF/CR in supplied and resolved
directory paths before snapshot export/verification can select a trimmed neighbor
([Shared #95](https://github.com/dragginzgame/shared-tooling/issues/95)).
The adopted baseline also forbids these operational directory names, preserving
existing artifacts and allowing deliberate negative fixtures. This workspace's
ordinary directory identity needs no rename or reset.

Formatting now reports compact results and retains full failed command output.
The configured CI failure collector archives formatting and tool evidence after
the selected gate. The new `test-formatting-evidence` target checks the actual
wrapper's exact failure status and full log. It now uses Jobs' actual Makefile
and selected includes with a Cargo substitute to prove sorter failure stops
rustfmt, successful checks report one line, and both complete failed logs reach
the local archive. It does not invoke hosted upload. Maintained release fixtures additionally refuse
unsafe modes with both `MAKEFLAGS` and `MFLAGS` erased, as well as harmless-looking
command-line and file assignments to `MFLAGS`, before release/publication effects.

The original 0.2.11 adoption's focused Jobs release, actual formatter/hook,
formatting-retention and canonical
Shared release/formatter fixtures pass on Linux with Bash 5.2/GNU Make 4.3 and
genuine Bash 3.2.57/GNU Make 3.81. Logs remain under
`/tmp/ic-jobs-shared-0211.zDs1gB/`. Those selected Make/formatter payloads are
unchanged by 0.2.12; the earlier runs retain their original source binding.
The 0.2.12 snapshot-distribution fixture and extended Jobs formatting/archive
fixture pass separately on Linux Bash 5.2/GNU Make 4.3 and genuine Bash 3.2.57/
GNU Make 3.81. Logs remain under `/tmp/ic-jobs-shared-0212.gyuP1R/`.
Current snapshot integrity, pin admission, actual release adapters, formatting,
ShellCheck, workflow lint, documentation links and whitespace checks pass.
An initial lint attempt stopped on an unresolved source-file annotation in the
new fixture; the annotation was corrected before final lint qualification.
The 0.4.4 lock remained unchanged during this pre-release batch at SHA-256
`682325851ac31ad8e4ea1a0a6abd1501223d45dec059c9e70d98113089df259b`.
This tooling-only patch changes no Rust API, record, dependency selection or
compiler floor. No full local CI/release gate, Rust requalification, native macOS,
live IC or hosted upload qualification was run for this source. No commit, push,
release, publication or real hook activation was performed by the agent for
the local batch. The maintainer subsequently delivered 0.4.5; matching-source
Linux and both macOS results now complete acceptance for
[#12](https://github.com/dragginzgame/ic-jobs/issues/12) and
[#13](https://github.com/dragginzgame/ic-jobs/issues/13).

The preceding IC Jobs 0.4.3 was released at
`7939c8a53add5ab019cc26e97bd93274b5ed3a43` and pushed by the maintainer.
The exact local annotated `v0.4.3` object
`0a81bc12e25993a995df326827bd6bf74a0cedec` matches GitHub's tag reference;
the retained local release plan is complete. The maintainer reports it live,
and an exact crates.io observation confirms 0.4.3 present without binding
registry bytes to a local artifact.
Release-commit [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37954681664)
has passed Linux, native macOS Intel and Apple Silicon, including five-tool
setup, offline admission and the configured selected-graph gate. Jobs
`113902068497`, `113902069034` and `113902068936` qualify released 0.4.3,
not the subsequent management fixture or incoming Metrics graph.
The preceding 0.4.2
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37951742672)
has passed Linux; both native macOS jobs have since been cancelled without
passing. The preceding 0.4.1
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37941395642)
has passed Linux; both native macOS jobs have since been cancelled. The preceding 0.4.0
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37936803386)
has passed Linux and macOS Intel; Apple Silicon has since been cancelled.
The preceding 0.3.1
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37925974097)
passed Linux and macOS Intel; Apple Silicon has since been cancelled. These older
source-bound native gaps are not relabelled by the later all-host 0.4.3 result.
The preceding 0.3.0
[CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37918180627)
has passed Linux, macOS Intel and Apple Silicon at its exact release source,
including explicit five-tool installation, offline admission and the configured
selected-graph gate. The preceding
0.2.3 [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37909918403)
has passed Linux, macOS Apple Silicon and macOS Intel at its exact release SHA.
Its original native job identities survived subsequent pushes and completed
successfully. These results do not qualify later source or live IC recovery.
The preceding 0.2.2 [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37903041952)
has passed Linux, macOS Apple Silicon and macOS Intel at its exact release SHA.
Those earlier native jobs retain their original identities after the 0.2.3 push:
queued retention is observed, and both retained native jobs have completed successfully.
Running retention is now observed separately: the original 0.3.1 Intel job's
validation step ran from 13:25:40Z to 13:28:07Z on 2026-10-09, crossing creation
of the subsequent 0.4.0 push run at 13:26:03Z, and completed successfully.
See [the hosted retention evidence](https://github.com/dragginzgame/ic-jobs/issues/3#issuecomment-6082354943).
Release 0.2.1 [CI](https://github.com/dragginzgame/ic-jobs/actions/runs/37899775906)
passed all three hosts. Its earlier native results do not qualify later source
or live IC recovery. Owning issues retain the outstanding native/hosted acceptance.
The library owns job policy and checked metadata transitions. Applications own
persistence and effects; the optional adapter uses the consumer-selected IC Timers
dependency graph.

The maintainer requested notes for an application-owned canister management API,
IC Memory-governed stable storage and timer reconstruction across upgrades.
The [consumer composition](../testing/consumers.md#proposed-durable-canister-consumer)
records ownership, mutation/rollback ordering, startup and blocked-effect rules;
[#11](https://github.com/dragginzgame/ic-jobs/issues/11) owns implementation and
IC Testkit qualification. The 0.4.4 continuation implements the native notification
management fixture described below; actual endpoints, stable storage and upgrade
evidence remain pending the reviewed replacement allocation contract in
[IC Memory #44](https://github.com/dragginzgame/ic-memory/issues/44).

Delivered **0.4.4** extends the existing application-owned notification consumer
with manager-authorized create, inspect, bounded job-ID pagination, pending
cancellation and due dispatch. Admission precedes payload/storage access; all
restoration uses `Job::restore`. Replies carry the earliest committed deadline,
and failed writes retain the previous queue/deadline without exposing delivery.
Cancellation cannot undo Running/Uncertain effects; reconstruction and management
dispatch preserve blocked intent until destination evidence permits disposition.
No library API, record, scheduling policy or compiler-floor change is made.
This is compatible test-consumer work delivered as a patch.
The replacement upstream cut was not available for that adoption; no provisional 0.33 range adapter,
duplicate allocator or lifecycle export is introduced in Jobs.
Focused Linux Rust 1.88 tests pass for all 17 consumers in each core/all-feature
selection, including the six new management boundary checks. Optional timer
coverage projects committed deadlines without registering a platform timer.
Warnings-denied development Clippy, formatting, documentation links and whitespace
checks pass. Native caller policy and byte-store reconstruction are not evidence
of Candid authentication, real stable-memory persistence, message rollback or IC
upgrades. Native macOS and live IC qualification remain separate. No full local
CI, commit, release, push, upload or live qualification gate is run for this batch.

The authorized IC Memory addition declares `ic-memory = "0.33"` in the root
catalog and inherits it only as a Jobs development dependency, locked to 0.33.4.
Reviewed public main is `9137192d4b3425a229fa21df5f343883f56d80e8` (0.33.4);
its delivered contract predates the host-wide pool hard cut. Both fixtures contribute
key-only requests for their combined record/payload or maintenance state stores.
Native composition checks reject conflicting consumer identities and preserve
job bytes/deadlines; request order does not change the sealed declaration meaning.
Fresh-runtime opens refuse before bootstrap. No host range adapter, global
registration, storage trait, library dependency or lifecycle export is added.
The byte-replacement stores remain authoritative until the final host integration
can adopt [IC Memory #44](https://github.com/dragginzgame/ic-memory/issues/44).
All 19 consumer tests pass separately at Rust 1.88 for core and optional timers
on the current Memory/Timers/Metrics graph. Library-only normal dependency trees
confirm Memory is absent from both feature selections. Rust 1.88 native and Wasm
library checks pass separately for core and timers. Warnings-denied consumer
Clippy, locked metadata, declaration checks, formatting, snapshot integrity,
documentation links and whitespace checks also pass. No complete CI/release gate
or native macOS/live IC qualification was run for this dependency addition.
This native declaration
evidence does not establish IC stable-store persistence, allocation recovery,
Canic/IcyDB composition, message rollback or upgrades. The addition extends
compatible 0.4.4, subsequently delivered by the maintainer.
The [Memory owner initially reported local implementation](https://github.com/dragginzgame/ic-memory/issues/44#issuecomment-6095440378)
of the 0.34.0 shared pool, namespace grants and unmanaged exclusions. That work
was then uncommitted; Jobs still selected 0.33.4. Delivery and native Jobs adoption
are now recorded for pending 0.4.6 above. Final Canic/IcyDB/Jobs coordination and
real IC recovery remain at #44/#11; earlier reports do not establish that evidence.

The authorized [Make repair](https://github.com/dragginzgame/ic-jobs/issues/12)
adopts committed Shared Tooling **0.2.9** at
`f8a70ba348e9975a6eb5b337860b00bc8a0b36d1`, verified as public main, through
the canonical exporter from `/tmp/ic-jobs-shared-029-fix.HZfYBT/source`.
The 74-file selection recorded the exact committed version. The admission
helper follows the selected include rather than an ambient runtime root and
uses `MAKE_COMMAND` without importing argument-bearing recursive `MAKE` files.
Fleet reports and hosted artifact transport are outside Jobs' selection.

Shared's hidden-mode gap was tracked at
[Shared #30](https://github.com/dragginzgame/shared-tooling/issues/30).
Before repair, `-i release-patch MAKEFLAGS=` dispatched a substitute runner
exiting 23 and returned false success; inputs/logs remain at
`/tmp/ic-jobs-make-flags-review.amyFpk/`. The extended maintained Jobs fixture
also failed its new cleared-flags case before implementation, retained at
`/tmp/jobs-release-tooling.DhtoP6/`. Jobs 0.4.4 checked invocation modes retained
in `MFLAGS` at its own Makefile boundary, before the shared includes. This
explicit consumer-owned adapter is permitted by the baseline; no vendored
guard was patched. The supplementary parser is now retired by the committed
canonical repair and actual-consumer regressions delivered in 0.4.5 above.
Shared #30 is closed; source-bound consumer acceptance for #12 now passes.

Focused release/publication, actual adapter, root/recursive-Make and formatting
fixtures pass on Linux with Bash 5.2.21/GNU Make 4.3 and genuine
Bash 3.2.57/GNU Make 3.81. The maintained Jobs cases cover all four unsafe modes,
long forms and a cluster, direct/inherited selections and cleared/replaced
`MAKEFLAGS`, refusing release/resume, formatting and publication before fetch,
gate or source mutation. Ordinary keep-going/silent flags, quoted selections and
parallel recursive Make still work without jobserver warnings. Real local Git
release/recovery uses substituted gate/cache/registry/upload effects; this is
not live release or complete-gate evidence. Logs remain in the adoption directory.
Snapshot integrity, declaration checks, formatting, ShellCheck, documentation
links and whitespace checks pass. The first post-repair fixture attempt stopped
on an uncached incoming Timers 0.16.4 at `/tmp/jobs-release-tooling.WDnSXs/`;
`cargo fetch --locked` then prepared that exact graph without altering its bytes.
The compatible repair was delivered in 0.4.4 without library/API/storage/MSRV
changes. No sibling edits, commit, push, real release/publication, hook activation,
full local CI or native macOS/live IC qualification were performed.

The maintainer requires Rust 1.88 for the complete package. Manifest metadata,
native/Wasm minimum checks, CI setup and current support documentation now use
that floor for both core and timers. Release 0.2.0 drops the previously advertised
1.85 floor and changes the consumer toolchain contract; consumers must upgrade
to 1.88. The compatible publication-path and release-cache fixes are delivered
in 0.2.3. The developer IC bundle hard cut, shared-tooling fixes and locked
dependency refresh are delivered in 0.3.0. The final-pin-record admission and
application-owned test consumers are delivered in 0.3.1. The optional IC Timers
package-identity change is delivered in 0.4.0. Test-consumer recovery coverage
and shared hook fixes are delivered in 0.4.1. The locked Timers refresh and shared
Make adoption are delivered in 0.4.2. The compatible Shared Tooling 0.2.7 Make
admission and fixture isolation fixes are delivered in 0.4.3; package metadata
is 0.4.4; the compatible management fixture, Memory declaration and Make repair
are delivered in 0.4.4. The next canonical Make and formatting-evidence repair is
collected under the undated 0.4.5 draft. The tooling refresh makes no library or dependency
contract change.
Job APIs, stored fields, scheduling policy and the Rust floor are unchanged.
Earlier 1.85 qualification below records historical evidence only.

The released 0.4.2 lock selects Timers 0.16.3 and Metrics 0.3.2, at SHA-256
`5f1e57f836b5265a545e0f3161b59419f6e2b76503c72bd06b4a11341825f496`.
Its finalized changelog names the earlier Timers 0.16.2 working selection; the
released lock owns the actual graph. The published entry is preserved.
The pre-release maintainer lock independently advanced Metrics to 0.3.3, retaining
Timers 0.16.3; its tested tooling-only bytes were preserved at SHA-256
`c5204f0b9bc6f8dc54fec4786541f7c08f4dda8012d179f87fd49064c672ebd1`.
Release 0.4.3 retains that dependency selection, updating the local package row;
its lock SHA-256 is
`5c4f56be2c0abbc1cd137ec841c4c3d77dcd87e70416b2fa9f73e07fd02e7322`.
The local tooling fixtures did not qualify that Rust graph. Exact-source 0.4.3
Linux and native macOS CI now qualify the released graph separately; live IC
evidence remains pending.

The management batch's tested maintainer lock selected Metrics 0.3.4 while
retaining Timers 0.16.3. Its bytes were preserved throughout that batch at SHA-256
`7f7086e572d3476482c5ce4f1e94f9295f12a35c0d5498d3190f936a351987d4`.
The focused 0.4.4 native consumer checks above use that graph; they do
not relabel released 0.4.3 CI or establish a complete dependency/MSRV/Wasm gate.
An intervening maintainer lock selected Metrics 0.3.5, still with Timers 0.16.3,
at SHA-256 `8a3246b1c611e5d5f7842e514fcbe1c64c86f90ea474c973e4abb736da6e744d`.
The earlier incoming maintainer lock selected Metrics 0.3.5 and Timers 0.16.4 at
SHA-256 `af774a3bdd96463c2dc7662e3174a0c4b89ef10c9686bc839fc923282a54cfec`.
That graph is cache-prepared and preserved byte-for-byte during the Make repair;
metadata fixtures do not establish Rust/MSRV/Wasm qualification for it.
The pre-release 0.4.4 tested lock also selects development IC Memory 0.33.4 and its exact
stable-structures 0.7.2 substrate, retaining Timers 0.16.4 and Metrics 0.3.5,
at SHA-256 `67dfef93be379e92df50716137d2bb5bce1a6effd11ea3a99994115b07aca9ca`.
Locked cache preparation and metadata inspection pass; the consumer tests above
qualify this graph separately from the older management-only results. Native
macOS and live IC evidence remained outstanding for that local batch. Release
0.4.4 retains those dependency selections and updates the local package row;
its lock digest and exact-source CI status are recorded above. Live IC remains pending.

The current [Make follow-up](https://github.com/dragginzgame/ic-jobs/issues/10)
adopts committed Shared Tooling 0.2.7 at
`47d6ae6488b8007323fa7c2e22a6efa11d77ae63`, verified as public main, through
the canonical exporter from `/tmp/ic-jobs-shared-027-review.ri8p21h9/source`.
The selection grows to 74 files with `make/execution.mk`; both it and its existing
script companion are included in the release adapter fixture. Outer Make now
refuses ignore-errors, dry-run, touch and question modes before recipe effects.
The maintained Jobs fixture checks direct and inherited modes for all release
entrypoints and formatting, preserving Git identity, source and fetch/gate state.
The canonical release checker isolates its tooling root from inherited selections.
Focused checks pass on Linux Bash 5 and genuine Bash 3.2.57: real-Git release and
publication fixtures with substituted gate/registry/upload effects, actual release
adapter checks with a conflicting inherited root, and actual formatter/hook
installation with sorting and source/index preservation. Normal parallel Make
with a quoted variable and the default help goal also pass. Snapshot integrity,
dependency declarations, documentation links, formatting, ShellCheck and whitespace
checks pass. Logs remain in the adoption directory. The incoming Metrics lock
bytes above are unchanged; tooling fixtures do not qualify that Rust graph.
Native macOS and live IC qualification remain separate. No full CI, release,
publication, commit, push or hook activation was performed by the agent for
this adoption. The maintainer subsequently delivered it in 0.4.3, whose exact
Linux, Intel macOS and Apple Silicon CI now pass as recorded above, completing
native consumer acceptance for #10. Live IC qualification remains separate.

The authorized [Make adoption](https://github.com/dragginzgame/ic-jobs/issues/10)
selects committed Shared Tooling 0.2.6 at
`ce13a5314916891fd239d9b199b4a91b04775054`, verified as public main, through
the canonical exporter from `/tmp/ic-jobs-shared-026._zai8h6t/source`.
The snapshot grows from 71 to 73 files by explicitly selecting `make/release.mk`
and `make/rust-format.mk` with their existing companions. The subsequent
Make-execution follow-up was excluded from that adoption. Standard release routing and
conflicting-goal admission now have the shared owner; Jobs retains direct-only
guards, release-tool admission, metadata/gate adapters and explicit setup/check
extensions. Formatting uses the same prepared pins/toolchain and now carries
offline/no-auto-install settings throughout both recipes. The default goal
remains help. No Rust function, type, record or scheduling policy changes.
Focused checks pass on Linux Bash 5 and genuine Bash 3.2.57: actual Make release
argument/failure/conflict checks; direct-only admission and exported environment;
formatter missing/wrong-version refusal and failure isolation; real formatter/hook
sorting, partial-stage rejection and unrelated edit/index/lock preservation; and
real-Git direct release/recovery/publication fixtures with substituted gate,
registry and upload effects. The Make adapter fixture now includes both new files.
Logs and the additional actual-Make probes remain in the adoption directory.
Snapshot, dependency declarations, formatting, docs, ShellCheck and whitespace
checks pass; incoming Cargo.lock remains byte-identical at the digest below.
The maintainer subsequently delivered this adoption in 0.4.2, whose exact Linux
CI passes as recorded above; its macOS jobs were subsequently cancelled. The
later 0.4.3 result qualifies the adopted tooling on all three hosts. No full
CI, real release/publication, commit, push or hook activation was performed by
the agent during that adoption.
The [reviewed follow-up](https://github.com/dragginzgame/ic-jobs/issues/10#issuecomment-6083691125)
reproduced ignore-errors mode bypassing formatter admission in a disposable Jobs
0.2.6 Make copy, then verified refusal before formatter calls with the committed
0.2.7 guard. That follow-up is delivered in 0.4.3; 0.4.2 records the earlier
delivered 0.2.6 baseline.

The released 0.4.1 lock selects IC Timers 0.16.1 and IC Metrics 0.3.2, at SHA-256
`4d27cde30ee4dc43d019e134ef3a55b17a2c34381e4f00db96f8e35b6e4dc1e5`.
The earlier working lock selected Timers 0.16.2, changing only that package's
version/checksum; its tested bytes were preserved at SHA-256
`cb85b8c1879b98aa384e224828dfa225cf37bae09ad296bad08937c34127c015`.
The 15 published Timers library source files match 0.16.1 byte for byte, and the
tagged library comparison is unchanged. No adapter rewrite, dependency identity,
job storage conversion or compiler-floor change is needed. Explicit locked fetch
prepared the missing registry input without changing Cargo.lock. Focused Linux
qualification on this incoming graph passes Rust 1.88 core/all-feature tests
(23/24 job tests and 11 consumer tests in each selection), core/all-feature native
and Wasm checks, warnings-denied development Clippy, rustdoc and both doctests.
This evidence is separate from released 0.4.1/0.4.2 CI and the later incoming graph;
native macOS and live IC recovery remain separate. No full CI, commit, release, push or
upload was performed for this continuation.

The released 0.4.0 catalog/lock select registry IC Timers 0.16.0 and IC Metrics
0.3.2, retaining the maintainer's other refreshed lock selections. Released bytes
are preserved at SHA-256
`f68c66b218cdfcfee9b33b8c93a30db20ccbd7190478e0a6e00ddc55454798d5`.
The earlier working adoption selected Metrics 0.3.1 and retained its lock at
`b2dad5af4c1540a6f4f2f26f01426032ff64f36209ff308e0e846faeea1aee62`.
The optional adapter exposes Timers types, so consumers must align timer owners
to 0.16 and exchanged summaries to Metrics 0.3 or the Timers reexport. This is a
pre-1.0 breaking dependency identity and requires the next minor; Job APIs,
durable fields and scheduling transitions do not change. Tagged upstream source
comparison finds no timer runtime or Metrics arithmetic changes between the
released 0.14.23/0.2.20 and selected 0.16.0/0.3.1 library source. No adapter shim,
database or host dependency is added. The prepared cache supports focused Linux
Rust 1.88 core/all-feature native and Wasm checks, all 24 job tests, all nine
consumer tests in each feature selection, warnings-denied Clippy, rustdoc and
both doctests. Formatting, declarations, documentation links, whitespace checks
and verified all-feature packaging pass; packaging still uses local metadata
0.3.1 and uploaded nothing. Those local adoption results retain their earlier
graph; the maintainer subsequently delivered 0.4.0 with the refreshed lockfile.
Its exact Linux CI passes as recorded above; native macOS and live IC acceptance
remain separate. The agent ran no full local CI, package version bump, release,
push or publication for that adoption.

The 0.4.1 consumer continuation extends only the private test consumers. A separate
destination retains bounded exact-attempt/payload rejection evidence, enabling
explicit retry disposition after a lost rejection reply while absence remains
blocked. Disposition writes must commit before retry admission; stale evidence
is refused and the original retry budget still ends in Failed. Accepted execution
receipts take precedence for duplicate requests. The byte store now supports a
failure after one successful write to qualify partial maintenance-batch recovery:
only the first occurrence survives, and reconstruction resumes at the uncommitted
occurrence. This is native storage-composition evidence, not message-wide IC
rollback, live provider delivery or an actual upgrade. No library API, record,
dependency or compiler-floor changes are made.
Focused Linux qualification passes on the released graph: all 11 consumer tests
in each core/all-feature Rust 1.88 run, warnings-denied development Clippy,
formatting, documentation links and whitespace checks. The 0.4.0 lockfile remains
byte-identical during that batch. No full CI, native macOS/live IC gate, commit,
release, push or publication was performed by the agent for that batch. The
maintainer subsequently delivered it in 0.4.1 with the Timers 0.16.1 lock;
its exact Linux CI passes as recorded above.

The Shared Tooling refresh delivered in 0.4.1 adopts committed 0.2.5 at
`04e07b4bf54e7aeb03eb7804a845cee27b7305df`, confirmed as public main, through
the canonical exporter from `/tmp/ic-jobs-shared-025.rNcuhh/source`. The 71-file
selection stays intact and pending consumer work is preserved. The hook and
installer preserve literal newline-ending paths, refuse existing different hook
selections without modifying configuration, and retain failed Git observations.
See [shared #89](https://github.com/dragginzgame/shared-tooling/issues/89).
The released Jobs hook payload fails the new canonical regression, retaining
`/tmp/git-hooks-test.WsFlSx` and `hooks-before.log` in the adoption directory.
The fixed canonical hook fixture passes on Linux Bash 5 and genuine Bash 3.2.57,
including configured local/inherited hook paths, newline-ending checkout names,
failed Git reads, index preservation and prepared-tool discovery. Jobs' actual
formatter/installer adoption passes with real formatters and its current Make
adapter, including manifest sorting, partial-stage refusal and failure isolation.
Logs remain in the adoption directory. New snapshot companion declarations
concern optional upstream test/LOC suites outside Jobs' selection; no executable
or test-suite addition is needed. CI queue guidance is adopted without scheduling
maintenance or changing CI policy. No further Jobs library repair is identified
by this refresh; native macOS and live IC qualification remain separate.
Snapshot integrity, declarations, documentation links, formatting, standard
release Make adapters, ShellCheck and whitespace checks pass. Cargo.lock remained
byte-identical during that adoption at the 0.4.0 released digest above. No full CI,
commit, release, push, publication, native macOS or live IC run was performed by
the agent for that refresh.
The previously uncommitted Apple Make formatter fixture follow-up is included
in the reviewed 0.2.6 source adopted above. Jobs does not vendor that upstream
fixture; its actual consumer Make/hook checks remain separate from canonical
fixture results. Linux Bash 3.2 does not establish native macOS qualification.

The maintainer requested in-repository test consumers.
[Notification and cache-maintenance fixtures](../testing/consumers.md)
now exercise the public API with bounded application-owned byte storage, immutable
notification payloads and a separate destination receipt owner. Every load uses
`Job::restore`. Notification delivery is exposed only after Running intent commits;
lost replies and failed result writes retain blocked intent until explicit receipt
reconciliation. The notification queue now supplies up to four unique jobs to
the scheduler, preserving earliest/tied ordering and allowing other due work to
proceed past blocked intent. Insertion/cancellation rederive the shared wakeup;
excess/duplicate retained records and corrupt future work reject before dispatch.
Interruption before delivery retains blocked intent even when no receipt exists.
Local cache effects commit with their metadata, with at most two
recurring runs per wake. `make test-consumers` qualifies core/timers separately on
Rust 1.88 and is selected by configured CI. Timer tests project actual fixture
deadlines through `complete_batch`; native execution does not qualify live
watchdog registration, provider delivery, stable memory or IC upgrade/rollback.
The consumer batch introduced no dependency, library API or storage-contract change.
Focused Linux checks passed before 0.3.1 delivery: all nine consumer tests in each
core/all-feature Rust 1.88 invocation, warnings-denied development Clippy,
formatting, documentation links and whitespace checks. The released lockfile is
unchanged during that batch. The agent ran no full CI, native macOS or live IC gate,
commit, release or publication for the consumer batch; the maintainer subsequently
delivered it in 0.3.1, whose exact Linux CI passes as recorded above.

The released 0.2.3 lockfile selects IC Timers 0.14.23 and IC Metrics 0.2.18 within the
existing compatible requirements. Explicit `cargo fetch --locked` downloaded the
missing Timers input before delivery; that pre-release lockfile's SHA-256 remained
`3c423922eda86146028b688b0ac37a83ea4cd01490b5149aebc8107d36cfc980`.
Focused Linux checks passed on that graph: Rust 1.88 native and Wasm library
checks for core/all features, all 24 job tests with timers, and warnings-denied
development Clippy. This replaces the earlier missing-cache check limitation;
it does not qualify native macOS or live IC recovery.
The released 0.3.0 lockfile selects IC Metrics 0.2.20 with IC Timers 0.14.23.
Before release, that maintainer selection was preserved byte-for-byte at SHA-256
`be8149670998dccb7516e292f831b43e3c28131ac248917402a99f79bdc8d23e`.
Its cached graph now passes focused Linux Rust 1.88 core/all-feature native and
Wasm checks, all 24 job tests with timers, and warnings-denied development Clippy.
The tagged Metrics 0.2.18-to-0.2.20 arithmetic source and package requirements are
unchanged; the upstream fixes concern host tooling. Native macOS and live IC
recovery remain separate qualification requirements for this working tree.

The initial compatible batch delivered in 0.3.0 adopted committed Shared Tooling revision
`926a20606591214ab29faa236b0b584e4857439e` through the canonical exporter from
`/tmp/ic-jobs-shared-038.Nj6oHA/source`, preserving the 72-file selection.
The sibling's dirty VERSION bump is excluded. This commit contains the 0.1.38
pin-exception fix but records VERSION 0.1.37; no published 0.1.38 tag was observed.
The hook keeps original-checkout prepared executable lookup while formatting
isolated index inputs. Pin exceptions must be one validated JSON array, so later
documents cannot mask malformed exceptions. The selected Cargo installer also
adopts single-document receipt admission, original failure status and ancestor
rechecks; its consumer-selected mode remains inactive in Jobs.
See [#6](https://github.com/dragginzgame/ic-jobs/issues/6) for reproduction,
adoption and qualification evidence. This clone already has
`core.hooksPath=.githooks`; source adoption does not activate other clones.
Focused qualification passes on Linux Bash 5 and genuine Bash 3.2.57: canonical
dependency, hook and Rust installer fixtures, including multi-document refusal,
local-only executable lookup, wrong/missing tools and payload preservation.
The installer uses substitute Cargo; the hook fixtures combine executable
substitutes with real formatting cases. Jobs' actual Make fmt/fmt-check adapter
also passes disposable-checkout adoption with prepared formatters, including
manifest sorting, partial-stage refusal, failure isolation and unrelated edits.
Snapshot integrity, actual declarations, documentation links, formatting,
release Make adapters, ShellCheck and whitespace checks pass. The full local
gate, native macOS and live IC qualification were not run for this pending batch;
no commit, release, push or upload was made.

The hard cut delivered in 0.3.0 adopts committed Shared Tooling revision
`8140e3dd1b44409d682c721889ab702f438c6a17` from the clean isolated checkout
`/tmp/ic-jobs-shared-hard-cut.IcFDos/source`. That export retained all 72 files;
Jobs never selected the retired PocketIC checkers/fixture. This source contains
Shared's 0.2.0 hard cut but records VERSION 0.1.38; no published 0.2.0 tag was
observed. Upstream native acceptance remains separate from this local adoption.
The IC matrix/validator/installer now select exactly five tools. Jobs has no
PocketIC/Testkit dependency or server-path/equality caller, so no replacement
server installer or product dependency is added. The existing CI matrix now
explicitly prepares the reviewed bundle and checks it offline on all three hosts.
See [#7](https://github.com/dragginzgame/ic-jobs/issues/7) for adoption acceptance.
An existing six-tool bundle must be replaced by explicit `make install-ic-tools`
before `make ic-tools-check`; previous bundles, pins, receipts and evidence stay
retained. This clone had no installed IC bundle before adoption. No job storage
reset or consumer data conversion is needed. Future application-owned IC recovery
qualification uses Testkit's selected setup/check contract.
Focused Linux qualification passes: the canonical IC installer fixture on Bash 5
and genuine Bash 3.2.57 covers synthetic selections for all three hosts, rejects
six-tool matrices before effects and preserves old bundle bytes/receipts through
failed and successful replacement. Actual `make install-ic-tools` verified the
official five-tool Linux assets; `make ic-tools-check` passed offline. Setup reuse
with a failing curl substitute kept the same active selection, pins and receipt;
the prior six-tool matrix is refused against the actual installer. Installed
artifacts remain under `.tools/ic-set.*`; logs and before-adoption inputs are
retained under `/tmp/ic-jobs-shared-hard-cut.IcFDos/`.
Snapshot, actual declarations, documentation links, formatting, Make adapters,
ShellCheck, actionlint and whitespace checks pass. The incoming Cargo.lock digest
is unchanged. No job code or public symbol was removed; the installer drops only
PocketIC version/download/archive branches and the catalog drops its three rows.
No full local CI, native macOS or live IC run was performed for this hard cut;
delivery and exact-source native acceptance remain with the owning issue. No
commit, version bump, release, push or upload was made.

The subsequent authorized removal omits `scripts/ci/release-pr.sh` and its
snapshot record; the current selection has 71 files at the same shared revision.
Jobs supports direct release delivery only, enforced by both Make and its
metadata adapter. The shared runner loads the removed transport only for PR
delivery; no retained local PR plan exists. Canonical PR-adoption guidance stays
unchanged at its owner. The existing real-Git fixture now checks PR refusal
before fetch, gate or source mutation. Job/scheduler surfaces and restore/recovery
checks remain live and retained. See
[#8](https://github.com/dragginzgame/ic-jobs/issues/8) for the exact removed
function inventory and qualification.
Canonical refresh and the 71-file verifier pass after selection pruning.
`LC_ALL=en_US.UTF-8 CARGO_NET_OFFLINE=true make test-release-tooling` passes on
Linux Bash 5 and genuine Bash 3.2.57, exercising the supported direct
patch/minor/major/recovery and publication flows plus PR-mode refusal before
fetch/gate/Git or metadata mutation. Gate/registry/upload effects are substituted;
no live release or publication was performed. Documentation links, formatting,
release Make adapters, ShellCheck, runner syntax and whitespace checks pass.
The incoming Cargo.lock remains byte-identical. No full CI, native macOS or live
IC qualification was run by the agent for this cleanup. The maintainer subsequently
delivered it in 0.3.0; its exact CI now passes all three supported hosts as
recorded above.

The adoption delivered in 0.3.1 selects committed Shared Tooling 0.2.2 at
`ee48bb37c98c771e77b92fd891f0757d8c1c8b99` through the canonical exporter
from `/tmp/ic-jobs-pin-final-row.u6skwt/source`, retaining the 71-file selection.
Both IC installer loops now consume the final populated pin record without a
trailing newline; the validator, original pin bytes and per-tool checks stay
authoritative. The released Jobs installer reproduced the missing final tool
in the canonical synthetic fixture, retained at `/tmp/ic-tools-test.xgzxR7`;
before/fixed logs are under `/tmp/ic-jobs-pin-final-row.u6skwt/`.
See [#9](https://github.com/dragginzgame/ic-jobs/issues/9) for consumer acceptance.
Shared's independent CI executable publisher is outside Jobs' selected executable
set; its revised guidance is adopted without adding that helper or a new mode.
The corrected canonical IC fixture passes on Linux Bash 5 and genuine Bash
3.2.57 with synthetic assets for all three host selections, including missing
final newline, wrong final-tool version and original pin/receipt preservation.
Jobs' actual prepared bundle passes offline checks under Bash 3.2; setup reuse
with a failing curl substitute retains its active selection, pins and receipt.
During that tooling batch, the released 0.3.0 Cargo.lock stayed unchanged at SHA-256
`0bad777cf80f3560ef09092f7bc1bb857e6ec7eccbbce09186d7ca63f9f6ea83`.
Snapshot, actual declarations, docs, formatting, Make adapters, ShellCheck and
whitespace checks pass. No full local CI, native macOS or live IC execution was
run by the agent for this patch; no commit, package bump, release, push or upload
was performed by the agent. The maintainer subsequently delivered it in 0.3.1;
its exact Linux CI passes, while native consumer acceptance remains with #9.

Release 0.2.3 adopts reviewed committed Shared Tooling 0.1.35 at
`be550afa57fe9e16872e5110b5cd69c24b4fa9e8` through the canonical exporter from a
clean temporary checkout. The 72-file selection still omits fleet reporting.
Its new selected Cargo-tool mode is not activated in Jobs; the fixed Rust-tool
bundle remains the setup contract. The owner's fixed/selected installer fixture
passes with substitute Cargo, and snapshot/pin/docs/format/Make checks pass.
The public GitHub description matches the maintained library scope; sibling
repositories remain read-only.

Release preflight now uses `cargo fetch --locked` after source/candidate admission
and saved-intent selection, before the offline gate or metadata preparation.
Explicit Cargo offline settings remain authoritative. Fetch failure retains
Cargo's diagnostics/status and leaves metadata unchanged; ordinary version and
metadata checks remain offline. The new fixture reproduced the old forced-offline
dispatch at `/tmp/jobs-release-tooling.esB0Uj/cold-offline.log`. Its corrected
cases cover cold preparation, explicit offline refusal, failed fetch,
prepared-cache reuse and dirty-source refusal before any fetch, with original
failure status and no gate/version effects. See
[#5](https://github.com/dragginzgame/ic-jobs/issues/5).

The delivered publication fix preserves trailing newlines in the checkout
path, using the metadata adapter's existing path-sentinel pattern in the
publisher and release fixture. The extended fixture reproduced the released
publisher's incorrect `cd` at
`/tmp/jobs-release-tooling.2oZeDV/publication.log`; failure occurred before any
registry observation or upload. The initial fixture-edit failure is retained
at `/tmp/jobs-release-tooling.EZZHDM`; its remote mapping was corrected before
the publisher reproduction. See
[#4](https://github.com/dragginzgame/ic-jobs/issues/4) for the defect and delivery
qualification. The corrected focused release/publication fixture passes with
real local Git and substituted gate/registry/upload effects under
`LC_ALL=en_US.UTF-8`, also using Bash 3.2.57 on Linux from a source checkout
ending in two newlines with `CDPATH=/tmp`. The source copy is retained at
`/tmp/jobs-release-checkout.iInCSL`. ShellCheck, documentation links, snapshot
integrity, release Make adapters and whitespace checks pass. The agent did not
run the complete gate or deliver this patch; the maintainer subsequently
released/published 0.2.3. Its exact release CI now passes all three supported
hosts as recorded above, completing the native acceptance of the path and cache
repairs. The combined cache/path release fixture
also passes under the offline gate environment and Bash 3.2.57 on Linux from
`/tmp/jobs-release-cache-checkout.xAx7mC/source checkout` followed by two newlines,
with `CDPATH=/tmp`. Gate/registry/upload and cache outcomes in that fixture are
substituted; the actual locked registry fetch above is separate evidence.
ShellCheck also passes for the adopted Rust installer and consumer adapters.

## Delivered 0.2.2 tooling evidence

Release 0.2.2 adopts Shared Tooling 0.1.34 at reviewed committed
revision `3d33cd250fcae7dbe5cabe44b2abd6b2c91a1822` through the canonical exporter
from a clean temporary checkout. The 72-file snapshot omits the unused fleet
reporter and its manifest record; local LOC reporting, pinned cloc and required
verification companions remain selected. The canonical Make include now explains
the optional fleet selection instead of calling a missing file. See
[#1](https://github.com/dragginzgame/ic-jobs/issues/1) for adoption evidence.
No fleet reporter regression is selected by Jobs CI, and sibling repositories
remain read-only. The public GitHub description matches the maintained scope.

The same snapshot adopts the independent IC installer reuse correction from
[shared #79](https://github.com/dragginzgame/shared-tooling/issues/79): compare
validated pin records rather than file presentation. Comment or row-order edits
reuse a verified bundle without rewriting receipts; changed records, checksums
and versions retain their admission checks. Actual tool pins are unchanged.

The CI workflow now includes the pushed commit's SHA in its concurrency group;
only superseded revisions of a pull request share a cancellable group. This
preserves queued and running push qualification without changing the three-host
matrix, permissions, gates or timeout. Delivery and hosted queued/running
retention across consecutive pushes are qualified, with evidence in
[#3](https://github.com/dragginzgame/ic-jobs/issues/3).

Focused Linux tooling checks passed: the committed owner's command fixture with
substitute installers/reporters and IC installation/retention fixture with
synthetic assets for all three host selections; actual offline host-tool
verification and local
`make cloc`; optional fleet-report refusal; snapshot integrity, documentation
links, dependency declarations, release Make adapters, formatting, ShellCheck
and actionlint. The first local LOC attempt lacked cloc; explicit pinned
`make install-host-tools` prepared the checkout-local bundle, with pins unchanged.
Install/check aggregate wiring was inspected with Make dry runs; the complete
Rust/IC bundles were not installed or checked here. The full CI/release gate and
native macOS qualification were not run by the agent for that working-tree
batch. The maintainer subsequently delivered it as 0.2.2; its native macOS CI
qualification passes on all three supported hosts as recorded above.

## Delivered restore and earlier tooling evidence

Continuation source review found that `Job::restore` admitted contradictory
pending records, tracked in [#2](https://github.com/dragginzgame/ic-jobs/issues/2).
Three new regression tests reproduced the incorrect admission against released
0.2.0 before the fix. Restoration now rejects zero-attempt success with no
completed recurring occurrence, retry completion before its scheduled time,
and successful recurring successors inconsistent with completion. The existing
Schedule successor calculation owns those deadlines; CatchUp completion is
checked against its previous scheduled occurrence. Running/Uncertain remain
blocked and public transition error behavior is unchanged. There are no public
API, stored-field, dependency or compiler-floor changes. Records produced by
valid public transitions remain admitted; this is the compatible 0.2.1 patch.
The recurring transition/restore series now covers AfterCompletion as well as
both fixed-rate policies, including retries, uncertainty and cancellation.
Two additional regression cases reproduced shifted first-occurrence deadlines
and cancellation timestamps preceding a retained completion in the earlier
working patch. Restoration now requires the first occurrence's original deadline
in every state and derives a shared completion-time lower bound for untouched
recurrences, including cancelled ones. Pending successors still reuse the
canonical scheduling calculation. Valid cancellation at completion or after
the next deadline is retained; recovery cases also cover zero-time completion,
maximum representable intervals/deadlines and unchanged scheduler selection.
Focused Linux qualification passed: 24 all-feature and 23 core job tests on
Rust 1.88.0, core/all-feature native and Wasm minimum checks, warnings-denied
Clippy, rustdoc and both doctests, and verified all-feature packaging.
Formatting, snapshot integrity, dependency declarations and documentation links
pass. The agent did not run a full gate or deliver this repair; the maintainer
subsequently released and pushed it as 0.2.1, whose three-host native CI now
passes as recorded above. Live IC recovery remains unqualified.

The initial fleet cleanup review found no intentional local caller but awaited
a committed upstream optional-selection correction. That blocker was resolved
by Shared Tooling 0.1.33 and the current 0.1.34 adoption described above.
Focused Linux qualification passed with Rust 1.88.0: native and Wasm library
checks both without timers and with all features, plus all 18 job tests with
all features against that lockfile. Formatting, documentation links, dependency
declarations and the unchanged shared snapshot also verify. Full CI, release
gates, native macOS and live IC qualification were not run for this batch.

The maintainer's subsequent `make release-minor` failed in the CI gate at the
development Clippy `manual_is_multiple_of` lint, newly applicable after raising
the advertised floor. The failure is retained at
`.git/release-state/validation-failures/20261009T061629Z-1284332-0-ci.log`.
Fixed-rate restoration now uses `u64::is_multiple_of`, supported by the required
Rust 1.88 compiler; schedule validation still rejects zero intervals before
this check. Accepted records and error transitions are unchanged. Focused repair
checks passed: warnings-denied Clippy, all 18 job tests on Rust 1.88.0, core and
all-feature native/Wasm minimum checks, and formatting. No release was retried
or metadata bumped by the repair.

The earlier Shared Tooling 0.1.30 adoption used committed revision
`4e274a2219c0b0cc3af68ec65658b373253518fb` through the canonical exporter,
using a clean temporary checkout and excluding dirty sibling work. The snapshot
contains 73 files, including the explicitly selected shared release-source
checker. Both metadata and publication adapters delegate source observations;
only the metadata adapter permits the three local release files. Initial
preflight refusals identify that validation/preparation have not started, and
Git observation failures remain distinct from dirty source.
The shared PocketIC setup pin advances to 16.1.0; no executable installation or
live protocol qualification was performed. Optional npm checks and Cargo-install
qualification are not activated. Focused Linux checks passed: locale-sensitive
real-Git release/publication fixtures with substitute gate/registry/upload
effects, all-path diagnostics, literal allowances, rename-source refusal,
Git-observation failure and source/index preservation. ShellCheck, snapshot
integrity, documentation links, dependency declarations, formatting and standard
Make release adapters passed. The public GitHub description matches current
purpose. No full CI, commit, release, push or upload was performed.

Upstream inspection on 2026-10-09 confirmed IC Host Tooling main at 0.8.4
(`97187b2a46d6f8a6964224a36a133d858ef0d223`); the earlier IC Host Tools repository
remains at 0.2.0. No host dependency is needed by the scheduler library now.
Bounded process capture and child cleanup could serve a future application-owned
PocketIC test harness outside the canister package. No host-crate adoption was
performed.

The maintainer subsequently requested removal of the older GitHub repository
`dragginzgame/ic-host-tools` (0.2.0). Its local checkout was already absent.
A fresh Git mirror at `be7d73908225e73ab5babf8fbb9fe959f40d439b`, repository
metadata, all 10 issues and 15 comments, and empty release metadata were archived
and verified before attempting removal. The retained backup is
`.git/retired-repositories/ic-host-tools-20261009/backup.tar`, with a checked
SHA-256 beside it. GitHub rejected DELETE with HTTP 403: the authenticated CLI
token lacks `delete_repo` scope. No enabled browser session or repository-deletion
connector was available. On the subsequent 0.2.0 continuation, authenticated
GitHub inspection returned 404 for the old repository while the token retained
`repo` access, consistent with its removal outside this agent's attempted DELETE.
The backup remains retained. This retirement did not remove crates.io packages
or the successor `dragginzgame/ic-host-tooling`.

## Earlier implementation and qualification evidence

Core, persistence example, focused state/recovery tests and repository tooling
are implemented. Local Linux validation passed: 14 focused tests with the timer
adapter, Clippy with warnings denied, rustdoc and its example, the runnable
persistence example, and native/Wasm checks at Rust 1.85 core / Rust 1.88 timers.
The adopted snapshot and documentation links verify; release entrypoints passed
their substitute-runner check. Dependency declarations now pass against the
actual tracked lockfile. The complete validation-only release gate passed on
Linux with the locale repair before the 0.1.2 scheduler work. Live IC and native
macOS qualification remain pending.
The public GitHub repository `dragginzgame/ic-jobs` is configured as `origin`,
with authenticated HTTPS push credentials. The initial source is committed at
`da4a555`; the tooling repair is committed at `de61cd3`. The first release,
0.1.1, is tagged at `d5a2603` and the exact annotated tag is present on `origin`.
The retained release plan is complete. A current crates.io observation reports
`ic-jobs` 0.1.1 present; that observation alone does not bind registry bytes to
the local artifact.

Release and publication setup is committed. The Shared Tooling runner remains
at reviewed revision
`1872ed2c20f6c70689bb2249050b1d673c60bfa0`, with its annotated-tag and exact
crates.io observation helpers added to the snapshot. `make publish` separately
requires clean source, the matching pushed annotated tag and a confirmed absent
registry version. Pinned cargo-edit setup, package verification, publication dry
runs and configured CI coverage are available. Source changes must be committed
and Cargo credentials prepared before actual release/publication.

Focused Linux checks passed: real local Git patch/minor/major release fixtures,
failure before mutation, completed-release resume and publication refusals with
substitute gate/registry/upload effects, also under Bash 3.2. Cargo's actual
publication dry run verified the all-feature tarball without upload; rustdoc and
its example, native/Wasm Rust 1.85 core / Rust 1.88 timers, formatting, dependency
declarations, snapshot integrity and documentation links passed. The initial
tarball failure exposed a workspace-relative README include, now fixed with
crate-local README files. The initial symlink implementation was rejected
by pre-commit snapshot admission and replaced with regular package files.
An offline publication dry run was refused by
Cargo's required HTTP observation; `publish-check` now explicitly uses registry
access. These passes do not establish native macOS or live publication evidence.

A maintainer invocation of `make release-patch` failed in the validation gate's
release fixture at its changed-path assertion; logs and the fixture remain at
`.git/release-state/validation-failures/20261008T172821Z-3674222-0-ci.log` and
`/tmp/jobs-release-tooling.KU6QTX`. The failure was reproduced with
`LC_ALL=en_US.UTF-8`: locale-sensitive sorting disagreed with the expected byte
order after successful delivery to the fixture's local bare remote. The comparison
now explicitly uses the C locale, and failed fixture assertions name their source
line. The actual repository's version and tags were not changed by that failed
gate or its repair.

With the repair applied, `LC_ALL=en_US.UTF-8 make test-release-tooling` and
`LC_ALL=en_US.UTF-8 make release-verify` passed. The latter ran the full `ci`
validation target through the same nested logger path as the failed attempt;
it did not prepare metadata, commit, tag or push. The maintainer subsequently
committed the repair and completed the normal 0.1.1 release, with fresh validation
as required by the standard runner.

The corrected changelog separates the untagged initial 0.1.0 implementation from
the dated 0.1.1 delivery tooling and fixes. The 0.1.1 release date is preserved;
the documentation correction does not change package metadata, tags or the
public API/storage contract.

The selected next draft is 0.1.2 for an additive scheduler wrapper. Its release-tooling
prerequisite is implemented: fixtures read the actual workspace version for
increment, failure and publication checks instead of assuming the initial 0.1.0.
The hardcoded assumption was reproduced failing against the released 0.1.1 at
`/tmp/jobs-release-tooling.YMX527`; the revised fixtures pass with
`LC_ALL=en_US.UTF-8`. ShellCheck, snapshot integrity and documentation links pass.
Package metadata remains 0.1.1.

The accepted consumer requirement is a layer between application job intent and
actual due-work/wakeup selection. `Scheduler` now borrows a bounded batch of
application-owned, validated jobs. It derives the earliest pending deadline and
starts one earliest due attempt, returning its exact Running record for the
consumer to commit before executing the effect. Ties follow batch order; restored
Running/Uncertain and terminal jobs remain excluded. All transition checks stay
in `Job::start`, with errors leaving the batch unchanged and no fallthrough to
another job. Storage, payloads, handlers, global indexing and lifecycle remain
consumer-owned; the existing IC Timers adapter receives the derived deadline.
Only a batch covering the global earliest work may supply the global wakeup.

Focused Linux qualification for the working-tree wrapper passed: 18 job tests
with all features, Clippy with warnings denied, rustdoc and both examples,
native/Wasm compilation at Rust 1.85 core / Rust 1.88 timers, verified all-feature
packaging, and the runnable persistence/recovery example. Current-version release
fixtures passed under `LC_ALL=en_US.UTF-8`; declaration, snapshot, formatting and
documentation checks passed. The complete CI/release gate and live IC/native
macOS qualification were not run for this scheduler batch. There are no dependency,
MSRV, durable-record or existing public API changes; 0.1.2 remains the compatible
pending release. No commit, release, tag push or upload was made for this work.

See [the README](../../README.md), [safety contract](../../SAFETY.md) and
[delivery procedure](../releases/README.md).
