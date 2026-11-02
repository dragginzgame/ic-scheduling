# IC Jobs

Durable job metadata and scheduling policies for Internet Computer applications.
Applications own persistence, effects and IC Timers integration; IC Jobs
validates metadata transitions and selects due work from bounded iterators.

## Use

```rust
use ic_jobs::{Job, JobId, Outcome, RetryPolicy, Schedule};

let mut job = Job::new(
    JobId(1),
    Schedule::Once { at_ns: 100 },
    RetryPolicy::NONE,
);
let attempt = job.start(100)?;
// Persist job.record() and the immutable payload before dispatching the effect.
job.finish(attempt, 101, Outcome::Success)?;
// Persist job.record() again after recording the result.
# Ok::<(), ic_jobs::JobError>(())
```

Times are absolute IC nanoseconds supplied by the application. A due timestamp
is an eligibility threshold, not a punctual-execution guarantee.

The crate supports one-shot deadlines, fixed-rate intervals, recurrence after
completion, missed-run policies, bounded retries, pending cancellation and exact
attempt correlation. Persist the canonical `JobRecord` beside the application
payload and reconstruct it through `Job::restore`.

Validated `Job` values implement `Serialize` transparently through the canonical
`JobRecord` writer, so application snapshots may serialize jobs directly.
`Job` has no `Deserialize` implementation: decode `JobRecord`, then call
`Job::restore` before using the job.

`pause` suspends pending work durably; `resume` restores its original deadline
and missed-run policy. `reschedule` replaces scheduling intent from an unstarted
occurrence forward, retaining job/effect identity and dispatch sequence. Paused
work stays paused, and cancellation accepts it. Rescheduling an attempted
occurrence refuses, including retry waiting. Running and Uncertain effects remain
blocked until explicit disposition. Management does not change payloads or retry
budgets. Commit each record and its due-time index before reconciling a timer.

## 0.16 stored-record hard cut

`JobState::Running` is a unit variant. For a running attempt, the dispatch time
is `JobRecord::updated_ns`; match `JobState::Running` without a timestamp field.
JSON encodes this state as `"Running"`. The previous
`{"Running":{"started_ns":100}}` shape is refused. Transitioning to
`Uncertain { started_ns }` preserves the original dispatch time before updating
the transition timestamp, including across later unresolved reconciliation.
Both states remain blocked after validated restoration.

`JobRecord::last_outcome` is `Option<RecordedOutcome>`: an outcome and its
`at_ns` timestamp are present together. Read them with
`record.last_outcome.map(|result| result.outcome)` and
`record.last_outcome.map(|result| result.at_ns)`. The separate
`last_outcome_ns` field is removed. Management preserves the complete latest
result; it cannot replace the result's timestamp with a later management time.

The JSON field is now `"last_outcome":{"outcome":"Success","at_ns":101}`,
or `"last_outcome":null` before any recorded outcome. Both fields inside a
recorded result are required. Retired serialized shapes are refused, including
records with the old `last_outcome_ns` field. There is one current record shape
and no compatibility reader, converter or automatic reset. Consumers select
their codec and own its retained-byte contract.

The repository maintainer explicitly approved retiring earlier installations
without preserving their records for this cut. Consumers must retire/reset
earlier installations under their own authority and create fresh state before
adopting this format. Updating source or invoking restore never clears storage.
Current-format Running/Uncertain records still restore as blocked; only explicit
result or retry-safety evidence permits disposition through the Jobs API.

## Checked configuration

Recurring `Schedule` intervals use `std::num::NonZeroU64`; absolute deadlines
still use `u64` and may be zero. Build retry budgets with
`RetryPolicy::new(max_attempts, initial_delay_ns, max_delay_ns)?`, or use
`RetryPolicy::NONE`. Read the immutable values with `max_attempts()`,
`initial_delay_ns()` and `max_delay_ns()`.

```rust
use std::num::NonZeroU64;
use ic_jobs::{Job, JobId, RetryPolicy, Schedule};

let interval = NonZeroU64::new(10).unwrap();
let retry = RetryPolicy::new(3, 5, 20)?;
let job = Job::new(JobId(1), Schedule::AfterCompletion {
    first_at_ns: 0,
    every_ns: interval,
}, retry);
assert_eq!(job.next_due_ns(), Some(0));
# Ok::<(), ic_jobs::JobError>(())
```

Raw inputs must reject zero before constructing a schedule. Invalid retry
configuration returns `JobError::InvalidRetryPolicy`; invalid serialized
configuration fails decoding.
Every decoded `JobRecord` still passes through `Job::restore` to check its history.
Checked configuration does not grant admission to a retired record format.

