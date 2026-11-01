# Current handoff

## Platform 0.16 delivery complete — 2026-11-01

The requested adoption is delivered on origin/main and both crates are published:
[IC Timers 0.35.3](https://crates.io/crates/ic-timers/0.35.3) and
[IC Jobs 0.33.3](https://crates.io/crates/ic-jobs/0.33.3).
Both canonical ledgers are finalized; no pending release remains from this batch.
The original checkout was synchronized to the delivered source with hash guards;
its full candidate and incoming dependency edits were preserved and incorporated.
No sibling checkout or Canic source was changed.

The graph selects Platform/macros/guard 0.16.0, canonical Types 0.15.1,
Endpoint 0.3.0, Memory/Storage 0.60.6, Testkit/Native 0.76.0 and Wire/Derive 0.13.3.
Build/Host 0.73.4, Logging 0.23.5 and Metrics 0.24.4 were retained in the qualified
lock. Timers keeps one executor/IC0 provider and exact matching guard. Jobs stays
Serde-only; the Platform integration belongs to its private qualification app.
Public APIs, stored formats and timer behavior are unchanged by this adoption.

## Release and publication identities

Adoption source is `d71acc27aab91f03bb1a5e7403cf84a93018d7ff`.
Timers tag `timers-v0.35.3` identifies
`4b9c407ba784d76daf1065cd9f78e2fc288a202c`; annotated tag object is
`ace0b118aabc8e3a9edcebd6b56defc1ae08e271`.
Jobs tag `jobs-v0.33.3` identifies
`fd153791b1293f38238da83491fc112c8412ea28`; annotated tag object is
`a947ac461318d06461a7149cca1d26f70b0d0b14`.
Each standard release atomically pushed main and its exact tag, followed by the
separate authorized publication command. Fresh GitHub readback confirms both tags.

Fresh registry metadata confirms both versions are present and not yanked.
Downloaded archives match local uploaded archives and embedded release VCS identity:
Timers SHA-256 `4ea122bd1ef28da459d084558eb83d55c2c89563cfdf6fe308cfca380b075f3d`;
Jobs SHA-256 `a085969d20bd21f15b6f76f32b4fcf11cb118b5b674aff95054ca12c8717c042`.
Do not repeat uploads, rewrite tags or replay completed delivery effects.

## Validation and retained evidence

Both standard complete release gates passed: combined CI, strict Clippy,
Rust 1.91/native/Wasm, packaging, tooling/recovery fixtures, the 44-case watchdog
suite and the policy-cohort promotion gate. Timers CI took 404 seconds; Jobs CI
310 seconds. Qualification sources are the adoption commit and Timers release
commit respectively; the standard metadata receipts bind final versioned payloads.
Jobs' separate installed recovery qualification passes all 11 subjects on original,
O3, Os and Oz Wasm, including intent/receipt recovery, upgrade and rollback.
This is Linux x86_64 and installed IC evidence. Hosted CI runs asynchronously;
local qualification does not establish native macOS results.

Delivery checkout and evidence: `/tmp/ic-scheduling-platform-delivery.e3mlrmqt/`.
Keep `source/` and both complete journals under `source/.git/release-state/` for
exact-attempt reconciliation. `timers-release.log`, `jobs-release.log` and both
publish logs retain commands and effects. Per-group `*-publication-receipt.json`,
`*-registry-after/` and original `.crate` files bind public payload identities.
Per-group `*-release-evidence/` preserves journals, final locks, source archives
and tag identities. `adoption-source.tar` and `adoption-Cargo.lock` preserve the
preparation source separately from the final versioned graphs.

`timers-qualification/` and `jobs-release-qualification/` preserve each original
runtime/cohort diagnostic set, all five Wasms/build stamps, server output, reports
and checksum manifests. `jobs-live-qualification/` preserves all four exact Jobs
Wasm variants, capture records, server logs and successful live tests. Original
live evidence also remains `/tmp/jobs-recovery.lc1IRO/`. `candidate/`,
`initial-identity.json` and `checkout-sync-receipt.json` preserve checkout custody.
Do not discard original inputs or substitute rebuilt artifacts for saved evidence.

Focused adoption evidence remains `/tmp/ic-scheduling-platform-retry.5hinanvm/`.
Earlier unavailable-adapter attempts remain `/tmp/ic-scheduling-platform016.90riv96s/`
and `/tmp/ic-scheduling-platform-retry.rnr3b32a/`; their failures are historical.
Prior Timers 0.35.2 and 0.35.1 release custody remains respectively
`/tmp/ic-scheduling-timers0352.AO0pyy/` and `/tmp/ic-scheduling-timers0351.7OO3LK/`.
The [prior committed handoff](https://github.com/dragginzgame/ic-scheduling/blob/fd153791b1293f38238da83491fc112c8412ea28/docs/status/current.md)
retains older source identities, issue #84/PR #85 delivery and recovery locations.

## Boundaries and separate consumer work

The prior Canic Jobs proof is [PR #722](https://github.com/dragginzgame/canic/pull/722),
source `4759bec` from Canic 0.114.0. Its evidence remains
`/tmp/ic-scheduling-improvements.VvCLTy/`; no Canic merge, release or adoption is
selected here. Sibling checkouts remain read-only. The older checkout
`/tmp/canic-jobs-adapter.dPFVZ7FD/source` is unavailable; do not reuse its claims.

Shared 0.27.23 at `619d194875369e0f72b45477b66adb430acbf6b7` remains the intact
132-file reviewed snapshot. Keep all nine members in their root catalog/group
alias rosters, with independent group versions and no shared root version.
Require explicit RELEASE_GROUP for release/resume/publication; direct delivery
uses origin/main and upload remains separate. Preserve saved source, dependencies,
policy, payload and receipts on recovery. All restores use Job::restore; retained
Running/Uncertain effects stay blocked until exact evidence or owner disposition.
Preserve bounded queue bytes, immutable payloads, attempts and allocation custody.
Logging stays heap-only at 32 KiB; Metrics reset neither clears logs nor reuses IDs.
Timers owns the sole private provider with confined FFI. Rust floor stays 1.91;
native execution alone never qualifies IC recovery. Sibling Timers source remains
provenance, not a build input; merge custody is `/tmp/ic-scheduling-merge.7PRSi3Nq/`.
