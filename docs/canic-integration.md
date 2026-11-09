# Canic integration preparation

This is an implementation guide for a proposed Canic-owned adapter. No Canic
Jobs adapter or mount API is implemented or qualified by this repository.
Start with one application-owned durable queue and an explicit effect handler.
Jobs' [adapter contract](../crates/ic-jobs/README.md#adapter-integration-contract)
owns the transition/commit ordering; the
[acceptance checklist](test-consumers.md#adapter-acceptance-checklist) maps it to
reference scenarios. Canic owns the eventual integration and installed evidence.

A concrete private queue is implemented and qualified in
[Canic `4759bec`](https://github.com/dragginzgame/canic/commit/4759beca60e0923cf91a16991541ab03c119d1ac).
Its [qualification guide](https://github.com/dragginzgame/canic/blob/4759beca60e0923cf91a16991541ab03c119d1ac/docs/features/jobs-qualification.md)
records the actual application-owned memory, lifecycle, authorization, Wire and
shared-timer composition. Native and installed PocketIC checks cover intent
rollback, lost-response uncertainty, exact receipt recovery and corrupt restore.
That source is open for review in
[Canic PR #722](https://github.com/dragginzgame/canic/pull/722).
It introduces no public Canic Jobs mount API. Existing paid-operation leases
retain their own assessment under [Canic #502](https://github.com/dragginzgame/canic/issues/502).

## Ownership at the composition point

| Concern | Jobs supplies | Canic/application supplies |
| --- | --- | --- |
| Metadata | `JobRecord`, validated `Job`, checked transitions | Unique job IDs, immutable payloads, domain operation bindings and any longer history |
| Persistence | Serde representation; `Job::restore` validation | Bounded Wire codec/framing, storage grants, atomic records/index writes and refusal policy |
| Scheduling | `next_due_ns`, `start_next` for bounded candidates | Global due index, batch/backlog budgets and the existing shared IC Timers runtime |
| Lifecycle | No lifecycle exports | Synchronous restore and wakeup reconstruction under Canic's single lifecycle owner |
| Effects | Exact `Attempt` correlation and explicit result transitions | Authorized handler, transport, receipt authentication and retry-safety evidence |
| Management | Pause/resume/reschedule/cancel transitions | Caller admission, endpoints, atomic writes and reconciliation of committed demand |
| Observability | No metrics/logging provider | Canic attribution, bounded reporting and committed-transition observations |

The Jobs library depends only on Serde and requires Rust 1.99.1.
Adopting it does not require Canic to select this repository's Storage,
Platform, Timers or Wire versions. Canic's composed graph must retain its own
single runtime identities and qualified installation format. This repository's
Storage 0.64 and 0.68 physical-format cuts apply to its qualification fixtures;
they are not Jobs library installation prerequisites. Canic's own release/reset rules remain its
authority and never follow implicitly from this document.

## Persist and dispatch

Persist one canonical `JobRecord` beside its immutable application payload and
domain operation binding. Decode under Canic-owned byte/work limits and restore
through `Job::restore` before publishing authority. The native
[Wire persistence tests](../qualification/ic-jobs-test-consumer/tests/stored_wire.rs) exercise the
public record/attempt values, checked history and blocked restoration. Their
limits are fixture policy, not Canic's production budget or stable envelope.
Jobs supplies no universal stable byte format; Canic owns that contract.

Select bounded candidates, apply `start_next` tentatively, and atomically commit
the Running record, payload binding and due-index changes. Expose delivery only
after successful commit. On IC, synchronous trap rollback covers the whole
message segment: a preceding stable write alone is not a committed intent.
Reconcile the existing Timers watchdog from committed global demand before an
asynchronous continuation can issue the effect. A partial candidate batch's
deadline does not describe the complete queue.

An effect handler receives the committed attempt and immutable payload. After
every await, reload committed authority, authenticate and match the evidence,
then commit `finish` or `resolve` before reporting the result. Do not write a
pre-await snapshot over newer management or reconciliation state. The packaged
[persisted-job example](../crates/ic-jobs/examples/persisted_job.rs) demonstrates
failed commits, duplicate receipts, stale continuations after a safe retry and
preservation of an independently edited label. Its byte store and counted
destination are substitutes; they do not establish IC rollback or transport.

During lifecycle reconstruction, open Canic-owned storage, restore all retained
records and reconstruct the volatile wakeup synchronously before downstream
hooks. Reject corrupt records without creating replacement state. Keep allocation
bootstrap and timer custody at the existing host composition point.

## Identity and existing domain recovery

`JobId` identifies a retained job history. `ExecutionId { job, occurrence }`
identifies one logical effect across retries; `Attempt` adds the strictly
increasing dispatch sequence. The scheduler's returned iterator index is only
an ephemeral position. Fresh/reinstalled histories need fresh job IDs; restoring
an older backup requires external-effect reconciliation before dispatch.

Canic domain operation IDs and paid-effect journals retain their own authority.
An adapter must persist an immutable binding to those identities and must not
derive a fresh domain operation on every retry. An `Attempt` is correlation
data, not a caller credential or proof that an effect is safe to repeat.

Canic's domain async-job leases and Jobs' uncertainty protocol are different
contracts. Lease expiry, timeout, transport rejection, missing receipts and
watchdog takeover cannot unblock a Running/Uncertain job. For restored Running
intent, retain the exact attempt; if its result remains unknown, transition it
to Uncertain. Resolve only with authenticated result or explicit retry-safety
evidence. Known-safe retries preserve payload and logical execution identity
while advancing the attempt sequence. Duplicate or stale replies must preserve
the committed terminal/successor state.

Funding, pool maintenance, renewal and other existing domain workflows need
individual assessment of their journal/replay/lease semantics before replacement.
This guide authorizes neither a wholesale replacement nor a second recovery
registry. Keep one owner for each scheduling decision and external disposition.

## Acceptance in Canic

Use the [native scenarios](test-consumers.md#adapter-acceptance-checklist) as
requirements for Canic-owned tests, including rejected intent/result commits,
invalid future records, management during an await and exact stale/duplicate
reply handling. Exercise its actual Wire storage envelope, allocation grants,
authorization and endpoint composition. Native codec tests alone do not qualify
that storage integration.

Installed PocketIC cases must cover same-release restoration, message-wide
storage/timer rollback and lost-response receipt recovery without redelivery.
Bind results to Canic's exact source, catalog/lock, tool identities and installed
Wasm digest; follow its own cross-release installation policy. Neither this
repository's historical recovery run nor a successful adapter build establishes
Canic composition. The [current handoff](status/current.md) distinguishes this
repository's recorded graphs and remaining qualification.
