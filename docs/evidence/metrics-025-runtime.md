# Metrics 0.25 Runtime companion

This source companion to the pending Runtime adoption selects published Metrics
0.25.0 for Timers' existing producer-owned measurement counters. It changes no
timer implementation, arithmetic, Serde representation or package version.
Native consumers composing those counters with Wire 0.14 metadata need the same
Metrics supplier; copying or converting the exported counters is unnecessary.

Focused Linux qualification on 2026-11-01 uses a library-only workspace beneath
`target/metrics-025-proof`. It copies `crates/ic-timers` verbatim and projects the
root catalog into a workspace whose only member is that library. Published
Runtime/guard 0.17.1, Types 0.16.0 and Metrics 0.25.0 resolve without a patch or
replacement. Rust 1.91 passes all 175 library tests and Wasm compilation. Strict
development-compiler Clippy passes for the library and its tests.

This is source/library qualification, not acceptance of Scheduling's entire
workspace, installed timer recovery, a publication or native macOS. The parent
Runtime adoption still owns its complete registry lock and coordinated
Testkit/Wire Endpoint qualification. That pending lock is preserved here;
dependent consumers resolve the committed Timers source under their own locked
integration graph. The companion should be reviewed with the parent adoption
before any Scheduling release or publication.
