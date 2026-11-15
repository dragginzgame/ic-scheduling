# Qualification

Independent probes and drivers live in this workspace, with their own dependency
catalog and lockfile. All packages remain unpublished and inherit the shared
compiler, Toko pin and lint baseline. The license path points to the repository
root. Qualification consumes product packages; the reverse dependency is forbidden.

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

The supplier probe does not import Timers or install a global timer. The complete
Once transport and watchdog remain unqualified. Native unit tests stay beside
their owning product code; the calendar artifact supplies no IC trap evidence.
