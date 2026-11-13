#!/usr/bin/env bash
# Shared companions: scripts/ci/check-timer-dependencies.sh scripts/ci/fixture-scratch.sh vendor/toko-tooling/scripts/ci/prepare-script-fixture.sh
set -euo pipefail
root="$(git rev-parse --show-toplevel)"
# shellcheck source=scripts/ci/fixture-scratch.sh
source "$root/scripts/ci/fixture-scratch.sh"
fixture="$(fixture_create_workspace "$root" "timer-library-boundary.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Failed library-boundary fixture retained"' EXIT

# Real Cargo resolution against the maintained catalog and lock. Empty fixture
# source is never built; no sibling source or network dependency is introduced.
bash "$root/vendor/toko-tooling/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture" \
    scripts/ci/check-timer-dependencies.sh scripts/ci/check-timer-provider.sh vendor/toko-tooling/scripts/ci/check-make-execution.sh vendor/toko-tooling/scripts/ci/check-cargo-dependency-boundaries.sh ci/workspace-manifests.txt \
    ci/cargo-dependency-boundaries.json
RUSTUP_AUTO_INSTALL=0 cargo metadata --locked --offline --format-version 1 > "$fixture/workspace.json"
platform_version="$(perl -MJSON::PP -0777 -e '
    my $metadata = decode_json(<>);
    my @platform = grep { $_->{name} eq "ic-platform-runtime" && $_->{version} =~ /^0\.22\.[0-9]+$/ }
        @{$metadata->{packages}};
    @platform == 1 or die "expected one selected runtime Platform\n";
    print $platform[0]{version};
' "$fixture/workspace.json")"
cp "$root/ci/workspace-manifests.txt" "$fixture/member-manifests"
manifests=(Cargo.toml Cargo.lock qualification/Cargo.toml qualification/Cargo.lock)
while IFS= read -r manifest; do
    manifests+=("$manifest")
    package="${manifest%/Cargo.toml}"
    mkdir -p "$fixture/$package/src"
    touch "$fixture/$package/src/lib.rs" "$fixture/$package/src/main.rs"
done < "$fixture/member-manifests"
for manifest in "${manifests[@]}"; do
    mkdir -p "$fixture/$(dirname "$manifest")"
    cp "$root/$manifest" "$fixture/$manifest"
done
printf 'fn main() {}\n' > "$fixture/crates/ic-timers/build.rs"
git init -q "$fixture"
cd "$fixture"
reset_manifests() {
    for manifest in "${manifests[@]}"; do
        cp "$root/$manifest" "$manifest"
    done
    # Prepared Wire selection for deliberate library-edge mutations below.
    TIMER_FIXTURE_WIRE_REQUIREMENT="$(sed -n 's/^ic-wire = //p' "$root/qualification/Cargo.toml")" \
        perl -pi -e 's/^\[workspace.dependencies\]/[workspace.dependencies]\nic-wire = $ENV{TIMER_FIXTURE_WIRE_REQUIREMENT}/' Cargo.toml
}
reset_manifests

checker=scripts/ci/check-timer-dependencies.sh
bash "$checker"
# The test-only simulator graph is admitted, but promoting it into the native
# build coordinator must fail even when the lock already contains every package.
perl -pi -e 's/^\[dependencies\]/[dependencies]\nic-testkit.workspace = true/' qualification/ic-timers-pocketic/Cargo.toml
if bash "$checker" > worker-failure.log 2>&1; then
    echo 'error: build coordinator admitted its test-only simulator dependency' >&2
    exit 1
fi
grep -q 'forbidden ic-testkit' worker-failure.log
reset_manifests

# Runtime APIs belong only to entrypoint-owning probes; the published library
# must keep its executor/IC0-only boundary even when runtime adds no new macro.
perl -pi -e 's/"executor", "ic0"/"executor", "ic0", "runtime"/' crates/ic-timers/Cargo.toml
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
if bash "$checker" > runtime-failure.log 2>&1; then
    echo 'error: library admitted Platform runtime APIs' >&2; exit 1
fi
grep -q 'forbidden dependency feature edge: ic-platform-runtime/runtime' runtime-failure.log
reset_manifests

# Empty Platform defaults still violate the library feature allowlist, even
# without introducing another package. Probe and protocol policies are separate.
perl -pi -e 's/"executor", "ic0"/"executor", "ic0", "default"/' crates/ic-timers/Cargo.toml
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
if bash "$checker" > default-failure.log 2>&1; then
    echo 'error: library admitted Platform default features' >&2; exit 1
