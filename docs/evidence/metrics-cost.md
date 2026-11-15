# Metrics feature cost — 2026-11-15

Timers 0.45.4 adds a small qualification-only paired workload. No product behavior
changed in this slice. The same `metrics-probe` source is built in release mode
with the metrics feature disabled and enabled. Both builds export the same setup,
echo, state, global-timer and provider-dispatch entrypoints. One asynchronous Once
returns ExpectedFailure after a real self-call; one synchronous watchdog returns
NoWork. Both stop normally. The feature-off normal dependency graph excludes
ic-metrics; the enabled graph adds only pinned ic-metrics 0.30.8.

The state query reports committed callback counts and measured brackets. With
metrics enabled it also reads the real aggregate; with metrics disabled it emits
zero diagnostic fields of the same wire width. Consequently binary differences
include the diagnostic read/encoding path as well as product recording. These
are fixture-inclusive costs, not a minimal-link or universal per-consumer delta.

## Observed result

| Raw release Wasm | Bytes |
| --- | ---: |
| Metrics disabled | 91,355 |
| Metrics enabled | 94,896 |
| Difference | +3,541 |

Three fresh canisters per build produced identical values within each row:

| Instruction bracket | Disabled | Enabled | Difference |
| --- | ---: | ---: | ---: |
| Initialize, bind and arm both declarations | 489,337 | 490,706 | +1,369 |
| Message prefix through Once factory entry | 486,371 | 486,369 | −2 |
| Message prefix through Once resumption after echo | 483,097 | 483,180 | +83 |
| Message prefix through watchdog callback entry | 484,439 | 484,488 | +49 |

The small negative factory difference reflects build/code-generation effects;
it is not evidence that recording acceptance has negative cost. Prefixes include
runtime/provider entry and instrumentation; they exclude the rest of callback
execution and settlement. In particular, these are not whole-async-lifetime
instruction deltas or a complete overhead total. No additional runtime-baseline
optimization or legacy comparison was attempted here.

Both builds report exactly one Once start/completion and one watchdog invocation,
with no further execution after additional rounds. Enabled diagnostics report
two acceptances, two completions and one expected failure, with no abandonment,
submission deferral, provider fault or overflow. Disabled diagnostics remain zero.

## Reproduction and identity

`make check-metrics POCKET_IC_BIN=/absolute/path/to/pocket-ic` builds and retains
both files under qualification/target, then runs the existing driver in paired
mode. No server download occurs. Probe Clippy passes for both feature selections,
and host-driver Clippy, formatting and diff checks pass. The normal full installed
Once/watchdog/Jobs suite also passes on the new .4 versions after the driver's
paired mode was added. Wasm export inspection confirms identical entrypoint sets.

Rust 1.100.0, Timers 0.45.4, Runtime 0.22.5, Metrics 0.30.8, Testkit 0.106.0 and
PocketIC client/server 16.1.0/16.0.0. Local installed evidence only; this does not
qualify a deployed consumer, retained-memory bound or full-table overhead.

SHA-256:

- Disabled Wasm: 040fd3aad177a8454246996020dc7b6036a214dddd428fb1a861e3e4f6d7bea4.
- Enabled Wasm: b386b640777e56e7e9051a941a415b9e623a346f36a7c5282b22d30bb0c41eb8.
- Product lock: 97c31b1ddf7a5d6e9f5c706a9929d8f7cc0e2b3eda09ac43dd6d16300a7df136.
- Qualification lock: fc8dad436e922662a72f814bb82dffd7794b53c1cf367e0d275b8f3df03db91f.
