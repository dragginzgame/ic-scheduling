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
