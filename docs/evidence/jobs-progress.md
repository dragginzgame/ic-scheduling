# Jobs occurrence progress — 0.37.2, 2026-11-15

Implemented a private progress core before wiring Jobs into Once. Saffron remains
the only calendar calculator; no timer transport, callback registry, recurrence
algorithm, business retry policy or persistence codec was added. The public
calendar preview API and off-by-default calendar dependencies are unchanged.

An occurrence is a schedule revision plus intended timestamp, to be combined
with the host's stable job key when forming a business identity. A separate
checked selection sequence distinguishes local delivery attempts. Cancelling
unaccepted selection and later selecting the same occurrence produces fresh local
authority without inventing a different business occurrence.

The core retains one selection, which may be queued, accepted or blocked by
uncertainty. Acceptance returns scheduled and actual start context. It must occur
before consumer factory invocation in the future adapter. It cannot accept early,
accept twice or select overlapping work. Normal completion retires the occurrence
and advances the exclusive cursor, independent of the consumer's business outcome.

Pause invalidates queued selection while preserving accepted/blocked identity.
Schedule edits increment the revision, retain the paused state and do not move
the existing settled cursor backward. Completion from an older running revision
releases its selection without replacing newer schedule progress or pause state.

The future adapter may block either accepted state or queued state surviving an
initial work-message rollback, but only from matching Timers recovery evidence.
Pause, resume and schedule edit cannot clear that barrier. Explicit owner
reconciliation retires the uncertain occurrence without replaying it; the adapter
must also reconcile Timers' exact current recovery token. This core does not grant
permission to retry an uncertain business effect or infer termination from time.
Exhaustion faults before revision/selection reuse and retains unresolved identity.

Six native tests establish the above transitions, including stale delivery after
reselecting the same occurrence, old-revision completion, both queued/accepted
uncertainty and backward progress admission. With the four existing calendar
tests, all ten Jobs tests pass. Commands:

```sh
cargo test -p ic-jobs --all-features --locked --offline
cargo clippy -p ic-jobs --all-targets --all-features --locked --offline -- -D warnings
cargo clippy -p ic-jobs --all-targets --no-default-features --locked --offline -- -D warnings
cargo clippy -p ic-jobs --target wasm32-unknown-unknown --all-features --locked --offline -- -D warnings
make fmt-check
```

Rust 1.100.0 and the existing product lock were used. No dependencies were added.
The module is private with a scoped non-test dead-code expectation until the
adapter uses it. No serialized representation, restoration protocol or public
recovery capability has been selected.

Required next proofs are selection-to-Once composition, initial factory trap
mapping, schedule/pause precedence across an actual await, supplier search bounds,
missed-run selection and host persistence/reconstruction. Local tokens still
require job-scoped authentication from the future binding layer. Native state
tests do not establish IC commit ordering or durable effect safety.

The maintainer deferred further runtime-cost investigation and comparison with
the legacy provider; that work remains open rather than blocking this Jobs state
increment. This is not release-ready execution or a consumer migration.

## Installed skip-late execution preview — 2026-11-15

Jobs now has an off-by-default `execution` feature, enabling calendar support and
wasm32-only dependencies on the current local Timers crate and Platform Runtime.
The default normal dependency graph remains empty; the standalone calendar Wasm
probe still has no IC imports and all eight calendar checks pass. No second timer
provider, transport queue, calendar implementation or compatibility facade was added.
The release-group map records Jobs' optional dependency on Timers.

The public wasm32 preview binds one retained Once per job. The caller supplies a
parsed schedule and explicit lateness tolerance. Selection stores the calendar
revision/time before timer arming. The callback receives that identity and actual
start time; acceptance occurs before invoking its future factory, outside state
borrows. Business outcomes do not cause automatic retry. Late delivery skips
consumer invocation. After completion or skip, one Saffron search selects strictly
after current time and settled progress. There is no missed-occurrence enumeration.

Owner edits during an accepted attempt change Jobs state without placing a
competing successor override in Once. The wrapper derives the successor from the
latest schedule and pause state after the accepted callback settles. This preserves
pause/resume/edit ordering, including a pause followed by resume during the await.
Queued changes invalidate/reselect eligibility through the existing Once capability.

Timer recovery is associated with the retained selected occurrence, including
queued Jobs state after an initial factory trap rolls acceptance back. Recovery
status exposes revision/time and a nonserializable token scoped to both this job
and the exact timer recovery authority. Ordinary pause/resume does not clear it.
Explicit reconciliation retires the uncertain occurrence and leaves the job paused;
resume chooses a new future occurrence. Reusing a consumed recovery is rejected.
Terminal progress faults prevent subsequent consumer admission.

Six installed cases pass against the actual adapter:

- Normal async work retains selected time and reports actual start separately.
- Initial factory trap blocks the original selected occurrence without a committed start.
- Post-await trap blocks that occurrence with one committed start and no finish.
- Both traps remain blocked through pause/resume and another minute; explicit
  recovery permits a future occurrence, and stale recovery is refused.
- Pause/resume plus schedule edit during the await keeps the running revision and
  selects its successor from the new two-minute schedule.
- Pause during the await leaves the job paused after completion.
- Zero tolerance skips a deliberately late occurrence without invoking business work.

The first two trap cases share recovery assertions; the fixture has six modes.
These are local installed results, not a real consumer migration or durable
exactly-once guarantee.

### Supplier search observations

The installed probe also checks fixed Saffron outcomes over the full supported
upper timestamp bound, with instruction brackets around parse and search:

| Rule/case | Parse instructions | Search instructions | Outcome |
| --- | ---: | ---: | --- |
| Every minute from epoch | 242,751 | 1,413 | 60 seconds |
| Feb 29 after 2099-03-01 | 5,843 | 3,709 | 2104-02-29 09:00 UTC |
| Monthly 31st after 2026-04-01 | 4,174 | 1,939 | 2026-05-31 09:00 UTC |
| Feb 31 from epoch | 5,843 | 715 | None |
| Every minute after final representable minute | 2,751 | 1,398 | None |

The first parse includes cold-path costs; these are raw brackets, not equivalent
steady-state comparisons. This verifies selected sparse/impossible cases without
adding a custom search engine. It is not a comprehensive worst-case search budget
for every admitted 256-byte expression.

### Qualification boundary

All ten native Jobs tests pass, along with native all-feature/default-free and
Wasm Clippy, probe/driver Clippy, Wasm Rustdoc and formatting. The full installed
Once/watchdog suite passes with Jobs enabled. The public API remains a preview:
host persistence/reconstruction, durable revision identity, catch-up, coalescing,
full expression search budgets and multi-job failure pressure remain release gates.
Tokens and closures are volatile; this API does not support restoring unresolved
work across upgrade or snapshot restoration. No publication is enabled.

Reproduce with `make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic`.
Observed with Rust 1.100.0, Runtime 0.22.5, Saffron 0.1.0, Chrono 0.4.45,
Testkit 0.106.0 and PocketIC client/server 16.1.0/16.0.0. The existing server
identity is recorded in executor-probe.md.
Combined installed probe Wasm SHA-256:
169fc9c66a742fff5c9d824e7adae2c25788eaf8300b6caf3a73b285a4b976fd.
Product lock SHA-256:
ec9ee2b1cb05c18e7d99fc331b171ed032e4a90005a1cf825a4560f1846c6698.
Qualification lock SHA-256:
a513228f62a5062ca2dbe22236bbf1eb9e2c4d0c729e782b876892976531b7b0.
