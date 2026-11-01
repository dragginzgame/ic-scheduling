# Current handoff

## Active batch

The user authorized three improvements: repair recovery artifact custody, add a
read-only release-readiness command and qualify a concrete Canic-owned Jobs queue.
All three are implemented and qualified. The first two are delivered in `77fc661`.
Canic source is committed and pushed at
[`4759bec`](https://github.com/dragginzgame/canic/commit/4759beca60e0923cf91a16991541ab03c119d1ac),
branch `codex/jobs-application-qualification`, from delivered Canic 0.114.0 at
`e753785953e6805461ac337868c32163f65c3209`. The isolated clone is
`/tmp/ic-scheduling-improvements.VvCLTy/canic/`; normal sibling checkouts stay read-only.
Automatic approval review rejected Canic PR creation because it interpreted this
workspace's sibling-read-only rule as prohibiting that external write. Explicit
user approval for the prepared review request is pending; do not bypass rejection.
No new own version, tag, release or package upload is selected.

Unrelated supplier updates are arriving in the working Cargo.lock. Leave them
unstaged and untouched. Source-qualified root supplier selections remain those
committed at `6810b1d`; temporary observation of an unprepared working graph does
not qualify newer selections.

## Implemented root improvements

Recovery upload patterns now select the driver's `timer-qualification-*` directories
and the promoted `target/timer-cohorts.json` receipt, alongside original consumer
Wasm and logs. A fixture expands the actual YAML upload paths against runtime,
server, original-Wasm and cohort artifacts. Upload remains selected after failure.

`make release-status` reports group current/latest published/proposed versions,
exact-commit CI, local/remote annotated tag observations and blocked inputs.
`RELEASE_GROUP=jobs|timers` narrows it; `STATUS_OFFLINE=1` makes local observations
only, and `STATUS_JSON=1` emits structured output. It installs nothing and never
writes source, refs, remote state or release journals. Network/cache failures
remain unknown/blocked. Failed working metadata admission can project versions
from committed HEAD only, explicitly labelled. It is informational: the standard
release/publication owners still admit every real effect and run complete gates.

Focused evidence is `/tmp/ic-scheduling-improvements.VvCLTy/`: real consumer
Make/workflow routing with substituted tool effects, actual upload-selection
expansion, and readiness cases with real Git and substituted registry/CI/group
metadata. Readiness covers publication gaps, offline/unknown observations,
unprepared working metadata, dirty/occupied sources, invalid groups and preservation
of local/remote refs. Actual public-registry/CI/tag readback also succeeded.
Pinned workflow/shell lint, Perl syntax, formatting and local link checks pass.
No new product compilation, full local CI or live IC recovery was needed for
these root tooling-only edits. Hosted CI runs asynchronously against delivered source.

## Canic qualification completed

The existing managed lifecycle probe now uses published IC Jobs 0.33.2. Its
application owns eight immutable one-shot jobs, bounded Wire storage, memory
grants, owner/Fleet authorization, exact destination receipts and lifecycle
reconstruction through the existing Timers 0.35.0 runtime. Every retained record
passes Job::restore. Retained Running attempts become Uncertain; only exact
receipts resolve them. This is private application qualification, not a public
framework mount API or replacement of Canic's paid-operation lease protocol.
[#502](https://github.com/dragginzgame/canic/issues/502) remains a separate assessment.

Five native queue tests, probe/harness Clippy, the integration's Rust 1.91 check,
governed inventory/worker partition and the exact installed PocketIC case pass.
The installed case exercises an actual intent-writing trap before submission,
same-release wakeups, a lost reply after a committed destination effect,
uncertainty without redelivery, stale/duplicate receipt disposition, forged caller
refusal and corrupt restore without reset. Native cases cover immutable payload
mismatch, independent post-await work, fresh installation IDs and full-capacity
frames/receipts. The native Wire producer admits the same installed Wasm; all
seven endpoint identities and direction-specific budgets match. Formatting,
changed-document links and diff checks pass. Full Canic CI/release validation was
not run locally; no merge, version transaction or publication is selected.

The [Canic qualification guide](https://github.com/dragginzgame/canic/blob/4759beca60e0923cf91a16991541ab03c119d1ac/docs/features/jobs-qualification.md)
records the scope and exact governed selector. Source archive, original Wasm,
source/lock identity, build stamp, successful server output, schema and command
logs are retained outside target at `/tmp/ic-scheduling-improvements.VvCLTy/`.
Machine-readable custody is `canic-qualification-receipt.json`; original runtime
files and their checksums are in `canic-runtime-evidence/`. Installed Wasm SHA-256
is `d44298e9531dff37116a6518a0ebf729530b5a1219f1c80d46ae2057e6455bfd`.
Initial fixture failures and their logs are retained separately; passing evidence
belongs to `canic-installed-retained.log`, `canic-native-complete.log`,
`canic-final-checks-qualified.log` and `canic-contracts.log`.

The older private checkout `/tmp/canic-jobs-adapter.dPFVZ7FD/source` referenced by
prior handoffs is unavailable. No prior results or source are reused from it;
retained historical commit `fc14bbe1baa3c0fdc09b0550e25a1d3fdcd9d2d1` remains provenance.

## Workspace and recovery custody

Jobs stays 0.33.2 and Timers 0.35.0. Compatible drafts remain Jobs 0.33.3 and
Timers 0.35.1. All nine packages retain independent group versions, root catalogs,
full alias rosters and group ledgers; no root VERSION or shared workspace version.
Shared 0.27.23 at `619d194875369e0f72b45477b66adb430acbf6b7` remains an intact canonical
132-file snapshot. Jobs remains Serde-only, independent of platform/storage/runtime.

Prior Observability/CI selection evidence is `/tmp/ic-scheduling-observability-ci.fLZWMG/`:
243 native tests, Rust 1.91 native/Wasm Clippy, both complete consumer release
fixtures and 18 workflow-selection cases. #83 is delivered and closed. Code and
release gates remain complete; prose checks keep all three stable host job names
and explicitly do not claim native qualification. Prior live consumer evidence
stays `/tmp/jobs-recovery.2IcNgn/` with its original source/lock/tool selection.

Published Jobs `jobs-v0.33.2` / `4a76520d2265ab598113f00c4fbde0111268ffe2` and
Timers `timers-v0.35.0` / `e6bace3fa4d6a8fb6719e51b967aea3ae952cf80` retain complete
journals and registry/archive/VCS evidence. Timers 0.35 custody remains
`/tmp/ic-scheduling-timers035.602bpJ/`; never replay effects or rewrite artifacts.
Earlier evidence is linked from the
[prior handoff](https://github.com/dragginzgame/ic-scheduling/blob/6810b1d4ab55ab2a0da8dc301f29cbdda84d9c6a/docs/status/current.md).

Require explicit RELEASE_GROUP for release/resume/publication; direct delivery
remains origin/main and publication is separate. Preserve saved source, policy,
payload and receipts on recovery. All restores use Job::restore. Running/Uncertain
effects stay blocked until exact receipts or explicit owner disposition. Preserve
bounded JQW queue bytes, attempts, immutable payloads, rollback and allocation
ledger/queue custody. Logging stays heap-only at 32 KiB; Metrics reset neither
clears logs nor reuses IDs. Timers keeps the sole private provider, confined FFI
and one exact Platform guard. Rust floor remains 1.91. Native execution alone
never qualifies IC recovery. Sibling Timers source is provenance, not a build
input; migration custody stays `/tmp/ic-scheduling-merge.7PRSi3Nq/`.
