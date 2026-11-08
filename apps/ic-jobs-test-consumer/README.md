# Jobs test consumer

This package also owns the native notification/maintenance composition harness
in `tests/composition.rs`, plus stored-record Wire checks in `tests/stored_wire.rs`.
`make test-consumers` aggregates both targets and the packaged adapter example;
`check-msrv` qualifies core and consumer targets separately on Rust 1.99.1. Storage and
Timers integration dependencies belong here, outside the core package tests.

Ordinary native library builds expose `api` DTOs and management/delivery identities/limits,
and depend on Platform's Principal, Wire and Serde. Jobs, Storage and Timers are
selected only for Wasm builds and native tests; JSON is native-test-only. The
recovery harness can share the DTOs without
pulling in the consumer's storage or timer implementation. The existing native
normal/build graph check enforces this boundary.

IC Logging is likewise selected only for consumer Wasm and native tests. The
published Jobs library and normal native DTO/codec graph do not depend on it.

Native notification snapshots serialize validated jobs directly. Maintenance
keeps its validated job, cache entries and effect counter in one snapshot,
committing after each completed occurrence. Both readers decode records and
enter through `Job::restore`; serialized metadata keeps its canonical shape.

This unpublished canister is an application-owned consumer of Jobs, Storage and
Timers. It is a qualification fixture with deliberate fault endpoints, and must
not be deployed as a production service. The Jobs library acquires no storage,
authentication, payload, lifecycle or direct CDK timer ownership.

The fixture and native callers use Wire 0.7's one-pass decoder. Decode success
does not prove that re-encoding reproduces the original frame: Serde visitors
can normalize inputs. Current RPC DTOs use ordered records, enums, sequences,
strings and Principals; receipt reconciliation compares every typed delivery
field, rather than re-encoded bytes or a hash. Request/reply budgets remain
explicit and apply independently to encoding and decoding. Nonminimal integers
are refused as malformed. Rebuild the fixture and harness together; method IDs,
emitted RPC layouts are unchanged. The queue format cut below is separate.
Future exact-frame retries or signatures must retain the original immutable
frame bytes.

