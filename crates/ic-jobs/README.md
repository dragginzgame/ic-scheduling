# ic-jobs

UTC cron, one-shot, fixed-rate and completion-relative scheduling with optional
Internet Computer execution. Both default features and default dependencies are
empty.

- `calendar`: five-field, minute-precision UTC cron through Saffron, plus interval
  schedules and bounded occurrence previews. No named time zones or seconds field.
- `execution`: enables calendar support and the `wasm32` Jobs adapter on ic-timers.
  Callbacks run in isolated Once messages with explicit skip-late or bounded
  catch-up policy. Catch-up discards excess backlog after a bounded batch.
  Jobs share Timers' 64 declarations, eight transport flights and four submissions
  per control turn; each Job retains one declaration.

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
