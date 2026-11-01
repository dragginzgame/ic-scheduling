# Shared Tooling and IC Host consumer audit — 2026-10-08

## Scope and verdict

This is a source review of the latest committed upstream changes against IC
Timers' actual callers. It is not a full security audit of either sibling, timer
runtime qualification, or authorization to adopt or repair their code.

**Full Shared Tooling refresh readiness: FAIL.** The committed release runner
still has the independently demonstrated concurrent symbolic-ref preservation
failure in [Shared #62](https://github.com/dragginzgame/shared-tooling/issues/62#issuecomment-6057947562).
Its repair is uncommitted. The archive helper itself has passing upstream native
evidence; its consumer integration remains open. IC Host's inspected ownership
paths need no additional API for the maintained timer harness. That conclusion
does not qualify the incoming consumer dependency graph.

Methods: [flow convergence](../../../audits/flow-convergence-and-duplication.md),
[code hygiene](../../../audits/code-hygiene.md) and
[complexity](../../../audits/complexity-and-technical-debt.md), from adopted Shared
revision `0ba0ad00ed94848e54ecc82629b6b7873b7284c0`. Local overlay:
[code hygiene](code-hygiene.md), Git blob
`287cff14da8edba945be3c093efd35dbc8522e9d`; AGENTS blob
`7574dc2f75d9eb5f16c3ba8fbc480cc34396e49a`. Local command exceptions leave
tests, builds, lint, dependency mutation and release execution user-owned.

Reviewed identities:

- IC Timers released `ae26b854a1a473c5d2d153705c2d13570f378d38`, 0.14.15;
  incoming catalog/lock and release/evidence documentation were already dirty.
  During inspection an external lock update advanced Testkit 0.22.0 / Host 0.5.1
  to Testkit 0.22.2 / four Host packages 0.5.2. Those changes were preserved.
- Shared Tooling `eeb72e741199bd8574280eacb3542d8379b912f6`; its commit
  subject says 0.1.25, but committed VERSION is 0.1.24. Changelog, release
  documentation and runner/fixture have later uncommitted changes, excluded
  from committed qualification and adoption readiness.
- IC Host `c7014995bf0890c1df9cd9b9a6ec14ea70f98c6f`, 0.5.2; only its status
  document was dirty. Product comparison against 0.5.1 changes just the child
  owner and its tests, not artifact, filesystem or tool capture implementation.
- Testkit caller source: 0.22.0 `2951fd19e58799580e60ec0f6f5864d296271a62`
  and 0.22.2 `2da9f92fbe7fc31c37136e99cac46179e08911ad`. Their `pic` startup
  and module implementations are identical; the selected registry startup
  bytes match committed 0.22.2. Later dirty Testkit source was excluded.

## Owners and execution paths

| Behavior | Owner and trace | Consumer obligation |
| --- | --- | --- |
| Failure evidence | Local collector creates identity, selects roots, invokes tar, then CI uploads; shared action selects roots and calls the new archiver | Root selection, original failure status, source/run identity, upload and retention stay with the caller |
| Archive creation | Shared archiver admits output and root/path pairs, rejects overlaps and symlink parents, passes `./PATH` to tar, retains partial output on failure | A fresh output per attempt; strict downloaded-member admission remains a separate trust boundary |
| Snapshot refresh | Shared exporter stages committed blobs/modes, checks second-line companion declarations and destinations, copies files, publishes manifest, verifies | Explicit file selection and common revision for all three local snapshots; interrupted publication is not atomic |
| Release observation | Runner confirms captured-URL branch/tag delivery, derives matching configured upstream, conditionally updates its tracking ref | Optional observation repair must preserve concurrent ref mapping and must not repeat delivered pushes |
| PocketIC lifecycle | [Harness](../../../crates/ic-timers-pocketic/tests/runtime/harness/mod.rs) spawns an explicit audited binary through Testkit, connects an instance, drops instance before server | Executable admission and tests remain local; Testkit owns readiness/deadlines/diagnostics; Host owns child cleanup |
| Child exit and cleanup | Host `try_wait` observes with WNOWAIT, signals group before reaping; `terminate` attempts group signal, direct fallback and reap; Drop attempts cleanup | No independent reaper, escaped-process containment promise or bounded cleanup guarantee |
| Background handoff | Host `poll_exit` reserves leader identity; `handoff` requires successful exit, reaps without group signalling and relinquishes ownership | Caller must own background stop/readiness; this is inappropriate for our ephemeral managed server |

## Findings

### MEDIUM / P2: installed bundles dominate failure artifacts

**Problem/evidence.** The [local collector](https://github.com/dragginzgame/ic-timers/blob/651efce370bd6aec6290b74118eb709488062d0a/scripts/ci/collect-failure-evidence.sh:30)
and [canonical shared action](https://github.com/dragginzgame/shared-tooling/blob/eeb72e741199bd8574280eacb3542d8379b912f6/.github/actions/retain-failure-evidence/action.yml#L37)
select every `host-set.*` / `ic-set.*`. Successful installers retain these
directories and activate relative symlinks to them. The
[local fixture](https://github.com/dragginzgame/ic-timers/blob/651efce370bd6aec6290b74118eb709488062d0a/scripts/ci/test-failure-evidence.sh:19) models `.tools/host`
as an ordinary directory, missing the actual successful activation layout.

Actual late-Linux artifact 11541230299 in
[run 37756960783](https://github.com/dragginzgame/ic-timers/actions/runs/37756960783)
downloaded as a 373,200,529-byte ZIP. Of 766,116,256 regular-file tar bytes,
active IC and host bundles occupy 743,871,580 and 22,231,146 respectively:
over 99.99%. This is a consumer measurement, not a shared artifact measurement.
The archive hash and original observations remain with
[the evidence owner](../evidence/release-history.md#hosted-qualification-at-01415) and
[#30](https://github.com/dragginzgame/ic-timers/issues/30#issuecomment-6057035304).

**Why/change.** Compressing and transporting installed executables obscures a
small failure record. Keep selection policy in collectors: omit successful
activated payloads for unrelated validation failures, preserve useful small
receipts/pins and failed candidates. Do not blindly exclude active targets:
an active installation failing verification can itself be relevant evidence.
Retain that subject through explicit selection. Resolve only managed relative
selection names; do not dereference arbitrary links or remove input files.

**Impact/proof.** Small implementation; medium risk of lost diagnostics if
selection is too broad. No crate API, production Wasm, heap or timer instruction
impact. Archive input drops materially for the measured unrelated-failure case;
compressed size and elapsed-time savings require measurement. Test real host/IC
activation links, failed active verification, failed/unselected candidates,
absent/unmanaged links and original bytes/modes/status. Qualify native collector
and hosted upload/download on all three hosts. Likely files: local collector and
fixture; upstream action and native retention fixtures. Owners:
[Timers #30](https://github.com/dragginzgame/ic-timers/issues/30) and new
[Shared #66](https://github.com/dragginzgame/shared-tooling/issues/66).

### MEDIUM / P2: direct archive adoption breaks strict consumer admission

**Problem/evidence.** Shared [archiver line 56](https://github.com/dragginzgame/shared-tooling/blob/eeb72e741199bd8574280eacb3542d8379b912f6/scripts/ci/archive-evidence.sh#L56)
uses `./PATH`; our [verifier](https://github.com/dragginzgame/ic-timers/blob/651efce370bd6aec6290b74118eb709488062d0a/scripts/ci/verify_failure_evidence.py:15)
rejects any dot component. Its output would therefore be refused. Shared
archive failures return 1; our fixture expects tar's status 23 and reuses an
occupied output, which the shared helper intentionally refuses. These are
adapter incompatibilities, not defects in the qualified archiver.

**Change/compatibility.** Normalize one conventional leading `./` before
canonical-name and duplicate admission; retain absolute/traversal/internal-dot
and alias-duplicate refusals. Give independent attempts fresh outputs. Preserve
the original failed operation's status in outcome records and assert helper
failure/partial retention independently of tar's native status. Document member
spelling and helper exit status upstream. No alternate legacy helper mode or
shared validator framework is needed.

**Impact/proof.** Small implementation, medium risk at the evidence trust
boundary; no public timer API or production cost/size/heap impact. Native tests
must exercise both spellings, duplicate aliases, unsafe names, occupied output,
partial failure and original status preservation. New hosted artifacts need
qualification after integration; old #23 acceptance remains source-bound.
Files: verifier, collector, fixture and shared helper guide. Owner: Timers #30;
[upstream feedback](https://github.com/dragginzgame/shared-tooling/issues/59#issuecomment-6058355719)
does not reopen resolved archive issue #59.

### MEDIUM / P2: committed runner still overwrites a raced symbolic mapping

This existing finding has one owner,
[Shared #62](https://github.com/dragginzgame/shared-tooling/issues/62#issuecomment-6057947562).
The [committed conditional update](https://github.com/dragginzgame/shared-tooling/blob/eeb72e741199bd8574280eacb3542d8379b912f6/scripts/ci/run-release.sh#L273)
can replace a concurrently introduced symbolic ref whose resolved old OID is
unchanged. Confirmed remote delivery is unaffected; this violates local mapping
preservation. Upstream's uncommitted prepared-transaction repair is not covered
by the committed CI pass. Do not duplicate the issue or patch a vendored copy.

Small upstream repair, medium ref-transaction risk; no timer API, Wasm, heap or
instruction delta. Acceptance must preserve relationship/referent under the
same-OID race and completed resume with a single push, on supported native Git
hosts. Once committed and qualified, review one aligned snapshot refresh and
the archive caller integration together. Adoption is a separate authorized task.

## Deliberate no-action decisions

- Keep Host's ordinary waiting separate from explicit handoff. Reaping before
  cleanup would lose group identity; handoff deliberately trades cleanup for
  caller-owned background lifetime. The maintained harness wants cleanup.
- Keep Host `capture_command`'s bounded direct-child contract distinct from
  Testkit's managed server. It overrides IO with null stdin and bounded pipes;
  our server uses file-backed diagnostics and application readiness. There is
  no local capture caller to justify the broader group/IO proposal already in
  [Host #5](https://github.com/dragginzgame/ic-host-tooling/issues/5).
- Do not add direct Host dependencies, a second process owner, timer persistence,
  artifact caches, gzip/Wasm inspection or descriptor-custody adapters. Current
  callers supply no deletion proof or measurable benefit for those additions.
- Shared snapshot companion checking and additive file selection are useful
  existing improvements. Repeated uncommitted refresh friction has its own
  [#64](https://github.com/dragginzgame/shared-tooling/issues/64); our current
  snapshots are not dirty. Cargo-example installation proposal
  [#65](https://github.com/dragginzgame/shared-tooling/issues/65) has no local caller.

## Evidence and limits

Source/diff, owner/API/fixture inspection, locked offline Cargo metadata and
authenticated read-only GitHub issue/job queries ran. Metadata resolves all four
local members at 0.14.15 and no Host/Testkit package in the library graph;
the externally updated lock hash stayed
`f1fd58c2bdcbbaf68f4141870d90a7a66771537dcaed52ab3aa8bc05d1166908`.
No test, build, lint, CI dispatch, release command or
sibling file mutation ran. Upstream feedback was posted; this report and local
handoff/changelog documentation record the review without implementing findings.

Reused exact-source evidence: Shared
[37762726615](https://github.com/dragginzgame/shared-tooling/actions/runs/37762726615)
passed Linux, Intel/ARM macOS and lint/security. Host
[37762087718](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37762087718)
passed Linux, Rust 1.88 and both native macOS gates. Testkit 0.22.2
[tag 37762453183](https://github.com/dragginzgame/ic-testkit/actions/runs/37762453183)
passed checks, portable hosts and PocketIC concurrency on all three hosts;
tag MSRV was skipped. Its same-source main run was still executing Intel checks
at inspection, with all three MSRV jobs passed. None of these upstream runs
qualifies IC Timers' incoming graph. Production Wasm/instruction savings are
zero expected for this tooling scope, not newly measured.

The best remaining work is a bounded collector/integration repair after the
committed tracking correction. No new IC Host feature or timer release is
justified by this audit alone.
