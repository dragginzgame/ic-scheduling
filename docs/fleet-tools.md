# Scheduling source layout and LOC reporting

This workspace uses the reviewed Shared Tooling classifier. Its canonical
[source classification contract](https://github.com/dragginzgame/shared-tooling/blob/b37a882314d22bf7ab393b3fe561809377ffd22d/docs/fleet-tools.md#source-classification)
owns the reporting format and category rules. This page documents Scheduling's
package roles and commands; it does not define a separate counting policy.

## Source classification

Run from the repository root with the prepared host tools and Rust compiler:

```sh
make cloc
bash scripts/dev/cloc.sh --subcrates .
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
| `ic-jobs-canister-tests` | Artifact-capture tooling, with explicit test-harness and example path roles. |

Keep test owners in `tests.rs`, `*_tests.rs` or `tests/`, usage demonstrations in
`examples/`, benchmarks in `benches/`, and development utilities in their tooling
owner. Existing private packages stay in the root workspace. Package metadata
does not change feature selection, publication eligibility or runtime ownership.

The JSON report records file hashes, categories and classification diagnostics.
Macro expansion and conditional attributes require review; zero unclassified
lines alone is not a proof of compilation or deployment behavior. A category
correction changes accounting, not the amount of maintained source.

## Scratch and evidence

Local qualification and fixture helpers default to `target/tmp`, so normal
workspace cleanup can remove generated runs. An explicit `TMPDIR`, or
`RUNNER_TEMP` for hosted recovery evidence, retains its caller-owned location.
These overrides are local to the invocation; helpers do not change global
environment settings. Required release recovery evidence stays under the
existing bounded Git-side custody policy.
