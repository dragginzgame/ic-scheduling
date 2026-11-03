# Ten simplifications: measurements, 2026-11-03

All ten requested changes are implemented locally. This batch reduces duplicated
policy and invalid argument combinations; it does not claim a general speed-up.
No public library API, scheduling policy or persisted queue format changes.
Package versions remain Jobs 0.34.1 and Timers 0.38.1; numbered changelog drafts
are 0.34.2 and 0.38.2. No commit, push, release or publication was performed.

## Structural results

Counts are source observations of the named duplication, not complexity scores.
Module moves count as moves, not deleted functionality.

| Item | Before | After and retained boundary |
| --- | --- | --- |
| Typed runner CLI | Separate Clap declaration and manual typed extraction; 4 `get_one`/`get_many` calls | One typed command definition; 0 manual extraction calls. Worker argument projection round-trips all three commands. |
| Management methods | Separate ID and exported-name rosters | One roster for all 22 exact ID/name pairs. No protocol renumbering. |
| Jobs installation | 2 valid installation recipes in harness/demo | 1 shared recipe with caller-owned configuration and diagnostic handling. Invalid-init fixture remains independent. |
| Probe codecs | 8 handwritten admission/decode/encode helpers across 2 probes | 0 such helpers; 64 existing entrypoints select canonical Endpoint with explicit method IDs and request/reply budgets. The real awaited gate exchange remains. |
| Native fault selection | Scenario fault selection duplicated across hand-written and generated tests | 1 typed scenario controller. Low-level provider injection and fixture reset remain separate mechanics. |
| Forbidden host dependencies | Concrete Build/Host/Testkit names repeated in 2 rosters | 1 family pattern applied to 4 selected graphs; a new unlisted Host fixture is rejected by the actual Make target. Other exclusions and native harness allowances remain. |
| Consumer observations | Queue directly owns 2 diagnostic adapters and coordinates event recording | Queue owns 1 observation coordinator delegating to the existing Metrics and Logging adapters; events still follow durable commit. |
| Counter exactness | 15 repeated overflow assertions across the 7-counter cohort and 8-counter snapshot projections | 1 assertion loop called with explicit counter arrays. Other independent scenario assertions remain. |
| Runtime authority | 48 call sites pair a claim with an optional execution token | 0 pairs. A private authority value derives its claim; callback-token expiry is still validated before mutation. |
| Queue snapshot boundary | Queue file: 667 physical lines, mixing snapshot codecs/admission with storage/effects | Queue file: 461 physical lines. Snapshot representation, bounded codec and collection admission have one private child owner. `Job::restore`, Cell custody, effect ordering and uncertainty remain intact. |

Across maintained Rust under `apps/` and `crates/`, the same Cloc selection reports
31,503 → 31,496 code lines (**−7**), 1,869 → 1,876 comment lines, 1,932 → 1,939 blank
lines and 90 → 92 files. This small net change includes tests and private module
extraction. It is not evidence of reduced execution cost or development time.
The temporary measurement driver is excluded.

## Controlled inputs

Baseline source is `1e9821379a38f6f2b8a3b893d5987210ef48e933` plus the preserved
incoming supplier lock. Endpoint dependency declarations and Clap's derive
feature were admitted before the baseline, without changing baseline Rust.
The measured after source adds the ten changes and removes unused direct probe
dependencies. All **314 external package rows**, checksums and resolved external
features match between the measured copies. Their lock difference is confined
to local member dependency edges.

Both sides use Rust/Cargo 1.99.1, `wasm32-unknown-unknown`, Linux x86_64,
PocketIC 16.1.0 and the unchanged `timer-probe` recipe: release inheritance,
thin LTO, `opt-level = "z"`, panic abort and symbol stripping; no post-link
optimizer. Server SHA-256:
`b44e1eccd66e02328146b3e209e3403e3a3428a25d25555aca2f67500e5ae9db`.
Cohort recipe identity:
`037d3f02ab2e0d5871abfb70dd78b1ecf33f19f408d4b56a2ddd854957db45bf`.

The consumer/runtime-probe artifacts use that same Cargo profile as qualification
proxies, not a claim about another application's production deployment recipe.
All sizes below are uncompressed bytes from the final saved modules. A task-only
parser validates the Wasm magic/version and section lengths and reads section
ID 10's payload length; “code” means **code-section payload bytes**, excluding
the section ID/length prefix. No IC code-limit/headroom claim is made.

## Wasm footprint

| Artifact | Module before | Module after | Δ bytes | Code before | Code after | Δ bytes |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Baseline cohort | 65,973 | 67,636 | +1,663 | 58,721 | 59,831 | +1,110 |
| Once | 122,688 | 120,820 | −1,868 | 107,918 | 105,537 | −2,381 |
| AfterCompletion | 122,778 | 120,841 | −1,937 | 107,965 | 105,511 | −2,454 |
| Watchdog | 121,346 | 124,326 | +2,980 | 106,618 | 108,920 | +2,302 |
| Jobs consumer | 437,771 | 438,423 | +652 | 398,463 | 399,070 | +607 |
| Runtime probe | 327,898 | 325,416 | −2,482 | 295,713 | 288,795 | −6,918 |

