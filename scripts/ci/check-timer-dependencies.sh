#!/usr/bin/env bash
# Shared companions: ci/timer-dependency-boundaries.json scripts/ci/check-cargo-graph.pl
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

# Timers owns roots, feature policy and qualified runtime sources. Shared owns
# Cargo tree admission, canonical name matching and rendered identity counts.
CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 perl -MJSON::PP -MCwd=getcwd <<'PERL'
use strict;
use warnings;
open my $policy, '<', 'ci/timer-dependency-boundaries.json' or die "$!\n";
my $rules = decode_json(do { local $/; <$policy> });
close $policy or die "$!\n";
open my $rustc, '-|', 'rustc', '-vV' or die "$!\n";
my $identity = do { local $/; <$rustc> };
close $rustc or die "rustc identity failed\n";
my ($host) = $identity =~ /^host: ([A-Za-z0-9_-]+)$/m;
defined($host) or die "missing Rust host\n";
open my $cargo, '-|', 'cargo', 'metadata', '--locked', '--offline', '--format-version', '1'
    or die "$!\n";
my $bytes = do { local $/; <$cargo> };
close $cargo or die "Cargo runtime identity observation failed\n";
my $metadata = decode_json($bytes);
my @platform = grep { $_->{name} eq 'ic-platform-runtime' && $_->{version} =~ /^0\.20\./ }
    @{$metadata->{packages}};
@platform == 1
    && ($platform[0]{source} // '') eq 'registry+https://github.com/rust-lang/crates.io-index'
    or die "expected one qualified registry ic-platform-runtime 0.20 runtime owner\n";
my @guard = grep { $_->{name} eq 'ic-platform-runtime-guard' } @{$metadata->{packages}};
@guard == 1 && $guard[0]{version} eq $platform[0]{version}
    && ($guard[0]{source} // '') eq 'registry+https://github.com/rust-lang/crates.io-index'
    && ($guard[0]{links} // '') eq
        'ic-cdk async executor, see https://github.com/dfinity/cdk-rs/blob/links-pin/TROUBLESHOOTING.md'
    or die "expected one matching registry Platform runtime guard\n";
my %features;
$features{$_} = 1 for keys %{$platform[0]{features}};
for my $target ('wasm32-unknown-unknown', $host) {
    for my $root (qw(ic-timers ic-timers-runtime-probe ic-timers-size-probe ic-timers-test-protocol)) {
        my $library = $root eq 'ic-timers';
        my $protocol = $root eq 'ic-timers-test-protocol';
        my @check = ('perl', 'scripts/ci/check-cargo-graph.pl', '--consumer', getcwd(),
            '--package', $root, '--target', $target, '--all-features');
        for my $rule (@$rules) {
            my $name = $rule->{dependency};
            next if !$protocol && $name eq 'ic-platform-runtime-guard';
            next if !$library && !$protocol && $name eq 'ic-platform-macros';
            push @check, '--forbid', $name;
        }
        push @check, '--forbid', 'ic-platform';
        push @check, '--forbid-pattern', '^ic-(?:build(?:-|$)|host-|testkit(?:-|$))';
        if ($library) {
            push @check, '--forbid-pattern', '^ic-timers-';
            # Metrics owns validated Serde transport.
            # Neither its host codecs nor host parsing belong in deployed Wasm.
            if ($target eq 'wasm32-unknown-unknown') {
                push @check, '--forbid-pattern', '^ic-wire(?:-|$)';
                push @check, map { ('--forbid', $_) } qw(serde_json serde_bytes postcard);
            }
            push @check, map { ('--forbid', $_) }
                qw(ic-relay ic_principal sha2 digest crc32fast data-encoding);
        }
        if ($protocol) {
            push @check, map { ('--forbid', $_) } qw(ic-platform-runtime ic-timers);
        } else {
            push @check, '--require', 'ic-platform-runtime=1', '--require', 'ic-platform-runtime-guard=1';
            my %allowed = map { $_ => 1 } ($library ? qw(executor ic0) : qw(cdk runtime executor ic0));
            push @check, map { ('--forbid-feature', "ic-platform-runtime/$_") }
                grep { !$allowed{$_} } sort keys %features;
        }
        system(@check) == 0 or die "Timers dependency boundary failed: $root on $target\n";
    }
}
# Only the test binary owns simulator calls and probe codecs. The normal native
# coordinator builds artifacts, supervises the worker and handles JSON receipts.
my @worker = ('perl', 'scripts/ci/check-cargo-graph.pl', '--consumer', getcwd(),
    '--package', 'ic-timers-pocketic', '--target', $host, '--all-features');
push @worker, map { ('--forbid', $_) }
    qw(ic-platform-runtime ic-platform-runtime-guard ic-platform-pocket-ic ic-cdk-executor ic0 ic-testkit pocket-ic candid ic-wire ic-timers ic-timers-test-protocol);
system(@worker) == 0 or die "Timers build coordinator dependency boundary failed\n";
PERL
