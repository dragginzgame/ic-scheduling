# Frozen API wire

`api-pre-platform.didbin` was encoded before Platform adoption from the unchanged
Jobs 0.17.0 API declarations at `312ff2486f7d15d60be92a14c58cc2bbd45e3d05`,
using Candid 0.10.38 on Rust 1.91. The incoming workspace also selected Memory
0.48.2 and direct Host 0.27.2; this fixture encodes API values, not stored bytes.

The original argument tuple covered installation configuration, all timing/task
variants, every status, a delivery envelope, summary and consumer errors. After
the Platform 0.2 Wire cut, current tests use these frozen Candid bytes only as
rejected protocol inputs. No current Candid encoder or reader remains, and
ordinary checks do not regenerate the file. Its SHA-256 is
`0735c60e61923f3dfba86b182ffbbdbf98d2b8ea769796b9fb1e24522c8117f5`.

## Principal transport continuity

`principal-platform076.json` freezes current-contract Init/Delivery Wire frames
and stored JSON from Jobs 0.17.45 at
`23ae2c813bfd60ca20ec377f6745045235e33b41`, using Platform 0.7.6's external
Principal 0.1.5, Wire 0.7.21 and Rust 1.91. Each row uses principal bytes `[2; n]`
for lengths zero through 29. Delivery fields use u64::MAX and a 256-byte `x`
message. The producer verified that the external-Principal tuple frames and JSON
matched the then-current owned DTOs before freezing them.

The Platform 0.8 feature split no longer supplies external Principal Serde as an
incidental capability. The existing compatibility test compares owned DTOs
against these frozen outputs without enabling another protocol. Ordinary checks
never regenerate the file. Its SHA-256 is
`a2361ae0062abdaa8b1d393bdfca3981d916c63f68bc95491b3a79529b9f1c1d`.
