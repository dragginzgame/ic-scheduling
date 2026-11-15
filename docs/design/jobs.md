# UTC Jobs scheduler

The [release-scope decision](release-scope.md) now requires legacy scheduling,
retry and unresolved-record capabilities. Latest-due coalescing is deferred.
Fixed-rate intervals are available through `Schedule::fixed_rate`; they share
the existing off-by-default feature boundary and execution policies. Calendar
calculation remains exclusively Saffron's responsibility.

Status: implemented UTC scheduler, 2026-11-15. The first rewrite was published
as 0.37.9; this tree targets 0.38.0 with the unified schema-7 checkpoint API. The maintainer explicitly selected
Saffron and UTC, superseding the earlier named-zone and seconds-dialect scope.
The opt-in wasm32 executor binds one retained Once per job. It uses
explicit SkipLate or bounded CatchUp policy; host-owned checkpoints support
settled/queued reconstruction, including remaining catch-up work. Coalescing
is deferred. No custom calendar
engine is retained. See [progress and execution evidence](../evidence/jobs-progress.md).

## Current implementation

Saffron is pinned to 0.1.0 with its std feature. Chrono 0.4.45 is selected in the
locks with default features disabled and alloc enabled. No clock, system-zone,
IANA database, JavaScript time integration, Jiff or Croner dependency remains.
Saffron owns parsing, Gregorian arithmetic and next-occurrence calculation.

All calendar code and dependencies are behind off-by-default `calendar`.
The separate off-by-default `execution` feature enables `calendar` and adds
target-gated Timers/Platform Runtime dependencies on wasm32. Default builds have
no normal dependency edges. Execution is IC-specific; native builds
retain the pure calendar and occurrence-state tests.
Timers has no dependency on Jobs. Cargo features are additive: a consumer that
enables `ic-jobs/calendar` includes it in that build.

`Schedule::parse(expression)` accepts at most 256 bytes and exactly five fields:
minute, hour, day of month, month, weekday. Every rule is UTC and matches minute
boundaries. Saffron owns grammar and extensions, including last-day and nth-weekday
rules. No zone parameter, local-label resolver, seconds field, year field, macro
or command tail is exposed. Use named weekdays to avoid dialect ambiguity.

