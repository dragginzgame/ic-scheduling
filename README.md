# IC Scheduling — new architecture

Replacement scheduling libraries for the functionality required
by **Canic management and buffered Entropy**, plus a new **calendar-aware Jobs scheduler**. Relay is explicitly out of scope.

Start with the [architecture and implementation plan](docs/design/DESIGN.md)
and its [consumer requirements](docs/evidence/consumer-requirements.md).
The [source inventory](docs/evidence/source-inventory.json) identifies the local
working files inspected on 2026-11-15, including hashes for dirty sources.

The architecture separates a small retained-timer library (`ic-timers`)
from a cron-like calendar scheduler (`ic-jobs`). Read the [Jobs design](docs/design/jobs.md)
for UTC calendar rules and execution semantics. Both permit breaking
redesign. The [release scope](docs/design/release-scope.md) preserves legacy
Timers/Jobs capabilities while deferring additional proposals. The old architecture,
demo application and release framework are not carried forward.

The current published releases are Timers 0.46.11 and Jobs 0.39.9.
Schema 7 replaces Jobs 0.37.9's schema 6.
Jobs supports Saffron-backed five-field UTC cron, one-shot deadlines, anchored
fixed-rate intervals and completion-relative recurrence. Schedule support is opt-in via the
`calendar` feature. Its separate opt-in `execution` feature provides installed-canister
skip-late and bounded catch-up execution on Once with host-owned settled/queued
checkpoints and explicit blocked reconstruction of unresolved attempts; Jobs coalescing
is deferred.
Timers has a
[wasm32 Once provider](docs/evidence/once-provider.md), with installed checks for
completion, trap recovery, owner precedence, held-flight pressure, low-cycle
submission deferral and armed-deadline upgrade reconstruction. The
[watchdog provider](docs/evidence/watchdog-state.md) also has installed prearm,
trap-survival and owner-precedence checks. Timers' opt-in `metrics` feature exposes
aggregate counters and bounded per-declaration inventory using ic-metrics arithmetic,
with host-owned export. Actual consumer adoption and production-specific budgets
remain host responsibilities.
Once callbacks can return `OnceNext::After(positive_delay)` to recur relative to
completion, including awaits. Overflow stops recurrence; newer owner commands
take precedence and traps still require explicit recovery.
Published on 2026-11-16: [Timers 0.45.11](https://crates.io/crates/ic-timers/0.45.11)
and [Jobs 0.38.1](https://crates.io/crates/ic-jobs/0.38.1). These replace the
0.44 / 0.36 APIs with a breaking redesign. Both registry artifacts and GitHub
release tags are verified. Native, Wasm, local installed-canister and archive
release gates passed; Entropy integration was excluded by maintainer direction
for these two versions. Hosted CI runs separately. See the [delivery evidence](docs/evidence/release-candidate.md) and
[grouped changelogs](CHANGELOG.md).
Existing Scheduling, Canic and Entropy
are unchanged. Recorded local evidence does not establish deployment or production resource budgets.

## Development

Rust is pinned to 1.100.0 with Clippy, rustfmt and the `wasm32-unknown-unknown`
target. Toko Tooling is pinned to 0.45.22 in both Cargo workspaces and installed under
`vendor/toko-tooling/`, with its checksum receipt at `.toko-tooling.snapshot`.
Read [the engineering baseline](TOKO-TOOLING.md) and [local instructions](AGENTS.md).

From the repository root, prepare the declared Rust toolchain with rustup, then:

```sh
make install-tools
make tools-check
make install-hooks
make check
make lint PACKAGE=ic-timers
make doc PACKAGE=ic-jobs
make fmt-check
```

Calendar checks: `cargo test -p ic-jobs --features calendar --lib --locked` and
`make check-calendar-wasm`. The latter requires a separately prepared Node.js
runtime (observed here: 24.21.0), runs a standalone Wasm engine probe and prints
its binary/code-section sizes. It does not qualify installed IC behavior.

Tool installation is explicit. Cargo checks may fetch missing locked dependencies.
Set `CARGO_NET_OFFLINE=true` for an offline invocation with prepared dependencies.
For direct shell access to the installed common tools:

```sh
export PATH="$PWD/.tools/host/bin:$PWD/.tools/rust/common/bin:$PATH"
```

Dependency maintenance uses the shared commands:

```sh
make deps-list
make deps-update
make deps-update DEPENDENCY_PACKAGE=toko-tooling
```

`deps-update` accesses the registry and updates locks; `DEPENDENCY_PRECISE`
optionally selects a version within the manifest requirement. To adopt a new
Toko release, change its exact Cargo pin in both workspaces, update the locks, then run
`make toko-tooling-preview` and `make toko-tooling-update`.
Run `make install-hooks` after setup or a tooling update in each clone. It sets
repository-local `core.hooksPath` to `vendor/toko-tooling/.githooks`. The canonical
hook formats the staged selection and rejects partially staged files; CI still
checks formatting independently. `make check-formatting-hooks` exercises this
checkout's hook with prepared tools, including both maintained workspaces.

[Qualification](qualification/README.md) has its own manifest and lockfile and
contains calendar, executor, timer/Jobs, metrics and isolated Entropy probes. Dependency and formatting commands include
both workspaces. Standard Toko release/publication entrypoints are installed; see
[release operations](docs/operations/releases.md).

## Timer lifetimes and observations

Binding requires `Lifetime::Retained` or `Lifetime::RemoveWhenStopped`.
Dropping a handle does not cancel work. Explicit `remove` refuses accepted or
uncertain Once work and running/settling watchdog tickets. Transient declarations
are removed after a committed stop and definitive settlement; newer owner demand
wins. Old handles and queued deliveries cannot control replacement declarations.
Callbacks and their captures are released outside registry borrows.

The provider admits at most 64 declarations shared by Once, watchdogs and Jobs,
with eight outstanding transport flights and four submissions per control turn.
Pressure defers delivery; deadlines are eligibility times, not punctuality guarantees.

With `metrics`, `inventory()` returns sorted bounded declaration snapshots,
including eligibility, watchdog custody and normal-result/failure observations.
Incarnations identify this runtime only; observations contain no recovery tokens
and do not establish external-effect success. Hosts own export and history.

## Optional calendar support

Canisters that only need timers depend on `ic-timers`; it does not pull in Jobs
or any calendar/time-zone dependencies. Calendar recurrence in `ic-jobs` is
explicitly opt-in (the default feature set is empty):

```toml
[dependencies]
ic-jobs = { version = "0.39.9", default-features = false, features = ["calendar"] }
```

Omit `calendar`
to exclude Saffron and Chrono. Cargo features are additive;
another dependency enabling it will enable it for that build as well. Calendar
preview delegates to Saffron; the separate `execution` feature enables the
installed-canister execution adapter described above.

With `calendar` enabled, `Schedule::parse("0 9 * * *")` means daily at **09:00 UTC**.
`next_after(after_ns, through_ns)` returns an optional unsigned Unix nanosecond
instant after the lower bound and through the upper bound. Cron expressions have
five fields and minute precision; time zones and a seconds field are not supported.
See the [UTC Wasm measurements](docs/evidence/calendar-saffron.md).

`Schedule::fixed_rate(first_at_ns, every_ns)` uses a positive `NonZeroU64`
interval and preserves its original anchor. Binding selects the original first
deadline, including zero and overdue deadlines; the explicit lateness/catch-up
policy controls delivery. Bounded catch-up freezes an oldest-first batch and
discards excess backlog; it does not promise to replay every missed occurrence.
Cron binding continues to select after current time.
In Jobs 0.38.0, checkpoint schema 7 distinguishes no settled work from a settled zero timestamp
and preserves Active, Paused or Cancelled control state plus pending retry budgets.

`Schedule::once(at_ns)` selects one deadline under the same delivery policies,
including zero and overdue deadlines. After settlement or lateness skipping it
becomes idle; an uncertain attempt still requires explicit recovery.

`Job::cancel()` terminally cancels unaccepted work, including remaining catch-up
occurrences. It survives restoration and refuses running/uncertain work. Jobs keeps its retained timer declaration; timer clients can explicitly remove
settled declarations.

`Schedule::after_completion(first_at_ns, every_ns)` waits a positive interval
after each normal settlement. Pause and restoration preserve the deadline;
traps require explicit recovery. Preview methods can show only the first deadline.

`bind` and `restore` take an explicit `RetryPolicy` (`NONE` disables retries).
Only `Outcome::RetryableFailure` requests another attempt, using a total-attempt
budget and capped exponential backoff. The job key, revision and scheduled time
remain the same effect identity; `Context::attempt` starts at one. Pending retry
counts and deadlines survive pause and host-owned checkpoints. Traps remain
blocked for explicit recovery. `recover` requires `RecoveryDisposition::Retire`
or `RetrySafe`; the latter asserts that repeating effects is safe and retains
the attempt budget. Both leave the job paused. `unresolved_checkpoint` and `restore_unresolved`
preserve running/uncertain attempt identity across fresh-runtime reconstruction;
restoration is always blocked. `checkpoint()` and `unresolved_checkpoint()` both
return `Checkpoint`; unresolved data is
attached with `with_unresolved` and read with `unresolved`. Ordinary `restore`
rejects unresolved data, while `restore_unresolved` requires it. Schema 6 and the
old `UnresolvedCheckpoint` wrapper require explicit host migration. Hosts own
atomic encoding and the upgrade boundary.
Checkpoints retain the latest returned outcome or recovery disposition, keyed
by `Attempt`; older outcome history belongs to the host.

The executor prerequisite probe runs with
`make check-executor POCKET_IC_BIN=/absolute/path/to/pocket-ic` using a prepared
PocketIC 16.0.0 server. It checks normal async completion and initial/post-await
traps; see [the observed results](docs/evidence/executor-probe.md). The product Once slice has its own
`make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic` target; see its
[evidence and remaining gates](docs/evidence/once-provider.md).
