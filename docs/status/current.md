# Current handoff

## Complexity review implemented — 2026-11-01

All ten recommendations are implemented in two local batches on base
`af71b4b903f3cdc231afad5a56857f3c3692c9da`. The first batch covered 1, 4 and 9;
the continuation covers 2, 3, 5, 6, 7, 8 and 10. The user has now authorized
release and crates.io publication of both groups: Timers 0.35.4 first, then
Jobs 0.33.4, through their standard full gates. Preparation has not yet changed
Cargo versions or the lock. Delivery uses an isolated checkout at
`/tmp/ic-scheduling-complexity-delivery.y314l_zn/source/`; the incoming working
source and its custody manifest are retained in that parent evidence directory.

Ordinary completion now distinguishes scheduling, stopping and cancellation.
Watchdog requests resolve one schedule before mutation, retaining lazy cadence
overflow checks for coalesced/awaiting requests. Provider bindings explicitly
stage or commit arm observations. Pending-command priority, inactive reasons,
abandoned-scheduler repair, checked failures and capture cleanup are preserved.

One cohort catalog generates selection, typed measurement slots and strict JSON
fields. Measurements are indexed by Cohort; report schema 2 and field names stay
unchanged. A pending build-context builder admits the first whole successful
identity and rejects mismatches without partial updates. Only a complete context
can become a receipt. The named probe checkpoint retains the original tuple's
field order/bytes and lifecycle-owned reconstruction.

Release fixtures now have isolated metadata, interruption, recovery and
publication phases with scoped inputs. Their modes and negative checks remain.
The fixtures use disposable local Git remotes and substitute full gates/uploads;
passing them is not an actual release, publication or full CI qualification.

The private Jobs canister uses typed Wire attributes with explicit IDs, original
argument tuples and budgets; init retains its bounded lifecycle decoder.
Oversized private RPCs now produce CanisterCalledTrap instead of the former
guard rejection, with unchanged-state assertions. Installed runtime/cohort
subjects live under `crates/ic-timers-pocketic/tests/runtime/`, separate from
host-only utility tests. `make test-timer-tooling` starts no IC server. Watchdog
receipts exclude utility subjects. Ordinary probe observations use one snapshot
branch with explicit defaults. Published library APIs and stored queue formats remain intact.

## Current validation and retained evidence

Rust 1.91 native Clippy passes for the four changed Timers/tooling/probe packages;
Wasm Clippy passes for Timers and the runtime probe. The Jobs consumer also builds
on Rust 1.91/Wasm against this Timers source. All 144 focused native tests pass:
44 registry, 86 runtime, 10 driver/report, one CLI and three protocol subjects.
The protocol check proves named-checkpoint bytes match the prior tuple. Report
checks retain missing/unknown/duplicate-field refusal and atomic identity admission.
Provider and native/Wasm dependency checks, ShellCheck, formatting, documentation
links and whitespace checks pass. The shortened watchdog request flow no longer
needs its previous function-size lint exception.

Installed Linux x86_64 qualification passes seven selected Timers subjects,
the cohort measurement/comparison subject and all 11 Jobs recovery subjects.
These cover idle/cancellation, awaiting completion, immediate scheduling,
abandoned-scheduler repair, inter-stage cancellation, provider churn and upgrade
reconstruction, plus Jobs rollback/receipts/restore boundaries. The comparison
uses the retained first-batch report with matching compiler/server/lock/recipe
context. Once, AfterCompletion and Watchdog Wasms each decrease by 137 bytes;
ordinary work and watchdog start/duplicate instruction costs increase slightly.
Exact deltas remain in the comparison log; no general speed improvement is claimed.
Both Jobs and Timers pass local metadata and recovery/publication fixture modes.
Full CI, complete watchdog selection, optimized Jobs variants and macOS checks
were not selected for this development batch.

Continuation evidence: `/tmp/ic-scheduling-complexity-rest.7bSQcO3E/`. It retains
the incoming first-batch source/diff, final working archive/diff and lock, logs,
receipt-matched original Wasms/build stamps, baseline/current cohort reports and
passing IC diagnostics. Original live evidence remains
`/tmp/jobs-recovery.WQTGhi/`, `/tmp/timer-qualification-2607204-0/` and
`/tmp/timer-qualification-2661835-0/`. Keep original artifacts instead of replacing
them with later builds. The initial tool-selection/setup and lint failures are
retained beside their successful retries. Managed metadata-tool setup required
shared-store write access and then reused the locked tool cache successfully.

First-batch evidence remains `/tmp/ic-scheduling-simplification.ut9fdf13/`,
including its handoff in the incoming archive. Those older runtime artifacts do
not qualify the later Timers changes. Successful release fixtures clean their
disposable repositories under the existing fixture contract; their command logs
remain in the continuation evidence. The separately authorized delivery now
runs full gates and publication; inspect its saved journals before retrying an
uncertain effect. Do not infer completion from the earlier focused checks.

## Completed delivery and boundaries

Jobs 0.33.3 and Timers 0.35.3 are already published with finalized ledgers.
Do not repeat uploads or rewrite those tags. Full delivery identities and custody
remain in the
[committed delivery handoff](https://github.com/dragginzgame/ic-scheduling/blob/af71b4b903f3cdc231afad5a56857f3c3692c9da/docs/status/current.md)
and `/tmp/ic-scheduling-platform-delivery.e3mlrmqt/`, including its release journals.
The graph and intact Shared 0.27.23 snapshot have not changed during cleanup.

Keep nine catalog/group members, explicit RELEASE_GROUP, origin/main direct
release delivery and separate authorized upload. Jobs stays Serde-only and
consumer-owned; every restore uses Job::restore and uncertain effects stay blocked.
Preserve queue/payload/attempt custody. Logging stays heap-only at 32 KiB; Metrics
reset does not clear logs or reuse IDs. Timers owns the sole private provider,
confined FFI and matching Platform guard. Rust floor is 1.91; native execution
alone never qualifies IC recovery. Sibling repositories stay read-only.
Separate Canic [PR #722](https://github.com/dragginzgame/canic/pull/722), source
`4759bec`, retains `/tmp/ic-scheduling-improvements.VvCLTy/`; no Canic delivery is selected.
