# Saffron UTC observation — 2026-11-15

The maintainer explicitly selected Saffron and UTC. This replaces the
[Croner/time-zone implementation](calendar-library.md). Published Saffron source
was inspected, not copied. No custom cron parser or calendar arithmetic remains.

## Selection and contract

- Saffron =0.1.0, defaults disabled, std enabled for supplier error integration.
- Chrono 0.4.45 in both locks, defaults disabled, alloc enabled; no host clock.
- Normal product graph: Jobs → Saffron → Chrono/nom; Jobs → Chrono;
  Chrono → num-traits; nom → memchr. No Croner, Jiff or IANA data remains.
- All these dependencies and the public calendar API require the off-by-default
  `calendar` feature. Disabled Jobs has no normal dependencies.

Schedule::parse accepts at most 256 bytes and exactly five fields, all UTC with
minute precision. No zone argument or seconds dialect remains. Saffron handles
names, lists/ranges/steps, last-day/nth-weekday rules, date arithmetic and search.
The adapter only admits inputs and converts unsigned Unix nanosecond bounds.
Reference: [Saffron API](https://docs.rs/saffron/0.1.0/saffron/struct.Cron.html).

## Local checks

Four native adapter tests pass, with multiple fixtures covering leap-century
behavior, monthly 31st, last day, second Monday, weekday ranges, impossible dates,
invalid/oversized layouts, fractional/exact window edges and timestamps above
signed-i64 nanoseconds up to the final minute within u64::MAX.

Eight standalone Wasm outcomes pass through Node 24.21.0. The module has no host
imports. Product/qualification Clippy and product Rustdoc pass with warnings
denied; shared formatting passes. Disabled-feature wasm32 builds also pass and
cargo tree confirms no normal dependencies without calendar.

Commands:

- cargo test -p ic-jobs --features calendar --lib --locked --offline
- make lint PACKAGE=ic-jobs; make doc PACKAGE=ic-jobs
- make check-calendar-wasm
- cargo check -p ic-jobs --no-default-features --target wasm32-unknown-unknown --locked --offline
- cargo clippy --manifest-path qualification/Cargo.toml --all-targets --locked --offline -- -D warnings
- make fmt-check

## Artifact measurement

Compiler: rustc 1.100.0 (0ef1bc386 2026-11-09), standard Cargo release profile.
Artifact: qualification/target/wasm32-unknown-unknown/release/calendar_probe.wasm.

| Measurement | Saffron UTC | Previous Croner + zones |
| --- | ---: | ---: |
| Full uncompressed module, bytes | 106,105 | 675,747 |
| Code section including id/length, bytes | 77,016 | 284,436 |
| Initial linear memory, bytes | 1,114,112 | 1,376,256 |
| Linear memory after fixtures, bytes | 1,114,112 | 1,441,792 |

The full probe shrank 569,642 bytes (84.3%). This compares whole implementations
and changed UTC fixtures, including removal of time-zone support; it is not an
isolated parser benchmark or the size of a finished canister. Linear memory is
allocated Wasm memory, not live heap. No instruction-speed claim is made.

First-party physical lines including comments/blanks: product adapter 99,
native tests 86, qualification Rust/JS 90; total 275. No implementation was moved
into other adapters. This is a local inventory, not a dependency-inclusive or
whole-repository LOC claim. Supplier code is included in the linked sizes above.

SHA-256:

- Wasm: 737b634bff75b1a98b413ec723f8930de5e7204086a8b048358214bc8a80e61a
- Product lock: e7ca1ee97042c7c1a05498ab5b81777cfd9168cb2b56361722b48caa5c0adf71
- Qualification lock: aa0552c7afa984a411c50a068ef38ac94afcb5e55d350e906c495ca3c3fc0e79

## Remaining scope

Search is synchronous and the upper horizon filters results rather than limiting
supplier work. Qualify costs for supported rules before integrating IC execution.
No custom resumable engine is planned. Named zones, local-time/DST policies and
seconds-enabled cron are outside scope. Timer execution, persistence, upgrades,
missed-run handling and failure custody remain unimplemented/unqualified.
These are local observations, not remote or installed IC proof.

## Later supplier API review — 2026-11-15

Execution, checkpoint and catch-up qualification added after this initial calendar
observation is recorded in [Jobs progress evidence](jobs-progress.md). The scope
statement above describes the earlier calendar-only slice.

The pinned 0.1.0 source exposes `next_from`, `next_after`, forward `iter_from` and
`iter_after`, plus membership testing through `contains`. `CronTimesIter`
implements `Iterator` and `FusedIterator`, with no reverse iteration or direct
latest-occurrence-before operation. Therefore latest-due coalescing cannot be
delegated as a single supported supplier operation. Forward enumeration could
grow with downtime; no custom backward/calendar search was introduced to fill
this gap. Coalescing remains a separate open design item rather than silently
being approximated by the oldest-first bounded catch-up policy.

This is a local observation of the pinned Cargo registry source, not a claim
about other supplier versions. Inspected `saffron-0.1.0/src/lib.rs` SHA-256:
9c869ccfb3e29dcd02b69fb2c2c0a87fea97244ce65d21fe78cd40fb9d66f8c1.