`next_after(after_ns, through_ns)` returns an optional UTC nanosecond timestamp
within an exclusive lower and inclusive upper bound. The domain is 0 through
u64::MAX Unix nanoseconds; conversion does not narrow to signed-i64 nanoseconds.
A valid impossible rule returns None, as does a window without an occurrence.
Parsing errors remain distinct. Searches read no clock and perform no effects.
The horizon filters results, not engine work; installed calendar measurements cover representative rules, not a universal
search-cost bound. Do not implement a second parser, calendar algorithm or
resumable search engine. See [Saffron API](https://docs.rs/saffron/0.1.0/saffron/)
and [local evidence](../evidence/calendar-saffron.md).

## Scope and ownership

Build `ic-jobs` above `ic-timers`. Jobs determines which calendar occurrence is
due; Timers owns isolated execution, deadlines, transport and abandonment custody.
Jobs must not install another global-timer provider. Canic and Entropy continue
to use Timers directly. Neither must adopt a generic job model.

A job binds a callback once and associates it with a validated schedule and
execution policy. Support async callbacks. Callback context identifies the
scheduled occurrence and actual start time so consumers can distinguish delayed
execution and construct business idempotency keys. Schedule updates affect future
eligibility, not the identity of already accepted work.

Jobs owns schedule revisions, occurrence progress, pause/resume and missed-run
selection. The host owns stable persistence and lifecycle reconstruction. The
consumer owns external effects, business retries and reconciliation. A calendar
occurrence becoming due is never permission to replay an uncertain effect.

## Scheduling contract

Preview and future execution must use the same supplier calculation. Current
recurrence is five-field UTC cron with Saffron's lists, ranges, steps, names and
calendar extensions. Last-day and nth-weekday rules require no local algorithm.
Day-field matching follows Saffron's OR behavior when both are restricted.
A monthly 31st skips short months; February 29 skips non-leap years. Supplier
syntax is the contract, not compatibility with every cron implementation.

Anchored elapsed intervals now use `Schedule::fixed_rate(first_at_ns, every_ns)`.
The positive interval is a `NonZeroU64`; checked integer arithmetic preserves
the original anchor without calendar interpretation. Binding selects the original
first deadline, even when zero or overdue, then applies the explicit policy.
Cron binding selects after current time. `Schedule::once(at_ns)` admits one UTC
instant, including zero and overdue instants, under the same delivery policy.
It becomes idle after settlement or policy-based skipping; uncertainty still
requires explicit recovery.

`Schedule::after_completion(first_at_ns, every_ns)` selects the first deadline
the same way, then waits a positive interval after each normal settlement.
NoWork, ExpectedFailure and lateness skipping all settle. RetryableFailure settles
only when its retry budget is exhausted or its next deadline would overflow. CatchUp applies its age limit to the single pending
occurrence; there is no completion-relative backlog. Preview methods show only
the first deadline because later ones depend on actual settlement time.

The settled cursor includes the completion horizon for this schedule. Pause and
restoration retain it, so resuming does not restart the delay. Normal settlement
of an old revision cannot advance a newer definition's cursor. An edit uses the
new first deadline if it is after the edit horizon, otherwise horizon plus delay.
Retiring recovery records the reconciliation horizon and pauses; later resume
selects from that horizon without replaying the uncertain occurrence. Retry-safe
recovery instead retains the occurrence until its final settlement. Overflow
leaves the job idle. Completion-relative scheduling needs no additional checkpoint field: restore checks the exact
selected deadline against this cursor and rejects accumulated batch records.
They must not introduce a second cron engine. `*/N` steps within a field,
not an arbitrary elapsed interval. Typed calendar builders are not required.

Named time zones, local timestamps, DST policies, time-zone database migration,
seconds-enabled cron and holiday calendars are outside scope. No compatibility
facade or UTC-offset workaround will emulate the removed API. Clock observations
that move backward must not replay settled work; forward jumps use missed-run
rules. UTC removes local clock changes, not the need for execution progress.

## Late execution, overlap and failure

`Schedule::catch_up(after, now, CatchUpLimit)` previews oldest-first bounded
catch-up. Limits require a count from 1 through 64 and an explicit maximum age.
The age boundary is inclusive and the settled cursor exclusive. The plan exposes
selected timestamps, whether the count limit omitted further age-eligible work,
and a retirement endpoint that never rewinds the cursor. Older and excess work
would be skipped only after selected work settles or is explicitly reconciled.
Calculation performs at most count + 1 Saffron searches and stores at most count
timestamps; it does not enumerate the age-excluded interval. This bounds calls
to the supplier, not each supplier search's instruction cost.

`bind` and `restore` now take an explicit `Policy`. `SkipLate { lateness_ns }`
invokes a selected occurrence within tolerance, otherwise records NoWork and
skips it. It then searches strictly after current time and settled progress.

`CatchUp(limit)` freezes an oldest-first batch from the same preview calculation.
An already selected occurrence outside the age limit is skipped before planning
eligible remaining work. At most the selected count invokes consumer work in a
batch, one occurrence at a time through Once. The age limit is evaluated when
the batch is selected; selected work can age further during pressure or pause.
Pause cancels delivery eligibility but retains the remaining batch. Resume and
host restore continue that exact selection without resetting the count or age
window. Completion retires the fixed window and skips through the then-current
time, so excess and occurrences arriving during the batch cannot replenish it.
The next occurrence is strictly in the future. An edit discards the old pending
batch while preserving already accepted occurrence identity. Retiring recovery
retires the uncertain occurrence, discards remaining batch work, skips through
current time and leaves the job paused; resume selects future work. Retry-safe
recovery retains any committed batch and its remaining quota.

Batch selection never replaces the committed occurrence immediately before a
consumer factory call. An initial-message trap therefore retains the same
uncertain identity even when new batch state rolls back. The preview is neither
a recovery token nor authority to replay business effects.

A schedule specifies intended eligibility, not a guarantee of execution at an
exact wall-clock instant. IC message delivery and resource pressure can delay it.
Report scheduled time and actual start separately.

Coalescing to the latest due occurrence is deferred from this release;
there is no implicit policy default. Catch-up requires explicit limits on
age and count and a documented disposition for excess occurrences. Neither a
long outage nor a minute-level schedule may allocate an unbounded backlog or
require enumerating every missed instant in one message.

The pinned supplier has forward search and membership APIs, with no reverse or
latest-before operation. Coalescing therefore needs a separately reviewed bounded
approach or supplier support; no custom calendar search is authorized by its
presence in this scope. See the [supplier API observation](../evidence/calendar-saffron.md#later-supplier-api-review--2026-11-15).

The initial execution contract is one accepted future per job. Other jobs may
run independently within measured global limits. While a job runs, additional
occurrences are handled by its missed-run policy after settlement; they do not
create concurrent futures. Per-job parallel execution is a separate future
requirement with a separate custody and resource model.

Pause invalidates queued unaccepted work and preserves progress. Resume applies
an explicit missed-run policy. Changing a schedule increments its revision and
invalidates old unaccepted eligibility. Running work retains its occurrence
identity; its completion cannot overwrite a newer schedule or pause decision.

`Job::cancel` terminally cancels pending, paused or idle work. Running and uncertain
work returns Unsettled and must be settled or explicitly reconciled first.
Cancellation invalidates timer eligibility, clears unaccepted selection and
discards any remaining catch-up batch and pending retry. Repeated cancellation succeeds; resume,
pause and edit return Cancelled. Checkpoint restoration preserves the terminal
state. This does not remove the retained timer registration or release its captures.

Jobs 0.37.9 requires an explicit RetryPolicy when binding or restoring. NONE
disables retries. Only Outcome::RetryableFailure permits repeating the occurrence's
effects after a normal return; ExpectedFailure remains final settlement. The
positive total-attempt budget includes the initial attempt. Delays double from
the positive initial delay, clamp to the configured cap and use checked addition
from failure completion. Exhaustion or deadline overflow settles the occurrence.
Calculation takes constant time even for maximum counters.

Retries preserve the host job key, revision and scheduled time as their logical
effect identity. Context and Armed/Running/RecoveryRequired status expose the
one-based attempt; Armed also exposes ready_at separately from scheduled_at. A
retry consumes a fresh local delivery token, retains its catch-up quota and
bypasses the initial occurrence's lateness test. After-completion recurrence
starts its next interval only after final settlement. Pause retains the consumed
budget and retry deadline; resume does not reset either. Schedule edits discard
pending retries but preserve any accepted old attempt's identity until settlement.
An old revision cannot request retries or advance the new definition's cursor.

Checkpoint schema 7 persists pending retry-safe work through RetryProgress:
scheduled time, consumed-attempt count, last failure/reconciliation time and retry deadline.
Restore checks the schedule, remaining budget and exact policy-derived deadline
before admitting a timer. Hosts must atomically retain the matching retry policy.
Explicit unresolved records preserve running/uncertain attempt identity. Factory/post-await
traps block the same attempt for explicit reconciliation; time, pause/resume and
calendar policy cannot bypass Timers' recovery barrier.

Recovery requires an exact live recovery token and an explicit RecoveryDisposition.
Retire discards the retry/batch, retires the occurrence and skips through now.
RetrySafe is the owner's assertion, after reconciling business effects, that
repeating the occurrence is safe. It preserves the logical identity and consumed
budget, retains any committed batch and anchors backoff at reconciliation time.
It refuses a replaced schedule revision, exhausted budget or overflowing deadline
without changing the blocked state. Both choices leave the job paused; resume is
separate. A failed retry request does not prevent later retirement. The latest normal result or reconciliation disposition is retained as a bounded
RecordedOutcome keyed by Attempt; it is observation, not replay authority.
No exactly-once remote effect guarantee is claimed.

## Persistence and composition gate

Ordinary host-owned checkpoints support settled or queued work.
It preserves revision, settled cursor, selected occurrence and pause state, and
validates selected occurrences against the supplied schedule and policies on restore.
The host owns encoding, atomic storage and association with the stable job key,
schedule and delivery/retry policies. Running and recovery-required jobs refuse
ordinary export;
installed tests cover armed/paused reconstruction and refusal after both trap
types. Three-day stopped-canister upgrades exercise overdue selection under
generous and zero lateness tolerances: execute the selected occurrence once or
skip it, then select a future occurrence without a catch-up burst. Explicit unresolved reconstruction now has a separate lifecycle contract below.

Installed qualification also restores settled progress ahead of current time
under both policies. Pause/edit/resume preserves the cursor; no occurrence at or
before it executes, and future work resumes with the current revision. This
qualifies that restored-state condition, not a replica clock rollback or heap
snapshot. See the future-cursor observation in [execution evidence](../evidence/jobs-progress.md).

Checkpoint schema 7 retains the bounded remaining-batch record and represents
the settled cursor as `Option<u64>`. None means nothing settled; Some(0) means
timestamp zero has settled. Decoding validates strictly increasing timestamps
after any settled cursor, the fixed
endpoint and agreement with selected/paused state. Restore checks every remaining
timestamp against the schedule and rejects batches incompatible with the policy/count
before timer admission. The host must retain the exact policy and definition
association. `ControlState` records Active, Paused or Cancelled. Cancelled records
cannot contain selected work, a remaining batch or a pending retry; they restore inactive without
evaluating recurrence. Only schema 7 is accepted in this tree. Earlier schemas, including published
0.37.9 schema 6, require explicit host migration; no automatic migration is provided.

Timer handles and closures are volatile. Persist versioned job definitions and
occurrence progress through an explicit host contract; rebind callbacks using
stable application job keys after upgrade. Do not serialize timer capabilities.
Include schedule revisions, the settled/selected progress cursor, and unresolved
accepted occurrences in the persistence design. A restored
in-flight occurrence requires reconciliation, not automatic replay.

Installed qualification covers the commit boundary between acceptance and
business work, including factory traps, persisted attempt identity and still-live
attempt export at upgrade preparation. Stable state does not make an arbitrary
heap snapshot safe; heap snapshot restoration remains unsupported.

Each job uses one retained Once capability, sharing Timers' 64-declaration limit
with direct timer clients. Installed tests cover revision precedence, occurrence
context, persistence, recovery and 64-job pressure without a second transport.
The table limit is an admission bound, not a throughput guarantee.

Installed supplier-search measurements cover representative rules. Keep parse errors
distinct from no occurrence in a window; neither may become an immediate timer
retry loop. Saffron search returns an optional occurrence, not a search-error type. No bespoke search budgeting algorithm is planned.
Expressions are capped at 256 bytes and catch-up batches at 64 occurrences.
Saffron search has no resumable work budget; a result horizon does not bound its
internal work. Calculation and preview must not borrow timer
state across consumer code.

## Implemented sequence and continuing acceptance gates

1. Freeze the cron dialect, date range and policies with executable examples.
   Implement and test pure next-occurrence calculation and preview first.
2. Qualify Saffron on the target Wasm runtime. Pin dependencies, document updates
   and measure binary/instruction cost.
3. Bind one async job through Once; prove occurrence identity, schedule updates,
   pause/resume, no overlap and failure custody.
4. Add host persistence and qualify upgrades with overdue and unresolved work.
5. Add bounded catch-up and multi-job pressure tests; measure the declared limits.

Required calendar fixtures include leap years and century exceptions, short
months, monthly 31st versus last-day rules, weekday combinations, nth weekdays,
year boundaries, impossible dates, fractional bounds and unsigned timestamp
limits. Assert independently specified UTC results in native and Wasm probes.

Required execution gates include delayed delivery, long downtime, no overlap,
stale tickets after pause/edit, async failure, initial and post-await abandonment,
explicit recovery, upgrade reconstruction and clock discontinuities. Prove bounds
for impossible/sparse schedules and large missed-run windows. Preview and execution must agree for the same policy and input state.

Count Jobs code, dependencies, persistence adapters, consumer glue and its new
qualification artifacts separately and in repository totals. The old timer-only
LOC target is not a budget for the enlarged product. No old Jobs scaffold or test
hierarchy is presumed reusable. Release requires these gates in addition to the
applicable timer gates, not merely a cron parser that returns timestamps.

## Reference semantics

[Saffron 0.1.0](https://docs.rs/saffron/0.1.0/saffron/) defines the selected cron
dialect and UTC next-occurrence behavior. Supplier source was inspected, not
copied. Reference reviewed 2026-11-15.

## Explicit unresolved reconstruction

`Job::unresolved_checkpoint` exports Running or RecoveryRequired work without
stopping it. Hosts must capture it at the upgrade-admission boundary and atomically
store its definition, stable job key, policies and record. An earlier export must
not be reused after execution changes. Ordinary `checkpoint` still refuses
unresolved work, preserving conservative hosts' upgrade guards.

Checkpoint schema 7 carries optional unresolved Attempt data
(revision, scheduled time and one-based number) and the export phase, attached
with `with_unresolved` and read with `unresolved`. Unresolved progress is paused
with no independently queued selection/retry. Ordinary `restore` rejects this
data; `restore_unresolved` requires it. The separate UnresolvedCheckpoint type
and its nested version are removed. Schema 6 is rejected: hosts migrating old
records must decode and validate them explicitly using schema 7 constructors. It preserves
any committed catch-up batch, latest outcome and newer owner revision/cursor.
Old accepted identity can therefore survive an owner edit without validating it
against the replacement schedule. Same-revision records must match their schedule,
budget, cursor and batch before admission.

`restore_unresolved` is only for a fresh runtime after the old local execution has
been discarded. It creates an inactive retained timer and a blocked Jobs attempt
with fresh local recovery authority. Both Running and Uncertain exports restore
RecoveryRequired; repeated upgrades retain that barrier. Resume, pause, cancel,
time advances and owner edits cannot authorize replay. Retire or explicit RetrySafe
reconciliation uses the same live recovery API; changed revisions cannot retry.
Remote effects remain the owner's reconciliation responsibility. This does not
support heap snapshots or live cloning.

Checkpoint schema 7 includes an optional latest RecordedOutcome. Decode it with
its validated constructor and attach it through `with_last_outcome`; preserve it
when encoding either ordinary or unresolved progress. Returned business outcomes
are distinct from Retired and RetryAuthorized owner dispositions. Observation
never changes eligibility. Older outcome history belongs to application storage.
