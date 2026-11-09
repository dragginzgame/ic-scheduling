# Timers in IC Scheduling

This workspace owns both published libraries and all their qualification packages.
Use the root [agent instructions](../../AGENTS.md), [release guide](../releasing.md)
and [current handoff](../status/current.md). Older evidence and design documents
retain their tested source identities; old repository commands are historical.

## Commands

- `make test-timers`: aggregate minimum-compiler library, doctest, protocol and
  host-only tooling tests.
- `make test-timer-tooling`: host-only driver/discovery/receipt tests; no PocketIC.
- `make check-timer-provider check-timer-dependencies`: provider confinement and
  native/Wasm dependency admission, including the matching Platform runtime guard.
- `make test-timer-dependencies`: deliberate forbidden-edge rejection fixtures.
- `make check-timer-probes`: probe compilation and each size-probe feature selection.
- `make install-testkit-tools install-consumer-server`: prepare the shared CLI/server.
- `make test-timer-watchdog`: explicit installed watchdog/runtime qualification.
- `make test-timer-cohorts`: explicit installed policy comparison with retained reports.
- `make test-canister`: explicit Jobs/Timers composition recovery qualification.

The installed subjects live in the dedicated `runtime` integration target; the
qualification driver selects it and retains exact subject names in its receipts.
Direct `cargo test --all-targets` would execute installed subjects without the
driver's artifact admission. Use the explicit driver commands above for IC work;
use `--lib --bin ic-timers-pocketic` for host-only driver tests.

One cohort catalog owns selection and typed measurement slots. The report keeps
its existing JSON field names and strict completeness checks. Build identity is
admitted as a whole from the first successful build; later builds must match it.

For the consumer-facing Metrics cut, graph alignment and fresh native/installed
acceptance, use the [0.39 → 0.40 adoption checklist](adoption/0.40.md).

For changes to local release-group fixtures, select
`bash scripts/release/test-tooling.sh metadata` for preparation/rollback or
`bash scripts/release/test-tooling.sh recovery` for interruption, resume and
publication admission. The latter substitutes uploads and the full release gate
inside disposable repositories; it never publishes a real crate.

The root owns one Cargo lock, Rust 1.99.1 floor, development toolchain, formatting,
CI and publication path. The `timer-probe` profile retains its previous optimization
settings. Ordinary builds default to both published crates. Probe packages remain
unpublished. The timer registry, callbacks and unsafe IC dispatch remain private
to Timers; consumers still own durable job state and lifecycle reconstruction.

## Host support

Linux and macOS Intel/Apple Silicon remain supported qualification hosts.
Native tests cannot establish IC recovery. The manually selected recovery workflow
runs both Jobs recovery and Timers watchdog/cohort gates on each host. Check the
handoff for what was actually run on this combined source and graph.

## Runtime composition qualification

Scheduling owns Timers' composition with its root-selected Platform Runtime,
instead of gating Platform's releases on a downstream Timers dependency
([#91](https://github.com/dragginzgame/ic-scheduling/issues/91)). The supported
cohort is Runtime 0.21 with the matching exact guard; the current lock selects
0.21.0. The mandatory installed subjects and policy cohorts qualify this selected
source and lock; earlier Runtime 0.20 evidence does not substitute for that gate.

| Required observation | Scheduling owner |
| --- | --- |
| Initialize/reinitialize without losing active callback authority | Installed `runtime_composition_preserves_calls_cleanup_and_lifecycle`; native `initialization_is_required_and_idempotent` |
| Callback awaits a real Runtime call, then completes | Same installed composition subject, selecting raw and prepared calls with identical Wire requests/budgets |
| Callback continuation trap cleans execution authority; later work succeeds | Same composition subject plus `ordinary_continuation_trap_retires_running_work_and_releases_expired_authority` |
| Cancellation causes no callback mutation; explicit rearm dispatches | Same composition subject plus `absent_ordinary_demand_is_idle_and_explicit_cancellation_stays_distinct` |
| Dispatch succeeds after upgrade | Same composition subject plus the retained readiness/reconstruction proof in `trapped_work_keeps_committed_successor_and_executor_is_private` |
| One selected executor and exact registry guard in each native/Wasm graph | `check-timer-dependencies` and its forbidden-edge/source rejection fixtures |

The composition subject is mandatory in the existing Watchdog qualification
roster. Its private controls select the call transport and repeat initialization;
they add no product API, timer provider, persisted state or Candid facade.
Testkit continues to own simulator/server lifecycle. For focused development:

```bash
cli="$(bash scripts/dev/testkit-tool.sh --check)"
export POCKET_IC_BIN="$("$cli" check)"
cargo run --manifest-path qualification/Cargo.toml -p ic-timers-pocketic --locked --offline -- focused \
  tests::lifecycle::runtime_composition_preserves_calls_cleanup_and_lifecycle
```

Use the prepared tools described in [repository setup](../../README.md#repository-setup).
The driver
binds subjects, source, lock, compiler and exact Wasm bytes in its retained receipt.

## Release and retained evidence

Jobs and Timers have independent versions and canonical
[Jobs](../../releases/jobs/CHANGELOG.md) and
[Timers](../../releases/timers/CHANGELOG.md) ledgers. The
[coordinated history](../../releases/history.md) retains the workspace migration.
Select `RELEASE_GROUP=jobs` or `timers` explicitly for release or publication;
publication requires its separate authorization.
Historical receipts under `evidence/`, `audits/` and `changelog/` describe their
original checkouts and cannot qualify this migration. Original sibling source
remains available for rollback and provenance.

### Volatile mixed-policy qualification

The main runtime probe owns the Watchdog fault controls and observations.
`StartVolatilePair` adds a heap-only after-completion registration alongside that
Watchdog without setting retained readiness; `StopVolatilePair` removes both.
The separate startup-probe module, methods and snapshot DTO are retired.
The main snapshot carries completion counts/timestamps for both policies under
fresh method identity 69; rebuild the unpublished probe and host decoder together.
The four-field upgrade checkpoint and the published Timers API are unchanged.
Exact lifecycle subjects retain trap/exhaustion isolation, overdue coalescing and
absence of implicit reconstruction after upgrade. The retained-Watchdog upgrade
subject separately qualifies reconstruction from explicitly stored readiness.
