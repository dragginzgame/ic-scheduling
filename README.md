# IC Scheduling — new architecture

A design-first replacement repository for the scheduling functionality required
by **Canic management and buffered Entropy**, plus a new **calendar-aware Jobs scheduler**. Relay is explicitly out of scope.

Start with the [architecture and implementation plan](docs/design/DESIGN.md)
and its [consumer requirements](docs/evidence/consumer-requirements.md).
The [source inventory](docs/evidence/source-inventory.json) identifies the local
working files inspected on 2026-11-15, including hashes for dirty sources.

The proposed architecture separates a small retained-timer library (`ic-timers`)
from a cron-like calendar scheduler (`ic-jobs`). Read the [Jobs design](docs/design/jobs.md)
for UTC calendar rules and execution semantics. Both permit breaking
redesign; unused timer policy families, a demo application and the old release
framework are not carried forward.

Status: early implementation. Jobs has Saffron-backed five-field UTC cron
occurrence preview. Calendar support is opt-in via the
`calendar` feature. Jobs execution is not implemented. Timers has an initial
[wasm32 Once provider](docs/evidence/once-provider.md), with installed checks for
completion, trap recovery, owner precedence, held-flight pressure, low-cycle
submission deferral and armed-deadline upgrade reconstruction. Watchdog, metrics,
instruction measurements and remaining failure/lifecycle qualification are open.
Development targets are
Timers `0.45.1` and Jobs `0.37.1`, following the initial development checkpoint
(`0.45.0` / `0.37.0`). These remain unpublished; the last observed crates.io
versions were `0.44.3` / `0.36.3` (registry checked 2026-11-15).
Track changes in the [grouped changelogs](CHANGELOG.md). Development delivery is
GitHub-only for now; both crates retain `publish = false` until the rewrite is
complete and crates.io publication is explicitly requested.
Existing Scheduling, Canic and Entropy
are unchanged. Size, performance and runtime safety remain unproven until the
implementation and consumer qualification gates pass.

## Development

Rust is pinned to 1.100.0 with Clippy, rustfmt and the `wasm32-unknown-unknown`
target. Toko Tooling is pinned to 0.44.3 in Cargo and installed under
`vendor/toko-tooling/`, with its checksum receipt at `.toko-tooling.snapshot`.
Read [the engineering baseline](TOKO-TOOLING.md) and [local instructions](AGENTS.md).

From the repository root, prepare the declared Rust toolchain with rustup, then:

```sh
make install-tools
make tools-check
make check
make lint PACKAGE=ic-timers
make doc PACKAGE=ic-jobs
make fmt-check
```

Calendar checks: `cargo test -p ic-jobs --features calendar --lib --locked --offline` and
`make check-calendar-wasm`. The latter requires a separately prepared Node.js
runtime (observed here: 24.21.0), runs a standalone Wasm engine probe and prints
its binary/code-section sizes. It does not qualify installed IC behavior.

Tool installation is explicit and may use the network. Ordinary checks use the
prepared tools and locked dependencies offline. For direct shell access to the
installed common tools:

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
Toko release, change its exact Cargo pin, update its lock selection, then run
`make toko-tooling-preview` and `make toko-tooling-update`.

[Qualification](qualification/README.md) has its own manifest and lockfile and
contains the calendar Wasm probe and installed executor probe. Dependency and formatting commands include
both workspaces. No release pipeline or Git hook is installed.

## Optional calendar support

Canisters that only need timers depend on `ic-timers`; it does not pull in Jobs
or any calendar/time-zone dependencies. Calendar recurrence in `ic-jobs` is
explicitly opt-in (the default feature set is empty):

```toml
[dependencies]
ic-jobs = { path = "path/to/ic-scheduling-new/crates/ic-jobs", default-features = false, features = ["calendar"] }
```

Use the appropriate checkout path during GitHub-only development. Omit `calendar`
to exclude Saffron and Chrono. Cargo features are additive;
another dependency enabling it will enable it for that build as well. Calendar
preview currently delegates to Saffron; job execution remains unimplemented.

With `calendar` enabled, `Schedule::parse("0 9 * * *")` means daily at **09:00 UTC**.
`next_after(after_ns, through_ns)` returns an optional unsigned Unix nanosecond
instant after the lower bound and through the upper bound. Cron expressions have
five fields and minute precision; time zones and a seconds field are not supported.
See the [UTC Wasm measurements](docs/evidence/calendar-saffron.md).

The executor prerequisite probe runs with
`make check-executor POCKET_IC_BIN=/absolute/path/to/pocket-ic` using a prepared
PocketIC 16.0.0 server. It checks normal async completion and initial/post-await
traps; see [the observed results](docs/evidence/executor-probe.md). The product Once slice has its own
`make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic` target; see its
[evidence and remaining gates](docs/evidence/once-provider.md).
