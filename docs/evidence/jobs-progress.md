# Jobs occurrence progress — 0.37.2, 2026-11-15

Implemented a private progress core before wiring Jobs into Once. Saffron remains
the only calendar calculator; no timer transport, callback registry, recurrence
algorithm, business retry policy or persistence codec was added. The public
calendar preview API and off-by-default calendar dependencies are unchanged.

An occurrence is a schedule revision plus intended timestamp, to be combined
with the host's stable job key when forming a business identity. A separate
checked selection sequence distinguishes local delivery attempts. Cancelling
unaccepted selection and later selecting the same occurrence produces fresh local
authority without inventing a different business occurrence.

The core retains one selection, which may be queued, accepted or blocked by
uncertainty. Acceptance returns scheduled and actual start context. It must occur
before consumer factory invocation in the future adapter. It cannot accept early,
accept twice or select overlapping work. Normal completion retires the occurrence
and advances the exclusive cursor, independent of the consumer's business outcome.

Pause invalidates queued selection while preserving accepted/blocked identity.
Schedule edits increment the revision, retain the paused state and do not move
the existing settled cursor backward. Completion from an older running revision
releases its selection without replacing newer schedule progress or pause state.

The future adapter may block either accepted state or queued state surviving an
initial work-message rollback, but only from matching Timers recovery evidence.
Pause, resume and schedule edit cannot clear that barrier. Explicit owner
reconciliation retires the uncertain occurrence without replaying it; the adapter
must also reconcile Timers' exact current recovery token. This core does not grant
permission to retry an uncertain business effect or infer termination from time.
Exhaustion faults before revision/selection reuse and retains unresolved identity.

Six native tests establish the above transitions, including stale delivery after
reselecting the same occurrence, old-revision completion, both queued/accepted
uncertainty and backward progress admission. With the four existing calendar
tests, all ten Jobs tests pass. Commands:

```sh
cargo test -p ic-jobs --all-features --locked --offline
cargo clippy -p ic-jobs --all-targets --all-features --locked --offline -- -D warnings
cargo clippy -p ic-jobs --all-targets --no-default-features --locked --offline -- -D warnings
cargo clippy -p ic-jobs --target wasm32-unknown-unknown --all-features --locked --offline -- -D warnings
make fmt-check
```

Rust 1.100.0 and the existing product lock were used. No dependencies were added.
The module is private with a scoped non-test dead-code expectation until the
adapter uses it. No serialized representation, restoration protocol or public
recovery capability has been selected.

Required next proofs are selection-to-Once composition, initial factory trap
mapping, schedule/pause precedence across an actual await, supplier search bounds,
missed-run selection and host persistence/reconstruction. Local tokens still
require job-scoped authentication from the future binding layer. Native state
tests do not establish IC commit ordering or durable effect safety.

The maintainer deferred further runtime-cost investigation and comparison with
the legacy provider; that work remains open rather than blocking this Jobs state
increment. This is not release-ready execution or a consumer migration.

## Installed skip-late execution preview — 2026-11-15

Jobs now has an off-by-default `execution` feature, enabling calendar support and
wasm32-only dependencies on the current local Timers crate and Platform Runtime.
The default normal dependency graph remains empty; the standalone calendar Wasm
probe still has no IC imports and all eight calendar checks pass. No second timer
provider, transport queue, calendar implementation or compatibility facade was added.
The release-group map records Jobs' optional dependency on Timers.

The public wasm32 preview binds one retained Once per job. The caller supplies a
parsed schedule and explicit lateness tolerance. Selection stores the calendar
revision/time before timer arming. The callback receives that identity and actual
start time; acceptance occurs before invoking its future factory, outside state
borrows. Business outcomes do not cause automatic retry. Late delivery skips
consumer invocation. After completion or skip, one Saffron search selects strictly
after current time and settled progress. There is no missed-occurrence enumeration.

