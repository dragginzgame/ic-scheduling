// Bound CI commands and sanitize diagnostics before printing or retaining them.
import fs from 'node:fs';
import path from 'node:path';
import { spawn } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const limit = 64 * 1024;
function sanitize(text, environment) {
    let safe = text.replace(/\x1b\[[0-?]*[ -/]*[@-~]/g, '');
    for (const [name, value] of Object.entries(environment)) {
        if (/token|password|secret|credential|(?:private|access|api).?key/i.test(name) && value?.length >= 4) {
            safe = safe.split(value).join('[REDACTED]');
        }
    }
    return safe
        .replace(/-----BEGIN [^-]*PRIVATE KEY-----[\s\S]*/g, '[PRIVATE KEY OMITTED]')
        .replace(/(?:gh[pousr]_[A-Za-z0-9_]+|github_pat_[A-Za-z0-9_]+)/g, '[REDACTED]')
        .replace(/[A-Za-z0-9+/=]{80,}/g, '[LONG ENCODED VALUE OMITTED]')
        .replace(/https?:\/\/[^\s/]*@/g, 'https://[REDACTED]@')
        .replace(/([?&](?:token|key|signature|credential|password)=)[^\s&]+/gi, '$1[REDACTED]')
        .split('\n').map(line => /authorization|password|secret|credential|private.?key|payload|\bBearer\s+\S+|(?:^|\W)(?:token|(?:access|api).?key)["']?\s*[:=]/i.test(line)
            ? '[SENSITIVE LINE OMITTED]' : line).join('\n');
}

// Only explicitly reported Cargo build attempts, never whole stores or symlinks.
function attemptLog(directory, store) {
    if (fs.lstatSync(store).isSymbolicLink()) throw new Error('store symlink refused');
    const base = fs.realpathSync(store);
    const relative = path.relative(base, directory);
    if (!/^rust-[a-f0-9]{64}\/set\.[A-Za-z0-9]+$/.test(relative)) {
        throw new Error('attempt outside the selected Rust store boundary');
    }
    let current = base;
    for (const component of [...relative.split('/'), 'install.log']) {
        current = path.join(current, component);
        if (fs.lstatSync(current).isSymbolicLink()) throw new Error('attempt symlink refused');
    }
    if (!fs.statSync(current).isFile()) throw new Error('attempt log is not a regular file');
    const handle = fs.openSync(current, 'r');
    try {
        const size = fs.fstatSync(handle).size;
        const bytes = Buffer.alloc(Math.min(size, limit));
        fs.readSync(handle, bytes, 0, bytes.length, Math.max(0, size - limit));
        return `${size > limit ? '[TRUNCATED: final 64 KiB]\n' : ''}${bytes.toString()}`;
    } finally { fs.closeSync(handle); }
}

export async function runPhase(phase, command, args, environment = process.env) {
    if (!/^[a-z0-9-]+$/.test(phase) || !command) throw new Error('expected phase and command');
    const seconds = Number(environment.CI_PHASE_TIMEOUT_SECONDS ?? 0);
    if (!Number.isInteger(seconds) || seconds < 0 || seconds > 86400) throw new Error('invalid CI phase timeout');
    let tail = Buffer.alloc(0), total = 0, launchError = '';
    const grouped = seconds > 0 && process.platform !== 'win32';
    const child = spawn(command, args, { env: environment, detached: grouped, stdio: ['ignore', 'pipe', 'pipe'] });
    function capture(chunk) {
        total += chunk.length;
        tail = Buffer.concat([tail, chunk]).subarray(-limit);
    }
    child.stdout.on('data', capture);
    child.stderr.on('data', capture);
    child.on('error', error => { launchError = error.message; });
    let stopped = 0, termination;
    function send(signal) {
        if (!child.pid) return;
        try {
            if (grouped) process.kill(-child.pid, signal);
            else child.kill(signal);
        } catch (error) {
            if (error.code !== 'ESRCH') capture(Buffer.from(`\nTermination failed: ${error.message}\n`));
        }
    }
    function stop(status, reason, signal = 'SIGTERM') {
        if (stopped) return;
        stopped = status;
        capture(Buffer.from(`\n${reason}; terminating command group, then SIGKILL after 2 seconds.\n`));
        send(signal);
        // Keep this grace even if the leader exits first: descendants may ignore TERM.
        termination = new Promise(resolve => setTimeout(() => { send('SIGKILL'); resolve(); }, 2000));
    }
    const interrupt = () => stop(130, 'CI command interrupted', 'SIGINT');
    const terminate = () => stop(143, 'CI command terminated');
    if (seconds) {
        process.on('SIGINT', interrupt);
        process.on('SIGTERM', terminate);
    }
    const deadline = seconds ? setTimeout(() => stop(124, `CI phase timeout after ${seconds} seconds`), seconds * 1000) : undefined;
    const [code, signal] = await new Promise(resolve => child.on('close', (...result) => resolve(result)));
    clearTimeout(deadline);
    await termination;
    process.removeListener('SIGINT', interrupt);
    process.removeListener('SIGTERM', terminate);
    const status = stopped || (code ?? (signal === 'SIGINT' ? 130 : signal === 'SIGTERM' ? 143 : 1));
    const heading = `Setup phase=${phase} tool=${path.basename(command)} status=${status}${signal ? ` signal=${signal}` : ''}`;
    const raw = `${total > limit ? '[TRUNCATED: final 64 KiB]\n' : ''}${tail.toString()}${launchError}`;
    const safe = sanitize(raw, environment);
    console.log(`${heading}\n${safe || '[No diagnostics emitted]'}`);
    if (status === 0) return 0;
    const output = path.resolve('target/ci-setup');
    try {
        if (fs.existsSync('target') && fs.lstatSync('target').isSymbolicLink()) throw new Error('target symlink refused');
        fs.mkdirSync(output, { recursive: true });
        if (fs.lstatSync(output).isSymbolicLink()) throw new Error('evidence symlink refused');
        fs.writeFileSync(path.join(output, `${phase}.log`), `${heading}\n${safe || '[No diagnostics emitted]'}\n`, { flag: 'wx' });
        const store = environment.TOKO_TOOLING_STORE
            ?? path.join(environment.XDG_DATA_HOME ?? path.join(environment.HOME, '.local/share'), 'toko-tooling/tools');
        const attempts = [...raw.matchAll(/^Cargo tool attempt retained: (.+)$/gm)].slice(0, 4);
        for (const [index, match] of attempts.entries()) {
            try {
                const log = sanitize(attemptLog(match[1], store), environment);
                fs.writeFileSync(path.join(output, `${phase}-attempt-${index}.log`), log, { flag: 'wx' });
            } catch (error) {
                const notice = `Attempt evidence missing/refused: ${sanitize(error.message, environment).slice(0, 2048)}`;
                console.error(notice);
                fs.appendFileSync(path.join(output, `${phase}.log`), `${notice}\n`);
            }
        }
        console.error(`Sanitized setup evidence: ${output}/${phase}.log; at most four 64 KiB attempt logs. Raw output is not retained.`);
    } catch (error) {
        console.error(`Setup evidence collection failed: ${sanitize(error.message, environment)}; original status=${status}`);
    }
    return status;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
    process.exitCode = await runPhase(process.argv[2], process.argv[3], process.argv.slice(4));
}
