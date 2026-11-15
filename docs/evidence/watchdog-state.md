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

## Consumer-owned request admission — 0.45.2, 2026-11-15

A new installed fixture follows Entropy's ownership shape: a synchronous watchdog
checks a consumer-owned busy flag and launches a consumer-owned protected future
only when idle. Testkit holds that future's real target reply. Ten explicit At(0)
commands provide further admission opportunities without advancing simulated time
while the HTTP barrier is held. Eleven callbacks commit, but the independent
request counters stay at one start, zero finishes and busy=true. The watchdog
outstanding-ticket flag is false after each settled transport turn.

Releasing the exact held request produces one finish and clears busy. A final
Stop callback leaves the watchdog inactive. This establishes that watchdog
settlement does not own or retire the consumer's physical request. It is a
synthetic ownership fixture, not an Entropy migration; failure/backoff/cancellation
of that consumer future and delayed watchdog-flight pressure remain open.

The full installed Once/watchdog suite passes on Timers 0.45.2. Probe and driver
Clippy pass with warnings denied. Both packages remain unpublished; Jobs is now
0.37.2 with no new execution implementation. Supplier versions are unchanged.
Tested Wasm SHA-256:
e119def6e3745c3f5ddf812bd9b944838aa857a9e0348c891eddca26a3080b19.
Product lock SHA-256:
f9b3f3971c2ebdd62f828840b413aa0efed8bda16c56cbf38676d13dfd649583.
Qualification lock SHA-256:
ebacffa4f6993789cc4f2010964d7466d6595fccf59447998d0400da40bdfef3.

## Full-flight watchdog delay — 0.45.2, 2026-11-15

The installed fixture now occupies all eight provider flights with Once callbacks
awaiting Testkit-held replies, then arms a watchdog in the same canister. The
watchdog uses a 10-nanosecond cadence solely to cross multiple periods through
PocketIC's ordinary round-time progression. The driver does not call advance_time
while replies are held; ordinary ticks still advance simulated time slightly.
This is not a recommended cadence or an instruction-efficiency measurement.

While capacity remains full, the public deadline advances through multiple periods,
the outstanding-ticket flag stays true and the callback count stays zero. Changing
only consumer callback behavior to Stop does not replace pending eligibility.
Releasing exactly one of the eight replies allows one watchdog callback to execute
and settle. The watchdog becomes inactive with one committed callback, while seven
physical requests remain held. Releasing the remaining requests settles all eight
Once blockers without another watchdog callback.

Together with native token-identity tests, this establishes coalescing under the
shared flight bound and eventual delivery after capacity becomes available. It
does not establish fairness under every mixed workload or a production latency
bound. Arbitrarily delayed work is not required to observe a future successor
at execution time; the control message's committed successor is the invariant.
The short-cadence fixture therefore avoids the timely-delivery-only assertion
used by the earlier basic scenarios.

The full installed Once/watchdog suite and probe/driver Clippy pass. Formatting
and diff checks pass. Product code, dependency locks and compiler/supplier versions
are unchanged from the preceding 0.45.2 observation. The new combined probe hash
is 75c89ead3cf1d0d3a42f744770ad51149deb4da11972707516dd32ff97a27d42.
Remaining work includes short-cadence refusal stress, measured
resource limits, consumer failure/reconciliation integration, metrics and Jobs
execution.

## Short-cadence delivery refusal — 0.45.2, 2026-11-15

The existing low-cycle fixture now also binds a watchdog with a 10-nanosecond
cadence. It burns liquid cycles below half the quoted dispatch cost, then observes
cadence advancement with one outstanding ticket and no callback acceptance. After
100 milliseconds of simulated time, the driver restores funding and runs another
100 rounds. The deadline advances again but remains before the first one-second
retry floor; the outstanding flag stays true and callback starts stay zero.

Advancing beyond the retry floor then produces exactly one callback, which returns
Stop. Final status is inactive with no outstanding ticket. This is direct installed
coverage of the transport guard that allows cadence control while suppressing
premature submission; it would fail if a short cadence wake bypassed that guard.
No product changes or fault-injection hooks were needed for this scenario.

Unlike the held-reply pressure scenarios, this fixture has no held physical request,
so its explicit time jumps do not age an in-flight HTTP barrier. The short cadence
is test-only. The case does not measure control instruction cost, prove queue-full
refusal behavior, or establish a recommended minimum cadence.

The full make check-once suite passes with this case enabled. Probe and host driver
Clippy pass with warnings denied; formatting and diff checks pass. Compiler,
supplier selections and lock identities are unchanged from the preceding 0.45.2
observations. Tested combined probe Wasm SHA-256:
e4f045d2e809f3cd3899fb6863995bbf47abf7b3b171ca02812d142479cd9161.

Resource measurements, metrics, actual consumer migrations and Jobs execution
remain unfinished. These local simulator observations do not close remaining
consumer lifecycle/reconciliation or general failure-custody obligations.
