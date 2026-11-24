# Entropy integration qualification

This is a reviewable migration draft, not an adopted Entropy change. The source
repository remains read-only. Entropy is the stable consumer target; Canic is
being rewritten and needs a separate integration review when its API settles.

`prepare.mjs` requires a local Entropy repository containing commit
`6c400b2c0259323ad1464fc692a4c4c378a2b101`. It reads only that immutable commit,
regardless of the checkout's current branch or working changes. It materializes
tracked product and qualification sources under ignored
`qualification/target/entropy-integration/`,
applies `timers.patch`, and adds local Cargo overrides for the Timers candidate.
It does not copy Entropy's release machinery or modify its checkout. The patch
changes the dependency cohort, target gates, retained timer adapter and the
probe's explicit Runtime/Wire codec and Testkit startup callers.
The existing consumer tests run from that isolated copy.

Run from the Scheduling root with prepared Rust/Wasm tools and a PocketIC 16.0.0
server. Local loopback access is required for installation. With dependencies
cached, set `CARGO_NET_OFFLINE=true` explicitly for an offline run.

```sh
entropy_stage="$(node qualification/entropy-integration/prepare.mjs ../ic-entropy)"
entropy_build="$PWD/qualification/target/entropy-integration-build"
entropy_installed="$PWD/qualification/target/entropy-installed-build"
bash vendor/toko-tooling/scripts/ci/run-nonempty-cargo-test.sh \
    --manifest-path "$entropy_stage/Cargo.toml" --target-dir "$entropy_build" \
    --workspace --all-features
bash vendor/toko-tooling/scripts/ci/run-nonempty-cargo-test.sh \
    --manifest-path "$entropy_stage/Cargo.toml" --target-dir "$entropy_build" \
    -p ic-entropy-ic --no-default-features --locked
for entropy_features in '--all-features' '--no-default-features'; do
    cargo clippy --manifest-path "$entropy_stage/Cargo.toml" --target-dir "$entropy_build" -p ic-entropy-ic "$entropy_features" --all-targets --locked -- -D warnings
    cargo clippy --manifest-path "$entropy_stage/Cargo.toml" --target-dir "$entropy_build" -p ic-entropy-ic "$entropy_features" --target wasm32-unknown-unknown --locked -- -D warnings
done
cargo build --manifest-path "$entropy_stage/qualification/Cargo.toml" --target-dir "$entropy_installed" -p entropy-probe --target wasm32-unknown-unknown --release
POCKET_IC_BIN=/absolute/path/to/pocket-ic \
ENTROPY_PROBE_WASM="$entropy_installed/wasm32-unknown-unknown/release/entropy_probe.wasm" \
bash vendor/toko-tooling/scripts/ci/run-nonempty-cargo-test.sh \
    --manifest-path "$entropy_stage/qualification/Cargo.toml" --target-dir "$entropy_installed" \
    -p entropy-probe --test installed --locked -- --nocapture
```

The first resolution in each isolated workspace updates its copied lockfile for
the local override; subsequent checks are locked. The helper never reads working-tree
source: review the consumer changes and patch before changing the pinned revision.
The shared Cargo test runner preserves test failures and rejects selections that
execute zero passing tests, including the fresh-source tests without buffering.
Actual adoption also needs an approved dependency source and reviewed lock changes;
the first published rewrite is Timers 0.45.9. There is no compatibility facade.

The patch targets the Runtime 0.23 / Types 0.21 / Protocol 0.3 cohort and the
Timers 0.46 series. Preparation binds Timers to the actual local candidate
version before applying the local path override, including during the standard
pre-bump release gate. The source repository remains unchanged.

Runtime 0.23.14 publishes the replicated-execution observation required by the
consumer's query-refusal guard, resolving
[Platform #19 — Expose replicated-execution observation for Entropy's source admission](https://github.com/dragginzgame/ic-platform/issues/19).
The patch preserves the typed refusal before source claim/submission, using the
public boolean observation without admitting a task or mutating state. Both
fixture workspaces select Runtime 0.23.25, Types 0.21.4, Protocol 0.3.8 and
Toko Tooling 0.46.3. The installed probe uses Testkit 0.110.4 and Wire 0.24.6.
Wire framing now uses root-level `encode` /
`decode` with an explicit method identity and request/response direction;
the existing frame IDs, schemas and resource limits are retained. Actual Entropy
source remains unchanged. See the dated evidence for the tested selections.

Both direct fixture catalogs and Scheduling's native held-reply graph select
Protocol 0.3.8. Testkit 0.110.4 admits Protocol `>=0.3.2, <0.4.0`, superseding
the earlier exact-patch compatibility fix from
[Tooling #9 — Admit published Protocol 0.2.7 in Testkit's held-reply fixtures](https://github.com/dragginzgame/ic-tooling/issues/9).
The shared type and runtime singleton boundaries remain qualified independently
in both fixture workspaces; no transitive override is needed.

See [recorded evidence](../../docs/evidence/entropy-integration.md) for results,
hashes and the remaining lifecycle limits.
