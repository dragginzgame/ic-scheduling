# First-release scope: legacy capabilities plus accepted calendar work

Maintainer decision, 2026-11-15: retain capabilities present in legacy Timers/Jobs;
defer additional proposed capabilities. This supersedes earlier omission decisions
where they removed legacy behavior. It does not require old API names, a
compatibility facade, old storage formats or transplanting the old architecture.
Already accepted UTC cron, bounded catch-up and optional features remain in scope.

## Crates.io completion target

Completed on 2026-11-15: Timers 0.45.9 and Jobs 0.37.9 are published.
See [qualification and delivery](../evidence/release-candidate.md). The ordered
checklist below records the completed first-release scope. Subsequent releases
Timers 0.45.10 and Jobs 0.38.0 are also published; schema 7 descriptions below
refer to Jobs 0.38.0, not the Jobs 0.37.9 artifact. Entropy qualification was
explicitly waived for these two subsequent versions; all Scheduling gates passed.

Maintainer direction, 2026-11-15: finish this rewrite soon and publish both crates
to crates.io. This replaces the GitHub-only delivery target, without waiving the
required legacy capabilities or failure guarantees. The completed sequence was:

1. **Jobs durable recovery:** persist exact accepted/uncertain attempt identity
   and recorded outcomes; reconstruct unresolved work blocked, with explicit
   disposition. Qualify the acceptance/factory-trap boundary and actual upgrades,
   including stale results and owner edits. Live Retire/RetrySafe recovery and
   pending retry checkpoints are already implemented.
2. **Timer lifetime and diagnostics:** implement explicit removal and transient
   lifetime behavior without stale handles/completions affecting replacement
   registrations. Release captures outside registry borrows. Provide the bounded
   inventory/failure observations needed for legacy parity, without recreating
   the old diagnostic framework.
3. **Freeze and qualify:** finalize public APIs and host persistence contracts;
   run the required native, Wasm, installed failure/lifecycle and resource checks
   against the candidate. Reconcile historical open-gate prose with actual
   evidence. Include existing Entropy adapter evidence and any changes that
   invalidate it. Actual adoption in sibling repositories is separate work.
4. **Package and publish:** verify current registry state and first-party supplier
   dispositions, complete package metadata and registry dependency versions,
   verify packaged artifacts and clean-checkout builds, finalize grouped
   changelogs and publish Timers before Jobs. Verify registry artifacts and
   GitHub delivery. Qualification packages remain unpublished.

The first published rewrite versions are Timers 0.45.9 and Jobs 0.37.9. The public Cargo
sparse index checked on 2026-11-15 lists latest non-yanked stable versions 0.44.3
and 0.36.3 respectively before this publication. Both candidates are minor
series changes relative to the published APIs. Intermediate GitHub checkpoints
are not crates.io releases and do not require another version increment by
themselves.

Product registry metadata, versioned local dependencies and packaged README files
are prepared. Native archive verification passes; the extracted archive pair
compiles for Wasm. Cargo 1.100 temporary-registry verification has a recorded
limitation; registry-only Jobs verification passed against published Timers.
Release adapters, both deliveries and registry readback are now verified.

Legacy performance comparisons, optional dependency refreshes, new calendar
features and migrations in consumer repositories were excluded from the first
release gate. Required resource/failure checks remain part of subsequent releases.

## Capability boundary

Evidence is the clean local legacy Scheduling checkout at
`0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e`, especially Jobs' README, `policy.rs`,
`job.rs` and Timers' public exports. This is source evidence, not deployment proof.

| Capability | Release disposition |
| --- | --- |
| Retained Once and pre-armed watchdog execution | Implemented; preserve failure/ownership guarantees |
| Five-field UTC cron | Accepted new requirement; implemented through Saffron |
| Latest-due coalescing for Jobs | Not in legacy Jobs; defer. Watchdog tick coalescing remains implemented |
| Fixed-rate elapsed intervals | Implemented through `Schedule::fixed_rate` |
| One-shot Jobs | `Schedule::once` implemented in 0.37.7 using shared admission, delivery policy and recovery |
| Jobs recurrence after completion | `Schedule::after_completion` added in 0.37.8; shared cursor preserves completion-relative deadlines |
| Bounded retries/backoff with stable effect identity | 0.37.9 integrates explicit retry-safe outcomes and pending retry checkpoints; unresolved records now reconstruct blocked with exact attempt identity |
| Persisted Running/Uncertain records and explicit disposition | Explicit unresolved export/restore and Retire/RetrySafe dispositions implemented; installed running/trapped upgrades pass |
| Pause/resume, reschedule and pending cancellation | Implemented; 0.37.8 adds terminal cancellation with checkpoint reconstruction and unsettled-work refusal |
| Durable attempt correlation and recorded outcomes | Attempt identifies revision/time/number; schema 7 retains the latest normal result or recovery disposition |
| Completion-relative timer recurrence | `OnceNext::After` added in 0.45.7 through the existing Once provider; no separate registration family |
| Legacy transient timers and explicit removal | Implemented with checked incarnations, unsettled-work refusal and installed capture-lifetime checks |
| Legacy inventory/failure diagnostics | Bounded sorted inventory and declaration failure observations implemented behind metrics, without control authority |
| Named zones, seconds cron, holiday calendars, parallel runs per job | Additional proposals, not legacy requirements; defer |

Fixed-rate recurrence uses the shared execution, catch-up and checkpoint paths.
From 0.37.7, binding selects its original first deadline, including zero and
overdue instants; the explicit policy controls delivery. The checkpoint distinguishes
unstarted progress from settled timestamp zero. Cron still starts after now.
One-shot Jobs use the same first-deadline contract and become idle after settlement
or policy-based skipping. Completion-relative Jobs preserve their next deadline
through pause and reconstruction using the settled cursor. Legacy catch-up
could retain an entire backlog; the new bounded/discard-excess policy is an accepted
redesign, not an identical behavioral replacement.

Schema 7 records Active, Paused or Cancelled control state. `Job::cancel` discards
unaccepted selection, pending retries and remaining catch-up work; it refuses running or uncertain
work. Cancellation survives reconstruction and cannot be resumed or edited.
The underlying timer declaration and captures remain retained; removal is
available through Timers; Jobs deliberately retains its declaration.

Persisted unresolved work must restore blocked, retain exact occurrence/attempt
identity, and require explicit result or retry-safety evidence. Neither timeouts
nor upgrades authorize replay. Hosts still own storage, payloads and lifecycle.
Keep heap-snapshot support separate from validated record reconstruction.

Consumer repositories remain read-only and may be in flux. Finish independently
testable crate behavior here; actual consumer adoption and production resource
budgets remain separate gates. This expands the remaining work beyond the prior
estimate based on a reduced first-release scope.
