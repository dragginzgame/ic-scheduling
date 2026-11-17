# Entropy consumer migration qualification

## Current Platform cohort — 2026-11-17

The maintained immutable-source adapter now selects Runtime/macros/guard
0.23.16, Protocol 0.3.4, Types 0.21.2 and Testkit 0.108.0. Wire resolves to
0.24.3 through the existing compatible requirement. Both direct fixture
catalogs select the new Platform versions; the read-only Entropy checkout
and its pinned source revision are unchanged.

The selected Protocol HTTP and randomness modules are byte-identical to the
prior 0.3.2 selection. New unrelated capabilities stay disabled.
The complete `make qualify` gate passes, including native all-target and Wasm
Clippy, native feature tests, the actual release probe and its installed
`background_refill_and_lifecycle` in 21.58 seconds,
including query refusal, source contention/stale custody, refill,
suspension/reconstruction, upgrade and reinstall. Metadata records one
Runtime, one matching executor guard and one Types identity in both staged
workspaces; logs and metadata remain under
`target/evidence/platform-02316/`. This is local Linux/PocketIC 16.0.0
qualification, not an Entropy adoption or deployment claim.

The prior Runtime 0.23.15 / Protocol 0.3.2 / Testkit 0.107.4 observation remains
under `target/evidence/platform-02315/`. Standard captured-source gates passed
that installed journey for both Scheduling delivery batches (0.46.1 / 0.39.1 and the
compatible-range 0.46.2 / 0.39.2 follow-up). See the
[verified release record](release-candidate.md).

