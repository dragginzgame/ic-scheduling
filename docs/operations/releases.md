# Scheduling releases

The product groups are `timers` and `jobs`; qualification packages never publish.
The canonical manifests own independent versions and `releases/` owns their notes.
Toko owns release sequencing, capture, recovery, explicit staging and atomic push.
The local hooks own the complete crate-specific gate and source-bound evidence.

Prepare Rust 1.100.0/Wasm, Node, cached dependencies, a PocketIC 16.0.0 server and
an Entropy repository containing the immutable revision required by its
qualification adapter.
Export absolute `POCKET_IC_BIN` and `ENTROPY_SOURCE` paths. Release setup fetches
both existing lockfiles with `--locked`, installs pinned tools and selects the
published `ic-cargo-metadata` 0.1.0 executable. Checks then run offline. Recovery only
admits saved tools; it does not refresh dependencies or repair locks.

The standalone renderer replaces the displaced Build 0.73.0 CLI selection,
resolving
[Tooling #8 — Supply a released successor for the displaced Cargo release metadata editor](https://github.com/dragginzgame/ic-tooling/issues/8).
Install package/bin `ic-cargo-metadata` 0.1.0 in release profile without features.
Toko's admitted absolute executable selection, bounded-input contract,
transaction/rollback ownership and independent Cargo graph checks are retained.

Runtime compatibility belongs to the package requirement and the selected
Runtime's own guard, not a Scheduling patch pin. Published Timers 0.46.3 /
Jobs 0.39.3 requirements admit `^0.23.14`; both development locks now select
0.23.17. The earlier publication composition checks covered 0.23.14, 0.23.15
and 0.23.16 with one matching Runtime/guard. Local Testkit 0.109.0 qualification
uses 0.23.17; it does not replace those registry publication observations.
Published Timers 0.46.0 is immutable and still requires exactly Runtime 0.23.14;
published 0.46.1 / 0.46.2 require exactly 0.23.15. Consumers blocked by those pins
can upgrade to Timers 0.46.3 / Jobs 0.39.3. Registry-only consumers compile for
native and Wasm with all three qualified Runtime patches and one matching guard.

For Timers 0.45.10/0.45.11 and Jobs 0.38.0/0.38.1, the maintainer waived the
Entropy journey. Their release gates require `POCKET_IC_BIN` but not
`ENTROPY_SOURCE`; all Scheduling-owned checks and archive checks remain required.
Ordinary `make qualify` and subsequent releases retain Entropy qualification.

`make qualify` runs formatting/tool admission, strict Clippy early, native tests,
Wasm documentation, canonical hook qualification, installed failure/lifecycle/resource checks, the isolated
Entropy journey and archive checks. The Toko validation runner retains failures.
Source tests and extracted archive tests have separate responsibilities. Cargo
1.100's temporary registry limitation is recorded in the candidate evidence.

Final-version archive admission selects the release group at the Cargo leaf:
Jobs packages and checks only `ic-jobs`, using the registry Timers supplier;
Timers packages both products and checks Jobs against its extracted supplier
archive. Full qualification and repository CI retain paired archives. Run
`node qualification/archive-scope-check.mjs` after changing this dispatch; it
checks the actual argv, reverse-consumer coverage and failure propagation with
substituted Cargo commands. It does not establish native or Wasm compilation.

Release verification selects scope using Toko's annotated-ancestor baseline
observer against the selected group's previous release. Only Jobs Rust source
and Jobs changelog changes admit the Jobs mapping. Missing baselines, shared
locks/manifests/tooling, unknown paths and every Timers release retain the full
gate. Timers changes affect Jobs as a reverse consumer. Repository CI remains
full and does not iterate over release groups.

The Jobs mapping selects `ic-jobs` at native/Wasm Clippy, test and documentation
leaves, plus the Once driver and Once/calendar probes. Disabled Jobs features
have no runnable tests; native/Wasm Clippy checks their compilation. It runs the
standalone calendar and complete installed Once suite, including all Jobs
recovery, checkpoint and resource scenarios. Independent executor, Timers
metrics-cost and Entropy journeys are excluded only for that admitted mapping.
The integrated Once driver still includes timer cases; it has no qualified
complete Jobs-only mode, so these checks intentionally remain.

Qualification receipts use Toko's atomic receipt writer and bind source, group,
version, date and destination. A retry archives and invalidates old success
before execution; failed retries and changed receipt bytes cannot authorize
preparation. Older frozen batches retain their original adapter. Run
`node qualification/validation-scope-check.mjs` for the local selection and
receipt boundary; its gate effects are simulated, not runtime qualification.

On the prepared Linux release host, the full Rust stage uses Toko's prospective
final-version source and exact-stage evidence. Timers and Jobs can share that
stage only when the projected source, command, environment, tools, compiler and
observed dependencies match. Custom Cargo/compiler overrides or ancestor Cargo
configuration retain ordinary execution, as do macOS and narrow Jobs validation.
Whole-gate receipts are never substituted for stage proofs. Preparation rechecks
the candidate projection and original successful proof and logs; missing or
changed evidence refuses admission. Restore the exact retained evidence or rerun
verification; never manufacture a success receipt.

Run `node qualification/rust-stage-check.mjs --real` explicitly on the prepared
Linux host. It executes the real Rust stage in a disposable repository, prepares
the supplier version, checks cross-group reuse, and exercises evidence loss,
corruption and candidate drift followed by recovery. The 2026-11-17 local run
observed 68.5 seconds for execution and 33.7 seconds for reuse including evidence
checks. These are stage observations, not an end-to-end delivery speed claim.

Installed Once, executor, metrics and Entropy journeys continue to execute for
each applicable release: they consume current source and external runtime inputs,
and have no qualified final-candidate stage evidence contract. Format, tooling
and metadata gates also remain fresh. Final archive checks run again after
version preparation, and registry-only publication admission remains separate.
This is the local adoption boundary for
[deployment validation #4](https://github.com/dragginzgame/ic-scheduling/issues/4);
it does not change uncertain-upload reconciliation or authorize publication.

The standard commands are `make release-patch`, `release-minor` and
`release-major`, with an explicit `RELEASE_GROUP=timers` or `jobs`. A complete pair
uses `RELEASE_GROUPS=timers,jobs`, supplier first. All increments run the same gate.
The runner advances the canonical manifest version; pending changelog headings
must match that next version. Clean committed source is required before capture.
Only direct delivery to the selected GitHub branch is configured.

Publication uses the selected group's `make publish-check` / `make publish`;
`PACKAGE` may narrow only within that group. It requires clean tagged source and
retains exact-command evidence under ignored `target/publication`. Do not retry
an uncertain upload: inspect the retained intent and registry first. The standard
batch runner owns saved-attempt recovery (`release-batch-status`,
`release-batch-resume`, and explicit reconciliation).

The first release completed both groups and verified registry readback. Jobs
registry verification extracts outside the product workspace; the original
inside-workspace extraction was corrected after its pre-upload refusal. See the
[delivery record](../evidence/release-candidate.md) for the completed recovery.

The native CI matrix is Ubuntu 24.04 x86-64 and the GitHub-hosted macOS 15 ARM64 runner. It qualifies locked setup, format,
native tests, Wasm compilation, archives and metadata transactions. Full installed
PocketIC and pinned Entropy qualification currently runs on the prepared Linux
release host. CI runs one redacted repository-history secret scan, outside the
host matrix. A Linux pass alone does not claim macOS qualification.

Gitleaks setup uses the installed Toko CI tool installer and its reviewed version
and checksum catalog. Native and Entropy Cargo test selections use Toko's
nonempty-test runner, preserving Cargo failures and rejecting zero-test success.

CI setup, native qualification and history scanning run through `ci/setup.mjs`.
Failures retain sanitized final
64 KiB diagnostics and at most four explicitly reported Rust attempt logs of
64 KiB each, without following attempt symlinks or uploading tool stores. The
workflow uploads only `target/ci-setup/`, keeps artifacts for seven days and
links the artifact in its result. Missing logs, collection failures and upload
failures are reported separately; the original command failure remains fatal.
Command deadlines return status 124, send TERM to the command's process group,
then KILL after a two-second grace, including descendants that outlive the leader.
The deadlines precede the Actions step and job limits so ordinary command hangs
can finish collection and upload. A hard runner/job termination or cancellation
can still prevent upload; local rehearsal does not establish hosted custody.

Budgets selected on 2026-11-17 from five successful retained CI runs:
[43914912997](https://github.com/dragginzgame/ic-scheduling/actions/runs/43914912997),
[43955013371](https://github.com/dragginzgame/ic-scheduling/actions/runs/43955013371),
[43956784538](https://github.com/dragginzgame/ic-scheduling/actions/runs/43956784538),
[44059386656](https://github.com/dragginzgame/ic-scheduling/actions/runs/44059386656)
and [44061753917](https://github.com/dragginzgame/ic-scheduling/actions/runs/44061753917).
Durations use job start/completion timestamps, excluding queue time; these are
observations, not worst-case guarantees for future runner or download conditions.

| Job | Observed duration | Job limit | Command limits |
| --- | --- | --- | --- |
| Native Ubuntu 24.04 | 177–217 s | 20 min | Setup 8 min; qualification 8 min |
| Native macOS 15 | 213–306 s | 20 min | Setup 8 min; qualification 8 min |
| Repository secrets | 5–7 s | 5 min | Scanner setup 60 s; history scan 60 s |

Across these samples, native setup peaked at 159 s and qualification at 139 s.
The eight-minute phase budgets allow about three times those observations.
Actions step backstops are nine minutes for native commands and two minutes for
scanner commands; checkout, evidence upload and custody reporting each have a
one-minute limit. Revisit the budgets when maintained CI work or observed timings
change, keeping command deadlines below hard limits and leaving collection time.

Run `node qualification/setup-check.mjs` when changing this boundary. Its offline
failure rehearsal covers redaction, hidden attempt selection, missing output,
symlink refusal, truncation, collector failure, successful completion and a hung
command with a TERM-resistant descendant. Native CI runs the rehearsal on both
hosts; local Linux results alone do not qualify macOS signals or hosted upload.

The first hosted macOS rehearsal at source `d7a8158` exposed a fixture that
discarded a large piped write through `process.exit`. The wrapper retained and
[uploaded its sanitized failure log](https://github.com/dragginzgame/ic-scheduling/actions/runs/44067540364/artifacts/14320149254),
and the custody step reported that artifact. The fixture now uses `exitCode` so
its write drains. The [follow-up CI run at `31d814e`](https://github.com/dragginzgame/ic-scheduling/actions/runs/44069071026)
passed both native hosts and secret scanning, including the diagnostic/timeout
rehearsal. The earlier artifact proves a hosted failure upload, not a completed
cancellation/timeout artifact-retrieval exercise.
