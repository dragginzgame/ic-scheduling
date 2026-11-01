# IC Scheduling

One Rust workspace for durable jobs and observable timer primitives on the
Internet Computer. Jobs and Timers have independent release versions within one
dependency catalog, lockfile, toolchain and CI.

- [`ic-jobs`](crates/ic-jobs/README.md): durable records and checked job policies.
- [`ic-timers`](crates/ic-timers/README.md): volatile callbacks, wakeups and watchdogs.

Open [ic-scheduling.code-workspace](ic-scheduling.code-workspace) for both libraries
in one editor workspace. Timer-specific commands are in the
[development guide](docs/timers/development.md).

IC Jobs provides durable job metadata and scheduling policies for Internet
Computer applications. Applications persist records, maintain their due-time
index and execute handlers; IC Timers owns volatile wakeups.

## Use

The [package API guide](crates/ic-jobs/README.md), also included in rustdoc,
contains usage examples, checked configuration, batch scheduling, timer
integration and migration requirements. The packaged guide remains self-contained.
See [SAFETY.md](SAFETY.md) for persistence and recovery obligations.

Run `cargo run -p ic-jobs --locked --example persisted_job` for the packaged
[adapter example](crates/ic-jobs/examples/persisted_job.rs), including failed
commits and receipt reconciliation. The [adapter contract](crates/ic-jobs/README.md#adapter-integration-contract)
and [acceptance checklist](docs/test-consumers.md#adapter-acceptance-checklist)
describe the host obligations. The [Canic integration guide](docs/canic-integration.md)
maps those obligations to the proposed adapter; it does not advertise an existing
Canic API. Jobs depends only on Serde, so Canic owns its integration dependency
versions. The unpublished [canister consumer](apps/ic-jobs-test-consumer/README.md)
demonstrates application-owned storage, management and lifecycle integration.
Its PocketIC harness is compiled by CI; live qualification is explicitly selected.

To queue jobs for seconds, minutes or weeks and see their current schedule, run
the [local canister queue demo](crates/ic-jobs-canister-tests/README.md). `make demo`
opens a terminal connected to the real fixture canister in local PocketIC;
`watch` shows countdowns and `advance` fast-forwards IC time.

## Scope and validation

The library owns metadata and checked transitions. Applications own storage,
payloads, handlers, longer audit history and disposition of uncertain effects.
It supplies no global queue, database, lifecycle exports or exactly-once effects.

| Boundary | Owner and entry point |
| --- | --- |
| Validated records, attempts, transitions and due-time selection | Published `ic-jobs`: `Job::restore`, `Job` methods, `next_due_ns` and `start_next`; Serde is its only runtime dependency |
| Payloads, authorization, stable queue and lifecycle endpoints | Consumer applications; the unpublished `apps/ic-jobs-test-consumer` demonstrates this integration |
| Stable allocation and fixture Cell admission | IC Memory; IC Storage supplies governed native test cells |
| Volatile wakeups and watchdog recovery | IC Timers, called directly by the consumer |
| Principal and explicit-codec canister runtime | Published IC Platform; Jobs retains endpoints, schemas, limits and error policy |
| Installation, management and delivery encoding and bounded decoding | Published IC Wire; Jobs retains method identities, limits and receipt policy |
| PocketIC transport and startup | IC Testkit; Jobs supplies Wire frames and retains scenario policy |
| Shared native management client | Private `ic-jobs-canister-test-support`, used only by the recovery harness and queue example |
| Artifact retention and native filesystem/process mechanics | IC Build and IC Host; Jobs retains optimizer flags and evidence naming |
| Common setup, formatting and release mechanics | Reviewed Shared Tooling snapshot; Jobs owns its gates, direct-delivery policy and package upload admission |

## Dependency exposure

Only `ic-jobs` and `ic-timers` are publishable. All seven supporting packages
set `publish = false`; their separation keeps host tooling out of Wasm and
qualification dependencies out of the libraries.

| Private package | Purpose |
| --- | --- |
| `ic-jobs-test-consumer` | Consumer-owned durable state, qualification canister and native composition tests |
| `ic-jobs-canister-tests` | Native artifact capture, installed recovery tests and queue demo |
| `ic-jobs-canister-test-support` | Management transport shared by recovery tests and the demo, excluded from the capture binary |
| `ic-timers-pocketic` | Native runtime qualification and cohort runner |
| `ic-timers-runtime-probe` | Wasm canister exercising timer runtime behavior |
| `ic-timers-size-probe` | Separate minimal Wasm builds measuring each timer policy's size |
| `ic-timers-test-protocol` | Wire types shared by the native runner and Wasm probes |

Root `[workspace.dependencies]` entries select versions; each member opts into
them. Their presence in the root catalog does not put them in every build.
The Jobs and Timers blocks list all nine members with full versions and paths,
including unused private qualification entries; group policy rosters keep those
real requirements aligned during releases.
The catalog labels `Production dependencies` and `Tests and tooling only`
describe use here, independently of registry publication. Serde is shared by
production and tests, so it belongs to production. The qualification Wasm and
capture/recovery harness are `Tests and tooling packages`; `ic-jobs` and
`ic-timers` are the production packages. Metrics, Platform and Thiserror are
production dependencies of Timers. Paired grouped sort commands preserve these
boundaries in `make fmt`, `make fmt-check` and the formatting hook. Release
metadata checks use the same grouped policy for working and selected commits.

| Built artifact | Candid exposure | CDK exposure |
| --- | --- | --- |
| Published `ic-jobs` library | None | None; normal dependencies contain only Serde |
| Published `ic-timers` library | None | Platform's embedded executor and IC0; no full CDK |
| Qualification `ic_jobs_test_consumer.wasm` | None; all owned inputs/replies use Wire | Platform's Wasm-only `cdk` runtime supplies entrypoints; upstream `ic-cdk` is absent, and Timers shares its embedded executor |
| Native recovery tests and queue demo | PocketIC requires Candid for external simulator/management protocols; Jobs RPCs use Wire | Platform's `cdk` feature is enabled only for the qualification Wasm |
| `jobs-capture-artifact` host binary | No Candid dependency in its normal/build graph | No CDK dependency in its normal/build graph |

The Wasm above is an unpublished test fixture, not a production service. This
checkout does not establish what other deployed canisters use. Audit each actual
application's selected target graph and source rather than counting manifests.
For example, inspect the qualification Wasm's paths with:

```sh
cargo tree -p ic-jobs-test-consumer --target wasm32-unknown-unknown --edges normal,build --invert ic-platform@0.15.3 --locked --offline
cargo tree -p ic-jobs-test-consumer --target wasm32-unknown-unknown --edges normal,build --invert ic-platform-runtime-guard --locked --offline
```

The selected published boundary is IC Platform, with defaults disabled.
The [workspace catalog](Cargo.toml) owns requirements; [Cargo.lock](Cargo.lock)
owns the resolved versions. The [handoff](docs/status/current.md) records the
selected adoption and its source-bound evidence.
The qualification app selects Platform's `cdk` feature only on Wasm. Native
consumer builds explicitly select Platform's `principal` feature for Serde DTOs
and Wire contracts. Jobs, Memory and Timers remain Wasm/test dependencies; JSON
is limited to native fixtures and tooling. Parser, Agent,
certification and crypto features are not selected. Platform 0.15 owns the reviewed
runtime adapters and generator; its embedded executor and IC0 are shared with Timers. The qualification Wasm selects neither Candid nor full `ic-cdk`.
Owned native DTO principals exclude external Principal and SHA-2 dependencies.
The Wasm application selects neither principal conversion nor SHA-2 hashing;
stable-structures retains its own external Principal dependency.
Native tests select `principal-self-authenticating` explicitly;
the harness's `pocket-ic` feature supplies external conversions.

Installation, management, fixture controls and delivery use IC Wire native Serde
frames. Consumer-owned method identities and request/reply budgets live in
`management_wire` and `delivery_wire`. Wire supplies the codec hooks used by
Platform's entrypoints, including unit RPCs. Local guards retain oversized-request
rejections before argument copies; the automatic Wire attributes are not selected.
The init hook
performs that admission explicitly because lifecycle macros have no guards.
No Candid service or compatibility endpoint is generated. Frozen earlier Candid
bytes are negative fixtures only.

This is an unpublished fixture protocol cut. Quiesce management/delivery activity,
dispose of outstanding old management requests and rebuild the fixture and native
harness together. Delivery identities remain unchanged. The queue now stores
bounded Wire under `JQW\x01`; retained JSON refuses without conversion or reset.
Use fresh qualification state after explicitly retiring the old installation;
resolve its Running/Uncertain effects with the old binary and exact evidence first.
The [consumer guide](apps/ic-jobs-test-consumer/README.md) describes the boundaries, and the
[handoff](docs/status/current.md) records qualification by source. Native Testkit
uses its published Platform facade for external simulator interfaces. The
harness keeps the upstream Principal for PocketIC calls and converts to its
owned Principal when constructing application DTOs. No decoder or transport is replaced.

For the production library, locked normal/build graphs on native Linux and Wasm,
with all features selected, contain Serde and its macro dependencies and exclude
every tests/tooling-only catalog dependency (apart from the selected root itself).
The qualification Wasm intentionally includes its test integrations above; those
cannot be prohibited there using the production library's exclusion list.

`make check-dependency-boundaries` runs the reviewed shared checker and is part of
CI/release validation. It composes the [shared IC inventory](ci/external-ic-dependencies.txt)
with [local bans](ci/cargo-dependency-boundaries.json) on direct generator and
retired decoder dependencies, including aliases,
across member normal/build/dev/target declarations. It does not enforce feature
selection, inspect unused root catalog
entries or scan independent workspaces; this workspace has no maintained independent
Cargo roots and its catalog and feature selections are reviewed separately.

`make check-dependency-graphs` adds locked, offline normal/build graph admission
in CI/release validation: all-feature native and Wasm core graphs exclude the
local tests/tooling and IC integrations; the all-feature qualification Wasm excludes
host-only packages, JSON, Candid and full CDK. It requires one Timers, Platform runtime guard and entrypoint generator; registry
executor and IC0 packages are excluded. The application, Timers and Memory share Platform 0.15, including the
stable-structures supplier.
The graph excludes application principal conversion and SHA-2.
A fourth context excludes Jobs, Memory, Timers, JSON and the IC runtime from
ordinary native consumer builds, including the
Platform `cdk` feature.
The fifth context keeps Candid, PocketIC and Testkit out of the native capture
binary's normal/build graph. Candid remains only in native recovery tests:
Testkit enables Platform's PocketIC integration, and PocketIC itself uses Candid
management and transport DTOs. Testkit owns no Candid codec adapter. Disabling
dependency defaults
cannot remove those required edges. Jobs-owned RPCs continue to use Wire.
The policy lives in Make; Shared owns the [observation/checker](docs/cargo-graph-checks.md).
The [snapshot manifest](.shared-tooling.snapshot) records the reviewed Shared
Tooling version, source revision and payload digests. No sibling checkout is a
runtime prerequisite.

Remaining enforcement limits: these are explicit known-name policies for the
selected roots/targets/features, not inference from role comments or exhaustive
feature-matrix admission. Rendered Cargo identities do not authenticate sources.
Jobs has no production-canister deployment command. Downstream deployments must
bind their own target/features, exclusions, successful locked build, source/lock
identity and final artifact digest to the bytes installed. A passing graph or
snapshot check alone does not establish deployment admission.

## Consumer qualification

`make test-consumers` aggregates native notification/maintenance scenarios, stored
Wire conformance and the packaged adapter example on Rust 1.91. They cover commit
failures, reconstruction, exact receipt reconciliation, stale continuations and
bounded recurring work. The composition fixtures request IC Memory allocations
through the [workspace-selected dependency](Cargo.toml); the host fixture reopens actual
stable cells. See [test consumers](docs/test-consumers.md) for their storage and
timer boundaries.

Choose checks for the affected caller. For native consumer storage/timer
composition, use `bash scripts/dev/minimum-rust.sh cargo test -p ic-jobs-test-consumer --test composition
--locked --offline`; for persisted Wire records, select `--test stored_wire`;
for endpoint codecs, select `--test api_wire`. Run the packaged adapter example
with `bash scripts/dev/minimum-rust.sh cargo run -p ic-jobs --example persisted_job --locked --offline`.
Qualify the core Rust floor separately with `bash scripts/dev/minimum-rust.sh cargo check -p ic-jobs
--lib --locked --offline`. For the consumer's Wasm floor, use `bash scripts/dev/minimum-rust.sh cargo
check -p ic-jobs-test-consumer --lib --target wasm32-unknown-unknown --locked
--offline`. Lint the affected consumer with `cargo clippy -p ic-jobs-test-consumer
--all-targets --locked --offline -- -D warnings`, or select the Wasm target
when runtime integration changes.

The minimum-compiler launcher reads `workspace.package.rust-version` from
[Cargo.toml](Cargo.toml), normalizes its toolchain selection and installs nothing.
Make and both hosted workflows use that same projection. Hosted setup prepares
the pinned host parsers first; development Rust comes from `rust-toolchain.toml`.

`make clippy`, `make check-wasm`, `make check-msrv` and the other package-group
targets aggregate several checks. `make ci` runs the full release suite and
requires an explicit request or release authorization for local execution.

Hosted CI classifies the exact tested commits using the reviewed prose allowlist
in [ci/docs-only-paths.txt](ci/docs-only-paths.txt). Eligible documentation changes
run host-tool admission, snapshot integrity, local link and changed-whitespace
checks while preserving all three `validate` job names. These passes establish
documentation checks only. Source, tooling, release policy and unknown paths retain
the complete native matrix; missing history selects full checks and failed
classification fails validation. Releases always retain the complete gate.
`make test-ci-change-selection` exercises the actual workflow commands with real
Git comparisons and substituted native setup/Make effects.

Release-adapter fixtures can be selected independently with
`make test-release-tooling-retention`, `make test-release-tooling-delivery`,
`make test-release-tooling-recovery`, `make test-release-tooling-metadata`, or
`make test-release-tooling-publication`. The existing `test-release-tooling`
aggregate still runs every group and the publication eligibility fixture.
The retention selection needs no prepared metadata editor or formatters.

The package and its direct Timers qualification consumer require Rust 1.91. Compiler
selections are recorded in [rust-toolchain.toml](rust-toolchain.toml) and
[the local tool catalog](ci/tool-versions.env). Linux and macOS Intel/Apple Silicon
host workflows are maintained. See [the current handoff](docs/status/current.md)
for source-bound checks and outstanding native/live qualification; native tests
do not establish IC message rollback or upgrade recovery.

## Repository setup

The reviewed [Shared Tooling snapshot](.shared-tooling.snapshot) records the
canonical revision, entrypoints and payload digests. The
[common Make interface](docs/make-interface.md) owns setup, formatting, hooks,
snapshot checks and release routing. Jobs retains product checks, dependency
selections and release/publication policy.

Run `make install-tools` to prepare common host/Rust tools and Jobs' selected IC
bundle for optimized qualification; `make tools-check` admits them offline.
Shared Tooling's common Rust set contains cargo-sort and cargo-sort-derives.
Jobs has no Candid extractor caller and selects no extractor installation; older
retained installations are preserved outside the newly selected common PATH.
For an interactive shell, prepend
`$PWD/.tools/host/bin:$PWD/.tools/ic/bin:$PWD/.tools/rust/common/bin` to PATH.
Use the same `SHARED_TOOLING_STORE` for setup, checks and evidence collection;
see [local setup](docs/local-setup.md) for prerequisites, storage and reuse rules.
`make help` lists check groups and points to focused selections.
`make install-hooks` enables the reviewed formatting hook after tool preparation;
`make formatting-hook-check` verifies it in disposable repositories.
This target exercises Jobs' actual manifests, Make recipes, hook refresh and
preservation behavior. Shared Tooling owns the general hook/frontend test matrix;
this repository has no frontend formatter integration.

Standard formatting reports each manifest/Rustfmt pass and its elapsed time.
Eligible unchanged inputs can reuse a successful result after tool admission;
`make fmt-check FORMAT_RUST_ARGS=--no-cache` explicitly runs every pass. Shared
eligibility rules keep unsupported source layouts and staged hook worktrees
uncached, with failed formatter logs retained for diagnosis.

The unpublished harness uses the [selected Testkit dependency](Cargo.toml) and
its separately admitted CLI: run `make install-testkit-tools` and
`make testkit-tools-check` when updating that selection. PocketIC setup/check and
live execution remain separate; see [consumer qualification](apps/ic-jobs-test-consumer/README.md).
Common setup and ordinary CI do not launch PocketIC.

Release metadata qualification additionally selects IC Build's published
`ic-build-release-metadata` executable through `make install-release-tools` and
offline `make release-tools-check`. See [release preparation](docs/releasing.md#preparation)
for its explicit path and byte allowance. The capture CLI uses Clap for argument
parsing and preserves the original/optimized capture invocation forms.

## Delivery

The public repository is [dragginzgame/ic-scheduling](https://github.com/dragginzgame/ic-scheduling).
Select `RELEASE_GROUP=jobs` or `RELEASE_GROUP=timers` explicitly for releases,
resume and publication. There is no default release group or shared root version.
The [group policy](ci/cargo-release-groups.json) covers all nine members.
See the [Jobs ledger](releases/jobs/CHANGELOG.md), [Timers ledger](releases/timers/CHANGELOG.md)
and [shared history through 0.33.1](releases/history.md).

`make release-status` observes both groups' current, latest published and proposed
versions, exact-commit CI, release tags and uncommitted/locked inputs. Use
`RELEASE_GROUP=jobs` or `timers` to narrow it, `STATUS_OFFLINE=1` for local-only
observations and `STATUS_JSON=1` for structured output. It installs nothing and
changes no source, refs, registry or release state. Unavailable network/cache
observations remain unknown or blocked; a failed working-graph admission is
explicitly marked when versions are projected from committed HEAD instead.
The standard release/publication commands retain their independent full admission.
`make version` inventories both groups; add `RELEASE_GROUP` to inspect one.
`make RELEASE_GROUP=jobs package` verifies its crate
archive offline; `make publish-check` performs a publication dry run with registry
access. Both permit working edits and upload nothing.

Explicitly authorized `make release-patch`, `make release-minor` and
`make release-major` commands run the complete gate, prepare metadata, commit,
tag (`jobs-vX.Y.Z` or `timers-vX.Y.Z`) and atomically push to `origin`/`main`.
Both groups retain the full combined release gate. Package upload is a separate
`make publish` operation requiring clean source and the matching pushed annotated
tag. See [delivery procedure](docs/releasing.md) for prerequisites and interruption
recovery. Configuring these commands does not authorize a release or upload.
