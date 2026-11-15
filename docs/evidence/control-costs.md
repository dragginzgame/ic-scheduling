# Installed owner-control costs — 0.45.2, 2026-11-15

Later evidence: [capture retention](capture-retention.md) exercises a full mixed
table with payloads and maximum-length identities; [metrics cost](metrics-cost.md)
compares feature-disabled/enabled artifacts. These supplement the observations
below without turning them into general resource guarantees.

The existing installed probe now measures actual synchronous Timers operations
using ic0.performance_counter(0), before and after each call in a single update
message. No bracket spans an await, self-call, callback lifetime or other message.
Three freshly installed canisters run each case. All three samples agreed for
every reported value; the driver prints min/max rather than assuming equality.

The small table contains two Once declarations and one watchdog. The full table
contains 63 Once declarations and one watchdog. Filler declarations are inactive,
callbacks capture no business data, and no flights are outstanding. Deadlines are
in the future; declarations are stopped before the measurement message returns.
The fixture verifies final inactive states and refusal of a 65th declaration.

## Observed instructions

Values are raw bracket deltas, including counter/wrapper/assertion overhead.
The empty counter bracket costs 200 instructions; no subtraction is performed.
Binding costs include allocation and admission. The final binding is the third
or 64th declaration; the first two binding rows precede filler admission.

| Operation | 3 declarations | 64 declarations |
| --- | ---: | ---: |
| Empty counter bracket | 200 | 200 |
| Initialize | 1,089 | 1,089 |
| Bind first Once | 1,763 | 1,763 |
| Bind second declaration, watchdog | 1,707 | 1,707 |
| Bind final declaration | 1,840 | 24,156 |
| Once first arm | 1,951 | 4,570 |
| Once replace deadline | 1,963 | 4,582 |
| Once repeat equal deadline | 1,948 | 4,567 |
| Once inactive | 1,401 | 4,451 |
| Watchdog start cadence | 2,455 | 5,505 |
| Watchdog keep cadence | 2,448 | 5,498 |
| Watchdog replace deadline | 2,439 | 5,489 |
| Watchdog inactive | 1,907 | 4,957 |

The combined probe reports 18 linear-memory pages after either workload. This
includes the fixture, runtime, allocator and module memory, not just the retained
table. Page granularity does not demonstrate equal heap consumption or bound
consumer callback captures.

These are local simulator measurements, not production prices, latency guarantees,
a comparison with the old scheduler, or a justified capacity budget. In particular,
the table mix is not a worst-case identity/capture distribution. Full-flight scans,
control-message dispatch, Once completion, watchdog work admission, callback capture
retention and equivalent real-consumer workloads remain unmeasured. Keep the
provisional bounds explicitly provisional until those cases are assessed.

## Reproduction and identity

Run the standard installed suite with a prepared server and local loopback access:

```sh
make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic
```

The command builds the combined probe with Cargo's release profile and reports
`control_cost` rows. No optimization/profile changes or new dependencies were
introduced for measurement. Probe and driver Clippy pass with warnings denied;
formatting and diff checks pass. All earlier installed Once/watchdog cases pass
in the same run.

Observed with Rust 1.100.0, Platform Runtime 0.22.5, Testkit 0.106.0, PocketIC client
16.1.0 and server 16.0.0. Server identity is recorded in
[executor evidence](executor-probe.md). Timers is 0.45.2 and Jobs is 0.37.2;
publication remains disabled.

Combined probe Wasm SHA-256:
eeb06e03d36403f1e976c22ea80ebec4577c3082b4543177338011d17cec991c.
Product lock SHA-256:
f9b3f3971c2ebdd62f828840b413aa0efed8bda16c56cbf38676d13dfd649583.
Qualification lock SHA-256:
ebacffa4f6993789cc4f2010964d7466d6595fccf59447998d0400da40bdfef3.

## Dispatch and resume prefixes — 2026-11-15

