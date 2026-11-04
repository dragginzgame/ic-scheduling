# IC Timers changelog

This ledger covers `ic-timers`, `ic-timers-pocketic`, `ic-timers-runtime-probe`,
`ic-timers-size-probe` and `ic-timers-test-protocol`. Jobs uses the
[Jobs ledger](../jobs/CHANGELOG.md). The shared Scheduling release through 0.33.1
is retained once in [coordinated history](../history.md).
The older Timers entries below are preserved as imported provenance; an undated
entry alone does not establish publication.

## [0.40.2] - Unreleased

### 🧹 Cleanup

- Converge ordinary scheduling requests, pending-command arbitration and completion
  planning in one control owner. Preserve deadline selection, command precedence,
  failure transitions and callback observations.
- Group registry entry payloads, provider-handle mechanics and snapshot projection
  in a private child module while retaining registry admission and effect ordering.
- Declare named counter storage, zero values and observation getters together;
  preserve public observations, explicit event classification and Metrics arithmetic.

### 📦 Dependencies

- Qualify the selected patch graph with Logging 0.24.6, Metrics 0.29.1, Storage
  0.66.2 and Host Artifacts/FS/Process 0.74.3. Preserve the Runtime cohort and
  consumer-owned reporting, storage and recovery contracts.

## [0.40.1] - 2026-11-04

### 🧹 Cleanup

- Move inline unit tests and native test-provider/fault support into ordinary
  `tests.rs` modules. Preserve timer authority, test behavior and production
  visibility; use the standard-library Wasm intrinsics in the existing probes.

### 🛠️ Tooling

- Check retained release state before work and around adapter steps. Refuse
  oversized or stale bulk evidence and nested build/cache trees while preserving
  recovery files and the adapter's original failure status.

### 🧪 Testing

- Adopt the reviewed Shared Tooling 0.29.4 snapshot and its test-layout rules,
  validation evidence fixes and native dependency-owner policy. Preserve the
  complete release plan and existing installed-runtime gates.
- Copy the classifier's declared companion closure into CI-selection fixtures,
  preserving actual workflow routing with the new shared parser dependency.
- Organize runtime and registry tests into behavior modules for focused Cargo
  selection, keeping shared fixtures and every existing case.
- Cover cancel/rearm and callback execution through retained owners at full
  registration capacity for all three policies, preserving claim identity and
  watchdog scheduler/work separation.

### 📚 Documentation

- Document all six registration/reconciliation capabilities for consumer custody
  checks and add a 0.39 → 0.40 adoption checklist covering Metrics types, composed
  dependency graphs and consumer-owned native/installed qualification.

## [0.40.0] - 2026-11-04

### ⚠️ Breaking

- Adopt Metrics 0.29 for public measurement recorders. Align direct Metrics
  dependencies; recorder Serde traits, mean/delta/interval queries and public
  reconstruction constructors are removed. Report owners retain their DTOs,
  codecs, validation and arithmetic. Timer recording, overflow quality and
  scheduling behavior are unchanged.

### 📦 Dependencies

- Select Testkit 0.84.1 with Platform 0.21's external-protocol adapters after the
  separate PocketIC client's retirement. Keep Testkit-owned clients, Runtime
  0.19, Types 0.18 and server 16.1.0; qualify the selected TOML tooling updates
  through the metadata, release and archive checks.

## [0.39.1] - 2026-11-03

### 📦 Dependencies

- Adopt Testkit 0.84's maintained PocketIC client in native qualification and
  remove the separate Platform client dependency. Keep the deployed timer API,
  Runtime guard, probe protocol and explicitly selected installed gates unchanged.
- Select Build 0.74.2, Wire/Derive 0.16.1 and Endpoint 0.6.1 in the private artifact
  and probe graph. The published timer library's dependency requirements are unchanged.

### 📚 Documentation

- Update consumer examples and integration guidance for published Timers 0.39.0,
  Metrics 0.28 and Runtime 0.19. Clarify that compatible dependency requirements
  must resolve one shared timer provider and preserve consumer-owned lifecycle
  and recovery policy.

## [0.39.0] - 2026-11-03

### ⚠️ Breaking

- Adopt Metrics 0.28 for the public measurement types and Runtime 0.19 with its
  exact executor guard. Consumers exchanging measurement values must update
  direct Metrics dependencies; applications must align all Runtime/Endpoint
  suppliers before adopting this Timers release.

### 📦 Dependencies

- Qualify Types 0.18, Wire/Derive 0.16, Endpoint 0.6, Storage 0.66 and Testkit
  0.83 in the combined consumer. Native qualification selects PocketIC client
  and facade 0.20 to match Testkit's public client type.
- Adopt the reviewed Shared Tooling 0.28.2 snapshot, including complete owned-tree
  inventory admission and updated release-group checks. Retain explicit group
  versions and runtime safety requirements.

### 🧹 Cleanup

- Carry callback or registration authority as one private value, deriving the
  claim from it while retaining running-attempt validation and failure ordering.
- Use shared Wire Endpoint bindings in the qualification probes and a typed
  runner command definition. Preserve method identities, tuple payloads and
  explicit byte, node and depth budgets; malformed probe diagnostics now come
  directly from Wire.

### 🧪 Testing

