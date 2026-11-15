# Installed executor observation — 2026-11-15

Fresh qualification fixture for the async primitive needed by the Once transport.
No old provider, fixture or release machinery was copied. This probe does not
import ic-timers: it qualifies the supplier separately before provider adoption.
Product dependencies remain unchanged.

## Selection and reproduction

- Published ic-platform-runtime =0.22.5, defaults disabled, runtime feature.
  Its protected executor task and raw Call own future polling/call cleanup.
- pocket-ic client =16.0.0 and separately prepared pocket-ic-server 16.0.0.
- Rust 1.100.0 (0ef1bc386 2026-11-09); release wasm32-unknown-unknown probe,
  native debug driver. Qualification has its own catalog and lock.

Run with a prepared server (no downloads performed by this command):

```sh
make check-executor POCKET_IC_BIN=/absolute/path/to/pocket-ic
```

The server needs local loopback access. The first sandboxed attempt could not
bind a port; the successful runs used the approved loopback-capable environment.
The failed waiting driver was interrupted. This is a local installed PocketIC
observation, not a production deployment or remote network result.

## Observed outcomes

Launch records control state, then sends an unbounded self-call to a self-only
work endpoint. Work either traps immediately, awaits another self-only echo then
traps, or completes after that await. A retained future guard records ordinary
versus trap-recovery destruction. The launch caller waits for the work response.

| Scenario | Launch result | Committed work state | Future destruction |
| --- | --- | --- | --- |
| Normal async completion | Success | Started and completed | Ordinary context |
| Initial work trap | Work call rejected | Work-start change rolled back | No committed guard-drop marker |
| Trap after await | Work call rejected | Started, not completed | Trap-recovery context |

Control state from the earlier message survives both traps. All three expected
outcomes passed on installed Wasm. External calls to work/echo and a malformed
launch argument were rejected; those rejected requests left event state unchanged.
Authentication precedes argument decoding in work. The probe uses a fixed one-byte
scenario input; it does not define the eventual production dispatch wire format.

Additional checks: focused qualification Clippy with warnings denied, shared
formatting and dependency-group listing pass. The Make target was exercised.

SHA-256 identities for the formatted, rerun artifact:

- Wasm: 17504d999fb7e6d8524bbc87c320e6a80d96292e8a6fba359f20321b1242e105
- Qualification lock: af2b19235b11444aa492b19cb1771f3544dffbc088d34147c748683cbc52a125
- Server: 69e324bdb68d32d878b7a9504b1379f08f8d1921272bacb065b0fabb3d0f3792

## Consequences and limits

A destructor does not provide committed recovery evidence for an initial work
trap. The timer provider therefore still needs a flight record committed before
work execution, plus exact rejection/acceptance settlement. The post-await result
supports using this runtime's protected-task recovery context for local cleanup,
not treating every rejection or elapsed deadline as proof of termination.

No global timer or watchdog is installed by this fixture. It proves an earlier
message's control marker survives isolated work traps, not that a watchdog
successor is correctly pre-armed. It does not yet cover callback/cleanup traps,
submission refusal, cancellation, pressure, bounded flights, stale wire authority,
upgrade/snapshot reconstruction or consumer reconciliation. No instruction-cost
or completed timer-provider qualification is claimed. Those gates remain open.
