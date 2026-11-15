# IC Scheduling New

Follow the installed TOKO-TOOLING.md engineering baseline and this local overlay.
Read docs/design/DESIGN.md, docs/design/jobs.md and
docs/evidence/consumer-requirements.md before implementation.
The current scope decision is docs/design/release-scope.md: preserve legacy
capabilities without copying the old architecture; defer additional proposals.

The Rust workspace is unpublished, with product packages under
crates/. Timers and Jobs have independent development versions and changelogs
under releases/, mapped in ci/cargo-release-groups.json. Keep publish = false
until the rewrite is complete and crates.io publication is explicitly requested.
Jobs delegates five-field UTC recurrence to Saffron, plus a
standalone Wasm calendar probe. Calendar support is off by default behind the
`calendar` feature. Named zones and seconds cron are outside scope; do not
hand-code calendar logic or recurrence workarounds. Timers has a wasm32 Once
provider with retained binding, authenticated isolated dispatch and explicit
recovery. Installed local tests cover basic completion, initial/post-await traps
and owner precedence, held-flight pressure, foreign timer detection, consumer
cleanup traps, low-cycle submission deferral and host reconstruction of an armed
deadline across upgrade. Metrics are opt-in and inert, using ic-metrics arithmetic;
recording must never invoke callbacks or change eligibility. Remaining consumer,
resource and lifecycle gates are unfinished. See qualification/README.md and
docs/evidence/once-provider.md.
Watchdog core, shared-table binding and IC transport are connected. Installed
checks cover prearmed successor survival, owner precedence, full-flight delay and
synthetic consumer physical-request admission. Consumer migration and measured
resource bounds remain open.
See docs/evidence/watchdog-state.md.
Jobs has occurrence-progress tests and an opt-in wasm32 Once adapter with explicit
SkipLate or bounded CatchUp policy. Host-owned checkpoints support armed
and paused reconstruction; running or recovery-required work refuses export.
Schema 2 preserves remaining catch-up batches across pause and upgrade. Age is
evaluated at batch selection; finishing skips excess and occurrences that arrived
during the batch. Recovery discards the remaining batch and leaves the job paused.
Unresolved-work persistence remains unfinished; Jobs coalescing is deferred.
See docs/evidence/jobs-progress.md.

The root Cargo.toml owns product dependencies. Qualification owns its own
catalog and lockfile; dependencies point from qualification to products only.
Follow ic-platform’s qualification boundary: use `ic-testkit::pocket_ic`, not
a direct `pocket-ic` dependency. The old ic-platform PocketIC feature is retired.
Use make lint PACKAGE=<name> and make doc PACKAGE=<name> for focused checks.
Use the shared make deps-list/deps-update commands for dependency maintenance.

- Build from the named Canic and Entropy requirements and the maintainer-requested
  calendar-aware Jobs scheduler. Jobs may break its old API. Relay is explicitly excluded.
- Other repositories are read-only unless the maintainer separately authorizes edits.
- Do not transplant the old tree, test suite or release machinery wholesale.
- Preserve effect ownership, isolated IC messages and pre-armed watchdog safety.
- Treat source evidence as a dated local observation, not remote/deployment proof.
- Include adapters and moved code in reduction measurements. Never trade away
  required failure behavior or use formatting compression to meet a LOC target.
