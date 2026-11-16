// Focused offline rehearsal of the CI setup failure boundary; no real tools run.
import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';

const script = path.resolve('ci/setup.mjs');
fs.mkdirSync('target/ci-setup-fixtures', { recursive: true });
const root = fs.mkdtempSync(path.resolve('target/ci-setup-fixtures/run-'));
const secret = 'fixture-private-token-12345';
function check(name, source, prepare = () => {}) {
    const cwd = path.join(root, name);
    fs.mkdirSync(cwd);
    const store = path.join(cwd, '.store');
    const attempt = path.join(store, `rust-${'a'.repeat(64)}`, 'set.fixture');
    fs.mkdirSync(attempt, { recursive: true });
    prepare(cwd, attempt);
    const result = spawnSync(process.execPath, [script, name, process.execPath, '-e', source], {
        cwd, encoding: 'utf8', env: { ...process.env, TOKO_TOOLING_STORE: store, FIXTURE_TOKEN: secret, ATTEMPT: attempt },
    });
    assert.equal(result.status, 23, `${name}: original failure must survive collection`);
    assert.ok(!`${result.stdout}${result.stderr}`.includes(secret));
    return { cwd, result };
}
const hidden = check('hidden', `
    const fs=require('fs');
    fs.writeFileSync(process.env.ATTEMPT+'/install.log', 'compiler failed\\nAuthorization: Bearer '+process.env.FIXTURE_TOKEN);
    fs.writeFileSync(process.env.ATTEMPT+'/private.env', process.env.FIXTURE_TOKEN);
    console.error('compilation failed; status=23\\nCargo tool attempt retained: '+process.env.ATTEMPT);
    console.error('token='+process.env.FIXTURE_TOKEN);
    process.exit(23);
`);
const evidence = path.join(hidden.cwd, 'target/ci-setup');
assert.deepEqual(fs.readdirSync(evidence).sort(), ['hidden-attempt-0.log', 'hidden.log']);
const retained = fs.readFileSync(path.join(evidence, 'hidden-attempt-0.log'), 'utf8');
assert.match(retained, /compiler failed/);
assert.ok(!retained.includes(secret));
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
const large = check('large', 'console.error("x".repeat(200000)); process.exit(23)');
assert.match(large.result.stdout, /TRUNCATED/);
assert.ok(fs.statSync(path.join(large.cwd, 'target/ci-setup/large.log')).size < 66 * 1024);
console.log('CI setup failure: hidden attempt selection, redaction, missing diagnostics, symlink refusal, size limit and original status pass.');
