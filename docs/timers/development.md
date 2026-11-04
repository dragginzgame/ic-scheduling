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