Owner edits during an accepted attempt change Jobs state without placing a
competing successor override in Once. The wrapper derives the successor from the
latest schedule and pause state after the accepted callback settles. This preserves
pause/resume/edit ordering, including a pause followed by resume during the await.
Queued changes invalidate/reselect eligibility through the existing Once capability.

Timer recovery is associated with the retained selected occurrence, including
queued Jobs state after an initial factory trap rolls acceptance back. Recovery
status exposes revision/time and a nonserializable token scoped to both this job
and the exact timer recovery authority. Ordinary pause/resume does not clear it.
Explicit reconciliation retires the uncertain occurrence and leaves the job paused;
resume chooses a new future occurrence. Reusing a consumed recovery is rejected.
Terminal progress faults prevent subsequent consumer admission.

Six installed cases pass against the actual adapter:

- Normal async work retains selected time and reports actual start separately.
- Initial factory trap blocks the original selected occurrence without a committed start.
- Post-await trap blocks that occurrence with one committed start and no finish.
- Both traps remain blocked through pause/resume and another minute; explicit
  recovery permits a future occurrence, and stale recovery is refused.
- Pause/resume plus schedule edit during the await keeps the running revision and
  selects its successor from the new two-minute schedule.
- Pause during the await leaves the job paused after completion.
- Zero tolerance skips a deliberately late occurrence without invoking business work.

The first two trap cases share recovery assertions; the fixture has six modes.
These are local installed results, not a real consumer migration or durable
exactly-once guarantee.

### Supplier search observations

The installed probe also checks fixed Saffron outcomes over the full supported
upper timestamp bound, with instruction brackets around parse and search:

| Rule/case | Parse instructions | Search instructions | Outcome |
| --- | ---: | ---: | --- |
| Every minute from epoch | 242,751 | 1,413 | 60 seconds |
| Feb 29 after 2099-03-01 | 5,843 | 3,709 | 2104-02-29 09:00 UTC |
| Monthly 31st after 2026-04-01 | 4,174 | 1,939 | 2026-05-31 09:00 UTC |
| Feb 31 from epoch | 5,843 | 715 | None |
| Every minute after final representable minute | 2,751 | 1,398 | None |

The first parse includes cold-path costs; these are raw brackets, not equivalent
steady-state comparisons. This verifies selected sparse/impossible cases without
adding a custom search engine. It is not a comprehensive worst-case search budget
for every admitted 256-byte expression.

### Qualification boundary

All ten native Jobs tests pass, along with native all-feature/default-free and
Wasm Clippy, probe/driver Clippy, Wasm Rustdoc and formatting. The full installed
Once/watchdog suite passes with Jobs enabled. The public API remains a preview:
host persistence/reconstruction, durable revision identity, catch-up, coalescing,
full expression search budgets and multi-job failure pressure remain release gates.
Tokens and closures are volatile; this API does not support restoring unresolved
work across upgrade or snapshot restoration. No publication is enabled.

Reproduce with `make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic`.
Observed with Rust 1.100.0, Runtime 0.22.5, Saffron 0.1.0, Chrono 0.4.45,
Testkit 0.106.0 and PocketIC client/server 16.1.0/16.0.0. The existing server
identity is recorded in executor-probe.md.
Combined installed probe Wasm SHA-256:
169fc9c66a742fff5c9d824e7adae2c25788eaf8300b6caf3a73b285a4b976fd.
Product lock SHA-256:
ec9ee2b1cb05c18e7d99fc331b171ed032e4a90005a1cf825a4560f1846c6698.
Qualification lock SHA-256:
a513228f62a5062ca2dbe22236bbf1eb9e2c4d0c729e782b876892976531b7b0.

## 0.37.3 host checkpoint reconstruction — 2026-11-15

The opt-in execution API now exposes schema-1 `Checkpoint` data and `restore`.
Progress includes schedule revision, exclusive settled cursor, selected but
unaccepted occurrence and pause state. Decoding rejects unsupported versions,
revision zero, selected work while paused and selection at/before the cursor.
Restore checks a selected timestamp against the supplied Saffron schedule before
admitting a timer declaration. Hosts own encoding, atomic storage and association
with the stable job key, schedule and lateness policy; the library cannot verify
that association from progress alone.

