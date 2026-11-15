# Candidate qualification: Timers 0.45.9 / Jobs 0.37.9

Local observation, 2026-11-15, from the implementation changes following GitHub
checkpoint `bc36b91`. This is source/installed-simulator evidence, not deployment
or crates.io delivery. Historical open-gate prose in the individual evidence
ledgers describes earlier slices; this records the current combined candidate.

## Implementation and qualification

- Jobs: explicit bounded retry-safe results and owner recovery, schema 6 progress,
  schema 1 unresolved records and latest outcome observations. Actual upgrades
  preserve running/uncertain attempt identity, consumed budgets, owner revisions
  and catch-up custody. Reconstruction remains blocked until explicit disposition.
  Ordinary unresolved export still refuses. No exactly-once effects are claimed.
- Timers: retained/remove-on-stop lifetimes, explicit removal, checked declaration
  incarnations, bounded slot reuse and capture release outside registry borrows.
  Installed tests cover initial/post-await traps, transient cleanup traps, owner
  precedence, reentrant destructors and obsolete queued Once/watchdog deliveries.
- Optional diagnostics: bounded sorted inventory and per-declaration normal
  results/failure observations, without control capabilities. Native and installed
  checks verify inert reads, reset behavior and canonical arithmetic.
- Native tests: Jobs 41 with all features / 23 without default features; Timers
  28 / 25 respectively. Strict native/Wasm Clippy and Wasm Rustdoc pass.
- The complete `check-once`, `check-executor`, `check-calendar-wasm` and
  `check-metrics` targets pass, including pressure, refusal, authentication,
  cleanup/fault custody, upgrades, recurring schedules, cancellation and retries.
- The isolated pinned Entropy adapter passes native tests, native/Wasm Clippy
  with/without features and its unchanged installed background-refill/lifecycle
  journey. The source repository was not edited. The patch explicitly selects
  retained watchdog lifetime. Actual adoption remains separate.

Rust 1.100.0, PocketIC 16.0.0. Live sparse-index review found the selected suppliers
current: Runtime 0.22.5, Metrics 0.30.8, Testkit 0.106.0 and Toko 0.44.4. Latest
published stable Scheduling versions were Timers 0.44.3 and Jobs 0.36.3.

| Artifact | SHA-256 |
| --- | --- |
| Combined installed probe | `f97c2913ffae0b6f7512cfe5dbc430b54bcc511ec2eb18e3cef2e061a66c32c8` |
| Entropy installed probe | `f1ea33d4b8fecb37ffc38a24de03cb86cbc174933f5c04f21f8958e4d995791c` |
| Entropy adapter patch | `f8bc67d07cbad158f5b1333a8203aed700b51387669195261de9451ce1280c23` |
| Product lock | `f4a9eec05b30405b4af44b8b2b91d44cb6354d8f61b2ae8682def9009fc5b881` |
| Qualification lock | `bb9e961b782edbd6456b769c39a020905069f5299bede00b2ceb9270adce6911` |

The small metrics probe measured 89,431 bytes disabled and 97,280 enabled
(+7,849). Three fresh canisters gave identical per-mode instruction samples:
setup +1,848, Once factory prefix +126, Once resume prefix +74 and watchdog
prefix +144. These are fixture deltas, not full production execution costs.
Calendar and optional execution remain excluded from timer-only consumers.

## Packaging boundary

Both candidate archives pass Cargo's native verification. Timers verifies for
Wasm. Cargo 1.100 multi-package Wasm verification fails resolving unpublished
Timers from its own temporary registry with `no hash listed`; a fresh target
directory reproduces it. Compiling the extracted Jobs archive with its only
local override pointing to the extracted Timers archive passes Wasm checks.
The normalized published dependency still names crates.io Timers; no path is
published. Registry-only Jobs verification is required after Timers publication.

This record does not claim release-adapter qualification, GitHub delivery,
registry availability, macOS execution or production consumer budgets. Heap
snapshots and legacy performance comparisons are outside the accepted release
scope; no code-reduction percentage is claimed from this expanded feature set.

## Release workflow preparation

The official Toko exporter installed the 0.44.4 release/publication closure; all
71 managed files verify. Both real group metadata transactions pass, including
qualification lock synchronization, unselected-group preservation, invalid public
selection refusal and downgrade refusal. ShellCheck and Actionlint pass. The
canonical manifests retain the preceding GitHub checkpoint versions until the
standard runner finalizes 0.45.9/0.37.9; this avoids advancing past the intended
candidate merely to adopt release tooling. Native macOS CI is now configured;
its first successful run and actual delivery are not claimed by this record.
