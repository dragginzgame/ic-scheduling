> Current delivery: [0.46.2 / 0.39.2 qualification](release-candidate.md).
> Earlier open-gate statements below describe their dated implementation slice.

# Installed Once provider slice — 2026-11-15

The ic-timers 0.45.1 development increment now connects its retained Once core
to an IC global timer and authenticated isolated self-calls. The new once-probe
imports the actual product; it is not another model or a replacement provider.
This is a local PocketIC observation, not production/deployment qualification.

## Current supplier qualification — 2026-11-17

Both products and independent qualification select Runtime/guard 0.23.16 and
optional Metrics 0.30.9. Native held-reply and staged Entropy qualification now
use Testkit 0.108.0 / Protocol 0.3.4 with Types 0.21.2, removing the old Protocol
0.2 hold. The full installed executor, Once/watchdog/Jobs lifecycle/resource,
paired Metrics and pinned Entropy suites pass on Linux/PocketIC 16.0.0.
The complete `make qualify` and archive gate passes; source identities and
metadata checks are recorded in the [prepared release evidence](release-candidate.md).
Logs remain under `target/evidence/platform-02316/`. This is local qualification,
not publication or macOS CI evidence.

## Compatible Runtime requirement — 2026-11-17

Scheduling now declares `Runtime = "0.23.14"` (`^0.23.14`) in the product,
qualification and direct Entropy fixture catalogs. The locks now select 0.23.16.
The singleton guard remains owned and exactly selected by Runtime itself;
Scheduling's former `=0.23.15` constraint was not needed to enforce one executor.

The published Timers 0.46.0 / Runtime 0.23.15 resolver conflict is reproduced
under `target/evidence/runtime-pin/published-0460-conflict.log`. A maintained
composition check now selects actual published Runtime 0.23.14, 0.23.15 and
0.23.16 separately with the local Timers and execution-enabled Jobs packages. All
resolve with one matching Runtime/guard and compile for Wasm. A valid legacy
0.22.5 request still refuses specifically because its executor guard's Cargo
`links` identity conflicts; no guard bypass or extra executor was introduced.
The current fixture output is `target/evidence/platform-02316/composition.log`.

This relaxes a consumer requirement without changing provider code. Existing
installed behavior evidence on 0.23.14 and 0.23.15 remains dated below; the current
0.23.16 selection is qualified above. No future Runtime patch is pre-qualified.

## Runtime 0.23.15 adoption — 2026-11-17

Products and the independent qualification workspace now select published
Runtime/guard 0.23.15. Both locks were resolved through the shared dependency
updater; Types remains 0.21.2. Source review against 0.23.14 found only three
internal module relocations (`completion`, `ic0`, `registry`), with identical
file contents and unchanged public entrypoints. The released Runtime archive
SHA-256 `654fbae9abca1d9f1ccf1aa7835ba1303b08bc828706f12c0e8a45446a206410`
matches its non-yanked registry record at source
`80a5b488666629dd28152f6bcb6d489091d55994`.

Testkit 0.107.4 admits Protocol `^0.2.7`; the native held-reply graph selects
0.2.8. Direct Entropy qualification selects Protocol 0.3.2. The HTTP codec used
across this boundary is byte-identical, while unused breaking Protocol surfaces
remain outside the selected features. Keep the native dependency within its
supplier's supported range and revisit it when Testkit admits Protocol 0.3.

Strict product native/Wasm Clippy, native driver Clippy and all three migrated
Wasm probe lints pass. Installed executor lifecycle/admission, the full
Once/watchdog/Jobs failure/lifecycle/resource suite, paired metrics and the
actual Entropy query-refusal/source-custody/upgrade journey pass. Metadata for
products, qualification and the consumer fixture each records one Runtime and
one executor guard. Logs and metadata remain under
`target/evidence/platform-02315/`. These are Linux/PocketIC 16.0.0 observations.
The complete standard captured-source and archive gates subsequently passed
for both 0.46.1 / 0.39.1 deliveries and the compatible-range 0.46.2 / 0.39.2
follow-up; see the [release record](release-candidate.md).

## Runtime adoption completion — 2026-11-16

Published Runtime 0.23.14 (source `a813d4ea5378515984a547a9f4e9d37f2124cf14`)
adds the side-effect-free `raw::in_replicated_execution()` boolean needed by
Entropy. The downloaded archive SHA-256
`6f91abeacf7b1602e67b49391ea1e66d1c8f7e45779de79a5835b9c8dac3afb8`
matches its non-yanked registry record. The source diff from 0.23.13 changes
only that observation. Both Scheduling workspaces now select Runtime/guard
0.23.14 and Toko Tooling 0.44.11. The Entropy adapter selects Protocol 0.2.6;
the maintained held-reply driver retains Protocol 0.2.5 through Testkit 0.107.2's
exact supplier requirement. Types remains 0.21.2 throughout.

