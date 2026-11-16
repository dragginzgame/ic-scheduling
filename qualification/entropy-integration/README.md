# Entropy integration qualification

This is a reviewable migration draft, not an adopted Entropy change. The source
repository remains read-only. Entropy is the stable consumer target; Canic is
being rewritten and needs a separate integration review when its API settles.

`prepare.mjs` requires a local Entropy repository containing commit
`6c400b2c0259323ad1464fc692a4c4c378a2b101`. It reads only that immutable commit,
regardless of the checkout's current branch or working changes. It materializes
tracked product and qualification sources under ignored
`qualification/target/entropy-integration/`,
applies `timers.patch`, and adds local Cargo overrides for unpublished Timers.
It does not copy Entropy's release machinery or modify its checkout. The patch
changes the dependency version, target gates and buffered timer adapter only.
The existing consumer tests run from that isolated copy.

Run from the Scheduling root with prepared Rust/Wasm tools, cached dependencies
and a PocketIC 16.0.0 server. Local loopback access is required for installation.

```sh
entropy_stage="$(node qualification/entropy-integration/prepare.mjs ../ic-entropy)"
entropy_build="$PWD/qualification/target/entropy-integration-build"
entropy_installed="$PWD/qualification/target/entropy-installed-build"
cargo test --manifest-path "$entropy_stage/Cargo.toml" --target-dir "$entropy_build" --workspace --all-features --offline
cargo test --manifest-path "$entropy_stage/Cargo.toml" --target-dir "$entropy_build" -p ic-entropy-ic --no-default-features --locked --offline
for entropy_features in '--all-features' '--no-default-features'; do
    cargo clippy --manifest-path "$entropy_stage/Cargo.toml" --target-dir "$entropy_build" -p ic-entropy-ic "$entropy_features" --all-targets --locked --offline -- -D warnings
    cargo clippy --manifest-path "$entropy_stage/Cargo.toml" --target-dir "$entropy_build" -p ic-entropy-ic "$entropy_features" --target wasm32-unknown-unknown --locked --offline -- -D warnings
done
cargo build --manifest-path "$entropy_stage/qualification/Cargo.toml" --target-dir "$entropy_installed" -p entropy-probe --target wasm32-unknown-unknown --release --offline
POCKET_IC_BIN=/absolute/path/to/pocket-ic \
ENTROPY_PROBE_WASM="$entropy_installed/wasm32-unknown-unknown/release/entropy_probe.wasm" \
cargo test --manifest-path "$entropy_stage/qualification/Cargo.toml" --target-dir "$entropy_installed" -p entropy-probe --test installed --locked --offline -- --nocapture
```

The first resolution in each isolated workspace updates its copied lockfile for
the local override; subsequent checks are locked. The helper never reads working-tree
source: review the consumer changes and patch before changing the pinned revision.
Actual adoption also needs an approved dependency source and reviewed lock changes;
the first published rewrite is Timers 0.45.9. There is no compatibility facade.

The patch retains the initial `^0.45.4` requirement. The local override resolves
compatible later 0.45 versions from this checkout; the linked evidence
records the exact version originally qualified, not every subsequent build.

See [recorded evidence](../../docs/evidence/entropy-integration.md) for results,
hashes and the remaining lifecycle limits.
