# Current handoff

## Deployment authorized — 2026-11-05

Deliver this checkout's complete pending batch from `main` to `origin/main`,
then publish Timers 0.40.7 followed by Jobs 0.34.6. Both require fresh complete
37-stage gates; no proof reuse is enabled. Logging 0.24.8 and Metrics 0.29.3
arrived after the focused adoption evidence below and are included for fresh
qualification. Siblings remain read-only. Release/publication is not yet complete.

## Testkit 0.87 adoption pending — 2026-11-05

The requested update selects published Testkit 0.87.0 and Platform 0.23.0,
resolving Testkit 0.86's former facade blocker. Runtime/macros/exact guard remains
0.19.2 and Types 0.18.1. The targeted resolution preserves all other incoming
supplier selections and removes six unused synchronization/signal packages.
Root-owned Tokio is optional behind the Jobs harness feature and a Timers native
dev dependency; it is absent from default capture and deployed normal graphs.

Jobs recovery, queue and Timers runtime/cohort callers now own async runtimes,
await network/progress operations and explicitly close instances before their
servers stop. Queue command, initialization and cleanup failures remain visible;
updates gain no automatic replay. Public Jobs/Timers APIs, Wasm timer ABI,
restore admission and uncertain-effect disposition remain unchanged.

Focused Rust 1.99.1 Linux evidence: strict native/Wasm Clippy, six native Jobs
cases, supplier/provider/Candid graph checks, 11 installed Jobs recovery cases,
35 Timers subjects, policy cohorts/calibration, the queue scenario and all five
terminal shutdown modes pass. Testkit CLI 0.87 setup/offline admission and the
PocketIC 16.1.0 bundle check pass. Raw logs/input hashes are under
`target/evidence/testkit-adoption/`; runtime receipt:
`target/tmp/timer-qualification-3865447-0/runtime.json`; cohort receipt:
`target/evidence/testkit-adoption/cohorts.json`; Jobs recovery custody:
`target/tmp/jobs-recovery.pwXl26/`. No full CI or fresh native macOS execution is
claimed. Pending notes remain Jobs 0.34.6 / Timers 0.40.7; package versions are
unchanged. No commit, push, release or publication was performed for this update.

## Shared Tooling 0.30.4 adoption pending — 2026-11-05

The prior requested refresh uses committed
`2aa41c7394300302fe802b69b31b855bdc89b671` through the canonical exporter;
all 149 payloads match. Scheduling runs the shared lexical Candid check once
within common boundary admission, before Cargo checks, with an empty allowance
policy. The 37-stage roster and release custody remain unchanged. Shared already
owns full runner proof parsing; no local parser migration was needed.

The [source/evidence guide](../fleet-tools.md#documentation-and-experimental-evidence)
applies compact-topic retention and `target/evidence/` raw-output guidance.
The docs inventory found no tracked Wasm or compressed archives. Historical
ledger compaction is identified; no history/recovery deletion was performed.
Focused snapshot, links, actual Make admission/refusal, cloc, formatting/hooks,
ShellCheck and actual/undated ledger projection evidence remains under
`target/evidence/shared0304/`. Prior Platform-only evidence under
`target/platform-adoption-20261105/` retains its original 0.22.3 scope; the new
Testkit evidence above supersedes its resolved blocker, not its source identity.

## Latest completed delivery

Jobs 0.34.5 and Timers 0.40.6 remain committed, tagged, pushed and published.
The [delivered handoff](https://github.com/dragginzgame/ic-scheduling/blob/9414cad9667822ee26ac02d6f41cf639bc2affd4/docs/status/current.md)
retains exact source/tag/checksum identities, supplier graph and qualification.
Both complete 37-stage gates passed independently, each with 35 installed Timers
subjects and cohorts. Publication separately verified lint and official archives.
Current pending work above is separate from those published artifacts.

Preserve `.git/release-state/deployment-cleanup0406.4fc5n5eC/`, completed
`timers-0.40.6.plan` / `jobs-0.34.5.plan` and version-2 receipts. Custody retains
inputs, logs, diagnostics, official archives and completion records. Attempts
`validation.v9g365AV` and `validation.z7wdeMUo` remain under
`target/release-validation/`; preparation remains under
`target/deployment-cleanup0406/`. The failed first attempt correctly refused an
in-flight Wire lock change; its evidence is not a pass. Wire 0.17.2 was restored
after delivery and is now part of the pending qualified graph above. Preserve
stash `0f004eda7ab7e02c96edb801416838dff89a4c72` and custody's Wire-only patches;
never pop old stashes or replay completed uploads. Other writes were permitted
to resume after that completed delivery.

## Earlier deliveries and protected custody

Jobs 0.34.4 / Timers 0.40.5 remain published. Their
[delivered handoff](https://github.com/dragginzgame/ic-scheduling/blob/5e9a41509d40e4537e49c61b6d1430f84e4f329e/docs/status/current.md)
retains identities and the Shared audit. Preserve
`deployment-upstreams0405.xgEN56MD/` under `.git/release-state/`.
The [Timers 0.40.4 handoff](https://github.com/dragginzgame/ic-scheduling/blob/0c3f310a17a1a0ea03f9c6e030dbb9559a448332/docs/status/current.md)
and [0.40.2/earlier handoff](https://github.com/dragginzgame/ic-scheduling/blob/af1ff7db20125e29edc82ad9323c317b6cd1de6e/docs/status/current.md)
retain older delivery identities, frozen measurements and qualification limits.

Preserve `.git/release-state/` custody `deployment-timers0404.OHlQFZ45/`,
`deployment-timers0403.XBMtMHVE/`, `deployment-timers0402.G6arr0ab/`,
`deployment-consumer-support.lHbrHngp/`, `deployment-timers0400.IkyEFM1l/`,
`deployment-timers0391.k7IkTIwf/`, `deployment-current.0NCjlIWS/`,
`deployment-timers0381.jnjcs5qg/`, `deployment-upstreams.UjAXfFHz/`,
`deployment-msrv-87.uUHPASl6/` and `deployment-wire-tooling.w6Z6gClk/`, including
journals, receipts and archives. Preserve version-1/interrupted proofs and
stashes with their original source; do not relabel evidence or replay uploads.
Shared admission enforces retained byte/file/age budgets without implicit cleanup.

## Architectural boundaries

Eight members retain explicit versions across Jobs (three) and Timers (five).
Root manifests/lock own suppliers. Jobs stays Serde-only, consumer-owned and
validated through `Job::restore`; uncertain effects need explicit disposition.
Timers retains its sole private provider, FFI confinement and exact Runtime guard.
Canic owns reporting/application integration; copied Metrics observations create
no endpoint. Storage owns admitted Cells; retired Memory images are refused
without writes. Same-release upgrades do not prove old-Wasm migration or rollback.
See the [consumer checklist](../timers/adoption/0.40.md). Sibling repositories
remain read-only; dependency adoption authorizes no supplier release.
