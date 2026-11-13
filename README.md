# IC Scheduling

A product workspace for durable jobs and observable timer primitives on the
Internet Computer. Jobs and Timers have independent release versions within one
dependency catalog, lockfile, toolchain and CI.

Repository-owned source, tests, tooling, documentation, fixtures and generated
materials are licensed under the [Toko Studios Network Licence 1.0](LICENSE.md).
External dependencies retain their own terms. Contributions follow the canonical
[contribution policy](CONTRIBUTING.md).

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
and [acceptance checklist](docs/testing/consumers.md#adapter-acceptance-checklist)
describe the host obligations. The [Canic integration guide](docs/integration/canic.md)
maps those obligations to the proposed adapter; it does not advertise an existing
Canic API. Jobs depends only on Serde, so Canic owns its integration dependency
versions. The unpublished [canister consumer](qualification/ic-jobs-test-consumer/README.md)
demonstrates application-owned storage, management and lifecycle integration.
Its PocketIC harness is compiled by CI; live qualification is explicitly selected.

## Scope and validation

The library owns metadata and checked transitions. Applications own storage,
payloads, handlers, longer audit history and disposition of uncertain effects.
It supplies no global queue, database, lifecycle exports or exactly-once effects.

| Boundary | Owner and entry point |
| --- | --- |
| Validated records, attempts, transitions and due-time selection | Published `ic-jobs`: `Job::restore`, `Job` methods, `next_due_ns` and `start_next`; Serde is its only runtime dependency |
| Payloads, authorization, stable queue and lifecycle endpoints | Consumer applications; the unpublished `qualification/ic-jobs-test-consumer` demonstrates this integration |
| Stable allocation and fixture Cell admission | IC Storage supplies checked allocation leases and bounded Cell framing; the consumer validates its queue payload |
| Volatile wakeups and watchdog recovery | IC Timers, called directly by the consumer |
| Principal and explicit-codec canister runtime | Published IC Platform; Jobs retains endpoints, schemas, limits and error policy |
| Installation, management and delivery encoding and bounded decoding | Published IC Wire; Jobs retains method identities, limits and receipt policy |
| PocketIC transport and startup | IC Testkit; Jobs supplies Wire frames and retains scenario policy |
| Shared native management client | Feature-gated `ic-jobs-canister-tests::harness`, used by the recovery tests |
| Artifact retention and native filesystem/process mechanics | IC Build and IC Host; Jobs retains optimizer flags and evidence naming |
| Common setup, formatting and release mechanics | Reviewed Toko Tooling snapshot; Jobs owns its gates, direct-delivery policy and package upload admission |

## Dependency exposure

Only `ic-jobs` and `ic-timers` are publishable. All six supporting packages
set `publish = false`; their separation keeps host tooling out of Wasm and
qualification dependencies out of the libraries.

| Private package | Purpose |
| --- | --- |
| `ic-jobs-test-consumer` | Consumer-owned durable state, qualification canister and native composition tests |
| `ic-jobs-canister-tests` | Lightweight native artifact capture; explicit `harness` feature for installed recovery tests |
| `ic-timers-pocketic` | Native runtime qualification and cohort runner |
| `ic-timers-runtime-probe` | Wasm canister exercising timer runtime behavior |
| `ic-timers-size-probe` | Separate minimal Wasm builds measuring each timer policy's size |
| `ic-timers-test-protocol` | Wire types shared by the native runner and Wasm probes |

Each workspace catalog selects its direct dependencies; members inherit only
what they use. The product root contains Jobs and Timers and their release groups.
Six private packages belong to [qualification/Cargo.toml](qualification/Cargo.toml),
with an independent lock and unchanged private versions. They are validation
obligations, not product release-group members. Product packages never depend
on qualification. `make cloc` includes both workspaces; `make cloc-qualification`
reports the private workspace separately with its existing LOC categories.
`make deps-list` reports all eight local packages, their versions and product
release groups without network access. Cargo catalogs contain used dependencies;
the release JSON policy owns complete product group membership. Metrics, Platform
and Thiserror are production dependencies of Timers. Paired grouped sort commands
run in `make fmt`, `make fmt-check` and the formatting hook; release metadata checks
use the same sorting policy for working and selected commits.

| Built artifact | Candid exposure | CDK exposure |
| --- | --- | --- |
| Published `ic-jobs` library | None | None; normal dependencies contain only Serde |
| Published `ic-timers` library | None | Runtime's executor and IC0; no full CDK |
| Qualification `ic_jobs_test_consumer.wasm` | None; all owned inputs/replies use Wire | Runtime's Wasm-only `cdk` feature supplies entrypoints; upstream `ic-cdk` is absent, and Timers shares that executor |
| Native recovery tests | PocketIC requires Candid for external simulator/management protocols; Jobs RPCs use Wire | Runtime's `cdk` feature is enabled only for the qualification Wasm |
| `jobs-capture-artifact` host binary | No Candid dependency in its normal/build graph | No CDK dependency in its normal/build graph |

The Wasm above is an unpublished test fixture, not a production service. This
checkout does not establish what other deployed canisters use. Audit each actual
application's selected target graph and source rather than counting manifests.
For example, inspect the qualification Wasm's paths with:

```sh
cargo tree --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --target wasm32-unknown-unknown --edges normal,build --invert ic-platform-runtime --locked --offline
cargo tree --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --target wasm32-unknown-unknown --edges normal,build --invert ic-platform-runtime-guard --locked --offline
```

The selected published boundary is IC Platform, with defaults disabled.
The [workspace catalog](Cargo.toml) owns requirements; [Cargo.lock](Cargo.lock)
owns the resolved versions. The [handoff](docs/status/current.md) records the
selected adoption and its source-bound evidence.
The qualification app selects Runtime's `cdk` feature only on Wasm. Native
consumer builds select canonical Types directly for Serde DTOs
and Wire contracts. Jobs, Storage and Timers remain Wasm/test dependencies; JSON
is limited to native fixtures and tooling. Parser, Agent,
certification and crypto features are not selected. Runtime 0.22.1 owns the
deployed executor and generator shared with Timers and Wire Endpoint 0.13 /
Wire 0.22. Storage 0.74 uses that same Runtime through its IC0-only dependency.
The qualification Wasm
selects neither Candid nor full `ic-cdk`.
Owned native DTO principals exclude external Principal and SHA-2 dependencies.
The Wasm application selects neither registry Principal conversion nor SHA-2 hashing.
Storage supplies native IO metering under its test-support feature.
Native tests select `principal-self-authenticating` explicitly;
DTOs select Types 0.20 without the retired `principal-external` feature. The
native harness uses canonical Types Principal for application DTOs and Testkit
0.102.0's upstream `ic_testkit::pocket_ic::nonblocking` client. Callers own their
Tokio runtime and retain the managed server through awaited best-effort closure.
Each client selects a 60-second polling budget, separate from its server startup
deadline. Upstream transport panics and logging behavior apply.
Managed `PocketIcStartupConfig::build_instance` supplies its
explicit server endpoint and retains server custody. The deployed
graph has one Runtime 0.22 executor, exact matching guard and IC0 backend.
Runtime, Endpoint and application DTOs share canonical Types 0.20. Native
transport converts these identities explicitly to upstream PocketIC identities;
its management DTOs use Types 0.20.1 / Platform 0.32.1 at that native
boundary. DTO-only graphs stay free of
registry Principal, Candid and runtime dependencies. Stored identities and Wire
frames are unchanged.

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
Storage 0.73 requires fresh whole-store backing, including replacement of 0.71
installations. It rejects earlier physical
images without writes or growth; no automatic migration or reset is added.
Use fresh qualification state after explicitly retiring the old installation;
resolve its Running/Uncertain effects with the old binary and exact evidence first.
The [consumer guide](qualification/ic-jobs-test-consumer/README.md) describes the boundaries, and the
[handoff](docs/status/current.md) records qualification by source. Native Testkit
uses its published Platform facade for external simulator interfaces. The
harness keeps the upstream Principal for PocketIC calls and converts to its
owned Principal when constructing application DTOs. No decoder or transport is replaced.
Wire 0.22 retains upstream Postcard acceptance: redundant varints within width bounds
decode successfully, while over-width values still refuse. Encoder bytes, method
identities and budgets are unchanged; retain original frames for exact retries or hashes.

For the production library, locked normal/build graphs on native Linux and Wasm,
with all features selected, contain Serde and its macro dependencies and exclude
every tests/tooling-only catalog dependency (apart from the selected root itself).
The qualification Wasm intentionally includes its test integrations above; those
cannot be prohibited there using the production library's exclusion list.

`make check-candid-boundary` scans first-party Rust under `crates/`
with the [shared lexical boundary check](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/docs/validation/candid-boundary.md), its CBOR mode
and [empty Candid/CBOR allowance policy](ci/candid-boundary.json). Owned application
DTOs use Wire; external types and CBOR codecs stay with Platform. This runs once as a prerequisite
of the common dependency-boundary stage, including CI and selected release plans.
It does not replace review of generated code or cross-module aliases.

`make check-dependency-boundaries` runs the reviewed shared checker and is part of
CI/release validation. It composes the [shared IC inventory](ci/external-ic-dependencies.txt),
[CBOR package inventory](ci/cbor-dependencies.json)
with [local bans](ci/cargo-dependency-boundaries.json) on direct generator and
retired decoder dependencies, including aliases,
across member normal/build/dev/target declarations. It does not enforce feature
selection, inspect unused root catalog
entries; Make explicitly checks both workspace roots and scans source from both
trees. Their catalogs, locks and feature selections are reviewed separately.
Cargo metadata verifies the explicit eight-member inventory; there are no browser roots or
maintained generated Rust entrypoints. Macro expansion, generated strings and
arbitrary cross-module aliases still need review. Historical CBOR backing images
remain refusal fixtures; no application encoding or retained bytes change here.

`make check-dependency-graphs` adds locked, offline normal/build graph admission
in CI/release validation: all-feature native and Wasm core graphs exclude the
local tests/tooling and IC integrations; the all-feature qualification Wasm excludes
host-only packages, JSON, Candid and full CDK. It requires one Timers, Platform runtime guard and entrypoint generator; registry
executor and IC0 packages are excluded. The application, Timers and Storage share one Runtime identity through the
selected Platform owners. Storage owns its collections; the deployed graph
excludes the retired Memory package and stable-structures substrate.
The graph excludes registry Principal conversion and SHA-2.
A fourth context excludes Jobs, Memory, Storage, Timers, JSON and the IC runtime from
ordinary native consumer builds, including the
Platform `cdk` feature.
The fifth context keeps Candid, PocketIC, Tokio and Testkit out of the native capture
binary's normal/build graph. Native Testkit uses Platform's external-protocol
adapters at the Candid boundary; PocketIC itself uses Candid
management and transport DTOs. Testkit uses Platform's owned management codec to
prepare cached snapshots with coherent heap, timer and hook state. Disabling
dependency defaults
cannot remove those required edges. Jobs-owned RPCs continue to use Wire.
The policy lives in Make; Shared owns the [observation/checker](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/docs/validation/cargo-graph-checks.md).
The [snapshot manifest](.toko-tooling.snapshot) records the reviewed Shared
Tooling version, registry archive checksum and payload digests. No sibling checkout is a
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
Wire conformance and the packaged adapter example on Rust 1.100.0. They cover commit
failures, reconstruction, exact receipt reconciliation, stale continuations and
bounded recurring work. The composition fixtures request IC Storage allocations
through the [qualification-selected dependency](qualification/Cargo.toml); the host fixture reopens actual
stable cells. See [test consumers](docs/testing/consumers.md) for their storage and
timer boundaries.

Choose checks for the affected caller. For native consumer storage/timer
composition, use `bash scripts/dev/minimum-rust.sh cargo test --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer --test composition
--locked --offline`; for persisted Wire records, select `--test stored_wire`;
for endpoint codecs, select `--test api_wire`. Run the packaged adapter example
with `bash scripts/dev/minimum-rust.sh cargo run -p ic-jobs --example persisted_job --locked --offline`.
Qualify the core Rust floor separately with `bash scripts/dev/minimum-rust.sh cargo check -p ic-jobs
--lib --locked --offline`. For the consumer's Wasm floor, use `bash scripts/dev/minimum-rust.sh cargo
check -p ic-jobs-test-consumer --lib --target wasm32-unknown-unknown --locked
--offline`. Lint the affected consumer with `cargo clippy --manifest-path qualification/Cargo.toml -p ic-jobs-test-consumer
--all-targets --locked --offline -- -D warnings`, or select the Wasm target
when runtime integration changes.

The compiler launcher reads `workspace.package.rust-version` from
[Cargo.toml](Cargo.toml) and requires the same pin in `rust-toolchain.toml`.
`make rust-policy-check` records and admits actual Cargo/rustc identities without
installing anything. Both hosted workflows prepare that single compiler, its
components and Wasm target once, after preparing the host parsers. Full CI retains
independent core, consumer, Wasm and archive coverage on that compiler; the focused
`check-msrv` target remains available without repeating it in the aggregate.

Both workspace roots define the common Rust, Clippy and Rustdoc lint minimum.
Safe packages inherit their root's unsafe prohibition and missing-docs denial.
Timers mirrors the minimum in its documented package exception so its private FFI
can use a narrow expectation; its reviewed pedantic and individual style checks
remain. Line-count advice and whole nursery/restriction groups are excluded.
`bash scripts/ci/test-consumer-tooling.sh lint-policy` checks every actual member
and compiles disposable consumers that reject ignored Results and unsafe declarations.

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

The package and its direct Timers qualification consumer require Rust 1.100.0. Compiler
selections are recorded in [rust-toolchain.toml](rust-toolchain.toml) and
[the local tool catalog](ci/tool-versions.env). Linux and macOS Intel/Apple Silicon
host workflows are maintained. See [the current handoff](docs/status/current.md)
for source-bound checks and outstanding native/live qualification; native tests
do not establish IC message rollback or upgrade recovery.

## Repository setup

The reviewed [Toko Tooling snapshot](.toko-tooling.snapshot) records the
installed registry archive checksum, entrypoints and payload digests. The
[common Make interface](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/docs/adoption/make-interface.md) owns setup, formatting, hooks,
snapshot checks and release routing. Jobs retains product checks, dependency
selections and release/publication policy.

The [documentation index](docs/README.md) covers Scheduling-owned guides by
topic. Shared reference guides remain upstream at reviewed commit-pinned links;
only the root [Toko baseline](TOKO-TOOLING.md), required executable tooling and
runtime inputs are installed here. The snapshot receipt identifies those managed
files. See the [online-reference migration](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/docs/adoption/migrations.md#online-reference-documentation-in-0395).

Both workspace catalogs pin `toko-tooling = "=0.41.1"`; every real
member inherits it as a dev-dependency with an intentional cargo-machete exemption.
Cargo.lock selects its registry checksum, and `.toko-tooling.snapshot` records
the authenticated installed files. After editing the root pin, run
`cargo update -p toko-tooling` in both workspaces, then `make toko-tooling-update`.
For deliberate dependency maintenance, `make deps-update` runs `cargo update -v`
first in the product workspace, then in `qualification/`, stopping on failure.
Use `DEPENDENCY_PACKAGE=name DEPENDENCY_PRECISE=version` to narrow both updates.
Completed lockfile changes remain if a later update fails; review both locks.
This command does not change manifests or install tooling after a pin change.
`make deps-list` is the installed offline package/version/group inventory for both
workspaces. Cargo dependency/metadata lints are denied in both roots and Timers'
equivalent local table. Ordinary Clippy recipes use `CARGO_BUILD_WARNINGS=deny`;
Rustdoc retains explicit warning denial, including malformed table checks.
`make toko-tooling-install` installs the existing Cargo selection; neither
installation command edits Cargo.lock. CI, release and publication entrypoints
require `make toko-tooling-check` before effects. See the
[installation guide](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/docs/adoption/consuming-snapshots.md#cargo-selection-and-installation).

Run `make install-tools` to prepare common host/Rust tools and Jobs' selected IC
bundle for optimized qualification; `make tools-check` admits them offline.
Shared Tooling's common Rust set contains cargo-sort and cargo-sort-derives.
Jobs has no Candid extractor caller and selects no extractor installation; older
retained installations are preserved outside the newly selected common PATH.
For an interactive shell, prepend
`$PWD/.tools/host/bin:$PWD/.tools/ic/bin:$PWD/.tools/rust/common/bin` to PATH.
Use the same `TOKO_TOOLING_STORE` for setup, checks and evidence collection;
see [local setup](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/docs/adoption/local-setup.md) for prerequisites, storage and reuse rules.
`make help` lists check groups and points to focused selections.
`make install-hooks` enables the reviewed formatting hook after tool preparation;
`make formatting-hook-check` verifies it in disposable repositories.
This target exercises Jobs' actual manifests, Make recipes, hook refresh and
preservation behavior. Shared Tooling owns the general hook/frontend test matrix;
this repository has no frontend formatter integration.

`make cloc` reports library, test, example and tooling source separately. Private
qualification packages declare their roles in Cargo metadata; see
[source layout and LOC reporting](docs/operations/source-layout.md).
Use `make cloc CLOC_DEPTH=1` for module rows, or
`bash scripts/dev/cloc.sh -d 1 ic-timers::runtime` to inspect one source branch.
Use `bash scripts/dev/cloc.sh --wide .` to show full names in table output;
JSON reports always retain full names.

Standard formatting reports each manifest/Rustfmt pass and its elapsed time.
Eligible unchanged inputs can reuse a successful result after tool admission;
`make fmt-check FORMAT_RUST_ARGS=--no-cache` explicitly runs every pass. Shared
eligibility rules keep unsupported source layouts and staged hook worktrees
uncached, with failed formatter logs retained for diagnosis.

The unpublished harness uses the [selected Testkit dependency](Cargo.toml) and
its separately admitted CLI: run `make install-testkit-tools` and
`make testkit-tools-check` when updating that selection. PocketIC setup/check and
live execution remain separate; see [consumer qualification](qualification/ic-jobs-test-consumer/README.md).
Common setup and ordinary CI do not launch PocketIC.

Release metadata qualification additionally selects IC Build's published
`ic-build-release-metadata` executable through `make install-release-tools` and
offline `make release-tools-check`. See [release preparation](docs/releases/README.md#preparation)
for its explicit path and byte allowance. The capture CLI uses Clap for argument
parsing and preserves the original/optimized capture invocation forms.

## Delivery

The public repository is [dragginzgame/ic-scheduling](https://github.com/dragginzgame/ic-scheduling).
Select `RELEASE_GROUP=jobs` or `RELEASE_GROUP=timers`, or an explicit frozen roster
such as `RELEASE_GROUPS=timers,jobs`. There is no default group or shared
root version. Batch recovery selects the saved roster with `BATCH`.
The [group policy](ci/cargo-release-groups.json) covers the two products; private
qualification packages keep independent versions and remain required by validation
plans. Shared multi-lock preparation covers both manifests/locks with rollback;
see [release validation and workspace-owned pin checks](docs/releases/validation-groups.md).
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
Releases capture clean committed source in Shared's managed frozen checkout.
Later normal-checkout lock edits remain pending while the captured graph finishes.
Include crates.io publication in that same batch when authorized:

```sh
make RELEASE_GROUPS=timers,jobs RELEASE_PUBLISH_TARGET=publish release-patch
```

Omitting the publication target delivers Git only. The normal branch is advanced
explicitly through `release-batch-reconcile BATCH=...` after preserving deferred
edits; it is never reset or automatically merged. Each group retains its qualified
validation plan and clean/tag-bound publication checks. See the
[delivery procedure](docs/releases/README.md) for setup, recovery and retirement.
Configuring these commands does not authorize a release or upload.
