#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"
perl <<'PERL'
use strict;
use warnings;
use File::Find;
my $root = 'crates/ic-timers/src';
open my $facade, '<', "$root/lib.rs" or die "$root/lib.rs: $!";
my $source = do { local $/; <$facade> };
$source =~ /^mod platform;$/m or die "platform must remain a private module\n";
$source =~ /^#!\[forbid\(private_interfaces\)\]$/m
    or die "private types must not leak into public interfaces\n";
$source =~ /^#!\[deny\(unsafe_code\)\]$/m
    or die "unsafe code must be denied by default\n";
my $ffi = "$root/platform/ffi.rs";
-f $ffi or die "missing FFI boundary\n";
find({ no_chdir => 1, wanted => sub {
    return unless /\.rs$/ && -f $_;
    my $file = $_;
    open my $input, '<', $file or die "$file: $!";
    my $text = do { local $/; <$input> };
    if ($file =~ m{/platform/} && $text =~ /\bpub\s+(?:(?:async|unsafe)\s+)*(?:struct|enum|type|fn|use|mod|trait|const|static|extern)\b/) {
        die "platform items require restricted visibility: $file\n";
    }
    if ($file ne $ffi && $text =~ /(?:\bunsafe\s*(?:\{|extern|fn)|unsafe\(export_name|(?:allow|expect)\s*\(\s*unsafe_code\b|ic_platform_runtime::ic0::call_(?:new|on_cleanup)\b)/) {
        die "unsafe operations and IC callback registration belong to FFI: $file\n";
    }
    if ($file !~ m{/platform/} && $text =~ /\bic_platform_runtime\b/) {
        die "timer execution belongs to the private platform: $file\n";
    }
}}, $root);
# Probe transport owns a Wire contract; reject accidental Candid defaults locally.
for my $probe (qw(ic-timers-runtime-probe ic-timers-size-probe ic-timers-test-protocol ic-timers-pocketic)) {
    my @probe_roots = grep { -d $_ } map { "crates/$probe/$_" } qw(src tests);
    next unless @probe_roots; # Disposable provider fixtures may omit probes.
    find({ no_chdir => 1, wanted => sub {
        return unless /\.rs$/ && -f $_;
        my $file = $_;
        open my $input, '<', $file or die "$file: $!";
        my $text = do { local $/; <$input> };
        $text !~ /(?:ic_platform_runtime::candid|\bcandid::|\bCandidType\b|\b(?:query_candid_args|update_candid_args|stable_save|stable_restore)\s*[(<])/s
            or die "probe protocols must use Wire: $file\n";
        $text !~ /#\[ic_platform_runtime::cdk::(?:query|update)\b/
            or die "probe entrypoints require canonical Wire Endpoint attributes: $file\n";
        while ($text =~ /#\[ic_wire_endpoint::(query|update|\$kind)\b(.*?)\]/sg) {
            my ($kind, $attributes) = ($1, $2);
            my $macro = $kind eq '$kind';
            !$macro || $file eq 'crates/ic-timers-test-protocol/src/rpc/mod.rs'
                or die "probe entrypoints require the protocol-owned declaration macro: $file\n";
            my $owner = $macro ? '\$crate::rpc::' : 'ic_timers_test_protocol::rpc::';
            my $method = $macro ? '\$method' : '[A-Za-z0-9_]+';
            $attributes =~ /id\s*=\s*${owner}Method::$method\s+as\s+u64/
                && $attributes =~ /request\s*=\s*${owner}REQUEST_LIMITS/
                && $attributes =~ /response\s*=\s*${owner}RESPONSE_LIMITS/
                or die "probe entrypoints require explicit protocol identity and Wire budgets: $file\n";
        }
    }}, @probe_roots);
}

PERL
echo "Provider boundary checks passed"