fi
grep -q 'forbidden dependency feature edge: ic-platform-runtime/default' default-failure.log
reset_manifests
cat >> qualification/ic-timers-test-protocol/Cargo.toml <<'EOF'

[build-dependencies]
ic-platform-runtime = { workspace = true, features = ["executor"] }
EOF
RUSTUP_AUTO_INSTALL=0 cargo tree --manifest-path qualification/Cargo.toml --offline -p ic-timers-test-protocol --target wasm32-unknown-unknown > /dev/null
if bash "$checker" > protocol-failure.log 2>&1; then
    echo 'error: protocol admitted Platform authority' >&2; exit 1
fi
grep -q 'forbidden ic-platform-runtime' protocol-failure.log
reset_manifests

# A renamed target-specific build dependency must be checked by package name;
# the same package used only in dev-dependencies is outside this library graph.
# Lock preparation is confined to this disposable fixture, never the checkout.
# Pin the deliberate alias to the already prepared package, rather than resolving
# an independent old Wire graph that a clean offline CI cache cannot provide.
RUSTUP_AUTO_INSTALL=0 cargo metadata --manifest-path qualification/Cargo.toml --locked --offline --format-version 1 > wire-alias-metadata.json
perl -MJSON::PP -0777 -i -pe '
    BEGIN {
        open my $input, "<", "wire-alias-metadata.json" or die "$!\n";
        my $metadata = decode_json(<$input>);
        my ($protocol) = grep { $_->{name} eq "ic-timers-test-protocol" } @{$metadata->{packages}};
        my ($node) = grep { $_->{id} eq $protocol->{id} } @{$metadata->{resolve}{nodes}};
        my ($edge) = grep { $_->{name} eq "ic_wire" } @{$node->{deps}};
        my @wire = grep { $_->{id} eq $edge->{pkg} } @{$metadata->{packages}};
        close $input or die "$!\n";
        @wire == 1 or die "expected one prepared Wire package\n";
        $version = $wire[0]{version};
    }
    s/^ic-wire = /leaked-wire = { package = "ic-wire", version = "=$version" }\nic-wire = /m
        or die "missing workspace Wire declaration\n";
' Cargo.toml
for table in "target.'cfg(target_arch = \"wasm32\")'.dependencies" "target.'cfg(unix)'.build-dependencies"; do
    cp "$root/crates/ic-timers/Cargo.toml" crates/ic-timers/Cargo.toml
    # Build-dependency cfgs describe the host; normal dependencies describe the
    # selected target. Exercise both with real Cargo rather than invented trees.
    printf '\n[%s]\nleaked-wire.workspace = true\n' "$table" >> crates/ic-timers/Cargo.toml
    RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
    if bash "$checker" > build-failure.log 2>&1; then
        echo "error: library admitted a renamed target-specific Wire dependency: $table" >&2
        exit 1
    fi
    grep -q 'forbidden pattern matches ic-wire' build-failure.log
done
reset_manifests
# Tooling selection already owns a dev-dependencies table in every member.
perl -0777 -pi -e '
    s/^\[dev-dependencies\]\n/[dev-dependencies]\nic-wire.workspace = true\n/m
        or die "missing member dev-dependencies table\n";
' crates/ic-timers/Cargo.toml
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
bash "$checker"
reset_manifests

# Metrics' required Serde transport must not admit host JSON parsing into Wasm.
cat >> crates/ic-timers/Cargo.toml <<'EOF'

[target.'cfg(target_arch = "wasm32")'.dependencies]
serde_json.workspace = true
EOF
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
if bash "$checker" > json-failure.log 2>&1; then
    echo 'error: library admitted JSON parsing into Wasm' >&2
    exit 1
fi
grep -q 'forbidden serde_json' json-failure.log
reset_manifests

# Relay owns application delivery above Timers. A canonical-name check must also
# reject renamed target/build edges; the local stub resolves without downloads.
mkdir -p relay-substitute/src
cat > relay-substitute/Cargo.toml <<'EOF'
[package]
name = "ic-relay"
version = "0.0.0"
edition = "2024"

