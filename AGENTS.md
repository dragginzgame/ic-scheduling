# IC Scheduling agent instructions

Apply the reviewed [Toko Tooling baseline](TOKO-TOOLING.md) first.
Root `[workspace.dependencies]` selects exact `toko-tooling = "=0.38.2"`.
Every real member in both workspaces inherits its owning exact pin as a
dev-dependency with a cargo-machete exemption. Both selections must match the
single authenticated repository snapshot.
The exact pin prevents a Cargo update from silently changing the installer/receipt
contract; its scoped compatibility exception is in `ci/dependency-pinning-exceptions.json`.
Each Cargo.lock owns its archive checksum; [.toko-tooling.snapshot](.toko-tooling.snapshot)
records installed files. After editing both pins, resolve both Cargo locks with
Cargo and run `make toko-tooling-update` at the repository root;
`make toko-tooling-install` installs the existing selection. Never import Rust
APIs solely for tooling selection.
Product commands use the root manifest; private checks use
`--manifest-path qualification/Cargo.toml`. `make cloc-qualification` reports the
private workspace separately. Release plans select the standard shared
`qualification_workspace` transaction; see `docs/release-validation-groups.md`.
`make check-pins` retains the common declaration/workflow/lock checks and calls
the shared inheritance function with product-group and private qualification
version ownership. Keep all dependency-table checks; private packages belong
to no product release group.
This file is the local overlay. Read [the current handoff](docs/status/current.md).
Offline governance and directly used helpers remain explicitly selected; installing
the standalone baseline does not implicitly install its detailed references.

- Scope is this workspace; other sibling repositories remain read-only.
- In `ic-jobs`, keep metadata/storage authority consumer-owned. Do not add lifecycle exports,
  a database dependency, global handler registry or direct IC timer provider.
- Every restore path uses the validated Job::restore boundary. Keep error
  transitions unchanged and uncertain effects blocked until explicit disposition.
- New storage/scheduling policy must have a concrete consumer requirement.
- Consumer integration uses Storage's checked allocation/backing APIs and its
  test-support IO meters. The retired `ic-memory` crate must not be reintroduced.
- For Canic adapter work, read [the integration guide](docs/canic-integration.md).
  It describes a proposed consumer integration, not an implemented Canic API.
- Follow the [shared validation cadence](TOKO-TOOLING.md#validation-and-evidence).
  Local focused checks include package tests, MSRV/Wasm checks and tooling
  fixtures; `make ci` is the full repository suite. Releases use the qualified
  [group plan](docs/release-validation-groups.md): proven Jobs-only changes select
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
  The grouped root catalog lists product paths/full versions and ledgers;
  qualification owns its complete catalog, current product source paths and lock.
  Policy alias rosters cover each product catalog entry. Sibling Timers source is provenance,
  not a build input. Grouping does not relax runtime or combined-consumer checks.
- The private Jobs qualification canister is `qualification/ic-jobs-test-consumer`;
  it is qualification-only, not a product or Canic fleet. Timers owns downstream
  Runtime composition qualification; see [its acceptance map](docs/timers/development.md#runtime-composition-qualification)
  when changing the selected Runtime cohort or call/executor integration.
- Rust floor and pinned compiler: 1.99.1, approved in [#87](https://github.com/dragginzgame/ic-scheduling/issues/87).
  This owner-approved policy supersedes the old lowest-floor promise. Qualify core
  and consumer-owned Timers integration separately with the same compiler; preserve
  native host, Wasm, archive and explicitly selected installed-runtime coverage.
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
