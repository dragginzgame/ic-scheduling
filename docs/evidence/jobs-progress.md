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

### Restored cursor ahead of current time — 0.37.4, 2026-11-15

An installed fixture restores two minute-scheduled jobs with a settled cursor ten
minute boundaries ahead of the observed clock. One uses SkipLate with unlimited
lateness, the other CatchUp with unlimited age and a three-occurrence limit. Both
begin at revision 7 with no selected occurrence in the supplied checkpoint.
The adapter selects exactly the first minute after the cursor for both policies.

Two pause/edit/resume sequences happen while the clock remains behind the cursor.
Each preserves the settled cursor and next timestamp, removes selection while
paused, and increments the revision. Eleven sampled clock positions, ending one
second after the cursor, retain zero callback executions. The next two minute
occurrences then execute once each, in order, at revision 9; the next selection
advances one minute each time. CatchUp may retire through its planning endpoint,
whereas SkipLate settles the occurrence; neither rewinds progress.

This directly qualifies a host-restored future cursor under actual IC timer
delivery. It models the state relationship produced by a backward clock
discontinuity but does not move PocketIC's clock backward, qualify replica clock
behavior, restore unresolved effects or establish heap-snapshot safety. Existing
stopped-canister and delayed-delivery cases own the forward-jump evidence.

Run `make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic`; look for the
`jobs ahead cursor:` result. The complete installed suite and strict probe/driver
Clippy pass. No product changes or dependency updates were needed. This fixture
adds 172 physical Rust lines across the existing boundary probe and driver;
include them in qualification measurements.

Environment: Rust 1.100.0, Runtime 0.22.5, Testkit 0.106.0, PocketIC client 16.1.0
and prepared server 16.0.0. Timers 0.45.4 and Jobs 0.37.4 remain unpublished.

| Artifact | SHA-256 |
| --- | --- |
| Combined once_probe.wasm | `6a11757bcf7a9efa6eb3bfd565e52353ff3cabc5485c4f4cb6ab529dae26e4bb` |
| Product Cargo.lock | `97c31b1ddf7a5d6e9f5c706a9929d8f7cc0e2b3eda09ac43dd6d16300a7df136` |
| Qualification Cargo.lock | `fc8dad436e922662a72f814bb82dffd7794b53c1cf367e0d275b8f3df03db91f` |

### Recovery authority across jobs — 0.37.5, 2026-11-15

The installed initial-trap and post-await-trap scenarios now exercise recovery
tokens against unrelated jobs before reconciling the original attempt. Each
scenario creates a SkipLate target and a CatchUp target, and tries foreign
recovery while each target is armed and then paused: eight wrong-job attempts
across the two trap scenarios.

Every attempt returns `StaleRecovery`. The target's complete checkpoint remains
equal to its pre-attempt checkpoint, while the source remains RecoveryRequired
with the same revision and occurrence. Source checkpoint export still refuses
with Unsettled. Merely passing a recovery token to another job cannot retire the
uncertain occurrence or modify another job's cursor, selection or pause state.

The existing scenario then obtains fresh authority from the source, reconciles
it successfully, rejects a duplicate stale token, and resumes only a newly
selected future occurrence. The unrelated targets remain paused; their callbacks
trap if accidentally invoked. This preserves the original no-replay assertions
and adds a distinct job-identity boundary check. It does not provide durable
recovery tokens or qualify restoring unresolved work.

The full installed suite, strict probe/driver Clippy, formatting and diff checks
pass. No product code or dependency changes were needed. The fixture adds 45
physical Rust lines and one updated driver result label. Reproduce with
`make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic`; its Jobs result now
includes `recovery isolation`.

Environment remains Rust 1.100.0, Runtime 0.22.5, Testkit 0.106.0, PocketIC client
16.1.0 and server 16.0.0. Timers 0.45.5 and Jobs 0.37.5 remain unpublished.

| Artifact | SHA-256 |
| --- | --- |
| Combined once_probe.wasm | `9ce333c7676c4837cebe78745da4814b452a32f4c74f5b44f75163ad3f51d766` |
| Product Cargo.lock | `86d7018768a193a877ddd5fc5aca206acc60ea9eaaf33e3a73853f78d49b4b53` |
| Qualification Cargo.lock | `bf05e74b9c910abf8563625db02c0e6c68192e8f69dd547e68165cbe2ab18def` |

### Bounded native progress exploration — 0.37.5

