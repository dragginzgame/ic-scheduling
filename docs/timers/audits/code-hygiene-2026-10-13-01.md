# IC Timers Code Hygiene Audit

Date: 2026-10-13. Trigger: adopt the new Metrics patch and repeat the audit.

The reviewed timer paths contain no newly demonstrated correctness or authority
defect. The mature runtime does not justify another API cut from this review.
The prepared verifier correction needs execution evidence; two probe import
paths need a small convention correction. Provider queue retention remains a
documented limit rather than a new scheduler obligation.

**Source review: PASS WITH FINDINGS. Fresh complete qualification: BLOCKED.**
Library compilation and source inspection do not establish that the pending
probe/tooling changes or current dependency graph pass native/PocketIC gates.
Tests, lint, probe builds and release execution remain maintainer-owned under
[AGENTS.md](../../../AGENTS.md).

## Scope and identities

Method: [code hygiene](../../../audits/code-hygiene.md) and
[IC application addendum](../../../audits/ic-canister-applications.md), from reviewed
Shared `626fc2bd3125195520fb870b92abfec11b01454d` (0.10.2). The
[local overlay](code-hygiene.md) has SHA-256
`875a81861e6e6c47c1896dfdfa1baa342221996bcde5716de80a03678ea42f0f`;
AGENTS has SHA-256
`fb003c730623685ce622932dfafcbdb2d120a1c9e1e883665de3650a57519210`.
[Method digests](evidence/code-hygiene-2026-10-13-01/method-inputs.sha256)
retain the exact questions used for this run.

Consumer HEAD is released 0.22.1,
`52a0fce4082b087342f5b04db717e94bf5448193`. The reviewed working tree includes
the pending 0.22.2 verifier/publication/Candid-helper cleanup and incoming lock
changes. The root lock already selected Metrics 0.9.1 and Host
artifacts/fs/process 0.17.6 at entry; this task preserves it byte-for-byte.
Only the Metrics requirement changes from `0.9` to `0.9.1`.

Metrics archive SHA-256 is
`59af3d107a6b06bee0a31b56d9ec014ed77b8f2cd69c96b419561efd57aee812`;
its VCS receipt identifies published commit
`804ca4727b74d641bbe3be9155e20de914eb3ad9`. Authored arithmetic-package source
is identical to released 0.9.0 at
`fe4d14f2a24138794878a17a354885a1e7222487`. The dependency-free arithmetic
graph, public summary type, overflow semantics and supported compiler floor
are unchanged. Stable-store changes in 0.9.1 are outside Timers' dependency path.

The [source roster](evidence/code-hygiene-2026-10-13-01/source-inputs.sha256)
has SHA-256 `0c83cebba471e7a717b826d6cf16a008eb98c9326b0ed6daa0c43cfe6e76b5e1`.
It binds the reviewed runtime, tests, probes, manifests, lock, workflows and
tooling inputs. Before-images and the reviewed diff remain at
`/tmp/ic-timers-metrics091-audit.dVRqqwVF/`; generated audit evidence is excluded
from authored change counts.

Scope includes all five maintained packages, the public facade, production and
native platform implementations, relevant negative-test bodies, probe wire
records, lifecycle hooks, provider binding and local evidence/publication callers.
The pinned CDK provider is inspected as an upstream dependency, not modified.
Downstream application composition, security/vulnerability freshness, real
network performance and a full release-runner audit are excluded. Missing
current native/PocketIC qualification remains a gap, not an exclusion.

Earlier August hygiene reports used the retired local checklist. Aggregate
comparison is **N/A (method change)**. The provider boundary, ordinary completion
clock and separately committed Watchdog successor remain comparable semantic
anchors, with their original execution evidence retaining its original scope.

## Execution paths and invariants

