# Scheduling: Shared Tooling adoption and standards audit

Date: 2026-11-05. Scope: this Scheduling workspace, its eight packages and
repository-owned Make, CI, formatting, release and qualification callers.
Starting source: `0c3f310a17a1a0ea03f9c6e030dbb9559a448332`, initially clean.
Changes are local and uncommitted. Sibling sources remain read-only.

The reviewed Shared Tooling revision is
`07dce12cbe4a798fd4a6d263095518678c4f6b45` (0.30.0), verified against upstream
`main` at observation. Adoption uses the canonical exporter and independently
verified snapshot, not producer working-tree edits. The local overlay is
`AGENTS.md`. Methods: `audits/code-hygiene.md`, `audits/flow-convergence-and-duplication.md`
and `audits/ci-release-integrity.md`, under `audits/README.md`.
All paths below are repository-relative.

## Findings and corrections

| Finding | Severity | Correction and owner |
| --- | --- | --- |
| Private fixtures and qualification tools were counted as library/application source. | LOW | All six private packages now declare `package.metadata.shared-tooling.loc`; test/example path overrides preserve their actual use. Shared's classifier remains the sole counting implementation. |
| Three crate-wide Cargo lint allowances concealed avoidable warnings or had become obsolete. | LOW | Remove all three. Add 39 public timer `# Errors` sections; remove unused Markdown/module-name allowances. Document native harness failures and the artifact-verifier panic boundary too. |
| Repo-owned fixtures and qualification scratch defaulted outside Cargo's clean boundary. | LOW | Default shell/native helpers to `target/tmp`, preserving explicit caller/CI locations. Validate directory names and retain original failure/evidence custody. Shared allocators and Host FS still own allocation/cleanup mechanics. |
| The GitHub description understated Scheduling's ownership of Timers. | LOW | Correct the same repository's description to name durable Jobs and observable, recovery-aware Timers. Existing collaborator-only PR admission is retained. |

No maintained `test/` or `test.rs` owner was found. Existing test bodies/helpers
already use plural test paths or explicitly declared fixture packages. No aliases,
compatibility directories, symlinks or package moves are introduced.

Cargo lint tables do not support `expect`. Replacing the warnings with blanket
source expectations would retain unnecessary suppression. Fixing documentation
and removing obsolete allowances leaves no Rust/Cargo `allow` entries in maintained
source. Existing scoped expectations retain their reasons and enabled
`unfulfilled_lint_expectations` checking.

## Applicable rule coverage

| Rule family | Source/caller evidence and disposition |
| --- | --- |
| Snapshot adoption and local authority | `.shared-tooling.snapshot`, exported payload verification, `AGENTS.md`; only reviewed committed Shared files are adopted. Consumer LOC guidance is owned locally in `docs/operations/source-layout.md`. |
| Contribution and delivery authority | No staging, source commits, releases, publication or sibling edits for this audit. The same-repository description correction is separately observed through GitHub. |
| Ownership and simplicity | Jobs remains a Serde-only model; Timers owns the sole private provider. No new scheduler, lifecycle API, registry or dependency is added. |
| Complete/reviewable changes | Six role declarations, actual `make cloc` caller, public failure docs, scratch callers and operator docs are updated together. Group notes remain pending. |
| Rust workspace layout | Virtual root, eight standard `crates/`/`apps/` members, approved Jobs/Timers groups, explicit member versions and grouped root path/version catalog are preserved. |
| Dependency ownership and pinning | Child dependency inheritance, root-owned lock, immutable action refs and canonical Platform ownership are checked through existing admission scripts. Dependency versions and lock bytes are unchanged. |
| Compiler policy | Root/member declarations, `rust-toolchain.toml`, `minimum-rust.sh` and CI select 1.99.1; distinct native/Wasm consumers remain explicit. |
| Rust hygiene and test layout | Existing ordinary module discovery is retained. No executable Rust text patching, source `include!` indirection or test-path aliases are introduced. Parser/compiler fixture snippets remain data for their stated purpose. |
| Lint suppressions and public APIs | Strict native/Wasm Timers Clippy checks and Rustdoc cover the removed allowances and explicit error docs. Native verifier panic/failure documentation is improved. |
| Unsafe/native boundary | `scripts/ci/check-timer-provider.sh` retains private platform/FFI confinement; `ffi.rs`'s scoped unsafe expectation and native/Wasm export attributes remain intentional. |
| CLI contracts | Artifact capture, queue client and qualification driver use Clap. Custom queue-duration parsing is domain syntax in Clap's value parser, not a second argument parser. |
| Metrics and Logging | Timers records instrumentation through Metrics; the Jobs qualification app uses canonical Metrics/Logging. Canic retains reporting endpoints. Native qualification arithmetic does not acquire Metrics. |
| IC RPC and budgets | Owned DTOs use Serde/Wire. Management, delivery and probe RPC constants own request/response budgets; callers reuse those contracts. No owned Candid DTO or parallel codec is introduced. |
| Storage and uncertain effects | Qualification uses Storage's checked handles; decoded Jobs enter `Job::restore`. Commit-before-dispatch, exact receipts and explicit uncertain-effect disposition remain unchanged. |
| Wasm/encoding ownership | Existing IC Build recipes, target flags and artifact custody remain. No optimizer change, custom gzip encoder or footprint/performance claim is made. |
| Formatting and hooks | Reviewed selected-index hook is enabled; root/child grouped manifest sorting, Rustfmt and matching checks remain. Actual consumer hook/manifest-preservation fixtures qualify the new tables. |
| Validation selection and duplication | CI/group rosters reach separate lint/docs/archive leaves. Installed gates and standalone publication retain fresh boundary admission; no unqualified stage reuse or weakened gate is enabled. Producer regression suites are not replayed just to use cloc. |
| CI authority and promotion | Push/PR CI and manually selected recovery use read-only tokens, pinned actions, checkout credentials disabled and per-workflow/ref cancellation. Product graphs and failure evidence retain their existing owners. |
| Native host support | Linux plus Apple/Intel macOS matrix remains. Current Linux-focused results do not establish a fresh native macOS or installed-runtime pass. |
| Release identities and changelogs | Jobs 0.34.4 and Timers 0.40.5 are compatible pending notes. Actual package versions remain 0.34.3/0.40.4; root changelog remains an index. |
| Artifacts, scratch and cleanup | Shared release runner enforces aggregate/file/age budgets for Git-side custody. Source/lock/installed receipts retain original identities. No cleanup, ref pruning or relocation of historical evidence occurs. |
| Reporting and handoff | Classification is accounting, not code deletion. Macro/cfg diagnostics are retained and reviewed; matching code/docs/config identities qualify each reported check. |
| Frontend/npm, cross-runtime generation and crypto replacement | Not applicable: no maintained frontend/npm/browser codec or cryptographic implementation change. No speculative dependency, generator or migration is added. |

