# Jobs changelog

## [0.39.9] - 2026-11-19

### 🧹 Cleanup

- Apply the shared Rust return spacing rules under Toko Tooling 0.45.21.

## [0.39.8] - 2026-11-19

### 🧹 Cleanup

- Adopt Toko Tooling 0.45.16, group same-feature re-exports and remove execution
  test conditions already enforced by their parent modules.

## [0.39.7] - 2026-11-19

### 🛠️ Development

- Adopt Toko Tooling 0.45.3 and qualify exact-stage reuse and batch timing summaries.
- Adopt Toko Tooling 0.45.13, honor invocation-scoped Cargo offline settings and
  align Rust module layout and refusal assertions with the engineering baseline.

## [0.39.6] - 2026-11-18

### 🛠️ Development

- Centralize the metadata-editor selection used by CI, release and qualification;
  verify that offline admission rejects a mismatched selection.

## [0.39.5] - 2026-11-17

### 🛠️ Development

- Adopt Testkit 0.110.0 and Toko Tooling 0.45.0, including the maintained Entropy fixture.

## [0.39.4] - 2026-11-17

### 🛠️ Development

- Adopt Testkit 0.109.0 in native qualification and the pinned Entropy fixture.
- Select Jobs implementation validation from an annotated release baseline,
  retain full gates for shared inputs, and reject stale qualification success
  after failed retries. Reuse exact final-candidate Rust stage evidence across
  groups on qualified Linux hosts, with evidence integrity checks before preparation.