Export consults the actual timer state and rejects running or recovery-required
work, including an initial factory trap whose Jobs acceptance rolled back.
Reconstruction creates fresh local capabilities and selection authority. No
timer or recovery tokens are serialized. This does not support unresolved-work
restoration, heap snapshots or durable exactly-once effects.

Local installed checks pass for:

- Armed reconstruction preserving the selected occurrence and edited revision,
  followed by one successful execution with the original occurrence context.
- Paused reconstruction remaining paused across another minute, followed by
  explicit resume and execution under the preserved revision.
- Checkpoint refusal while the async callback is running.
- Actual upgrade refusal after initial and post-await traps, preserving blocked
  progress and the existing explicit recovery path.
- A selected timestamp inconsistent with the schedule rejected before binding;
  the same declaration identity can then be admitted by Timers.

All 12 native Jobs and 20 Timers tests pass. Jobs native all-feature/default-free
and Wasm Clippy, probe/driver Clippy, Wasm Rustdoc, formatting and the full installed
Once/watchdog/Jobs suite pass. Long-downtime reconstruction, unresolved-work
persistence, additional missed-run policies and multi-job pressure remain open.
No crates.io publication or deployed-consumer qualification is claimed.

Reproduce with `make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic` using
the tool and supplier versions recorded above. The tested combined probe, before
subsequent documentation-only edits, has SHA-256:
d6b3ba80233bfe4a86d64cabfb7ab3cf3ae66cc5fdda67c4695b4cf5553d4107.
Product lock SHA-256:
4a87b4bb7b5fb8423a6aa5f5b736464e6c9c6dc08acf32477dd4b1d9034a1422.
Qualification lock SHA-256:
e09d42bf2d9e432811ee65bb7e09fe448e7e92efa52cf3bd287dd0cc46763d25.

### Downtime continuation — 2026-11-15

The host fixture now stores its lateness-policy choice alongside the checkpoint
in its own versioned stable record. This is fixture-owned encoding, not a product
serialization format or a migration claim for an older fixture record.

Two installed scenarios stop an armed canister, advance simulated time by three
days, upgrade while stopped and restart it. With maximum lateness tolerance,
exactly one callback executes with the original selected revision/time and a
post-downtime actual start. With zero tolerance, no consumer callback executes.
Both select a deadline strictly after simulator time and within the next minute;
additional rounds produce no catch-up burst. These are checks of the initial
skip-late policy, not implementation of bounded catch-up or coalescing.

The full installed suite, probe/driver Clippy, Wasm Jobs Rustdoc and formatting
pass. Product scheduling logic did not need a change; its revision-context API
comment now reflects preservation through a matching checkpoint. Unresolved-work
restoration and broader lifecycle/clock-discontinuity qualification remain open.

Tools, supplier versions and lock hashes are unchanged from the checkpoint
section above. Combined installed probe Wasm SHA-256:
d60cca417a5ac144ec03f04232bd3108ce19996a02a46030e6f1df89383ac79c.

### Multi-job pressure continuation — 2026-11-15

A dedicated qualification fixture admits twelve every-minute Jobs plus 52
inactive Once declarations into the same table. A further Jobs bind returns
`Timer(Capacity)`, establishing shared admission rather than an independent Jobs
allowance. These private limits remain provisional, not advertised capacity.

Eight job callbacks issue real calls held by Testkit's held-reply fixture. All
eight remain Running with exactly one start and no completion; the other four
jobs remain unaccepted. The host pauses one of those queued jobs. Additional
rounds leave all held starts unchanged and the paused job unstarted. Requests
are released in reverse order, independent of dispatch order.

