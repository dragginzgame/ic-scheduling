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

## Watchdog rejection and handle lifetime — 0.45.5

The follow-up repeats all three rejected-binding cases through both Once and
watchdog admission, including a watchdog attempt against an existing Once
identity. All 960 rejected captures are destroyed, and destructor reentry remains
safe. The earlier 64 retained payloads, 96 completed attempt guards and 192
callback outcomes remain unchanged. All 14 memory observations remain 22 pages.

After settlement the fixture drops every public Once and watchdog handle. All
64 captures remain live; attempting to reuse either kind's existing identity
still returns DuplicateIdentity, and a new identity still returns Capacity.
This verifies that handle destruction does not unregister a retained declaration.
It does not claim that stopping a canister or dropping a host-side reference
reclaims provider state; normal upgrade/reinstall remains a separate lifecycle.

The full installed suite, strict probe/driver Clippy, formatting and tooling
snapshot checks pass. Product behavior and supplier versions are unchanged;
Timers 0.45.5 and Jobs 0.37.5 are unpublished development versions. Reproduce
with the same `make check-once` command. The fixture extensions add 50 net physical
Rust lines across probe and driver (including formatting of the added cases).

| Artifact | SHA-256 |
| --- | --- |
| Combined once_probe.wasm | `9043b1248b70a96251569184c1c83ef44957cb3cad04db0b830251fb9c400718` |
| Product Cargo.lock | `86d7018768a193a877ddd5fc5aca206acc60ea9eaaf33e3a73853f78d49b4b53` |
| Qualification Cargo.lock | `bf05e74b9c910abf8563625db02c0e6c68192e8f69dd547e68165cbe2ab18def` |

### Armed callbacks after handle destruction

A second fresh installed canister now binds the same full mixed table, arms
every declaration one second ahead, and drops every public handle in the same
update. The update returns with all 64 captures retained and no callback run
inline. After advancing two seconds and allowing dispatch to settle, all 32 Once
callbacks have completed their actual self-call awaits, their 32 attempt guards
are released, and all 32 watchdog callbacks have returned Stop. No public handle
is available to drive or rescue that execution.

Advancing another five seconds and allowing further rounds leaves all custody
counters and memory pages unchanged. Thus Stop still suppresses future watchdog
opportunities when the owner no longer holds a control capability. Identity and
capacity refusals are checked immediately after handle destruction, as in the
inactive case. The original inactive-handle check remains in the suite.

This adds 60 net physical Rust lines to the earlier fixture and driver. Strict
Clippy, formatting and the complete installed suite pass; no product change was
needed. Versions, environment and lock hashes are unchanged from the 0.45.5
observation above. The updated combined probe Wasm SHA-256 is
`3d5138b15dbdcd50580b4d959ff6af59463798644dc8bc985db017d4222945ad`.
The same `make check-once` command prints an `armed handle drop:` result.
