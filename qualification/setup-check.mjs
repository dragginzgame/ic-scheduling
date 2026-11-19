// Focused offline rehearsal of the CI setup failure boundary; no real tools run.
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';

const script = path.resolve('ci/setup.mjs');
fs.mkdirSync('target/ci-setup-fixtures', { recursive: true });
const root = fs.mkdtempSync(path.resolve('target/ci-setup-fixtures/run-'));
const secret = 'fixture-private-token-12345';
const accessKey = 'fixture-access-value';
const bearer = 'fixture-bearer-value';
const payload = 'fixture-payload-value';
function check(name, source, prepare = () => {}, environment = {}, expectedStatus = 23) {
    const cwd = path.join(root, name);
    fs.mkdirSync(cwd);
    const store = path.join(cwd, '.store');
    const attempt = path.join(store, `rust-${'a'.repeat(64)}`, 'set.fixture');
    fs.mkdirSync(attempt, { recursive: true });
    prepare(cwd, attempt);
    const result = spawnSync(process.execPath, [script, name, process.execPath, '-e', source], {
        cwd, encoding: 'utf8', timeout: 10000,
        env: { ...process.env, CI_PHASE_TIMEOUT_SECONDS: '0', TOKO_TOOLING_STORE: store,
            FIXTURE_TOKEN: secret, FIXTURE_ACCESS_KEY: accessKey, ATTEMPT: attempt, ...environment },
    });
    assert.ifError(result.error);
    assert.equal(result.status, expectedStatus, `${name}: command status must survive collection`);
    assert.ok(!`${result.stdout}${result.stderr}`.includes(secret));
    return { cwd, result };
}
const hidden = check('hidden', `
    const fs=require('fs');
    const diagnostics = 'Authorization: Bearer '+process.env.FIXTURE_TOKEN+
        '\\nBearer ${bearer}\\n{"payload":"${payload}"}\\n'+process.env.FIXTURE_ACCESS_KEY;
    fs.writeFileSync(process.env.ATTEMPT+'/install.log', 'compiler failed\\n'+diagnostics);
    fs.writeFileSync(process.env.ATTEMPT+'/private.env', process.env.FIXTURE_TOKEN);
    console.error('compilation failed; status=23\\nCargo tool attempt retained: '+process.env.ATTEMPT);
    console.error('token='+process.env.FIXTURE_TOKEN);
    console.error(diagnostics);
    process.exit(23);
`);
const evidence = path.join(hidden.cwd, 'target/ci-setup');
assert.deepEqual(fs.readdirSync(evidence).sort(), ['hidden-attempt-0.log', 'hidden.log']);
const retained = fs.readFileSync(path.join(evidence, 'hidden-attempt-0.log'), 'utf8');
assert.match(retained, /compiler failed/);
const reported = fs.readFileSync(path.join(evidence, 'hidden.log'), 'utf8');
for (const value of [secret, accessKey, bearer, payload]) {
    assert.ok(!retained.includes(value));
    assert.ok(!reported.includes(value));
    assert.ok(!hidden.result.stdout.includes(value));
}
const missing = check('missing', 'process.exit(23)');
assert.match(missing.result.stdout, /No diagnostics emitted/);
const collection = check('collection', 'console.error("setup failure"); process.exit(23)', cwd => {
    fs.mkdirSync(path.join(cwd, 'target'));
    fs.writeFileSync(path.join(cwd, 'target/ci-setup'), 'blocks collector');
});
assert.match(collection.result.stderr, /collection failed.*original status=23/);
const symlink = check('symlink', `console.error('Cargo tool attempt retained: '+process.env.ATTEMPT); process.exit(23)`, (cwd, attempt) => {
    fs.writeFileSync(path.join(cwd, 'outside.log'), secret);
    fs.symlinkSync(path.join(cwd, 'outside.log'), path.join(attempt, 'install.log'));
});
assert.match(symlink.result.stderr, /attempt symlink refused/);
assert.deepEqual(fs.readdirSync(path.join(symlink.cwd, 'target/ci-setup')), ['symlink.log']);
// Let piped diagnostics drain before exit; process.exit can discard writes on macOS.
const large = check('large', 'console.error("x".repeat(200000)); process.exitCode = 23');
assert.match(large.result.stdout, /TRUNCATED/);
assert.ok(fs.statSync(path.join(large.cwd, 'target/ci-setup/large.log')).size < 66 * 1024);
const success = check('success', 'console.log("completed")', undefined, { CI_PHASE_TIMEOUT_SECONDS: '5' }, 0);
assert.ok(!fs.existsSync(path.join(success.cwd, 'target/ci-setup')));
const started = Date.now();
const timeout = check('timeout', `
    const {spawn}=require('child_process');
    require('fs').writeFileSync(process.env.ATTEMPT+'/install.log', 'compiler was active\\n'+process.env.FIXTURE_TOKEN);
    console.error('Cargo tool attempt retained: '+process.env.ATTEMPT);
    process.on('SIGTERM', () => process.exit(0));
    spawn(process.execPath, ['-e', \`
        process.on('SIGTERM', () => require('fs').writeFileSync('descendant-term', 'received'));
        console.error('descendant ready');
        setInterval(() => {}, 100);
    \`], {stdio:'inherit'});
    console.error('before hang '+process.env.FIXTURE_TOKEN);
    setInterval(() => {}, 100);
`, undefined, { CI_PHASE_TIMEOUT_SECONDS: '1' }, 124);
assert.ok(Date.now() - started < 8000, 'TERM-resistant descendant must be killed within the grace');
assert.equal(fs.readFileSync(path.join(timeout.cwd, 'descendant-term'), 'utf8'), 'received');
const timeoutLog = fs.readFileSync(path.join(timeout.cwd, 'target/ci-setup/timeout.log'), 'utf8');
assert.match(timeoutLog, /status=124/);
assert.match(timeoutLog, /CI phase timeout after 1 seconds/);
assert.match(timeoutLog, /descendant ready/);
assert.ok(!timeoutLog.includes(secret));
const timeoutAttempt = fs.readFileSync(path.join(timeout.cwd, 'target/ci-setup/timeout-attempt-0.log'), 'utf8');
assert.match(timeoutAttempt, /compiler was active/);
assert.ok(!timeoutAttempt.includes(secret));
console.log('CI command boundary: failure status, bounded/redacted evidence, collector refusal, success and timeout process-group termination pass.');
