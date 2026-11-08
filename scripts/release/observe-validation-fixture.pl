#!/usr/bin/env perl
use strict;
use warnings;
use Cwd qw(abs_path getcwd);
use Digest::SHA qw(sha256_hex);
use JSON::PP;
use DynaLoader ();
# Load the fixture's core Perl runtime closure as well as this observer's.
use File::Find;
use File::Temp;
use File::Path;
use File::Spec;
use POSIX;
use Getopt::Long;
use Fcntl;
use Config;
use File::Copy;
use FindBin;
use lib;
use Encode;
use Time::HiRes;

my $json = JSON::PP->new->canonical->utf8;
@ARGV == 0 || (@ARGV == 1 && $ARGV[0] eq '--eligible') or die "unknown fixture observer arguments\n";
my $eligible = @ARGV;
my (%files, %tools);
sub add {
    my ($path) = @_;
    -f $path or die "missing fixture input: $path\n";
    $files{$path} = 1;
}
sub output {
    open my $in, '-|', @_ or die "execute $_[0]: $!\n";
    local $/; my $raw = <$in> // '';
    close $in or die "failed observation: @_\n";
    return $raw;
}
open my $list, '<', 'ci/validation-fixture-inputs.txt' or die $!;
while (<$list>) {
    chomp;
    /\A[A-Za-z0-9_.\/-]+\z/ && !m{(?:^|/)\.\.(?:/|$)} or die "invalid fixture input\n";
    add($_);
}
close $list or die $!;
# Every executable reached by the shell fixture or its validation helpers.
for my $name (qw(bash sh perl git make jq yq cargo rustc cp mv rm mkdir mktemp tr grep find head tail cmp cat dirname date tee sed awk sort touch env sha256sum shasum ldd wc uname)) {
    my ($path) = grep { -f $_ && -x $_ } map { "$_/$name" } split /:/, $ENV{PATH};
    defined($path) or die "missing fixture tool: $name\n";
    my $resolved = abs_path($path);
    my $managed = getcwd() . '/.tools/host/bin/';
    if ($name ne 'cargo' && $name ne 'rustc' && index($path, $managed) != 0
        && $resolved !~ m{\A/(?:usr/)?bin/}) {
        die "unqualified fixture tool path: $name\n";
    }
    if (index($path, $managed) == 0) {
        $name =~ /\A(?:jq|yq)\z/ or die "unqualified managed fixture tool: $name\n";
        open my $binary, '<:raw', $path or die $!;
        my $magic;
        read($binary, $magic, 4) == 4 && $magic eq "\x7fELF"
            or die "managed fixture tool is not a native payload: $name\n";
        close $binary or die $!;
    }
    $tools{$name} = $path;
    add($path);
}
exit if $eligible;
# Rustup proxies dispatch actual toolchain payloads; hash those, not just rustup.
my $sysroot = output($tools{rustc}, '--print', 'sysroot');
$sysroot =~ s/\n\z//;
add("$sysroot/bin/$_") for qw(rustc cargo);
# Linux-only admission includes each tool's native loader/library closure.
for my $program (values %tools, "$sysroot/bin/rustc", "$sysroot/bin/cargo") {
    # ldd returns nonzero for scripts and static tools; those have no loader graph.
    open my $in, '-|', $tools{bash}, '-c', 'ldd "$1" 2>/dev/null || :', 'fixture-ldd', $program or die $!;
    while (<$in>) { add($1) if m{(?:=>\s+|^\s*)(/[^\s]+)}; }
    close $in or die $!;
}
add('/etc/ld.so.cache') if -f '/etc/ld.so.cache';
add(abs_path($_)) for values(%INC), @DynaLoader::dl_shared_objects;
my @identity;
for my $path (sort keys %files) {
    open my $in, '<:raw', $path or die $!;
    push @identity, [$path, (stat($in))[2] & 07777, Digest::SHA->new(256)->addfile($in)->hexdigest];
    close $in or die $!;
}
print $json->encode({schema=>1,files=>[sort keys %files],configuration=>{
    identity=>sha256_hex($json->encode(\@identity)),
    compiler=>{rustc=>output($tools{rustc}, '-vV'),cargo=>output($tools{cargo}, '-vV')},
    modes=>{map { $_->[0]=>$_->[1] } @identity},
}}),"\n";
