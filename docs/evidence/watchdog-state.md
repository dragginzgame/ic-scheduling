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

## Shared binding increment — 2026-11-15

Private watchdog bindings now use the same append-only table and admission path
as Once. The 64-declaration ceiling and identity uniqueness apply across both
kinds. Typed private handles validate table, slot and kind before control; local
ticket authority remains distinct from serialized transport authority.

A bounded control pass advances watchdog successors without consumer calls.
Selection returns retained pending tickets. Synchronous invocation enters Running,
releases the table borrow, invokes the retained callback, then applies its result.
The callback may reenter owner control. Definitive transport settlement must still
release its ticket separately. These operations are private and do not yet arm
or submit watchdog work through the IC provider.

Two added tests cover mixed identity/capacity admission, absence of inline work,
reentrant owner precedence, duplicate invocation, delayed cadence, stale settlement
and runtime teardown. All 19 library tests and native/Wasm Clippy pass. The installed
Once suite also passes after the shared-table refactor, including pressure,
low-cycle refusal, traps and host upgrade admission. Tested Once Wasm SHA-256:
cdd2e97b27719c835f65d8893da147e212b41888a5c3a9f46727166063a8a1bc.
Supplier/toolchain/lock selections remain as recorded in the Once evidence.

No installed watchdog execution is claimed. Next integrate both kinds into fair
transport selection, include pending watchdog work and cadence in wake selection,
retain synchronous flight custody through replies, and prove prearm-before-work
using installed trap cases. Once and watchdogs must share the existing flight
bound, not acquire independent capacity or a second global timer.

## Installed watchdog increment — 2026-11-15

The wasm32 provider now exposes bind_watchdog, Watchdog::set/status and synchronous
WatchdogResult decisions. Both kinds share one declaration table, rotating scan,
eight-flight bound, four-submission control budget and authenticated endpoint.
The existing private endpoint name is retained; the committed flight determines
work kind, not an additional caller-controlled field. No second global timer,
callback queue or asynchronous watchdog lease was introduced.

Before submitting any work, timer control advances every due watchdog successor
in a bounded pass. Wake selection separates cadence control from delivery:
submitted tickets do not cause a busy loop, and full flights/refused submissions
do not prevent cadence advancement. A cadence wake before the delivery retry
floor advances control state without resubmitting work. Synchronous watchdog
reply/rejection settles its flight; a trapped work message rolls back its local
changes while preserving the previously committed control successor. Consumer-
owned async work remains the consumer's responsibility.

Four installed scenarios pass in the existing make check-once probe:

- Continue retains the prearmed successor across subsequent cadence turns.
- Repeated work traps leave a live successor, with no committed callback result;
  changing only callback behavior allows a later cadence to complete and stop.
- Reentrant repeated Inactive defeats callback At(0), with no later work.
- Reentrant owner At defeats callback Stop.

Callbacks inspect the public status before making decisions, confirming a
future successor and outstanding ticket in these timely-delivery scenarios.
The driver verifies ticket settlement afterward. This does not establish a
future deadline at every arbitrarily delayed delivery: control cadence and
message delivery are independent.

All earlier installed Once scenarios still pass. Twenty native tests pass,
including mixed-kind rotation and separate cadence/pending-delivery wake selection.
Native/Wasm product Clippy, fixture/driver Clippy, Wasm Rustdoc and formatting pass.
Compiler, suppliers and both lock identities are unchanged from the preceding
Once evidence. Tested combined probe Wasm SHA-256:
263ecf0b79bf0fc1bb81d27c6032f60a3ded62f0b1385e793cd4f4be3a907923.

Still required: installed delayed-flight watchdog pressure, short-cadence refusal
stress, Entropy-style physical-request admission, instruction/resource measurements,
metrics and actual consumer migration. These observations establish an initial
watchdog slice, not release readiness or a general cleanup/lifecycle guarantee.
