# Timers changelog

## [0.45.12] - Unreleased

### 🛠️ Development

- Adopt Toko Tooling 0.44.6 and qualify installed behavior with Testkit 0.107.1.
- Read the pinned Entropy integration commit independently of its working tree,
  preserving immutable source admission while allowing concurrent consumer work.
- Retain Runtime 0.22.5: released 0.23.11 lacks the timer, incarnation and
  trap-aware executor primitives required by the provider. Revisit migration
  when Platform supplies those capabilities; no replacement runtime is embedded here.

## [0.45.11] - 2026-11-16

### 🛠️ Development

- Adopt Toko Tooling 0.44.5 and activate the canonical staged-formatting hook,
  with focused qualification across product and qualification workspaces.
  Fixes [#1](https://github.com/dragginzgame/ic-scheduling/issues/1).

## [0.45.10] - 2026-11-15

### 🛠️ Development

- Place modules with child files under their own `mod.rs`, preserving public paths.

- Use public timer commands directly in the engines, removing duplicate enums
  and conversions without changing the public API.

## [0.45.9] - 2026-11-15

### ✨ Added

- Require an explicit retained or remove-on-stop lifetime at binding. Add safe
  removal, reusable bounded slots and checked declaration incarnations so stale
  handles and queued deliveries cannot control replacements.
- Keep running or uncertain Once work and unsettled watchdog tickets allocated.
  Release detached callback captures outside internal borrows; qualify reentrant
  destruction and cleanup traps in installed canisters.
- Extend optional metrics with bounded, sorted declaration inventory and per-timer
  acceptance, completion, abandonment and expected-failure observations. Inventory
  carries no control or recovery authority and uses ic-metrics arithmetic.

## [0.45.8] - Development checkpoint

GitHub-only version checkpoint; no Timers behavior changes in this iteration.
Not a crates.io release; qualification gates remain open.

## [0.45.7] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### ✨ Added

- Add `OnceNext::After` for completion-relative recurrence through the existing
  retained Once provider. Require a positive delay; stop on timestamp overflow.
  Preserve newer owner commands and explicit recovery after abandonment.

## [0.45.6] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### ♻️ Refactored

- Remove the test-only watchdog scan and Once-only selection mode. Native tests
  now use the production rotating selector, with only kind-checking helpers.
  Remove 16 net Rust lines across implementation and tests.
- Borrow the fixed 16-byte dispatch envelope from an array instead of allocating
  and growing a Vec. Preserve the authenticated incarnation/sequence wire format
  and let the protected async frame own the bytes through call settlement.
- Store callback trait objects directly in Rc/RefCell, removing the inner Box
  for Once factories and watchdog callbacks. Shared lifetime, exclusive invocation
  and rejected-capture drop ordering are unchanged; per-attempt futures stay pinned.

## [0.45.5] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### ⚡ Performance

- Select Once and watchdog wakes in one table pass over existing flight records,
  removing two temporary vectors and 22 net product lines. Installed control
  measurements improve at 3 and 64 declarations; binding costs increase slightly.
  Record both outcomes and artifact sizes in the control-cost evidence.

### ♻️ Refactored

- Simplify declaration admission with borrowed identity checks and ordinary
  parameter/local drop ordering. Remove the private identity wrapper and callback
  Option/take bookkeeping; rejected identities no longer allocate owned strings.
  Preserve reentrant callback destruction outside the table borrow. Successful
  binding costs rise slightly in the measured fixture; document the trade-off.

### 🛠️ Development

- Adopt Toko Tooling 0.44.4 in both workspace locks and the official snapshot.
  Verify all eight package directories match Cargo names; executable tools and
  product suppliers are unchanged.

### 🧪 Testing

- Extend full-table capture qualification to rejected watchdog bindings and
  verify dropping all public handles preserves captures, identities and occupied
  declaration slots. Armed Once/watchdog work still executes and settles after
  every handle is dropped, with no recurrence after Stop. No provider
  implementation change was required.

## [0.45.4] - Development checkpoint

GitHub-only checkpoint; not a crates.io release. Qualification gates remain open.

### 🧪 Testing

- Qualify capture retention at the full mixed 64-declaration limit with maximum
  identity lengths, 30,720 control commands, reentrant rejected-capture destruction
  and three complete async/synchronous execution batches; record memory observations.
- Qualify a pinned Entropy migration in an isolated copy: native buffered and
  fresh-only checks, Wasm compilation and its existing installed refill/lifecycle
  journey. Retain a reviewable adapter patch; actual consumer adoption remains open.
- Add paired metrics-disabled/enabled Wasm qualification with identical work and
  exports, three fresh canisters per build, verified outcomes and measured setup
  and per-message instruction prefixes. Record fixture-inclusive binary costs.

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