A second installed workload arms 1, 8 or 64 Once declarations together. Each
callback awaits a real self-echo and returns Stop. Three fresh canisters run each
batch, producing 3, 24 and 192 samples respectively. The driver requires every
callback to finish and every declaration to become inactive, then checks that
additional rounds do not change counts or samples.

These are absolute per-message counter-0 readings at named execution points,
not differences across awaits. They include work preceding that point in its
message and the measuring call. They are **not complete dispatch or settlement
costs**, and the three points must not be summed as a callback lifetime cost.
First-poll samples also include the fixture's earlier factory sample recording.

| Observation point | 1 declaration | 8 declarations | 64 declarations |
| --- | ---: | ---: | ---: |
| Consumer factory entry | 486,315 | 486,277–487,371 | 486,277–487,371 |
| Consumer first future poll | 487,544 | 487,499–488,586 | 487,499–488,586 |
| Consumer resume after echo | 483,245 | 483,245–483,248 | 483,245–483,248 |
| Empty echo endpoint entry | 235 | 235 | 235 |

The similar 8/64 ranges are observations of this bounded-flight workload, not a
proof of constant complexity or a worst-case bound.

### Independent runtime baselines

The large prefix prompted an additional qualification endpoint that does not use
Timers: it samples raw entry, entry into Platform Runtime's tracking executor
context, and the first poll of an otherwise empty protected task. Each fresh
canister runs it before setup and again after all work and extra settlement rounds.

| Point | Fresh runtime | Warmed runtime |
| --- | ---: | ---: |
| Raw endpoint entry | 230 | 230 |
| Tracking executor context entered | 482,587 | 480,737 |
| Empty protected task first poll | 485,360 | 482,827–482,839 |

All nine fresh-runtime samples agree. Warmed context entry remains expensive,
so the dominant observed component is not explained by first-time initialization
alone. This isolates the cost to the runtime entry path in this artifact; it does
not establish which supplier primitive causes it or prove a supplier defect.
No sibling/source dependency was edited, and no executor custody or trap behavior
was traded away to reduce the number. A focused supplier investigation is warranted
before selecting production resource budgets. Do not subtract these baselines and
call the remainder a qualified standalone scheduler cost: the paths differ.

The full installed suite passes. Probe/driver Clippy, formatting and diff checks
pass. The earlier owner-control measurements remain unchanged. Product source,
compiler/supplier selections and locks are unchanged by this fixture increment.
Combined probe Wasm SHA-256:
7f624e6f595a1a84c6ae062c77d62b78622a372e2f5b805cf40e7dd13fd42f3e.

Complete control-message dispatch and post-consumer settlement costs still need
an attributable measurement boundary. This increment deliberately leaves those
gates open rather than reporting consumer-prefix observations as total costs.

## Single-pass wake selection — 0.45.5

Rearming previously collected separate Once/watchdog flight-reference vectors
and scanned the table separately for delivery deadlines and watchdog cadence.
It now passes a borrowed, cloneable iterator over the existing flight records
to one wake-selection pass. Cloning this iterator copies traversal state, not
flight records. No temporary vectors are allocated. Submitted-ticket identity,
runtime identity, replacement handling and independent watchdog cadence remain
part of the selection checks; the flight bound and dispatch budget are unchanged.

This removes 22 net physical product lines across bindings, watchdog bindings and
transport. No code was moved into consumers or a new helper package. Existing
native tests were adapted to the combined result, preserving stale/replacement
and mixed-cadence assertions. All 21 Timers tests, strict native/Wasm Clippy,
formatting and the complete installed suite pass, including held-flight pressure,
foreign timer detection, trap recovery and 64-job catch-up.

The existing control fixture ran unchanged before and after the product edit,
with three fresh canisters per workload and identical compiler, locks and build
profile. All three samples agreed for each value below. Brackets include counter
and fixture overhead; no baseline subtraction is applied.

