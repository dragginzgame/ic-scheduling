![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# IC Timers code-hygiene overlay

Use the [shared code-hygiene method](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/audits/code-hygiene.md) and
[common audit contract](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/audits/README.md) from the reviewed revision
recorded in the [snapshot](../../../.toko-tooling.snapshot).
[AGENTS.md](../../../AGENTS.md) identifies the adopted baseline and supplies local
command authority under the shared validation cadence. This overlay
selects product scope for a requested review, including a review before a minor
release or after a substantial API change. It adds no automatic gate or schedule.

## Product authorities and scope

Trace the affected facade and owners under `crates/ic-timers/src`, including
their colocated tests. Include `testing/crates/ic-timers-runtime-probe`,
`testing/crates/ic-timers-size-probe` and `testing/crates/ic-timers-pocketic`
when provider, recovery or measurement assertions are
in scope. Repository tooling reviews include affected `scripts/`, workflows,
Make recipes, package inputs and every member of the single root Cargo workspace.

The [IC canister application addendum](https://github.com/dragginzgame/toko-tooling/blob/12459b6029115b830b8477879d3bea2f030b6e63/audits/ic-canister-applications.md)
adds boundary questions for the canister probes and source-bound Wasm evidence.
The library does not own consumer Candid services, stable storage or lifecycle
registration. This adoption adds no product audit or validation gate.

Retain these local obligations:

- The crate root exposes the intended facade. `platform` is the only direct
  IC dispatch/executor boundary and remains private; backend handles and functions
  do not become public control paths. Implementation imports use defining
  owners rather than accidental crate-root re-exports.
- Snapshots are inert observations, validated configuration admits inputs,
  and claims/callback contexts carry runtime authority. Constructors and public
  validation boundaries retain typed rejection and negative coverage.
- Delegated callback capabilities expire with the exact claim, role and work
  attempt. A registration's longer lifetime cannot authorize stale delivery.
- Registry arbitration, provider binding, effect-selected handle cleanup and
  callback finalization retain their distinct suspension and rollback obligations.
  Unaffected handles remain bound; there is no production restoration path.
  Preserve atomic transitions when reviewing large functions or similar flows.
- Scheduler starts, work dispatches, work starts, completions, unacknowledged
  attempts and measurements remain separate events. Counters, totals, deadlines
  and generations retain checked or documented saturating arithmetic.
- [Architecture](../architecture.md), [callback authority](../design/0.5-policy-specific-callback-authority.md)
  and [delivery ownership](../design/callback-delivery-ownership.md) own runtime
  interpretation. [SAFETY.md](../../../crates/ic-timers/SAFETY.md) owns recovery guarantees and their
  maintained PocketIC proof; a native substitute or hygiene verdict cannot
  establish IC trap/rollback behavior.
- README, architecture, status, changelog and safety statements stay aligned
  with the actual implementation and source-bound evidence. Applications retain
  durable intent and reconstruct runtime declarations after upgrades.
- [Release rules](../development.md) own forward-only version preparation, exact
  tags, pre-1.0 minor compatibility cuts and repository-only delivery, including
  the maintainer-approved repository-only release exception.

The shared methods own generic API, import, error, dependency, artifact and
documentation questions. Scope exclusions must name their reason; unavailable
required recovery proof is a gap. A structural review does not replace lifecycle,
security, persistence, deployment or performance qualification.

## Inspection and validation commands

Cheap inspection permitted by AGENTS.md includes:

```bash
git diff --check
rg 'unwrap\(|expect\(|panic!|todo!|unimplemented!|TODO|FIXME|HACK' crates/ic-timers/src
rg 'pub(\(| )|ic_cdk_timers|ic-cdk-timers' crates/ic-timers/src Cargo.toml crates/ic-timers/Cargo.toml
bash scripts/ci/verify-toko-tooling-snapshot.sh
```

Inspect command effects before selecting additional evidence. Applicable locked
metadata and `cargo tree --workspace --duplicates --locked --offline` inspect
the selected graphs without upgrading them. Locked dependency preparation follows
the shared Cargo network policy; dependency upgrades retain separate authority. Package-content inspection belongs with the
existing package owner and does not authorize packaging/build execution.

| Verification class | Local selection and authority |
| --- | --- |
| Focused native test | A named owner regression with `cargo test --locked -p ic-timers --lib <fully-qualified-test> -- --exact`; follows the shared focused-check cadence. Record that the intended test actually executed. |
| Library compile-only development check | Select `check` / `wasm-check` for affected library compile coverage, with locked inputs and `RUSTUP_AUTO_INSTALL=0`. Use offline mode with prepared dependencies; compile test targets without executing them. |
| Other builds and lint | Select the affected library or workspace probe and its current toolchain/features; choose focused tests, doctests, Clippy or probe builds for the changed contract under the shared cadence. |
| Provider or repository lint/fixtures | Existing `provider-check` and applicable focused repository fixtures under the shared cadence; a complete repository aggregate remains a broad gate. |
| Broad gates | `make ci`, `make msrv` and `make release-verify` remain maintainer-owned. |
| Recovery and performance | Select directly affected maintained PocketIC subjects through the release guide; complete recovery/cohort qualification remains part of the owner release gate. |

Only the existing cohort instruction intervals, exact loaded Wasm bytes and
documented memory/cycle subjects support cost claims. Use comparable inputs and
retain the limits in the delivery/measurement owner; structural counts are not
performance evidence. Audit findings supply no automatic repair, dependency
update or release authority.

## Reports and historical comparisons

Requested reports remain in `docs/audits/`, with dated, uniquely named runs and
their necessary artifacts. Record the exact shared method revision, consumer
commit, relevant dirty changes and this overlay's content identity. Preserve
all existing dated reports. The old checklist can be retrieved at
[released 0.13.5](https://github.com/dragginzgame/ic-timers/blob/98c4b296d7461525c15a01e30adbe33b75bcfa38/docs/audits/code-hygiene.md)
for historical reproduction; it is ineligible for new runs. Aggregate comparison
with that method is `N/A (method change)`; unchanged individual assertions retain
their original proof scope. The [adoption review](../evidence/shared-tooling-history.md#audit-method-adoption-review)
maps the retired questions and walks an existing report without executing a
fresh product audit.
