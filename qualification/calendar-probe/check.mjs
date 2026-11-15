// Standalone Wasm engine smoke checks; no IC simulator, npm packages or network.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';

const path = process.argv[2];
assert(path, 'usage: node qualification/calendar-probe/check.mjs PATH.wasm');
const bytes = readFileSync(path);
const module = await WebAssembly.compile(bytes);
assert.deepEqual(WebAssembly.Module.imports(module), [], 'calendar artifact must not require host clock/filesystem imports');
const { exports } = await WebAssembly.instantiate(module);
const initialMemory = exports.memory.buffer.byteLength;
const ns = text => BigInt(Date.parse(text)) * 1_000_000n;
const cases = [
  [0, '2026-03-29T00:00:00Z', '2026-03-29T09:00:00Z', '2026-03-29T09:00:00Z'],
  [1, '2026-02-01T00:00:00Z', '2026-02-28T09:00:00Z', '2026-02-28T09:00:00Z'],
  [2, '2026-02-01T00:00:00Z', '2026-02-09T09:00:00Z', '2026-02-09T09:00:00Z'],
  [3, '2099-03-01T00:00:00Z', '2104-02-29T09:00:00Z', '2104-02-29T09:00:00Z'],
  [4, '2026-01-01T00:00:00Z', '2100-01-01T00:00:00Z', 0n],
  [5, '2026-01-01T00:00:59.999Z', '2026-01-01T00:01:00Z', '2026-01-01T00:01:00Z'],
  [5, '2026-01-01T00:01:00Z', '2026-01-01T00:01:59.999Z', 0n],
];
for (const [scenario, after, through, expected] of cases) {
  assert.equal(exports.calendar_next(scenario, ns(after), ns(through)), typeof expected === 'bigint' ? expected : ns(expected), String(expected));
}
// JS exposes Wasm i64 as signed; unsigned nanoseconds above i64::MAX still work.
const max = (1n << 64n) - 1n;
const lastMinute = max / 60_000_000_000n * 60_000_000_000n;
assert.equal(BigInt.asUintN(64, exports.calendar_next(5, lastMinute - 1n, max)), lastMinute);

// The engine has already validated the module. Walk standard sections to report
// code-section bytes, including its id and encoded length, separately from data.
let offset = 8;
let codeBytes = 0;
while (offset < bytes.length) {
  const start = offset;
  const id = bytes[offset++];
  let length = 0;
  let shift = 0;
  let byte;
  do {
    byte = bytes[offset++];
    length += (byte & 127) * 2 ** shift;
    shift += 7;
  } while (byte & 128);
  offset += length;
  if (id === 10) codeBytes += offset - start;
}
assert.equal(offset, bytes.length);
assert(codeBytes > 0);
console.log(JSON.stringify({
  node: process.version,
  cases: cases.length + 1,
  sha256: createHash('sha256').update(bytes).digest('hex'),
  wasm_bytes: bytes.length,
  code_section_bytes: codeBytes,
  initial_linear_memory_bytes: initialMemory,
  final_linear_memory_bytes: exports.memory.buffer.byteLength,
}, null, 2));