The crate-local progress tests now enumerate all 161,051 length-five sequences
over eleven commands: select either of two timestamps, accept before/after due,
complete, block, reconcile, pause, resume, and edit with either a past or advanced
cursor. Each sequence starts from fresh progress; 805,255 command steps execute.
Issued selections are retained by the test to exercise stale authority after
pause, edit, completion or reconciliation invalidates them.

The assertions require a nondecreasing settled cursor, no state mutation on
failed commands, and preservation of accepted/blocked identity across unrelated
owner commands. Accepted or blocked work refuses checkpoint export; other states
round-trip through public checkpoint validation and private reconstruction.
Invalidated selections cannot accept, complete, block or reconcile current work.

This is bounded exploration of the private occurrence state, not a proof over
arbitrary sequence lengths, counter exhaustion, calendars, catch-up batches or
IC execution. Existing targeted boundary and installed tests retain those proof
responsibilities. The test uses ordinary Rust enumeration with no random seed,
property-test supplier, alternate transition model or product changes.

Rust 1.100.0: all 17 all-feature Jobs tests and all 10 default-feature-disabled
tests pass. Each suite completes in approximately 0.1 seconds locally. Strict
all-feature Clippy, formatting and diff checks pass. The change adds 107 physical
test lines beside `progress.rs`; it does not change the Wasm artifact or locks.
Reproduce with `cargo test -p ic-jobs --all-features --locked --offline` and
`cargo test -p ic-jobs --no-default-features --locked --offline`.

### Completed-future destructor reentry — 0.37.5

The installed Jobs fixture now returns a small future wrapper whose destructor
reenters its own job after the inner future completes a real self-call await.
This exercises destruction of the returned future itself, not only a local
guard dropped inside the async body. The destructor observes Running, confirms
checkpoint export still returns Unsettled, edits the schedule and pauses the job.

The callback completes once, without a state-borrow trap. Normal settlement
preserves the newer pause, and advancing through another minute does not invoke
another occurrence. The wrapper is qualification-only; no product change or
new dependency was required. This case covers normal completed-future destruction,
not a destructor that traps or arbitrary cleanup during unresolved work.

The complete installed suite, strict probe/driver Clippy, formatting and diff
checks pass. Reproduce with the existing `make check-once` command; the Jobs
result includes `reentrant future destruction`. The 0.37.5 toolchain and lock
identities above remain unchanged. Combined probe Wasm SHA-256:
`1ae603d930574d76750336f77ef357cae7c0886b03f38b137f92c1db2ff9a8f2`.

### Full-table catch-up execution — 0.37.5

An additional workload uses all 64 shared declaration slots for actual Jobs,
rather than combining twelve Jobs with inactive Once fillers. Each job uses an
every-minute schedule and CatchUp with unlimited age and a three-occurrence
limit. A 65th Job is refused with timer Capacity before execution.

The driver advances time by 301 seconds before delivery, leaving more overdue
occurrences than each job's quota. All 64 jobs then complete three ordered
occurrences with real self-call awaits: 192 starts and 192 completions. Each
callback asserts that its job's previous start already completed before another
start is admitted. Timestamps must increase; each job ends on its original first
selected timestamp plus two minutes. Every job returns to Armed state after
settlement. Eighty additional rounds leave all counts and recorded timestamps
unchanged: excess overdue work does not replenish the selected batch.

The driver stops this canister after that single-batch observation because the
fixture deliberately rejects more than three lifetime callbacks. That fixture
limit is not a product lifetime limit. Existing tests separately exercise future
recurrence, pause, traps, upgrades and eight held physical calls.

This establishes completion under this full-table catch-up workload, not a
production throughput, memory or fairness bound. It does not combine every
maximum, such as 64 occurrences in each of 64 jobs or indefinitely held effects.
No product change was required. The fixture reuses the existing pressure module
and adds 96 net physical Rust lines across probe and driver.

The complete installed suite, strict probe/driver Clippy, formatting and diff
checks pass. Reproduce using `make check-once` with the prepared server; look for
`jobs bulk:`. Versions, compiler, suppliers and locks remain as recorded for
0.37.5 above. Combined probe Wasm SHA-256:
`1950d4cdf8509f753d1d2ae5fb7ffb917e90cb8a93fd448ae43c89da824deb03`.

### Checkpoint validation ownership — 0.37.5

