// Qualify local group routing and real metadata transactions without publishing.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { cpSync, mkdtempSync, readFileSync, mkdirSync } from 'node:fs';
import { resolve } from 'node:path';
const root = process.cwd();
const env = { ...process.env, PATH: `${root}/.tools/host/bin:${root}/.tools/rust/common/bin:${process.env.PATH}` };
function run(command, args, cwd = root, extra = {}, succeeds = true) {
  const result = spawnSync(command, args, { cwd, env: { ...env, ...extra }, encoding: 'utf8' });
  if (result.error) throw result.error;
  assert.equal(result.status === 0, succeeds, `${command} ${args.join(' ')}\n${result.stdout}\n${result.stderr}`);
  return result.stdout.trim();
}
for (const target of ['release-patch', 'release-minor', 'release-major', 'publish', 'publish-check']) {
  run('make', [target], root, { RELEASE_GROUP: '', RELEASE_GROUPS: '' }, false);
  run('make', [target, 'RELEASE_GROUP=unknown'], root, {}, false);
}
for (const kind of ['patch', 'minor', 'major']) {
  assert.equal(run('perl', ['vendor/toko-tooling/scripts/ci/validate-release-inputs.pl', 'next-version', '0.45.8', kind]), { patch: '0.45.9', minor: '0.46.0', major: '1.0.0' }[kind]);
}
const editor = run('bash', ['vendor/toko-tooling/scripts/dev/install-rust-tools.sh', '--consumer', root, '--package', 'ic-cargo-metadata', '--version', '0.1.0', '--bin', 'ic-cargo-metadata', '--profile', 'release', '--check']);
mkdirSync('qualification/target/release-hooks', { recursive: true });
const shared = resolve('vendor/toko-tooling/scripts/ci/cargo-release-group.sh');
for (const group of ['timers', 'jobs']) {
  const fixture = mkdtempSync(`${root}/qualification/target/release-hooks/${group}-`);
  // These are transaction fixtures, not additional working checkouts.
  for (const path of ['Cargo.toml', 'Cargo.lock', 'crates', 'ci', 'releases', 'LICENSE.md']) cpSync(`${root}/${path}`, `${fixture}/${path}`, { recursive: true });
  mkdirSync(`${fixture}/qualification`);
  for (const path of ['Cargo.toml', 'Cargo.lock', 'once-probe', 'once-driver', 'executor-probe', 'executor-driver', 'calendar-probe', 'metrics-probe']) cpSync(`${root}/qualification/${path}`, `${fixture}/qualification/${path}`, { recursive: true });
  const previous = run('bash', [shared, 'version', group], fixture);
  const ledger = readFileSync(`${fixture}/releases/${group}/CHANGELOG.md`, 'utf8');
  const pending = ledger.match(/^## \[(\d+\.\d+\.\d+)\] - Unreleased$/m)?.[1];
  const version = pending ?? previous.replace(/\d+$/, value => String(Number(value) + 1));
  const other = group === 'timers' ? 'jobs' : 'timers';
  const preserved = [`crates/ic-${other}/Cargo.toml`, `releases/${other}/CHANGELOG.md`];
  const original = preserved.map(path => readFileSync(`${fixture}/${path}`, 'utf8'));
  const context = { RELEASE_PREVIOUS: previous, RELEASE_VERSION: version, RELEASE_DATE: '2026-11-15', CARGO_RELEASE_METADATA_EDITOR: editor, CARGO_RELEASE_METADATA_MAX_BYTES: '8388608' };
  run('bash', [shared, 'prepare', group], fixture, context);
  run('bash', [shared, 'check', group], fixture, context);
  preserved.forEach((path, i) => assert.equal(readFileSync(`${fixture}/${path}`, 'utf8'), original[i]));
  const manifest = readFileSync(`${fixture}/Cargo.toml`, 'utf8');
  run('bash', [shared, 'prepare', group], fixture, { ...context, RELEASE_PREVIOUS: version, RELEASE_VERSION: previous }, false);
  assert.equal(readFileSync(`${fixture}/Cargo.toml`, 'utf8'), manifest);
  console.log(`${group}: real group transaction, qualification lock synchronization, unselected preservation and downgrade refusal pass`);
}
console.log('Release hooks: invalid public selections refuse; standard increments and actual group metadata transactions pass. No delivery effects invoked.');
