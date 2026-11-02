# Current handoff

## Delivered Rust 1.99.1 policy — 2026-11-02

[Issue #87](https://github.com/dragginzgame/ic-scheduling/issues/87) is completed.
The compiler-policy implementation is `a2a6cdd423843d8f7ce5f639c8aeac7711c953c6`.
Both independent groups are pushed to `origin/main` with annotated tags and
published, non-yanked crates.io versions:

| Group | Version | Release commit | Tag |
| --- | --- | --- | --- |
| Jobs | 0.34.0 | `41391e23c3116fe60901331cd95ac78ec2f11460` | `jobs-v0.34.0` |
| Timers | 0.37.0 | `1389d8e5e55ae3a8efbe6a49d66fc9164055be81` | `timers-v0.37.0` |

Independent registry archive downloads match their index checksums and exact
release commits. Both archived manifests and index records require Rust 1.99.1.
All nine members inherit that floor. Development and release use the same pin.
The local compiler projection admits actual Cargo/rustc identities and rejects
divergent or moving pins. Hosted setup prepares that compiler and Wasm once.

## Qualification and ownership

Each group passed its complete local Linux CI, the 35 installed watchdog cases
and the policy-cohort comparison before release preparation and publication.
Core, consumer, Wasm, proc-macro, archive and isolated-consumer coverage remains
in the gate. The Linux/macOS Intel/Apple Silicon workflow identities are retained;
these local results do not claim fresh native macOS execution or the separate
live Jobs recovery matrix. Independent core qualification remains explicit.

Jobs stays Serde-only. Timers owns the private IC timer provider and its confined
FFI. Restore paths retain validated Job::restore; uncertain effects remain blocked
until explicit disposition. This compiler change introduces no new storage or
scheduling policy. Shared executable snapshot 0.27.26 is unchanged.

The release used an isolated checkout with the committed dependency lock.
Unrelated working lock updates were excluded. Concurrent ordinary-checkout edits
to AGENTS.md and the consumer README remain user-owned; preserve them alongside
the lockfile edits during synchronization. No sibling repository was delivered.

## Durable evidence and recovery

The ordinary checkout's `.git/release-state/deployment-msrv-87.uUHPASl6/` retains
source/input snapshots, all attempt logs, registry archives and verification,
release journals/receipts, and source-bound installed qualification evidence.
See `delivery-receipt.json`, `release-journals/` and the runtime evidence archives.
Shared-store installer contention stopped two attempts without a version change;
standard private-store setup then passed every admission and complete gate.
No live lock was removed and no validation was bypassed.

The isolated delivery checkout is under `target/msrv-87/delivery/`; its temporary
scratch and private tool store are under its own target directory. Do not replay
completed increments or uploads. No cleanup was requested or performed outside
the reviewed installers' own successful scratch policy.

Earlier delivery and qualification narratives remain available in Git history at
`a2a6cdd423843d8f7ce5f639c8aeac7711c953c6:docs/status/current.md` and at their
existing evidence owners. Historical 1.91 artifacts were not relabelled or deleted.
The prior Wire/tooling delivery evidence remains under
`.git/release-state/deployment-wire-tooling.w6Z6gClk/` in the ordinary checkout.
