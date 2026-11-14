# Scheduling source layout and LOC reporting

This workspace uses the reviewed Shared Tooling classifier. Its canonical
[source classification contract](https://github.com/dragginzgame/toko-tooling/blob/1542683b6d8af295d5366f95f85a292e978ff5ea/docs/fleet-tools.md#source-classification)
owns the reporting format and category rules. This page documents Scheduling's
package roles and commands; it does not define a separate counting policy.

Installed shared executables and operational inputs live in `vendor/toko-tooling/`
and are recorded by `.toko-tooling.snapshot`. Scheduling owns the root Makefile,
`scripts/`, workflows, product policies and `docs/`. Refresh managed files with
`make toko-tooling-update`; edit local callers outside the vendor tree.

## Source classification

Install the launcher once on your user PATH after preparing the repository's
host tools and Rust compiler. It selects this checkout's installed reporter and
tools on every invocation:

```sh
mkdir -p "$HOME/.local/bin"
install -m 755 vendor/toko-tooling/scripts/dev/toko-cloc "$HOME/.local/bin/toko-cloc"
export PATH="$HOME/.local/bin:$PATH"
```

From the repository root:

```sh
toko-cloc
toko-cloc -d 0
toko-cloc -d 1 ic-timers
toko-cloc --wide -d 1
toko-cloc --manifest qualification/Cargo.toml
toko-cloc --json
```

The product workspace contains Jobs and Timers. The independent `qualification/`
workspace contains six private packages. Root LOC reporting includes both;
their package manifests declare fixture and tooling roles explicitly:

| Package | Role |
| --- | --- |
| `ic-jobs`, `ic-timers` | Published libraries; Cargo targets distinguish their tests, examples and build scripts. |
| `ic-jobs-test-consumer` | Test canister, DTOs and recovery fixtures. |
| `ic-timers-runtime-probe`, `ic-timers-size-probe`, `ic-timers-test-protocol` | Installed qualification probes and their shared test protocol. |
| `ic-timers-pocketic` | Build/qualification tooling, with test sources classified separately. |
| `ic-jobs-canister-tests` | Artifact-capture tooling, with the optional harness, capture tests and recovery tests explicitly classified as tests. |

Keep test owners in `tests.rs`, `*_tests.rs` or `tests/`, usage demonstrations in
`examples/`, benchmarks in `benches/`, and development utilities in their tooling
owner. Private packages belong to the qualification workspace. Package metadata
does not change feature selection, publication eligibility or runtime ownership.

The Tests column includes test-only helpers, deterministic oracles, canister
fixtures, probes and protocol source, as well as test bodies. The PocketIC host
driver and artifact-capture binary count as Build/Tooling; their tests count as
Tests. The classifier splits test-only items and module wiring from enclosing
library/tooling files. Rust doctests in documentation comments are outside this
source-code count, as are shell tooling fixtures and non-Rust assets.

The JSON report records file hashes, categories and classification diagnostics.
Table labels are shortened to fit the available width; `--wide` retains full
names. JSON labels and numeric counts remain complete in either display mode.
Macro expansion and conditional attributes require review; zero unclassified
lines alone is not a proof of compilation or deployment behavior. A category
correction changes accounting, not the amount of maintained source.

### Historical Tooling 0.36 limitation

The 2026-11-09 audit reconciled all 138 admitted Rust paths against the report
and reviewed every classification diagnostic. After the qualification removals,
all 133 remaining paths and source hashes still reconcile. Whole test files, private fixture
packages and test-module wiring have the intended roles. Macro diagnostics
describe authored macro bodies with the same role as their enclosing source;
the FFI conditional attributes choose export names, not test ownership.

The earlier classifier missed two test-only fault-injection blocks inside
call-argument closures: four code lines in
[`runtime/effects.rs`](../../crates/ic-timers/src/runtime/effects.rs) and six in
[`runtime/dispatch.rs`](../../crates/ic-timers/src/runtime/dispatch.rs). Their
test-only imports were split correctly, but those ten lines remained in
Library/App instead of Tests. Tooling 0.39 includes the producer's closure
traversal fix ([Toko Tooling #367](https://github.com/dragginzgame/toko-tooling/issues/367)).
The earlier counts and receipts retain their original source identity; use the
current combined report for the current graph. Category corrections do not
remove source or establish compilation or runtime behavior.

## Scratch and evidence

Local qualification and fixture helpers default to `target/tmp`, so normal
workspace cleanup can remove generated runs. An explicit `TMPDIR`, or
`RUNNER_TEMP` for hosted recovery evidence, retains its caller-owned location.
These overrides are local to the invocation; helpers do not change global
environment settings. Required release recovery evidence stays under the
existing bounded Git-side custody policy.

## Documentation and experimental evidence

Follow the [shared retention contract](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/rules/engineering-details.md#changelogs-and-artifact-preservation).
Local command/CI results and the owning changelog retain routine check outcomes.
Raw experimental output belongs in `target/evidence/`; required release recovery
records remain under their existing custody contract.

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
[shared adoption guidance](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/docs/releases/group-validation-adoption.md#replacing-existing-release-evidence-parsers).