## Deliberate separations and limits

The initial classifier's 21 notes describe 20 macro bodies and the conditional
FFI export attributes. Macro definitions are counted where authored; no expanded
code or deployment size is inferred. The FFI alternatives are reviewed against
native/Wasm compilation. Zero unclassified source is not a runtime proof.

Ordinary and Watchdog control, Jobs restore admission, callback capture lifetime,
provider binding and recovery receipts protect distinct contracts; they remain
separate. This is a governance/source/tooling audit, not a fresh proof of deployed
application behavior, storage migration, cryptographic security or performance.

Hosted CI run `41884732991` succeeded for the starting handoff commit, but its
three native jobs selected documentation-only checks and skipped product
validation. It is not cited as native runtime qualification for this draft.
Timers 0.40.4's retained release evidence remains bound to its original source.
No live recovery, full CI/release gate or hosted dispatch is activated by this audit.

## Verification and conclusion

**PASS for the reviewed local source/tooling scope after the corrections above.**
The selected obligations below passed; this is not a full release or native-host
matrix verdict. Logs and the source inventory are under `target/shared-tooling030/`.

| Selected obligation | Result |
| --- | --- |
| Canonical export and snapshot integrity | 147 payloads independently verified against Shared Tooling 0.30.0. |
| Actual `make cloc` and JSON inventory | All 138 tracked Rust files accounted for; no legacy test owners or unclassified lines. |
| Removed lint allowances | Strict native/test and Wasm Timers Clippy pass without the three allowances. |
| Native tooling and API docs | Driver Clippy, 11 driver tests, four artifact-capture tests, queue-client test compilation and strict Rustdoc pass. |
| Scratch admission | Actual worker refuses raw/missing/invalid-alias roots before run allocation; shell caller refuses an invalid name before creating it. |
| Formatting and hooks | Grouped Cargo sorting, Rustfmt and real selected-index hook/preservation fixtures pass. |
| Both release groups | Real metadata/editor projection, rollback and refusal fixtures pass; product stages/uploads are substituted. |
| Consumer callers | Selected CLI/input/capture phases, recovery evidence, formatting evidence, CI selection and release-status fixtures pass with their documented substitutes. |
| Dependency/provider boundaries | Root inheritance/pinning, external IC declaration admission, group validation and provider checks pass. |
| Custody and documentation | Retained-state budget admission, links, source coverage, ShellCheck and diff checks pass; lock bytes remain unchanged. |

Initial failed formatting attempts and the invalidated running-driver attempt
remain retained; they are not treated as passes. No complete producer regression
matrix, live PocketIC recovery, full CI/release gate or new hosted run was executed.
