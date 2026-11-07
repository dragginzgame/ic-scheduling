![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Architecture

This document is a technical reference for developers working on or
integrating IC Timers. For a general introduction, start with the
[project README](../../crates/ic-timers/README.md).

## Overview

The application owns the lasting record of what work needs to happen. IC
Timers owns the temporary scheduling state used while the canister is running.
After an upgrade, the application uses its lasting state to rebuild the timers
it still needs.

A registration capability lets one owner control one named timer. The runtime
coordinates callbacks and scheduling, while one shared registry holds timer
names, states, counters, callbacks, and provider handles. Only the private
platform boundary owns one-shot IC dispatch and integrates the selected executor.

![Ownership and control flow from durable application state through the IC Timers runtime to its private platform boundary](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-runtime-ownership.svg)

The sections below describe these boundaries in implementation-level detail.

## Capability ownership

The published [crate facade](../../crates/ic-timers/src/lib.rs) exports registration,
reconciliation, callback contexts, checked schedules and inert snapshots. Its
implementation modules and platform handles are private. The Cargo `links` key
rejects competing packages with the same global-timer ownership key; complete
canister graphs must still exclude older providers that lack that key.

| Capability | Canonical owner | Local integration |
| --- | --- | --- |
| Volatile scheduling, claim/generation authority, policy arbitration and timer observations | `crates/ic-timers/src/{runtime,registry,control,schedule,snapshot}` | Public crate facade |
| Watchdog control state and request/cancel/dispatch/completion transitions | Private `crates/ic-timers/src/registry/watchdog.rs` | Parent registry retains claim admission, declaration lifetime, observations and provider-handle ownership |
| One-shot dispatch, pending queue and authenticated IC callbacks | Private `crates/ic-timers/src/platform`; `ic-platform-runtime` owns the shared executor for future polling and call-context cleanup | Only `platform/ffi.rs` owns unsafe exports/callback registration |
| Measurement arithmetic | `ic-metrics` | Timer-specific event names, recording/sample boundaries and memory extents in `snapshot/metrics.rs` |
| Durable jobs, readiness, lifecycle order and operator-health labels | Consuming application/framework | Registration claims and snapshots; no durable task database or application lifecycle exports here |
| Wasm compilation and artifact-cache custody | `ic-build`, with explicit child custody from `ic-host-process` | `crates/ic-timers-pocketic/src/build/mod.rs` selects packages, profile, features and artifact lifetime |
| Native report alias admission and atomic publication | `ic-host-fs` | The qualification driver protects its reviewed baseline and selects report schema, path and permissions |
| PocketIC client types and raw call transport | `ic-testkit::pocket_ic`; canonical identities from `ic-platform-types` | Native tests retain bounded Wire requests |
| PocketIC server selection, admission, lifecycle and bounded progress | `ic-testkit` | `scripts/dev/testkit-server.sh` selects the root-lock CLI; the native harness supplies instance configuration |
| Controlled probe codecs | Published `ic-wire::native` and Serde; Testkit owns native typed call handling | The unpublished protocol owns method identities/limits; Wire Endpoint binds canister entrypoints; the native harness selects the protocol and caller |
| Common tools, fixture dependency closure, evidence transport and release mechanics | Reviewed Shared Tooling snapshot | Local Make selections, product release/source policy and evidence identity; see [adoption boundaries](development.md) |

The [root manifest](../../Cargo.toml) owns all eight maintained packages and every
direct dependency, with one lockfile. The four unpublished packages own runtime
and size probe canisters, the native PocketIC driver, and shared probe wire
records. The protocol package selects Wire and Serde only: it carries values,
not scheduling authority. The native platform substitute is test-only; it does
not replace IC execution or prove recovery. Runtime subjects and policy cohorts
remain separate evidence selections in the [release guide](development.md).

Timers 0.40 uses Metrics 0.29 for the public `MeasurementCounter` and
`MeasurementSummary` identities, including snapshot getter results. Consumers
exchanging values with direct Metrics dependencies must select the same identity.
Metrics owns recording and overflow quality. Consumers own report DTOs, codecs,
validation, means and interval admission; recorders implement neither Serde nor
Wire. Timers 0.39 uses Metrics 0.28. Timers does not depend on a stable
metrics adapter or introduce a persisted-format migration.

## Candid and CDK dependency boundary

Registry-published `ic-platform-runtime` 0.20 is the selected execution boundary.
The current lock and source-qualified receipts are recorded in
[current status](../status/current.md).
Runtime owns the maintained IC0 bindings and executor. The root catalog disables
default Runtime features. The library selects only `executor` and `ic0`; the two
canister probes select `cdk` for entrypoint contexts,
protected calls/spawning, traps, cycle burning and stable byte I/O. The native
harness selects Testkit 0.92's async-only `pocket_ic` module for clients and
`ic-platform-types` 0.18 with `principal-external` for canonical identities.
Clients select a 60-second request budget independently of server startup limits.
The protocol package selects Wire and Serde without a direct Runtime dependency.

The private platform uses `ic_platform_runtime::ic0` for system calls,
retaining context placement, protected spawning and callback/trap cleanup.
Runtime selects its exact matching executor singleton guard. Applications must
align all executor and Endpoint suppliers with Runtime 0.20; removing or
overriding the guard does not establish compatibility. Deployed roots have one
Runtime executor owner and no standalone
`ic-cdk-executor` or `ic0` package. Native substitutes cannot
qualify those IC paths. [The call ownership boundary](../../crates/ic-timers/SAFETY.md#platform-and-application-call-ownership)
keeps application call builders and Relay above the runtime. The library's Runtime
selection remains executor/system-only, excluding Principal/Account, SHA-2, text
encoding and Candid. Metrics 0.29 has no production dependencies.
The Wasm library excludes Wire, Postcard, JSON parsing and
host tooling; Timers declares no direct codec dependency. Runtime types do not
cross the published timer API or probe wire records.

Probe records use Serde and published Wire's native codec. The shared
`test-protocol::rpc` roster owns explicit method IDs; both endpoint hooks and
native callers bind to those identities. Request limits admit at most 1 KiB,
16 KiB allocation charge, 1,024 nodes and depth 16; replies retain a 64 KiB byte
ceiling with a 1 MiB allocation charge, 16,384 nodes and depth 32. These are Wire
budgets, not renamed Candid quotas. Pre-copy guards check incoming IC size before
Runtime copies arguments. Decode failure prevents the handler; reply encoding
failure traps rather than committing a partial response. Empty RPCs still carry
a method-bound frame. Init and upgrade hooks take raw empty arguments.

Wire Endpoint 0.8 binds the probes' explicit-codec entrypoints to Runtime contexts
and protected spawning. Wire owns codec admission and failure diagnostics; the
probes retain method identities and limits. Real self-call gates and queue pressure
use raw Wire frames, retaining their IC suspension and call-failure behavior. The
production dispatch remains its authenticated eight-byte payload, without a Wire
envelope.

All unsafe callback registration remains confined to Timers' private FFI owner;
Runtime owns the executor and system bindings it calls.

The runtime fixture's named four-field checkpoint preserves the original
`(bool, bool, u64, u64)` field order and encoded bytes. It uses a
bounded Wire frame with a checked little-endian length prefix in its existing
stable-memory fixture page. Both upgrade participants use this one format;
there is no Candid reader or cross-release probe migration. Database/application
storage is outside this fixture. Stable byte I/O and cycle burning remain
Runtime's maintained adapters, originally adopted under
[Platform #19 — Timers stable-helper and cycle-burn boundary](https://github.com/dragginzgame/ic-platform/issues/19).

No workspace member declares Candid or any CDK-family package directly.
The library and both qualification canisters have no Candid or full CDK/macros
in their normal/build graphs. Probe graphs retain Runtime's generator and the
shared executor; only the canisters select `cdk`. Raw counter/page observations
use Runtime's IC0 exports. Its maintained spawning adapter preserves the
unfinished-task drop policy. Call/error/stable-error types belong to Runtime;
no such runtime types cross the published timer API or the probe wire boundary.

The native driver retains published Testkit for server admission, lifecycle and
bounded progress. Native tests borrow clients through `PocketIcInstance::instance()`
while retaining the owner through explicit awaited closure, before the server
stops. Native async tests own the Tokio runtime; client Drop performs no network
cleanup. Testkit owns PocketIC clients/DTOs
in `ic_testkit::pocket_ic`. Probe RPCs use Testkit's typed query/update helpers
with shared Wire identities/limits. Candid and management types remain upstream transport
implementation details. The retired `ic-testkit-candid` package remains absent.
The driver selects PocketIC, Types, Testkit, Wire and the probe protocol only for its
test binary. Its normal build coordinator uses Build and Host for artifacts and
process custody, plus Clap/Serde/JSON for commands and receipts. It does not link
the simulator or codecs. Build, Host and Testkit share the published family
selected by the root lockfile. The harness uses Wire 0.17 and Testkit 0.88, with
canonical Types 0.18 identities. The independent Platform PocketIC client is absent.
Managed startup supplies Testkit's required server URL and retains server custody.
Timers passes external client types across the test boundary, not owned
Principal values.
Testkit also selects Candid/management support for native provider
fixtures; PocketIC retains Candid for its simulator protocols. These are native
qualification dependencies. Independent library and canister graphs
remain Candid-free and select Runtime 0.20.
These host-only providers do not enter deployed graphs; there is no unpublished
override.
The local provider check also rejects Candid codec paths and default-codec
query/update attributes in maintained probe source.

The Candid-free runtime introduced in Platform 0.2.0 was requested in
[#31 — Support Candid-free raw and Wire canister entrypoints](https://github.com/dragginzgame/ic-platform/issues/31).
Wire Endpoint binds consumer-owned method identities and limits to Runtime hooks.
Wire retains the native Serde codec and its encoder bytes;
the current probe records need no layout or limit changes. Testkit's optional
helpers own request encoding, one call without retry, response decoding and typed
failure context in the native harness. Raw malformed-input tests remain direct
PocketIC calls. Decoding requires only Deserialize and rejects malformed primitives
without reserializing values. Maps, sets and custom visitors may normalize input;
successful decoding is not proof of byte canonicality. Current probe records use
structs, tuples, scalars and vectors, with no signature/hash or exact-frame retry
protocol. Such protocols must retain original frames and own semantic validation.
The protocol package must not acquire a Runtime dependency to
hide the canister endpoint bindings. Host metadata generation
remains native-only and is excluded from deployed probe graphs.

The published library retains three direct dependencies: Metrics owns counter
arithmetic, Runtime owns executor/system bindings, and Thiserror owns the
maintained typed error implementations. Reimplementing those capabilities to
remove their declarations would create competing owners. Runtime supplies
the lean executor/IC0 feature boundary requested in
[#40 — Keep executor/IC0-only graphs free of Principal, Serde and crypto dependencies](https://github.com/dragginzgame/ic-platform/issues/40);
Timers consumes it without bypassing Runtime or copying bindings. Metrics adds
recording without a transport, Platform or runtime dependency.

The [dependency policy](../../ci/cargo-dependency-boundaries.json) is enforced through
Shared's canonical checker in `actions-check`, alongside pin/inheritance checks.
It forbids these direct upstream declarations in every member. Metadata covers aliases and all
normal/build/dev and target declarations; enabled features and transitive
reachability for the published library are enforced by its local all-feature
normal/build graph check for Wasm and the actual native host. Probe graph
qualification remains separate. See
[deployment exclusions](development.md).

## Current runtime

Direct registration uses one inert `TimerStart` configuration. Retained starts
carry an optional initial schedule; remove-on-stop starts carry a required one.
The existing initial-registration owner decodes that configuration once,
validates requested time before allocating a claim, and returns scheduled
authority only after provider binding commits its successful-arm facts. `DeclarationLifetime`
remains a snapshot observation. See the [typed setup contract](design/0.20-typed-registration.md).

First lifecycle reconciliation allocates and initializes its claim through the
same creation owner as direct registration, under one registry borrow. It applies
provider effects before publishing authority into the owner slot. There is no
public inactive-registration/control detour. Failed creation uses the common
cleanup owner to remove its exact declaration and handles before releasing captures. Existing registrations retain their callback
and failure observations on later control errors; an unused replacement factory
is dropped outside registry borrows before applying that control.

The crate root is the only public facade. It re-exports runtime operations and
provider-neutral values directly; every implementation and value-grouping
module is private. In particular, neither `callback`, `platform`, `registry`, `schedule`,
nor `snapshot` is a consumer import path.

The private `callback` module owns public `OnceDecision` / `OnceRunResult` and
`AfterCompletionDecision` / `AfterCompletionRunResult`. Public futures cannot
return the other policy's result; only after-completion decisions contain
configured recurrence. The existing runtime erasure owner creates the consumer
future before wrapping its result conversion, preserving synchronous callback
factory timing and suspension across the original future. One private
`OrdinaryRunResult` and `OrdinaryDirective` feed the existing arbitration. There
is no public generic policy marker or conversion between result policies.

The remaining module hierarchy keeps six responsibilities separate:

![The schedule, snapshot, and control modules connect to the registry; the runtime coordinates callbacks and effects through the private platform boundary](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-architecture-modules.svg)

1. `schedule` owns validated cadence, requested schedules, post-run
   private erased directives, and checked nanosecond/deadline conversion. Explicit requests
   and ordinary directives resolve to one private `ResolvedSchedule` carrying
   deadline, requested delay and mode; Stop resolves to no successor. Pending
   commands retain that value directly, and completion does not derive control
   metadata from a directive snapshot.
2. `snapshot` owns inert public identity and observation values. Its private
   `identity`, `model`, and `metrics` children separate validation, closed
   state/outcome types, and saturating measurements. Only the registry builds
   the top-level timer and atomic inventory snapshots and initializes their
   observation aggregates. Platform reads construct inert `MemoryPageExtent`
   values directly. Those aggregates and extents expose no public construction
   or control path.
   Outcome observations project their latest classification and work count from
   one stored terminal event. Success, failure and unacknowledged timestamps and
   the expected-failure streak remain independent history.
3. `control` is the private ordinary generation/registration state machine. It
   owns checked callback generations, immediate schedule, reconciliation and
   stopping transitions. One counter owns ordinary allocation history and
   the active callback generation; state variants do not store another copy.
   Its inactive state owns its reason, its running
   state owns its pending command, and scheduled state carries neither. The
   registry owns exact running-work authorization. One ordinary request selection
   owns both deadline replacement and pending-command arbitration in `control`;
   its completion planner carries the chosen schedule and observation together.
   Completion arms its selected successor through the shared checked arming
   operation or stops with its selected reason, in the same atomic transition.
   Checked arming returns canonical `TimerControlFailure`
   values directly, without a private error conversion.
   Ordinary directive resolution also returns `TimerControlFailure` directly;
   explicit scheduling requests retain `ScheduleError` at their input boundary.
   The schedule owner classifies invalid successor proposals, while the registry
   owns terminal state and completion accounting.
   Ordinary completion selects its schedule, directive observation and
   cancellation outcome together before successor allocation. Authoritative
   reconciliation reuses its checked schedule, preserving request metadata;
   cancel/unregister discard the callback proposal. Pending ensures still validate
   the callback and preserve its directive observation even when the ensure wins.
   Resolution and allocation failures share one terminal finalization branch and
   the common removal exit; explicit consumer invariant failure still wins.
   Cancellation in the registry stops scheduled state without allocating a
   generation and queues a command for running work. Stopping with a reason
   discards the running command through state replacement.
   Ordinary Once and after-completion admission retain their policy checks before
   selecting the shared ensure/reconcile command. The arbitration engine does not
   repeat the caller's policy distinction.
   The private `registry/effect` module carries ordinary-work or Watchdog-scheduler
   wakeup variants through runtime dispatch. Only ordinary deliveries receive an
   abandonment guard, constructed before the future's first poll. A Watchdog
   dispatch stores one successor identity and
   derives its work token with the same claim and generation. Runtime application
   admits live claim ownership, policy and generation during handle installation;
   structurally malformed token pairs need no second validation path.
   The registry builds provider effects and retains cancellation policy without a
   separate cancellation or completion action. Scheduling requests return only
   an optional initial or replacement arm kind; generation and deadline remain
   in control state. Scheduling and completion share one checked arming operation.
   Within the same atomic transition, the registry builds each successful arm
   from its selected deadline and the allocated generation without a second
   registration-consistency recovery path.
4. `registry` is the provider-call-free fixed-capacity canonical owner for
   structured identities, callback closures, claim generations,
   policy-specific state, authorized application of ordinary/watchdog decisions,
   deterministic snapshots, and provider-neutral effects. It also
   owns every bound provider handle; only `runtime` invokes `platform` to
   create or clear those handles. The private `registry/entry` child groups typed
   entry payloads, provider binding/consumption and observation projection; claim
   admission, transition ordering and terminal removal stay with the registry.
   Each entry has one policy-specific payload
   containing its control and correctly typed callback. An ordinary payload
   without cadence is Once; one with cadence is AfterCompletion. Public policy
   observations are derived from that payload rather than stored separately.
   Both policy state machines own their inactive reasons; active states carry
   no inactive reason, and snapshots project the reason from the state itself.
   Watchdog awaiting-work state owns its pending command through both dispatch
   and execution. Work acceptance preserves it; leaving that state discards it,
   including when the scheduler retires an unacknowledged attempt.
   Watchdog control retains one generation counter. Each dispatch gives its
   successor and work the same fresh generation, distinguished by role. Scheduling
   requests while work is dispatched or running cannot replace that pair: exact
   scheduling waits for normal completion, and ensures coalesce or become pending
   commands. Completion either retains the successor, replaces it after expiring
   the work attempt, or stops. A later recovery scheduler also expires the old
   attempt before advancing the generation. No state repeats allocation history.
   Authorization checks generation together with state, claim and role; awaiting
   snapshots expose that generation once and project `attempt_status` directly,
   without a separate attempt wrapper. Tokens and owned handles keep their
   independent delivery stamps for stale-callback rejection and separate slots.
   Ordinary completion computes its removal-on-stop decision once from pending
   unregister and declaration lifetime; every terminal exit uses it. A successful
   arm retains its declaration. Cancellation derives immediate removal once
   from final inactive state and lifetime for both ordinary and Watchdog
   declarations; running work retains pending authority.
   A private ordinary delivery guard exists before the first provider poll and
   owns the queued token without another identity clone. Normal return disarms it.
   A dropped confirmed delivery retires only its exact scheduled/running generation,
   records Unacknowledged and leaves retained state Abandoned. Transient lifetime
   or pending unregister removes the entry. The guard performs no provider calls
   and releases removed callbacks outside the registry borrow. Explicit cancellation,
   replacement and unconfirmed installation failure remain distinct from abandonment.
   The native drop and PocketIC trap subjects passed in the 0.13.2 hosted
   release gates; [source and host scope](development.md) remain
   separate from Watchdog recovery and pending source qualification.
   Watchdog completion decides removal once after selecting its final state:
   inactive declarations follow their lifetime and pending unregister command,
   while retained or replaced successors keep their declaration.
   Ordinary request, Watchdog request and Watchdog scheduler failures decide
   terminal removal through the entry already selected and authorized by their
   transition. The shared entry predicate uses that transition's failure and
   declaration lifetime; no second lookup reconstructs the decision. The registry's
   existing removal exit transfers the removed entry into its transition.
   Public control and Watchdog dispatch/completion detach only the handles their
   effects will clear, after the registry transition. Removed entries transfer
   all handles into cleanup. Unaffected handles stay bound; no restoration step
   can fail or reinstall them. Runtime clears detached handles before arming
   replacements or releasing removed captures outside registry access.
   Watchdog work completion retains its trap/rollback rule; see the
   [capture ownership contract](design/callback-delivery-ownership.md#capture-removal-and-coalesced-requests).
   Watchdog completion resolves pending-command precedence, stop reason and
   schedule metadata together before applying the selected action. Exact requests
   retain their validated scheduling origin; invariant failure still wins.
   Watchdog cancellation selects the handles to clear in one state match. An
   immediate stop uses that selection for state replacement, cancellation
   accounting and provider cleanup; running work keeps its pending command, and
   inactive cancellation preserves the existing reason. No generation is allocated.
   Ordinary schedule requests share counter, request-metadata and coalescing
   updates. Recurring ensure submits an existing scheduled deadline unchanged;
   it calculates a cadence deadline only for inactive or running declarations.
   The request owner commits mode once for successful arms or exact reconciliation;
   coalesced ensures retain it, while equal exact requests update it.
   Watchdog requests commit scheduling mode once after a successful initial or
   replacement arm; coalesced, pending and failed arms preserve it while recording
   requested delay. A cadence deadline is calculated only for an inactive Watchdog.
   The common Watchdog request exit records coalescing for a no-effect transition
   without failure, keeping exhausted initial arms distinct from coalesced demand.
   Owned provider roles follow entry policy and handle slot rather than a copied
   field. Installation and consumption reject policy/role mismatches before using
   a slot. Cleanup carries only the selected linear handles; it does not rebuild
   callback tokens for a restoration path.
5. `platform` owns pending one-shot tasks, authenticated self-call dispatch and
   IC system facts. Its handle is linear and it owns no recurrence policy.
   The bounded queue removes cancelled records immediately. The FFI module alone
   registers callbacks and exports the global timer/internal dispatch entrypoints;
   see the [backend design](design/0.26-owned-timer-backend.md).
   Rearm validates the returned previous IC timer value against its last write;
   an expired one-shot may legitimately return zero. Conflicting observations
   trap with IC message rollback, within the detection limits in [SAFETY](../../crates/ic-timers/SAFETY.md).
   Page reads return the same inert extent representation used in snapshots,
   without a parallel platform value or a registry conversion.
6. `runtime` owns the one canister-local registry static, erases consumer
   callbacks for registry storage, exposes registration claims, applies
   provider effects, and drives live `Once`/`AfterCompletion` dispatch and the
   two-role watchdog protocol. Private `capability`, `registration`, `dispatch`
   and `effects` modules separate the public handles, creation/reconciliation,
   callback delivery and provider binding; the parent retains shared authority
   checks and the registry static. Initial and successor scheduler futures share the
   ordinary pre-poll delivery guard. Confirmed destruction records a lost
   scheduler in canonical control; owner reconciliation rearms it without
   replacing still-queued work. Maintained PocketIC round-pressure stop/restart
   subjects cover initial and successor scheduler abandonment with all three
   owner reconciliation modes. Scheduler-body normal return remains the protocol
   assumption; these subjects do not qualify arbitrary scheduler traps.
   Its three policy-specific delegated work
   contexts store their exact private callback token directly. Their control
   methods use the shared claim-transition boundary, which validates running-work
   authority before the registry operation and subsequent handle detachment.
   Each private token contains its registration claim, callback generation and
   role. Context control and callback cleanup borrow that claim instead of
   reconstructing it. Owned delivery tokens remain cloneable; registration
   capabilities remain non-clone. Borrowed claims still require the operation's
   exact running-work or provider-ownership validation.
   Claim-originated effect failures retire the declaration instead of leaving
   registry state scheduled without a provider handle. Lifecycle verification
   checks the exact claim and immutable declaration metadata directly rather
   than reconstructing authority from a snapshot. Rejected operations leave
   provider handles bound. One effect finalizer clears the selected detached
   capabilities and applies arms, retiring the exact claim after unexpected
   provider failure. Callback finalization remains separate to preserve Watchdog
   rollback rules.
   One consuming binding operation installs a raw provider handle or clears it
   on rejection after releasing the registry borrow. Arm callers receive only
   the typed error; Watchdog dispatch separately clears its installed
   successor if later work binding fails. Final handle installation and successful
   arm accounting share one registry borrow and one role/generation/state check.
   A staged Watchdog successor does not commit arm counters; successful work
   installation commits both Watchdog dispatch counters. Retaining an installed
   successor performs no new binding or arm accounting.
   Effect application consumes one complete registry effect; its shape is
   validated before cleanup or platform calls. Its matched branches perform
   binding directly without passing the effect through another variant match or
   duplicating token and delay arguments beside it. One initial/replacement
   arm kind flows from ordinary control through provider binding, and the
   non-empty set of callbacks to clear is also a closed value rather than
   independent booleans. The entry-local exact-claim predicate is shared by
   callback acceptance and fired-handle consumption. Measurements and provider
   installation retain canonical claim lookup with their own error semantics;
   identity reuse cannot transfer handle authority to a stale callback.
   Work acceptance checks the exact claim, policy, work role, generation and
   eligible state, then transitions to running state and returns that entry's
   correctly typed callback. Dispatch releases the registry borrow before
   invoking consumer work; there is no separate callback lookup or second borrow.
   Ordinary and Watchdog work delivery consume the matching fired handle through
   that selected entry before acceptance, without a separate registry consumption
   pass. Scheduler delivery consumes its handle before the Watchdog transition;
   its effect then selects cleanup from the surviving or removed declaration.
   Duplicate or stale acceptance returns no callback and keeps its existing stale
   accounting. Installation owns the required role, generation and state checks;
   pure transition tests model successful application through those same predicates.
   Delegated control and both work-completion paths share one entry-local
   running-work predicate for the exact claim, work role, generation and running
   state. Completion checks it before any mutation; provider binding and completed
   measurements retain their distinct validation boundaries.

Absolute deadlines remain authoritative in policy control state and its snapshot
projection. Provider arm effects carry only the resolved delay, callback authority
and arm kind; they do not store another deadline representation.

The registry and runtime transition functions remain cohesive even where they
are long: each audited function owns one atomic transition or one provider
binding path. Their tests live in directory-local `tests.rs` files so test
volume does not obscure production flow.

## Canonical runtime

One canister-local runtime owns the bounded registry and its structured
identity (`owner`, `subsystem`, `name`). Framework and application schedulers
must contribute to that same live registry rather than building private
inventories. All three policies, lifecycle reconstruction, measurements, and
the real-canister watchdog evidence matrix are implemented.

The runtime provides:

- one-shot, after-completion interval, and pre-armed watchdog policies;
- serial execution with overdue work coalesced into one pending run;
- cancellation requested safely by the running callback;
- synchronous lifecycle restoration followed by deferred application work;
- registration-lifetime counters, last outcomes, deadlines, instruction consumption,
  and bounded memory-page extent/growth observations;
- exact-claim observation of armed provider wake-up ownership without a
  snapshot-derived control path;
- bounded identity components and allocation-conscious hot paths;
- one atomic inventory snapshot carrying the runtime epoch even when empty;
  and
- portable snapshot DTOs so Canic, IcyDB, and standalone canisters can
  expose the same operator view.

Diagnostic event storage and arithmetic use published Metrics `MeasurementCounter`
values. Timers retains named events and registration/reset identity; it has no
parallel integer counter arithmetic. Instruction observations already use Metrics
summaries. The 0.29 getter cut requires downstream projections to preserve wide
totals and actual-overflow quality, or use checked narrowing. Page extent sampling
and functional failure streaks remain timer-specific. Probe wire records only
project values; the protocol package depends on Wire and Serde alone.

Operator-health labels belong to consumers. Snapshots expose runtime state,
scheduling mode and callback outcomes for a single consumer-owned projection.

Metrics are collected once at the registry boundary. Consumers may adapt
the atomic inventory to their own status endpoint or metrics encoder without
wrapping every callback independently.

The canonical snapshot is designed as a semantic superset of Canic's current
timer status, scheduling counters, and instruction metrics. In particular,
callback starts and completions remain separate because traps and instruction
exhaustion can prevent post-run measurement, and consecutive expected failures
remain cheap functional state because recovery decisions consume them. See the
[observability and Canic parity contract](design/observability.md).

## Lifecycle and observations

The registry holds at most 64 declarations. Ordinary work may be async;
Watchdog work is synchronous so its separate work message cannot cross an
`await`. Provider handles are private, linear capabilities owned by the registry.

The live snapshot omits elapsed IC time: message time cannot truthfully measure
synchronous callback duration. It distinguishes scheduler and work instructions,
plus start/end and maximum-growth Wasm/stable memory-page observations. Page
extent is not exact allocator liveness. A dispatched Watchdog attempt without a
committed completion is *unacknowledged*, not definitely trapped, and has no
fabricated measurement.

Lifecycle reconciliation always installs retained declarations so a
caller-owned `Option<Registration>` cannot outlive a remove-on-stop canonical
entry. Transient `RemoveWhenStopped` timers use direct registration and are
recreated explicitly by their owner if later desired; cancellation removes a
transient declaration even before its first schedule. One private lifecycle
seam installs and verifies the exact retained claim for all three policies.

## Safety boundary

An after-completion interval is not a watchdog: if its callback traps or runs
out of instructions, code after the callback cannot arm a successor. A
watchdog policy commits the successor before invoking fallible work while
preventing concurrent logical execution. Maintained
[PocketIC subjects](../../crates/ic-timers-pocketic/tests/runtime/tests/mod.rs) exercise
trap, actual instruction exhaustion, upgrade reconstruction, external-ingress
rejection, independent timers, insufficient cycles followed by top-up,
an armed Watchdog and its committed scheduler/work gap surviving a genuine freeze
and top-up without repair calls,
retirement without replay when ordinary async work reports a failed self-call
while frozen, stop/resume, overdue coalescing and cancellation in the scheduler/work gap.
[SAFETY.md](../../crates/ic-timers/SAFETY.md) owns the guarantees and evidence limits; historical
server versions and receipts do not qualify a new source/artifact combination.

After normal progress, a Watchdog may replace that exact cadence successor with
a deadline of now. The replacement remains a scheduler callback in a later
replicated message; that scheduler again pre-arms cadence safety before it
queues work. Initial immediate reconciliation and running-work requests use the
same declaration, claim, generation, pending-command, and provider-handle path.
See the [immediate continuation design](design/immediate-watchdog-continuation.md).

Exact Watchdog reconciliation can also move a sleeping deadline
earlier or later. Dispatched or running work retains the committed cadence
recovery successor until normal completion applies the pending schedule or
`WatchdogDecision::ScheduleAt`. Snapshots carry a registration identity so
consumers can detect counter replacement within one runtime epoch. See the
[continuity and deadline contract](design/0.8-registration-continuity-and-deadlines.md).

## Consumer integration

For current Canic application integration, use Canic's
[native timer guide](https://github.com/dragginzgame/canic/blob/main/docs/features/runtime/native-timers.md).
Applications register directly with `ic-timers`; Canic owns framework lifecycle,
durable business recovery, snapshot coordination and observation endpoints.
The dated adoption records below qualify only their recorded sources and graphs.

Read-only inspection on 2026-10-18 found one registry `ic-timers` 0.27.1 package
in each available Canic, IcyDB and Jobs lockfile, with no `ic-cdk-timers` package.
These are source/graph observations, not downstream build or deployment results;
dirty sibling work was preserved and no sibling suite was run for this review.

- Canic's [`workflow/runtime/timer`](https://github.com/dragginzgame/canic/blob/5583496bfa95069809249e8e5d16c77f1ffd744f/crates/canic-core/src/workflow/runtime/timer/mod.rs)
  integrates native claims and framework authority/suspension policy, including
  a recovery Watchdog. Its pending
  `crates/canic-core/src/ops/runtime/timer_status/mod.rs` owns operator-health
  projection in the inspected worktree; that extraction is not yet in its base
  commit. Timers must not absorb those application decisions.
- IcyDB's SQL, lifecycle and startup-timer test canisters import Timers. Its
  [manifest](https://github.com/dragginzgame/icydb/blob/14051952f449ab0f730c97512a42011b3e06061e/Cargo.toml)
  also selects a pinned `canic-icydb` adapter; that adapter retains database
  readiness and lifecycle policy. Available source still contains the standalone
  primitive timer probe. Its prepared retirement is separate from the already
  pending mixed-policy qualification here; no sibling deletion is claimed.
- Jobs' [`apps/job-consumer`](https://github.com/dragginzgame/ic-jobs/blob/312ff2486f7d15d60be92a14c58cc2bbd45e3d05/apps/ic-jobs-test-consumer/src/canister.rs)
  uses a Watchdog to dispatch its consumer-owned queue. The published Jobs
  library owns job transitions, not a timer registry or global provider.

Composed canisters must qualify one resolved Timers package and global-timer
owner, synchronous reconstruction from durable authority, and progress/recovery
for each participating policy against their actual linked artifact. The backend
bounds 130 pending records and 128 outstanding dispatches; these do not bound
executor or application memory.

A consumer may retain a bounded custody collection of opaque registration
claims. It makes claims enumerable without duplicating scheduling state,
deadlines, generations, counters, pending commands or provider handles. Callback
contexts delegate only policy-valid operations for the exact running generation
and role; they expire at completion and cannot substitute for stored claims.

The [Canic](adoption/canic.md), [IcyDB](adoption/icydb.md) and
[Toko Miner](adoption/toko-miner.md) adoption records preserve older exact-version
subjects. Their API tables and historical provider limits are not current
integration instructions. The numbered design records preserve decisions that
preceded the owned backend; current behavior is defined by the facade,
implementation and maintained tests above.

Provider-effect application lives in the private `runtime/effects.rs` owner.
The registry selects handles for cleanup after its transition, leaving unaffected
capabilities installed. Runtime clears those detached handles before applying
arms, then releases removed callback captures outside registry borrows. Policy arbitration remains in the registry's atomic transitions; the
module boundary adds no provider, public interface or compatibility layer.
