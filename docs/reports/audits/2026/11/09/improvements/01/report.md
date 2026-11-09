# Scheduling improvement audit

2026-11-09, run 01. **PASS WITH FINDINGS** within the structural review scope:
three LOW maintenance findings, without a demonstrated product contract failure.
This is source inspection, not fresh runtime, security or release qualification.

## Scope and identities

Methods: `audits/complexity-and-technical-debt.md` and
`audits/flow-convergence-and-duplication.md`, under `audits/README.md` and the
local `AGENTS.md`. Reviewed snapshot: Toko Tooling 0.36.0, registry archive
`de0dd05b6437061622a76942baa25d40d2181497e7ba66d9fd809132a3082936`.
Consumer HEAD: `25c4167f48017d43fb4ec9dc31afa89f2dc14323`.
AGENTS SHA-256: `aa3fd54f72dd911293ef79c0ab900a076ad0e9e29b590127f16dcff7732ad76d`.
Cargo.lock SHA-256: `9e42f3a985265b1ca87271795177f08b3f3fee44bb7d05aa184dbbd1080cfb04`.

The working tree includes the pending Cargo-owned Tooling migration, supplier
adoption, Testkit instance annotations and memory-growth aggregation cleanup.
Those changes are inputs to this review, not changes made by it. The preceding
report used Tooling 0.31.11 and older inputs; no aggregate comparison or new LOC,
timing or footprint claim is made. Previously removed demo/CLI code and completed
clock/cleanup consolidation are not new recommendations.

Inventory covers the eight workspace packages, their manifests and consumer-owned
Make/release/qualification callers. Traces concentrate on control ownership,
publication lint selection, queue restore/commit and repeated fixture setup.
Shared snapshot implementation remains producer-owned. Siblings, Canic adoption,
new persistence/security baselines, full CI, installed recovery, performance and
supported-host qualification are outside this review. No separate structural
overlay exists; the local authority and scope are stated here.

## State and flow map

| Axis / flow | Owner and retained boundary |
| --- | --- |
| Once / after-completion / Watchdog | Typed registration selects EntryKind; pure ordinary and Watchdog controls retain their different completion contracts. |
| Running commands and terminal removal | Controls arbitrate commands; registry admits claims and owns declaration lifetime; runtime applies provider effects after releasing borrows. |
| Callback claim / generation / role | Registry validates all three; staged scheduler binding and committed work binding have distinct accounting obligations. |
| Recorded Jobs and external effects | Bounded decoding enters Job::restore; the consumer commits before returning delivery intent; uncertain effects require explicit disposition. |
| Jobs / Timers release scope | Qualified source/group plan selects obligations; publication separately admits source, tags and registry availability. |
| Native / installed qualification | Native fixtures model selected behavior; actual IC rollback and recovery remain installed obligations. |

## Ranked findings

### 1. LOW — ordinary unregistration bypasses its pure control owner

Evidence: `crates/ic-timers/src/registry/mod.rs:648` destructures
`TimerControl.registration` and directly writes `OrdinaryPending::Unregister`.
In contrast, `crates/ic-timers/src/registry/watchdog.rs:164` already owns the same
kind of deferred-request decision through `defer_unregister`. Ordinary request,
stop and completion arbitration otherwise live in `control.rs`.

Consequence: an ordinary running-state change needs coordinated semantic edits
in both the registry and its control owner. Owner: ordinary control/registry.
Disposition: localize when this boundary is next repaired. Move the ordinary
deferred-unregister decision into TimerControl, following the existing Watchdog
shape; leave claim admission, immediate removal and provider cleanup in registry.
No new public API, state, generic control engine or persisted representation is
needed. Benefit is clearer ownership and a smaller registry branch, not a large
code cut. Validate running versus inactive/scheduled unregistration, pending
command precedence, suspended completion and abandonment.

### 2. LOW — publication discards the qualified lint scope

Evidence: `scripts/release/publish.sh:28` always dispatches aggregate `clippy`;
`Makefile:333` does likewise for `publish-check`. Meanwhile the group plan and
`clippy-jobs` / `clippy-timers` leaf recipes already represent distinct obligations.
The Jobs lint recipe retains the combined consumer; it is not just the core crate.

Consequence: proven Jobs-only validation cannot carry its selected lint scope
through publication, so unrelated standalone Timers lint remains coupled to it.
This is conservative extra work, not an unsafe upload or a measured runtime cost.
Owner: consumer publication/validation adapter. Disposition: consolidate when
publication admission is next qualified. Use the admitted plan to select the
necessary existing leaves; retain complete lint for shared, Timers, uncertain or
missing proof. A group name alone must not authorize narrowing or evidence reuse.
Keep lint before effects and retain fresh source/tag/registry/archive admission.
Validate actual dry-run/upload callers, group/workspace fallback, lint failure
and input drift with registry effects substituted. Do not weaken release gates.

### 3. LOW — successful Watchdog fixture construction still repeats

Evidence: identical inactive retained registrations using `test_cadence(5)` and
`stopping_watchdog(|| {})` occur in `runtime/tests/watchdog.rs:84`, again at
line 126, and `runtime/tests/provider_failures.rs:106`. These are under
`crates/ic-timers/src/`. The existing `runtime/tests.rs` already provides the
equivalent successful-construction helper for Once registrations.

Consequence: the same setup changes across test families while assertions remain
scenario-specific. Owner: native Timers fixtures. Disposition: consolidate when
these subjects are next edited, using one narrow retained-Watchdog constructor.
Keep callback behavior explicit where it is the subject; preserve direct creation
in initial-admission, capture/drop and refusal tests. No new test framework or
shared package is warranted. Expected benefit is modest; no deletion estimate is
claimed. Rerun the affected immediate-scheduling and replacement-failure subjects.

## Rehearsals and intentional retention

Three source rehearsals correspond to the findings: changing deferred ordinary
unregistration currently crosses control/registry; narrowing proven Jobs-only
publication crosses the plan/Make/script boundary; changing successful Watchdog
setup crosses native fixture files. The proposed owners remove those particular
coordination points without adding a behavior axis.

No justified crate or direct-dependency deletion emerged from manifest/caller
inspection. Jobs stays Serde-only at runtime; the default capture binary keeps
its optional native harness disabled. Runtime, size, host and protocol packages
have distinct qualification responsibilities. This is not a fresh cargo-machete
execution. Keep independent generated-command and suspended-work oracles, checked
Storage admission, restore validation, staged/committed provider bindings and
native/installed proofs. The recent Metrics aggregation carries extra bookkeeping;
it should not be represented as a memory-footprint improvement.

## Checks and limitations

Performed source/manifests/caller inspection and textual duplicate discovery;
matching text was traced before classification. No product tests, builds, dependency
updates, full CI, hosted checks, issue mutations, commits or delivery were run.
Prior passing validation retains its own input identities and is not a fresh
audit execution. Only this report was added; documentation links and diff checks
were checked. No findings were repaired. This report is review evidence, not an
additional active issue tracker.