These are whole-batch measurements; they do not isolate each refactor's causal
contribution. Canonical Endpoint reuse is retained despite the Watchdog/baseline
footprint increases and small RPC cost increases below. There is no optimizer
or workload change hidden in this comparison.

## Installed execution costs

Existing counters bracket handler bodies. They exclude Endpoint admission,
decoding and encoding. Values below are **before → after instructions**.

| Cohort | Start | Ensure again | Cancel after cohort dispatch | Scheduler | Work |
| --- | ---: | ---: | ---: | ---: | ---: |
| Once | 17,776 → 17,776 | 5,820 → 5,810 | 5,364 → 5,325 | 0 → 0 | 5,121 → 5,121 |
| AfterCompletion | 20,150 → 20,150 | 6,606 → 6,567 | 11,354 → 11,315 | 0 → 0 | 10,411 → 10,411 |
| Watchdog | 19,049 → 19,049 | 6,457 → 6,418 | 9,806 → 9,767 | 13,585 → 13,585 | 5,141 → 5,141 |

Dispatch cycle charges are unchanged: Once 16,775,665; AfterCompletion 16,781,472;
Watchdog 33,559,673. Minimal and representative Watchdog calibration measurements
are also unchanged, including representative work at 1,905,169 instructions.
The complete reports retain snapshot, inventory and memory-sampling measurements.

A separate, identical native driver installs saved before/after Wasm and samples
canister cycle balances around full Wire update calls. It starts, ensures again,
then cancels **before timer execution**, without advancing IC time. Replies and
changed-state flags are checked. This is distinct from cancellation after cohort
dispatch above. The exact same compiled driver/server are used for both runs.
One deterministic sample per cell is reported; there is no statistical claim.

| Cohort | Start cycles before → after | Ensure again cycles before → after | Cancel cycles before → after |
| --- | ---: | ---: | ---: |
| Baseline | 6,977,052 → 6,977,786 | 6,977,052 → 6,977,786 | 6,965,052 → 6,965,786 |
| Once | 6,994,931 → 6,995,379 | 6,982,872 → 6,983,310 | 6,974,836 → 6,975,245 |
| AfterCompletion | 6,997,291 → 6,997,739 | 6,983,630 → 6,984,039 | 6,976,390 → 6,976,799 |
| Watchdog | 6,995,953 → 6,996,867 | 6,983,244 → 6,984,119 | 6,974,589 → 6,975,464 |

Full RPC costs rise **409–914 cycles** in these samples (less than 0.014%).
Do not infer full-call improvements from the smaller handler instruction counts.

## Qualification and evidence

- 268 focused native cases passed, including all 175 Timers tests, consumer
  storage/retained-image tests, shared installation callers and CLI round-trips.
  Explicit live tests remain ignored in this native selection.
- All 35 installed watcher subjects passed across the initial run's 34 passes
  and a focused rerun of the malformed-Wire subject. Its assertion now checks
  Wire's canonical malformed diagnostic and trap code, retaining no-mutation
  checks. The initial run failed only on the retired custom diagnostic prefix.
- Before and after policy-cohort qualification passed independently. The selected
  installed Jobs management/upgrade/real-watchdog recovery case passed with the
  saved after consumer Wasm.
- Strict native Clippy, consumer/runtime Wasm Clippy, all four size-probe feature
  selections, formatting, modified shell-script ShellCheck, actual dependency
  graph/boundary targets and dependency/provider rejection fixtures passed.

Local evidence is under `target/redundancy-ten.Gf1FhLzZ/`, selected by
`target/current-redundancy-ten`: before/after reports and Wasm, artifact hashes,
source snapshots, Cloc JSON, command logs/statuses, retained installed diagnostics,
comparison admission, and task-only parser/RPC driver source plus driver binary
hash. These are regenerable development evidence, not release receipts.

Core reproduction commands from the isolated source:

```sh
POCKET_IC_BIN=<admitted-server> cargo run -p ic-timers-pocketic --locked --offline -- cohorts --report <report.json>
cargo build -p ic-jobs-test-consumer -p ic-timers-runtime-probe --profile timer-probe --target wasm32-unknown-unknown --locked --offline
```

The built-in strict `--baseline` comparator was **not** used: local dependency-edge
cleanup changes the full lock SHA. It remains strict and unchanged. The separate
controlled comparison checks equal external packages/features, compiler, server,
host, target, profile and recipe identities; original receipts remain intact.
No full `make ci`, macOS qualification, complete Jobs recovery matrix or release
qualification is claimed.

## Concurrent checkout graph

While this batch was measured, the ordinary checkout's lock was independently
refreshed to Storage 0.65.5 and an additional Runtime 0.19.0. That work is
preserved; only the two probe members' dependency-edge edits were applied to it.
Every applied Rust source file matches the qualified isolated source, but the
ordinary checkout's newer external graph is **not** the measured graph above.
Locked/offline Cargo metadata passes there; the actual dependency-graph target
fails because the consumer renders **two Runtime identities**, where one is
required. The exact Runtime guard and singleton policy remain unchanged.

`applied-root-graph-guards.log` and the preserved concurrent lock record that
failure separately from the passing isolated checks. Align published Storage,
Wire/Endpoint, Testkit and Runtime suppliers before release qualification; do
not treat these refactor results as approval of the concurrent supplier refresh.
See [the current handoff](status/current.md).
