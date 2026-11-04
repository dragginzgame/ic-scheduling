#!/usr/bin/env perl
use strict;
use warnings;
use FindBin qw($RealBin);
use lib "$RealBin/../ci";
use ValidationPlan qw(json bytes digest document relative_path);
use JSON::PP;
use File::Find;
use File::Spec;
use File::Temp qw(tempfile);
use File::Path qw(make_path);
use Cwd qw(abs_path);
use POSIX qw(uname);

# Scheduling owns product scope, configuration, source custody and metadata
# projection. Shared owns roster/baseline observation and run-evidence parsing.
my ($operation, $directory) = @ARGV;
defined($operation) && $operation =~ /\A(?:roster|ci-stages|release-stages|select|seal|check|identity)\z/
    && @ARGV == ($operation =~ /\A(?:roster|ci-stages|release-stages)\z/ ? 1 : 2)
    or die "usage: validation.pl roster|ci-stages|release-stages | select|seal|check|identity DIRECTORY\n";
my $root = abs_path("$RealBin/../..");
chdir $root or die "enter release root: $!\n";
my $policy_path = 'ci/release-validation.json';
my $decoder = JSON::PP->new->utf8;
# These four standalone Timers obligations are release-only. Every other policy
# stage belongs to full CI, including newly declared stages. No core CI is skipped.
my %installed = map { $_ => 1 } qw(install-testkit-tools install-consumer-server test-timer-watchdog test-timer-cohorts);
# Closed omissions, not a second stage roster. A new obligation is required by
# default. Tooling/configuration changes cannot enter Jobs-only scope below.
my %jobs_omissions = (
    (map { $_ => 'Standalone Timers sources, probes and supplier graph are unchanged; Jobs/Timers consumer checks remain required' }
        qw(clippy-timers check-timer-provider check-timer-dependencies check-timer-probes test-timers test-timer-dependencies docs-timers package-timers)),
    (map { $_ => 'Release/CI/formatting tooling and its configuration are unchanged since the admitted Jobs baseline' }
        qw(test-release-tooling test-release-validation test-formatting-evidence test-ci-change-selection test-release-status formatting-hook-check)),
    (map { $_ => 'Installed Timers inputs and supplier graph are unchanged; installed Jobs recovery remains separately selected' }
        keys %installed),
);
sub observe {
    open my $in, '-|', @_ or die "execute $_[0]: $!\n";
    my $raw = '';
    while (1) {
        my $n = read $in, my $chunk, 65536;
        defined($n) or die "read observation: $!\n";
        last unless $n;
        $raw .= $chunk;
        length($raw) <= 4194304 or die "observation exceeds 4 MiB\n";
    }
    close $in or die "failed observation: @_\n";
    $raw =~ s/\n\z//;
    return $raw;
}
sub read_json { document($root, relative($_[0])) }
sub relative { relative_path(File::Spec->abs2rel(File::Spec->rel2abs($_[0]), $root)) }
sub hash_file { digest($root, relative($_[0])) }
sub shared { observe($^X, 'scripts/ci/validation-evidence.pl', $_[0], $root, @_[1..$#_]) }
sub write_json {
    my ($path, $value) = @_;
    !-e $path && !-l $path or die "refuse to replace validation evidence: $path\n";
    open my $out, '>:raw', $path or die "write $path: $!\n";
    print {$out} json($value) or die "write $path: $!\n";
    close $out or die "close $path: $!\n";
}
sub roster {
    my $cargo = $decoder->decode(bytes($root, 'ci/cargo-release-groups.json'));
    make_path('target/release-validation');
    my ($out, $path) = tempfile('groups.XXXXXXXX', DIR=>'target/release-validation', UNLINK=>1);
    print {$out} json([map { $_->{name} } @{$cargo->{groups}}]) or die "write Cargo group names: $!\n";
    close $out or die "close Cargo group names: $!\n";
    my @stages = split /\n/, shared('roster', $policy_path, $path);
    !grep { $_ eq 'ci' || $_ eq 'release-verify' } @stages or die "recursive validation stage\n";
    for my $stage (keys %installed) {
        grep { $_ eq $stage } @stages or die "missing standalone Timers obligation: $stage\n";
    }
    grep { !$installed{$_} } @stages or die "missing common CI obligations\n";
    my $standalone = 0;
    for my $stage (@stages) {
        $standalone = 1 if $installed{$stage};
        !$standalone || $installed{$stage} or die "common CI must precede standalone Timers stages\n";
    }
    return @stages;
}
if ($operation =~ /\A(?:roster|ci-stages|release-stages)\z/) {
    my @roster = roster();
    print "$_\n" for grep { $operation ne 'ci-stages' || !$installed{$_} } @roster;
    exit;
}
my $group = $ENV{RELEASE_GROUP} // '';
$group =~ /\A(?:jobs|timers)\z/ or die "select RELEASE_GROUP=jobs or timers\n";
my $source = $ENV{RELEASE_SOURCE} // '';
$source =~ /\A(?:[0-9a-f]{40}|[0-9a-f]{64})\z/
    && observe('git', 'cat-file', '-t', $source) eq 'commit'
    or die "validation needs an admitted RELEASE_SOURCE commit\n";
sub configuration {
    my @host = uname();
    my %environment = map { $_ => $ENV{$_} }
        grep { /\A(?:RUSTFLAGS|RUSTDOCFLAGS|RUSTDOC|RUSTUP_(?:TOOLCHAIN|HOME)|RUSTC(?:_WRAPPER|_WORKSPACE_WRAPPER|_BOOTSTRAP)?|CARGO_HOME|CARGO_ENCODED_RUSTFLAGS|CARGO_(?:BUILD|TARGET|PROFILE)_.+)\z/ }
        keys %ENV;
    return {os=>$host[0], architecture=>$host[4], make=>observe('make', '--version'),
        rustc=>observe('rustc', '-vV'), cargo=>observe('cargo', '-vV'), environment=>\%environment};
}
sub inventory {
    return [sort grep { length } split /\0/,
        observe('git', 'ls-files', '-z', '--cached', '--others', '--exclude-standard')];
}
sub release_policy {
    roster();
    return read_json($policy_path);
}
sub source_catalog {
    local $ENV{RELEASE_COMMIT} = $source;
    my $catalog = $decoder->decode(observe('bash', 'scripts/release/group.sh', 'plan'));
    $catalog->{group} eq $group or die "wrong Cargo group\n";
    return $catalog;
}
sub select_scope {
    my ($baseline, $dirty) = @_;
    my $scope = 'workspace';
    my $reason = 'Unknown or shared scope retains the complete release gate';
    if ($group eq 'timers') {
        $reason = 'Timers owns the provider, installed watcher and policy-cohort contracts';
    } elsif ($dirty) {
        $reason = 'Uncommitted inputs require the complete release gate';
    } elsif ($baseline->{available}) {
        # Closed product ownership mapping. Manifests, locks, tooling and unknown
        # files broaden; an annotated ancestor alone never grants an omission.
        my @outside = grep { !m{\A(?:
            crates/ic-jobs/(?:src|tests|examples)/|
            apps/ic-jobs-test-consumer/(?:src|tests)/|
            crates/ic-jobs-canister-tests/(?:src|tests|examples)/|
            releases/jobs/CHANGELOG\.md\z
        )}x } @{$baseline->{changes}};
        if (!@outside) {
            $scope = 'group';
            $reason = 'Jobs-owned changes since its annotated release; standalone Timers and tooling inputs are unchanged';
        }
    }
    return ($scope, $reason);
}
sub select_stages {
    my ($policy, $scope, $reason) = @_;
    return [map {
        my $required = $scope eq 'workspace' || !exists $jobs_omissions{$_};
        {target=>$_, required=>$required ? JSON::PP::true : JSON::PP::false,
            reason=>$required ? $reason : $jobs_omissions{$_}}
    } @{$policy->{stages}}];
}
sub select_plan {
    my $release_policy = release_policy();
    observe('git', 'rev-parse', 'HEAD') eq $source or die "stale validation source\n";
    my $catalog = source_catalog();
    my $tag = "$group-v$catalog->{previous}";
    my $baseline = $decoder->decode(shared('baseline', $source, "refs/tags/$tag"));
    my $dirty = length observe('git', 'status', '--porcelain=v1', '--untracked-files=all');
    my ($scope, $reason) = select_scope($baseline, $dirty);
    my $context = {schema=>1, source=>$source, group=>$group, attempt=>relative($directory),
        baseline_tag=>$tag, baseline_object=>$baseline->{object},
        dirty=>$dirty ? JSON::PP::true : JSON::PP::false,
        configuration=>configuration(), inventory=>inventory()};
    write_json("$directory/context.json", $context);
    # CI and release use the same flat roster. Shared checkpoints each selected
    # obligation; omissions retain their own explicit not-applicable reason.
    write_json("$directory/release-policy.json", $release_policy);
    write_json("$directory/plan.json", {schema=>1, source=>$source, group=>$group,
        scope=>$scope, policy=>relative("$directory/release-policy.json"),
        inputs=>[@{$context->{inventory}}, relative("$directory/context.json")],
        stages=>select_stages($release_policy, $scope, $reason)});
}
sub admission {
    my ($base, $proof) = @_;
    my $context = read_json("$base/context.json");
    my $plan = read_json("$base/plan.json");
    $proof->{schema} == 1 && $proof->{kind} eq 'group-validation-evidence'
        && $proof->{source} eq $source && $proof->{group} eq $group
        && $context->{source} eq $source && $context->{group} eq $group
        or die "validation source/group changed\n";
    $proof->{plan_sha256} eq hash_file("$base/plan.json")
        && $proof->{policy_sha256} eq hash_file("$base/release-policy.json")
        && $proof->{context_sha256} eq hash_file("$base/context.json")
        or die "validation selection/context changed\n";
    json(read_json("$base/release-policy.json")) eq json(release_policy())
        or die "release policy differs from the declared full roster\n";
    my $catalog = source_catalog();
    $context->{baseline_tag} eq "$group-v$catalog->{previous}"
        or die "validation baseline does not match the admitted source version\n";
    my $baseline = $decoder->decode(shared('baseline', $source, "refs/tags/$context->{baseline_tag}"));
    json($context->{baseline_object}) eq json($baseline->{object})
        or die "validation baseline tag changed\n";
    my ($scope, $reason) = select_scope($baseline, $context->{dirty});
    $plan->{scope} eq $scope
        && json($plan->{stages}) eq json(select_stages(release_policy(), $scope, $reason))
        or die "validation selection differs from the product mapping\n";
    json($context->{configuration}) eq json(configuration()) or die "validation host/compiler/build configuration changed\n";
    json($context->{inventory}) eq json(inventory()) or die "validation source inventory changed\n";
    if (defined $context->{baseline_object}) {
        observe('git', 'rev-parse', "refs/tags/$context->{baseline_tag}") eq $context->{baseline_object}
            or die "validation baseline tag changed\n";
    }
    my %metadata = map { $_=>1 } @{$catalog->{files}};
    my %generated = map { ("$context->{attempt}/$_", hash_file("$base/$_")) } qw(plan.json context.json release-policy.json);
    # Shared already admitted every runner event, timing, log and checkpoint.
    # Custody below checks that sealed evidence and all current inputs retain
    # those bytes. Only independently admitted release metadata may be projected.
    for my $name (@{$context->{inventory}}) {
        exists $proof->{inputs}{$name} or die "missing source inventory input: $name\n";
    }
    for my $name (keys %{$proof->{inputs}}) {
        next if $operation ne 'seal' && $metadata{$name};
        my $actual = $generated{$name} // digest($root, $name);
        $actual eq $proof->{inputs}{$name} or die "validated source input changed: $name\n";
    }
    for my $name (keys %generated) {
        ($proof->{inputs}{$name} // '') eq $generated{$name} or die "missing generated selection input\n";
    }
    return {schema=>2, kind=>'scheduling-group-validation', source=>$source,
        group=>$group, scope=>$plan->{scope}, shared_proof_sha256=>hash_file("$base/shared-proof.json")};
}
sub evidence_inventory {
    my ($base) = @_;
    my %files;
    find({no_chdir=>1, wanted=>sub {
        !-l $_ or die "symlink in retained validation evidence\n";
        return if -d $_;
        my $name = File::Spec->abs2rel($_, $base);
        return if $name eq 'execution.json';
        $files{$name} = hash_file($_);
    }}, $base);
    return \%files;
}
if ($operation eq 'select') {
    select_plan();
} elsif ($operation eq 'seal') {
    observe('git', 'rev-parse', 'HEAD') eq $source or die "validation HEAD moved\n";
    my @runs = grep { -f "$_/parsed-inputs.json.plan.json" } glob "$directory/logs/validation.*";
    @runs == 1 or die "missing or ambiguous validation run\n";
    my $proof = $decoder->decode(shared('verify', relative("$directory/plan.json"), $source,
        $group, relative($runs[0]), relative("$directory/context.json")));
    write_json("$directory/shared-proof.json", $proof);
    my $value = admission($directory, $proof);
    $value->{files} = evidence_inventory($directory);
    write_json("$directory/execution.json", $value);
} else {
    if ($operation eq 'identity') {
        my $name = bytes($root, relative("$directory/validation-evidence"));
        $name =~ /\A(validation\.[A-Za-z0-9]+)\n\z/ or die "invalid validation evidence pointer\n";
        $directory .= "/$1";
    }
    my $saved = read_json("$directory/execution.json");
    $saved->{schema} == 2 && $saved->{kind} eq 'scheduling-group-validation'
        or die "unknown group validation receipt; preserve its original adapter\n";
    json(evidence_inventory($directory)) eq json($saved->{files}) or die "retained validation evidence changed\n";
    my $value = admission($directory, read_json("$directory/shared-proof.json"));
    $value->{files} = $saved->{files};
    json($value) eq json($saved) or die "retained validation admission changed\n";
    print "scheduling-group-validation-v2\n", hash_file("$directory/execution.json"), "\n"
        if $operation eq 'identity';
}
