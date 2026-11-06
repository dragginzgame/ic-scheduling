# IC Jobs changelog

This ledger covers `ic-jobs`, `ic-jobs-test-consumer` and
`ic-jobs-canister-tests`. See the [Timers ledger](../timers/CHANGELOG.md)
and [coordinated history through 0.33.1](../history.md).

## [0.34.9] - 2026-11-06

### 📦 Dependencies

- Qualify the retained consumer graph with Runtime/macros/exact guard 0.19.4,
  Metrics 0.29.5, Logging 0.24.10, Platform 0.23.4, Storage 0.68.1,
  Testkit 0.89.3 and Wire 0.17.5. The public Jobs model remains Serde-only.

### 🛠️ Tooling

- Adopt Shared Tooling 0.31.0 and use captured source for selected Jobs releases
  and optional publication. Later normal-checkout lock updates stay pending;
  saved setup, recovery and archive checks retain the delivered graph. Preserve
  deferred edits before explicitly reconciling the normal branch.
- Use the 0.31 release settings and schema-2 batch journals with recorded
  publication commits. Finish older batches with their original tooling before
  upgrading; retired settings are rejected.
- Isolate nested consumer-tooling fixtures from the surrounding release batch
  selection and include candidate setup inputs in CI drift admission.

## [0.34.8] - 2026-11-06

### 📦 Dependencies

- Adopt Testkit 0.89 for private qualification, with Wire 0.17.4, Endpoint 0.7.2,
  Runtime/macros/exact guard 0.19.3 and Types 0.18.2. Retain the incoming Platform
  0.23.3, Testkit 0.89.1, Logging 0.24.9 and Metrics 0.29.4 patches in qualification.
  Harnesses do not rely on the removed automatic client logging;
  public Jobs APIs, Wire bytes and retained storage layouts are unchanged.

### 🛠️ Tooling

- Adopt reviewed Shared Tooling 0.30.7. Enforce Platform ownership of CBOR codecs
  in common Rust source and Cargo dependency admission, with no local exceptions.
  Owned application DTOs remain Wire-only; public library contracts are unchanged.

## [0.34.7] - 2026-11-06

### 📦 Dependencies

- Adopt Build 0.75, Storage 0.68 and Testkit 0.88 for the private qualification
  consumer and harnesses. Retain Host 0.74.4 and the incoming Platform 0.23.2 /
  Wire 0.17.3 selections. The public Serde-only Jobs API is unchanged.

### 🧭 Migration Notes

- Use fresh qualification backing for Storage 0.68's new Cell framing. Previous
  Cell images are refused without writes or growth, retaining their blocked
  attempts and receipts for explicit owner disposition. No migration, reset or
  replay is added. Cells no longer have checksums; the consumer still validates
  its bounded queue identity, records and restoration invariants.

## [0.34.6] - 2026-11-05

### 📦 Dependencies

- Adopt Testkit 0.87 and native Platform 0.23 with the deployed Runtime/macros/exact
  guard 0.19.2 cohort. Preserve Types 0.18.1 identity, compatible Runtime minima
  and the public Serde-only Jobs contract.
- Retain Logging 0.24.8, Metrics 0.29.3, Wire 0.17.2, Endpoint 0.7.1 and
  Build/Host 0.74.4 in the locked qualification graph.

### 🧪 Testing

- Migrate the installed recovery harness and queue demo to caller-owned async
  runtimes and explicit instance closure. Preserve uncertain-effect handling and
  keep Tokio out of the default artifact-capture binary's normal dependency graph.
- Cover terminal command-error shutdown alongside quit, EOF and Ctrl+C cleanup.

### 🛠️ Tooling

- Adopt reviewed Shared Tooling 0.30.4, including workspace scratch defaults,
  portable classified LOC output and numbered-draft changelog admission.
- Run the shared Candid source-boundary check once in common boundary admission,
  with no declaration allowances; owned application DTOs remain Wire-only.
- Apply compact experimental-evidence guidance to future documentation and raw
  output, preserving existing provenance and active recovery records.

## [0.34.5] - 2026-11-05

### 🛠️ Tooling

- Centralize workspace fixture scratch setup while retaining Shared allocation,
  caller TMPDIR overrides, invalid-path refusal and per-suite failure custody.

### 📦 Dependencies

- Retain the published Storage 0.67.1 patch in the locked combined-consumer graph.
  The compatible Storage 0.67 minimum and public library contracts are unchanged.

## [0.34.4] - 2026-11-05

### 🛠️ Tooling

- Adopt Shared Tooling 0.30.0 and its source classifier, with explicit roles for
  private qualification, artifact-capture and example source.
- Keep local fixture and recovery scratch under `target/tmp` by default while
  preserving explicit caller and CI evidence locations.
- Select Jobs and combined-consumer release checks individually for proven
  Jobs-only changes. Omit unchanged standalone Timers and release-tooling
  self-tests while retaining full repository CI and conservative shared-input
  qualification. Refuse evidence whose omissions differ from the product mapping.

### 📦 Dependencies

- Align private qualification on Storage 0.67.0, Testkit 0.86.0, Wire 0.17.1
  and Endpoint 0.7.0. They share one Wire identity; codec bytes and retained
  storage layouts are unchanged, so no backing reset is required. Codec-only
  callers no longer compile Wire's optional metadata derive.
- Select Logging 0.24.7, Metrics 0.29.2 and native Platform 0.22.1 in the root
  lock. Keep Types 0.18.1 and the exact Runtime 0.19.1 cohort, explicit managed
  server startup and the public Serde-only Jobs contract. Build/Host remain
  at their latest published 0.74.3 selections.

## [0.34.3] - 2026-11-04

### 📦 Dependencies

- Adopt Testkit 0.84.1's maintained PocketIC client for the private recovery harness
  and queue example, removing the separate Platform client dependency. Preserve
  explicit harness selection, caller identities and uncertain-effect recovery;
  the public Serde-only Jobs contract is unchanged.
- Adopt Metrics 0.29 for private job-outcome instrumentation. Preserve the
  consumer-owned report fields, Wire bytes, reset identity and actual-overflow
  flags without serializing recorder types. The public Jobs API is unchanged.

### 🛠️ Tooling

- Adopt retained-state admission around release adapter steps, preserving
  recovery evidence and refusing excessive storage or nested build/cache trees.

### 🧪 Testing

- Adopt Shared Tooling 0.29.4 and move private counter, artifact-capture and queue
  CLI tests/support into ordinary test modules. Keep the lightweight capture
  graph, explicit recovery harness and existing test behavior.
- Split queue tests by management, dispatch, recovery, admission, persistence and
  observability so each behavior can be selected independently.

## [0.34.2] - 2026-11-03

### 📦 Dependencies

- Update the private integration cohort to Metrics 0.28, Runtime 0.19, Types
  0.18, Storage 0.66, Testkit 0.83, Wire 0.16 and Endpoint 0.6. Select the native
  PocketIC client and facade on Testkit's matching 0.20 line, resolving the
  earlier client-identity blocker. The public Serde-only Jobs contract is unchanged.
- Select Logging 0.24.5 for bounded heap-only consumer diagnostics.
- Adopt the reviewed Shared Tooling 0.28.2 snapshot, including complete owned-tree
  inventory admission and updated release-group checks. Retain explicit group
  versions and runtime safety requirements.

### 🧹 Cleanup

- Fold the private management transport into `ic-jobs-canister-tests::harness`
  behind the explicit `harness` feature. Recovery tests and the queue example
  share that module; the default artifact-capture build retains its lightweight
  dependency graph. Remove the redundant support package from the workspace and
  Jobs release catalog, updating native entrypoints and graph checks together.
- Give the qualification consumer one management-method roster, one committed
  observation coordinator and a private bounded snapshot module. Retain RPC IDs,
  stored queue bytes, validated restoration and consumer-owned effects.
- Share the valid canister installation recipe between the recovery harness and
  queue demo. Keep deliberately invalid installation tests independent.

### 🧪 Testing

