# Installed capture retention — 0.45.4

Local PocketIC observation, 2026-11-15. This gate is independent of changing
consumer APIs and does not modify any sibling repository or product behavior.

## Workload and results

The standard installed suite now fills all 64 declarations with 32 Once and 32
watchdog callbacks. Each callback owns a distinct 4,096-byte boxed payload; all
three identity components use the admitted maximum of 64 bytes. Thus the fixture
retains 262,144 payload bytes and 12,288 identity bytes, excluding allocation and
provider overhead. These are deliberately chosen consumer payloads, not limits
the provider imposes on arbitrary user closures.

Ten update messages each perform 16 rounds of three demand commands per handle:
30,720 commands total. Once changes/repeats/stops a future deadline; watchdog
changes/keeps/stops its cadence. Every round also tries duplicate, full-table
and invalid-identity Once bindings with fresh payload captures. Rejected capture
destructors reenter initialization and query all retained Once handles, checking
that user destruction happens outside registry borrows.

After churn, three execution batches run all callbacks. Each Once awaits an
actual self-call and retains a Drop guard across the await. The driver verifies:

- All 64 original payload captures remain live throughout control and execution.
- All 480 rejected captures are dropped, without executing their callbacks.
- All 192 retained callbacks execute, including 96 Once attempts.
- All 96 per-attempt guards are released and no guard remains live after settling.
- Additional idle rounds leave those counters unchanged.

Each callback checks every payload byte before reporting execution. Linear memory
was 22 Wasm pages after binding, after each churn message and after every settled
execution batch. That is 1,441,792 bytes of whole-module memory, including the
fixture, allocator and runtime. Page stability is an observation, not a precise
allocator high-water measurement or proof against every possible leak.

The complete installed suite passes with this fixture, including existing trap,
pressure, watchdog, metrics, Jobs catch-up and upgrade cases. Strict Wasm probe
and native driver Clippy pass. Product implementation and dependency selections
are unchanged.

## Limits

This establishes retained-capture custody and completed-attempt destruction for
the stated mixed workload. It does not bound arbitrary consumer captures,
measure peak live allocations or prove destructor behavior for every trap path.
Existing failure-custody tests own trap evidence. Full-flight pressure and timing
have separate fixtures; their observations are not combined into a worst-case
latency or memory guarantee. The provisional table/flight constants remain
subject to production workload budgets and consumer integration.

## Reproduction and artifact identity

Run `make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic` with the prepared
server and local loopback access. The driver prints one `retention:` row, including
all 14 page observations. The maintained fixture adds 246 physical Rust lines
across probe and driver; include these in qualification cost measurements.

Environment: Rust 1.100.0, Runtime 0.22.5, Testkit 0.106.0, PocketIC client 16.1.0
and server 16.0.0. Both crates remain unpublished (Timers 0.45.4, Jobs 0.37.4).

| Artifact | SHA-256 |
| --- | --- |
| Combined once_probe.wasm | `b70c8ff24425220897db1fa66f02aa212a597f883dd248b2de0690b094abd035` |
| Product Cargo.lock | `97c31b1ddf7a5d6e9f5c706a9929d8f7cc0e2b3eda09ac43dd6d16300a7df136` |
| Qualification Cargo.lock | `fc8dad436e922662a72f814bb82dffd7794b53c1cf367e0d275b8f3df03db91f` |