[workspace]
EOF
touch relay-substitute/src/lib.rs
perl -pi -e 's/exclude = \["qualification"\]/exclude = ["qualification", "relay-substitute"]/' Cargo.toml
perl -pi -e 's/^\[workspace.dependencies\]/[workspace.dependencies]\napplication-delivery = { package = "ic-relay", path = "relay-substitute" }/' Cargo.toml
for table in "target.'cfg(target_arch = \"wasm32\")'.dependencies" "target.'cfg(unix)'.build-dependencies"; do
    cp "$root/crates/ic-timers/Cargo.toml" crates/ic-timers/Cargo.toml
    printf '\n[%s]\napplication-delivery.workspace = true\n' "$table" >> crates/ic-timers/Cargo.toml
    RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
    if bash "$checker" > relay-failure.log 2>&1; then
        echo "error: library admitted application Relay below timer infrastructure: $table" >&2
        exit 1
    fi
    grep -q 'forbidden ic-relay' relay-failure.log
done
reset_manifests

# The common direct guard composes with Timers' stricter deployment policy.
# Principal is intentionally absent from the local declaration list: it must
# still reject an optional renamed target/build edge through the common owner.
direct_checker=vendor/toko-tooling/scripts/ci/check-cargo-dependency-boundaries.sh
bash "$direct_checker" --consumer "$fixture" --external-ic
perl -pi -e 's/^ic-platform-runtime = /external-principal = { package = "ic_principal", version = "0.1" }\nic-platform-runtime = /' Cargo.toml
cat >> crates/ic-timers/Cargo.toml <<'EOF'

[target.'cfg(target_arch = "wasm32")'.build-dependencies]
external-principal = { workspace = true, optional = true }
EOF
RUSTUP_AUTO_INSTALL=0 cargo metadata --offline --format-version 1 > /dev/null
bash "$direct_checker" --consumer "$fixture"
if bash "$direct_checker" --consumer "$fixture" --external-ic > direct-failure.log 2>&1; then
    echo 'error: common IC boundary admitted a renamed optional target/build dependency' >&2
    exit 1
fi
grep -q 'ic-timers -> ic_principal' direct-failure.log
reset_manifests