| Subject | Actual owner and inspected behavior |
| --- | --- |
| Creation | `runtime::register_initial` validates the requested schedule before allocating authority, applies initial reconciliation and retires the exact new claim on error. Callback captures outlive provider cleanup. |
| Identity | `registry::register` rejects duplicates/capacity and uses checked `u64` claim allocation. Callback tokens carry identity, claim generation, callback generation and policy role. |
| Scheduling | `TimerSchedule::resolve` checks duration encoding and addition. Logical deadlines are absolute; provider arms use non-negative relative delay. Past absolute deadlines become immediate delivery requests. |
| Ordinary requests | `TimerControl::request_deadline` retains an existing earlier/equal ensure deadline; exact reconciliation can move it either way. No-op transitions leave handles installed. |
| Firing | `begin_ordinary` consumes only the matching owned slot, validates the token and moves the exact generation to Running before exposing the callback. |
| Callback execution | `dispatch_ordinary` releases the registry and callback-factory borrows before awaiting. Consumers can mutate other timers and issue arbitrated commands against running work. |
| Ordinary completion | `complete_ordinary` arbitrates commands before validating a discarded proposal. Invariant failure stops; exact reconciliation wins; ensure selects the earlier successor; unregister remains terminal. |
| Recurrence | `RecurAfterCompletion` adds cadence to the actual completion timestamp. There is no historical-tick replay or overlap of one registration's ordinary work. |
| Watchdog dispatch | `begin_watchdog_scheduler` allocates one checked generation and derives successor from actual dispatch time. `apply_effect` arms that successor, queues zero-delay work and confirms both effects before returning. |
| Watchdog failure | Work is synchronous. Completion/binding invariant failures trap its message so rollback preserves the previously committed scheduler successor. Scheduler failure itself is outside the recovery guarantee. |
| Cancellation | Scheduled handles are cleared; cancellation/unregistration during running ordinary work becomes a pending command and does not terminate its future. Retained callback factories remain until unregister; transient stopped entries are removed. |
| Abandonment | `OrdinaryDelivery` is constructed before first polling. Drop retires only confirmed current authority without provider calls; retained entries become Abandoned, and no completion or cost sample is fabricated. |
| Destruction | `RegistryTransition::removed` carries captures beyond the registry borrow and provider cleanup. Runtime drops it outside those borrows. Consumer Drop code must remain bounded and safe in CDK cleanup. |
| Observation | Snapshots have private top-level fields and registry-only constructors. Measurements use the exact registration claim; identity reuse cannot redirect an old sample into a replacement. |
| Lifecycle | `initialize_runtime` is explicit and idempotent within one heap. The library owns no stable state or lifecycle export. The runtime probe reconstructs retained intent in its existing init/post-upgrade owner. |
| Determinism | BTreeMap orders inventory by identity. The provider heap orders enqueue records by deadline/insertion counter; this does not make inter-message callback completion order a Timers public contract. |