One released callback traps after its await and enters recovery-required state.
The other seven complete, and the three eligible queued jobs then complete with
their original scheduled timestamp. The paused job stays unstarted. After its
explicit resume and another minute, all eleven healthy jobs execute their next
eligible occurrence, while the uncertain job retains its original context and
one-start/zero-finish counts. No explicit time advance occurs while held HTTP
barriers remain open.

The full installed suite, probe/driver Clippy, formatting and diff checks pass.
No product scheduling change was needed. This establishes this bounded pressure
scenario and failure isolation; it does not measure maximum throughput, retained
memory or latency bounds, or cover every mixed Jobs/Once/watchdog workload.

Tools, suppliers and locks are unchanged. Reproduce with the existing
`make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic` command.
Combined installed probe Wasm SHA-256:
270f07adfc6a15802ef951c1a5161f5b04ba1faf055258e4666f129a8f7b5085.

### Idle, outcome and exhaustion continuation — 2026-11-15

An installed five-job fixture adds the following boundary evidence:

- `NoWork` and `ExpectedFailure` each settle one occurrence without immediate
  retry. Both execute later calendar occurrences, with increasing scheduled
  timestamps, over three successive minute advances.
- A valid impossible February-31 schedule stays Idle without invoking work.
  Editing it to every minute admits revision 2 and executes once. Editing back
  to the impossible rule creates revision 3, clears selected work and stays idle
  through another minute. Its checkpoint has no selected occurrence.
- Restoring revision `u64::MAX` can arm work, but editing then returns Faulted.
  Status stays terminal, resume/checkpoint refuse, and subsequent timer delivery
  does not invoke consumer work. The fixture uses observable successful callbacks
  to detect accidental invocation rather than a trap that could roll back evidence.
- Restoring a settled cursor of `u64::MAX` yields Idle. Pause/resume preserves
  that cursor without overflow or callback invocation.

The full installed suite, probe/driver Clippy, formatting and diff checks pass.
This adds platform evidence for existing behavior; no product scheduling changes
were required. It does not qualify all supplier search costs or clock changes.
Toolchain, suppliers, lock hashes and reproduction command remain as recorded
above. Combined installed probe Wasm SHA-256:
f808653754d1723294f75b6f55bf270fa06a5bea6e05254130b4ce73547ccfd7.

### Bounded catch-up preview — 2026-11-15

The calendar feature adds a pure oldest-first planning API with explicit age
and count bounds (1–64). At most count timestamps are retained and count + 1
Saffron searches are performed, including the excess-work probe. The exclusive
settled cursor and inclusive age boundary jointly determine the search start;
the plan never enumerates the older excluded interval or rewinds its retirement
endpoint. Zero age admits only an occurrence exactly at the supplied time.

All 15 native Jobs tests pass, including full-u64 windows, age/count boundaries,
impossible rules and invalid limits. Installed planning checks report:

| Window | Selected | Excess | Planning instructions |
| --- | ---: | --- | ---: |
| Every minute, epoch through u64::MAX, count 64 | 64 | Yes | 324,193 |
| Every minute, 600-second endpoint, 300-second age, count 3 | 3 | Yes | 5,770 |
| February 31, epoch through u64::MAX | 0 | No | 808 |

These brackets include plan allocation and supplier searches, exclude parsing,
and are raw ordered samples, not a comprehensive worst-case supplier budget.
Full installed regressions, native default-free/all-feature and probe/driver
Clippy, Rustdoc and formatting pass. The default normal dependency graph remains
empty. No new supplier dependency was added.

This is preview only. Execution still uses skip-late behavior. Catch-up execution
must preserve quota and retirement progress across owner changes and upgrades
before consuming plans; no replay authority or unresolved-work persistence is
provided by this API. This is an explicit remaining implementation step.

Toolchain, supplier and lock identities remain unchanged. Combined tested probe
Wasm SHA-256, before subsequent documentation-only edits:
fdebdf6e31ebd0aa9e6d25bbcbe353656d5a34642565665f8bb1853ceb6b44ea.

