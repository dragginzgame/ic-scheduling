# Jobs changelog

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
