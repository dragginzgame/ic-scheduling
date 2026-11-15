// Materialize pinned consumer sources under our ignored target directory; never edit Entropy.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { execFileSync } from 'node:child_process';

const revision = '6c400b2c0259323ad1464fc692a4c4c378a2b101';
const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, '../..');
const source = path.resolve(process.argv[2] ?? path.join(root, '../ic-entropy'));
function git(...args) { return execFileSync('git', ['--no-optional-locks', '-C', source, ...args]); }
if (git('rev-parse', 'HEAD').toString().trim() !== revision
    || git('status', '--porcelain').length !== 0) {
    throw new Error('Entropy source must be the clean pinned revision; review and repin before proceeding.');
}
const parent = path.join(root, 'qualification/target/entropy-integration');
fs.mkdirSync(parent, { recursive: true });
const staged = fs.mkdtempSync(path.join(parent, 'run-'));
const names = git('ls-tree', '-r', '--name-only', '-z', revision).toString().split('\0').filter(Boolean);
for (const name of names) {
    if (!['Cargo.toml', 'Cargo.lock', 'LICENSE.md'].includes(name)
        && !name.startsWith('crates/') && !name.startsWith('qualification/')) continue;
    const destination = path.join(staged, name);
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, git('show', `${revision}:${name}`));
}
const patch = path.join(here, 'timers.patch');
execFileSync('git', ['apply', '--check', patch], { cwd: staged });
execFileSync('git', ['apply', patch], { cwd: staged });
// Registry publication is disabled; resolve the draft dependency to this exact checkout.
const override = `\n[patch.crates-io]\nic-timers = { path = ${JSON.stringify(path.join(root, 'crates/ic-timers'))} }\n`;
for (const name of ['Cargo.toml', 'qualification/Cargo.toml']) {
    fs.appendFileSync(path.join(staged, name), override);
}
fs.writeFileSync(path.join(parent, 'latest-path'), staged + '\n');
console.log(staged);