### Catch-up execution and persisted remaining work — 2026-11-15

The adapter now takes explicit `Policy::SkipLate` or `Policy::CatchUp`. Catch-up
uses the previously qualified supplier planner, retaining at most 64 selected
timestamps and a fixed window endpoint. One Once capability executes those
occurrences serially. Age applies at batch selection, not again after each await
or pause. Final settlement skips excess and occurrences that arrived during the
batch, then arms a strictly future occurrence; it does not replenish the count.
An over-age previously selected occurrence may first settle as NoWork before the
eligible batch starts, without invoking consumer work for that old occurrence.

Pause preserves remaining selection. Schedule edits discard pending old-revision
batch work while preserving accepted identity. Explicit effect reconciliation
discards the remaining batch, skips through current time and leaves the job
paused. Initial-message batch construction does not replace the committed
occurrence, so factory rollback retains the correct uncertain identity.

Checkpoint schema 2 carries remaining timestamps and the fixed batch endpoint.
Native decoding validates bounds, order and consistency with the cursor and
queued/paused state. Restore checks every timestamp against Saffron and verifies
policy/count compatibility before admission. Installed negative cases prove
wrong-calendar, wrong-policy and excessive-count records leave declaration
identities available. The earlier unpublished schema 1 is explicitly rejected;
this is not a deployed-data migration. Hosts still own exact policy/definition
association, encoding and atomic storage.

Eight installed scenarios pass after ten-minute delivery delays:

- Three oldest occurrences run under count 3; count 1 runs just one.
- A two-minute age limit skips older work and executes the two eligible occurrences.
- Pause during the first occurrence retains the remaining two across actual
  upgrade and another ten minutes. Resume finishes exactly the original batch.
- The same pause/upgrade exercise with finite age retains its original remaining
  selection even after it ages beyond that limit; the quota is not reset.
- An edit during the first await discards the remaining old batch and selects
  future work under the new two-minute rule.
- Initial-factory and post-await traps retain the original occurrence, refuse
  upgrade, resist ordinary resume and require explicit reconciliation. Recovery
  discards the backlog; only the next future occurrence subsequently runs.

All 16 Jobs and 20 Timers native tests pass, as does the full installed regression
suite, product native/default-free/Wasm Clippy, probe/driver Clippy, Wasm Rustdoc
and formatting. No new supplier dependency was introduced. Coalescing,
unresolved-work restoration and broader resource/clock qualification remain open.
Tools, supplier versions and lock hashes are unchanged from the earlier sections.
The tested combined probe before subsequent getter/documentation-only edits has
Wasm SHA-256:
d9a1853b716ee28a165633a7d4a9652efa8b31454ee196a33ce46b6195778285.

### Full batch limit and later-occurrence traps — 2026-11-15

Installed catch-up coverage now includes a two-hour delivery delay at the maximum
admitted count of 64. All 64 timestamps execute in order, exactly once locally,
then the next selected timestamp lies strictly in the future. Additional rounds
do not replenish the batch. A fixture-owned active-call assertion covers every
await and fails if another occurrence starts before settlement or explicit
reconciliation. This is a single-job bound check, not a throughput measurement
or a durable exactly-once guarantee.

Two more scenarios trap on the second occurrence, respectively in its factory
and after its await. The first occurrence remains completed; recovery identifies
the second timestamp. Upgrade and ordinary resume remain blocked, and explicit
reconciliation drops the remainder without repeating the first or uncertain
occurrence. Only a newly selected future occurrence runs afterward.

The final full installed suite, probe/driver Clippy, formatting and diff checks
pass. The initial overlap assertion incorrectly counted reconciled abandoned
starts as active; the corrected fixture tracks active execution separately from
historical starts and completions. No product change was required by these cases.
Toolchain, supplier and lock identities remain unchanged. Combined probe Wasm
SHA-256:
493f5abeb692468e569cc74cbd1b79a7e1707746c3a4a86d47b36c27fa25bca7.