- Scope final Jobs archive admission to Jobs and its published Timers supplier;
  retain paired archives for Timers reverse-consumer qualification. This addresses
  the archive dispatch portion of [deployment validation #4](https://github.com/dragginzgame/ic-scheduling/issues/4).
- Reuse Toko's pinned CI installer for Gitleaks and its nonempty Cargo test
  runner for native and Entropy qualification.
- Fix macOS CI's diagnostic-size rehearsal by waiting for its fixture output to
  drain before exit. Product code and published dependencies are unchanged.

## [0.39.3] - 2026-11-17

### 🛠️ Development

- Qualify execution with Runtime/guard 0.23.16 and optional Metrics 0.30.9,
  using Testkit 0.108.0 / Protocol 0.3.4 for native and pinned consumer fixtures.
  The Testkit error-type change is private to qualification; Jobs' API is unchanged.
- Bound native and scanner CI jobs using measured timings, with earlier command
  deadlines that retain sanitized failure evidence before the hard job limit.
- Allow the execution adapter to share the host's compatible Runtime 0.23 patch
  from 0.23.14, retaining one executor guard and the current 0.23.16 lock selection.
  Resolves [Scheduling #3 — Qualify Timers with Platform Runtime 0.23.15](https://github.com/dragginzgame/ic-scheduling/issues/3).

## [0.39.2] - 2026-11-17

### 🛠️ Development

- Allow compatible Saffron 0.1 patch updates instead of requiring exactly
  0.1.0. The current locked selection remains 0.1.0.

## [0.39.1] - 2026-11-17

### 🛠️ Development

- Qualify optional execution on the shared Runtime 0.23.15 cohort with Types
  0.21.2; schema 7 and calendar behavior are unchanged.
- Refresh private native fixtures to Testkit 0.107.4 / Protocol 0.2.8, resolving
  the exact-patch constraint in [Tooling #9 — Admit published Protocol 0.2.7 in Testkit's held-reply fixtures](https://github.com/dragginzgame/ic-tooling/issues/9).
- Use Protocol 0.3.2 and Wire's explicit frame codec in direct consumer
  qualification; retain the native supplier's supported Protocol 0.2 range.
- Adopt the standalone release metadata renderer in native transaction checks
  and delivery, resolving [Tooling #8 — Supply a released successor for the displaced Cargo release metadata editor](https://github.com/dragginzgame/ic-tooling/issues/8).

## [0.39.0] - 2026-11-16

### 💥 Changed

- Move optional execution to Timers' Runtime 0.23 cohort. Hosts using Runtime
  0.22 must migrate their runtime composition before enabling the new execution
  adapter. Checkpoint schema 7 and calendar behavior remain unchanged. Tracks
  [Scheduling #2 — Migrate Timers to Runtime 0.23 with one executor and retained watchdog recovery](https://github.com/dragginzgame/ic-scheduling/issues/2).

### 🛠️ Development

- Qualify the execution adapter and schema-7 upgrade probes on the shared
  Runtime 0.23.14 / Types 0.21.2 / Testkit 0.107.2 selection.
- Retain bounded, sanitized CI setup failure diagnostics and selected hidden
  Rust attempt logs as seven-day artifacts, preserving the failing status.
- Adopt Toko Tooling 0.44.11 across product and qualification workspaces.

## [0.38.2] - 2026-11-16

### 🛠️ Development

- Adopt Toko Tooling 0.44.6 and qualify execution with Testkit 0.107.1.
- Migrate the shared-flight pressure probe to Testkit's admitted held-reply catalog.
- Retain the qualified Runtime 0.22.5 dependency until Platform exposes the
  timer and trap-recovery capabilities needed by the execution provider.

## [0.38.1] - 2026-11-16

### 🛠️ Development

- Adopt Toko Tooling 0.44.5, restore strict workspace dependency linting and
  remove the unused Jobs catalog alias while preserving its release membership.
- Activate and qualify the canonical staged-formatting hook across both workspaces.
  Fixes [#1](https://github.com/dragginzgame/ic-scheduling/issues/1).

## [0.38.0] - 2026-11-15

### 💥 Changed

- Replace UnresolvedCheckpoint with schema 7 Checkpoint carrying optional
  unresolved attempt data. Ordinary restoration rejects unresolved records;
  explicit unresolved restoration still blocks until owner reconciliation.
  Schema 6 requires explicit host migration.
- Consolidate progress and execution policy into one state owner, including
  fault status and restored attempt identity. Retain retry budgets and lifecycle
  guards; policy transitions can now be tested natively.

### 🛠️ Development

- Qualify release metadata using the declared pending version, including minor
  releases, instead of assuming a patch increment.

- Refresh current documentation for pending versus published APIs, schema migration,
  bounded catch-up and provider capacity; retain dated qualification history.

- Place modules with child files under their own `mod.rs`, preserving public paths.

- Keep standalone registry archive verification outside the product Cargo
  workspace so the pre-upload Wasm check can resolve its published dependencies.

## [0.37.9] - 2026-11-15

### ✨ Added

- Add bounded retries for explicit RetryableFailure outcomes, with a total-attempt
  ceiling and capped exponential backoff. Preserve effect identity, catch-up quota
  and owner precedence; traps still require explicit recovery.
- Preserve pending retry counts and deadlines across pause and host-owned upgrades.
- Add explicit Retire/RetrySafe recovery dispositions. Retry-safe reconciliation
  preserves occurrence identity and consumed budget; both choices leave the job
  paused. Unavailable retries leave the attempt blocked.

### 💥 Changed

- Require a RecoveryDisposition in recover.
- Require RetryPolicy in bind/restore; NONE retains single-attempt behavior.
  Context and status expose the attempt number; Armed separates ready_at from
  the occurrence's scheduled time.
- Checkpoint schema 6 retains pending retry data and the latest returned outcome
  or explicit recovery disposition. Reject earlier preview schemas.
- Add explicit unresolved checkpoint export and fresh-runtime reconstruction.
  Preserve exact revision, occurrence and attempt number; restore blocked until
  explicit Retire/RetrySafe reconciliation. Ordinary checkpoints still refuse
  unresolved work. Hosts own atomic persistence and the upgrade boundary.

## [0.37.8] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### ✨ Added

- Add terminal `Job::cancel` with idempotent cancellation, rejected resume/edit,
  batch disposal and refusal while work is running or uncertain. Retained timer
  declarations remain allocated.
- Add completion-relative Jobs schedules. Preserve the completion-based deadline
  through pause and checkpoint reconstruction using the existing progress cursor;
  retain explicit delivery policy, owner edit precedence and recovery barriers.

### 💥 Changed

- Checkpoint schema 4 replaces the pause flag with Active/Paused/Cancelled control
  state. Preserve terminal cancellation across reconstruction, reject remaining
  work in cancelled records and refuse earlier preview schemas.

## [0.37.7] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### ✨ Added

- Add `Schedule::once` for a single deadline, including zero and overdue instants.
  Reuse delivery policy, checkpoints and recovery; settlement or lateness skipping
  exhausts the schedule without an automatic retry.

### 🐛 Fixed

- Admit fixed-rate jobs at their original first deadline, including overdue and
  zero timestamps. Reuse explicit skip-late/catch-up policies and preserve the
  distinction between unstarted progress and a settled zero occurrence.

### 💥 Changed

- Checkpoint schema 3 uses an optional settled cursor and admits zero in selected
  and pending batch timestamps. Reject earlier preview schemas; update the host
  upgrade fixtures and validate every restored occurrence against its schedule.

## [0.37.6] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### ✨ Added

- Add anchored fixed-rate elapsed intervals through `Schedule::fixed_rate`,
  reusing catch-up, execution and checkpoint validation. Checked unsigned
  arithmetic ends recurrence at overflow; cron remains delegated to Saffron.
  Full legacy schedule admission semantics remain an open scope item.

### ♻️ Refactored

- Return one concrete adapter future instead of boxing it before Timers boxes
  and pins it. Keep acceptance synchronous before the consumer factory; combine
  selection validation and acceptance under the existing state owner.

## [0.37.5] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### 🛠️ Development

- Adopt the shared Toko Tooling 0.44.4 baseline and verify package-directory
  naming across product and qualification workspaces.

### 🧪 Testing

- Fill all 64 shared declarations with catch-up jobs and qualify 192 ordered,
  nonoverlapping async occurrences after delayed delivery, with excess skipped
  and no batch-quota replenishment.
- Exercise completed-future destructor reentry: status/checkpoint reads and
  schedule edit/pause remain safe, and settlement preserves the newer pause.
- Exercise 161,051 bounded command sequences directly against occurrence progress:
  cursor monotonicity, unchanged state on failed commands, preserved uncertainty,
  stale-token rejection and settled checkpoint round-trips. No new dependency.
- Check wrong-job recovery-token rejection after initial and post-await traps,
  against armed and paused targets under both policies. Rejection preserves the
  target checkpoint and source uncertainty; correct recovery still succeeds.

### ♻️ Refactored

- Move definition-dependent checkpoint validation beside the checkpoint data,
  keeping timer admission in the Wasm adapter. Native tests now cover paused
  batches, count limits and invalid occurrences at the full 64-item boundary;
  restore errors and frozen-age behavior are unchanged.

## [0.37.4] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### 🧪 Testing

- Qualify restoring a settled cursor ahead of current time under SkipLate and
  CatchUp. Pause/edit/resume preserves progress, settled calendar boundaries do
  not replay, and new occurrences resume with the updated revision.

## [0.37.3] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### ✨ Added

- Add bounded oldest-first catch-up previews with explicit age and 1–64 count
  limits, excess reporting and a non-rewinding retirement endpoint. Recurrence
  remains delegated to Saffron.
- Add explicit `Policy::SkipLate` and `Policy::CatchUp` to bind/restore. Catch-up
  batches retain age/count decisions across pause and schema-2 checkpoint restore;
  completion skips excess/new arrivals, edits discard pending old batches, and
  recovery discards remaining catch-up work without replay.
- Add validated host-owned checkpoints and restore for settled/queued Jobs
  progress, preserving revision, cursor, selected occurrence and pause state.
  Running or uncertain work refuses export; schedule mismatch rejects restore
  before timer admission. Hosts own storage and matching job definitions.

### 🧪 Testing

- Cover catch-up age/count/cursor boundaries, impossible rules and full unsigned
  timestamp windows in native tests and installed Wasm planning measurements.
- Exercise catch-up execution, finite-age paused upgrades, count preservation,
  schedule edits and initial/post-await recovery; reject invalid restored batches
  before timer admission. Earlier unpublished schema-1 checkpoints are rejected.
- Exercise the full 64-occurrence execution limit after two hours of downtime,
  and factory/post-await traps on a later occurrence after prior work has settled.
- Cover checkpoint validation and progress round trips, installed armed/paused
  upgrades, export refusal during execution and upgrade refusal after initial
  and post-await traps.
- Exercise three-day stopped-canister upgrades with generous and zero lateness
  tolerance, preserving the selected occurrence and host-owned policy while
  avoiding catch-up bursts.
- Qualify twelve Jobs under shared-table and eight-flight pressure, queued pause,
  reverse reply settlement and post-await recovery isolated from healthy jobs.
- Qualify ordinary-outcome recurrence, impossible schedule edits and restored
  revision/timestamp exhaustion without accidental consumer invocation.

## [0.37.2] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### ✨ Added

- Add opt-in wasm32 `execution` on one retained Once per job, with explicit
  lateness tolerance, skip-late recurrence, occurrence context, owner controls and
  exact timer recovery mapping. Persistence and other missed-run policies remain open.

- Add private occurrence progress with schedule revisions, separate selection
  authority, scheduled/actual start context, pause/edit precedence and explicit
  reconciliation after uncertainty.

### 🧪 Testing

- Cover stale selection after cancellation, no overlap, old-revision completion,
  uncertainty across pause/edit/resume, monotonic settled progress and exhaustion.

## [0.37.1] - Development checkpoint

GitHub-only checkpoint; not a crates.io release.

No product changes in this iteration. Crates.io publication remains disabled.

## [0.37.0] - Development checkpoint

Recorded in local checkpoint commit 776a076. Not a crates.io release.

Development target for the breaking redesign after published `ic-jobs 0.36.3`
(crates.io index checked 2026-11-15). Scheduling execution remains unimplemented;
publication remains disabled.

### ✨ Added

- Make calendar support opt-in through `calendar`, with no default features.
  Disabled builds exclude Saffron and Chrono; Timers remains independent.
- Delegate five-field UTC cron parsing and recurrence to pinned Saffron 0.1.0.
  Support supplier calendar extensions, including last-day and nth-weekday rules.
- Preview UTC occurrences with exclusive/inclusive unsigned nanosecond bounds.
  Remove Croner, Jiff, time-zone data, local-time APIs and seconds dialects.
  No custom parser or calendar engine is retained.

### 🧪 Testing

- Verify UTC rules, calendar boundaries, impossible dates, rejected input and
  unsigned timestamp limits natively and in a standalone Wasm probe.
- Measure Wasm footprint with the actual adapter. IC execution and instruction
  costs remain unqualified.

### 🛠️ Development

- Establish an unpublished Jobs crate scaffold on Rust 1.100.0 for the planned
  calendar-aware scheduler.
- Adopt Toko Tooling 0.44.3 with shared setup, formatting and dependency commands,
  including `make deps-list` and `make deps-update`.
