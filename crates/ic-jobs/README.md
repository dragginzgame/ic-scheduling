# ic-jobs

UTC cron, one-shot, fixed-rate and completion-relative scheduling with optional
Internet Computer execution. Both default features and default dependencies are
empty.

- `calendar`: five-field, minute-precision UTC cron through Saffron, plus interval
  schedules and bounded occurrence previews. No named time zones or seconds field.
- `execution`: enables calendar support and the Jobs adapter on ic-timers.
  The API is available for native type-checking. Binding requires an initialized
  canister timer runtime. Callbacks run in isolated Once messages with explicit
  skip-late or bounded catch-up policy. Catch-up discards excess backlog after a bounded batch.
  Jobs share Timers' 64 declarations, eight transport flights and four submissions
  per control turn; each Job retains one declaration.
- `metrics`: enables execution and aggregate job diagnostics. Counters cover
  committed callback starts, started retries, lateness skips and each normal
  business outcome. A skipped dispatch does not count as a callback returning
  `NoWork`.

With `metrics`, `ic_jobs::metrics()` reads the counters after the first successful
binding. A host such as Canic can share its heap store by supplying an
`Rc<ic_jobs::MetricsStorage>` through `ic_jobs::initialize_with_metrics` before binding
or restoring jobs. Both readers see the same counters. Successful restore and
upgrade reconstruction never replay old observations. A normal upgrade resets
the metrics window independently of job progress.

The execution adapter accepts compatible Platform Runtime 0.23 patches from
0.23.21, sharing the single Runtime/guard selected with Timers and the host.

Use `Schedule::parse("0 9 * * *")` for 09:00 UTC daily. The other constructors are
`once`, `fixed_rate` and `after_completion`; timestamps are Unix nanoseconds.
Binding takes a delivery policy and `RetryPolicy` (`NONE` disables retries).
Only an explicit retryable outcome or owner assertion of retry safety can retry.
Retries preserve occurrence identity and consume a bounded attempt budget.

Pause, resume, edits and terminal cancellation preserve owner precedence.
Accepted or uncertain effects cannot be cancelled away. Recovery requires an
exact capability and `Retire` or `RetrySafe` disposition; either leaves work paused.

Jobs 0.38.0 uses schema 7. Jobs 0.37.9 uses schema 6 and a separate
unresolved-record wrapper; migration to the following checkpoint API is explicit.

Hosts own atomic persistence and the upgrade boundary. Schema 7 checkpoints
preserve settled/queued progress, retries and the latest outcome. Explicit
unresolved records preserve accepted attempt identity and reconstruct blocked
in a fresh runtime. Both exports return `Checkpoint`; `unresolved()` identifies
its optional attempt data and `with_unresolved` validates decoded data. Ordinary
checkpoint export refuses unresolved work and ordinary `restore` rejects it;
`restore_unresolved` requires it. Old schemas require explicit host migration.
Neither upgrades nor elapsed time authorize replay. Hosts own older outcome
history and business-effect reconciliation; heap snapshots are not supported.

This is a breaking redesign of the 0.36 API and its storage formats. It does not
provide the legacy standalone mutable Job metadata API or Serde record codec. See the
[repository](https://github.com/dragginzgame/ic-scheduling) for design,
grouped changelogs and installed-canister qualification evidence.
