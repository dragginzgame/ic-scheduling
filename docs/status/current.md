# Current handoff

## Release preparation — 2026-11-03

The owner requested deployment of the accepted Scheduling batch. Prepare Timers
0.39.0 first, then Jobs 0.34.2, followed by their separate crates.io publications.
This checkout is `/home/adam/projects/ic-scheduling` on `main`; delivery targets
`origin` (`dragginzgame/ic-scheduling`) branch `main`, with tags `timers-v0.39.0`
and `jobs-v0.34.2`. Direct integration-branch push approval is still required by
`rules/contributions.md`; complete local preparation before requesting it.
No supplier release or alternate delivery checkout is authorized.
The owner selected Logging 0.24.5 after an unexpected lockfile change interrupted
qualification of 0.24.4. Both locks and the interrupted attempt are preserved.
The published 0.24.5 archive checksum is admitted and its Rust source matches
0.24.4; qualify the selected identity before delivery.

Incoming source is preserved under
`.git/release-state/deployment-current.0NCjlIWS/`. Development `target/` scratch
was absent at the start of this release request; the historical observations
below retain their original limits, and fresh release checks must be retained
in Git-owned state. Current release verification results are recorded there.

## Jobs harness folded locally — 2026-11-03

The redundant `ic-jobs-canister-test-support` package is removed. Its unchanged
management transport now lives in `ic-jobs-canister-tests::harness`, enabled only
by the explicit `harness` feature. Recovery, queue and queue-process targets
require that feature; Make and both launch scripts select it. The workspace and
Jobs release catalog now contain eight total members and three Jobs members.
The default `jobs-capture-artifact` normal/build graph is byte-for-byte unchanged;
PocketIC, Testkit, Wire and the qualification consumer remain excluded.

Strict harness/caller Clippy, four capture cases, the recovery reply-budget case,
queue parser case, both feature graphs and release-group/inheritance checks pass.
Two installed callers pass: management upgrade/watchdog recovery and the queue
recurrence scenario, using the unchanged canister Wasm from the preceding batch.
The three affected tooling phases pass: demo commands, frozen-artifact recovery
and CI input-drift admission. The last fixture now supplies its existing local
release checkpoint inputs. No full CI or new macOS matrix was selected.
`target/current-jobs-harness-fold` retains before/after capture graphs, the moved
module's original bytes, Cargo metadata, focused logs and installed diagnostics.

## Adopted supplier cohort and prior evidence

Shared Tooling remains 0.28.2 at `5cf67f81c50c56ac17389ae71a3a87d46e86e8a2`
(145 canonical payloads). Logging 0.24.5, Testkit 0.83 and facade/PocketIC 0.20
resolve the old client-identity blocker. Other supplier lines are Metrics 0.28,
Runtime/macros/guard 0.19, Types 0.18, Storage 0.66, Wire/Derive 0.16 and Endpoint
0.6. Rust 1.99.1 remains pinned. Runtime's exact guard and typed rejection versus
transport-failure distinction remain intact; uncertain effects gain no retry.
No retired ic-memory, ic-stable-structures or ic-testkit-native is in the graph.

`target/current-tooling-observability-update` retains the preceding snapshot,
Logging and Testkit adoption: graphs/lint, three Logging unit cases, three exact
installed Timers subjects, three installed Jobs cases and group metadata fixtures.
Installed tests use fresh/same-source upgrades, not deployed old-Wasm migration.
Earlier `target/current-cohort-update` evidence remains bound to Testkit 0.82 /
PocketIC client 0.19: 268 native cases, affected native/Wasm Clippy, all 35 installed
watchdog subjects/cohorts, six Jobs cases and the normalized Timers archive.
Do not relabel that evidence as complete qualification of the new native client.
Earlier blocked proposals remain at `target/current-pocketic-update`,
`target/current-upstream-update` and `target/current-adopt-deploy`.
No sibling source or release was changed.

## Pending release scope

Public Metrics type identity changes require Timers minor notes: the pending
entry is now 0.39.0, including the earlier cleanup and group-CI work. Consumers
exchanging measurement values select Metrics 0.28 and align their Runtime/Endpoint
suppliers. Jobs' pending entry stays 0.34.2; its published Serde-only API is
unchanged. Actual package versions remain Jobs 0.34.1 and Timers 0.38.1.
Implementation batches stayed local; source commits are now authorized for
release preparation. No new release tags, pushes or publication have occurred.

The earlier deployment request covers both Scheduling groups and crates.io;
complete release gates and final publication admission remain required. Shared
Tooling 0.28 additionally requires explicit authorization for the direct push
from this checkout's main to dragginzgame/ic-scheduling main. A configured direct
default alone is insufficient; follow the adopted contribution/delivery contract.
Do not publish the modified tree under an already published package version.

## Group validation and preserved work

[Group plans](../release-validation-groups.md) remain adopted locally. Clean
Jobs-owned changes since its annotated current tag select complete native/Wasm
CI without standalone Timers tool/server/watchdog/cohort stages. Timers, shared,
manifest/lock/compiler changes and unknown/dirty scope retain all five stages.
This mixed adoption therefore needs the full gate for either release group.
Hosted Linux/macOS Apple Silicon/Intel and combined-consumer checks are unchanged.

Plan/policy, checkpoints, native/compiler/build settings and actual stage evidence
are bound to durable group receipts and re-admitted during preparation/recovery.
No proof reuse or historical-receipt conversion is enabled. Selector/rejection,
Jobs narrow-plan and Timers full-plan metadata/journal/local-delivery fixtures
passed; product commands/uploads were substituted. Their evidence remains at
`target/current-group-ci-adoption`. New-graph metadata fixtures also pass above.
Shared's new generic evidence/roster/baseline helpers are available; the existing
Scheduling receipt admission remains until a separately qualified migration.
Snapshot adoption alone does not remove local recovery/version-projection checks.

The ten earlier simplifications remain in the normal checkout. Their
[measurements](../redundancy-measurements-2026-11-03.md) use the older frozen
Runtime 0.18 graph, not the newly adopted graph. Keep those comparisons distinct;
`target/current-redundancy-ten` retains their source/artifact evidence.

## Delivered state and durable custody

Last delivered Timers is 0.38.1 at `4de549f2ebfeec98c7ffdda2fab1e50c86fc9842`
(preparation `7560270`); Jobs is 0.34.1 at
`db1a35afd3ad076d8f8eab4f43b5d277e2f16ed2`. Delivered main handoff was
`1e9821379a38f6f2b8a3b893d5987210ef48e933`. No completed release is replayed.
Timers owns the sole private provider/unsafe FFI boundary. Jobs remains Serde-only
with consumer-owned metadata, storage and lifecycle; every restore uses `Job::restore`.
Retained 0.64 Cell images preserve blocked attempts/receipts under Storage 0.66;
retired Memory images remain refused without writes/reset. No consumer migration
or destructive retirement is authorized without disposition of outstanding effects.

Preserve `.git/release-state/deployment-timers0381.jnjcs5qg/`,
`.git/release-state/deployment-upstreams.UjAXfFHz/`,
`.git/release-state/deployment-msrv-87.uUHPASl6/` and
`.git/release-state/deployment-wire-tooling.w6Z6gClk/`, including journals,
receipts, registry archives and incoming work. Do not pop preservation stashes:
they contain already delivered source/old versions. Historical detail remains at
`1e9821379:docs/status/current.md` and `af0e67d:docs/status/current.md`.
Development evidence under target remains subject to normal clean.
