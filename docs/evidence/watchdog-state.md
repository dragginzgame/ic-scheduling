# Watchdog state core — 2026-11-15

Implemented a private declaration-local watchdog core. This is not a public
watchdog API or installed watchdog execution. No old scheduler code/tests were
copied and no dependencies were added. The required behavior comes from buffered
Entropy's synchronous admission check and the design's prearmed successor rule.

The core separates the current deadline from its outstanding work ticket. A due
control turn advances the successor to control time plus nonzero cadence, skipping
missed periods. It creates a ticket only when none is outstanding. Pending delivery
remains available between cadence turns, so a submission refusal need not wait
for another cadence. Transport will own the delivery backoff and flight limit.

The ticket progresses through Pending, Running and Settling. Completion applies
the synchronous result but retains the ticket until definitive transport settlement.
Cadence advancement preserves that ticket through delayed delivery and settlement.
Inactive or a changed absolute deadline invalidates unaccepted eligibility;
stale physical flights must still be bounded and settled by transport.

Every owner command immediately updates demand. While the callback runs, a flag
records that an owner decision supersedes its result, including equal commands.
This requires no owner revision counter because only one synchronous callback
can be accepted. Continue preserves the latest successor; Stop and At replace it
only when no owner command intervened. Ticket counters and deadline arithmetic
fault before wrapping, preventing reuse of stale authority.

Six native tests cover delayed delivery across multiple cadences, no second
acceptance, settlement custody, preserved successors after proven work termination,
equal owner commands, cancellation/replacement, stale completion/settlement,
callback decisions and counter/deadline exhaustion. Together with Once and binding
coverage, all 17 library tests pass with Rust 1.100.0. Commands:

```sh
cargo test -p ic-timers --lib --locked --offline
cargo clippy -p ic-timers --all-targets --locked --offline -- -D warnings
cargo clippy -p ic-timers --target wasm32-unknown-unknown --locked --offline -- -D warnings
make fmt-check
```

These tests assume the caller has established settlement; they do not prove IC
trap termination or a committed successor. The core has no callbacks, executor,
timer setter, second queue or metrics. A scoped non-test dead-code expectation
marks the temporary unwired module and must be removed during integration.

Next, integrate retained synchronous callbacks into the existing bounded binding
table and transport. Installed qualification must prove the control message commits
the successor before work executes, traps preserve it, delayed delivery cannot
starve an outstanding ticket, and Entropy-style admission never launches another
physical request while one remains outstanding. No performance, deployment or
release-readiness claim follows from this native increment.
