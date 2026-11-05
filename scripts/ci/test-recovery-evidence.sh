#!/usr/bin/env bash
set -euo pipefail
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
export PATH="$root/.tools/host/bin:$PATH"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
scratch_parent="${TMPDIR:-$root/target/tmp}"
case "$scratch_parent" in *$'\r'*|*$'\n'*) echo 'scratch directory contains CR/LF' >&2; exit 2 ;; esac
[[ -n "${TMPDIR:-}" ]] || mkdir -p "$scratch_parent"
fixture="$(fixture_create "$scratch_parent/scheduling-recovery-evidence.XXXXXX")"
fixture_complete=false
finish() {
    local status=$?
    trap - EXIT
    fixture_completion_status "$status" "$fixture_complete" || status=$?
    printf 'Recovery upload-selection evidence retained: %s\n' "$fixture" >&2
    exit "$status"
}
trap finish EXIT
yq -o json '.jobs.recovery.steps' "$root/.github/workflows/recovery.yml" > "$fixture/steps.json"
jq -er '[.[] | select(.id == "recovery-evidence")] |
    if length == 1 then .[0] else error("expected one evidence upload") end |
    select(.if == "${{ always() && steps.checkout.outcome == '\''success'\'' }}") |
    .with.path' "$fixture/steps.json" > "$fixture/paths"
export RECOVERY_FIXTURE="$fixture"
perl - "$fixture/paths" <<'PERL'
use strict;
use warnings;
use File::Path qw(make_path);
use File::Glob qw(bsd_glob GLOB_NOSORT);
use File::Find qw(find);
my $root = $ENV{RECOVERY_FIXTURE};
my @required = (
    'temp/jobs-recovery/source.txt',
    'temp/jobs-recovery/test.log',
    'temp/jobs-recovery.abc123/ic_jobs_test_consumer.wasm',
    'temp/timer-qualification-123-0/runtime.json',
    'temp/timer-qualification-456-0/pocketic-1/stderr.log',
    'workspace/target/timer-cohorts.json',
);
for my $path (@required) {
    my $parent = "$root/$path";
    $parent =~ s{/[^/]+\z}{};
    make_path($parent);
    open my $fh, '>', "$root/$path" or die "create $path: $!\n";
    print {$fh} "retained evidence for $path\n";
    close $fh or die "close $path: $!\n";
}
open my $paths, '<', $ARGV[0] or die "read selection: $!\n";
my %selected;
my $temp_expression = '${{ runner.temp }}';
my $workspace_expression = '${{ github.workspace }}';
while (my $pattern = <$paths>) {
    chomp $pattern;
    next if $pattern eq '';
    $pattern =~ s/\Q$temp_expression\E/$root\/temp/g;
    $pattern =~ s/\Q$workspace_expression\E/$root\/workspace/g;
    die "unresolved expression\n" if $pattern =~ /\$\{\{/;
    for my $path (bsd_glob($pattern, GLOB_NOSORT)) {
        next unless -e $path;
        if (-d $path) {
            find({no_chdir => 1, wanted => sub { $selected{$File::Find::name} = 1 if -f $_ }}, $path);
        } else { $selected{$path} = 1; }
    }
}
die "upload omits $_\n" for grep { !$selected{"$root/$_"} } @required;
print "All original, timer runtime, server diagnostic and cohort receipts reach the actual upload selection.\n";
PERL
fixture_complete=true
