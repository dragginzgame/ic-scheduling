Status: implemented replacement architecture, 15 November 2026. The first rewrite
releases were Timers 0.45.9 and Jobs 0.37.9; current releases are 0.46.3 and 0.39.3.
Scope: Canic management, buffered Entropy and UTC Jobs scheduling; Relay is
excluded. This document records ownership and design constraints; dated evidence
records qualification, and consumer adoption remains separate.

Scope update: the maintainer now requires legacy Timers/Jobs capabilities and
defers additional proposals. [Release scope](release-scope.md) supersedes earlier
feature-omission decisions below; preserve the new ownership architecture without
assuming every removed legacy capability can remain absent.

Implementation note: Once, watchdogs and opt-in metrics are implemented and have
installed local qualification. See [Once evidence](../evidence/once-provider.md),
[watchdog evidence](../evidence/watchdog-state.md), [metrics](../evidence/metrics.md),
[control costs](../evidence/control-costs.md) and
[capture retention](../evidence/capture-retention.md). An isolated Entropy migration
passes its existing installed journey. Actual consumer adoption, Canic integration,
and production resource budgets are host-owned; the current local lifecycle
suite passes (see [candidate evidence](../evidence/release-candidate.md)). Historical
observations below do not establish completion.

Build a small retained-timer service around the two observed consumer workflows, and a separate calendar-aware `ic-jobs` package above it. The [Jobs design](jobs.md) records the current scope and contract. Bind work once, update demand through a capability, and isolate business execution from timer control. Reuse requires proof that a small implementation fits this architecture. Existing modules, signatures and test organization have no presumption of survival.

The practical debt target is **zero known unresolved correctness obligations at release**, with explicit ownership and measured complexity. Zero future technical debt cannot be guaranteed.

# 1 Evidence and required behavior

The supplied proposal establishes two Canic management workers and one Entropy refill watchdog, excludes Relay, and reports 5,425 product lines and 22,242 qualification/test lines in a dirty tree. Those counts are supplied estimates, not independently reproduced benchmarks.

Focused public-source checks establish the following:

