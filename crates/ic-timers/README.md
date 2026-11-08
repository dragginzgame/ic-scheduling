![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

<!-- helper-navigation:start -->
<p align="center">
  <a href="https://github.com/dragginzgame/canic"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/canic.svg" width="18" height="18" alt=""> <strong>canic</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/icydb"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/icydb.svg" width="18" height="18" alt=""> <strong>icydb</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-timers"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-timers.svg" width="18" height="18" alt=""> <strong>ic-timers</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-memory"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-memory.svg" width="18" height="18" alt=""> <strong>ic-memory</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-query"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-query.svg" width="18" height="18" alt=""> <strong>ic-query</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-backup"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-backup.svg" width="18" height="18" alt=""> <strong>ic-backup</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-blob-storage"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-blob-storage.svg" width="18" height="18" alt=""> <strong>ic-blob-storage</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-testkit"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-testkit.svg" width="18" height="18" alt=""> <strong>ic-testkit</strong></a>
</p>
<!-- helper-navigation:end -->

> A shared scheduler for background tasks in applications running on the
> Internet Computer.

## What is IC Timers?

The Internet Computer is a network that runs software. Programs on it are
called *canisters*. Like other applications, a canister may need to do some
work later or repeat work in the background.

IC Timers is a tool that developers can add to a canister to organize that
work. You can think of it as a shared alarm clock and task list for the whole
application. It can tell the application when to run a task, keep related
timers in one place, and report what happened when they ran.

For example, an application might use IC Timers to:

- remove expired records;
- process a queue a few items at a time;
- run regular database maintenance;
- try important work again after an interrupted attempt; or
- show operators which background tasks are waiting, running, or stopped.

People using the application do not interact with IC Timers directly. They
benefit from background work that is easier for the application's developers
to organize, monitor, and recover.

IC Timers does not permanently store an application's tasks. The application
keeps the lasting record of what needs to happen and rebuilds its timers after
an upgrade. Its Watchdog mode can preserve another attempt when work fails,
but it does not promise that a task will happen exactly once.

![Application tasks flow through IC Timers to the Internet Computer timer system and one shared status view](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-how-it-helps.svg)

## Technical overview

| Field | Value |
| --- | --- |
| API line | `0.40` |

IC Timers is written in Rust 2024 and supports Rust
1.99.1 and newer. The private backend owns one-shot dispatch through IC
system calls and the shared executor through `ic-platform-runtime`. See the
[backend design](../../docs/timers/design/0.26-owned-timer-backend.md)
for implementation and qualification boundaries.

One IC Timers runtime can manage up to 64 named timers and 128 timer handles.
It supports one-time work, work that repeats after returning normally, and
Watchdog work that prepares another attempt before it starts. It also reports
timer status, outcomes, and bounded resource measurements.

![Application needs matched to IC Timers scheduling and observation capabilities](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-application-needs.svg)

## Why a shared timer runtime?

A framework, a database, and application code can all schedule work independently.
One runtime gives them shared scheduling authority and an inventory.

![Separate component timers compared with one shared IC Timers registry and status view](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-shared-registry.svg)

The shared runtime addresses these canister-wide
questions:

- Which logical timers exist, and who owns each one?
- Is a timer inactive, armed, running, overdue, cancelled, or stale?
- What happened on its last run, and what did that run cost?
- What should be reconstructed after an upgrade?
- Must recurrence wait for normal completion, or survive failed work?

If every consumer builds a registry, recurrence loop, metrics table, and
upgrade protocol, operators still lack one reliable inventory and the most
failure-sensitive logic is duplicated. `ic-timers` puts that coordination
above one private dispatch backend.

## Choose how a task should run

IC Timers offers three modes. `Once` runs a task one time unless explicitly
rescheduled. `AfterCompletion` can repeat after a normal return when its result
requests recurrence. `Watchdog` prepares another attempt before starting important
work. The timeline below shows the difference.

![Timelines showing when Once, AfterCompletion, and Watchdog schedule their work and successor](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-policy-timelines.svg)

`Watchdog` deliberately uses two messages. The small scheduler callback
validates its generation, arms the next cadence successor, queues immediate
work, and returns. Only the later work callback invokes consumer code.

> **Watchdog limit:** Watchdog protects against failure in the task itself. It
> cannot promise recovery if its small scheduling step fails or exceeds the
> Internet Computer's work limit.

A Watchdog can drain successful bounded work without waiting a full cadence:

```rust,ignore
let decision = match outcome {
    SuccessfulProgress { more_work: true } => WatchdogDecision::ContinueImmediately,
    RetryableFailure => WatchdogDecision::Continue,
    Quiescent | TerminalFailure => WatchdogDecision::Stop,
};
```

Use `WatchdogReconcileState::ScheduledImmediately` when lifecycle
reconstruction discovers inactive actionable debt, or call
`WatchdogRegistration::ensure_scheduled_immediately()` when a retained claim
discovers new debt later. “Immediately” arms a zero-delay scheduler for a later
replicated message; it never calls consumer work synchronously. The scheduler
still pre-arms the cadence successor before every work attempt, and normal
`ContinueImmediately` replaces that exact successor rather than adding one.

The 64-registration and 128-owned-handle limits bound this runtime's inventory
and live capabilities. The backend holds at most 130 pending task records and
removes cancelled records immediately. Outstanding self-calls are bounded separately at 128;
these limits do not cap future captures, executor memory or total canister memory.
Capture sizes and running async work still require application budgets. See
[the safety limits](SAFETY.md).

Registry capacity limits new declarations. An existing retained claim can still
ensure or reconcile a wake-up when all registration slots are occupied; cancel
and normal Stop keep that declaration available for later work. Keep the claim
instead of registering another owner on each request. This reserves declaration
capacity only: delay validation and provider/task admission can still fail.

Ordinary recurrence is cheaper and is the correct default when work must
finish normally before another invocation is allowed. Use `Watchdog` only
when committing the next wake-up before fallible synchronous work is the
required protocol.

Ordinary callback results follow their policy. `register_once` and
`reconcile_once` require futures returning `OnceRunResult` with a `OnceDecision`;
the after-completion entry points require `AfterCompletionRunResult` with an
`AfterCompletionDecision`. Both permit Stop, immediate continuation, a relative
retry and an absolute deadline. Only `AfterCompletionDecision` includes
`RecurAfterCompletion`, which uses the configured cadence. A Once declaration
can still explicitly reschedule itself. Invariant-failure results force Stop,
and authoritative nested commands keep their existing arbitration precedence.
Configured recurrence can follow success, no work or a returned retryable
failure. A trap or instruction exhaustion prevents ordinary completion from
scheduling it; recovery-critical work needs Watchdog.

When the provider or Platform runtime drops a confirmed ordinary delivery without normal
completion, its guard retires the exact generation. A retained declaration becomes
inactive with `InactiveReason::Abandoned` and an Unacknowledged
observation. Its owner can explicitly rearm it. Transient declarations and pending
unregistrations are removed; other pending schedules are discarded. No completion,
work count or performance sample is fabricated. A future that remains alive and
pending still owns Running state; cancellation does not interrupt it.

This abandonment path is covered by native drop and real-await PocketIC fixtures.
The 0.13.2 hosted release gates passed those subjects on both supported macOS
hosts; [qualification is bound to that source](../../docs/timers/development.md). It does
not recover application effects committed before an await or automatically retry
work. Capture destructors must remain bounded, nontrapping and safe in executor cleanup.

Completion classification records what happened; the supplied decision selects
what to schedule. A retryable failure paired with `Stop` does not automatically
retry, and success paired with `Stop` does not recur. The result constructors
preserve the decision for success, no work and retryable failure, and force
`Stop` for invariant failure.

Result construction does not check relative delays. After authoritative commands
have been applied, the runtime checks any selected delay's nanosecond encoding
and successor deadline. An invalid selected delay stops the timer with a typed
control failure, observable in a retained declaration's snapshot. A winning
exact reconciliation can discard an invalid callback delay before that check.

## Ingress admission

The canister's existing inspection hook should refuse
`ic_timers::is_timer_dispatch_method(&method)` before applying its ordinary
ingress policy. Timers exports no `canister_inspect_message` of its own:

```rust,ignore
#[ic_platform_runtime::cdk::inspect_message]
fn inspect_message() {
    let method = ic_platform_runtime::cdk::api::msg_method_name();
    if ic_timers::is_timer_dispatch_method(&method) {
        return; // Do not accept private timer ingress.
    }
    inspect_application_ingress(); // Existing host policy owns acceptance.
}
```

For Canic, this check belongs before the existing generated payload inspector's
acceptance; do not define a second inspection export. Canic adoption is separate
from this crate's API. Inspection applies only to ingress, so the private endpoint
always authenticates the self-caller too. A rejected foreign inter-canister call
still incurs target execution costs. The
[IC inspection contract](https://docs.internetcomputer.org/references/ic-interface-spec/canister-interface/#ingress-message-inspection)
does not make inspection an authorization or zero-cost guarantee.

## Minimal `Once` example

The examples require Timers 0.41.0 with Runtime 0.20 and Metrics 0.29.
Align the complete canister graph with the
[runtime compatibility requirements](#runtime-compatibility):

```toml
[dependencies]
ic-timers = "0.41.0"
```

Every framework and application crate linked into the same canister must resolve
the same exact `ic-timers` package ID. Compatible version requirements are supported;
exact manifest pins are not required. Check the composed locked graph, since
different resolved versions contain independent timer registries.

With typed registration setup, initialize the runtime from the
canister's existing lifecycle owner, create and schedule a timer in one call,
and retain its non-clone registration capability:

```rust
use ic_timers::{
    TimerStart, OnceRegistration, TimerCompletion, OnceDecision,
    TimerIdentity, OnceRunResult, TimerSchedule, initialize_runtime,
    register_once,
};
use std::{error::Error, time::Duration};

fn declare_cleanup_timer() -> Result<OnceRegistration, Box<dyn Error>> {
    initialize_runtime()?;

    let timer = register_once(
        TimerIdentity::try_new("my-canister", "maintenance", "cleanup")?,
        TimerStart::Retained {
            initial: Some(TimerSchedule::After(Duration::from_secs(30))),
        },
        |_context| async {
            // Perform one bounded unit of application work.
            OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
        },
    )?;

    Ok(timer)
}
```

The returned registration is the sole-owner control capability. Keep it in
volatile owner state so that code can inspect, ensure, reconcile, cancel, or
unregister that exact claim. The capability types are `#[must_use]` and are
intentionally not cloneable. Dropping a registration does not cancel or unregister
its timer. Call `unregister()` for explicit removal; losing a retained claim can
leave its declaration occupying registry capacity until the next Wasm instance.

For fixed declarations, use `reconcile_once`,
`reconcile_after_completion`, or `reconcile_watchdog` during both `init` and
`post_upgrade`. Those helpers always create retained declarations, including
inactive ones. Transient `RemoveWhenStopped` declarations use the direct
registration functions.

Lifecycle helpers fill an empty registration slot only after the
requested initial state succeeds. If that first call returns an error, the
fresh declaration and handles are cleaned up and its captures are released;
the slot remains empty, allowing a retry to install its new callback. A failed
later reconciliation retains the existing registration and its callback.

All three registration functions take one `TimerStart` before the callback.
`Retained { initial: None }` creates an inactive retained declaration;
`Retained { initial: Some(schedule) }` starts one scheduled.
`RemoveWhenStopped { initial: schedule }` requires a schedule and removes the
declaration when it stops. `DeclarationLifetime` remains an observation in
snapshots, rather than a separate constructor input.

A scheduled start returns authority only after its first wakeup is installed.
A returned error retires the failed creation and provider handles before
dropping callback captures. Durations and deadline arithmetic remain checked.
The first deadline is independent of recurrence cadence: set `initial` to
`TimerSchedule::After(Duration::from_nanos(cadence.as_nanos()))` inside the
chosen start configuration to begin at that cadence.

Typed starts preserve the atomic construction contract introduced in 0.18.
The 0.21 line extends creation cleanup to first lifecycle reconciliation;
existing later control retains its failure semantics. See the
[typed setup decision and qualification scope](../../docs/timers/design/0.20-typed-registration.md).

## Registration and reconciliation capabilities

The complete public entry-point set is:

| Policy | Direct creation | Retained owner-slot reconciliation |
| --- | --- | --- |
| Once | `register_once` | `reconcile_once` |
| AfterCompletion | `register_after_completion` | `reconcile_after_completion` |
| Watchdog | `register_watchdog` | `reconcile_watchdog` |

Direct creation takes `TimerStart` and returns one control capability. Retained
reconciliation fills or reuses the consumer's `Option<Registration>` slot.
Include all six functions in consumer registration-custody checks, including
renamed imports, function-value aliases and calls inside macros. In particular,
`register_watchdog` creates authority just as the other direct constructors do.
Method calls that control existing claims and observation-only functions have
their own custody rules. Comments and string literals are not executable calls.

## Runtime ownership

![Ownership and control flow from durable application state through the IC Timers runtime to its private platform boundary](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-runtime-ownership.svg)

The registry is volatile. It stores no stable timer policy, provider handle,
generation, snapshot, epoch, or application recovery authority. After an
upgrade, consumers derive desired timers from their own durable state and
reconstruct them synchronously before downstream post-upgrade work.

Callbacks run without a registry borrow. Nested ensure, reconcile, cancel,
and unregister requests are arbitrated by one canonical pending command and
the exact callback generation.

Since the 0.8 API, a sleeping Watchdog can reconcile its own exact
wake-up with `reconcile_schedule(Some(TimerSchedule::At(deadline_ns)))` and
return `WatchdogDecision::ScheduleAt(next_deadline_ns)` after successful work.
The scheduler still commits the cadence recovery successor before work runs.
Return `Stop` when idle and reconstruct with `WatchdogReconcileState::ScheduledAt`
when durable demand supplies a deadline. See the
[0.8 contract](../../docs/timers/design/0.8-registration-continuity-and-deadlines.md).

## Truthful observability

![Timer state, outcomes, instruction use, memory growth, and registration identity flowing into one read-only operational snapshot](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-observability.svg)

`timer_snapshot` and `timer_inventory` return inert values; snapshots never
become mutation authority. The inventory carries the runtime epoch even when
its ordered timer slice is empty. `timer_inventory` is the 0.6 replacement for
the removed bare-vector `timer_snapshots` function.

Operator-health labels are consumer policy. Derive them once from the snapshot's
runtime state, scheduling mode and outcomes, then share that projection between
status responses and metrics. The library exposes these facts without assigning
an application health label.

| Observation | Meaning |
| --- | --- |
| Identity and policy | Deterministic ownership and the configured execution contract |
| Runtime state | Policy-specific inactive, armed, running, successor, and watchdog-attempt state |
| Counters | Requested, armed, started, completed, outcomes, cancellations, stale work, coalescing, and unacknowledged work |
| Instructions | Completed scheduler/work sample count plus total, latest, and maximum measurements |
| Memory | Latest start/end Wasm and stable page extents plus maximum observed growth |
| Registration identity | Runtime epoch plus registration sequence; changes when counters are replaced, including within one epoch |

Before subtracting cumulative measurements, compare `registration_id()` within
the same canister, compare the same callback role and units, and require advancing
source times and append-only continuity. Cancellation preserves the identifier;
unregister/re-register changes it. Increasing numbers alone cannot detect a reset.
Callback `generation()` does not identify a counter lifetime. Keep the registration
ID alongside copied observability values; `registration_id().epoch()` supplies
the runtime reset boundary. Observability values contain only registration-local
measurements.

Every `TimerCounters` getter returns the shared Metrics `MeasurementCounter`:
`total()` is `u128`, `total_overflowed()` records actual overflow, and
`exact_total()` rejects an overflowed observation. An exact `u128::MAX` is still
exact. `merge()` combines classified recording totals. These are inert copies;
changing a returned counter cannot change the runtime. Memory sample counts
still conservatively treat `u64::MAX` as saturated.

Instruction measurements use the selected Metrics ordered
`MeasurementSummary` values with `u128` totals and `u64`
samples and counts. Totals
above `u64::MAX` remain exact. An exactly reached cap remains exact, while actual
count/total overflow
is recorded by `aggregate().samples_overflowed()` and `aggregate().total_overflowed()`.
Observe count, total and maximum through `summary.aggregate()`; the
ordered latest observation remains `summary.latest()`.
Zero samples means no completed observation; one zero sample is a completed zero
measurement. Latest and maximum samples remain available after overflow. Preserve
the overflow flags when projecting reports; clipped count/total fields alone lose
quality information. Report owners decide mean/rounding, interval admission and
transport validation after establishing units and registration continuity.

Timers **0.40** uses
`ic_timers::{MeasurementCounter, MeasurementSummary}` from Metrics **0.29**.
Align direct Metrics dependencies when exchanging these values with other
libraries. Recorders implement neither Serde nor Wire and expose no means,
deltas or interval queries. Consumers own their report DTOs and codecs.
Timers 0.39 uses Metrics 0.28; Timers 0.38 uses Metrics 0.27.
The [root dependency catalog](../../Cargo.toml) and
lockfile select that package identity; different Metrics lines have distinct Rust
types. Preserve wide totals in reports/codecs, or use checked conversions when
a narrower consumer field is required. The
[ownership boundary](../../docs/timers/architecture.md#capability-ownership) identifies the
measurement and runtime owners.

Each instruction measurement covers the accepted `ic-timers` execution
interval: it begins immediately before callback acceptance and ends after
completion processing and any successor binding. It is not an exclusive
measurement of application code, nor is it the complete IC message. Provider
dispatch before entering the runtime, provider return/reply work, page reads,
and the bounded summary write remain outside the instruction delta. Do not use
the library aggregate alone as proof against the IC message instruction limit.

Trapped or instruction-exhausted work produces no fabricated sample. A
terminal `RemoveWhenStopped` callback can delete its declaration before the
final measurement is retained; no timer then remains to expose that sample.
Consumers that require a durable terminal audit receipt must store it outside
the volatile timer registry rather than treating a snapshot as authority.

Wasm and stable-memory values are 64 KiB page extents: they are runtime
high-water observations, not exact live bytes. Sub-page allocator liveness
remains an owner-derived measurement. Async ordinary samples may also include
canister activity interleaved while the callback future awaits.

Read scheduler and work memory independently, and preserve `Option` when
projecting the values:

```rust
use ic_timers::MemoryPageSummary;

struct CompletedMemoryObservation {
    samples: u64,
    wasm_start_pages: u64,
    wasm_end_pages: u64,
    stable_start_pages: u64,
    stable_end_pages: u64,
    maximum_wasm_growth_pages: u64,
    maximum_stable_growth_pages: u64,
}

fn completed_memory(
    summary: MemoryPageSummary,
) -> Option<CompletedMemoryObservation> {
    let latest = summary.latest()?;
    Some(CompletedMemoryObservation {
        samples: summary.samples(),
        wasm_start_pages: latest.start().wasm_pages(),
        wasm_end_pages: latest.end().wasm_pages(),
        stable_start_pages: latest.start().stable_pages(),
        stable_end_pages: latest.end().stable_pages(),
        maximum_wasm_growth_pages: summary.maximum_wasm_growth_pages()?,
        maximum_stable_growth_pages: summary.maximum_stable_growth_pages()?,
    })
}

// Given a TimerSnapshot named `snapshot`:
let performance = snapshot.observability().performance();
let scheduler = completed_memory(performance.scheduler_memory_pages());
let work = completed_memory(performance.work_memory_pages());
```

Here `None` means no callback of that role completed normally. `Some` with
equal start/end extents and maximum growth of zero is a real completed sample
that observed no page growth.

`has_armed_wakeup()` is a claim-scoped observation of canonical provider-handle
ownership. It is not a delivery guarantee or durable demand. When durable
demand requires a timer, call `ensure_scheduled()` or
`ensure_scheduled_immediately()` unconditionally instead of using the
observation as a check-then-arm guard.

## Lifecycle and shared-registry rules

![Upgrade lifecycle from durable application state through timer reconstruction to resumed background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-upgrade-lifecycle.svg)

1. The canister's existing lifecycle owner calls `initialize_runtime()`.
2. Frameworks and applications reconcile their retained declarations from
   consumer-owned durable authority.
3. Application hooks run only after required reconstruction.
4. The same sequence runs during `init` and `post_upgrade`.

The crate exports no lifecycle hook, lifecycle macro, public Candid endpoint,
or consumer-work executor.

> 🚨 Every consumer linked into one canister must resolve the same
> `ic-timers` Cargo package ID. Two resolved versions create two independent
> library statics and therefore two registries.

The backend defers dispatch for one second when submission cannot proceed or
its 128 outstanding calls are occupied. Transient rejects also retry after one
second if the task is still pending. Top-up does not bypass an existing delay.
Remove other global-timer providers from the complete canister graph; direct
`ic-cdk-timers` coexistence is unsupported. Cargo's timer ownership key rejects
multiple versions that declare that same key. Older releases lack the key, so
dependency-graph and final-Wasm checks must still verify one timer owner.

Canic, IcyDB, and application timers motivated this shared model. The
[historical Canic adoption record](../../docs/timers/adoption/canic.md) and
[IcyDB adoption record](../../docs/timers/adoption/icydb.md) preserve the exact downstream
mapping and evidence without making patch-specific worktree state part of
this README.

The owned backend does not change application responsibility for durable state
or for reconciling uncertain external effects.

## Guarantees and limits

| Guarantee | Boundary |
| --- | --- |
| Deterministic bounded identity | Three non-empty components, each at most 64 UTF-8 bytes |
| Unique canonical ownership | One declaration per identity within one resolved runtime |
| Checked scheduling | Positive cadence and checked deadline arithmetic |
| Harmless stale delivery | Identity, claim, callback generation, and role are validated |
| Fail-closed binding | Failed provider effects cannot leave false scheduled state |
| Safe cancellation | Owned provider handles are cleared; already-running work is not interrupted |
| Watchdog recovery | Pre-armed successor survives consumer-work trap or exhaustion |
| No catch-up storm | Overdue watchdog cadence coalesces from current dispatch time |

Read [SAFETY.md](SAFETY.md) before relying on the watchdog protocol or
operational measurements.

## Development and evidence

IC Timers is maintained with IC Jobs in the IC Scheduling workspace.
See the [combined development guide](../../docs/timers/development.md),
[root README](../../README.md), and [current handoff](../../docs/status/current.md)
for commands and qualification. The [architecture](../../docs/timers/architecture.md),
[design records](../../docs/timers/design/0.26-owned-timer-backend.md) and
[retained changelog](../../releases/timers/CHANGELOG.md) document the runtime and its history.

## Runtime compatibility

Timers 0.41 selects Runtime 0.20.0 or a compatible 0.20 patch
for IC0 and execution. Runtime requires its exact matching executor guard.
Applications retaining Runtime 0.19 must align all executor and Endpoint
suppliers together before adopting it. Timers 0.40 remains on Runtime 0.19;
removing or overriding the singleton guard is not a compatibility fix.

Consumers exchanging Timers 0.41's public measurement types with a direct
Metrics dependency select Metrics 0.29. Canister entrypoints select Runtime's `cdk`
feature; canonical identities come from Types 0.18. The qualification consumer
selects Storage 0.68, Wire Core 0.17 and Endpoint 0.8. The native harness selects
Testkit 0.93, which owns the PocketIC client at `ic_testkit::pocket_ic`; no separate
client dependency is needed. Managed startup supplies the client's explicit server
endpoint and retains server custody. No retired Testkit Native or Memory crate is used.

These are the current integration selections, not additional dependencies of
the published Timers library. Native and Wasm compilation do not establish
installed IC recovery qualification; consumers retain their own lifecycle and
persisted-state recovery obligations.

For the public Metrics cut, dependency alignment and consumer qualification,
follow the [0.39 → 0.40 adoption checklist](../../docs/timers/adoption/0.40.md).

## License

MIT
