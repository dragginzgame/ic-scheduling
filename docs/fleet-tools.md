# Scheduling source layout and LOC reporting

This workspace uses the reviewed Shared Tooling classifier. Its canonical
[source classification contract](https://github.com/dragginzgame/toko-tooling/blob/1542683b6d8af295d5366f95f85a292e978ff5ea/docs/fleet-tools.md#source-classification)
owns the reporting format and category rules. This page documents Scheduling's
package roles and commands; it does not define a separate counting policy.

## Source classification

Run from the repository root with the prepared host tools and Rust compiler:

```sh
make cloc
bash scripts/dev/cloc.sh -d 0 .
bash scripts/dev/cloc.sh -d 1 ic-timers
bash scripts/dev/cloc.sh --json .
```

The root workspace includes all eight members. Their package manifests declare
the private fixture and tooling roles explicitly:

| Package | Role |
| --- | --- |
| `ic-jobs`, `ic-timers` | Published libraries; Cargo targets distinguish their tests, examples and build scripts. |
| `ic-jobs-test-consumer` | Test canister, DTOs and recovery fixtures. |
| `ic-timers-runtime-probe`, `ic-timers-size-probe`, `ic-timers-test-protocol` | Installed qualification probes and their shared test protocol. |
| `ic-timers-pocketic` | Build/qualification tooling, with test sources classified separately. |
| `ic-jobs-canister-tests` | Artifact-capture tooling, with the optional harness, capture tests and recovery tests explicitly classified as tests. |

Keep test owners in `tests.rs`, `*_tests.rs` or `tests/`, usage demonstrations in
`examples/`, benchmarks in `benches/`, and development utilities in their tooling
owner. Existing private packages stay in the root workspace. Package metadata
does not change feature selection, publication eligibility or runtime ownership.

The Tests column includes test-only helpers, deterministic oracles, canister
fixtures, probes and protocol source, as well as test bodies. The PocketIC host
driver and artifact-capture binary count as Build/Tooling; their tests count as
Tests. The classifier splits test-only items and module wiring from enclosing
library/tooling files. Rust doctests in documentation comments are outside this
source-code count, as are shell tooling fixtures and non-Rust assets.

The JSON report records file hashes, categories and classification diagnostics.
Macro expansion and conditional attributes require review; zero unclassified
lines alone is not a proof of compilation or deployment behavior. A category
correction changes accounting, not the amount of maintained source.

### Reviewed Tooling 0.36 limitation

The 2026-11-09 audit reconciled all 138 admitted Rust paths against the report
and reviewed every classification diagnostic. After the qualification removals,
all 133 remaining paths and source hashes still reconcile. Whole test files, private fixture
packages and test-module wiring have the intended roles. Macro diagnostics
describe authored macro bodies with the same role as their enclosing source;
the FFI conditional attributes choose export names, not test ownership.

Two test-only fault-injection blocks inside call-argument closures are missed by
the selected classifier: four code lines in
[`runtime/effects.rs`](../crates/ic-timers/src/runtime/effects.rs) and six in
[`runtime/dispatch.rs`](../crates/ic-timers/src/runtime/dispatch.rs). Their
test-only imports are split correctly, but these ten lines remain in Library/App
instead of Tests. No source is omitted and the total is unchanged. Zero
Unclassified does not detect this case. Correct closure traversal in the producer
and adopt its released installer ([Toko Tooling #367](https://github.com/dragginzgame/toko-tooling/issues/367));
do not edit snapshot-owned scripts, classify
these entire production files as tests, or restructure runtime code for counting.

## Scratch and evidence

Local qualification and fixture helpers default to `target/tmp`, so normal
workspace cleanup can remove generated runs. An explicit `TMPDIR`, or
`RUNNER_TEMP` for hosted recovery evidence, retains its caller-owned location.
These overrides are local to the invocation; helpers do not change global
environment settings. Required release recovery evidence stays under the
existing bounded Git-side custody policy.

## Documentation and experimental evidence

Follow the [shared retention contract](../rules/engineering-details.md#changelogs-and-artifact-preservation).
Routine checks use their existing command/CI result and pending notes; they do
not need a new report directory. For a maintained experiment topic, keep one
current summary (normally up to 150 lines) plus at most two selected small data
files, at most 100 KiB per topic and 5 MiB across the repository. Raw experimental
output belongs in `target/evidence/`, rather than tracked docs or compressed bundles.

The 2026-11-05 pre-refresh inventory found 134 tracked docs files totaling 2,185,215 bytes;
this includes shared manuals, design contracts and protected history, so it is a
conservative upper bound rather than an experimental-evidence total. No tracked
Wasm or compressed archive was present. The six small audit logs/manifests total
12,575 bytes and retain their original source-bound audit references.

The large `docs/status/history-*` pages and the 586,974-byte pair of
`docs/timers/evidence/` ledgers are historical compaction candidates. Current
handoffs link them, so review their provenance/recovery obligations and repair
references under an authorized cleanup. Their names do not grant a permanent
retention exception. Active release/recovery records, maintained API/design
contracts and test vectors retain their separate owners. Do not create another
permanent archive by default, or claim a summary/hash proves raw artifacts that
are no longer retained.

## Validation ownership

Scheduling already retains Shared's complete validation proof and adds its own
source/host/group and metadata-projection custody. Its release adapter does not
parse runner event/timing files again. Keep that division under the
[shared adoption guidance](group-validation-adoption.md#replacing-existing-release-evidence-parsers).
