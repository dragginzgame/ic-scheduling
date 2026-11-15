# Aggregate timer diagnostics — 2026-11-15

Timers 0.45.3 adds off-by-default `metrics`, using pinned ic-metrics 0.30.8 with
default features disabled. With it enabled, `ic_timers::metrics()` reads six
aggregate counters for the live provider incarnation. Host code owns encoding
and export. There is no per-declaration inventory, exporter callback, instruction
attribution across awaits or persistent metric state.

## Event boundary

| Counter | Recorded fact |
| --- | --- |
| accepted | Committed logical acceptance, including conservative accounting of a current rejected initial delivery |
| completed | Consumer callback returned normally, independent of successor disposition |
| abandoned | Confirmed Once abandonment or rejected current synchronous watchdog delivery |
| expected_failures | Normal return classified ExpectedFailure by the consumer |
| deferred_submissions | Actual self-call submission refused before acceptance |
| provider_faults | First terminal provider fault in this incarnation; isolated entry exhaustion is excluded |

An initial work trap rolls back its work-message counters. The existing rejection
path records acceptance/abandonment only when it actually transitions the current
ticket; stale tickets are not counted. Post-await Once cleanup records abandonment
only when the exact accepted attempt transitions. Watchdog rejection accounting
uses definitive settlement of its current outstanding ticket. These diagnostic
events are not proof that business effects happened. NoWork is a normal completion,
and ExpectedFailure remains separate from retry/successor authority.

The recorder uses a Cell containing a fixed-size copyable aggregate. It borrows
no application state and invokes no user code. Counters use MeasurementCounter:
totals clip at u128::MAX and retain a sticky actual-overflow flag; reaching the
cap exactly remains exact. Recording does not wrap or trap on arithmetic overflow.
Repeated initialize calls preserve totals; a normal upgrade creates a new volatile
incarnation with zero counters. Reading remains available after a provider fault.

## Primitive selection

This uses the supplier scalar arithmetic primitive, not its recording framework,
storage adapters or instrumentation macros. Reimplementing saturation/quality
flags locally would duplicate the canonical arithmetic requirement. No old timer
metrics module was transplanted. The optional normal dependency graph adds only
ic-metrics; disabled Timers builds omit it. Native graph checks are not Wasm-size
or instruction-overhead measurements, which remain open.

The cached published package reports source revision
`2f84d32aad0c198c5201daff5b0a151f28f56323`, path `crates/ic-metrics`, with
TOKO STUDIOS NETWORK LICENCE version 1.0. It is consumed as a dependency, with no
source copied into this repository. Local supplier `src/counter/mod.rs` SHA-256:
accd7af704202363d29c627a9d58c5f47027f66db305524173fe91241f7edb74.

## Qualification

All 21 Timers and 16 Jobs native tests pass, including adapter overflow and
outcome separation. The full installed Once/watchdog/Jobs suite passes with
metrics enabled. Additional assertions establish:

- Initial and post-await Once traps each produce one acceptance and abandonment.
- Consumer cleanup failure produces a terminal provider fault without pretending
  the accepted work completed or was successfully reconciled.
- Normal Once and watchdog NoWork/ExpectedFailure returns produce four completions
  and two expected failures, with no replay or abandonment.
- Metrics reads from within callbacks are reentrant, and initialize preserves counts.
- Repeated low-cycle Once refusals increase deferrals without acceptance; later
  funding produces one completion. Watchdog cadence wakes during one refusal's
  backoff do not increase the refused-submission count.
- Watchdog traps preserve prearmed scheduling while counting rejected deliveries.
- Foreign callers cannot create acceptance events; normal upgrade resets totals.

Product native/Wasm Clippy with metrics enabled and disabled, probe/driver Clippy,
Wasm Rustdoc, formatting and diff checks pass. The combined qualification probe
explicitly enables the feature. Existing cost fixtures now run with that choice;
their prior metrics-free values remain historical observations, not updated
performance claims. This is local installed evidence, not deployed-consumer proof.

Reproduce with `make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic`.
Rust 1.100.0, Runtime 0.22.5, Metrics 0.30.8, Saffron 0.1.0, Chrono 0.4.45,
Testkit 0.106.0 and PocketIC client/server 16.1.0/16.0.0 were used.
Tested probe before subsequent documentation-only edits, SHA-256:
9b6720a1df472eae3df41b6d06dc15e688744e236cfafab622fe1ef364fffd7c.
Product lock SHA-256:
9689d02b6c50a44a959133c791d7acad3bbff1414c5c7b124a26b0ffb9442aff.
Qualification lock SHA-256:
e8ba0945dbaddeb38757ed0a043b54475e1d3dfc349e06e9754e08122a231e7a.

The 0.45.4 continuation measures paired feature-off/on costs separately in
[metrics cost evidence](metrics-cost.md); the original identities above remain
the 0.45.3 accounting observation.
