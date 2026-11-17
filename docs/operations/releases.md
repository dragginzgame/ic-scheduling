# Scheduling releases

The product groups are `timers` and `jobs`; qualification packages never publish.
The canonical manifests own independent versions and `releases/` owns their notes.
Toko owns release sequencing, capture, recovery, explicit staging and atomic push.
The local hooks own the complete crate-specific gate and source-bound evidence.

Prepare Rust 1.100.0/Wasm, Node, cached dependencies, a PocketIC 16.0.0 server and
an Entropy repository containing the immutable revision required by its
qualification adapter.
Export absolute `POCKET_IC_BIN` and `ENTROPY_SOURCE` paths. Release setup fetches
both existing lockfiles with `--locked`, installs pinned tools and selects the
published `ic-cargo-metadata` 0.1.0 executable. Checks then run offline. Recovery only
admits saved tools; it does not refresh dependencies or repair locks.

The standalone renderer replaces the displaced Build 0.73.0 CLI selection,
resolving
[Tooling #8 — Supply a released successor for the displaced Cargo release metadata editor](https://github.com/dragginzgame/ic-tooling/issues/8).
Install package/bin `ic-cargo-metadata` 0.1.0 in release profile without features.
Toko's admitted absolute executable selection, bounded-input contract,
transaction/rollback ownership and independent Cargo graph checks are retained.

For Timers 0.45.10/0.45.11 and Jobs 0.38.0/0.38.1, the maintainer waived the
Entropy journey. Their release gates require `POCKET_IC_BIN` but not
`ENTROPY_SOURCE`; all Scheduling-owned checks and archive checks remain required.
Ordinary `make qualify` and subsequent releases retain Entropy qualification.

`make qualify` runs formatting/tool admission, strict Clippy early, native tests,
Wasm documentation, canonical hook qualification, installed failure/lifecycle/resource checks, the isolated
Entropy journey and archive checks. The Toko validation runner retains failures.
Source tests and extracted archive tests have separate responsibilities. Cargo
1.100's temporary registry limitation is recorded in the candidate evidence.

The standard commands are `make release-patch`, `release-minor` and
`release-major`, with an explicit `RELEASE_GROUP=timers` or `jobs`. A complete pair
uses `RELEASE_GROUPS=timers,jobs`, supplier first. All increments run the same gate.
The runner advances the canonical manifest version; pending changelog headings
must match that next version. Clean committed source is required before capture.
Only direct delivery to the selected GitHub branch is configured.

Publication uses the selected group's `make publish-check` / `make publish`;
`PACKAGE` may narrow only within that group. It requires clean tagged source and
retains exact-command evidence under ignored `target/publication`. Do not retry
an uncertain upload: inspect the retained intent and registry first. The standard
batch runner owns saved-attempt recovery (`release-batch-status`,
`release-batch-resume`, and explicit reconciliation).

The first release completed both groups and verified registry readback. Jobs
registry verification extracts outside the product workspace; the original
inside-workspace extraction was corrected after its pre-upload refusal. See the
[delivery record](../evidence/release-candidate.md) for the completed recovery.

The native CI matrix is Ubuntu 24.04 x86-64 and the GitHub-hosted macOS 15 ARM64 runner. It qualifies locked setup, format,
native tests, Wasm compilation, archives and metadata transactions. Full installed
PocketIC and pinned Entropy qualification currently runs on the prepared Linux
release host. CI runs one redacted repository-history secret scan, outside the
host matrix. A Linux pass alone does not claim macOS qualification.

CI setup phases run through `ci/setup.mjs`. Failures retain sanitized final
64 KiB diagnostics and at most four explicitly reported Rust attempt logs of
64 KiB each, without following attempt symlinks or uploading tool stores. The
workflow uploads only `target/ci-setup/`, keeps artifacts for seven days and
links the artifact in its result. Missing logs, collection failures and upload
failures are reported separately; the original setup failure remains fatal.
Run `node qualification/setup-check.mjs` when changing this boundary. Its offline
failure rehearsal covers redaction, hidden attempt selection, missing output,
symlink refusal, truncation and collector failure. Hosted upload/retrieval is a
separate observation, not established by that local rehearsal.
