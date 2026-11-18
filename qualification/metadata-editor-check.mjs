// Exercise real installation/admission in isolation, including a stale selection.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';

const root = process.cwd();
fs.mkdirSync('qualification/target', { recursive: true });
const fixture = fs.mkdtempSync(path.resolve('qualification/target/metadata-editor-'));
fs.cpSync('vendor', path.join(fixture, 'vendor'), { recursive: true });
fs.mkdirSync(path.join(fixture, 'ci'));
const script = path.join(fixture, 'ci/metadata-editor.sh');
const original = fs.readFileSync('ci/metadata-editor.sh', 'utf8');
fs.writeFileSync(script, original);
function run(args, succeeds = true) {
    const result = spawnSync('bash', ['ci/metadata-editor.sh', ...args], {
        cwd: fixture, encoding: 'utf8',
        env: { ...process.env, PATH: `${root}/.tools/host/bin:${process.env.PATH}` },
    });
    assert.ifError(result.error);
    assert.ok(Number.isInteger(result.status));
    assert.equal(result.status === 0, succeeds, result.stdout + result.stderr);
    return result.stdout.trim();
}
run(['--check'], false); // No prepared selection must not trigger installation.
const installed = run([]);
assert.ok(path.isAbsolute(installed));
assert.equal(run(['--check']), installed);
run(['--version', '0.0.0'], false); // Callers cannot override the owner.
// Intentionally request an unprepared version, independently of the current pin.
const mismatch = original.replace(/--version \S+/, '--version 0.0.0');
assert.notEqual(mismatch, original);
fs.writeFileSync(script, mismatch);
run(['--check'], false);
fs.writeFileSync(script, original);
assert.equal(run(['--check']), installed);
console.log('Metadata editor: install/check agree; missing, mismatched and overridden selections refuse; restoring the owner recovers.');
