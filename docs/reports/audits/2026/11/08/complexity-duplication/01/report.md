# Scheduling complexity and duplication audit

2026-11-08, run 01. **PASS WITH FINDINGS**: three bounded maintenance findings;
no demonstrated product contract failure in the inspected flows. This is a
structural conclusion, not fresh runtime, security or release qualification.

## Scope and identities

Requested follow-up after the ten cleanup items, with Shared Tooling refreshed
first. Methods: `audits/complexity-and-technical-debt.md` and
`audits/flow-convergence-and-duplication.md`, under `audits/README.md`, at Shared
0.31.11, `6c529e6940a97226cca826726491e03173b08cd2`.
Consumer HEAD: `354dd5143d7c2d848096d836301fd349a68e9ad8`; dirty inputs include the
previous Build 0.79 / Shared 0.31.10 adoption, workflow wiring, supplier lock and
notes, followed by this policy-only refresh. No Rust source differs from HEAD.
AGENTS.md is the product authority, SHA-256
`c9ec868bf4dfb4960d00b3f0e67598883a1698eaa8b568203e221a3fd5547372`.
There is no separate adopted complexity overlay; this report declares the first
bounded complexity scope. The older code-hygiene checklist is a different method.

Inventory covers all eight packages; traces focus on ordinary completion,
registration/capture cleanup, consumer restore/commit, qualification artifacts,
release-stage selection and publication. Excluded: Canic/sibling implementations,
new protocol/security/persistence correctness baselines, full hosted/native gates,
installed IC replay and algorithm/runtime benchmarking. Those require distinct
proof; absence here is not PASS. Sibling source was read-only.
Current lock SHA-256:
`670684d27c1dd4262af10c9ff9462e78d34107da80a9185ec9516e80843b1fea`.
Endpoint/macros 0.8.1 arrived in the pending lock after earlier focused execution;
that earlier 47fbb591 lock's test results are not current-graph qualification.

## Measurements and comparison

Fresh shared `cloc.sh --json` / `-d 1` counts are in `metrics.json`.

| Category | LOC | Change since the completed ten-item cleanup |
| --- | ---: | ---: |
| Production library/application | 5,465 | 0 |
| Test-classified source | 25,056 | 0 |
| Examples | 752 | 0 |
| Build tooling | 907 | 0 |
| Unclassified | 0 | 0 |

Timers production is 4,793; Jobs is 672. The 25,056 figure includes 7,673 lines
in the private consumer application, runtime/size probes and protocol package;
it is not 25,056 lines of test assertions. No new code cut or speed-up is claimed.
`cargo machete` 0.9.2 found no unused direct dependencies. Locked normal/build
trees confirm Jobs is Serde-only and the default capture binary excludes Testkit.
Raw inventories, graph observations and textual discovery are under
`target/tmp/audit-complexity-20261108/`; text matches were manually traced.
The 2026-11-05 standards audit used different scope/methods: overall comparison
is N/A. Cleanup LOC counts remain comparable individual anchors.

## State and ownership map

| Maintained axis | Owner and constraints |
| --- | --- |
| Once / after-completion / Watchdog | EntryKind derives policy from typed payload; no parallel independent policy tag. |
| Retained / remove-on-stop | Registry lifetime; reconciliation constructors require retained declarations. |
| Ordinary inactive / scheduled / running, five pending command variants | TimerControl owns selection/arbitration; pending state exists only during running work. |
| Watchdog inactive / scheduled / awaiting work, five pending variants | WatchdogControl owns pre-armed successor semantics; distinct from ordinary completion. |
| Declaration claim / callback generation / role / provider handle | Registry admits authority; runtime binds/clears linear handles. These identities are not interchangeable. |
| Jobs recorded state and uncertain effects | Job::restore and Job transitions own validation; consumer owns storage and explicit disposition. |
| Runtime / focused / cohort qualification; four cohort features | Driver owns selection, Cohort owns catalog, Build owns exact artifacts; receipts bind their original inputs. |

No unconstrained Cartesian state count is meaningful: types and admission remove
invalid combinations. Test command enums add maintenance, not product policies.

## Entry-to-result traces and retained decisions