Concrete anchors: [creation](../../../crates/ic-timers/src/runtime/mod.rs#L658),
[registry allocation](../../../crates/ic-timers/src/registry/mod.rs#L731),
[ordinary completion](../../../crates/ic-timers/src/registry/mod.rs#L1197),
[Watchdog scheduler](../../../crates/ic-timers/src/registry/mod.rs#L1353),
[effect binding](../../../crates/ic-timers/src/runtime/mod.rs#L1024),
[abandonment](../../../crates/ic-timers/src/runtime/mod.rs#L1232),
[observation](../../../crates/ic-timers/src/registry/mod.rs#L1637) and
[probe lifecycle](https://github.com/dragginzgame/ic-scheduling/blob/bb67e6f23ec50a95cf1b58f675cd5a8d8c60f4ff/crates/ic-timers-runtime-probe/src/lib.rs#L31).

### Timing and failure interpretation

For ordinary cadence T, the next logical deadline is the completion message's
IC timestamp plus T. If an async callback spans C units of IC time across messages,
each recurrence includes C plus T and delivery latency. Synchronous callback
execution does not advance the timestamp within its message: there is no general
N(C+T) wall-clock formula or guaranteed T-length physical pause after Rust returns.
Expensive synchronous work can make the newly armed deadline overdue by the next
message. This is an IC clock boundary, not historical-tick replay or same-timer
overlap. A ten-minute or 24-hour pause produces one late invocation followed by
the selected successor. Watchdog derives its recovery successor from scheduler
time; delayed scheduler delivery shifts its phase and coalesces missed periods.

Zero relative delay requests a later provider message. Cadence zero is rejected;
one nanosecond is representable but is not an execution-precision promise.
Duration encoding and timestamp addition are checked at `u64` boundaries.
The IC specifies constant time within an entry point and monotonic canister
time across upgrades, which makes synchronous resolution/provider re-reading
coherent. The native fake can deliberately set arbitrary time and is not proof
of that platform contract. [IC system-time specification](https://docs.internetcomputer.org/references/ic-interface-spec/canister-interface/#system-time).

Rust callback unwinding is not a substitute for IC message rollback. Maintained
PocketIC subjects cover real await, pre-await and continuation traps, synchronous
Watchdog trap/exhaustion, upgrade reconstruction and capture release. The native
fake models ready/pending futures and exact token transitions; it does not model
transaction rollback, provider self-call limits or cancelled heap records.

Non-wrapping wrapper counters require no new machinery: even one billion
increments per second takes about 584 years to exhaust `u64`. Exhaustion returns
typed registration/control errors; cancellation does not need another generation.
This observation does not prove that upstream provider keys never wrap.

### Cost and ownership decisions

The registry is capped at 64 entries; 1,000 or 100,000 live wrapper declarations
are rejected workloads. BTreeMap lookup/insertion/removal are logarithmic and
inventory materialization is linear in at most 64 entries. The registry does not
search for the next global deadline: the provider owns that queue. A timer wheel,
second heap or bounded dispatcher in this wrapper has no demonstrated benefit.

The pinned provider already avoids a global timer write when insertion does not
advance its earliest deadline. Wrapper coalescing avoids provider operations for
duplicate/later ensure requests and identical exact reconciliation. Cancellation
removes the task but retains its deadline record; repeated replacement can grow
the provider heap despite bounded live wrapper handles. This known limit is
explicit in [SAFETY.md](../../../crates/ic-timers/SAFETY.md#L231), with a real-Wasm churn subject.
The native substitute intentionally releases cancelled tasks immediately.

Consumer work runs in separate messages. Immediate continuation does not invoke
consumer work recursively. The provider also has a global outstanding-dispatch
limit and a global-timer instruction guard; these are provider mechanisms, not
fairness or delivery guarantees supplied by Timers. Other direct provider users
can compete with this runtime. No stricter fairness API is justified here.

Closure/future retention is intentional: a retained declaration owns its callback
factory, and running async work remains alive until completion or actual executor
abandonment. A Drop-cancelling registration handle would conflate custody with
runtime lifetime and could cancel work when an owner temporarily moves a value.
The explicit non-clone capability remains the simpler contract.

Metrics owns instruction-summary arithmetic. Event counters and page observations
retain distinct documented units and conservative-cap semantics; replacing them
with generic arithmetic or moving timer lifecycle metadata upstream would remove
meaning rather than duplication. No stable Metrics adapter, cron system, retry
framework, persistence layer or additional async runtime belongs in this change.

## Findings and dispositions

### MEDIUM P2 Prepared corrections still need execution evidence

**Problem/evidence:** [#53](https://github.com/dragginzgame/ic-timers/issues/53)
demonstrates the released verifier's default/qualified IC catalog mismatch.
The current correction chooses the qualified catalog and tests the actual CLI
with differing defaults, wrong version/digest and missing qualified input. Those
assertions have not run for the prepared source. Publication fixture and typed
probe calls also have pending focused qualification.

**Why/change:** correct source cannot establish deployed artifact admission or
native probe behavior. Run the maintained downloaded-evidence/publication fixtures
and prepared native probe compile/lint selections when authorized; preserve the
normal owner-operated recovery/cohort gates. Close #53 from its actual regression
evidence and reconcile [#51](https://github.com/dragginzgame/ic-timers/issues/51)/
[#52](https://github.com/dragginzgame/ic-timers/issues/52) against their required
native hosts. Do not weaken acceptance merely to reduce the issue count.

**Impact:** compatible contributor work; no timer API change or production
Wasm/instruction/heap saving. Implementation small, risk low. A valid qualified
archive must pass the real CLI while wrong and unavailable selections fail;
publication must preserve index/refs and reject dirty/unborn source. Owners are
the existing evidence verifier, publication fixtures and PocketIC harness.

### LOW P3 Two probe paths still use core in std packages

**Problem/evidence:** memory observation uses `core::arch::wasm32` in
[size probe](https://github.com/dragginzgame/ic-scheduling/blob/bb67e6f23ec50a95cf1b58f675cd5a8d8c60f4ff/crates/ic-timers-size-probe/src/lib.rs#L181) and
[churn probe](https://github.com/dragginzgame/ic-scheduling/blob/bb67e6f23ec50a95cf1b58f675cd5a8d8c60f4ff/crates/ic-timers-runtime-probe/src/churn/mod.rs#L80).
Both packages use std; the adopted hygiene convention selects its std re-export.

**Change/impact:** use `std::arch::wasm32` when these probe files next receive an
authorized cleanup. Two-line mechanical correction, trivial implementation and
low risk; no public compatibility impact or expected instruction/heap/Wasm
change. Probe Wasm compilation on the advertised floor proves the import is
available. This is a convention issue, not a runtime bug or reason for a release.

### Documentation corrected during the bounded adoption

README still named the prepared 0.22 API and two separate 30-second startup
deadlines. Current Testkit harness source uses one total 60-second deadline.
The current API/example pin and startup description now follow the delivered
0.22.1 source. This completes existing dependency/harness documentation
propagation and adds no README freshness release gate. No open work is retained
for this corrected LOW observation.

## Checks and comparison limits

- Full locked offline Cargo metadata resolves one Metrics 0.9.1 identity and five
  workspace members. The published-library tree contains no Host or Testkit.
- Prepared Linux x86_64 Rust 1.99 native library/all-test-target compilation passes
  in 2.38 seconds; Rust 1.88 equivalent compilation passes in 2.84 seconds.
- Rust 1.88 Wasm library compilation passes in 2.52 seconds. All compile commands
  use `--locked --offline` and `RUSTUP_AUTO_INSTALL=0`.
- Root, supplemental and helper Shared integrity checks pass at 0.10.2 with
  70/37/11 payloads. Diff checks pass; the incoming lock, real index and refs are
  preserved. [Compile logs](evidence/code-hygiene-2026-10-13-01/native-check.log),
  [MSRV native](evidence/code-hygiene-2026-10-13-01/msrv-native-check.log),
  [MSRV Wasm](evidence/code-hygiene-2026-10-13-01/msrv-wasm-check.log) and
  [library tree](evidence/code-hygiene-2026-10-13-01/library-tree.txt) retain scope.
- Tests, doctests, Clippy, probe builds, PocketIC execution, setup and release
  commands did not run. The existing source-bound 0.22.1 Linux/MSRV hosted
  results do not qualify this dirty graph or replace supported native macOS gates.
- No optimized artifacts or instruction measurements were produced. Identical
  arithmetic source suggests no arithmetic-code saving; it is not byte-identity
  evidence for a linked application. Build-policy experiments remain with IC Build.

## Are we actually done?

**C. The crate is mature and only minor ergonomic/test improvements remain**
within the inspected runtime design. Generation authority, callback mutation,
checked scheduling, async abandonment and separate Watchdog commit boundaries
have clear owners and maintained negative subjects. No new required API or
architectural replacement survived this review. Current execution qualification
remains incomplete, and provider limits remain operational obligations.

## Best remaining work

1. Qualify and finish #53's prepared CLI repair using its existing suite.
2. Complete native-host acceptance for #51/#52 after delivered caller agreement.
3. Correct the two probe std import paths during the next authorized probe edit.

## One next release

Keep the single prepared 0.22.2 patch: Metrics 0.9.1 declaration, verifier
selection repair, Shared clean-source reuse and Testkit Candid-call consolidation.
The exact scope and evidence owner are in
[releasing.md](../evidence/release-history.md#prepared-0222-contributor-cleanup).
No new runtime feature, persistence model, optimizer policy or hard cut is
justified by this audit. Expected runtime cost change from the unchanged Metrics
arithmetic is none; linked Wasm/instruction deltas are unmeasured. Acceptance
requires the focused fixtures and complete maintainer-operated native/PocketIC
gate, without changing versions or running releases in this audit.

## Future research

**Immutable identity sharing:** callback-token creation and cloning copy three
boxed identity components. A private Rc identity could reduce allocation/copy
work without a new public type. Measure the existing Once/after-completion/
Watchdog cohorts at equal identity lengths before implementation; retain the
current model unless instruction/heap savings justify reference-counting cost.
This is an allocation hypothesis, not a measured Wasm improvement or release item.

**Provider key reuse:** SlotMap 1.1.1 documents possible key aliasing after
2^31 deletion/insertion cycles in one slot. The provider retains cancelled
deadline records containing those keys, while the wrapper's acceptance check
does not independently compare current time with its deadline. A potential early
delivery after provider-key wrap deserves an isolated boundary reproduction,
including whether an old record can coexist with the reused slot under the
actual scheduler. At a hypothetical 1,000 same-slot reuses/second, wrap is about
24.9 days; this is not a demonstrated IC workload rate. No such failure was
executed here. Keep it out of this patch and do not claim wrapper u64 counters
prove upstream key uniqueness. [SlotMap's documented bound](https://docs.rs/slotmap/1.1.1/slotmap/#performance-characteristics-and-implementation-details).

## Task change metrics

These are task-attributed changes over the incoming dirty tree; prior cleanup,
lock changes and generated evidence are excluded. They are not audit scores.

| Measure | Added / removed / net |
| --- | --- |
| Rust physical lines | +0 / -0 / 0 |
| Type declarations | 0 added, 0 deleted, 0 moved, 0 renamed |
| Type declaration physical lines | +0 / -0 / 0 |
| Documentation physical lines | +337 / -18 / +319 |

Cargo declaration lines change +1/-1/0. No runtime symbols are changed.
