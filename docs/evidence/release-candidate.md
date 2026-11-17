# Scheduling release qualification and delivery

## Completed: Timers 0.46.3 / Jobs 0.39.3

The maintainer identified the remaining Runtime exact-patch dependency as a
consumer constraint. Scheduling now accepts `^0.23.14` in product and
qualification catalogs and the maintained Entropy adapter. Both dependency
locks now select Runtime/guard 0.23.16 and Metrics 0.30.9. Native held-reply and
both pinned Entropy fixture catalogs select Protocol 0.3.4 with Testkit 0.108.0;
Types remains current at 0.21.2. The old native Protocol 0.2 hold is removed.
Types, Protocol and Testkit declarations admit compatible patches; Runtime
continues to select its coordinated singleton guard exactly.

The published Timers 0.46.0 / direct Runtime 0.23.15 conflict reproduces.
The products compose with actual published 0.23.14, 0.23.15 and 0.23.16
consumers, each resolving one Runtime/guard and compiling for Wasm. The
incompatible Runtime 0.22.5 guard still refuses. See the
[provider evidence](once-provider.md) and
`node qualification/runtime-composition-check.mjs`.

Published Timers 0.46.1 / 0.46.2 support Runtime 0.23.15 but still pin it exactly;
0.46.0 pins 0.23.14. Those immutable releases cannot be edited retroactively.
Timers 0.46.3 and Jobs 0.39.3 now publish the compatible requirement, resolving
[Scheduling #3 — Qualify Timers with Platform Runtime 0.23.15](https://github.com/dragginzgame/ic-scheduling/issues/3).
Testkit's Protocol 0.3 error-type change is private to qualification and changes
no Scheduling library API, so both releases are compatible patches. Metrics
keeps its compatible `^0.30.8` requirement; its locked selection advances.

The selected supplier archives match their latest stable, non-yanked registry
records, reviewed on 2026-11-17. Runtime 0.23.16 and Metrics 0.30.9 have identical
`src/` bytes to their prior selections. Testkit's held-reply modules are identical
after module-directory moves; Protocol's exercised HTTP/randomness modules are
unchanged. Published source identities are Platform
`69ba457c282167ebac271ae35a3e8339b4d24e54`, Metrics
`2b0ebb98aacf746c8c38d1a70599dada8b0ce572` and Testkit
`78f1d2fbfbda6cac3b39f49dfe8aaa3c26472135`.

Before capture, `make qualify` passed all eleven unchanged targets on the dirty normal checkout:
tools/snapshot, format/hooks, native and Wasm lint/tests/docs, calendar, installed
executor, full Once/watchdog/Jobs failure/lifecycle/resource checks, paired Metrics,
the pinned Entropy journey and package/archive checks. Product, qualification
and both staged Entropy graphs each resolve one Runtime/guard and one Types
identity; the held-reply graph now uses Protocol 0.3.4. Default product metadata
still excludes Metrics. Logs, metadata and registry records are retained under
`target/evidence/platform-02316/`. This is local Linux/PocketIC 16.0.0 evidence;
macOS CI and source-bound release/publication gates are separate observations.

Batch `batch-Jq5rvHFC` captured committed source
`d7a8158977855ddf8f25175e1ee4154ef4a075e5`, then delivered Timers before Jobs.
Both complete captured-source gates and prepared archive checks passed. Jobs
also passed registry-only all-feature Wasm archive verification against published
Timers 0.46.3. Both annotated tags and `origin/main` match the release commits;
standard reconciliation fast-forwarded the normal checkout.

Independent registry downloads match their non-yanked index checksums and
embedded source commits:

| Package | Release commit | Registry SHA-256 |
| --- | --- | --- |
| ic-timers 0.46.3 | `1ff3de1ec101dc1eeda54048e531dd762f9f0abe` | `978acbba1761d3d45e0e8424368ab0ab69b2bfbb996ac1fe3321cc6f8358f2c5` |
| ic-jobs 0.39.3 | `3c4ec293ed3734d299a00ed40e19b4965b89e161` | `e0c465a675bf4d0c2a40f5145ad1e58850b99d5c1a249ac48d2066d9f60b4caa` |

Both registry entries declare Runtime `^0.23.14`; Jobs requires Timers `^0.46.3`.
Registry-only consumers selecting both releases and exact Runtime 0.23.14,
0.23.15 or 0.23.16 each compile for native and Wasm with one matching executor
guard and no path overrides. The previous Timers 0.46.2 / Runtime 0.23.16
conflict reproduces. Readbacks remain under `target/publication/readback-0463-0393/`;
consumer checks and full batch logs remain under `target/evidence/deploy-0463-0393/`.

Hosted Linux passed on source `d7a8158`; macOS exposed a large-output fixture's
premature `process.exit`, with its sanitized failure artifact successfully
uploaded and linked. The Git-only follow-up at
`31d814ef07faeb0c9afc589bc4d8cd37833b0032` changes that fixture to `exitCode`.
Its local rehearsal and [full Ubuntu/macOS CI with secret scanning](https://github.com/dragginzgame/ic-scheduling/actions/runs/44069071026)
pass, including the diagnostic/timeout checks. This affects no published product
code. Pending 0.46.4 / 0.39.4 notes collect that contributor-only correction;
current published versions remain 0.46.3 / 0.39.3.

## Completed: Timers 0.46.2 / Jobs 0.39.2

The maintainer requested compatible Metrics/Saffron requirements while the
preceding immutable release batch was already delivering. The normal checkout
was first reconciled with that completed batch. Root requirements now use
`ic-metrics = "0.30.8"` and `saffron = "0.1.0"` instead of exact pins.
Cargo interprets these as `^0.30.8` and `^0.1.0`, admitting compatible
patches while excluding the next minor series. No current type/runtime conflict
requires the exact patch restrictions. Feature selections are unchanged.

Locked/offline metadata passed for both product and qualification workspaces.
At the requirement edit both lockfiles were byte-identical to their previous
inputs. Release preparation subsequently advanced only the selected local
product identities. A before/after metadata comparison confirms all external
package identities are unchanged; Metrics remains 0.30.8 and Saffron 0.1.0.
Evidence remains under `target/evidence/dependency-ranges/`.

Batch `batch-KiZkiMsE` captured source
`26dc48583ea44f5674ca977006a29e864c5c694d` and delivered Timers 0.46.2 then
Jobs 0.39.2. Each passed the unchanged complete captured-source gate: tools,
format/hooks, native/Wasm lint, native feature variants, documentation,
calendar, installed executor and full Once/watchdog/Jobs, metrics, pinned
Entropy and archives. Prepared metadata passed separate archive admission;
Jobs also passed registry-only all-feature Wasm archive qualification against
published Timers 0.46.2. Standard reconciliation fast-forwarded the normal
checkout. The new metadata renderer remained the admitted 0.1.0 executable.

Independent registry downloads match the non-yanked index checksums and embedded
source commits:

| Package | Release commit | Registry SHA-256 |
| --- | --- | --- |
| ic-timers 0.46.2 | `e38d3af0a88ac714ec123468c989dd65a26a680a` | `138b380cfecf7608fd697109d7347c002acb68985da8cadcd9fc6b6768c6b63c` |
| ic-jobs 0.39.2 | `6d9f4b96a156410170a59ac378a0909bb41a6706` | `37262586d830f36305db5845b6755014a5be07d07e3ff546338dc1cef5f61fee` |

Both published manifests/index entries contain the intended compatible
requirements, and remote annotated tags `timers-v0.46.2` / `jobs-v0.39.2`
peel to those commits. Readbacks remain under
`target/publication/readback-0462-0392/`; logs remain under
`target/evidence/deploy-0462-0392/` and the standard retained batch locations.
Focused post-push review of the range edit, affected manifests and both release
commits found no actionable issue. The
[Jobs release CI run](https://github.com/dragginzgame/ic-scheduling/actions/runs/44061205676)
was running its native Ubuntu/macOS jobs at this observation; the secret scan
passed. No fresh hosted matrix success is claimed, and no sibling source was
edited or published.

## Completed: Timers 0.46.1 / Jobs 0.39.1

Products and independent qualification now select Runtime/guard 0.23.15.
Direct Entropy consumers select Protocol 0.3.2; Types remains current at
0.21.2. Testkit 0.107.4 resolves the earlier exact-patch constraint from
[Tooling #9 — Admit published Protocol 0.2.7 in Testkit's held-reply fixtures](https://github.com/dragginzgame/ic-tooling/issues/9).
Its native held-reply graph selects Protocol 0.2.8 within the supported
`^0.2.7` range. That exercised HTTP module is identical to 0.3.2. Revisit
the native selection when Testkit supports Protocol 0.3; no unsupported
transitive override or extra declaration was introduced.

Registry downloads match these non-yanked supplier identities:

| Package | Source commit | Registry SHA-256 |
| --- | --- | --- |
| Runtime 0.23.15 | `80a5b488666629dd28152f6bcb6d489091d55994` | `654fbae9abca1d9f1ccf1aa7835ba1303b08bc828706f12c0e8a45446a206410` |
| Protocol 0.3.2 | `80a5b488666629dd28152f6bcb6d489091d55994` | `b2b1db444c839e816c1f2b6edec5ef8966a3f022a73a12819cc0db8477e5b560` |
| Native Protocol 0.2.8 | `4e14822ca504b43e47942aeda4074ca2766da944` | `e6c994e205363960ac400252160967bfdec2e9574e5086232d0ccef88ffa5d77` |
| Testkit 0.107.4 | `67ac61f89a7311e278027bee3078de7e24a0a241` | `8629db43fa7a3e3655184c6173a5ccdecf67a87d29f0c929228bec7f8a1c392b` |

Strict product native/Wasm Clippy, native drivers and the three Wasm probe
lints pass. Installed executor checks, the complete Once/watchdog/Jobs suite,
paired metrics and the actual Entropy query-refusal/lifecycle journey pass.
Metadata for products, qualification and the consumer fixture each records
one Runtime and one executor guard. The [provider evidence](once-provider.md)
and [consumer evidence](entropy-integration.md) record the changed boundaries;
logs and metadata remain under `target/evidence/platform-02315/`.
These are Linux/PocketIC 16.0.0 observations, not a fresh hosted matrix.

Release preparation reviewed the live first-party registry selections on
2026-11-17: Runtime 0.23.15, Protocol 0.3.2, Types 0.21.2, Testkit 0.107.4,
Metrics 0.30.8, Wire 0.24.3 and Toko Tooling 0.44.11 remain current. The
supported native Protocol 0.2.8 selection is retained as described above.
Supplier records remain under `target/evidence/deploy-0461-0391/suppliers/`.

The newly published `ic-cargo-metadata` 0.1.0 resolves
[Tooling #8 — Supply a released successor for the displaced Cargo release metadata editor](https://github.com/dragginzgame/ic-tooling/issues/8).
Registry archive SHA-256
`5430d9278184df2c90d096712bd44b5dffa5c619dba550a646ee979c13400d0d`
matches source `de86fe21dc746eda79e662008f0d670f20970ac7`. Scheduling now
selects that package/bin, release profile and no features in release setup,
native CI and the real transaction fixture. Registry tool installation and
absolute-path admission pass. Both actual group transactions pass with the
new renderer, including qualification locks, unrelated-group preservation,
invalid selections and downgrade refusal. Toko retains transaction, source/graph admission and rollback ownership. Local fixture evidence is
`target/evidence/deploy-0461-0391/metadata-transactions.log`.

Batch `batch-ctELp59j` captured source
`f9fcf56ce7e8c0b35adac492731d7271ef297b3e` and completed both supplier-first
releases and registry publication. Each group passed the complete standard
captured-source gate, including native/Wasm lint, native feature variants,
documentation, calendar, executor, Once/watchdog/Jobs, metrics, pinned Entropy
and archives. Prepared metadata received its separate archive checks. Jobs
also passed registry-only all-feature Wasm archive qualification against
published Timers 0.46.1. Standard reconciliation fast-forwarded the clean
normal checkout. No sibling source was edited or published.

Independent registry downloads match the non-yanked sparse-index checksums
and embedded source commits:

| Package | Release commit | Registry SHA-256 |
| --- | --- | --- |
| ic-timers 0.46.1 | `7c83dc9cc68be8e289eee8a1ae086a1b22930d65` | `49da2550fbafb683db45df0354deda35abddd7f1aab8606d6590fcdc7f095fd1` |
| ic-jobs 0.39.1 | `9c52b71702f0aff0d3e40bf3eb9044f9c4b295a7` | `7ae6adc263236b9243fb67718df0c768d60c5236bfe730c59c29781868f2a588` |

Remote tags are `timers-v0.46.1` and `jobs-v0.39.1`. Readbacks remain under
`target/publication/readback-0461-0391/`; batch logs remain under
`target/evidence/deploy-0461-0391/` and standard recovery locations. Focused
post-push review found no actionable issue in the delivered source/metadata;
the subsequently requested Metrics/Saffron range relaxation was delivered
in the following 0.46.2 / 0.39.2 batch. The [Jobs release CI run](https://github.com/dragginzgame/ic-scheduling/actions/runs/44059386656)
was in progress at this observation; no hosted macOS success is claimed.

## Qualification maintenance — 2026-11-16

The qualification workspace now selects Testkit 0.107.3 and its Protocol 0.2.6
held-reply dependency. The maintained pinned Entropy fixture also adopts
Wire 0.24.2 through explicit framed `encode` / `decode` calls. Runtime/guard
0.23.14, Types 0.21.2 and Metrics 0.30.8 remain current. Product manifests,
root lockfile, source and packaged file inventories are unchanged; this is
Git-only qualification delivery. Timers 0.46.0 and Jobs 0.39.0 remain the
published versions. Pending development notes are retained for their next
actual releases rather than publishing unchanged product payloads.

Registry archive review selected these non-yanked supplier identities:

| Package | Source commit | Registry SHA-256 |
| --- | --- | --- |
| ic-testkit 0.107.3 | `3c3fd63a6c16f4f1c38eef7984d211791022ba37` | `f1abdecdc13e767e8f5f7f83f2c19e48a414d6a7eea2e2ffdbfe95222c56d6c2` |
| ic-wire 0.24.2 | `1da48059e36132b09775f6130638b8b9c3169538` | `852a98f3b818bf1e6841c63845d88ded595b659181377e749cf8cf50b12b48c6` |

Strict native driver and native/Wasm Entropy probe Clippy pass. Installed
executor lifecycle/admission checks, the full Once/watchdog/Jobs suite and the
actual Entropy query-refusal/refill/suspension/upgrade/reinstall journey pass.
The real release metadata transaction fixture passes for both groups, retaining
independent qualification-lock synchronization, unselected identities and
downgrade refusal. Metadata shows one Runtime and one executor guard in both
qualification graphs, and one Wire 0.24.2 in the consumer fixture. Logs and
package inventories remain under `target/evidence/qualification-ic-refresh/`.
These are local Linux/PocketIC 16.0.0 observations, not a fresh hosted matrix.

The separately selected release metadata executable remains on qualified
ic-build 0.73.0. Current ic-build 0.87.3 has no `cli` feature, binary target or
equivalent rendering export. Its missing supported successor is tracked by
[Tooling #8 — Supply a released successor for the displaced Cargo release metadata editor](https://github.com/dragginzgame/ic-tooling/issues/8).
New-editor adoption is not claimed. Upstream simulator-owned IC package
versions remain in PocketIC 16.1.0's supported graph; no forced transitive
major-version overrides or sibling source changes were introduced.

## Completed: Timers 0.46.0 / Jobs 0.39.0

2026-11-16: Scheduling adopted Toko Tooling 0.44.11 and published Runtime
0.23.14 / Types 0.21.2 / Testkit 0.107.2. The maintained Entropy adapter uses
Protocol 0.2.6. Scheduling's held-reply driver retains Protocol 0.2.5 through
Testkit's exact requirement. Both maintained workspaces resolve one Runtime
and one executor guard; qualification retains its independent catalog and lock.
Hosts composing Timers with Runtime 0.22 must migrate their callers together,
so these releases advance the minor versions. Jobs checkpoint schema 7 and
calendar behavior remain unchanged.

Batch `batch-waYcz4QJ` captured adoption commit
`fc5a4f16c3be0b48cec0532b0d091192ace8c039`, then completed both supplier-first
releases and registry publication. Each group passed its complete standard gate:
tool/snapshot/format/hook admission, strict native and Wasm Clippy, native
feature variants, Wasm documentation, calendar, installed executor, complete
Once/watchdog/Jobs failure/lifecycle/resource checks, paired metrics, the pinned
Entropy native/Wasm/installed journey and archives. Prepared metadata received
its separate archive checks. Jobs additionally passed the registry-only
all-feature Wasm archive check against published Timers 0.46.0. Normal-checkout
reconciliation fast-forwarded through the standard batch command.

The maintainer chose to wait for
[Platform #19 — Expose replicated-execution observation for Entropy's source admission](https://github.com/dragginzgame/ic-platform/issues/19).
Runtime 0.23.14 supplies that boolean, and the actual Entropy query-refusal and
lifecycle journey passes without bypassing its guard. No Entropy waiver was
used and no sibling source was edited or published. This completes
[Scheduling #2 — Migrate Timers to Runtime 0.23 with one executor and retained watchdog recovery](https://github.com/dragginzgame/ic-scheduling/issues/2).

Toko Tooling 0.44.11 also requires actionable CI setup diagnostics. Scheduling
retains bounded, sanitized failures and selected hidden Rust attempt logs,
with a seven-day artifact lifetime and explicit upload/missing-evidence results.
The focused offline failure rehearsal passes; hosted failure upload/retrieval
has not been observed. Both exact tooling pins, lockfiles and all 76 managed
snapshot files agree with the reviewed registry archive.

Independent registry downloads match the non-yanked sparse-index checksums and
embedded source commits:

| Package | Release commit | Registry SHA-256 |
| --- | --- | --- |
| ic-timers 0.46.0 | `ccb6bd7c7aed6ce51a1f67274029d1370fdc74e1` | `5824a0528889edf09e57818528e5dda36f6ce9ab9819650e07a7d9867add9a17` |
| ic-jobs 0.39.0 | `38fb219f1fda831b6d8439acfd6282d8b86947cc` | `665db056c7b2cce1e56094bc8dbb6fca03b5c6d7ab204f4dd472f66167108bae` |

Remote annotated tags `timers-v0.46.0` and `jobs-v0.39.0` peel to those commits.
Readbacks remain under `target/publication/readback-0460-0390/`; batch recovery
and publication evidence remain in their standard retained locations. The
normal log is `target/evidence/platform-02314/release-batch.log`. Focused
post-push review of the adoption and both release commits found no actionable
issue. Local installed evidence is Linux/PocketIC 16.0.0. The
[Jobs release CI run](https://github.com/dragginzgame/ic-scheduling/actions/runs/43954676663)
was in progress at this observation; no hosted macOS qualification is claimed.

## Completed: Timers 0.45.12 / Jobs 0.38.2

Supplier review on 2026-11-16 selected published Toko Tooling 0.44.6 and
Testkit 0.107.1. Metrics 0.30.8 remains current. Runtime 0.23.11 was probed and
refused during Cargo resolution because its `runtime` feature no longer exists.
Review of its released source at `cbbf26ade6644ed6fbcadfe3ccbafe809c362f25`
also found no public global-timer setter, canister-version primitive or
trap-aware protected-task API. Scheduling needs all three for timer ownership,
upgrade incarnation and abandonment custody. Retain the qualified Runtime
0.22.5 graph for this delivery; migration remains blocked until Platform
provides the required primitives and their installed failure contract can pass.
No local runtime replacement or weakened recovery behavior is introduced.

The metadata editor remains the qualified IC Build 0.73.0; optional adoption
of current 0.87.3 is deferred to the next release-tool maintenance batch. This
delivery does not change its metadata contract. The pinned Entropy journey
remains required: its source is read from the exact Git commit, independently
of the consumer checkout's current branch and dirty files. No Entropy source
or working-tree changes are included. Existing version-specific waivers
did not apply to these releases.

The exporter preview and installation passed, retaining the same 76 managed
files and entrypoint selection. Product APIs, Jobs schema 7 and supported
consumer build requirements are unchanged; both are patch releases.
Chrono 0.4.45, Saffron 0.1.0 and Tokio 1.54.1 also remain current stable selections.

Capture `batch-CsXseGvy` stopped in qualification-driver Clippy before version
preparation or delivery: Testkit 0.107.1 removed `pic::fixtures`. Product native
tests passed, but the complete gate did not. The driver now uses Testkit's
`held::{Catalog, Fixture, Limits, Reply}` directly, retaining slot/reply/name
budgets and converting PocketIC identifiers through their byte boundary.
The failed capture and logs remain retained. A fresh capture qualified the
changed driver; this adapter does not support reusing partial failed-gate receipts.

Batch `batch-L1VZpgtf` completed both supplier-first releases and publication on
2026-11-16. Each group passed its complete captured-source gate: strict native
and Wasm Clippy, native feature variants, Wasm documentation, calendar,
executor, Once/watchdog failure/lifecycle/resource checks, metrics comparison,
the pinned Entropy native/Wasm/installed journey and archive verification.
Jobs additionally passed the registry-only all-feature Wasm archive check
against published Timers. The standard reconciliation fast-forwarded the
normal checkout; no sibling source was edited.

Independent downloads matched the sparse-index checksums and embedded source
commits:

| Package | Release commit | Registry SHA-256 |
| --- | --- | --- |
| ic-timers 0.45.12 | `459294282341af10b0f20c1bd53e595ece548fc9` | `1f7011aa45f76d32ccc065482ea7dda6755e58f1ea205fd361d0c3b4146215ac` |
| ic-jobs 0.38.2 | `3ec37ab990cb4b13fc252d1813643254e6ce2eb2` | `67732eb7c44bdb10a82d027ac0352b09c42af08f410757f296e0a34add673358` |

Remote tags are `timers-v0.45.12` and `jobs-v0.38.2`. Readback artifacts remain
under `target/publication/readback-04512-0382/`; standard batch and publication
receipts remain in their existing recovery locations. Post-push review of the
adoption, affected qualification callers and both release commits found no
actionable issue. The [Jobs release CI run](https://github.com/dragginzgame/ic-scheduling/actions/runs/43914621435)
was in progress at this observation; hosted macOS success is not claimed.
The Runtime 0.23.11 migration remains blocked as described above.

## Completed: Timers 0.45.11 / Jobs 0.38.1

2026-11-16: deploy the completed issue #1 tooling fixes using Toko Tooling 0.44.5.
Live registry review confirms Runtime 0.22.5, Metrics 0.30.8, Testkit 0.106.0 and
Toko Tooling 0.44.5 remain the latest non-yanked stable selections. Both locks
change only Toko Tooling. Public package versions remain unchanged until the
standard supplier-first patch release prepares them.

Carry forward the maintainer's explicit instruction to exclude Entropy from
this deployment, scoped to these two versions. Its checkout has advanced beyond
the adapter's pinned revision; no fresh Entropy qualification is claimed.
Scheduling's native/Wasm, installed failure/lifecycle/resource, metrics and
archive gates remain required, with the newly adopted canonical hook checker.
Focused catalog lint, deliberate unused-alias rejection, group metadata
transactions, hook preservation/refusal cases and local hook activation passed
before source capture. Hosted Linux/macOS hook qualification remains separate.

Both groups passed the complete selected captured-source gate and final archive
checks, then published successfully in batch `batch-Ji8w7KBh`. Jobs passed the
registry-only all-feature Wasm check against published Timers. The normal checkout
was fast-forwarded through standard batch reconciliation. Independent registry
downloads matched the sparse-index checksums and embedded source commits:

| Package | Release commit | Registry SHA-256 |
| --- | --- | --- |
| ic-timers 0.45.11 | `63972519969497199dd54f3a0fba6ea7c236c98c` | `3b2cc50b49c7fb6f919c05d13ad47b6fcc2cb95718d3d38c48374728e73f8954` |
| ic-jobs 0.38.1 | `d774cdaa0301fab7aa8051133818d6d37c8846b1` | `7928150a1e62f4d62e853448f868636183b6e55b477e007bad3373bf0786c429` |

Remote tags are `timers-v0.45.11` and `jobs-v0.38.1`. Readback is retained under
`target/publication/readback-04511-0381/`; batch recovery and publication evidence
remain in the standard `.git/release-state/frozen/` and `target/release-frozen/`
locations. Focused post-push review found no new actionable issue. The repository
had zero open GitHub issues after delivery; #1 remains completed. The
[Jobs release CI run](https://github.com/dragginzgame/ic-scheduling/actions/runs/43894479344)
was in progress at this observation; no hosted macOS pass is claimed yet.

## Completed: Timers 0.45.10 / Jobs 0.38.0

Registry freshness reviewed on 2026-11-15 before source capture: the selected
first-party suppliers remain the latest non-yanked stable releases: Runtime
0.22.5, Metrics 0.30.8, Testkit 0.106.0 and Toko Tooling 0.44.4. No dependency
refresh is required. Timers retains its public API; Jobs changes checkpoint
storage to schema 7 and consolidates progress ownership. Both groups passed
the captured-source Scheduling gate and archive checks before publication.

Maintainer exception, 2026-11-15: omit the Entropy integration journey for these
two versions only. Batch `batch-9KO0urcM` stopped before version preparation or
delivery because the external Entropy checkout's lockfiles changed during
qualification. Its native, Wasm, installed lifecycle/resource and metrics checks
passed; a separate archive check also passed. Preserve that failed batch as
evidence. A new source capture records this exception and reruns the remaining
release gate. Ordinary qualification and later versions still include Entropy;
these releases do not claim fresh Entropy integration qualification.

Both releases completed on 2026-11-15. Native feature variants, native/Wasm
Clippy, Wasm documentation, calendar/executor/Once/metrics probes, installed
failure/lifecycle/resource checks and extracted archive verification passed.
Jobs additionally passed registry-only all-feature Wasm verification against
published Timers before upload. The release metadata fixture was corrected to
use declared pending versions rather than assume patch increments; both groups'
transaction, preservation and refusal checks pass.

| Package | Release commit | Registry SHA-256 |
| --- | --- | --- |
| ic-timers 0.45.10 | `bdce527484ccf3cede46f8f8330b71efe8be37d2` | `e9bc6f83f3b47cdfc531fb78ab9fab02781dd5e2709bee1de67829f1395cb2ac` |
| ic-jobs 0.38.0 | `2e63eca87e1b778c6125ce5064b079fe55eca64d` | `0b7c292662ba6f40c136a1c32cf0472073f3d212142a83c39f6f68a5bdf66b97` |

Independent registry downloads matched the sparse-index checksums and embedded
source commits. Remote annotated tags `timers-v0.45.10` and `jobs-v0.38.0`
resolve to those commits. Batches `batch-5gltZZ7C` and `batch-5wnJft7r` completed
and were reconciled into the normal checkout. Their release/publication evidence
is retained under `.git/release-state/frozen/` and `target/release-frozen/`;
registry readback is under `target/publication/readback-04510-0380/`.
The interrupted pre-delivery batch `batch-_ZWc1Z_4` is also retained; it was
stopped to correct the metadata fixture and did not publish.

Post-push review covered the delivered implementation, adapters, caller fixtures
and release metadata; no further actionable finding remained. Hosted CI is
asynchronous: the Timers run was superseded by the Jobs push; the
[Jobs release run](https://github.com/dragginzgame/ic-scheduling/actions/runs/43764591401)
was still running at this observation. No macOS success is claimed here.

## Historical candidate: Timers 0.45.9 / Jobs 0.37.9

Local observation, 2026-11-15, from the implementation changes following GitHub
checkpoint `bc36b91`. This is source/installed-simulator evidence, not deployment
or crates.io delivery. Historical open-gate prose in the individual evidence
ledgers describes earlier slices; this records the current combined candidate.

## Implementation and qualification

- Jobs: explicit bounded retry-safe results and owner recovery, schema 6 progress,
  schema 1 unresolved records and latest outcome observations. Actual upgrades
  preserve running/uncertain attempt identity, consumed budgets, owner revisions
  and catch-up custody. Reconstruction remains blocked until explicit disposition.
  Ordinary unresolved export still refuses. No exactly-once effects are claimed.
- Timers: retained/remove-on-stop lifetimes, explicit removal, checked declaration
  incarnations, bounded slot reuse and capture release outside registry borrows.
  Installed tests cover initial/post-await traps, transient cleanup traps, owner
  precedence, reentrant destructors and obsolete queued Once/watchdog deliveries.
- Optional diagnostics: bounded sorted inventory and per-declaration normal
  results/failure observations, without control capabilities. Native and installed
  checks verify inert reads, reset behavior and canonical arithmetic.
- Native tests: Jobs 41 with all features / 23 without default features; Timers
  28 / 25 respectively. Strict native/Wasm Clippy and Wasm Rustdoc pass.
- The complete `check-once`, `check-executor`, `check-calendar-wasm` and
  `check-metrics` targets pass, including pressure, refusal, authentication,
  cleanup/fault custody, upgrades, recurring schedules, cancellation and retries.
- The isolated pinned Entropy adapter passes native tests, native/Wasm Clippy
  with/without features and its unchanged installed background-refill/lifecycle
  journey. The source repository was not edited. The patch explicitly selects
  retained watchdog lifetime. Actual adoption remains separate.

Rust 1.100.0, PocketIC 16.0.0. Live sparse-index review found the selected suppliers
current: Runtime 0.22.5, Metrics 0.30.8, Testkit 0.106.0 and Toko 0.44.4. Latest
published stable Scheduling versions were Timers 0.44.3 and Jobs 0.36.3.

| Artifact | SHA-256 |
| --- | --- |
| Combined installed probe | `f97c2913ffae0b6f7512cfe5dbc430b54bcc511ec2eb18e3cef2e061a66c32c8` |
| Entropy installed probe | `f1ea33d4b8fecb37ffc38a24de03cb86cbc174933f5c04f21f8958e4d995791c` |
| Entropy adapter patch | `f8bc67d07cbad158f5b1333a8203aed700b51387669195261de9451ce1280c23` |
| Product lock | `f4a9eec05b30405b4af44b8b2b91d44cb6354d8f61b2ae8682def9009fc5b881` |
| Qualification lock | `bb9e961b782edbd6456b769c39a020905069f5299bede00b2ceb9270adce6911` |

The small metrics probe measured 89,431 bytes disabled and 97,280 enabled
(+7,849). Three fresh canisters gave identical per-mode instruction samples:
setup +1,848, Once factory prefix +126, Once resume prefix +74 and watchdog
prefix +144. These are fixture deltas, not full production execution costs.
Calendar and optional execution remain excluded from timer-only consumers.

## Packaging boundary

Both candidate archives pass Cargo's native verification. Timers verifies for
Wasm. Cargo 1.100 multi-package Wasm verification fails resolving unpublished
Timers from its own temporary registry with `no hash listed`; a fresh target
directory reproduces it. Compiling the extracted Jobs archive with its only
local override pointing to the extracted Timers archive passes Wasm checks.
The normalized published dependency still names crates.io Timers; no path is
published. Registry-only Jobs verification is required after Timers publication.

This record does not claim release-adapter qualification, GitHub delivery,
registry availability, macOS execution or production consumer budgets. Heap
snapshots and legacy performance comparisons are outside the accepted release
scope; no code-reduction percentage is claimed from this expanded feature set.

## Release workflow preparation

The official Toko exporter installed the 0.44.4 release/publication closure; all
71 managed files verify. Both real group metadata transactions pass, including
qualification lock synchronization, unselected-group preservation, invalid public
selection refusal and downgrade refusal. ShellCheck and Actionlint pass. The
canonical manifests retain the preceding GitHub checkpoint versions until the
standard runner finalizes 0.45.9/0.37.9; this avoids advancing past the intended
candidate merely to adopt release tooling. Native macOS CI is now configured;
its first successful run and actual delivery are not claimed by this record.

## Completed delivery

Both crates were published on 2026-11-15, with independent registry downloads
matching the live sparse-index checksums and embedded source commits:

| Package | Release commit | Registry SHA-256 |
| --- | --- | --- |
| ic-timers 0.45.9 | `bcf945c134c838ccc26208ea671a8500f7f0b7b3` | `43219d539588feafbe5aeb1b30d32ebfc19c4843daca1b67ef9f10d9af6ecf7e` |
| ic-jobs 0.37.9 | `e30de83e4ac7d0927d256fa20580773ff4a9c92c` | `dd8f588d286624e355ddb9954b6e801777cb7535bf1b8817f31ecc610dfc3a52` |

GitHub tags are `timers-v0.45.9` and `jobs-v0.37.9`. Both were pushed atomically
with their release commits. [Native CI](https://github.com/dragginzgame/ic-scheduling/actions/runs/43757640848)
passed on Ubuntu 24.04 and macOS 15 ARM64, including setup, tests, archives and
real metadata transactions; its single repository-history secret scan passed.
The captured-source full gate passed separately for each release group, including
the installed lifecycle/resource suite and pinned Entropy journey.

Jobs' first publication admission stopped before upload because its manually
extracted archive was inside the parent Cargo workspace. The identical archive
was then extracted outside that workspace; locked registry preparation and
all-feature Wasm checking passed against published Timers, without a path override.
No Jobs upload intent/dispatch had been created by the failed admission. After
rechecking source, tag, package eligibility and registry absence, Toko's standard
single-attempt publication runner uploaded that unchanged archive once. Registry
readback verified it. The batch's saved publication outcome was explicitly
reconciled as published, then the normal checkout was fast-forwarded through the
standard batch reconciliation command. Source, tags and registry versions were
not replaced. The helper now uses an external temporary directory.

Batch `batch-JYVpRsua` is complete. Retained local evidence is under
`.git/release-state/frozen/batch-JYVpRsua/` and
`target/release-frozen/batch-JYVpRsua/target/publication/`; downloaded registry
archives and the successful registry-only Wasm log are in
`target/registry-verification/`. No retained artifacts were automatically removed.
The optional metadata editor remains the qualified IC Build 0.73.0 selection;
0.86.0 was observed available and its nonessential refresh deferred.