Definition-dependent validation now lives beside the checkpoint data instead of
inside the Wasm adapter's restore function. The private validator reads only the
checkpoint, Saffron schedule and policy. The adapter maps refusal to the same
InvalidCheckpoint error before allocating a timer declaration. Public APIs and
restore semantics are unchanged; no parallel validator or calendar logic was added.

Native coverage now admits full 64-occurrence batches in both queued and paused
forms, rejects a 63-count policy and SkipLate for those batches, and rejects an
off-calendar timestamp in the final pending position. A zero-age policy still
accepts the valid frozen batch: age was decided when the batch was selected and
must not be re-applied during restore. Selected occurrences without a batch are
also checked against the supplied schedule.

All 18 all-feature Jobs tests and 10 default-disabled tests pass, alongside
strict native and Wasm Clippy, formatting and the installed regression suite.
The existing installed invalid-restore checks still prove rejection leaves the
identity available for valid timer binding. This refactor adds six net product
lines and 43 test lines; its benefit is local ownership and native testability,
not a claimed LOC reduction.

Compiler/runtime/testkit selections remain unchanged. Toko Tooling is now 0.44.4
in both locks; its adoption changed policy documentation, not executable tooling.

| Artifact | SHA-256 |
| --- | --- |
| Combined once_probe.wasm | `c19dc66d73e942f068b76dbc163c2d4aec33c5f9a093df51cc38908850d8047b` |
| Product Cargo.lock | `e08355a6cfd2a9ee3f06c6fb1dbf581a6831357d92f9476aabe5a15e838d06b6` |
| Qualification Cargo.lock | `14ac0c780f15e1e15f03fa691f63504119369a8d3dceb903a89360fc94f2ce7d` |

### One future allocation boundary — 0.37.6

The Jobs callback adapter now returns a single concrete async future, instead
of a boxed trait-object future which Timers boxed again. Success and fault paths
share that future type; Timers remains the owner of dynamic storage and pinning.
The existing State owner now validates the current selection and records
acceptance in one method. The adapter releases the state borrow before invoking
the consumer factory, synchronously, before the returned future is polled.

This preserves initial-message rollback evidence and acceptance-before-factory
ordering. A late occurrence still settles as NoWork; invalid state remains
terminal and does not invoke the factory. No extra trait, state enum, allocator
or dependency was introduced. Across the adapter and State module this removes
three net physical product lines and the extra per-attempt Box allocation.

All 18 native Jobs tests, strict native/Wasm Clippy, formatting and the complete
installed suite pass. Installed cases cover factory/post-await traps, wrong-job
recovery, future-destructor reentry, pause/edit precedence, checkpoints and full
64-job catch-up. No new fixture was needed.

With unchanged fixture source, compiler, suppliers and build profile, the combined
probe decreases from 432,259 to 429,022 bytes (3,237 bytes). This is a fixture-wide
size observation, not a measured per-occurrence instruction saving or a universal
consumer size guarantee. Reproduce with `make check-once` and the prepared server.
The current Timers 0.45.6 callback-storage change is present in both artifacts.

| Artifact | SHA-256 |
| --- | --- |
| Before combined probe | `c63857831fd24af66db7124d5cf9b7b00497cc59bbcd83ebe7828ed70bb050c4` |
| After combined probe | `ae5c98e1e7b3f9a2099e793d785b3775dc2ce470fe08dba2916c7b2a17eaec50` |
| Product lock | `7f35cb46261358848c714d16a5e4782bbbe8e60d9ae876440285ed4440fa12e1` |
| Qualification lock | `7e7a5a3bed5fc8a8d0b9c872671d4585b88b90b9b89a365b18271da2a6f9af2c` |

### Anchored fixed-rate intervals — 0.37.6

The legacy-capability scope now includes elapsed fixed-rate recurrence.
`Schedule::fixed_rate` calculates the next anchored timestamp with checked
unsigned arithmetic and a positive interval. It uses the existing bounded
catch-up, execution and checkpoint validation paths; no dependency or separate
executor was added. Calendar rules still belong exclusively to Saffron.

All 20 all-feature native Jobs tests pass, including anchor alignment, exclusive
windows, unsigned overflow and combined catch-up age/count limits. Strict Jobs,
probe and driver Clippy pass. The installed suite passes with the bulk fixture
exercising both cron and fixed-rate schedules: each mode completes 192 ordered,
nonoverlapping awaits across 64 jobs, discards excess and does not replenish its
batch quota. This is local PocketIC evidence, not production qualification.

