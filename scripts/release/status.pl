#!/usr/bin/env perl
use strict;
use warnings;
use FindBin;
use lib "$FindBin::Bin/../../vendor/toko-tooling/scripts/ci";
use StableVersion qw(stable version_cmp);
use JSON::PP qw(decode_json encode_json);
use Getopt::Long qw(GetOptions);
use File::Temp qw(tempfile);
use Cwd qw(abs_path);

my ($offline, $json) = map { ($ENV{$_} // '') eq '' ? 0 : $ENV{$_} } qw(STATUS_OFFLINE STATUS_JSON);
die "STATUS_OFFLINE and STATUS_JSON must be 0 or 1\n" unless "$offline" =~ /\A[01]\z/ && "$json" =~ /\A[01]\z/;
GetOptions('offline' => \$offline, 'json' => \$json) && !@ARGV
    or die "usage: status.pl [--offline] [--json]\n";
my $root = abs_path("$FindBin::Bin/../..");
chdir $root or die "cannot enter workspace: $!\n";
$ENV{PATH} = "$root/.tools/host/bin:$ENV{PATH}";
$ENV{GIT_OPTIONAL_LOCKS} = 0;
$ENV{GIT_TERMINAL_PROMPT} = 0;

# Queries may allocate private transient capture files, but never modify source,
# refs, release state, dependency selections or remote state. No shell evaluation.
sub observe {
    my @command = @_;
    my ($errors) = tempfile(UNLINK => 1);
    my $pid = open my $out, '-|';
    return (127, '', '') unless defined $pid;
    if (!$pid) {
        open STDERR, '>&', $errors or exit 127;
        exec { $command[0] } @command;
        exit 127;
    }
    local $/;
    my $bytes = <$out> // '';
    close $out;
    my $status = $? == 0 ? 0 : ($? >> 8 || 1);
    seek $errors, 0, 0;
    my $diagnostic = <$errors> // '';
    return ($status, $bytes, $diagnostic);
}
sub required {
    my @result = observe(@_);
    die "read-only observation failed: $_[0]\n" if $result[0];
    my $value = $result[1];
    $value =~ s/\n\z//;
    return $value;
}
sub read_json {
    my ($path) = @_;
    open my $fh, '<', $path or die "read $path: $!\n";
    local $/;
    return decode_json(<$fh>);
}
sub public_json {
    my ($url) = @_;
    return undef if $offline;
    my ($status, $bytes) = observe('curl', '--disable', '--fail', '--silent', '--show-error',
        '--proto', '=https', '--connect-timeout', '5', '--max-time', '10',
        '--max-filesize', '1048576', '--user-agent', 'ic-scheduling-release-status', $url);
    return undef if $status;
    return eval { decode_json($bytes) };
}

my $policy = read_json('ci/cargo-release-groups.json');
my $selection = $ENV{RELEASE_GROUP} // '';
my @groups = map { $_->{name} } @{$policy->{groups}};
die "select an existing RELEASE_GROUP\n" if $selection ne '' && !grep { $_ eq $selection } @groups;
@groups = ($selection) if $selection ne '';
my $head = required('git', 'rev-parse', '--verify', 'HEAD');
my $branch = required('git', 'branch', '--show-current');
my $status = required('git', 'status', '--porcelain=v1');
my $state = required('git', 'rev-parse', '--git-path', 'release-state');
my $manifest = decode_json(required('yq', '-p', 'toml', '-o', 'json', '.', 'Cargo.toml'));
my $repository = $manifest->{workspace}{package}{repository} // '';
my ($slug) = $repository =~ m{\Ahttps://github\.com/([A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+?)(?:\.git)?\z};
my $ci = {state => 'unknown', reason => $offline ? 'offline' : 'observation unavailable'};
if ($slug) {
    my $runs = public_json("https://api.github.com/repos/$slug/actions/runs?head_sha=$head&per_page=20");
    if (ref($runs) eq 'HASH' && ref($runs->{workflow_runs}) eq 'ARRAY') {
        my ($run) = grep { ($_->{head_sha} // '') eq $head && ($_->{name} // '') eq 'CI' }
            @{$runs->{workflow_runs}};
        $ci = $run ? {state => $run->{status}, conclusion => $run->{conclusion}, url => $run->{html_url}}
            : {state => 'not observed'};
    }
}
my @blockers;
push @blockers, 'Working tree has uncommitted changes' if $status ne '';
push @blockers, 'Release lock is occupied' if -e "$state/lock" || -l "$state/lock";
push @blockers, 'Source is detached from a branch' if $branch eq '';
push @blockers, 'Exact-commit CI has not reported success'
    unless ($ci->{state} // '') eq 'completed' && ($ci->{conclusion} // '') eq 'success';
my @rows;
for my $group (@groups) {
    local $ENV{RELEASE_GROUP} = $group;
    local $ENV{RELEASE_COMMIT};
    delete $ENV{RELEASE_COMMIT};
    my ($plan_status, $plan_bytes) = observe('bash', 'scripts/release/group.sh', 'plan');
    my $projection = 'working';
    if ($plan_status) {
        push @blockers, "$group working dependency/group metadata is not admitted; prepare the locked cache and check release groups";
        local $ENV{RELEASE_COMMIT} = $head;
        ($plan_status, $plan_bytes) = observe('bash', 'scripts/release/group.sh', 'plan');
        $projection = 'committed';
    }
    die "committed group metadata is unavailable: $group\n" if $plan_status;
    my $plan = decode_json($plan_bytes);
    my ($current, $package) = @{$plan}{qw(previous primary)};
    die "invalid admitted group identity\n" unless stable($current) && $package =~ /\A[a-zA-Z][a-zA-Z0-9_-]*\z/;
    open my $ledger, '<', $plan->{changelog} or die "read group ledger: $!\n";
    my $proposed;
    while (my $line = <$ledger>) {
        next unless $line =~ /^## \[([^\]]+)\] - Unreleased\s*$/;
        $proposed = $1;
        die "invalid pending version\n" unless stable($proposed);
        last;
    }
    my $registry = public_json("https://crates.io/api/v1/crates/$package");
    my $published;
    if (ref($registry) eq 'HASH' && ref($registry->{crate}) eq 'HASH'
        && ($registry->{crate}{id} // '') eq $package
        && stable($registry->{crate}{max_stable_version})) {
        $published = $registry->{crate}{max_stable_version};
    }
    my $tag = "refs/tags/$group-v$current";
    my ($tag_status, $tag_kind) = observe('git', 'cat-file', '-t', $tag);
    my $tag_commit = $tag_status ? undef : required('git', 'rev-parse', "$tag^{commit}");
    my $remote = 'unknown';
    if (!$offline && !$tag_status && $tag_kind eq "tag\n") {
        my $object = required('git', 'rev-parse', $tag);
        my ($destination_status, $destination) = observe('git', 'remote', 'get-url', '--push', '--all', 'origin');
        $destination =~ s/\n\z//;
        if (!$destination_status && $destination ne '' && $destination !~ /[\r\n]/) {
            my ($remote_status, $observation) = observe('git', '-c', 'http.lowSpeedLimit=1',
                '-c', 'http.lowSpeedTime=10', 'ls-remote', '--exit-code', '--', $destination, $tag);
            $remote = $remote_status == 2 ? 'missing' : $remote_status ? 'unknown'
                : $observation eq "$object\t$tag\n" ? 'matches' : 'differs';
        }
    }
    my @notes;
    push @notes, 'Current version is projected from HEAD; working graph admission failed' if $projection eq 'committed';
    push @notes, 'Current source version is newer than the latest observed publication'
        if defined $published && version_cmp($current, $published) > 0;
    push @notes, 'Pending version is newer than the latest observed publication'
        if defined $published && defined $proposed && version_cmp($proposed, $published) > 0;
    push @notes, 'Registry observation unavailable; publication state is unknown' unless defined $published;
    push @notes, 'Current version has no local annotated release tag'
        if $tag_status || $tag_kind ne "tag\n";
    push @notes, 'Current release tag does not identify HEAD'
        if defined $tag_commit && $tag_commit ne $head;
    push @rows, {group => $group, package => $package, current => $current, current_source => $projection,
        published => $published, proposed => $proposed,
        tag => {name => $tag, commit => $tag_commit, remote => $remote}, notes => \@notes};
}
my $report = {schema => 1, source => {commit => $head, branch => $branch,
    clean => $status eq '' ? JSON::PP::true : JSON::PP::false,
    changes => $status, ci => $ci}, groups => \@rows, blockers => \@blockers,
    release_gate => 'The selected standard release workflow must run its complete gate; CI success may cover documentation only.'};
if ($json) { print JSON::PP->new->canonical->pretty->encode($report); }
else {
    print "Source: $head (", $branch || 'detached', ")\n";
    print "CI: $ci->{state}", defined $ci->{conclusion} ? " / $ci->{conclusion}" : '', "\n";
    printf "%-9s %-12s %-12s %-12s\n", 'Group', 'Current', 'Published', 'Proposed';
    for my $row (@rows) {
        printf "%-9s %-12s %-12s %-12s\n", @{$row}{qw(group current)},
            $row->{published} // 'unknown', $row->{proposed} // 'none';
        print "  $_\n" for @{$row->{notes}};
    }
    print "Blocker: $_\n" for @blockers;
    print "$report->{release_gate}\n";
}
