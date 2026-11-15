# IC Scheduling New

Follow the installed TOKO-TOOLING.md engineering baseline and this local overlay.
Read docs/design/DESIGN.md, docs/design/jobs.md and
docs/evidence/consumer-requirements.md before implementation.

The Rust workspace is unpublished, with product packages under
crates/. Timers and Jobs have independent development versions and changelogs
under releases/, mapped in ci/cargo-release-groups.json. Keep publish = false
until the rewrite is complete and crates.io publication is explicitly requested.
Jobs delegates five-field UTC recurrence to Saffron, plus a
standalone Wasm calendar probe. Calendar support is off by default behind the
`calendar` feature. Named zones and seconds cron are outside scope; do not
hand-code calendar logic or recurrence workarounds. Timers has a private Once
state core and retained bindings; public initialization and transport remain
unimplemented. A local installed executor probe covers normal completion and
initial/post-await traps; it does not qualify the timer provider. The independent
qualification workspace contains calendar and executor probes; see
qualification/README.md.

The root Cargo.toml owns product dependencies. Qualification owns its own
catalog and lockfile; dependencies point from qualification to products only.
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
