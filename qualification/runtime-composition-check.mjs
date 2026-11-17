// Verify real consumer composition across the supported Runtime patch range.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';

const root = process.cwd();
const parent = path.join(root, 'qualification/target/runtime-composition');
fs.mkdirSync(parent, { recursive: true });
const run = fs.mkdtempSync(path.join(parent, 'run-'));
function cargo(args, cwd, succeeds = true) {
    const result = spawnSync('cargo', args, { cwd, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 });
    if (result.error) throw result.error;
    assert.equal(result.status === 0, succeeds, `${args.join(' ')}\n${result.stdout}\n${result.stderr}`);
    return result;
}
for (const version of ['0.23.14', '0.23.15', '0.23.16', '0.22.5']) {
    const supported = version !== '0.22.5';
    const fixture = path.join(run, version);
    fs.mkdirSync(path.join(fixture, 'src'), { recursive: true });
    fs.writeFileSync(path.join(fixture, 'Cargo.toml'), `[workspace]
[package]
name = "runtime-composition"
version = "0.0.0"
edition = "2024"
publish = false
[dependencies]
ic-timers = { path = ${JSON.stringify(path.join(root, 'crates/ic-timers'))}, features = ["metrics"] }
ic-jobs = { path = ${JSON.stringify(path.join(root, 'crates/ic-jobs'))}, features = ["execution"] }
ic-platform-runtime = { version = "=${version}", default-features = false, features = ${supported ? '["call-ic0", "system-raw"]' : '["runtime"]'} }
`);
    fs.writeFileSync(path.join(fixture, 'src/lib.rs'), `pub use ic_jobs::Policy;\npub use ic_timers::Observation;\npub use ic_platform_runtime::call::${supported ? 'Wait' : 'WaitPolicy as Wait'};\n`);
    const result = cargo(['metadata', '--format-version', '1', '--offline'], fixture, supported);
    fs.writeFileSync(path.join(fixture, 'metadata.stdout'), result.stdout);
    fs.writeFileSync(path.join(fixture, 'metadata.stderr'), result.stderr);
    if (!supported) {
        assert.match(result.stderr, /ic-platform-runtime-guard/);
        assert.match(result.stderr, /links/);
        console.log('Runtime 0.22.5: incompatible executor guard still refuses.');
        continue;
    }
    const metadata = JSON.parse(result.stdout);
    for (const name of ['ic-platform-runtime', 'ic-platform-runtime-guard']) {
        const packages = metadata.packages.filter(item => item.name === name);
        assert.equal(packages.length, 1, `one ${name}`);
        assert.equal(packages[0].version, version);
    }
    cargo(['check', '--target', 'wasm32-unknown-unknown', '--locked', '--offline', '--target-dir', path.join(parent, 'build')], fixture);
    console.log(`Runtime ${version}: Timers/Jobs consumer resolves and compiles for Wasm with one matching guard.`);
}
console.log(`Runtime composition evidence: ${run}`);