The actual pinned Entropy release Wasm builds and its installed
`background_refill_and_lifecycle` passes, including the original typed query
refusal before source mutation, fresh-request custody, background fill,
suspension, upgrade and reinstall. All 24 native tests pass. The maintained
patch also migrates its Testkit startup and standalone example context guard;
no temporary guard bypass or sibling edit is used. Logs remain under
`target/evidence/platform-02313/entropy-02314-installed-fixed.log`.
This clears the consumer API blocker behind
[Platform #19 — Expose replicated-execution observation for Entropy's source admission](https://github.com/dragginzgame/ic-platform/issues/19).
Both standard captured-source gates, final archives and registry delivery now
pass; see the [completed delivery record](release-candidate.md).

## Earlier Runtime adoption review — 2026-11-16

Published Runtime 0.23.13 at `9dfdc0083fa5d6e56da7b90c04c20de4b2d3cc9f`
supplies timer roots, timer arming, canister version, raw stable-memory access
and liquid-cycle observation/burning. Its archive SHA-256
`9f5f46eebbf414e38b7ad6db694ce05bebeae40ff8df28eba6c1e48a6c308df4`
matches the non-yanked sparse-index record. Source review confirms the supplying
primitives requested by these two issues are now public:

- [Platform #14 — Supply a stable-memory boundary for Scheduling's installed upgrade probes](https://github.com/dragginzgame/ic-platform/issues/14):
  the six lifecycle/checkpoint fixtures can use the new raw stable-memory APIs.
  Growth returns `Option<u64>` and reads/writes take the offset first; callers
  must migrate those signatures while preserving actual upgrade persistence.
- [Platform #15 — Supply cycle-pressure APIs for Scheduling's installed submission-refusal tests](https://github.com/dragginzgame/ic-platform/issues/15):
  the real low-cycle Once/watchdog scenarios can use `raw::liquid_cycle_balance`
  and `raw::cycles_burn`, alongside the existing `Call::required_cycles` quote.

Published Testkit 0.107.2 resolves the Types conflict recorded in
[Tooling #5 — Publish a Testkit Types requirement compatible with Runtime 0.23.13](https://github.com/dragginzgame/ic-tooling/issues/5).
Both maintained workspaces now resolve Runtime/guard 0.23.13 and Types 0.21.2;
qualification selects Protocol 0.2.5 and Testkit 0.107.2. Full metadata records
under `target/evidence/platform-02313/` show one Runtime and one guard, with no
legacy Runtime/Types selections in those graphs.

The transport now uses timer roots for delivery and update roots for work.
Bounded flight records retain opaque task observations. Only committed
`TerminatedByCleanup` authorizes abandoning accepted work; a reject or a task
left Running does not. Initial traps retain the earlier committed dispatch
when no task handle can escape. Synchronous submission refusal ends the current
control turn immediately after setting backoff. Failed cleanup remains a
terminal provider fault. Host upgrade guards explicitly trap on unsettled
checkpoints rather than relying on a Rust panic's platform error classification.

Strict product Wasm Clippy, the three migrated probe builds, native driver
Clippy, the installed executor cases and the complete installed Once/watchdog/
Jobs failure, pressure, upgrade and resource suite pass. Paired metrics-off/on
checks pass. Logs remain under `target/evidence/platform-02313/`. These are local
Linux/PocketIC 16.0.0 observations; they are not hosted macOS or registry delivery.

The pinned Entropy adapter now maps the raw_rand/HTTP codecs, task roots,
explicit entrypoints and candidate Timers version to the new cohort. Its 24
native tests pass. Actual Wasm integration still fails at the missing replicated-
execution observation tracked by
[Platform #19 — Expose replicated-execution observation for Entropy's source admission](https://github.com/dragginzgame/ic-platform/issues/19).
A static-only fixture isolated that absent guard to check the remaining APIs;
that fixture was never executed and does not establish query-refusal behavior.
No guard bypass is retained in the maintained patch. Full Entropy installed
acceptance and release admission remain pending. The implementation is tracked in
[Scheduling #2 — Migrate Timers to Runtime 0.23 with one executor and retained watchdog recovery](https://github.com/dragginzgame/ic-scheduling/issues/2).
No supplier source was edited.
The maintainer explicitly chose to wait for Platform #19 before deployment;
earlier version-specific Entropy waivers do not apply to this candidate.

## Implemented boundary

On wasm32, initialize is idempotent for the live heap. bind_once retains a
non-Send callback factory under bounded owner/group/name components. Once exposes
set, status and recover. Native builds expose common types and test the private
state/binding layers, without offering a fake platform scheduler.

Global-timer control reserves bounded flight records before sending self-calls.
No consumer callback runs in that message. Dispatch authenticates self-caller
before decoding exactly 16 bytes: initialization canister version and checked
flight sequence, both little-endian u64. Pending flight lookup supplies declaration
and ticket authority; incoming bytes cannot directly choose a callback slot.
A normal upgrade changes the initialization version; the stale-message fence
is implemented but upgrade delivery remains unqualified. Snapshot restore remains
unsupported. Handles and closures are volatile and must be rebound by the host.

Factory acceptance precedes user code, outside table/flight borrows. Initial
factory traps roll back the work message; the earlier flight record remains and
its rejection blocks the still-current ticket for explicit recovery without
calling the factory again. Post-await runtime cleanup abandons the accepted
attempt. Ordinary rearming is refused until owner reconciliation supplies the
current recovery token. A rejection alone never retires a still-Running future:
that situation faults the provider. Unexpected task cancellation also faults it.

Submission refusal/preflight insufficiency preserves eligibility and backs off;
it is not treated as accepted work. This path is implemented but not yet fault-
injection qualified. Public control errors distinguish validation/recovery errors
from terminal provider faults; the latter stop further dispatch and report Faulted.
Owner commands and callback successors remain separate from business effects.

Provisional, unmeasured bounds: 64 retained declarations, 8 simultaneous flights,
4 submissions per control turn, 1-second pressure/refusal retry. A rotating scan
and duplicate-flight suppression avoid a second scheduling queue. These numbers
are not advertised production capacity or latency guarantees.

Cargo links identity is ic-timers-global-timer (matching the old provider's
singleton identity), preventing co-linking incompatible versions. The installed
Wasm was inspected for exactly one canister_global_timer and the expected
canister_update __ic_scheduling_once export. Previous platform timer values are
checked against the provider's observation; foreign writes fault the provider.
This detection does not prove exclusive ownership against arbitrary host code.

## Installed outcomes

Five fixture scenarios passed:

1. At(0) does not execute inline; isolated work awaits a real echo call and stops.
2. Initial factory trap leaves RecoveryRequired, with no committed callback start.
3. Post-await trap leaves RecoveryRequired, with a committed start but no finish.
4. During the await, repeated owner Inactive defeats callback At(0).
5. During the await, repeated owner At(0) defeats callback Stop and produces one
   later attempt after settlement, not overlapping work.

Both abandonment scenarios refuse ordinary rearm, accept explicit recovery to
Inactive, and subsequently complete a freshly armed attempt. External dispatch
calls are rejected. Native core/binding tests (10) pass. Native/Wasm Clippy,
Wasm Rustdoc, fixture/driver Clippy and formatting checks pass.

Reproduce installed checks using a separately prepared server:

```sh
make check-once POCKET_IC_BIN=/absolute/path/to/pocket-ic
```

The server needs local loopback access. No check downloads a server. Run native
checks with cargo test -p ic-timers --lib --locked --offline; use
cargo clippy -p ic-timers --target wasm32-unknown-unknown --locked --offline -- -D warnings
for the actual provider. The platform API is wasm32-only, so native documentation
alone does not cover it.

Compiler: Rust 1.100.0 (0ef1bc386 2026-11-09). Platform Runtime =0.22.5,
PocketIC client/server 16.0.0. Product and qualification locks select thiserror
2.0.18 for the same tested runtime graph; PocketIC 16.0.0 requires that exact
version. No direct thiserror dependency was added to product code.

Recorded tested Wasm (before subsequent documentation-only edits):
905859887700493cff6aa2773bc2fd5eba262413066e7976f4182d1f818bc80f.
Product lock: 173b63045f39ff598c4c454f7806fde0114da3e45108a39d51f9d194a949d054.
Qualification lock: 89cc6423f8aefb118d36fdb0eee2f51605d7a10f330460f245d293374514eb93.
Server identity is recorded in the [executor prerequisite](executor-probe.md).

## Remaining gates

No watchdog implementation, metrics adapter, pressure/instruction measurements,
submission-refusal injection, callback/cleanup-failure injection, stale-wire and
upgrade scenarios, or actual consumer migration is claimed. Observation is part
of OnceResult but is not yet exported to metrics. Native private-core dead-code
expectations now apply only where the wasm32 transport is absent.

The first vertical Once slice is executable; the rewrite is not release-ready.
Qualify remaining failure custody and limits before expanding into watchdog
coalescing or Jobs execution. No crates.io publication is enabled.

## Platform qualification boundary follow-up — 2026-11-15

The Once driver now uses ic-testkit::pocket_ic, matching ic-platform's supported
qualification boundary. Testkit 0.106.0 selects client 16.1.0; all five installed
scenarios passed again against the prepared 16.0.0 server. See the
[executor follow-up](executor-probe.md#platform-qualification-boundary-follow-up--2026-11-15)
for the new qualification lock identity. The current rebuilt Once Wasm hash is
024400f66cd0320c3795d51ebd3643b69ba77a8a8e8bb313524a064126205ac4.
The preceding documentation-only edits account for a new artifact identity;
no provider behavior was changed by this dependency-boundary correction.

## Held-flight pressure and ownership follow-up — 2026-11-15

A pressure review found that any pending flight delayed unrelated due work by
one second even with capacity available. Deadline selection now excludes only
currently submitted tickets, identified by runtime, slot and ticket identity.
A cancelled/replaced ticket cannot hide fresh demand. Available capacity schedules
other due work at the next timer opportunity; full capacity and refused submission
retain the one-second backoff. There is still one authoritative declaration table.

The installed Once suite now uses Testkit 0.106.0 held-reply-fixtures with its
nonblocking PocketIC client and a caller-owned Tokio runtime. The generated helper
holds real calls behind manual HTTP barriers; simulated time is not advanced
while replies are held, and exact requests are explicitly released. No raw held-
reply machinery or fault-injection feature was added to product code.

The new installed scenario verifies:

- All 64 retained declarations admit, and the 65th returns Capacity.
- Twelve due declarations produce eight held calls and leave four unstarted.
- Repeated equal owner commands on running work cannot produce another future.
- Inactive preserves the eight accepted calls but cancels four waiting demands.
- Reverse-order reply release completes the eight accepted attempts once each.
- Explicitly rearming the four waiting declarations starts and completes each once.
- A deliberate foreign global-timer write makes control return Faulted, makes all
  declaration statuses Faulted, and does not produce another callback start.

All five earlier lifecycle scenarios still pass. Eleven native tests now include
submitted-ticket exclusion, replacement and cross-runtime authority. Product
native/Wasm Clippy and fixture/driver Clippy pass; formatting is checked.

Reproduce with the same make check-once command. Qualified workload is functional
pressure, not an IC instruction/memory benchmark or an external-effect exactly-
once guarantee. Submission refusal, callback/cleanup failure and upgrade/stale-
wire scenarios remain open, as do watchdog and metrics integration.

Tested Wasm SHA-256:
c6b9bcc3f466c066b71659be5f3dfb3a1abda88a12950c1bfa15bd1e3d3425ba.
Qualification lock SHA-256:
38c272cee58929ac45931ec1f04ad59eefd0f739e26e2460156eff95e3302c11.
Compiler, runtime, client and server selections are unchanged from the preceding
Testkit observation. All additional host fixture dependencies remain confined to
qualification.

## Cleanup and armed-upgrade follow-up — 2026-11-15

The installed suite now also passes a consumer destructor trap during post-await
trap recovery. The accepted attempt retains one committed start and no finish;
status becomes terminal Faulted. Reinitialization rejects and twenty further
rounds do not replay work. This qualifies that concrete cleanup failure, not every
possible reply-handler or executor cancellation failure. Product code required
no change for this scenario.

A qualification-only self-call relay checks empty, 15-byte and 17-byte envelopes
are rejected after caller authentication. Unknown current-epoch sequences, a
wrong epoch and a settled flight replay leave callback counts unchanged.

A separate host fixture persists an armed absolute deadline, upgrades the actual
canister, and binds a fresh callback from that host-owned record. The runtime
version changes, the deadline is retained, no callback runs early, and one runs
after advancing past the deadline. An old-version envelope is inert. This old
message does not collide with a live new flight; delayed old responses and upgrades
with accepted work remain unqualified. The fixture admits upgrade only in Armed
state. Snapshot restoration remains unsupported. Stable memory belongs only to
this host fixture; the timer product persists no capabilities.

All earlier completion, recovery, precedence and held-flight pressure checks pass
in the same `make check-once` run. Wasm probe and native driver Clippy pass with
warnings denied; `make fmt-check` passes. Rust 1.100.0, Runtime 0.22.5, Testkit
0.106.0, PocketIC client 16.1.0 and server 16.0.0 remain the tested selections.

Tested Wasm SHA-256:
541031fc0ab09e41c9a2195ee71c97606c476a784eddf3e7e21bc9037cd21623.
Product lock SHA-256:
be82b5212f1b5f054a2270f68fe96bad9987f35a1347b2f3cee0d417a4dd7595.
Qualification lock SHA-256:
38c272cee58929ac45931ec1f04ad59eefd0f739e26e2460156eff95e3302c11.

Submission-refusal injection, remaining callback/cleanup custody, live-flight
lifecycle scenarios, instruction measurements, watchdogs, metrics and consumer
migration remain release gates. These are local observations, not deployment proof.

## Low-cycle submission refusal follow-up — 2026-11-15

A new qualification-only fixture reduces its own liquid cycle balance below half
of the runtime-quoted self-call cost. The observed quote was 42,102,448,000 cycles
on this PocketIC application subnet; this is an environment observation, not a
production price or a measured scheduler execution cost. No dependency, production
hook or alternate transport was added.

The installed check proves control runs under this pressure: after the initially
armed timer fires, the platform timer moves at least one second later while the
handle stays Armed and the callback-start count stays zero. After advancing time,
a second deferred timer is observed with no acceptance. Funding the canister does
not bypass that retained retry deadline. Advancing beyond it results in exactly
one callback start, Inactive status and a cleared global timer.

The observer uses an update call to read the previous global timer and immediately
restore the same value. It does not invoke the provider's arm operation or repair
its state. Checking the changed timer distinguishes actual refusal/backoff from
a canister that never executed its timer. The fixture exercises real low-cycle
submission pressure but does not distinguish runtime preflight refusal from
call_perform refusal, or qualify queue-full submission refusal.

`make check-once` passes including all earlier scenarios. Wasm probe and native
driver Clippy pass with warnings denied; formatting and diff whitespace checks
pass. Compiler, runtime, Testkit, client/server and both locks are unchanged from
the preceding observation. Tested Wasm SHA-256:
d7d79d36c45072296697096b175042060286bb1a1cc98e3e61ef3f7593a572c1.

Remaining gates include other submission-refusal causes, callback/cleanup custody,
live-flight upgrades, instruction measurements, watchdogs, metrics and actual
consumer migration. Snapshot restoration remains unsupported. This is local
installed evidence, not deployment proof.

## Held work and host upgrade admission — 2026-11-15

The lifecycle fixture now supports a real held call through Testkit's existing
held-reply fixture. With one accepted Once future suspended, the driver submits
both a duplicate current flight envelope and another version with the same
sequence. Neither changes the single committed start or Running status. This
checks inert dispatch against an accepted flight, not delivery of an obsolete
unaccepted ticket across an upgrade.

An actual upgrade while Running fails with the typed CanisterCalledTrap code.
The fixture's pre-upgrade hook admits only Armed state. After the rejected
upgrade, the version, demand observation, start count and Running status are
unchanged, and exactly one target request remains held. Releasing that original
request settles the attempt to Inactive with one total start. No simulated time
is advanced while the reply is held.

The host then explicitly arms new future demand. Upgrade succeeds, preserving
that absolute deadline and installing fresh volatile bindings; after the deadline,
the new callback runs once and stops. The fixture's diagnostic run count resets
with its heap, so this is a new post-upgrade count, not a durable business journal.

This qualifies a concrete **host refusal/drain/reconstruction policy**. The
library does not install upgrade hooks or prevent a controller from bypassing
host admission. It does not qualify upgrading with unresolved accepted effects,
restoring executor callbacks, delayed old responses after successful upgrade, or
snapshot restoration. Real Canic/Entropy lifecycle integration remains open.
The initialization API now states this ownership boundary explicitly.

`make check-once` passes all prior and new cases. Probe and driver Clippy pass
with warnings denied, Wasm Rustdoc passes with warnings denied, and formatting
and diff checks pass. The compiler, supplier versions, simulator and lock hashes
are unchanged from the preceding observation. Tested Wasm SHA-256:
f178bc78e2d39a615e328789f9db7e99c6c9253e1fa5d23b8c84cf7842d7816d.

Watchdog implementation, metrics, measured resource bounds and remaining failure
custody still block release. The next implementation step remains watchdog
coalescing once the required Once custody boundaries are accepted; these host
fixtures are not a consumer migration or a release-readiness claim.
