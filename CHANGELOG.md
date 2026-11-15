# IC Scheduling changelogs

Timers and Jobs have independent versions and release groups:

- [Timers — 0.45.0, unreleased](releases/timers/CHANGELOG.md)
- [Jobs — 0.37.0, unreleased](releases/jobs/CHANGELOG.md)

The group package manifests own their versions; the root has no shared version.
Development delivery is GitHub-only for now. Both crates retain `publish = false`;
crates.io publication waits for completion and explicit authorization.

This repository starts with the rewrite. Published history remains in the
[existing Scheduling repository](https://github.com/dragginzgame/ic-scheduling).
