<!-- Archived from docs/releasing.md on 2026-10-15. -->

> Historical evidence only. Statements of current/pending state below are scoped
> to their recorded source and date. Use the [current procedure](../development.md)
> and [handoff](../../status/current.md) for active work.

![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Releasing

This guide is for maintainers preparing and publishing a new IC Timers
release. Library users do not need to follow this process.

The workspace follows semantic versioning. Before releasing, maintainers
classify the change, choose the appropriate version, run the required validation,
let the bump label the current changelog draft, and then publish the
release through the repository's release commands.

## Completed 0.23.1 native acceptance

Released **0.23.1**, `2d54724bc836ca09308d6742bf95c3f1ddecc3f4`, has complete
[main-source qualification](https://github.com/dragginzgame/ic-timers/actions/runs/38707139274):
[Linux CI/probes/PocketIC](https://github.com/dragginzgame/ic-timers/actions/runs/38707139274/job/116329230960),
[native/Wasm MSRV](https://github.com/dragginzgame/ic-timers/actions/runs/38707139274/job/116329230845),
[Intel macOS](https://github.com/dragginzgame/ic-timers/actions/runs/38707139274/job/116329231002)
and [Apple Silicon](https://github.com/dragginzgame/ic-timers/actions/runs/38707139274/job/116329231069)
pass. Both macOS jobs ran the complete native release gate. Its graph selects
Metrics 0.10.0, Testkit 0.44.0 and Shared `1444f1e` across 84/41/18 payloads,
retaining the explicit Binaryen 132 catalog. This closes #52/#55/#59; the earlier
#51 adoption is superseded by the qualified Metrics 0.10 cut. PR #56 is closed
as already delivered. Earlier failed/cancelled runs keep their original scope.
This receipt does not qualify the later 0.23.2 source or pending PR graphs.

The following preparation records retain their historical source scope.

## Prepared 0.22.3 shared-owner cleanup

Current HEAD is delivered 0.22.2 at
`26f1cdfc399fd4a321b73f9484a2234cae8a43de`. This batch is repository-only:
Shared 0.12.3 adoption and contributor-flow consolidations. It changes no timer
source/API or canister cost. Cargo release versions remain 0.22.2; incoming Host
0.17.11 selections are preserved. Release execution remains maintainer-owned.

The [adoption record](shared-tooling-history.md#prepared-shared-tooling-0123) identifies
all three exports and the approved strict changelog contract. Shared owns pure
literal rendering, coordinated member projection and graph-only admission, exact
stable-version comparison, finalizer increase admission, and fixture EXIT cleanup.
Timers retains source/candidate checks, byte/mode-preserving installation, external
lock selections and exact before-image rollback. Graph checks require no dated
changelog, but full release metadata checks still do.

The release adapter keeps one immutable export for manifest ordering, clearing
ambient commit selection before checking that tree. Current-worktree/index
admission, Testkit preparation and tag checks remain explicit. Generic downloaded
archive reads, tool receipts and controlled hosted failure bodies now reuse Shared;
Timers retains source/job/host/run/attempt, scenario and product assertions.
The existing qualified catalog and required native/PocketIC gates remain selected.
Shared's mutating metadata preparation and publication runner are not adopted.

The changed fixtures retain numbering/refusal, historical bytes/mode, failed
producers, literal rendering, rollback, selected-commit metadata, coherent/mixed
member graphs, complete lock identity and evidence rejection. Snapshot integrity,
Bash/Python syntax and locked metadata are separate from consumer fixture/native
qualification. The maintainer approved all 15 affected shell fixtures and the
downloaded-evidence Python suite; they pass. A missing copied helper and stale
lockfile-fixture expectations were corrected while retaining rejection coverage.
The live root graph check passes and the incoming root lock, real index and refs
remain unchanged. No complete release-gate or native macOS result is inferred.

## Prepared 0.22.2 contributor cleanup

The maintainer reports 0.22.1 live at
`52a0fce4082b087342f5b04db717e94bf5448193`. The compatible 0.22.2 draft combines
contributor cleanup with an explicit Metrics 0.9.1 minimum requirement. Timer
source, public summary types and instruction measurement boundaries are
unchanged. Package release versions remain maintainer-owned.

The incoming root lock already selects Metrics 0.9.1 and Host artifacts/fs/process
0.17.6; it is preserved byte-for-byte. Metrics' cached archive matches checksum
`59af3d107a6b06bee0a31b56d9ec014ed77b8f2cd69c96b419561efd57aee812`
and release commit `804ca4727b74d641bbe3be9155e20de914eb3ad9`. The arithmetic
package has identical authored source to 0.9.0, no dependencies and Rust 1.85
support; Timers retains Rust 1.88. Library/test-target compilation passes on
prepared Rust 1.99 and 1.88, and Wasm library compilation passes on 1.88, all
locked/offline with `RUSTUP_AUTO_INSTALL=0`. No tests or optimized artifact builds
ran for this adoption. Logs and input identities remain at
`/tmp/ic-timers-metrics091-audit.dVRqqwVF/` and with the
[renewed source audit](../audits/code-hygiene-2026-10-13-01.md).

The [download verifier](https://github.com/dragginzgame/ic-timers/blob/651efce370bd6aec6290b74118eb709488062d0a/scripts/ci/verify_failure_evidence.py) repair for
[#53](https://github.com/dragginzgame/ic-timers/issues/53) now selects
`ci/ic-tools-qualified.tsv`, matching Make and the local collector's qualified
Binaryen 132 selection. The existing downloaded-evidence suite exercises the
actual CLI with differing default/qualified catalogs, changed versions/digests
and a missing qualified catalog. Exact caller bytes, semantic installation
receipt checks and source/job/native-host identity checks remain mandatory.
Shared [#168](https://github.com/dragginzgame/shared-tooling/issues/168) owns the
separate proposed automatic cross-caller selection check; this local repair
does not implement that check or qualify Binaryen 133.

Publication's [clean-source adapter](https://github.com/dragginzgame/ic-timers/blob/32ea72e253fdf2adbeda76b320fdc70be885d1ed/scripts/ci/ensure-clean.sh) retains its
existing-commit guard and delegates dirty/staged/untracked inspection to the
adopted Shared `check-release-source.sh` with no allowed metadata exceptions.
Its maintained fixture covers failed Git observations, unchanged index/ref
bytes, dirty/staged/untracked source and an unborn checkout.

The native PocketIC harness reuses selected Testkit 0.37.0's `CandidCallExt` for
single-result replies, including unit endpoints. Candid's unit decoder admits
empty replies. Multi-argument replies keep their raw `Decode!` paths; decoding
one tuple value would change their wire interpretation. Private timer-executor
ingress rejection remains raw. Probe method names, callers, arguments and
canister code are unchanged; no dependency or new generic adapter is added.

Before-images, incoming Cargo inputs and index/ref observations are retained at
`/tmp/ic-timers-cleanup0222.me2TVYCf/`. Rust formatting, changed Bash syntax and
diff inspection pass. Focused downloaded-evidence/publication fixtures and
native probe compilation/Clippy have not run. Complete owner release validation
and PocketIC execution retain their own authority; no performance saving is
claimed from native harness line removal.

## 0.22.0 delivery and prepared 0.22.1 tooling

The maintainer reports 0.22.0 live at
`ffe175b2ae1b38719ec649286468b96a08a2f111`. Its completed standard receipt binds
owner release validation to source `9ad2c39b1f6bf89a12098612111aebb66e23aa72`
and staged tree `5f00837b17b2a0bda924d33eb8ce9c4a49f09c61`. Source-to-release
differences are only workspace/local-lock versions and changelog finalization.
The normal gate retains native tests, MSRV, probe lint and PocketIC recovery/
policy cohorts. Receipt completion is not a fabricated passing raw log, agent
replay or native macOS qualification.

The actual delivered graph selects Metrics 0.9.0, Testkit 0.37.0 and Host
artifacts/fs/process 0.17.2, with PocketIC 16.1.0 and Shared 0.10.0. Host's
three cached registry archives match the lock checksums and VCS producer
`e3afa6ddf022309e7ba97f6cefb312c48738db4e`; their runtime source is unchanged
from 0.17.1. Earlier preparation selected 0.17.1 and retains that exact scope.
No further Host dependency update is needed, and dirty 0.17.3 is excluded.

[Tag run 38399245736](https://github.com/dragginzgame/ic-timers/actions/runs/38399245736)
passes only tag truth. The same-SHA
[main run 38399245629](https://github.com/dragginzgame/ic-timers/actions/runs/38399245629)
has passed Linux checks and MSRV; Apple Silicon is running and Intel is queued
at final readback. Skipped tag jobs do not qualify native macOS.
Issues [#51](https://github.com/dragginzgame/ic-timers/issues/51) and
[#52](https://github.com/dragginzgame/ic-timers/issues/52) have delivered source
adoptions, but remain open for their final hosted native acceptance.

The later [Shared 0.10.2 refresh](shared-tooling-history.md#prepared-shared-tooling-0102)
is repository-only work under undated 0.22.1 notes. No timer source, Cargo input,
pin, installer/build-policy identity or gate membership changes. Common tool
cache transport is not newly enabled. Integrity/syntax/documentation checks do
not qualify new shared fixture behavior; tests, lint, setup and release execution
remain user-owned.

## Prepared 0.22.1 harness ownership

The unpublished PocketIC harness now uses selected Testkit 0.37.0's
`PocketIcStartupConfig::build_instance` and keeps its `PocketIcInstance` owner
through recovery and cohort execution. The ordinary-trap setup helper returns
that same owner with the canister ID. Callers borrow `instance` without
destructuring the server, so upstream field ordering owns instance deletion
before server termination. The factory still requires the gate-admitted
`POCKET_IC_BIN` and explicitly selects an application subnet.

One 60-second deadline covers server readiness and instance construction,
replacing separate 30-second deadlines. The maximum startup budget remains
60 seconds, but either phase may now consume more than 30 seconds. Synchronous
instance deletion is still unbounded; startup admission does not bound Drop.
All canister assertions, cohort features, compiler/profile selections and
Testkit setup/check gates remain in place. This is unpublished host plumbing;
the timer API, production dependency graph and canister source are unchanged.
Focused probe compilation and lint are separately approved development checks;
startup/recovery/cohort and native host qualification remain owner-operated.
The approved Rust 1.88 `cargo check` and Clippy commands for
`ic-timers-pocketic --all-targets` pass with locked, offline inputs and
`RUSTUP_AUTO_INSTALL=0`. Clippy initially rejected a redundant `must_use`
attribute on the factory; the final source relies on the upstream owning type's
attribute and passes both commands. Logs and task before-images remain at
`/tmp/ic-timers-harness0221.y2PU7FAH/`. No tests, PocketIC execution, Wasm build,
setup or release command ran for this cleanup. Cargo inputs, real index and refs
are preserved. No production Wasm or instruction saving is claimed.

IC Build's specification has feature/cohort controls, but its committed 0.2.2
acquisition compares package and library names literally and constructs artifact
paths with the package spelling. Locked Cargo metadata shows that our hyphenated
probe packages have default underscored cdylib names. This source-level blocker
is reported in [IC Build #16](https://github.com/dragginzgame/ic-build/issues/16);
no IC Build acquisition or Wasm experiment was run. Even after that fix, adoption
would require a native adapter, dependency and retained artifact owners to replace
roughly 25 lines of Make build/path plumbing. The current recipes use Cargo's
incremental targets and introduce no custom cache. No measured time saving or
smaller total adapter has been established, so this batch retains the recipes
and adds no IC Build dependency or second cache.

The version-writer extraction is proposed on
[Shared #159](https://github.com/dragginzgame/shared-tooling/issues/159#issuecomment-6147567013).
The narrow shared operation would render a literal workspace version from an
admitted source to stdout, preserving unrelated bytes and rejecting ambiguous
selectors before output. Installation and release transaction rollback remain
local. The qualified local writer stays until the upstream helper exists and
its consumer fixtures can establish equivalence; snapshots are unchanged.

## Prepared 0.22.1 contributor cleanup

Standard Shared Tooling release and resume commands are the sole automated
delivery owner. The local `release-x`, `release-stage`, `release-commit` and
`release-push` commands and their independent commit/tag helper are removed.
Fresh arbitrary-version delivery is no longer supported locally. Version-only
`patch`, `minor`, `major` and `bump-x` preparation remain user-owned; they do not
create shared release intent or authorize delivery. For a standard interrupted
release, rerun its original target or select its exact saved version through
`release-resume`. Unjournaled earlier manual preparations require maintainer
reconciliation; this cleanup does not infer or discard effects from them.

The shared runner keeps its exact source/index/destination checks, journal and
interruption recovery. Timers retains metadata adapters, the full release gate,
publication clean-worktree/exact-tag admission and the version helper's rollback
fixtures. Removed delivery tests retire with their commands; publication guard
failure and dirty-state cases remain in `test-tag-at-head.sh`.

The unpublished `ic-timers-test-protocol` package owns the seven wire record
definitions used by the runtime/size canisters and native PocketIC decoder.
It has only inherited Candid and Serde dependencies, with no runtime/provider
authority. The producer's cohort label remains borrowed and the decoder's owned;
one generic record carries both text representations. A native interop test checks
their identical Candid types, borrowed-to-owned decoding and wide totals.
The root catalog and sole lockfile add only that local package and its links;
external selections and existing package versions are preserved.

Runtime and cohort observation endpoints construct application metadata once,
using empty defaults before populating available timer fields. Their wire fields,
canister assertions, feature isolation and counter measurement boundaries stay
unchanged. The published timer library has no new dependency or API change.
Probe Wasm/instruction deltas remain unmeasured; final artifacts and PocketIC
subjects retain their owner-operated qualification. Source before-images and
focused evidence belong at `/tmp/ic-timers-cleanup0221.zH22uPmx/`.

Approved locked/offline checks pass: Rust 1.88 native probe/test-target compilation,
the protocol interop test, and runtime/four-cohort Wasm compile-only checks under
`timer-probe`; Rust 1.99 probe/protocol Clippy with warnings denied. Version
preparation, tag/publication, fixture-completion and release-gate fixtures pass.
The gate fixture initially expected only the two library test commands; its
updated independent expectation also requires the protocol test and checks its
failure propagation. Original failure output and fixture remain retained.
No PocketIC execution, optimized Wasm artifact build, setup or real release ran.
Registry identities, existing local versions, real index and refs are preserved.
Native macOS and complete release qualification remain owner-operated.

## Prepared 0.22 upstream boundary

The incoming root catalog/lock select Metrics 0.9.0, Testkit 0.37.0 and Host
artifacts/fs/process 0.17.1. Full locked offline metadata resolves one package
identity for each; cached official archives match lock checksums and producer
VCS receipts. No direct Host or IC Build dependency is added to the timer library.
Host remains transitive through the unpublished native harness. PocketIC stays
16.1.0 and Testkit retains CLI/server provisioning and admission.

Metrics 0.9 changes the public summary package identity, actual-overflow accessor
names and error cases. Existing wide/zero accounting fixtures follow that API
directly; there is no legacy bridge. Consumers exchange summaries through the
Timers re-export or coordinate Metrics 0.9. This requires undated 0.22.0 notes,
superseding the prepared 0.21.5 patch. Cargo versions, release execution and
publication remain maintainer-owned.

The [Shared 0.10 adoption](shared-tooling-history.md#prepared-shared-tooling-0100) owns
canonical host/Cargo and IC catalogs. Make explicitly selects
`ci/ic-tools-qualified.tsv` for the retained Binaryen 132 exception, and failure
collection uses that same file. Other tool versions/digests match shared defaults.
Its existing compact-evidence fixture follows the exception through generated
bundle receipts and copied-checkout qualification; execution remains owner-owned.
Standalone IC installer use must pass this exception with `--pins`; an omitted
selection uses the snapshot's Binaryen 133 default. No maintained product recipe
runs that optimizer, so unoptimized PocketIC acceptance does not qualify a switch.
Perform owner-operated `make install-tools` and `make tools-check` before
formatting and the normal complete release gate. Prior tools and failed evidence
remain retained. New source/payload checks do not establish native/macOS/IC
acceptance; the current handoff owns their exact scope.

## Root workspace and dependencies

The maintainer's dependency-centralization request supersedes the independent
`testing/` workspace. The root `Cargo.toml` owns `ic-timers` and the four
unpublished packages under `testing/crates/`, with one root dependency catalog
and `Cargo.lock`. Member dependencies and package versions inherit from the
root; probe publication stays disabled. There is no `testing/Cargo.toml` or
`testing/Cargo.lock` compatibility path.

`ic-timers` remains the default member. Library check, test, lint, docs, MSRV
and Wasm targets select it explicitly. Formatting covers all five members.
The full release gate retains every probe lint configuration, watchdog/ordinary
recovery subject and policy cohort. Probe Wasm builds use `--profile timer-probe`
and the existing ignored `testing/target` and cohort target directories;
loaders select the resulting `timer-probe` output directory. The root ordinary
release profile is unchanged. Dependency resolution now uses root resolver 3;
fresh measurements and native qualification are required for the combined graph.

Released 0.14.21's root catalog selects compatible Testkit 0.25, locked to 0.25.3,
with all four Host crates at 0.8.4 transitively, and ic-metrics 0.2.15. The PocketIC client/server pair selects
16.1.0; prior runtime evidence remains bound to its earlier pair. Its client's exact thiserror 2.0.18 requirement determines the
shared thiserror selection; the old library-only 2.0.21 graph is not retained.
All four member versions inherit the root release identity, and release
preparation updates their local lock records together. Publication reported by
the maintainer does not establish complete native or measurement qualification.
Historical adoption evidence below remains tied to its
recorded versions, workspace shapes and hosts.

The [host record](#host-support) and
[Shared Tooling adoption](shared-tooling-history.md#shared-tooling-committed-0132-follow-up)
record current source and qualification gaps. Released 0.14.21 passes the complete
hosted native gates and matching tag truth, including its selected Metrics graph.
The earlier missing-package offline inspection remains historical. The compatible
0.14.22 draft prepares repository-only compact failure collection, with Cargo
identity unchanged. Later external lock edits selecting Metrics 0.2.16, TOML 1.1.8
and toml_parser 1.1.5, plus all four Host crates at 0.8.5, are retained separately;
that graph needs fresh qualification.
Normal fetch still prepares the selected lock before validation; no contributor
dependency update, lock edit, fetch or local qualification ran.

Release-source admission delegates to the reviewed shared checker. The adapter
permits only `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md` as release outputs;
it reports all other staged, unstaged and untracked paths, quoting unusual names.
Failed Git observations preserve their own error rather than claiming dirty
source. The runner adds the initial preflight context only before that attempt
starts validation or version preparation. Later prepared/committed checks retain
their own phase semantics. No guard repairs files or stages a lock to make release
proceed. [#32](https://github.com/dragginzgame/ic-timers/issues/32) owns native
consumer acceptance.

## Pre-1.0 compatibility

The implementation policy and the version boundary are separate decisions.
Superseded pre-1.0 APIs are still removed as hard cuts without deprecated
aliases, forwarding shims, dual behavior, or compatibility features. A public
removal, signature change, or incompatible public semantic change must
nevertheless advance the minor compatibility line. For example, a breaking
change after 0.3.8 targets 0.4.0, not 0.3.9. Backwards-compatible fixes may use
a patch release.

Version 0.3.7 removed the public `TimerFuture` alias in a patch release. That
was a SemVer mistake because Cargo requirements compatible with 0.3.6 may
select 0.3.7 automatically. The alias remains removed under the hard-cut
policy; the correction is to use a minor version for future public removals,
not to restore a compatibility shim.

## Repository updates versus crate releases

Run this before selecting a version:

```text
make release-impact
```

The classifier compares the worktree with the most recent reachable canonical
release tag and reports:

- `crate` when the publishable crate's source or manifests changed;
- `repository` when only paths outside the publishable crate source and
  manifests changed, such as documentation, evidence, external tests, CI, or
  release tooling; or
- `none` when no path differs.

The workspace may already have an untagged version, so its matching tag is not
required. An explicit base passed to the classifier must still resolve to a
commit. Missing release history, malformed tags and Git failures remain errors.

Tracked and untracked changes are read as NUL-delimited Git paths after both
queries complete successfully. Git's display quoting, whitespace and non-ASCII
filenames cannot change a path's classification. Duplicate records do not change
the strongest impact, so no joining, decoding or sorting is needed. A temporary
record file preserves NUL bytes and is removed on exit; a failed Git query cannot
produce an apparently unchanged subject even after emitting plausible records.
The existing impact fixture covers untracked and staged crate paths with spaces,
tabs, line breaks, quotes and UTF-8 bytes under `core.quotePath=true`, plus partial
query failures and cleanup. Fixture comparisons capture successful output before
checking its value. Shell and embedded fixture-shell syntax and source flow were
reviewed; diff whitespace checks passed. These fixture scenarios have not been
executed, and this change supplies no native macOS qualification.

This is a conservative mechanical boundary, not an API compatibility oracle.
For `crate`, review the public API and semantic contract and choose patch or
minor according to the rule above. Repository-only work normally stays
untagged: validate it with `make repository-check` plus any focused owner-local
evidence and bundle it into the next code-bearing release. This avoids forcing
exact-pinned shared-registry consumers to coordinate a package identity that
does not change runtime behavior.

An explicit maintainer-owned version-bump or release target overrides that
default. For a `repository` subject, version preparation prints an advisory;
the user-operated release targets retain the complete release gate. It never invents crate
impact or silently weakens validation. A `none` subject is still rejected.

Keep completed user-visible changes in one numbered, undated section at the top:

```text
## [0.13.2]
```

Automated contributors derive the candidate from the latest finalized release
and the complete batch under the [shared changelog rules](../../../rules/changelogs.md).
Reuse that section for later compatible work; before 1.0, a breaking batch needs
the next minor line. Honour a valid maintainer-selected target. This selects
notes only, without changing Cargo, either lockfile or release defaults. Do not
add `Unreleased`, an unnumbered Draft or a separate release-note queue.

Contributors maintain the numbered pending release as the first release section.
The matching user-operated bump dates it automatically. Shared 0.12.0 rejects
versionless `Draft`, a pending section below history, a headingless changelog or
a missing file before mutation. Empty notes remain advisory. If the first release
section is the exact previous version and no pending candidate exists, preparation
can insert an empty candidate above that history. The maintainer approved this
contract in place of the earlier permissive-layout exception on 2026-10-14.

During preparation the helper refuses competing undated release candidates,
a named draft that conflicts with the requested bump, or an already dated target.
Both finalizer calls receive the bump's validated previous package version;
undated numbered sections at or below it are retained as history. Version
components compare by length and text without floating-point conversion.
Same-date finalization is still refused. See the
[finalizer review](shared-tooling-history.md#changelog-finalizer-review-and-history-fix)
for the ownership and fixture scope.
The 0.14.12 draft delegates selection and pending-body formatting to the shared
AWK owner, preserving historical bytes. The local wrapper owns reader/selector
status, check-only admission, modes and atomic output; its isolated fixtures
include the canonical selector explicitly.
It cannot silently select among batches, override a chosen minor boundary or
relabel published history. It also rejects a requested version that is not a strict
canonical-SemVer increase, an existing exact release tag, or a subject with no
changes since the release-impact base. Version preparation accepts the
current worktree and does not require a preparatory commit. A
repository-only subject emits an advisory but may proceed when the maintainer
has explicitly invoked the bump or release target.

One helper, `scripts/release/workspace-version.sh`, owns reading and changing
`[workspace.package].version`. The repository manifest uses one literal,
double-quoted canonical SemVer field in that table. Table order, indentation
and trailing comments do not select a dependency version. Missing, duplicate
or noncanonical workspace versions fail before mutation or staging. The writer
requires the expected previous version and changes only that value. Both locked
workspace metadata checks also require exactly one resolved `ic-timers` package
at the workspace version; coherent lockfiles for a different package version
are rejected.

README versions, examples, layout and presence are not release prerequisites.
Release preparation leaves README bytes and modes unchanged and does not stage
it. The former local version-projection helper and test are removed from every
release phase and CI. README edits follow ordinary source admission and explicit
maintainer staging; stale examples alone cannot refuse a release. Periodic
advisory documentation review is requested in
[Shared Tooling #100](https://github.com/dragginzgame/shared-tooling/issues/100),
using the shared maintenance catalog rather than another local gate.

The current publication guards reject failed Git observations and dirty source;
standard delivery commit/tag observation and recovery belong to the adopted
Shared Tooling runner. Local tag/publication fixtures retain empty and plausible
failed observations without authorizing effects. Earlier independent commit/tag
qualification is historical and does not define another current delivery path.

The repository checks retain producer failure status during Git configuration
inventory and provider-source inspection. The shared structured Actions checker
captures the complete NUL inventory before reading configurations and removes
its scratch directory on exit; nested paths and spaces retain their meaning.
Provider validation rejects a failed search or sort before comparing the allowed
source path. The repository fixture injects inventory, search and ordering
failures with empty or matching records, checks inventory cleanup, and retains
accepted pinned workflows and rejected moving references. The updated shared
checker fixtures cover quoted/folded references, Docker digests, composites,
reusable workflows, malformed inputs and independent tracked lockfiles. The new
adoption fixtures have not been executed locally; this repository-only work
changes no timer runtime contract and supplies no new native macOS qualification.

Standard release targets run `bump-version.sh --check` before deployment
validation. This preflight checks the requested version, impact, unambiguous
draft selection without changing version
metadata or running tests. The bump helper accepts exactly one `patch`, `minor`,
`major` or canonical
`x.y.z` argument after an optional leading `--check`. Missing or extra arguments,
including a misplaced or repeated check flag, fail with usage status 2 before
reading release metadata. The preparation fixture checks these rejections against
unchanged metadata bytes and permission bits.

The version-preparation fixture copies the shared `next-release-version.sh` used
by the bump helper into its isolated repository. It checks all three increment
preflights without metadata mutation and retains real rollback coverage. The
separate release-gate fixture checks standard Make delegation, selected
remote/branch forwarding, resume selection and runner failure propagation using
a recording stub. Shared phase ordering and interruption recovery belong to
`test-release-runner.sh`, included in `release-check`.
The maintainer reported the missing-helper failure before these changes. Shell
and embedded fixture-shell syntax and source flow were reviewed; the corrected
fixtures remain unexecuted by the automated contributor and need maintainer
qualification on the declared hosts.

The maintainer's subsequent release gate reached the missing-changelog rollback
case and reported `version preparation accepted a failed update with no changelog`.
The fixture had already restored the real lock updater, disabling its requested
failure. It now retains that injector until the case finishes, checks that the
root-update failure and rollback were reached, verifies removal of the newly
created changelog and compares the remaining metadata bytes and modes. The
production bump helper is unchanged. Source, shell and embedded fixture-shell
syntax were reviewed; rerunning the corrected fixture remains maintainer-owned.

Standard delivery admits the canonical source through the shared runner and
consumer adapter before preparation. Plain `patch`, `minor`, `major` and `bump-x`
remain available for dirty-worktree metadata preparation. Their preflight and
rollback fixture does not stage, commit or tag as part of the bump.

If a standard release is interrupted, rerun its original target to reconcile
saved intent. A prepared date or version does not prove a release tag,
publication or completed delivery. Retain earlier effects and metadata for
maintainer reconciliation rather than starting an independent delivery path.

Cargo owns package identity. The handoff reads it directly instead of storing a
second version projection. Release and deployment checks do not read changelog
layout, release-note headings or `Status:` prose. A dated changelog records version
preparation; it does not establish tagging, publication or deployment. Keep
supporting evidence with its owner and preserve historical records, without
turning them into release prerequisites or a parallel issue tracker.

Before version mutation, the helper also scans
the compact status for target-version wording likely to become stale, such as
`candidate`, `unreleased`, or a next action to publish after release. This is
advisory: it prints a warning and always continues. Free-form prose is never a
post-mutation release blocker. Canonical Cargo identity, resolved lockfiles and
exact annotated tags remain enforced facts.

Use one of the standard release families:

```text
make release-patch
make release-minor
make release-major
```

For an exact saved unfinished release, use:

```text
make release-resume VERSION=x.y.z
```

This resumes admitted shared intent; it does not prepare a fresh arbitrary version.

All release execution is user-owned: version bumps, tests, staging, commits,
tags, pushes and publication. Automated contributors prepare only the next
changelog draft and directly relevant evidence. The user runs `make patch`, `make
minor`, `make major`, or `make bump-x VERSION=...` when ready to update the
workspace version and the root lockfile. Normal delivery stages only its admitted
metadata through the shared runner.

The selected shared contract includes the PR helper. Standard entrypoints come
from `make/release.mk`; this consumer uses separate override assignment and
export for `RELEASE_DELIVERY=direct`, preserving GNU Make 3.81 parsing.
Target-specific bindings fix snapshot routing to `$(CURDIR)`.
Ambient environment or Make-variable
PR selections do not switch the patch/minor/major or resume commands. Adopting PR
delivery requires separate merged-source adapters and qualification; this refresh
grants no merge authority or contributor release effects.

Standard commands may be rerun after any interrupted attempt. Before preparation,
the shared runner starts fresh preflight and complete validation on current
source; older preparation-free records are preserved as evidence. After
preparation may have begun, a normal target selects the saved release before
computing another increment and reconciles its exact source, version, date and
destination. Plans are internal recovery records and do not require a different
command. Uncommitted preparation remains bound to its source and kind. Once the
release commit exists, newer descendant commits or a different requested
increment are allowed: finish the saved release, then run fresh preflight and
the full gate for the requested increment from the actual local version.
Identity, payload, destination and concurrency conflicts still reject.
An explicit exact recovery selector remains available:

```text
make release-resume VERSION=0.13.0
```

The vendored shared runner fixture owns restart and automatic recovery coverage;
the local Make fixtures verify direct delegation, selection forwarding and
selected-commit metadata checks. Explicit resume finishes only its saved release.
The superseded local retry wrapper and its duplicate fixtures are removed.

Released 0.14.14 adopts Shared Tooling 0.1.23's
[release-integrity repair](https://github.com/dragginzgame/shared-tooling/issues/58).
After the final consumer check, the runner independently rechecks the committed
payload, index, worktree and exact annotated-tag object before dispatch. Completed
direct resume observes the exact local/remote tag and destination branch ancestry
before reporting success, without replaying commits, tags or pushes. Conflicts or
unavailable observations retain the selected plan and failure evidence. The
canonical runner fixture owns these refusal and exact-version repair/retry cases;
the local committed-release fixture checks explicit direct-policy selection as
well as immutable metadata/receipt binding. Both remain user-operated gates.
The [adoption owner](shared-tooling-history.md#shared-tooling-0123-preparation) records
preparation and upstream/consumer qualification limits;
[#29](https://github.com/dragginzgame/ic-timers/issues/29) closed after the
unchanged adoption passed both complete native gates in 0.14.15.

The user-operated release targets run the complete release gate, update the
workspace version and all five local member identities in the sole root lockfile,
commit, create an annotated `vX.Y.Z` tag, and push the branch/tag atomically. If the workspace version
has no saved unfinished intent, the requested bump runs. A fresh `make release-patch`
advances the patch version; rerunning an unfinished patch release recovers it. For
example, with Cargo at 0.8.2 and a current draft, stage and commit the code-bearing
changes, then run `make release-patch` to validate, bump and release 0.8.3. The bump
itself also supports a dirty worktree without a preparatory commit.
Standard releases, exact saved-intent resume, validation logging and pre-commit
formatting require executing Make
modes that propagate failures. The shared admission probe rejects ignore-errors,
dry-run, question, touch and version-only controls before protected effects.
Use ordinary Make modes for those commands. The reviewed Shared 0.2.11 include
independently probes MAKEFLAGS and retained MFLAGS, and refuses assignments
that erase MFLAGS. Clearing/replacing MAKEFLAGS cannot admit hidden unsafe modes.
Release selections and normal parallel-job controls remain inherited.
Hook and version-preparation regressions print their retained scratch paths on
failure, including per-scenario Make output and before/after fixture state.
Successful fixture runs remove their scratch directories. Shared delivery
fixtures own ordering and interrupted-effect recovery.
Release metadata and the root lockfile are checked before the release commit;
unstaged and untracked work is rejected before committing or tagging.

### Changed source at preparation intent

Shared Tooling 0.6.3 lets a normal direct release target restart saved `prepare`
intent after a newer clean descendant commit. The runner requires an empty saved
index tree, no preparation file set, clean index/worktree/untracked inventory,
matching destination and canonical metadata at the saved previous or candidate
version. It archives the original plan unchanged under `release-state`, then
computes the requested increment from current metadata and repeats preflight and
the complete gate. See the
[canonical restart contract](../../releases.md#source-changes-after-saved-preparation-intent).
Unchanged HEAD and explicit resume retain exact-source recovery; staged/later
phases, conflicts and occupied locks still refuse unsafe restart. Release
execution remains maintainer-owned.

Timers' current preparation owner is `scripts/release/bump-version.sh`.
It captures exact incoming `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md` bytes in
`ic-timers-bump.*` under the selected temporary root, installs catchable-signal
rollback, then finalizes notes, changes the workspace version and rewrites local
lock identities. Successful completion or successful rollback removes those
backups. This is temporary rollback, **not a durable pre-mutation journal**;
process termination or host loss can interrupt it. No generic `VERSION.metadata`
file is maintained. Those facts do not establish that preparation never began.
The adopted restart contract instead relies on local-only preparation and the
newer clean commit recording the metadata disposition. The helper does not stage,
commit, tag, publish or deploy. Reconcile any incomplete rollback against its
retained originals before fresh preparation; never relabel earlier validation.

Timers has no per-candidate validation receipt or saved-notes file overwritten by
fresh verification. The shared logger allocates a unique directory per invocation
and retains failures under distinct run identities; its `latest` files are views
of retained evidence. Existing optional persistent log selection is also unique
per invocation. Host's
[receipt-preservation finding](https://github.com/dragginzgame/ic-host-tooling/issues/67)
therefore has no matching local replacement path. This adoption performs no real
recovery, changes no log-retention policy and adds no second release-state machine.

If a standard release stops after the version bump, rerun the same standard
target or select its exact saved intent with `release-resume VERSION=x.y.z`.
The shared runner owns commit/tag/push reconciliation without an independent
local phase command. Publication remains a separate user-owned `make publish`
after delivery succeeds.

`make fmt` and `make fmt-check` sort manifests with cargo-sort 2.1.4 before
formatting/checking Rust for every member of the single root workspace. The exact
tool pin lives in `ci/tool-versions.env`; `make update-dev` installs it with
`--version` and `--locked`, and hosted jobs prepare it before gates. Hooks and
format checks never install tools. Prepared standard-release metadata is checked
for manifest ordering before staging, so a commit hook does not repair the
runner's saved payload. `testing-check` uses the same formatting gate.

Formatting success prints one line. Failure prints the failing command's exit
status and the path to complete retained stdout/stderr; Make then returns its
normal recipe-failure status. In CI, the wrapper uses RUNNER_TEMP and the local
failure collector archives `formatting.*` there. Local runs otherwise use TMPDIR
or /tmp. Successful logs remove themselves. The
[0.2.11 adoption owner](shared-tooling-history.md#shared-tooling-0211-formatting-and-make-admission)
records source identity and pending consumer qualification.

The vendored formatting hook exports the index to disposable scratch, formats
all root workspace members, then copies/stages only the fully staged selection. Partial
staging, formatter failure or concurrent edits reject without discarding working
changes. Unselected and unrelated edits remain untouched. `make install-hooks`
is explicit per-clone activation; the installer refuses conflicting hook paths.
The local consumer fixture covers library and probe members in one workspace, partial staging and formatter
failure isolation using the actual formatting targets. Its new scenarios remain
unexecuted by the automated contributor.

The non-release `make patch`, `make minor`, `make major`, and
`make bump-x VERSION=...` targets stop after the version-file update for
review without running build, lint or test suites.

### Host support

Current completed acceptance is the [0.23.1 receipt](#completed-0231-native-acceptance)
above. The following records describe earlier sources, including their historical
queue/outage states; they do not reopen completed issues or qualify newer edits.

The released 0.16.3 workflow adds Linux execution of the existing PocketIC
recovery and policy-cohort targets after CI/probe lint. Testkit's root-lock
selected CLI prepares and admits the server; the product harness then starts
fresh managed servers and instances. Its matching Linux product run now passes;
both native macOS gates remain queued. Keep
[#34](https://github.com/dragginzgame/ic-timers/issues/34) open for those results.
See [the qualification scope](#linux-product-qualification-for-0163).

Released 0.14.12 is `72e8f5d9769d00fbe165b16cd6eb2d81cf1b0a67`.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37639154601)
passed. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37639154602)
passed Linux checks, MSRV and the complete Apple Silicon gate. Intel macOS failed
runner acquisition after five attempts, without executing any job steps or
producing an artifact; this supplies no Intel
source qualification or evidence of a fixture defect. #24/#25/#28 retain their
native qualification requirement. The new 0.14.13 driver is outside that source.

Released 0.14.11 is `eab55f8c20f7b144551f86885aefc7dbaa9fd4ea`.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37631528851)
passed. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37631528673)
passed Linux, MSRV and both complete native macOS gates, including the released
byte-exact consumer finalization fixtures. This does not qualify the 0.14.12
canonical-selector/PocketIC consolidation or incoming Testkit 0.21.1 lock edit.
No contributor validation ran for the new worktree.

Released 0.14.10 is `479c4b8c8b6412babf7c98ea17c948eacdaeadc3`.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37617070319)
passed. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37617070245)
passed Linux/MSRV and subsequently both complete native macOS jobs. The preceding
0.14.9 macOS jobs were subsequently cancelled, so its previously pending result
never became complete native qualification. Neither release qualifies later
dirty lockfile changes or uncommitted Shared Tooling repairs. Earlier source-bound
records follow; normal green CI does not prove #23's hosted-failure artifacts.

Released 0.14.9 is `7e98cbc969b1e6d6098786a7f36ac3ba66d8165a`.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37614583521)
passed. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37614583799)
passed Linux/MSRV; both native macOS jobs remained pending at inspection. The later
Shared Tooling 0.1.18 worktree has no hosted qualification. Earlier source-bound
evidence follows and must not be relabelled as this release's native result.

Released 0.14.8 is `7194dcdb092a33b90886d46b304545d5c09ca0a6`.
Its [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37601234917)
passed Linux, MSRV and both complete native macOS gates; matching
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37601234587)
passed. This qualifies the released two-workspace tooling source, not the current
single-workspace/Testkit dependency preparation. Green jobs do not prove the
failure-only artifact-upload acceptance in issue #23.

The maintainer subsequently committed the single-root consolidation at
`007dbe30d5e3bf64f3434d55b34cd1da5c0e08e7`. Its
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37606855402)
passed Linux, MSRV and both complete native macOS gates. That source is untagged
0.14.9 preparation. Later fixture changes and the external lockfile refresh
remain outside this qualification; normal green CI still does not qualify #23.

macOS host workflows are required by the
[adopted engineering baseline](../../../DRAGGINZGAME.md#host-support).
The requirement is separate from executed qualification; canister execution
continues to target Wasm on the Internet Computer.

| Host | Current workflow configuration and evidence scope |
| --- | --- |
| Linux x86_64 | Hosted Rust/MSRV jobs use Ubuntu runners. Released 0.16.3 passes product recovery and policy cohorts with the prepared internal toolchain; Testkit owns server admission from the root-lock selection. 0.16.4 also passes at its own Testkit 0.28.0 / Metrics 0.3.5 / Host 0.10.1 graph; newer transitive lock changes remain separate. |
| macOS 15, Intel x86_64 | Declared host target. PR/main job uses `macos-15-intel`, Apple's Bash 3.2 and the complete release gate. The gate passed for released 0.14.15. 0.16.3 fails before the gate on Make override/export parsing; the repaired 0.16.4 native gate passes with actual recovery and policy-cohort subjects. Historical failures retain their original scope below. |
| macOS 15, Apple Silicon arm64 | Declared host target. PR/main job uses `macos-15`, Apple's Bash 3.2 and the complete release gate. The gate passed for released 0.14.15. 0.16.3 fails before the gate on Make override/export parsing; the repaired 0.16.4 native gate passes with actual recovery and policy-cohort subjects. Historical failures retain their original scope below. |

Released **0.14.21** is `5c6b7f45d72f9051b4e155410ec47339885dc4f2`;
the maintainer reports it pushed. Matching
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37819731178)
and [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37819730920)
pass. Linux checks/probe lint, MSRV and both complete native macOS release gates
qualify Shared 0.1.29's actual consumer release-source adapter and its
Testkit 0.25.3 / Host 0.8.4 / Metrics 0.2.15 / PocketIC 16.1.0 graph.
All three host logs report passing release-index, release-runner and installer/
evidence fixtures. [#32](https://github.com/dragginzgame/ic-timers/issues/32) is
closed. This does not qualify the new 0.14.22 compact collector or relabel the
cancelled upstream run; registry publication was not independently checked.

Released **0.14.20** is `40611eff87b3165e58528c597debaa95427cdf5a`;
the maintainer reports it pushed. Matching [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37807464542)
passes. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37807464532)
passes Linux checks/probe lint, explicit MSRV and both complete native macOS
gates. This completes [#31](https://github.com/dragginzgame/ic-timers/issues/31)'s
consumer acceptance. Its graph is Testkit 0.25.2 / Host 0.8.2 / Metrics 0.2.14 /
PocketIC 16.1.0; later results do not relabel this source's evidence.

Released **0.14.19** is `e637224e018afd758175e21de3afe3b95fa1a9ed`;
the maintainer reports it live. Matching [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37803792774)
passes. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37803792258)
passes Linux checks/probe lint and explicit MSRV. Intel is running and Apple
Silicon is queued at inspection, so complete native acceptance is pending. The
selected graph is Testkit 0.25.1 / Host 0.8.2 / Metrics 0.2.14 / PocketIC 16.1.0.
The subsequent 0.14.20 tooling adoption requires its own source-bound qualification;
it does not relabel these results or qualify compact evidence collection.

Released **0.14.18** is `c8d670d1e3181bebb1e67fcfc3eed0dcde0bddd2`.
[Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37790176635)
passes Linux CI/probe lint and MSRV; Apple Silicon is running and Intel is queued
at inspection. Matching [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37790175542)
passes. This graph selects Testkit 0.24.0 / Host 0.7.1 / Metrics 0.2.13 and
PocketIC 16.0.0. Released **0.14.17**'s
[main run](https://github.com/dragginzgame/ic-timers/actions/runs/37774925568)
now passes Linux, MSRV and both complete native macOS gates at `031e6c67`.
That supplies complete normal-gate acceptance of its Testkit 0.23 / Host 0.6 /
Metrics 0.2.12 / PocketIC 16.0.0 graph; separate manual failure transport remains
scoped in the [evidence owner](#evidence-path-repair-and-01417-qualification).
Neither source qualifies the incoming PocketIC 16.1.0 client or db039 tooling.

Released **0.14.15** is `ae26b854a1a473c5d2d153705c2d13570f378d38`.
[Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37752855158)
passed Linux, MSRV and both complete native macOS gates; matching
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37752855388)
passed. The released graph selects Testkit 0.21.3, Metrics 0.2.11, Host 0.4.6
and PocketIC 16.0.0; all four local members are 0.14.15. This closes #29 for
the unchanged Shared Tooling 0.1.23 adoption, but does not qualify the incoming
Testkit 0.22 graph or #23's skipped failure-only uploader/download path.

Released 0.14.14 is `6fc76e9ffaabad575fe5f044029e6fdd063e4e32`.
[Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37750074305)
passed Linux, MSRV and Apple Silicon; Intel was cancelled. Its
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37750074677)
passed. Its root lock selects Testkit 0.21.3, Metrics 0.2.10, Host 0.4.6
and PocketIC 16.0.0; all four local packages are 0.14.14. No hosted result
for 0.14.14 is relabelled as execution evidence for a different dependency graph.

Released 0.14.13 is `0b12c8a6dbe5f359f5499df67313ffa5c02af446`.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37655294418)
passes. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37655294064)
passes Linux, MSRV and both complete native macOS gates. The released lock
contains Testkit 0.21.2, Metrics 0.2.9,
all four split Host packages at 0.4.6 and PocketIC 16.0.0. The source-bound
[adoption acceptance](shared-tooling-history.md#consumer-adoption-qualification)
closes #24/#25/#28 and records subsequent complete current-release host acceptance.
#23's original early/late failing jobs and six verified downloads remain required.

Host 0.4.6 `0fb05f9e18f032425188d68e1d69317a0f0127d5`
[passes all declared native owner gates](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37648086908).
Testkit 0.21.2 `2db7b4f6b616b484408695656e26207628d74c5f` now has passing
[main](https://github.com/dragginzgame/ic-testkit/actions/runs/37651807993) and
[tag](https://github.com/dragginzgame/ic-testkit/actions/runs/37651807716) CI.
Those results qualify upstream; they do not substitute for this consumer's
separate source-specific native and artifact observations.

Preparation for subsequently released 0.14.14 selected Testkit 0.21.3 and compatible
`serde_spanned`/TOML-family patches. Committed Testkit 0.21.3
`a8e83a1940e5f44927c6df95b5d1269a3ac699bc` passes its exact-source
[main native, MSRV and PocketIC-concurrency gates](https://github.com/dragginzgame/ic-testkit/actions/runs/37661836622)
and [tag CI](https://github.com/dragginzgame/ic-testkit/actions/runs/37661836641).
Review against 0.21.2 finds no change to production library implementation or
the `pic` startup APIs used by this harness: the sole library source change
removes duplicated `cfg(test)` cache-path cases. Full root
`cargo metadata --manifest-path Cargo.toml --locked --offline --format-version 1`
passed without rewriting the incoming lock. During contributor preparation all
four local packages remained 0.14.13 and inherited their dependency declarations;
Host stayed outside the timer library graph. The finalized 0.14.14 notes record
the selection, with subsequent consumer execution scoped above. The earlier
0.14.13 run qualifies its own Testkit 0.21.2 graph.

The released 0.14.14 lock selects Metrics 0.2.10. Committed upstream
`90262c3b086ee39016a6f36902a610a78a139301` passes
[its source-bound CI](https://github.com/dragginzgame/ic-metrics/actions/runs/37745384375).
Its library source is byte-equivalent to 0.2.9; the patch adopts upstream
contribution/release tooling and updates documentation. Future dirty sibling
work is excluded. Full locked offline metadata passes with this selection and
does not rewrite the lockfile. Its actual consumer native and PocketIC execution
remains part of the user-operated gate for the new graph, separately from the
released 0.14.13 graph and the Shared Tooling source/export checks.

Released 0.14.15 selects Metrics 0.2.11, committed upstream
`69b110b8fbefdac4773eac7631796f9dcb3f41a0`. Review against 0.2.10 finds no
library source change; the patch adopts the same Shared Tooling 0.1.23 release
repair and records upstream evidence. Its
[owner run](https://github.com/dragginzgame/ic-metrics/actions/runs/37748731541)
passed Linux, MSRV and both native macOS gates. Preparation's full root
locked offline metadata passed without rewriting the incoming lock; subsequent
0.14.15 consumer qualification is recorded above. This changes upstream tooling
and evidence, not metrics library implementation.

Host 0.5.1 `81f9809861159def2fd0987fcb7961cda4afd969` is committed; its
[exact-source CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37750135927)
passed Linux, MSRV and both native macOS gates.

### Incoming PocketIC lock alignment

After 0.14.18 release, the retained root lock selects PocketIC client 16.1.0
under Testkit 0.24.0's compatible requirement. The maintainer chose to qualify
that pair rather than restore client 16.0.0. The first release attempt correctly
failed exact admission against the old server pin at 16.0.0; its raw failure
remains in `.git/release-state/validation-failures/20261008T143516Z-2304745-1-pocketic-check.log`.

The 0.14.19 draft now prepares matching 16.1.0 server archives and independently
computed binary digests for Linux x86_64 and both native macOS architectures.
The [artifact review](#pocketic-artifact-pins) checks published archive identity
before decompression and inspects binary headers without execution. The existing
`ci/ic-tools.tsv` becomes the one consumer-owned matrix, removed from the shared
audit file roster through a fresh canonical export; installers and admission
helpers remain exact shared bytes. This keeps product qualification choices
from being overwritten by a generic snapshot refresh. Other tool pins are unchanged.

Make's default cache now selects `target/tools/pocket-ic/16.1.0/pocket-ic`.
Automatic provisioning verifies both hashes and exact version; explicit overrides
remain read-only and must match 16.1.0. The independent consumer fixture updates
all three expected host hashes/URLs and rejects hash-matching server 16.0.0.
No lock or Cargo package version is changed by this repair. The actual shared
alignment helper now reports **16.1.0** using cheap locked/offline metadata.
This check executes neither server binaries nor canisters. Snapshot integrity,
Bash syntax, source pin/cache/fixture consistency and diff checks also pass.
Fresh native startup, watchdog/rollback and policy-cohort execution remains required in the complete
user-operated release gate; source/hash/metadata inspection cannot supply those
results. Old 16.0.0 receipts and frozen artifact transport retain their own scope.

### Testkit 0.24 selection

During the 0.14.18 path repair, a concurrent root catalog/lock update selected
registry Testkit 0.24.0; the current incoming lock selects all four Host crates at
0.7.1 and Metrics 0.2.13. Those edits are preserved; this contributor ran no update
or fetch. Registry Rust files for Host 0.7.1 match 0.7.0, and Metrics 0.2.13 match
0.2.12; these latest patches have no Rust source delta. Cheap full locked offline metadata
passes, and the selected registry Rust source matches Testkit's released owning
checkout `e7a9c6c`. The harness has no `artifacts::read_wasm` call or exhaustive
Host process-error matches, so neither upstream public cut needs an adapter here.
The root catalog remains the only direct-dependency owner; Host/Testkit stay
outside the timer library graph. Timer runtime/API, audited PocketIC 16.0.0 and
expected production Wasm/instruction cost are unchanged. This selection requires
its own full user-operated consumer gate; the frozen 0.14.17 observations below
execute Testkit 0.23 / Host 0.6 and do not qualify this new graph.

### Testkit 0.23 adoption

The maintainer reports Testkit 0.23.0 live. The incoming root catalog/lock selects
that registry version and all four Host crates at 0.6.0; this preparation
preserves those existing dependency edits. Full locked offline metadata passes
without rewriting either file. All 38 selected Testkit Rust source files match
released commit `59b1c1de924116752282eac48c6531dce159ccc9`.

The actual harness calls `spawn`, `start_managed_server`, `url`, `connect` and
`try_build`, then drops the instance before its server. None of those call
signatures changed; there are no startup-error variant patterns, Host
`ExecutionError` literals or direct Host dependencies here. No harness shim or
second process owner is needed. Host/Testkit remain outside the timer library
graph, so no production Wasm, heap or timer instruction delta is expected.
The startup diagnostic changes and version-probe group cleanup stay with
[Testkit #30](https://github.com/dragginzgame/ic-testkit/issues/30) and
[Host #5](https://github.com/dragginzgame/ic-host-tooling/issues/5).

Released 0.23.0 expands the existing startup-error enum; it does not include the
accepted common error record from our handed-off candidate. That candidate is
still unapplied and unqualified. Applying that public cut after 0.23.0 requires
a new Testkit minor and fresh notes; do not overwrite finalized 0.23.0 history.
Our selection consumes only the actual released contract. Server cleanup and
PocketIC instance Drop remain synchronous and unbounded.

At inspection, Testkit's exact-source
[main](https://github.com/dragginzgame/ic-testkit/actions/runs/37772507706) and
[tag](https://github.com/dragginzgame/ic-testkit/actions/runs/37772507963) runs are
queued. Host 0.6 source `6f066e727c977e0b7ec8d3d77821df8508b95c64` has an
[in-progress native run](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37769906817).
Earlier Testkit/Host passes do not qualify this graph. This compatible,
repository-only 0.14.17 draft keeps the full maintainer-operated native/PocketIC
gate; no contributor tests, builds, lint or release commands ran.

### Testkit 0.22 adoption

The maintainer authorized adopting committed Testkit **0.22.0**
`2951fd19e58799580e60ec0f6f5864d296271a62`. Its exact-source
[tag run](https://github.com/dragginzgame/ic-testkit/actions/runs/37752946475)
passed native checks, portable-host qualification and PocketIC concurrency on
Linux, Intel macOS and Apple Silicon. The same source's
[main run](https://github.com/dragginzgame/ic-testkit/actions/runs/37752947368)
passed checks, portable-host qualification and MSRV on all three hosts; its
Intel PocketIC-concurrency job was cancelled, so the overall main run is
cancelled. The tag run supplies all three concurrency passes and executes actual
native checks, not just tag admission; its MSRV job is skipped. These exact-source
upstream results do not execute this consumer.

Initial incoming root catalog/lock selected Testkit 0.22.0 and all four Host
packages at 0.5.1; contributor preparation preserved those bytes. Full locked
offline metadata passed; every member dependency table inherits the root
catalog, all local members remain 0.14.15, and Host/Testkit remain outside the
timer library graph. Registry `pic` module/startup bytes match the reviewed
committed Testkit source.

During the subsequent [consumer audit](../audits/upstream-consumer-review-2026-10-08.md),
an external lock update selected Testkit **0.22.2**
`2da9f92fbe7fc31c37136e99cac46179e08911ad` and all four Host packages
at **0.5.2** `c7014995bf0890c1df9cd9b9a6ec14ea70f98c6f`. It was preserved;
locked offline metadata passes without lock mutation, and Host/Testkit remain
outside the library graph. Testkit's relevant startup/module source is unchanged
from 0.22.0; registry startup bytes match committed 0.22.2. Its
[exact tag run](https://github.com/dragginzgame/ic-testkit/actions/runs/37762453183)
passes native checks, portable hosts and concurrency on all three hosts; tag MSRV
is skipped. Its same-source main run was still executing Intel checks at
inspection, with all three MSRV jobs passed. Host 0.5.2's
[exact CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37762087718)
passes Linux, MSRV and both native macOS gates. These observations do not
qualify the incoming consumer graph; no dependency file was written by the audit.

The maintained [harness](../../../crates/ic-timers-pocketic/tests/runtime/harness/mod.rs)
still explicitly spawns the audited PocketIC binary and connects a fresh
instance to its owned server, retaining instance-before-server destruction.
Testkit delegates server spawn, polling, termination and background reaping to
Host's `OwnedChild`; readiness, deadlines and captured diagnostics remain
Testkit's responsibility. Group signalling precedes leader reaping; cleanup
remains synchronous and unbounded, and PocketIC instance Drop remains unbounded.
The existing APIs used here are unchanged. The Host artifact API and observed
Cargo-build process-group hard cuts require Testkit's 0.22 minor, but our harness
consumes neither of those surfaces. There is no timer public/API semantic cut:
the compatible **0.14.16** draft records this test-only adoption
([Testkit #25](https://github.com/dragginzgame/ic-testkit/issues/25)). No direct
Host dependency, parallel child owner or Cargo patch is introduced. Its digest
exclusion fix has no local caller.

No consumer tests, builds, lint, PocketIC runs or release commands ran during
preparation. The full user-operated gate must qualify this graph, including
native probe lint, watchdog/recovery and policy cohorts. No production Wasm,
heap or timer instruction change is expected from this test-only selection;
no new size/cost measurement is claimed. This repository-only batch can be
bundled with later code work; a maintainer-selected patch retains the complete gate.

Inspection on 2026-10-06 of the 0.13.1 source at
`54bbcfc4985d4657578150cbe7112795297115fd` found both native macOS jobs in
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37370535919)
failed in `test-release-gate.sh` at the default PocketIC path comparison. The
verified artifacts and preceding snapshot/release fixtures passed on each host;
the complete gate did not pass. Make selected the physical `/private/var/...`
workspace, while the fixture expected the logical `/var/...` temporary path.
The fixture now derives its expected cache from `pwd -P` and deliberately enters
through a directory symlink on every host. It retains exact default, environment,
command-line, same-as-default and empty override checks; production provisioning
is unchanged. Source review, shell syntax and diff checks are the preparation
scope. Subsequent 0.13.2 CI qualified the repaired fixture on both native hosts,
as recorded below; the earlier failed attempt remains separate evidence.

For that same source, the Linux `checks` job passed its CI and nested-probe lint
steps. The separate MSRV job and
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37370535920)
failed to acquire hosted runners, without executing their steps. These service
failures do not qualify those gates and are distinct from the macOS fixture bug.

Inspection on 2026-10-06 of release commit
`134899f1620b29f51711ff479c37c681db3fb9ec` found
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37425186029)
successful: Linux checks and MSRV passed, and both macOS 15 jobs passed the
complete release gate under native Bash 3.2. The Apple Silicon job log records
138 native tests, doctests, all 13 PocketIC recovery/ordinary/churn subjects,
and policy cohorts passing with the exact audited PocketIC 16.0.0 artifact.
The Intel complete gate also passed. The matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37425186206)
passed. This qualifies that committed source and its selected dependencies;
it does not establish publication, downstream composition, other host versions,
or qualification for later source. Historical preparation-only and
failed records below retain their original scope rather than describing current
0.13.2 qualification.

For release commit `864397a7c21eec4f396fe9617dbb8d8e1f9cfc73` (0.13.3), the
[tag job](https://github.com/dragginzgame/ic-timers/actions/runs/37430538306)
passed when inspected on 2026-10-06. Its
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37430538342)
passed Linux checks, MSRV and both native macOS complete release gates. The
Apple Silicon log records 142 native tests, doctests, all 14 PocketIC runtime
subjects and policy cohorts passing with the audited PocketIC 16.0.0 artifact.
The [callback ownership record](../design/callback-delivery-ownership.md#capture-removal-and-coalesced-requests)
compares the exact 0.13.2/0.13.3 instruction subjects, including initial-arm and
cancellation increases. This qualifies released 0.13.3 source, not the subsequent
working-tree release-tooling and host-only measurement changes.

For release commit `aa0e933eb56ff0e3e6832ad822f77c95a1392199` (0.13.4),
the maintainer reports publication live and the
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37433514158)
passed. The matching
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37433514493)
passed MSRV but failed Linux checks and both native macOS complete gates.
Linux reached `test-release-index.sh`, reported a shallow source clone and exited
128, with the adapter output hidden in the fixture's deleted temporary file.
Source inspection identifies its next-version preflight's reachable-tag
requirement as an unmet prerequisite in the default shallow main checkout.
Both macOS hosts failed `test-committed-release.sh` with
`committed check accepted manifest ordering failure`; later runtime/PocketIC
and cohort stages were not reached. These results do not qualify the complete
0.13.4 release gate, despite the successful tag job. Registry publication was
not independently checked.

For release commit `98c4b296d7461525c15a01e30adbe33b75bcfa38` (0.13.5),
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37442171230)
passed. The matching
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37442170942)
passed Linux checks, MSRV and both complete native macOS release gates.
The completed host logs confirm the selected-commit and real-index fixtures
passed. Each macOS job records 142 native tests, doctests,
14 PocketIC runtime subjects and policy cohorts with audited PocketIC 16.0.0.
The [cohort review](../design/callback-delivery-ownership.md#released-0135-cohort-review)
records exact installed byte sizes and comparison limits. The tagged root lock
selects registry ic-metrics 0.1.7, while the testing/probe lock selects 0.1.6;
both satisfy the tagged compatible 0.1.6 requirement. Results qualify their
recorded graphs only, not later manifest edits, publication or composed consumers.

For release commit `902323a9e896ce3771044fdc23a7a2d03d49cf28` (0.14.0),
the maintainer reports publication live and the matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37476634555)
passed. Inspection on 2026-10-06 of
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37476635415)
found Linux checks, MSRV and both complete native macOS gates successful.
Both independent release locks select registry ic-metrics
0.2.0. The completed Apple Silicon log records 142 native tests, doctests,
14 PocketIC runtime subjects and policy cohorts with the audited ARM64 server
digest `781f643d4b16105e7544ca810a972f99c0ef1919016c680faa93f10909a14496`.
Its six cohort/calibration rows match the same-host 0.13.5 rows exactly, including
Wasm bytes, as recorded in the [measurement owner](../design/callback-delivery-ownership.md#ic-metrics-02-adoption).
Registry publication was not independently checked; the subsequent parser/checker
adoption has its separate failed qualification below.

Release `fbd319de7a60d6475232439e398fff2263a9d666` (0.14.1) is reported live.
Its [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37483380254)
passed MSRV but failed Linux checks and both macOS complete gates; matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37483380756)
also failed. All four failures report `scripts/ci/test-dependency-pins.sh:
line 45: rg: command not found`. Parser setup and the preceding host-tool
fixtures passed; the shared rejection fixture then could not inspect its output.
This is missing consumer CI provisioning, not proof of a checker admission bug
or timer runtime failure. It does not qualify the complete 0.14.1 release gate.
The compatible repair adds explicit ripgrep bootstrap through apt on Linux and
Homebrew on both macOS hosts, before fixtures, with a command-availability check.
The shared snapshot, ordinary offline validation and the complete gate stay
unchanged. Shell/source/diff inspection is repair evidence; rerunning native
qualification remains maintainer-owned. Details belong in the
[adoption owner](shared-tooling-history.md#structured-checker-ci-repair).

Pushed 0.14.2 at `88aedf0a5353d176037062ae262dd67bae11beae` passed
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37489635619)
and main [Linux/MSRV jobs](https://github.com/dragginzgame/ic-timers/actions/runs/37489635451).
Both complete macOS jobs were queued at inspection; latest inspected complete
all-host qualification remains 0.14.0. These results qualify the earlier
helper/CI repair on the inspected hosts, not the subsequent formatter worktree.

For release commit `e001ab98195d3c8541430a934fd76f756c0717d2` (0.14.3),
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37493326312)
passed Linux checks, MSRV and both complete native macOS release gates.
The matching [tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37493325538)
passed its exact-tag/main-ancestry checks. At that inspection it was the latest all-host
qualification, including formatter prerequisites, changelog preparation and the
earlier Cargo/IC helper wiring. Registry publication was not independently
checked. These results do not qualify the later 0.14.4 tag-checker and tooling
worktree. The [0.1.11 refresh owner](shared-tooling-history.md#shared-tooling-0111-refresh)
records exact upstream all-host evidence and pending consumer qualification.

For release commit `49e4e8a25025c6a6329c993ea85255341474682b` (0.14.4),
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37505847432)
passed Linux checks, MSRV and both complete native macOS release gates. Matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37505847427) passed
exact tag/version and main ancestry admission. These results qualify tag-checker
delegation, the 0.1.11 logger refresh and the repaired restricted-PATH fixture.
The maintainer-authorized closure of #16 records that evidence. These results
qualify that release, independently of later source. Registry publication was not
independently checked.

For release commit `c84d4e4d26f968a9d7f2d37f30f7fed447692c0f` (0.14.5),
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37526800896)
passed Linux checks, MSRV and both complete native macOS release gates. Matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37526801007)
passed exact tag/version and main ancestry admission. All three hosts' logs pass
the consumer snapshot export/corruption fixture, the shared standard-release
command checker and release runner command substitutes. Both macOS gates record
142 native tests, 14 PocketIC runtime subjects, doctests and policy cohorts.
This completed consumer qualification closed #17/#18 for the actual adopted
0.1.12 wiring. The
[adoption owner](shared-tooling-history.md#shared-tooling-0112-adoption) records its scope
and the separate unqualified Make-mode boundary reported upstream. Normal hosted
success does not qualify ignore-errors or non-executing Make modes. Registry
publication was not independently checked. No new local validation ran during
the evidence review.

Released 0.14.6 adopts Shared Tooling 0.1.13 and moves
the three host/probe packages under the independent `testing/crates/` root.
Source-bound upstream Linux/macOS success and cheap consumer locked metadata,
path/hash and integrity inspection are recorded in the
[workspace adoption owner](shared-tooling-history.md#shared-tooling-0113-workspace-adoption).
At `0c90c391dff5960a7502fc15a0718b03631f2515`,
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37584151377)
passed Linux, MSRV and both complete native macOS gates.
Matching [tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37584150869)
passed. #19 was closed with this source-bound complete qualification; this is
the latest inspected complete consumer release. Make/CI workspace roots, package
selectors, `testing/target` artifacts and the exact PocketIC gate remain.
The preserved root-lock ic-metrics update is qualified with this release.

Released 0.14.7 adopts Shared Tooling 0.1.14 and the shared Make execution guard.
Upstream Linux and both macOS jobs passed at the exact exported revision;
consumer preparation checks cover snapshot integrity, source/mode comparison,
shell syntax, documentation and diff inspection. No contributor tests, builds,
lint or formatter ran. At released `a30bfe01d9ce82ba691f5ddd1980f9a4b7c0454a`,
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37592893527) passed
Linux/MSRV and both complete native macOS gates.
Matching [tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37592893540)
passed. The logs record Make admission, gate, hook and collector fixtures passing
on Linux and both macOS hosts; #20 closes with this complete source-bound
qualification. New .8 edits retain their own pending qualification. See the
[Make admission owner](shared-tooling-history.md#shared-tooling-0114-make-admission).

Version preparation uses Bash, Perl, Git, Cargo and the explicitly installed
jq/yq parser pair. It owns regular metadata
files; symlinked or non-file outputs are rejected before mutation. Applicable
Make targets use GNU Make. macOS 15 supplies Bash 3.2, GNU Make 3.81 and the
standard BSD/Unix tools used here. Required host tools are Git, Perl with core
`JSON::PP`, `File::Compare` and `Digest::SHA`, `curl`, `gzip`, and Rustup/Cargo.
The SHA-256 boundary uses `Digest::SHA`, without requiring GNU `sha256sum` or a
Homebrew tool installation. Native host prerequisites and setup must be qualified
at their owning workflow boundary.

For local macOS setup, install Rustup, then run `make update-dev` to install the
development toolchain declared in `rust-toolchain.toml`, its components, Wasm
target and pinned host/IC bundles. The full release gate also needs the MSRV toolchain:

```text
rustup toolchain install 1.88.0 --profile minimal --component clippy --component rustfmt --target wasm32-unknown-unknown
make release-verify
```

These are maintainer-operated setup and validation commands, not publication.
The macOS CI jobs install both toolchains and the Wasm target, verify the OS and
architecture against this matrix, and prepend `/bin` to `PATH` so nested
`env bash` wrappers exercise Apple's Bash 3.2. The two jobs run only for PR/main;
their complete gate includes dependency preparation, native CI, MSRV, nested
probe linting, the maintained PocketIC subjects and policy cohorts. Existing
Linux jobs and the smaller tag job remain separate. Linux checks also run the
maintained PocketIC subjects/cohorts with the prepared internal toolchain; the
separate MSRV job retains minimum-version library and probe qualification.
The jobs use explicit
[GitHub runner labels](https://docs.github.com/en/actions/reference/runners/github-hosted-runners#standard-github-hosted-runners-for-public-repositories).
CI configuration supplies a qualification path; only passing native execution
for the matching revision supplies evidence. Adding these jobs does not claim
that they have passed or qualify other macOS versions.

The version-preparation and impact fixtures use positional arguments when a
command may take no arguments, avoiding empty-array expansion under Bash 3.2's
`set -u`. Their changed scenarios have not been executed.

The repository and release-gate fixtures compare ordered newline records
directly with `cmp`. They no longer depend on Bash 4's `mapfile` or turn those
records into joined arrays. Expected gate order remains independent of Makefile
variables, including repeated PocketIC prerequisites and the exact prefix before
an injected leaf failure. Provisioning checks compare both override path and
automatic-install selection, including an empty override. Source and shell syntax
were reviewed; the changed fixtures have not been executed. This removes one
known Bash 3.2 obstacle without establishing native macOS qualification.

The preparation fixture uses the same direct record comparison for phase order
and the four staged metadata paths. It captures Git output with an ordinary
command before comparing, retaining failure propagation without an intermediate
array or process substitution. Empty-index and no-tag assertions also capture
Git output before testing it, so producer failures cannot satisfy those assertions.

Preservation fixtures retain actual file copies rather than checksum manifests.
The formatting-hook fixture compares working bytes and independently compares
the binary staged diff. The preparation fixture copies its metadata and unrelated
files with their modes, then compares bytes with `File::Compare` and permission
bits with Perl `stat`. Preflight rejection, symlink rejection, every injected
rollback failure, interruption, missing-changelog restoration and tag rejection
retain their existing subjects. Mode comparison also applies to the preflight
and tag-rejection checkpoints. Fixture mutations use Perl instead of GNU `sed -i`.
These changes remove fixture-only `sha256sum`, GNU `stat -c` and GNU in-place sed
requirements; they do not remove cryptographic verification of external binaries.
Shell and embedded Perl syntax and source flow were reviewed. The changed
fixtures have not been executed, and native macOS qualification remains open.
On 2026-10-05, read-only changelog finalization for the selected 0.11.8 target,
the current README version projection, both locked offline Cargo metadata checks
and diff whitespace checks passed. Cargo versions and both lockfiles were unchanged.

For 0.11.8, the PocketIC verification fixture compared exact ordered event records,
including both cache and download checks during rejection. Debris searches run
as ordinary commands before empty-result assertions. This tightens the fixture's
producer-failure handling without changing the audited version, digest, binary
verification or override ownership. Source and shell syntax were reviewed;
the changed fixture has not been executed.

### CI failure evidence

All four validation/tag jobs prepare a dedicated `$RUNNER_TEMP/ic-timers-fixtures` directory
after checkout and set subsequent steps' `TMPDIR` to it. Each job ends with
failure-only archive and upload steps, after installation and its final check;
the macOS matrix retains separate Intel and Apple Silicon artifacts. Earlier
failure status is never turned into success by collection.

[The collector](https://github.com/dragginzgame/ic-timers/blob/651efce370bd6aec6290b74118eb709488062d0a/scripts/ci/collect-failure-evidence.sh) selects that scratch
tree, `.git/release-state/validation-failures`, `target/validation-failures` and
the physical `.tools/rust` tree and Testkit provisioning attempts. Shared selection
owns retention/compaction of referenced host/IC bundles; on store rejection,
[local retention](#shared-store-rejection-and-local-failure-retention) remains
available with explicit incomplete status. General build caches, successful Testkit
server bundles and fixture `.git` entries are excluded. Tar records symlinks without following them and
preserves modes, including the metadata modes used by release regressions.
`identity.txt` records actual checkout and event SHAs, job, host, run and attempt.
If no diagnostic payload exists yet, the archive still records that identity.

The released 0.15.0 collector reports `archive_bytes` (the completed compressed
tar's byte size) and `archive_seconds` (time spent in the archiver, using Bash's
whole-second counter) in its collection log. The existing upload includes that
log, so fresh hosted evidence retains the measurements without another artifact
or archive pass. Zero seconds means completion within the counter's resolution;
this is not a subsecond benchmark. Selection/check time, upload/download time and
the outer artifact ZIP size are separate. Failed archiving emits no completed
measurements. With the maintainer's explicit #30 instruction, the focused
collector and downloaded-verifier fixtures passed locally on 2026-10-09. Native
qualification of this measurement addition remains separate from
the frozen 0.14.23 compact round trips [below](#compact-hosted-qualification-at-01423).

The official upload action is pinned to 7.0.1 at
[`043fb46d1a93c77aae656e7c1c64a875d1fc6a0a`](https://github.com/actions/upload-artifact/tree/043fb46d1a93c77aae656e7c1c64a875d1fc6a0a).
Its [documented permission behavior](https://github.com/actions/upload-artifact/blob/043fb46d1a93c77aae656e7c1c64a875d1fc6a0a/README.md#permission-loss)
requires tar for mode preservation. The uploader selects only the tar archive
and collection log, with a unique source/job/OS/architecture/attempt name and
the repository's normal retention period. Download the artifact from the failed
run and extract its `ic-timers-failure-evidence.tar.gz` with `tar -xzpf` in a
scratch directory. A collection failure prints and uploads its diagnostic log;
its archive may be absent or incomplete. Hosted job logs remain available too.

The maintained `release-check` gate selects
[collector fixtures](https://github.com/dragginzgame/ic-timers/blob/651efce370bd6aec6290b74118eb709488062d0a/scripts/ci/test-failure-evidence.sh) for contents, identity,
mode preservation, symlink handling, exclusions, empty payloads, rejected inputs
and archive failure propagation. These released fixtures passed on Linux and
both native macOS hosts at 0.14.7 `a30bfe01d9ce82ba691f5ddd1980f9a4b7c0454a`;
see the [host record](#host-support).

The released .8 follow-up preserves failed committed-release, staging/index,
lockfile and repository-check scratch under TMPDIR, prints its location and
retains the original failure status. Successful runs still clean up. Its collector
fixture drives each actual producer through early injected tool failure, checks
status/input preservation, then compares those inputs after collection/extraction.
Those paths select substitutes before builds or real Git writes. The complete
released 0.14.15 gate passes these maintained cases on Linux and both native
macOS hosts; contributor preparation ran no tests. The original .7 evidence
does not qualify subsequent fixture additions.
Closing
[#23](https://github.com/dragginzgame/ic-timers/issues/23) requires native Linux
and both macOS fixture qualification plus downloadable evidence from
maintainer-controlled early-installer and late-check failures on the declared
hosts, preserving the failing job result. Normal green CI does not prove upload
execution. Contributor preparation dispatches no failing workflow or release.
Force-terminated runners cannot guarantee collection.

Released repository-only 0.14.13 adds an explicit manual qualification
path to the same CI workflow. The workflow is now on main; choose
**CI → Run workflow → failure_stage: early**, then repeat with **late** at the
same frozen source ref, such as released `v0.14.15`.
[GitHub's manual input contract](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#onworkflow_dispatchinputs)
requires the workflow on the default branch. The equivalent maintainer commands
are:

```sh
gh workflow run ci.yml --repo dragginzgame/ic-timers --ref v0.14.15 -f failure_stage=early
gh workflow run ci.yml --repo dragginzgame/ic-timers --ref v0.14.15 -f failure_stage=late
```

Each dispatch selects Linux checks and both native macOS jobs. Early qualification
runs before Rust/tool setup; it calls the real host installer in a fresh scratch
consumer with a curl substitute that retains partial download bytes and exits
22. Late qualification runs only after Linux's CI, nested lint and product
PocketIC targets or macOS's complete release gate succeed; it calls the real
logger against a failing
scratch Make recipe, preserving Make status 2 and raw/combined failure logs.
This proves workflow retention ordering with controlled substitutes, not a real
network outage or a canister failure. The original jobs stay failed; the driver never
uses `continue-on-error`. Existing final failure-only collector/upload steps own
the artifacts. CI keeps only the newest run per workflow and branch or PR ref,
including manual dispatches, under
[the shared policy](https://github.com/dragginzgame/shared-tooling/issues/108).
A manual observation can therefore supersede normal validation or another stage
on the same ref; dispatch observations sequentially when both stages are needed.
Cancelled runs do not qualify their source. PR/main gates retain their normal
scope; manual evidence runs skip the duplicate MSRV/tag jobs.

The [driver](https://github.com/dragginzgame/ic-timers/blob/651efce370bd6aec6290b74118eb709488062d0a/scripts/ci/qualify-failure-evidence.sh) retains
`ic-timers-fixtures/hosted-<stage>.<suffix>/` with `before.txt`, `after.txt`,
`scenario.log` and `status.txt`. Review each downloaded archive's identity against
its actual source, job, host, run and attempt; compare before/after bytes and 0640
modes, and require the selected stage and expected status in `status.txt`. Early
artifacts must include the real retained
`tool-store/host-<64-hex-identity>/set.<generation>/bin/jq` within the scenario,
containing the controlled rejected bytes as a regular file. The checkout's
`consumer/.tools/host-set.*` entry is a reference, not that physical payload.
Late artifacts must include
the controlled error in `target/validation-failures/latest-combined.log` and its
raw per-target log. A generic setup/runner failure or missing controlled marker
does not qualify the scenario. Record both run links and all six host/stage
artifact observations on #23 before closing it.

The manual-only `failure-evidence` matrix downloads those three artifacts through
the exact-pinned download action adopted by Shared Tooling 0.1.20. It checks
the [archive verifier](https://github.com/dragginzgame/ic-timers/blob/651efce370bd6aec6290b74118eb709488062d0a/scripts/ci/verify_failure_evidence.py)'s rejection
fixtures, then verifies the downloaded tar without extracting it. Identity must
match the selected source, original job/host, run and attempt. Known input bytes,
0640 modes, controlled status, partial installer bytes or raw/combined logger
output must all be present. Duplicate/unsafe archive paths and substituted links
cannot stand in for required regular evidence files. Python 3 from the Ubuntu
runner is used only in this manual verification job; it adds no developer,
macOS, library or release-gate prerequisite.

Require all three verification jobs to pass while the original three jobs fail
at the selected injection. A missing artifact, source mismatch or unrelated
setup failure cannot qualify the run. Early and late dispatches together supply
six host/stage observations; keep their run links on #23. Rerunning only a
verification job cannot qualify artifacts from another attempt: rerun the
complete selected manual dispatch when retrying this evidence.

The existing collector fixture now exercises both driver stages in an isolated
checkout with fake CI identity, rejects invalid/nonmanual calls, and compares
retained inputs/logs after archive extraction. This is prepared local coverage;
neither those new fixtures nor the hosted dispatches ran during preparation.

Released #24/#25/#28 are closed after 0.14.13 supplies the missing complete
Intel gate for the unchanged 0.14.12 adoption. The
[source-bound acceptance](shared-tooling-history.md#consumer-adoption-qualification)
records exact source identity, the 82 unchanged files and each native host's
actual dependency graph. Apple Silicon subsequently passed at the exact 0.14.13
source as well. The maintained driver/collector fixture now passes all three
native gates; #23's downloaded early/late failure evidence remains separate and
was unexecuted at that historical review. No contributor rerun or dispatch was
performed during that preparation.

#### Shared failure archiver adoption

Current follow-up: all six frozen 0.14.17 hosted observations below are complete,
and released 0.14.21 passes the collector/path fixtures on all native hosts. The
compatible **0.14.22** draft now calls the already-vendored shared compact selector
with `ci/tool-versions.env` and `ci/ic-tools.tsv`. It retains full failed/changed/
unselected bundles; freshly verified exact active bundles retain their pins,
check logs, selection identity and IC receipts under `tool-evidence/`. Original
source/job/run/attempt identity, failure status, raw logs, fixture payloads,
partial-archive retention and upload/download names remain. A selector failure
aborts before archiving and retains partial selection output and metadata.
No executable compatibility path or separate full-mode caller remains locally.

The reviewed committed 0.1.32 follow-up also reuses equivalent validated IC pin
records across comments/order, preserving original installation receipts. The
download verifier uses the canonical AWK admission for those receipt records while
still requiring exact source-selected caller-pin evidence. Changed, malformed
and duplicate selections fail. Both local collector and downloaded-byte fixtures
cover distinct caller/installed provenance; no receipt rewrite occurs.

[Shared #66](https://github.com/dragginzgame/shared-tooling/issues/66) has complete
native compact upload/download acceptance at `db039347d2372b877c1c46dcdd2b5c3aa9412009`;
the currently selected selector is byte-identical. The actual consumer fixture
adds tiny authenticated host/IC omission, corrupt-active retention, pins/receipt/
identity/log comparisons and failed-producer coverage. Existing path, status,
mode/link and archive-refusal cases remain. The existing downloaded verifier now
requires late compact selection records, exact caller pins/IC receipts and no
verified payload; negative fixtures reject missing/changed evidence and retained
active payloads. These checks qualify the current collector at its own source,
without relabelling the frozen 0.14.17 observations. No test/build/lint or hosted dispatch
ran during preparation. #30 remains open for the new collector's complete
user-operated native gate and source-bound early/late hosted transport, then
measurement of actual bytes and collection time. Old large-artifact measurements
prove the opportunity, not this collector's saving. The collector changes no
timer API or dependency selection; incoming lock edits are separate. No timer
Wasm/instruction/heap change or new IC recovery guarantee is claimed.

#### Compact hosted qualification at 0.14.23

On 2026-10-09 the maintainer's instruction to complete #30 authorized the focused
collector/download-verifier fixtures and two new manual dispatches at frozen
**v0.14.23**, `10a392f98d42701959d0c1d2deddfbef5c96144a`, attempt 1:
[early 37912633994](https://github.com/dragginzgame/ic-timers/actions/runs/37912633994)
and [late 37912637422](https://github.com/dragginzgame/ic-timers/actions/runs/37912637422).
These qualify the released compact caller; they do not execute the uncommitted
0.14.24 archive measurement output or incoming lock selections. No duplicate
observation was dispatched.

Both Linux producers reached their selected controlled failure (early status 22,
late status 2) and archived/uploaded successfully. The late producer first passed
its normal CI and probe lint. Actual downloaded ZIPs passed the maintained verifier
from the frozen source, including original identity, modes, status and compact
pin/receipt admission. Measurements are separate for the outer ZIP and inner tar:

| Stage / host | Artifact | ZIP bytes | tar.gz bytes | Regular payload bytes |
| --- | --- | ---: | ---: | ---: |
| Early Linux/X64 | 11606299473 | 1,341 | 872 | 779 |
| Late Linux/X64 | 11606958330 | 5,118 | 4,649 | 20,555 |
| Early macOS/ARM64 | 11610964142 | 10,711 | 10,240 | 781 |
| Early macOS/X64 | 11615583808 | 10,711 | 10,240 | 779 |
| Late macOS/X64 | 11617154805 | 10,711 | 10,240 | 20,699 |
| Late macOS/ARM64 | 11619245834 | 10,711 | 10,240 | 20,700 |

The late Linux ZIP is 99.9986% smaller than the previously measured 373,200,529-byte
0.14.15 ZIP. This is an observed cross-release artifact comparison, not a
same-source benchmark: retained diagnostics and tool selections differ. The
current verifier proves the verified active payloads are omitted while required
logs/pins/receipts and controlled failure evidence remain.

GitHub's archive-step timestamps fall within one whole-second bucket on Linux
at both stages. This measures the entire collection step, including selection,
and gives no subsecond archiver timing. The frozen source has no `archive_seconds`
output; the released 0.15.0 instrumentation needs its own complete native gate.
Local download/unpack/verification timings are separate from hosted collection.

Early ARM reaches its controlled status 22 and archives/uploads successfully;
its actual downloaded ZIP passes the frozen-source verifier. ZIP SHA-256 is
`ac6b1567b831cae7805e351266be2a8ebf07be79682b0b1c8f244033432d2aa8`; inner tar
SHA-256 is `a364c4887a9b4a03665d33341bdbbc4ef6d4cd853284651ad8d55ad1989c8f27`.
Early Intel and both late macOS producers subsequently reach their selected
failures; actual downloads also pass the frozen-source verifier. Both late macOS
jobs first pass their complete native release gate. All six original producers
archive/upload successfully, and all six hosted verifier jobs pass: early jobs
113819934118 / 113819934339 / 113819934371 and late jobs
113844785934 / 113844786086 / 113844786094. The overall workflows intentionally
remain failed because their producers preserve statuses 22 and 2.

The remaining macOS archive-step timestamp spans are one second for early Intel,
three seconds for late Intel and one second for late ARM. These are whole-step
API observations, including selection, not precise archiver measurements. The
completed six observations satisfy #30's compact transport acceptance, without
relabelling #23's earlier six observations or qualifying the later 0.15.0
five-tool/Testkit handoff and measurement instrumentation. Retained identities,
ZIPs, SHA-256 hashes, logs, API step timestamps and measurement reports are under
`/tmp/ic-timers-issue30.vdZlUd/`; hosted artifact retention still has its normal
expiry. Focused local fixtures passed, and Cargo manifest/lock plus the existing
worktree were byte-preserved through those checks. No commit, release, version
mutation or dependency update ran.

The following records describe the earlier archiver-only preparation.

The 0.14.17 draft prepares
[#30](https://github.com/dragginzgame/ic-timers/issues/30) through the reviewed
0.1.26 [snapshot](shared-tooling-history.md#shared-tooling-0126-preparation).
`collect-failure-evidence.sh` still owns checkout/event/job/host/run/attempt
identity and selection: fixture scratch, release-state validation logs, native
validation logs and all host/IC candidate sets. It passes root/relative-path
pairs to `archive-evidence.sh`, replacing its inline tar/exclusion assembly.
The shared helper's payload is unchanged from the all-host-qualified archive
source `eeb72e7`.

The helper refuses occupied output paths and returns status 1 on an archive
failure, preserving original inputs and any partial archive. A retry must use
fresh runner scratch or preserve/relocate the previous archive first; collection
does not overwrite retained output. Original controlled command status remains
in the scenario's `status.txt` and job outcome, independently of helper failure.
The existing workflow output name/upload/download identity is unchanged.

The download verifier admits one conventional `./` prefix before canonical-name
and duplicate checks. `identity.txt` plus `./identity.txt` is still a duplicate;
absolute, traversal, empty and internal-dot components remain refused. Both
released bare-name archives and newly prefixed archives have the same evidence
identity contract. No additional wire discriminator or alternate verifier exists.

Maintained fixtures now include colon/newline filenames, actual managed relative
activation symlinks, retained selected/unselected bundles, partial tar failure,
occupied-output refusal and fresh paths for independent collection attempts.
The manual verifier adds prefixed acceptance and alias/traversal rejection.
Those fixtures have not run during preparation. Native Linux/Intel/ARM consumer
qualification and fresh early/late hosted round trips remain acceptance work;
the closed #23 observations below retain their original source and scope.

Full installer retention remains the default. The measured >99.99% installed-tool
overhead is not fixed by this archiver adoption, and no size reduction is claimed.
[Shared #66](https://github.com/dragginzgame/shared-tooling/issues/66) owns the
compact-selection policy: exclude a managed active bundle only after successful
verification bound to that exact selection; retain it on failed verification,
changed selection or unknown ownership. Host lacks the IC receipt shape, so
glob-based exclusions would lose useful diagnostics. Keep #30 open until the
adapter's consumer acceptance and the selection obligation are resolved.

No named function, method or type is deleted by the adapter. Inline archive
mechanics are replaced by their shared owner. No timer runtime/API, provider,
PocketIC version or release-gate selection changes; expected production
Wasm/instruction impact is zero. Source, syntax and snapshot checks are
preparation evidence only.

#### Evidence path repair and 0.14.17 qualification

The compatible **0.14.18** draft repairs consumer-owned evidence bootstrap for
[#30](https://github.com/dragginzgame/ic-timers/issues/30), following the reviewed
pattern from [Shared #67](https://github.com/dragginzgame/shared-tooling/issues/67).
The collector, qualification driver and fixture anchor relative operands before
physical `cd`. Captured PWD carries a non-newline suffix, so command substitution
cannot strip legal terminal newline bytes or mix CDPATH diagnostics into a path.
The driver still rejects a different physical workspace and accepts a matching
alias. Temporary roots are resolved explicitly; Git object-path capture preserves
the producer's framing separately. No shared payload is patched or new path API
introduced. No named function, method or type is removed.

The maintained fixture adds copied consumer scripts under a physical checkout
ending in a newline, inherited CDPATH, a matching workspace alias, a different
workspace rejection and a newline-ending runner directory. It drives both stages
and compares retained status/input/log bytes after actual shared archiving.
The reviewed [Shared Tooling 0.1.27 refresh](shared-tooling-history.md#shared-tooling-0127-preparation)
repairs the canonical bootstrap, so these additional cases now copy the real
installer/logger and their companions. The temporary path-only substitutes are
removed. Both normal and newline-ending path cases check the real retained
candidate bytes, controlled status and late combined log. This coverage does not
claim that every vendored script works from a newline-ending checkout. Fixture
execution, Bash 3.2 and native qualification of the local fix remain user-owned;
preparation ran integrity/source/syntax/diff checks only.

The maintainer's explicit request to do the proposed hosted qualification
authorized two new frozen-source dispatches at **v0.14.17**,
`031e6c67dccdd043ff11e20d9978359a4ec6afc8`, attempt 1:
[early 37776644653](https://github.com/dragginzgame/ic-timers/actions/runs/37776644653)
and [late 37776654992](https://github.com/dragginzgame/ic-timers/actions/runs/37776654992).
These execute the released shared archiver and Testkit 0.23 / Host 0.6 graph,
not the uncommitted path repair or incoming dependencies. Do not redispatch a
lost reply or relabel old #23 acceptance. Original job failures are intentional;
acceptance requires each expected failure, successful archive/upload and exact
downloaded identity/bytes/modes/status.

Both Linux checks jobs reached their selected controlled failure and passed
archive/upload. Early artifact **11549169587** is 1,342 bytes; late artifact
**11550069656** is 373,203,770 bytes (GitHub-reported ZIP sizes). Both Linux downloads pass the byte-exact released verifier. The native
late macOS observations now reach the intended controlled failure after complete
native release gates, archive/upload successfully, and pass the hosted downloaded
verifier: Intel artifact **11555254656** (228,372,951-byte ZIP), verifier job
**113361599275**; Apple Silicon artifact **11556628023** (213,944,791-byte ZIP),
verifier job **113361599424**. Linux hosted verifier **113361599302** also passes.
The early Intel job **113309082998** reaches its controlled failure and uploads
artifact **11553654328** (10,711-byte ZIP); its local download passes the byte-exact
released verifier. Early Apple Silicon subsequently reached its intended failure
and uploaded artifact **11557756245** (10,711-byte ZIP,
`sha256:b30b088be65b1836b7e2ef9a2fd95fa17092b4f5799c188f912f9d58b238be91`).
All three early hosted verifier jobs **113366289284 / 113366289320 / 113366289845**
pass, including rejection cases, actual download and exact source/status/bytes/
modes. The Apple Silicon verifier log explicitly names that artifact and reports
`Downloaded early failure evidence qualified for macos macOS/ARM64`.
Both runs remain at attempt 1 with their intentional failed producer outcomes:
**all six** source-bound host/stage observations are now complete. The macOS
late and early ARM archives were verified by maintained hosted jobs, without
another local download. Earlier local Linux/Intel receipts keep their scope.
Artifacts, original job logs and the frozen verifier remain under
`target/evidence/hosted-failure-artifacts/`; the final ARM verifier log inspected
on 2026-10-09 is retained at `/tmp/ic-timers-01417-early-arm-verifier-api.log`.
No new dispatch, local verifier execution or release operation ran in that review.

At the frozen source, full installer retention remains selected. The compact
selector's corrected db039 full/compact transport now passes all three native
hosts; current consumer preparation is recorded in the
[adoption owner](shared-tooling-history.md#compact-consumer-evidence-preparation).
No consumer archive-size saving or stronger IC recovery guarantee is claimed.
The frozen observations qualify 0.14.17's archiver and original graph, not later
path changes, compact selection, unrelated outages or other package identities.

#### Hosted qualification at 0.14.15

On 2026-10-08 the maintainer explicitly authorized both manual qualification
dispatches and inspection of the six artifacts. Both runs select immutable
`v0.14.15`, commit `ae26b854a1a473c5d2d153705c2d13570f378d38`, attempt 1:
[early 37756952434](https://github.com/dragginzgame/ic-timers/actions/runs/37756952434)
and [late 37756960783](https://github.com/dragginzgame/ic-timers/actions/runs/37756960783).
These execute the released Testkit 0.21.3 / Host 0.4.6 graph, not the separately
prepared Testkit 0.22 adoption.

Both runs completed with all three original jobs failed at their intended
injection and all three hosted download-verification jobs successful. The overall
run conclusions are **failure by design**. Each hosted verifier also passed its
negative/rejection cases. Late Linux passed normal CI/probe lint and both native
macOS jobs passed the complete release gate before the controlled failures.

| Stage | Original host | Artifact ID | Hosted / local downloaded verification |
| --- | --- | --- | --- |
| Early | Linux/X64 | 11539929341 | Passed / passed |
| Early | macOS/ARM64 | 11541110407 | Passed / passed |
| Early | macOS/X64 | 11541391837 | Passed / passed |
| Late | Linux/X64 | 11541230299 | Passed / passed |
| Late | macOS/ARM64 | 11540443940 | Passed / passed |
| Late | macOS/X64 | 11542322043 | Passed / passed |

All six downloaded archives match exact source/job/host/run/attempt, controlled
status 22/2, known input bytes and 0640 modes. Early rejected download bytes and
late raw/combined logs survived actual upload/download. Local verification used
the unchanged released `verify_failure_evidence.py`, with each original run's
identity supplied explicitly; tar entries were read without extraction. Archives,
full original/verifier logs, run/job/artifact metadata and hash observations are
retained under ignored `target/evidence/hosted-failure-artifacts/`. Exact tar
SHA-256 values and acceptance are also recorded in the
[closure evidence](https://github.com/dragginzgame/ic-timers/issues/23#issuecomment-6057617574).

An initial local late-Linux verifier invocation preceded download completion
and refused the missing file; verification after completion passed. Original
logs were temporarily unavailable through `gh run view --log` while their run
was active; full logs were retained after completion. Intel's earlier runner
queues and partial observations are superseded by actual terminal acceptance.
No inconclusive attempt was counted as qualification.

[#23](https://github.com/dragginzgame/ic-timers/issues/23) is closed with all six
hosted and local downloaded observations. This qualifies retained diagnostic
identity/bytes/modes through controlled failure ordering, not real network outage
behavior, force-terminated runners or canister recovery. The incoming Testkit
0.22 graph remains outside these frozen-source runs. No local fixture suite,
version bump, staging, commit, tag, push or publication was performed.

Actual late-Linux download also proves the collector includes active installed
tool bundles. Artifact 11541230299 is 373,200,529 bytes; installed host/IC bundle
targets contribute over 99.99% of its 766,116,256 uncompressed regular-file bytes.
The collector's `host-set.*` / `ic-set.*` selection matches successful installer
targets as well as failed candidates. The existing fixture models an ordinary
`.tools/host` directory rather than the installers' relative symlink layout.
The measured finding and smallest consumer-owned exclusion/fixture repair are
recorded on [#30](https://github.com/dragginzgame/ic-timers/issues/30#issuecomment-6057035304).
This archive-scope follow-up is separate from #23's required upload/download
observations and from the not-yet-committed Shared Tooling archive helper.

The subsequently completed **0.16.3** native jobs
[ARM 113880561216](https://github.com/dragginzgame/ic-timers/actions/runs/37948400741/job/113880561216)
and [Intel 113880561677](https://github.com/dragginzgame/ic-timers/actions/runs/37948400741/job/113880561677)
both fail during tool preparation at `Makefile:270: multiple target patterns`,
before any native release gate. Logs retained under
`/tmp/ic-timers-0165-review.5lms07fg/` bind that failure to the combined
target-specific override/export syntax. **0.16.4** uses separate global override
assignment and export for direct delivery, removing that exact parse defect.
Its two native jobs are queued at inspection, so neither the source repair nor
older Linux checks qualify native acceptance. Do not retain the superseded
"0.16.3 native jobs queued" description as current evidence or rerun those jobs.

### Formatter prerequisites

Both `make fmt` and `make fmt-check` depend on `format-tools-check`. The reviewed
shared guard at `scripts/ci/check-format-tools.sh` requires successful exact
cargo-sort 2.1.4 output using the `ci/tool-versions.env` pin and successful
rustfmt availability for the selected toolchain. Failed probes reject even if
stdout looks correct. They force Cargo
offline and disable rustup automatic installation; setup remains explicit through
`make update-dev` or CI. The guard neither formats nor builds. The following
formatter recipes in `make/rust-format.mk` cover every member of the single root
workspace with their existing options. The hook's isolated index must include
both guards, `scripts/ci/run-formatting.sh`, all four Make includes, the current
Makefile and `ci/tool-versions.env`.
These formatting entrypoints also bind snapshot routing to their current root;
an inherited external snapshot cannot replace the indexed prerequisite checker.
Their pin input is bound to the indexed `ci/tool-versions.env`, preserving the
previous consumer-owned formatter selection rather than ambient host pin routing.
Fixture wiring and pending native qualification belong in the
[adoption owner](shared-tooling-history.md#shared-tooling-0211-formatting-and-make-admission).

The adjacent `make/execution.mk` admits the running Make executable before
recipes, using the probe beside the selected include. Shared 0.2.11 probes
MAKEFLAGS and MFLAGS independently through GNU Make's option parser. It rejects
ignore-errors, dry-run, touch and question modes even when MAKEFLAGS is cleared
or replaced, and refuses assignments that erase Make's retained MFLAGS evidence.
[Shared #30](https://github.com/dragginzgame/shared-tooling/issues/30) records the
completed upstream repair. Recursive `MAKE` arguments stay with the consumer invocation;
they do not enter the isolated probe. Runtime snapshot/delivery/pin bindings
remain authoritative at their existing target boundaries. All scratch callers
copy the execution companion and probe before their first Make parse; the
index-hook fixture stages them too. This requires no tool installation or Cargo
selection and does not qualify actual consumer execution until the gate runs.

The consumer hook fixture keeps different index and unstaged `Cargo.lock` bytes.
Successful selected-file formatting and direct checks must preserve both; existing
mode-refusal, partial-staging and failed-formatter comparisons also include the
working lock. This closes #35's missing lock-preservation assertion without
changing hook behavior. These additional pending 0.16.6 cases remain unexecuted
by the contributor.

### Structured dependency checks and host parsers

`make actions-check` retains the existing gate entry point and delegates to
the reviewed shared structured checker. It parses workflows, composite actions
and every root workspace member manifest. Git inventory failures, malformed
metadata and missing or untracked workspace lockfiles fail closed. The checker
does not resolve dependency versions, install tools or provide runtime evidence.
The live caller enables `--cargo-inheritance`: member package versions and
ordinary, development, build and target-specific dependencies must inherit their
root. The library and probes share that catalog and one lockfile.
The shared stable version reader validates the selected manifest with offline
Cargo before projecting TOML; version mutation remains consumer-owned. Parser
setup is therefore required before `make version` and release preflight too.

Before local validation, run `make update-dev` for complete development setup,
or explicitly prepare the complete pinned host bundle:

```text
make install-host-tools
make host-tools-check
```

The root [common Make include](../../../make/tools.mk) selects jq 1.8.2, Mike Farah yq
4.47.2, ripgrep 15.2.0 with PCRE2 and cloc 2.10 from the single
[host pin owner](../../../ci/tool-versions.env). The shared installer authenticates all
selected payloads before executing any, checks versions, and activates the bundle
together. Previous/failed candidates remain under ignored `.tools/`.
`actions-check` depends on this offline bundle check before reading declarations;
absent or changed tools require explicit setup. Existing parser-only bundles must
be explicitly refreshed with `make install-host-tools` once.

Make, `update-dev` and hosted CI use the same common setup/check targets and
checkout-local PATH. No separate system jq/yq/ripgrep/cloc installation is
required. The repository fixture still supplies an explicit local yq path.
PocketIC keeps its separate exact audited admission owner. Follow the
[bootstrap prerequisites](../../local-setup.md#bootstrap-prerequisites) for Linux and
macOS and prepare this repository's Rust toolchain before the common aggregate.
The reviewed installer itself requires no sudo.

At that revision, `make cloc` reported all four members of the root workspace, including the
unpublished probes. `CLOC_MANIFEST=Cargo.toml` explicitly selects that same graph;
there is no independent testing manifest. The refreshed reporter isolates its
fixture workspaces and excludes configured build output, including aliases.
Fleet tooling inventories now run centrally from Shared Tooling; this consumer
does not select the optional reporter or its dedicated regression suite. The
[current adoption owner](shared-tooling-history.md#shared-tooling-committed-0134-follow-up)
records that selection boundary. Counts do not establish instruction or Wasm
savings.

The [0.1.18 adoption owner](shared-tooling-history.md#shared-tooling-0118-refresh)
records source inspection and pending Linux/native macOS consumer qualification.
Current [Shared 0.3 adoption](shared-tooling-history.md#shared-tooling-030-complete-toolset)
includes the complete Cargo toolset in both aggregates. `update-dev` bootstraps
the pinned Rust toolchain, calls `install-tools` then `tools-check`, and installs
the formatting hook. CI prepares its declared toolchain before the same ordered
aggregate, exposing `.tools/rust/bin` alongside host and IC paths. There is no
separate global cargo-sort installation. The selected pin file already contains
cargo-sort 2.1.4, cargo-sort-derives 0.13.0 and candid-extractor 0.1.6.
No install, test/build/lint or formatter runs during contributor preparation.
Ordinary checks never download tools.

### Pinned IC tool setup

`make install-ic-tools` explicitly prepares Quill 0.5.4, ICP CLI 1.6.0, didc
0.6.2, ic-wasm 0.11.1 and wasm-opt 132 from
[`ci/ic-tools.tsv`](../../../ci/ic-tools.tsv). `make ic-tools-check` verifies this
five-tool bundle offline. `make install-tools` / `make tools-check` run host,
IC and Cargo tools, then `install-testkit-server` / `pocketic-check` through the
ordered local target lists. Standalone owner targets remain available, and the
ordered release preflight/validation rosters retain their established boundaries.
Shared Tooling 0.2.0 removes PocketIC from its IC policy. An existing
six-tool bundle fails admission until explicit setup selects a new bundle;
previous bundles, pins and receipts are retained.

`make install-testkit-server` invokes the [local adapter](https://github.com/dragginzgame/ic-timers/blob/651efce370bd6aec6290b74118eb709488062d0a/scripts/dev/testkit-server.sh),
which reads the single registry Testkit selection from the root lockfile. The
reviewed shared Cargo installer prepares only that package's `ic-testkit-server`
binary in release profile and checks its exact receipt/bytes. The CLI owns
server setup under `.tools/testkit-server`, asset hashes and compatibility.
`make pocketic-check` invokes both offline admissions and prints the admitted
absolute server path. No global CLI, local server catalog, client alignment rule
or Make binary override is retained. Failed admission never invokes setup.
Missing or invalid CLI admission retains the canonical installer's exact selection
diagnostic and failure status; the local adapter adds `run make install-testkit-server`.

Standard release preflight now calls `fetch`, `install-testkit-server`, then
`pocketic-check`, after admitting source/version/notes and before validation or
version mutation. Each call finishes before the next, including when the outer
Make is parallel. Existing saved-release reconciliation decides whether this
preflight is needed; no setup prerequisite bypasses it. Ordinary checks remain
offline and do not install. Explicit Cargo offline policy is inherited by setup.

Developer update and the Linux/MSRV/native CI preparation sites explicitly set
up the CLI/server after host prerequisites. `release-verify` runs locked fetch,
explicit Testkit setup, then offline admission before the complete existing
product gates, preserving direct user invocation without a prior preflight.
Valid selected installations are reused. Each watchdog/cohort command first
depends on offline `pocketic-check`, then rechecks after compiling its probes
and passes only the admitted path to the test harness. Product tests still own
fresh servers/instances and their existing startup deadlines; this handoff adds
no shared-server reuse or recovery guarantee.

Shared archive selection preserves failed CLI builds and Testkit `.setup-v1-*`
attempts without selecting successful server bundles. Scope and pending native
qualification belong in the [adoption record](shared-tooling-history.md#shared-tooling-020-hard-cut).
Ordinary validation never downloads. Setup neither deploys a canister nor selects
credentials or a network target.

### Testkit 0.27 preparation

After released IC Timers 0.16.0
`984c2f0a92f7e3ebde604f88895b12fb2b78cb2c`, incoming maintainer edits select
Testkit 0.27.0 in the sole root catalog/lock. Contributors preserved those edits.
The undated 0.16.1 notes prepare this repository-only update; production timer
source, public APIs and canister dependencies are unchanged.

The downloaded registry startup owner and CLI match Testkit's tagged 0.27.0
source at `f2d9fc6f197bcb9e669a7abaa1135f4cdffdd9ee`. Its startup error is a
record with `failure()`, bounded `output()` and independent command/server cleanup
reports. The private [harness](../../../crates/ic-timers-pocketic/tests/runtime/harness/mod.rs)
uses retained spawn/connect/build APIs and `expect`, without matching retired
error variants. Its error's `Debug` includes the original cause and cleanup/output
fields; adding downstream error reconstruction would duplicate the owner.
The CLI adapter already derives its version from the lock. No consumer rewrite,
old error alias or second CLI selection is needed.

Locked offline metadata on 2026-10-09 resolves one registry Testkit 0.27.0,
Metrics 0.3.1 and PocketIC 16.1.0, with all four local members still at 0.16.0.
Registry source comparison and metadata are retained at the directory referenced
by `/tmp/ic-timers-testkit027-current`; manifest and lock bytes are preserved.
These are preparation checks, not startup or compilation qualification.

[Released Testkit 0.27 CI](https://github.com/dragginzgame/ic-testkit/actions/runs/37923315911)
passes Linux portable-host, MSRV and PocketIC concurrency jobs; its Linux complete
checks remain in progress and macOS gates queued at inspection. The corresponding
[second run](https://github.com/dragginzgame/ic-testkit/actions/runs/37923315803)
also has pending complete/native checks. Keep adoption acceptance pending until
upstream native gates and the selected consumer graph's setup/offline check,
probe compilation, actual startup/recovery and policy cohorts pass on all three
hosts. The existing 0.16.0 runs use Testkit 0.26.0 and cannot qualify 0.27.0.
No duplicate hosted dispatch, new test/build/lint, tool setup or release ran in
this preparation. No production Wasm, instruction or heap delta is expected
from this host-only dependency update; none was measured.

The maintainer subsequently released **IC Timers 0.16.1** at
`6b508cc0ebcb215d04c2c54ce234d984cb771eda`, selecting Testkit 0.27.0, Host 0.9.2
and Metrics 0.3.2. Its [main Linux/MSRV](https://github.com/dragginzgame/ic-timers/actions/runs/37938451791)
and [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37938451752)
pass; both complete native macOS jobs remain queued. Setup and fixtures alone
do not establish actual Linux product startup or all-host #34 acceptance.

A later incoming lock selects **Testkit 0.27.1**. The owner's entire library/CLI
source is unchanged from 0.27.0; this patch repairs its own Make CLI build ordering
and heavy branch/tag CI duplication. Timers already builds the lock-selected
published CLI through the shared installer and needs no new adapter. All 40
downloaded Testkit source files match tagged 0.27.1; all eight Metrics 0.3.2
source files match its tag and are unchanged from 0.3.1. Locked offline metadata
resolves one identity for every selected owner, preserving Cargo bytes and all
four local package versions at 0.16.1. These are source/graph checks at
`/tmp/ic-timers-shared025.3v_vh3a0/`, not native execution qualification for this
incoming graph. Its full maintainer gate remains required.

A concurrent later lock update advances all four Host crates to **0.9.3**.
Their complete library source trees are unchanged from tagged 0.9.2. A fresh
locked offline metadata check resolves one of each at 0.9.3 and preserves this
new incoming lock's bytes; `metadata-host093.json` retains the later graph
separately from the earlier 0.9.2 inspection. No contributor dependency update
ran, and neither graph check establishes execution qualification.

### Linux product qualification for 0.16.3

Issue [#34](https://github.com/dragginzgame/ic-timers/issues/34) requires actual
consumer setup, offline admission and product startup/recovery/cohort execution
on Linux and both native macOS hosts. Released 0.16.2 at
`1e1255dc489dead90bbcbf7cd404deedf89300d0` passes
[Linux checks/MSRV](https://github.com/dragginzgame/ic-timers/actions/runs/37941328947),
but its Linux workflow stops after probe lint; setup and adapter fixtures cannot
establish product startup. Both matching macOS gates remain queued at inspection.

The pending 0.16.3 workflow adds one step in the existing Linux checks job:
`make pocketic-watchdog pocketic-cohorts MSRV="$IC_TIMERS_INTERNAL_TOOLCHAIN"`.
It follows CI and probe lint and precedes controlled late failure qualification.
The already-prepared internal toolchain supplies Cargo, rustfmt and the Wasm
target; no second Linux toolchain install or duplicate MSRV check is added.
The separate MSRV job retains its minimum-version checks. macOS keeps the
complete release gate, including its existing MSRV-selected probe execution.

Both product targets use the existing [Make owners](../../../Makefile): offline
Testkit admission precedes builds, admission is rechecked afterward, and only
the resulting server path reaches the probes. The private
[harness](../../../crates/ic-timers-pocketic/tests/runtime/harness/mod.rs) starts a fresh
managed server and IC instance for each subject. The watchdog target selects
the existing `tests::` subjects; the cohort target selects the existing size and
instruction subject with its measurement output. Normal step failure stops the
job and uses the existing failure collector/uploader. No new harness, alternate
server catalog, test fallback or dispatch is introduced.

This change adds native build/execution time to Linux CI; it has no production
Wasm, instruction or heap impact. During preparation, duration and hosted
execution remained unmeasured.
Source/workflow parsing, embedded shell syntax, document links and unchanged
locked graph checks are preparation evidence only, retained with the dirty
worktree diff at `/tmp/ic-timers-0163-issues/`. No contributor test/build,
lint, installation or release ran. Keep #34 open until the matching source's
Linux product step and both macOS complete gates supply setup/admission/startup
and recovery/cohort evidence. Earlier released runs retain their original scope.

Released **0.16.3** at `25957e206fbd351656870e9f87a23c47eed0c015` now passes
[Linux checks](https://github.com/dragginzgame/ic-timers/actions/runs/37948400741/job/113880561620),
MSRV (113880561635) and [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37948400870).
The retained Linux log proves Testkit setup/admission of its 16.1.0 Linux server,
adapter and staged-hook/Make fixtures, **14** actual product recovery/lifecycle
subjects and the policy-cohort subject with all four installed-Wasm/instruction
rows. The product step uses Rust **1.99.0**, `timer-probe` profile, Testkit **0.27.1**,
Host **0.9.3** and Metrics **0.3.2**. This completes Linux product acceptance for
that source; both macOS jobs (113880561216/113880561677) remain queued.
Raw logs and job identities are retained at `/tmp/ic-timers-0164-review/`.
These measurements must not be compared as a runtime optimization against
historical Rust 1.88 builds, or relabelled for the incoming Host 0.9.4 graph.
No contributor tests or duplicate workflow dispatch ran; only existing hosted
results were inspected. Keep #34/#35 open for native macOS acceptance.

### Host 0.9.4 graph preparation

After released IC Timers **0.16.3**
`25957e206fbd351656870e9f87a23c47eed0c015`, an incoming maintainer lock selects
all four Host crates at **0.9.4** instead of released **0.9.3**. Preserve that
lock. The sole root catalog still selects Testkit 0.27; no direct Host dependency
or alternate process owner is added. Testkit remains the private harness and
published CLI/server owner.

Host 0.9.4 at `4e3daebd5df07c6449279535668436024a45c02b` changes its repository's
Shared Make wiring. All **66** registry library source files across artifacts,
fs, process and tools are byte-identical to 0.9.3 and match tag `v0.9.4`. Its
[exact hosted run](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37947548868)
remains pending at inspection. Library source equality avoids an unnecessary
Timers adapter rewrite; it does not qualify the new package selections.

Full locked offline metadata resolves one of each Host crate at 0.9.4, Testkit
0.27.1, Metrics 0.3.2 and PocketIC 16.1.0, with all four local members still at
0.16.3. Cargo bytes are unchanged through inspection. Source comparison and graph
evidence are retained at `/tmp/ic-timers-0164-review/`. This describes the root
test graph; the published Testkit CLI's own locked build remains with that owner.

The undated 0.16.4 notes contain repository-only dependency maintenance. No
production Wasm/instruction/heap change is expected or measured. Prefer bundling
this with the next code-bearing release unless the maintainer explicitly selects
a repository-only patch. No contributor dependency update, test/build/lint,
installation, staging, commit or release ran. The new graph still requires the
normal native gate and must not inherit 0.16.3's source-bound CI evidence.

On the later 2026-10-09 upstream review, crates.io lists Testkit **0.27.2**,
Metrics **0.3.4** and all four Host crates **0.9.7**. Committed Testkit
`1a8f2ff570ea1c3bd58b215e28af52e5d99870e8` has no library/CLI source change from
released 0.27.1; Metrics `5a5f1dab1f7ee3e1e5624c9d889148c39abf45f2` has no
crate source change from released 0.3.2. Host
`ca62e661918db2f4320743b9042a4993a5fff2aa` changes only
`crates/ic-host-tools/src/response/mod.rs` relative to incoming 0.9.4: it folds
empty-text validation into the existing hex digit scan, retaining typed errors
and JSON empty-response acceptance. Our harness uses Testkit's PocketIC startup,
not this response decoder. Testkit already re-exports the Host families for
actual host consumers; adding direct dependencies would create no benefit here.
The custom timer memory-page summary retains paired extents and growth maximums,
not the cumulative sample sum owned by Metrics. Keep those distinct semantics.
No new adapter, timer feature or production code deletion follows from this
review, and dependency selection/qualification remains user-owned.

The incoming lock at the start of this adoption already selects those latest
Testkit/Metric/Host releases. Preserve it without a contributor Cargo update.
Full locked offline metadata resolves one Testkit 0.27.2, Metrics 0.3.4, each
Host crate 0.9.7 and PocketIC 16.1.0, with four local members still at 0.16.3.
Both the saved incoming lock and metadata are retained under
`/tmp/ic-timers-shared028.zqdya2y6/`; they supersede the earlier graph preparation,
not the released execution evidence. Source equivalence does not qualify the new
package identities or their published CLI build. The complete user-owned gate
remains required.

### Testkit 0.28 and Host 0.10 released graph

The maintainer's pushed **0.16.4** at
`da921fc899d2c7c99ed0a6cacac6e2111307ac71` actually selects Testkit **0.28.0**,
all four Host crates **0.10.1**, Metrics **0.3.5** and PocketIC **16.1.0**.
Four root-owned members remain 0.16.4. This supersedes earlier preparation graph
selection, including the versions described by the finalized development prose;
it does not relabel any earlier execution evidence. Registry publication is not
independently checked. The sole root catalog owns
Testkit 0.28 and Metrics 0.3; members retain workspace inheritance.

Review Testkit commit `48cfff270d5e228c0e09d6ee452786fb14c9a66e` and Host
`c7bdc3d4e1c658957202eebd76bff2c51e22f645`: Host consolidates durable writes on
`write_with(path, options, producer)` and preserves publication/cleanup failure
state. Testkit updates those calls and preserves error evidence in cache and
server provisioning paths. Host 0.10.1 also synchronizes newly observed parent
directories when a competing creator wins mkdir. Our private harness consumes
PocketIC managed startup, never the changed durable-write APIs or cache error
matches. Therefore no timer adapter, direct Host dependency, compatibility shim,
public timer cut or retained-data reset follows. The published CLI is still
installed/check-admitted by its selected Testkit version; that owner CLI's locked
build remains distinct from the root test graph. Metrics 0.3.5 changes inspector
fixtures/docs, not measurement library arithmetic.

Full locked offline metadata is retained at `/tmp/ic-timers-0165-metadata.json`.
The current root Cargo/pin bytes are preserved through Shared 0.2.9 export under
`/tmp/ic-timers-shared029.00umv319/`.

[Exact Linux job 113946212101](https://github.com/dragginzgame/ic-timers/actions/runs/37967730569/job/113946212101)
has now passed with this pushed source/graph. The actual log proves Testkit server
setup/offline admission, standard release adapters/gate, corrected version
preparation, actual staged-hook formatting/failure isolation, 142 native library
tests, 14 maintained PocketIC recovery subjects and all four policy cohorts.
Product/cohort commands use internal Rust **1.99.0** and the root `timer-probe`
profile; the separate MSRV job passes at **1.88.0**. Raw evidence remains at
`/tmp/ic-timers-0165-review.5lms07fg/0164-linux-job.log`. Tag truth also passes;
both native macOS jobs remain queued at inspection. No contributor test/build/
lint/setup/Cargo update/release runs for this evidence review. These results
qualify the released Linux graph, not native macOS behavior or the pending
Shared 0.2.9 consumer selection. Keep #34/#35 open for their remaining acceptance.

The four retained 0.16.4 cohort rows exactly match the earlier 0.16.3 Linux
observations at the same internal Rust 1.99.0 and profile: Wasm sizes are
241280 / 286066 / 286835 / 288401 bytes for baseline / once / after-completion /
watchdog, and all reported instruction/cycle fields are unchanged. This is a
comparison of existing hosted observations, not a new benchmark execution or a
claim about unmeasured workloads. The older raw log remains at
`/tmp/ic-timers-0164-review/linux-checks.log`; current measurements remain tied to
the newer graph above. No optimization or footprint regression is evidenced in
these maintained subjects.

Both native jobs subsequently complete successfully at the exact 0.16.4 source:
[Intel 113946212038](https://github.com/dragginzgame/ic-timers/actions/runs/37967730569/job/113946212038)
and [ARM 113946212124](https://github.com/dragginzgame/ic-timers/actions/runs/37967730569/job/113946212124).
Their raw logs prove host-specific Testkit PocketIC 16.1.0 provisioning/check
admission, the complete native release gate, corrected version preparation,
actual staged-hook formatting/isolation, 142 library tests, 14 recovery subjects
and all four cohorts. Logs are retained at `/tmp/ic-timers-0164-native.cutpdaam/`.
Native canister commands explicitly use **Rust 1.88.0** despite the internal host
toolchain being 1.99.0. Linux canister subjects use 1.99.0. Different native and
Linux cohort rows therefore are not an equal-input performance comparison.
No contributor tests, builds or deployment checks ran to obtain these results.
[Issue #34 closes with completed three-host acceptance](https://github.com/dragginzgame/ic-timers/issues/34#issuecomment-6095003127).
#35's original Make/hook acceptance also completes; its new uncommitted upstream
concise-formatting follow-up remains open. These results qualify the tagged graph
and its Shared 0.2.8 consumer, not the pending 0.2.9 selection.

An incoming lock on 2026-10-10 updates cc, smallvec and syn to **1.7.0 / 1.16.3 /
3.0.7**. Preserve this maintainer input. Cheap locked offline metadata at
`/tmp/ic-timers-0165-incoming-metadata.json` retains one Testkit/Metrics/PocketIC
selection and four local 0.16.4 members; it does not execute or qualify the new
transitive graph. Source-bound release evidence above remains tied to the old
lock. No contributor Cargo mutation, version change or validation rerun occurs.

### 0.16.5 Linux and tag acceptance

Released **0.16.5** `0d7b85ec6658f91421bd13c44fa19592d8cc029e` selects
Testkit **0.28.0**, Host **0.10.1**, Metrics **0.3.6**, PocketIC **16.1.0** and
four local 0.16.5 members, with all three snapshots at committed Shared **0.2.11**
`83efac446348dea024798a331d77933b24b429dc` (55/30/11 files).
[Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38036823188)
passes Linux checks **114168872952**, MSRV **114168873087**, ARM
**114168873084** and Intel **114168873187**.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38036823190)
**114168872998** passes at the same source.

Actual raw logs `/tmp/ic-timers-0165-linux-job.log` and
`/tmp/ic-timers-0165-tag-job.log` prove the repaired hook output assertion,
sorter failure isolation, complete retained-log archive case, cleared/replaced
Make-mode refusal and release/version-preparation fixtures. Linux additionally
runs 142 library tests, 14 maintained PocketIC recovery subjects and all four
policy cohorts under its explicit internal Rust **1.99.0** canister selection.
This qualifies the released Linux graph, including the Metrics/cc/smallvec/syn
refresh; it does not alone prove native consumer acceptance. Shared's exact
0.2.11 upstream Linux/native acceptance is complete
and its #30/#91/#92 are closed, separately from consumer qualification.

ARM's raw `/tmp/ic-timers-0165-arm-job.log` additionally proves actual hook
formatting/failure isolation, release/version preparation, 142 library tests,
14 recovery subjects and all four policy cohorts. Its canister builds explicitly
select Rust **1.88.0**, while the host test toolchain is 1.99.0. This qualifies
the released ARM graph; do not compare its cohort values against Linux's
different canister toolchain as an optimization measurement.

Intel's `/tmp/ic-timers-0165-intel-job.log` now proves the same actual hook,
release/version, 142 library-test, 14 recovery-subject and four-cohort acceptance
at this exact source, with explicit Rust 1.88.0 canister builds. This completes
execution of the released formatting/Make adoption across all three hosts.
#35 retains the new explicit working/index lock-preservation proof; the 0.16.6
lock/path/preflight cases and graph remain outside that released evidence.

Pending 0.16.6 adds explicit index/working lock-preservation assertions to the
consumer fixture and updates formatter prerequisites. Those new assertions are
unexecuted; the released 0.16.5 evidence does not qualify them. No contributor
build/test/lint/setup or release runs; observations above inspect existing jobs.

### 0.17.1 pushed source and pending graph

Pushed release source **0.17.1** is
`2a8710834c9283ccc46daa4e4c93b1cc4629c6bc`. Its committed root lock selects Metrics
**0.5.0**, Testkit **0.31.0**, all four Host **0.11.0** packages and PocketIC
**16.1.0**, with four local 0.17.1 packages and Shared **0.3.1** snapshots
`fa452afaa5012866eb1c20820dfa8038c106e7ec` (55/30/11).
[Matching main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38051205368)
passes Linux checks **114210531711** and MSRV **114210531705**; ARM
**114210531587** and Intel **114210531746** remain queued at observation.
[The tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38051205322)
is still running at the initial observation. These are job-state observations,
not downloaded fixture proof or complete host qualification.

The incoming maintainer-owned root lock now selects Metrics **0.5.1** under the
existing compatible `0.5` requirement. Inspection preserves those exact bytes,
all manifest/pin inputs and the real index. Full `cargo metadata --locked
--offline` stops because that archive is uncached; retained stderr at
`/tmp/ic-timers-shared032.rXciNI/metadata.err` records the failed attempt. There is
no contributor fetch, dependency mutation, rollback to 0.5.0 or claim that the
new full graph resolved. The normal maintainer-owned fetch/setup/release gate
retains its existing roles. Pending Shared 0.3.2 work is repository-only; no timer
source, production dependency requirement or measured optimization is added.

### 0.17.2 preparation inputs

During the subsequent Shared 0.3.3 inspection, the incoming maintainer-owned root
catalog selects Testkit `0.32` and its lock resolves **0.32.0**, all four Host
packages at **0.12.2**, Metrics **0.5.1** and PocketIC **16.1.0**. All four local
members remain **0.17.1**. Full `cargo metadata --locked --offline` now succeeds;
its graph and preserved inputs are retained at
`/tmp/ic-timers-shared033.0vrxoaq1/`. This supersedes the earlier cache blockage
and Testkit preparation observation for the current worktree, without changing
the released 0.17.1 graph or its evidence. No contributor fetch, catalog/lock
mutation, setup or qualification runs. Metadata resolution alone establishes
neither selected CLI/server admission nor native or PocketIC acceptance; the
normal user-operated gate must qualify this selected graph and new snapshots.

### Host 0.12.5 review

Committed Host **0.12.5**
`85f051c60b2a6f37717c4274b1e31caf5b9d3453` matches remote main. The incoming
maintainer-owned root lock now resolves all four Host packages at **0.12.5**
through Testkit **0.33.0**, with Metrics **0.5.4** and local packages still 0.17.4.
Locked offline metadata passes and all 66 selected Host source files match that
committed revision. Catalog, lock, pins and index are preserved; inspection
evidence is at `/tmp/ic-timers-host0125-review.mr_aqtps/`.

The only Rust implementation change from 0.12.4 is early NUL pathname rejection
in durable publication, before parent creation, staging or producer callbacks
([Host #54](https://github.com/dragginzgame/ic-host-tooling/issues/54)). Host's
process/tools/artifact Rust sources and public signatures are unchanged. Its
release/publication guard repairs remain Host-owned. There is no timer-runtime
adoption or measured canister Wasm/instruction benefit.

Distinguish the workspace test graph from the installed owner CLI. The selected
Cargo installer uses `cargo install --locked`, so published Testkit 0.33.0's CLI
build uses its own shipped lock, which selects Host **0.12.4**. Testkit's pending
0.33.1 lock already selects Host 0.12.5, but those dirty bytes are not publication
or qualification. The CLI's download evidence, executable publication and failure
records use Host durable writes; qualify a published Testkit update through the
existing selected-CLI/server route rather than overriding that build graph.
Timers' probes retain managed startup through Testkit and no direct Host import.
[Exact-source Host CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38061390080)
is queued at observation. No contributor tests, builds, setup, dependency update
or sibling mutation run; native acceptance remains source-bound and user-owned.

### 0.17.6 release acceptance

Released source is **0.17.6** `3174935874796c3ffae30b0dfcf9c0c8f4557322`.
The committed root graph selects four local 0.17.6 packages, Metrics **0.5.6**,
Testkit **0.33.1**, all four Host packages at **0.12.9**, and PocketIC **16.1.0**.
The installed Testkit CLI retains its separately packaged locked build graph;
workspace Host selection does not override that executable. All three Shared
snapshots select reviewed **0.3.8** `67285b28a98b7c4211ad32de726709d4e87edea4`
with **56/30/11** files.

[Exact-source main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38073888597)
completes successfully, including Linux checks
[114276787241](https://github.com/dragginzgame/ic-timers/actions/runs/38073888597/job/114276787241),
MSRV [114276787355](https://github.com/dragginzgame/ic-timers/actions/runs/38073888597/job/114276787355),
Intel [114276787367](https://github.com/dragginzgame/ic-timers/actions/runs/38073888597/job/114276787367)
and ARM [114276787456](https://github.com/dragginzgame/ic-timers/actions/runs/38073888597/job/114276787456).
[Matching tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38073888232/job/114276723092)
also passes at the same SHA. Run/job metadata, all five raw logs, their SHA-256
identities and matched line references are retained at
`/tmp/ic-timers-0176-acceptance.EkYVSQ/` (`inspection.json` owns the readback).

Linux and both macOS logs show completed local fixture-boundary and mandatory
assertion checks, collector exit/retention and full archive cases, real-index
release admission, version-preparation rejection/rollback, ordered Testkit
admission and actual shared-hook failure-isolation cases. The macOS host step
explicitly admits Darwin 15, the selected architecture and Apple's Bash 3.2
before adding `/bin` to the execution path. Both native release gates end with
`VALIDATION PASSED`; all three hosts execute 142 library tests, 14 maintained
PocketIC recovery subjects and all four policy cohorts. This establishes
consumer acceptance for the committed repair and graph, not just producer
acceptance or availability of a release.

This evidence closes [#38](https://github.com/dragginzgame/ic-timers/issues/38),
[#39](https://github.com/dragginzgame/ic-timers/issues/39),
[#40](https://github.com/dragginzgame/ic-timers/issues/40) and
[#41](https://github.com/dragginzgame/ic-timers/issues/41). Interrupted older runs
are not counted. Intentional early/late live-workflow artifact qualification
steps are skipped in this ordinary run; no fresh manual failure dispatch or
artifact round trip is claimed. The contributor downloads and inspects existing
completed logs only, without rerunning tests, setup, CI or release commands.
This acceptance-record update does not warrant another release or changelog
entry; keep it untagged for the next code-bearing batch.

### 0.17.13 delivery and remaining issue acceptance

Released **0.17.13** is `e6ac645aac1b185bfaa3b146c2b8e1dd38b3299f`, selecting
Shared **0.6.3** `4fd979d13c32adb3cb2f51f213ebfc4f98339b3d`, Metrics **0.6.4**,
Testkit **0.33.8**, all four Host **0.13.5** and PocketIC **16.1.0**.
The maintainer's local release journal records `complete` for source
`560c9a55f0679380fa6933222acfac564da9ebec`. The exact release commit has that sole
parent; native runtime test bytes are identical, and local `v0.17.13` resolves to
the release commit. This records delivery through the normal release contract,
not an independently downloaded test-execution log or native macOS acceptance.
Read-only journal/source comparisons are retained at
`/tmp/ic-timers-01713-issues.3zinjt3e/`.

[Full CI](https://github.com/dragginzgame/ic-timers/actions/runs/38149582052)
passes Linux/MSRV with both native jobs queued at inspection.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38149582074)
passes separately and cannot substitute for those gates.
[#46](https://github.com/dragginzgame/ic-timers/issues/46)'s six-case native fixture
cleanup correction and its formatting repair are delivered. Downloaded Linux job
**114500137818** actually executes
`runtime::tests::fixture_cleanup_drops_queued_and_suspended_work_before_tls_teardown`
and passes 145 library tests, one doctest, seven frontier checks, 14 PocketIC
recovery subjects and four policy cohorts. The raw log is retained at
`/tmp/ic-timers-01713-qualification.n07_3uur/released-linux.log`, SHA-256
`3bfee2dcd411dc9b3e346982cba8033343a4d806f703c1c149caddc5ba1d2ce8`.
This supplies #46's source-bound execution evidence; complete native and artifact
acceptance remain separately owned by #44.
[#44](https://github.com/dragginzgame/ic-timers/issues/44) retains complete native
and controlled early/late artifact qualification. No additional runtime repair
is justified by these two issues.

For #44, the maintainer can select frozen `v0.17.13` in the existing CI manual
dispatch, first `failure_stage=early`, then `late` **after the first completes**.
Use the [existing qualification procedure](#ci-failure-evidence) and
record both source-bound run links and all six host/stage verification results.
The original jobs must fail at the selected controlled boundary and all three
download-verification jobs must pass per stage. Queued jobs, setup failures or
ordinary successful CI do not satisfy this artifact boundary. No workflow is
dispatched or validation run by the initial issue-maintenance pass. These
acceptance records warrant no new changelog draft or package release.

The maintainer subsequently authorized both qualification stages at frozen
`v0.17.13`, sequentially after tag CI finished. Tag CI passes. The authorized
[early run](https://github.com/dragginzgame/ic-timers/actions/runs/38150377457)
selects the exact release SHA. Its Linux producer reaches controlled exit 22,
archives and uploads successfully; both native producers remain queued. Late qualification must
wait for this run to complete, preserving its original failing producer results
and verification observations. Dispatch identity, source tag and observations
are retained at `/tmp/ic-timers-01713-qualification.n07_3uur/`. Neither dispatch
acceptance nor queued jobs count as successful artifact qualification.

### Early verifier shared-store path correction

The authorized early run exposed a local verifier defect under
[#44](https://github.com/dragginzgame/ic-timers/issues/44). Linux artifact
**11706432177**, uploaded ZIP SHA-256
`c673b3502a05f8207002274a3f561ff4ae32d2fd69f2e9b18f411e8afcde83df`, contains the
correct rejected download bytes under the scenario's physical shared host
generation. The frozen 0.17.13 verifier instead requires an obsolete physical
checkout slot and rejects the valid archive. Its synthetic rejection fixture
still constructs that old layout; the collector fixture resolves and compares
physical bytes but had not passed its actual archive to the download verifier.
The collector and canonical Shared installer require no patch.

Pending compatible **0.17.14** replaces the old candidate path match with the
current `tool-store/host-<64-hex>/set.<alphanumeric>/bin/jq` shape. It keeps exact
source/job/host/run/attempt, input mode/bytes, controlled status and one regular
candidate admission. Updated negative cases reject obsolete/malformed paths,
multiple physical candidates and linked payloads. The existing Ubuntu-only
download-verifier suite now runs the actual early driver and collector in an
isolated checkout/store, then verifies their archive with matching source
identity. This adds no Python prerequisite to native macOS or the Make release
gate. No legacy reader, link traversal or recovery bypass is added.

The corrected local verifier admits the exact downloaded Linux artifact that
the frozen verifier rejects; both outputs and the original ZIP are retained at
`/tmp/ic-timers-01713-qualification.n07_3uur/`. Python/Bash syntax and diff checks
pass. The new synthetic/producer fixtures are prepared, not locally executed;
the explicit authorization covers the two frozen hosted stages and their artifact
inspection. The fixed dirty verifier is not the hosted verifier of 0.17.13 and
does not make that early run accepted. Native producers/hosted verification and
late qualification retain their own pending observations. Qualify the delivered
correction at its frozen source through the existing maintainer workflow before
closing #44. Crate APIs, production behavior, dependencies and Wasm are unchanged.

### 0.17.14 delivery and frozen failure qualification follow-up

Released **0.17.14** is `cb86d87ce67f7aac1b127bdfaa2c6f8a7f68c2a0`.
It delivers the verifier correction above and selects Testkit **0.34.0**, all
four Host **0.14.1**, Metrics **0.6.4** and PocketIC **16.1.0**, with the same
Shared **0.6.3** revision and canonical 56/33/11 exports. Its
[full CI](https://github.com/dragginzgame/ic-timers/actions/runs/38153029865)
passes Linux/MSRV; Intel is in progress and ARM queued at inspection. The
previous 0.17.13 full run was cancelled by the newer main push. Do not infer
missing native acceptance from the preserved earlier Linux pass.

The authorized frozen 0.17.13
[early run](https://github.com/dragginzgame/ic-timers/actions/runs/38150377457)
is now complete. All three original producers reach controlled exit 22 and
successfully upload. All three hosted download verifiers fail the already-fixed
obsolete candidate-path check. Exact downloaded ZIP identities and digests are:

| Host | Artifact | Uploaded/downloaded ZIP SHA-256 |
| --- | --- | --- |
| Linux/X64 | 11706432177 | `c673b3502a05f8207002274a3f561ff4ae32d2fd69f2e9b18f411e8afcde83df` |
| macOS/ARM64 | 11706454805 | `4c150a78e1662de6c229c374380376208c8544f544fea17e437620658fc431be` |
| macOS/X64 | 11708025680 | `fa2caec47a90d6291ae0163e84ef209fde193129ac2d91a7d48ead3fc82d1dcb` |

The byte-identical released 0.17.14 verifier admits all three actual tar payloads
locally, with each original source/job/host/run/attempt and unchanged frozen
caller pin bytes. ZIP inspection retains only the named tar/collector entries;
archive verification reads members without extraction. Raw logs, archives,
replay outputs and digest/source checks remain at
`/tmp/ic-timers-01713-qualification.n07_3uur/`. This qualifies the artifact replay,
not the frozen hosted verifier. Preserve the original failed observations.

After early completed, the already-approved late stage was dispatched at the
same annotated `v0.17.13`, dereferenced to the exact original source. Its
[late run](https://github.com/dragginzgame/ic-timers/actions/runs/38154578484)
has Linux in progress and both macOS producers queued. Late acceptance remains
pending. A separate approval request selects corrected frozen `v0.17.14` for
new sequential early/late stages only after this late run finishes; no such
approval or new-source dispatch is inferred from the original authorization.
[#44](https://github.com/dragginzgame/ic-timers/issues/44) stays open for complete
native and six corrected-source hosted artifact observations. This acceptance
record introduces no additional tooling/runtime patch or package release.

A separate worktree lock update from `syn` 3.0.7 to 3.0.8 appeared during the
0.18 preparation and is preserved. It is not part of the released 0.17.14 graph
or the frozen qualifier inputs; no old CI observation qualifies that new graph.

### 0.17.12 source and native cleanup follow-up

Released 0.17.12 is `549d1f5e9e69ec2d58d37c72760ffd93b88b175f`, with Shared
0.6.2 `7f1ca16b0dfa98021d2671017ecb7de767409b0e`, Metrics 0.6.4,
Testkit 0.33.7, all four Host packages at 0.13.5 and PocketIC 16.1.0.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38144421659)
passes. [Full CI](https://github.com/dragginzgame/ic-timers/actions/runs/38144421665)
has Linux/MSRV/ARM passing and Intel running at inspection.
Downloaded Linux job **114484683406** actually executes
`platform::fake::tests::removed_futures_can_schedule_from_their_destructors`
and passes all 145 library tests, doctest/compiler-frontier checks, 14 PocketIC
recovery subjects and four policy cohorts. Its raw log is retained at
`/tmp/ic-timers-01713.mgzz1bha/released-linux.log`, SHA-256
`bc04c2a390e19cd98b3f99c893dd704cf8df49547f4d51bf370a45de2c25f042`.
The previous 0.17.11 Linux/MSRV jobs passed but both native jobs were cancelled;
those different inputs cannot establish current source acceptance.
[#45](https://github.com/dragginzgame/ic-timers/issues/45) has source-bound Linux
execution of its released removal regression. Keep
[#44](https://github.com/dragginzgame/ic-timers/issues/44) open for complete native
and controlled artifact acceptance.

[Shared 0.6.2 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38143010876)
passes Linux portable, lint/security and ARM portable; Intel is still queued.
That source describes the released graph. Pending 0.17.13 now adopts
[Shared 0.6.3](shared-tooling-history.md#shared-tooling-063-committed-preparation-restart);
qualification is not inferred from moving sibling status or publication alone.

Incoming maintainer Cargo.lock selects Testkit **0.33.8**, with Host **0.13.5**,
Metrics **0.6.4**, PocketIC **16.1.0** and all four local packages at **0.17.12**.
Cheap locked offline metadata resolves one ID per selected package. Testkit's
released source `f17892332f4b100ffea4b28528927bfb0b4e1c5c` has no product Rust
change from 0.33.7. Its selected registry package's shipped Cargo.lock now selects
all four Host packages at 0.13.5; the prior distinction between root and packaged
CLI graphs remains part of setup admission. No new direct Host dependency,
server policy or alternate setup path is introduced. Published Host 0.14.0
changes completed-process and observer APIs, while this Testkit release still
requires Host 0.13. Adoption waits for the Testkit owner's
[#61](https://github.com/dragginzgame/ic-testkit/issues/61) minor release; do not
force the transitive graph with a direct dependency or local patch.

Pending 0.17.13 repairs only native fixture ownership under
[#46](https://github.com/dragginzgame/ic-timers/issues/46). After clearing queued
futures while TLS is live, fixture cleanup detaches the remaining registry and
provides an empty initialized native runtime before dropping callback captures
outside the borrow. Both declaration lifetimes now exercise inactive, queued and
suspended captures. This teardown is test-only and proves no IC persistence,
upgrade or trap recovery behavior. Production implementation and Wasm are unchanged.

Prepared source, incoming lock/index identities, metadata and review outputs are
retained at `/tmp/ic-timers-01713.mgzz1bha/`. Incoming catalog/lock/pins/index are
preserved byte-for-byte. Diff, links and cheap metadata are the preparation checks;
no native regression, test/build/lint, setup, workflow dispatch or release is run.
The additional Shared 0.6.3 refresh preserves these inputs and prepared native
changes at `/tmp/ic-timers-shared063.dfz0hesa/` and updates release guidance to the
canonical committed-preparation restart contract.

### Shared 0.6.1 correction and Host 0.13.4 selection

The pending 0.17.11 contributor-tooling patch adopts Shared 0.6.1
`731909525038c58d6f80b498e80f8630b11c685d` through 56/33/11 canonical exports.
[The adoption owner](shared-tooling-history.md#shared-tooling-061-failure-and-native-fixture-corrections)
records the reviewed source, preserved inputs, failure semantics and required
setup. No local installer patch or reduced validation gate is introduced.
Source-bound consumer native qualification remains under
[#44](https://github.com/dragginzgame/ic-timers/issues/44).

The incoming maintainer lock already selects Metrics **0.6.4**, Testkit **0.33.6**,
all four Host packages **0.13.4**, PocketIC **16.1.0** and four local members at
**0.17.10**. Locked offline metadata resolves exactly one ID per selected package.
The unchanged root catalog owns all direct dependencies; Host remains transitive
through Testkit and absent from the timer library's normal dependency closure.
Requested precise Host updates require no version changes; unrelated Cargo
edge-normalization edits are discarded to preserve the incoming lock bytes.

Host's annotated `v0.13.4` tag resolves to committed source
`46483b96c21419aa41ee2cbcdfa2e8efbbfdab21`. Its new
`durable::open_private_regular_lock_file` admits a no-follow, nonblocking regular
sidecar in an existing parent, with owner-only creation and caller-owned locking.
Timers has no product caller for this filesystem API; no direct dependency,
sidecar adapter or parallel lock owner is added. Existing opener behavior is
preserved by the reviewed common descriptor path. Source review and metadata
are not execution qualification; [Host CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38139845295)
is still running. The separately installed Testkit CLI uses its packaged lock,
so root Host selection alone does not upgrade that executable's Host dependencies.

The completed released 0.17.10 ARM job **114465681030** also fails at the canonical
Cargo fixture immediately after common setup, matching Intel's boundary. Its raw
log is retained in `/tmp/ic-timers-shared061.ynyutnkh/timers-01710-arm.log`.
This does not qualify the new 0.6.1 payload or incoming graph. No local behavioral
checks or release effects run; snapshot integrity, Bash/stub syntax, documentation,
diff and cheap locked offline metadata are the preparation checks.

### Shared 0.6.0 Cargo storage adoption

Released **0.17.10** adopts committed Shared **0.6.0**
`75180980a0c4e4252d8a915aaecfaefd92bf9a4e` through **56/33/11** exports.
[The adoption owner](shared-tooling-history.md#shared-tooling-060-cargo-storage-adoption)
records exact source, required companion, preserved inputs and previews. Cargo
storage now follows the same per-user owner as host/IC tools, through exact source,
packaged-lock, compiler and native-environment identities. Common command PATH
and the Testkit adapter's returned executable remain the only local selections;
old local Cargo slots are retained evidence, not reusable installation authority.
Run `make install-tools` then `make tools-check` with the same store setting.
Source cache preparation is explicit; offline checks must not provision tools.

The shared selector now supplies local Rust evidence plus referenced physical
Rust generations. The collector selects that tree only through successful owner
output, avoiding overlapping archive paths, while keeping its independent safe
local selection on failure. A new local case constructs a failed generation and
reference through the canonical store helper, and compares extracted logs/build
bytes, modes and links. Rejected-store cases exclude shared Rust payloads as well
as host/IC payloads. The shared Rust fixture remains selected by `release-check`
and owns compiler/package/native changes, two-checkout reuse and failure retention;
no alternate builder or store manager is introduced.

The [local rejection repair](#shared-store-rejection-and-local-failure-retention)
is delivered in the same release. Upstream also repairs its action, including archive
upload before incomplete collection fails. That owner behavior does not replace
Timers' existing checkout/event identity and manual artifact verifier. The normal
complete CI/MSRV/probe/recovery/cohort gate and Linux/Intel/ARM qualification remain
required under [#44](https://github.com/dragginzgame/ic-timers/issues/44).
No tests/builds/lint/setup or release execution runs during preparation. This
repository-only adoption leaves the library API, dependencies and consumer build
requirements unchanged; no measured product Wasm/instruction saving is claimed.

### 0.17.10 release qualification

Released source is `c4ce6480a2cbc656b35bde832d70c21befcf56db` with Shared 0.6.0
`75180980a0c4e4252d8a915aaecfaefd92bf9a4e`, Metrics 0.6.2, Testkit 0.33.5, all
four Host packages at 0.13.1 and PocketIC 16.1.0. In
[main run 38137975076](https://github.com/dragginzgame/ic-timers/actions/runs/38137975076),
Linux job **114465680937** and MSRV job **114465680861** pass. The downloaded
Linux log actually runs the canonical shared Cargo fixture, local failure-evidence
cases, 144 library tests, 14 PocketIC recovery subjects and four policy cohorts.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38137975652) passes
at the same SHA. These results qualify this released Linux graph, not incoming
lock changes or complete native adoption.

Intel macOS job **114465681015** fails in the shared Cargo fixture immediately
after its first common setup. The downloaded
[artifact 11701608183](https://github.com/dragginzgame/ic-timers/actions/runs/38137975076/artifacts/11701608183)
matches uploaded ZIP SHA-256
`c077dd2d2ffd963adfe190aad28eeaf883545e33d3e51c3972a0602effd54239`.
Its identity records the exact source/event SHA, macOS/X64, job/run/attempt.
Retained `ic-timers-fixtures/rust-tools-test.LAjs0s/builds` has exactly three
completed common builds, corroborating the next statement's padded-count failure
reported in [Shared #124](https://github.com/dragginzgame/shared-tooling/issues/124#issuecomment-6109107469).
The original failure is preserved. ARM macOS remains running at inspection;
complete native acceptance under [#44](https://github.com/dragginzgame/ic-timers/issues/44#issuecomment-6109107659)
remains open. This ordinary-failure artifact round trip does not substitute for
the explicit controlled early/late qualification scenario.

Logs, job metadata, downloaded ZIP, selected archive bytes and digest records are
retained at `/tmp/ic-timers-01710-qualification.y69x5ito/`. Member inspection reads
selected regular files without extracting or following links. Shared's exact-source
0.6.0 run also passes Linux/lint and fails both macOS portable jobs. Its pending
0.6.1 working tree addresses #124 and the common Cargo setup failure/lock propagation
tracked in [Shared #120](https://github.com/dragginzgame/shared-tooling/issues/120#issuecomment-6108988869).
Remote main remains `7518098`; adopt the reviewed committed owner correction before
claiming repair, without patching snapshot bytes or weakening native gates.

The incoming maintainer lock selects Metrics 0.6.3, Testkit 0.33.6 and all four
Host packages at 0.13.3, with PocketIC still 16.1.0. Locked offline metadata
resolves one package ID for each and leaves that lock byte-for-byte unchanged.
These selections remain separate from released-source qualification. No tests,
builds, lint, setup, workflow dispatch or release execution runs during this
inspection; downloaded execution evidence and metadata reads are the only checks.
This acceptance/feedback record creates no new changelog or release subject.

### Shared 0.5.0 storage adoption

Released **0.17.9** adopts committed Shared **0.5.0**
`a306180ce534227fcb98b882fba9a526d5d747e0` through the **56/32/11** canonical
exports. The supplemental export adds the required store helper and fixture;
[the adoption record](shared-tooling-history.md#shared-tooling-050-storage-adoption)
owns previews, source and preserved inputs. This replaces the interrupted 0.4.2
policy-only adoption. No local store manager or executable relocation is added.

Run `make install-tools` then `make tools-check`. Host/IC payloads now use the
[canonical per-user store](../../local-setup.md#shared-executable-storage), with local
`.tools/host/bin` / `.tools/ic/bin` command selections unchanged. If setting
`SHARED_TOOLING_STORE`, use the same absolute location for setup, offline checks
and failure collection. Older local host/IC bundles remain retained but fail
active admission until explicit setup selects a shared bundle. Different pins
and platforms select independent bundles; updating one checkout cannot retarget
another. Cargo tool slots stay local, with `.tools/rust/common/bin` on PATH;
compiled-tool sharing remains outside this release. Testkit keeps its own server
and packaged-lock admission. Consumer Binaryen 132 and all other pins are retained.

Failure collection delegates shared bundle selection/admission to the refreshed
owner, retaining only referenced physical bundles rather than scanning the store.
The local fixture uses canonical installers with isolated authenticated fake
archives and checks compact provenance, corrupted shared payload/library bytes,
reference retention and exclusion of unrelated store data. Intentional early
failure uses a separate scenario store, so cached successful setup cannot bypass
the injected failure; that physical scratch and its logs are archived together.
Copied adapters include the new helper and use supported space/alias paths.
Existing formatting and full Rust-tree retention remain part of the same batch.

[Timers #44](https://github.com/dragginzgame/ic-timers/issues/44) remains open for
actual setup/offline reuse, selected two-checkout/partial-failure/corruption
fixtures, local retention cases, formatting-hook checks and complete Linux/native
Intel/ARM consumer acceptance with the incoming Metrics 0.6.2 / Testkit 0.33.5 / Host 0.13.1 graph.
Producer CI, mocks and previous 0.17.8 acceptance cannot replace those gates.
A live early/late artifact round trip retains its explicit manual-dispatch scope.
No tests/builds/lint/setup or release runs during preparation. Artifact deletion,
Cargo/version mutation, staging, commits and release execution remain user-owned.

The [package-impact rules](../../../rules/changelogs.md#classify-the-released-packages-own-impact)
classify this as contributor tooling, compatible for the IC Timers library.
The adoption was delivered in 0.17.9; no consumer API/runtime/build requirement
changes or measured Wasm/instruction saving are claimed. Keep the normal complete
release gate rather than weakening it for a tooling-only subject.

Released source is `46785c3830786ccb17da1987cc9519c046b900d9`.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38134351101)
passes; [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38134350782)
is queued at inspection. Tag-only success does not qualify the new shared-store
fixtures or native hosts and cannot close #44.

### Shared-store rejection and local failure retention

Pending **0.17.10** repairs the local consumer impact of
[Shared #121](https://github.com/dragginzgame/shared-tooling/issues/121).
The 0.17.9 collector exits before archiving local evidence if the shared selector
cannot admit the configured store, even when no host/IC tools exist yet.
The repair captures selector status and diagnostics, ignores all partial selection
output on failure, and archives the independently selected local logs, Rust tree,
Testkit attempts and fixture scratch. A physical local `.tools` directory also
contributes host/IC references and candidates; links are recorded without following
their targets. A redirected `.tools` root is excluded. No rejected store payload
is selected, and installer/check/store admission and shared snapshot bytes are unchanged.

An incomplete archive contains `tool-evidence/selection-status.txt` with the
original status and `shared_collection=incomplete`, `selection.log` and the raw
`partial-selections.nul` diagnostic. Collection exits with the original selector
status after producing the archive, retains metadata scratch, and emits no complete
collection measurements. Existing failure-only workflow upload steps still upload
the archive and collection log. Archiver failure remains a separate failure and
does not overwrite any previously retained output.

Maintained cases cover injected partial selector output and actual symlinked-store
rejection with physical, absent and redirected local tool roots. They compare
extracted local logs/build bytes and references, retain the rejection/status and
exclude physical shared bundles and external targets. Existing valid compact/full
retention, archive failure and occupied-output cases remain. These cases are
prepared, not executed: tests/builds/lint and native Linux/Intel/ARM qualification
remain user-owned under [#44](https://github.com/dragginzgame/ic-timers/issues/44).
The upstream composite action and its full/compact acceptance remain owned by
Shared #121; this change repairs only Timers' local collector.

### 0.17.8 release evidence

Released source **0.17.8** is
`afb773f20ed7bf4e11d86f8d2e5db916fe9f1654`. Its root lock selects Metrics
**0.6.1**, Testkit **0.33.3**, all four Host packages at **0.13.0** and PocketIC
**16.1.0**, with four local 0.17.8 packages. Shared selections remain **0.3.11**
`db143ae9ae140eb66826c413360723d23c078f66`, **56/30/11** files. These are
the maintainer's delivered selections; evidence inspection changes no Cargo input.

[Main run 38125888063](https://github.com/dragginzgame/ic-timers/actions/runs/38125888063)
binds that SHA to successful Linux checks **114430051840** and MSRV
**114430052089**. Intel macOS **114430052061** and ARM macOS **114430052077**
both complete successfully. [Tag run 38125888250](https://github.com/dragginzgame/ic-timers/actions/runs/38125888250)
passes tag-truth **114430050032** at the same SHA; other tag jobs are deliberately
skipped. Downloaded native logs establish acceptance for
[#42](https://github.com/dragginzgame/ic-timers/issues/42) and
[#43](https://github.com/dragginzgame/ic-timers/issues/43) at this released source,
independently of tag-only success.

Downloaded plaintext logs, exact run/job metadata and SHA-256 hashes are retained
at `/tmp/ic-timers-0178-acceptance.ncHCWdZo/`. Matched Linux evidence includes:

- `linux.log:1091` invokes the actual release-gate fixture and line 1092 reports
  its completed checks, including the committed separated diagnostic cases.
- `linux.log:1234` passes the measurement/registration-lifetime sequence;
  lines 1272–1273 pass both new Metrics boundary tests; line 1288 reports all
  **144** library tests passing.
- `linux.log:1485` reports **14** maintained recovery subjects passing;
  lines 1811–1814 emit all **four** cohort subjects and line 1817 passes their
  comparison gate. Measurements remain bound to this graph and Linux execution;
  no cross-host or historical regression conclusion is drawn from them here.
- The successful MSRV log identifies Rust **1.88.0** and completes the selected
  library checks plus the maintained documentation tests.

The completed Intel/ARM logs and refreshed run/job metadata are retained in the
same evidence directory, with SHA-256 hashes. Both native host-admission steps
complete under Apple's Bash 3.2 on their selected macOS 15 architectures.

- Intel `macos-intel.log:1377` completes the actual diagnostic fixture; line 1516
  passes registration identity; lines 1555–1556 pass both Metrics boundaries;
  line 1570 reports **144** library tests. Line 1777 passes **14** recovery
  subjects, lines 2106–2109 emit all **four** cohorts and line 2127 ends the gate
  with `VALIDATION PASSED`.
- ARM `macos-arm.log:1382` completes the diagnostic fixture; line 1520 passes
  identity; lines 1560–1561 pass both Metrics boundaries; line 1575 reports
  **144** library tests. Line 1780 passes **14** recovery subjects, lines
  2109–2112 emit all **four** cohorts and line 2130 ends the successful gate.

This pass downloads and inspects existing hosted evidence only. No local test,
build, lint, setup, CI redispatch or release runs. The superseded 0.17.7 Intel job
is cancelled and supplies no missing qualification. Intentional early/late
live-artifact failure dispatches are skipped; no fresh artifact round trip is
claimed. These results qualify the released graph, not incoming Host 0.13.1 /
Testkit 0.33.4 or the new Shared installer bytes. Evidence maintenance alone does
not justify another release; pending 0.17.9 separately contains compatible contributor-tooling adoption.

Local preparation checks pass for edited document paths, diff whitespace and full
locked offline metadata, including one selected Metrics 0.6.1 package identity.
Catalog, lock, pin catalogs, all snapshot manifests and staging are unchanged.
No Rust or shell source is changed by the acceptance-record update.

### Metrics 0.6 consumer qualification

Released **0.17.7** `1d21b9f55a8d9121355d216471495f92b0db7865` already selects
Metrics **0.6.0**, Testkit **0.33.2**, Host **0.12.10**, PocketIC **16.1.0** and
Shared **0.3.11**. The [identity owner](../design/callback-delivery-ownership.md#ic-metrics-06-released-graph)
records the public Metrics identity/mean change delivered in a patch in error;
future public hard cuts require a minor boundary, with no compatibility shim.

Preparation for released **0.17.8** addressed [#43](https://github.com/dragginzgame/ic-timers/issues/43)
without another dependency update or public semantic change. The snapshot
performance tests drive both real recording methods with synthetic instruction
values: empty versus completed zero, exact `u64::MAX`, a zero sample at that cap,
actual total overflow, sticky overflow after another zero, retained latest/maximum
and copied pre-overflow observations. The other callback role remains empty.
Metrics owns unreachable private sample-count overflow seeding; no test-only
production mutation path or count constructor is added here.

The existing native registration-identity test now records seven instructions
per completed callback and checks the measurement lifetime through retained stop,
cancellation, replacement and deterministic runtime reset. A replacement with
numerically larger readings admits arithmetic subtraction while its registration
identity differs; callers must reject that comparison before trusting an interval.
This tests native wiring and identity, not IC rollback or upgrade semantics.
Existing normal-completion/trap rejection, watchdog recovery, policy-cohort and
native host/MSRV gates retain their original responsibilities.

At preparation, new cases were written without execution. The
[released-source evidence](#0178-release-evidence) now proves execution on Linux
and both supported native macOS hosts, with the selected Metrics graph and
existing PocketIC evidence. This closes the consumer qualification for #43;
new dependency/tool selections retain their own gate. No new production recording logic,
platform reads, public item, persisted format, tool pin or claimed measured Wasm/
instruction delta is introduced. Cargo/catalog/lock mutation remains user-owned.

Preparation inspection passes: diff whitespace, edited document paths, all three
unchanged Shared snapshot identities and full locked offline metadata. The root
catalog/lock, tool pins, snapshot selections and staging are unchanged. The diff,
metadata and integrity logs are retained at
`/tmp/ic-timers-0178-metrics.A8nru6af/`. These checks do not compile or execute the
new Rust tests and cannot replace the user-operated gate.

[0.17.7 main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38123424231)
has successful MSRV/Linux checks and queued macOS jobs at observation.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38123424158)
passes at its exact SHA; that smaller job cannot close
[#42](https://github.com/dragginzgame/ic-timers/issues/42) or qualify pending #43
tests. Earlier 0.17.6 evidence remains bound to its original graph.

### Shared 0.3.11 portability, diagnostic and setup adoption

Released **0.17.7** includes reviewed Shared **0.3.11**
`db143ae9ae140eb66826c413360723d23c078f66` through all three exports;
[the adoption owner](shared-tooling-history.md#shared-tooling-0311-portability-setup-and-diagnostics)
records exact source and the corrected version annotation from Shared #114.
The incoming root lock selects Host 0.12.10;
the adoption preserves that maintainer change without selecting another version.
Shared #111/#112 physically anchor the selected host fixture and release-command
checker, including child temporary directories. Existing complete release checks
exercise both owners; optional producer portability fixtures are not imported.
[Timers #42](https://github.com/dragginzgame/ic-timers/issues/42) applies
[Shared #110](https://github.com/dragginzgame/shared-tooling/issues/110)'s reproduced
summary-loss defect to our real release adapter. Fatal/LLVM/missing-file
classifications previously existed only in live output; the selected runner now
uses the same diagnostic predicate for retained selection. Raw log handling,
bounded context, original failure status and literal consumer prefixes remain.
[Shared #101](https://github.com/dragginzgame/shared-tooling/issues/101)'s Rust
setup fix also keeps preflight, installed-tool probes and Cargo in the same
consumer configuration context, preserving explicit toolchain selection.

The existing local release-gate fixture exercises all diagnostic families through
its actual Make adapter with both prepared ripgrep and stock grep paths. Nine
ordinary lines separate each family, exceeding the four-line context window, so
neighboring matching errors cannot mask an omitted classifier rule. A 200-line
tail forces the early diagnostic retention path. Cases require status 2 and each
diagnostic in live output, highlighted retained output and the GitHub summary;
raw and combined logs stay undecorated. Passing/ignored `error::` names and
literal-prefix near matches remain neutral context. All summary paths are owned
by the fixture. The selected shared Rust fixture owns external-caller and
explicit-toolchain setup cases. Cases are written, not executed here. Our
source-inspection feedback on the upstream adjacent diagnostic block is now
addressed in committed 0.3.11: nine ordinary lines separate each family. The
producer's reported controlled regression probe rejects a deliberately missing
retained fatal rule with the spaced cases, while the old adjacent cases accepted
it. That is upstream evidence, not a consumer execution. The producer integration
fixture remains unselected; Timers retains its actual-adapter coverage.

Permitted preparation checks pass: all 56/30/11 snapshot identities, syntax for
70 shell scripts and 57 embedded Bash bodies plus the new Make recipe, diff
whitespace, local links and full locked offline metadata. No recipe/fixture body
is executed. These initial setup/diagnostic preparation checks compared catalog,
lock, both pin catalogs and raw index with the then-incoming inputs at
`/tmp/ic-timers-shared039.0vht4_71/`. The later canonical portability refresh
preserves the newly incoming Host 0.12.10 lock and records source/export logs and
inputs at `/tmp/ic-timers-shared0310.2pI56T14/`. Earlier dirty
0.17.6 acceptance records are retained rather than rolled back.
The portability refresh passes all three integrity checks, shell syntax, diff
whitespace and full locked offline metadata, preserving its own incoming catalog,
lock, pins and raw index. No fixture or recipe body is executed.
The final canonical 0.3.11 refresh retains unchanged selected payload bytes and
records source, export logs and preserved inputs at
`/tmp/ic-timers-shared0311.IIBZGj5v/`. Its manifests now report the corrected
producer version. All three integrity checks, syntax, diff whitespace and locked
offline metadata pass; incoming catalog, lock, pins and raw index are preserved.

Current/native qualification must come from the existing full user-operated gate
for committed consumer source. Earlier 0.17.6 native evidence remains valid for
its source and cannot close #42 or admit the pending Shared/Host changes. Shared's
0.3.11 exact-source lint job passes; Linux and both macOS jobs are still running
at observation. The superseded 0.3.10 run was cancelled. Host's exact-source
Linux/MSRV and both macOS jobs pass. Those
producer results do not qualify consumer adoption. This is repository tooling
only; normally bundle it untagged into code-bearing work. A maintainer-selected
release retains the complete gate. No timer/API, dependency mutation, optimizer
change, extra server route or claimed product Wasm/instruction delta is introduced.

### Mandatory Bash 3.2 assertions

Released **0.17.5** `d0fbdfb3288205f9f804c1706ed9ffa036dd2ea5` selects Metrics
**0.5.5**, Testkit **0.33.1**, Host **0.12.8**, PocketIC **16.1.0** and four local
0.17.5 packages. Its [tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38071494892)
passes; [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38071495004)
remains queued at observation. These states do not close the native-evidence
obligations of [#38](https://github.com/dragginzgame/ic-timers/issues/38),
[#39](https://github.com/dragginzgame/ic-timers/issues/39) or
[#40](https://github.com/dragginzgame/ic-timers/issues/40).

Pending **0.17.6** adopts the [Shared 0.3.8 exports](shared-tooling-history.md#shared-tooling-038-mandatory-assertions)
and repairs [#41](https://github.com/dragginzgame/ic-timers/issues/41). The
production release adapter explicitly refuses a successfully read but mismatching
workspace version before source admission, fetch or setup. The local lock updater
explicitly refuses missing, directory and symlink outputs before temporary output
creation or Cargo. Successful behavior and failure status 1 remain unchanged.
Intentional boolean predicates, including `semver_greater_than`, retain their
return semantics. Mandatory assertions in eight local fixtures and their embedded
stubs also exit explicitly, so completion cannot override a skipped assertion.
The macOS workflow explicitly refuses the wrong OS, architecture, OS version or
Bash version before selecting its native execution path.

The existing fixture-completion check extracts actual retained-path/status
assertions and the three workflow host-value assertions, supplies contradictory
inputs in disposable initialization/trap
copies, and attempts completion afterward. It requires status 1 and preserved
evidence. The version-preparation fixture exercises the actual lock updater
against missing/directory/symlink outputs, requiring status 1, no Cargo call,
empty output scratch and unchanged metadata/target. Existing preflight mismatch
cases now have explicit assertions. Cases are written, not run; acceptance needs
source-bound current Bash and native macOS Bash 3.2 logs from the existing full
gate. No contributor test, setup or release command runs. Preparation
checks pass: 56/30/11 snapshot integrity, syntax for 70 shell scripts, 56 embedded
Bash bodies, 96 completion prefixes, five contradiction prefixes and 28 workflow
run bodies, plus diff whitespace and locked offline metadata. Generated prefixes
and workflow bodies are inspected, not executed. Catalog/pin bytes and the raw
index remain unchanged; the external Metrics lock has a separate preserved copy.

During preparation an external update advances the selected Metrics lock to
**0.5.6**; preserve it rather than restoring the initial 0.5.5 bytes. Published
Metrics 0.5.6's sibling source comparison changes no library Rust source. The
workspace Host 0.12.8 selection differs from the cached published Testkit 0.33.1
CLI's shipped lock, which selects Host **0.12.7**; root selection alone does not
update that installed executable. No direct Host dependency or CLI override is
introduced. Dependency qualification remains separate from this tooling repair.
The batch is repository-only, with no timer/API change or product Wasm/instruction
delta. Keep it untagged for bundling unless the maintainer explicitly selects a
release; the complete user-operated gate remains required.

### 0.17.4 delivery and Shared 0.3.6 preparation

Current released source is **0.17.4**
`d7ae76c8b23ea4b0a868eb86330531ef8bd70116`, with four local 0.17.4 packages,
Metrics **0.5.3**, Testkit **0.32.2**, all four Host packages at **0.12.4** and
PocketIC **16.1.0**. Its
[tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38060308653)
passes; [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38060308649)
is queued at observation. Earlier job states and inspected source do not establish
complete native acceptance for this graph or the pending tooling repair.

Pending **0.17.5** adopts committed Shared **0.3.6** through all three exports,
repairs failed hook observations and delegates Testkit CLI lock selection to the
shared installer. The [adoption owner](shared-tooling-history.md#shared-tooling-036-hook-and-lock-selection)
records exact source, changed consumers, failure cases and permitted inspection.
The incoming maintainer-owned catalog/lock separately select Testkit **0.33.0**,
Host **0.12.4** and Metrics **0.5.4**; those bytes and the index are preserved.
Qualification remains the complete user-operated gate, including current/native
Bash, selected CLI/server admission and existing product PocketIC subjects.
No optimizer pin, deployment route or validation obligation is changed.

### 0.17.3 delivery and fixture log isolation

The maintainer reports **0.17.3** live at
`4cc3c64e6b773d73edb66f6bdf2d91cce5e68790`. The four local packages select
0.17.3, with Metrics **0.5.3**, Testkit **0.32.1**, all four Host packages at
**0.12.3** and PocketIC **16.1.0**. All three Shared snapshots remain 0.3.3.
The [main run](https://github.com/dragginzgame/ic-timers/actions/runs/38056525080)
has Linux **114226065101** running and MSRV **114226065026** passing, with ARM
**114226065141** and Intel **114226065188** queued at observation. The
[tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38056525399)
has tag-truth **114226066046** running. These states are not complete execution
evidence for [#38](https://github.com/dragginzgame/ic-timers/issues/38) or
[#39](https://github.com/dragginzgame/ic-timers/issues/39); both stay open pending
source-bound current/native Bash logs. No workflow is redispatched.

Pending **0.17.4** clears inherited `VALIDATION_LOG_DIR` and
`VALIDATION_FAILURE_LOG_DIR` in the consumer-owned release-gate fixture. The
fixture already owns logger checkout/depth and disables GitHub summaries, but
its direct logger calls still inherited caller-selected evidence directories.
Deliberate failed target output could therefore populate real validation logs;
an inherited retained-log directory also collected synthetic successful runs.
Clearing both selections lets each synthetic logger use fixture-owned roots.
The root Make release adapter still supplies its fixture-local failure directory.
The production runner retains its intentional inheritance for real nested checks.
This follows the producer fixture repair in
[Shared #105](https://github.com/dragginzgame/shared-tooling/issues/105), without
editing a shared file or adding a runner/installation path.

Preserved incoming catalog, lock, pin catalogs, index and fixture source are at
`/tmp/ic-timers-fixture-logs.a34mj0d4/`. Permitted shell syntax, diff, six changed
local links, snapshot integrity (56/30/11) and locked offline metadata checks pass;
fixture execution and current/native qualification remain user-owned. This is
repository-only work with no timer/API, Wasm or instruction change. No dependency
or release metadata is mutated beyond the undated changelog draft.

### Local fixture completion follow-up

After the maintainer reported **0.17.2** live at
`1cc87a467c4552b95d8a9728334e4e29eff0bc4c`, pending **0.17.3** repairs fourteen
consumer-owned fixture EXIT boundaries. Cleanup requires both successful status
and explicit completion after the assertions; incomplete/failed fixtures retain
their inputs, diagnostics and actual nonzero status. Release-gate and hook
admission now precede their first helper calls. Selected shared files are
unchanged and remain governed by their three immutable 0.3.3 snapshots.

The new [boundary check](https://github.com/dragginzgame/ic-timers/blob/651efce370bd6aec6290b74118eb709488062d0a/scripts/ci/test-fixture-completion.sh) copies the
actual initialization and EXIT admission of all sixteen local CI/release
fixtures, including itself and Testkit. It truncates each copy at the trap and
injects nounset, failed command, explicit nonzero, premature-zero, completed-zero
and completed-but-failed exits before any fixture body. It checks exact status,
successful cleanup and preservation of an evidence file for failure. Each probe
gets a fresh path marker and scratch paths containing spaces. The check is first
in `release-check`, so existing Linux/macOS gates run it with their selected Bash;
the standalone Testkit exit probes are consolidated there. No shared fixture is
patched or independently copied into the local boundary roster. These cases are
written, not run; [#38](https://github.com/dragginzgame/ic-timers/issues/38) remains
open for current-Bash and native Bash 3.2 qualification.

The incoming lock selects Testkit **0.32.1** under the existing `0.32` requirement.
During inspection external lock updates advance all four Host packages from
0.12.2 to **0.12.3** and Metrics from 0.5.1 to **0.5.2**. Preserve those newer
selections; no contributor lock write or rollback occurs. The root catalog,
pin catalogs and index compare exactly with the initial copies. This is
repository-only work with no runtime/API or measured Wasm/instruction change;
dependency and release execution remain maintainer-owned. Preparation inputs and
downloaded historical native logs are retained under
`/tmp/ic-timers-fixture-completion.9z8952mm/`.

Permitted syntax inspection covers sixteen local scripts, 31 embedded Bash
bodies and all 96 generated probe prefixes using the actual awk generator;
none of those prefixes or fixture bodies is executed. All three snapshot
integrity checks, seven added local documentation links and diff whitespace pass.
Initial locked offline metadata
resolves the Testkit 0.32.1/Host 0.12.2 selection. A fresh locked offline check
also resolves the externally updated Testkit 0.32.1/Host 0.12.3/Metrics 0.5.2 graph,
without changing its lock bytes. This is resolution, not execution qualification.

The same batch also repairs the production collector's status-only EXIT cleanup,
tracked separately by [#39](https://github.com/dragginzgame/ic-timers/issues/39).
`collect-failure-evidence.sh` requires explicit completion after final archive
diagnostics before successful cleanup. Incomplete zero-status exits become
failure; actual nonzero status and scratch evidence are retained. Archive
selection, paths, byte/mode handling and ordinary successful output are unchanged.
The existing evidence fixture adds six exit cases against copies of the actual
collector initialization/trap, truncated before Git inspection, tool selection
or archiving. Cases check status, successful cleanup, retained evidence bytes and
the retained-path diagnostic; full collection cases stay in their existing owner.
Syntax for both affected scripts and six generated collector prefixes plus diff
whitespace pass. No prefix or fixture executes. Cargo, lock, pins and index match
the separate preserved inputs at `/tmp/ic-timers-collector-completion.vb52tc9h/`.
Both #38 and #39 remain open for user-operated current/native Bash qualification.

The last local preflight repair captures the workspace-version reader's exit
status before comparing its output with `RELEASE_PREVIOUS`. Failed empty or
matching output cannot authorize source admission, fetch or setup. The existing
real-index fixture wraps that reader with a Bash stub and checks status 23,
untouched fetch/preparation logs, unchanged metadata/index and no release intent;
a successful mismatching read still refuses with status 1. The normal successful
preflight remains covered by its existing cases. Syntax for the adapter, index
fixture and four embedded Bash stubs plus diff checks pass; cases are written,
not run. Preserved inputs are at `/tmp/ic-timers-preflight-reader.1hfppy0_/`.
The same unchecked-observation pattern in the immutable pre-commit hook is
reported to [Shared #106](https://github.com/dragginzgame/shared-tooling/issues/106).
There is no active downstream hook patch or duplicate hook path; canonical
producer repair and source-bound qualification must precede its refresh.

### Host 0.12.2 and Testkit 0.32 review

Remote Host's latest committed release is **0.12.2**
[`e1ef99e6a4c6d05f0b0d8364f8586c6cc358dadc`](https://github.com/dragginzgame/ic-host-tooling/commit/e1ef99e6a4c6d05f0b0d8364f8586c6cc358dadc).
All tracked Rust source is unchanged from 0.11.0. The 0.12 line's changes cover
setup, release-tool ownership, jobserver propagation and fixture retention;
the applicable shared Make/fixture fixes are adopted through Shared 0.3.2 rather
than a new direct Host dependency. Its
[CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38051203749) passes
Linux and MSRV with both macOS jobs queued at observation; source equality is
not new runtime or native qualification.

Testkit's local clean preparation commit `fb62fee8696443676c92a3f86d413251dffd9327`
selects Host 0.12 through its four public reexports and prepares 0.32.0. Remote
main remains released **0.31.0** `f1ae9e6d3b0f3f20ec1e1f1b49c8b1dea3155e0a` at
inspection. The prepared release changes no Rust source but changes public
Host package identity, so its 0.32 minor boundary remains Testkit-owned.
Timers consumes startup/CLI calls through Testkit and exposes no Host types.
Wait for published, qualified Testkit 0.32 before reviewing a root catalog/lock
update; keep the current 0.31/Host 0.11 path without a sibling dependency, patch,
second native route or borrowed unreleased library bytes. No canister Wasm or
timer instruction savings follow from these native tooling changes.

### 0.17.0 release acceptance

The maintainer reports **0.17.0** live at
`5e0d0865248f6ebfc1f98c896581f21e2ce67831`; its annotated `v0.17.0` tag resolves to
that same commit. All four local packages are 0.17.0. The released root graph
selects one Metrics **0.5.0**, Testkit **0.31.0**, each Host **0.11.0** package and
PocketIC **16.1.0**, with Shared **0.3.0** snapshots at
`88a73139a0f083344c41a6f6f4b5c3a8aca7dc1d` (55/30/11). The
[measurement owner](../design/callback-delivery-ownership.md#ic-metrics-05-released-graph)
records the public type identity and unchanged published arithmetic source.
Testkit's separately installed CLI uses its packaged lock, also selecting Host
0.11; no direct Host dependency is added to the timer library.

[Matching main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38048325468)
passes Linux checks **114202244232** and MSRV **114202244227**.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38048325379)
**114202243658** passes. Intel **114202244037** and ARM **114202244196** now pass.
No job is redispatched. Existing Linux/MSRV raw job logs and metadata are retained under
`/tmp/ic-timers-shared031.aFkkRB/`; completed-job REST readback supplies Linux and
MSRV logs during the earlier unfinished run. Newly downloaded Intel and ARM logs
are retained under `/tmp/ic-timers-fixture-completion.9z8952mm/`.

Linux logs establish actual execution of selected-commit and real-index release
checks, version preparation, root lock coherence, ordered Testkit adapter
failure propagation, 142 library tests, 14 maintained recovery subjects and all
four policy cohorts. Both native macOS logs now show complete tool setup/check,
Shared 0.3.0 snapshot integrity, ordered Testkit adapter failure propagation,
142 library cases, all 14 maintained recovery subjects, four policy cohorts and
the final `VALIDATION PASSED` marker, selecting Metrics 0.5.0 and Timers 0.17.0.
Together these qualify the released graph and README-gate removal on all three
hosts; they do not qualify later graphs or fixture changes.
[#36](https://github.com/dragginzgame/ic-timers/issues/36) and
[#37](https://github.com/dragginzgame/ic-timers/issues/37) now have complete
source-bound adoption qualification. The contributor inspected existing hosted
results and locked offline metadata; no new test/build/lint/setup or release ran.
Historical 0.17.0 notes describe the earlier 0.4/0.30 preparation; current usage
must follow the delivered graph rather than those version examples.

### 0.16.6 release acceptance

The maintainer reports **0.16.6** live at release commit
`0b929539686a5c428a6a3af96c2a88139cc5553d`. All four local packages are 0.16.6;
the root graph selects Testkit **0.28.1**, Host **0.10.2**, Metrics **0.3.7** and
PocketIC **16.1.0**. All three released snapshots select Shared **0.2.13**
`5864f468d39f8f9d1bd26fca1afe0e20f25f1b5e`, retaining **55/30/11** files.
The separately installed Testkit CLI uses its packaged lock, including Host
0.10.1; the root graph does not change that installation contract.

[Matching main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38041494927)
passes MSRV **114182479493**, Linux checks **114182479646**,
Intel **114182479613** and ARM **114182479680**.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38041495172)
**114182480244** passes at the same SHA, with duplicate full tag jobs skipped.
Release availability, metadata and tag truth do not qualify the new execution
paths. The completed execution evidence below establishes consumer acceptance
for this released graph, independently of the new incoming selections.

This source adds explicit staged/working lock-preservation assertions, forbidden
directory cases, selected-CLI failure isolation and ordered preflight preparation.
The [Shared adoption owner](shared-tooling-history.md#shared-tooling-0213-selected-cli-preparation)
records their scope; prior 0.16.5 logs qualify only that earlier source.
Downloaded `/tmp/ic-timers-0166-{linux,intel,arm}-job.log` files now prove actual
hook formatting and failure isolation, directory admission, Testkit adapter
failure handling, release/version preparation, 142 library tests, 14 maintained
recovery subjects and all four cohorts on each host. The delivered hook fixture
contains the explicit staged/working lock-preservation assertions, so these
logs complete [#35](https://github.com/dragginzgame/ic-timers/issues/35)'s remaining
three-host proof. This does not qualify the later Shared 0.3 or incoming graph.
No contributor build/test/lint/setup or release commands ran; this records
existing hosted jobs.
[Completion feedback](https://github.com/dragginzgame/ic-timers/issues/35#issuecomment-6096552698)
closes #35 with its source-bound all-host proof.

### Dependency pin exceptions

[Exact exception records](https://github.com/dragginzgame/ic-timers/blob/651efce370bd6aec6290b74118eb709488062d0a/ci/dependency-pinning-exceptions.json) retain three
existing qualified selections rather than changing dependencies to make the new
checker pass. `ic-cdk-timers =1.0.0` fixes provider behavior audited in
[SAFETY](../../../crates/ic-timers/SAFETY.md), including cancellation heap retention and dispatch limits.
`ic0 =1.2.0` preserves the production platform bindings and counter-1 reader
reviewed in the [measurement owner](../design/callback-delivery-ownership.md#ic-metrics-02-adoption).
The root catalog retains `ic-cdk =0.20.3` for probe execution/suspension.
Current source-bound Linux PocketIC 16.1.0 recovery/cohort qualification is
recorded [above](#testkit-028-and-host-010-released-graph), including both complete
native macOS gates at their explicit canister toolchain. The historical 16.0.0 observations retain their own
source and do not define today's server selection. Testkit uses a compatible 0.20 requirement;
its former exact 0.17.3 exception is retired. These are product qualification boundaries, not blanket exact-pin policy.

Each exception matches its declaring root, dependency name and literal version.
Changing any selection requires reviewing its reason and rerunning the affected
source/runtime/host qualification; the old record cannot admit a new version.
The root lockfile and complete locked/offline metadata check remain required. Registry compatibility ranges, path inheritance and local workspace
ownership are unchanged. No exception permits a floating action or Docker image.

### PocketIC artifact pins

The following table is historical 0.14.19 artifact-review evidence. It is not
an executable pin catalog for the pending 0.15.0 handoff. Testkit now owns current
server selection, authentication and compatibility; the consumer adapter uses
its setup/check contract as described [above](#pinned-ic-tool-setup).

On 2026-10-08, official [release metadata](https://api.github.com/repos/dfinity/pocketic/releases/tags/16.1.0)
and the [PocketIC 16.1.0 release](https://github.com/dfinity/pocketic/releases/tag/16.1.0)
identify source `e9e42a6dd74acb9049017863e944cd5799be5b2b`. Assets **621918359**
(Linux x86_64), **621918384** (Darwin x86_64) and **621918361** (Darwin arm64)
were downloaded by exact asset ID. Each gzip matched the published archive hash
and byte size before decompression. The decompressed files supplied independent
binary hashes and ELF/Mach-O architecture checks. None was executed. Original
archives, non-executable decompressed bytes and `review.json` remain under
`target/evidence/pocketic-16.1.0-artifact-review/`. Artifact integrity does not
establish runtime recovery, startup/version execution or native macOS acceptance.
The complete user-operated gate must supply that fresh qualification.

| Release asset | Archive SHA-256 | Binary SHA-256 |
| --- | --- | --- |
| `pocket-ic-x86_64-linux.gz` | `131219d90dcf9bf6f3ed8ee02d8f55504ea24a6db382f5680675b7bd6b7ed3bf` | `b44e1eccd66e02328146b3e209e3403e3a3428a25d25555aca2f67500e5ae9db` |
| `pocket-ic-x86_64-darwin.gz` | `af9ad2d781530a43556ef2d1c8f93db99a2425c6cabdc78520f61922399ed530` | `a2ad872a5d84778b25a254c4eb4a8df99917b5792edaa7702d730de2d7de664d` |
| `pocket-ic-arm64-darwin.gz` | `9ae843fbb7ae6c6eb30137671c8629a80ef53b3a3652eb85b344847ac39f9fcd` | `2ffd9d5ae103cbb85289424056e68920459e702bf27317e38003b019a0a959d1` |

The former consumer server matrix, extracted-binary checker, downloader and
verification fixture are removed together in the hard cut. Their prior
qualification describes the historical route only; do not use the table to
recreate server admission alongside Testkit.

The 0.11.10 cache-type scenarios cover directories, FIFOs, rejected file/directory/
dangling symlinks and accepted verified file symlinks with both installation modes.
They check failure before download, link-target preservation and empty rejected
directories. Source, shell and embedded fixture-shell syntax, read-only 0.11.10
changelog preparation and diff whitespace checks passed; these scenarios remain
unexecuted and do not supply native host qualification.

### Testkit harness qualification

The original 0.17.3 harness adoption used published exact `ic-testkit` 0.17.3,
whose complete upstream PocketIC types remain available through the shared crate.
There is no direct `pocket-ic` dependency in this workspace. Locked dependency
metadata resolves one `ic-testkit` and one PocketIC 16.0.0 package; no declared
dependency MSRV exceeds Rust 1.88. This is metadata evidence, not successful
compilation on that toolchain.

The private harness requires the gate-selected `POCKET_IC_BIN`. It uses testkit
to start a caller-owned server and construct a fresh application-subnet instance,
with a 30-second deadline for each startup phase. Fixtures retain both bindings,
dropping the instance before the server. No baseline pool, implicit download,
test serialization lock or shared IC state is introduced. Upstream synchronous
instance deletion remains unbounded; the startup deadline does not cover it.

Both recovery and cohort fixtures keep their existing canister assertions.
The artifact verifier and its independent fixtures use the new host-specific
archive/binary pins, including explicit rejection of a 15.0.0 server. Old 15
receipts and sampling measurements remain historical and do not qualify 16.
The timer crate, provider and canister source are unchanged by this harness
migration; no Wasm savings or instruction improvement is claimed.

Preparation inspected the published crate archive against the registry checksum,
the official PocketIC release metadata and all three verified archive headers.
Dependency fetching, locked offline metadata, Rust formatting/parsing, shell and
embedded fixture-shell syntax, source flow and diff whitespace checks passed.
No PocketIC server binaries, builds, lint gates, native tests or PocketIC
fixtures were executed.
The 0.13.2 native macOS gates now supply matching compilation, lint, recovery
and cohort qualification for this harness, scoped in the host matrix above.
For changed source, the maintainer runs the complete `release-verify` gate,
including MSRV, probe lint, watchdog/ordinary recovery and policy cohorts. Record fresh
Wasm/instruction subjects and native Linux/macOS qualification rather than
reusing receipts from the previous simulator. Package version and Git release
execution remain maintainer-owned.

### Deployment validation

The 0.12 callback result cut used the minor release boundary for its incompatible
public signatures. `make test` runs workspace targets followed by API doctests;
`make msrv` compiles native all-targets and the library Wasm path, then runs those
doctests on Rust 1.88.0. Hosted MSRV setup explicitly prepares the Wasm target. Thus the
positive and compile-fail policy boundaries participate in PR/main validation
and the existing complete release gate on each declared host. The recording-Cargo
fixture checks order and failure propagation for both targets; it is not compiler
evidence. Automated preparation covered source, formatting/parsing, shell syntax
and read-only metadata review. Maintainer validation owns execution and new
native host/PocketIC/cohort qualification; preparation checks alone do not supply
that evidence. Cargo versions, locks and release execution stay
maintainer-owned; see the [cut's contract](../design/0.5-policy-specific-callback-authority.md#ordinary-callback-results).

Deployment validation belongs to the user. The combined `release-*` targets
run the complete release gate before bumping the version. The
gate can also be run directly before committing, tagging and pushing:

```text
make release-verify
```

Ordinary development and handoff follow Shared's
[validation cadence](../../../DRAGGINZGAME.md#validation-and-evidence), adopted by the
maintainer on 2026-10-14. Use the local command map in the
[hygiene overlay](../audits/code-hygiene.md#inspection-and-validation-commands)
for affected library/probe compilation, tests, lint and shell contracts.
For example, prepared offline library compilation remains available:

```text
CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 make --no-print-directory check
```

The former compile-only permission is superseded by that cadence. Focused checks
retain their own source/host scope and do not replace the complete owner release
gate. Wait for active validation before mutating or compiling its inputs; preserve
Cargo selection and evidence. Versions, release execution and publication remain
maintainer-owned; see [AGENTS.md](../../../AGENTS.md).

The gate starts with `make fetch`: `cargo fetch --manifest-path Cargo.toml --locked`
for the single root workspace, without restricting the target. This explicitly prepares
the selected lockfile's sources, including target-specific dependencies that
native builds may never download but unfiltered offline metadata needs. Fetching
uses the configured Cargo registry/cache and network policy; a download failure
stops before validation and version mutation. No offline failure is retried
online, no dependency version is selected anew, and no build runs in this phase.
The full validation gate follows successful preparation.

For standalone version preparation or offline work, prepare the cache while
network access is available:

```text
make fetch
```

An error such as `failed to download js-sys ... --offline was specified` means
the selected archive is absent from the cache. A different cached version is
insufficient. Populate the root lockfile's cache and retry the requested operation;
retain its selected versions. `update-dev` installs the toolchain and hook,
and does not prepare these dependency caches.

That gate includes `make ci`, the Rust 1.88 MSRV check, warning-denied linting
of every supported unpublished probe configuration, the maintained watchdog/recovery,
ordinary-await and provider-churn PocketIC subjects, and the four policy cohorts.
Release preparation explicitly installs the root-lock selected Testkit CLI and
its authenticated server. The subsequent offline admission and watchdog/cohort
commands use Testkit's admitted path. No custom binary override, consumer server
catalog or implicit validation-time downloader remains. See the
[setup contract](#pinned-ic-tool-setup).

After the version changes, the helper reads Cargo's no-deps member inventory
and updates every local package identity in `Cargo.lock` through the shared
rewriter. External selections are preserved. It then runs offline
`cargo metadata --locked` with full dependency resolution against the root
manifest through `check-lockfiles.sh`. The no-deps inventory is not the validation
gate: it does not establish lockfile coherence. Full locked resolution catches
stale member versions without building packages or repeating evidence suites.
The original `release-stage` target staged the root lockfile automatically;
current standard delivery delegates staging to the shared runner.

The checker captures successful Cargo output before parsing JSON. Cargo failures
retain their exit status and diagnostic without a cascading parse error from
empty or partial output. Updated root-graph fixtures retain locked-fetch failures,
failed metadata with empty or plausible output, stale package identities,
rollback/interruption, mode preservation and selected-commit admission. They
have not been executed by the contributor during this consolidation.

The maintainer's reported 0.11.8 preparation attempt
failed on uncached `js-sys 0.3.104`, then reported metadata rollback. Read-only
inspection found that archive and eight other selected testing archives absent
from the local default cache; it does not establish results in another cache or
native macOS qualification.

The original `release-stage` target selected only the three outputs the bump owns. Workspace members
inherit their versions, so their manifests are not version-bump outputs and
remain under the maintainer's separate code-staging ownership. Stage and commit
the intended implementation and supporting evidence before the combined release;
the release commit still rejects unrelated unstaged or untracked work.

Before mutation, the helper captures only its three output files: the workspace
manifest, root lockfile and changelog. Failed
commands and handled `INT`/`TERM` interruptions restore their pre-bump contents
and modes, including existing user edits; a previously absent file is removed
on rollback. If restoration fails, the backup is
retained and its path is reported. These shell traps do not cover a forced kill
or machine failure. Unrelated files, handoff/evidence documents and Git staging
are untouched by the bump. Builds and evidence artifacts remain intact on success,
failure and retry; release and publication targets do not append `cargo clean`.
Explicit cleanup is a separate maintainer action.

After the release tag is pushed, publish the crate with:

```text
make publish
```

The publish target requires a clean worktree with an annotated tag in the exact
`refs/tags/vX.Y.Z` namespace at `HEAD`, verifies the package, and then publishes
`ic-timers` to crates.io. A same-named branch or lightweight tag is rejected.

Hosted full CI and MSRV validation run on pull requests and `main`. A tag push
at the same commit does not repeat those Rust builds. Its small tag-only job
instead verifies that the event tag exactly matches the Cargo version, the
tagged commit is reachable from `main`, package metadata is coherent, and the
annotated version tag points to `HEAD`. This preserves protection against an
independently pushed tag from an unmerged commit without running identical
validation twice at one SHA.

The tag job verifies its event version, main reachability and annotated tag
before fetching dependencies. It then prepares the root locked cache and invokes
`check-lockfiles.sh` against the tagged checkout itself, before running the
release fixtures. Fixture success alone does not establish coherence of the
checkout's actual locks. Failed downloads, locked resolution or resolved
`ic-timers` identity stop the job; the root graph must resolve one package at
the Cargo version. This uses the existing fetch and metadata owners and does
not add compilation to their checks. The changed workflow shell was checked
for syntax and its sequence reviewed; diff whitespace checks passed. Hosted
execution and fixture execution remain unverified for this change.

For the maintainer-selected 0.11.9 target, the named undated changelog preflight,
current README projection, both locked offline metadata checks, workflow shell
syntax and diff whitespace checks passed. Cargo versions, both lockfiles and
the maintainer's existing staging were unchanged. Tests, builds, hosted execution
and Git release effects remain maintainer-owned; this repository-only patch
retains the complete release gate despite having no runtime changes.

## Standard release runner

The three standard SemVer entry points use the [common release contract](../../releases.md)
with explicit `RELEASE_REMOTE=origin` and `RELEASE_BRANCH=main`. The complete local
release gate retains its product qualification. Consumer adapters select the
three metadata outputs and the complete root graph. Publishing stays separate.
Normal targets select unfinished preparation intent before another increment.
The 22-file snapshot is exported from committed Shared Tooling revision
`cb86188c5956866564de4fb6ec6be67b27981ab9`; the
[adoption record](../development.md) distinguishes upstream qualification from
consumer evidence. This refresh addresses
[#10](https://github.com/dragginzgame/ic-timers/issues/10).

The runner owns commit ancestry, saved source/tree/subject, destination, locking,
annotated tagging and exact atomic branch/tag publication. Local committed,
tagged and push callbacks instead inspect `RELEASE_COMMIT`: the current adapter
exports its immutable source tree into an owned temporary directory and invokes
the current workspace-version and lockfile check owners there. The root
workspace manifest-sort check and the exact dated changelog heading remain
required. No scripts from the older tree are executed. Failed archive reads,
including partial output, fail closed; the temporary copy is removed without
touching build artifacts, plans or validation logs. This is cold release-path
disk work and has no crate, Wasm, runtime instruction or heap impact.

Preflight reads staged, unstaged and untracked paths separately, with NUL records
and rename detection disabled, before admitting only the three metadata outputs.
Its existing bump check rejects candidate/changelog conflicts before validation
or intent creation. After admission, preflight invokes the existing `make fetch`,
`make install-testkit-server` and `make pocketic-check` owners sequentially for
the complete locked root graph and selected CLI/server. It does not require cached archives before
that owner can populate them. Fetching retains Cargo's configured network/cache
policy and the existing lock selection; there is no offline-to-online retry or dependency
update. The complete gate retains its existing fetch-first ordering, and a
failed fetch, setup or admission stops preflight before validation or release mutation.
Locked fetching prepares an already consistent graph; it does not repair stale
lockfiles after a dependency requirement changes. The maintainer's 0.13.5 attempt
from `9d10b49851620296b55878b6aafb4ef92db6d45e` passed the root fetch and stopped
at the testing fetch: the root required/selected `ic-metrics 0.1.6`, while
`testing/Cargo.lock` still selected 0.1.5. An explicitly authorized targeted
offline update corrected only that package record. Both existing locked metadata
checks passed, with all other selections preserved; tests and full native release
qualification remain separate. Future dependency preparation must trace every
affected workspace, including path-dependent probes, and align applicable locks
before cache fetching. The requested common guidance is owned by
[Shared Tooling #6](https://github.com/dragginzgame/shared-tooling/issues/6), without
unlocking release fetches or changing release phase order.
The final commit adapter checks the entire index, requires
all three outputs to be tracked and the worktree to match the index, then applies
the existing metadata checks. Staged implementation changes hidden by restoring
only the working file cannot pass either admission boundary.

The complete `release-verify` target now delegates its unchanged target list,
in the same fail-fast order, to the canonical validation runner with its root
explicitly bound to Make's current repository. Raw failed
attempts are retained uniquely under the Git directory's
`release-state/validation-failures/`; `latest.log` is only a convenience copy.
Later attempts preserve earlier raw logs. Failed retention preserves the owned
temporary logs and reports their path. This adds logging and scratch disk work
to user-operated validation without package publication, cleanup, another gate
or runtime instrumentation. The logger's optional ripgrep branch has a stock
grep fallback; no new host installation is required.

The 0.14.4 logger refresh keeps passing/ignored `error::` test names and retained
context ordinary while highlighting actual diagnostics. Dispatched child loggers
choose their own checkout; inherited release selections, log policy and nesting
depth remain. The existing release-gate fixture covers these source-reviewed
cases through the actual adapter and both prepared/stock search paths. No target
membership, ordering or gate is removed; fresh consumer execution remains pending.

The tag checker takes explicit commit/version arguments only in those late
callbacks. Its no-argument mode still checks the current workspace and HEAD for
standalone tagging and publishing. Missing commit selections, floating refs,
lightweight/wrong tags, invalid metadata or failed checks stop recovery.
The compatible 0.14.4 worktree delegates those checks to the recorded shared
`check-release-tag.sh`; the local adapter retains only identity selection.
Diagnostics come from the shared owner, including the selected SHA for a wrong
target. The [adoption record](shared-tooling-history.md#annotated-tag-checker-adoption)
owns source, unchanged caller contracts and pending qualification.
When newer fixes exist, completing the older release does not implicitly publish
those fixes: the runner validates them afresh before preparing the requested
next increment. Changed remote history must be established before replay;
publication, deployment and cleanup remain separate.

`test-committed-release.sh` exercises the actual local Make callbacks and metadata
owners with Git/Cargo command stubs: selected metadata despite newer HEAD,
the complete locked root graph, corruption of all three metadata outputs, tag
conflicts, archive failure before/after output, failed resolution/manifest sorting,
missing selection and temporary-copy cleanup. New cases cover stale, noncanonical
and absent README content independently of metadata integrity; these cases have
not been executed by the contributor. The real-Git tag fixture checks an earlier
selected commit separately from HEAD. The shared runner fixture
owns the patch/minor/major and same/different-kind interruption matrix, lost
push replies, fresh-gate failure/retry, conflicts and locking. These fixtures
remain in the complete release gate on Linux and both native macOS jobs.
The real-index fixture reuses the current commit without creating commits
and checks hidden staged source, stale staged metadata, valid staged metadata,
untracked outputs and preservation of the selected index. It also injects failed
staged/unstaged/untracked producers with empty or plausible partial NUL output,
and failed index/worktree comparison. The cache scenarios exercise actual
preflight and the real fetch recipe with Cargo stubs, requiring both locked fetches
in order and stopping at either workspace's failure. Changelog, package metadata,
locks, source/index and absence of release intent remain checked independently.
These are producer and orchestration fixtures, not successful downloads or live
release execution. The existing actual
release-gate fixture now checks raw failure retention across failed/successful
attempts and execution with a tool path excluding ripgrep. Full gate sequencing,
failure propagation and PocketIC override assertions are retained.
The 0.13.4 hosted attempt failed at the subjects recorded in the
[host matrix](#host-support). The released 0.13.5 repair explicitly returns each
metadata-check failure, including workspace-version extraction, both manifest
ordering checks, README projection and locked resolution. Git path admission
also returns failed producer status explicitly. This avoids relying on Bash 3.2
errexit inside a function invoked from the selected-tree subshell. The fixture
injects each workspace's sort and lock failure independently, uses explicit
stub failure exits, checks temporary-copy cleanup and verifies sorting failures
stop before metadata resolution or tag lookup.

The index fixture deliberately uses a depth-one local transport clone and gives
only that isolated clone a controlled current-version baseline tag. It does not
fetch remote history, create commits or change production impact classification.
The existing index-preservation, failed-Git-query and ordered locked-fetch
subjects remain. Both fixtures print their captured adapter output on failure
before cleanup, so subsequent hosted failures retain the underlying error.
Source and shell/embedded-shell syntax inspection and diff checks are preparation
evidence. The later Linux and both native macOS 0.13.5 hosted gates pass the repaired
fixtures, scoped in the [host matrix](#host-support). No release
was executed during contributor repair.

The prepared host-only cohort row also appends `wasm_bytes`, measured from the
exact vector installed into PocketIC for each baseline/policy artifact. The
existing cohort gate builds and measures those artifacts as before; there is no
new build or evidence command. Earlier hosted logs have instruction subjects
but no byte counts, so no historical Wasm delta is inferred. The completed
0.13.5 native macOS gates supply executed output, scoped in the
[cohort review](../design/callback-delivery-ownership.md#released-0135-cohort-review).

### Metrics 0.3.6 incoming graph

The preserved incoming root lock for pending 0.16.5 selects Metrics **0.3.6**,
Testkit **0.28.0**, Host **0.10.1** and PocketIC **16.1.0**, with four local
members still **0.16.4**. Metrics 0.3.5 and 0.3.6 have byte-identical sets of
all seven published `src/**/*.rs` files in the Cargo registry cache; upstream
0.3.6 changes repository snapshot diagnostics and the private Wasm inspector's
Host selection. This requires no timer arithmetic adapter or public semantic cut.
The incoming lock also updates cc, smallvec and syn. Locked offline metadata is
preparation evidence only; released 0.16.4's tests/cohorts do not qualify the new
graph. No contributor Cargo mutation, build/test/lint or measurement runs.

### 0.16.6 incoming Host and Metrics graph

The preserved incoming root lock selects Testkit **0.28.1**, all four Host crates
**0.10.2**, Metrics **0.3.7** and PocketIC **16.1.0**; all four local members
remain **0.16.5**. Full locked offline metadata is retained at
`/tmp/ic-timers-0166-incoming-metadata.json`. It proves one resolved package per
selection, not execution qualification of this incoming graph.

Relative to Host 0.10.1, the three other Host libraries' published Rust sources
are byte-identical. Host Fs changes only durable pathname writers and their tests:
reject `/` or `/.` suffixes before normalization, parent creation, staging or the
producer, preserving existing bytes and BeforePublication/InvalidInput errors.
[Host 0.10.2](https://github.com/dragginzgame/ic-host-tooling/commit/6b755763aca71b7ea8c3de40edf032093dfdc62c)
retains that source identity. Our harness uses Testkit's managed startup, whose
startup/server/provisioning sources match 0.28.0 in the published 0.28.1 package;
it needs no writer adapter or direct Host dependency. Host belongs to the native
probe graph and is absent from the production ic-timers package's dependencies.

The selected CLI is independently installed with `cargo install --locked`.
Testkit 0.28.1's packaged Cargo.lock selects Host **0.10.1**, whereas Testkit 0.28.0
selected **0.10.0**. Thus CLI setup gains the 0.10.1 competing-parent sync fix by
selecting the new Testkit package; our root Host 0.10.2 update alone does not put
the directory-suffix repair into that installed executable. Preserve the owner's
locked build rather than adding a second catalog, local override or unlocking it.
The existing ordered release roster already performs explicit selected-CLI setup
and offline admission before CI; [Shared #96](https://github.com/dragginzgame/shared-tooling/issues/96)
identifies Timers as an existing prevention example.

All seven published Metrics Rust source files match 0.3.6 exactly; 0.3.7 changes
its repository formatter/admission tooling. No arithmetic adapter or public
semantic hard cut is required. New package identities still require the normal
user-operated gate; no Wasm/instruction improvement is measured or claimed.

[Host exact-source CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38037252544)
passes Linux native, MSRV and Intel, with ARM queued at inspection.
These upstream results and metadata do not qualify the consumer's new graph.
No contributor Cargo mutation, build/test/lint/setup, commit or release runs.
Snapshot directory admission belongs to the
[0.2.12 adoption owner](shared-tooling-history.md#shared-tooling-0212-directory-admission).

### Host 0.11 ownership review

Latest committed Host **0.11.0** is
[`1d768c80a5bb87e3330a6b7bacfdfc543063968f`](https://github.com/dragginzgame/ic-host-tooling/commit/1d768c80a5bb87e3330a6b7bacfdfc543063968f).
Locked offline trees confirm all four Host crates at **0.10.2** enter this
workspace through the unpublished `ic-timers-pocketic` harness's Testkit **0.28.1**
dependency. The published `ic-timers` library graph contains none of them.
The harness imports only Testkit's PocketIC API and uses its managed-server child
owner; the CLI additionally uses Host artifact inspection, filesystem publication
and process admission. Testkit's independently installed CLI still uses its own
packaged lock, as scoped above. No Host type enters the public timer facade.

The Host 0.11 hard cut unifies output limits, adds per-stream terminate/truncate
retention and composes cleanup errors. Published Testkit 0.28.1 still uses removed
`CommunicationLimits`, old output-limit fields and individual execution-error
cleanup fields. Its 0.10 requirements cannot select Host 0.11. Adoption belongs
to [Testkit #47](https://github.com/dragginzgame/ic-testkit/issues/47), including
its public Host reexports and resulting minor compatibility boundary. Timers has
no direct Host caller to migrate or reason to add a parallel dependency.
[Consumer feedback](https://github.com/dragginzgame/ic-testkit/issues/47#issuecomment-6095989932)
records this trace at that existing owner.
Our probes build through Make/Cargo, not Testkit's Wasm-cache build API; Host's
bounded live build diagnostics therefore supply no immediate timer optimization.

[Host release CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38040092044)
was queued at inspection. Source review and graph checks do not qualify Host
0.11 or a future Testkit adoption. Current Shared **0.2.13** remains the adopted
remote revision; no further snapshot refresh or Cargo mutation is required by
this inspection. Required native/product gates remain with the future selected
graph's maintainer-operated validation.

### Testkit 0.29 and Host 0.11 preparation

During pending **0.16.7** preparation after released 0.16.6, maintainer catalog
and lock edits select published Testkit **0.29.0** and all four Host **0.11.0**
packages. Testkit's remote release is
[`e15cc2acfd9324f6877f854415005f91d169a031`](https://github.com/dragginzgame/ic-testkit/commit/e15cc2acfd9324f6877f854415005f91d169a031).
All 40 published Rust source files match that committed release byte for byte.
Complete locked offline metadata at `/tmp/ic-timers-0167-testkit029-metadata.json`
retains one Testkit/Metrics/PocketIC identity and four local **0.16.6** members.
Metrics remains **0.3.7**, PocketIC **16.1.0**. Incoming Cargo bytes are retained
under `/tmp/ic-timers-shared0214.axid05wo/incoming/`; the contributor does not
mutate dependency selection or package versions.

The unpublished native harness uses Testkit's unchanged `pic` startup/builder
calls. Version-probe output quotas and deadlines are unchanged by the Host
migration: Testkit expresses them with per-stream `OutputLimit::Terminate` and
`timeout: Some(...)`. Provisioning likewise keeps its prior quotas/deadlines.
Host's composed cleanup errors stay with Testkit; Timers does not flatten or
reconstruct them. No harness adapter or direct Host dependency is needed; the
library dependency closure contains no Host/Testkit package. Testkit's Cargo
build runner preserves its existing output policy and no deadline, and our
Make/Cargo probe builds do not invoke it. No timer Wasm/instruction saving is
measured or claimed.

The root lock also selects CLI **0.29.0** through the existing adapter. Its
published packaged Cargo.lock selects all four Host **0.11.0** packages, unlike
the released 0.28.1 CLI's independent 0.10.1 graph. Keep `cargo install --locked`,
the owner's setup/check commands and admitted absolute server path. PocketIC
selection stays 16.1.0. No retained server bundle/cache reset or compatibility
route is introduced. Testkit's public Host reexport hard cut requires its 0.29
minor release; Timers exposes none of those types, so this remains compatible
0.16.7 repository-only qualification work.

[Matching Testkit CI](https://github.com/dragginzgame/ic-testkit/actions/runs/38041559012)
is queued at inspection. [Testkit #47](https://github.com/dragginzgame/ic-testkit/issues/47)
owns upstream migration acceptance. Source equality and metadata are preparation,
not executed startup/recovery or native qualification. The normal user-operated
release gate must prepare/admit the selected CLI/server and execute library,
MSRV, probe lint, watchdog/recovery and policy-cohort checks on supported hosts.
No contributor builds/tests/lint/setup or release effects run for this review.
[Consumer feedback](https://github.com/dragginzgame/ic-testkit/issues/47#issuecomment-6096202942)
records the published selection and unchanged harness at its migration owner.


## 0.18.0 delivery and 0.19.0 preparation checkpoint

Released **0.18.0** at `52bc4bad399122be040cf155d9b4563b5509e009` passes
[full main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38159662258)
on Linux, Rust-floor and both native macOS lanes; its
[tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38159662226)
passes separately. Earlier pending observations are superseded for this exact
source, not relabeled as qualification of subsequent dependency changes.

The explicitly approved old-source [late qualifier](https://github.com/dragginzgame/ic-timers/actions/runs/38154578484)
is complete at frozen `v0.17.13` / `e6ac645aac1b185bfaa3b146c2b8e1dd38b3299f`.
Linux 114514798873, ARM 114514798678 and Intel 114514798953 fail at the controlled
late step, then upload successfully. Download verification passes in Linux
114535962669, ARM 114535962592 and Intel 114535962580. This is the intentionally
failing producer workflow with successful verifier jobs, not an ordinary CI
failure to repair. Early's frozen verifiers still reject the obsolete candidate
path. Keep [#44](https://github.com/dragginzgame/ic-timers/issues/44) open for both
stages at one corrected frozen source; prior dispatch authority remains limited
to its named ref, and no new dispatch has occurred.

Pending **0.19.0** adopts the incoming Metrics 0.7.0 / Testkit 0.34.1 / Host 0.14.2
root graph and committed Shared 0.6.4. The incoming Cargo and consumer pin bytes
are preserved under `/tmp/ic-timers-019-adoption.9184ql8f/preserved/`. The explicit
compile-only exception yields a successful locked/offline library/all-targets
check (2.43 seconds), including test-target compilation without execution.
Formatting, snapshot integrity, syntax/diff and cheap metadata checks pass.
Behavioral tests, Clippy, probe/MSRV builds, IC policy cohorts and native host
qualification remain pending for these changed inputs. The wider public Metrics
identity requires a minor line; only notes are prepared, while package metadata
remains 0.18.0. No release effects, tool setup or automatic cleanup are performed.


## Host 0.14.3 and minimum-Wasm qualification follow-up

The later incoming root lock selects all four Host packages at **0.14.3**, through
Testkit **0.34.1**. Each cached official archive matches its selected checksum and
identifies released `543b137458feba0f6dfd3e458b680fb2be3d9e6d` / `v0.14.3`.
Library Rust paths are unchanged from 0.14.2; Host's new patch delivers contributor
Shared 0.6.5 receipt admission. Timers already adopts that committed Shared owner.
No direct Host dependency or consumer process/filesystem reimplementation is needed.
The incoming lock is preserved, with review inputs at `/tmp/ic-timers-host0143.j57w4_7_/`.
Earlier observations keep their original Host selection.

[Timers #49](https://github.com/dragginzgame/ic-timers/issues/49) identified that the
public Rust 1.88 floor was compiled natively but the maintained Wasm path used the
development compiler. The existing `msrv` recipe now runs native check, library
Wasm check, then API doctests. Hosted MSRV setup explicitly prepares
`wasm32-unknown-unknown`. The actual release-command fixture records all three
Cargo calls and rejects failure at each boundary. The same fixture's stale old
CI order is also aligned with the earlier compiler/Clippy admission.

The explicitly approved compile-only development permission covers the already
prepared compiler and target: `rustc 1.88.0 (6b00bc388 2025-06-23)` with
`RUSTUP_TOOLCHAIN=1.88.0 CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 make --no-print-directory wasm-check`
passes in 2.84 seconds. It builds only the library Wasm check, not tests, doctests,
Clippy, Testkit/Host probes or the complete MSRV target. No setup, fetch, package
mutation or lock rewrite occurs. Behavioral command fixtures and delivered-source
hosted MSRV/Wasm acceptance remain pending before issue closure. The pending
release stays 0.19.0 for the separate public Metrics cut; this qualification repair
is compatible and does not raise any Rust floor.


The same prepared Rust 1.88 toolchain also passes the locked/offline native
library/all-targets check in 3.20 seconds. Library test targets are compiled,
not executed. This supplements the 2.84-second Wasm proof above; it does not
run the full `msrv` target's doctests or qualify probe/Testkit/Host native behavior.
Incoming Cargo files remain byte-identical to the saved Host 0.14.3 inputs.


## 0.19.0 delivery and 0.19.1 maintenance

The maintainer reports **0.19.0 live** at
`81b82589ca08aeb966edaf77a09f3f611bcce079`, matching local HEAD and annotated tag.
[The tag job](https://github.com/dragginzgame/ic-timers/actions/runs/38221325648)
passes. [Full main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38221325703)
passes Linux 114718018090, MSRV 114718018369 and ARM 114718018350;
Intel 114718018302 is running at this inspection. The unfinished Intel lane
remains distinct from completed Linux/ARM/minimum-Wasm evidence.

The completed ARM raw log is retained at
`/tmp/ic-timers-019-issue-review/019-arm.log`, SHA-256
`bad8161b7dc5724d852f826aad2b4f0ba61ec02712591f9625ff6e2c7ca42c1f`.
It checks out the exact released SHA, passes the wide instruction-total
regression and release-gate fixtures, and executes native/Wasm/MSRV checks and
both recovery/cohort selections. This is source-bound ARM evidence for the
released graph, not qualification of the dirty 0.19.1 inputs.

The actual MSRV job log selects `rustc 1.88.0 (6b00bc388 2025-06-23)`, checks
native all-targets, then executes the locked/all-features library Wasm check
successfully in 3.82 seconds before API doctests and probe lint.
[Issue #49](https://github.com/dragginzgame/ic-timers/issues/49) is closed
from delivered-source evidence. Its raw log is retained at
`/tmp/ic-timers-shared066.jqtm0bp5/019-msrv.log`, SHA-256
`3448b2bb96f05a8af6d803702db50e6387dbdf2c4041e82a0b5c666423e399f2`.
The log contains the exact checkout SHA and compiler identity. Other issue/native
and controlled-artifact acceptance is not inferred from this one job.

Pending **0.19.1** adopts Shared 0.6.6 with cargo-sort-derives 0.14.1 and the
requested coherent Host **0.14.4** family. All four official Host archives match
the Cargo-generated candidate checksums and identify released
`f93cd8854d500616a73eda713d215a20ff0617c7`. Their 21/22/23/12 Rust payload files
respectively match the prior 0.14.3 archives byte-for-byte. No direct Host runtime
dependency or process/filesystem adapter is added.

The following Host 0.14.5 release at
`bb8f1e71f3d4599618d0f4084d27fa637408f1fb` matches upstream main on inspection.
Its complete diff from 0.14.4 changes only Cargo package versions/lock entries
and the changelog heading. The `crates/` diff is empty. No new Host API or
implementation is available to adopt, so this batch retains the reviewed 0.14.4
graph without another dependency update.

The offline resolver was run in an isolated copy, then reviewed before applying
its lockfile. Host's four versions/checksums advance; every other package version
stays fixed. Cargo also regenerates Windows dependency edges for errno,
quinn-udp, rustix, rustls-platform-verifier, tempfile and winapi-util from the
already-selected windows-sys 0.61.2 identity to 0.52.0. Both package identities
remain present. The full Cargo-generated result is retained; no partial manual
registry rewrite conceals the resolver's graph normalization. Source/archive
comparison and the candidate review are at `/tmp/ic-timers-host0144.5jgzykdz/`.

The approved offline, locked library/test-target compile check passes in 2.72
seconds, with no test execution. Formatting, snapshot integrity and cheap locked
metadata pass. The new common toolset requires explicit maintainer preparation
and offline admission. Tool/hook/release behavioral fixtures, Clippy, full/native
validation and simulator qualification are not run locally for this batch.
Package versions remain 0.19.0; only the undated 0.19.1 notes are prepared.
This maintenance changes no timer API and claims no Wasm/instruction saving.

The subsequent requested Shared refresh selects committed
`8e8cc21265272e0234326c53a8c7bf9e7d8d77df` at upstream main. This implements
the 0.7 entrypoint selection contract; its committed `VERSION` and canonical
manifest annotations still say 0.6.6. The adoption owner records all three
explicit root sets and derived payloads, complete previews and canonical apply
logs at `/tmp/ic-timers-shared070.n53MkYKh/`. All 59/35/11 prior payloads remain;
there is no retirement, dependency/pin change or new product command. Integrity,
syntax, local links and diff checks pass; no runtime build or user-owned
behavioral/native gate is rerun for unchanged Rust source. Saved Cargo/tool pins
and index entries preserve the incoming batch's identities.

The maintainer then requested Shared **0.8.0**, adopted canonically at
`045a6f22a9de497bfb3af90cb2571957384806f2`. Committed VERSION and manifest
annotations now agree. The exporter reuses recorded entrypoints and
automatically retires verified unchanged obsolete payloads without the removed
`--retire-file` flag. All three previews preserve every current root/payload;
this refresh removes nothing. The 59/35/11 complete preview/apply logs and
preserved inputs are at `/tmp/ic-timers-shared080.IMvYFaPt/`.
Integrity/local file links/diff checks pass; incoming Cargo files, tool/IC pins
and real index entries stay identical. Only selected guidance and provenance
change from the preceding adoption, so no compile or user-owned behavioral/
native gate is repeated. Existing artifacts, prior failed observations and
the pending 0.19.1 boundary remain intact.

For [#44](https://github.com/dragginzgame/ic-timers/issues/44), the existing
qualification workflow can now use frozen `v0.19.0` at the exact SHA above.
The proposed sequence, pending explicit maintainer dispatch authorization, is
early after main CI completes, then late only after early completes:

```sh
gh workflow run ci.yml --repo dragginzgame/ic-timers --ref v0.19.0 -f failure_stage=early
gh workflow run ci.yml --repo dragginzgame/ic-timers --ref v0.19.0 -f failure_stage=late
```

Require all three original producers to reach the selected controlled failure
and upload, and all three download verifiers to pass at each stage. Retain run,
job, artifact, source and digest identities with the existing evidence procedure.
No new dispatch has run; the old-source failed early run remains unchanged.

## 0.19.1 delivery and issue acceptance

Subsequent inspection establishes committed/tagged/pushed **0.19.1** at
`0c24fac2fc1d38d216f6880a581f5ff9b85020e2`, matching HEAD and the annotated
remote tag's commit. Main run
[38231417336](https://github.com/dragginzgame/ic-timers/actions/runs/38231417336)
is underway, with Linux/MSRV running and Intel/ARM queued; tag run
[38231416722](https://github.com/dragginzgame/ic-timers/actions/runs/38231416722)
is running at inspection. The committed root lock selects Metrics 0.7.1,
Testkit 0.34.2, the four Host 0.14.5 packages and PocketIC 16.1.0. The incoming
Testkit 0.34.3 lock edit is preserved separately and has no result from these
committed-source runs. Source/tag evidence does not independently prove registry
publication.

The new main push cancels 0.19.0's Intel full gate during PocketIC preparation.
The original raw log is retained at
`/tmp/ic-timers-019-issue-review/019-intel-cancelled.log`, SHA-256
`987e5fb881a926d05d604ec64e2c038fe0fdfc91e69635a03182680c46694f6a`.
At exact checkout `81b82589ca08aeb966edaf77a09f3f611bcce079`, locked all-target
library compilation completes at 09:15:07 UTC, before Clippy and release
fixtures. The release-gate fixture succeeds at 09:32:22 UTC and the
repository-check fixture at 09:33:49 UTC. The latter exercises compiler/Clippy
fail-fast ordering and successful complete once-only routing. Library tests
also complete before cancellation. Linux and ARM complete the same owning
checks in successful gates. No matched timing improvement is claimed.

This completes [#48](https://github.com/dragginzgame/ic-timers/issues/48)'s
compile-permission and ordering acceptance. The cancelled full Intel job stays
cancelled: its remaining simulator/report qualification is retained by #47,
and complete common-Make/artifact qualification by #44. The actual Rust and
ordering fixture bytes are unchanged in 0.19.1, but this does not qualify its
new dependency/tooling inputs. No new local test, build or release ran for this
issue closure, and no changelog draft is needed for existing-evidence acceptance.

The maintainer explicitly approves both existing failure-qualification stages
at frozen **v0.19.1**, sequentially after matching CI completes. Approval and
remote annotated-tag readback are saved before dispatch at
`/tmp/ic-timers-0191-qualification.IQiwIXf8/`; the exact source is the SHA above.
The concrete approved commands are:

```sh
gh workflow run ci.yml --repo dragginzgame/ic-timers --ref v0.19.1 -f failure_stage=early
gh workflow run ci.yml --repo dragginzgame/ic-timers --ref v0.19.1 -f failure_stage=late
```

Early must complete before dispatching late. Record each intent and reconcile
its response/run identity without repeating a possibly accepted request. Require
all three original producers to reach their selected controlled failure and
upload, and all three download verifiers to pass in each stage. Late's original
native gate must complete before its controlled failure. Both stages are
awaiting matching CI; neither has been dispatched. Older archives and failed
hosted verifiers remain unchanged. This specific approval does not authorize a
new source ref, release, sibling edit or unrelated validation.

The matching frozen-tag CI completes successfully. Main separately passes
Linux/MSRV while native jobs remain queued; the manual frozen-tag ref has its
own concurrency group and does not cancel those main jobs. One **early** stage
is then dispatched and reconciled uniquely to
[38234040990](https://github.com/dragginzgame/ic-timers/actions/runs/38234040990)
at the exact approved source/ref. Original tag/run JSON, before/after run
inventory, dispatch intent, stdout/stderr and receipt are retained in the same
qualification directory. Early is queued at inspection, not qualified. Late
remains undispatched and must wait for early completion. No approved request is
repeated and the old-source observations remain unchanged.

Early Linux producer **114760214224** reaches the intended controlled installer
failure, exit 22, and successfully archives/uploads. Artifact **11739996419**
has uploaded and downloaded ZIP SHA-256
`109a9e235176c83d6640a5779915086ddee9be69057ae385bea4900b74d7a2b6`.
The original producer log SHA-256 is
`880e48b2586fef3d5964fa842aae576989f5a31eaa9ae2424a8d0e12cc624bc0`.
ZIP, raw log and server artifact metadata remain in the same qualification
directory. Intel/ARM producers remain queued; the three download verifiers and
complete early acceptance are not established. Late is still undispatched.

Read-only replay uses the exact committed 0.19.1 archive verifier and its frozen
pin inputs, with early/Linux job identity, run 38234040990 and attempt 1.
It admits the downloaded Linux tar, including preserved bytes/modes and the
physical rejected-download payload. The frozen files and result remain under
`frozen-source/` and `early-linux-local-verify.log` in the same directory.
This is local artifact inspection, not execution of the three hosted download
verifiers or complete early acceptance. Main's Intel native job has started;
its gate is still incomplete and ARM remains queued.

Early ARM producer **114760214442** subsequently reaches controlled exit 22
and successfully archives/uploads artifact **11741875127**. Uploaded/downloaded
ZIP SHA-256 agrees:
`beed68c6015b790bf4f458c0e71c3dd4bdd3b5538f9b71f1a9e121999db4d462`.
Original raw-log SHA-256 is
`a74c24bf8a90aa408f81e4930e201d3db15522bbfafc591bb6ba01fe9ac58bf0`.
The exact frozen verifier admits this tar locally with job `macos`, host
`macOS/ARM64`, original run 38234040990 and attempt 1. ZIP, raw log, artifact
metadata and `early-arm-local-verify.log` remain in the qualification directory.
Intel's producer and all three hosted download verifiers remain outstanding;
local replay does not complete their acceptance. Both main native jobs have
started their full gates, but neither has a completed result at inspection.

## Prepared Shared 0.8.3 for 0.19.2

The maintainer requests Shared 0.8.3, adopted from committed upstream-main
`8cca7e67b03ce6e701b6e7dcec2e9753eb98337e`. Exact committed VERSION and
canonical annotations agree. The [adoption owner](../development.md) records
stricter record admission, source-owned atomic payload replacement, retained
build diagnostics/redirect refusal, and formatter catalog selection.
Complete previews and apply logs are retained at
`/tmp/ic-timers-shared083.2RFeaOnh/` with all incoming identities.

The baseline is now 67 files, with audit/helper counts 35/11. Seven common
installer/build/checksum companions were already physically owned by the audit
snapshot; the additional maintenance procedure completes the catalog. No
retirement, second tool store, optional local checker or scheduler is introduced.
Local formatter targets bind the Rust catalog to reviewed root pins; existing
hook/format coverage is extended for environment and command-line overrides.
The selected owner fixtures remain in the normal complete release gate.

Canonical integrity, changed shell syntax, owner-document file links and diff
checks pass. Incoming Cargo files, tool/IC selections and the real index remain
identical. Product Rust is unchanged, so no compiler run is repeated; tests,
lint, setup and native/tooling qualification remain user-owned. Only compatible
undated **0.19.2** notes are prepared; package versions remain 0.19.1.
Existing frozen v0.19.1 qualification retains its exact source and graph and
does not establish acceptance of the newly adopted 0.8.3 bytes.

## Prepared Shared 0.8.4 update

The same compatible 0.19.2 batch advances to requested committed Shared 0.8.4
`dca2bdd018afa6be598f03bb3cce1bab79c66973`. All three canonical exports retain
67/35/11 payloads and recorded entrypoints. Clean-source previews precede apply;
incoming identities and logs remain at `/tmp/ic-timers-shared084.rd6lPKII/`.
Uncommitted sibling 0.9 parser work is excluded. The [adoption owner](../development.md)
records the selected LOC JSON reporter/fixture and companion-selection guidance.
Existing formatter binding and earlier 0.8.3 safeguards are retained.

Integrity, changed Bash syntax, owner-document file links and diff checks pass;
Cargo files, pin selections and real index entries remain identical. No runtime
Rust change, tool installation, test, build, lint or release runs for this update.
LOC/JSON fixture and native/full acceptance remain user-owned. Package versions
stay 0.19.1, and the existing frozen v0.19.1 qualification keeps its original
source/graph rather than being attributed to the new 0.8.4 snapshot.

## Prepared requested Shared 0.9.0 source update

The compatible 0.19.2 batch now selects committed Shared
`019ce8236cc7cee14ea5540d468b939a2053ef30`, whose subject and changelog name
0.9.0 but whose committed VERSION remains 0.8.4. Canonical annotations retain
that value and exact source identity; [Shared #137](https://github.com/dragginzgame/shared-tooling/issues/137)
owns reconciliation. The [adoption owner](../development.md) records the shared
manifest reader, NUL rejection and separate syntax/integrity/ownership contracts.

Complete clean-source previews precede canonical apply for all three bundles;
67/35/11 files and existing roots remain intact. Preserved inputs and logs are
at `/tmp/ic-timers-shared090.ucWTtqQQ/`. Integrity, changed Bash syntax, document
links and diff checks pass; the root manifest, pins and real index entries are unchanged.
Cargo.lock changed concurrently to Testkit 0.34.4, Host 0.14.8 and Metrics 0.7.2;
its original and observed bytes are retained without repair or qualification.
No test, build, lint, setup or release runs for this update. Native/behavioral
acceptance stays user-owned, and frozen v0.19.1 results retain their exact scope.

## Completed 0.19.1 native and early qualification

[0.19.1 main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38231417336)
is complete and successful at exact source
`0c24fac2fc1d38d216f6880a581f5ff9b85020e2`: Linux 114752068103,
MSRV 114752067743, Intel 114752067890 and ARM 114752068062. The tag run passes
separately. Both native logs establish the wide-total/mean role/order regression,
149 library tests, API/frontier checks, native/Wasm/MSRV/probe validation,
15 selected recovery/report tests and the separate policy-cohort selection.
The report codec uses synthetic wide input; small actual instruction measurements
are not relabelled as observed values above u64. The committed graph remains
Metrics 0.7.1, Testkit 0.34.2, Host 0.14.5 and PocketIC 16.1.0.

Raw completed logs are retained under
`/tmp/ic-timers-0191-qualification.IQiwIXf8/`:

| Log | SHA-256 |
| --- | --- |
| `main-linux.log` | `939f7717ae734125954becebfa29cfc6447f829148a6d8848ab2a76253ab397a` |
| `main-intel.log` | `64b23e3c96d582bf87981f8f09bd311b4dd04e0525469a65ded4da1fcd7c6d19` |
| `main-arm.log` | `4d55b4f9a6d0e87a61c624be4b467c415753cbbe6f8e2783f8e57c01471a94a8` |

This closes [#47](https://github.com/dragginzgame/ic-timers/issues/47)'s owning
native/report adoption. Earlier cancelled logs keep their original outcomes;
these results do not qualify incoming Testkit 0.34.3 or Shared 0.8.4 maintenance.

The authorized frozen [early run 38234040990](https://github.com/dragginzgame/ic-timers/actions/runs/38234040990)
is also complete. All three original producers reach controlled exit 22 and
archive/upload successfully. All three hosted downloaded verifiers pass:
Linux 114791741093, Intel 114791741108 and ARM 114791741301. Original producer
jobs remain failed; overall run failure is expected and is not hidden.
The last Intel producer 114760214487 uploads artifact **11743982790**, whose
uploaded/downloaded ZIP SHA-256 matches
`1aa0209e118d2729aed444e9ae32c7e14e24ebc9469b60a798564b47a36cbbae`.
Its original raw-log SHA-256 is
`f847a55b67161a67523db5671d39a5fcaae884e42034231fa0a45af0b5815462`.
The same frozen verifier also admits that archive locally; hosted acceptance
comes from the three successful verifier jobs, not from this replay.
All archives, run/job/artifact JSON, original logs and receipts are retained
in the same qualification directory alongside the earlier Linux/ARM digests.

Only after that completed early stage, the approved **late** dispatch is made
once and uniquely reconciled to
[38245658495](https://github.com/dragginzgame/ic-timers/actions/runs/38245658495)
at the same frozen v0.19.1 source/ref. Exact tag readback, before/after manual
run inventory, intent, stdout/stderr, receipt and original run JSON are saved
before/after the one request. It is queued at inspection; no late acceptance
is claimed. #44 remains open until all three late original producers finish
the complete gate and selected controlled failure/upload, and all three hosted
downloaded verifiers pass. No new source selection, repeated dispatch, local
product test, release effect or sibling edit occurs.

## 0.19.2 delivery and corrected Shared provenance

Timers 0.19.2 is committed, tagged and pushed at
`8b3e09fe82ee6a7e9cebda904536b1717bfc1fd3`. Remote annotated tag object
`08897405c6c929b68cecd5223c8650b5cfb90fef` dereferences to that source.
The committed graph selects Metrics 0.7.2, Testkit 0.34.4, all four Host 0.14.9
and PocketIC 16.1.0. Hosted runs 38247833714/38247833419 are incomplete at
inspection; neither publication nor complete hosted acceptance is inferred.

Shared's corrected 0.9.0 source `aec7a149f581a42b9531fc747b46024fc0314fcf`
changes only VERSION from the adopted preceding source. All three complete
previews retain every payload as KEEP. Canonical apply updates annotations and
source revisions while preserving identical 67/35/11 payloads. Saved inputs,
manifests and logs remain at `/tmp/ic-timers-shared090-corrected.L7AXQHLo/`;
the [adoption owner](../development.md) records the correction separately from
the original mismatched-version observation. Integrity/link/diff checks pass,
the root manifest/pin/index inputs are identical and no local test/build/lint/setup/release
or sibling mutation runs. Undated 0.19.3 notes are repository-only; package
versions stay 0.19.2 and no metadata-only publication is recommended.
Cargo.lock changes concurrently to Testkit 0.34.5; original and observed inputs
are retained separately, without write, restoration or qualification by this pass.

The approved late run still selects frozen v0.19.1. Linux producer 114798745212
completes CI/probe lint/PocketIC recovery and policy cohorts, then intentionally
exits 2 at the late qualification step and archives/uploads successfully.
Artifact 11746077825 reports uploaded ZIP SHA-256
`bcc9ed0393d7d64aa1eb2bba250bbe1fa95a35732f17ba420de19a1efc913f0c`.
The downloaded ZIP matches that digest. Only its named tar member is copied;
the exact frozen verifier admits it locally with original source/run/attempt
and pin inputs, without extracting arbitrary archive paths. This is archive
inspection, not successful hosted late verification or fresh product execution.
Original raw producer log SHA-256 is
`f976ae7aa50af0038489080307657a6dd37604287cdca35c0d24f02529957e38`.
The original log and run/job/artifact observations are retained at
`/tmp/ic-timers-0191-qualification.IQiwIXf8/`. Both macOS producers remain
queued and hosted late download verification is incomplete at inspection;
#44 remains open. No existing approval is repeated or changed to another source.

## 0.19.3 delivery and prepared Shared 0.9.1 cleanup

Timers 0.19.3 is committed, tagged and pushed at
`1e451ba35613f2efde601073636c63ed97a91c06`, with remote annotated tag object
`35298ffad70aca7a64ee3ff752e5173a1c72717a` dereferencing to that source.
The committed graph selects Metrics 0.7.2, Testkit 0.34.5, Host 0.14.9 and
PocketIC 16.1.0. Tag run 38249543691 passes; main run 38249542866 is queued at
inspection. Registry publication and full native acceptance are not inferred.
The incoming lock selection changes Metrics to 0.7.3 and remains untouched.

The requested maintenance continues with reviewed Shared 0.9.1
`e8a9ea6b660f216166eb00d0af6a32462bdbda4a`. Complete clean-source previews
precede canonical apply for all three bundles, retaining 67/35/11 files and
every existing root without additions or retirements. The [adoption owner](../development.md)
records common host/IC lifecycle handling, lock cleanup/status preservation and
selected owner fixtures. Incoming Cargo/pin/index inputs remain identical;
integrity, Bash syntax, owner-document links and diff checks pass. Evidence is
at `/tmp/ic-timers-shared091.LNqFhTsF/`. No local tests, builds, lint, setup or
release commands run for adoption; native/behavioral qualification remains
user-owned. Undated 0.19.4 notes are compatible repository-only maintenance.

The original /tmp qualification/adoption directories are absent at this later
inspection. No cause is inferred and no cleanup is performed by this work.
Available frozen v0.19.1 early and late run/job/artifact JSON, original producer
logs and exact ZIP archives are re-downloaded into
`/tmp/ic-timers-0191-qualification.IQiwIXf8/restored-readback-20261012/`.
All three early ZIP and raw-log hashes match their previously recorded values;
the early run's source/ref/attempt and three hosted verifier successes are
checked independently against GitHub again. Linux late ZIP/log hashes also
match their prior observations. Original local dispatch intent/reply files and
v0.17.13 archive copies are unavailable; they are not recreated as if original.
Historical records and approval remain in the existing issue/conversation;
neither stage is dispatched again or switched to a new source.

ARM late producer **114798745283** passes its complete native release gate,
reaches controlled exit 2 and archives/uploads successfully. Artifact
**11751795650** has matching uploaded/downloaded ZIP SHA-256
`53f924106ed3bc1b4e7958d0f9977a601bc431259baa3928b0438dadbaf91ad5`.
Its original producer-log SHA-256 is
`8b71962e7e28ac1231da3f4c4f64202f5d1767303b9d9d8c004e1bee3811442b`.
The exact frozen verifier admits ARM's named tar payload locally with the
original source/run/attempt and committed pin inputs. This replay is archive
inspection, not successful hosted late download verification or new native work.
Source remains frozen v0.19.1/attempt 1; Intel and hosted late artifact
verification are incomplete at inspection. #44 remains open, separately from
the newer released dependency graph and pending 0.9.1 snapshot.

## Prepared Shared 0.9.2 and completed frozen qualification

The same undated 0.19.4 contributor batch selects committed Shared 0.9.2
`c118062a070a2e238f1eb3989b6fd700f369ce1e`. All three canonical previews
precede apply, retaining 67/35/11 payloads and recorded roots without additions
or retirements. The [adoption owner](../development.md) records NUL admission
before evidence hashing and CR/LF installer-path rejection before configuration
or setup. Saved inputs/logs remain at `/tmp/ic-timers-shared092.W81svtKr/`.
Integrity, changed Bash syntax, owner-document links and diff checks pass;
Cargo/pin/index inputs match saved bytes/entries. No local tests, build, lint,
setup or release commands run for adoption. Behavioral/native acceptance of the
new snapshot is distinct from the completed frozen qualification below.

The approved sequential **early then late** pair is complete at frozen
v0.19.1 `0c24fac2fc1d38d216f6880a581f5ff9b85020e2`, attempt 1. Main CI
38231417336 is re-confirmed successful at that exact source. Early run
38234040990 has all three controlled exit-22 producers/uploads and all three
successful hosted verifiers. Late run
[38245658495](https://github.com/dragginzgame/ic-timers/actions/runs/38245658495)
has successful full Linux and Intel/ARM native gates, followed only by the
selected controlled exit 2 and successful archives/uploads on every producer.
Hosted downloaded-artifact verifiers pass: Intel **114854856791**, Linux
**114854856857**, ARM **114854857036**. Both overall run conclusions remain
intentional failure; no original producer or old failed attempt is relabelled.

Final late uploaded/downloaded ZIP identities:

| Host | Artifact | SHA-256 |
| --- | --- | --- |
| Linux/X64 | 11746077825 | `bcc9ed0393d7d64aa1eb2bba250bbe1fa95a35732f17ba420de19a1efc913f0c` |
| macOS/ARM64 | 11751795650 | `53f924106ed3bc1b4e7958d0f9977a601bc431259baa3928b0438dadbaf91ad5` |
| macOS/X64 | 11752557455 | `9e62a54e5e116d21b5884304f28c2c503084b0dd37b473de9cb54afb68fe982a` |

Intel producer **114798745371** completes its full native gate before the
controlled failure. Raw log SHA-256 is
`690e490636358928dde96da47a227cc8912e6a59a5b7828ab64180c9ec1287ea`.
Final run/job/artifact JSON, uploaded/downloaded ZIPs, producer and hosted
verifier logs are retained in the existing restored-readback directory. Its
missing original local intent/reply and v0.17.13 copies remain explicitly
unavailable; no new dispatch or fabricated receipt replaces them. The existing
approval, original submitted run identities and GitHub source-bound outcomes
remain authoritative. This closes #44's named acceptance at its delivered
Shared 0.8.0/59/35/11 graph; it does not qualify the newer pending 0.9.2 bytes.

## Prepared 0.20 typed constructor hard cut

The maintainer approves cleaner registration setup at the next minor boundary.
The single pending draft is now **0.20.0**, including the previously prepared
compatible tooling notes. Cargo package versions remain 0.19.3 and the incoming
Metrics 0.7.3 lock selection is preserved. The source base is released
`1e451ba35613f2efde601073636c63ed97a91c06`, with uncommitted typed-start changes.
No bump, staging, commit, tag, push or publication is performed.

`TimerStart` replaces independent lifetime/optional-schedule constructor inputs
across all direct registration policies. The [design owner](../design/0.20-typed-registration.md)
records valid starts, atomic binding/capture ownership, removed missing-schedule
rejection and downstream mapping. Library/native fixtures, runtime/size cohorts,
PocketIC rejection endpoint/caller and public examples use the current contract.
`DeclarationLifetime` remains an inert observation; lifecycle helper signatures
and scheduling/recovery contracts remain the same by source inspection.

Approved development commands complete with locked/offline prepared inputs:

```bash
CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 make check
CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 cargo +1.88.0 check -p ic-timers --lib --all-features --target wasm32-unknown-unknown --locked --offline
```

The first compiles library and library test targets with Rust 1.99.0; the second
compiles the production library Wasm path at Rust 1.88.0. Neither executes tests
or proves the complete MSRV gate.

The maintainer separately authorizes focused native tests, doctests,
library/probe Clippy and probe compile checks with locked/offline prepared inputs,
excluding setup, PocketIC execution and release commands. Those checks pass:

| Subject | Compiler | Result |
| --- | --- | --- |
| Library all-target/all-feature native tests | 1.99.0 | 149 passed |
| Library all-feature doctests | 1.99.0 and 1.88.0 | Nine passed on each compiler, including eight compile-fail subjects |
| Library all-target/all-feature Clippy, warnings denied | 1.99.0 | Passed |
| Runtime/PocketIC all-target Clippy, warnings denied | 1.88.0 | Passed; no PocketIC test execution |
| Size-probe all-target Clippy, warnings denied | 1.88.0 | Baseline/Once/AfterCompletion/Watchdog selections all passed separately |
| Runtime-probe Wasm compile-only check | 1.88.0 | Passed with `timer-probe` profile |
| Size-probe Wasm compile-only checks | 1.88.0 | All four separate cohorts passed with `timer-probe` profile |

Every command uses `CARGO_NET_OFFLINE=true`, `RUSTUP_AUTO_INSTALL=0`, `--locked`
and `--offline`. Full commands and original logs are retained at
`target/development/typed-start.rFyIOguo/`, with the released base identity,
before/after source patches, Cargo input hashes and index entries. The patches
are identical across validation, and Cargo inputs/index entries are unchanged.
The graph selects incoming Metrics 0.7.3, Testkit 0.34.5, Host 0.14.9 and
PocketIC 16.1.0. These results belong to the uncommitted typed-start source,
not the released 0.19.3 commit alone or the forthcoming Metrics 0.8 adoption.

No setup, PocketIC execution or release command runs. Native substitutes and
probe compilation do not establish IC trap/rollback or recovery qualification.
Document links and diff checks pass. No measured Wasm, heap or instruction delta
is claimed. The existing complete release gate remains required before delivery;
frozen 0.19.1 qualification cannot stand in for it.

Canic is notified in [its existing timer-family owner](https://github.com/dragginzgame/canic/issues/33#issuecomment-6127168880).
Its current dirty direct-registration adapter needs the argument update upon
adoption. Jobs uses unchanged lifecycle helpers; all linked owners still select
one exact Timers identity. Sibling files/pins remain separately owned and are
not mutated by this preparation.
The [focused qualification follow-up](https://github.com/dragginzgame/canic/issues/33#issuecomment-6127448281)
records the new native/doctest/Clippy/probe compile results separately from
publication and IC acceptance.

## Prepared 0.21 first lifecycle reconciliation

The maintainer approves a semantic hard cut for fresh `reconcile_*` calls:
publish callback authority into the owner slot only after the requested initial
state succeeds, and retire the exact failed creation before returning an error.
Later failure on an existing slot preserves its callback and failure observations.
The [design owner](../design/0.20-typed-registration.md#prepared-021-lifecycle-creation)
records cleanup ordering, unused-factory destruction, counter identity, caller
implications and prepared native/IC subjects. Public signatures stay unchanged.
The one undated 0.21.0 draft also includes the separately owned Metrics 0.8 cut.

The source base is released 0.20.0
`01031c912058089756aed87fafa4918476c12bd4`, with incoming maintainer root
requirement/lock changes selecting Metrics 0.8.0. The selected graph retains
Testkit 0.34.7, Host 0.14.10 and PocketIC 16.1.0. No Cargo or lockfile mutation,
staging, commit, tag, push or publication is performed by this preparation.

Prepared library compile-only checks pass with `CARGO_NET_OFFLINE=true` and
`RUSTUP_AUTO_INSTALL=0`:

```bash
cargo check -p ic-timers --all-targets --all-features --locked --offline
cargo +1.88.0 check -p ic-timers --lib --all-features --target wasm32-unknown-unknown --locked --offline
```

The first compiles library test targets on Rust 1.99.0 without executing them;
the second checks the production library Wasm path at Rust 1.88.0. Source,
input hashes, index entries, commands and original logs remain at
`/tmp/ic-timers-021-reconciliation.sFVjKHjS/`.

The maintainer explicitly approves focused native tests, doctests, library/probe
Clippy and probe compile checks with locked/offline prepared inputs, excluding
setup, PocketIC execution and release commands. All selected checks pass:

| Subject | Compiler | Result |
| --- | --- | --- |
| Library all-target/all-feature native tests | 1.99.0 | 154 passed, including the five new lifecycle subjects and existing report/metrics boundaries |
| Library all-feature doctests | 1.99.0 and 1.88.0 | Nine passed on each compiler |
| Library all-target/all-feature Clippy, warnings denied | 1.99.0 | Passed |
| Runtime/PocketIC all-target Clippy, warnings denied | 1.88.0 | Passed; no PocketIC test execution |
| Size-probe all-target Clippy, warnings denied | 1.88.0 | All four separate feature selections passed |
| Runtime and four size-probe Wasm compile-only checks | 1.88.0 | Passed with `timer-probe` profile |

Every focused command uses `CARGO_NET_OFFLINE=true`, `RUSTUP_AUTO_INSTALL=0`,
`--locked` and `--offline`. The receipt retains complete commands, original logs,
input hashes and the identical before/after checked source patch. Cargo inputs
and real index entries remain unchanged. No setup, PocketIC or release command
runs. The new IC initial-reconciliation rejection subject and all existing
recovery/policy cohorts still need the complete user-operated release gate.
Compiler checks and native fault injection do not prove IC rollback. No Wasm,
heap or instruction delta is measured.
Canic has the [semantic cut notice](https://github.com/dragginzgame/canic/issues/33#issuecomment-6128606209);
its source/pins remain separately owned.

### PocketIC fixture clock repair

The maintainer-operated release attempt at prepared commit
`48299c0c23765ce89445a57727364228388988fc` passes 15 watchdog subjects but fails
the new initial-reconciliation subject before canister creation. Its absolute
clock reset to one second after the Unix epoch is earlier than the instance's
current `1620328630000000001` ns. PocketIC returns
`SettingTimeIntoPast((1620328630000000001, 1000000000))`; no timer request is
executed by that subject. This is a fixture error, not new runtime failure evidence.

The fixture now advances the existing clock by one second. It establishes
positive time for the intended `now + u64::MAX` rejection without moving time
backward; all three policy assertions, inventory/capture checks and later-round
checks remain. The library and canister probe are unchanged. Rust 1.88.0
runtime/PocketIC probe Clippy passes with locked/offline inputs. The original
failure log, exact prebuilt probe hash, input hashes, checked host-test patch and
logs remain at `/tmp/ic-timers-021-clock.l0OCwT2E/`. Cargo inputs, the prebuilt
probe and index entries are preserved. Approval for only the corrected PocketIC
subject is requested separately; no replay result or complete release acceptance
is claimed yet.

## 0.21 delivery and issue 50 acceptance

The maintainer reports **0.21.0 live**, committed/tagged/pushed at
`a6b518722438d2f7e2dccd7ccfef07debc35b5b3`. The local `v0.21.0` resolves to
that commit. Its completed standard direct-release record binds the original
validation source to `854e85f0a5d3129be3aa545352cfae4b2073ce12` and the delivered
tree to `03d8ef1a20ee78aa71d815d3f97bfc681b8d2544`. Source-to-release differences
are only normal Cargo/lock version advancement and changelog finalization.
The retained runner permits preparation only after the complete `release-verify`
gate succeeds with unchanged source; this supplies maintainer-operated acceptance
for native CI/MSRV/probe lint, watchdog/recovery and policy-cohort PocketIC gates.
No successful raw gate log or agent PocketIC replay is fabricated. The old
clock-reset attempt remains failed in its original log; the repaired fixture is
in the validated source. The pending request to replay it is no longer needed.

Delivered graph: Metrics 0.8.0, Testkit 0.34.7, all four Host 0.14.10 and
PocketIC 16.1.0, with Shared 0.9.2 at 67/35/11 files. The earlier focused
qualification remains separately bound to its prepared source. This closes
[#50](https://github.com/dragginzgame/ic-timers/issues/50#issuecomment-6129282698)
at the named delivered graph. The tag [run 38283128868](https://github.com/dragginzgame/ic-timers/actions/runs/38283128868)
passes; full main [run 38283128647](https://github.com/dragginzgame/ic-timers/actions/runs/38283128647)
is queued at inspection. No new complete hosted/macOS acceptance follows from
the local receipt or tag result. The inspected receipt, exact delivered Cargo
files and metadata-only release diff remain at
`/tmp/ic-timers-shared097.Oh8uAbQu/`.

## Prepared 0.21.1 contributor graph

The incoming maintainer catalog/lock selects published Testkit 0.35.0, sourced
from `ad5bdfa5221ec25743fbc56e028fb958f2a6a558`, with Host artifacts/fs/process
0.15.1 and unchanged PocketIC 16.1.0. Probe callers consume Testkit's maintained
PocketIC surface; none uses its removed generic artifact or Host-tools re-export.
No direct IC Build/Host dependency or additional catalog is introduced.
Locked/offline metadata resolves all four maintained members with inherited
direct dependencies, one Timers identity and one Metrics 0.8.0 package.
The lock graph shrinks from 299 to 288 packages by retiring unused build-tool
dependencies; this is a graph count, not a measured build-time or Wasm saving.

The paired [Shared 0.9.7 refresh](shared-tooling-history.md#prepared-shared-tooling-097)
retains the current roots and 67/35/11 payloads. Integrity and syntax pass, and
incoming Cargo files, pins and real index entries match saved inputs. Rust
product source and probe imports are unchanged. No new native/probe execution,
lint, PocketIC or setup result is claimed for this later graph. Earlier 0.21
acceptance does not qualify these incoming contributor dependencies.

Run **`make install-tools`** explicitly before validation to refresh the common
Cargo build-policy identity and the selected Testkit CLI/server through their
existing owners. Setup is user-owned and is not performed by this preparation;
offline checks neither repair old selections nor fall back to them. Undated
0.21.1 notes record repository-only work, normally bundled into the next
code-bearing release. An explicitly chosen maintainer tooling patch still
retains the full release gate. Cargo versions remain 0.21.0; no staging, commit,
tag, push or publication is performed.

## 0.21.1 delivery and prepared 0.21.2 tooling

The maintainer's completed direct-release receipt records source
`f935c706cfdb7e2554b0a2a95b06763efcc009c9`, delivered tree
`a971b09581c1433747caae7a20195cdd30bcd9b2`, and release commit
`a97dcb4b5f091d98afd35038a4cbeb15fba1d7d6` (0.21.1). That source contains the
neutral-cwd Make adapter and Shared 0.9.7/Testkit 0.35 contributor graph. The
completed standard receipt supplies owner-operated full-gate acceptance; it does
not replace or erase the original failed fixture evidence. Exact-source hosted
[run 38288927727](https://github.com/dragginzgame/ic-timers/actions/runs/38288927727)
and [run 38288927426](https://github.com/dragginzgame/ic-timers/actions/runs/38288927426)
pass at 2026-10-13 readback. These exact-source results remain separate from
newer tooling preparation.

The [0.9.10 refresh](shared-tooling-history.md#prepared-shared-tooling-0910) is a later
repository-only batch under undated 0.21.2 notes, separate from that acceptance.
It supersedes the earlier prepared 0.9.8 selection and removes the Make adapter
because the canonical fixture now isolates its compiler context and creates no
synthetic commits. The new reporting rule is explicitly selected as a governance
root. All user-operated gates remain; no Rust API/runtime, package-version,
Cargo-lock or workflow changes. Snapshot, syntax and diff inspection do not
qualify new fixture behavior; execution remains with the maintainer. No setup is
required solely by this refresh because the Cargo installer/build-policy identity
and executable pins are unchanged.

## 0.21.2 delivery and prepared 0.21.3 tooling

Release 0.21.2 is committed/tagged/pushed at
`1065512b6d6d231aa8cad25e3a668b1a65c99b51`; the local annotated tag resolves to
that commit and GitHub reports matching push runs. The completed standard direct
receipt binds owner validation to source `d3d9ecad898e42bf0c60c3ccce1f876e1362bbae`
and delivered tree `1a15c83ee63313da0a64ffc130ff4c8c8ca9f105`. Source-to-release
changes are only workspace Cargo/lock versions and changelog finalization.
Delivered selections are Shared 0.9.10, Metrics 0.8.1, Testkit 0.35.1,
Host artifacts/fs/process 0.15.2 and PocketIC 16.1.0. Original failed fixture
evidence is retained; no successful raw log or agent replay is fabricated.
Hosted [run 38363085866](https://github.com/dragginzgame/ic-timers/actions/runs/38363085866)
passes and [run 38363085915](https://github.com/dragginzgame/ic-timers/actions/runs/38363085915)
is in progress at inspection. No complete new hosted/macOS acceptance is claimed.

The [Shared 0.9.13 refresh](shared-tooling-history.md#prepared-shared-tooling-0913) is later
repository-only preparation under undated 0.21.3 notes. The incoming maintainer
lockfile selects Metrics 0.8.2 and deranged 0.5.9; preserve those bytes rather than
restoring the delivered selection. This tooling refresh does not qualify that
later graph or change library source. Run **`make install-tools`** explicitly
before validation because the Cargo build-helper identity changed. Earlier
generations remain retained. Setup, behavioral/full gates and release execution
remain user-owned; no Cargo version, lock, index or release-ref mutation is made
by this preparation.

## 0.21.3 delivery and prepared 0.21.4 cleanup

The maintainer reports 0.21.3 live at
`18ca1de856643f316b79c4a22774f5b0c34c423b`. Its completed standard receipt binds
owner validation to `51ff94b4a2ff7f78f5280dc93a4ca961dd160417` and delivered tree
`435d71194f56b4513104d796c34e1dd9ee59c0cf`. Source-to-release changes are only
workspace Cargo/lock versions and changelog finalization. The delivered graph
selects Shared 0.9.13, Metrics 0.8.2, Testkit 0.35.1, Host artifacts/fs/process
0.15.2 and PocketIC 16.1.0. Earlier failed evidence is retained; no successful
raw full-gate log or agent replay is fabricated.

The later [callback ownership cleanup](../design/callback-delivery-ownership.md#prepared-0214-callback-ownership-simplification)
uses undated 0.21.4 notes with no public API or semantic cut. Rust 1.99.0 and
1.88.0 compile-only library/all-target/all-feature checks and the 1.88.0 Wasm
library check pass with prepared locked/offline inputs and
`RUSTUP_AUTO_INSTALL=0`. No test executes in these compile-only checks.
The maintainer separately authorizes focused native library tests and library
Clippy, excluding PocketIC, setup and release commands. On Rust 1.99.0,
`cargo test -p ic-timers --lib --all-features --locked --offline` passes all
154 subjects; `cargo clippy -p ic-timers --all-targets --all-features --locked
--offline -- -D warnings` passes. Both retain offline/toolchain environment
settings above. Existing capture cleanup, suspended-work and reconciliation
subjects retain their assertions. No size or instruction delta is measured.
Commands/logs and saved source/Cargo/index inputs stay at
`/tmp/ic-timers-complexity0214.CsNLubA0/`; the full user-operated release gate
still owns native/IC lifecycle and cohort acceptance for the new source.

## 0.21.4 delivery and prepared 0.21.5 tooling

The maintainer reports 0.21.4 live at
`44935d6188a0da82314e00a56f5f68a9d0c59638`. The completed standard receipt binds
owner validation to source `903a548263ea2bfdd1eca395c63703dac31fee2c` and delivered
tree `cc2bec1f20ba29516dd7d270005b5dd521007799`. Source-to-release changes are
only Cargo/lock workspace versions and changelog finalization. This supplies
owner-operated full native/MSRV/probe, watchdog/recovery and cohort acceptance;
no successful raw log or agent replay is fabricated. Earlier focused tests use
their original source/graph and are not re-labelled as execution of later inputs.

Delivered graph selects Metrics 0.8.3, Testkit 0.36.0, Host artifacts/fs/process
0.16.0, PocketIC 16.1.0 and Shared 0.9.13. Callback storage and lifecycle-slot
cleanup are delivered without another public API cut. No matched before/after
footprint or instruction result is available, so no saving is claimed.
Hosted [run 38375894239](https://github.com/dragginzgame/ic-timers/actions/runs/38375894239)
is in progress and [run 38375894348](https://github.com/dragginzgame/ic-timers/actions/runs/38375894348)
is queued at inspection; no complete new hosted/macOS acceptance is claimed.

The later [Shared 0.9.14 refresh](shared-tooling-history.md#prepared-shared-tooling-0914)
is repository-only work under undated 0.21.5 notes. It changes CI log inspection,
not runtime behavior or executable setup identity. Cargo inputs, tool pins and
real index entries are preserved. No new product code, tests, lint, setup or
release execution is added; the focused 0.21.4 permission does not cover these
new helper bytes. The complete release gate remains maintainer-operated.

## Actual tag admission and regression ownership

The tag-truth job retains workspace/event-tag equality, exact annotated tag
admission, main reachability and the complete locked graph. It checks all three
actual snapshot receipts directly. It does not dispatch the release-check
regression roster; the focused actual-workflow fixture remains in that roster,
owned by PR/main and the complete release gate. Existing tool setup and failure
collection/upload remain unchanged. Main reachability and tag success do not
prove completion of the independent native/full source qualification.


## Complexity batch delivery — 2026-11-01

The ten-item complexity batch from preparation source
`54028ab5a7e41539348b0975e7a27573026a13ab` is delivered as Timers 0.35.4 and
Jobs 0.33.4. Their annotated tags identify `14917d09e7a55c16eda1b7d739f8213ac9d4209e`
and `96f2d94e5af16ffb31c270d6b94a400caf917423` respectively. Timers was published first;
Jobs followed against that supplier version. Both canonical ledgers are finalized.

Both standard full release gates passed, including combined CI and the installed
35-subject timer/runtime and policy-cohort gates. Final Jobs recovery passed all
11 subjects on original, O3, Os and Oz Wasm. This is Linux x86_64 evidence;
hosted native macOS qualification remains an independent asynchronous observation.

Registry downloads match retained Cargo uploads and their embedded release VCS
identities: Timers SHA-256 `ed9bf73e5cab8dff71c5b0b922cd61e587aa7212993e7c71929d8c060f61bf27`;
Jobs SHA-256 `0285cc4debb0076968538599e96224febea2382b75fe2ed3454c8fd96b69596a`. Complete journals, preparation source/locks,
original qualification artifacts and registry/publication receipts remain under
`/tmp/ic-scheduling-complexity-delivery.y314l_zn/`. The
[current handoff](../../status/current.md) records custody and ongoing boundaries.

Later concurrent lockfile edits in the original checkout select Build/Host
0.73.5, Memory/Storage 0.60.7 and Testkit/Native 0.76.1. They are preserved locally,
uncommitted and outside these published source/graph receipts. The incoming and
rebased lockfiles are retained separately in the same delivery evidence.
