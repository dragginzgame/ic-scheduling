// Exercise the actual archive dispatcher without compiling unrelated products.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';

const root = process.cwd();
fs.mkdirSync('qualification/target', { recursive: true });
const fixture = fs.mkdtempSync(path.resolve('qualification/target/archive-scope-'));
const bin = path.join(fixture, '.tools/host/bin');
fs.mkdirSync(bin, { recursive: true });
fs.writeFileSync(path.join(bin, 'cargo'), `#!/usr/bin/env node
const fs = require('fs');
const args = process.argv.slice(2);
const offline = process.env.CARGO_NET_OFFLINE ?? null;
fs.appendFileSync(process.env.CALLS, JSON.stringify({ args, offline })+'\\n');
if (args[0] === process.env.FAIL_COMMAND) process.exit(23);
if (args[0] === 'metadata') console.log(JSON.stringify({packages:[
  {name:'ic-timers',version:'0.46.3'}, {name:'ic-jobs',version:'0.39.3'}
]}));
`, { mode: 0o755 });
function check(scope, offline = null, failure = '') {
    const calls = path.join(fixture, `${scope}-${offline ?? 'unset'}-${failure || 'pass'}.jsonl`);
    const env = { ...process.env, CALLS: calls, FAIL_COMMAND: failure,
        PATH: `${root}/.tools/host/bin:${process.env.PATH}` };
    if (offline === null) delete env.CARGO_NET_OFFLINE;
    else env.CARGO_NET_OFFLINE = offline;
    const result = spawnSync('bash', [path.join(root, 'ci/qualify.sh'), 'archives', scope], {
        cwd: fixture, encoding: 'utf8', env,
    });
    assert.ifError(result.error);
    const invocations = fs.existsSync(calls)
        ? fs.readFileSync(calls, 'utf8').trim().split('\n').map(JSON.parse) : [];
    const commands = invocations.map(call => call.args);
    if (scope === 'unknown') {
        assert.equal(result.status, 2);
        assert.deepEqual(commands, []);
        return;
    }
    assert.equal(result.status, failure ? 23 : 0, result.stderr);
    for (const call of invocations) {
        assert.equal(call.offline, offline);
        assert.ok(!call.args.some(arg => arg === '--offline' || arg.includes('net.offline')));
    }
    assert.deepEqual(commands[0], ['package', ...(scope === 'jobs' ? ['-p', 'ic-jobs'] : ['--workspace']),
        '--all-features', '--allow-dirty', '--locked']);
    if (failure === 'package') {
        assert.equal(commands.length, 1);
        return;
    }
    assert.deepEqual(commands[1], ['metadata', '--no-deps', '--locked', '--format-version', '1']);
    const checks = commands.filter(args => args[0] === 'check');
    assert.equal(checks.length, scope === 'jobs' || failure ? 1 : 2);
    assert.ok(checks[0].includes('--locked'));
    if (scope === 'jobs') {
        assert.ok(checks[0].includes('target/package/ic-jobs-0.39.3/Cargo.toml'));
        assert.ok(!checks[0].includes('--config'));
    } else if (!failure) {
        assert.ok(checks[0].includes('target/package/ic-timers-0.46.3/Cargo.toml'));
        assert.ok(checks[1].includes('--config'));
    }
}
for (const scope of ['workspace', 'timers', 'jobs']) {
    for (const offline of [null, 'true', 'false']) check(scope, offline);
    check(scope, null, 'package');
    check(scope, null, 'check');
}
check('unknown');
console.log('Archive scope: Jobs-only, Timers reverse consumer, full CI, Cargo network settings and failure propagation pass.');
