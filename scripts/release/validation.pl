#!/usr/bin/env perl
use strict;
use warnings;
use JSON::PP;
use Digest::SHA qw(sha256_hex);
use File::Find;
use File::Spec;
use POSIX qw(uname);

# Scheduling owns applicability and receipt composition. Shared owns plan
# admission, checkpointing and Make execution. No stage-proof reuse is enabled.
my $json = JSON::PP->new->canonical->utf8;
my ($operation, $directory) = @ARGV;
@ARGV == 2 && $operation =~ /\A(?:select|seal|check|identity)\z/
    or die "usage: validation.pl select|seal|check|identity DIRECTORY\n";
my $root = observe('git', 'rev-parse', '--show-toplevel');
chdir $root or die "enter release root: $!\n";
my $group = $ENV{RELEASE_GROUP} // '';
$group =~ /\A(?:jobs|timers)\z/ or die "select RELEASE_GROUP=jobs or timers\n";
my $source = $ENV{RELEASE_SOURCE} // '';
$source =~ /\A(?:[0-9a-f]{40}|[0-9a-f]{64})\z/
    && observe('git', 'cat-file', '-t', $source) eq 'commit'
    or die "validation needs an admitted RELEASE_SOURCE commit\n";
my $policy_path = 'ci/release-validation.json';

