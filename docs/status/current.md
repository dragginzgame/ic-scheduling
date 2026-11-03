# Current handoff

## Shared validation consolidation — 2026-11-03

[#88 — Consolidate Jobs/Timers plan evidence](https://github.com/dragginzgame/ic-scheduling/issues/88)
is implemented locally in the normal checkout; delivery is pending. Shared's executable now owns
annotated-baseline/roster observation and run verification; its bounded JSON,
path and hash helpers replace the local copies. Scheduling retains product
selection, configuration/source custody and exact metadata projection.
The local event/timing parser is removed.

`ci/release-validation.json` is the sole full roster: 30 common obligations,
including the former snapshot prerequisite, and four standalone Timers stages.
Normal CI and clean Jobs-only releases execute the same common stages. Release
plans generate aggregate `ci` plus the four standalone stages from that roster;
Timers/shared/unknown/dirty scope requires all five, preserving the existing
full-source checkpoint cadence. Hosted Linux/macOS hosts and product
checks remain unchanged. Full CI runs once rather than looping over groups.

Version-2 receipts retain Shared's canonical proof and bind every evidence byte.
They support the existing Git-owned relocation and metadata preparation contract
without rerunning, rewriting or reinterpreting Shared evidence. Historical v1
receipts must use their original source/adapter. No reuse or receipt conversion
is enabled. See [the consumer mapping](../release-validation-groups.md).
Focused mapping, actual Make CI-drift and both groups' real metadata/journal
recovery checks pass. Final guards also reject incomplete source inventories and
generated policies. Product/setup/uploads are substituted in the release fixtures;
no new full CI or native host matrix ran for this tooling-only batch. Evidence is
under `target/current-issue-88`, with a durable copy in the delivery state below.
No product code, dependency selection or package version changed for this work.

## Delivery state

The owner requested Timers 0.39.0, then Jobs 0.34.2, with separate crates.io
publications. Actual package versions remain Timers 0.38.1 and Jobs 0.34.1.
Timers needs a minor release because public Metrics identities moved to 0.28;
consumers must align Metrics and Runtime/Endpoint suppliers. Jobs stays
Serde-only and its pending entry remains a compatible patch.

`/home/adam/projects/ic-scheduling` is on `main`, ahead of public main with local
source commits `643f9b8` and `af87eab`. Planned destination is
`origin/main` (`dragginzgame/ic-scheduling`), with tags `timers-v0.39.0` and
`jobs-v0.34.2`. The owner has now requested the direct push to `origin/main`;
commit and deliver the qualified #88 fix, then complete the established release
and publication workflow. No new release tags or publication have occurred. No supplier release,
alternate checkout or unsolicited PR is authorized.

At `af87eab`, full CI's 29 recorded targets plus the snapshot prerequisite,
the version-1 five-stage Timers gate, all 35 watchdog subjects and policy cohorts
passed. All 44 Jobs installed recovery cases passed across original/O3/Os/Oz
Wasm. Artifacts, receipts, logs and source custody are preserved under
`.git/release-state/deployment-current.0NCjlIWS/`. The owner selected Logging
0.24.5 after an unexpected lock change interrupted the earlier 0.24.4 attempt;
both locks and failed/interrupted evidence are preserved there. The published
0.24.5 Rust source matches 0.24.4 and its archive checksum was verified.

That qualification predates #88's adapter changes. Product source/dependencies
are unchanged, but fresh complete release admission must use the new generated plan and
receipt contract. Do not reuse or reinterpret the old group receipts. No new
macOS run or deployed old-Wasm migration is established by the local evidence.

## Supplier cohort and preserved implementation

Shared Tooling is 0.28.2 at `5cf67f81c50c56ac17389ae71a3a87d46e86e8a2`
(145 canonical payloads). Other selections are Logging 0.24.5, Testkit 0.83,
facade/PocketIC 0.20, Metrics 0.28, Runtime/macros/guard 0.19, Types 0.18,
Storage 0.66, Wire/Derive 0.16 and Endpoint 0.6. Rust 1.99.1 remains pinned.
Runtime's exact guard and typed rejection versus transport failure stay intact;
uncertain effects gain no retry. No retired Memory, Stable Structures or Testkit
Native crate is in the graph. Sibling repositories remain read-only.

The redundant support crate is folded into `ic-jobs-canister-tests::harness`,
behind the explicit `harness` feature. Recovery/queue/process targets and Make
launchers select it. The workspace has eight members, three in Jobs. Capture's
default normal/build graph remains byte-for-byte unchanged and lightweight.

The ten simplifications remain in source. Their
[measurements](../redundancy-measurements-2026-11-03.md) use the older frozen
Runtime 0.18 graph, not the current graph. Earlier development evidence under
`target/` was cleaned before delivery preparation; retain the Git-owned evidence
above and the measurements' original source/artifact bounds.

## Delivered state and durable custody

Last delivered Timers is 0.38.1 at `4de549f2ebfeec98c7ffdda2fab1e50c86fc9842`
(preparation `7560270`); Jobs is 0.34.1 at
`db1a35afd3ad076d8f8eab4f43b5d277e2f16ed2`. The last recorded public main is the delivered
handoff `1e9821379a38f6f2b8a3b893d5987210ef48e933`; completed uploads are not replayed.
Timers owns the sole private provider/unsafe FFI boundary. Jobs retains
consumer-owned metadata, storage and lifecycle, always restoring through `Job::restore`.
Retained 0.64 Cells preserve blocked attempts/receipts under Storage 0.66;
retired Memory images remain refused without writes/reset. No consumer migration
or destructive retirement is authorized without disposition of outstanding effects.

Preserve `.git/release-state/deployment-timers0381.jnjcs5qg/`,
`.git/release-state/deployment-upstreams.UjAXfFHz/`,
`.git/release-state/deployment-msrv-87.uUHPASl6/` and
`.git/release-state/deployment-wire-tooling.w6Z6gClk/`, including journals,
receipts, registry archives and incoming work. Do not pop preservation stashes;
they include delivered source/old versions. Historical detail remains at
`1e9821379:docs/status/current.md` and `af0e67d:docs/status/current.md`.
Development evidence under target remains subject to normal clean.
