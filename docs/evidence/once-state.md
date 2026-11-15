# Once state core — 2026-11-15

Historical increment notes: the core and bindings are now integrated into the
[initial Once provider](once-provider.md). Unwired descriptions below identify
the state of each earlier checkpoint.

Implemented a private per-declaration state core in ic-timers. This is one part
of the planned Once vertical slice, not a usable timer runtime or a completed
implementation review. No old scheduler code or tests were copied.

The core owns eligibility, one unaccepted ticket, the accepted attempt and its
latest owner override. Ticket and attempt counters are separate and checked.
It contains no callbacks, futures, timer writes, transport queue or metrics.

Six native tests establish:

- A newer owner command wins over callback completion, including equal repeated
  demands. Running attempts cannot produce a second due ticket.
- Repeating an armed deadline retains its ticket; cancellation/replacement
  defeats stale delivery. Duplicate acceptance/completion has no authority.
- A normal completion without an owner override applies the callback successor.
- Confirmed abandonment discards pending demand and blocks ordinary rearming.
  Inactive cannot erase uncertainty; only the current recovery token can clear it.
- Stale completion, abandonment and recovery do not mutate newer state.
- Ticket/attempt exhaustion enters a terminal fault before counter reuse.

Commands passed locally:

- cargo test -p ic-timers --lib --locked --offline
- make lint PACKAGE=ic-timers
- make doc PACKAGE=ic-timers
- cargo check -p ic-timers --target wasm32-unknown-unknown --locked --offline

## Integration obligations

Tokens here are declaration-local, not public capabilities. The future bounded
entry table and authenticated transport must validate runtime/declaration
identity before forwarding them. Never use a token from one entry on another.
Repeated due selection returns the same reservation; the transport must track
its submission and settlement without submitting duplicate flights.

Acceptance changes state before callback factory invocation. The transport must
commit a dispatch record in the preceding message to survive an initial factory
trap. An abandonment call requires proven local termination; elapsed time is not
that proof. These native tests do not establish either platform obligation.

The module carries a scoped non-test dead-code expectation while it is unwired.
Remove that expectation when transport integrates it. It is private specifically
to avoid exposing an unqualified manual-dispatch or recovery API.

At this initial-core checkpoint, still required: retained binding and declaration
authority (implemented in the increment below), isolated self-calls,
actual async completion, post-await abandonment/cleanup probes, capacity and
pressure measurements, lifecycle behavior, and the separately pre-armed watchdog.
No installed IC execution, deployment, performance or binary-size claim follows
from these checks. This core does not add dependencies or change Jobs behavior.

## Retained binding increment — 2026-11-15

A private binding layer now connects the core to a bounded append-only declaration
table (provisional capacity 64). Each identity has three nonempty UTF-8 components
of at most 64 bytes. Duplicate identities never replace callbacks; failed admission
leaves the table unchanged. Handles hold weak authority to the exact runtime
allocation and a non-reused slot. Dropping a handle does not delete a declaration.
Runtime teardown invalidates handles; this is not a snapshot-restoration protocol.

One retained FnMut factory creates boxed non-Send futures. Acceptance changes the
entry to Running, then releases the table borrow before invoking that factory.
Polling and dropping the returned future are also outside table borrows. Rejected
callback captures drop after admission releases its borrow. A rotating scan
selects due reservations; transport still owns flight admission and duplicate
submission suppression. No provider queue or global timer has been introduced.

Four new native tests cover retained original callbacks and reentrant rejected
capture destruction, reentry during factory/poll/future destruction, runtime and
declaration-scoped authority, capacity/identity admission, and rotating selection.
Together with the six core tests, ten tests pass. Focused Clippy, Rustdoc and
wasm32-unknown-unknown compilation also pass. No dependencies were added.

Bindings and dispatch remain private and carry temporary non-test dead-code
expectations until transport is wired. The current future result is the scheduling
demand only; observation/metrics integration remains future work. These methods
must not be exposed as a usable timer API because set does not arm an IC timer.

The future IC transport must commit its flight record before the work message,
authenticate wire authority, and retain enough evidence to settle initial factory
traps. It must establish executor termination before calling abandon. Native
future Drop does not automatically clear Running or grant recovery. The local
weak-pointer identity is not a wire token, stable identifier or restoration fence.
This increment does not qualify isolated messages or trap recovery.

## Shared selector cleanup — 0.45.6

The current implementation removes the test-only watchdog table scan and the
Once-only flag on the shared selector. Production already always included both
kinds. Native Once and watchdog tests now invoke that same rotating selector;
their local helpers only assert/extract the expected dispatch kind. The old
assertion that a Once-only scan ignored watchdog work is retired with that mode;
callback-not-run-inline, mixed rotation and cadence-separation assertions remain.

The change removes 28 lines from implementation files (including test-only
helpers) and adds 12 net lines in test files: 16 net Rust lines removed overall.
No scheduling logic was relocated to test helpers. All 21 native Timers tests,
strict native/Wasm Clippy, formatting and the full installed suite pass. No
performance improvement is claimed. Compiler, suppliers and locks remain those
of the current 0.45.6 iteration. Reproduce with the native Timers tests and
`make check-once`; combined probe SHA-256:
`3a77b74cbf56bcde4e57fe5415e25d0cba7d8b8cde587490e1dcf399155b04e9`.

## Completion-relative recurrence — 0.45.7

`OnceNext::After(NonZeroU64)` restores the legacy completion-relative recurrence
capability through retained Once. The adapter reads IC time after the consumer
future returns and adds the positive delay with checked arithmetic. Overflow
produces inactive demand. Existing settlement owns newer-command precedence;
initial and post-await traps still block for explicit recovery. There is no
new registration family, state machine, dependency or implicit retry policy.
Initial eligibility remains an explicit `OnceDemand` supplied by the owner.

The focused installed `--after-completion` case passes. It observes Running
during a sequence of actual self-calls, advances time by 20 seconds, and checks
that the armed deadline equals recorded completion plus ten seconds. Nine seconds
later it has not rerun; after the deadline it runs once more and stops. Separate
cases prove owner stop overrides After, unsigned overflow stops, and initial and
post-await traps remain blocked after further time advances. Strict product,
probe and driver Clippy pass. This is local PocketIC evidence, not deployment or
full legacy timer parity; transient removal and diagnostics remain outstanding.

The change adds seven product lines; the fixture and driver are additional
qualification code, not a product reduction. No performance claim is made.
Reproduction commands are in [qualification](../../qualification/README.md).
Rust 1.100.0, Runtime 0.22.5, Testkit 0.106.0 and the prepared PocketIC 16.0.0
server are unchanged. Only local product versions changed in the locks.

| Artifact | SHA-256 |
| --- | --- |
| Combined probe | `6ddc1689391c37e0d493b29b0c2c8eced138d7e4a8536322565998fa9e78deaf` |
| Product lock | `dcf7370ec7815f0a37d36fa4b94dbda8232806fe746298eb2c50a7f8f95ac9e0` |
| Qualification lock | `edab8441b7adf7dfb3eb6ef3375e02385aae65f4b9125cba5d93c51d8bb81add` |
