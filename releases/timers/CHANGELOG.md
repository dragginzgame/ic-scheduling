# Timers changelog

## [0.45.3] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### ✨ Added

- Add opt-in aggregate metrics backed by ic-metrics 0.30.8 counters: committed
  logical acceptance, normal return, abandonment, expected failure, refused
  submission and terminal provider fault. Overflow saturates with a sticky flag;
  recording has no scheduling authority or exporter callbacks.

### 🧪 Testing

- Check initial/post-await Once traps, watchdog rejection, low-cycle deferral,
  ordinary outcomes, reentrant reads, initialization preservation and upgrade
  reset. Native coverage verifies overflow remains diagnostic.

## [0.45.2] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### 🧪 Testing

- Sample dispatch/resume message prefixes at 1, 8 and 64 declarations. Add
  independent cold/warm executor baselines, isolating a large runtime-entry
  component without claiming complete callback or settlement costs.

- Measure synchronous owner operations at three and 64 declarations in installed
  Wasm, with three fresh-canister samples per case and explicit counter overhead.
  Record the observed costs without claiming async lifetime or full-flight costs.

- Verify a short watchdog cadence cannot bypass low-cycle delivery backoff after
  funding is restored; cadence advances before one eventual callback completion.

- Hold all eight shared flights while watchdog cadence advances; releasing one
  flight admits one coalesced callback without replaying missed opportunities.

- Exercise consumer-owned async admission from synchronous watchdogs: eleven
  admission callbacks leave exactly one held physical request, while watchdog
  transport settles independently. Releasing the request completes it once.

## [0.45.1] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Remaining qualification gates
continue to block publication.

### ✨ Added

- Connect watchdogs to the shared IC timer, bounded flights and authenticated
  dispatch. Add synchronous binding, demand and status APIs; preserve cadence
  advancement independently of delivery pressure and retain tickets until settlement.

- Add the private watchdog state core: cadence advancement, coalesced work tickets,
  owner precedence, settlement custody and checked exhaustion.
- Retain private synchronous watchdog callbacks in the same bounded declaration
  table as Once, with shared identity admission and callback execution outside
  table borrows.

- Add the initial wasm32 Once provider: idempotent initialization, retained
  binding, explicit owner control/status/recovery and isolated async execution.
- Bound self-call flights and per-turn submission, authenticate dispatch, fence
  upgrade-era messages, back off submission pressure and detect timer interference.
  Retain flight evidence so initial and post-await traps cannot silently replay work.
- Add the Cargo global-timer singleton guard. Watchdog pressure qualification and
  metrics remain unfinished.

### 🐛 Fixed

- Exclude already-submitted tickets from the next deadline calculation. Pending
  work no longer adds a one-second delay to unrelated due declarations while
  flight capacity is available; full capacity and submission refusal still back off.

### 🧪 Testing

- Hold a live Once attempt through a refused host upgrade, verify duplicate
  dispatch is inert and the original reply settles once, then upgrade with fresh
  armed demand. Unresolved-work upgrade reconciliation remains unqualified.

- Exercise real low-cycle submission refusal: retain armed work without acceptance,
  back off repeatedly, and complete once after funding and the retry deadline.

- Verify that a consumer destructor trapping during cleanup leaves a terminal
  fault with no replay. Exercise malformed/stale self-dispatch and host-owned
  reconstruction of an armed future deadline across an actual upgrade.

- Exercise all 64 declarations and the eight-flight limit using Testkit-held
  replies. Check cancellation, repeated demand without overlap, reverse-order
  settlement, later rearming, and visible faults on foreign global-timer writes.

- Route installed drivers through `ic-testkit::pocket_ic`, following ic-platform’s
  qualification boundary; remove direct PocketIC dependency selection.

- Add actual-provider PocketIC scenarios for completion, initial/post-await traps,
  explicit recovery and owner-command precedence across an await. Run with
  `make check-once`; resource measurements and remaining lifecycle/failure gates
  still block release.

Crates.io publication remains disabled.

## [0.45.0] - Development checkpoint

Recorded in local checkpoint commit 776a076. Not a crates.io release.

Development target for the breaking rewrite after published `ic-timers 0.44.3`
(crates.io index checked 2026-11-15). The private Once state core is implemented;
retained bindings are private and tested. Public initialization and IC execution
remain unimplemented/unqualified. Publication is disabled.

### ✨ Added

- Implement private Once eligibility and attempt custody with owner-command
  precedence, stale-ticket rejection, explicit recovery and checked counters.
  No dispatch API or timer provider is exposed yet.

- Add private retained callback bindings, bounded identity/table admission,
  runtime/declaration-scoped authority and rotating due selection. Callback
  invocation, polling and destruction occur outside table borrows.

### 🧪 Testing

- Add an independent installed executor probe using Platform Runtime 0.22.5 and
  PocketIC 16.0.0. Verify isolated async completion, initial/post-await traps and
  self-only admission. Reproduce with `make check-executor`; provider integration
  and cleanup-failure gates remain open.

- Cover no-overlap eligibility, command precedence, cancellation/replacement,
  stale completion/recovery, abandonment and counter exhaustion natively.
- Verify retained captures, reentrant consumer code/destructors, stale and
  cross-declaration authority, and capacity admission.
- Check native lint/docs and Wasm compilation; installed IC gates remain open.

### 🛠️ Development

- Establish an unpublished Timers crate scaffold on Rust 1.100.0 and reserve
  a separate qualification location for installed probes.
- Adopt Toko Tooling 0.44.3 with shared setup, formatting and dependency commands,
  including `make deps-list` and `make deps-update`.
