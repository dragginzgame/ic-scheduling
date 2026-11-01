#!/usr/bin/env bash
# Shared companions: scripts/ci/check-timer-dependencies.sh scripts/ci/fixture-lifecycle.sh scripts/ci/prepare-script-fixture.sh
set -euo pipefail
root="$(git rev-parse --show-toplevel)"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/timer-library-boundary.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Failed library-boundary fixture retained"' EXIT

# Real Cargo resolution against the maintained catalog and lock. Empty fixture
# source is never built; no sibling source or network dependency is introduced.
bash "$root/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture" \
    scripts/ci/check-timer-dependencies.sh scripts/ci/check-cargo-dependency-boundaries.sh \
    ci/cargo-dependency-boundaries.json
RUSTUP_AUTO_INSTALL=0 cargo metadata --locked --offline --format-version 1 > "$fixture/workspace.json"
platform_version="$(perl -MJSON::PP -0777 -e '
    my $metadata = decode_json(<>);
    my @platform = grep { $_->{name} eq "ic-platform-runtime" && $_->{version} =~ /^0\.17\.[0-9]+$/ }
        @{$metadata->{packages}};
    @platform == 1 or die "expected one selected runtime Platform\n";
    print $platform[0]{version};
' "$fixture/workspace.json")"
perl -MJSON::PP -MFile::Spec -0777 -e '
    my $metadata = decode_json(<>);
    my %members = map { $_ => 1 } @{$metadata->{workspace_members}};
    for my $package (@{$metadata->{packages}}) {
        next unless $members{$package->{id}};
        my $manifest = File::Spec->abs2rel($package->{manifest_path}, $metadata->{workspace_root});
        $manifest !~ m{^(?:\.\./|/)} && $manifest !~ /[\r\n]/ or die "invalid member manifest\n";
        print "$manifest\n";
    }
' "$fixture/workspace.json" > "$fixture/member-manifests"
manifests=(Cargo.toml Cargo.lock)
while IFS= read -r manifest; do
    [[ "$manifest" == Cargo.toml ]] || manifests+=("$manifest")
    package="${manifest%/Cargo.toml}"
    mkdir -p "$fixture/$package/src"
    touch "$fixture/$package/src/lib.rs" "$fixture/$package/src/main.rs"
done < "$fixture/member-manifests"
for manifest in "${manifests[@]}"; do
    cp "$root/$manifest" "$fixture/$manifest"
done
printf 'fn main() {}\n' > "$fixture/crates/ic-timers/build.rs"
git init -q "$fixture"
cd "$fixture"
reset_manifests() {
    for manifest in "${manifests[@]}"; do
        cp "$root/$manifest" "$manifest"
    done
}

checker=scripts/ci/check-timer-dependencies.sh
bash "$checker"
# The test-only simulator graph is admitted, but promoting it into the native
# build coordinator must fail even when the lock already contains every package.
perl -pi -e 's/^\[dependencies\]/[dependencies]\nic-testkit.workspace = true/' crates/ic-timers-pocketic/Cargo.toml
if bash "$checker" > worker-failure.log 2>&1; then
    echo 'error: build coordinator admitted its test-only simulator dependency' >&2
    exit 1
fi
grep -q 'forbidden ic-testkit' worker-failure.log
reset_manifests
for feature in candid principal; do
    TIMER_FORBIDDEN_FEATURE="$feature" perl -pi -e 's/"executor", "ic0"/"executor", "ic0", "$ENV{TIMER_FORBIDDEN_FEATURE}"/' crates/ic-timers/Cargo.toml
    if cargo tree --offline -p ic-timers > "$feature-failure.log" 2>&1; then exit 1; fi
    grep -q 'does not have that feature' "$feature-failure.log"
    reset_manifests
done

# Runtime APIs belong only to entrypoint-owning probes; the published library
# must keep its executor/IC0-only boundary even when runtime adds no new macro.
perl -pi -e 's/"executor", "ic0"/"executor", "ic0", "runtime"/' crates/ic-timers/Cargo.toml
if bash "$checker" > runtime-failure.log 2>&1; then
    echo 'error: library admitted Platform runtime APIs' >&2; exit 1
fi
grep -q 'forbidden dependency feature edge: ic-platform-runtime/runtime' runtime-failure.log
reset_manifests

# Empty Platform defaults still violate the library feature allowlist, even
# without introducing another package. Probe and protocol policies are separate.
perl -pi -e 's/"executor", "ic0"/"executor", "ic0", "default"/' crates/ic-timers/Cargo.toml
if bash "$checker" > default-failure.log 2>&1; then
    echo 'error: library admitted Platform default features' >&2; exit 1
fi
grep -q 'forbidden dependency feature edge: ic-platform-runtime/default' default-failure.log
reset_manifests
for feature in candid principal-external; do
    TIMER_FORBIDDEN_FEATURE="$feature" perl -pi -e 's/\["cdk"\]/["cdk", "$ENV{TIMER_FORBIDDEN_FEATURE}"]/' crates/ic-timers-runtime-probe/Cargo.toml
    if cargo tree --offline -p ic-timers-runtime-probe > "probe-$feature-failure.log" 2>&1; then exit 1; fi
    grep -q 'does not have that feature' "probe-$feature-failure.log"
    reset_manifests
done
cat >> crates/ic-timers-test-protocol/Cargo.toml <<'EOF'

[build-dependencies]
ic-platform-runtime = { workspace = true, features = ["executor"] }
EOF
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers-test-protocol --target wasm32-unknown-unknown > /dev/null
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
RUSTUP_AUTO_INSTALL=0 cargo metadata --locked --offline --format-version 1 > wire-alias-metadata.json
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
cat >> crates/ic-timers/Cargo.toml <<'EOF'

[dev-dependencies]
ic-wire.workspace = true
EOF
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
perl -pi -e 's/^resolver = /exclude = ["relay-substitute"]\nresolver = /' Cargo.toml
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
direct_checker=scripts/ci/check-cargo-dependency-boundaries.sh
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
grep -q 'expected one qualified registry ic-platform-runtime 0.17 runtime owner' identity-failure.log
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

cmp "$root/Cargo.lock" Cargo.lock
fixture_complete=true
echo 'Library, probe, protocol and build coordinator dependency policy checks passed'
