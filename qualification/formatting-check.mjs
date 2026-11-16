// Supply Scheduling's current inputs to Toko's consumer hook qualification.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';

const root = process.cwd();
mkdirSync('qualification/target', { recursive: true });
const scratch = mkdtempSync(resolve('qualification/target/formatting-input-'));
const manifest = readFileSync('Cargo.toml', 'utf8');
const unsorted = manifest.replace(/^(ic-metrics = [^\n]+)\n(ic-platform-runtime = [^\n]+)/m, '$2\n$1');
assert.notEqual(unsorted, manifest, 'expected real dependency declarations to perturb');
const input = `${scratch}/unsorted.toml`;
writeFileSync(input, unsorted);
const managed = readFileSync('.toko-tooling.snapshot', 'utf8').split('\n')
  .filter(line => line.startsWith('file\t')).map(line => line.split('\t')[3]);
const result = spawnSync('bash', [
  'vendor/toko-tooling/scripts/ci/check-formatting-hooks.sh', root,
  'crates/ic-jobs/src/lib.rs', 'Cargo.toml', input, '--rust-sources',
  'Cargo.lock', 'qualification/Cargo.lock', 'ci/cargo-release-groups.json',
  '.toko-tooling.snapshot', ...managed,
], { stdio: 'inherit' });
if (result.error) throw result.error;
if (result.status === 0) rmSync(scratch, { recursive: true });
else console.error(`Formatting perturbation retained at: ${scratch}`);
process.exit(result.status ?? 1);