## Scheduling a batch

`next_due_ns` reads a finite iterator of job references, and `start_next`
starts one earliest due attempt from an iterator of mutable job references.
Slices and vectors can still be supplied directly. They leave storage,
payload lookup, effect execution and completion persistence with the application.

```rust
use ic_jobs::{Job, JobId, RetryPolicy, Schedule, next_due_ns, start_next};

// Application-owned payloads stay beside the validated jobs.
let mut entries = [
    (Job::new(JobId(1), Schedule::Once { at_ns: 200 }, RetryPolicy::NONE), "later"),
    (Job::new(JobId(2), Schedule::Once { at_ns: 100 }, RetryPolicy::NONE), "first"),
];
assert_eq!(next_due_ns(entries.iter().map(|(job, _)| job)), Some(100));
if let Some((index, attempt)) = start_next(entries.iter_mut().map(|(job, _)| job), 100)? {
    assert_eq!(index, 1); // Position in this iterator, not a durable identity.
    assert_eq!(attempt.execution.job, JobId(2));
    let running_record = entries[index].0.record();
    // Commit running_record with entries[index].1 before dispatching the effect.
    // Record its outcome with the owning Job's finish method and persist again.
    # let _ = running_record;
}
# Ok::<(), ic_jobs::JobError>(())
```

Equal deadlines follow the supplied iterator order. Returned indexes refer to
that order, including when an iterator filters or reorders the source collection. A selected job's transition error
leaves every record unchanged; it does not select another job. Use a bounded
candidate batch from the application's due-time index. The reported deadline
covers only that batch; the storage owner supplies any wider queue's earliest
deadline to IC Timers.

A batch limit bounds one callback, not the entire catch-up backlog. For example,
86,400 pending one-second occurrences need at least 43,200 batches with a
two-attempt limit. Earliest-deadline selection can delay newer jobs until that
historical work drains. Prefer `Skip` for maintenance that does not need every
occurrence; use `CatchUp` only with an application-owned backlog/cycle budget.
This is a workload calculation, not a measured throughput or delivery guarantee.

A restored Running job remains Running. If its reply cannot be reconciled directly,
record Uncertain with the active attempt and use `resolve` only after establishing
the actual result or retry safety. A lost reply does not authorize another effect.
Uncertain effects block dispatch until
the application establishes the result or explicitly determines that repetition
is safe. `ExecutionId` identifies the logical effect across retries; `Attempt`
identifies one dispatch. Never reuse a job ID or change an occurrence's payload
under the same effect identity.

## Timer integration

Depend on IC Timers directly when the application needs wakeups. Applications own
the volatile watchdog, lifecycle initialization, bounded callbacks and any
consumer-owned asynchronous delivery continuation. This crate supplies no stable
storage, global queue, handler registry or exactly-once effects.

After committing records and due-time index changes, pass the earliest pending
deadline to `ic_timers::reconcile_watchdog`, mapping `None` to
`WatchdogReconcileState::Inactive` and a deadline to `ScheduledAt`. A bounded
callback returns `WatchdogRunResult::new` with its actual completion classification
and `WatchdogDecision::Stop` or `ScheduleAt` for the new deadline. Reconcile after job
insertion or management; after upgrades, restore all records and reconstruct the
watchdog synchronously before downstream hooks. Unresolved effects need a separate
application reconciliation workflow. Watchdog callbacks cannot await: external
delivery requires a persisted intent and consumer-owned continuation/outbox.

Timer completion classifies the dispatch callback, separately from each job's
effect outcome. A bounded batch can succeed while recording a known job failure
or preparing an external attempt whose result is still unknown. Report no work
for an empty batch. A watchdog recovery successor restores a callback opportunity;
it cannot make a permanently failing handler progress. Dispose of known no-effect
handler failures through the job's exact attempt, retaining fail-closed handling
for corrupt storage and unresolved external effects.

The application owns its Timers selection; this package selects neither Timers
nor Metrics. Align all timer owners in a canister to the same package identity.
For exchanged measurement summaries, use `ic_timers::MeasurementSummary` to share
Timers' selected Metrics identity rather than copying a separate version
requirement. The application's storage backend separately owns physical-format
compatibility and retained-installation disposition.

Timers publishes a new watchdog registration only after its requested
initial state succeeds. A failed first reconciliation leaves the consumer slot
empty and releases the new callback; retry can install the callback supplied
by that retry. Later failures retain existing authority.

## Adapter integration contract

A framework adapter composes the public Jobs API with its host's persistence,
authorization, lifecycle and effect execution. The packaged
`examples/persisted_job.rs` is a runnable reference:

