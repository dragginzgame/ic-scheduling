import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
const root = process.cwd();
fs.mkdirSync('qualification/target', { recursive: true });
const fixture = fs.mkdtempSync(path.resolve('qualification/target/scope-'));
function run(command, args, env = {}, expected = 0) {
    const result = spawnSync(command, args, { cwd: fixture, encoding: 'utf8',
        env: { ...process.env, PATH: `${root}/.tools/host/bin:${process.env.PATH}`, ...env } });
    assert.ifError(result.error);
    assert.equal(result.status, expected, result.stdout + result.stderr);
    return result.stdout.trim();
}
for (const item of ['ci', 'vendor/toko-tooling']) {
    fs.cpSync(path.join(root, item), path.join(fixture, item), { recursive: true });
}
run('git', ['init', '-q']);
run('git', ['config', 'user.name', 'fixture']);
run('git', ['config', 'user.email', 'fixture@example.invalid']);
fs.writeFileSync(path.join(fixture, '.gitignore'), '/target/\n');
run('git', ['add', '--', 'ci', 'vendor', '.gitignore']);
run('git', ['commit', '-qm', 'baseline']);
run('git', ['tag', '-am', 'qualified baseline', 'jobs-v0.39.3']);
function change(name) {
    fs.mkdirSync(path.dirname(path.join(fixture, name)), { recursive: true });
    fs.appendFileSync(path.join(fixture, name), '\n// fixture change\n');
    run('git', ['add', '--', name]);
    run('git', ['commit', '-qm', name]);
}
const select = (group, previous = '0.39.3', status = 0) => run('bash', ['ci/validation-scope.sh'], {
    RELEASE_GROUP: group, RELEASE_PREVIOUS: previous,
    RELEASE_SOURCE: run('git', ['rev-parse', 'HEAD']),
}, status);
assert.equal(select('jobs'), 'workspace');
change('crates/ic-jobs/src/state.rs');
assert.equal(select('jobs'), 'jobs');
const policy = JSON.parse(fs.readFileSync(path.join(root, 'ci/release-validation.json')));
fs.writeFileSync(path.join(fixture, 'Makefile'), policy.stages.map(target =>
    `.PHONY: ${target}\n${target}:\n\t@printf '%s\\n' '${target}:$(QUALIFICATION_SCOPE)' >> "$$CALLS"\n\t@test "$$FAIL_TARGET" != '${target}'\n`).join('\n'));
fs.writeFileSync(path.join(fixture, '.gitignore'), '/target/\n');
const calls = path.join(fixture, 'target/calls');
fs.mkdirSync(path.dirname(calls), { recursive: true });
function dispatch(failure, expected) {
    fs.writeFileSync(calls, '');
    run('bash', ['ci/qualify-release.sh'], {
        RELEASE_GROUP: 'jobs', RELEASE_PREVIOUS: '0.39.3',
        RELEASE_SOURCE: run('git', ['rev-parse', 'HEAD']), CALLS: calls, FAIL_TARGET: failure,
    }, expected);
    return fs.readFileSync(calls, 'utf8').trim().split('\n');
}
const selected = policy.stages.filter(stage => !['check-formatting-hooks', 'check-executor', 'check-metrics', 'qualify-entropy'].includes(stage));
assert.deepEqual(dispatch('check-executor', 0), selected.map(stage => `${stage}:jobs`));
const failed = dispatch('qualify-rust', 2);
assert.ok(!failed.some(call => call.startsWith('check-calendar-wasm:')), 'required failure must stop later stages');
// Dispatcher fixtures are temporary inputs, not source changes in the baseline test.
fs.unlinkSync(path.join(fixture, 'Makefile'));
fs.rmSync(path.join(fixture, 'target'), { recursive: true });
assert.equal(select('timers'), 'workspace');
assert.equal(select('jobs', '99.0.0'), 'workspace');
select('unknown', '0.39.3', 2);
change('Cargo.lock');
assert.equal(select('jobs'), 'workspace');
console.log('Release scope: isolated Jobs, Timers reverse consumers, missing baseline, shared locks and unknown groups pass.');
run('git', ['remote', 'add', 'origin', 'https://example.invalid/fixture.git']);
const mock = fs.mkdtempSync(path.resolve('qualification/target/scope-tools-'));
fs.writeFileSync(path.join(mock, 'make'), '#!/bin/sh\nmkdir -p target/release-validation\nprintf "ordinary\\n" > "target/release-validation/rust-$RELEASE_GROUP-$RELEASE_SOURCE.path"\nif [ "${FIXTURE_DRIFT:-}" = 1 ]; then echo drift >> Cargo.lock; fi\nexit "${FIXTURE_STATUS:-0}"\n', { mode: 0o755 });
const source = run('git', ['rev-parse', 'HEAD']);
const environment = {
    PATH: `${mock}:${root}/.tools/host/bin:${process.env.PATH}`,
    RELEASE_SOURCE: source, RELEASE_GROUP: 'jobs', RELEASE_PREVIOUS: '0.39.3',
    RELEASE_VERSION: '0.39.4', RELEASE_DATE: '2026-11-17',
    RELEASE_REMOTE: 'origin', RELEASE_BRANCH: 'main',
};
run('bash', ['ci/release.sh', 'verify'], environment);
const receipt = path.join(fixture, '.git/release-state', `scheduling-jobs-${source}.validated`);
const valid = fs.readFileSync(receipt, 'utf8');
assert.match(valid, /scheduling-qualification-v2/);
run('bash', ['ci/release.sh', 'verify'], { ...environment, FIXTURE_STATUS: '23' }, 23);
assert.ok(!fs.existsSync(receipt), 'failed retry must invalidate old success');
run('bash', ['ci/release.sh', 'verify'], environment);
assert.equal(fs.readFileSync(receipt, 'utf8'), valid);
const lockBytes = fs.readFileSync(path.join(fixture, 'Cargo.lock'));
run('bash', ['ci/release.sh', 'verify'], { ...environment, FIXTURE_DRIFT: '1' }, 1);
assert.ok(!fs.existsSync(receipt), 'input drift must not publish a receipt');
// Restore only the disposable fixture's deliberately changed input.
fs.writeFileSync(path.join(fixture, 'Cargo.lock'), lockBytes);
run('bash', ['ci/release.sh', 'verify'], environment);
fs.appendFileSync(receipt, 'tampered\n');
run('bash', ['ci/release.sh', 'prepare'], environment, 1);
console.log('Release receipt: successful retry, failed-attempt invalidation and tampered-evidence refusal pass. Gate effects were simulated.');
