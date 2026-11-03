# IC Scheduling agent instructions

Apply the reviewed [Dragginzgame baseline](DRAGGINZGAME.md) first.
Its exact revision is recorded in [.shared-tooling.snapshot](.shared-tooling.snapshot).
This file is the local overlay. Read [the current handoff](docs/status/current.md).

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
- Follow the [shared validation cadence](DRAGGINZGAME.md#validation-and-evidence).
  Local focused checks include package tests, MSRV/Wasm checks and tooling
  fixtures; `make ci` is the full repository suite. Releases use the qualified
  [group plan](docs/release-validation-groups.md): Jobs-only changes may omit
  standalone Timers installed checks; shared, Timers or uncertain inputs retain
  them. Explicit live PocketIC qualification remains separate from native CI.
- Timers owns the sole private IC timer provider; retain its unsafe FFI confinement,
  dependency checks, runtime qualification and policy cohort gates. Jobs stays
  Serde-only and does not acquire a runtime dependency on Timers.
- Both libraries and all probes belong to this root workspace. Issue #82 approves
  independent `jobs` and `timers` release groups in `ci/cargo-release-groups.json`.
  Both groups use explicit matching package versions, including all private
  qualification members. There is no root workspace version. Group ledgers live
  at `releases/jobs/CHANGELOG.md` and `releases/timers/CHANGELOG.md`; the root
  changelog is an index. Release/resume/publication requires an explicit group.
  The grouped root dependency catalog lists every member's path/full version and
  ledger; policy alias rosters cover each catalog entry. Dependencies and the lock
  remain root-owned. Sibling Timers source is provenance,
  not a build input. Grouping does not relax runtime or combined-consumer checks.
- Rust floor and pinned compiler: 1.99.1, approved in [#87](https://github.com/dragginzgame/ic-scheduling/issues/87).
  This owner-approved policy supersedes the old lowest-floor promise. Qualify core
  and consumer-owned Timers integration separately with the same compiler; preserve
  native host, Wasm, archive and explicitly selected installed-runtime coverage.
- Host workflows support Linux and macOS Intel/Apple Silicon. Bind native and
  live IC recovery qualification to their tested source and dependency graph;
  native tests do not establish IC recovery.
- Use the common contribution and release authority rules. The public remote is
  `dragginzgame/ic-scheduling`; configuring tooling does not authorize releases or
  publication. Direct release delivery uses `origin`/`main`; `make publish` is
  a separate, explicitly authorized crates.io upload.

## Runtime integration

Timers and CDK probes select Runtime directly; native DTOs use canonical Types.
Root manifests and locks own the supplier cohort. Preserve the exact Runtime
guard and explicitly selected installed IC recovery qualification. Supplier
adoption or a dependency blocker never authorizes another repository's release.
