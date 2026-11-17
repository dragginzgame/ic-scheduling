// Explicit, expensive adoption check; never part of ordinary CI.
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
const root = process.cwd();
assert.deepEqual(process.argv.slice(2), ['--real']);
fs.mkdirSync('qualification/target', { recursive: true });
const fixture = fs.mkdtempSync(path.resolve('qualification/target/rust-stage-'));
console.log(`Real stage fixture: ${fixture}`);
function run(command, args, cwd = fixture, env = {}, expected = 0) {
    const result = spawnSync(command, args, { cwd, encoding: 'utf8', maxBuffer: 32 * 1024 * 1024,
        env: { ...process.env, PATH: `${fixture}/.tools/host/bin:${fixture}/.tools/rust/common/bin:${process.env.PATH}`, ...env } });
    assert.ifError(result.error);
    const diagnostic = `${command} ${args.join(' ')}\n${result.stdout}\n${result.stderr}`;
    if (expected === 'failure') {
        assert.ok(Number.isInteger(result.status) && result.status !== 0, diagnostic);
    } else {
        assert.equal(result.status, expected, diagnostic);
    }
    return result.stdout.trim();
}
const deleted = new Set(run('git', ['ls-files', '--deleted', '-z'], root).split('\0'));
const files = run('git', ['ls-files', '--cached', '--others', '--exclude-standard', '-z'], root)
    .split('\0').filter(file => file && !deleted.has(file));
for (const file of files) {
    assert.ok(fs.lstatSync(path.join(root, file)).isFile());
    fs.mkdirSync(path.dirname(path.join(fixture, file)), { recursive: true });
    fs.copyFileSync(path.join(root, file), path.join(fixture, file));
    fs.chmodSync(path.join(fixture, file), fs.statSync(path.join(root, file)).mode);
}
run('git', ['init', '-q']);
run('git', ['config', 'user.name', 'Stage qualification']);
run('git', ['config', 'user.email', 'stage@example.invalid']);
run('git', ['config', 'core.hooksPath', '/dev/null']);
run('git', ['add', '--', ...files]);
run('git', ['commit', '-qm', 'Disposable current-source qualification']);
run('make', ['install-tools']);
const installer = 'vendor/toko-tooling/scripts/dev/install-rust-tools.sh';
const editor = run('bash', [installer, '--consumer', fixture, '--package', 'ic-cargo-metadata', '--version', '0.1.0', '--bin', 'ic-cargo-metadata', '--profile', 'release']);
function stage(group) {
    const source = run('git', ['rev-parse', 'HEAD']);
    const env = { RELEASE_SOURCE: source, RELEASE_GROUP: group, QUALIFICATION_SCOPE: 'workspace' };
    const start = performance.now();
    const output = run('bash', ['ci/rust-stage.sh'], fixture, env);
    fs.writeFileSync(path.join(fixture, 'target', `${group}-stage.log`), output);
    const sealed = JSON.parse(run('bash', ['ci/rust-stage.sh', 'check'], fixture, env));
    console.log(`${group}: ${sealed.admission.mode}; ${(performance.now() - start) / 1000}s`);
    return { sealed, env };
}
const first = stage('timers');
assert.equal(first.sealed.admission.mode, 'executed');
const previous = JSON.parse(fs.readFileSync(path.join(fixture, 'ci/cargo-release-groups.json')));
assert.ok(previous.groups.some(group => group.name === 'timers'));
const oldVersion = run('bash', ['vendor/toko-tooling/scripts/ci/cargo-release-group.sh', 'version', 'timers']);
const nextVersion = fs.readFileSync(path.join(fixture, 'releases/timers/CHANGELOG.md'), 'utf8').match(/^## \[([^\]]+)\] - Unreleased$/m)[1];
run('bash', ['vendor/toko-tooling/scripts/ci/cargo-release-group.sh', 'prepare', 'timers'], fixture, {
    RELEASE_PREVIOUS: oldVersion, RELEASE_VERSION: nextVersion, RELEASE_DATE: '2026-11-17',
    CARGO_RELEASE_METADATA_EDITOR: editor, CARGO_RELEASE_METADATA_MAX_BYTES: '8388608',
});
const changed = run('git', ['diff', '--name-only']).split('\n').filter(Boolean);
run('git', ['add', '--', ...changed]);
run('git', ['commit', '-qm', 'Simulated supplier version preparation']);
const second = stage('jobs');
assert.equal(second.sealed.admission.mode, 'reused');
assert.deepEqual(second.sealed.origin, first.sealed.origin);
// Missing or changed original output cannot authorize preparation.
const proof = JSON.parse(fs.readFileSync(second.sealed.admission.execution_proof));
const log = proof.logs.stdout.path;
const bytes = fs.readFileSync(log);
fs.appendFileSync(log, '\nfixture corruption\n');
run('bash', ['ci/rust-stage.sh', 'check'], fixture, second.env, 'failure');
fs.writeFileSync(log, bytes);
run('bash', ['ci/rust-stage.sh', 'check'], fixture, second.env);
const proofPath = second.sealed.admission.execution_proof;
fs.renameSync(proofPath, `${proofPath}.saved`);
try {
    run('bash', ['ci/rust-stage.sh', 'check'], fixture, second.env, 'failure');
} finally {
    fs.renameSync(`${proofPath}.saved`, proofPath);
}
run('bash', ['ci/rust-stage.sh', 'check'], fixture, second.env);
const pointer = path.join(fixture, 'target/release-validation', `rust-jobs-${second.env.RELEASE_SOURCE}.path`);
const attempt = fs.readFileSync(pointer, 'utf8').trim();
const projection = JSON.parse(fs.readFileSync(path.join(attempt, 'projection.json')));
const candidateFile = path.join(projection.directory, 'ci/qualify.sh');
const candidateBytes = fs.readFileSync(candidateFile);
fs.appendFileSync(candidateFile, '\n# fixture drift\n');
try {
    run('bash', ['ci/rust-stage.sh', 'check'], fixture, second.env, 'failure');
} finally {
    fs.writeFileSync(candidateFile, candidateBytes);
}
run('bash', ['ci/rust-stage.sh', 'check'], fixture, second.env);
console.log('Real Rust stage: cross-group reuse, changed logs, missing proof, candidate drift and restored-evidence recovery pass. No publication performed.');