The older exact-patch blocker described below is resolved by Testkit 0.107.4,
as supplied in
[Tooling #9 — Admit published Protocol 0.2.7 in Testkit's held-reply fixtures](https://github.com/dragginzgame/ic-tooling/issues/9).
Testkit 0.108.0 now admits Protocol `>=0.3.2, <0.4.0`, so Scheduling's native
held-reply graph also selects 0.3.4. Its installed held-flight/watchdog/upgrade
cases pass. No older native Protocol selection or transitive override remains.

## Protocol 0.2.7 adoption — 2026-11-16

Both direct fixture catalogs now select published Protocol 0.2.7, source
`26834168a74cf76a4b8f93d4450c3da4f9e3f5db`. Downloaded archive SHA-256
`db4ea27c2705897b24b1c7c239db916abbb4c15eb97c1adcda896d0ca3891ff4`
matches its non-yanked registry record. The selected `raw_rand` and
`http_request` modules are byte-identical to 0.2.6; new unrelated capabilities
remain disabled. No application custody or scheduling code changes.

Fresh resolution retains Runtime/guard 0.23.14, Types 0.21.2 and Testkit
0.107.3. The existing Wire `0.24.2` requirement selects compatible 0.24.3,
source `77a19e9d6a3efef7191e3ef3265b94aa4cf6c06b`, archive SHA-256
`9bb811860c129acbf0de96b16f44cad6b554c0e12e3cc3b67f0e5ad0109292e2`.
That source diff changes only the unselected TypeScript Platform projection;
the exercised codec modules are unchanged. Metadata records one Runtime and
one executor guard, with Protocol 0.2.7 and Wire 0.24.3.

Native all-target and Wasm probe Clippy pass. The actual release-Wasm installed
`background_refill_and_lifecycle` passes in 21.16 seconds, preserving query
refusal, HTTP/source contention and stale custody, background refill,
suspension/reconstruction, upgrade and reinstall. Source, metadata and logs
remain under `target/evidence/protocol-027/`. This is local Linux/PocketIC
16.0.0 evidence; the Entropy repository remains read-only.

Scheduling's separate held-reply graph is still constrained to Protocol 0.2.6
by published Testkit 0.107.3. A minimal published-package resolver fixture with
direct Protocol 0.2.7 and Testkit's held-reply feature fails with exit 101.
Complete repository adoption is pending
[Tooling #9 — Admit published Protocol 0.2.7 in Testkit's held-reply fixtures](https://github.com/dragginzgame/ic-tooling/issues/9).
No supplier override, extra executor or removed failure coverage is used.

## Qualification supplier refresh — 2026-11-16

The same immutable consumer revision now qualifies Testkit 0.107.3 and
Wire 0.24.2, retaining Protocol 0.2.6, Runtime/guard 0.23.14 and Types 0.21.2.
The probe and native driver replace the removed `ic_wire::native` entrypoints
with root-level `encode` / `decode` and explicit request/response frame
directions. Method IDs, value schemas and the shared byte/allocation/node/depth
limits are unchanged. The maintained patch materializes and applies cleanly.

Strict native all-target and Wasm probe Clippy pass. The actual release-Wasm
`background_refill_and_lifecycle` passes in 22.21 seconds, including query
refusal before source mutation, refill, source contention/stale custody,
suspension, upgrade and reinstall. Metadata resolves one Runtime and one
executor guard, with one current Wire selection. Logs remain under
`target/evidence/qualification-ic-refresh/`. This is local Linux/PocketIC 16.0.0
qualification, not adoption or delivery in the Entropy repository.

## Runtime 0.23 migration — 2026-11-16

The maintained adapter still materializes only immutable Entropy commit
`6c400b2c0259323ad1464fc692a4c4c378a2b101`; sibling branch/working changes are
not qualification inputs. It binds the actual local Timers candidate, migrates
Runtime to 0.23.14, Types to 0.21.2, Protocol to 0.2.6 and Testkit to 0.107.2,
and uses explicit entrypoint codecs and timer/update roots. The source guard
and standalone example use Runtime's public replicated-execution boolean.
The guard still returns `Error::NonReplicated` before source custody/mutation.

All 24 native tests and the actual release-Wasm installed
`background_refill_and_lifecycle` pass locally on Linux/PocketIC 16.0.0.
The installed observation includes query refusal, background fill, source
contention/stale custody, suspension/reconstruction, upgrade and reinstall.
The existing installed test assertions are preserved; its Testkit startup now
uses `ic-testkit::pocket_ic::PocketIcBuilder` directly. No sibling files change.
The [current adoption evidence](once-provider.md) records supplier identity
and the earlier blocked attempt. The standard captured-source release gate
requalified both release groups successfully; see the [delivery record](release-candidate.md).

## Original timer migration — 2026-11-15

Local observation, 2026-11-15. Entropy is not being rewritten; it is the stable
integration target. Canic's current API is provisional during its rewrite.
This qualifies an isolated migration draft, not adoption, remote state or deployment.

## Source and scope

Entropy 0.3.1 at `6c400b2c0259323ad1464fc692a4c4c378a2b101` was clean before
and after qualification. [The patch and preparation instructions](../../qualification/entropy-integration/README.md)
bind that source to local Timers 0.45.4. Only two manifests and the buffered
timer-binding module change. Consumer qualification sources remain unchanged.

The adapter retains one lazily bound Watchdog at one-second cadence. Disabled,
idle and paused demand become Inactive; scheduled demand becomes At; pending
refill keeps the cadence. The callback maps Stop/Continue/At and NoWork directly.
Opt-out before first binding still creates no declaration. Entropy retains its
pool model, physical source admission, generation, cancellation guard, migratory
future, retry ceiling and backoff. There is no scheduler-owned business retry.

Timers is optional and Wasm-only. Runtime remains Wasm-only and is already needed
for fresh acquisition. The fresh-only Wasm dependency graph contains no Timers.
The installed buffered graph selects one Timers 0.45.4 and Runtime/guard 0.22.5.
Final Wasm inspection found one `canister_global_timer` export and the new
`canister_update __ic_scheduling_once` dispatch endpoint.

Count migration glue in future reduction measurements: the buffered module grows
from 183 to 199 physical lines, including target gates. This is no LOC or
performance improvement claim.

## Results

Rust 1.100.0, Node 24.21.0; prepared PocketIC server 16.0.0 through Entropy's
existing ic-testkit 0.106.0 driver. No direct PocketIC dependency was introduced.

- Native all-feature tests: 7 core and 17 buffered adapter tests passed.
- Fresh-only adapter tests: 7 passed. Documentation tests: one passed, one ignored.
- Strict Clippy passed for buffered and fresh-only native and Wasm builds.
- The existing `entropy-probe` release Wasm built successfully.
- Unmodified installed `background_refill_and_lifecycle` passed in 23.36 seconds.
  It exercises background fill, bounded draw/budget refusal, fresh/buffer
  separation, source contention and stale custody, suspension/reconstruction,
  upgrade and reinstall using actual management randomness in local PocketIC.

This does not qualify heap snapshots, upgrades with unresolved effects, every
provider fault, or production resource cost. Canic integration and actual
consumer adoption remain open. Existing provider failure tests retain their
separate proof responsibilities.

## SHA-256 artifact record

| Artifact | SHA-256 |
| --- | --- |
| Entropy product source lock | `375aa459ebd44e1c41c0b009120dc1fdeb95d38a978dd6677b8735131c5758d6` |
| Entropy qualification source lock | `e1b00653fd1b061198a1a2b2cd3ed3330e688dc97ea8a83a9b9ee486328d3649` |
| Migration patch | `f1e63931ddbf77ffbda01e42958abd1207c88eb5e70909d6f304f0b29d7026df` |
| Staged product lock | `73ac4aa70cc05cb8450a5224a9f863a656e6b55d1c4715e052232fe400901104` |
| Staged qualification lock | `8ccb38fae47f00eb2a19b6e992d33bde2fb145b0b49fb20d9c3b90a3f711c732` |
| Installed entropy_probe.wasm | `277ce37b1a015e9d97be48fde5489f4632750f75c8a58b2a457c09045501240c` |

Scheduling base: `7715fc81d533bbd9bed613a62c80f6f9d18a4a37`; this continuation
adds qualification only and does not change Timers product implementation.
Generated consumer workspaces and build artifacts are ignored, not maintained
copies of the old test suite. Reproduction commands are in the linked README.