- Scheduling at commit `0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e` exposes multiple registration/reconciliation policies and inventory APIs. Its watchdog implementation has cancel, ensure, immediate, reconciliation and unregister variants. These are evidence of the old design surface, not requirements for the replacement. [Runtime source](https://github.com/dragginzgame/ic-scheduling-legacy/blob/0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e/crates/ic-timers/src/runtime/mod.rs), [watchdog source](https://github.com/dragginzgame/ic-scheduling-legacy/blob/0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e/crates/ic-timers/src/registry/watchdog/mod.rs).

- The published crate manifest identifies `ic-timers` 0.44.3, a Cargo `links` guard, Metrics, Platform Runtime and thiserror dependencies. The new dependency graph must be justified independently. [Manifest](https://github.com/dragginzgame/ic-scheduling-legacy/blob/0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e/crates/ic-timers/Cargo.toml).

- Canic derives timer demand from its management journal and upgrade state; its callback performs an async step and derives the successor from current business state. Preserve these decisions in Canic. [Canic engine](https://github.com/dragginzgame/canic/blob/b36f6a39a67f487def523c847423fdfdb8d60c50/crates/canic-management/src/engine/mod.rs).

- Buffered Entropy owns its refill model, generation, asynchronous source request and backoff. Its synchronous watchdog launches work only when that model admits it. Opting out does not create a timer declaration. [Entropy buffer](https://github.com/dragginzgame/ic-entropy/blob/6c400b2c0259323ad1464fc692a4c4c378a2b101/crates/ic-entropy-ic/src/buffered/mod.rs).

The Internet Computer supplies one global timer per canister. SDK timer implementations isolate tasks using self-calls. These are platform constraints and a useful isolation pattern, not a mandate to reproduce the existing provider. [Timer concepts](https://docs.internetcomputer.org/concepts/timers/), [timer implementation guide](https://docs.internetcomputer.org/guides/backends/timers/).

This is a focused design review, not a complete audit of those repositories or a qualification of the supplied dirty tree.

# 2 Decisions that change the architecture

| Baseline choice | Replacement decision | Complexity removed |
| - | - | - |
| Reconcile carries slot, identity, cadence and replacement callback | Bind once; demand updates take only a typed handle and demand | Repeated declaration validation and discarded callback factories |
| Registry plus provider pending queue | One bounded entry table owns deadlines and eligibility; provider owns only transport flights | Duplicate pending scheduling state and stale queue tombstones |
| Watchdog scheduler self-call followed by work self-call | Bounded global-timer control prearms successor and submits isolated work | One dispatch role and message boundary, subject to platform qualification |
| Abandonment behavior left open | Once enters recovery-required state and rejects ordinary rearming | Implicit replay decisions and ambiguous precedence |
| Watchdog behavior under backlog implicit | At most one outstanding work ticket per watchdog; missed ticks coalesce | Catch-up queues and ticket invalidation starvation |
| Large snapshots as diagnostics | Capability status plus optional bounded inventory/failure observations | Mandatory diagnostic framework and memory histories |
| LOC target used as success proxy | Structural obligations, resource bounds and consumer measurements decide acceptance | Incentive to hide code in consumers or weaken tests |

The extra watchdog scheduler message may be restored only if a concrete installed failure demonstrates why the combined control step cannot meet its invariants. Record that evidence and revise the design before implementation continues.

# 3 Product and ownership boundaries

Publish `ic-timers` and `ic-jobs` only after their respective qualification gates.
Qualification packages remain unpublished. Jobs' optional execution feature depends on
Timers; its calendar-only feature does not. Timers must not depend on Jobs or
calendar logic. The module and API design below describes `ic-timers`; the
[Jobs design](jobs.md) adds its separate obligations. Private timer modules are
implementation boundaries, not separate packages.

| Owner | Exclusive responsibility |
| - | - |
| Canic | Durable intent, approval, external-effect uncertainty, management retries, journal reconciliation and whether another step is safe |
| Entropy | Pool state, refill admission, generation, outstanding physical source request, backoff and retry exhaustion |
| Jobs | UTC calendar rules, occurrence identity, missed-run decisions and job execution progress; never business-effect replay authority |
| Timer engine | Retained declarations, demand precedence, deadlines, work tickets, accepted attempts and recovery state |
| IC transport | Global-timer export, authenticated self-dispatch, bounded flight records, response handling and transport cleanup |
| Platform Runtime | Future polling, executor task lifetime and supported cancellation mechanics |
| Metrics adapter | Inert outcome recording; no scheduling decisions |
| Host lifecycle | Initialization, upgrade reconstruction and any supported snapshot-restoration procedure |

The engine owns no business journal. The transport owns no callback registry or second schedule queue. Metrics cannot make an attempt eligible. Dropping a public handle does not cancel work or delete the declaration.

Start with `api`, `engine`, `transport` and `metrics` modules. Keep unsafe platform bindings in one private transport submodule. Extract further modules only when they clarify an existing responsibility. Do not introduce a general reducer, effect language, plugin system, backend framework or scheduler policy trait.

Use a bounded table and scan for due entries. Three observed declarations do not justify a heap with cancellation bookkeeping. The current private bounds are 64 declarations, eight transport flights and four submissions per control turn. Installed pressure tests qualify these limits locally; they are not production throughput guarantees. Each entry has at most one current logical deadline and one work ticket. Transport flights have a separately fixed bound. A rotating scan cursor prevents low-index entries monopolizing dispatch.

# 4 Public contract

The canonical command/result types are in [api.rs](../../crates/ic-timers/src/api.rs);
wasm32 binding and handle methods are in
[transport.rs](../../crates/ic-timers/src/transport.rs). Both binding functions take
an identity tuple and explicit Lifetime; watchdog binding also takes a positive
cadence. Once supports Stop, At and completion-relative After results. Both handle
types support explicit removal subject to settlement guards. Methods use the
shared Error type. Callbacks have no Send requirement or public control context.

**Binding.** Initialization is idempotent for the current live runtime. Binding creates an inactive declaration with the selected lifetime and captures its callback once. Duplicate identity returns `DuplicateIdentity`; it does not return another capability or replace the callback. Consumers bind lazily into their existing optional slot, then call `set`. They must not rebind on every demand update. An unsuccessful bind leaves no entry. Release rejected or detached callback captures outside internal borrows.

**Identity.** Retain the bounded `(owner, group, name)` tuple only as declaration uniqueness and metric attribution. Validate each nonempty UTF-8 component at 64 bytes maximum. Identity is not execution authority and is not accepted by control methods. No normalization or silently truncated labels.

**Handles.** `Once`, `Watchdog` and `RecoveryToken` are opaque and nonforgeable; handles are not serializable or publicly constructible. Do not implement `Clone` unless a demonstrated consumer requires shared control. A handle contains runtime/declaration authority, not a raw reusable table index.

**Time.** Public timestamps are absolute nanoseconds. Zero and past timestamps mean eligible on the next dispatch opportunity; they never mean disabled. Translate them to a valid nonzero platform deadline internally. No callback executes inline in `set`. Check additions for overflow. A watchdog with no current deadline starts `KeepCadence` at now plus cadence; otherwise that command preserves the current deadline.

**Commands.** Every successful `set` is an explicit owner decision. While an attempt runs, even a repeated equal demand supersedes its eventual return value. Repeating an equal armed deadline before dispatch does not create another ticket or consume another wake generation. `Inactive` invalidates future eligibility, never claims to undo external effects.

**Errors.** Define small typed errors: uninitialized runtime, duplicate/invalid identity, declaration capacity, stale handle, recovery required, stale recovery token, deadline/counter exhaustion, and terminal provider fault. Validation errors leave scheduling state unchanged. Recoverable transport pressure is deferred internally after admission, rather than ambiguously returned as failed scheduling. A terminal control fault leaves the entry stopped/faulted and exposes that state; document this distinct error postcondition.

**Status.** Status is a bounded projection for the named capability: inactive, armed deadline, running, recovery-required token, or terminal fault; watchdog additionally reports whether a work ticket is outstanding. It exposes no callback data, registry inventory or recoverable authority by identity. Status does not authorize external-effect replay.

# 5 Once state and uncertainty

Use explicit states: `Inactive`, `Armed(deadline)`, `Running(attempt, override)`, `RecoveryRequired(token)`, and `Faulted(reason)`. A transport ticket can exist before acceptance; it does not mean the callback has started.

| Event | Required transition |
| - | - |
| Set At while inactive or armed | Retain one current deadline and invalidate any superseded unaccepted ticket |
| Accept current due ticket | Consume eligibility; enter Running before invoking the factory or polling the future |
| Set during Running | Replace the single pending owner override; never start a second future |
| Normal completion with no override | Apply callback Stop or At |
| Normal completion with override | Apply the latest owner demand, regardless of callback result |
| Confirmed abandonment | Retire attempt; enter RecoveryRequired; discard pending successor demand |
| Ordinary set At in RecoveryRequired | Return RecoveryRequired; remain blocked |
| Set Inactive in RecoveryRequired | Keep recovery-required state; it cannot erase uncertainty |
| Recover with current token | Consume token and apply supplied demand |
| Completion or recovery carrying stale authority | No scheduling mutation |

Keep declaration authority, unaccepted-ticket generation and accepted-attempt identity distinct. Use checked counters; exhaustion faults the entry before reuse. Do not use a single overloaded generation for all three lifetimes.

An abandoned attempt does not automatically honor a newer queued demand. That demand may predate discovery of an uncertain external effect. Canic must inspect its journal and reconcile the outstanding operation before calling `recover`. Recovery may legitimately select `Inactive`.

This is deliberately stricter than the baseline. The extra recovery operation is justified by a named consumer safety obligation, rather than by a general diagnostics feature.

A timeout is not proof that an async operation stopped. Never retire a still-live future merely because a deadline elapsed. Enter recovery-required only when the executor/transport contract proves local termination, or through a separately qualified lifecycle reset. Once also needs a defined route from an initial factory trap to recovery: a dispatch record committed before work starts must survive that trap.

No exactly-once guarantee extends to a remote management effect. The supported guarantee is one accepted local future per declaration, no automatic replay after abandonment, and rejection of stale completions.

# 6 Watchdog state and backlog

A watchdog is a periodic opportunity for a synchronous admission check. It is not an async execution lease.

Keep its current deadline, owner command revision, ticket identity and ticket state separately. Use one outstanding ticket across pending dispatch, accepted synchronous work and response settlement. Advancing cadence does not invalidate that ticket.

The watchdog core represents the owner revision as a per-running-callback
`owner_changed` flag: every command immediately updates demand and sets the flag,
including equal commands. One synchronous callback needs no unbounded revision
counter to determine whether its result was superseded. See the
[state and installed provider evidence](../evidence/watchdog-state.md); binding
and IC transport are implemented and locally qualified.

At a due control turn:

1. Compute successor as dispatch time plus cadence, using checked arithmetic.

2. Record that successor.

3. Create one work ticket only if none is outstanding.

4. Submit bounded isolated work and finish the control message successfully.

Those updates become committed together at the end of the control message. Merely calling the platform timer setter earlier in that message is not a separate commit.

If another cadence deadline arrives while the ticket remains outstanding, advance the deadline and coalesce the tick. Do not enqueue a second ticket and do not continually replace the old ticket with a newer generation. This prevents starvation when work delivery is slower than cadence.

The synchronous result has these meanings:

- `Continue` preserves the latest committed successor, including advancement while delivery was delayed.

- `Stop` invalidates future eligibility.

- `At(t)` replaces the successor with an absolute deadline.

- A newer owner command during reentrant work takes precedence over the returned decision.

`Inactive` invalidates queued unaccepted work. An already running callback cannot be undone. An explicit later `At` starts new eligibility, but stale transport flights remain bounded and cannot regain authority.

A trapped watchdog work message leaves the previously committed successor available. Release its outstanding ticket only after definitive transport/runtime settlement. A repeated work trap may cause one later admission check per cadence; there is no immediate retry loop. Entropy's model remains responsible for refusing another source request while one is physically outstanding.

No phase-locked catch-up semantics: missed cadence periods are skipped. If consumers later require fixed phase, treat that as a new requirement and cost, not an interpretation of `Continue`.

# 7 Message boundaries and transport

The normal path is:

```text
owner demand
    -> authoritative entry table
    -> bounded global-timer control
       [select due entries, prearm successors, retain flight records]
    -> isolated self-call
       [validate ticket, accept attempt, invoke consumer work]
    -> response or qualified cleanup
       [settle flight without replaying accepted work]
```

The global-timer handler invokes no callbacks, callback destructors, future factories, metric exporters or arbitrary consumer code. It performs bounded bookkeeping and submission only. This removes the need for an additional scheduler self-call while preserving the watchdog commit boundary before business work.

Transport records carry bounded tokens and lifecycle state, not closures. The entry table remains the source of pending work. Reserve flight capacity before submission. If submission is refused, retain unaccepted eligibility and schedule a bounded later delivery opportunity. If accepted, retain custody until the matching response or proven cleanup settlement.

A failed or ambiguous response is not permission to replay. Consult committed ticket/attempt state. If execution may have been accepted, resolve its lifecycle first; Once requires recovery after confirmed abandonment.

The provider uses explicit private constants for declarations, flights, submissions per turn and a one-second delivery retry delay. Installed measurements and pressure tests cover these values; production workload budgets remain host-owned. Full capacity defers work fairly without growing memory. Delivery backoff is transport policy; consumer retries remain consumer policy.

Authenticate self-caller before decoding anything beyond the fixed bounded envelope or touching scheduler state. Reject malformed length, unknown role and stale authority. A private method name is not access control. Do not add Serde/Wire merely to encode a handful of fixed-width token fields.

No runtime borrow may span callback invocation, future polling, callback destruction or an external API operation capable of reentry. Private transition functions may return small specific actions, but must not grow into a general command interpreter.

The IC cleanup callback runs when a reply/reject callback traps and can itself trap; it is not a universal business-work finalizer. Ordinary Rust destructor tests do not prove Wasm trap recovery. [IC interface specification](https://legacy.internetcomputer.org/docs/references/ic-interface-spec).

Therefore, before adopting a Platform Runtime primitive, establish with installed probes how it handles initial work traps, traps after an await, cancellation, reply-handler traps and cleanup-handler failure. Cleanup code must be bounded and avoid consumer callbacks or fallible metric recording. An unrecoverable cleanup fault must be visible as a terminal failure; no unconditional self-healing claim is permitted.

# 8 Lifecycle and observation

Initialization installs volatile runtime state; the host retains lifecycle ownership. After upgrade, bind fresh callbacks and derive demand from current consumer state. Timer state is not persisted. The platform timer must be explicitly rearmed after upgrade. [Upgrade guidance](https://docs.internetcomputer.org/guides/security/canister-upgrades/).

Snapshot restoration is a separate acceptance path. A heap snapshot may contain stale handles, tasks and transport flights. Reinitializing a counter inside that heap is not proof of a fresh incarnation. Support restore only through a qualified host procedure that reconstructs executor/provider state and rejects old messages before enabling new demand. If Canic's deployed workflow requires this, release is blocked until that procedure passes; otherwise declare snapshot restore unsupported for this release.

Cargo `links` guards prevent an important class of duplicate provider linkage. Final Wasm inspection must also establish one global-timer export and the intended dispatch endpoint. Neither check prevents arbitrary direct global-timer writes by other code. Review the whole consumer dependency graph and test ownership interference. The old implementation's observed-prior-value check is a detection mechanism, not proof of exclusive ownership. [Ownership source](https://github.com/dragginzgame/ic-scheduling-legacy/blob/0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e/crates/ic-timers/src/platform/ownership/mod.rs).

Record only useful facts: accepted, completed, abandoned, expected consumer failure, deferred submission and terminal control fault. Keep consumer outcome separate from schedule disposition. NoWork does not mean provider failure; ExpectedFailure does not request retry.

Counters must have documented overflow behavior, such as saturation plus an overflow flag. Metrics must not trap or alter eligibility. Do not attribute an entire interleaved async lifetime's instruction counter delta to one callback: measure attributable execution segments, or omit that aggregate until the executor exposes a valid measurement boundary.

Implementation update: 0.45.3 provides opt-in aggregate counters using
ic-metrics 0.30.8 scalar arithmetic. Recording is inert and export remains host-owned.
Counters reset on normal upgrade, not repeated initialization. Provider-fault
counting excludes isolated entry exhaustion. See [metrics qualification](../evidence/metrics.md)
for the event boundaries, overflow behavior and remaining measurement limits.

# 9 Deletions and controlled reuse

Keep job scheduling and retry policy in Jobs. Timers supports completion-relative
`OnceNext::After`, explicit lifetimes/removal, immediate watchdog demand through
`At`, and optional bounded inventory/failure observations. These preserve accepted
legacy capabilities without recreating separate registration families. Do not add
public callback-control contexts, paired memory histories or generic registry effects.

Do not copy the old repository scaffold, native test hierarchy, qualification cohorts, custom release-recovery framework or Jobs protocol fixtures. Create only the two required product packages and the minimal installed probe/driver packages required by the next acceptance gate. Jobs is a fresh design; the old Jobs implementation has no compatibility presumption.

Expected direct timer product dependencies are Platform Runtime and Metrics. Jobs uses optional Saffron-backed UTC calendar support, with pinned supplier selection and measured Wasm cost. Each enabled feature must be tied to a used primitive. thiserror is optional convenience, not architecture. Host tooling, Build, consumer crates and qualification dependencies stay outside the product graph. Timers has no production Storage or serialization requirement. Jobs needs upgrade-safe execution progress; its host persistence contract must be qualified before selecting storage or serialization dependencies.

Reuse is permitted only after the destination behavior and tests exist. Each reused unit needs a short record naming its source commit, required behavior, dependencies, ownership assumptions, alternatives and proof that excluded policies did not come with it. Review licensing and attribution. Reuse small primitives; importing an entire module and deleting later is not a greenfield step.

Old tests are a defect catalogue. Turn relevant failures into new black-box acceptance cases. Copy a fixture only if it expresses a preserved external contract, not an old internal representation.

# 10 Implementation sequence and hard cut

1. **Freeze the requirement ledger.** Enumerate the three consumer declarations, every call that changes demand, lifecycle paths, external effects and recovery owners. Capture consumer-visible trace fixtures. Identify whether snapshot restore is launch-critical.

2. **Implement one vertical Once slice.** Fresh crate, bounded table, one authenticated self-call and one actual async completion. Prove the message boundary and absence of overlap before generalizing capacity.

3. **Prove failure custody.** Add pressure, initial trap, post-await failure and stale response probes. Implement RecoveryRequired and consumer journal reconciliation. Do not continue if executor abandonment cannot be established.

4. **Implement watchdog coalescing.** Prearm in bounded control, isolate synchronous work, hold one outstanding ticket and prove delayed delivery cannot starve it. Test with Entropy's physical request admission.

5. **Add only required Metrics and bounds.** Measure at the declared capacity and exhausted flight limit. Set constants and fault postconditions from evidence.

6. **Migrate real consumers in isolated changes.** Canic binds two callbacks once, updates demand and implements explicit recovery. Entropy binds lazily once and retains its source-generation/backoff rules. Count all migration glue as part of the cost.

7. **Qualify the new Jobs scheduler.** Follow the [Jobs implementation sequence and gates](jobs.md). Qualify calendar calculation independently, then its execution and upgrade behavior on the timer layer. Existing Canic and Entropy behavior does not require adopting Jobs.

8. **Perform the hard cut.** Switch each final Wasm to exactly one provider. Remove old API use, dependencies, features, aliases, fixtures and release paths. Publish only after both consumer suites and lifecycle gates pass.

Do not run old and new timer providers together in one canister as a migration technique. Compare separate artifacts or test deployments.

Preserve business behavior and externally exposed consumer APIs unless a separate explicit contract change is accepted. This includes Canic approval/journal safety and Entropy opt-out, bounded refill and retry behavior. Internal Rust timer APIs deliberately break; provide no permanent compatibility facade. If required for release coordination, a temporary adapter must have one named consumer, a deletion commit and removal before the final gate.

Use an explicitly breaking crate release under the repository's versioning policy. Keep qualification artifacts and lockfiles pinned to that candidate. Reverting a dependency does not reverse already issued business effects; rollback must follow consumer journal semantics.

# 11 Acceptance gates

Each invariant gets a primary proof owner. Native transition tests establish ordering; installed canister tests establish platform behavior. Avoid recreating every case at every layer.

| Gate | Required evidence |
| - | - |
| Contract | The pinned Entropy adapter compiles without old timer types; Canic integration is deferred while that repository is rewritten |
| Once precedence | Owner At B during A overrides A's Stop; owner Inactive overrides A's At C; equal repeated commands follow documented precedence |
| No overlap | Suspended A plus repeated demand never creates B before A settles |
| Recovery | Initial trap and post-await abandonment produce a current recovery token; ordinary rearm is refused; stale recover/completion cannot mutate state |
| Watchdog commit | Work trap cannot erase successor committed in control message |
| Watchdog pressure | Hold work beyond multiple cadences; outstanding ticket remains one and eventually runs after pressure clears |
| Entropy admission | Slow source request plus many watchdog opportunities never produces a second physical request |
| Cancellation | Inactive defeats queued stale tickets; it makes no promise to cancel an already issued remote effect |
| Resource bounds | Full table, full flights and repeated replacement have bounded memory/captures and fair dispatch |
| Time and exhaustion | Zero, past deadlines, overflow and exhausted counters cannot wrap into valid authority or create a busy loop |
| Authentication | External caller, malformed token and obsolete ticket are rejected before callback execution |
| Trap custody | Installed tests cover submission refusal, reply/reject, work trap, callback trap and cleanup failure with explicit resulting state |
| Lifecycle | Fresh-runtime upgrade reconstructs demand and unresolved Jobs remain blocked; heap snapshots are outside this release |
| Ownership | Dependency and Wasm checks establish a single configured provider; interference test exposes foreign writes |
| Consumer behavior | Timer recovery never authorizes automatic replay; the pinned Entropy adapter preserves opt-out, backoff and retry exhaustion |
| Removal | No old scheduler compatibility layer, copied policy family or duplicate provider remains |

Record compiler, dependency locks, simulator/replica version, Wasm hashes and commands beside the gate results. Fault injection belongs in qualification artifacts and must not silently ship enabled.

Do not claim improved performance from LOC reductions. Installed measurements
cover control/dispatch instruction brackets, Wasm size, retained captures and
bounded pressure behavior. Equivalent legacy workload comparisons and actual
Canic integration are deferred under the accepted release scope; the recorded
costs do not establish an overall performance improvement over the old crates.

Treat the baseline's 1,400–1,800 product-line target as a review trigger, not a correctness limit. A missed target requires explanation of remaining responsibilities; meeting it does not establish architectural improvement. Count migrated consumer glue and qualification tooling, so complexity cannot disappear merely by moving repositories.

# 12 Completion criteria

The crate release is complete when the Jobs and timer gates in the accepted
[release scope](release-scope.md) pass, excluded features remain absent, resource
limits have local qualification, and no correctness-critical obligation is
deferred. Actual consumer adoption and production workload budgets are separate
from this crate release. The first published rewrite and its qualification are
recorded in the [delivery evidence](../evidence/release-candidate.md).

The first implementation review must show a working vertical slice and which old responsibilities it eliminated. “Imported provider,” “ported registry,” or “copied tests” is not redesign progress by itself.

Any proposal to add an abstraction must name the consumer behavior it enables, its state owner, its failure contract and the simpler alternative rejected. Any unresolved launch-critical recovery obligation blocks release. That is the enforceable meaning of the debt target.