# Runtime's transitive backend is admitted above; it must not let either
# workspace bypass Platform with a renamed, optional target/build SDK edge.
for package in crates/ic-timers qualification/ic-timers-runtime-probe; do
    catalog=Cargo.toml
    consumer="$fixture"
    [[ "$package" != qualification/* ]] || { catalog=qualification/Cargo.toml; consumer="$fixture/qualification"; }
    perl -pi -e 's/^\[workspace.dependencies\]/[workspace.dependencies]\nexternal-system = { package = "ic0", version = "1.2.0" }/' "$catalog"
    cat >> "$package/Cargo.toml" <<'EOF'

[target.'cfg(target_arch = "wasm32")'.build-dependencies]
external-system = { workspace = true, optional = true }
EOF
    RUSTUP_AUTO_INSTALL=0 cargo metadata --manifest-path "$catalog" --offline --format-version 1 > /dev/null
    if bash "$direct_checker" --consumer "$consumer" --external-ic > direct-ic0-failure.log 2>&1; then
        echo "error: common IC boundary admitted direct SDK dependency from $package" >&2
        exit 1
    fi
    grep -q "${package##*/} -> ic0" direct-ic0-failure.log
    reset_manifests
done

# A source override at the same version is not the qualified runtime owner.
mkdir -p platform-substitute/src
cat > platform-substitute/Cargo.toml <<EOF
[package]
name = "ic-platform-runtime"
version = "$platform_version"
edition = "2024"

[features]
default = []
executor = []
ic0 = []
cdk = []
pocket-ic = []
candid = []
management-canister-types = []
principal = []
principal-external = []
principal-self-authenticating = []
runtime = []
stable-structures = []

[workspace]
EOF
touch platform-substitute/src/lib.rs
cat >> Cargo.toml <<'EOF'

[patch.crates-io]
ic-platform-runtime = { path = "platform-substitute" }
EOF
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
if bash "$checker" > identity-failure.log 2>&1; then
    echo 'error: library admitted an unqualified Platform source at the same version' >&2
    exit 1
fi
grep -q 'expected one qualified registry ic-platform-runtime 0.22 runtime owner' identity-failure.log
reset_manifests

# The feature-selected helper is also registry-owned and must match the facade.
mkdir -p guard-substitute/src
cat > guard-substitute/Cargo.toml <<EOF
[package]
name = "ic-platform-runtime-guard"
version = "$platform_version"
edition = "2024"
links = "ic-cdk async executor, see https://github.com/dfinity/cdk-rs/blob/links-pin/TROUBLESHOOTING.md"
build = "build.rs"

[features]
default = []

[workspace]
EOF
touch guard-substitute/src/lib.rs
printf 'fn main() {}\n' > guard-substitute/build.rs
cat >> Cargo.toml <<'EOF'

[patch.crates-io]
ic-platform-runtime-guard = { path = "guard-substitute" }
EOF
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
if bash "$checker" > guard-failure.log 2>&1; then
    echo 'error: library admitted an unqualified runtime guard source' >&2
    exit 1
fi
grep -q 'expected one matching registry Platform runtime guard' guard-failure.log
reset_manifests

# An alias must not let a consumer depend directly on Platform's private helper.
TIMER_FIXTURE_PLATFORM_VERSION="$platform_version" perl -pi -e '
    s/^\[workspace.dependencies\]/[workspace.dependencies]\nprivate-guard = { package = "ic-platform-runtime-guard", version = "=$ENV{TIMER_FIXTURE_PLATFORM_VERSION}" }/
' Cargo.toml
cat >> crates/ic-timers/Cargo.toml <<'EOF'

[target.'cfg(unix)'.build-dependencies]
private-guard.workspace = true
EOF
RUSTUP_AUTO_INSTALL=0 cargo metadata --offline --format-version 1 > /dev/null
if bash "$direct_checker" --consumer "$fixture" --external-ic > direct-guard-failure.log 2>&1; then
    echo 'error: library admitted a direct renamed runtime-guard dependency' >&2
    exit 1
fi
grep -q 'ic-timers -> ic-platform-runtime-guard' direct-guard-failure.log
reset_manifests

# The actual Make admission must reject a new Host family member without
# needing that concrete package name in another exclusion roster.
cp "$root/Makefile" Makefile
cp -R "$root/make/." make/
mkdir -p future-host/src
printf '[package]\nname="ic-host-unlisted-fixture"\nversion="0.0.0"\nedition="2024"\n' > future-host/Cargo.toml
printf '' > future-host/src/lib.rs
cat >> crates/ic-jobs/Cargo.toml <<'EOF'

[target.'cfg(unix)'.dependencies]
ic-host-unlisted-fixture = { path = "../../future-host" }
EOF
cargo metadata --offline --format-version 1 > /dev/null
if make --no-print-directory check-dependency-graphs > future-host-failure.log 2>&1; then
    echo 'error: Jobs admitted an unlisted Host family member' >&2; exit 1
fi
grep -q 'forbidden pattern.*ic-host-unlisted-fixture\|ic-host-unlisted-fixture.*forbidden pattern' future-host-failure.log
reset_manifests

# Static provider admission follows the real probe declarations without building
# a canister. Mutations retain the surrounding source and change one obligation.
for package in ic-timers ic-timers-runtime-probe ic-timers-size-probe ic-timers-test-protocol; do
    tree=qualification
    [[ "$package" != ic-timers ]] || tree=crates
    cp -R "$root/$tree/$package/src/." "$tree/$package/src/"
done
bash scripts/ci/check-timer-provider.sh
probe=qualification/ic-timers-runtime-probe/src/lib.rs
contract=qualification/ic-timers-test-protocol/src/rpc/mod.rs
cp "$probe" original-probe.rs
cp "$contract" original-contract.rs
for mutation in identity budget bare; do
    cp original-probe.rs "$probe"
    cp original-contract.rs "$contract"
    case "$mutation" in
        identity) perl -0777 -pi -e 's/id = \$crate::rpc::Method::\$method as u64/id = 0/ or die "missing probe identity binding\n"' "$contract" ;;
        budget) perl -0777 -pi -e 's/request = \$crate::rpc::REQUEST_LIMITS/request = another_limit/ or die "missing probe budget binding\n"' "$contract" ;;
        bare) printf '\n#[ic_platform_runtime::cdk::query]\nfn unadmitted_probe() {}\n' >> "$probe" ;;
    esac
    if bash scripts/ci/check-timer-provider.sh > "provider-$mutation-failure.log" 2>&1; then
        echo "error: probe admitted $mutation mutation" >&2; exit 1
    fi
    grep -q 'probe entrypoints require' "provider-$mutation-failure.log"
done
cp original-probe.rs "$probe"
cp original-contract.rs "$contract"

cmp "$root/Cargo.lock" Cargo.lock
fixture_complete=true
echo 'Library, probe, protocol and build coordinator dependency policy checks passed'
