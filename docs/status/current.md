# Current handoff

## Timers 0.41.4 delivery selected — 2026-11-08

All four accepted cleanup items are implemented locally. The interactive queue
example, dedicated parser/process/live tests, launcher, Make recipes and obsolete
consumer-tooling fixture are removed. Artifact capture, native management transport
and installed Jobs recovery remain. Cargo removes only ctrlc and its four now-unused
macOS backend packages; all supplier versions from the pending adoption remain.

Native Timers tests share successful inactive Once registration setup. Runtime
assertions focus on actual provider effects and public-result handoff; owner-local
outcome/counter/directive matrices remain. All 89 runtime tests, including generated
command sequences and independent suspension/abandonment checks, are retained.

The private probe protocol replaces three next-work setter methods with one typed
request (fresh method ID 66); its roster shrinks from 64 to 62. Host and probe must
be rebuilt together. Message order, one-time decision consumption, installed subject
names, ordinary/watchdog semantics and checkpoint bytes remain unchanged.
Public library APIs and all eight package versions remain unchanged. Pending notes
remain Jobs 0.34.16 and Timers 0.41.4: private tooling cleanup is patch-compatible
with both published libraries and does not itself require publication.

The earlier supplier adoption remains: Storage 0.69.1, Testkit 0.94.0, Build 0.80.0
and Host Artifacts/FS/Process 0.76.0. Root requirements advance for the incompatible
Testkit/Build/Host generations; Storage's compatible 0.69.0 minimum remains.
Testkit requires Platform 0.27 transitively; Build adds its HMAC 0.13/SHA-2 0.11
closure. All six requested archives were authenticated against registry checksums.
No sibling source or release changed. Current application lock SHA-256:
`00c1d1485a9642cc6b0bd727e3a0d9d526e9b591b5e4f062a59966f05e27502b`.

Focused Linux/Rust 1.99.1 evidence is under
`target/evidence/cleanup-four-20261108/`: locked metadata, snapshot/pin/dependency
and release-adapter admission; strict affected native/Wasm Clippy; 122 passing
native tests; and selected CLI/server admission, input-drift and group-recipe
fixtures using substitute tool effects. Formatting, shell lint and doc links pass.
The prior supplier check evidence remains under `target/evidence/suppliers-20261108/`.
Complete product/release gates and the full installed runtime/cohort roster have
not yet run for this graph. No fresh installed Jobs recovery or macOS pass is claimed.

The three changed installed control subjects passed through the focused driver
with PocketIC 16.1.0 against the final source. Receipt:
`target/tmp/timer-qualification-3360410-0/runtime.json`; the earlier passing run is
also retained. This focused selection does not replace the full runtime roster.
Rust LOC falls from 31,359 to 30,517 (-842): tests -306, examples -536; libraries
remain 5,465, build tooling 907 and unclassified zero. Comparison uses the captured
dirty starting tree, excluding preceding supplier work. Raw physical Rust churn
is +97/-990/net -893 and is distinct from classified code LOC.

Authorized delivery selects only Timers 0.41.4 for a fresh frozen batch, with
crates.io publication and direct `origin/main` delivery. Preview archives compared
with the verified 0.41.3/0.34.15 downloads show changed packaged Timers tests and
Cargo.lock; Jobs is byte-identical except VCS provenance, including its normalized
manifest and packaged lock. Jobs' private demo removal/adoption ships in Git only;
its 0.34.16 notes remain pending. Public dependency requirements remain unchanged.
Comparison evidence: `target/evidence/deploy-cleanup-20261108/publication-impact.json`.

## Delivered baseline — Timers 0.41.3 and Jobs 0.34.15

Both groups were tagged, pushed to `origin/main` and published to crates.io via
`batch-OGberYFh` on 2026-11-08. Intent is complete at index 2 and guarded
reconciliation finished. Do not replay publication. The later handoff commit is
`49a83343c958e0f4a498f115897724c2539bb35b`.

- Captured source: `392c8be1ee3657936fe78dc381b35bc00ed14020`.
- Timers release: `627154ad66920ce41ffa51be5d3ebb51db3f32ef`,
  tag `timers-v0.41.3`; archive SHA-256
  `65e4235764d429e67295a508f8ec053706e7d47f1e635ef7090885084b29e3c4`.
- Jobs release: `33683baa66739472bb3f20fcef22ff4b2a82cf30`,
  tag `jobs-v0.34.15`; archive SHA-256
  `1dc3a29eadb0bed1aaf840c1402419413ecabc12893741a1ade6d9e9e91d5d8e`.

Both complete 36-stage plans and all 35 installed Timers runtime cases per group
passed on Linux/Rust 1.99.1 with PocketIC 16.1.0. Registry bytes, upload bytes,
clean tagged source and all packaged Rust files matched; neither version was
yanked. This evidence belongs to the delivered graph, not the pending refresh.
It does not establish fresh installed Jobs recovery or a new macOS pass.
Hosted matrices were asynchronous; pending/cancelled work is not a pass.

Durable delivery evidence:
`.git/release-state/frozen/batch-OGberYFh/verification/`; original group journals
remain in its `repository.git/release-state`. Raw preparation/release/reconcile
logs remain under `target/evidence/deploy-root-tooling-20261108/`.
First capture `batch-TkFUywgM` stopped at publication-fixture integrity admission,
before version mutation/tag/push/upload. The repaired fixture and both complete
release suites passed in the successful batch. Preserve both batches and logs.

## Tooling and recovery custody

Root metadata plus `toko-tooling.lock` select Tooling 0.34.0, independently of
Cargo's application graph. All 102 entrypoints and 165 managed files remain.
Use `make shared-tooling-update` after changing that declaration and
`make shared-tooling-install` to reinstall the dedicated lock. Keep tooling
admission before release cleanup, publication and other effects.

Completed migration, cleanup, earlier delivery and supplier-deferral details are
preserved in the [preceding handoff](https://github.com/dragginzgame/ic-scheduling/blob/49a83343c958e0f4a498f115897724c2539bb35b/docs/status/current.md).
Preserve earlier batches, tags, source pins, supplier branches/stashes, archives
and recovery evidence. Trimming this summary does not retire any artifact.

Jobs remains Serde-only and consumer-owned; every restoration uses Job::restore
and uncertain effects stay blocked. Timers retains its sole private IC provider,
unsafe confinement, exact Runtime 0.20.2 guard and runtime/cohort gates. The retired
ic-memory remains excluded. Sibling source and releases remain read-only.