Binding still selects strictly after current time. Initial overdue deadlines and
timestamp-zero execution remain unfinished; this increment does not establish
full legacy schedule parity. One-shot and after-completion schedules also remain
required. See the [scope decision](../design/release-scope.md).

Reproduce with `make check-once` using the prepared PocketIC server; look for
`jobs bulk: mode=0` and `mode=1`. Compiler, suppliers and locks are unchanged from
the 0.37.6 observation above. Combined probe SHA-256:
`cc20b1fdecd42701c9fa1219a4ca08156192f087e0f6d7d6a73628f7c0437b17`.
The fixture now exercises an additional schedule mode, so this artifact is not
a like-for-like binary-size comparison with earlier probes.

### Original fixed-rate deadline admission — 0.37.7

Fresh fixed-rate binding now selects the definition's original first deadline,
including epoch zero and overdue instants. Cron still begins strictly after now.
Both use the existing Once execution/recovery path and explicit delivery policy.
The progress cursor is now optional: None means no occurrence has settled;
Some(0) prevents replay after timestamp zero settles. There is no zero sentinel.

Checkpoint schema 3 preserves this distinction and permits zero as the first
selected/batch occurrence. Earlier preview schemas are rejected. Native decoding
and definition validation still reject inconsistent cursors, selections and
batches. Hosts own encoding and definition association; unresolved work still
refuses checkpoint export. The existing cron-only upgrade fixtures require a
present cursor and use updated host format identifiers, rather than silently
flattening None to zero.

Inclusive recurrence lookup is shared by catch-up and checkpoint validation.
Saffron still performs all calendar search; its inclusive API is used only at
an exact minute boundary because both supplier APIs truncate their input minute.
Fractional lower bounds use its exclusive search. Fixed-rate arithmetic remains
checked, including the final representable unsigned occurrence.

All 23 all-feature native Jobs tests pass, including zero admission/settlement,
blocked recovery, checkpoint round-trips, zero batches, fractional cron bounds
and unsigned endpoints. Strict native Jobs and Wasm probe/host driver Clippy pass,
as do Rustdoc and formatting. The focused installed `--jobs-initial` run passes:
zero/overdue/future first deadlines, bounded catch-up, skip-late, reconstruction
of unstarted selected-zero progress, existing armed/paused upgrades, downtime
policies, full 64-count batches and first/later trap recovery. Unstarted zero
reconstruction is exercised directly; actual upgrade fixtures remain cron-based.

This adds required behavior rather than claiming a size or performance reduction.
Reproduction uses the focused command in [qualification](../../qualification/README.md).
Compiler, suppliers and locks match the current 0.45.7/0.37.7 iteration. Local
combined probe SHA-256:
`7827872d077475fc08222fc729b6f4a19dbf6940381fbfaa7cb590547414c54c`.

### One-shot Jobs — 0.37.7

`Schedule::once(at_ns)` adds a single deadline to the existing schedule owner.
It shares first-deadline admission with fixed-rate schedules, including zero
and overdue instants. The existing delivery policy determines whether late work
runs; settlement or skipping exhausts the schedule. No execution state machine,
transport, dependency or checkpoint schema was added for this schedule form.
Public `next_after` retains its exclusive lower bound, including at zero.

All 25 native Jobs tests pass. New native cases cover zero/unsigned endpoints,
one-occurrence catch-up exhaustion, age exclusion and exact-deadline checkpoint
validation. Strict native Jobs and Wasm probe/host driver Clippy, Rustdoc and
formatting pass. The installed `--jobs-one-shot` selection reuses the initial
deadline fixture for both delivery policies, zero/overdue/future admission and
unstarted reconstruction. It verifies idle status after completion or skipping
and no recurrence after another simulated hour. Factory and post-await traps
retain uncertainty and refuse checkpoints; pause/resume does not clear recovery.
Explicit recovery then resume leaves the exhausted one-shot idle, with no new
callback after a further hour.

This is local installed evidence, not a guarantee of exactly-once external
effects. Jobs retry policy, after-completion recurrence and durable unresolved
records remain unfinished. Reproduce via the focused command described in
[qualification](../../qualification/README.md); compiler, suppliers and locks
are unchanged. Combined probe SHA-256:
`c50cb59677c6e0eccfbb6b696fb1adff6136c396f2b6621b1d6a295c4868e86e`.

