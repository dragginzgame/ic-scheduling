# Current handoff

## Deployment authorized — 2026-11-06

Release and publish Timers 0.40.8 followed by Jobs 0.34.7 from this checkout's
`main` to `origin/main`. Both require fresh complete 37-stage gates. Incoming
`cc` 1.11.1 arrived after the focused proof below and is retained for fresh
qualification. Other selected supplier versions are unchanged. No retained
installation is reset; siblings remain read-only. Delivery is not yet complete.

The first full gate stopped at an installed assertion coupled to Wire's former
`wire Malformed` display text. Its inputs/logs/diagnostics are preserved under
`.git/release-state/deployment-upstreams0408.lbj697GX/`. The corrected subject
checks typed native malformation, IC trap and unchanged registered-timer state;
focused Clippy and that installed subject pass. A fresh source/full gate is
required; the failed gate is not qualification and no version/tag/push occurred.

## Build / Storage / Testkit adoption pending — 2026-11-06

The requested update selects published Build 0.75.0, Storage 0.68.0 and Testkit
0.88.0. Cargo registry metadata, exact published sources and targeted resolution
admit all three. Host stays 0.74.4; the incoming Platform 0.23.2 and Wire 0.17.3
selections are preserved. Runtime/macros/exact guard remains 0.19.2 and Types
0.18.1. Shared Tooling remains at reviewed 0.30.4; no unrelated refresh was made.

Build/Testkit keep every API used here. The new editor renders both group plans
without changing workspace metadata. Storage 0.68 replaces checksummed
`ICCELL01` Cells with `SCL1` framing. Use fresh qualification backing: previous
Cells are refused during recovery/creation with zero writes/growth and identical
bytes. Frozen metadata retains blocked attempts and exact receipts for explicit
owner disposition. No legacy reader, converter, reset or replay was added.
The bounded queue identity/codec and `Job::restore` validation remain unchanged;
valid-looking byte corruption is no longer covered by a collection checksum.
Native engine mutation panics require discarding the failed runtime/leases;
installed IC rollback remains a separate proof.

Focused Linux/Rust 1.99.1 evidence passes: native/Wasm strict Clippy, 93 native
consumer/harness/coordinator cases, supplier/provider/Candid graphs, 11 installed
Jobs recovery cases, one Timers trap/upgrade subject and policy cohorts. Both
updated executable selections and PocketIC 16.1.0 are admitted. The real Build
metadata editor renders the exact Jobs and Timers manifest/lock projections in
scratch; this is not a release transaction or full gate. Evidence is under
`target/evidence/adoption-20261106/`; runtime receipt:
`target/tmp/timer-qualification-3005854-0/runtime.json`; Jobs recovery:
`target/tmp/jobs-recovery.FRR5xM/`. No full CI or new native macOS execution is
claimed. Notes are Jobs 0.34.7 / Timers 0.40.8; package versions are unchanged.
No commit, push, release or publication was performed for this adoption.

## Latest completed delivery — 2026-11-05

Jobs 0.34.6 and Timers 0.40.7 remain committed, tagged, pushed and published.
The [delivered handoff](https://github.com/dragginzgame/ic-scheduling/blob/d4504c0fa657839c0a4125b61d7a96b6d145a502/docs/status/current.md)
retains exact source/tag/checksum identities, graph selections and qualification.
Both independent 37-stage gates passed, each including 35 installed Timers
subjects and cohorts. Jobs additionally passed 11 installed recovery cases,
the queue scenario and five terminal shutdown modes. Publication separately
verified lint and official archive/VCS identity. Do not replay uploads or relabel
that older graph as the pending adoption above.

Preserve `.git/release-state/deployment-testkit0407.f2yycLCV/`, complete
`timers-0.40.7.plan` / `jobs-0.34.6.plan` and their version-2 receipts. Custody
holds original inputs, publication logs, official archives/metadata and installed
receipts. Attempts `validation.JsfV9f9Y` / `validation.adU7Qux3` remain under
`target/release-validation/`; delivery logs under `target/deployment-testkit0407/`.
The initial uncached-dependency refusal is retained separately from passing proof.

## Earlier deliveries and protected custody

The linked delivered handoff retains older release identities, immutable handoff
links, measurements, audits and qualification limits. Preserve `.git/release-state/`
custody `deployment-cleanup0406.4fc5n5eC/`,
`deployment-upstreams0405.xgEN56MD/`, `deployment-timers0404.OHlQFZ45/`,
`deployment-timers0403.XBMtMHVE/`, `deployment-timers0402.G6arr0ab/`,
`deployment-consumer-support.lHbrHngp/`, `deployment-timers0400.IkyEFM1l/`,
`deployment-timers0391.k7IkTIwf/`, `deployment-current.0NCjlIWS/`,
`deployment-timers0381.jnjcs5qg/`, `deployment-upstreams.UjAXfFHz/`,
`deployment-msrv-87.uUHPASl6/` and `deployment-wire-tooling.w6Z6gClk/`, including
journals, receipts and archives. The old Wire-drift failure remains a failure;
retain stash `0f004eda7ab7e02c96edb801416838dff89a4c72` and custody's Wire-only
patches without popping them or restoring old versions. Preserve version-1 /
interrupted proofs with their original source. No implicit artifact cleanup is
authorized; Shared admission retains byte/file/age budgets.

## Architectural boundaries

Eight members retain explicit versions across Jobs (three) and Timers (five).
Root manifests/lock own suppliers. Jobs stays Serde-only, consumer-owned and
validated through `Job::restore`; uncertain effects need explicit disposition.
Timers retains its sole private provider, FFI confinement and exact Runtime guard.
Canic owns reporting/application integration; copied Metrics observations create
no endpoint. Storage owns admitted Cells; retired Memory images are refused
without writes. See the [consumer checklist](../timers/adoption/0.40.md).
Sibling repositories remained read-only; no supplier release was performed.
