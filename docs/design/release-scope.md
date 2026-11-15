# First-release scope: legacy capabilities plus accepted calendar work

Maintainer decision, 2026-11-15: retain capabilities present in legacy Timers/Jobs;
defer additional proposed capabilities. This supersedes earlier omission decisions
where they removed legacy behavior. It does not require old API names, a
compatibility facade, old storage formats or transplanting the old architecture.
Already accepted UTC cron, bounded catch-up and optional features remain in scope.

Evidence is the clean local legacy Scheduling checkout at
`0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e`, especially Jobs' README, `policy.rs`,
`job.rs` and Timers' public exports. This is source evidence, not deployment proof.

| Capability | Release disposition |
| --- | --- |
| Retained Once and pre-armed watchdog execution | Implemented; preserve failure/ownership guarantees |
| Five-field UTC cron | Accepted new requirement; implemented through Saffron |
| Latest-due coalescing for Jobs | Not in legacy Jobs; defer. Watchdog tick coalescing remains implemented |
| Fixed-rate elapsed intervals | Legacy requirement; implementation started in 0.37.6 |
| One-shot Jobs | `Schedule::once` implemented in 0.37.7 using shared admission, delivery policy and recovery |
| Jobs recurrence after completion | Legacy requirement; still to implement in Jobs |
| Bounded retries/backoff with stable effect identity | Legacy requirement; still to implement without retrying uncertain work implicitly |
| Persisted Running/Uncertain records and explicit disposition | Legacy requirement; unresolved-work restoration remains a release gate |
| Pause/resume, reschedule and pending cancellation | Preserve capabilities; current pause/edit exist, terminal cancellation needs completion |
| Durable attempt correlation and recorded outcomes | Legacy requirement; reconcile with the new checkpoint/context contract |
| Completion-relative timer recurrence | `OnceNext::After` added in 0.45.7 through the existing Once provider; no separate registration family |
| Legacy transient timers and explicit removal | Still required; earlier omission is no longer assumed approved |
| Legacy inventory/failure diagnostics | Preserve useful observable behavior through the new ownership model; current aggregate metrics are not full parity |
| Named zones, seconds cron, holiday calendars, parallel runs per job | Additional proposals, not legacy requirements; defer |

Fixed-rate recurrence uses the shared execution, catch-up and checkpoint paths.
From 0.37.7, binding selects its original first deadline, including zero and
overdue instants; the explicit policy controls delivery. Schema 3 distinguishes
unstarted progress from settled timestamp zero. Cron still starts after now.
One-shot Jobs use the same first-deadline contract and become idle after settlement
or policy-based skipping. Jobs recurrence after completion remains unfinished. Legacy catch-up
could retain an entire backlog; the new bounded/discard-excess policy is an accepted
redesign, not an identical behavioral replacement.

Persisted unresolved work must restore blocked, retain exact occurrence/attempt
identity, and require explicit result or retry-safety evidence. Neither timeouts
nor upgrades authorize replay. Hosts still own storage, payloads and lifecycle.
Keep heap-snapshot support separate from validated record reconstruction.

Consumer repositories remain read-only and may be in flux. Finish independently
testable crate behavior here; actual consumer adoption and production resource
budgets remain separate gates. This expands the remaining work beyond the prior
estimate based on a reduced first-release scope.