| Operation | 3 entries before → after | 64 entries before → after |
| --- | ---: | ---: |
| Once arm | 1,955 → 1,760 | 4,574 → 3,773 |
| Once replace | 1,967 → 1,772 | 4,586 → 3,785 |
| Once repeat equal | 1,952 → 1,757 | 4,571 → 3,770 |
| Once inactive | 1,405 → 1,219 | 4,455 → 3,232 |
| Watchdog start | 2,472 → 2,245 | 5,522 → 4,258 |
| Watchdog keep cadence | 2,465 → 2,238 | 5,515 → 4,251 |
| Watchdog replace | 2,456 → 2,231 | 5,506 → 4,244 |
| Watchdog inactive | 1,911 → 1,688 | 4,961 → 3,701 |
| Bind first Once | 1,896 → 1,963 | 1,896 → 1,963 |
| Bind watchdog | 1,834 → 1,920 | 1,834 → 1,920 |
| Bind final declaration | 1,951 → 2,038 | 24,145 → 24,354 |

Binding cost increased despite no change to admission code. These are whole-build
observations; do not attribute every change to the removed allocations. The
measured control workload has no outstanding flights, so it establishes the scan
improvement rather than quantifying allocation savings under flight pressure.
Combined probe size fell from 444,713 to 442,555 bytes (2,158 bytes); this is not
a minimal-library Wasm measurement. Module memory stayed at 18 pages in these
control cases. No legacy performance comparison or production budget is claimed.

Reproduce with `make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic`, retaining
the `control_cost` rows. Environment: Rust 1.100.0, Runtime 0.22.5, Testkit 0.106.0,
PocketIC client 16.1.0/server 16.0.0; Toko Tooling 0.44.4. The compared product and
qualification lock hashes are `e08355a6cfd2a9ee3f06c6fb1dbf581a6831357d92f9476aabe5a15e838d06b6`
and `14ac0c780f15e1e15f03fa691f63504119369a8d3dceb903a89360fc94f2ce7d`.

| Artifact | SHA-256 |
| --- | --- |
| Before combined probe | `c19dc66d73e942f068b76dbc163c2d4aec33c5f9a093df51cc38908850d8047b` |
| After combined probe | `32fd23d5030b9036b3527be0bfbd9a9d3d95111effad6ca8024f01a910fc4c21` |

## Simpler declaration admission — 0.45.5

Admission now compares borrowed identity components against the retained strings
and allocates owned identity strings only after duplicate/capacity checks pass.
The private identity wrapper and Option/take bookkeeping around callback custody
are removed. The callback-kind function parameter outlives the local table-borrow
guard, so early returns release rejected user captures after releasing that borrow.
No custom cleanup guard or additional ownership abstraction is introduced.

This removes another eight net physical product lines. Existing native and
installed cases verify error precedence, retained original callbacks, capacity
refusal and safe reentrant destruction. In particular, all 960 rejected captures
in the mixed Once/watchdog retention fixture are destroyed safely. All 21 native
Timers tests, strict Clippy, formatting and the complete installed suite pass.

The unchanged control fixture gives the following successful-bind observations;
all three samples agree. This does not measure rejected-bind instruction cost.

| Operation | Before | After |
| --- | ---: | ---: |
| First Once binding | 1,963 | 2,142 |
| Watchdog binding | 1,920 | 2,095 |
| Third declaration | 2,038 | 2,212 |
| 64th declaration | 24,354 | 24,406 |

Successful binding costs increased. The gain is simpler ownership and avoidance
of three identity-string allocations on duplicate/capacity rejection, not a
general speedup. Once control measurements are unchanged from the preceding
single-pass result. Watchdog start/keep/replace decrease by 13 instructions;
do not attribute small whole-build changes to a specific primitive without
isolated evidence. Raw combined probe size falls from 442,555 to 442,186 bytes
(369 bytes). Compiler, supplier selections, lock hashes and reproduction commands
are unchanged from the preceding comparison.

| Artifact | SHA-256 |
| --- | --- |
| Before combined probe | `32fd23d5030b9036b3527be0bfbd9a9d3d95111effad6ca8024f01a910fc4c21` |
| After combined probe | `a448ddceecf04dc19961761211fd08038249d231e98648b98a7c6eb2ce02c5df` |
