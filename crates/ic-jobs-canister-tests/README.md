# Local queue demo and canister tests

The queue demo is a terminal client connected to a real Jobs qualification
canister installed in **local PocketIC**. Management calls use Wire; the canister
owns storage, dispatch, counters and its Timers watchdog. The host only displays
the returned schedule and drives PocketIC's clock and rounds.

From the repository root, prepare the selected local server once:

```bash
make install-testkit-tools install-consumer-server
make demo
```

At the `jobs>` prompt, run one command per line:

```text
sample
watch 6s
advance 1m
advance 1w
list
```

`sample` queues a one-shot job in five seconds, another in one minute and a weekly
job whose first run is in one week. `watch 6s` displays the countdown once per
second and shows the first job completing. `advance` jumps IC time immediately,
so you can witness minutes or weeks of scheduling without waiting that long.
The table shows each job's state, next deadline and actual dispatch count, plus
the canister's counter, next wakeup and watchdog state. `RAN` messages report
dispatches observed in the canister after advancing time.

Add your own jobs and manage them by the ID printed by `add`:

```text
add quick --after 3s
add minute-report --after 2m --every 1m
add weekly-report --after 1w --every 1w
list
pause 5
advance 1h
resume 5
advance 1s
cancel 5
help add
quit
```

Names are single words. Durations are whole numbers followed by `s`, `m`, `h`,
`d` or `w`; one week means seven days, not a calendar rule. Recurring jobs skip
missed intervals by default. Add `--catch-up` with `--every` to retain overdue
occurrences. Each `advance` delivers 32 canister rounds; a large catch-up backlog
may remain visibly due. Use another `advance` or `watch` to continue processing.
The client never calls the manual dispatch endpoint to bypass the watchdog.

Time is paused at the prompt. `watch` waits between one-second advances; its
total wall time also includes RPC and display work. The session has a one-hour
server limit. Each launch creates a fresh temporary canister; `quit` or EOF ends
the session; Ctrl+C also shuts down its owned server. Interrupting `watch` during
its wait stops before the next IC time advance. Jobs are retained inside
the canister during the session, including
completed/cancelled jobs, so its existing limit of 32 jobs applies. The payload
is a counter increment, providing an observable canister effect for each run.
This fixture is not a production deployment or a persistent background service.

## Package boundary

The default package builds `jobs-capture-artifact` without PocketIC, Testkit,
Wire or the qualification-consumer library. Enable `--features harness` to build
`ic_jobs_canister_tests::harness`, the recovery test or the queue example. The
Make demo/recovery targets select this feature explicitly; those targets retain
their existing separate server admission and live-test selection.

## CLI tests

```bash
# Parser/input tests; compiles the live scenario without starting a server.
bash scripts/dev/minimum-rust.sh cargo test -p ic-jobs-canister-tests --example queue --features harness --locked --offline

# Explicit local canister test, with visible schedules and assertions.
make test-demo

# Explicit real terminal process tests: quit, EOF and idle/watch Ctrl+C.
make test-demo-cli
```

The live scenario uses the same Clap commands as the terminal to queue seconds,
minutes and weeks, then checks actual canister state after fast-forwarding,
pausing, resuming and cancelling weekly recurrence. Ordinary `make check-consumer`
includes the parser test and compiles the ignored live tests. The process test
launches a fresh owned server for each shutdown mode, waits for a successful
canister listing, and checks both terminal and server exit within bounded waits.
The watch case queues work due at the first advance and verifies that interruption
during the wait dispatches none of it. The separate
[recovery harness](../../apps/ic-jobs-test-consumer/README.md) retains its own explicit
PocketIC gate and recovery evidence; the demo does not replace it.
