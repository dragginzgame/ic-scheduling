# ic-jobs

UTC cron, one-shot, fixed-rate and completion-relative scheduling with optional
Internet Computer execution. Both default features and default dependencies are
empty.

- `calendar`: five-field, minute-precision UTC cron through Saffron, plus interval
  schedules and bounded occurrence previews. No named time zones or seconds field.
- `execution`: enables calendar support and the `wasm32` Jobs adapter on ic-timers.
  Callbacks run in isolated Once messages with explicit skip-late or bounded
  catch-up policy. Jobs share Timers' bounded declaration and transport capacity.

Use `Schedule::parse("0 9 * * *")` for 09:00 UTC daily. The other constructors are
`once`, `fixed_rate` and `after_completion`; timestamps are Unix nanoseconds.
Binding takes a delivery policy and `RetryPolicy` (`NONE` disables retries).
Only an explicit retryable outcome or owner assertion of retry safety can retry.
Retries preserve occurrence identity and consume a bounded attempt budget.

Pause, resume, edits and terminal cancellation preserve owner precedence.
Accepted or uncertain effects cannot be cancelled away. Recovery requires an
exact capability and `Retire` or `RetrySafe` disposition; either leaves work paused.

Hosts own atomic persistence and the upgrade boundary. Schema 6 checkpoints
preserve settled/queued progress, retries and the latest outcome. Explicit
unresolved records preserve accepted attempt identity and reconstruct blocked
in a fresh runtime. Ordinary checkpoint export refuses unresolved work.
Neither upgrades nor elapsed time authorize replay. Hosts own older outcome
history and business-effect reconciliation; heap snapshots are not supported.

This is a breaking redesign of the 0.36 API and its storage formats. See the
[repository](https://github.com/dragginzgame/ic-scheduling-new) for design,
grouped changelogs and installed-canister qualification evidence.
