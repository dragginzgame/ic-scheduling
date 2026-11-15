# Timers changelog

## [0.45.1] - Unreleased

Next development iteration. No changes recorded yet; crates.io publication remains disabled.

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