| Entry / behavior | Owner derives → contract carries → consumer projects |
| --- | --- |
| Ordinary public/context commands | runtime authority → TimerControl::complete/OrdinaryCompletion → registry observations/effect → runtime provider action. |
| Direct and fresh reconciled registration | register_initial/register_reconciled → register_initialized → finish_transition → provider cleanup before capture Drop; owner slot publishes only on success. |
| Stored Jobs read / typed commit | bounded decode → Job::restore → Snapshot admission; commit encodes/checks collection before Storage write and exposes effects only afterward. |
| Probe acquisition / qualification | WasmBuildEvidence::acquire → retained record/context → installed consumers → verify_current before accepting receipt. |
| Release / upload | closed scope map → ordered plan and sealed proof → metadata projection; publication separately re-admits source/tag/registry before upload. |

Keep native and installed tests separate: mocks do not prove IC rollback.
Keep different failed-creation, later-reconciliation and destructor paths: their
empty-slot, retained-authority and borrow/cleanup obligations differ.
Keep the 24,576 generated four-command traces and independent suspended oracle;
size alone does not justify deleting their fault/authority witnesses.
Keep the three-policy typed public facade, sole provider and inert snapshots;
a generic registration API would weaken compile-time restrictions for little gain.

## Findings

1. **MEDIUM — repeated release qualification cost; owner: release validation.**
   `ci/release-validation.json`, `scripts/release/validation.pl:145`,
   `Makefile:280` and `scripts/release/publish.sh:27` retain whole fixture execution
   across full group gates and aggregate publication lint. In the two retained
   successful release runs, test-release-tooling plus test-release-validation
   consumed 1,550/1,900s and 1,502/1,852s: 81.3% of combined stage time. These are
   historical measurements, not current timings or a removable-time estimate.
   Disposition: accept until a bounded exact-input stage is qualified. Start with
   one fixture whose complete tools/configuration/source closure can be admitted
   through Shared's existing reuse contract. Source, version, host and environment
   differences must force execution; do not skip by group name or relabel receipts.
   Rehearse publication admission separately; archive/registry checks stay fresh.

2. **LOW — duplicate Jobs compilation within one gate; owner: stage roster.**
   `Makefile:114–115` repeats the package/targets/features/locked/offline compiler
   selection already covered by the first clippy-jobs command at `Makefile:192`.
   The retained check stage took 4s and 5s, far smaller than finding 1. Disposition:
   fix when the roster is next qualified: prove no distinct obligation, retire that
   stage and adjust selection/evidence fixtures together. Keep default-feature,
   Wasm, archive and lint obligations; do not add another skip option.

3. **LOW — repeated test clock and cleanup projection; owner: local harnesses.**
   Jobs recovery has 14 get_time sites, with the same multi-line clock conversion
   repeated at lines 565/584/609/898/1054. Jobs and Timers runtime subjects contain
   44 explicit close chains. These are repeated plumbing, not duplicate scenarios.
   Disposition: consolidate when touching those fixtures using Jobs-local now_ns
   and consuming close helpers, plus a Timers-local close helper. Every observation
   still reads the IC clock at its original point; every close is still awaited
   before server teardown. Do not introduce a generic test framework or shared
   lifecycle crate. Count net LOC and rerun affected installed/cleanup subjects
   before claiming any reduction; a large test-suite cut is not yet evidenced.

## Three source-inspection rehearsals

| Near-term edit | Expected owner / spread | Present friction or retention |
| --- | --- | --- |
| Change test clock/close diagnostics | local Jobs Fixture / Timers harness; application scenarios remain semantic consumers | 14 reads and 44 close sites repeat plumbing; finding 3. |
| Adjust release validation cost | consumer roster/selector + shared proof adapter + negative/recovery fixtures | All 37 obligations currently required for shared/Timers changes; findings 1–2 need qualification, not deletion alone. |
| Use Build batch for four cohort builds | driver/Cohort → Build → report verifier | Bare batch outcomes lack WasmBuildEvidence's caller-facing verify_current wrapper; metadata keys include differing feature arguments. No automatic resolution saving is established. Keep current acquisitions until equivalent freshness and measured benefit exist. |

Dependency review found no justified package deletion. Keep protocol DTOs free of
Timers/runtime dependencies; two tiny producer counter projections do not justify
coupling that package to Timers. Keep native/runtime/size/protocol packages for
their distinct roles. No new dependency, public mode or persisted format is needed.

## Checks and limits

Executed: snapshot hash/mode verification and independent selected-source compare,
shared LOC reports, cargo-machete, locked dependency trees and source inspection.
Only documentation consistency/diff checks follow this report. No product test,
build, full CI, hosted scan, release or publication was run for the audit. No
finding was repaired; prior adoption and delivery evidence keeps its original
source/lock identity. This report is evidence, not a parallel issue backlog.
