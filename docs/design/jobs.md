# UTC Jobs scheduler

Status: Saffron-backed UTC preview, 2026-11-15. The maintainer explicitly selected
Saffron and UTC, superseding the earlier named-zone and seconds-dialect scope.
Timer execution remains unimplemented. No custom calendar engine is retained.

## Current implementation

Saffron is pinned to 0.1.0 with its std feature. Chrono 0.4.45 is selected in the
locks with default features disabled and alloc enabled. No clock, system-zone,
IANA database, JavaScript time integration, Jiff or Croner dependency remains.
Saffron owns parsing, Gregorian arithmetic and next-occurrence calculation.

All calendar code and dependencies are behind off-by-default `calendar`.
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
The horizon filters results, not engine work; measure IC instruction cost before
execution integration. Do not implement a second parser, calendar algorithm or
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

One-time UTC instants and anchored elapsed intervals remain future scheduler
forms; they must not introduce a second cron engine. `*/N` steps within a field,
not an arbitrary elapsed interval. Typed calendar builders are not required.

Named time zones, local timestamps, DST policies, time-zone database migration,
seconds-enabled cron and holiday calendars are outside scope. No compatibility
facade or UTC-offset workaround will emulate the removed API. Clock observations
that move backward must not replay settled work; forward jumps use missed-run
rules. UTC removes local clock changes, not the need for execution progress.

## Late execution, overlap and failure

A schedule specifies intended eligibility, not a guarantee of execution at an
exact wall-clock instant. IC message delivery and resource pressure can delay it.
Report scheduled time and actual start separately.

Define a lateness tolerance: an occurrence delivered within it runs normally;
older occurrences follow the configured missed-run policy. Proposed default is
coalescing to the latest due occurrence. Also support skipping missed occurrences
and bounded catch-up in occurrence order. Catch-up requires explicit limits on
age and count and a documented disposition for excess occurrences. Neither a
long outage nor a minute-level schedule may allocate an unbounded backlog or
require enumerating every missed instant in one message.

The initial execution contract is one accepted future per job. Other jobs may
run independently within measured global limits. While a job runs, additional
occurrences are handled by its missed-run policy after settlement; they do not
create concurrent futures. Per-job parallel execution is a separate future
requirement with a separate custody and resource model.

Pause invalidates queued unaccepted work and preserves progress. Resume applies
an explicit missed-run policy. Changing a schedule increments its revision and
invalidates old unaccepted eligibility. Running work retains its occurrence
identity; its completion cannot overwrite a newer schedule or pause decision.

Expected callback failure records an outcome; future calendar occurrences remain
independent. Automatic retry of the same occurrence is not part of the initial
contract. Confirmed abandonment blocks that job in recovery-required state, even
when later occurrences become due. Only owner reconciliation can clear it. A
calendar policy cannot bypass Timers' recovery barrier. No exactly-once remote
effect guarantee is claimed.

## Persistence and composition gate

Timer handles and closures are volatile. Persist versioned job definitions and
occurrence progress through an explicit host contract; rebind callbacks using
stable application job keys after upgrade. Do not serialize timer capabilities.
Include schedule revisions, the settled/selected progress cursor, and unresolved
accepted occurrences in the persistence design. A restored
in-flight occurrence requires reconciliation, not automatic replay.

Before implementation is considered viable, prove the commit boundary between
recording occurrence acceptance and invoking business work, including a factory
trap. Define how installed recovery evidence maps back to the persisted occurrence
and how host upgrade preparation records still-live attempts. Stable state by
itself does not make an arbitrary heap snapshot safe; snapshot restore remains
subject to the timer design's separate qualification gate.

Start by evaluating one retained Once capability per job. Do not finalize this
mapping until a slice proves revision precedence, occurrence context, persistence
and recovery without a second transport or bypassing timer custody. Account for
Jobs in the shared timer declaration limit; the provisional 64-entry timer bound
is not an advertised Jobs capacity. If this model fails, document the concrete
requirement and revise the composition before introducing a dispatcher framework.

Qualify supplier search costs before execution integration. Keep parse errors
distinct from no occurrence in a window; neither may become an immediate timer
retry loop. Saffron search returns an optional occurrence, not a search-error type. No bespoke search budgeting algorithm is planned.
Define limits for jobs, expression size, preview count, search work and catch-up
before installed pressure tests. Calculation and preview must not borrow timer
state across consumer code.

## Implementation sequence and acceptance

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
