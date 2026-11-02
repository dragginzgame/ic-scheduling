# Current handoff

## Delivered Timers maintenance — 2026-11-02

Timers 0.38.1 was pushed to `origin/main` with annotated tag `timers-v0.38.1`
and published to crates.io. Its release commit is
`4de549f2ebfeec98c7ffdda2fab1e50c86fc9842`; source preparation is `7560270`.
The downloaded non-yanked registry archive matches its official checksum,
exact release source, original manifest and Rust 1.99.1 metadata.
Jobs remains at published 0.34.1; no other repository was changed or released.

This compatible patch corrects adoption guidance and graph commands. Watchdog
control and pure scheduling/completion decisions now live in a private registry
child. Parent registry claim authority, entry mutation and provider-handle
ownership are unchanged, as are public APIs, timer semantics and error ordering.
The extracted blocks retain identical code after visibility/formatting normalization.

All 175 focused Timers tests passed, including 24,576 generated command traces
with provider admission/binding faults and suspended-completion failure and
abandonment cases. Strict native/Wasm Clippy, documentation links, provider
confinement, graph commands and package-file inclusion passed.
The complete release gate separately passed full local Linux CI, all 35 installed
PocketIC watchdog cases and the policy-cohort comparison. Publication repeated
clean-source, exact remote-tag and full Clippy admission, then verified the archive.

[Hosted patch CI](https://github.com/dragginzgame/ic-scheduling/actions/runs/41517675149)
was running at the recorded observation. Earlier `af0e67d` CI passed Linux,
macOS Apple Silicon and Intel; that result predates this maintenance patch.
No fresh macOS run or separate live Jobs recovery matrix is claimed here.

## Dependency and consumer boundary

This patch retains the previously delivered supplier lock: Runtime/guard 0.18.1,
Types 0.17.0, Wire/Derive 0.15.0, Endpoint 0.5.0, Storage 0.65.0, Metrics 0.27.1,
Logging 0.24.2, Testkit 0.80.1, Build 0.74.0, Host Artifacts/FS/Process 0.74.1 and
native Platform PocketIC 0.18.0. Shared snapshot 0.27.28 at
`8c7fae6605c69a930e3b1f14e1ff5c4c9dafbcb7` is unchanged; all 142 payloads verify.
Standard tools and PocketIC 16.1.0 were admitted in the isolated checkout.
Both groups retain the complete release gate; optional group narrowing/reuse is
not activated. Rust 1.99.1 remains the floor and pinned compiler for all nine members.

Timers 0.38 requires one aligned Runtime 0.18 executor and exact matching guard.
Consumers retaining Runtime 0.17 must update all executor/endpoint suppliers
together; Timers alone cannot resolve that graph. Consumers exchanging public
measurement types also select Metrics 0.27. The guard must not be overridden.
Jobs remains Serde-only and consumer-owned for storage, metadata and lifecycle.
Timers retains the sole private IC timer provider and unsafe FFI confinement.
No ic-memory, ic-stable-structures or ic-testkit-native remains in the delivered lock.

The qualification consumer uses Cells, not Storage 0.65's changed map format.
Its real 0.64 image with Running/Uncertain attempts and exact receipts reopens on
0.65/Wire 0.15/Types 0.17 without writes, growth or redispatch. Bounded Cell loading,
validated Job::restore and refusal ordering remain intact. Empty recovery never
creates replacement state. The older 0.63/Memory image stays refused without
writes/reset; its unchanged Wire payload preserves blocked jobs. Earlier JSON/CBOR
images retain provenance/refusal coverage. Native Cell continuity establishes
neither old-map compatibility nor installed consumer recovery/rollback.
Retiring incompatible installations still requires disposition of outstanding
effects and explicit authorization. This release migrates or deploys no consumer.

## Evidence and checkout custody

`.git/release-state/deployment-timers0381.jnjcs5qg/` retains incoming source/tree
hashes, focused checks, release/publication logs, journals, validated/prepared
receipts, installed runtime diagnostics, cohort receipt, official registry archive
and verification receipt, hosted-CI observation and synchronization evidence.
The source remains at `target/upstream-release-source.jyvBPzEt/`.
Do not replay completed increments, tags or uploads.

The ordinary checkout was fast-forwarded to the delivered source. Concurrent
supplier-only Cargo.lock updates were captured and restored separately, retaining
all new release package rows. Those newer supplier selections were excluded from
this release and remain unqualified here. The complete pre-synchronization state
is also retained in the named stash identified by `preserved-maintenance-stash.txt`
in this deployment directory. Do not pop it: it includes already delivered source
and old release versions. Earlier preservation stashes/journals remain intact.

Earlier Jobs 0.34.1 and Timers 0.38.0 delivery evidence remains at
`.git/release-state/deployment-upstreams.UjAXfFHz/`. Their source adoption is
`8d6453f`; the Jobs release commit is `db1a35afd3ad076d8f8eab4f43b5d277e2f16ed2`.
`target/current-timers-maintenance` retains the original focused patch evidence;
`target/current-all-upstream-adoption` retains old-reader/adoption evidence.
[Issue #87](https://github.com/dragginzgame/ic-scheduling/issues/87) remains complete;
its delivery evidence is `.git/release-state/deployment-msrv-87.uUHPASl6/`.
Earlier context is preserved at `af0e67d:docs/status/current.md`; original
Wire/tooling evidence remains `.git/release-state/deployment-wire-tooling.w6Z6gClk/`.

Next work is separate qualification of the pending supplier updates and downstream
Runtime cohort adoption with consumer-owned installed recovery/rollback checks.
Library publication for this maintenance batch is complete.
