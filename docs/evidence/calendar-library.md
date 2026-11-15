# Superseded Croner observation — 2026-11-15

Historical baseline: Croner has been replaced by Saffron and UTC. See
[the current observation](calendar-saffron.md). The results below apply to the
removed implementation.

Local observation of the Croner replacement, not installed IC or deployment
proof. This supersedes the implementation described in
[the historical custom search baseline](calendar-search.md).

## Selection and ownership

Croner is pinned to 4.0.3, default features disabled, with its Jiff backend.
Jiff is selected at 0.2.45 with only std explicitly enabled; jiff-tzdb is pinned
to 0.1.10, IANA 2026f. The published Croner and Cronexpr 1.7.1 sources were
inspected, not transplanted. Cronexpr's five-field interface did not satisfy our
explicit seconds dialect. References:
[Croner](https://docs.rs/croner/4.0.3/croner/),
[Cronexpr](https://docs.rs/cronexpr/1.7.1/cronexpr/).

Croner owns cron grammar, calendar matching, next-occurrence calculation and
recurrence DST semantics. Jiff owns civil types and zone conversion. Jobs owns
only expression admission, bundled-zone selection and the timer timestamp
adapter. There is no custom search cursor, calendar arithmetic or cron parser.
Supplier dependencies include derive_builder and strum and their build macros;
this is one recurrence engine, not a claim of one package in the dependency graph.

Fixed-time recurrence uses the first valid same-date instant after a gap and the
earlier fold instant. Interval rules skip gaps and can visit both fold halves.
The previous configurable policies are removed rather than approximated.

## Checks

- Six native adapter tests passed, covering layout/size/zone admission, seconds,
  last day, nth weekday, monthly 31st, leap-century exception, historical second
  offsets, fractional timestamp bounds, half-hour folds, gaps and one-time
  ambiguity. These are repository contracts, not a copy of upstream's test suite.
- Product and qualification Clippy passed with warnings denied, locked/offline.
- Product Rustdoc passed with warnings denied. Shared formatting and dependency
  catalog checks passed in both workspaces.
- Eight standalone Wasm outcomes passed through Node 24.21.0, including a
  deliberately expected search failure for Apia's whole skipped date. The module
  has no host imports. No IC instruction measurements were performed.

Commands: cargo test -p ic-jobs --features calendar --lib --locked --offline;
make lint PACKAGE=ic-jobs; make doc PACKAGE=ic-jobs; make check-calendar-wasm;
cargo clippy --manifest-path qualification/Cargo.toml -p calendar-probe
--all-targets --locked --offline -- -D warnings; make fmt-check; make deps-list.

Compiler: rustc 1.100.0 (0ef1bc386 2026-11-09), standard Cargo release profile.
Artifact: qualification/target/wasm32-unknown-unknown/release/calendar_probe.wasm.

| Measurement | Bytes |
| --- | ---: |
| Complete uncompressed Wasm module | 675,747 |
| Code section including id and length | 284,436 |
| Initial linear memory | 1,376,256 |
| Linear memory after fixtures | 1,441,792 |

The historical custom artifact was 572,697 bytes. The replacement is larger;
no binary reduction or runtime improvement is claimed. The fixtures now exercise
different supplier policies and an explicit failure, so this is not equivalent
functional acceptance. Linear memory is allocated memory, not retained heap.

Current first-party physical lines, including comments/blanks: product adapter
and zone code 261; native tests 178; qualification Rust and JS 94; total 533.
No code was moved into a first-party adapter elsewhere. This is an inventory,
not a whole-repository or dependency-inclusive LOC reduction claim. Supplier
implementation remains part of the linked artifact measurements above.

SHA-256:

- Wasm: f42ae5bf4e179554daa59d5531e776b71d900da2ad9bbb5820a44f24cee7491b
- Product lock: d9717935eb35d9f66bf7cadb5bb40879c490c91405031f7c95bb2774024a9030
- Qualification lock: 578117d9bf075dc8442babcb42bc185e770fa99c06143b3cc01f1ca0049f3462

## Known limitations and remaining qualification

The supplier's synchronous search has no caller work budget/resumable cursor.
Its source iteration ceiling is 31,622,400 and its fixed-gap scan ceiling is
7,200 seconds. Those are not IC instruction limits. A short caller horizon does
not limit engine search work. Sparse/impossible schedules remain unqualified.
The whole-date Apia fixture returns TimeSearchLimitExceeded, propagated as an
engine error rather than None. No automatic retry follows that error.

The maintainer accepts rare historical gaps such as Apia as unsupported; they
are not release blockers. Preserve the explicit error without implementing a
workaround. Before execution integration, measure worst-case IC work for the
supported schedules. A custom resumable search engine is not required.
Do not recreate a second recurrence engine to hide these limitations. Timer
custody, capacity, persistence, upgrade and time-zone data migration gates also
remain open. This change consolidates calendar ownership; it does not establish
that the scheduler is ready to execute jobs.

## Optional calendar follow-up — 2026-11-15

The maintainer accepts a simpler supplier contract and explicitly excludes
hand-written calendar workarounds. Saffron 0.1.0 was inspected as a simpler
alternative: its next-occurrence methods accept DateTime<Utc>, not named local
time zones ([API](https://docs.rs/saffron/0.1.0/saffron/struct.Cron.html)). Using
it for local recurring intent would require additional conversion/search policy.
The cron 0.17.0 API supports generic Chrono time zones, but that is not evidence
of a smaller linked artifact; no comparative Wasm benchmark was performed.
Retain the already measured Croner integration, without expanding its semantics.

Added off-by-default `calendar` to ic-jobs. It gates every calendar module/export
and all three dependencies. Qualification explicitly opts in. Both
`cargo tree -p ic-jobs --no-default-features --edges normal --locked --offline`
and `cargo tree -p ic-timers --edges normal --locked --offline` show only the
respective product package, with no normal dependencies. Disabled Jobs builds
also pass the wasm32-unknown-unknown check. This establishes absence of calendar
dependencies in those configurations, not the size of a finished consumer.

The line inventory and Wasm hash above predate this feature-gating follow-up;
they remain a dated measurement, not the current artifact identity.

Feature-enabled recheck: six native tests and eight Wasm outcomes passed; enabled
and disabled Clippy, enabled Rustdoc and shared formatting passed. The new Wasm
hash is 49c9b6da0307f5b503b1619fd3b0ccc9ede6507aaec746984bbdd945e18a8a4c.
Full module/code-section sizes remain 675,747/284,436 bytes; observed linear
memory is unchanged. Feature gating adds no size to this enabled probe.
