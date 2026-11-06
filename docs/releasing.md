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
changelog is an index. Jobs owns two private qualification packages; Timers
owns four private probe/harness packages.
All remain unpublished. Earlier coordinated history through 0.33.1 is retained
once in [shared history](../releases/history.md); its existing
`v0.33.1` tag remains historical. Group adoption changes no numeric version.

## Commands and delivery

Single-group release starts accept `RELEASE_GROUP=jobs` or `RELEASE_GROUP=timers`.
A frozen batch can instead select its complete roster once through
`RELEASE_GROUPS=timers,jobs`. Missing/invalid selections and duplicate groups
refuse before setup. `PACKAGE` cannot override a frozen roster. Standalone archive,
publication and legacy recovery commands retain explicit single-group selection.
Read-only `make version` inventories both groups without a selection. For example:

```sh
make version
make RELEASE_GROUP=timers version
make RELEASE_GROUP=jobs release-patch
make RELEASE_GROUPS=timers,jobs RELEASE_PUBLISH_TARGET=publish release-patch
make release-batch-status BATCH=batch-XXXXXXXX
make release-batch-resume BATCH=batch-XXXXXXXX
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

Release starts capture clean committed source through Shared's
[frozen batch workflow](release-batches.md). Setup runs in the captured checkout
before version lookup. Later `cargo update` writes in the normal checkout stay
deferred and do not change the active graph, archives or publication. Initial
capture still refuses dirty source. Select `RELEASE_PUBLISH_TARGET=publish`
at batch start to include uploads; the default remains Git-only delivery.

The normal branch stays at its original commit. Preserve deferred work durably,
then use `make release-batch-reconcile BATCH=...` for a guarded fast-forward and
reapply the dependency changes without restoring older workspace versions. Resume
uses the batch's saved group, destination and source; uncertain publication needs
verified registry reconciliation before `PUBLICATION_RESULT=published` or
`not-published`. Completed resume never republishes. Retirement is a separate
explicit deletion command after preserving required evidence. See Shared's guide
for the full status/resume/reconcile/retire contract.

Shared 0.31 uses schema-2 batch intents with a recorded publication commit.
Finish, reconcile and retire pre-0.31 batches using their original reviewed
tooling before adoption. Retired release settings refuse; use the current
names in [Shared's upgrade instructions](release-batches.md#consumer-adoption).
Existing batch storage paths and source pins remain unchanged.

For an existing same-checkout journal, retain the legacy command
`make RELEASE_GROUP=timers release-resume VERSION=X.Y.Z` against its recorded
source/adapter. Shared Tooling 0.31.0 owns automatic single-group capture;
clearing `RELEASE_GROUPS` cannot turn a new grouped increment into live
same-checkout delivery.

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

One queue serializes frozen batches, and each captured repository's release lock
protects its Cargo graph. Its Git-owned `release-state/jobs-X.Y.Z.plan` or
`timers-X.Y.Z.plan` binds group,
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
root lock selects the group-capable IC Build editor; no alternate editor or
handwritten metadata renderer is used. Frozen setup prepares missing tools and
the locked cache before version lookup, and installs the captured format hook.
Fresh preflight re-admits that cache/tool selection offline. Resume admits the
saved source's editor selection without fetching or installing against partial
working metadata. Standard same-checkout preflight retains explicit preparation.

Shared metadata preparation changes only the admitted group's versions,
declared requirements, local lock rows and selected ledger. It retains originals
and candidates, restores selected metadata on handled failure, and keeps failed
evidence. Source admission and exact staging remain consumer/runner boundaries.
Abrupt interruption still requires inspecting the retained attempt before resume.

Both groups use the qualified [release validation plan](release-validation-groups.md).
Jobs-owned changes since its admitted annotated release baseline select the Jobs
and combined-consumer native/Wasm, lint, docs, graph and archive obligations.
Unchanged standalone Timers checks and release-tooling self-tests are explicitly
not applicable. Timers, shared inputs, changed manifests/locks and uncertain scope
retain the complete 37-stage release roster. All plans name individual checks;
none hides work inside `ci`. Repository CI retains all 33 common obligations on
its three native hosts, including both public crate archives. The `clippy`,
`docs-check` and `package-all` commands remain complete aggregates.
Independent core qualification stays explicit; the Jobs model remains Serde-only.
The real Jobs consumer still composes Jobs, Timers, Storage, Wire and Runtime.
Its commit-before-dispatch, exact attempts/receipts, uncertain effects and
trap/upgrade obligations remain mandatory. One lockfile continues to resolve all
eight members; release groups do not isolate incompatible runtime stacks.

Version-2 validation receipts bind Shared's admitted proof plus the canonical group plan/policy,
input checkpoint, native/compiler/build configuration and successful stage
evidence. Required stages are executed; excluded stages are not applicable.
No proof reuse is enabled. Durable evidence stays beside the group's journal;
missing, changed or failed evidence refuses preparation and recovery. Older
whole-workspace attempts must use their recorded source/adapters, rather than
having their receipts reinterpreted as group qualification.

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
The legacy adapter subfixtures substitute product gates, tool setup and
registry/upload effects; Cargo metadata, the selected editor and Git effects are
real and local. The frozen subfixture adds real captured setup, native/Wasm lint
and archive verification, retaining substituted broad product/registry effects.
`make test-release-validation` separately exercises the actual selector, shared
plan runner, input checkpoints and composed receipt checks with substituted
native/installed commands, including Jobs omission and full-gate fallback.
Focused subsets are `test-release-tooling-metadata`, `-recovery`, `-delivery`,
`-publication`, `-retention` and `-frozen`. The latter qualifies the actual frozen
consumer setup, release/recovery, archive and publication-admission hooks with
registry effects substituted. `test-release-tooling-frozen-installed` explicitly
adds focused captured-source IC execution; it is separate from ordinary CI.
These fixtures do not establish installed recovery or native macOS execution.
Use the [current handoff](status/current.md) for source-bound qualification limits.
