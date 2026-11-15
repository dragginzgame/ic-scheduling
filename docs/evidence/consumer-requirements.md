# Consumer requirements and evidence

Observed 2026-11-15 in local working trees. Scope: Canic and Entropy; **Relay is
excluded by explicit maintainer instruction**. Other siblings were scanned for
Scheduling dependencies, including manifest aliases and Rust imports. No remote
branch, crates.io reverse-dependency census or production deployment was queried.
Absence here means no use found in this bounded local inventory, not no possible
external users. Generated/build artifacts and vendor trees were excluded.

[Source hashes and repository revisions](source-inventory.json) bind this review
to actual working files. A dirty checkout's HEAD alone does not identify those
bytes. The older Scheduling Canic integration guide is historical context, not
proof of a current Jobs consumer.

## Direct consumers

Maintainer clarification: Entropy is a stable integration target; Canic and other
repositories are being rewritten. Preserve the named requirements, but review
Canic's eventual API before migration. The [isolated Entropy migration](entropy-integration.md)
now compiles and passes its existing installed journey against the new provider;
the Entropy repository itself has not been changed.

| ID | Owner and source | Observed requirement | Authority that stays outside Scheduling |
| --- | --- | --- | --- |
| C1 | [Canic management engine](../../../canic/crates/canic-management/src/engine/mod.rs), `open`, `schedule`, `next` (around lines 103, 265, 251) | Two independently retained async one-shot registrations; absolute deadline or inactive; callback returns stop/next timestamp; reuse registration on repeated reconcile | Two-child durable journals, approvals, readiness, cycle reserves, management calls, result classification and upgrades |
| C2 | Same source, `schedule` callback and management `step` | Callback may await; separate worker messages; no automatic replay of an uncertain create/install effect | `observing` exclusion, committed phase, exact response evidence and explicit reconciliation |
| E1 | [Entropy buffered module](../../../ic-entropy/crates/ic-entropy-ic/src/buffered/mod.rs), `reconcile` (around line 112) | Retained watchdog at one-second cadence; inactive, cadence-scheduled, or absolute scheduled demand; idempotent shared initialization | Pool capacity, demand, retry ceiling/backoff, opt-out and host reconstruction |
| E2 | Same source, `drive` (around line 159) | Synchronous watchdog callback; launches consumer-owned migratory async refill; Continue/Stop/ScheduleAt decisions | Seed acquisition, generation check, cancellation guard, refusal recovery and re-reconciliation after the await |
| L1 | Both consumers' lifecycle paths | Reconstruct volatile declarations synchronously from current consumer state; initialization must not erase another owner's timers | Lifecycle entrypoints, stable storage admission, freshness and reconstruction ordering |

Canic's root catalog requests `ic-timers = "0.44.1"`; its management package
inherits that declaration. Entropy requests `0.44.0`, selected only through the
`buffered` feature. These are manifest requirements, not statements about the
version deployed by either consumer. The new library must remain optional for
Entropy users who do not select buffering.

### Exact exercised API surface

Canic imports `OnceDecision`, `OnceRegistration`, `OnceRunResult`,
`TimerCompletion`, `TimerIdentity`, `TimerSchedule`; it invokes
`initialize_runtime` and `reconcile_once`. Its callbacks ignore the context
argument. It uses `Stop` and `ScheduleAt`, and completion `no_work`/`success(1)`.
Its registration slots are retained and not mutated through separate handle APIs.

Entropy imports `TimerCadence`, `TimerIdentity`, `WatchdogReconcileState` and
`WatchdogRegistration`; it invokes `initialize_runtime` and `reconcile_watchdog`.
It uses Inactive/Scheduled/ScheduledAt and Continue/Stop/ScheduleAt. Its callback
also ignores the context. It uses `TimerCompletion::no_work()`.

Neither in-scope consumer directly requests after-completion recurrence, transient
remove-on-stop registration, context control capabilities, immediate watchdog
commands, an inventory export, full memory-page summaries or the failure-streak
query. This is permission to omit those from the *new design*, not permission to
silently change the old published library.

## Jobs finding

No `ic-jobs` declaration/import was found outside Scheduling's own package,
example and qualification workspace in the inspected local sources, after the
Relay exclusion. Current Canic management owns its own bounded phase journal;
it does not use Jobs. Entropy owns its own bounded refill model.

The previous [Jobs shadow experiment](../../../ic-scheduling/docs/reports/experiments/jobs-shadow-2026-11-15.md)
retained the complete Jobs contract. It reached 618 or 624 lines versus 676,
not the 30% target; the reducer version regressed the measured native workload.
This is evidence against starting from an API-compatible Jobs recode. It does
not establish that Jobs is defective or that deleting it from the old repository
is safe for unknown users.

**Original decision, superseded by maintainer direction on 2026-11-15:** the
observed consumers did not justify a Jobs package. The maintainer now explicitly
requires a new `ic-jobs` scheduler for upcoming use, permits breaking redesign,
and initially requested calendar and time-zone support. The subsequent maintainer
decision selects Saffron with five-field UTC cron only; named time zones and
seconds-enabled cron are outside scope. This is a new product requirement, not
evidence of an existing consumer. See the [Jobs design](../design/jobs.md).
Do not replace Canic's journals or Entropy's state with a generic job model.

## Safety requirements from the provider, not surface call counts

| ID | Evidence owner | Requirement |
| --- | --- | --- |
| S1 | [Provider FFI](../../../ic-scheduling/crates/ic-timers/src/platform/ffi.rs) | One global-timer export; self-only bounded dispatch endpoint; separate task messages; exact response/cleanup ownership |
| S2 | [Provider backend](../../../ic-scheduling/crates/ic-timers/src/platform/backend.rs) and [ownership observation](../../../ic-scheduling/crates/ic-timers/src/platform/ownership/mod.rs) | Bounded pending/flight tables; pressure deferral; genuine competing-owner refusal while admitting a fired one-shot's zero reset |
| S3 | [Runtime dispatch](../../../ic-scheduling/crates/ic-timers/src/runtime/dispatch.rs) | No registry borrow across user code or await; abandoned accepted work is not inferred successful; watchdog predecessor commits successor before work |
| S4 | [Reconciliation](../../../ic-scheduling/crates/ic-timers/src/runtime/registration.rs) | Fresh failure leaves slot empty; existing identity/kind/cadence mismatch refuses; repeat reconcile keeps the original callback; release captures outside borrows |
| S5 | [Current registry](../../../ic-scheduling/crates/ic-timers/src/registry/mod.rs) and [entry custody](../../../ic-scheduling/crates/ic-timers/src/registry/entry.rs) | Separate registration incarnation and dispatch generation; obsolete completions cannot control replacements; logical scheduling cannot claim an absent provider handle |
| S6 | [Metrics recording](../../../ic-scheduling/crates/ic-timers/src/snapshot/metrics/mod.rs) | Instrumentation is observation, not authority; use canonical Metrics arithmetic rather than hand-written saturation/aggregation |

These remain necessary even when a consumer does not import a corresponding
diagnostic type. Removing a rare failure test requires preserving its invariant
at an appropriate independent boundary, not merely showing a low call count.