```text
cargo run -p ic-jobs --locked --example persisted_job
```

It retains one job and its immutable payload in a bounded JSON snapshot. Atomic
byte replacement and a counted destination substitute for storage and transport;
the example does not establish IC rollback, timer delivery or a production codec.
Its assertions cover rejected intent commits, failed completion commits, cold
restoration of Running intent, blocked uncertainty, duplicate receipts and exact
receipt resolution without another effect. A safe retry retains its logical
effect/payload and rejects a late older continuation; completion reloads current
authority and preserves management metadata edited during delivery. The example's
4 KiB ceiling is consumer policy. Its JSON byte store is illustrative: adapters
own their codec and retained format. The published library depends only on Serde
and does not require the qualification consumer's Memory/Storage versions.

| Owner | Responsibility |
| --- | --- |
| IC Jobs | Validate JobRecord history, transition attempts and select due work |
| Host/adapter | Storage transactions, identity allocation, authorization, budgets, lifecycle and wakeups |
| Application/destination | Immutable payload meaning, effects, receipt authentication and retry-safety evidence |

An adapter follows this ordering:

1. Obtain trusted caller/time context. Decode within consumer byte/work limits,
   check collection identities and bounds, and restore every candidate through
   `Job::restore`, including future and blocked records. Never initialize or
   replace retained state merely because decoding or restoration fails.
2. Apply `start_next` to a bounded tentative candidate set. Atomically commit
   the Running record, immutable payload and any due-index changes **before**
   exposing a delivery intent. A failed write discards tentative jobs and leaves
   committed bytes, indexes and published in-memory authority unchanged.
3. Reconcile the host's wakeup from committed global demand. A deadline from a
   partial candidate batch is not the whole queue's next deadline. On IC, failure
   to rearm must trap/rollback the same message segment before effects escape.
4. Execute the effect through an explicit application handler. Watchdog callbacks
   remain synchronous and bounded; asynchronous calls use consumer-owned
   continuations. Durable `ExecutionId` identifies the logical effect across
   retries; the iterator index is temporary and `Attempt` identifies one dispatch.
5. After a reply or await, reload current committed authority. Check the exact
   attempt and immutable payload, classify the result, then commit the transition
   before publishing success or reconciling the next wakeup. A failed result
   commit may leave Running intent even though the destination performed an effect.

On reconstruction, open the host's committed storage and validate retained
records before reconstructing the volatile watchdog and invoking downstream hooks.
Memory/Timers integrations share the host's selected runtime identities and keep
allocation bootstrap and lifecycle ownership at the host composition point.

Running and Uncertain records remain blocked. Missing replies, timeouts and
transport rejection alone establish neither success nor permission to repeat.
Use `finish(..., Outcome::Uncertain)` for a restored Running attempt when its
result cannot be established, then `resolve` only with explicit result or
retry-safety evidence. Authenticate receipt sources outside Jobs; an `Attempt`
is correlation data, not an authorization credential. Never reuse a job ID or
change an occurrence's payload under the same effect identity.

No storage trait, global handler registry, lifecycle export or timer provider is
required from this library. Framework-specific endpoint models, management
authorization, metrics attribution and reset policy remain with their consumers.

## 0.14 API cut

The two batch functions accept finite iterators of job references. Existing direct
slice/vector calls remain valid. Wrappers relying on implicit slice coercion
should pass an explicit slice or iterator; adapt code tied to the old function
signatures. Use `entries.iter().map(|entry| &entry.job)` for deadline
reads and the mutable projection for selection. The index belongs to that
iterator's order. This changes neither stored records nor transition semantics.

## 0.13 API cut

`start_next` returns `(index, attempt)`, updating the selected job in place.
Replace three-element tuple destructuring with the pair. Read
`jobs[index].record()` when preparing the durable intent, and commit it with the
immutable payload/index before dispatching any external effect. This signature
change does not change stored records, selection order, transitions or recovery.

## 0.11 API cut

Replace `Scheduler::new(&mut jobs).start_next(now)` with
`start_next(&mut jobs, now)` and use `next_due_ns(&jobs)` for deadline reads.
Remove `features = ["timers"]` from the Jobs dependency. Call IC Timers directly
as described above; the `ic_jobs::timers` module is removed. This cut preserves
the Jobs stored-record layout, job identities and transition behavior.

The package and its qualification consumer require Rust 1.99.1.
See the [repository guide](https://github.com/dragginzgame/ic-jobs) and
[safety contract](https://github.com/dragginzgame/ic-jobs/blob/main/SAFETY.md)
for the complete persistence and recovery obligations.
