# Current handoff

## Complexity batch delivered — 2026-11-01

All ten reviewed complexity improvements are delivered on origin/main. Both
canonical group ledgers are finalized and both crates are published:
[IC Timers 0.35.4](https://crates.io/crates/ic-timers/0.35.4) and
[IC Jobs 0.33.4](https://crates.io/crates/ic-jobs/0.33.4). No pending release remains
from this batch. Do not repeat uploads, rewrite tags or replay completed effects.
No sibling repository or Canic integration was changed.

The batch names ordinary completion outcomes and provider binding phases,
resolves watchdog schedules before mutation, unifies cohort selection and typed
measurements, admits whole build identity and names byte-compatible checkpoint
fields. It separates installed subjects from host-only utility tests and isolates
release-fixture phases. Jobs uses typed Wire RPC attributes with explicit IDs,
argument tuples and budgets. Oversized private requests now trap instead of
returning the former guard rejection; state-preservation assertions remain.
Published library APIs, queue formats, restoration and uncertain-effect policy
remain intact. Cargo dependencies and the reviewed tooling snapshot are unchanged.

## Release identities and registry verification

Preparation source is `54028ab5a7e41539348b0975e7a27573026a13ab`.
Timers tag `timers-v0.35.4` identifies `14917d09e7a55c16eda1b7d739f8213ac9d4209e`;
its annotated object is `aec700646fa316e7e7cf88c81b094f417d38cfaf`.
Jobs tag `jobs-v0.33.4` identifies `96f2d94e5af16ffb31c270d6b94a400caf917423`;
its annotated object is `9b1fdfe89c044a2ef6fc3d50ddb221e4ba223302`.
Timers was released/published first, followed by Jobs against Timers 0.35.4.
Each standard release atomically pushed main and its exact tag, followed by the
separate authorized publication command. Fresh GitHub readback confirms both tags.

Fresh registry metadata confirms both versions are present and not yanked.
Downloaded packages match the retained Cargo upload archives and embedded release
VCS identity. Timers SHA-256 is `ed9bf73e5cab8dff71c5b0b922cd61e587aa7212993e7c71929d8c060f61bf27`;
Jobs SHA-256 is `0285cc4debb0076968538599e96224febea2382b75fe2ed3454c8fd96b69596a`.
Cargo retained uploads under `target/package/tmp-crate/` and `tmp-registry/`;
verified independent copies are saved outside target in the delivery evidence.

## Qualification and custody

Both complete standard release gates passed: strict Clippy, full combined CI,
Rust 1.91/native/Wasm, formatting, packaging, graph/tooling/recovery fixtures,
all 35 installed timer runtime subjects and the policy-cohort gate. CI took
378 seconds for Timers and 279 seconds for Jobs. Source-bound qualification is
on the preparation commit and Timers release commit respectively; the standard
metadata receipts bind each final versioned payload.
Jobs' final installed recovery passes all 11 subjects on original, O3, Os and Oz
Wasm, including rollback, restore refusal, lost-result intent and receipts.
These are Linux x86_64 and installed IC observations. Hosted three-host CI runs
asynchronously; this handoff does not claim completed native macOS qualification.

Delivery checkout/evidence: `/tmp/ic-scheduling-complexity-delivery.y314l_zn/`.
Keep `source/` and its complete `.git/release-state/` journals. Per-group
`*-release-evidence/` retains copied journals, final locks/source archives,
runtime/cohort receipts, original Wasms/build stamps and server diagnostics.
`preparation-source.tar`, `jobs-preparation-source.tar` and their matching locks
retain the qualified inputs before each group's version preparation.
Per-group `*-publication-receipt.json`, `*-registry-after/` and uploaded/downloaded
`.crate` copies bind public payload identity. Publication was performed once
per group; later verification only observed those completed effects.

`jobs-live-qualification/` retains all four original Jobs Wasm variants, capture
records, optimizer identity and server/test logs. Original live evidence remains
`/tmp/jobs-recovery.HPXjXO/`. Original timer qualification directories are recorded
in each group's `identity.json` and remain alongside their copied diagnostics.
The metadata transaction cleans successful scratch backups under its existing
contract; Git, retained preparation archives and locks preserve their original bytes.

`incoming-source.tar`, `incoming-manifest.json` and `incoming.diff` preserve the
initial dirty checkout. The synchronization result is recorded in
`checkout-sync-receipt.json`; any retained Git stash is a backup of an already
incorporated batch and must not be replayed over delivered source. Development
proof remains `/tmp/ic-scheduling-complexity-rest.7bSQcO3E/` and
`/tmp/ic-scheduling-simplification.ut9fdf13/`; it does not replace release receipts.

## Retained boundaries and older delivery

Keep nine catalog/group members, explicit RELEASE_GROUP, origin/main direct
delivery and separately authorized upload. Jobs remains Serde-only and consumer
owned. All restores use Job::restore; Running/Uncertain effects stay blocked
until exact evidence or owner disposition. Preserve queue, payload and attempt
custody. Logging stays heap-only at 32 KiB; Metrics reset does not clear logs or
reuse IDs. Timers retains the sole private provider, confined FFI and matching
Platform guard. Rust floor is 1.91; native execution alone does not qualify IC
recovery. The Shared 0.27.23 snapshot remains intact; siblings stay read-only.

Earlier Platform 0.16 delivery custody remains
`/tmp/ic-scheduling-platform-delivery.e3mlrmqt/` and the
[committed prior handoff](https://github.com/dragginzgame/ic-scheduling/blob/af71b4b903f3cdc231afad5a56857f3c3692c9da/docs/status/current.md).
Separate Canic [PR #722](https://github.com/dragginzgame/canic/pull/722), source
`4759bec`, retains `/tmp/ic-scheduling-improvements.VvCLTy/`; no Canic delivery is selected.
