# IC Scheduling

Follow the installed TOKO-TOOLING.md engineering baseline and this local overlay.
Read docs/design/DESIGN.md, docs/design/jobs.md and
docs/evidence/consumer-requirements.md before implementation.
The current scope decision is docs/design/release-scope.md: preserve legacy
capabilities without copying the old architecture; defer additional proposals.

The package manifests own the independent `ic-timers` and `ic-jobs` versions.
`ci/cargo-release-groups.json` selects the release groups and their changelogs
under `releases/`. Qualification packages remain private. The release scope guide owns
publication and archive checks.

Jobs delegates five-field UTC recurrence to Saffron, plus a
standalone Wasm calendar probe. Calendar support is off by default behind the
`calendar` feature. Named zones and seconds cron are outside scope; do not
hand-code calendar logic or recurrence workarounds. Timers has a wasm32 Once
provider with retained binding, authenticated isolated dispatch and explicit
recovery. Preserve pre-armed successor safety, owner precedence, bounded pressure,
trap/cleanup behavior and upgrade reconstruction. Metrics remain opt-in and inert;
recording must never invoke callbacks or change eligibility. Read
[Once evidence](docs/evidence/once-provider.md),
[Watchdog evidence](docs/evidence/watchdog-state.md) and
[qualification guidance](qualification/README.md) for affected checks and limits.
Consumer adoption and production budgets remain host-owned.

Jobs has occurrence-progress tests and an opt-in Once adapter for installed canisters with explicit
SkipLate or bounded CatchUp policy. Host-owned checkpoints support armed
and paused reconstruction; ordinary export refuses running or recovery-required work.
Preserve deadlines, occurrence progress, finite retry budgets, settled cursors
and terminal cancellation across supported checkpoints. Only RetryableFailure
outcomes or explicit RetrySafe recovery authorize bounded retries. Recovery uses
an exact token with Retire/RetrySafe disposition and leaves the job paused.
Unresolved restoration stays blocked; ordinary export refuses unresolved work.
Hosts own atomic persistence and fresh-runtime reconstruction, not heap snapshots.
Jobs coalescing remains deferred. Read [Jobs design](docs/design/jobs.md) and
[progress evidence](docs/evidence/jobs-progress.md) before changing these contracts;
they own schema details, transitions and current qualification status.

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
