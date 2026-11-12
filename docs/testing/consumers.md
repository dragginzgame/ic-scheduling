# Test consumers

## Adapter acceptance checklist

Use this checklist when adopting Jobs through a host adapter such as Canic.
The packaged [persisted-job example](../../crates/ic-jobs/examples/persisted_job.rs)
and [adapter contract](../../crates/ic-jobs/README.md#adapter-integration-contract)
describe the commit and recovery sequence. Adapter implementations qualify their
own persistence and transport boundaries using equivalent scenarios. The
[Canic integration guide](../integration/canic.md) maps ownership and identity
requirements without advertising an implemented adapter.

| Required observation | Existing native reference |
| --- | --- |
| Failed intent commit exposes no delivery and preserves committed state | [notification_commit_failure_never_exposes_delivery](../../qualification/ic-jobs-test-consumer/tests/composition.rs) |
| Lost replies, failed result commits and interrupted delivery preserve blocked intent until matching evidence permits reconciliation | [interrupted_delivery_and_failed_commits_reconcile_without_redelivery](../../qualification/ic-jobs-test-consumer/tests/composition.rs) |
| A known safe retry keeps its payload and rejects old replies | [retained_retry_preserves_payload_and_rejects_late_results](../../qualification/ic-jobs-test-consumer/src/queue/tests/recovery.rs) |
| Reconciliation requires exact payload/effect evidence | [reconciliation_requires_matching_payload_and_prefers_success_evidence](../../qualification/ic-jobs-test-consumer/tests/composition.rs) |
| Invalid records refuse before effects, including records not currently due | [actual Queue admission](../../qualification/ic-jobs-test-consumer/src/queue/tests/admission.rs) |
| Authorization preserves caller and job authority | [actual Queue management](../../qualification/ic-jobs-test-consumer/src/queue/tests/management.rs) and [installed endpoint recovery](../../qualification/ic-jobs-canister-tests/tests/recovery.rs) |
| Failed metadata commits preserve the previous committed deadline | [failed_management_commits_preserve_committed_deadlines](../../qualification/ic-jobs-test-consumer/tests/composition.rs) |
| Retained allocation bindings and blocked effects survive cold reopening | [host_pool_cells_reopen_committed_jobs_without_replaying_blocked_effects](../../qualification/ic-jobs-test-consumer/tests/composition.rs) |
| Wire records retain states/attempts and still require validated restoration | [stored Wire conformance](../../qualification/ic-jobs-test-consumer/tests/stored_wire.rs) |
| Duplicate receipts and late continuations cannot overwrite terminal/successor state | [packaged adapter example](../../crates/ic-jobs/examples/persisted_job.rs) |
| Completion preserves independent management edits made during delivery | [packaged adapter example](../../crates/ic-jobs/examples/persisted_job.rs) |

Native tests establish composition and refusal behavior. Installed acceptance
must separately cover upgrade reconstruction, same-message rollback of storage
and timer changes, and lost replies followed by receipt recovery. Existing
[canister recovery cases](../../qualification/ic-jobs-canister-tests/tests/recovery.rs) include
`management_upgrade_and_real_watchdog_delivery`,
`synchronous_trap_rolls_back_storage_identity_and_timer_changes`,
`invalid_restore_refuses_upgrade_and_preserves_the_controller_snapshot`, and
`lost_result_keeps_intent_blocked_and_receipt_resolution_does_not_redeliver`.
They qualify this repository's fixture on its recorded graph; a Canic adapter
needs its own installed evidence for its actual storage, endpoints and lifecycle.

## Existing consumer qualification

The canister consumer's fixed job-outcome counters are updated after successful
queue commits, independently of Timers' callback observations. Native queue tests
cover committed local success/permanent failure, retry dispatch, uncertainty,
explicit reconciliation, stale/mismatched receipts, rejected Cell writes,
authorization, reset nonmutation and empty windows after reconstruction. The
Wire test preserves u128 totals, actual-overflow flags and distinct inspection/reset
method identities. These tests do not establish actual IC trap rollback or costs.
The explicitly selected live case
`job_metrics_are_authorized_resettable_and_restart_on_upgrade` checks manager
access, reset nonmutation and empty observations after same-format reconstruction.
Historical runs and their exact graph boundaries are recorded in the
[current handoff](../status/current.md). Compilation alone is not live acceptance;
earlier executions do not qualify a changed Storage format.

`make test-consumers` aggregates the small effect and transaction fixtures in
[the integration fixture](../../qualification/ic-jobs-test-consumer/tests/composition.rs), stored Wire
conformance and the packaged adapter example on Rust 1.99.1, using the selected
locked graph. Select `--test composition` or `--test stored_wire` directly for
focused consumer work; run the example separately with
`bash scripts/dev/minimum-rust.sh cargo run -p ic-jobs --example persisted_job --locked --offline`.
Configured CI also runs this target on the three supported hosts. The harness
and its support modules belong to the unpublished consumer package; the core
package has no Storage or Timers development dependency. The composition
fixtures retain failed-commit injection, durable rejection evidence and two
components sharing one host allocation bootstrap. The canister Queue has a
different receipt protocol and commit boundary; it does not replace those proofs.
`check-consumer` selects
the consumer library and frozen API-wire tests separately, avoiding a second
composition or stored Wire run. Installation, management and delivery use Wire
with consumer-owned identities and limits. Typed Wire attributes retain each
endpoint's explicit identity, singleton/multi-argument tuples and traversal
budgets. Their shared pre-copy header/length admission traps malformed or
oversized requests; the former oversized-request guard rejection is retired.
Lifecycle initialization retains its separate bounded decoder. The harness's external PocketIC
interfaces retain required Candid; Testkit uses Platform's external Candid
decoder limits/error types without a separate decoder dependency here.

The [stored Wire target](../../qualification/ic-jobs-test-consumer/tests/stored_wire.rs) checks every
job state, scheduling policy, safe-retry identity, wide job IDs and active attempt
round-trips through `ic_wire::native`. It verifies that a decoded but inconsistent
history still refuses at `Job::restore`, and rejects truncated/trailing input and
byte-budget overflow. This is an additional native consumer codec proof. It
changes neither the canister's bounded Wire queue nor the packaged example's JSON store,
selects no Canic storage envelope and adds no Wire dependency to the core crate.

These consumers exercise the public API without adding storage, payloads,
handlers or lifecycle ownership to the Jobs library. Each application decodes a
bounded JSON snapshot and passes every restored record through `Job::restore`.
The byte store models atomic replacement and can refuse a commit while retaining
the previous bytes. It is a test storage owner, not a stable-memory implementation.

| Consumer | Application requirement | Exercised behavior |
| --- | --- | --- |
| One external effect | Retain one immutable payload with its job, commit before delivery, and reconcile independently supplied evidence | Refused intent/result/resolution commits preserve previous bytes; blocked work never redispatches; exact rejection evidence permits a bounded retry; confirmed success takes precedence |
| Local effect transaction | Commit a job transition and its counter together | Refused commits retain the previous bytes; reopening after a partially committed sequence resumes without repeating work; corrupt records refuse before a write |

The single-effect fixture owns no collection or receiver engine. Its bounded
snapshot carries one validated job and immutable payload. Independently supplied
`Evidence` holds successful execution receipts and exact-attempt rejections;
absence of evidence leaves unresolved work blocked. Successful matching execution
evidence takes precedence over rejection, and stale attempts cannot resolve the
current dispatch. A failed resolution commit leaves the old bytes and retry
budget in force. This fixture models fallible consumer commits; the real Queue
has its own receipt protocol and commit boundary.

Collection admission, ordering, cancellation past blocked work, retry payload
identity and stale-result rejection are exercised by the actual Queue's
[admission](../../qualification/ic-jobs-test-consumer/src/queue/tests/admission.rs) and
[recovery](../../qualification/ic-jobs-test-consumer/src/queue/tests/recovery.rs) tests.
Those tests retain the complete snapshot's corruption checks, including future
records, and the receiver's independent duplicate-receipt checks. The retry case
uses an admitted retained job with a budget; creation still defaults to no retries.

The local-effect fixture stages one job completion and its counter in one
replacement. It adds no cache policy, expiry collection or batch driver. A failure
before the first commit preserves the initial state; failure after one successful
commit preserves that occurrence and resumes at the next deadline. The actual
Queue and Jobs tests retain scheduling, bounded-batch and missed-run policy
coverage. Byte-store fault injection does not model IC message-wide rollback.

Reconstruction keeps only committed bytes and discards volatile consumer state.
The tests cover refused intent/result commits, lost replies, receipt reconciliation,
and corrupt metadata rejection before work. This is native composition evidence.
These native fixtures do not establish IC message rollback, installed lifecycle
reconstruction, watchdog provider delivery or external canister calls. The separate
live harness uses IC Testkit's selected server setup/check contract; see the
source-bound evidence in the handoff rather than treating native passes as live
qualification or claiming that no live qualification has ever run.

## Native composition and management owners

The single-effect and local-effect fixtures keep failed-commit and storage
composition proofs. Every load passes its record through `Job::restore`, and no
delivery escapes before its intent commit. The real qualification canister retains the Jobs/Timers composition proof;
Timers owns its completed-batch result projection tests.

The actual Queue owns collections, management, pagination, receipts and caller
authorization. Native Queue tests and installed endpoint tests qualify those
boundaries. The host fixture separately opens committed IC Storage allocations;
no fixture assigns component memory IDs.

## Consumer memory requests

The unpublished consumer selects IC Storage from the qualification workspace catalog;
`qualification/Cargo.lock` records the exact release. Each fixture contributes one checked `MemoryRequest`
with a distinct authority and permanent logical key: `test.notifications.queue.v1`
for the single job and immutable delivery payload, and
`test.local.effect.v1` for the local-effect job and counter.
These names describe the fixture's intended stores; no numeric slots or private
ranges are selected by the consumers.

The host composes both requests into one `SealedDeclarationSnapshot`. Native
checks establish order-independent declaration meaning, refusal of a foreign
request reusing a consumer key, and unchanged retained job bytes/deadlines after
that refusal. A fresh native `MemoryRuntime` refuses opens before bootstrap;
sealed requests alone do not grant committed allocation authority. Authority
labels express ownership policy, not caller authentication.

The fault-injection consumers retain their bounded byte store. A separate native
host fixture uses IC Storage allocation leases and cells: namespace grants admit the
two namespaces without component ranges or numeric IDs; the host bootstraps once
before component adoption/open. Stable cells persist the same bounded snapshot
bytes. Same-format Storage 0.73 cold reopens retain allocation IDs, payloads and pending deadlines; reads
restore through the existing `Job::restore` boundary. Running/Uncertain intent
remains blocked while an independent local-job cancellation persists across a
subsequent cold reopen, without changing the blocked effect bytes.
Foreign-owner bootstrap fails without changing backing bytes or granting opens.
This is actual Storage collection persistence on native VectorMemory, not an IC
upgrade, message rollback, final Canic/IcyDB composition or live timer evidence.
Storage 0.73 rejects earlier whole-store backing, including 0.71, without writes/growth. The
[consumer's installation boundary](../../qualification/ic-jobs-test-consumer/README.md)
requires fresh backing or explicit owner-authorized retirement, preserving
unresolved effects and evidence; it supplies no automatic reset or migration.
Canic/IcyDB composition needs those applications' own qualification. No old
allocation API or fallback remains in Jobs' consumers. The current format and
installation disposition are documented with the
[canister consumer](../../qualification/ic-jobs-test-consumer/README.md); Jobs' library acquires no
storage dependency or lifecycle hook.

The frozen 0.71.2 backing retains one Running and one Uncertain delivery with
exact receipts. Native recovery and creation must refuse it without writes or
growth. The installed recovery subject seeds those bytes into a stopped fixture,
then verifies post-upgrade rejection leaves the complete stable image unchanged.
It does not run an old application or establish production backup compatibility.

## Durable canister consumer

The unpublished [application canister](../../qualification/ic-jobs-test-consumer/README.md) now
implements an authenticated management API, bounded stable queue, local counter
tasks, external delivery receipts and lifecycle-owned timer reconstruction.
All decoded records still cross `Job::restore`; restored Running/Uncertain work
stays blocked. It owns a new test installation rather than changing these native
consumers' existing snapshots or any deployed format.

`make check-consumer` runs native queue and endpoint-codec tests, builds the Rust
1.99.1 Wasm, and compiles the separate Testkit harness while executing its native
tests. Native/Wasm Clippy is a separate CI selection. Ordinary CI includes these
checks without launching a server. Explicit Testkit CLI/server
preparation and `make test-canister` select the live upgrade, rollback, watchdog
and lost-result scenarios. Durable management now covers pause/resume,
unstarted-occurrence rescheduling, wakeup reconstruction and real management trap
rollback. The live harness also checks Wire endpoint refusal, authorization,
Metrics and receipt persistence. The handoff records historical passing
graphs separately from current qualification. Select live subjects explicitly
for upgrades, invalid restore, blocked effects/receipts, Wire refusal and Metrics. They do not establish cross-version installed
migration/rollback or old-map compatibility. The original small native consumers
remain focused models; they do not substitute for those IC observations.

The queue now contains known no-effect local counter failures as terminal jobs,
while keeping the two-attempt bound and corrupt-record refusal. The additional
real-watchdog scenario traps after its stable batch write, observes the actual
canister trap log, checks rollback and an armed successor, and clears the fault
without rearming before requiring subsequent progress. Failed and completed jobs
must remain terminal after upgrade, with no duplicate committed local effect.
Matching installed evidence includes the tested source, dependency graph and Wasm
variants; the current handoff records those identities and limits.

The current [configuration tests](../../crates/ic-jobs/tests/jobs/configuration.rs)
round-trip current records and retain unresolved dispatch as blocked. Frozen
retired JSON and Candid replies are negative inputs. Earlier physical images are
ledger-refusal evidence, not a compatibility promise. The frozen Memory 0.55.2
installation still contains valid current queue JSON whose Running job restores
as blocked; its outer CBOR ledger refuses without any reset. Jobs expands its
checked-in physical images before any runtime acquires custody, enforcing the
three-page fixture ceiling and producer/schema assertions. The captured 0.63
Wire installation retains both Running/Uncertain intent and exact receipts; its
old physical image is also refused without writes. Current Cells use explicit
load ceilings and checksums; empty recovery never initializes replacement state. Both retained opening
and fresh creation refuse those ledgers with zero writes and identical backing
bytes. Current-format corruption remains a queue-wide refusal. Raw zero intervals
return `InvalidInput` without committing, and the fault endpoint deliberately
mutates encoded bytes without adding an unchecked configuration constructor.
These native checks do not establish live IC upgrade qualification.
