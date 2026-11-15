# Qualification

Independent probes and drivers live in this workspace, with their own dependency
catalog and lockfile. All packages remain unpublished and inherit the shared
compiler, Toko pin and lint baseline. The license path points to the repository
root. Qualification consumes product packages; the reverse dependency is forbidden.
Host drivers use `ic-testkit::pocket_ic`, matching ic-platform’s qualification
boundary. Testkit 0.106.0 selects PocketIC client 16.1.0; the prepared 16.0.0
server is supported. Do not add a direct `pocket-ic` dependency.

`calendar-probe` builds the actual Saffron-backed UTC Jobs adapter as a
standalone Wasm module. Its raw export selects fixed calendar scenarios; the
Node.js driver checks eight UTC outcomes, rejects host imports, and reports
full Wasm/code-section bytes and initial/final linear-memory size. Run from root:

```sh
make check-calendar-wasm
```

Prepare the declared Rust toolchain, dependencies and Node.js separately; the
command runs offline and never installs tools. Node 24.21.0 was used for the
recorded observation. See [calendar evidence](../docs/evidence/calendar-saffron.md).
The raw probe ABI and its zero sentinel are qualification-only.

Shared `make deps-list`, `make deps-update` and formatting include both workspaces.

`executor-probe` and `executor-driver` check Platform Runtime's protected-task
completion and initial/post-await trap behavior in a locally installed PocketIC
canister. Run `make check-executor POCKET_IC_BIN=/absolute/path/to/pocket-ic`
with a prepared 16.0.0 server and local loopback access. The command builds and
runs offline; it does not install the server. See [executor evidence](../docs/evidence/executor-probe.md).

The supplier probe does not import Timers or install a global timer. The product Once slice now has a separate
`once-probe`/`once-driver` pair and `make check-once POCKET_IC_BIN=...` target.
It exercises the actual global timer, self-dispatch, trap recovery, owner
precedence, full declaration/flight admission, foreign timer detection, consumer
cleanup traps, low-cycle submission deferral, malformed/stale envelopes and
armed-deadline upgrade reconstruction;
see [Once provider evidence](../docs/evidence/once-provider.md).
The held-upgrade scenario verifies a host pre-upgrade guard refuses Running work,
preserves its original reply settlement, and permits reconstruction of fresh
armed demand afterward. It does not qualify upgrading with unresolved effects.
Watchdog, instruction measurements and remaining failure/lifecycle gates are open. Native unit tests stay beside
their owning product code; the calendar artifact supplies no IC trap evidence.

The Once fixture is wasm32-only. Lint it with
`cargo clippy --manifest-path qualification/Cargo.toml -p once-probe --target wasm32-unknown-unknown --locked --offline -- -D warnings`.
Lint its host driver separately with `-p once-driver --all-targets`.

The Once pressure scenario uses Testkit’s `held-reply-fixtures` and its
nonblocking client. It keeps simulated time fixed while HTTP barriers are held,
releases exact requests explicitly, then awaits instance cleanup. This feature
is selected only by the Once host driver, not product or probe Wasm.

The refusal fixture burns cycles in its own test canister below the quoted
self-call cost. It observes the timer value without changing it, proving retry
control ran, then verifies repeated deferral and completion after funding. It
does not add a product fault-injection feature or test queue-full refusal.