sub observe {
    open my $in, '-|', @_ or die "execute $_[0]: $!\n";
    local $/; my $text = <$in> // '';
    close $in or die "failed observation: @_\n";
    $text =~ s/\n\z//;
    return $text;
}
sub regular {
    my ($path) = @_;
    my $absolute = File::Spec->rel2abs($path);
    my $route = '';
    for my $part (split m{/}, $absolute) {
        next unless length $part;
        $route .= "/$part";
        !-l $route or die "symlink in validation evidence/input: $path\n";
    }
    -f $absolute or die "missing regular validation evidence/input: $path\n";
    return $absolute;
}
sub bytes {
    open my $in, '<:raw', regular($_[0]) or die "read $_[0]: $!\n";
    local $/; my $text = <$in> // '';
    close $in or die "close $_[0]: $!\n";
    return $text;
}
sub digest {
    open my $in, '<:raw', regular($_[0]) or die "hash $_[0]: $!\n";
    my $hash = Digest::SHA->new(256)->addfile($in)->hexdigest;
    close $in or die "close $_[0]: $!\n";
    return $hash;
}
sub read_json {
    my $text = bytes($_[0]);
    my $value = $json->decode($text);
    $text eq $json->encode($value)."\n" or die "noncanonical validation JSON: $_[0]\n";
    return $value;
}
sub write_json {
    my ($path, $value) = @_;
    !-e $path && !-l $path or die "refuse to replace validation evidence: $path\n";
    open my $out, '>:raw', $path or die "write $path: $!\n";
    print {$out} $json->encode($value), "\n" or die "write $path: $!\n";
    close $out or die "close $path: $!\n";
}
sub configuration {
    my @host = uname();
    my %environment = map { $_ => $ENV{$_} }
        grep { /\A(?:RUSTFLAGS|RUSTDOCFLAGS|RUSTDOC|RUSTUP_(?:TOOLCHAIN|HOME)|RUSTC(?:_WRAPPER|_WORKSPACE_WRAPPER|_BOOTSTRAP)?|CARGO_HOME|CARGO_ENCODED_RUSTFLAGS|CARGO_(?:BUILD|TARGET|PROFILE)_.+)\z/ }
        keys %ENV;
    return {os => $host[0], architecture => $host[4], make => observe('make', '--version'),
        rustc => observe('rustc', '-vV'), cargo => observe('cargo', '-vV'),
        environment => \%environment};
}
sub inventory {
    return [sort grep { length } split /\0/,
        observe('git', 'ls-files', '-z', '--cached', '--others', '--exclude-standard')];
}
sub policy {
    my $value = read_json($policy_path);
    $json->encode([sort keys %$value]) eq $json->encode([qw(groups schema stages)])
        && $value->{schema} == 1 or die "unknown release validation policy\n";
    my $cargo = $json->decode(bytes('ci/cargo-release-groups.json'));
    $json->encode([sort @{$value->{groups}}])
        eq $json->encode([sort map { $_->{name} } @{$cargo->{groups}}])
        or die "validation groups differ from Cargo release policy\n";
    my @roster = split /\n/, observe('make', '--silent', '--no-print-directory', 'release-validation-stages');
    $json->encode(\@roster) eq $json->encode($value->{stages})
        or die "validation stage roster differs from the complete Make release gate\n";
    return $value;
}
sub select_plan {
    my $policy = policy();
    observe('git', 'rev-parse', 'HEAD') eq $source or die "stale validation source\n";
    my $catalog = $json->decode(observe('bash', 'scripts/release/group.sh', 'plan'));
    $catalog->{group} eq $group or die "wrong Cargo group\n";
    my $tag = "$group-v$catalog->{previous}";
    my ($baseline, $tag_object);
    # Missing, lightweight, divergent or otherwise uncertain baselines broaden
    # to the full roster. An annotated release tag is a trusted release boundary,
    # not a reusable execution proof or an arbitrary caller-selected Git base.
    if (system('git', 'show-ref', '--verify', '--quiet', "refs/tags/$tag") == 0
        && observe('git', 'cat-file', '-t', "refs/tags/$tag") eq 'tag') {
        $tag_object = observe('git', 'rev-parse', "refs/tags/$tag");
        my $commit = observe('git', 'rev-parse', "refs/tags/$tag^{commit}");
        $baseline = $commit if system('git', 'merge-base', '--is-ancestor', $commit, $source) == 0;
    }
    my $dirty = length observe('git', 'status', '--porcelain=v1', '--untracked-files=all');
    my $scope = 'workspace';
    my $reason = 'Unknown or shared scope retains the complete release gate';
    if ($group eq 'timers') {
        $reason = 'Timers owns the provider, installed watcher and policy-cohort contracts';
    } elsif ($dirty) {
        $reason = 'Uncommitted inputs require the complete release gate';
    } elsif (defined $baseline) {
        my @changed = grep { length } split /\0/,
            observe('git', 'diff', '--name-only', '-z', '--no-renames', $baseline, $source, '--');
        # Closed ownership mapping, not a generic extension or group-name skip.
        # Manifests, locks, build scripts, shared code and unknown paths broaden.
        my @outside = grep { !m{\A(?:
            crates/ic-jobs/(?:src|tests|examples)/|
            apps/ic-jobs-test-consumer/(?:src|tests)/|
            crates/ic-jobs-canister-tests/(?:src|tests|examples)/|
            releases/jobs/CHANGELOG\.md\z
        )}x } @changed;
        if (!@outside) {
            $scope = 'group';
            $reason = 'Jobs-owned changes since its annotated release; standalone Timers probe inputs and dependency graph are unchanged';
        }
    }
    my $context = {schema => 1, source => $source, group => $group,
        baseline_tag => $tag, baseline_object => $tag_object, baseline_source => $baseline,
        dirty => $dirty ? JSON::PP::true : JSON::PP::false,
        configuration => configuration(), inventory => inventory()};
    write_json("$directory/context.json", $context);
    my $context_path = File::Spec->abs2rel(File::Spec->rel2abs("$directory/context.json"), $root);
    my @stages = map {
        my $required = $scope eq 'workspace' || $_ eq 'ci';
        {target => $_, required => $required ? JSON::PP::true : JSON::PP::false,
            reason => $_ eq 'ci' ? 'Complete native/Wasm CI retains Jobs, Timers and combined-consumer contracts' : $reason}
    } @{$policy->{stages}};
    write_json("$directory/plan.json", {schema => 1, source => $source, group => $group,
        scope => $scope, policy => $policy_path,
        inputs => [@{$context->{inventory}}, $context_path], stages => \@stages});
}
sub execution {
    my ($base, $origin) = @_;
    my $context = read_json("$base/context.json");
    my $plan = read_json("$base/plan.json");
    my @logs = grep { -f "$_/parsed-inputs.json.plan.json" } glob "$base/logs/validation.*";
    @logs == 1 && -d $logs[0] && !-l $logs[0] or die "missing or ambiguous validation run\n";
    my $logs = $logs[0];
    $json->encode(read_json("$logs/parsed-inputs.json.plan.json")) eq $json->encode($plan)
        && $json->encode(read_json("$logs/parsed-inputs.json.policy.json")) eq $json->encode(policy())
        or die "admitted validation plan/policy changed\n";
    $plan->{source} eq $source && $plan->{group} eq $group
        && $context->{source} eq $source && $context->{group} eq $group
        or die "validation source/group changed\n";
    $json->encode($context->{configuration}) eq $json->encode(configuration())
        or die "validation host/compiler/build configuration changed\n";
    $json->encode($context->{inventory}) eq $json->encode(inventory())
        or die "validation source inventory changed\n";
    if (defined $context->{baseline_object}) {
        observe('git', 'rev-parse', "refs/tags/$context->{baseline_tag}") eq $context->{baseline_object}
            or die "validation baseline tag changed\n";
    }
    my $checkpoint = read_json("$logs/plan-inputs.json");
    my $context_input = $plan->{inputs}[-1];
    ($checkpoint->{$context_input} // '') eq digest("$base/context.json")
        or die "validation selection context changed\n";
    # Version preparation is independently admitted by the existing exact
    # metadata receipts. Every other source input must still equal the gate.
    my $catalog = $json->decode(observe('bash', 'scripts/release/group.sh', 'plan'));
    my %metadata = map { $_ => 1 } @{$catalog->{files}};
    for my $name (@{$context->{inventory}}) {
        exists $checkpoint->{$name} or die "missing validation input: $name\n";
        next if $operation ne 'seal' && $metadata{$name};
        digest($name) eq $checkpoint->{$name} or die "validated source input changed: $name\n";
    }
    my @timings = split /\n/, bytes("$logs/timings.tsv");
    shift(@timings) eq "target\tresult\tseconds\tlog" or die "invalid validation timings\n";
    my @required = grep { $_->{required} } @{$plan->{stages}};
    @timings == @required or die "missing required validation evidence\n";
    my %states;
    my @events = split /\n/, bytes("$logs/plan-events.tsv");
    shift(@events) eq "target\tstate\treason" or die "invalid validation events\n";
    for (@events) { my ($target, $state) = split /\t/; $states{$target} = $state; }
    my @obligations;
    for my $stage (@{$plan->{stages}}) {
        my $target = $stage->{target};
        my $expected = $stage->{required} ? 'PASS' : 'not-applicable';
        ($states{$target} // '') eq $expected or die "incomplete/failed validation stage: $target\n";
        push @obligations, {target => $target, state => $stage->{required} ? 'executed' : 'not-applicable'};
        next unless $stage->{required};
        my ($observed, $result, $seconds, $path) = split /\t/, shift @timings;
        $observed eq $target && $result eq 'PASS' && $seconds =~ /\A[0-9]+\z/
            or die "required validation stage did not pass: $target\n";
        my $relative = File::Spec->abs2rel($path, $origin);
        $relative !~ m{\A\.\.(?:/|\z)} or die "validation log escaped its attempt\n";
        regular("$base/$relative");
    }
    return {schema => 1, kind => 'scheduling-group-validation', source => $source,
        group => $group, original_directory => $origin, scope => $plan->{scope},
        configuration => $context->{configuration}, obligations => \@obligations};
}
sub evidence_inventory {
    my ($base) = @_;
    my %files;
    find({no_chdir => 1, wanted => sub {
        !-l $_ or die "symlink in retained validation evidence\n";
        return if -d $_;
        my $name = File::Spec->abs2rel($_, $base);
        return if $name eq 'execution.json';
        $files{$name} = digest($_);
    }}, $base);
    return \%files;
}

if ($operation eq 'select') {
    select_plan();
} elsif ($operation eq 'seal') {
    observe('git', 'rev-parse', 'HEAD') eq $source or die "validation HEAD moved\n";
    my $value = execution($directory, File::Spec->rel2abs($directory));
    $value->{files} = evidence_inventory($directory);
    write_json("$directory/execution.json", $value);
} else {
    if ($operation eq 'identity') {
        my $name = bytes("$directory/validation-evidence");
        $name =~ /\A(validation\.[A-Za-z0-9]+)\n\z/ or die "invalid validation evidence pointer\n";
        $directory .= "/$1";
    }
    my $saved = read_json("$directory/execution.json");
    $saved->{schema} == 1 && $saved->{kind} eq 'scheduling-group-validation'
        or die "unknown group validation receipt\n";
    my $value = execution($directory, $saved->{original_directory});
    $value->{files} = evidence_inventory($directory);
    $json->encode($value) eq $json->encode($saved) or die "retained validation evidence changed\n";
    print "scheduling-group-validation-v1\n", digest("$directory/execution.json"), "\n"
        if $operation eq 'identity';
}
