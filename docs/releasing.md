# IC Scheduling release and publication

Jobs and Timers stay in one development workspace with two independent release
identities, approved in [#82 — Version Jobs and Timers independently](https://github.com/dragginzgame/ic-scheduling/issues/82).
The policy is [ci/cargo-release-groups.json](../ci/cargo-release-groups.json).
The grouped root Cargo dependency catalog lists every member's full version and
path once, beside its group's ledger. Each policy alias roster covers the complete
catalog, including private qualification members that no other member uses.

| Group | Version authority | Canonical changelog | Published package |
| --- | --- | --- | --- |
| `jobs` | `crates/ic-jobs/Cargo.toml` literal `package.version` | [Jobs](../releases/jobs/CHANGELOG.md) | `ic-jobs` |
| `timers` | `crates/ic-timers/Cargo.toml` literal `package.version` | [Timers](../releases/timers/CHANGELOG.md) | `ic-timers` |

Every public and private group member has an explicit version equal to its
group's primary package. The root has no `workspace.package.version` and its
changelog is an index. Jobs owns three private qualification packages; Timers
owns four private probe/harness packages.
All remain unpublished. Earlier coordinated history through 0.33.1 is retained
once in [shared history](../releases/history.md); its existing
`v0.33.1` tag remains historical. Group adoption changes no numeric version.

## Commands and delivery

Every command below accepts `RELEASE_GROUP=jobs` or `RELEASE_GROUP=timers`.
Release, resume and publication reject missing, empty or unknown selections
before setup or validation. `PACKAGE` never selects a group; a supplied package
must belong to the explicit group. Read-only `make version` inventories both
groups when no selection is supplied. For example:

```sh
make version
make RELEASE_GROUP=timers version
make RELEASE_GROUP=jobs release-patch
make RELEASE_GROUP=timers release-minor
make RELEASE_GROUP=timers release-resume VERSION=X.Y.Z
```

Release execution remains maintainer-owned. A notes or implementation request
never authorizes a release, tag, push or upload. Both groups use the existing
[Shared release runner](releases.md#explicit-cargo-release-groups), direct-only
[delivery policy](../ci/release-delivery.txt), `origin` and `main`.
Group tags are `jobs-vX.Y.Z` and `timers-vX.Y.Z`; commit subjects are
`Release jobs X.Y.Z` and `Release timers X.Y.Z`. No old tag is renamed.
Unfinished attempts from earlier layouts retain their recorded source, policy,
adapter and file plan, including earlier grouped attempts. Resume them against
that recorded layout; adoption does not convert journals or reinterpret tags.

`make release-files` emits the selected group's exact NUL-delimited file plan.
Each group admits the root manifest/lock, its ledger under `releases/` and all
of its literal-version member manifests. The root index and shared history are
outside every release file plan and are never finalized as group notes.
The Timers path requirement in the root catalog follows a Timers release;
Jobs' package identity stays unchanged. This updated consumer declaration is
not republished under an existing Jobs version. Release Jobs separately when
its own contract requires it. Jobs-only preparation preserves that requirement
and all Timers versions and notes. Shared admission refuses incomplete group
membership or unlisted local path/version requirements.

One repository release lock protects the shared Cargo lock. Plans are retained
at `.git/release-state/jobs-X.Y.Z.plan` or `timers-X.Y.Z.plan`, and bind group,
source, destination, phase and exact file custody. Resume selects the same group
and version and reconciles saved intent before any fresh preparation. Keep
failed attempts, original metadata, logs and artifacts. Resolve uncertain remote
or registry observations before retrying; do not recreate tags or force-push.

The adapter also keeps `GROUP-X.Y.Z.receipts/validated` and `prepared` beside
the journal. Shared's receipt helper binds validation to the source, previous/
next versions, date, destination, group and policy blob. The prepared receipt
binds the exact selected metadata bytes. Preparation and every subsequent check
admit these receipts, including selected-commit checks after commit/tag. A new
validation archives prior receipt evidence before replacing it; failed or missing
receipts stop recovery rather than authorizing another effect.

## Preparation and verification

Rust 1.99.1 is both the consumer floor and pinned release compiler. The release
CI gate runs `rust-policy-check` before product compilation and records actual
Cargo/rustc identities. Native and Wasm package coverage, archive verification
and the explicit installed watchdog/cohort gate remain required.

Prepare selected dependencies with `cargo fetch --locked`. Use
`make install-tools`, `make tools-check`, `make install-testkit-tools` and
`make install-release-tools` for explicit setup and offline admission. The
root lock selects the group-capable IC Build 0.73.6 editor; no alternate editor
or handwritten metadata renderer is used. Standard fresh release preflight
admits source and notes, prepares the locked cache and tools, then admits the
editor. Resume admits the saved source's editor selection without fresh setup.

Shared metadata preparation changes only the admitted group's versions,
declared requirements, local lock rows and selected ledger. It retains originals
and candidates, restores selected metadata on handled failure, and keeps failed
evidence. Source admission and exact staging remain consumer/runner boundaries.
Abrupt interruption still requires inspecting the retained attempt before resume.

Both groups retain the complete `release-verify` gate: combined CI, Testkit/server
preparation, installed Timers watchdog and policy-cohort qualifications. Group
selection does not narrow it. CI packages both public crates with `package-all`.
Independent core qualification stays explicit; the Jobs model remains Serde-only.
The real Jobs consumer still composes Jobs, Timers, Memory, Wire and Platform.
Its commit-before-dispatch, exact attempts/receipts, uncertain effects and
trap/upgrade obligations remain mandatory. One lockfile continues to resolve all
nine members; release groups do not isolate incompatible runtime stacks.

## Publication

`make package` packages only the selected public crate offline. `make publish-check`
runs the complete Clippy gate followed by the selected package's crates.io dry run;
it allows working edits and uploads nothing. `make publish` is a separate,
explicitly authorized upload of the selected package only. Cargo credentials must
permit that package. No private group member is eligible for publication.

Publication requires clean source and the selected group's annotated tag at HEAD,
with the identical tag object observed at the sole push URL. Registry observation
must confirm the exact selected package version is absent; existing, unknown or
failed observations block upload. Identity/source checks repeat after network
admission and Clippy. An interrupted upload is reconciled against crates.io before
retrying. These commands do not bump versions, commit, tag or delete artifacts.

## Focused fixtures

`make test-release-tooling` exercises both actual consumer adapters in disposable
Git repositories and bare remotes. It checks exact plans/commits, unrelated
versions/requirements/ledgers, manifest/lock rollback, group-tag delivery, retained
plans, cross-group resume refusal, same-group resume and publication selection.
Validation-receipt tampering refuses before metadata mutation.
The full product gate, tool setup and registry/upload effects are substituted;
Cargo metadata, the selected metadata editor and Git effects are real and local.
Focused subsets are `test-release-tooling-metadata`, `-recovery`, `-delivery`,
`-publication` and `-retention`. The latter selects failed-candidate custody checks.
These fixtures do not establish installed recovery or native macOS execution.
Use the [current handoff](status/current.md) for source-bound qualification limits.