- Delegate baseline, roster and run-evidence admission to Shared Tooling. Use one
  ordered policy for full CI and selected releases; preserve complete common CI,
  standalone Timers gates and exact source/metadata recovery. Keep historical
  receipts with their original adapters.
  [#88 — Consolidate Jobs/Timers plan evidence](https://github.com/dragginzgame/ic-scheduling/issues/88).

- Handle native client failures explicitly during clock/cycle observation and
  simulated progression. Preserve typed canister rejection assertions; report
  demo transport errors through its existing error path without automatic retry.
- Adopt explicit group validation plans and durable stage evidence. Timers
  releases retain their complete installed watchdog/cohort gate; repository
  CI and combined-consumer checks remain complete.
- Share native provider-fault selection and installed counter-overflow checks
  while preserving scenario-specific outcomes and exact measurement admission.
- Check probe endpoint identities and budgets at the provider boundary, with
  rejection fixtures for missing protocol declarations and bare entrypoints.

## [0.38.1] - 2026-11-02

### 🛠️ Maintenance

- Keep watchdog control and pure schedule/completion decisions in a private
  registry module. Claim authority, provider-handle ownership and public timer
  behavior remain unchanged.
- Correct consumer guidance for the Runtime 0.18, Metrics 0.27 and Wire 0.15
  cohort. Explain why applications retaining Runtime 0.17 cannot update Timers
  alone, and remove the stale patch-specific graph command.

### 🧪 Testing

- Extend generated command interleavings with provider admission and binding
  failures, checking declaration retirement, stale authority and handle cleanup.
  Cover binding failure after suspended ordinary work completes as well as
  admission failure and abandonment.

## [0.38.0] - 2026-11-02

### ⚠️ Breaking

- Align the publicly re-exported measurement types with Metrics 0.27. Consumers
  exchanging those types with a direct Metrics dependency must select 0.27 too;
  optional native report metadata uses Wire 0.15. Arithmetic remains unchanged.
- Select Runtime 0.18 and its exact executor guard. Applications must align
  other executor and endpoint suppliers with that generation; callback,
  cancellation and deployed scheduling behavior are unchanged.

### 📦 Dependencies

- Adopt Wire 0.15 and Testkit 0.80 with the independent Platform PocketIC 0.18
  client for private checkpoints and native qualification. Preserve encoded
  checkpoints and keep client transport outside deployed Wasm.
- Select Build 0.74.0 and Host Artifacts/FS/Process 0.74.1 for qualification
  builds and retained reports. The Rust 1.99.1 requirement is unchanged.
- Refresh Shared Tooling to 0.27.28. Retain the complete release gate;
  optional group-scoped validation plans remain unselected.

## [0.37.0] - 2026-11-02

### ⚠️ Breaking

- Require Rust 1.99.1 for library consumers and pin the same compiler for
  development and releases. [#87](https://github.com/dragginzgame/ic-scheduling/issues/87).

### 🧪 Testing

- Prepare one compiler in hosted workflows and admit actual Cargo/rustc
  identities through `rust-policy-check`. Retain native hosts, independent
  core/consumer checks, Wasm, archives and explicit installed-runtime gates.

## [0.36.0] - 2026-11-01

### ⚠️ Breaking

- Move IC0/execution to direct Runtime 0.17.1. Align every executor supplier
  together, including Endpoint 0.4 and canonical Types 0.16; retain exact
  singleton admission and callback/cancellation behavior.
- Align the public Metrics types with Metrics 0.25. Consumers using the native
  Wire metadata bridge must align on Wire 0.14; measurement arithmetic and
  deployed timer behavior remain unchanged.

### 📦 Dependencies

- Adopt Wire/Derive 0.14.1 for private probe, checkpoint and driver codecs.
  Preserve request identities, budgets and checkpoint bytes; the deployed timer
  provider and its dependency boundary are unchanged.
- Select the current Build/Host 0.73.6 and Logging 0.23.7 qualification patches.
- Refresh the reviewed Shared Tooling snapshot to 0.27.26, retaining grouped
  Cargo metadata and dependency admission. Use Shared's current native build
  identity and Cargo tool retention policy in the existing setup workflow.

## [0.35.4] - 2026-11-01

### 🧪 Testing

- Separate installed runtime/cohort subjects from the qualification driver's
  host-only tests. `make test-timer-tooling` runs receipt, discovery and command
  checks without starting PocketIC; the explicit IC driver selects the dedicated
  integration target and refuses utility tests in watchdog qualification.
- Give cohort selection and measurements one typed catalog. Admit compiler,
  Cargo and target identity together from the first successful build; reject
  later mismatches without partially changing the retained identity. Preserve
  report JSON, strict completeness and field validation, and comparison rules.
- Isolate release-fixture metadata, interruption, recovery and publication phases
  so their injected faults and temporary settings cannot affect later phases.

### 🧹 Cleanup

- Name ordinary completion outcomes and resolve watchdog schedules once before
  mutation. Make provider staging and arm confirmation explicit while retaining
  command ordering, abandonment repair, cleanup and cancellation/error accounting.
- Project ordinary timer observations from one snapshot with explicit defaults
  for absent demand, retaining identities, counters and inactive reasons.
- Use named runtime-probe checkpoint fields while preserving encoded bytes and
  upgrade reconstruction.

## [0.35.3] - 2026-11-01

### 📦 Dependencies

- Adopt Platform 0.16.0 with one matching executor guard; align private
  qualification consumers on Testkit/Native 0.76.0 and Wire/Derive 0.13.3.
  Preserve canonical Types 0.15.1,
  the executor/IC0-only library boundary and existing timer behavior.

## [0.35.2] - 2026-11-01

### 🩹 Fixed

- Reconcile absent ordinary scheduling demand to idle without recording explicit
  cancellation. Retain registration identity and observations, preserve existing
  inactive reasons and allow later demand to rearm the claim. Running requests
  retain command ordering, invariant-failure priority and remove-on-stop semantics.
  This supplies Canic's terminal intent-cleanup boundary
  ([#84](https://github.com/dragginzgame/ic-scheduling/issues/84),
  [Canic #545](https://github.com/dragginzgame/canic/issues/545)).

### 🧪 Testing

- Distinguish demand removal from explicit cancellation in the private IC probe.
  Cover scheduled and awaiting work, retained identity, cancellation accounting,
  expired contexts and rearming through the installed provider. Keep the new
  subject in the watchdog qualification roster.

## [0.35.1] - 2026-11-01

### 📦 Dependencies

- Adopt Build/Host 0.73.3, Testkit/Native 0.75.2 and Wire/Derive 0.13.2 for
  private qualification tools. Refresh the managed release editor and Testkit CLI;
  retain artifact custody, bounded simulator startup and timer runtime behavior.
- Adopt Metrics 0.24.1, preserving public measurement types and arithmetic.

### 🔧 Changed

- Add read-only group release readiness through `make release-status`, including
  published/proposed versions, tags, exact-commit CI and uncertain observations.
- Adopt Shared Tooling 0.27.23 and conservative documentation-only CI selection.
  Preserve all three stable host checks, full code/release gates and explicit
  group versions; no canonical `VERSION` file is introduced.
  [#83 — Adopt conservative documentation-only CI selection](https://github.com/dragginzgame/ic-scheduling/issues/83).

### 🩹 Fixed

- Isolate readiness-fixture group and commit inputs from the invoking release,
  so group-scoped release gates still exercise both groups' observations.
- Include timer runtime diagnostics and the final cohort receipt in hosted
  recovery artifacts; verify the actual workflow's upload paths.

## [0.35.0] - 2026-10-31

### ⚠️ Breaking

- Adopt Metrics 0.24 for the public measurement types. Consumers exchanging
  these values must align Metrics identities; its optional native schemas now
  use Wire 0.13. Timer runtime builds keep native metadata disabled, preserving
  arithmetic, report shapes and deployed dependency boundaries.

## [0.34.0] - 2026-10-31

### ⚠️ Breaking

- Adopt Metrics 0.23.2 for public measurement types. Consumers exchanging these
  values must align Metrics identities. With defaults disabled, native schemas
  require Metrics' `native-wire` feature and Wire 0.12.
- Adopt Platform 0.15 for the runtime and its matching singleton guard. Runtime
  consumers coordinate on this facade; callbacks, retained bytes and arithmetic
  semantics remain unchanged.

### 🔧 Changed

- Align private probes and typed host calls on Wire 0.13 and Testkit 0.75.
- Version and release Timers and its four private probes independently of Jobs.
  [#82 — Version Jobs and Timers independently](https://github.com/dragginzgame/ic-scheduling/issues/82).

All notable changes to this project are recorded here.

## [0.33.0]

### ⚠️ Breaking

- Adopt Metrics 0.18 for public measurement types and Platform 0.9's embedded
  executor/IC0 owner. Coordinate direct Metrics dependencies and all canister
  runtime consumers on these new lines; mixed executor owners are rejected.

### 📦 Dependencies

- Adopt published Platform 0.9.1, Metrics 0.18.1, Wire 0.7.25 and
  IC Build/Host/Testkit 0.70.5. Keep probe Wire
  contracts and decoder budgets, with simulator capabilities confined to native tests.
- Retain Wire 0.7.25 until Testkit publishes compatible Wire 0.8 limit/error types
  ([Tooling #270](https://github.com/dragginzgame/ic-tooling/issues/270)); an isolated
  0.8 adoption currently fails the maintained native harness's type checks.

### 🛠️ Tooling

- Refresh Shared Tooling to 0.27.11. Qualify the embedded runtime owner and its
  version-matched, feature-selected singleton guard instead
  of retired registry providers, retain source/feature rejection coverage, and
  confine both unsafe allowances and expectations to the FFI boundary.

## [0.32.4] - 2026-10-30

### 🧪 Testing

- Add exact runtime-subject selection to the existing PocketIC driver, retaining
  source-bound Wasm custody, timeout handling and strict discovery admission.
  Focused runs do not replace the full release recovery and cohort gates.
- Retain bounded discovery diagnostics and successful runtime receipts with
  selected subjects and build identities. Preserve invocation evidence after
  success and keep original producer errors when saving diagnostics fails.
- Cover freezing after the Watchdog commits its successor and before queued work
  starts. Share freeze/top-up assertions with the armed-timer regression and
  require unchanged ownership, bounded overdue delivery and continued cadence.
- Cover freezing during async `Once` and `AfterCompletion` work. Verify that
  application-declared call failure retires without replay or stuck execution,
  explicit retry preserves ownership, and recurrence follows successful completion.

## [0.32.3] - 2026-10-30

### 🛠️ Tooling

- Select development and minimum Rust compilers from their canonical files in
  setup, Make and CI, preserving explicit MSRV overrides. Prepare host parsers
  before compiler selection and keep Cargo-tool setup after Rust bootstrap.

### 📚 Documentation

- Update the public API examples and workspace description, correct obsolete
  handle-restoration guidance, and separate current setup and ownership rules
  from historical adoption evidence.

## [0.32.2] - 2026-10-30

### ♻️ Refactoring

- Leave unaffected provider handles bound and clear only the capabilities selected
  by each registry effect. Remove handle restoration and its token-bearing
  cleanup wrapper. Resolve Watchdog completion decisions together with their
  schedule metadata, preserving timer
  APIs, command precedence, callback cleanup and rollback rules.
- Share immediate stopping callbacks in native and installed probe fixtures and
  remove unused registration storage from the scheduling trace.

### 🛠️ Tooling

- Remove the redundant standalone snapshot-corruption suite. Keep snapshot
  admission and its failure ordering in the actual tag-workflow fixture; Shared
  Tooling owns the generic verifier rejection matrix.

## [0.32.1] - 2026-10-29

### 📦 Dependencies

- Adopt published Metrics 0.17.2, Platform 0.8.1, Wire 0.7.22 and
  IC Build/Host/Testkit 0.70.0, preserving timer APIs, measurement arithmetic,
  executor/IC0 identities, probe protocols and dependency boundaries.
- Keep deployed probe runtimes free of external Principal and hashing
  dependencies through Platform's owned Principal boundary. Native Testkit now
  shares Platform 0.8, removing the older 0.7 dependency
  ([IC Tooling #267 — Adopt Platform 0.8 in Testkit](https://github.com/dragginzgame/ic-tooling/issues/267)).

## [0.32.0] - 2026-10-29

### ⚠️ Breaking

- Adopt Metrics 0.17 for the public measurement counters and summaries. Consumers
  exchanging these types with a direct Metrics dependency must select its 0.17
  line. Serde transport is now required; native builds include Metrics-owned Wire
  metadata, while the Wasm timer library remains Wire- and Candid-free.

### 📦 Dependencies

- Adopt published Platform 0.7.4, Wire 0.7.21 and IC Build/Host/Testkit 0.69.9.
  Preserve the executor/IC0 identities, timer dispatch and framed probe protocols.

## [0.31.16] - 2026-10-28

### 📦 Dependencies

- Adopt published IC Platform 0.7.2 and Wire 0.7.19, preserving executor/ic0-only
  library features, framed probe protocols and existing resource limits.
- Adopt IC Build/Host/Testkit 0.69.6 and remove the temporary Build pin after
  the upstream macOS source-inventory correction
  ([IC Tooling #262](https://github.com/dragginzgame/ic-tooling/issues/262)).

## [0.31.15] - 2026-10-28

### 🧪 Tests

- Qualify explicit Watchdog owner repair after stop/restart abandons a queued
  initial or successor scheduler. Real round pressure establishes the missing
  interleaving for all three reconciliation modes; runtime behavior is unchanged
  ([#77](https://github.com/dragginzgame/ic-timers/issues/77)).

## [0.31.14] - 2026-10-28

### 🔧 Changed

- Document the Platform/Timers/application call ownership boundary and reject
  application Relay dependencies below the published runtime, including renamed
  target-specific build dependencies. No runtime API or behavior changes
  ([#80](https://github.com/dragginzgame/ic-timers/issues/80)).

## [0.31.13] - 2026-10-28

### ➕ Added

- Let hosts reject private timer ingress from their existing inspection hook,
  while retaining endpoint authentication for foreign inter-canister calls
  ([#79](https://github.com/dragginzgame/ic-timers/issues/79)).

### 🩹 Fixed

- Let owner reconciliation restore a confirmed abandoned Watchdog scheduler,
  including an unchanged deadline, while preserving queued work and effect
  uncertainty. Native regressions cover the repair; the installed stop/restart
  interleaving remains unqualified
  ([#77](https://github.com/dragginzgame/ic-timers/issues/77)).
- Detect conflicting global-timer writes on the next backend rearm, preserving
  legitimate timer expiry and trapping with message rollback on a mismatch
  ([#78](https://github.com/dragginzgame/ic-timers/issues/78)).

### 📦 Dependencies

- Adopt IC Build/Host/Testkit 0.69.4, preserving timer APIs, probe protocols and
  feature selections. Prepare the selected contributor tools with
  `make install-rust-tools install-release-editor install-testkit-server`.

### 🔧 Changed

- Adopt Shared Tooling 0.27.3 with independently source-checked snapshot payloads
  and report-only unattended-maintenance rules. Optional sccache and schedules
  remain inactive.

## [0.31.12] - 2026-10-28

### 🧪 Testing

- Require a genuine freeze/top-up PocketIC regression: preserve an overdue
  watchdog while updates reject, then recover and resume cadence using only
  external funding, without an upgrade or timer repair call.

### 📦 Dependencies

- Adopt Build/Host/Testkit 0.69.3 and Platform 0.7.1 in the root catalog and
  lockfile, retaining current Wire 0.7.18 and Metrics 0.16.3. Preserve timer
  protocols and feature selections. Prepare contributor tools through
  `make install-release-editor install-testkit-server`.

### 🔧 Changed

- Adopt Shared Tooling 0.27.2, including formatting-cache detection around
  conditional Rust attributes, dependency-finding ownership and native bootstrap
  guidance. Stage the hook fixture's complete companion closure instead of
  maintaining a second helper list; optional stack-diagram refresh remains inactive.

## [0.31.11] - 2026-10-27

### 🔧 Changed

- Adopt Shared Tooling 0.26.24 with visible per-pass formatting timings and
  reuse of successful results for unchanged inputs. Keep prepared-tool admission,
  selected-file hook isolation and retained failure diagnostics; include the
  upstream hook fix that keeps submodule contents opaque.

## [0.31.10] - 2026-10-27

### 📦 Dependencies

- Adopt Build/Host/Testkit 0.69.0, aligning native tooling with Platform 0.7
  and removing its older Platform dependency. Timer APIs and deployed features
  remain unchanged.

### 🧭 Migration Notes

- Prepare the updated contributor tools with
  `make install-release-editor install-testkit-server`.

## [0.31.9] - 2026-10-27

### 📦 Dependencies

- Adopt Platform 0.7.0 and Build/Host/Testkit 0.68.12. Timers does not use the
  retired Candid compiler APIs; runtime features and executor identities remain
  unchanged. Prepare the selected contributor tools again through
  `make install-release-editor install-testkit-server`.

### 🥾 Governance

- Adopt Shared Tooling 0.26.22, clarifying owned-type Candid restrictions and
  native input/error-handling guidance. Existing deployment checks remain in
  place; the updated rules do not add executable enforcement.

## [0.31.8] - 2026-10-27

### 📦 Dependencies

- Adopt Platform 0.6.20 and Build/Host/Testkit 0.68.11; align the catalog with
  selected Wire 0.7.18 and Metrics 0.16.3. Contributor tools must be prepared
  again through `make install-release-editor install-testkit-server`.

### 🧹 Cleanup

- Delegate qualification build retention and freshness checks to IC Build's
  published evidence owner, preserving exact artifact selection and cohort
  report admission ([IC Tooling #255](https://github.com/dragginzgame/ic-tooling/issues/255)).

### 🧪 Testing

- Replace numeric native-test policies, trace commands and arbitration scenarios
  with named cases and explicit expectations. Preserve exhaustive interleavings,
  lifecycle cleanup coverage and independent outcome assertions; timer behavior
  and published APIs are unchanged.

## [0.31.7] - 2026-10-27

### 🧪 Testing

- Retire redundant Shared regression suites from local release validation;
  retain Timers' actual dependency, metadata, changelog and source-policy checks.
- Prepare release fixtures through the canonical dependency copier and keep
  Testkit setup/admission checks in their dedicated fixture. Release tests retain
  the parallel qualification barriers and driver-failure checks.

## [0.31.6] - 2026-10-27

### 🧹 Cleanup

- Share fresh lifecycle initialization across timer policies and resolve ordinary
  scheduling directly into execution and observation values. Preserve callback
  authority, failure precedence and cleanup ordering.
- Replace numeric/boolean probe policy selectors with named Wire records and
  centralize cohort selection. Rebuild the unpublished runtime probe and native
  driver together; published timer APIs and cohort report formats are unchanged.

### 🧪 Testing

- Consolidate lifecycle-refusal assertions and derive dependency-fixture members
  from the workspace. Remove redundant compile-only CI dispatch and the generic
  Shared evidence-test dispatch while retaining Timers' integration checks.

### 📚 Documentation

- Adopt Shared Tooling 0.26.21 and its emoji category headings for current and
  future release notes. Preserve release identities, dates and older history;
  changelog presentation adds no deployment gate.

## [0.31.5] - 2026-10-26

### 📝 Summary

- Adopt published Build/Host/Testkit 0.68.7 for host qualification and the
  lock-selected release editor and Testkit CLI. Testkit now checks the final
  synchronous startup deadline consistently with async construction. Preserve
  timer APIs, probe wire contracts and deployed dependency exclusions.
- Adopt Shared Tooling 0.26.20: compact task-specific guidance, focused batch
  validation and opt-in line/symbol reporting. Selected executable helpers and
  product release gates remain unchanged; optional npm checks stay unselected.

## [0.31.4] - 2026-10-26

- Adopt published Platform/macros 0.6.18 and Wire/derive 0.7.16. Preserve timer
  APIs, executor/IC0 identities and probe wire contracts. Platform's new optional
  BLAKE3 feature remains disabled; Wire retains varint rejection and decode limits.

## [0.31.3] - 2026-10-26

- Adopt published Wire/derive 0.7.15 and Build/Host/Testkit 0.68.6 for probe and
  host qualification, retaining existing frame bytes, limits and timer APIs.
  Qualify the incoming Platform/macros 0.6.17 selection and refresh both
  lock-selected CLIs.
- Adopt Shared Tooling 0.26.19. Normal release commands now start fresh from
  current committed metadata; use `release-resume VERSION=x.y.z` for a saved
  attempt. Preserve validation evidence and current-source/ref checks. Historical
  Draft labels no longer block new releases; fixture-retention failures have
  clearer diagnostics.

## [0.31.2] - 2026-10-25

- Qualify published Metrics 0.16.2, Build/Host/Testkit 0.68.2 and Wire/derive
  0.7.9. Retain timer APIs, probe wire contracts and the
  Candid-free deployed graphs. Refresh the lock-selected editor and Testkit CLI.
- Adopt Platform/macros 0.6.16, qualifying refactored probe entrypoint generation
  with explicit Wire codecs and unchanged executor/IC0 identities. Principal
  decoding now preserves structured format errors from its deserializer.
- Adopt Shared Tooling 0.26.16: preserve incomplete earlier delivery evidence
  when advancing from manually committed release metadata, and distinguish
  busy Cargo-tool admission with status 75. Local checks remain immediate and
  offline; optional read-only waiting does not install or repair tools.

## [0.31.1] - 2026-10-25

- Share fresh/existing lifecycle reconciliation decisions and select watchdog
  completion reasons and schedule metadata in one precedence pass. Preserve
  public APIs, callback authority, cleanup ordering and successor recovery.
- Consolidate probe Wire gate exchanges, watchdog fault injection and native
  cancellation assertions without removing policy or installed-IC test cases.
- Separate unfinished cohort build context from complete schema-2 receipts;
  require measurements in completed reports and couple each artifact lease to
  its checked build inputs. Retire duplicate metric text records in favor of
  the existing JSON receipt and comparison output.
- Repository tooling: retire three generic Shared Tooling test dispatches and
  snapshot payloads through the same reviewed revision's exporter. Keep local
  fixture/macOS assertions and product gates. Reset dependency-boundary fixture
  manifests together between independent mutation cases.

## [0.31.0] - 2026-10-24

- **Breaking:** adopt Metrics 0.16.0. `MeasurementCounter` and
  `MeasurementSummary` now have the Metrics 0.16 Rust identity, including values
  returned by timer snapshots. Align direct Metrics dependencies when exchanging
  these values. Arithmetic APIs and implementations are unchanged; Timers does
  not use the stable adapter extracted to Persistence, and needs no storage reset.
- Qualify the incoming published Build/Host/Testkit 0.68.1 selection and refresh
  both lock-selected CLIs. Platform/macros 0.6.15 and Wire/derive 0.7.2 remain
  current. Retain Thiserror 2.0.18 because the selected PocketIC client requires
  that exact version; the other direct registry dependencies are already current.

## [0.30.13] - 2026-10-24

- Qualify the incoming published Platform/macros 0.6.15 selection. Their Rust
  sources match 0.6.14; retain executor/IC0 identities, callbacks and probe
  protocols. Platform's optional Wire metadata remains disabled, so its metadata
  dependency update does not change the selected Wire 0.7.2 graph.

## [0.30.12] - 2026-10-24

- Adopt published Build/Host/Testkit 0.68.0 and Wire/derive 0.7.1 together,
  removing the Testkit limits mismatch tracked in
  [ic-tooling #248](https://github.com/dragginzgame/ic-tooling/issues/248).
  Refresh the lock-selected release editor and Testkit CLI.
- Remove decode-only Serialize bounds from unpublished probe adapters. Retain
  method identities, encoder bytes and existing resource limits; qualify
  nonminimal-varint rejection and Deserialize-only native responses. Wire 0.7
  no longer proves byte canonicality by reserializing decoded values. Published
  timer APIs and deployed library dependencies are unchanged.

## [0.30.11] - 2026-10-24

- Adopt published Build/Host/Testkit 0.67.0 and its explicit bounded Cargo
  configuration observation. Refresh the lock-selected release editor and
  Testkit CLI; Platform 0.6.13, Wire 0.6.12 and Metrics 0.15.8 remain current.
- Bind cohort comparisons to Build's source-independent recipe identity
  ([#241 — Build recipe identity for cohort comparisons](https://github.com/dragginzgame/ic-tooling/issues/241)).
  Qualification receipts now require schema 2; regenerate reviewed baselines.
  Recipe digests add configuration admission without exposing environment values.
- Adopt committed Shared Tooling setup phase/duration diagnostics, preserving
  path-only stdout, offline admission and existing tool identities.

- Simplify private registration claim access while preserving reconciliation
  validation, callback retention and factory cleanup ordering.
- Represent qualification selections with a complete typed mode, keeping cohort
  report paths together and preserving the existing CLI and recovery gates.
- Consolidate directive conversion coverage through both public callback decision
  types, retaining checked-delay rejection and policy/cadence assertions.

## [0.30.10] - 2026-10-24

- Adopt Shared Tooling 0.26.10 with literal formatter catalog admission, offline
  tool diagnostics and common external-IC dependency guards alongside Timers'
  deployment policy. CI tool caching remains opt-in and disabled here.

- Require complete typed cohort receipts, including compiler and artifact metadata,
  and compare the admitted report directly before publication. Complete schema-1
  receipts keep their JSON fields; incomplete metadata now rejects consistently.
- Consolidate native failure tests and remove duplicated consumer-policy and
  codec checks. Suspended command sequences assert their outcome before cleanup
  while avoiding redundant abandonment cases. Retain IC recovery evidence.
- Remove redundant instruction bookkeeping from the unpublished mixed-policy
  probe and its wire records; callback progress, isolation and upgrade checks remain.
  Rebuild the probe and native driver together. Published timer APIs are unchanged.

## [0.30.9] - 2026-10-24

- Qualify the incoming published Build/Host/Testkit 0.66.3 selection and refresh
  both lock-selected CLIs. Build excludes generated targets in nested explicit
  Cargo workspaces while retaining declared source inputs. Platform/macros 0.6.12,
  Wire/derive 0.6.11 and Metrics 0.15.7 are current published selections. Retain
  Thiserror 2.0.18 because the selected PocketIC client requires that exact version.

- Require named recovery subjects and reject ignored qualification selections.
  Validate exact cohort metric fields, retain per-instance PocketIC logs on
  failures, and upload successful hosted cohort receipts without extra builds.
- Confine Platform imports to the private provider boundary, including aliases.
  Isolate provider-effect application in a private runtime module without changing
  transition ordering, public APIs or recovery semantics.
- Exercise suspended ordinary command sequences through completion, abandonment
  and backend rejection. Document and test that dropping a registration does not
  cancel or unregister it. Mark consumer adoption records as historical evidence.

- Reject empty/missing qualification test selections before canister builds.
  Require a fresh complete cohort report matching this invocation's build context,
  and publish it only after successful subjects, input checks and any comparison.
- Protect reviewed cohort baselines with Host's inode-aware alias admission and
  atomically publish complete candidate reports through published Host FS 0.66.3.
- Keep tag-only metadata checks on the shared host-tool setup/check path without
  provisioning the release editor, Testkit CLI or PocketIC server.
- Delegate native probe Wire calls to Testkit 0.66.3's typed helpers, preserving
  method identities, caller selection and codec limits. Keep deliberately
  malformed-frame tests on raw transport; deployed timer/probe graphs are unchanged.
- Keep the renamed-Wire rejection fixture on the prepared current lock selection,
  so offline CI reaches its dependency-boundary assertions without downloading
  obsolete Wire packages ([#75](https://github.com/dragginzgame/ic-timers/issues/75)).
- Correct README setup guidance to require explicit optional IC-tool selections
  ([#76](https://github.com/dragginzgame/ic-timers/issues/76)) and update its
  published API example to 0.30.8. This is advisory documentation upkeep; it adds
  no release gate and changes no runtime behavior.

## [0.30.8] - 2026-10-23

- Adopt published Platform/macros 0.6.11 and Wire/derive 0.6.10, retaining
  Metrics 0.15.7 and Build/Host/Testkit 0.66.2. Keep existing native probe
  records, frame limits and public timer APIs unchanged. Host metadata stays
  disabled, and probe records do not use packed fields.
- Refresh the lock-selected Build/Testkit CLIs through the existing setup
  targets. Native qualification retains client/server ownership and Host artifact/
  process contracts; Thiserror remains simulator-constrained at 2.0.18.
- Adopt reviewed Shared Tooling 0.26.9 for relative-path and inherited-CDPATH
  fixture handling. Retain the current tool-build policy and qualified tool
  selections; offline checks remain non-mutating.

## [0.30.7] - 2026-10-23

- Adopt published Platform/macros 0.6.10 and Metrics 0.15.7. Their Rust sources
  match the prior selections; retain the Rust 1.91 floor, executor/IC0 identities,
  native Wire protocols and public timer/measurement APIs. Platform's optional
  host metadata feature remains unselected.

## [0.30.6] - 2026-10-23

- Adopt aligned Build, Host and Testkit 0.66.0. Native qualification borrows
  PocketIC clients through `instance()`, retaining server ownership until client
  teardown; public timer APIs and probe protocols are unchanged.
- Adopt reviewed Shared Tooling 0.26.8, fixing installer-owner cleanup races and
  removing the global Cargo source-preparation lock. Explicitly prepare the
  Cargo tools and selected CLIs after its build-policy change; old tools remain
  retained and offline checks never install replacements.

## [0.30.5] - 2026-10-23

- Adopt aligned Build, Host and Testkit 0.65.0 for native qualification. Keep
  Timers' raw PocketIC/native Wire integration and public timer API unchanged.
  Re-run `make install-release-editor install-testkit-server` for the selected CLIs.
- Adopt published Wire/derive 0.6.5, preserving native frames and traversal
  limits while using Wire's small-u64 serialization optimization.
- Adopt reviewed Shared Tooling 0.26.7. Explicit Cargo-tool setup can wait
  briefly for an admitted installer owner, then recheck and reuse its result;
  offline checks remain immediate and foreign locks remain untouched.

## [0.30.4] - 2026-10-23

- Adopt published Wire/derive 0.6.2. Timers already uses the retained native
  Serde codec, so retiring Wire's finite API requires no probe protocol rewrite.
  Preserve method identities, frame bytes, resource limits and decoder errors;
  no public timer API changes.

## [0.30.3] - 2026-10-22

- Adopt published Wire/derive 0.5.20 and Metrics 0.15.5. Wire improves Wasm
  canonical-byte comparison and generated-name hygiene; Metrics arithmetic is
  unchanged. Timer APIs, probe wire formats and decoder limits remain unchanged.
- Adopt aligned Build, Host and Testkit 0.64.0 for native qualification. Import
  PocketIC clients directly from Platform while Testkit retains server lifecycle
  and progress helpers. Re-run `make install-release-editor install-testkit-server`
  to prepare the selected CLIs; the published timer API is unchanged.

## [0.30.2] - 2026-10-22

- Qualify published Platform/macros 0.6.7, Metrics 0.15.4, Wire/derive 0.5.19
  and IC Tooling 0.63.3. Preserve the incoming lockfile selections, runtime APIs,
  executor identity and probe wire formats.
- Adopt reviewed Shared Tooling 0.26.6, including graph-feature observation and
  retained failure-evidence fixes. Pass an explicit empty IC executable selection
  to host-only compact evidence collection; Testkit still owns PocketIC setup.
- Keep Shared's generic command, formatter, Make-interface and LOC regression
  suites upstream. Retain Timers' actual setup, hook/index, release/recovery and
  product checks ([#74](https://github.com/dragginzgame/ic-timers/issues/74),
  [prepared PR #73](https://github.com/dragginzgame/ic-timers/pull/73)).

## [0.30.1] - 2026-10-21

- Adopt published Platform/macros 0.6.0. Keep only executor/IC0 features in the
  library, removing unused Principal, Serde, crypto and text dependencies from
  its graph while preserving executor 2.0.0 and IC0 1.2.0. Enforce that exclusion
  with a negative Principal-leak fixture
  ([#40 — Keep executor/IC0-only graphs free of Principal, Serde and crypto dependencies](https://github.com/dragginzgame/ic-platform/issues/40)).
- Select Principal explicitly for native simulator IDs; canister probes retain
  their runtime-implied Principal feature. Public timer APIs and Wire formats
  are unchanged.
- Adopt published IC Tooling 0.63.0 across Build, Host and Testkit, removing the
  older native Platform identity and Testkit-owned Candid codec selection.
  Prepare the release editor and Testkit server CLI with explicit `cli` features;
  library dependencies remain CLI-free. Re-run `make install-release-editor
  install-testkit-server` before offline checks. PocketIC retains transitive Candid.

## [0.30.0] - 2026-10-21

- **Breaking dependency identity:** adopt Metrics 0.15 for the public
  `MeasurementCounter` and `MeasurementSummary` types. Consumers exchanging
  these values with a direct Metrics dependency must also select 0.15.
  Timer scheduling, arithmetic behavior and probe Wire formats are unchanged.
- Keep Platform, Testkit, Wire and the probe protocol in the native driver's
  dev-dependencies. The normal build coordinator excludes the simulator and
  codec stack; integration tests retain their existing dependencies. Enforce
  that boundary in CI and reject accidental promotion into normal dependencies.

## [0.29.7] - 2026-10-20

- Adopt published Build 0.24.0 and Host Artifacts/Process 0.32.0 for native
  qualification, resolving the injected-spawner type mismatch without a shim.
  Adopt Testkit 0.60.0 so the native driver shares Platform 0.5.0; Testkit retains
  Host 0.31.2 internally. Keep Wire 0.5.2 and the existing probe protocols.
  Prepare and admit the root-lock-selected release editor and Testkit CLI/server
  through their canonical installers; runtime dependencies and APIs are unchanged.

- Adopt published IC Platform/macros 0.5.0, preserving executor 2.0.0, IC0 1.2.0,
  explicit Wire codecs and timer callback semantics. Allow its `runtime` feature
  for probe entrypoints while retaining the library's executor/IC0-only boundary.
  Candid remains native-only through Testkit/PocketIC.

- Adopt reviewed Shared Tooling 0.25.7. Retain failed Cargo metadata output,
  root inputs and independent command/logger statuses without retries; qualify
  their inclusion in Timers' existing CI failure archive.
- Run the existing library Clippy gate before CI compilation and package checks.
  Order publication admission, Clippy, packaging and publication sequentially;
  failure stops later stages under parallel Make without changing lint scope.
- Use Shared's grouped formatter defaults directly and refresh its fixture
  dependency discovery. Product tool selections and runtime guarantees are unchanged.

## [0.29.6] - 2026-10-20

- Correct the safety guide's stale pending-backend description and distinguish
  retired-provider receipts from current source-bound qualification and host gaps.
- Bound the qualification build/test worker through Host process-group ownership
  and give each Testkit server a hard TTL, covering stalled builds, test calls
  and synchronous instance deletion. Preserve typed timeout and exit evidence.
- Exercise 7,776 generated command traces across all timer policies and declaration
  lifetimes, checking replacement/stale authority, isolation and handle cleanup.
  These native checks supplement the maintained PocketIC recovery subjects.
- Retain structured cohort costs with compiler, server, lockfile, profile and exact
  artifact identities. Add explicit baseline comparisons that reject incompatible
  contexts and report deltas without silently updating baselines or setting budgets.
  Receipt/hashing dependencies are confined to the unpublished host driver.

## [0.29.5] - 2026-10-20

- Adopt published IC Platform 0.4.0. Use its explicit external Principal in the
  PocketIC harness, preserving simulator API identities, Wire payloads and the
  qualified executor/IC0 selections. The published timer API is unchanged.
- Adopt Shared Tooling 0.25.4 for explicit Cargo-tool feature selection and
  receipt admission. Refresh prepared tools under the updated build policy;
  retain Timers' existing featureless editor/Testkit selections and release gates.
- Align the qualification driver with published Host Process 0.31.1 and Build
  0.22.1, requiring the Build patch that exposes the matching process identity.
- Retain redirected evidence-parent links and independent failure logs without
  traversing their children; report partial collection instead of losing the
  archive or claiming completion.

## [0.29.4] - 2026-10-20

- Delegate dependency graph checking to Shared Tooling 0.25.2 while retaining
  Timers' runtime identity and feature policy. Cover both probe canisters and
  the protocol package alongside the library.
- Select probe RPCs through the existing method enum, keeping names and frame
  IDs together and removing string lookup failures without changing Wire bytes.
- Adopt IC Build's published release editor for Shared's metadata contract;
  prepare it with `make install-release-editor` before standalone version
  preparation, and include it in standalone release validation's ordered setup.
  Keep release execution maintainer-owned. Use Clap for the native
  qualification driver's existing mode selection.
- Archive completed handoff narratives and keep current source, pending work
  and qualification limits together in the compact status document.

## [0.29.3] - 2026-10-19

- Adopt published IC Platform 0.2 for Candid-free probe entrypoints, raw calls
  and protected spawning, removing full CDK from the workspace graph. Keep Wire
  formats and executor/IC0 identities; remove obsolete hidden-entrypoint flags
  ([#31 — Support Candid-free raw and Wire canister entrypoints](https://github.com/dragginzgame/ic-platform/issues/31)).
- Align the qualification harness with published IC Build 0.22 and Host Process
  0.30, resolving the injected-spawner mismatch without a compatibility adapter.
- Enforce the library's resolved dependency and Platform-feature boundary for
  Wasm and the native host, rejecting Candid, full CDK and qualification tooling
  while retaining the qualified executor and IC0 identities.
- Extend backend ownership coverage across submission, retry, cancellation,
  terminal delivery and callback-slot reuse. Timer APIs and runtime behavior
  are unchanged.

## [0.29.2] - 2026-10-19

- Update the qualification harness to IC Build 0.21 and its matching Host
  Process 0.29 spawner boundary, keeping build and process types aligned.
- Refresh the selected IC Platform to 0.1.18 and IC Wire to 0.4.6.
  The harness dependency changes are repository-only; the published timer API
  and scheduling contracts are unchanged.

## [0.29.1] - 2026-10-19

- Align the PocketIC harness with the Host Process 0.27 family exposed by
  IC Build 0.19, fixing the injected-spawner type error in `testing-check` and
  removing the unused Host 0.28 dependency copies.

- Move unpublished probe RPCs, self-call gates and upgrade checkpoints to
  published IC Wire. Remove local Candid records, codec calls and explicit
  Candid feature selections. Preserve timer scheduling and callback contexts;
  the published library API and its fixed internal dispatch payload are unchanged.
- Rebuild both probe canisters and their native driver together. The private
  endpoints and checkpoint have one current Wire format with no legacy reader.
  Candid remains transitive through Platform CDK and PocketIC/Testkit; this is
  not a Candid-free qualification graph.

## [0.29.0] - 2026-10-19

### Breaking

- Delegate diagnostic event arithmetic to IC Metrics. All `TimerCounters`
  getters now return the reexported `MeasurementCounter`, carrying a `u128`
  total and actual-overflow flag. `work_completed()` merges its classified
  counters without losing overflow provenance. Timer events, registration
  resets and scheduling policy retain their meanings.
- Consumers must replace integer arithmetic and cap inference with Metrics'
  checked totals, merges and deltas, and preserve wide totals and overflow flags
  in serialization. Compare full registration identities before interval
  calculations. There are no legacy integer getters. See the
  [counter contract](../../docs/timers/design/observability.md#counter-semantics).

## [0.28.2] - 2026-10-18

- Add real-Wasm qualification for a full timer batch under queue pressure and
  suspended work, including cancellation, replacement and continued dispatch.
  Add deadline-order traces covering callback-created timers, overdue delivery
  and self-cancellation. Published runtime behavior is unchanged.
- Keep generic host/Cargo installer and release-runner regression matrices at
  Shared Tooling. Retain Timers' tool admission, release integration, failure
  evidence and runtime qualification; canonically retire their unused snapshot
  payloads without changing the reviewed Shared revision.
- Select Rust 1.99.1 for development and hosted qualification, preserving the
  independent Rust 1.91 library/probe MSRV and dependency selections.

## [0.28.1] - 2026-10-18

- Adopt published IC Platform 0.1.12 for probe entrypoints, calls, protected
  spawning, stable save/restore and cycle burning. Remove every direct Candid
  and CDK-family declaration and the last local pin/policy exception. Preserve
  the qualified executor/system identities, wire contracts and existing upgrade
  encoding/error behavior through Platform's exact upstream helpers
  ([Platform #19 — Timers stable-helper and cycle-burn boundary](https://github.com/dragginzgame/ic-platform/issues/19)).
- Adopt published Testkit 0.54 and remove `ic-testkit-candid`. The native driver
  uses Platform's decoder limits directly, preserving all five ceilings and
  typed errors; dependency policy forbids the retired package in every member.

## [0.28.0] - 2026-10-18

### Breaking

- Select Metrics 0.14 for the publicly reexported `MeasurementSummary` and
  timer performance accessors. Its Rust type identity changes from Metrics
  0.13; align consumers that exchange these values with Metrics directly.
  Snapshot fields, arithmetic and timer scheduling contracts are unchanged.

### Changed

- Adopt published IC Platform 0.1.8 for the shared CDK executor, raw IC system
  calls, probe Candid DTOs and native encoding. Remove direct executor/IC0
  dependencies and their pin exceptions now that Platform owns the exact
  qualified identities; preserve callback/trap contexts and wire formats.
  Retain probe CDK/Candid and Testkit's native limits identity pending Platform
  #1 and #2; host parser/Agent features remain disabled.
- Reuse checked pending ordinary schedules at completion, selecting their
  directive observation and cancellation outcome in the same arbitration pass.
- Share watchdog installation in the runtime probe while retaining separate
  callbacks, durable scheduling intent and completion counters.

### Development

- Enforce Candid/CDK direct dependency ownership through Shared's canonical
  checker, with explicit exceptions for the remaining published API blockers.
- Group production and test/tooling packages and dependencies by their workspace
  roles; use matching grouped sorting in formatting, hooks and release checks.
- Prepare remaining local Make/hook/setup fixtures through Shared's declared
  dependency closures; keep product assertions and supplier-owned fixtures separate.
- Correct current API/backend/dependency guidance, document capability owners,
  and archive superseded migration handoffs without changing timer contracts.
  Retain the size probe's direct Candid dependency required by CDK macro output.
- Own mixed-policy trap/exhaustion, overdue coalescing and volatile upgrade
  qualification previously duplicated in IcyDB's standalone timer probe.
- Qualify the owning runtime proof and isolated Testkit decoder retirement;
  retain published Testkit 0.53 until its replacement is available. Track IcyDB's
  corresponding proof retirement in [#379](https://github.com/dragginzgame/icydb/issues/379),
  with database lifecycle qualification remaining consumer-owned.
- Adapt probe calls and startup to selected Testkit 0.53, and select explicit
  Cargo child custody, environment and retention through selected IC Build 0.17.

- Adopt committed Shared Tooling 0.24.0, retaining canonical uploader 7.0.2,
  direct-only delivery and complete failure-evidence semantics
  ([#67 — Adopt the canonical Shared artifact uploader 7.0.2](https://github.com/dragginzgame/ic-timers/issues/67)).
- Prepare script/Make fixtures from declared companion closures, retire the
  Make-only copier, and delegate host-only compact verification to Shared.
  Keep supplier CI fixtures at their owner and preserve Timers' actual workflow.
- Common Cargo setup now prepares only the selected sorters; Candid extraction
  is not a Timers tool requirement.

## [0.27.1] - 2026-10-17

### Added

- Add `WatchdogRunResult::completed_batch` for caller-defined work counts and
  optional deadlines, keeping failure classification and scheduling policy local
  ([#64 — Share completed-batch watchdog result construction used by Jobs](https://github.com/dragginzgame/ic-timers/issues/64)).

### Changed

- Initialize fresh lifecycle reconciliation through the direct-registration owner,
  retaining atomic publication and capture cleanup without a second control pass.
- Share ordinary ensure-command arbitration after policy admission, and retain
  typed wakeup routing so abandonment guards belong only to ordinary deliveries.

### Fixed

- Align the Testkit adapter fixture with host/Cargo-only aggregate setup and
  checking after retiring the IC-tool selection.
- Classify the complete library directory and packaged root README/lockfile as
  crate impact, including license changes previously labelled repository-only.

### Development

- Adapt the probe driver to selected IC Build 0.14's typed Cargo recipe API,
  preserving the locked profile and exclusive cohort feature selections.
- Adopt reviewed Shared Tooling 0.20.0 with common host/Cargo setup.
- Retire the unused qualified IC-tool selection and CI PATH; require host-only
  compact evidence while retaining unselected legacy sets and failed diagnostics.
  Keep generic IC installer qualification at Shared rather than in Timers' gate.
- Complete the downloaded-evidence fixture's catalog-parser companion copy.
- Share one direct-only release declaration between Make and standalone adapters;
  reject unsupported delivery selections before effects.

## [0.27.0] - 2026-10-17

### Breaking

- Remove `TimerProcessCondition` and `TimerSnapshot::process_condition()`.
  Consumers own operator-health labels and derive them from snapshot state,
  scheduling mode and outcomes. Update status and metrics adapters together with
  the dependency selection.

### Changed

- Reuse checked initial schedules and prepare completion schedules together with
  their snapshots, preserving request validation and pending-command arbitration.
- Apply timer effects using the handles detached by their transition owner;
  remove fallback registry lookups while retaining binding cleanup and rollback.
- Commit final provider-handle admission and successful-arm accounting together;
  preserve staged Watchdog recovery and remove the separate confirmation step.

### Development

- Adapt probe Candid calls and installation to Testkit 0.51, preserving anonymous
  callers, explicit funding and finite reply-decoding limits.
- Complete the failure-evidence fixture's synthetic host catalog and copied
  installer companions for strict offline catalog validation.
- Adopt the reviewed Shared 0.19.0 snapshot and retire release-prose scanning and
  its fixtures. Structured release metadata and changelog checks remain required.
- Delegate isolated probe builds and exact Wasm artifact discovery to IC Build;
  retain artifact ownership until PocketIC subjects finish reading their inputs.
- Copy only the required release adapter into committed-release fixtures and
  remove duplicate changelog-helper copying from version-preparation fixtures.

## [0.26.1] - 2026-10-17

### Development

- Share checked duration/deadline arithmetic between scheduling and backend
  admission while preserving each boundary's error contract.
- Keep hook tests focused on actual consumer formatting and refusal; consolidate
  Make flag aliases and environment permutations at the Shared owner.
- Construct timer effects with valid callback roles and one Watchdog dispatch
  identity; retain live ownership/generation checks and partial-arm cleanup.
- Keep generic archive-entry rejection coverage with Shared's owner suite;
  retain local evidence identity, qualified-pin and collector integration checks.
- Use Testkit's fixed-round progression directly in PocketIC probes and supply
  explicit finite Candid reply limits required by Testkit 0.50.
- Update the owned-backend handoff for the 0.26.0 release, distinguishing retained
  candidate evidence from published-version downstream adoption and open qualification.

## [0.26.0] - 2026-10-16

### Breaking

- Replace the CDK timer provider with an independently implemented private
  one-shot backend. Composed canisters must remove other global-timer providers
  and resolve one IC Timers package identity. Cargo rejects duplicate packages
  declaring the new ownership key; older releases still require graph checks.
  Existing registration policies and durable application records are retained.
- Defer dispatch by one second after insufficient cycles, submission failure or
  transport saturation. A top-up does not bypass that delay. Transient responses
  retry only a still-pending task, without replaying accepted work.

### Changed

- Remove cancelled deadline records immediately and bound pending tasks and
  outstanding self-calls separately. Expose typed capacity and identity-exhaustion
  errors; running async callbacks retain their cancellation semantics.
- Keep MIT licensing for the independently authored backend, retain the CDK async
  executor, and confine unsafe IC entrypoints/callback registration to one private
  module. See the [0.26 design](../../docs/timers/design/0.26-owned-timer-backend.md) for the
  message-boundary contract and qualification requirements.

### Development

- Use the retained runtime module for release-index dirty-source checks after
  the platform module moved into a directory.
- Replace the retired timer-provider pin exception with the executor 2.0.0
  compatibility exception required by dependency-policy CI.
- Qualify seven IcyDB lifecycle/startup subjects with the owned dispatcher and
  prepare isolated Canic/IcyDB adoption patches. Canic's supported probe builds
  remain blocked by fixture Candid-export type-resolution errors.
- Cover admission failure before both Watchdog arms, including cleanup after the
  successor is already installed. Clarify the Cargo ownership key's limits for
  older releases that do not declare it.
- Check Canic core/control-plane and IcyDB lifecycle Wasm composition against the
  candidate; identify downstream dispatcher admission, ingress-test and provider
  graph assertions that must change during coordinated adoption.
- Qualify the unchanged Jobs consumer against the candidate backend in an isolated
  copy: 42 native tests and seven live recovery subjects pass, including upgrades
  and uncertain-result reconciliation. Downstream version adoption remains separate.
- Verify cancellation and registration replacement during submission deferral,
  including stale authenticated delivery and rescheduling after cancellation.
- Exercise real self-call queue pressure on PocketIC, retaining pending timer
  work through submission deferral and completing it once after the retry delay.
- Exercise real dispatch-response traps on PocketIC for both ordinary policies,
  verifying cleanup preserves pending work for delayed retry and later dispatch.

## [0.25.2] - 2026-10-16

### Development

- Keep CI ordering and failure propagation tests in one fixture; retain separate
  repository-only admission and provider-boundary checks.
- Exercise workspace-version read failures through actual version preparation
  instead of repeating the shared reader's malformed-input matrix.
- Adopt Shared's reader-owner tests and narrower Make helper export from
  [Shared #241 — Keep exhaustive qualification at Shared owners](https://github.com/dragginzgame/shared-tooling/pull/241).
  Retain parallel-dispatch support and include automatic cleanup-refusal probes.
- Keep exhaustive logger qualification with Shared; retain Timers' actual Make
  routing, failure status, raw evidence, summaries and nested-checkout checks.
- Inline release-impact admission at the version-preparation boundary and remove
  its single-caller script. Verify rejected classifications before any mutation.
- Reuse Shared's best-effort fixture log replay while preserving saved failure
  status and scratch retention in release-index and committed-release checks.
- Move generic workflow-inventory regressions to Shared's dependency checker.
  Keep a local caller smoke check and Timers' orchestration/provider boundaries
  ([Shared #236](https://github.com/dragginzgame/shared-tooling/pull/236)).
- Adopt Shared 0.15.8 and its per-target Make admission probe. Keep Timers'
  actual targets, routing and no-effect checks while leaving exhaustive flag
  permutations with Shared.
- Send only the measured memory-calibration intervals from the size probe;
  derive overhead once in the native report. Update the unpublished producer
  and decoder together without changing the sampling boundaries.
- Retire generic hook tree-observation and formatter-rollback cases already
  qualified by Shared. Keep actual Timers Make, workspace, lockfile, diagnostics
  and nested partial-staging integration checks.
- Narrow measurement tests to role isolation and forwarding through IC Metrics;
  retain empty/zero, wide samples and memory pairing without duplicating the
  dependency's arithmetic qualification.
- Remove the clean-source forwarding script. Make retains the existing-commit
  guard and calls Shared's source checker directly; publication fixtures exercise
  the real target, including failed Git observations and unborn checkouts.

## [0.25.1] - 2026-10-16

### Development

- Reuse Shared's fixture exit runner for completion and collector probes. Retain
  the local source roster, scratch-path contexts and collector diagnostics;
  remove duplicate exit injection, status and retained-directory checks.
- Call the canonical exact-tag checker directly from Make. Remove the local
  zero/two-argument adapter; retain fail-closed HEAD/version resolution and
  frozen-commit release checks at their real callers.
- Simplify probe fatal-result branches while retaining the same IC trap messages
  and typed rejection paths. Qualify current probe costs and recovery separately.
- Inline three single-use native probe transport wrappers at their call sites.
  Keep explicit Candid result shapes and the existing Testkit call semantics.
- Validate size-probe cohort selection with one compile-time count assertion;
  retain all four valid selections and reject every other feature combination.
- Align the release-gate fixture with the runtime-probe native regression already
  included in `make test`, including failure propagation from that command.
- Reuse Shared's evidence upload action for Timers' prepared archive and original
  collection log. Keep local evidence selection and qualify upload of available
  diagnostics when archive creation fails.
- Replace three runtime-probe command flags with one next-work decision. The
  latest request wins and normal execution consumes it once; traps retain it.
- Use one private instruction-measurement bracket in the size probe, preserving
  result wire records. Before/after PocketIC cohorts retain operation and callback
  instruction costs; probe Wasm size changes are recorded in the current evidence.

## [0.25.0] - 2026-10-15

### Breaking

- Raise the minimum supported Rust version to 1.91.0 across manifests, native/
  Wasm qualification, probe commands and hosted CI. This also admits the incoming
  workspace-selected Metrics 0.11 minimum compiler requirement.

### Development

- Adopt Shared's common hosted tool-preparation action and Make-fixture bootstrap.
  Retain local compiler/cache/product policy and fixture stubs/assertions; remove
  the private action and repeated common-file copy lists.
- Remove tracked Python bytecode and ignore interpreter cache directories.

- Reuse Shared's focused snapshot/archive verifier fixtures. Keep Timers' actual
  export, collector, source/run identity and qualified-pin checks; retire generic
  rejection permutations from the local fixtures.
- Call canonical workspace-version and graph readers directly. Remove their
  forwarding scripts and adapter-only graph fixture; retain stable-version,
  worktree-selection, failure-status and pre-mutation checks at real callers.

- Call Shared's release-prose advisory directly and retire the local forwarding
  script and forwarding-only fixture. Preserve pre-mutation, non-blocking checks.

- Delegate multi-result probe calls to the selected Testkit Candid helpers;
  retain the raw provider-ingress rejection boundary and existing reply shapes.
- Reuse Shared's fixture-prefix renderer for collector failure probes, retaining
  the complete exit/completion and evidence assertions.
- Share explicit Rust installation across local setup and hosted CI. `update-dev`
  now prepares the selected MSRV with Clippy/rustfmt and the Wasm target too;
  checks continue to require prepared inputs without installing them.

- Consolidate Shared snapshot ownership and reuse the canonical changelog
  preflight and workflow-step extraction owners from
  [Shared #217](https://github.com/dragginzgame/shared-tooling/pull/217). Retire
  the nested helper copies, obsolete manifests and independent changelog writer.
- Keep the active adoption guide compact; preserve source-bound receipts and
  historical links in its evidence archive.

## [0.24.0] - 2026-10-15

### Breaking

- Remove `TimerObservabilitySnapshot::epoch()`. Read the epoch through
  `TimerSnapshot::registration_id().epoch()` and keep that registration identity
  alongside copied measurements. Registration-local statistics no longer retain
  a duplicate epoch, so equality compares measurements alone. Cancellation and
  registration replacement semantics remain.

### Development

- Adopt reviewed Shared 0.15 plus the independent hosted-evidence verifier at
  exact contribution `58bad48`. Delegate release metadata mutation/rollback and
  partial tool-evidence collection to their canonical owners; remove local
  version setters and the lock update adapter.
- Use Shared's fixture-prefix renderer while preserving the fifteen-fixture
  path/failure matrix and selected mandatory assertion probes.
- Keep the active release guide and session handoff compact. Preserve earlier
  receipts in release/status archives and point historical links to their owner.

### Fixed

- Update unpublished probe callers for the incoming Testkit 0.45 fallible-only
  Candid/install API, retaining explicit failure expectations at test boundaries.
- Include the existing MIT license text in the published `ic-timers` crate.
  Preserve the root license's holder, year and wording.

## [0.23.2] - 2026-10-15

### Changed

- Remove duplicate normalization from private erased callback results. Keep
  public result normalization and registry terminal arbitration; the adversarial
  registry test now receives the contradictory directive it intends to reject.

### Development

- Fill the capacity probe from one inventory observation and retain only its
  extra-name counter, rather than unused registration handles and repeated
  full-inventory copies.
- Delegate controlled hosted-scenario artifact verification to Shared Tooling,
  retaining Timers' source/run identity, qualified pins and collector coverage.
- Share the complete CI tool preparation sequence across Linux, MSRV and macOS,
  preserving failed-command statuses, captured logs and PATH publication order.
- Keep registry-only fixture methods and dummy callback factories in the test
  module rather than the production implementation.
- Delegate hosted qualification admission and evidence-parent creation to Shared
  Tooling. Keep Timers' path selection and require rejection before filesystem
  effects, including for invalid stages, nonmanual runs and foreign checkouts.

## [0.23.1] - 2026-10-15

### Development

- Delegate manifest ordering and immutable release-source custody to Shared's
  metadata checker, retaining exact-commit admission and failed-check evidence.
  Explicitly preserve failed sorting/metadata producer statuses under Bash 3.2.
- Consolidate duplicate native probe helpers and move wide `ProbeSnapshot`
  serialization coverage into the native protocol suite, including optional
  registration and memory fields.

- Share the four failure archive/upload pairs through one local action, retaining
  Timers' archive layout, artifact identity and upload after collection failure.
- Delegate selected assertion-retention probes to Shared Tooling and use its
  physical-path allocator throughout the 15 consumer-owned shell fixtures.
  Qualify relative, spaced and symlinked temporary roots without weakening
  failure-status, completion or retained-evidence checks.
- Refresh the three canonical snapshots from the qualified Shared assertion
  contribution; include the release-input validator in the disposable version
  preparation fixture's companion files.

## [0.23.0] - 2026-10-15

### Breaking

- Adopt the selected Metrics 0.10 summary API. Project instruction count, total,
  maximum and integer means through `summary.aggregate()`; `summary.latest()`
  remains the ordered observation. The public `MeasurementSummary` re-export
  follows that hard cut without forwarding shims. Recording, callback attribution,
  empty/zero behavior and probe wire fields remain unchanged.

### Development

- Adopt the qualified Shared release-observation repair across all three
  snapshots. Fix Bash 3.2 empty-array admission and compare against the verified
  base commit; retain current snapshot roots, tools and application dependencies.

- Keep tag CI focused on actual annotated-tag/version, main reachability,
  adopted-snapshot integrity and locked-graph admission. Leave broad release
  regressions in PR/main and the owner release gate. Qualify the actual workflow
  from a shallow checkout, including rejected tags, corrupt snapshots, failed
  version reads and failed dependency preparation
  ([#55](https://github.com/dragginzgame/ic-timers/issues/55)).

## [0.22.6] - 2026-10-14

### Changed

- Share immutable timer identity components across clones, retaining validation,
  value-based equality/ordering/hashing, debug output and `Send`/`Sync`. Callback
  tokens and snapshots no longer allocate three copied strings per identity clone.

### Development

- Measure cohort cancellation through the exact registration's armed-wakeup
  query instead of constructing a diagnostic snapshot. Treat lookup failures as
  probe failures; verify repeated cancellation and absence of later callbacks.
- Share checked failure-window construction and the pre-failure schedule proposal
  between trap and instruction-exhaustion probes, retaining independent windows,
  trap precedence and the same-message rollback experiment.

## [0.22.5] - 2026-10-14

### Development

- Reuse Testkit's bounded readiness poller for five probe waits, preserving
  tick-before-observation boundaries and reporting the last pending observation.
- Build the four policy cohorts through one fail-fast recipe, retaining explicit
  mutually exclusive features and separate target directories.
- Delegate release-prose heuristics and semantic fixtures to the exact Shared
  Tooling contribution ([#191](https://github.com/dragginzgame/shared-tooling/pull/191));
  retain local caller binding and advisory pre-mutation invocation.
- Move complete release-change observation and generic Git regressions to Shared
  Tooling; retain local package-impact policy and reject failed partial streams.
- Propose zero-duration polling without redundant simulated-time requests in
  Testkit ([#96](https://github.com/dragginzgame/ic-testkit/pull/96)); local dependency
  adoption awaits owner publication.

## [0.22.4] - 2026-10-14

### Development

- Adopt reviewed Shared Tooling 0.12.8 across all three snapshots, including its
  focused development/handoff validation cadence and publication-guide closure.
  Retire the local compile-only approval rule; complete release qualification
  retains its owner-operated gate.
- Reuse Testkit's canister installation lifecycle across the recovery and policy
  fixtures, retaining fresh instance ownership, explicit funding and the existing
  measurement boundaries.
- Keep Cargo graph regressions with their Shared implementation; narrow the local
  fixture to selected-worktree admission, adapter binding and failure propagation.

- Prepare the complete root-lock dependency cache before hosted CI's offline
  release fixtures, including dependencies used only by unpublished probes.

## [0.22.3] - 2026-10-14

### Development

- Adopt Shared Tooling 0.12.3 across the three reviewed snapshots. Use its
  literal workspace-version renderer, fixture EXIT lifecycle and coordinated
  release metadata checker instead of local copies. The refresh includes the
  mixed-snapshot inventory fixture repair without changing tool selections.
- Check the complete workspace's lock identities against the selected release
  tree, retaining local manifest ordering, staged-source admission and Testkit
  preparation.
- Adopt the strict numbered changelog contract: reject versionless `Draft`,
  misplaced pending releases and missing files before mutation; preserve empty
  note advisories, historical bytes and metadata rollback.
- Reuse Shared's coordinated member projection and graph-only admission in the
  bump path; remove the local JSON projections and duplicate version comparison.
- Delegate generic downloaded archive/tool-receipt verification and controlled
  hosted failure stages to Shared, retaining Timers' identity and scenario
  assertions, qualified pin selections and product gates.

## [0.22.2] - 2026-10-14

### Changed

- Require Metrics 0.9.1 in the root dependency catalog. Its arithmetic crate
  source and public summary types are unchanged from 0.9.0; existing overflow,
  callback attribution and scheduling semantics are preserved.

### Development

- Verify downloaded failure evidence against the same qualified IC tool catalog
  used by setup and collection. Cover the actual verifier command when shared
  defaults differ, and reject changed or missing qualified pins.
  ([#53](https://github.com/dragginzgame/ic-timers/issues/53))
- Reuse the adopted Shared source-cleanliness checker for publication, retaining
  the committed-source requirement and failed-observation checks.
- Use Testkit's typed Candid calls for single-result probe replies, keeping raw
  calls for multiple reply arguments and the private executor rejection test.
  Timer APIs and execution semantics are unchanged by these harness changes.
- Refresh README API guidance and describe the combined 60-second Testkit
  startup budget used by the current harness.

## [0.22.1] - 2026-10-13

### Changed

- Resolve callback retry delays through the same checked relative-deadline path
  as initial scheduling, retaining retry classification and typed terminal
  failures. Extend the existing boundary fixture for zero and maximal encoded
  delays; public APIs and timer semantics are unchanged.

### Development

- Remove the independent exact-version delivery path and manual staging,
  commit/tag and push targets. Use the shared patch/minor/major runner and
  exact saved-intent resume; preserve publication admission and version-preparation
  rollback checks. Fresh arbitrary-version delivery is no longer a local command.
- Define the seven probe wire records once in an unpublished protocol package,
  preserving borrowed canister cohort labels and owned native decoding. Simplify
  empty/populated observations without changing fields or measurement boundaries;
  the published timer API and dependency graph are unchanged.
- Let Testkit own fresh PocketIC server/instance construction and drop ordering
  across recovery and cohort fixtures. Use one 60-second startup deadline in
  place of two 30-second phase budgets; retain explicit admitted-binary selection
  and the existing canister assertions.
- Propose literal workspace-version rendering upstream
  ([Shared #159](https://github.com/dragginzgame/shared-tooling/issues/159)) and
  report IC Build's default library-name admission blocker
  ([#16](https://github.com/dragginzgame/ic-build/issues/16)). Retain qualified
  local release writing and probe builds pending equivalent upstream support
  and a demonstrated benefit from build adoption.
- Refresh all three reviewed Shared Tooling snapshots to 0.10.2, retaining
  literal Cargo-tool cache identity after unrelated pin/comment/order changes
  and tracing canonical catalogs and explicit exceptions during adoption review.
  Current timer/runtime code and dependency selections are unchanged.
- Reconcile delivered 0.22.0 Metrics/catalog adoption against
  [#51](https://github.com/dragginzgame/ic-timers/issues/51) and
  [#52](https://github.com/dragginzgame/ic-timers/issues/52), distinguishing
  completed owner release validation and tag truth from
  the remaining hosted native qualification.

## [0.22.0] - 2026-10-13

### Breaking

- Adopt Metrics 0.9 for the public measurement-summary identity. Consumers
  exchanging summaries must select Metrics 0.9 or the Timers re-export; update
  actual-overflow accessors to `*_overflowed` and error matches to the new
  overflow names, removing unreachable mean/interval error variants. Numeric
  accounting and timer lifecycle semantics remain unchanged.

### Changed

- Record callback measurements directly from the captured start, simplifying
  private accounting while preserving sampling order, stale-registration
  rejection and public timer semantics. The isolated Wasm experiment measured
  no binary-size change from this simplification.

### Development

- Adopt Shared Tooling 0.10.0 across all three snapshots. Common catalogs now
  belong to the reviewed export; keep Binaryen 132 in an explicitly selected
  qualified exception for Make and failure evidence. Maintenance/LOC path
  rejection and release-lock diagnostics retain native failures.
- Review the incoming Testkit 0.37 and Host 0.17.1 registry selections, preserving
  the root dependency catalog and lockfile. Host stays in the unpublished native
  harness graph; provider and PocketIC ownership are unchanged.
- Record whole-canister LTO experiments and hand reusable build guidance to
  IC Build. Larger applications retain their own final-artifact qualification;
  timer profiles and public APIs are unchanged.
- Adopt Shared Tooling 0.9.14 to recover missing failed-step logs from only the
  selected run attempt's failed jobs, retaining original and recovered evidence.
  Missing or partially retrieved logs still fail inspection
  ([Shared #149](https://github.com/dragginzgame/shared-tooling/issues/149)).

## [0.21.4] - 2026-10-13

### Changed

- Store erased callbacks directly in their shared owner, removing redundant
  nested boxing while preserving mutable callback and capture lifetimes.
  Resolve lifecycle owner slots in one branch, removing an impossible conflict
  path. Public APIs, scheduling and failure semantics are unchanged.

## [0.21.3] - 2026-10-13

### Development

- Adopt Shared Tooling 0.9.13: preserve filesystem lock-acquisition and Cargo
  source-download errors with their original statuses, and require an explicit
  CI selection when the current branch is unavailable. Refresh common Cargo
  tools with `make install-tools` before validation; earlier builds remain
  retained. The timer runtime is unchanged
  ([Shared #145](https://github.com/dragginzgame/shared-tooling/issues/145),
  [#146](https://github.com/dragginzgame/shared-tooling/issues/146),
  [#148](https://github.com/dragginzgame/shared-tooling/issues/148)).

## [0.21.2] - 2026-10-13

### Development

- Adopt Shared Tooling 0.9.10 for Cargo-cache and restored tool-build checks,
  and compiler-isolated, commit-free Rust-tool qualification. Remove the
  temporary fixture invocation adapter; the timer runtime is unchanged
  ([Shared #142](https://github.com/dragginzgame/shared-tooling/issues/142),
  [#143](https://github.com/dragginzgame/shared-tooling/issues/143),
  [#144](https://github.com/dragginzgame/shared-tooling/issues/144)).
- Apply shared change-reporting rules for distinct deletion/move/rename counts
  and scoped Rust, type and documentation metrics
  ([Shared #141](https://github.com/dragginzgame/shared-tooling/issues/141)).

## [0.21.1] - 2026-10-12

### Development

- Run the canonical Rust-tool fixture from an isolated directory during release
  checks, so the checkout's toolchain file cannot trigger its mock staged-compiler
  rejection. Preserve the fixture's failure status and retained evidence
  ([Shared #143](https://github.com/dragginzgame/shared-tooling/issues/143)).
- Refresh all three Shared Tooling snapshots to 0.9.7 for bounded transient
  tool-download retries, complete Git identity checks and typed Cargo offline
  configuration admission. Explicit Cargo-tool setup refreshes the changed
  build-policy identity while retaining earlier builds.
- Select Testkit 0.35 and its Host 0.15 graph for unpublished probes, removing
  unused build-tool dependencies without adding a direct Host or IC Build
  dependency. The published timer API and runtime are unchanged.

## [0.21.0] - 2026-10-12

### Breaking

- Adopt Metrics 0.8 for the public instruction-summary type. Consumers that
  exchange summaries or match their mean errors must select Metrics 0.8 or use
  the Timers re-export. Tracked recording, wide totals and timer execution
  contracts remain unchanged; fractional reports can use the summary's checked
  `scaled_mean(scale)` directly
  ([#50](https://github.com/dragginzgame/ic-timers/issues/50)).
- First lifecycle reconciliation publishes a new registration only after its
  requested state succeeds. A failed first call leaves the owner slot empty
  and releases the new callback and handles, so a retry installs its supplied
  callback. Later failures retain existing authority; signatures stay unchanged.

### Testing

- Cover first-reconciliation rejection, provider binding/confirmation failure,
  capture cleanup and retry across all policies, preservation of existing
  authority, and unused callback-factory destruction ordering. Add the owning
  canister-probe/PocketIC rejection subject.
- Advance the first-reconciliation PocketIC fixture from its current clock;
  avoid a backward absolute-time reset that PocketIC correctly rejects.

## [0.20.0] - 2026-10-12

### Breaking

- All direct registration functions take `TimerStart` instead of separate
  lifetime and optional schedule arguments. `Retained { initial }` permits
  inactive declarations; `RemoveWhenStopped { initial }` requires a schedule.
  Remove `TimerError::InitialScheduleRequired`; an inactive transient start is
  unrepresentable. Lifetime observations, lifecycle reconciliation and callback
  execution semantics retain their existing contracts.

### Testing

- Update native and canister-probe constructor subjects, capture-release and
  provider-failure coverage, and policy cohorts to use typed starts. Cover the
  invalid inactive-transient construction at the compile-time boundary.

### Development

- Adopt Shared Tooling 0.9.1's common host/IC installation lifecycle. Attempt
  shared-lock release even if checkout cleanup fails, preserve the original
  installation status and failed candidates, and fail on cleanup-only errors.
- Refresh to Shared Tooling 0.9.2: reject NUL bytes in evidence checksum
  manifests before hashing, and reject supplied/resolved installer paths
  containing CR/LF before configuration loading or setup.
- Align the audit overlay with its reviewed snapshot and the approved locked,
  offline compile-only development exception; other validation stays user-owned.

## [0.19.3] - 2026-10-12

### Development

- Refresh all three snapshots from Shared Tooling's corrected 0.9.0 revision,
  aligning canonical version annotations with the adopted source. Selected
  payloads and timer behavior are unchanged
  ([Shared #137](https://github.com/dragginzgame/shared-tooling/issues/137)).

## [0.19.2] - 2026-10-12

### Development

- Adopt Shared Tooling's requested 0.9.0 source for stricter snapshot records,
  atomic payload replacement and complete Make installer dependencies
  ([Shared #131](https://github.com/dragginzgame/shared-tooling/issues/131),
  [#133](https://github.com/dragginzgame/shared-tooling/issues/133)).
- Use the canonical shared snapshot reader, rejecting NUL bytes before parsing
  and keeping syntax admission separate from payload integrity and ownership.
- Preserve both Cargo build observations when setup refuses changed inputs,
  and reject redirected diagnostic files
  ([Shared #134](https://github.com/dragginzgame/shared-tooling/issues/134),
  [#136](https://github.com/dragginzgame/shared-tooling/issues/136)).
- Bind formatting to the reviewed Rust catalog and cover external catalog
  overrides in the existing hook fixture
  ([Shared #135](https://github.com/dragginzgame/shared-tooling/issues/135)).
- Read workspace LOC counts as JSON with `bash scripts/dev/cloc.sh --json`,
  using the same counted rows as the human-readable report.

## [0.19.1] - 2026-10-12

### Development

- Adopt Shared Tooling 0.8.0 across all three snapshots. Recorded entrypoints
  derive required files, and refresh automatically retires verified obsolete
  payloads. Existing local selections and guards remain intact
  ([Shared #131](https://github.com/dragginzgame/shared-tooling/issues/131)).
- Select cargo-sort-derives 0.14.1, preserving literal `$crate` paths in derive
  sorting. Prepare the selected common toolset with
  `make install-tools`, then verify it with `make tools-check`
  ([Shared #113](https://github.com/dragginzgame/shared-tooling/issues/113)).
- Refresh the root-locked Testkit dependency graph to the coherent Host 0.14.4
  family. Host library Rust source and the timer API are unchanged.

## [0.19.0] - 2026-10-12

### Breaking

- Adopt Metrics 0.7 for public instruction summaries: totals and integer means
  are `u128`, while samples, counts, latest and maximum remain `u64`. Totals
  beyond the old limit retain exact means. Update public Metrics identities and
  report/codec fields together; narrow only with checked conversion
  ([#47](https://github.com/dragginzgame/ic-timers/issues/47)).

### Development

- Widen both canister probes and their matching PocketIC reports without
  truncation. Cover exact totals above `u64::MAX`, role/order isolation and wide
  Candid round trips.
- Adopt Shared Tooling 0.6.5's common Make entrypoint and staged-formatter receipt
  admission. Replace duplicate include, hook and snapshot wiring while retaining
  local release and Testkit
  policy ([Shared #130](https://github.com/dragginzgame/shared-tooling/issues/130)).
- Run compiler and Clippy checks before lengthy release fixtures, retaining
  every existing gate and fail-fast behavior. Permit offline, locked compile-only
  development checks while keeping tests and lint execution maintainer-owned
  ([#48](https://github.com/dragginzgame/ic-timers/issues/48)).
- Check the library's Wasm path with the advertised Rust 1.88 minimum in the
  existing MSRV gate. Prepare that target explicitly in hosted MSRV setup and
  cover command ordering and failure propagation
  ([#49](https://github.com/dragginzgame/ic-timers/issues/49)).

## [0.18.0] - 2026-10-11

### Changed

- Hard-cut `register_once`, `register_after_completion` and `register_watchdog`
  to take an explicit initial schedule before the callback. Scheduled creation
  returns authority only after provider binding; failure retires its exact new
  declaration and provider handles before releasing callback captures. Inactive
  construction remains available for retained declarations. Transient
  `RemoveWhenStopped` creation without a schedule returns the typed
  `InitialScheduleRequired` error. Update direct callers together; there is no
  old-signature forwarder.
- Select the first recurring deadline independently of its configured cadence.
  Initial `After`/`At` requests use their existing explicit scheduling modes,
  including zero delay; configured recurrence and callback continuation modes
  remain unchanged after dispatch.

### Development

- Fix the watchdog invariant-failure fixture's registration binding and
  address initial-registration Clippy diagnostics without relaxing lint gates.
- Update runtime and size probes to exercise scheduled creation directly. Add
  native input, capacity and provider-failure cases plus a real-canister invalid
  initial-schedule subject. Qualification and Wasm/instruction measurements
  remain pending under the complete maintainer-operated gate.

## [0.17.14] - 2026-10-11

### Development

- Verify controlled early-failure download evidence at its current Shared tool
  store generation, replacing the obsolete checkout-local path check. Reject
  malformed, duplicate and linked candidates, and pass actual producer/collector
  output through the existing Ubuntu download-verifier suite
  ([#44](https://github.com/dragginzgame/ic-timers/issues/44)).

## [0.17.13] - 2026-10-11

### Fixed

- Release retained and inactive callback captures before native test-fixture TLS
  teardown. Drop detached registry ownership outside its borrow while keeping an
  empty initialized runtime available to capture destructors. Extend cleanup
  coverage to both declaration lifetimes and inactive, queued and suspended work
  ([#46](https://github.com/dragginzgame/ic-timers/issues/46)).

### Development

- Adopt Shared Tooling 0.6.3. Normal direct release targets restart superseded
  preparation intent from a clean descendant commit, archive the original plan,
  select the requested increment from current metadata and repeat complete
  validation. Exact resume and staged/later recovery retain their existing
  boundaries ([Shared #126](https://github.com/dragginzgame/shared-tooling/issues/126)).
- Refresh the locked native test graph to Testkit 0.33.8. Its packaged CLI lock
  now selects Host 0.13.5, matching the workspace graph.

## [0.17.12] - 2026-10-11

### Fixed

- Release the native substitute's task-map borrow before dropping cancelled or
  discarded futures. Captured-value destructors can inspect and schedule timers
  during removal, matching the production provider's removal boundary. Cover
  queued cancellation, suspended cancellation and queued discard. Production
  timer behavior is unchanged
  ([#45](https://github.com/dragginzgame/ic-timers/issues/45)).

### Development

- Adopt Shared Tooling 0.6.2. Interrupted releases now report the saved plan,
  phase, candidate and both source commits, with guarded recovery guidance
  ([Shared #126](https://github.com/dragginzgame/shared-tooling/issues/126)).
  Document the local version helper's temporary-backup limits before retiring
  preparation intent. Source-bound validation and complete release gates remain
  required.
- Refresh the locked native test graph to Host 0.13.5 through Testkit.

## [0.17.11] - 2026-10-11

### Development

- Refresh the locked graph to Metrics 0.6.4, Testkit 0.33.6 and Host 0.13.4.
  Host stays transitive through Testkit; the timer API and PocketIC selection
  remain unchanged.
- Adopt Shared Tooling 0.6.1. Cargo setup now stops on failed lock acquisition,
  receipt admission or activation, preserves the original failure and evidence,
  and cleans up only locks it acquired
  ([Shared #120](https://github.com/dragginzgame/shared-tooling/issues/120)).
  Run `make install-tools`, then `make tools-check`; updated build-policy
  identities select verified generations while retaining previous installations.
- Correct the shared Cargo fixture's macOS padded build-count comparisons and
  exercise padded counts on every host
  ([Shared #124](https://github.com/dragginzgame/shared-tooling/issues/124),
  [#44](https://github.com/dragginzgame/ic-timers/issues/44)).

## [0.17.10] - 2026-10-11

### Development

- Adopt Shared Tooling 0.6.0 so common Cargo tools and the root-lock-selected
  Testkit CLI reuse verified shared builds. Run `make install-tools` followed by
  `make tools-check`; tool builds use the standard environment while application
  build settings remain local. Retain referenced shared Cargo build evidence
  without selecting the local Rust tree twice
  ([#44](https://github.com/dragginzgame/ic-timers/issues/44),
  [Shared #120](https://github.com/dragginzgame/shared-tooling/issues/120)).

### Fixed

- Preserve local CI failure logs, Rust build evidence and tool references when
  shared-store admission fails. Archive the rejection diagnostic and mark shared
  collection incomplete while retaining its original failure status
  ([Shared #121](https://github.com/dragginzgame/shared-tooling/issues/121),
  [#44](https://github.com/dragginzgame/ic-timers/issues/44)).

## [0.17.9] - 2026-10-11

### Development

- Adopt Shared Tooling 0.5.0 and its canonical Rust installation contract. Common
  formatters and the Candid extractor use verified versioned slots and
  `.tools/rust/common/bin`; update shell PATH exports and run `make install-tools`
  followed by `make tools-check`. Retain previous installations without an old
  PATH fallback ([#44](https://github.com/dragginzgame/ic-timers/issues/44)).

- Retain completed Rust tool slots, receipts and build logs alongside failed
  candidates in CI failure archives; cover byte/mode retention and links without
  following external targets ([#44](https://github.com/dragginzgame/ic-timers/issues/44)).
- Reuse verified host/IC bundles across checkouts through Shared Tooling's per-user
  store while retaining each repository's pins and command paths. Run explicit
  setup to replace active local selections; previous bundles remain retained.
  Keep failed shared-installation evidence and exclude unrelated store entries;
  isolate intentional failure qualification from cached valid bundles
  ([#44](https://github.com/dragginzgame/ic-timers/issues/44),
  [Shared #120](https://github.com/dragginzgame/shared-tooling/issues/120)).

## [0.17.8] - 2026-10-11

### Development

- Cover the already-selected Metrics 0.6 instruction-summary boundaries through
  both timer roles: empty and completed zero, exact cap, tracked overflow and
  retained latest/maximum samples. Check measurement continuity across
  cancellation, registration replacement and native runtime reset
  ([#43](https://github.com/dragginzgame/ic-timers/issues/43)).
- Correct observability guidance to distinguish tracked instruction overflow
  from conservatively saturated event counters; document the Metrics 0.6 public
  type identity and the 0.17.7 release-boundary mistake.

## [0.17.7] - 2026-10-11

### Development

- Adopt Shared Tooling 0.3.11 so Rust tool setup and offline
  checks use the selected repository's toolchain and Cargo configuration when
  invoked elsewhere
  ([Shared #101](https://github.com/dragginzgame/shared-tooling/issues/101)).
- Keep early fatal, LLVM and missing-file diagnostics in retained release summaries
  and GitHub summaries as well as live output; exercise both search backends with
  isolated diagnostics and neutral Rust test names
  ([#42](https://github.com/dragginzgame/ic-timers/issues/42),
  [Shared #110](https://github.com/dragginzgame/shared-tooling/issues/110)).
- Normalize shared host-tool fixture paths and release-command temporary roots,
  including nested Make checks, for symlink aliases and relative `TMPDIR`
  ([Shared #111](https://github.com/dragginzgame/shared-tooling/issues/111),
  [Shared #112](https://github.com/dragginzgame/shared-tooling/issues/112)).
- Refresh all shared snapshots from the corrected producer version so their
  source annotation reports 0.3.11 without a local metadata rewrite
  ([Shared #114](https://github.com/dragginzgame/shared-tooling/issues/114)).
- Record the incoming IC Host 0.12.10 lock selection and retain Testkit as its
  sole consumer; this Host release changes setup tooling, not library Rust code.

## [0.17.6] - 2026-10-10

### Development

- Adopt Shared Tooling 0.3.8 and explicitly stop on failed mandatory Bash 3.2
  assertions in local release guards and fixtures. Reject mismatched release
  versions, invalid lockfile outputs and incorrect macOS hosts before later
  operations, and retain
  contradictory fixture evidence rather than reporting success
  ([#41](https://github.com/dragginzgame/ic-timers/issues/41),
  [Shared #107](https://github.com/dragginzgame/shared-tooling/issues/107)).

## [0.17.5] - 2026-10-10

### Development

- Adopt Shared Tooling 0.3.6: preserve failed Git tree observations before the
  pre-commit hook can copy or stage files, with consumer failure-isolation cases
  ([#40](https://github.com/dragginzgame/ic-timers/issues/40),
  [Shared #106](https://github.com/dragginzgame/shared-tooling/issues/106)).
- Avoid false hook-test failures when Git refreshes its internal index cache;
  continue checking staged content and working-file preservation
  ([#40](https://github.com/dragginzgame/ic-timers/issues/40)).
- Delegate Testkit CLI lock selection to the shared Cargo installer, removing
  the local parser. Setup and offline checks share strict package admission and
  reject selection changes during installation
  ([Shared #96](https://github.com/dragginzgame/shared-tooling/issues/96)).

- Keep only the newest CI run per workflow and branch or PR, cancelling older
  queued and running checks while retaining the existing host matrix and gates
  ([Shared #108](https://github.com/dragginzgame/shared-tooling/issues/108)).

## [0.17.4] - 2026-10-10

### Development

- Isolate release-gate fixture logs from inherited validation directories, so
  deliberately failed checks cannot pollute the caller's retained evidence.
  Production log-directory inheritance remains unchanged.

## [0.17.3] - 2026-10-10

### Development

- Require explicit completion before local CI/release fixtures report success or
  remove inputs. Preserve failure status and incomplete evidence, including Bash
  3.2 premature zero-status exits. Check each actual cleanup boundary through
  disposable copies before fixture work; consolidate the Testkit exit probes
  into that shared local check ([#38](https://github.com/dragginzgame/ic-timers/issues/38)).
- Require completed failure-evidence collection before successful cleanup, so
  premature collector exits fail and retain scratch evidence. Extend the existing
  collection fixture with actual-boundary exit and retention cases
  ([#39](https://github.com/dragginzgame/ic-timers/issues/39)).
- Preserve failed workspace-version reads during release preflight, including
  failures that emit the expected version. Stop before fetch/setup and retain
  the reader's exit status; add failure and mismatch cases to the index fixture.

## [0.17.2] - 2026-10-10

### Development

- Adopt Shared Tooling 0.3.3 and preserve Cargo jobserver access in local Cargo
  and Testkit recipes. Standalone shared tool includes retain execution admission;
  selected shared fixtures and the local Testkit adapter require explicit
  completion before successful cleanup
  ([Shared #99](https://github.com/dragginzgame/shared-tooling/issues/99),
  [Shared #103](https://github.com/dragginzgame/shared-tooling/issues/103)).
- Reject malformed validation nesting depth before target dispatch and require
  explicit runner completion before success, retaining available evidence on
  premature exits ([Shared #104](https://github.com/dragginzgame/shared-tooling/issues/104)).
- Include the shared advisory README freshness task in the maintenance catalog,
  without adding a release gate, automatic rewrite or enabled schedule
  ([Shared #100](https://github.com/dragginzgame/shared-tooling/issues/100)).

## [0.17.1] - 2026-10-10

### Development

- Adopt Shared Tooling 0.3.1: check supported platforms, IC pins and the selected
  Rust/Cargo toolchain before aggregate setup downloads tools. Host-tool failures
  identify the tool, expected version, path and repair command
  ([Shared #101](https://github.com/dragginzgame/shared-tooling/issues/101)).
- Correct current adoption guidance for the released Metrics 0.5 and Testkit
  0.31 graph, including public measurement type identity. Timer behavior and
  dependency selections are unchanged.

## [0.17.0] - 2026-10-10

### Breaking

- Adopt Shared Tooling 0.3.0's complete ordered setup/check: host, IC and Cargo
  tools, then the selected Testkit CLI/server. Prepare the Rust toolchain before
  `make install-tools`, then run `make tools-check`. Direct host-installer callers
  must remove the retired optional ripgrep/cloc flags; existing installations
  and failure evidence are retained
  ([#36](https://github.com/dragginzgame/ic-timers/issues/36),
  [shared #98](https://github.com/dragginzgame/shared-tooling/issues/98),
  [adoption owner](../../docs/timers/evidence/shared-tooling-history.md#shared-tooling-030-complete-toolset)).
- Align public measurement summaries with Metrics 0.4. Consumers exchanging
  these types must select Metrics 0.4 or use `ic_timers::MeasurementSummary`;
  arithmetic and timer lifecycle behavior are unchanged
  ([#37](https://github.com/dragginzgame/ic-timers/issues/37),
  [identity owner](../../docs/timers/design/callback-delivery-ownership.md#ic-metrics-04-adoption)).

### Development

- Remove README version freshness from release preflight, preparation, CI and
  final metadata checks. Delete automatic README rewriting/staging and its
  projection test; keep package, lockfile, changelog and Git integrity guards.
  Periodic advisory review is requested in
  [Shared Tooling #100](https://github.com/dragginzgame/shared-tooling/issues/100).
- Use Testkit 0.30 for native qualification and selected CLI setup, retaining
  its managed PocketIC startup contract. Keep complete setup/check logs in CI
  failure archives and check the local Testkit extension's ordering under
  parallel Make.

## [0.16.7] - 2026-10-10

### Development

- Adopt Shared Tooling 0.2.14's CI inspection fix: report unavailable failed-step
  logs, retain partial logs/fetch errors, and distinguish completed successful
  runs with no failure logs. No CI rerun or timer behavior change is introduced
  ([shared #97](https://github.com/dragginzgame/shared-tooling/issues/97),
  [adoption owner](../../docs/timers/evidence/shared-tooling-history.md#shared-tooling-0214-ci-inspection)).
- Use Testkit 0.29 and Host 0.11 for native qualification and selected CLI setup.
  The existing PocketIC harness needs no adapter; timer APIs remain unchanged
  ([testkit #47](https://github.com/dragginzgame/ic-testkit/issues/47),
  [graph owner](../../docs/timers/evidence/release-history.md#testkit-029-and-host-011-preparation)).

## [0.16.6] - 2026-10-10

### Development

- Check that successful hook formatting and direct format checks preserve
  distinct staged and unstaged lockfile bytes; include Cargo.lock in existing
  mode-refusal and formatter-failure preservation checks
  ([#35](https://github.com/dragginzgame/ic-timers/issues/35)).
- Correct current formatter guidance to include the canonical reporter and
  Shared 0.2.11's independent MAKEFLAGS/MFLAGS admission.
- Refresh all three snapshots to committed Shared Tooling 0.2.13. Reject
  LF/CR directory names before snapshot export/verification can trim them into
  a different checkout; cover all consumer selections and physical aliases
  ([adoption owner](../../docs/timers/evidence/shared-tooling-history.md#shared-tooling-0212-directory-admission),
  [shared #95](https://github.com/dragginzgame/shared-tooling/issues/95)).
- Retain the incoming Testkit 0.28.1, Host 0.10.2 and Metrics 0.3.7 selections.
  Host rejects directory-suffixed publication targets; Metrics arithmetic source
  is unchanged. No timer adapter or public semantic change is needed
  ([graph owner](../../docs/timers/evidence/release-history.md#0166-incoming-host-and-metrics-graph)).
- Prepare the selected Testkit CLI/server after release preflight admission,
  then check it offline; failed CLI admission names its selection and the local
  setup command.
  Add preparation-order/failure, offline-policy and parallel early-refusal cases
  ([adoption owner](../../docs/timers/evidence/shared-tooling-history.md#shared-tooling-0213-selected-cli-preparation),
  [shared #96](https://github.com/dragginzgame/shared-tooling/issues/96)).
- Record the Host 0.11 review: native Testkit tooling owns adoption; the timer
  library has no Host dependency. Keep the selected 0.10 graph until Testkit's
  corresponding release ([upstream #47](https://github.com/dragginzgame/ic-testkit/issues/47)).

## [0.16.5] - 2026-10-10

### Development

- Correct the hook fixture's concise-output assertion to include the existing
  selected-file refresh confirmation after the formatter's one success line.
- Refresh all three snapshots to committed Shared Tooling 0.2.11, retaining
  source-version/revision diagnostics and consumer dependency/tool selections
  ([adoption owner](../../docs/timers/evidence/shared-tooling-history.md#shared-tooling-0211-formatting-and-make-admission)).
- Print one formatting success line; retain full stdout/stderr on failure with
  its exit status and log path. Carry the canonical reporter through disposable
  Make/index fixtures and archive its failed logs in CI
  ([#35](https://github.com/dragginzgame/ic-timers/issues/35)).
- Adopt canonical refusal of unsafe Make modes hidden by cleared or replaced
  MAKEFLAGS, including attempts to overwrite MFLAGS. Extend actual consumer
  release/formatter negatives without adding a local flag parser
  ([shared #30](https://github.com/dragginzgame/shared-tooling/issues/30)).
- Retain the incoming Metrics 0.3.6 lock selection. Its seven published Rust
  source files are identical to 0.3.5, so no arithmetic adapter change is needed;
  the refreshed graph needs separate qualification
  ([graph owner](../../docs/timers/evidence/release-history.md#metrics-036-incoming-graph)).

## [0.16.4] - 2026-10-09

### Development

- Check Make-mode refusal by exit status, absence of release phases and unchanged
  metadata in the version-preparation fixture. Remove its stale error-prose
  assertion, which rejected the correctly functioning Shared 0.2.8 guard.
- Adopt committed Shared Tooling 0.2.8 Make admission. Refuse ignore-errors,
  dry-run, touch and question modes before recipes when invocation flags are
  preserved; use the selected include and running Make executable while retaining
  recursive command arguments.
  Propagate companions to disposable Make/index fixtures and isolate the shared
  release smoke checker from ambient snapshot routing
  ([adoption owner](../../docs/timers/evidence/shared-tooling-history.md#shared-tooling-028-make-admission),
  [shared #30](https://github.com/dragginzgame/shared-tooling/issues/30)).
- Separate direct-delivery assignment from export for GNU Make 3.81 parsing.
  Keep the consumer policy authoritative under environment and command-line
  overrides. Shared #30 still tracks bypasses when command-line `MAKEFLAGS`
  hides invocation modes; this adoption does not claim that gap resolved.
- Retain incoming Testkit 0.27.2, Metrics 0.3.4 and Host 0.9.7 root-lock
  selections. Testkit and Metrics library sources are unchanged from the released
  graph; Host changes only response decoding outside our harness callers. No new
  Host adapter or timer API is needed. Qualification remains separate from 0.16.3
  ([qualification owner](../../docs/timers/evidence/release-history.md#host-094-graph-preparation)).

## [0.16.3] - 2026-10-09

### Development

- Adopt committed Shared Tooling 0.2.6 formatting and standard-release Make
  includes. Keep direct delivery authoritative even when environment or Make
  variables request PR delivery, and bind these entrypoints to the reviewed root
  snapshot; retain exact-version and phase adapters
  ([shared #91](https://github.com/dragginzgame/shared-tooling/issues/91)).
- Remove the duplicate formatter pin file. Make, CI, developer setup and the
  staged-index hook fixture use `ci/tool-versions.env`; move the shared formatter
  checker and its fixture into the root snapshot. Update disposable Make callers
  and release argument/failure coverage together
  ([shared #92](https://github.com/dragginzgame/shared-tooling/issues/92)).
- This batch is repository-only maintenance; timer APIs and production code are
  unchanged. Consumer fixture execution and native qualification remain pending
  ([adoption owner](../../docs/timers/evidence/shared-tooling-history.md#shared-tooling-026-make-adoption)).
- Run the maintained PocketIC recovery and policy-cohort targets in Linux PR/main
  CI after probe lint, using the prepared internal toolchain and Testkit-admitted
  server. This adds actual product startup evidence to the existing setup and
  fixture checks; it does not claim acceptance before the new job passes
  ([#34](https://github.com/dragginzgame/ic-timers/issues/34)).

## [0.16.2] - 2026-10-09

### Development

- Retain incoming Testkit 0.27.1 and Host 0.9.3 lock selections for the existing
  owner CLI adapter. Testkit library/CLI source is unchanged from 0.27.0, and
  Host library source is unchanged from 0.9.2; current graph qualification
  remains separate from the released 0.16.1 evidence.
- Refresh all three snapshots to committed Shared Tooling 0.2.5. Preserve literal
  trailing newlines in hook selections and checkout paths, and record required
  companions for selected test helpers. Keep the existing snapshot selections
  and consumer tool pins
  ([shared #89](https://github.com/dragginzgame/shared-tooling/issues/89),
  [shared #73](https://github.com/dragginzgame/shared-tooling/issues/73)).
- Complete frozen 0.14.23 early/late compact-artifact qualification on Linux and
  both macOS hosts: all six intended failures archive/upload successfully and
  all six hosted download verifiers pass. This evidence retains its original
  source and does not qualify later timer or tooling changes
  ([#30](https://github.com/dragginzgame/ic-timers/issues/30)).

## [0.16.1] - 2026-10-09

### Development

- Use the incoming root-catalog Testkit 0.27.0 selection for the private PocketIC
  harness and server CLI. Startup errors retain the original cause, bounded
  output and separate command/server cleanup failures. Timer APIs are unchanged;
  native acceptance remains pending
  ([qualification owner](../../docs/timers/evidence/release-history.md#testkit-027-preparation)).

## [0.16.0] - 2026-10-09

### Breaking

- Adopt registry `ic-metrics 0.3` through the root dependency catalog. The public
  `MeasurementSummary` now has the 0.3 package identity; consumers exchanging
  summaries with a direct Metrics dependency must align it to 0.3 or use
  `ic_timers::MeasurementSummary`. Arithmetic source is unchanged from 0.2.20.
  No compatibility alias is retained
  ([adoption evidence](../../docs/timers/design/callback-delivery-ownership.md#ic-metrics-03-adoption)).

## [0.15.0] - 2026-10-09

### Breaking

- Delegate PocketIC setup, artifact pins and offline admission to the root-lock
  selected Testkit CLI. Remove the local server downloader, client alignment and
  binary checkers, their tests, and the `POCKET_IC_BIN` Make override. Explicit
  `make install-testkit-server` prepares the owner CLI/server; `pocketic-check`
  checks it offline. Release preparation and CI select this route while retaining
  watchdog/recovery and policy-cohort gates
  ([shared #76](https://github.com/dragginzgame/shared-tooling/issues/76)).
- Adopt committed Shared Tooling 0.2.1 through all three snapshots and remove
  PocketIC from the shared IC bundle. Run explicit `make install-ic-tools` to
  replace the old six-tool selection; old bundles and evidence remain retained.
  Timer APIs are unchanged.

### Development

- Remove obsolete split-workspace fixture cleanup and single-manifest loops.
  Keep full root-graph resolution, failed-Cargo propagation and lock preservation.
- Retain incoming Testkit 0.26.0, Host 0.9.0 and Metrics 0.2.20 selections through
  the one root catalog/lock. The server adapter follows the same Testkit selection.
- Process every IC pin row during setup/check, including a final row without
  a newline ([shared #87](https://github.com/dragginzgame/shared-tooling/issues/87)).
- Adopt the shared single-document dependency-exception fix and checkout-local
  hook executable lookup ([shared #86](https://github.com/dragginzgame/shared-tooling/issues/86),
  [shared #85](https://github.com/dragginzgame/shared-tooling/issues/85)).
- Retain failed Testkit provisioning attempts and selected CLI build evidence
  without archiving admitted server bundles.
- Report compressed failure-archive bytes and archive creation time in collector
  logs, so hosted compact-evidence qualification can measure the actual output.
  Failed archives do not report completed measurements
  ([#30](https://github.com/dragginzgame/ic-timers/issues/30)).

## [0.14.23] - 2026-10-09

### Development

- Run fleet tooling inventories centrally in Shared Tooling, removing the unused
  reporter and its regression suite from this consumer. Keep local workspace LOC
  and pinned tool setup/check commands
  ([#33](https://github.com/dragginzgame/ic-timers/issues/33),
  [shared #83](https://github.com/dragginzgame/shared-tooling/issues/83)).
- Adopt the committed Shared Tooling PocketIC path fix for inherited `CDPATH`,
  unusual directory names and directories that disappear before Cargo starts
  ([shared #82](https://github.com/dragginzgame/shared-tooling/issues/82)).

## [0.14.22] - 2026-10-09

### Development

- Keep failure archives compact by rechecking active host/IC tool bundles with
  the consumer's pins. Verified bundles retain check logs, pins and IC receipts;
  failed, changed and unselected bundles retain their full payloads. Preserve
  original job identity, failure status, fixture bytes and validation logs
  ([#30](https://github.com/dragginzgame/ic-timers/issues/30),
  [shared #66](https://github.com/dragginzgame/shared-tooling/issues/66)).
- Reuse verified IC tool bundles across comment-only and reordered pin catalogs,
  preserving original installation receipts. Keep compact download verification
  aligned with the shared pin admission and retain native CI for every pushed
  commit while cancelling superseded PR runs
  ([shared #79](https://github.com/dragginzgame/shared-tooling/issues/79),
  [shared #80](https://github.com/dragginzgame/shared-tooling/issues/80)).
- Pass explicit absolute manifest/pin paths to PocketIC alignment so inherited
  `CDPATH` cannot redirect its directory observation
  ([shared #82](https://github.com/dragginzgame/shared-tooling/issues/82)).
- Update the release-gate fixture to require those absolute paths as separate
  arguments, using a physical workspace with spaces and a symlink entry point.
  Report unexpected arguments rather than failing silently.
- Retain incoming lock selections for ic-host 0.8.5, ic-metrics 0.2.16, TOML 1.1.8
  and its parser 1.1.5 through the existing root dependency catalog and Testkit.

## [0.14.21] - 2026-10-08

### Development

- Use reviewed Shared Tooling 0.1.29 to report every staged, unstaged and
  untracked path that prevents release, distinguishing failed Git observations
  from dirty source. Replace the private checker while preserving metadata
  allowances; initial preflight failures explain that validation and version
  preparation have not started for that attempt
  ([#32](https://github.com/dragginzgame/ic-timers/issues/32),
  [shared #74](https://github.com/dragginzgame/shared-tooling/issues/74)).
- Retain the incoming ic-metrics 0.2.15 lock selection through the root catalog.

## [0.14.20] - 2026-10-08

### Development

- Refresh Shared Tooling to reviewed 0.1.28: reject malformed active tool links
  before execution or downloads and keep consumer release-runner fixtures
  simulation-only. Include the maintenance catalog without activating scheduled
  work; preserve the consumer-owned PocketIC pins and complete release gate
  ([#31](https://github.com/dragginzgame/ic-timers/issues/31),
  [shared #75](https://github.com/dragginzgame/shared-tooling/issues/75),
  [shared #70](https://github.com/dragginzgame/shared-tooling/issues/70)).
- Retain the incoming Testkit 0.25.2 lock selection through the root catalog;
  the maintained PocketIC adapter and timer public APIs are unchanged.

## [0.14.19] - 2026-10-08

### Development

- Refresh the reviewed Shared Tooling 0.1.27 follow-up for literal consumer/pin
  paths and explicit installer-fixture dependencies. Keep full evidence retention
  while native compact-policy acceptance remains pending
  ([shared #66](https://github.com/dragginzgame/shared-tooling/issues/66),
  [shared #67](https://github.com/dragginzgame/shared-tooling/issues/67)).
- Align the retained PocketIC 16.1.0 client with independently reviewed server
  archives/binary digests for Linux and both macOS architectures. Keep the strict
  gate, automatic provisioning and override refusal; transfer the existing IC pin
  matrix to consumer ownership so snapshot refreshes preserve the selected pair.
  Prior 16.0.0 runtime evidence remains scoped to its original pair.
- Record completed native late-failure transport and Intel early-failure evidence
  at frozen 0.14.17, retaining the separate source and acceptance boundaries
  ([#30](https://github.com/dragginzgame/ic-timers/issues/30)).

## [0.14.18] - 2026-10-08

### Development

- Preserve evidence collection and qualification paths under inherited `CDPATH`
  and newline-ending checkout or temporary directories. Keep workspace identity,
  original failure status and archive retention checks
  ([#30](https://github.com/dragginzgame/ic-timers/issues/30),
  [shared #67](https://github.com/dragginzgame/shared-tooling/issues/67)).
- Refresh Shared Tooling to reviewed 0.1.27 for shell path bootstrap, complete
  host/IC evidence fixtures and dotted snapshot LOC reporting. Exercise the real
  installer and logger in the evidence path regression; full archive retention
  remains selected while compact-policy qualification is pending
  ([shared #66](https://github.com/dragginzgame/shared-tooling/issues/66),
  [shared #67](https://github.com/dragginzgame/shared-tooling/issues/67),
  [shared #69](https://github.com/dragginzgame/shared-tooling/issues/69)).
- Select Testkit 0.24 through the existing root dependency catalog; preserve the
  incoming Host 0.7.1 and Metrics 0.2.13 lock selections. The maintained PocketIC
  harness and timer public APIs are unchanged.

## [0.14.17] - 2026-10-08

### Development

- Select Testkit 0.23 and IC Host 0.6 through the root catalog, gaining startup
  cleanup diagnostics and wrapper-process cleanup without changing timer APIs
  ([Testkit #30](https://github.com/dragginzgame/ic-testkit/issues/30),
  [Host #5](https://github.com/dragginzgame/ic-host-tooling/issues/5)).
- Refresh Shared Tooling to reviewed 0.1.26, including safe local tracking updates
  after release and expanded tooling reports
  ([shared #62](https://github.com/dragginzgame/shared-tooling/issues/62),
  [shared #61](https://github.com/dragginzgame/shared-tooling/issues/61)).
- Delegate failure archiving to the shared helper; preserve partial archives and
  reject overwrites, while retaining source/status admission and all selected
  installer evidence ([#30](https://github.com/dragginzgame/ic-timers/issues/30),
  [shared #59](https://github.com/dragginzgame/shared-tooling/issues/59)).

## [0.14.16] - 2026-10-08

### Development

- Select Testkit 0.22 and IC Host 0.5.2 through the root dependency catalog.
  The existing PocketIC harness now uses the shared child-process owner;
  timer APIs, runtime behavior and the audited PocketIC version are unchanged
  ([upstream #25](https://github.com/dragginzgame/ic-testkit/issues/25)).
- Record the consumer audit of Shared Tooling and IC Host, including measured
  failure-artifact overhead and archive-adapter requirements. Implementations
  and reviewed snapshots remain unchanged
  ([upstream #66](https://github.com/dragginzgame/shared-tooling/issues/66)).

## [0.14.15] - 2026-10-08

### Development

- Preserve the incoming compatible Metrics 0.2.11 root lock selection. Its
  changes affect upstream release tooling and evidence; the metrics library
  implementation and timer APIs are unchanged.

## [0.14.14] - 2026-10-08

### Development

- Adopt Shared Tooling 0.1.23: recheck payload and exact tag identity after final
  release hooks, and verify published refs on completed release resume. Keep
  direct delivery explicit and preserve maintainer-owned commands
  ([#29](https://github.com/dragginzgame/ic-timers/issues/29),
  [upstream #58](https://github.com/dragginzgame/shared-tooling/issues/58)).
- Refresh the test-only root lock selection to compatible Testkit 0.21.3 and
  TOML dependency patches. Keep the existing root dependency catalog and
  PocketIC harness; timer behavior and public APIs are unchanged.
- Preserve the incoming Metrics 0.2.10 lock selection; its changes affect
  upstream tooling and documentation, with unchanged library implementation.

## [0.14.13] - 2026-10-07

### Development

- Add manual early/late failure qualification to the existing Linux and native
  macOS CI jobs, using the maintained installer/logger and normal artifact
  collector. Download and check each host's source identity, controlled failure,
  retained bytes and file modes. Retain original failed-job status; keep normal
  PR/main gates and release commands unchanged
  ([#23](https://github.com/dragginzgame/ic-timers/issues/23)).
- Select compatible IC Host 0.4.6 through Testkit in the root lockfile, including
  the [macOS compilation repair](https://github.com/dragginzgame/ic-host-tooling/issues/18);
  retain the [native filename qualification](https://github.com/dragginzgame/ic-host-tooling/issues/19)
  and keep Host outside the timer library graph. Preserve the incoming Metrics 0.2.9
  selection, which changes upstream tooling/evidence rather than library behavior.
- Select the compatible Testkit 0.21.2 documentation and host-qualification patch
  through the existing root dependency catalog.

## [0.14.12] - 2026-10-07

### Development

- Adopt committed Shared Tooling 0.1.19, including Rust-tool path admission
  repairs and complete failed-batch logs
  ([#28](https://github.com/dragginzgame/ic-timers/issues/28)).
- Replace the private changelog selector with the shared owner; retain local
  check-only admission, mode preservation and atomic replacement. Cover failed
  readers and producers without rewriting original notes
  ([#24](https://github.com/dragginzgame/ic-timers/issues/24)).
- Read PocketIC archive pins from the shared matrix and reuse canonical checksum
  and binary verification. Check locked client/server alignment before admission
  or provisioning; preserve automatic single-artifact installation and strict
  explicit overrides ([#25](https://github.com/dragginzgame/ic-timers/issues/25)).
- Select the compatible Testkit 0.21.1 host-qualification patch in the sole
  root lockfile; keep all member dependency declarations inherited from the
  root catalog.

## [0.14.11] - 2026-10-07

### Development

- Add byte-exact changelog finalization coverage for historical notes with no
  terminal newline or retained trailing whitespace, including imported undated
  history and releases with no pending notes. Prepare the consumer contract for
  shared-selector adoption ([#24](https://github.com/dragginzgame/ic-timers/issues/24)).

## [0.14.10] - 2026-10-07

### Development

- Adopt Shared Tooling 0.1.18: reject Make options and assignments passed as
  validation targets, preserve optional complete logs and timing summaries,
  and fix LOC fixture isolation and build-output exclusion. Keep the single
  root dependency catalog and complete audited PocketIC release gate
  ([Shared Tooling #30](https://github.com/dragginzgame/shared-tooling/issues/30),
  [#37](https://github.com/dragginzgame/shared-tooling/issues/37),
  [#31](https://github.com/dragginzgame/shared-tooling/issues/31),
  [#47](https://github.com/dragginzgame/shared-tooling/issues/47),
  [#50](https://github.com/dragginzgame/shared-tooling/issues/50),
  [#53](https://github.com/dragginzgame/shared-tooling/issues/53)).
- Refresh shared audit and setup guidance, including the canister application
  addendum. Timer behavior and dependency selections are unchanged.
- Keep `make build` scoped to the library after workspace consolidation;
  probe cohorts retain separate builds for their mutually exclusive features.

## [0.14.9] - 2026-10-07

### Development

- Consolidate library and unpublished probe dependencies in the root Cargo
  catalog and lockfile; every member inherits dependencies with `workspace = true`.
  Retire the independent testing manifest/lock and update release, formatting
  and fixture owners for the single graph. Keep library-only default builds and
  preserve probe optimization settings in the `timer-probe` profile.
  Reject stale library or probe identities in the shared lockfile.
- Resolve the current Testkit 0.20 selection and ic-metrics 0.2.7 together.
  Preserve the audited PocketIC 16.0.0 gate and the complete probe qualification.

## [0.14.8] - 2026-10-07

### Development

- Adopt Shared Tooling 0.1.15 and its common Make setup/check/LOC commands.
  Prepare and verify pinned jq, yq, ripgrep with PCRE2 and cloc together;
  CI uses that bundle instead of separately installing system ripgrep.
  Consolidate host/IC setup under the root snapshot, remove copied Make
  recipes and preserve the audited PocketIC release gate. This is
  repository-only work; timer behavior and dependency selections are unchanged.
- Remove the retired nested installer directory from shell syntax checks;
  repository fixtures keep it absent so a stale wildcard cannot pass unnoticed.
- Record and substitute host-tool admission in release-gate fixtures, checking
  that its failure stops later checks and retaining per-scenario Make output.
- Retain failed committed-release, staging/index, lockfile and repository-check
  fixtures for CI artifact collection instead of deleting their diagnostic
  inputs. Preserve original failure status and clean up successful runs
  ([#23](https://github.com/dragginzgame/ic-timers/issues/23)).

## [0.14.7] - 2026-10-07

### Development

- Adopt Shared Tooling 0.1.14: reject Make modes that skip execution or mask
  failures before release, validation and pre-commit formatting. Protect the
  local exact-version release path and cover isolated fixture exports
  ([#20](https://github.com/dragginzgame/ic-timers/issues/20)).
  Check that each failed exact-release phase stops the sequence, and retain
  failed hook/version fixtures with scenario logs for diagnosis.
  This is repository-only work; timer behavior and dependency selections are unchanged.
- Upload failed CI fixture, validation and installer evidence after every job's
  checks, preserving file modes and source/host identity
  ([#23](https://github.com/dragginzgame/ic-timers/issues/23)).

## [0.14.6] - 2026-10-07

### Development

- Adopt Shared Tooling 0.1.13 and place the three testing packages under their
  independent workspace's `crates/` directory, preserving package identities,
  both lockfiles and the complete release gate
  ([#19](https://github.com/dragginzgame/ic-timers/issues/19)).
  This is repository-only work; timer behavior and dependency selections are unchanged
  by the layout adoption.

## [0.14.5] - 2026-10-06

### Development

- Replace the copied standard-release smoke fixture with the reviewed shared
  checker, retaining local metadata, lockfile and recovery coverage
  ([#17](https://github.com/dragginzgame/ic-timers/issues/17)).
- Refresh Shared Tooling to 0.1.12: bind standard release pushes to the recorded
  destination URL and verify snapshots without executing inspected checksum helpers. Add
  isolated corruption checks for all three consumer snapshots
  ([#18](https://github.com/dragginzgame/ic-timers/issues/18)).
  This batch is repository-only; timer behavior and dependency selections are unchanged.

## [0.14.4] - 2026-10-06

### Development

- Include external `echo` in the release-gate fixture's restricted tool PATH,
  so Make's direct execution of logging recipes reaches the intended assertions.
  This repair is repository-only.
- Refresh the reviewed Shared Tooling snapshots to 0.1.11. Passing `error::`
  test names remain ordinary validation output, while actual diagnostics retain
  highlighting and failure logs. Adopt the paired maintenance policy and stricter
  host-tool version admission; validation and release execution remain user-owned.
  This is repository-only work.
- Delegate annotated release-tag validation to the reviewed shared checker,
  preserving HEAD checks for publication and saved-commit checks for release
  recovery ([#16](https://github.com/dragginzgame/ic-timers/issues/16)).
  This is repository-only work; timer behavior and dependency selections are
  unchanged.

## [0.14.3] - 2026-10-06

### Development

- Finalize release notes using the bump's known previous version. Preserve older
  undated sections as history, retaining strict rejection of competing pending notes
  and already dated targets. Compare version components exactly, retain complete
  candidate admission, and preserve file ownership and rollback. This is
  repository-only work.
- Check the reviewed cargo-sort version and prepared rustfmt before `fmt`,
  `fmt-check` and hook qualification. Reject failed version commands even when
  they print the expected version, and keep preparation explicit and offline
  checks free of tool installation. This is repository-only work; timer behavior
  and dependency selections are unchanged.

## [0.14.2] - 2026-10-06

### Fixed

- Prepare ripgrep before regression fixtures in Linux, MSRV, native macOS and
  tag CI jobs. This fixes 0.14.1's missing-tool failures while retaining the
  shared structured checker and complete release gate
  ([#13](https://github.com/dragginzgame/ic-timers/issues/13)).
  This repair is repository-only.

### Development

- Use the shared exact local-package lock rewrite for both release lockfiles,
  preserving external identities and checking the complete candidate before
  replacement ([#14](https://github.com/dragginzgame/ic-timers/issues/14)).
- Enforce Cargo inheritance against each independent workspace root and use
  the shared structured workspace-version reader. Version mutation and release
  recovery remain locally owned ([#15](https://github.com/dragginzgame/ic-timers/issues/15)).
- Add explicit `install-ic-tools` / offline `ic-tools-check` and combined
  `install-tools` / `tools-check` setup for the pinned six-tool IC bundle.
  Development setup and CI prepare it before validation; native macOS evidence
  still admits PocketIC by its independently audited binary hash
  ([#12](https://github.com/dragginzgame/ic-timers/issues/12)).
  This batch is repository-only; runtime and dependency selections are unchanged.

## [0.14.1] - 2026-10-06

### Development

- Replace line-based Actions validation with the reviewed shared YAML/TOML
  checker. Quoted and folded SHA references are accepted; moving references,
  unpinned Docker actions and malformed declarations are rejected. Preserve
  both workspace lockfiles and the existing qualified IC dependency pins
  ([#13](https://github.com/dragginzgame/ic-timers/issues/13)).
- Add explicit, checksum-verified jq/yq setup with `make install-host-tools`
  and offline verification with `make host-tools-check`. Explicit `update-dev`
  and CI prepare the parsers
  before validation; ordinary checks never download them. This is the parser
  prerequisite of [#12](https://github.com/dragginzgame/ic-timers/issues/12),
  without changing PocketIC provisioning or installing the broader IC toolset.
  This batch is repository-only.

## [0.14.0] - 2026-10-06

### Breaking

- Use published `ic-metrics 0.2` for measurement arithmetic. The publicly exposed
  `MeasurementSummary` now belongs to the 0.2 package identity; consumers that
  exchange it with a direct ic-metrics dependency must update that dependency to
  0.2 or use IC Timers' re-export. Summary values and saturation are unchanged
  ([ic-metrics #10](https://github.com/dragginzgame/ic-metrics/issues/10)).

### Changed

- Read instructions through the existing `ic0::performance_counter(1)` platform
  adapter and remove ic-metrics' retired `ic` feature. Both independent workspace
  locks select registry 0.2.0, preserving scheduler/work attribution,
  registration identity and native test fakes
  ([ic-metrics #10](https://github.com/dragginzgame/ic-metrics/issues/10)).

### Development

- Adopt the committed shared audit methods with an IC Timers overlay, preserving
  runtime safety obligations and historical reports while removing automatic
  repair and broad-validation instructions. Verify the audit snapshot alongside
  the existing tooling baseline ([#11](https://github.com/dragginzgame/ic-timers/issues/11)).
  This audit-tooling slice is repository-only.

## [0.13.5] - 2026-10-06

### Fixed

- Stop committed release metadata checks at the first failure on Bash 3.2,
  preserving manifest ordering and locked-resolution rejection on macOS.
- Exercise release-index checks in a controlled shallow checkout with its own
  baseline tag, removing their dependence on hosted checkout history. This
  addresses the hosted qualification failures in the release-tooling repair for
  [#10](https://github.com/dragginzgame/ic-timers/issues/10). These release-check
  repairs leave timer behavior unchanged.

### Changed

- Align both workspace locks with the published `ic-metrics 0.1.6` requirement,
  preserving all other dependency selections and allowing locked release
  preflight to resolve the testing workspace.

## [0.13.4] - 2026-10-06

### Fixed

- Recover an interrupted committed release through the normal release commands
  after newer fixes are committed. Check the saved release's metadata and tag at
  its exact commit, then run fresh validation for the requested next increment.
  This addresses [#10](https://github.com/dragginzgame/ic-timers/issues/10) using
  the reviewed Shared Tooling recovery fix. This is repository-only release
  tooling; package publication and cleanup remain separate.
- Reject staged implementation edits hidden by a restored working file, require
  the release index to match prepared metadata, and retain complete validation
  failure logs across retries through the shared validation runner.
- Prepare both locked dependency caches during release preflight through the
  existing fetch target, allowing cold-cache releases to reach validation without
  a separate manual fetch. Fetch and Git admission failures stop before mutation.

### Testing

- Report exact loaded Wasm byte sizes alongside policy-cohort instruction costs
  in the existing qualification gate, enabling baseline and release comparisons
  without separate builds or measurement commands.

## [0.13.3] - 2026-10-06

### Fixed

- Release removed and rejected callback captures after registry access ends,
  with removed timers' provider cleanup completed before capture destruction.
  This permits normal cancellation and unregistration destructors to inspect
  the registry without receiving an internal borrow error.

### Changed

- Leave bound provider handles in place for rejected and coalesced control
  requests, avoiding temporary identity copies and handle reinstallation.
  Scheduling, cancellation arbitration and Watchdog prearming are unchanged.
- Add native and PocketIC capture-release coverage and reconcile current
  qualification documentation with the successful 0.13.2 hosted gates.

## [0.13.2] - 2026-10-06

### Fixed

- Correct release-gate fixture comparisons when the workspace is reached through
  a directory symlink, including macOS's `/var` temporary paths. Exercise the
  alias on every host while retaining exact default and override PocketIC
  selection checks. Production provisioning and the complete release gate are
  unchanged; this is repository-only test tooling.

## [0.13.1] - 2026-10-05

### Changed

- Use published `ic-metrics 0.1.5` for the Wasm call-context instruction reader
  and shared summary, retaining callback attribution, registration identity and
  consumer-owned native/test behavior
  ([#9](https://github.com/dragginzgame/ic-timers/issues/9)).

### Development

- Align the handoff, callback contract and Shared Tooling adoption notes with
  the tagged 0.13.0 release and published registry `ic-metrics 0.1.3`. Separate
  release identity from recorded validation and remove obsolete pending-release
  and sibling-dependency instructions. This is repository-only documentation;
  runtime behavior and the public API are unchanged.

## [0.13.0] - 2026-10-05

### Changed

- Re-export `ic_metrics::MeasurementSummary` as the canonical summary while retaining
  the existing snapshot accessors, callback attribution and registration identity.
  Select published registry `ic-metrics 0.1.3` in both lockfiles, removing the
  sibling-checkout requirement ([#9](https://github.com/dragginzgame/ic-timers/issues/9)).
- Standardize the three SemVer release commands on the reviewed Shared Tooling
  runner, including automatic recovery when the same target is rerun and an atomic push of the selected branch
  and tag. Release preparation uses UTC dates and preserves both locked dependency selections
  and build artifacts.

### Fixed

- Keep the ordinary delivery guard captured until callback completion, preserving
  discard retirement and normal completion handling. Drop queued and suspended
  native fixture work before thread-local teardown, with the runtime still
  available and the task-map borrow released, preventing teardown aborts.

### Breaking

- Retire confirmed ordinary deliveries dropped without normal completion instead
  of retaining false Scheduled or Running authority. Add `InactiveReason::Abandoned`
  and project it as Failed; record Unacknowledged without fabricating a completion
  or measurement. Retained claims can explicitly rearm; transient declarations
  and pending unregistrations release their entries. Live futures still complete
  normally before pending control is applied; no retry or interruption is added.
  Consumers must handle the new `InactiveReason` variant and treat ordinary
  Unacknowledged observations as abandoned deliveries, without inferring
  Watchdog recovery or application rollback.

### Development

- Show the repository name first in VS Code window titles, followed by the active filename.
- Restart failed preflight/validation through the normal release target, and
  automatically reconcile prepared attempts at their saved version before any
  new increment. Remove the local retry wrapper in favour of the shared owner.
- Adopt the shared pre-commit hook to format and refresh only fully staged
  selected files, preserving unrelated edits and rejecting partial staging.
  Pin `cargo-sort` 2.1.4 in developer/CI setup and include manifest sorting for
  both workspaces in formatting and prepared-release checks.
- Use exact `ic-testkit` 0.17.3 for the host-side real-canister suites, with
  bounded startup and caller-owned servers for fresh IC instances. Update the
  strict PocketIC artifact gate to 16.0.0 on Linux and both macOS architectures;
  preserve override rejection and all timer assertions. Existing PocketIC 15
  receipts remain historical; the new harness requires renewed qualification.
- Repair isolated version-preparation checks after shared-runner adoption: include
  the shared version calculator, verify all three increments and current Makefile
  delegation, and keep failure injection active through missing-changelog rollback.
  Retain local exact-version rollback and staging coverage. Run
  the shared runner's phase/failure/resume fixtures in the existing release gate.
- Add native and PocketIC fixtures covering discarded ordinary deliveries,
  traps before and after an await, pending commands, stale authority and capture
  release for both ordinary policies and lifetimes. Keep native assertions and
  identity ownership compatible with warning-denied Clippy. Qualification remains pending;
  see the [callback contract](../../docs/timers/design/0.5-policy-specific-callback-authority.md#ordinary-delivery-abandonment).
- Reconcile current README, safety, callback-contract, release and handoff
  documentation with the completed 0.12 minor bump. Remove pending-bump
  instructions and describe the policy-specific return API as current, keeping
  automated preparation evidence separate from maintainer validation. This is
  repository-only documentation work; no crate behavior or package identity
  changes.
- Clarify in the public result rustdoc and README that success, no work and
  retryable failure preserve the supplied scheduling decision rather than
  choosing recurrence or retry automatically. Explain invariant-failure Stop
  normalization and checked relative-delay validation after authoritative
  command arbitration. Runtime behavior is unchanged.

## [0.12.0] - 2026-10-05

### Changed

- Hard-cut ordinary callback returns to `OnceRunResult` / `OnceDecision` and
  `AfterCompletionRunResult` / `AfterCompletionDecision`. Update both registration
  and lifecycle reconciliation entry points; Once keeps explicit continuation,
  retry and absolute deadlines but cannot request configured recurrence.
- Remove the shared public `TimerRunResult` and `TimerDirective` surface without
  aliases or conversions between policy result types. Retain one private erased
  result and scheduling owner for registry arbitration, checked overflow handling,
  invariant-failure stopping and inert snapshot projection. Keep Watchdog's
  callback contract and snapshot meanings unchanged.
- Update current examples, native fixtures and both probe canisters to typed
  results. Project either public decision into `TimerDirectiveSnapshot` through
  the existing checked conversion owner; retain no reverse conversion.

### Development

- Use unqualified `pub` for the internal ordinary callback, directive and erased
  result declarations inside private root modules, addressing the reported
  `redundant_pub_crate` lint while retaining their absence from the public API.
- Add positive and compile-fail API doctests covering all four ordinary entry
  points, crossed decisions/results and unavailable Once recurrence. Run doctests
  in the existing test and MSRV targets, and extend recording-Cargo gate fixtures
  to reject failures at either recipe command. Extend native runtime coverage
  across legal scheduling choices, overflow and authoritative reconciliation;
  keep private erased-policy corruption coverage. Execution remains pending
  maintainer validation, including the full PocketIC and policy-cohort gate.
- Cover callback-borrow rejection for both typed ordinary policies and both
  declaration lifetimes, including retained failure observations, transient
  removal and absent work measurements. Add live native recurrence cases for
  success, no work, retryable failure and invariant failure; correct the README
  to distinguish a normal return requesting recurrence from success-only
  recurrence. These fixture additions remain unexecuted.

## [0.11.10] - 2026-10-05

### Development

- Check unstaged implementation and untracked paths before combined release
  commands run validation or bump the version. Reuse the release-commit guard
  with NUL-delimited Git records, reject failed queries even after partial
  metadata-only output, and report Bash-escaped paths. Allow the five metadata
  outputs owned by the bump and release staging; preserve dirty-worktree support
  for standalone version preparation and the complete user-operated release gate.
  Extend worktree-admission and orchestration fixtures without staging files
  automatically.
- Reject nonregular PocketIC candidates and invalid automatic-install cache
  destinations before downloading. Preserve directories, FIFOs and rejected
  symlinks; continue accepting verified executable file symlinks without replacing
  them. Extend cache-type and link-preservation fixtures while retaining exact
  host-specific artifact pins.
- Require exactly one version argument after the bump helper's optional leading
  `--check`. Reject extra arguments and misplaced or repeated check flags before
  reading or changing release metadata; extend metadata-preservation fixtures.

## [0.11.9] - 2026-10-05

### Development

- Validate the tagged checkout's root and testing lockfiles in hosted tag CI
  after confirming the exact annotated tag and main reachability. Prepare the
  selected dependency caches before offline metadata checks; retain fixture
  validation without repeating the full Rust and MSRV builds owned by PR/main.
- Classify release impact from NUL-delimited Git paths so filenames containing
  tabs, line breaks, quotes or non-ASCII bytes cannot hide crate-source changes.
  Remove display-path joining and sorting; require both Git queries to succeed
  before classifying their records. Extend the existing fixture across untracked
  and staged paths, partial query failures and temporary-record cleanup.
- Verify the pinned PocketIC archive digest before decompression, then retain
  hash-before-execution and exact version checks for downloaded, cached and
  overridden binaries. Pin separate PocketIC 15.0.0 artifacts for Linux x86_64,
  macOS Intel and Apple Silicon; use core Perl SHA-256 support on both hosts.
  Extend failure-order, host-selection and cache-preservation fixtures.
- Declare macOS 15 Intel and Apple Silicon host targets and add native PR/main
  jobs that run the complete release gate under Apple's Bash 3.2 with both
  pinned Rust toolchains. Replace empty-array fixture argument expansion with
  positional arguments for Bash 3.2. Native qualification remains pending these
  jobs; tag pushes continue to avoid duplicate full validation.

## [0.11.8] - 2026-10-05

### Development

- Replace checksum manifests and separate mode tables in preservation fixtures
  with copies of the actual files. Compare bytes directly and preserve permission
  assertions for preflight rejection, rollback, interrupted preparation and tag
  rejection; retain independent Git index preservation checks.
- Compare preparation phases and staged paths as ordered records, removing joined
  arrays. Capture Git output directly so command failure cannot become an empty
  staging or tag result.
- Replace fixture-only GNU `sed -i` mutations with Perl, removing that host-specific
  dependency. PocketIC's audited binary hash verification remains unchanged;
  native macOS qualification is still pending.
- Stop clean-worktree and release-commit guards when Git queries fail. Distinguish
  a staged diff from a failed staged-diff query, and reject failed release-subject
  reads even when they emit a matching subject. Extend rejection fixtures across
  staged preparation and clean release retries without changing retry identity.
- Compare PocketIC fixture verification events as exact ordered records for
  accepted binaries and rejected overrides/downloads. Capture debris searches
  before asserting cleanup so a failed search cannot appear empty.
- Capture exact release-tag listings before checking absence during version
  preparation, release commits and interrupted tag retries. Reject failed queries
  even with plausible output; retain the prepared commit when its post-commit
  lookup fails and resume tagging that same commit on retry.
- Reject failed workflow discovery and provider-source searches even when they
  produce apparently valid records. Preserve nested workflow paths and temporary
  file cleanup, and extend the repository fixture across discovery, search and
  ordering failures with empty or matching output.
- Fetch the selected root and testing lockfiles' dependencies before the
  user-operated release gate, including target-specific sources needed by offline
  metadata checks. Add `make fetch` for cache preparation without building or
  changing versions. Preserve Cargo metadata failures before JSON parsing so a
  missing archive does not produce a second, misleading parse error.

## [0.11.7] - 2026-10-05

### Development

- Compare gate-fixture records directly instead of converting them through Bash
  4's `mapfile` and joined arrays. Preserve exact check order, duplicate PocketIC
  prerequisites, empty overrides and failure-stop verification without that
  newer-shell dependency. Check the full repository-gate prefix on failure rather
  than only its last recorded check.
- Refresh the reviewed Shared Tooling documentation baseline to `ca319ba`, using
  committed source bytes and `DRAGGINZGAME.md` as the shared entry point. Retain
  local contributor instructions and explicit maintainer ownership of validation
  and release execution.
- Adopt exact-symbol cleanup reporting and record required macOS host workflows
  and qualification gaps without changing the pinned PocketIC release gate.
- Keep successful changelog-finalization fixtures quiet while showing child
  diagnostics on unexpected failure. Preserve empty-note warnings during actual
  release preparation.

## [0.11.6] - 2026-10-05

### Changed

- Decide terminal declaration removal from the entry already selected by the
  ordinary request, Watchdog request or Watchdog scheduler transition. Remove
  the separate registry lookup while preserving lifetime rules, typed failures,
  accounting and provider cleanup ordering.

### Development

- Extend the Watchdog immediate-request coalescing fixture across both declaration
  lifetimes, checking that successful initial, replacement and duplicate requests
  retain transient declarations.
- Record terminal-removal ownership and pending verification in the
  [architecture reference](../../docs/timers/architecture.md#terminal-removal-verification).

## [0.11.5] - 2026-10-05

### Changed

- Let ordinary and Watchdog work delivery consume the matching fired handle and
  accept work through the same registry entry lookup. Remove the separate runtime
  consumption pass while retaining exact claim, role, generation and state checks.
- Combine Watchdog scheduler handle consumption and detachment in one registry
  operation before its transition. Retain empty detachment for removed or
  superseded claims and cleanup before terminal declaration removal.

### Development

- Adopt the reviewed Shared Tooling engineering baseline with an explicit local
  overlay preserving maintainer ownership of tests and release execution.
- Keep one versionless changelog draft until a release is selected. Let the
  bump helper label and date it without requiring a separate versioned note or
  handoff status marker. Remove changelog presentation from deployment gates,
  retaining package, lockfile, tag and PocketIC validation.
- Preserve metadata contents, modes and prior file absence during failed bumps;
  reject symlinked or non-file outputs before mutation.
- Stage only the five release metadata outputs, leaving member manifests and
  unrelated documentation under the maintainer's separate staging ownership.
  Consolidate repeated contributor testing and release-ownership instructions.
- Extend provider-role fixtures with mismatched claims as well as generations,
  and deliver the stale-identity callback through the native mock queue. Check that
  it records stale delivery without consuming the replacement's handle or work.
- Check that suspended after-completion work owns no provider wakeup.
- Record scope and pending verification in the
  [callback delivery evidence](../../docs/timers/design/callback-delivery-ownership.md) and
  [Shared Tooling adoption record](../../docs/timers/development.md).

## [0.11.4] - 2026-10-04

### Changed

- Record Watchdog request coalescing once after the selected transition, removing
  duplicate scheduled, dispatched and running updates. Terminal failures remain
  distinct from successful requests satisfied without another arm.
- Commit ordinary request scheduling mode once for successful arms or exact
  reconciliation, preserving mode changes when an exact deadline coalesces and
  mode retention for coalesced ensures.

### Development

- Extend existing fixtures to check dispatched and running request counts,
  non-coalesced terminal failures and exact ordinary mode observations at an
  unchanged deadline.
- Record scope and pending verification in the
  [0.11.4 release note](../../docs/timers/changelog/0.11.4.md).

### Documentation

- Restore the README's single structured API-line projection after the overview
  rewrite, allowing version-bump and release-truth checks to maintain it.

## [0.11.3] - 2026-10-04

### Changed

- Consolidate ordinary completion's directive and generation failures into one
  terminal finalization branch and one declaration-removal exit. Preserve command
  precedence, validation order, typed failures and completion accounting.
- Give Watchdog initial and replacement requests one scheduling-mode update after
  successful arming. Preserve observations for coalesced, pending and failed
  requests, and calculate cadence deadlines only when required.
- Remove the private callback-context forwarding wrapper. Policy-specific public
  contexts carry their exact work token directly and use the existing shared
  authorization and provider-transition path.

### Development

- Extend the ordinary terminal-failure matrix across both policies and lifetimes
  to cover oversized retry delays and deadline overflow at exhausted generations.
  Check that retained failures record a stopped directive and invariant completion.
- Extend existing Watchdog fixtures to check mode retention for equal deadlines,
  pending commands and exhausted replacements, including cadence coalescing at
  maximum time without unnecessary deadline calculation.
- Record scope and pending verification in the
  [0.11.3 release note](../../docs/timers/changelog/0.11.3.md).

### Documentation

- Add the shared helper navigation menu linking the eight maintained helper
  repositories through centrally hosted icons.
- Remove stale merge-conflict text from the README and align its technical
  overview with the `0.11` API line.

## [0.11.2] - 2026-10-04

### Changed

- Return ordinary and Watchdog work callbacks directly from the registry's
  acceptance transition. Remove the separate callback lookups, acceptance-status
  enum and second dispatch borrow while retaining exact claim, role, generation
  and state authorization.
- Fold the remaining context-only read lookup into context validation, retaining
  the shared running-work predicate and independent completion authorization.
- Decide cancellation lifetime removal once from the final state for both
  ordinary and Watchdog declarations. Remove the ordinary pre-state copy and
  fold its remaining generation-allocation helper into checked arming.

### Development

- Update acceptance fixtures to assert callback availability and reject duplicate
  acceptance across all three policies. Retain delegated-context and completion
  rejection coverage; remove assertions for the deleted lookup paths.
- Extend fresh cancellation coverage across both lifetimes, checking unchanged
  retained inventories and expiration of transient claims for every policy.
- Record scope and pending verification in the
  [0.11.2 release note](../../docs/timers/changelog/0.11.2.md).

## [0.11.1] - 2026-10-04

### Changed

- Consolidate Watchdog cancellation state selection, handle cleanup and immediate
  cancellation accounting in the registry command owner. Remove the separate
  cancellation helper and boolean cleanup selector while preserving running
  commands, generation history and declaration lifetimes.

### Development

- Extend running Watchdog unregistration coverage to check that a later
  cancellation preserves removal and does not count as an immediate stop.
- Record scope and pending verification in the
  [0.11.1 release note](../../docs/timers/changelog/0.11.1.md).

## [0.11.0] - 2026-10-04

### Changed

- Make cancellation stop existing work without allocating callback generations.
  Remove ordinary and Watchdog cancellation exhaustion paths; preserve pending
  commands for running work, actual handle cleanup and non-wrapping allocation
  when scheduling new callbacks.
- Treat cancellation at exhausted generations as `Cancelled` rather than a
  generation control failure. Subsequent generation values no longer include
  cancellation increments. This is a pre-1.0 semantic hard cut.
  Reconciliation to `None` and non-running unregistration share this behavior.
- Replace separate Watchdog scheduler and work-attempt allocation counters with
  one generation shared by each dispatched pair. Preserve role-scoped authority,
  separate handles and checked generation/deadline validation. Work and successor
  now share the observed dispatch generation rather than independent clocks.

### Removed

- Remove `WatchdogAttemptSnapshot` and the `AwaitingWork.attempt` wrapper. The
  awaiting-work snapshot now exposes `attempt_status` directly and represents
  its shared generation once through `successor_generation`.

### Development

- Update cancellation and exhaustion fixtures for state-based invalidation,
  cancellation at maximum generation and stale delivery after rearming.
- Cover shared-generation role boundaries and pending requests that preserve the
  dispatched/running pair until completion. Remove the independent attempt-counter
  exhaustion cases and retain allocation, deadline, cleanup and recovery fixtures.
- Add a runtime fixture rejecting mixed-generation dispatch pairs before provider
  arms, preserving inventory and testing both mismatched-pair orientations.
- Record the contract and pending verification in the
  [0.11.0 release note](../../docs/timers/changelog/0.11.0.md).

### Documentation

- Add plain-English introductions to the safety, architecture, observability,
  and release guides, and reorganize the documentation index by audience while
  preserving the detailed technical contracts and historical records. Add
  graphics for module relationships, safety responsibilities, and Watchdog
  failure behavior, and use the shared IC Timers banner across all reader-facing
  documentation.

## [0.10.23] - 2026-10-04

### Changed

- Move ordinary claim-policy validation into reconciliation and delete its
  single-caller helper. Preserve claim and policy rejection before cancellation
  or schedule resolution.
- Finalize Watchdog scheduler generation and deadline failures through one
  terminal path, preserving generation-first error selection, atomic allocation,
  queued-work cleanup and declaration lifetimes.

### Development

- Extend the Watchdog cancellation fixture to assert that ordinary reconciliation
  rejects inactivity, valid deadlines and oversized delays without changing the
  armed declaration or its inventory.
- Extend the scheduler exhaustion fixture for simultaneous generation and
  deadline overflow, retaining terminal-state, cleanup and unchanged-counter
  assertions.
- Record the cleanup and pending verification in the
  [0.10.23 release note](../../docs/timers/changelog/0.10.23.md).

## [0.10.22] - 2026-10-04

### Changed

- Use the canonical claim lookup for late measurement accounting and provider
  handle consumption, preserving no-op behavior for missing or superseded claims
  and the independent policy, role and handle-generation checks.
- Detach provider-handle pairs through their registry entry during Watchdog
  failure cleanup. Remove runtime pair assembly and the separate pair constructor,
  preserving cleanup order and harmless cleanup of empty or missing entries.
- Pass validated ordinary control directly to terminal-failure finalization,
  removing its repeated policy check and unreachable fallback. Preserve terminal
  reasons, wakeup cleanup and declaration-lifetime decisions.

### Development

- Extend the provider-cleanup fixture for empty and missing entries, and update
  restoration cleanup to use registry-owned pair detachment.
- Record the lookup cleanup and focused verification subjects in the
  [0.10.22 release note](../../docs/timers/changelog/0.10.22.md).

## [0.10.21] - 2026-10-04

### Changed

- Use the canonical claim lookup for provider installation and effect
  confirmation, removing their duplicate identity and claim-generation lookup.
  Preserve stale-callback errors and independent role, state and generation checks.

### Development

- Cover lifecycle reconciliation rejection through the public APIs: mismatched
  identity for Once, transient lifetime and mismatched cadence for AfterCompletion,
  and an expired Watchdog claim after identity reuse. Assert typed errors and
  preservation of inventory, armed handles and the live callback. Verify the
  retained AfterCompletion callback still runs at its original deadline.
- Extend the duplicate-registration fixture to cover reconstruction from an
  empty consumer slot when the identity is already occupied. Preserve the empty
  slot, live declaration, armed wakeup and original callback on rejection.
- Cover provider installation and effect confirmation after claim removal and
  identity reuse, preserving rejected-handle ownership and unchanged inventory.
- Record the cleanup scope and pending verification in the
  [0.10.21 release note](../../docs/timers/changelog/0.10.21.md).

## [0.10.20] - 2026-10-04

### Changed

- Move lifecycle declaration checks into the shared reconciliation operation and
  callback-context checks into the shared claim-transition operation. Remove two
  single-caller validation helpers while preserving rejection before timer
  mutation or provider-handle detachment.
- Store the latest outcome and reported work count as one private terminal event.
  Derive their public observations together while retaining independent success,
  failure and unacknowledged timestamps and failure-streak accounting.

### Development

- Add a focused outcome-history fixture covering empty and zero-work observations,
  interrupted attempts, subsequent completions and preservation of prior timestamps.
- Record the validation and outcome ownership changes and focused verification in
  the [0.10.20 release note](../../docs/timers/changelog/0.10.20.md).

## [0.10.19] - 2026-10-04

### Changed

- Express ordinary pending-command precedence in one match, removing the
  duplicated sticky-unregistration branch while preserving exact reconciliation
  and earliest-demand scheduling.
- Build ordinary successors and Watchdog dispatch tokens from their validated
  claims through the existing token constructor, without repeating ownership
  fields in each transition.
- Remove the forwarding cleanup-effect constructor. Select Watchdog cancellation
  cleanup once and share conditional wakeup cleanup on generation exhaustion.

### Development

- Extend ordinary lifecycle fixtures for unregistration followed by control
  requests and for preservation of schedule metadata at equal deadlines.
- Share repeated follow-up reconciliation setup in the Watchdog precedence
  fixture, retaining all nine cases without a function-size lint exception.
- Record the cleanup and pending verification in the
  [0.10.19 release note](../../docs/timers/changelog/0.10.19.md).

## [0.10.18] - 2026-10-04

### Changed

- Borrow claim and callback identities during registry cancellation, completion,
  unregistration and provider cleanup. Remove temporary owned copies while
  retaining independent identities in queued tokens, detached handles, effects
  and snapshots.
- Create the ordinary cleanup effect's owned identity only when a wakeup needs
  clearing; inactive cancellation and no-effect terminal paths keep it borrowed.

### Development

- Record the identity ownership cleanup and focused verification subjects in
  the [0.10.18 release note](../../docs/timers/changelog/0.10.18.md).

## [0.10.17] - 2026-10-04

### Changed

- Initialize the canonical registry lazily in its existing runtime slot. Read
  canister version and time only when creating the first epoch; repeated
  initialization returns the original epoch without reconstructing a discarded one.

### Development

- Extend the initialization fixture to check typed borrow-conflict rejection
  and preservation of the original inventory epoch.
- Record the cleanup and pending verification in the
  [0.10.17 release note](../../docs/timers/changelog/0.10.17.md).

## [0.10.16] - 2026-10-04

### Development

- Extend the identity-reuse fixture to check that stale measurements cannot
  update a replacement registration, while its own callback still records
  instruction and memory observations.
- Strengthen existing ordinary and Watchdog failure-accounting assertions to
  distinguish completion outcomes from terminal control failures.
- Record the test-only scope and pending verification in the
  [0.10.16 release note](../../docs/timers/changelog/0.10.16.md).

## [0.10.15] - 2026-10-04

### Changed

- Derive `TimerCounters::work_completed()` from the saturating sum of its four
  classified completion counters. Remove the separately maintained total while
  preserving public counter values and separate start and interruption counters.
- Apply wakeup arms and Watchdog dispatch directly in the validated effect
  branches. Remove two single-caller binding helpers and their redundant variant
  checks while preserving provider ordering, confirmation and failure cleanup.
- Carry inert `MemoryPageExtent` values directly from platform reads through
  callback measurement accounting. Remove the identical private page-count type
  and conversion helper while preserving units and sampling order.

### Development

- Replace the test-only completion-partition checker with direct outcome
  assertions. Cover mixed completion outcomes reaching saturation in the counter
  fixture; retain runtime failure, cleanup and lifecycle assertions.
- Check successful callback completion separately from successor-generation
  failure in the Watchdog lifetime fixture. A failed replacement does not
  reclassify the callback's committed success as an invariant failure.
- Record the cleanup and verification scope in the
  [0.10.15 release note](../../docs/timers/changelog/0.10.15.md).

## [0.10.14] - 2026-10-04

### Changed

- Carry the exact registration claim inside private callback tokens and borrow
  it for context control, Watchdog completion and provider cleanup. Remove claim
  reconstruction and its forwarding helpers while retaining exact running-work
  validation and non-clone public registration capabilities.

### Development

- Adapt the existing running-work negative fixture to construct stale claim and
  callback generations through the token constructor. Keep context-expiration,
  identity-reuse, cross-claim dispatch and provider cleanup subjects unchanged.
- Record the cleanup and verification scope in the
  [0.10.14 release note](../../docs/timers/changelog/0.10.14.md).

## [0.10.13] - 2026-10-04

### Changed

- Return canonical control failures directly from ordinary directive resolution.
  Remove the private directive-error wrapper and registry translation while
  preserving typed failures, command precedence and completion cleanup.

### Development

- Extend the existing schedule fixture to check oversized retry delays and
  overflowing recurrence deadlines at the directive boundary. Test execution
  remains maintainer-owned.
- Record the cleanup and verification scope in the
  [0.10.13 release note](../../docs/timers/changelog/0.10.13.md).

## [0.10.12] - 2026-10-04

### Changed

- Keep callback generation authority in the ordinary and Watchdog control
  counters. Remove repeated generations from private active states while
  preserving snapshot values, stale-callback rejection and checked allocation.
- Select the provider-handle slot in the registry's policy, role, state and
  generation validation match. Remove the second role lookup while preserving
  stale-token rejection, occupied-slot errors and rejected-handle cleanup.

### Development

- Seed exhaustion fixtures before arming callbacks so they reach the generation
  limit with real active tokens. Preserve cancellation, completion, successor
  retention and declaration-lifetime assertions.
- Format and check Rust in both workspaces through `make fmt` and `fmt-check`.
  Reuse the same check from `testing-check`, the commit hook and repository gates.
- Extend the existing staged-snapshot hook fixture to reject unformatted nested
  Rust while preserving the index and unrelated working edits.
- Keep provider churn page growth diagnostic rather than requiring a larger
  memory extent for test success. Preserve cancellation, continued work and stale
  deadline checks, plus the page measurements.
- Remove the release-prose advisory's obsolete `Open release line:` exemption
  and use the current workspace-version marker in its clean fixture.

### Documentation

- Describe ordinary arbitration as current behavior and link historical downstream
  qualification receipts instead of repeating their verdict in the observability
  contract. Record the cleanup and pending verification in the
  [0.10.12 release note](../../docs/timers/changelog/0.10.12.md).

## [0.10.11] - 2026-10-04

### Development

- Extend the native Watchdog dispatch-failure fixture to cover successor binding,
  work binding and effect confirmation for both declaration lifetimes. Check
  provider cleanup, absent consumer work, truthful retained counters and transient
  claim expiration across identity reuse. Test execution remains maintainer-owned.
- Record the test-only scope and verification subject in the
  [0.10.11 release note](../../docs/timers/changelog/0.10.11.md).

## [0.10.10] - 2026-10-04

### Changed

- Decide Watchdog completion's declaration removal once from its final inactive
  state, lifetime and pending unregister command. Remove the separate decisions
  in normal stop and failed successor replacement while preserving continuation,
  terminal reasons and provider cleanup.

### Development

- Extend the Watchdog terminal-failure lifetime fixture to cover generation
  exhaustion during immediate and exact-deadline completion replacement.
- Check that remove-on-stop Watchdogs retain their declaration when keeping or
  replacing a successor, including retention with an exhausted generation counter.
- Record the cleanup and verification scope in the
  [0.10.10 release note](../../docs/timers/changelog/0.10.10.md).

## [0.10.9] - 2026-10-04

### Changed

- Finalize ordinary callback results and callback-borrow failures through one
  completion helper. Remove the separate invariant-stop completion path and the
  unreachable ownership-mismatch recovery branch after successful acceptance.
  Preserve terminal cleanup, declaration lifetimes and measurement behavior.

### Development

- Add a focused native callback-borrow failure fixture covering terminal reasons,
  both declaration lifetimes, provider cleanup and absent work measurements.
- Record the cleanup and verification scope in the
  [0.10.9 release note](../../docs/timers/changelog/0.10.9.md).

## [0.10.8] - 2026-10-04

### Changed

- Store ordinary inactive reasons and pending commands in their corresponding
  inactive and running states. Remove the independent entry fields, paired
  terminal assignments and manual pending-command clearing.
- Use the same checked arming operation for ordinary requests and authorized
  completion successors. Stop directly with its selected reason without a
  separate optional-successor control operation.

### Development

- Correct redundant type visibility qualifiers inside the private control module;
  the crate facade and consumer visibility remain unchanged.
- Update control fixtures for reason-bearing inactive state and verify that
  failed generation allocation preserves a running attempt's pending command.
- Keep the compact handoff focused on current architecture and unresolved work;
  release state remains authoritative in Cargo, changelog and release-note status.
- Record the live 0.10.7 baseline and cleanup in the
  [0.10.8 release note](../../docs/timers/changelog/0.10.8.md).

## [0.10.7] - 2026-10-04

### Changed

- Finalize detached public-claim transitions in one helper. Remove the separate
  success-path helper while preserving restoration, retirement and typed errors.
- Compute ordinary completion's removal-on-stop rule once for normal stopping,
  invariant failure and checked failures. Derive cancellation removal directly
  from execution state and declaration lifetime without a mutable removal flag.

### Development

- Remove ordinary completion's obsolete `too_many_lines` lint expectation after
  the simplification brings it below Clippy's limit.
- Extend the existing transition-error fixture to check failed restoration,
  provider-handle cleanup, claim retirement and restoration-error precedence.
- Record the pushed 0.10.6 baseline and private cleanup in the
  [0.10.7 release note](../../docs/timers/changelog/0.10.7.md).

## [0.10.6] - 2026-10-04

### Changed

- Give ordinary scheduling and completion one checked arming operation that
  allocates a generation and installs its deadline atomically.
- Store Watchdog pending commands inside the awaiting-work state. Remove the
  independent field and explicit clearing when leaving that state; preserve
  reconciliation in the dispatched gap and running-work command precedence.
- Build ordinary provider effects from successful transition inputs and the
  allocated generation. Remove unreachable contradictory-state recovery paths
  while retaining input, callback-authority and provider-ownership validation.

### Development

- Use `if let` for ordinary completion's optional successor selection, addressing
  the maintainer-reported Clippy diagnostic without changing transition behavior.
- Retain behavioral transition and failure fixtures; remove obsolete assertions
  about an independent Watchdog pending field. Extend attempt-retirement coverage
  to check that its queued command cannot affect successor work.
- Record the live 0.10.5 baseline and this cleanup in the
  [0.10.6 release note](../../docs/timers/changelog/0.10.6.md).

## [0.10.5] - 2026-10-04

### Changed

- Store the Watchdog inactive reason inside its private inactive state. Remove
  the independent reason field and paired assignments; active states no longer
  carry irrelevant inactive metadata. Public snapshots and terminal behavior
  retain their existing contracts.
- Finalize cancellation removal once from its policy decision. Remove the
  redundant follow-up failure cleanup; preserve terminal errors, declaration
  lifetimes and provider cleanup effects.
- Route recurring ordinary coalescing through the shared scheduling-request
  handler. Remove the separate counter/metadata update path while retaining
  the existing deadline without allocating a generation or recalculating an
  unused cadence deadline.
- Remove the redundant expected-role argument from private callback-claim lookup.
  Provider installation and effect confirmation retain their operation-specific
  role, generation and state validation.

### Development

- Check the public inactive reason in the existing Watchdog generation-exhaustion
  and deadline-overflow fixtures while retaining atomicity and cleanup assertions.
- Extend the Watchdog failure/lifetime matrix to cover exhausted cancellation
  before and after work dispatch, including selected callback cleanup and the
  absence of successful cancellation counts.
- Extend recurring coalescing coverage to cadence and exact schedules with
  exhausted generations and overflowing hypothetical successors; preserve mode,
  request observations, counters and callback authority.
- Record the maintainer-reported pushed 0.10.4 baseline and this cleanup in the
  [0.10.5 release note](../../docs/timers/changelog/0.10.5.md).

## [0.10.4] - 2026-10-04

### Changed

- Keep ordinary completion authorization at the registry's exact running-work
  boundary. Remove the repeated generation argument and stale-completion check
  from the private control transition, whose only production caller has already
  validated that authority within the same atomic operation.
- Return canonical `TimerControlFailure` values directly from checked ordinary
  control. Delete the private error enum, conversion helper and unreachable
  stale-completion result branch; preserve generation-exhaustion handling.

### Development

- Update control fixtures to the authorized-completion operation and remove its
  superseded stale-generation fixture. The maintained registry matrix retains
  stale-token rejection and unchanged-observation coverage at the live boundary.
- Record the maintainer-reported live 0.10.3 baseline and this cleanup in the
  [0.10.4 release note](../../docs/timers/changelog/0.10.4.md).

## [0.10.3] - 2026-10-03

### Changed

- Give each registry entry one predicate for exact running-work ownership.
  Callback lookup, delegated control and ordinary/Watchdog completion share
  claim, role, generation and running-state validation; remove the separate
  completion checks for the same invariant.
- Keep callback acceptance, provider binding and post-completion measurements
  on their distinct validation boundaries.

### Development

- Cover rejection of unstarted, wrong-role, stale-claim, stale-generation,
  completed and removed work tokens across every policy without observation
  changes. Existing context-expiration and identity-reuse fixtures remain.
- Record the maintainer-reported pushed 0.10.2 baseline and this cleanup in the
  [0.10.3 release note](../../docs/timers/changelog/0.10.3.md).

## [0.10.2] - 2026-10-03

### Changed

- Give runtime provider binding sole ownership of clearing rejected handles.
  Initial arms, Watchdog successor/work dispatch and detached-handle restoration
  use the same consuming operation instead of repeating rejection cleanup.
- Use one Watchdog dispatch cleanup exit for work-binding and confirmation
  failures, confirming the dispatch only after work installation succeeds.
- Preserve registry validation, successor-before-work ordering, partial-dispatch
  cleanup, effect confirmation and callback rollback rules.

### Development

- Add focused binding coverage for unavailable runtime state and expired claims;
  retain installation and restoration fixtures. Extend the Watchdog dispatch
  fixture to cover work-binding and confirmation failures, complete handle cleanup
  and the absence of confirmed or started work.
- Record the live 0.10.1 baseline and this private cleanup in the
  [0.10.2 release note](../../docs/timers/changelog/0.10.2.md).

## [0.10.1] - 2026-10-03

### Changed

- Use one private resolved schedule for explicit requests, ordinary completion
  directives and pending commands. Resolve deadline, requested delay and scheduling
  mode together; remove duplicate intermediate models and registry conversions.
- Represent Stop as the absence of a successor schedule. Remove the private
  directive-snapshot mode mapping and its fallback to earlier entry metadata;
  public snapshots remain inert observations.

### Development

- Update existing schedule fixtures to check resolved scheduling metadata and
  preserve public snapshot conversion and validation coverage.
- Record the maintainer-reported live 0.10.0 baseline and this cleanup in the
  [0.10.1 release note](../../docs/timers/changelog/0.10.1.md).

## [0.10.0] - 2026-10-03

### Fixed

- Give Watchdog invariant failures precedence over pending cancellation when
  reporting the inactive reason and cancellation count. Such failures report
  `Failed`, matching ordinary completion, while retaining successor cleanup and
  declaration-lifetime behavior. This observable semantic change uses a new minor
  line.

### Changed

- Remove the private scheduling-action model that duplicates control generations
  and deadlines. Scheduling returns only an optional initial/replacement arm kind;
  the registry builds effects from authoritative control state.
- Derive owned provider roles from entry policy and handle slot instead of storing
  a second role. Preserve exact claim, generation and role validation and complete
  tokens on detached handles.

### Development

- Update existing control and arbitration fixtures and add focused coverage for
  malformed same-claim provider roles, cleanup and failure classification.
- Record scope and pending maintainer validation in the
  [0.10.0 release note](../../docs/timers/changelog/0.10.0.md).

## [0.9.5] - 2026-10-03

### Changed

- Remove unused absolute deadlines from private provider effects. Policy control
  state remains the deadline owner; effects retain resolved provider delays,
  callback authority and arm kind.
- Make ordinary cancellation return checked success or a typed error instead of
  a generic scheduling action. The registry owns cleanup and pending commands;
  remove the impossible arm-on-cancel and clear-on-schedule branches.

### Development

- Observe authoritative snapshot deadlines in the existing registry fixtures,
  preserving provider-delay, generation and malformed-token coverage.
- Update existing control fixtures to check cancellation state, repeat-cancel
  generation stability and atomic failure on generation exhaustion. Existing
  registry lifetime and runtime handle-cleanup fixtures remain the boundary checks.
- Use one workspace-version projection in the release handoff. Remove the
  redundant latest-release marker and its checks and fixtures.
- Remove the historical README sentence prohibition from release validation.
  Keep canonical API-line and dependency-pin checks, and verify that editorial
  prose does not block structurally valid releases.

### Documentation

- Replace stale preparation instructions with scoped historical evidence and
  current follow-up ownership. Record the completed 0.9.4 release and this
  cleanup in the [0.9.5 release note](../../docs/timers/changelog/0.9.5.md).

## [0.9.4] - 2026-10-03

### Changed

- Build ordinary completion effects from the checked control state and selected
  schedule. Remove the duplicate completion-action translation, pass-through
  cancellation flag and fabricated fallback schedule metadata. Keep cancellation
  policy in the registry, checked generations and existing failure/lifetime rules.

### Development

- Update control fixtures to observe resulting state and generations. Extend the
  existing nested cancellation/ensure fixture to check that only a winning
  cancellation increments its counter.

### Documentation

- Record the maintainer-reported live 0.9.3 release, matching artifacts and
  successful hosted checks/MSRV/tag validation. Close the shipped README issue
  [#3](https://github.com/dragginzgame/ic-timers/issues/3).
- Track scope and pending maintainer validation in the
  [0.9.4 release note](../../docs/timers/changelog/0.9.4.md).

## [0.9.3] - 2026-10-03

### Changed

- Remove ordinary completion's duplicate missing-cadence check. The directive
  resolver remains authoritative; illegal Once recurrence keeps the same typed
  terminal failure, Stop observation and declaration-lifetime behavior.
- Share effect-confirmation deduplication, armed-delay observation and wakeup
  counting after role-specific validation. Watchdog dispatch still validates its
  paired work attempt before recording either counter, including on repeated
  confirmation. Preserve provider binding and recovery ordering.

### Fixed

- Align the README API line and exact shared-registry dependency example with
  the current workspace version, addressing
  [GitHub issue #3](https://github.com/dragginzgame/ic-timers/issues/3).

### Development

- Project both README version fields from workspace truth during bumps, include
  the README in metadata rollback and check it for release drift. Extend the
  existing preparation and release-truth fixtures without changing gate scope.

### Documentation

- Record the live 0.9.2 release, successful hosted checks and MSRV/tag validation,
  and the disposition of the lifecycle-test advisory in GitHub issue #8.
- Refresh read-only downstream adoption evidence: Toko Miner's current dirty
  lockfile selects both timer 0.8.1 and 0.9.2; coherent adoption remains blocked.
- Record scope and pending maintainer validation in the
  [0.9.3 release note](../../docs/timers/changelog/0.9.3.md).

## [0.9.2] - 2026-10-03

### Changed

- Select ordinary arm kinds and allocate completion generations directly,
  removing derived optional planning values while preserving checked transitions.
- Use instruction sample count as the sole empty-summary marker. Public latest
  and maximum getters still distinguish no sample from a zero-valued sample;
  latest and maximum observations continue after count or total saturation.
  Derived Debug output reflects the new private field representation.
- Apply callback-role recovery to every unexpected binding error without repeating
  the complete error-variant list. Watchdog work still traps for rollback;
  ordinary callbacks and scheduler binding failures retain their cleanup paths.
- Share ordinary failed-completion bookkeeping between consumer invariant failures
  and checked control failures. Preserve their distinct inactive reasons, error
  results and reported work counts.
- Store each registry entry's control, callback and cadence in one policy-specific
  payload. Derive policy observations from it and remove the independent callback
  variants and missing-callback state. Preserve scheduling and recovery contracts.

### Development

- Satisfy Clippy with a positive initial-case branch in the ordinary terminal
  fixture, `Option::map_or_else` for optional completion failures and a const
  private entry constructor. Preserve
  exhaustion/lifetime coverage and distinct failure outcomes without suppressions.
- Follow up [GitHub issue #8](https://github.com/dragginzgame/ic-timers/issues/8)
  by separating readiness mapping, repeated message driving and measurement
  assertions from the ordered IcyDB-shaped lifecycle fixture. Read callback
  readiness once. Preserve lifecycle coverage and gate scope; supplemental MSRV
  lint validation remains pending.
- Verify cached and downloaded PocketIC artifacts through one hash-first path.
  Rejection diagnostics use that verification result without hashing or executing
  the rejected binary again. Explicit overrides remain untouched; rejected
  downloads preserve the cache and clean temporary artifacts.
- Give workspace-version reading and mutation one table-scoped owner across
  version display, bumping, staging, release-truth, commit and tag checks. Preserve
  dependency versions and reject failed version selection before staging. Require
  both locked workspaces to resolve one timer package at the workspace version.
- Exercise PocketIC path and automatic-install selection through the actual Make
  recipe instead of matching its source layout. Cover default, environment,
  command-line, identical-path and empty overrides while retaining audited pins.

### Documentation

- Record the maintainer-reported live 0.9.1 release and matching local artifacts,
  removing obsolete bump instructions from the handoff and preparation note.
- Clarify user-owned lockfile staging and mark the original 0.5 Watchdog
  reconciliation restriction as superseded by the 0.8 deadline contract.
- Track the next private implementation changes and pending validation in the
  [0.9.2 simplification note](../../docs/timers/changelog/0.9.2.md).

## [0.9.1] - 2026-10-03

### Changed

- Record running unregistration directly in the registry's pending command,
  removing unreachable control actions and error branches. Consolidate control
  tests around their local transitions; registry tests retain command arbitration.
- Deduplicate Watchdog effect confirmation with its unique successor generation,
  removing the redundant work-generation marker. Preserve idempotent arm and dispatch
  counters and validate work tokens before accepting a repeated confirmation.
- Let Rust enforce provider export and alias visibility, with non-suppressible
  private-interface checks. Keep direct-provider confinement and restricted
  platform declarations enforced by the repository checker.

### Development

- Remove historical Canic adoption status from release validation and automatic
  staging. Release truth remains owned by current package and release metadata.
- Use Make execution fixtures for release gates and all combined release flavours
  instead of recipe-text matching. Cover failure ordering, repeated
  bump invocation, metadata-only staging and the exact tag namespace.
- Exercise provider visibility fixtures on both hosted Rust toolchains, including
  alias leaks, local lint suppression and external access through glob imports.
- Remove unused deserialization derives and the runtime probe's direct `serde`
  dependency; query encoding, host decoding and stable state remain unchanged.

### Documentation

- Refresh the handoff, safety evidence wording and 0.9.0 delivery record for the
  tagged release, removing obsolete instructions to bump from 0.8.4. Release
  artifacts do not substitute for missing validation output.
- Scope the historical lifecycle-composition blocker to its original adoption
  subject and distinguish the later frozen receipt from current qualification.
- Track this work and pending maintainer validation in the
  [0.9.1 simplification note](../../docs/timers/changelog/0.9.1.md).

## [0.9.0] - 2026-10-03

### Changed

- Hard-cut ordinary completion arbitration: a pending exact reconciliation
  replaces the callback scheduling proposal before policy and deadline validation.
  Discarded proposals cannot terminate a valid exact successor. Explicit consumer
  invariant failure remains terminal. `latest_directive()` projects the effective
  exact `ScheduleAt` directive when reconciliation wins.
- Check formatting against a temporary snapshot of staged files during commit.
  The hook no longer formats working copies or rejects partial staging itself.

### Fixed

- Reject conflicting or duplicated workspace/latest-release markers, release-note
  headings/statuses, and current-release changelog headings, even when a correct
  marker also exists. Free-form release prose remains advisory.
- Resume an interrupted release commit/tag phase from the matching clean release
  commit without creating another commit. Validate an existing annotated tag at
  `HEAD`; reject arbitrary commits, conflicting tags and additional staged changes
  for an already tagged release. Combined release targets still always bump.
- Replace unconditional native polling of suspended futures with real wake
  notifications so unrelated due timers can progress.
- Document the suspension helper's intentionally non-`Send` future with a scoped
  Clippy expectation, matching the single-threaded runtime executor.
- Keep the PocketIC ordinary-work gate pending through awaited self-call replies
  instead of a stored waker across ingress contexts. Require an observed reply
  before interleaving assertions and report gate errors in fixture observations.

### Evidence and documentation

- Add native suspended-work/control and exact-reconciliation precedence matrices,
  real-canister await/ingress interleaving and provider-churn PocketIC fixtures,
  and shell regression fixtures for release recovery, contradictory metadata and
  preservation of staged/unstaged files by the hook. Execution remains user-owned.
- Document that registry and owned-handle bounds do not bound provider queue
  memory: cancellation in the pinned provider retains future deadline entries.
  Update the released 0.8.4 handoff and separate recorded release evidence from
  unexecuted 0.9.0 fixtures. See the [0.9.0 note](../../docs/timers/changelog/0.9.0.md).

## [0.8.4] - 2026-10-03

### Fixed

- Select the release target from the requested bump and changelog without
  requiring duplicate handoff markers or particular release-note status words.
- Preserve the complete Watchdog snapshot when an initial cadence request
  fails deadline validation, including its validated-request counter.
- Verify PocketIC binary hashes before executing cached, overridden or
  downloaded binaries, including diagnostic paths.
- Require an annotated release tag in the exact `refs/tags/` namespace at
  `HEAD`; reject same-named branches and lightweight tags.
- Restore pre-bump release metadata and both lockfiles when version preparation
  fails or receives a handled interruption, preserving existing user edits.

### Development

- Update both pinned Rust-toolchain action references to the revision proposed
  by Dependabot PR #5, retaining the existing toolchains and validation gates.
- Preflight the requested version, impact, changelog and release markers before
  the deployment gate. Combined release targets still always bump afterward.
- Remove the redundant arm-variant check and add regression fixtures for
  rejected requests, tag identity, binary verification order, preflight and
  bump rollback. Test execution remains user-owned.

### Documentation

- Corroborate 0.8.1 and 0.8.3 registry publication and refresh Toko Miner's dated
  dependency record without extending earlier managed evidence to its later graph.
- Record the decision to retain the coherent lifecycle fixture and current MSRV
  lint scope for advisory IC-TIMERS-003.
- Refresh the handoff and 0.8.3 note to reflect the completed release. Track
  follow-up scope and pending validation in the
  [0.8.4 release note](../../docs/timers/changelog/0.8.4.md).

## [0.8.3] - 2026-10-03

### Development

- Keep combined release targets explicit: `release-patch`, `release-minor`,
  `release-major` and `release-x` always run their matching version bump after
  deployment validation.
- Classify pending release changes against the most recent reachable release
  tag, so an untagged workspace version does not block the next bump. Explicit
  missing bases and Git failures still fail validation.
- Validate release metadata and both lockfiles before the release commit;
  reject unstaged or untracked work before committing and tagging.

### Documentation

- Reserve version bumps, tests, staging, commits, tags, pushes and publication
  for the maintainer. Automated contributors prepare only the next undated
  changelog section and release-line note.

## [0.8.2] - 2026-10-03

### Fixed

- Remove transient declarations on checked terminal control failures, including
  Watchdog scheduling generation exhaustion and scheduler deadline overflow.
  Clear owned provider callbacks before removing the declaration; retained
  declarations remain observable and inactive.
- Resolve both workspace dependency graphs when checking locked metadata after
  a version bump, so stale root or testing lockfiles fail validation.
- Propagate Git failures during release-impact classification and stop
  repository validation when classification or a required check fails.

### Changed

- Use one public-control handle-detachment path and one Watchdog scheduler-arm
  implementation, removing the duplicated reconciliation and completion flows.
- Restrict private platform items to crate visibility, preventing indirect
  public re-exports of timer authority.

### Development

- Check every shell script individually for syntax errors.
- Prepare versions from the current worktree without a preparatory commit or
  test run. Keep the complete gate in user-operated release targets. Tests,
  commits, release tags and pushes remain user-owned.
- Reject qualified, grouped, multiline and locally aliased public platform
  exports, including type aliases. Add executable regression fixtures for
  provider visibility, validation failures and stale lockfiles.

### Evidence

- Pass 108 native tests, including terminal transient cleanup and queued
  callback removal. These boundary-failure cases have native coverage.
- Pass all nine maintained PocketIC Watchdog subjects and all four policy
  cohorts with the audited PocketIC 15.0.0 binary. CI, Rust 1.88.0 MSRV,
  nested-probe lint, strict Wasm Clippy and package checks pass.

### Documentation

- Correct stale 0.8.1 pre-bump wording in the handoff and release note, and
  document the scope of native cleanup evidence in `SAFETY.md`.
- Record the runtime and tooling changes and validation results in the
  [0.8.2 release note](../../docs/timers/changelog/0.8.2.md).

## [0.8.1] - 2026-10-02

### Development

- Update the development and hosted CI toolchains to Rust 1.99.0 while
  retaining Rust 1.88.0 as the MSRV. Run warning-denied nested-probe linting
  on both toolchains in hosted CI, including every supported policy feature.
- Fix Rust 1.99 Clippy's empty-collection assertions in the native inventory
  test to include collection contents on failure, and collapse the nested
  unregister check in the runtime probe. No runtime or public API change.
- Remove the obsolete scheduler line-count suppression and replace the nine
  active Clippy suppressions with explained lint expectations, so obsolete
  exceptions fail warning-denied checks (Toko Miner IC-TIMERS-002).

### Documentation

- Align current guidance with the tagged 0.8.0 API, exact dependency example,
  Rust 1.99.0 toolchain and nine maintained PocketIC Watchdog subjects. Correct
  stale pending-release wording, distinguish historical downstream adoption
  evidence, and clarify implemented observation semantics. Repository-only;
  no runtime or public API change.
- Refresh the handoff and adoption records with dated 0.8.0 publication and
  scoped Canic/IcyDB/Toko Miner composition evidence. Retire the old lifecycle
  blocker while keeping current deployment and cost comparisons unverified
  (Toko Miner IC-TIMERS-001).

## [0.8.0] - 2026-09-20

### Added

- Expose an inert `TimerRegistrationId` on every timer snapshot, combining the
  runtime epoch and checked registration sequence so consumers can detect
  unregister/re-register counter resets within one epoch.
- Add exact Watchdog deadlines through registration/context
  `reconcile_schedule`, `WatchdogDecision::ScheduleAt`, and
  `WatchdogReconcileState::ScheduledAt`. Sleeping work owners can use one
  Watchdog without an auxiliary deadline timer.

### Changed

- Extend the current snapshot shape and exhaustive Watchdog enums at the 0.8
  minor boundary. Reuse the existing claim sequence, pending-command owner and
  pre-armed successor without compatibility paths or another runtime.
- Define continuity, saturation and source-window requirements for downstream
  interval measurements; update the local consumer projection and probe DTOs.

### Evidence

- Pass 105 native tests and nine audited PocketIC Watchdog subjects, including
  deadline sleeping/replacement, registration replacement with counter regrowth,
  upgrade reset, and trap/instruction-exhaustion recovery after a nested deadline
  proposal. Clippy, rustdoc, Wasm, MSRV, nested-probe lint and package checks pass.

## [0.7.1] - 2026-09-15

### Changed

- Update the exact `ic0` dependency from 1.1.0 to 1.2.0 in both workspace
  lockfiles. The upstream additions do not change the bindings used by the
  private platform module; no runtime or public API adaptation is needed.
- Update the test canisters' exact `ic-cdk` dependency and its macros from
  0.20.2 to 0.20.3. No probe source changes are required; this CDK update only
  affects the nested testing workspace.

## [0.7.0] - 2026-08-28

### Added

- Add progress-sensitive Watchdog continuation through
  `WatchdogDecision::ContinueImmediately`, claim- and context-scoped immediate
  ensure operations, and `WatchdogReconcileState::ScheduledImmediately` for
  the first actionable wake-up.

### Changed

- Hard-cut `reconcile_watchdog` from the generic `TimerReconcileState` to the
  policy-specific `WatchdogReconcileState`. Immediate demand replaces the one
  authoritative pre-armed successor at deadline now; cadence `Continue`, stop,
  cancellation, unregistration, generation ownership, and the two-message
  pre-arm protocol retain their existing meanings.
- Project an immediate successor through the existing snapshot fields as
  continuation mode with zero requested/armed delay. Equivalent or earlier
  deadlines and repeated/dispatched/running requests coalesce without another
  provider handle or snapshot format.
- Make release-truth finalization validate the separate latest-release and
  named-target markers semantically, then advance release history and remove
  the target marker. It no longer requires one exact candidate-prose sentence.

### Documentation

- Define immediate scheduling transitions, pending-command precedence,
  replicated rollback requirements, IcyDB integration calls, state-space
  change, and provider-call cost in a dedicated design record.

### Evidence

- Extend the pinned PocketIC Watchdog matrix with zero-delay initial scheduling
  and successful immediate continuation without cadence-time advancement. The
  complete eight-test trap, exhaustion, lifecycle, isolation, capacity, and
  cancellation matrix passes.
- Freeze native snapshot semantics when an immediate ensure coalesces with an
  overdue cadence wake-up: cadence scheduling mode and armed delay remain
  unchanged while the immediate request and coalescing counters advance.
- Record immediate-versus-cadence instruction/cycle observations and current
  compiler, optimized, and deterministic-gzip size cohorts. The controlled
  Watchdog final Wasm is 877 bytes (0.335%) above after-completion.

## [0.6.1] - 2026-08-15

### Changed

- Treat repository-only release impact as an advisory when the maintainer
  explicitly invokes a version-bump or release target. Empty release subjects
  still fail, and repository-only releases retain the complete validation
  gate.

### Documentation

- Clarify that callback instruction aggregates cover the accepted
  `ic-timers` execution interval, not the complete IC message: provider
  entry/exit, page reads, and the post-interval summary write remain outside.
- Require consumers that need a durable terminal audit receipt for a removed
  transient timer to own it outside the volatile registry; no tombstone or
  parallel authority is added.
- Freeze IcyDB's 0.6 performance-rebaseline and single-package qualification
  requirements without claiming that downstream adoption has occurred.

### Evidence

- Extend the PocketIC cohort with deterministic minimal and bounded
  representative Watchdog callbacks. Report scheduler/work instruction
  intervals and cycle deltas while marking complete-message instructions and
  their residual difference unavailable on PocketIC 15's public test surface.

## [0.6.0] - 2026-08-15

### Added

- Add `TimerInventorySnapshot` and `timer_inventory()` so one atomic bounded
  observation carries the runtime epoch even when the initialized registry has
  no timer declarations.

### Changed

- Define instruction observations as the accepted `ic-timers` execution
  interval, including acceptance, completion processing, and any successor
  binding rather than application code alone. Memory reads bracket that
  interval; provider entry/exit and the summary write remain outside its
  instruction delta.
- Document that terminal `RemoveWhenStopped` completion may remove its timer
  before the final measurement can be retained.

### Removed

- Remove the bare-vector `timer_snapshots()` inventory function. Consumers use
  `timer_inventory()` and read or consume its ordered timer vector; no alias or
  deprecated forwarding surface is retained.

### Documentation

- Refresh the README and compact handoff for the 0.6 API with a
  minimal registration example, scannable policy, ownership, observability,
  safety, development, and documentation tables, and clearer shared-registry
  guidance.
- Record IcyDB's completed exact-0.5.0 hard-cut adoption while preserving its
  tagged 0.3.4 and post-tag 0.3.8 evidence as historical subjects.
- Record Canic's completed exact-0.5.0/schema-3 adoption, including its memory
  projection and one-package graph, while preserving 0.3.8/schema 2 as
  historical evidence and keeping combined-framework qualification open.
- Add a public memory-summary projection example that distinguishes no
  completed sample from an observed zero-growth sample.

### Evidence

- Add a probe-only PocketIC cohort measurement for the start/end page reads
  excluded from callback instruction aggregates. PocketIC 15.0.0 reports an
  empty bracket and the four-read sampled bracket at 200 call-context
  instructions each, an observed delta of zero without changing the runtime
  API or measurement interval.
- Rerun the complete PocketIC watchdog matrix and policy cohorts against the
  hard-cut inventory API; recovery, isolation, capacity, ordering, and policy
  behavior remain green.

## [0.5.0] - 2026-08-15

### Changed

- Hard-cut the shared callback control capability into `OnceContext`,
  `AfterCompletionContext`, and `WatchdogContext`. Each callback now receives
  only the mutation operations legal for its declared policy, while exact
  callback-generation validation and expiry semantics remain unchanged.
- Give all callback contexts the same policy-shaped `ensure_scheduled` naming
  used by their registration capabilities. `OnceContext` accepts an explicit
  schedule, while after-completion and Watchdog contexts use their configured
  cadence.
- Keep nested request arbitration in the registry's existing pending-command
  state and callback generations. Remove request counters that were incremented
  but never read, compared, exposed, or used to select a winning command.
- Add allocation-free start/end Wasm-memory and stable-memory page-extent
  sampling for normally completed scheduler and work callbacks. Snapshots keep
  only the latest extent pair and maximum observed start-to-end growth for each
  role; they never total absolute page counts or fabricate samples for trapped
  or instruction-exhausted work.
- Route completed measurements from the callback token's canonical role rather
  than maintaining separate scheduler and work recording paths. An impossible
  policy/role pairing now fails as an ownership invariant instead of silently
  discarding the record.

### Removed

- Remove the public `TimerContext` type and its policy-probing `ensure_once`
  and `ensure_recurring` methods. No alias or deprecated forwarding surface is
  retained.
- Remove public `TimerError::WrongPolicy`; an invalid policy operation is no
  longer expressible through the callback API. Defensive internal policy
  mismatches fail as ownership invariants.
- Remove `TimerControlFailure::RequestSequenceExhausted` and its stable label.
  Generation, deadline, directive, and provider-binding failures remain.

## [0.4.1] - 2026-08-15

### Changed

- Share ordinary schedule resolution and retained lifecycle registration
  verification instead of maintaining policy-specific copies.
- Encode Once ensure, after-completion ensure, and authoritative reconciliation
  as one closed ordinary request kind, removing a boolean policy gate and its
  pass-through scheduling helper.
- Derive watchdog cancellation and transient-removal decisions from canonical
  post-transition state rather than an additional boolean result.
- Bind provider operations from one complete registry effect rather than
  passing duplicate token and delay arguments beside that effect.
- Validate provider-effect shape before cleanup or platform calls, including
  exact identity and claim-generation agreement between a Watchdog successor
  and its queued work.
- Centralize exact callback-claim ownership checks across dispatch,
  measurement, provider installation, and provider-handle consumption.
- Carry one closed initial/replacement arm kind from ordinary control through
  provider binding instead of splitting and rejoining duplicate arm variants.
  Clear effects likewise carry a non-empty callback selection, so "clear
  nothing" cannot be emitted and a Watchdog scheduler replacement is rejected
  before provider calls.
- Share the detach/transition/restore path used by cancellation and explicit
  unregistration, and remove compatibility-only annotations from private
  control errors.
- Correct public control documentation to distinguish armed wake-ups, pending
  successors, and running work that cancellation cannot interrupt.

### Fixed

- Remove a fresh `RemoveWhenStopped` declaration when it is cancelled before
  its first schedule. Once, after-completion, and Watchdog claims now expire
  consistently on cancellation regardless of whether they ever owned a
  provider handle.
- Reject malformed cross-registration Watchdog dispatch effects before any
  provider callback is armed or observability counter is confirmed.
- Prevent a late callback from an expired claim from consuming the provider
  handle of a replacement registration that reused the same identity, role,
  and callback generation.

## [0.4.0] - 2026-08-15

### Changed

- Correct the pre-1.0 release policy: hard cuts still remove superseded APIs
  without compatibility shims, but any public removal or incompatible public
  semantic change advances the minor compatibility line rather than a patch.
  The `TimerFuture` removal in 0.3.7 is retained and explicitly acknowledged
  as a SemVer mistake.
- Classify changes since the current version tag as crate-impacting,
  repository-only, or absent. Version bumps now reject repository-only
  publication before expensive validation, while `repository-check` provides
  the focused non-publishing validation path.
- Record that the current uncommitted Canic and IcyDB adoption worktrees both
  exact-pin 0.3.8. Development-time package alignment is complete; released
  combined composition still requires one-package qualification from tagged
  downstream subjects.
- Start the 0.4 hard-cut cleanup of the public facade and private runtime:
  remove superseded compatibility surface, consolidate duplicated paths, and
  keep only one canonical authority for each timer operation.
- Make the crate root the only public import path. The former public
  `schedule` and `snapshot` module paths are removed while their retained
  provider-neutral values remain available from `ic_timers::*`.
- Collapse identity validation into `TimerIdentity::try_new`, return identity
  components directly as `&str`, and report field-specific failures from one
  `TimerIdentityError` enum. The component bound is now named
  `MAX_TIMER_IDENTITY_COMPONENT_BYTES`.
- Keep observation construction registry-owned: observation fragments no
  longer implement `Default`, and policy/state helper projections that merely
  duplicated the canonical `TimerSnapshot` surface are private.
- Correct inactive-state documentation to distinguish retained callback
  authority from the absence of a scheduled or running callback generation.
- Consolidate exact-claim lifecycle verification, ordinary scheduled
  transitions, earliest/exact deadline mutation, provider-role slot selection,
  and lazy provider-handle detach/clear selection without adding another
  authority or compatibility layer.
- Mark private platform and detached provider capabilities `must_use` so new
  internal paths cannot silently ignore the obligation to bind, restore, or
  clear them.
- Consolidate watchdog terminal failure transitions so they clear pending
  commands consistently, and allocate the paired scheduler/work generations
  atomically before changing canonical state.
- Keep directive/policy mismatch internal to the registry instead of exposing
  it through the public scheduling-error vocabulary.
- Borrow exact registration claims during private unregistration and derive
  callback claims from one complete callback token, avoiding redundant
  identity allocation and independently supplied authority fields.

### Fixed

- Drain every detached watchdog provider handle before returning the first
  restoration failure. A failed wake-up restoration can no longer skip the
  remaining work-handle capability; focused fault injection covers the
  two-handle path.
- Avoid re-borrowing the registry when an exact detached provider capability
  is already available, eliminating eager alternate-handle evaluation.
- Restore all provider handles after any unexpected synchronous post-detach
  registry error. If restoration itself fails, retire the exact claim instead
  of dropping a linear capability or leaving false scheduled state.

### Removed

- Remove `TimerLabel`, `TimerLabelError`, `TimerIdentity::new`, and
  `MAX_TIMER_LABEL_BYTES`; the intermediate label abstraction duplicated the
  only supported identity constructor.
- Remove conversion from inert `TimerDirectiveSnapshot` observations back to
  executable `TimerDirective` commands.
- Remove duplicate `consecutive_expected_failures` forwarding methods from
  `TimerSnapshot` and `TimerObservabilitySnapshot`; the value remains on
  `TimerOutcomeSnapshot`, and the cheap identity-scoped runtime query remains
  public.
- Remove `TimerPolicy::cadence_ns`, `TimerCounters::provider_arms`, the public
  completion-partition checker, and public helpers on nested directive/runtime
  state. Consumers can use the typed cadence, separate committed arm counters,
  and coherent top-level snapshot projections directly.
- Remove the unreachable public `ScheduleError::MissingCadence` variant.
  Missing recurrence cadence is an illegal internal policy/directive pairing,
  not an error a consumer schedule request can produce.

## [0.3.8] - 2026-08-15

### Changed

- Record Canic's validated uncommitted exact-0.3.6 hard cut: one shared
  inventory now replaces its provider, registry/control state, timer counters,
  and timer-specific performance storage, while combined Canic+IcyDB
  qualification remains blocked on exact patch alignment.
- Clarify that Canic's removed global `TimerScheduled` counter was test-only,
  not a public parity surface; the maintained per-timer `schedules` field maps
  to committed `wakeups_armed`.
- Distinguish initializing an empty shared registry from declaring
  framework-owned jobs, allowing Fleet Coordinator to participate without
  inventing inactive Canic timers.
- Stop freezing Canic's maintained adoption status to one lifecycle state;
  release truth now requires only one structural status marker and does not
  interpret its prose.

## [0.3.7] - 2026-08-15

### Changed

- Make the crate hierarchy explicit: the crate root remains the convenience
  facade, `schedule` and `snapshot` remain public value groupings, and the
  control, registry, runtime implementation, and provider boundary remain
  private.
- Use defining-module imports internally, share ordinary callback erasure and
  duration validation, and document why atomic registry/runtime transitions
  remain cohesive.
- Correct lifetime, scheduling-mode, provider-arm, and work-count comments so
  observations do not overstate retained authority or delivery guarantees.
- Run full hosted Rust/MSRV validation once on pull requests and `main`; tag
  pushes now run only release-truth, exact-tag, and main-ancestry checks instead
  of duplicating the same builds at one commit.
- Add a pre-bump compact-status prose advisory for wording likely to become
  stale at release. It warns before expensive validation and always fails open;
  structural post-mutation release truth remains the only enforced boundary.

### Removed

- Hard-cut the accidental public `TimerFuture` erasure alias. Ordinary
  registration APIs continue to accept any compatible future; callback
  erasure is now entirely internal.

## [0.3.6] - 2026-08-14

### Changed

- Update the maintained IcyDB record: tagged IcyDB 0.226.1 adopted exact
  `ic-timers` 0.3.4, while its validated post-tag integration upgrades to
  0.3.5 and uses claim-scoped armed-wakeup observation without check-then-arm
  control flow.
- Keep release-truth validation structural: exact Cargo, changelog, release-note,
  and compact-status version markers remain enforced without interpreting
  free-form status prose or blocking a finalized version bump.

## [0.3.5] - 2026-08-14

### Added

- Add claim-scoped `has_armed_wakeup` observation to all three registration
  capabilities. The result reflects ownership of the exact future provider
  wake-up handle without making snapshots or observations into scheduling
  authority.
- Add a maintained IcyDB adoption record for its accepted 0.226.1 candidate,
  including the exact dependency, shared-registry inventory, recovery
  evidence, and measured costs.

## [0.3.4] - 2026-08-14

### Changed

- Hard-cut lifecycle reconciliation to retained declarations. The three
  reconciliation helpers no longer accept a declaration lifetime;
  `RemoveWhenStopped` callbacks remain available through direct registration.
- Make runner-executed release and provider-boundary checks portable without
  requiring `rg` on GitHub-hosted runners.

### Fixed

- Add policy-wide provider-binding fault evidence for initial and replacement
  arms, after-completion, partial watchdog binding, effect confirmation, and
  remove-on-stop cleanup.
- Finalize and validate mutable release truth mechanically, including the
  workspace status and version-specific release note, while keeping README and
  the Canic contract version-neutral.

## [0.3.3] - 2026-08-14

### Fixed

- Bind each consumer-work `TimerContext` to its exact callback generation and
  role. Context mutation remains available during nested work, while a context
  retained after completion expires and cannot control a successor or later
  registration.
- Fail closed when a public registration or lifecycle operation cannot bind
  its emitted provider effect. Newly armed and replaced handles are cleared,
  retained declarations become inactive with `ProviderBindingFailed`, and a
  later explicit ensure can recover without a false scheduled snapshot.

### Changed

- Reject explicit release versions that are equal to or lower than the
  workspace version, reject leading-zero SemVer components, and resolve tag
  collisions against the exact tag namespace.

## [0.3.2] - 2026-08-14

### Changed

- Correct the Canic adoption contract: all five dynamic built-ins are retained
  `Once` declarations, root canister-pool maintenance is retained
  `AfterCompletion`, public/lifecycle timers keep remove-on-stop lifetimes, and
  no Canic timer uses `Watchdog` yet.
- Permit one bounded Canic claim-custody collection for quiescence while
  forbidding it from duplicating scheduling state, counters, provider handles,
  generations, pending commands, or reconciliation authority.
- Freeze exact Canic metric projection: schedules use committed
  `wakeups_armed`, executions use `work_started`, successes combine successful
  and no-work completions, stale roles combine explicitly, latest delay uses
  the armed delay, and generation remains optional.
- Remove blanket dead-code suppression from the private platform and registry;
  pure registry fixture helpers are now compiled only for tests.

### Fixed

- Retain freshly reconciled inactive `Once`, `AfterCompletion`, and `Watchdog`
  declarations in the canonical inventory so fixed owners reserve bounded
  capacity before application hooks.
- Bring README, status, adoption, and dedicated release notes in line with the
  tagged and published 0.3.1 release.

## [0.3.1] - 2026-08-14

### Added

- Add exact `Once` and `AfterCompletion` schedule reconciliation, including
  the missing synchronous `reconcile_once` lifecycle helper, with focused
  deadline replacement and nested-request tests.
- Freeze a one-page Canic adapter contract covering identity and policy
  mapping, fallible facade changes, claim-scoped lifecycle composition,
  metrics projection, dependency unification, and the pre-1.0 hard cut.

### Changed

- Gate every release bump on normal CI, Rust 1.88, all supported probe lint
  configurations, the full watchdog PocketIC suite, and comparable policy
  cohorts; fail immediately and before version mutation unless PocketIC is the
  exact audited 15.0.0 binary.
- Automatically install the pinned PocketIC artifact into the ignored
  repository tool cache when no override is supplied, while retaining strict
  version/hash validation and never replacing an explicit override.
- Update and stage the nested `testing/Cargo.lock` during version bumps, then
  perform cheap locked-metadata checks for both workspaces without rerunning
  the already-completed behavioral evidence suite.
- Enforce in normal CI that `ic-cdk-timers` remains used only inside the
  private `platform` module and is wrapped rather than publicly re-exported.
- Treat every superseding pre-1.0 contract change as a hard cut by default,
  without deprecated, dual, fallback, or feature-gated legacy paths.
- Mark all three sole-owner registration capabilities `must_use` so accidental
  loss is diagnosed at compile time.
- Make the registry the sole owner of pending ordinary commands and scheduling
  metadata; private `TimerControl` now receives the registry's already
  arbitrated completion decision instead of maintaining a parallel pending
  machine.
- Explicitly keep Canic log-retention and cycle-top-up work on the ordinary
  after-completion path, reserve `Watchdog` for pre-armed recovery work, and
  reject a shared global suspend switch in favor of owner-specific claims.

### Fixed

- Trap callback messages on unexpected internal completion, accounting, or
  cleanup failures instead of silently discarding them; watchdog work thereby
  relies on message rollback and retains its earlier committed successor.
- Drive IcyDB-shaped retryable and terminal results through real scheduler and
  work callbacks, verifying continuation, successor ownership, truthful
  counters, and terminal clearing.
- Describe the published runtime as 0.3 rather than "unreleased 0.3" in crate
  documentation.

## [0.3.0] - 2026-08-13

### Added

- Add the 0.3 production runtime, centered on a bounded shared registry and a
  two-message watchdog that commits its successor before fallible consumer
  work.
- Freeze the 0.3 Patch 1 runtime contract: a 64-entry canonical registry,
  policy-specific callback and state models, linear provider ownership,
  truthful dispatch and completion counters, lifecycle composition, and
  comparable Wasm and instruction measurement subjects.
- Add the pure fixed-capacity registry engine with unique identity claims,
  deterministic inventory ordering, retained and remove-on-stop lifetimes,
  checked generation and request arbitration, and policy-specific ordinary
  and watchdog transitions.
- Add focused registry tests for the complete directive/decision matrix,
  duplicate and capacity failures, idempotent ensure and cancellation,
  claim-consuming unregistration, stale generations, nested commands,
  unacknowledged watchdog dispatches, and coherent Canic-shaped projection.
- Add one canister-local volatile runtime with a synchronous, idempotent
  initialization seam, callback-owning `Once` and `AfterCompletion`
  registrations, exact-claim control, and bounded live snapshot access.
- Add a native one-shot provider harness and focused runtime tests for linear
  handle replacement/cancellation, execution without a registry borrow,
  after-completion recurrence, nested callback commands, and declaration
  lifetime.
- Add the live synchronous `Watchdog` registration and its two-message
  one-shot protocol: a bounded scheduler owns the next cadence successor and
  queues separate immediate work before returning.
- Add owner-local watchdog execution tests plus an isolated PocketIC probe
  proving that an explicitly trapped work message leaves the previously
  committed successor able to retire the attempt and make later progress; the
  probe also verifies rejection of external CDK timer-executor ingress.
- Add synchronous, idempotent reconciliation helpers for retained
  after-completion and watchdog claims, with explicit scheduled/inactive
  desired state and configuration-conflict rejection.
- Populate normally completed scheduler/work instruction aggregates from IC
  call-context counter type 1, and add an IcyDB-shaped fixture covering
  consumer-owned readiness, one-page work, terminal stop, and commit-guard
  re-enablement.
- Extend the PocketIC probe through upgrade, proving a fresh runtime can
  reconstruct a scheduled watchdog from consumer-owned durable authority
  before the probe's downstream-hook observation and then make progress.
- Complete the PocketIC watchdog promotion matrix with real instruction
  exhaustion, insufficient-cycle deferral and top-up, stop/resume, overdue
  coalescing, scheduler/work-gap cancellation, duplicate demand, simultaneous
  timer isolation, commit-window ensure, and a measured ordered 64-entry
  inventory.
- Add mutually exclusive baseline, Once, after-completion, and Watchdog probe
  cohorts with reproducible Rust 1.88 raw Wasm, instruction, and PocketIC cycle
  measurements; document exact hashes, deltas, complexity, and downstream
  adoption work in the 0.3 evidence report.

### Changed

- Lower the declared minimum supported Rust version from 1.91.0 to 1.88.0
  after proving the current source and locked dependency graph compile for
  both native and `wasm32-unknown-unknown` targets.
- Hard-cut the pre-1.0 value model to validated positive cadence, configured
  after-completion recurrence, closed policy-specific states, private inert
  snapshots, truthful unacknowledged outcomes, split scheduler/work counters
  and instruction aggregates, and no fabricated elapsed-time field.
- Make the provider module private and its timer handle linear. Actual provider
  binding for ordinary timers now stores each returned handle in its canonical
  entry and increments actual-arm counters only after that ownership commits.
- Read IC time and canister version through the private platform boundary using
  the already-resolved exact `ic0` 1.1.0 dependency; retain the exact
  `ic-cdk-timers` 1.0.0 provider pin.
- Extend each watchdog entry's linear ownership to at most two exact provider
  handles: one cadence successor and one dispatched work callback. Terminal
  decisions clear both as applicable, while normal continuation retains the
  already-armed successor.
- Project a retained timer that stops on a retryable failure as `failed`,
  preserving Canic's current operator condition instead of reporting it as
  generically idle.
- Clarify that watchdog trap/exhaustion recovery protects the consumer-work
  message, freeze IcyDB's exact returned-result mapping, and require downstream
  dependency unification plus a canister-wide direct-provider inventory.
- Freeze Canic's adoption boundary: its infallible timer facade must expose
  typed registry-capacity and identity-bound failures rather than panic,
  evict, truncate, or silently ignore them.
- Stage named release notes under an explicit undated version heading and
  teach the release helper to add the date automatically during the version
  bump.

## [0.2.0] - 2026-08-13

### Added

- Add the candidate 0.2 provider-neutral `snapshot` API, including bounded
  structured timer identities, policy and scheduling state, outcomes,
  counters, performance measurements, runtime epochs, and one canonical
  snapshot type.
- Preserve operational distinctions required by Canic migrations: callback
  starts versus completions, completed versus interrupted work, requested
  schedules versus platform arms, and functional consecutive expected-failure
  state.
- Add saturating counter transitions, explicit observation resets, portable
  directive conversion, and total/latest/maximum instruction and elapsed-time
  aggregates.
- Add focused tests for identity validation and ordering, outcome transitions,
  counter partitions, absent measurements, epoch resets, and a Canic-shaped
  lossless projection fixture.
- Add the 0.2 observability and Canic parity contract, an explicit safety
  boundary, and a recurring code-hygiene audit.
- Add Dependabot coverage for Cargo and GitHub Actions dependencies.
- Add tested release automation that promotes `Unreleased` notes into a dated
  version section, plus a guarded crates.io publish target.

### Changed

- Rework the README to identify `ic-timers` as a wrapper around
  `ic-cdk-timers` and explain the shared coordination and recovery rationale.
- Harden the development gate with expanded Clippy lints, rustdoc, shell
  syntax, and pinned GitHub Actions checks.
- Mark public validation errors as non-exhaustive and document the pre-1.0
  public API compatibility policy.

### Not yet implemented

- Registry storage and live inventory population, measured callback execution,
  runtime adapters, lifecycle reconstruction, and recovery watchdog behavior
  remain future work.
- The candidate API remains subject to Canic and IcyDB review; a real downstream
  Canic adapter test is required before the 0.2 contract is accepted.

## [0.1.0] - 2026-08-13

### Added

- Initial workspace, CI, release helpers, and formatting hook.
- Canic-derived deterministic timer control and one-shot platform boundary.
- Architecture and release-line documentation for the planned shared runtime.

### Changed

- Use Rust 1.97.1 for normal development while retaining Rust 1.91.0 as the
  separately checked minimum supported Rust version.
- Document the staged path from public timer snapshot types to the registry,
  lifecycle integration, and recovery watchdog.
