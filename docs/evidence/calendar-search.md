# Superseded custom search observation — 2026-11-15

Historical baseline only: this custom implementation was replaced by Croner,
then by Saffron and UTC. See [the current observation](calendar-saffron.md).
The old tests and measurements below do not qualify the replacement.

Local native and standalone Wasm checks of the first Jobs occurrence-search
implementation. No replica, simulator, remote canister or deployment was used.

## Selection and purpose

- `jiff 0.2.45`, default features disabled, `std` enabled: TZif decoding,
  civil/UTC conversion and exact transition iteration.
- `jiff-tzdb =0.1.10`: bundled IANA 2026f bytes and version identity. The exact
  constraint makes database-rule updates explicit.
- Normal product edges: Jobs → Jiff → Jiff Core 0.1.7; Jobs → Jiff TZDB.
- First-party date matching and cron rules remain Jobs-owned. No supplier code
  was copied into the repository. Public API references:
  [Jiff TimeZone](https://docs.rs/jiff/0.2.45/jiff/tz/struct.TimeZone.html),
  [bundled database](https://docs.rs/jiff-tzdb/0.1.10/jiff_tzdb/).

Exact transitions permit UTC-ordered search without guessing DST widths or
buffering/sorting a lookahead window of local occurrences. Fixed offsets cannot
preserve local recurring intent. A system database would make Wasm behavior and
rule versions depend on the host. The bundled selection is therefore deliberate;
its final IC instruction and retained-job cost are still unqualified.

## Commands and results

- `cargo test -p ic-jobs --lib --locked --offline`: 22 tests passed.
- `make lint PACKAGE=ic-jobs`: passed with warnings denied.
- `make doc PACKAGE=ic-jobs`: passed with warnings denied.
- `cargo check -p ic-jobs --target wasm32-unknown-unknown --locked --offline`: passed.
- Qualification Clippy passed for `calendar-probe`, all targets, locked/offline.
- `make check-calendar-wasm`: builds the release-profile raw Wasm artifact and
  runs eight expected-result checks using Node 24.21.0. All passed; the module
  has no host imports. This is Wasm engine execution, not IC execution.

Compiler: `rustc 1.100.0 (0ef1bc386 2026-11-09)` on local Linux x86_64.
Standard Cargo release profile; no optimizer or deployment recipe was selected.
Measured artifact: `qualification/target/wasm32-unknown-unknown/release/calendar_probe.wasm`.

| Measurement | Bytes |
| --- | ---: |
| Complete uncompressed module | 572,697 |
| Code section, including section id and encoded length | 197,944 |
| Initial linear memory | 1,376,256 |
| Linear memory after the eight scenarios | 1,441,792 |

The driver first validates/executes the module, then walks its standard binary
section lengths to measure the code section. Linear memory is allocated Wasm
memory, not live heap or retained per-zone size. These absolute observations
include the fixture, runtime and data; no performance improvement is claimed.

SHA-256 identities:

- Wasm: `cacf0bcc37b3c4d7a88b5524e6194d2b7dea3ed18391213d49f1277c9ad61450`
- Product lock: `6892a57171de949a78fafaa025dd5e386351eb540baab9a334c0a1223807b3f0`
- Qualification lock: `13a886b002c301414b1aa01d9627917628640e5e258580bacc5f33f05bfca975`

## Remaining gates

The unit suite covers calendar validation, explicit dialects, gap coalescing,
fold ordering and selection, 30-minute changes, historical second offsets, a
skipped local date, budget resumptions, sparse/impossible rules, subsecond bounds
and timestamp exhaustion. A direct UTC enumeration oracle independently checks
transition-cursor results in selected small windows.

Still required: installed IC instruction measurements, retained-memory limits
across many jobs/zones, data-version migration, timer failure custody and job
persistence/upgrade behavior. Snapshot restore remains unqualified. A successful
calendar preview never grants authority to replay an uncertain business effect.
