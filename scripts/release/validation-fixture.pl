#!/usr/bin/env perl
# Scheduling selects one pure fixture; Shared owns execution/reuse and proof sealing.
use strict;
use warnings;
use Cwd qw(abs_path);
use FindBin qw($RealBin);
use lib "$RealBin/../ci";
use ValidationPlan qw(json document digest relative_path);
use File::Path qw(make_path);
use File::Copy qw(copy);
use JSON::PP;
use POSIX qw(uname);

chdir "$RealBin/../.." or die $!;
my $root = abs_path('.');
my $attempt = $ENV{SCHEDULING_VALIDATION_ATTEMPT};
exec 'bash', 'scripts/release/test-validation.sh' or die $! unless defined $attempt;
$attempt = relative_path($attempt);
my $context = document($root, "$attempt/context.json");
$context->{source} eq ($ENV{RELEASE_SOURCE}//'') && $context->{group} eq ($ENV{RELEASE_GROUP}//'')
    or die "fixture source/group differs from admitted plan\n";
my $destination = "$attempt/fixture-stage";
!-e $destination && !-l $destination or die "occupied fixture stage evidence\n";
make_path($destination, {mode=>0700});
$ENV{PATH} = "$root/.tools/host/bin:$ENV{PATH}";
sub save {
    my ($name, $value) = @_;
    open my $out, '>', "$destination/$name" or die $!;
    print {$out} json($value) or die $!;
    close $out or die $!;
}
my $reason = (uname())[0] ne 'Linux' ? 'native host is not qualified for fixture reuse' : '';
$reason ||= 'custom process/compiler configuration' if grep {
    /\A(?:BASH_ENV|BASH_FUNC_.+|ENV|PERL5OPT|PERL5LIB|LD_PRELOAD|LD_LIBRARY_PATH|TMPDIR|RUSTC|RUSTDOC|RUSTFLAGS|RUSTDOCFLAGS|RUSTC_WRAPPER|RUSTC_WORKSPACE_WRAPPER|CARGO_ENCODED_RUSTFLAGS|CARGO_BUILD_.+|CARGO_TARGET_.+|CARGO_PROFILE_.+|GIT_.+)\z/
} grep { $_ ne 'GIT_ASKPASS' && $_ ne 'GIT_PAGER' } keys %ENV;
$reason ||= 'parallel Make configuration' if ($ENV{MAKEFLAGS}//'') =~ /jobserver|(?:^|\s)-?j/;
if (!length $reason) {
    system "$root/scripts/release/observe-validation-fixture.pl", '--eligible';
    $reason = 'tool paths are not qualified for fixture reuse' if $?;
}
if (length $reason) {
    print "Validation fixture: ordinary execution ($reason)\n";
    system 'bash', 'scripts/release/test-validation.sh';
    exit($? & 127 ? 128 + ($? & 127) : $? >> 8) if $?;
    save('decision.json', {schema=>1,source=>$context->{source},group=>$context->{group},mode=>'ordinary',reason=>$reason});
    exit;
}
# The fixture clears Make/release overrides itself. No user Git configuration,
# hooks/templates, locale or auto-maintenance can become unobserved inputs.
my %environment = (PATH=>$ENV{PATH}, LC_ALL=>'C', LANG=>'C', TZ=>'UTC',
    GIT_CONFIG_NOSYSTEM=>'1', GIT_CONFIG_GLOBAL=>'/dev/null', GIT_TERMINAL_PROMPT=>'0',
    GIT_CONFIG_COUNT=>'2', GIT_CONFIG_KEY_0=>'gc.auto', GIT_CONFIG_VALUE_0=>'0',
    GIT_CONFIG_KEY_1=>'maintenance.auto', GIT_CONFIG_VALUE_1=>'false');
$environment{$_} = $ENV{$_} for grep { defined $ENV{$_} } qw(HOME CARGO_HOME RUSTUP_HOME RUSTUP_TOOLCHAIN);
save('environment.json', \%environment);
my @command = ('perl', 'scripts/ci/run-validation-stage.pl', '--state', 'target/validation-fixture-cache',
    '--stage', 'test-release-validation', '--contract', 'scheduling-validation-fixture/1',
    '--source', $context->{source}, '--group', $context->{group},
    '--observe', "$root/scripts/release/observe-validation-fixture.pl",
    '--environment', "$destination/environment.json", '--max-bytes', '536870912',
    '--admission-output', "$root/$destination/admission.json", '--', 'bash', 'scripts/release/test-validation.sh');
print "Validation fixture: executing unless a valid proof matches the exact current input context\n";
system @command;
exit($? & 127 ? 128 + ($? & 127) : $? >> 8) if $?;
my $admission = document($root, "$destination/admission.json");
my $proof = JSON::PP->new->decode(do { open my $in, '<', $admission->{execution_proof} or die $!; local $/; <$in> });
my $identity = $proof->{context}{observation}{configuration}{identity};
open my $seal, '-|', 'perl', 'scripts/ci/seal-validation-stage.pl', "$root/$destination/admission.json",
    $context->{source}, $context->{group}, $identity or die $!;
my $raw = do { local $/; <$seal> };
close $seal or die "fixture proof sealing failed\n";
my $sealed = JSON::PP->new->decode($raw);
my %copies;
for my $path (sort keys %{$sealed->{files}}) {
    my $name = 'original-' . scalar(keys %copies);
    copy($path, "$destination/$name") or die $!;
    digest($root, "$destination/$name", 536870912) eq $sealed->{files}{$path} or die "fixture proof changed during retention\n";
    $copies{$path} = $name;
}
save('decision.json', {schema=>1,source=>$context->{source},group=>$context->{group},mode=>$admission->{mode},seal=>$sealed,copies=>\%copies});
