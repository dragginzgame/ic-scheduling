# IC Scheduling agent instructions

Apply the installed [Toko Tooling baseline](TOKO-TOOLING.md) and its
revision-bound references. [.toko-tooling.snapshot](.toko-tooling.snapshot)
records the adopted payload; do not edit it or follow a moving sibling baseline.
This overlay owns product safeguards, local commands and approved exceptions.
The baseline owns common action authority, validation, compiler and reporting rules,
including scoped `git add -N` when validation requires new task-owned paths.

Both workspace catalogs/locks select the same tooling release and installed
receipt. Its exact-pin exception is in `ci/dependency-pinning-exceptions.json`.
After changing both pins and locks, use the repository-root installer.
Installed executable paths are under `vendor/toko-tooling/`; local callers stay
under `scripts/`, and Git hooks select `vendor/toko-tooling/.githooks`.

Product commands use the root manifest; private checks use
`--manifest-path qualification/Cargo.toml`. `make cloc-qualification` reports the
private workspace separately. Release plans select the standard shared
`qualification_workspace` transaction; see `docs/releases/validation-groups.md`.
`make check-pins` uses the common declaration/workflow/lock and inheritance
checker, including explicitly admitted private qualification versions.
Private packages belong to no product release group.
Read [the current handoff](docs/status/current.md).

- Scope is this workspace; other sibling repositories remain read-only.
- In `ic-jobs`, keep metadata/storage authority consumer-owned. Do not add lifecycle exports,
  a database dependency, global handler registry or direct IC timer provider.
- Every restore path uses the validated Job::restore boundary. Keep error
  transitions unchanged and uncertain effects blocked until explicit disposition.
- New storage/scheduling policy must have a concrete consumer requirement.
- Consumer integration uses Storage's checked allocation/backing APIs and its
  test-support IO meters. The retired `ic-memory` crate must not be reintroduced.
- For Canic adapter work, read [the integration guide](docs/integration/canic.md).
  It describes a proposed consumer integration, not an implemented Canic API.
- Follow the [shared validation cadence](TOKO-TOOLING.md#validation-and-evidence).
  Local focused checks include package tests, MSRV/Wasm checks and tooling
  fixtures; `make ci` is the full repository suite. Releases use the qualified
  [group plan](docs/releases/validation-groups.md): proven Jobs-only changes select
  Jobs and combined-consumer obligations, omitting unchanged standalone Timers
  and release-tooling fixtures. Shared, Timers or uncertain inputs retain the
  complete roster. Explicit live PocketIC qualification stays separate from native CI.
- Timers owns the sole private IC timer provider; retain its unsafe FFI confinement,
  dependency checks, runtime qualification and policy cohort gates. Jobs stays
  Serde-only and does not acquire a runtime dependency on Timers.
- Both libraries belong to the product root; six private harness/probe packages
  belong to the independent `qualification/` workspace and its own catalog/lock.
  The verified inventory is `ci/workspace-manifests.txt`. Issue #82 approves
  independent `jobs` and `timers` release groups in `ci/cargo-release-groups.json`.
  Both groups contain only their published product package and use literal
  package versions. Private qualification versions stay unchanged during releases. There is no root workspace version. Group ledgers live
  at `releases/jobs/CHANGELOG.md` and `releases/timers/CHANGELOG.md`; the root
  changelog is an index. Release starts require an explicit group or complete
  frozen group roster; batch recovery/publication use the saved group context
  selected by an explicit `BATCH`. Standalone publication requires an explicit group.
  Group membership and ledgers belong to the release JSON policy; `make deps-list`
  reports every product and private package. Catalogs retain only used dependency
  entries; group alias rosters cover those remaining product entries. Qualification
  owns its used catalog, current product source paths and lock. Sibling Timers source is provenance,
  not a build input. Grouping does not relax runtime or combined-consumer checks.
- The private Jobs qualification canister is `qualification/ic-jobs-test-consumer`;
  it is qualification-only, not a product or Canic fleet. Timers owns downstream
  Runtime composition qualification; see [its acceptance map](docs/timers/development.md#runtime-composition-qualification)
  when changing the selected Runtime cohort or call/executor integration.
- Rust floor and pinned compiler: 1.100.0, approved in [#93](https://github.com/dragginzgame/ic-scheduling/issues/93).
  This supersedes the 1.99.1 policy in [#87](https://github.com/dragginzgame/ic-scheduling/issues/87). Qualify core
  and consumer-owned Timers integration separately with the same compiler; preserve
  native host, Wasm, archive and explicitly selected installed-runtime coverage.
- Both roots own the common lint minimum and retain missing-docs denial and
  unsafe-code prohibition. Timers cannot inherit `unsafe_code=forbid`: its local
  table mirrors the minimum with `deny` and a private FFI expectation. Its reviewed
  pedantic, const-constructor and formatting checks remain; IC-local futures carry
  no Send promise. All other packages inherit their owning root's lint table.
- Host workflows support Linux and macOS Intel/Apple Silicon. Bind native and
  live IC recovery qualification to their tested source and dependency graph;
  native tests do not establish IC recovery.
- The public remote is `dragginzgame/ic-scheduling`. Direct release delivery uses
  `origin`/`main`; `make publish` uploads the explicitly selected group to crates.io.

## Runtime integration

Timers and CDK probes select Runtime directly; native DTOs use canonical Types.
Each owning workspace manifest/lock owns its supplier cohort. Preserve the exact Runtime
guard and explicitly selected installed IC recovery qualification. Supplier
adoption or a dependency blocker never authorizes another repository's release.
Storage's IC0-only Runtime may differ from Timers' executor generation; graph
admission must retain one executor guard and one IC0 backend. Native Testkit's
identity generation stays isolated from deployed DTO/entrypoint identities,
with explicit transport conversions and executor-free native harness checks.