- Delegate baseline, roster and run-evidence admission to Shared Tooling. Use one
  ordered policy for full CI and selected releases; preserve complete common CI,
  standalone Timers gates and exact source/metadata recovery. Keep historical
  receipts with their original adapters.
  [#88 — Consolidate Jobs/Timers plan evidence](https://github.com/dragginzgame/ic-scheduling/issues/88).

- Handle native client failures explicitly during clock/cycle observation and
  simulated progression. Preserve typed canister rejection assertions; report
  demo transport errors through its existing error path without automatic retry.
- Adopt scoped release validation: changes confined to Jobs run complete
  native/Wasm CI without standalone Timers watchdog/cohort installation.
  Shared, dependency, Timers or uncertain changes retain the full gate.
  Bind group selection and successful execution evidence to release recovery.
- Match forbidden Build, Host and Testkit dependency families in the consumer
  graph checks, including future family members.

## [0.34.1] - 2026-11-02

### 📦 Dependencies

- Adopt Runtime 0.18, Types 0.17, Wire 0.15, Endpoint 0.5 and Testkit 0.80
  with the independent Platform PocketIC 0.18 native client. Preserve RPC
  identities, budgets, queue payloads and validated Job restoration. The public
  Serde-only Jobs API and Rust 1.99.1 requirement are unchanged.
- Replace Memory with Storage 0.65 for private checked allocation and Cells.
  Use Storage's native IO meters and remove the Testkit Native bridge. Retain
  existing 0.64 Cell installations and blocked effects; retired Memory images
  are refused without writes or reset. Retiring those old installations still
  requires explicit owner disposition of outstanding effects and authorization.
- Select Logging 0.24.1, Metrics 0.27.0, Build 0.74.0 and Host
  Artifacts/FS/Process 0.74.1 for private diagnostics and artifact tooling.
- Refresh Shared Tooling to 0.27.28. Retain the complete release gate;
  optional group-scoped validation plans remain unselected.

## [0.34.0] - 2026-11-02

### ⚠️ Breaking

- Require Rust 1.99.1 for library consumers and pin the same compiler for
  development and releases. [#87](https://github.com/dragginzgame/ic-scheduling/issues/87).

### 🧪 Testing

- Prepare one compiler in hosted workflows and admit actual Cargo/rustc
  identities through `rust-policy-check`. Retain native hosts, independent
  core/consumer checks, Wasm, archives and explicit installed-runtime gates.

## [0.33.5] - 2026-11-01

### 📦 Dependencies

- Select Platform 0.17, direct Runtime 0.17.1 and Types 0.16 for private
  canister/DTO integration. Select Testkit/Native 0.78's matching Wire and native
  client identity, preserving server custody and the public Serde-only Jobs API.
- Adopt Wire/Derive 0.14.1 for private consumer codecs and qualification
  transports, with Endpoint 0.4.0. Preserve request identities, codec
  budgets, stored queue bytes and validated Job restoration.
- Adopt Memory and Storage 0.62.0 in the private qualification consumer. Use
  checked allocation keys, nonzero one-page bucket configuration, fresh Cell
  creation and retained-only loading. Keep queue bytes, validated Job restoration
  and blocked-effect recovery unchanged. The published Jobs API and its
  Serde-only dependency boundary are unchanged.
- Select the current Build/Host 0.73.6 and Logging 0.23.7 qualification patches.
- Refresh the reviewed Shared Tooling snapshot to 0.27.26, retaining grouped
  Cargo metadata and dependency admission. Use Shared's current native build
  identity and Cargo tool retention policy in the existing setup workflow.

## [0.33.4] - 2026-11-01

### 🧹 Cleanup

- Use typed Wire endpoint attributes in the private qualification canister, with
  explicit method identities, tuple requests and existing byte/traversal budgets.
  Header admission runs before copying the payload. Oversized management and
  delivery requests now trap through Wire instead of returning a guard rejection;
  authorized calls, receipt writes and stored queue formats retain their contracts.

### 🧪 Testing

- Exercise release preparation, interrupted delivery, resume and publication in
  isolated fixture phases with scoped group inputs and fault injection.

## [0.33.3] - 2026-11-01

### 📦 Dependencies

- Adopt Build/Host 0.73.3, Testkit/Native 0.76.0, Wire/Derive 0.13.3 and Endpoint
  0.3.0. Refresh lock-selected tools through managed setup; preserve codec frames,
  budgets and typed Testkit integration with the existing Platform Types identity.
- Adopt Memory/Storage 0.60.6, its required Platform 0.16.0 facade/guard and
  Metrics 0.24 in the private consumer, aligning
  their codec and optional native schemas with Wire 0.13. Keep Jobs Serde-only;
  retained queue/ledger bytes, restore admission and blocked-effect recovery are
  unchanged. Testkit now shares the consumer's Metrics 0.24 identity.
- Adopt Logging 0.23.3 and Metrics 0.24.1, retaining bounded heap-only logs,
  public measurement identities, counter arithmetic and diagnostic reset boundaries.

### 🔧 Changed

- Document the Canic-owned private queue qualification and its installed intent
  rollback, lost-response receipt recovery and corrupt-restore evidence. Preserve
  the proposed public adapter boundary and separate paid-operation assessment.
- Add `make release-status` to observe group versions, publication, tags, CI and
  blocked inputs without executing a release. Offline and JSON views preserve
  unknown observations and distinguish committed metadata from an unprepared graph.
- Adopt Shared Tooling 0.27.23 and its group-catalog rules. The root Cargo catalog shows
  all nine members in Jobs/Timers blocks with their changelog paths; release and
  dependency checks admit each full version, local path and owning alias roster,
  including unused private entries. Selected-group preparation preserves
  requirement operators and the other group's identities.
- Run snapshot and documentation checks for an exact reviewed prose allowlist,
  retaining stable checks on all three hosts. Code, tooling, policy and unknown
  changes keep full native CI; failed classification cannot pass validation.
  [#83 — Adopt conservative documentation-only CI selection](https://github.com/dragginzgame/ic-scheduling/issues/83).

### 🩹 Fixed

- Isolate readiness-fixture group and commit inputs from the invoking release,
  so group-scoped release gates still exercise both groups' observations.
- Retain timer qualification directories and the final cohort report in hosted
  recovery artifacts, including failure outcomes. Exercise the actual upload
  selection against original Wasm, runtime receipts and server diagnostics.

## [0.33.2] - 2026-10-31

### 📦 Dependencies

- Adopt Shared Tooling's symmetric Cargo/release-group rules at committed revision
  `39a0a7612dab88d97d8e8f612fbfcf0b04fe83e6`. Every group owns explicit package
  versions and a ledger under `releases/`; releases and publication require an
  explicit group. Existing numeric versions and package paths are preserved.
- Adopt Build/Host 0.73.1, Testkit 0.75, Memory/Storage 0.59 and Logging/Metrics 0.23.2 in
  the private consumer. Keep Jobs Serde-only and preserve durable formats.
- Align the consumer runtime on Platform 0.15 and Wire 0.13 with Endpoint 0.2.
  Keep the bounded queue format and one runtime owner; typed host calls share
  the same Wire limits and error identities.
- Use the group-capable IC Build 0.73.1 metadata editor and align Host Process 0.73.1
  for the Timers build callback.

### 🔧 Changed

- Version and release Jobs independently from Timers within the same workspace.
  [#82 — Version Jobs and Timers independently](https://github.com/dragginzgame/ic-scheduling/issues/82).

### 🩹 Fixed

- Prepare Clippy for the minimum Rust toolchain in hosted CI.

## [0.17.51] - 2026-10-30

### 🧹 Cleanup

- Project committed job events into counters and logs through one flow, and use
  one management authorization path for diagnostics. Reuse Wire's codec hooks
  while retaining request limits, rejection codes and synchronous rollback.
- Share host management-call plumbing and controller snapshot sequences; use
  typed artifact-capture arguments and smaller native corruption fixtures.
- Derive contributor builds and hosted compiler setup from the declared Rust
  policy. Isolate release-fixture groups from one another and refresh stale
  release/status guidance. Published Jobs APIs and stored formats are unchanged.

### 📦 Dependencies

- Adopt Metrics 0.18.0 for qualification-consumer diagnostics, retaining counter
  arithmetic, report formats and reset boundaries. The published Jobs library
  remains Serde-only.
- Adopt Logging 0.18.0 with an explicit 32 KiB retained-text budget that preserves
  the existing 128-row limits. Keep all-topic paging, distinguish normal eviction
  from append refusals, and retain heap-only diagnostics independent of job state
  ([#79 — Adopt ic-logging 0.18 in the job-consumer logging adapter](https://github.com/dragginzgame/ic-jobs/issues/79)).

### 🧪 Testing

- Cover maximum UTF-8 log rows, truncation projection and text-budget refusal
  accounting alongside the existing retention, pagination and commit-boundary tests.

## [0.17.50] - 2026-10-30

### 🧪 Testing

- Replace the qualification canister's JSON queue with bounded native Wire and
  an explicit format identity. Preserve validated restoration, immutable delivery
  identities and blocked-effect recovery. Keep the published Jobs API Serde-only
  ([#78 — Replace job-consumer JSON queue persistence with bounded IC Wire payloads](https://github.com/dragginzgame/ic-jobs/issues/78)).

### 📦 Dependencies

- Adopt Platform 0.8.3, Metrics/Logging 0.17.7, Wire 0.7.24, Timers 0.32.3,
  Memory/Storage 0.56.1 and Build/Host/Testkit 0.70.4. Refresh the lock-selected
  contributor tools with `make install-testkit-tools install-release-tools`.

### 🧭 Migration Notes

- **Qualification queue format cut:** retained JSON refuses without conversion
  or reset. Use fresh local test canisters. Before retiring any retained
  installation, keep its old binary/backups and resolve Running/Uncertain effects
  with exact receipts or an explicit owner-approved disposition. Source adoption
  grants no destructive reinstall or redispatch permission. Memory's allocation
  ledger remains a separate format boundary; same-format Wire recovery is retained.

## [0.17.49] - 2026-10-30

### 🧪 Testing

- Qualify public stored records and attempts through bounded Wire in the native
  consumer, including wide identities, all job states and refusal of invalid
  history. Keep the published library's Serde-only dependency boundary.
- Extend the runnable adapter example with duplicate receipts, stale completion
  after safe retry and management changes during delivery. Include the example
  and stored Wire checks in the normal consumer validation selection.

### 📚 Documentation

- Add a Canic adapter integration guide covering ownership, domain identities,
  commit ordering and installed acceptance. Correct stale codec, command and
  qualification claims across the README, safety contract and consumer guides.

## [0.17.48] - 2026-10-29

### 📦 Dependencies

- Adopt Build/Host/Testkit 0.70.2 and prepare the matching contributor CLIs with
  `make install-testkit-tools install-release-tools`. Retain existing process,
  artifact-capture and PocketIC lifecycle policies.
- Adopt Memory/Storage 0.56.0. Published Jobs APIs and JobRecord/queue JSON are
  unchanged; the qualification consumer now uses Memory's bounded Wire ledger.

### 🧭 Migration Notes

- **Qualification canister:** earlier CBOR allocation ledgers are incompatible.
  Use a fresh test installation or complete an explicitly owned transition before
  upgrading retained state. Keep old binaries/backups and blocked-effect evidence;
  never clear only the allocation ledger or reset automatically after refusal.
  No existing canister data is reset by this source update.

### 🧪 Testing

- Freeze a valid Memory 0.55.2 installation with Running intent and verify that
  recovery and fresh creation refuse it without writes while its Jobs payload
  still restores as blocked through Job::restore.

## [0.17.47] - 2026-10-29

### 📦 Dependencies

- Adopt Timers 0.32.1 and Metrics 0.17.4, preserving timer APIs, counter
  arithmetic, stored records and uncertainty handling. The consumer, Memory and
  Timers now share Platform 0.8; restore the single-facade graph requirement.
  Qualify the retained Wire/derive 0.7.23 selection without codec/protocol changes.

### 🧪 Testing

- Extend installed watchdog recovery coverage to verify that dispatch, success
  and permanent-failure counters roll back with a trapped message.
  [#76 — Add bounded job-outcome Metrics at committed consumer transitions](https://github.com/dragginzgame/ic-jobs/issues/76).

### 📚 Documentation

- Specify the consumer's single-attempt Platform transport and exact-receipt
  recovery boundary, keeping retry authority with durable Jobs state.
  [#75 — Keep job-consumer ICC execution under Platform ownership](https://github.com/dragginzgame/ic-jobs/issues/75).

## [0.17.46] - 2026-10-29

### 📦 Dependencies

- Adopt Platform/macros 0.8.1, keeping owned application principals independent
  of external conversion and hashing. Native tests explicitly select principal
  hashing and the harness selects PocketIC conversions. Published Timers
  retains its narrow Platform 0.7 re-exports while sharing one executor, IC0
  provider and entrypoint generator with the consumer. Jobs APIs, principal bytes,
  stored formats and retry authority remain unchanged.
- Adopt Build/Host/Testkit 0.70.0 and Wire/derive 0.7.22. Testkit now shares the
  consumer's Platform 0.8 types; Jobs needs no ledger or Registry type migration.
  Prepare the matching contributor CLIs with
  `make install-testkit-tools install-release-tools`. Wire bytes, codec budgets
  and stored records remain unchanged.

## [0.17.45] - 2026-10-29

### 📦 Dependencies

- Adopt Metrics/Logging 0.17.2, preserving counter and heap-history behavior.
  Logging's new checkpoint APIs do not change this consumer's heap-only policy
  or require a data migration.
- Adopt Memory/Storage 0.55.0, preserving allocation custody and collection
  formats. The upstream Metrics Stable format cut does not affect Jobs, which
  does not use that adapter; no retained-data reset or caller migration is needed.
- Adopt Build/Host/Testkit 0.69.13 and its lock-selected contributor CLIs through
  `make install-testkit-tools install-release-tools`. Preserve command admission,
  artifact retention, PocketIC selection and published Jobs APIs.

## [0.17.44] - 2026-10-29

### ➕ Added

- Add bounded consumer transition history using IC Logging 0.17.1, recorded after
  committed dispatch, outcome and reconciliation transitions. Manager-only
  `job_logs` cursor pages and the demo's `logs [--before SEQUENCE]` command expose
  recent redacted context with bounded scan work and explicit omitted counts.
  History is heap-only; published Jobs APIs, durable state and retry authority
  remain unchanged.

## [0.17.43] - 2026-10-29

### 📦 Dependencies

- Adopt Metrics 0.17.1 and Platform/macros 0.7.6 in the qualification consumer
  and native harness. Metrics retains its arithmetic and published crate identity
  after the IC Observability repository move; Platform retains explicit endpoint
  codecs and bounded decoding. Jobs APIs, stored formats and retry authority are
  unchanged; no consumer migration is required.

## [0.17.42] - 2026-10-29

### ➕ Added

- Add consumer-owned job-outcome diagnostics using IC Metrics counters at
  committed dispatch, completion and reconciliation boundaries. Manager-only
  `job_metrics` and `reset_job_metrics` Wire methods, and `metrics`/`reset-metrics`
  demo commands, expose wide totals and overflow quality scoped to the canister
  version and heap reset window. Stored Jobs state and retry authority are unchanged.
  [#76 — Add bounded job-outcome Metrics at committed consumer transitions](https://github.com/dragginzgame/ic-jobs/issues/76).

### 🩹 Fixed

- Bind demo fixture Cargo stubs before launching Make, preserving direct and
  recursive command isolation on macOS's GNU Make 3.81. Print the retained demo
  log on failure while preserving its exit status.

## [0.17.41] - 2026-10-29

### 📦 Dependencies

- Adopt Build/Host/Testkit 0.69.9 and its lock-selected contributor CLIs with
  `make install-testkit-tools install-release-tools`. Testkit restores cached
  baseline timer and low-memory-hook state coherently with heap snapshots.
  Jobs uses fresh instances and needs no custom baseline-recipe migration;
  library APIs, retained formats and uncertainty handling remain unchanged.
- Adopt Timers 0.32.0 and its Metrics 0.17 measurement types. The qualification
  consumer needs no caller migration; Metrics transport now includes Serde and
  native Wire metadata, while the Wasm timer subtree stays Wire/Candid-free.
  Heap recording stays disabled and published Jobs remains Serde-only.
- Adopt Memory/Storage 0.54.0. Its Metrics Stable type-identity change needs no
  Jobs caller migration because Jobs does not use that adapter. Backing custody,
  allocation, retained formats and restore checks remain unchanged.

## [0.17.40] - 2026-10-29

### 📚 Documentation

- Extend the packaged persisted-job example into an adapter reference with
  commit-failure injection, blocked reconstruction and exact receipt recovery.
  Document the host integration contract and link existing native/installed
  recovery scenarios as an adapter acceptance checklist. No public API changes.

### 📦 Dependencies

- Adopt Memory/Storage 0.53.1, Build/Host/Testkit 0.69.8, Platform 0.7.4,
  Timers 0.31.16 and Wire 0.7.21. Memory fixes backing-claim teardown ordering;
  Jobs APIs, restore checks, stored formats and uncertainty handling remain unchanged.
- Qualify transitive Metrics 0.16.5 through Timers. Optional heap recording stays
  disabled; Jobs remains Serde-only and needs no Metrics API or storage migration.
- Prepare selected contributor CLIs with
  `make install-testkit-tools install-release-tools` after updating dependencies.

## [0.17.39] - 2026-10-28

### 📦 Dependencies

- Adopt Memory/Storage 0.53.0 and Platform 0.7.3, retaining stored formats and
  the published Jobs API. Memory validates manager metadata at construction,
  initial bootstrap, lease opening and growth; ordinary IO trusts exclusive
  backing custody. Duplicate-owner refusal and sticky poisoning remain enforced.
- Preserve the incoming Tooling/Testkit 0.69.6 selection and prepare its
  contributor CLIs with `make install-testkit-tools install-release-tools`.
  Timers and Wire selections remain unchanged.

## [0.17.38] - 2026-10-28

### 📦 Dependencies

- Adopt Memory/Storage 0.52.0, Build/Host/Testkit 0.69.5, Platform 0.7.2,
  Timers 0.31.15 and Wire 0.7.19. The published Jobs API, retained formats,
  receipt validation and uncertainty handling remain unchanged.
- The qualification consumer supplies Memory's backing identity and releases
  old owners before reopening retained cells. Duplicate owners are refused;
  Memory now validates manager metadata during stable IO.
- Prepare selected contributor CLIs with
  `make install-testkit-tools install-release-tools` after updating dependencies.

## [0.17.37] - 2026-10-28

### 📦 Dependencies

- Adopt Build/Host/Testkit 0.69.3, Memory/Storage 0.51.13, Platform 0.7.1 and
  Timers 0.31.11, retaining Wire 0.7.18. The published Jobs API, stored formats,
  receipt checks and uncertainty handling remain unchanged.
- Prepare the lock-selected contributor CLIs with
  `make install-testkit-tools install-release-tools` after updating dependencies.

## [0.17.36] - 2026-10-27

### 🔧 Changed

- Adopt Shared Tooling 0.26.25. Standard formatting reports progress and elapsed
  time per pass, with prepared-tool checks retained and result reuse limited to
  eligible unchanged inputs. Jobs keeps its existing workspace and manifest policy.

### 🧪 Testing

- Qualify formatting failure logs, CI evidence collection and staged hooks with
  the shared driver. Preserve sorter-before-Rustfmt ordering and failed-log custody.
- Refresh the shared hook's opaque-submodule handling and optional standalone
  Rustfmt support without changing Jobs' selected source coverage.

## [0.17.35] - 2026-10-27

### 📦 Dependencies

- Adopt Build/Host/Testkit 0.69.0, Memory/Storage 0.51.9, Timers 0.31.9 and
  Platform/macros 0.7.0. Keep one Platform identity across the qualification
  canister and native harness. The published Jobs API, Wire payloads and stored
  formats remain unchanged. Prepare contributor CLIs
  with `make install-testkit-tools install-release-tools`.

### 🥾 Governance

- Adopt Shared Tooling 0.26.22, clarifying owned-type Candid restrictions and
  native input/error-handling guidance. This updates the reviewed rules without
  adding executable enforcement or new core dependencies.

## [0.17.34] - 2026-10-27

### 🧹 Cleanup

- Share delivery-sender authorization and bind outbound RPC names to their
  existing identities. Name internal queue totals and share native installation
  encoding without changing public APIs, message bytes or retained storage.
- Derive demo scheduling requests and labels together, and keep pagination
  sizing in one place.

### 🧪 Testing

- Isolate publication-test environments and simplify native fixture setup/imports.
  Keep codec implementation proofs with Wire while retaining Jobs' envelope,
  budget, malformed-reply and uncertain-effect qualification.

## [0.17.33] - 2026-10-27

### 🧹 Cleanup

- Simplify consumer dispatch results, native reconstruction and inspection
  replies while preserving validated restore, commit ordering and receipt checks.
- Share demo launch plumbing and deadline display formatting, keeping the
  existing commands and distinct Testkit server lifetimes.

### 🧪 Testing

- Let release retention fixtures run without metadata-editor or formatter setup.
  Name admission/management cases and reuse panic-retaining fixture cleanup.

## [0.17.32] - 2026-10-27

### 🧹 Cleanup

- Share delivery eligibility and outbound transport handling while preserving
  authentication, exact receipt comparisons and uncertain-effect blocking.
  Public APIs, endpoint codecs and stored formats remain unchanged.
- Keep Testkit executable selection in one consumer helper, with unchanged
  setup/admission boundaries and validation stopping if its inputs change.

### 🧪 Testing

- Isolate consumer-tooling phases and reuse canonical fixture setup utilities,
  retaining refusal ordering, frozen-artifact custody and publication checks.
- Name receipt mutation cases and CLI shutdown scenarios, and carry confirmed
  notification outcomes directly without an intermediate boolean mapping.

## [0.17.31] - 2026-10-27

### 📝 Summary

- Adopt Tooling/Testkit 0.68.9, Memory/Storage 0.51.5, Timers 0.31.5,
  Platform/macros 0.6.19 and Wire/derive 0.7.18, preserving the incoming lockfile.
  Jobs' public APIs, endpoint codecs and stored formats are unchanged; no client
  regeneration or retained-data migration is required.
- Adopt Shared Tooling 0.26.21 and its emoji section format for current and future
  root release notes. Existing release text and historical entries are preserved.

### 🧹 Cleanup

- Separate retained-job validation into ordered phases and compare delivery
  receipts without copying their payloads. Existing restore admission, error
  precedence, endpoint codecs and stored formats are preserved.
- Reuse the previous display's queue views within CLI watch sessions, retaining
  interruption checks before advancing IC time and fresh reads for other commands.

### 🧪 Testing

- Install receiver canisters only for delivery scenarios and avoid repeating
  codec checks for equivalent retained states in transition-history exploration.
- Offer independently runnable retention, delivery, recovery, metadata and
  publication release-fixture groups. The existing aggregate retains all groups
  and the publication eligibility fixture.

## [0.17.30] - 2026-10-26

### 📝 Summary

- Adopt Tooling/Testkit 0.68.7, Timers 0.31.4, Wire/derive 0.7.16 and the incoming
  Platform/macros 0.6.18 graph. Jobs' public APIs, endpoint codecs and stored
  formats are unchanged; no client regeneration or retained-data migration is
  required.
- Adopt Shared Tooling 0.26.19: normal release targets start fresh from current
  committed metadata. Use `release-resume VERSION=X.Y.Z` to recover a saved
  attempt. Historical changelog Draft labels no longer block a fresh release.
- Refresh Shared Tooling to 0.26.20 with focused validation guidance and a
  compact baseline backed by specialist rules. Detailed change accounting is
  opt-in; release gates retain their complete validation.

## [0.17.29] - 2026-10-26

- Adopt Tooling 0.68.5, Memory/Storage 0.51.3, Platform/macros 0.6.16,
  Timers 0.31.2 and Wire/derive 0.7.12, preserving the incoming resolved selections.
  Existing caller APIs, endpoint codecs and stored formats are retained;
  no client regeneration or retained-data migration is required.

## [0.17.28] - 2026-10-25

- Qualify Tooling 0.68.2, Memory/Storage 0.51.1, Metrics 0.16.2, Timers 0.31.1
  and Wire/derive 0.7.9, preserving the incoming resolved selections. Platform
  remains 0.6.15; Jobs' published API, RPC layouts and stored formats are unchanged.
- Adopt Shared Tooling 0.26.16, including distinct busy-tool admission and the
  common generated-RPC budget ownership rule. Existing tool pins and native
  consumer-owned budgets remain unchanged.
- Stop `watch` before advancing IC time when Ctrl+C arrives during its wait,
  preventing one last job dispatch after the stop request. Extend the real CLI
  process check to cover interruption during an active watch.

## [0.17.27] - 2026-10-25

- Use typed commands in the local queue client while preserving command names,
  flags, aliases and duration validation. Native management calls share Testkit's
  bounded Wire transport; recovery assertions retain typed canister rejections
  and malformed-frame tests keep raw transport.
- Qualify Metrics 0.16.1 through Timers 0.31.0 and adopt Wire/derive 0.7.8,
  retaining the incoming Memory/Storage 0.51.0 and Tooling 0.68.1 selections.
  Jobs' public API, RPC layouts and stored formats remain unchanged; Metrics
  remains transitive and no storage reset or client regeneration is required.

## [0.17.26] - 2026-10-24

- Adopt Platform/macros 0.6.15 and Timers 0.30.12 in the qualification consumer,
  retaining the incoming Wire 0.7.2 selection. Their library source is unchanged
  from the prior releases; no Jobs caller, stored-format or scheduling-policy
  migration is required. The published core remains Serde-only.

## [0.17.25] - 2026-10-24

- Adopt Wire/derive 0.7.1 and Build/Host/Testkit 0.68.0 for the qualification
  consumer and native tooling. Existing DTOs, emitted frames, stored JSON and
  Jobs' published API remain unchanged; no caller migration is required.
  Prepare the selected executables with
  `make install-release-tools install-testkit-tools` before offline checks.

## [0.17.24] - 2026-10-24

- Adopt Wire 0.7's one-pass decoder in the qualification consumer and native
  callers. Decode helpers require only deserialization; malformed integers remain
  refused. Rebuild the fixture and harness together. The published Jobs API,
  emitted RPC layouts, method IDs and stored JSON are unchanged.
- Adopt Shared Tooling 0.26.13, including setup timing diagnostics and committed
  release-preparation recovery. Use its common external IC dependency inventory
  while retaining Jobs' stricter local bans. Tool pins are unchanged.
- Qualify Platform/macros 0.6.14, Memory/Storage 0.50.16 and Timers 0.30.11.
  Metrics remains transitive at 0.15.8; native tooling stays at 0.67.0.

## [0.17.23] - 2026-10-24

- Adopt Build, Host and Testkit 0.67.0 for native tooling and qualification.
  Jobs' existing artifact capture and PocketIC APIs need no caller migration;
  the published library API and consumer-owned scheduling policy are unchanged.
  Rerun `make install-release-tools install-testkit-tools` to prepare the selected
  executables before offline checks.
- Correct the README's dependency graph description to reflect the single
  Platform identity shared by the fixture and Timers, and link snapshot metadata
  to its authoritative manifest. Resolves
  [#73 — Refresh advisory README dependency and snapshot claims against their current owners](https://github.com/dragginzgame/ic-jobs/issues/73).

## [0.17.22] - 2026-10-24

- Install only Binaryen for IC qualification. Contributors should rerun
  `make install-ic-tools` to prepare the narrowed selection. Release fixtures
  enforce the same selection during tool preparation and offline checks.
- Add explicit real-terminal shutdown tests for quit, EOF and Ctrl+C through
  `make test-demo-cli`, including owned PocketIC server cleanup.
- Reuse the post-advance job listing when displaying the queue, avoiding a
  redundant pagination pass during advances and watch updates.

## [0.17.21] - 2026-10-23

- Qualify Platform/macros 0.6.10, Metrics 0.15.7 and Timers 0.30.6 for the
  consumer fixture. Their runtime source is unchanged from the prior selections;
  Metrics remains transitive through Timers. Jobs APIs, stored formats, Wire
  budgets and the Rust 1.91 floor are unchanged.

## [0.17.20] - 2026-10-23

- Adopt Host, Build and Testkit 0.66.0. The recovery harness and queue client
  borrow PocketIC through its explicit accessor, retaining the owner for the
  full session. Published Jobs APIs, Wire frames and stored formats are unchanged.
- Adopt Shared Tooling 0.26.8, fixing setup contention during another owner's
  lock cleanup and removing the global Cargo source-cache setup lock. Tool
  versions and digests are unchanged; existing generations remain retained.

Rerun `make install-rust-tools install-release-tools install-testkit-tools` once
to prepare the tools under the updated build policy before offline checks.

## [0.17.19] - 2026-10-23

- Adopt Host, Build and Testkit 0.65.0 for native tooling and qualification.
  Jobs already uses Wire's native Serde framing; no public API, stored format
  or RPC migration is required by Testkit's new optional Wire helpers.
- Adopt Shared Tooling 0.26.7. Explicit tool setup can wait up to 60 seconds
  for another checkout's shared installation; offline checks still refuse
  immediately. Preserve the existing IC executable set through explicit setup,
  recovery and failure-evidence selections. Tool versions and digests are unchanged.
- Include the CI/release integrity audit in the snapshot selection, fixing the
  missing document linked from the audit index that blocked release validation.

Prepare the refreshed selections with
`make install-ic-tools install-release-tools install-testkit-tools` before offline checks.

## [0.17.18] - 2026-10-23

- Adopt Wire 0.6 for the qualification canister and native clients, retaining
  explicit Serde framing, method identities and resource limits. Jobs does not
  use the finite-schema APIs removed upstream; no client regeneration is needed.
- Select Platform 0.6.8, Memory/Storage 0.50.11 and Build/Host/Testkit 0.64.1.
  Testkit's additional Candid management support remains native-only. Published
  Jobs APIs, production dependencies, stored data and Rust 1.91 support are unchanged.

Prepare the selected executables with `make install-release-tools install-testkit-tools`
before running offline tooling checks.

## [0.17.17] - 2026-10-22

- Adopt Timers 0.30.3 for the qualification consumer and require its Wasm graph
  to share one Platform identity. Published Jobs dependencies, timer APIs,
  schedules and stored formats are unchanged.

## [0.17.16] - 2026-10-22

- Adopt Host 0.64 with matching Build and Testkit packages. Native recovery tests
  and the local queue client now import PocketIC types directly from Platform.
  Published Jobs APIs, production dependencies and stored data are unchanged.

Prepare the selected executables with `make install-release-tools install-testkit-tools`
before running offline tooling checks.

## [0.17.15] - 2026-10-22

- Add `make demo`: an interactive terminal connected to the local Jobs canister,
  showing queued jobs, deadlines, execution counts and watchdog state. Queue
  seconds/minutes/weeks, watch countdowns, fast-forward IC time and manage jobs.
- Add `make test-demo` for the live CLI scenario. Ordinary consumer checks run
  input tests and compile the live case without launching PocketIC. The published
  library API and production dependencies are unchanged.

## [0.17.14] - 2026-10-21

- Use the published Testkit native IO observer for qualification and remove the
  local write counter. Keep shared reset, backing-byte and no-write assertions,
  and reject overflowed counts. The observer remains a dev-dependency; public
  APIs, stored data and production dependencies are unchanged.
  [#56 — Adopt canonical Testkit memory IO observer after publication](https://github.com/dragginzgame/ic-jobs/issues/56).
- Select Memory/Storage 0.50.6 and Wire 0.5.12 for qualification.

## [0.17.13] - 2026-10-21

- Refresh native IO observer adoption guidance for the consolidated Tooling owner.
  Qualify the replacement against the current Jobs source in isolation; retain
  the local wrapper until the supplying registry package is published.
  [#56 — Adopt canonical Testkit memory IO observer after publication](https://github.com/dragginzgame/ic-jobs/issues/56).

## [0.17.12] - 2026-10-21

- Adopt matching Build, Host and Testkit 0.63.0 packages. Select their required
  `cli` feature only when preparing and checking the release editor and Testkit
  server CLI; library dependencies retain their default features.
- Testkit now shares Platform 0.6 and drops its Candid adapters and Metrics
  dependency. Jobs already uses Wire calls; Candid remains required transitively
  by PocketIC's external protocols. Published Jobs APIs, stored data and the
  Rust 1.91 floor are unchanged.
  [#223 — Remove Testkit-owned Candid call APIs and codec coupling](https://github.com/dragginzgame/ic-tooling/issues/223).
- Select Memory/Storage 0.50.3 and Wire 0.5.6 for qualification.

Prepare the new executables with `make install-release-tools install-testkit-tools`,
then run `make release-tools-check testkit-tools-check` before offline tooling checks.

## [0.17.11] - 2026-10-21

- Adopt Platform 0.6.0 and explicitly enable `principal` for native DTOs and
  recovery tests. Keep the published Jobs API, Rust 1.91 floor, Wire frames and
  stored JSON unchanged; no data reset or conversion is required.
- Retain Platform 0.5 transitively for released Timers 0.30 and native Testkit.
  Qualify both wrappers with one executor, raw-system provider and entrypoint
  generator; the qualification Wasm remains Candid-free.

## [0.17.10] - 2026-10-21

- Adopt Shared Tooling 0.25.9: keep saved preparation when its candidate version
  is committed without confirmed delivery, rather than silently incrementing
  again. [#292 — Prevent committed release preparation from silently incrementing twice](https://github.com/dragginzgame/shared-tooling/issues/292).
- Refuse publication when HEAD, workspace version or the release tag changes
  during admission or Clippy, including changes committed to a clean tree.
  Published Jobs APIs, stored data and consumer behavior remain unchanged.
- Select Timers 0.29.7 for qualification, sharing Platform 0.5 with the consumer
  and removing the older Platform identity from its locked graph.
- Adopt Tooling 0.62.1, Memory/Storage 0.50.1 and Platform 0.5.1, retaining Wire
  0.5.4 and the Rust 1.91 floor. Prepare the newly selected release editor and
  Testkit CLI with `make install-release-tools install-testkit-tools`.
- Share the delivery RPC codec hooks while preserving method identities, bounds,
  replies and stored bytes. Retire the two-Platform graph allowance and guard
  the native capture binary against Candid/PocketIC dependencies. Candid remains
  required by external PocketIC protocols in native recovery tests only.

## [0.17.9] - 2026-10-20

The published Jobs API, Serde-only runtime dependencies and Rust 1.91 minimum
remain unchanged. These changes affect contributor tooling and the unpublished
qualification application.

### Changed

- Align Build, Host and Testkit at 0.62.0, preserving optimizer capture,
  retained failure evidence and PocketIC lifecycle policy. Keep optional
  filesystem backends disabled and native tooling outside the qualification
  Wasm graph.
- Adopt matching Memory and Storage 0.50.0 dependencies for qualification.
  Stored formats, allocation identities and lease/reopen behavior are unchanged;
  no backing reset or data conversion is required by this update.
- Adopt Platform 0.5.0 and Wire 0.5.3. Remove retired no-op entrypoint options
  while preserving handler names, Wire frames and stored JSON. PocketIC Principal
  conversions remain explicit at application boundaries.
- Adopt Shared Tooling 0.25.8. Run the complete core, native qualification and
  Wasm Clippy gate before CI builds, tests, packaging and publication or its dry
  run. Lint failures stop later stages under parallel Make.
- Retain failed Cargo metadata diagnostics and input identities during release
  preparation. Automatically replace numbered `- Unreleased` changelog markers
  with the saved release date before committing the release.

### Contributor setup

After updating, prepare dependencies with `cargo fetch --locked`, then run
`make install-release-tools install-testkit-tools` and
`make release-tools-check testkit-tools-check`. Rebuild the fixture and harness
together before explicitly selecting live IC qualification.

## [0.17.8] - 2026-10-20

- Refresh the reviewed Shared Tooling snapshot to committed 0.25.4 source,
  including feature-bound tool receipts and actionable admission diagnostics.
  Rerun `make install-tools`, `make install-release-tools` and
  `make install-testkit-tools` before their offline checks.
- Adopt Platform 0.4.0's owned Principal in the unpublished fixture, converting
  explicitly at PocketIC boundaries. Preserve existing Wire frames and stored
  JSON and raw Wire calls. Keep one executor/runtime-provider identity while
  admitting the two released Platform versions used by the app and Timers.
  [#70 — Use IC Wire for the unpublished job-consumer protocol and recovery harness](https://github.com/dragginzgame/ic-jobs/issues/70).

- Adopt Testkit 0.57.0 and direct Host 0.31.1 dependencies in qualification
  tooling, retaining native Wire callers, capture/error policy and consumer-owned recovery.
  Published Jobs APIs, fixture messages and stored records remain unchanged.
  [#52 — Adopt shared child-status projection after Host publication](https://github.com/dragginzgame/ic-jobs/issues/52),
  [#63 — Adopt panic-safe diagnostic reporter after Testkit publication](https://github.com/dragginzgame/ic-jobs/issues/63).

## [0.17.7] - 2026-10-19

- Adopt Wire 0.5.0, Testkit 0.56.0 and Platform 0.2.2 for qualification tooling.
  Testkit now shares Host 0.30 and Platform 0.2 with the workspace, removing the
  older Host capability-filesystem dependency chain. Existing fixture Wire bytes,
  method identities, stored JSON and the published Jobs API remain unchanged.

## [0.17.6] - 2026-10-19

- Require one Platform identity in the qualification Wasm graph now that Timers
  0.29.3 shares Platform 0.2.1, fixing CI's stale two-version requirement.

- Adopt Platform 0.2.1's owned runtime and switch the unpublished fixture's
  installation, management and fault-control messages to bounded Wire frames.
  Rebuild the fixture and harness together and dispose of old Candid requests;
  no fallback or generated Candid service remains. Published Jobs APIs, stored
  JSON, delivery identities and uncertain-effect disposition stay unchanged.

- Use Host's exit-status projection and Testkit's panic-contained diagnostic
  writer in qualification tooling, preserving optimizer failure codes and the
  original canister installation error. Keep the published Jobs API and storage
  contracts unchanged.
- Adopt Host 0.30 for direct native tooling dependencies, keeping optional
  filesystem capability and temporary-file backends disabled. Testkit retains
  its released Host 0.29 selection; Build 0.22 uses Host 0.30.

## [0.17.5] - 2026-10-19

- Adopt reviewed Shared Tooling 0.25.0 source. Prepare and select the released
  IC Build metadata editor automatically for standard release commands, without
  shell exports. Keep recovery bound to its saved source's selection without
  installation or dependency fetching. Retire the Perl renderers and use Shared's
  grouped-formatting defaults. Standalone offline fixtures retain explicit
  [tool preparation](../../docs/releasing.md#preparation).
- Use Clap for the unpublished capture CLI's parsing, help and argument errors,
  preserving capture and optimizer invocations. Keep dependency-role headers in
  the root catalog and preserve the incoming Host 0.28.1 lockfile update.

## [0.17.4] - 2026-10-19

- Switch the unpublished qualification consumer's two delivery RPCs from Candid
  to bounded IC Wire frames, with no fallback. Quiesce deliveries, dispose of
  in-flight calls and rebuild/upgrade both fixture canisters together. Stored
  JSON, management Candid and the published Jobs API remain unchanged.
- Refresh qualification/tooling dependencies to Build 0.18.0 and Platform 0.1.15,
  preserving the incoming dependency updates and the core's Serde-only boundary.

## [0.17.3] - 2026-10-18

- Refresh locked qualification/tooling dependencies to Platform 0.1.13,
  Build 0.17.6, Host 0.27.6, Storage 0.5.4, Testkit 0.54.1 and Metrics 0.14.4.
  Keep the published Jobs library's Serde-only runtime boundary.

- Select Rust 1.99.1 consistently for development and both hosted workflows,
  including recovery evidence. Keep the core and consumer Rust 1.91 floors.
  [#69 — Qualify Rust 1.99.1 as the development compiler](https://github.com/dragginzgame/ic-jobs/issues/69).

- Use Timers' published completed-batch projection for watchdog results in the
  qualification canister and native maintenance consumer. Keep work-count meaning,
  deadlines and trap-before-delivery ordering consumer-owned.
  [#57 — Adopt Timers completed-batch projection](https://github.com/dragginzgame/ic-jobs/issues/57).

## [0.17.2] - 2026-10-18

- Keep native qualification DTO consumers independent of Jobs, Memory, Timers
  and JSON implementation dependencies; retain those dependencies for Wasm and
  native tests, and enforce the split in graph checks. Use Platform's Serde
  contract export in the recovery harness. Keep the full IC tool bundle required.

- Repair formatting-hook and release-tooling qualification's stale dependency
  pairs after the direct Candid declaration was removed. Qualify current member
  manifests and Rust sources together with the root manifest during pending
  dependency migrations.

- Align consumer guidance with the completed Platform/Testkit/Storage boundaries.
  Keep version selection in manifests and source-bound qualification in the
  handoff; archive superseded migration receipts without removing their evidence.

- Adopt published IC Platform 0.1.11 and its CDK frontend for the qualification
  canister. Route entrypoints, calls, lifecycle, protected spawning and traps
  through Platform; remove direct Candid/CDK declarations and forbid their
  reintroduction. Retain upstream CDK/executor identities, wire and storage
  contracts, and the separate live-recovery qualification gate.

## [0.17.1] - 2026-10-18

- Replace the native fixtures' direct stable-structures dependency with published
  IC Storage 0.5.3's governed cells. Construct oversized corruption through Memory's
  explicit fixture ceiling; preserve retained bytes and refusal behavior. Reject
  direct substrate dependencies across workspace members and exclude Storage from
  core and deployed qualification graphs. Storage remains a native dev dependency.

- Add resolved normal/build graph admission for native/Wasm core exclusions
  and Wasm qualification exclusions/Timers-executor identity. Jobs owns the
  target/feature policy; Shared owns the checker through reviewed topic source
  `6eb8c41785c5156aef62f082702bd54e8ce04606` (Shared PR #285; not a release).
  Preserve consumer depth, package-family, dependency-feature and optional
  identity-cap policies in the shared observer.
- Adopt published Testkit 0.54.0 and remove `ic-testkit-candid` from the harness
  and resolved graph. Use Platform's decoder limits/errors directly and forbid
  new direct declarations. Preserve all five budgets, argument-tuple reply handling
  and typed call errors; Testkit's required Metrics 0.14 stays native-only.
- Adopt published ic-platform 0.1.9 for qualification DTOs, Principal, encoding
  and bounded delivery replies. Update only Platform from locked 0.1.5; keep
  defaults disabled and select only Candid. Freeze pre-adoption API bytes; preserve wire,
  storage, decoder limits, error handling and the existing CDK runtime.
  Narrow direct Candid/CDK to Wasm macro/runtime exceptions. Add the shared direct
  dependency-boundary check to CI/release validation with explicit exceptions.
- Adopt Shared Tooling 0.24.0 at `cab12cd3dfa4fdb34846b25397f0d72881bb9ae0`.
  Label production versus tests/tooling dependencies and packages, preserve groups
  in formatting/hooks and selected-source release checks, and document the
  remaining automated deployment-exclusion gap. The common Rust tool set no
  longer includes Candid extraction; Jobs has no extractor caller.
- Document Candid/CDK exposure per artifact and annotate direct manifest users,
  including the separate Timers executor path and actual phase-out boundaries.
- Reuse Memory's bounded sparse-image fixture loader while retaining Jobs'
  retired-format refusal and unchanged-memory checks.
- Declare direct release delivery once and reuse Shared Tooling's admission
  from Make and standalone adapters. Keep Jobs' actual hook adoption check;
  leave generic hook/frontend regression coverage at its Shared owner.
- Align consumer and delivery guides with current storage, timer and publication
  ownership; retain superseded qualification receipts as historical evidence.
  The packaged timer guide delegates dependency selection to applications and
  uses Timers' measurement re-export instead of obsolete version requirements.
- Include package and fixture provenance guides in the shared documentation-link
  check alongside the application guide.

## [0.17.0] - 2026-10-17

- **Breaking:** Consumer inspection replies carry the deadline inside
  `Status::Pending { due_ns }`; `View::due_ns` is removed. Rebuild the qualification
  canister and harness together. Stored JobRecords and delivery receipts retain
  their formats; no reset or conversion is needed for this response change.
- Keep retry configuration fields in one checked representation while preserving
  numeric validation and canonical serialization. Capture modes share one CLI
  success receipt after successful retention.

- Reuse canonical restoration timing evidence and consumer reply/deadline
  projection, preserving stored records, read-only summaries and blocked effects.
- Keep delivery ID projection within admitted queue bounds and qualify large
  identities through dispatch and outstanding-delivery inspection.
- Share native notification identity lookups and commit-before-reply projection,
  preserving evidence precedence, payload checks and full-queue deadlines.
  Select both harness test targets in one Cargo invocation.
- Adopt Shared Tooling 0.23.0 and select Jobs' IC tool bundle explicitly for
  optimized qualification. Refresh prepared tools for ICP CLI 1.7.0; fixture
  preparation uses declared companion closures and retains failure evidence.

## [0.16.2] - 2026-10-17

- Fix consumer compilation with Memory 0.48's bounded, fallible cell writes;
  preserve capacity/error ordering and propagate write refusals before success.

- Share post-commit watchdog reconciliation across management and delivery
  completion, retaining caller authorization and trap ordering.
- Simplify consumer view projection after stored identity admission and reuse
  one decoded fixture snapshot per observation phase.

## [0.16.1] - 2026-10-17

- Own Jobs' publication admission locally while retaining Shared source, version,
  tag, registry and Make observers. Keep clean-source, matching pushed-tag and
  confirmed registry-absence checks before the separately authorized upload.
- Qualify publication failure retention alongside the real release adapter;
  keep the policy matrix at its owner and retain adapter dispatch, upload-status
  and interrupted-upload reconciliation checks with substituted registry effects.
- Share cancellation/rescheduling admission while preserving clock-first errors,
  attempted-work refusal and blocked unresolved effects. Keep native fixture
  snapshot and queue limits consistent at their read/write admission boundaries.

## [0.16.0] - 2026-10-17

- **Breaking:** `JobState::Running` is a unit variant; its dispatch time is
  `JobRecord::updated_ns`. Entering Uncertain preserves that original time.
  Update Running constructors/patterns and rebuild consumers together. The old
  timestamp-bearing Running JSON shape is refused.
- Share the qualification consumer's `DeliveryPayload` between tasks and delivery
  envelopes. Construct `Task::Deliver(payload)` and use `delivery.payload`;
  envelopes/receipts now nest destination and message under `payload`. Update
  senders and receivers together; flat retained receipts/replies are refused.
- **Installation action:** Retire affected earlier installations under the
  approved hard-cut disposition and create fresh qualification state. Source
  adoption performs no reset, conversion or deployment. Current-format unresolved
  attempts remain blocked until explicit result disposition.
- Native management replies use their fields directly, preserving authorization,
  commit ordering and complete-queue deadline projection.

- Delegate optimizer producer/log admission and capture-before-removal to Build
  0.13's published helper. Keep Jobs' exact optimizer flags, full-output capture,
  failure evidence, log routing and command exit policy.
- Share receipt lookup and select notification pages before response conversion.
  Native dispatch tests retain the complete committed reply and independent
  durable-state checks.

## [0.15.0] - 2026-10-17

- Fix recovery-harness compilation with Testkit 0.51's input records and
  explicit-caller Candid methods, preserving installation, diagnostics and
  rate-limit retry settings.

- **Breaking:** Store the latest outcome and its timestamp together in
  `JobRecord::last_outcome: Option<RecordedOutcome>`. Replace uses of the removed
  `last_outcome_ns` field with the recorded outcome's `at_ns`. Earlier record
  JSON is refused; there is no compatibility reader or conversion.
- **Installation action:** Retire earlier installations under the maintainer's
  approved disposition and create fresh state before using this format. Source
  adoption does not clear storage or reset a deployed canister.

- Restore stored consumer entries directly with their payloads, removing the
  intermediate job vector while preserving admission/refusal ordering.
  Reuse consumer identity lookups and keep inspection projections on the owned
  job/payload entry.
- Serialize validated jobs through their canonical record writer. Notification,
  maintenance and canister queue snapshots persist validated state directly,
  removing temporary record projections and manual synchronization. Notification
  management replies keep each record paired with its payload during conversion.
  The persistence example and roundtrip fixtures use the same direct writer.
- Keep native notification evidence queries with the destination; the sender
  retains result classification and explicit uncertain-effect resolution.
- Share terminal and retained-retry admission checks while retaining completion,
  permanent-failure, exhausted-retry and cancellation rules. These cleanups
  preserve the current 0.15 record format and transition behavior.
- Adapt the unpublished qualification consumer to Memory 0.46's narrowed exports
  and non-cloneable memory leases. Raw Cell access is a direct development-only
  fixture dependency; the published Jobs dependency boundary is unchanged.

## [0.14.1] - 2026-10-17

- Keep stored records and validated consumer state in one snapshot shape,
  preserving JSON bytes and restore/refusal ordering.

- Use Memory's bounded fixture-cell admission for consumer installation and
  recovery, retaining the Jobs codec and restore checks
  ([#50 — Adopt fallible Memory fixture-cell setup](https://github.com/dragginzgame/ic-jobs/issues/50)).
- Decode delivery replies through the shared native/Wasm Candid decoder with
  consumer-owned limits; unreadable replies retain uncertain-effect handling.
- Keep generic validation-runner regression coverage at Shared Tooling while
  retaining the actual Jobs CI admission-stop qualification.

## [0.14.0] - 2026-10-16

- **Breaking:** Batch scheduling accepts finite iterators of job references,
  allowing consumers to keep jobs and payloads together. Returned indexes refer
  to iterator order. Pass an explicit slice or iterator for wrappers that relied
  on slice coercion, and adapt code tied to the old function signatures. Direct
  slice/vector calls remain supported. Job records and stored
  consumer payloads retain their current format.
- Keep validated jobs beside their payloads in both qualification consumers and
  use one expected delivery envelope for dispatch, inspection and completion.
- Keep generic capture-retention qualification at IC Build while retaining Jobs'
  optimizer and recovery-wrapper coverage.
- Adapt allocation requests to the selected Memory 0.45 key-only API. Its outer
  ledger format requires explicit owner disposition of earlier installations;
  this change supplies no reset, migration or old-format reader.
- Select finite reply-decoding limits for the Testkit 0.50 recovery calls and
  qualify the maximum consumer page without launching PocketIC.

- Retain bounded status/log diagnostics when recovery-canister installation fails,
  preserving explicit anonymous senders and the original installation failure
  even if diagnostic collection or output fails
  ([PR #55 — Recovery install diagnostics](https://github.com/dragginzgame/ic-jobs/pull/55)).

## [0.13.3] - 2026-10-16

- Adopt Shared Tooling 0.16.0 and qualify actual formatting/release callers,
  retaining delivered Cargo-order fixture acceptance
  ([#53 — Shared Cargo dependency-order fixture preparation](https://github.com/dragginzgame/ic-jobs/issues/53)).
- Select retained-failure assertions explicitly and use the standard cleanup-inclusive
  probes ([#61 — Select retained-failure assertions explicitly](https://github.com/dragginzgame/ic-jobs/issues/61),
  [#60 — Standard shared cleanup-inclusive fixture probes](https://github.com/dragginzgame/ic-jobs/issues/60)).
- Delegate Make-fixture copying and locked release preparation to Shared, preserving
  selected Make execution
  ([#58 — Shared Make-fixture bootstrap](https://github.com/dragginzgame/ic-jobs/issues/58),
  [#59 — Shared release preflight with selected Make preservation](https://github.com/dragginzgame/ic-jobs/issues/59)).
- Retire unselected maintenance runtime/service payloads through canonical export;
  retain maintenance guidance and task catalogs
  ([#54 — Retire optional maintenance runtime payloads](https://github.com/dragginzgame/ic-jobs/issues/54)).

- Simplify consumer read-only extraction and replace the native fixture's
  command/result protocol with typed operations and committed-deadline replies.
- Keep generic refused-store evidence qualification at the Shared owner while
  retaining Jobs workflow ordering and formatting/fixture archive integration.

- Requalify the proposed Testkit memory observer, Timers batch result and Host
  status projection against current source, retaining registry publication gates.

- Reuse Testkit tick batches, register fixture cleanup directly and share recurring
  schedule construction across scheduler, history and management tests.
- Use one checked Testkit installer substitute across consumer qualification,
  retaining admission events before metadata and server effects.

- Reuse Shared's pipe-jobserver probe in formatting qualification and consolidate
  corrupted-queue assertions and valid retry-policy serialization cases.

- Consolidate completion-refusal and blocked-management qualification while
  keeping generic release-source allowance/rename checks at the Shared owner.

- Consolidate capture-failure and file-admission fixtures while retaining exact
  evidence custody; compile only the recovery target after running capture tests.

- Inline the consumer's single-use completion lookup and share upgrade invocation
  across recovery success, rejection and selective retry checks. Recovery updates
  share caller-explicit transport while retaining structured trap failures.
- Keep standalone formatter success/cleanup qualification at the Shared owner,
  retaining Jobs workflow and archive integration checks locally.

- Reuse raw committed-byte reconstruction in native consumer fixtures, resetting
  volatile fault controls without changing restore admission.
- Exercise release adapters through the actual Jobs Makefile and keep focused
  publication wiring/recovery checks while Shared owns generic eligibility cases.

## [0.13.2] - 2026-10-16

- Derive consumer inspection/list identities from the validated job at one view
  boundary and remove unused snapshot cloning from the composition fixture.

- Keep recovery-only dependencies in the harness development graph, so the
  standalone capture tool builds without Testkit or the consumer package.
- Reuse one checked Testkit CLI selection in the recovery workflow, retaining
  admission diagnostics. Let Cargo discover core integration-test targets.

- Consolidate release-fixture installer substitutes and retain focused Jobs
  admission checks. Reuse the patch-release fixture for metadata refusals
  instead of creating another complete repository.

- Use the canonical workflow run-body extractor with a stable CI gate ID while
  retaining Jobs evidence-ordering and failure checks.
- Remove unused Clone implementations from consumer request/response types.

- Move native consumer composition tests into the unpublished consumer package
  and remove Memory/Timers development dependencies from the core package.
- Reuse the admitted occurrence offset during restore and keep focused Jobs
  release-refusal checks while Shared owns the exhaustive Make-mode matrix.

- Remove redundant cargo-sort help probing after complete tool admission.

- Reuse the consumer durable-result wrapper for dispatch and management timer
  reconciliation, preserving idle zero-write and commit-before-delivery behavior.
- Inspect delivery payloads through shared references and move owned outstanding
  messages directly, retaining exact attempt and receipt checks.
- Replace local dependency-order fixture parsing with the canonical Shared helper.

## [0.13.1] - 2026-10-15

- Update recovery trap assertions for Testkit 0.47 structured call failures,
  retaining exact canister-trap admission and full failure diagnostics.

- Remove obsolete cargo-edit setup, admission and fixtures; use Shared metadata
  rendering directly while retaining common-tool and release failure checks.
- Archive historical qualification receipts and reconcile current native acceptance
  and upstream blockers in the handoff.

- Adopt Shared Tooling 0.15.2, including the common argument-checked Rust
  preparation helper. Preserve Jobs' explicit development/MSRV versions and Wasm
  targets; qualify its release and evidence callers against the refreshed helpers.
- Move dispatched notification payloads after committing intent, and simplify
  the fixture's one-shot write-failure control without changing failure ordering.

- Share JSON round-trip restoration across job and management tests, retaining
  checked restore admission. Remove unused Serde writers from Candid-only requests.

- Reuse checked restore arithmetic and one rescheduled deadline without changing
  record admission or transitions. Qualify retained capture through IC Build's
  reported evidence path rather than its directory layout.

## [0.13.0] - 2026-10-15

- **Breaking:** `start_next` returns `(index, attempt)` instead of the three-element
  tuple. Read the updated `jobs[index].record()` for durable intent and commit it
  with the immutable payload/index before dispatching effects. Stored records,
  selection order and transition errors are unchanged.
- Admit the selected Testkit CLI once per server setup/check command, preserving
  refusal before server effects. Filter notification pages before projecting rows.

- Share delivery confirmation between the qualification canister and its lost-result
  fixture, retaining uncertain handling and the confirmed-effect trap boundary.
  Use Testkit's structured canister installation and load notification rows in one pass.

- Simplify fixed-rate restore and consumer ID validation while preserving admitted
  records and refusal behavior. Improve recovery-test trap failure diagnostics.

## [0.12.1] - 2026-10-15

- Qualify IC Memory 0.41 in the unpublished consumer. Refuse frozen 0.40 ledger
  images without writes or reset; retained installations require explicit owner
  disposition before deployment. Jobs records and the library API are unchanged.
- Update recovery-harness calls for the selected IC Testkit 0.45 API.

- Consolidate manager authorization with queue access in the qualification
  canister, and make view projection use the matched consumer job identity.

- Adopt the committed Shared Tooling 0.15.0 snapshot, including shared release
  input validation and formatting-source discovery. Existing tool pins remain
  unchanged.
- Use shared allocation for disposable fixtures and retained recovery attempts,
  completion admission and assertion probes, preserving Jobs' cleanup and evidence
  ([#47 — Adopt shared fixture allocation](https://github.com/dragginzgame/ic-jobs/issues/47),
  [#48 — Adopt shared completion admission](https://github.com/dragginzgame/ic-jobs/issues/48)).
- Use IC Host 0.18.6 for native qualification tooling.

## [0.12.0] - 2026-10-15

- **Breaking:** Require Rust 1.91 or newer for Jobs and its qualification
  consumers. Prepare Rust 1.91.0 and its Wasm target before running the minimum
  compiler checks. Native qualification uses IC Build 0.4.0.

- Reuse IC Host's fixture-directory allocation and executable-script writer in
  capture tests, retaining consumer-owned cleanup and failure evidence.

## [0.11.0] - 2026-10-15

- **Breaking:** Replace `Scheduler` with `next_due_ns(&[Job])` and
  `start_next(&mut [Job], now_ns)`. Update batch callers to the slice functions;
  ordering, transition errors and returned dispatch records are unchanged.
- **Breaking:** Remove the `timers` feature and adapter module. Applications
  depend on IC Timers directly for watchdog reconciliation and completion.
  Job records and effect-reconciliation rules are unchanged.
- Produce optimized qualification artifacts through the Rust capture tool,
  reusing IC Build's retained-capture finalization. Preserve optimizer flags,
  failed producers and diagnostics; retire producers only after successful capture.

- Separate native consumer fixtures from scenarios, reuse admitted corruption
  lookup, and centralize Testkit CLI selection in developer Make targets.
- Share native notification lookup and consumer dispatch reconciliation while
  preserving caller-specific errors, authorization and fault boundaries.
- Share active-delivery extraction and committed-envelope launching in the test
  consumer, and remove the empty host qualification library target.
- Qualify IC Memory 0.39's refusal of frozen two-slot images without stable
  writes or implicit reset. Frozen Jobs JSON still reopens on current-format
  memory; retained pre-cut installations require their owner's explicit disposition.
- Update the recovery harness to IC Testkit 0.44's direct clock and rejection
  fields, removing calls to the retired convenience APIs.

## [0.10.4] - 2026-10-14

- Centralize consumer deadline derivation and Candid reply decoding while keeping
  receipt comparison and effect disposition local. Separate current qualification
  from frozen source-bound proof inventories.
- Extract owned consumer jobs and receipts by their validated indices, avoid
  cloning duplicate delivery acknowledgements, and reconcile fault-fixture timers
  directly from committed dispatch deadlines. Preserve stored ordering, exact
  receipt checks and deliberate await/trap boundaries.

## [0.10.3] - 2026-10-14

- Centralize validated consumer job lookup and return the fresh committed deadline
  from management/completion paths, avoiding timer reconciliation reloads. Filter
  native maintenance expirations once per callback while keeping per-occurrence
  commits and recovery after partial progress.
- Reuse the deadline admitted during queue opening for lifecycle reconciliation,
  share delivery bounds between incoming and retained receipts, and make the
  packaged API guide the canonical owner of examples and timer guidance.
- Keep idle dispatch read-only after complete storage admission, and reuse the
  committed dispatch deadline during manual timer reconciliation before delivery.
- Share delivery-envelope construction while preserving owned payloads and exact
  receipt refusal checks; reuse the native notification fixture's pagination
  vector instead of collecting a second one.

## [0.10.2] - 2026-10-14

- Reuse the committed batch's next deadline for watchdog completion and avoid
  decoding/restoring validated jobs again during snapshot writes. Preserve
  collection checks, frozen storage bytes and commit-before-delivery behavior.
- Adopt Shared Tooling's explicit Rust-toolchain preparation action in CI and
  recovery, removing local setup mechanics while retaining both callers' selected
  versions, targets and recovery evidence. Reject malformed inputs before setup
  ([#46](https://github.com/dragginzgame/ic-jobs/issues/46)).

## [0.10.1] - 2026-10-14

- Adopt Shared Tooling 0.12.7 and delegate coordinated release metadata,
  rollback and selected-source checks to its canonical helper. Keep Jobs'
  direct-delivery policy and admitted tool selection.
- Use Testkit's nanosecond clock extension in recovery fixtures and retain its
  0.40 instance owner for the fixture lifetime.
- Delegate fixture exit injection and retained-evidence assertions to Shared
  Tooling's canonical probes, including function nounset and cleanup-failure
  coverage. Jobs retains its fixture roster and archive/command assertions.

## [0.10.0] - 2026-10-14

- **Breaking:** `Scheduler::start_next` returns `(index, attempt, running_record)`.
  Use the index only in the supplied, unchanged batch; persist the record and
  stable job identity as before. Update tuple destructuring in consumers. Job
  records and effect-reconciliation rules are unchanged.
- Adopt Shared Tooling 0.12.5 and its focused-check handoff policy, preserving
  tool pins, contributor-owned metadata and the explicitly selected full gates.
- Validate local lockfile identities and versions in one parse; discard optimized
  temporary producers only after successful exact capture.

- Simplify native notification fixtures to retain one validated job-state
  collection and project borrowed payloads only at commit. Keep authorization,
  exact reconciliation evidence and commit-before-delivery behavior unchanged.
- Load maintenance state once per bounded batch while preserving a separate
  commit for each occurrence and recovery after a partially committed batch.
- Remove duplicated Memory declaration/bootstrap tests already owned by
  published IC Memory, retaining Jobs' stable-cell reopen integration coverage.

## [0.9.5] - 2026-10-14

- Complete shared fixture cleanup adoption and remove the redundant private
  uncertainty-state refusal after public transition admission. Consolidate
  workflow toolchain installation in one local action, preserving per-workflow
  targets and recovery evidence; common ownership is proposed in
  [Shared #177](https://github.com/dragginzgame/shared-tooling/issues/177).

- Reuse Shared Tooling's fixture cleanup/retention helper, including premature
  exit checks for Jobs' formatting fixture. Share the validated-to-stored
  snapshot projection and route manual dispatch through the management boundary.

- Consolidate test-consumer job lookup, logical receipt keys and synchronous
  completion/timer reconciliation, retaining exact envelope checks and existing
  authorization, refusal and commit ordering.

- Retain validated jobs across test-consumer snapshot operations, removing
  repeated restore calls and dispatch's second job batch while preserving the
  stored format, bounded scheduler order and commit admission checks.
- Share borrowed delivery eligibility and identity fields between outstanding
  envelope construction and exact receipt validation without copying payloads.
- Delegate recovery polling to Testkit's bounded `tick_until` helper, preserving
  the 30-tick budget and caller-owned readiness predicates.

## [0.9.4] - 2026-10-14

- Select adjacent dependency entries dynamically in Jobs' formatting fixture,
  so adding a sorted dependency does not break the hook qualification check.

- Reject stored records whose retry history requires more elapsed time than
  their retained evidence allows, including uncertain attempts and rescheduled
  successors. Valid records and explicit effect reconciliation remain unchanged
  ([#44](https://github.com/dragginzgame/ic-jobs/issues/44)).

- Adopt Shared Tooling 0.12.0: release candidates are numbered and top-positioned;
  current delivered implementations and their required CI matrix own routine
  issue acceptance. Contributor tooling changes leave Jobs’ API/storage unchanged.

- Delegate recovery-artifact capture and explicit retention to published IC Build,
  preserving frozen inputs, variant evidence and failed captures without changing
  live qualification authority ([#42](https://github.com/dragginzgame/ic-jobs/issues/42)).

## [0.9.3] - 2026-10-14

- Adopt Shared Tooling 0.11.4 and its canonical publication admission; retain
  Jobs’ package/features, upload authority and release metadata adapter
  ([#42](https://github.com/dragginzgame/ic-jobs/issues/42)).
- Qualify the selected IC Host 0.17.7 graph in the unpublished recovery harness;
  core scheduling and stored job records are unchanged.

## [0.9.2] - 2026-10-13

- Carry validated completion timestamps directly through recurring restore checks,
  retaining stored-record admission and refusal behavior.
- Reuse admitted notification state across management commands and deadline
  replies, keeping authorization and commit-before-delivery ordering.

## [0.9.1] - 2026-10-13

- Converge test-consumer management authorization, stable mutation and timer
  reconciliation through one synchronous boundary.
- Reuse the validated job's active attempt for delivery completion, retaining
  exact receipt matching and refusal ordering.
- Derive test-consumer deadline summaries without a temporary mutable job batch,
  retaining complete snapshot admission and checked job restoration.
- Compare completion envelopes and notification receipt evidence using borrowed
  payloads; notification cancellation carries its admitted state through commit
  and reply without repeated snapshot loads.

## [0.9.0] - 2026-10-13

- **Breaking:** adopt IC Timers 0.22 and its Metrics 0.9 measurement-summary
  identity. Align direct timer dependencies to 0.22; use the Timers summary
  re-export or Metrics 0.9 for exchanged measurements, updating Metrics overflow
  accessors and error matches. Job records and retained storage need no reset
  ([#41](https://github.com/dragginzgame/ic-jobs/issues/41)).

- Remove duplicate standalone formatter checks now owned by Shared Tooling,
  retaining Jobs’ workflow and failure-evidence integration checks
  ([#42](https://github.com/dragginzgame/ic-jobs/issues/42)).

- Simplify dispatch attempt increments using validated Pending-state budget
  guarantees, retaining sequence overflow and error atomicity
  ([#43](https://github.com/dragginzgame/ic-jobs/issues/43)).
- Qualify recovery against one frozen original artifact and remove repeated
  fixture authorization; keep variant hashes/logs and create authorization
  ([#42](https://github.com/dragginzgame/ic-jobs/issues/42)).

## [0.8.7] - 2026-10-13

- Adopt Shared Tooling 0.10: common setup, formatting and failure evidence use
  snapshot-owned tool catalogs. Host and Rust overrides are independent;
  tool versions and retained installations are unchanged
  ([#40](https://github.com/dragginzgame/ic-jobs/issues/40),
  [Shared #153](https://github.com/dragginzgame/shared-tooling/issues/153)).
- Retain filesystem release-lock errors and reject line breaks in LOC report
  paths before selecting a checkout
  ([Shared #152](https://github.com/dragginzgame/shared-tooling/issues/152),
  [#151](https://github.com/dragginzgame/shared-tooling/issues/151)).
- Separate Running and Uncertain restore history checks, retaining common
  timestamp admission and blocked-effect semantics
  ([#39](https://github.com/dragginzgame/ic-jobs/issues/39)).

## [0.8.6] - 2026-10-13

- Adopt Shared Tooling 0.9.15: reject maintenance paths containing line breaks
  before tool probes or state writes; retain failed-attempt log recovery guidance
  ([#37](https://github.com/dragginzgame/ic-jobs/issues/37),
  [Shared #150](https://github.com/dragginzgame/shared-tooling/issues/150),
  [#149](https://github.com/dragginzgame/shared-tooling/issues/149)).
- Adopt Testkit 0.36 and its IC Host 0.16 dependencies in the unpublished recovery
  harness. Refresh the lock-selected Testkit CLI; prior builds remain retained
  ([#38](https://github.com/dragginzgame/ic-jobs/issues/38)).

## [0.8.5] - 2026-10-13

- Remove the repeated one-shot restore deadline check, retaining common deadline
  admission and the occurrence-origin guard
  ([#36](https://github.com/dragginzgame/ic-jobs/issues/36)).
- Adopt Shared Tooling 0.9.13: preserve filesystem lock and Cargo source-download
  failure statuses, and clarify locked dependency-cache preparation. Refresh
  prepared common and release Rust tools; existing builds remain retained
  ([#35](https://github.com/dragginzgame/ic-jobs/issues/35),
  [Shared #145](https://github.com/dragginzgame/shared-tooling/issues/145),
  [#146](https://github.com/dragginzgame/shared-tooling/issues/146),
  [#147](https://github.com/dragginzgame/shared-tooling/issues/147)).

## [0.8.4] - 2026-10-13

- Adopt IC Memory 0.36 in test consumers using explicit owned allocation requests.
  Preserve retained storage keys, job records and consumer-owned bootstrap
  ([#34](https://github.com/dragginzgame/ic-jobs/issues/34)).

## [0.8.3] - 2026-10-13

- Simplify stored-record admission by removing checks already implied by retained
  invariants, preserving accepted records and rejection behavior
  ([#32](https://github.com/dragginzgame/ic-jobs/issues/32)).
- Complete test-consumer deliveries from one validated snapshot, preserving exact
  receipt checks, refusal ordering and blocked effects
  ([#33](https://github.com/dragginzgame/ic-jobs/issues/33)).

## [0.8.2] - 2026-10-13

- Fix retry-policy decoding for Serde formats that retain struct names: the checked
  reader now accepts the same `RetryPolicy` representation emitted by the writer.
  Preserve checked configuration and existing JSON/stable-memory bytes
  ([#30](https://github.com/dragginzgame/ic-jobs/issues/30)).

## [0.8.1] - 2026-10-12

- Adopt Shared Tooling 0.9.7: reject non-boolean Cargo offline configuration
  before tool setup or admission while respecting explicit environment overrides.
  Refresh common and release Rust tools to select the new build policy; existing
  builds remain retained ([#29](https://github.com/dragginzgame/ic-jobs/issues/29),
  [Shared #140](https://github.com/dragginzgame/shared-tooling/issues/140)).

## [0.8.0] - 2026-10-12

- **Breaking:** require nonzero recurring intervals and checked, immutable retry
  configuration. `Job::new` now returns `Job` directly; update constructors and
  exhaustive error matches. Malformed configuration fails decoding, while valid
  retained records keep their existing serialized shape and require no reset
  ([#28](https://github.com/dragginzgame/ic-jobs/issues/28)).
- **Breaking:** select IC Timers 0.21 for the optional adapter and its Metrics
  0.8 summary type. Align direct dependencies; failed first reconciliation now
  leaves the registration slot empty so retry can install its supplied callback.
- Select Testkit 0.35 for the native recovery harness. Jobs uses its maintained
  PocketIC API and needs no dependency on the artifact APIs moved to IC Build.

## [0.7.5] - 2026-10-12

- Adopt Shared Tooling 0.9.4: preserve installer failures while releasing locks,
  retry transient asset downloads within fixed limits, and reject malformed
  checksum manifests, consumer paths and Git identities
  ([#27](https://github.com/dragginzgame/ic-jobs/issues/27),
  [Shared #138](https://github.com/dragginzgame/shared-tooling/issues/138),
  [Shared #133](https://github.com/dragginzgame/shared-tooling/issues/133)).
- Reject stored dispatch sequences that exceed the retained occurrence retry
  budget, including after schedule replacement. Preserve valid histories and
  block the entire consumer queue before effects when counts are contradictory
  ([#26](https://github.com/dragginzgame/ic-jobs/issues/26)).

## [0.7.4] - 2026-10-12

- Adopt Shared Tooling 0.9.0: snapshot refresh and verification share one
  validated manifest reader, rejecting NUL bytes and emitting no records for
  malformed input. Existing snapshots need no conversion
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #133](https://github.com/dragginzgame/shared-tooling/issues/133)).
- Support structured workspace LOC reports through `cloc.sh --json`, alongside
  the existing human-readable output.

## [0.7.3] - 2026-10-12

- Adopt Shared Tooling 0.8.3: reject malformed snapshot fields, retain build
  identity diagnostics without following redirected evidence paths, and admit
  formatters against the selected Rust catalog. Include the read-only Cargo
  tool freshness guide and complete Make snapshot companions
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #133](https://github.com/dragginzgame/shared-tooling/issues/133),
  [Shared #134](https://github.com/dragginzgame/shared-tooling/issues/134),
  [Shared #135](https://github.com/dragginzgame/shared-tooling/issues/135),
  [Shared #136](https://github.com/dragginzgame/shared-tooling/issues/136),
  [Shared #131](https://github.com/dragginzgame/shared-tooling/issues/131)).
- Expand durable-record qualification across mixed management, retry and
  uncertainty histories and deadline extremes. Check serialization/restore
  agreement and unchanged records after rejected transitions
  ([#23](https://github.com/dragginzgame/ic-jobs/issues/23),
  [#24](https://github.com/dragginzgame/ic-jobs/issues/24)).
- Allow 45 minutes for native CI after cold Intel qualification reached the
  previous 30-minute limit during compilation
  ([#25](https://github.com/dragginzgame/ic-jobs/issues/25)).

## [0.7.2] - 2026-10-12

- Adopt Shared Tooling 0.8.0: snapshot refresh derives required companions from
  recorded entrypoints and automatically retires verified obsolete payloads.
  Keep Jobs' existing selection; no files are retired by this refresh
  ([Shared #131](https://github.com/dragginzgame/shared-tooling/issues/131)).
- Use the macro-safe cargo-sort-derives 0.14.1 pin.
  Refresh common Rust tools with `make install-rust-tools`, then
  `make rust-tools-check` ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #113](https://github.com/dragginzgame/shared-tooling/issues/113)).
- Refresh the selected Host 0.14.5, Memory 0.35.14, Metrics 0.7.1 and Testkit
  0.34.2 patches. Prepare/check the separately selected Testkit CLI before
  recovery qualification.
- Correct the timer adapter guides to the Timers 0.19 dependency already
  selected by Jobs 0.7.1. Applications sharing its public timer types must use
  the same Timers family; the job storage contract is unchanged.

## [0.7.1] - 2026-10-12

- Adopt Shared Tooling 0.6.5 and its common Make entrypoint for setup, formatting,
  hooks, snapshot verification and releases. Admit prepared Rust tools during
  staged formatting and stop later validation stages if selected inputs change
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #130](https://github.com/dragginzgame/shared-tooling/issues/130),
  [Shared #129](https://github.com/dragginzgame/shared-tooling/issues/129)).
- Qualify release recovery through the shared recipes without duplicate recipe
  warnings or suppressing diagnostics
  ([Shared #128](https://github.com/dragginzgame/shared-tooling/issues/128)).
- Refresh the locked Host crates to 0.14.3 through Testkit; published Rust
  sources, dependencies and the Rust 1.88 floor match 0.14.2.

## [0.7.0] - 2026-10-12

- **Breaking:** Select IC Timers 0.18 for the optional timer adapter. Consumers
  using its exposed timer types must update their IC Timers dependency to 0.18;
  direct timer registrations now require an explicit initial schedule. Jobs'
  watchdog reconciliation and stored job format are unchanged.
- Refresh the locked Host crates to 0.14.2 through the native Testkit harness;
  their published Rust sources, dependencies and Rust 1.88 floor are unchanged.

- Reject contradictory retained timing when restoring cancelled jobs or running
  attempts: retry backoff and recurring successor deadlines remain validated
  after management and dispatch. Invalid stable records block the complete
  consumer queue before effects ([#24](https://github.com/dragginzgame/ic-jobs/issues/24)).

## [0.6.1] - 2026-10-11

- Refresh the locked Timers selection to 0.17.14 and Metrics to 0.6.6. Their
  published Rust sources, dependency declarations and compiler requirements match
  the preceding patches; Jobs runtime, stored contracts and Rust 1.88 are unchanged.

## [0.6.0] - 2026-10-11

- **Breaking:** Add durable pause/resume and schedule replacement for unstarted
  occurrences. Update exhaustive state/error matches and stored JobRecord codecs
  for the new paused state, cadence origin and independent outcome timestamp.
  Retire/reset the approved disposable 0.5 test installations before creating
  fresh 0.6 consumer state; there is no old-format reader or migration path
  ([#23](https://github.com/dragginzgame/ic-jobs/issues/23)).
- Preserve deadlines, retry evidence and effect identities through management;
  running/uncertain effects remain blocked. Extend the existing consumer and
  qualification harness to management persistence, wakeup changes and rollback
  ([#23](https://github.com/dragginzgame/ic-jobs/issues/23)).

## [0.5.13] - 2026-10-11

- Adopt Shared Tooling 0.6.3: normal direct-release commands restart superseded
  preparation from committed metadata, preserve the previous plan and rerun
  complete validation
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #126](https://github.com/dragginzgame/shared-tooling/issues/126)).
- Refresh the locked graph to Timers 0.17.12 and Testkit 0.33.8. The Timers
  patch corrects its private native test provider; Jobs' production timer
  behavior is unchanged
  ([Timers #45](https://github.com/dragginzgame/ic-timers/issues/45)). Rerun
  `make install-testkit-tools` and `make testkit-tools-check` before recovery
  qualification.

## [0.5.12] - 2026-10-11

- Adopt Shared Tooling 0.6.2 for clearer interrupted-release source conflicts
  and guarded maintainer recovery guidance
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #126](https://github.com/dragginzgame/shared-tooling/issues/126)).
- Refresh the locked graph to Memory 0.35.10, Timers 0.17.11 and Host 0.13.5;
  retain the existing library contracts and Rust 1.88 floor.

## [0.5.11] - 2026-10-11

- Adopt Shared Tooling 0.6.1 so Cargo-tool setup stops on lock, validation and
  activation failures, preserves the original status and retains failed attempts.
  Refresh selections with `make install-tools` and `make tools-check`; prepare
  the selected Testkit CLI separately before recovery qualification
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #120](https://github.com/dragginzgame/shared-tooling/issues/120)).

## [0.5.10] - 2026-10-11

- Adopt Shared Tooling 0.6.0: share verified Cargo-built tools, including Jobs'
  pinned cargo-edit, and retain safe local failure evidence when shared-store
  admission refuses. Rerun `make install-tools` and `make tools-check`; prepare
  the selected Testkit CLI separately before recovery qualification
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21),
  [Shared #121](https://github.com/dragginzgame/shared-tooling/issues/121)).
- Keep release-tool qualification compatible with common Rust setup preflight
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21)).
- Align the consumer memory guide with the selected IC Memory 0.35 family and
  distinguish completed recovery evidence from qualification of the current
  watchdog scenario ([#22](https://github.com/dragginzgame/ic-jobs/issues/22)).

## [0.5.9] - 2026-10-11

- Adopt Shared Tooling 0.5.0 for shared verified host/IC bundles, checkout-local
  common Rust-tool slots, coordinated hook/PATH selection and retained
  installation evidence. Contributors should rerun `make install-tools` and
  `make tools-check`, and use `.tools/rust/common/bin` on their shell PATH
  ([#21](https://github.com/dragginzgame/ic-jobs/issues/21)).
- Keep formatting-adoption checks working with relative temporary directories
  ([Shared #116](https://github.com/dragginzgame/shared-tooling/issues/116)).
- Contain known local counter failures in the test consumer: fail the exact job
  without rolling back unrelated bounded work or repeatedly trapping its watchdog
  ([#20](https://github.com/dragginzgame/ic-jobs/issues/20)).
- Add native batch-order/reconstruction coverage and a real-watchdog rollback,
  surviving-wakeup and subsequent-progress qualification case
  ([#20](https://github.com/dragginzgame/ic-jobs/issues/20)).
- Clarify catch-up backlog costs and the distinction between timer callback
  completion and individual job outcomes. Empty consumer batches report no work.

## [0.5.8] - 2026-10-11

- Correct timer composition guidance to IC Timers 0.17 and its canonical
  measurement-summary re-export
  ([#19](https://github.com/dragginzgame/ic-jobs/issues/19)). Select Timers 0.17.8
  for the optional adapter and recovery consumer.

## [0.5.7] - 2026-10-11

- Align the recovery harness and lock-selected CLI on Testkit 0.33.2. Rerun
  `make install-testkit-tools` before live recovery qualification
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).

## [0.5.6] - 2026-10-11

- Adopt Shared Tooling 0.3.11: Rust-tool setup and checks use the selected
  consumer's Cargo configuration; retained validation summaries keep early
  fatal, LLVM and missing-file diagnostics
  ([Shared #101](https://github.com/dragginzgame/shared-tooling/issues/101),
  [Shared #110](https://github.com/dragginzgame/shared-tooling/issues/110)).
- Adopt the latest Shared release-command check, supporting relative and
  symlinked temporary directories without losing nested Make checks
  ([Shared #112](https://github.com/dragginzgame/shared-tooling/issues/112)).
- Include failed native tooling fixtures and their inner logs in CI artifacts
  ([#18](https://github.com/dragginzgame/ic-jobs/issues/18)).

## [0.5.5] - 2026-10-10

- Qualify the actual pre-commit hook and Jobs formatting in native CI, including
  failed Git observations, manifest sorting and preservation of staged/working
  files ([#17](https://github.com/dragginzgame/ic-jobs/issues/17)).

## [0.5.4] - 2026-10-10

- Select the Testkit CLI directly from `Cargo.lock` for setup, offline admission
  and recovery qualification; remove its duplicate version pin. Rerun
  `make install-testkit-tools` after changing the locked Testkit version
  ([#17](https://github.com/dragginzgame/ic-jobs/issues/17)).
- Adopt Shared Tooling 0.3.6: pre-commit stops on failed Git tree observations,
  preserving staging and working files
  ([shared #106](https://github.com/dragginzgame/shared-tooling/issues/106)).
- Adopt Shared Tooling 0.3.7: reject failed Cargo-tool activation, release
  metadata, Wasm-input and fixture checks on Bash 3.2, retaining failed evidence
  ([shared #107](https://github.com/dragginzgame/shared-tooling/issues/107),
  [#16](https://github.com/dragginzgame/ic-jobs/issues/16)).
- Cancel superseded CI runs while preserving the host matrix on the latest run
  ([shared #108](https://github.com/dragginzgame/shared-tooling/issues/108)).

- Keep only the newest CI run per workflow and branch or PR, cancelling older
  queued and running checks while retaining the existing host matrix and gates
  ([Shared #108](https://github.com/dragginzgame/shared-tooling/issues/108)).

## [0.5.3] - 2026-10-10

- Align the recovery harness and selected CLI on Testkit 0.32.2 for the updated
  consumer dependency graph. Rerun `make install-testkit-tools` before live
  qualification; Job APIs and stored records remain unchanged
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).

## [0.5.2] - 2026-10-10

- Align the recovery harness and its selected CLI on Testkit 0.32.1. Rerun
  `make install-testkit-tools` before live qualification
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).
- Adopt Shared Tooling 0.3.4: preserve Cargo's parallel-build coordination and
  reject unavailable setup prerequisites before downloads. Incomplete Bash 3.2
  validation and metadata checks fail while retaining originals and evidence
  ([#16](https://github.com/dragginzgame/ic-jobs/issues/16)).
- Select Binaryen 133 in the common tools; rerun `make install-ic-tools`.
  Explicit `test-canister-optimized` and the manual recovery workflow test the
  original and `-O3`, `-Os`, `-Oz` canisters with retained per-variant evidence.
  Normal canister builds remain unchanged
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).

## [0.5.1] - 2026-10-10

- Add a manually triggered canister-recovery workflow for Linux and both macOS
  architectures. Retain source/graph selections, the tested Wasm, test output and
  PocketIC logs; compile the harness before starting the bounded server lifetime
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).

## [0.5.0] - 2026-10-10

- **Breaking tooling:** Adopt Shared Tooling 0.3.0. `install-tools` prepares the
  complete host, IC and Rust toolsets in order, then selected cargo-edit;
  `tools-check` admits the same selections offline. Host setup always includes
  ripgrep with PCRE2 and cloc; the optional selection flags are removed. Rerun
  `make install-tools` before validation. CI and admitted release preflight use
  the same complete setup/check path
  ([#15](https://github.com/dragginzgame/ic-jobs/issues/15)).
- Align the explicit recovery harness's Testkit CLI with its selected 0.31.0
  library. PocketIC provisioning remains a separate explicit setup step.
- Require actual canister-trap rejections in recovery checks and use Testkit's
  bounded installation-rate-limit retry after controller snapshot loading
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).
- Prepare the development Wasm target in CI for the consumer's Clippy check.

## [0.4.7] - 2026-10-10

- Add an unpublished canister consumer with authorized task management, stable
  queue storage, timer reconstruction and independent delivery receipts. Add an
  explicit PocketIC harness for upgrade, rollback and lost-result qualification;
  ordinary CI builds it without launching the server
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).
- Include app-owned workspace members in committed release-metadata checks.

## [0.4.6] - 2026-10-10

- Adopt IC Memory 0.34 in the test consumers, using one host-owned allocation
  pool and key-only opens. Exercise stable-cell cold reopens with retained job
  payloads, deadlines and blocked effects
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11),
  [Memory #44](https://github.com/dragginzgame/ic-memory/issues/44)).
- Adopt Shared Tooling 0.2.13. Prepare the selected cargo-edit installation after
  release source/cache admission, preserve previous tool installations, and check
  tools before parallel validation dispatch
  ([#14](https://github.com/dragginzgame/ic-jobs/issues/14)).

## [0.4.5] - 2026-10-10

- Adopt Shared Tooling 0.2.12 and remove the temporary Make flag parser. Canonical
  admission rejects hidden unsafe modes and overwritten invocation evidence
  ([#12](https://github.com/dragginzgame/ic-jobs/issues/12),
  [shared #30](https://github.com/dragginzgame/shared-tooling/issues/30)).
- Keep formatting output concise while retaining full failure diagnostics locally
  and collecting them in CI artifacts
  ([#13](https://github.com/dragginzgame/ic-jobs/issues/13)).
- Reject line breaks in snapshot directory paths before export or verification
  can select a different checkout
  ([shared #95](https://github.com/dragginzgame/shared-tooling/issues/95)).

## [0.4.4] - 2026-10-10

- Exercise authorized create, inspect, bounded list, cancel and due-dispatch
  management flows in the notification consumer, deriving wakeup decisions from
  committed records and preserving blocked effects after reconstruction
  ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).
- Add IC Memory 0.33 to the test consumers for composable, key-only storage
  requests and collision rejection. Allocation and stable-store bootstrap remain
  host-owned ([#11](https://github.com/dragginzgame/ic-jobs/issues/11)).
- Refuse unsafe Make modes even when command-line `MAKEFLAGS` hides them. Adopt
  Shared Tooling 0.2.9 for selected-snapshot admission and recursive Make commands
  ([#12](https://github.com/dragginzgame/ic-jobs/issues/12),
  [shared #30](https://github.com/dragginzgame/shared-tooling/issues/30)).

## [0.4.3] - 2026-10-09

- Adopt Shared Tooling 0.2.7 so release and formatting commands refuse Make's
  ignore-errors, dry-run, touch and question modes before recipes run. Keep
  release adapter checks isolated from inherited tooling roots
  ([#10](https://github.com/dragginzgame/ic-jobs/issues/10),
  [shared #30](https://github.com/dragginzgame/shared-tooling/issues/30),
  [shared #7](https://github.com/dragginzgame/shared-tooling/issues/7)).

## [0.4.2] - 2026-10-09

- Adopt Shared Tooling 0.2.6 for release entrypoints and workspace formatting,
  replacing duplicated Make recipes while retaining direct-only release admission
  and explicitly prepared formatter requirements
  ([#10](https://github.com/dragginzgame/ic-jobs/issues/10)).
- Refresh the locked optional timer dependency to IC Timers 0.16.2 within the
  existing 0.16 requirement. Timer library source, Job records and scheduling
  behavior are unchanged.

## [0.4.1] - 2026-10-09

- Adopt committed Shared Tooling 0.2.5 to preserve literal hook selections and
  support hook setup/formatting in checkouts ending in newlines
  ([shared #89](https://github.com/dragginzgame/shared-tooling/issues/89)).
- Extend the test consumers with exact rejection evidence for lost replies,
  retry-budget exhaustion and recovery after a partially committed maintenance
  batch. Confirmed delivery receipts remain authoritative for duplicate requests.

## [0.4.0] - 2026-10-09

- **Breaking:** The optional adapter now uses IC Timers 0.16 and its IC Metrics
  0.3 package identity. Align all timer owners in a canister to IC Timers 0.16;
  use Metrics 0.3 or `ic_timers::MeasurementSummary` when exchanging telemetry.
  Job records and scheduling behavior remain unchanged.

## [0.3.1] - 2026-10-09

- Add notification and cache-maintenance test consumers for persistence failures,
  lost-reply reconciliation, safe retries, shared wakeup selection and bounded work. Expose
  `make test-consumers` for core/timers qualification and include it in configured CI.
- Install and verify the final selected IC tool even when its pin file has no
  trailing newline, preserving the original pins and version checks. Adopt the
  reviewed Shared Tooling 0.2.2 snapshot
  ([#9](https://github.com/dragginzgame/ic-jobs/issues/9),
  [shared #87](https://github.com/dragginzgame/shared-tooling/issues/87)).

## [0.3.0] - 2026-10-09

- **Breaking:** The shared IC setup now installs five tools; PocketIC provisioning
  belongs to IC Testkit. Explicitly rerun `make install-ic-tools`, then
  `make ic-tools-check`, to replace a six-tool installation. Previous bundles
  and receipts are retained; scheduler APIs and stored jobs are unchanged
  ([#7](https://github.com/dragginzgame/ic-jobs/issues/7),
  [shared #76](https://github.com/dragginzgame/shared-tooling/issues/76)).
- Keep prepared checkout-local tools available to isolated pre-commit formatting
  and reject multi-document dependency-pinning exception catalogs. Adopt the
  committed Shared Tooling fixes without adding a Cargo-tool mode
  ([#6](https://github.com/dragginzgame/ic-jobs/issues/6)).
- Refresh the locked optional-timers graph to IC Metrics 0.2.20 within the
  existing compatible dependency requirements.
- Remove the unused PR release transport from the selected snapshot, retaining
  Jobs' direct release and recovery workflow
  ([#8](https://github.com/dragginzgame/ic-jobs/issues/8)).

## [0.2.3] - 2026-10-09

- Preserve checkout paths ending in newlines when publishing, and qualify
  publication refusals and the intended upload invocation from unusual paths
  ([#4](https://github.com/dragginzgame/ic-jobs/issues/4)).
- Prepare the selected locked dependency cache during release preflight before
  offline validation, respecting explicit offline settings and preserving
  metadata on fetch failure. Adopt Shared Tooling 0.1.35
  ([#5](https://github.com/dragginzgame/ic-jobs/issues/5)).
- Refresh the locked optional-timers graph to IC Timers 0.14.23 and IC Metrics
  0.2.18 within the existing compatible dependency requirements.

## [0.2.2] - 2026-10-09

- Keep fleet reporting in Shared Tooling while retaining local tool setup,
  offline checks and workspace LOC reporting. Adopt the reviewed 0.1.34 snapshot
  and remove the unused consumer reporter
  ([#1](https://github.com/dragginzgame/ic-jobs/issues/1)).
- Reuse verified IC tool bundles when only pin comments or row order change,
  preserving checksums, version admission and original installation receipts
  ([shared #79](https://github.com/dragginzgame/shared-tooling/issues/79)).
- Preserve CI qualification for each pushed commit, including queued native
  checks; superseded revisions of the same pull request may still be cancelled
  ([#3](https://github.com/dragginzgame/ic-jobs/issues/3)).

## [0.2.1] - 2026-10-09

- Reject contradictory occurrence records during restoration, including successful
  one-shot work made pending again, shifted first-occurrence deadlines, retries
  before their scheduled time, and recurring deadlines or cancellation times
  inconsistent with preceding completion. Preserve valid recovery, CatchUp
  and cancellation behavior
  ([#2](https://github.com/dragginzgame/ic-jobs/issues/2)).

## [0.2.0] - 2026-10-09

- **Breaking:** Require Rust 1.88 for the package, with or without the optional
  timers feature. Upgrade older consumer toolchains; native and Wasm minimum
  checks now use the same floor.
- Report all staged, unstaged and untracked paths refused by release or
  publication preflight, preserving source and index bytes. Adopt Shared Tooling
  0.1.30 and its PocketIC 16.1.0 setup pin
  ([shared #74](https://github.com/dragginzgame/shared-tooling/issues/74),
  [shared #76](https://github.com/dragginzgame/shared-tooling/issues/76)).

## [0.1.2] - 2026-10-09

- Add a bounded batch scheduler that selects the earliest due job and returns
  its checked dispatch intent for application-owned persistence and execution.
- Qualify releases against the current workspace version so release tooling
  checks continue to work after the initial release.

## [0.1.1] - 2026-10-08

- Provide package verification, publication dry runs and a separate crates.io
  upload command for clean releases with matching pushed annotated tags.
- Include regular README and license files in the crate package, supporting
  packaged rustdoc and the repository's pre-commit formatting snapshot.
- Keep release fixture checks independent of the shell locale, and report the
  failing source line when a tooling check fails.

First tagged release, adding delivery tooling and fixes to the initial 0.1.0
implementation described below.

## [0.1.0]

- Introduce durable job metadata with one-shot and recurring schedules, bounded
  retries, pending cancellation and explicit uncertain-effect reconciliation.
- Add validated record reconstruction, a persistence example and an optional
  IC Timers watchdog adapter.

Initial implementation snapshot; no separate `v0.1.0` tag was created.
