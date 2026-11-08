# Test consumers

## Adapter acceptance checklist

Use this checklist when adopting Jobs through a host adapter such as Canic.
The packaged [persisted-job example](../crates/ic-jobs/examples/persisted_job.rs)
and [adapter contract](../crates/ic-jobs/README.md#adapter-integration-contract)
describe the commit and recovery sequence. Adapter implementations qualify their
own persistence and transport boundaries using equivalent scenarios. The
[Canic integration guide](canic-integration.md) maps ownership and identity
requirements without advertising an implemented adapter.

| Required observation | Existing native reference |
| --- | --- |
| Failed intent commit exposes no delivery and preserves committed state | [notification_commit_failure_never_exposes_delivery](../apps/ic-jobs-test-consumer/tests/composition.rs) |
| Lost replies, failed result commits and interrupted delivery preserve blocked intent until matching evidence permits reconciliation | [interrupted_delivery_and_failed_commits_reconcile_without_redelivery](../apps/ic-jobs-test-consumer/tests/composition.rs) |
| A known safe retry keeps its payload and rejects old replies | [known_rejection_retries_same_payload_and_rejects_old_reply](../apps/ic-jobs-test-consumer/tests/composition.rs) |
| Reconciliation requires exact payload/effect evidence | [reconciliation_requires_matching_payload_and_prefers_success_evidence](../apps/ic-jobs-test-consumer/tests/composition.rs) |
| Invalid records refuse before effects, including records not currently due | [notification queue admission](../apps/ic-jobs-test-consumer/tests/composition.rs) and [actual Queue admission](../apps/ic-jobs-test-consumer/src/queue/tests/admission.rs) |
| Authorization preserves caller and job authority | [actual Queue management](../apps/ic-jobs-test-consumer/src/queue/tests/management.rs) and [installed endpoint recovery](../crates/ic-jobs-canister-tests/tests/recovery.rs) |
| Failed queue mutations preserve the previous committed deadline | [failed_queue_mutations_preserve_committed_deadlines](../apps/ic-jobs-test-consumer/tests/composition.rs) |
| Retained allocation bindings and blocked effects survive cold reopening | [host_pool_cells_reopen_committed_jobs_without_replaying_blocked_effects](../apps/ic-jobs-test-consumer/tests/composition.rs) |
| Wire records retain states/attempts and still require validated restoration | [stored Wire conformance](../apps/ic-jobs-test-consumer/tests/stored_wire.rs) |
| Duplicate receipts and late continuations cannot overwrite terminal/successor state | [packaged adapter example](../crates/ic-jobs/examples/persisted_job.rs) |
| Completion preserves independent management edits made during delivery | [packaged adapter example](../crates/ic-jobs/examples/persisted_job.rs) |

Native tests establish composition and refusal behavior. Installed acceptance
must separately cover upgrade reconstruction, same-message rollback of storage
and timer changes, and lost replies followed by receipt recovery. Existing
[canister recovery cases](../crates/ic-jobs-canister-tests/tests/recovery.rs) include
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
[current handoff](status/current.md). Compilation alone is not live acceptance;
earlier executions do not qualify a changed Storage format.

The tooling fixture binds its Cargo stubs in Make's incoming PATH, including
direct and recursive demo recipes on macOS GNU Make 3.81. Failed demo commands
print their retained log and preserve the original exit status. The existing
recorded-command cases still verify build/admission order and refused effects.

`make test-consumers` aggregates two small application-owned consumers in
[the integration fixture](../apps/ic-jobs-test-consumer/tests/composition.rs), stored Wire
conformance and the packaged adapter example on Rust 1.99.1, using the selected
locked graph. Select `--test composition` or `--test stored_wire` directly for
focused consumer work; run the example separately with
`bash scripts/dev/minimum-rust.sh cargo run -p ic-jobs --example persisted_job --locked --offline`.
Configured CI also runs this target on the three supported hosts. The harness
and its support modules belong to the unpublished consumer package; the core
package has no Storage or Timers development dependency. The small composition
consumers retain failed-commit injection, durable rejection evidence and two
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

The [stored Wire target](../apps/ic-jobs-test-consumer/tests/stored_wire.rs) checks every
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
| Deferred notifications | Deliver a queue of at most four immutable messages at their deadlines, retry only a known rejection, reconcile a lost reply against destination receipts | Commit intent before exposing delivery; retain payload and execution identity across retries; reject stale replies; block unresolved work after reconstruction while other due jobs proceed; complete without redelivery after a confirmed receipt |
| Cache maintenance | Remove expired entries locally on a fixed-rate schedule, with at most two occurrences per wake and 32 cache entries | Commit local effects and metadata together; reconstruct pending deadlines; bound overdue CatchUp work; Skip missed intervals; cancel pending work; derive the next timer decision |

The notification scheduler produces an outstanding delivery for a separate
application continuation. The synchronous scheduling step makes no external call.
Its destination is an independent receipt owner: repeated execution identities
retain the original payload and produce one logical delivery. An absent receipt
leaves the uncertain job blocked. A known rejection before the effect permits an
explicit retryable classification; a missing reply does not.

The destination retains rejection evidence for the exact `Attempt` and payload,
so a lost rejection reply can be reconciled to a safe retry after sender
reconstruction. That disposition must commit before the next attempt becomes
eligible. Repeated requests retain the destination's recorded result; a confirmed
successful execution takes precedence over an earlier rejected attempt. Stale
attempt evidence cannot resolve the current dispatch. Exhausting the original
retry budget leaves the job terminal rather than allocating another attempt.

The notification queue supplies all its bounded records to `next_due_ns`, so the
derived deadline covers its earliest pending work. Equal deadlines retain queue
order. Enqueue and cancellation update the committed queue before the application
derives its wakeup again. Duplicate identities and excess records are rejected on
both insertion and reconstruction; a corrupt future record blocks dispatch of
an otherwise valid due record. Outstanding envelopes are inspected for
reconciliation, never automatically replayed. An interruption after intent
commit but before delivery still reconstructs blocked work: the receiver's absent
receipt alone does not establish retry safety.

Cache maintenance has only local effects, so it commits the pruned entries and
completed job transition in the same replacement. A failed replacement commits
neither. The loaded snapshot holds one validated job with its entries/counter;
there is no second record copy to synchronize. Notification and maintenance
snapshots serialize validated jobs directly through the canonical `JobRecord`
writer. Reads still decode records and pass through `Job::restore`.
Its bounded wake result constructs a direct IC Timers decision.
Tests check future and overdue scheduling decisions and stopping after
cancellation; the native fixture does not call the IC timer provider.
The store can also refuse the second write in a batch. The first committed
occurrence survives; reconstruction resumes at the second occurrence without
repeating the first. This byte-store fault injection does not model an IC trap's
message-wide rollback.

Reconstruction keeps only committed bytes and discards volatile consumer state.
The tests cover refused intent/result commits, lost replies, receipt reconciliation,
and corrupt metadata rejection before work. This is native composition evidence.
These native fixtures do not establish IC message rollback, installed lifecycle
reconstruction, watchdog provider delivery or external canister calls. The separate
live harness uses IC Testkit's selected server setup/check contract; see the
source-bound evidence in the handoff rather than treating native passes as live
qualification or claiming that no live qualification has ever run.

## Native composition and management owners

The notification fixture focuses on bounded payloads, commit-before-dispatch,
exact receipts and blocked-effect recovery. Every load validates all records
through `Job::restore`; corrupt future records also block otherwise due work.
Failed enqueue/cancel/dispatch commits preserve the previous bytes and deadline.
Dispatch exposes its envelope only after committing intent. Direct Timers
composition checks ScheduleAt/Stop projection without registering provider wakeups.

The actual Queue owns management, pagination and caller authorization. Its native
tests and installed endpoint tests qualify those boundaries. The notification
fixture uses the byte-replacement store to inject commit failures; it maintains
no second endpoint or authorization API. The host fixture below separately opens
committed IC Storage allocations. Neither fixture assigns component memory IDs.

## Consumer memory requests

The unpublished consumer selects IC Storage from the root workspace catalog;
`Cargo.lock` records the exact release. Each fixture contributes one checked `MemoryRequest`
with a distinct authority and permanent logical key: `test.notifications.queue.v1`
for job records and immutable notification payloads, and
`test.maintenance.state.v1` for the maintenance record, entries and counter.
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
bytes. Cold reopens retain allocation IDs, payloads and pending deadlines; reads
restore through the existing `Job::restore` boundary. Running/Uncertain intent
remains blocked, and cancellation persists across a subsequent cold reopen.
Foreign-owner bootstrap fails without changing backing bytes or granting opens.
This is actual Storage collection persistence on native VectorMemory, not an IC
upgrade, message rollback, final Canic/IcyDB composition or live timer evidence.
Canic/IcyDB composition needs those applications' own qualification. No old
allocation API or fallback remains in Jobs' consumers. The current format and
installation disposition are documented with the
[canister consumer](../apps/ic-jobs-test-consumer/README.md); Jobs' library acquires no
storage dependency or lifecycle hook.

## Durable canister consumer

The unpublished [application canister](../apps/ic-jobs-test-consumer/README.md) now
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
metrics/logging and receipt persistence. The handoff records historical passing
graphs separately from the current Storage 0.66 graph. Six selected installed
cases cover current-source upgrades, invalid restore, blocked effects/receipts,
Wire refusal and diagnostics. They do not establish cross-version installed
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

The current [configuration tests](../crates/ic-jobs/tests/jobs/configuration.rs)
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