Native codec tests retain Jobs' envelope shapes, method selection, resource
limits and refusal/uncertainty behavior. Exhaustive prefix truncation and generic
integer canonicality proofs stay with the selected Wire 0.7.23
[native tests](https://github.com/dragginzgame/ic-wire/blob/e90aed4f7dc0cd7ee31a9bcfb211044220ab0cec/crates/ic-wire/tests/native/mod.rs).
Jobs keeps representative truncated payloads and malformed replies at its own
request/reply boundaries, alongside the separately selected live endpoint tests.

The manager principal is supplied at installation and persisted. Management
calls use the actual IC caller before reading or mutating the queue. `create`
allocates an identity, `inspect` reads one job, `list` returns up to eight jobs
after an exclusive ID cursor, `cancel` cancels Pending/Paused work, and `dispatch_due`
starts at most two eligible attempts. All task history is retained: IDs cannot
be reused. `pause` suspends pending work; `resume` restores its original deadline
and policy. `reschedule` replaces timing from an unstarted occurrence forward,
preserving identity/payload and keeping paused work paused. Already-attempted,
blocked and terminal work refuses rescheduling. There is no force-run command.

Inspection/list replies carry pending time as `Status::Pending { due_ns }`;
other status variants carry no dispatch deadline. The separate `View::due_ns`
field is removed. Rebuild the qualification canister and harness together for
this management-response cut. Stored jobs and delivery receipts keep their
logical fields; this response change alone needs no storage reset or conversion.

The consumer host supplies Storage with the `test-jobs` authority and `test.jobs.`
namespace grant, then opens the permanent `test.jobs.consumer.management.v1` key.
Storage assigns the opaque allocation identity and supplies the exclusive lease.
Installation selects `StableCell::create`; recovery selects `StableCell::load`
with a 24 KiB ceiling, validating the header and length before decoding. Cells
have no checksum; the consumer validates the queue identity and records. This
does not detect every valid-looking byte change. One stable cell retains a
native Wire snapshot prefixed with the queue-owned identity `JQW\x01`:
manager/receiver configuration, ID authority, JobRecords, immutable payloads,
counter and receipts. The queue holds at most 32 jobs and 32 receipts, each
delivery payload is at most 256 UTF-8 bytes, and serialized state is at most
24 KiB including its four-byte identity. Wire admits at most 24,572 payload bytes,
1 MiB traversal allocation, 8,192 nodes and depth 32 in each direction. These
are codec traversal allowances, not measured heap usage. The full-queue native
case exercises 32 retained jobs and 32 receipts with 256-byte payloads, every
state/schedule shape and wide identities; the byte ceiling remains independent
of the collection limits. The complete bounded queue supplies the global
earliest deadline, so no separate unbounded due index is needed. Every decoded record crosses
`Job::restore`, including future work. Successful decoding retains validated jobs
for the complete synchronous operation. Each in-memory entry keeps its job and
payload together; the scheduler borrows only the jobs through an iterator. Commit
serializes validated jobs directly and checks collection metadata by borrowing
entries before replacing stable bytes. Encoded capacity still takes precedence
over collection errors. Storage 0.69's `StableCell::set` persists admitted bytes
and directly returns the previous value; construction and load retain typed errors.
Existing Storage 0.68 cell images need no conversion. As before,
engine/codec failures trap before a committed reply or deadline is returned.
IC execution supplies rollback. A native mutation panic may leave partial bytes
and poison the shared owner; discard its runtime and leases rather than retrying
through the failed queue.
Reads restore each record before its entry metadata
checks, after header admission and before receipt checks. Separate job/payload
alignment state is unnecessary. Each latest result stores its outcome
and timestamp together using the current `RecordedOutcome` shape.
Invalid retained state refuses recovery; installation cannot overwrite existing
allocations. Oversized-cell refusal preserves the exact cell header and payload;
bootstrap may update allocation bookkeeping before that refusal. The frozen
incompatible-ledger cases separately retain their zero-write proof.

Storage owns the backing interface, allocation directory and governed Cell used
by both native composition and the qualification canister. `MemoryBacking`
identifies aliases; the native metering wrapper delegates physical identity to
its VectorMemory. A cell or detached lease retains custody until it is dropped.
The consumer never assigns numeric IDs or creates unmanaged stores.
Storage's `test-support` feature supplies native `MeteredMemory` and
`MemoryIoCounts`, including backing identity. The old Testkit Native dependency
and its trait bridge are removed; deployed Wasm selects no metering helpers or
stable-structures substrate.

The oversized-payload fixture uses a larger Cell load ceiling and writes a
bounded oversized value; normal recovery refuses it without replacing bytes.
Native tests also damage the queue identity and reject empty recovery without creating
replacement storage. These checks do not establish IC rollback.

**Storage 0.64 installation boundary:** Storage replaces the separate Memory
package and fixed-slot ledger with an owned allocation directory and checksummed
Cell framing. Older complete physical images, including the previous 0.63 Wire
queue installation, cannot be upgraded in place. Both recovery and fresh creation
refuse them with zero writes and identical backing bytes. Use fresh qualification
backing; do not clear only the directory or queue. No reset or migration is
performed by this adoption. Preserve old binaries, locks, backups, exact blocked
attempts and receipts until their owner resolves the effects and authorizes any
destructive retirement. The queue's own `JQW\x01` payload remains unchanged.
Current Storage-format cold reopen still restores Running/Uncertain jobs blocked.

**Historical Storage 0.66 Cell continuity:** this consumer used Cells, not the changed map
format. A real 0.64 installation captured before the update reopens under 0.66,
Wire 0.16 and Types 0.18 with both Running/Uncertain attempts and receipts intact.
Cold admission, idle dispatch and refused fresh creation perform no writes or
growth. No reset is required for these qualified Cells. This native evidence
does not establish installed IC upgrade/rollback or compatibility for old maps.

**Storage 0.68 installation boundary:** the new copied ICSS Cell engine uses
`SCL1` framing and no checksum. The previous `ICCELL01` images, including the
frozen 0.64 installation above, are now refused during recovery and creation
with zero writes/growth and identical backing bytes. Use fresh qualification
backing. Preserve prior Wasm, locks, backups, blocked attempts and exact receipts;
their owner must resolve outstanding effects and separately authorize any
destructive retirement. No legacy reader, conversion or automatic reset is added.
The queue's `JQW\x01` bytes and validated Job restoration remain unchanged.

**Queue JSON retirement:**
[#78 — Replace job-consumer JSON queue persistence with bounded IC Wire payloads](https://github.com/dragginzgame/ic-jobs/issues/78).
The current reader accepts only `JQW\x01`
and bounded native Wire. Existing JSON queues are refused, including those
with an otherwise valid historical Memory ledger. There is no JSON fallback, converter, automatic
reset or in-place upgrade path. No retained installation is reset by this update.
For a local qualification run, create fresh test canisters. For any retained
installation, quiesce management and dispatch, keep the exact old Wasm, lockfile,
backups, immutable attempts and destination receipts, and use the old reader to
reconcile Running/Uncertain work or record its explicit owner-approved disposition.
An absent receipt, timeout or expired lease never authorizes redelivery.
Retire that old installation only after its obligations are resolved; any
destructive reinstall requires its owner's separate authorization. Do not clear
only the queue or allocation ledger or reuse old effect identities. Preserve
the old evidence after retirement. Sender and receiver queues both need this
disposition; updating the host harness cannot make retained JSON readable.

The frozen Memory 0.56.0 JSON installation retains both Running and Uncertain
attempts plus exact receipts. Current retained opening and fresh creation refuse
its complete retired physical image before payload loading, preserving every
backing byte without allocation bookkeeping writes.
Same-format Wire reopen/upgrade still restores blocked jobs without redispatch.

**0.16 hard cut:** Construct `Task::Deliver(DeliveryPayload { destination,
message })`; delivery and receipt envelopes contain `payload` beside their
job/occurrence/sequence identity. Rebuild senders, receivers and the recovery
harness together. The payload owns one validation rule, applied independently
to retained tasks and incoming/retained delivery envelopes. Exact whole-envelope
comparison and missing-evidence blocking remain unchanged.

Timestamp-bearing Running records and flat receipt envelopes from 0.15 are
refused, as is the older 0.7.5 snapshot. The maintainer selected the hard cut
without preserving affected earlier installations; create fresh test state under
consumer authority. Source adoption and failed recovery do not clear, convert or
overwrite retained cells. There is no compatibility reader or automatic reset.
Current-format Running/Uncertain jobs still restore as blocked. Frozen previous
record, snapshot and Candid bytes are negative inputs only; their prior reader
qualification and provenance are recorded with the stored-record fixtures.

The frozen 0.7.5, Memory 0.40 and Memory 0.55.2 physical images are refused during
both recovery and installation, before payload admission and with zero stable writes,
unchanged pages and identical backing bytes. The queue codec and Storage's outer
directory are independent admission boundaries. The stable key identifies the
consumer allocation; it does not make incompatible payload formats readable.
The Memory 0.55.2 image retains historical queue JSON with Running intent. A
test-only read of its frozen metadata crosses Job::restore as blocked; neither
that payload nor its outer ledger has a current reader. Producer/hash details
are recorded with the
[stored-record fixtures](../../crates/ic-jobs/tests/fixtures/README.md).

`Increment` performs a local counter effect and completes its job in one stable
replacement. A counter overflow has no effect and permanently fails that job,
including recurrence, without undoing unrelated attempts in the bounded batch.
Storage or transition failures still refuse the complete batch. `Deliver`
persists Running intent before calling an independent
receiver. The synchronous watchdog only prepares a bounded batch; an application
continuation performs each external call. An exact matching receipt records
Success. Other responses remain Uncertain. No automatic retries are configured.
Running and Uncertain records never redispatch after an upgrade.

`receive_delivery` and `delivery_receipt` accept only IC Wire native Serde frames,
with the method identities and limits in `delivery_wire`. Requests admit 1 KiB
frames, 16 KiB allocation accounting, 128 nodes and depth 16. Responses admit
8 KiB frames, 64 KiB allocation accounting, 1024 nodes and depth 16. These are
Wire traversal allowances, not Candid decoding quotas or measured peak memory.
An endpoint guard checks the raw request length before Platform copies it.
The shared DTO declaration order defines the Postcard layout. Identity, direction,
canonicality and complete consumption are checked before result admission.
Transport, decode and remote application errors remain unreadable evidence;
they do not become an authoritative absent receipt or a safe retry.
The endpoints delegate codecs to `ic_wire::endpoint` through Platform's explicit
hooks. The Wasm-only Wire `endpoint` feature supplies these helpers; its generated
attributes and default IDs/budgets are not selected. Local length guards preserve
the existing oversized-RPC rejection codes before copying; init still traps on
invalid arguments. Codec failures and response encoding failures trap, with a
response failure rolling back the synchronous receipt write. No Candid service
is generated. Manager-only diagnostic endpoints use the same authorized queue
accessor as other management calls, without loading durable state for heap reads.

There is one current protocol: quiesce delivery dispatch, finish or explicitly
dispose of in-flight calls, and rebuild/upgrade sender and receiver together
before resuming. There is no Candid fallback. Retained jobs/receipts keep their
logical identities through the separate queue format cut above; unresolved
effects still require exact evidence.
Installation, management and fixture controls also use Wire. Their method
identities are in `management_wire`; requests admit 4 KiB frames, 32 KiB allocation
accounting, 256 nodes and depth 16. Replies admit 64 KiB frames, 256 KiB allocation
accounting, 4096 nodes and depth 32, including eight bounded views. Unit RPCs
require a valid request frame. Init admits raw argument length before copying
because lifecycle macros have no guard hook. Post-upgrade has no arguments.

Rebuild the harness with this canister and dispose of old Candid management/init
requests before resuming activity. Earlier delivery frame identities remain
unchanged. Earlier Candid files are frozen negative inputs, with no
compatibility decoder. The native consumer DTO/codec graph has no PocketIC,
Candid, process or server dependency.

On Wasm, `ic-platform-runtime` supplies explicit-codec entrypoints, raw calls,
system helpers and protected spawning. Timers shares that Runtime owner and its
exact singleton guard; full upstream `ic-cdk` and a second executor are excluded.
Canonical Types owns application identities. Native DTO normal/build graphs stay
independent of the runtime; external client conversions remain at their boundary.
The root dependency catalog owns the selected Runtime, Types and Metrics versions.
The consumer selects Metrics arithmetic and reset bookkeeping only on Wasm and in
tests; RPCs project consumer-owned numeric DTOs. The Wasm timer subtree contains
neither Wire nor Candid. No stable-structures substrate remains in the workspace.
The consumer's own Wire dependency supplies its endpoint codecs. Metrics has no
optional `wire` feature; its optional keyed heap `recording` feature stays disabled.
Published `ic-jobs` remains independent of Metrics.
Metrics now lives in the IC Observability repository with the same published
crate identity and arithmetic. Runtime 0.20 and Wire 0.17 supply explicit endpoint
codecs and bounded decoding for this consumer and its host harness.
Its transport boundary is separate from Storage's installation cut above.
Native DTOs select canonical Types 0.18. Native tests explicitly select
`principal-self-authenticating`; the recovery harness selects Types' explicit
external conversions and Testkit 0.88's async-only `ic_testkit::pocket_ic` client.
Managed `PocketIcStartupConfig::build_instance` supplies the required server URL
and keeps server/instance custody together; clients do not implicitly provision it.
The harness owns its Tokio runtime and awaits instance closure before the server
stops; updates are not automatically retried or replayed.
The application uses its owned Principal;
PocketIC calls retain the external Principal and convert explicitly when
constructing application DTOs. Native DTO normal/build graphs exclude external
Principal and hashing dependencies; Wasm excludes the application conversion
feature and SHA-2. Principal bytes are unchanged.
Upstream adoption preserves RPC frames and job semantics; the queue retirement
above separately changes the codec. No timer, task or storage owner is replaced.
Native Testkit uses Platform 0.23 for external simulator interfaces and packet
codecs. PocketIC retains its required transitive Candid dependencies at that
boundary. The [workspace catalog](../../Cargo.toml),
[lockfile](../../Cargo.lock) and [handoff](../../docs/status/current.md) own
selection and source-bound qualification.

The live recovery scenarios retain their explicit gate. Native tests and
cross-compilation do not establish installed callback/trap/upgrade behavior.

Live recovery scenarios install only the sender by default; delivery scenarios
explicitly add a receiver. The receiver uses the same fixture Wasm with
`delivery_sender` configured to the
sender canister's principal. It authenticates this caller and retains each
logical job/occurrence receipt with its exact attempt and payload. Repetition of
the same envelope returns its existing receipt; conflicting payload/attempt
evidence is refused. `reconcile_delivery` reads a retained receipt and finishes
the exact blocked attempt. Missing evidence leaves the job blocked. Management
authentication and receiver authentication are independent of correlation IDs.

Each submission goes directly through Platform's `Call::unbounded_wait` once,
with the application-owned Wire frame encoded before the call. Running intent
commits before delivery escapes. Transport rejection or invalid reply bytes leave
the effect uncertain; the transport performs no physical retry or fallback.
Reopening, upgrading or losing a call future cannot authorize redispatch.
Reconciliation requires the exact retained receiver receipt. Fixed borrowed
method names fit the supported raw Platform route; no `ic-call` adapter is needed.
The [Platform ownership issue](https://github.com/dragginzgame/ic-jobs/issues/75)
records the source-bound installed qualification.

`init` creates state. `post_upgrade` bootstraps/open-validates retained memory,
restores the complete bounded queue, initializes Timers and reconstructs the
consumer's single watchdog. No provider handles or closures are persisted.
Successful create/management/dispatch/completion rederives its deadline; no Pending
work leaves it inactive. Reconciliation failure traps the same synchronous
segment as the stable write, preventing a falsely successful persisted schedule.
External-call continuations have separate message boundaries.

The shared batch budget is two attempts, including failed local attempts and
prepared deliveries. Watchdog work counts those handled attempts rather than
confirmed external effects, and an empty batch reports NoWork. CatchUp can still
require many later callbacks and delay newer deadlines; the batch limit is not
a backlog or cycle budget. Prefer Skip when only the current maintenance state
matters. The manager-only `fixture_trap_watchdog_batch` fault control traps after
the stable batch write and before any delivery escapes. Clearing it does not
rearm the timer, so the live recovery case must use the previously armed successor.

Run focused native/Wasm/host compilation with `make check-consumer`. Native
VectorMemory checks qualify cold reopening, bounded CatchUp/Skip batches,
refused writes, identity retention, exact receipt disposition and corrupt-record
refusal. They do not establish IC upgrades or message-wide trap rollback.
`make clippy` owns strict linting of core, native consumer/harness and Wasm
consumer code; CI runs it before builds and tests.

The host harness lives in [ic-jobs-canister-tests](../../crates/ic-jobs-canister-tests).
All its live tests are ignored by ordinary test execution. Explicit preparation
and live qualification are separate commands:

```bash
cargo fetch --locked
make install-testkit-tools
make install-consumer-server
make test-canister
```

To qualify the selected Binaryen optimizer against the same live recovery
scenarios, explicitly prepare it with `make install-ic-tools`, then run
`make test-canister-optimized`. This tests the original Wasm and its `-O3`, `-Os`
and `-Oz` outputs using the input's declared Wasm features. The Rust capture tool
owns each optimizer invocation, its log, exact capture and producer retirement;
IC Build owns capture finalization and retained-path errors. It
retains every input/output, hash, optimizer log and per-variant test/server log.
Failures stop before later variants and retain earlier evidence. This target
uses the unpublished host harness's `jobs-capture-artifact` executable, backed by
IC Build's exact capture and explicit retention. Jobs owns evidence filenames,
optimizer flags and final custody. Rename failures retain captured bytes and
print their location; success removes only the emptied staging child. The
original and every optimized output are captured before qualification, and the
capture executable's digest is recorded alongside source/tool evidence. Tooling
fixtures compile this helper and exercise it without launching PocketIC.
This target does not change the normal `build-consumer` artifact or select a production
optimization policy; ordinary CI still compiles without launching PocketIC.

The canonical Shared Cargo installer selects the published Testkit CLI directly
from the single `ic-testkit` package in [Cargo.lock](../../Cargo.lock), using
`--lockfile Cargo.lock` for both explicit setup and offline admission. There is
no separate CLI version pin. Rerun `make install-testkit-tools` when changing the
locked library selection; malformed, ambiguous or unsupported package selections
fail before installation. Testkit owns PocketIC setup,
offline admission and server lifecycle; this repo has no second server installer
or pin catalog. Normal checks install nothing. `test-canister` checks the CLI and
server before executing tests and uses Cargo metadata to locate the built Wasm,
including a caller-selected target directory. Linux x86-64 and both macOS
architectures use Testkit's supported server selections. Host Testkit/Host crates
remain outside the canister and public Jobs normal Wasm dependency graphs.

Select [the recovery workflow](../../.github/workflows/recovery.yml) through
**Actions → Canister recovery → Run workflow** and choose the
tag or branch to qualify. This manual trigger runs all eleven tests on the original and
three optimized variants on Linux, Apple Silicon and Intel macOS.
Ordinary push/PR CI still compiles the harness without starting
PocketIC. See [GitHub's manual workflow instructions](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/manually-run-a-workflow).

Each host uploads a `jobs-recovery-<host>-<run>-<attempt>` artifact for 14 days on
success or failure. It records the selected commit, lockfile, snapshot, Rust
toolchains, Testkit receipt, test outcome/output and the actual tested Wasm with
its SHA-256. Testkit captures server stdout/stderr in the same artifact; failed
setup additionally retains canonical tool evidence. `test-canister` admits the
prepared server and compiles Wasm and the host harness before starting the
15-minute server lifetime. Local runs retain the Wasm/hash and server logs in
the reported `jobs-recovery.*` directory under an explicit `RUNNER_TEMP`/`TMPDIR`
override, or the workspace's `target/tmp` by default.

The live harness covers management authorization, wakeup changes, real local and
external timer delivery, upgrade retention, overdue policies, synchronous
write/timer rollback, lost sender-result traps with retained destination receipts,
absent-receipt blocking and invalid-record upgrade refusal with explicit
controller-snapshot recovery. Management coverage adds paused/rescheduled
upgrade retention, authorization, wakeup stop/rearm, overdue resume policies and
real trap rollback after management and timer reconciliation. Deliberate `fixture_*` endpoints implement these
faults in the unpublished Wasm rather than faking IC behavior with `cfg(test)`.
Their API must never be copied into a production canister. Live execution and
native macOS acceptance are recorded separately in the
[handoff](../../docs/status/current.md). It records passing installed recovery on
earlier source/graphs, including all three hosts for 0.17.46, separately from the
Storage 0.66 native/Wasm checks and six selected installed cases. Those
cases use fresh current Wasm and same-source upgrades; they do not establish
cross-version installed migration, rollback or old-map compatibility. Harness
compilation alone does not establish installed behavior.

## Committed job-outcome diagnostics

The manager can query `job_metrics` or call `reset_job_metrics`; both use distinct
unit-request Wire identities and the existing request/reply limits. The interactive
queue client exposes `metrics` and `reset-metrics`. Existing summary/job replies
and retained snapshots are unchanged. Reset performs no stable write or timer
reconciliation and cannot change an attempt, retry budget or blocked effect.

The recorder is a fixed heap-local `MeasurementWindow` of `MeasurementCounter`
values from Metrics 0.29, with no job-ID/payload labels or dynamic table. The
consumer projects its own report DTOs; recorders implement neither Serde nor Wire.
It observes admitted Jobs
operations after the owning queue commit succeeds:

| Counter | Unit and admission |
| --- | --- |
| dispatched | Committed attempts, including local effects and remote intent |
| retry_dispatches | Dispatches after the first attempt of an occurrence |
| succeeded | Committed success, including exact receipt-confirmed resolution |
| retryable_failures | Committed retry-safe failures; budget exhaustion does not reclassify the outcome |
| permanent_failures | Committed permanent-failure classifications |
| uncertain | Committed uncertain results, including repeated unresolved resolutions |
| reconciliations | Successful committed `Job::resolve` operations, including still-uncertain dispositions |

Refused operations, stale/mismatched callbacks, idle turns and failed writes do
not increment these counters. Timer callback success is not job success. A
transport rejection classified as uncertain counts only after that outcome commits.
Each report preserves u128 totals and sticky actual-overflow flags; an exact
u128::MAX remains distinct from lost observations. No distribution is collected.

Compare deltas only for the same canister ID, IC canister version and available
reset sequence. The version is read from Platform at the endpoint, never supplied
by the request. The shared window advances its local sequence on reset and reports
None permanently after sequence exhaustion. Upgrades/reopening start empty heap
observations; they do not backfill history from durable JobRecords or promise
cross-upgrade continuity. Production query/reset paths retain manager authorization.
Settings changes can advance the IC version without clearing heap counters; the
version is a conservative continuity label. Only reset or reconstruction clears counts.

Native fixtures prove commit/refusal ordering and report encoding, with zero
stable payload changes on failed commits/reset. Linux live recovery also checks
that dispatch, success and permanent-failure observations roll back with a trapped
watchdog message. The [handoff](../../docs/status/current.md) binds that observation
to its source/graph and original/optimized Wasm. IC instruction/cycle costs remain
unmeasured; other hosts retain their own qualification.

## Committed transition logs

The consumer uses IC Logging 0.24.5 for a heap-only history of the most recent
128 events. Each retained message is at most 192 UTF-8 bytes and labels are at
most 32 bytes. The combined retained source/topic/message budget is 32 KiB,
derived as `128 * (192 + 2 * 32)` so every individually admitted row retains
the existing count-based capacity. This is a text-payload bound, excluding row
metadata, spare capacity and transient buffers; it is not a complete heap cap.
Count eviction bounds history; there is no age-retention timer.
Queue reconstruction starts an empty history and zero omitted count, without
backfilling durable jobs. `reset_job_metrics` does not clear history or reuse its
sequence IDs. A sequence belongs only to this heap instance; never carry a cursor
across reconstruction or use canister version alone as a durable log generation.

After a successful stable commit, dispatch records its job, occurrence, attempt
and retry classification. Local and remote outcomes record their admitted
classification, distinguishing explicit reconciliation from ordinary completion.
An uncertain result stays uncertain. Messages contain neither delivery payloads
nor caller/destination principals. Idle work, stale/mismatched completions and
failed stable writes add no events. A diagnostic append refusal increments a
wide, overflow-aware omitted counter without changing committed work or retry
authority. Every append refusal, including the typed total-text refusal, follows
that same omitted-counter path. Successful append receipts retain their sequence
and truncation in the engine's rows; their normal eviction count is not an append
refusal and is not added to omitted observations. Paging explicitly selects
`TopicMatch::Any`, preserving the endpoint's existing all-topic behavior without
adding a topic selector. The host converts its
explicit transition timestamp from nanoseconds to whole seconds.

The adapter does not persist Logging checkpoints. The 0.18 checkpoint contract
therefore changes no retained queue format or durable log bytes here; the normal
heap-history reset on reconstruction remains. Consumer qualification for
[#79 — Adopt ic-logging 0.18 in the job-consumer logging adapter](https://github.com/dragginzgame/ic-jobs/issues/79)
covers the byte budget, retained truncation, omission accounting and existing
job/receipt separation; its source/graph is recorded in the handoff.

Manager-only `job_logs` takes `JobLogRequest { before, min_level, limit }` through
the existing management Wire budgets. Limits from one through eight are admitted;
every query scans at most 32 rows, newest first. Follow `next_before` even when a
filtered page has no matches, keeping the same filter. `scanned` is work examined,
not a matching total. Queries copy bounded DTOs and do not mutate history.
Canister version accompanies the response as host context. This history is
diagnostic evidence only; durable jobs and exact receipts own effect/retry state.
Heap observations share IC message rollback; installed trap/upgrade behaviour
requires the explicitly selected live qualification.

The local queue demo exposes the same protected reader:

```bash
logs
logs --before 8
```

Use the cursor returned by the preceding page. The demo reports eight rows per
page and leaves severity filtering available through the typed Wire request.
