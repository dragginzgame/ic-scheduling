# Jobs changelog

## [0.37.3] - Unreleased

Next development iteration; crates.io publication remains disabled.

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
