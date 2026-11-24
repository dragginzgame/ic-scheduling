# Qualification

Independent probes and drivers live in this workspace, with their own dependency
catalog and lockfile. All packages remain unpublished and inherit the shared
compiler, Toko pin and lint baseline. The license path points to the repository
root. Qualification consumes product packages; the reverse dependency is forbidden.
Host drivers use `ic-testkit::pocket_ic`, matching ic-platform’s qualification
boundary. The lock selects Testkit 0.110.4 and PocketIC client 16.1.0. The prepared
16.0.0 server is supported. Do not add a direct `pocket-ic` dependency.

Testkit 0.110.0's held-reply feature admits Protocol `>=0.3.2, <0.4.0`. The
previous adoption selected 0.3.5, completing the migration beyond
[Tooling #9 — Admit published Protocol 0.2.7 in Testkit's held-reply fixtures](https://github.com/dragginzgame/ic-tooling/issues/9).
The current qualification lock selects Runtime and guard 0.23.25, Types 0.21.4,
Protocol 0.3.8 and optional Metrics 0.30.21. Types and Testkit use compatible
requirements. Runtime owns its exact singleton guard. Testkit's Protocol error
type remains inside private qualification.

On 2026-11-24, Toko Tooling 0.46.3 passed snapshot and tool verification.
With Runtime 0.23.25 and Testkit 0.110.4, the installed executor, Once, Jobs,
paired metrics and pinned Entropy lifecycle checks passed on PocketIC 16.0.0.
Local results are in `target/adoption-installed.log` and `target/adoption-entropy.log`.

The Toko Tooling 0.46.0 adoption passed the real Timers-to-Jobs Rust stage
reuse and evidence refusal checks. The stage runner preserved unset, explicit
online and explicit offline Cargo modes and refused a stage-injected override.
The real stage log is `target/toko-0460-rust-stage.log`.

The Runtime 0.23.23 and Toko Tooling 0.45.22 adoption with Metrics 0.30.18 passed the native and Wasm
composition matrix, the standalone calendar probe, and the installed Once,
watchdog, Jobs and paired metrics checks on PocketIC 16.0.0. The tooling audit
found zero discrepancies across 10 Cargo manifests. The local installed check
log is `target/adoption-04522-platform-02323-installed.log`.

The Runtime 0.23.22 and Toko Tooling 0.45.21 adoption passed native and Wasm
Clippy, the retained timer consumer checks, and the calendar, installed
executor, Once, paired metrics and pinned Entropy probes. The rebuilt probes
retain the previous release's Wasm export sets. Local logs are
`target/adoption-04521-platform-02322-installed.log` and
`target/adoption-04521-platform-02322-consumer.log`.

The 2026-11-19 Platform adoption passed host and Wasm checks for the retained
timer consumer, the installed executor and Once suites, the paired metrics
probe, and the pinned Entropy lifecycle. The staged Entropy fixture selected
Runtime, guard and macros 0.23.21 and Protocol 0.3.7. Its PocketIC server was
16.0.0. Local logs are `target/adoption-platform-02321-installed.log` and
`target/adoption-platform-02321-consumer.log`.

Local Testkit 0.110.0 / Toko Tooling 0.45.0 adoption on 2026-11-18 passed
native driver Clippy through the real Rust-stage fixture, installed executor and
complete Once checks, and the freshly rebuilt pinned Entropy
`background_refill_and_lifecycle` test with the prepared PocketIC 16.0.0 server.
Logs are `target/adoption-01100-installed.log` and
`target/adoption-0450-rust-stage.log`. Testkit's optional managed-server and CLI
features remain disabled. These are local Linux results, not hosted CI or release
evidence.

Earlier Testkit 0.109.0 adoption on 2026-11-17 passed native driver Clippy,
`make check-executor check-once` with the prepared PocketIC 16.0.0 server,
and the freshly rebuilt pinned Entropy `background_refill_and_lifecycle` test.
The complete Once run includes Jobs upgrade/recovery, retry and catch-up cases.
Logs are retained under `target/testkit-01090-*.log`. This is local Linux evidence;
hosted macOS validation and deployment remain separate. The optional Testkit
managed-server feature is not enabled by these callers.

Runtime requirements admit compatible 0.23 patches from 0.23.21. Run
`node qualification/runtime-composition-check.mjs` from the repository root when
changing that boundary. The check compiles a Timers/Jobs consumer for native and
Wasm with each of 0.23.21–0.23.25, verifies one matching Runtime/guard per build,
and confirms that the incompatible 0.22.5 guard still fails to resolve.
With the published dependencies cached, set `CARGO_NET_OFFLINE=true` to run offline.

`calendar-probe` builds the actual Saffron-backed UTC Jobs adapter as a
standalone Wasm module. Its raw export selects fixed calendar scenarios; the
Node.js driver checks eight UTC outcomes, rejects host imports, and reports
full Wasm/code-section bytes and initial/final linear-memory size. Run from root:

```sh
make check-calendar-wasm
```

Prepare the declared Rust toolchain and Node.js separately. Cargo may fetch locked
dependencies. Node 24.21.0 was used for the
recorded observation. See [calendar evidence](../docs/evidence/calendar-saffron.md).
The raw probe ABI and its zero sentinel are qualification-only.

Shared `make deps-list`, `make deps-update` and formatting include both workspaces.

The installed suite also checks [full-table capture retention](../docs/evidence/capture-retention.md):
maximum-length identities, retained payloads through repeated demand changes,
reentrant destruction of rejected captures and release of settled async attempts.

[Entropy integration](entropy-integration/README.md) stages a pinned, read-only
consumer checkout, applies a small timer migration and runs its existing native,
Wasm and installed checks against local Timers. It does not adopt
the migration in Entropy or duplicate its maintained test suite.

`executor-probe` and `executor-driver` check Platform Runtime's update-root
completion and initial/post-await trap behavior in a locally installed PocketIC
canister. Run `make check-executor POCKET_IC_BIN=/absolute/path/to/pocket-ic`
with a prepared 16.0.0 server and local loopback access. Cargo may fetch locked
dependencies. See [executor evidence](../docs/evidence/executor-probe.md).

The Runtime 0.23 probe observes successful cleanup through the retained task
handle after execution commits. Its five event bytes distinguish control,
start, completion, destruction and committed cleanup; it does not use a
destructor-time recovery flag.

The supplier probe does not import Timers or install a global timer. The product Once slice now has a separate
`once-probe`/`once-driver` pair and `make check-once POCKET_IC_BIN=...` target.
It exercises the actual global timer, self-dispatch, trap recovery, owner
precedence, full declaration/flight admission, foreign timer detection, consumer
cleanup traps, low-cycle submission deferral, malformed/stale envelopes and
armed-deadline upgrade reconstruction;
see [Once provider evidence](../docs/evidence/once-provider.md).
After building the probe, the driver accepts `--runtime-core`, `--refusal` and
`--jobs-behavior` for focused completion/trap, low-cycle and Jobs recovery cases.
For example:

```sh
cargo run --manifest-path qualification/Cargo.toml -p once-driver --locked -- \
  qualification/target/wasm32-unknown-unknown/release/once_probe.wasm --runtime-core
```

The held-upgrade scenario verifies a host pre-upgrade guard refuses Running work,
preserves its original reply settlement, and permits reconstruction of fresh
armed demand afterward. It does not qualify upgrading with unresolved effects.
Watchdog, instruction measurements and remaining failure/lifecycle gates are open. Native unit tests stay beside
their owning product code; the calendar artifact supplies no IC trap evidence.

The Once fixture is wasm32-only. Lint it with
`cargo clippy --manifest-path qualification/Cargo.toml -p once-probe --target wasm32-unknown-unknown --locked -- -D warnings`.
Lint its host driver separately with `-p once-driver --all-targets`.

For focused completion-relative recurrence checks, build the fixture and select
the driver's case from the repository root:

```sh
cargo build --manifest-path qualification/Cargo.toml -p once-probe \
  --target-dir qualification/target --release --target wasm32-unknown-unknown \
  --locked
POCKET_IC_BIN=/absolute/path/to/pocket-ic cargo run \
  --manifest-path qualification/Cargo.toml -p once-driver \
  --target-dir qualification/target --locked -- \
  qualification/target/wasm32-unknown-unknown/release/once_probe.wasm \
  --after-completion
```

This selection advances time during a callback's sequence of real self-calls,
then checks completion-relative deadlines, owner override, overflow, Stop and
initial/post-await trap barriers. The aggregate `make check-once` includes it.

Using `--jobs-initial` instead selects fixed-rate initial-deadline checks plus
the affected Jobs checkpoint lifecycle, downtime and catch-up upgrade cases.
It covers zero, overdue and future admission, both delivery policies, unstarted
reconstruction and the schema 7 host fixtures. Their existing cron-only storage
formats explicitly require a present cursor; the new zero fixture tests None.

`--lifetimes` checks retained/removable declarations, transient settlement,
reentrant capture destruction, cleanup traps and stale queued replacement for
both Once and watchdog dispatch. `--diagnostics` checks bounded sorted inventory,
normal-result counters, failure-streak resets and inert repeated reads.

`--jobs-unresolved` explicitly persists running/factory-trapped/post-await-trapped
attempts across actual upgrades. Repeated reconstruction remains blocked; owner
edits, consumed retry budgets, catch-up custody and latest recorded outcomes survive.
It also verifies that ordinary restore rejects an unresolved Checkpoint before
admitting a timer declaration. This opt-in host fixture is separate from ordinary checkpoint fixtures that refuse
unresolved upgrades. Its encoding is fixture-owned, not a general serializer.

`--jobs-retry` checks capped budgets, stable occurrence identity, success and
ordinary failure settlement, pending armed/paused upgrades, pause during work,
owner edits during attempt two, cancellation, catch-up quota and completion-relative
settlement. Initial/post-await traps on attempt two must block despite later time
and resume; unresolved upgrades refuse. Restore rejects mismatched retry policies.
It also checks explicit retry-safe reconciliation, paused upgrades after recovery,
retained catch-up quota and refusal for exhausted budgets, changed definitions,
overflowing deadlines or disabled retries.
The fixture owns its fixed definition and pending-only encoding, not a general
application serializer.

`--jobs-one-shot` runs the same admission fixture with one-shot schedules and
adds initial/post-await trap recovery. It checks idle exhaustion, including
after lateness skipping and explicit recovery, with later time advances proving
there is no recurrence.

`--jobs-completion` checks completion-relative Jobs: actual delayed awaits,
ExpectedFailure/NoWork settlement, lateness skipping, pause/resume, armed and
paused upgrades, owner edit precedence, timestamp overflow and trap recovery.
The host fixture persists the known completion schedule and its settled cursor;
unresolved-work upgrades still refuse. It also checks cancellation before delivery,
while paused and with a remaining catch-up batch; all stay cancelled across an
actual upgrade and later time advances. Running and uncertain cancellation refuses.

The same installed fixture/driver now exercises watchdogs through the shared
product provider: prearmed successor survival across work traps, Continue cadence,
reentrant owner precedence, consumer-owned request admission and full-flight
watchdog coalescing. Run with `make check-once`; the historical target
name is retained. See [watchdog evidence](../docs/evidence/watchdog-state.md).

The probe also enables Jobs `execution` and tests its actual Once composition:
occurrence/start context, skip-late behavior, edits and pause across an await,
initial/post-await traps, blocked replay and stale recovery. Installed Saffron
cases include sparse and impossible rules. Host-owned Jobs checkpoint tests cover
armed/paused reconstruction with preserved revisions, export refusal while running,
upgrade refusal after both trap types and invalid restore before timer admission.
Stopped-canister upgrades after three simulated days verify that restored
lateness policy either runs the selected overdue occurrence once or skips it;
both paths select a future deadline without replaying the missed interval.
The Jobs pressure fixture fills the shared table with twelve Jobs and 52 inactive
Once declarations, holds eight real calls, cancels queued eligibility and traps
one released callback. Healthy jobs continue while the uncertain job stays
blocked, including across the next calendar occurrence. No time jumps are made
while held HTTP barriers remain open.
Jobs boundary checks cover ordinary NoWork/ExpectedFailure recurrence, editing
impossible schedules into and out of eligibility, terminal revision exhaustion
and an idle exhausted timestamp range restored from host progress.
The same fixture measures bounded catch-up previews for full-range, age-limited
and impossible windows. A separate catch-up execution fixture checks age/count
limits, paused upgrades without replenishing the batch, finite-age selections
preserved after further downtime, schedule edits and both trap/recovery paths.
It also executes the full 64-occurrence limit and checks traps on a later batch
occurrence without replaying already completed work or refilling the quota.

The combined probe enables Timers `metrics`. Aggregate assertions cover ordinary
returns, Once/watchdog traps, conservative initial rejection accounting, genuine
submission refusals versus cadence wakes, terminal provider faults and upgrade
reset. Metrics queries use host-owned encoding; the product adds no exporter.
See [metrics evidence](../docs/evidence/metrics.md).

Run `make check-metrics POCKET_IC_BIN=/absolute/path/to/pocket-ic` for a separate
small `metrics-probe` built twice with metrics disabled/enabled. The existing
driver's `--metrics-cost` mode compares raw Wasm sizes and setup/factory/resume/
watchdog instruction brackets across three fresh canisters per build, while
asserting identical callback counts. See the [paired feature-cost evidence](../docs/evidence/metrics-cost.md)
for values and attribution limits. Lint this probe with an explicit wasm32 target,
both without default features and with `--features metrics`.
See [Jobs evidence](../docs/evidence/jobs-progress.md).

The driver also reports instruction samples for synchronous owner control at
three and 64 declarations. Each operation is bracketed within one update message;
three fresh canisters per case expose repeatability. See
[control cost evidence](../docs/evidence/control-costs.md) for values and limits.
Dispatch workloads add per-message prefixes at consumer factory, first poll and
post-await resumption, plus empty-endpoint and cold/warm executor baselines. They
do not measure the complete provider settlement path.

The Once pressure scenario uses Testkit’s `held-reply-fixtures` and its
nonblocking client. It avoids explicit time jumps while HTTP barriers are held,
releases exact requests explicitly, then awaits instance cleanup. This feature
is selected only by the Once host driver, not product or probe Wasm.

The refusal fixture burns cycles in its own test canister below the quoted
self-call cost. It observes the timer value without changing it, proving retry
control ran, then verifies repeated deferral and completion after funding. It
does not add a product fault-injection feature or test queue-full refusal.
Its short-cadence watchdog variant verifies that cadence advances during the
delivery retry delay, including after funding is restored, without early callback
acceptance. The 10-nanosecond cadence is a simulator stress setting only.

## Release adapters

`node qualification/release-check.mjs` qualifies actual public selection refusals,
all standard increment calculations, and each group's metadata transaction,
including qualification lock synchronization and preservation of the other group.
It uses the pinned prepared IC Build metadata editor and creates only ignored
transaction fixtures; it never pushes, tags or publishes. Run `make qualify` for
the complete local release gate, with prepared `POCKET_IC_BIN` and `ENTROPY_SOURCE`.