### Completion-relative Jobs — 0.37.8

`Schedule::after_completion` selects the explicit first deadline, then computes
the next deadline from normal settlement plus a positive interval. The existing
cursor includes that settlement horizon, so pause and host reconstruction retain
the deadline without a new queue, transport or checkpoint field. Schema 3 restore
checks the exact selected deadline against the cursor and rejects catch-up batches
for this schedule. Earlier-revision completion cannot advance a newer definition.

ExpectedFailure, NoWork and lateness skipping currently all settle. Bounded retry
policy remains unfinished; this does not infer permission to retry an uncertain
effect. CatchUp applies its age limit to the one pending occurrence, without an
invented backlog. Explicit reconciliation records a new horizon and pauses;
resume schedules from that horizon. Arithmetic overflow exhausts recurrence.
Calendar preview methods show only the initial deadline for this schedule form.

All 27 all-feature native Jobs tests pass, including exact checkpoint validation,
paused reconstruction, rejected batch state and unsigned overflow. Strict native
Jobs and Wasm probe/host driver Clippy, formatting and Rustdoc pass. The focused
installed `--jobs-completion` run proves a 20-second time advance during real
awaits anchors the next deadline at completion, ExpectedFailure/NoWork recurrence,
no burst after an hour's delay, lateness skipping, pause/resume, actual armed and
paused upgrades, newer owner edits, overflow and factory/post-await recovery.
Unresolved upgrades refuse. The affected `--jobs-initial` selection also passes,
including existing checkpoint lifecycle, downtime and bounded catch-up upgrade
and trap cases. These are local installed observations, not production evidence.

Reproduce using the focused driver selections in
[qualification](../../qualification/README.md). Compiler and suppliers are
unchanged; the locks advance only the local product versions. This increment
adds required functionality and its qualification; no LOC or performance gain
is claimed.

| Artifact | SHA-256 |
| --- | --- |
| Combined probe | `33e538a0ed2a7e2bace32838c70f80af7512703afda811d592c42042f9aeb70f` |
| Product lock | `4dd2fedbdbcc9d5b45ad122193de654e0ddc92c106eff4f344cefc8965478709` |
| Qualification lock | `1617ad0682b642f7fdfb6bd028bc9690a8734e9b5bd7cd82895e556f55d3561a` |

### Terminal pending cancellation — 0.37.8

`Job::cancel` now invalidates future timer eligibility and discards unaccepted
selection and remaining catch-up work. It refuses Running/RecoveryRequired work
with Unsettled; no unresolved identity or effect evidence is erased. Repeated
cancellation succeeds. Pause, resume and edit return Cancelled after cancellation.
The underlying timer registration and callback captures remain retained; removing
registrations is still separate unfinished work.

Checkpoint schema 4 replaces the pause flag with the shared ControlState enum:
Active, Paused or Cancelled. The constructor rejects selected work or a batch in
a cancelled record, and rejects earlier preview schemas. Cancelled restoration
stays inactive without recurrence selection. Existing cron-only host fixtures
explicitly refuse cancelled records rather than flattening them into a pause bit;
the completion fixture persists all three states and the optional cursor. Host
format identifiers were updated; no deployed-data migration is claimed.

All 29 native Jobs tests pass. New cases cover terminal cancellation after a
queued zero occurrence, checkpoint round-trip, refused future commands, paused
cancellation, unchanged running/blocked progress on refusal and invalid cancelled
records. Strict native Jobs and Wasm probe/host driver Clippy, Rustdoc, formatting
and diff checks pass.

The installed `--jobs-completion` selection now also cancels pending, paused and
restored paused-batch jobs before invocation. Each remains Cancelled after actual
upgrade and an hour's advance, with no callback invocation or remaining batch.
The fixture asserts refusal during real awaits and after both trap types, and
checks rejected resume/edit plus idempotent cancellation before and after upgrade.
The affected `--jobs-initial` selection passes existing initial-deadline, checkpoint
lifecycle, downtime and bounded catch-up upgrade/recovery checks under schema 4.
These are local installed checks, not production evidence.

Reproduce with the focused commands in [qualification](../../qualification/README.md).
Compiler, suppliers and locks remain those of the current 0.45.8/0.37.8 iteration.
Combined probe SHA-256:
`36374273fe5af238e83da5d8efa93adb600b0352fbcf085b523810b7bab5ec50`.
