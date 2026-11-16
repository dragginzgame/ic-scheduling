# IC Scheduling changelogs

Timers and Jobs have independent versions and release groups:

- [Timers changelog](releases/timers/CHANGELOG.md)
- [Jobs changelog](releases/jobs/CHANGELOG.md)

The group package manifests own their versions; the root has no shared version.
Developer-tooling maintenance in Timers 0.45.11 and Jobs 0.38.1 removes the unused Jobs dependency alias,
restores strict catalog linting and activates the canonical formatting hook with
focused qualification. Resolves [#1](https://github.com/dragginzgame/ic-scheduling/issues/1).

The first rewrite releases, Timers 0.45.9 and Jobs 0.37.9, were published on
2026-11-15. The latest published versions are Timers 0.45.11 and Jobs 0.38.1,
released on 2026-11-16 through the qualified grouped release workflow.
See [release operations](docs/operations/releases.md).

This repository starts with the rewrite. Earlier legacy history remains in the
[existing Scheduling repository](https://github.com/dragginzgame/ic-scheduling-legacy).
