# ic-timers

Isolated async Once timers and synchronous prearmed watchdogs for Internet
Computer canisters. The canister API is available on `wasm32`; this crate owns
the global timer and must not be combined with another timer provider.

Wasm builds accept compatible Platform Runtime 0.23 patches from 0.23.14.
A host may select a supported patch without requiring Timers to name that exact
patch. Cargo must still resolve one Runtime and its matching executor guard.

Call `initialize`, bind a callback with an explicit `Lifetime`, and update demand
through its returned capability. Timestamps are unsigned Unix nanoseconds.
The provider admits 64 declarations, eight outstanding transport flights and
four submissions per control turn. Pressure defers delivery; a due time does
not guarantee punctual execution. Callbacks execute in isolated messages. Newer owner commands take precedence
over callback results. Abandoned Once work remains blocked until explicit owner
reconciliation; a timeout never authorizes replay of business effects.

`Lifetime::Retained` keeps the declaration until explicit removal.
`Lifetime::RemoveWhenStopped` releases it after a committed stop and definitive
settlement. Dropping a handle does not cancel work. Removal refuses running or
uncertain Once work and running/settling watchdog tickets.

The optional `metrics` feature adds aggregate counters and bounded sorted
declaration inventory using ic-metrics arithmetic. Diagnostics carry no control
authority. No calendar dependencies are included.

Hosts own durable intent, effect reconciliation and fresh-runtime reconstruction
on upgrade. Heap snapshots and exactly-once remote effects are not provided.
This is a breaking redesign of the 0.44 API; it has no compatibility facade.

See the [repository](https://github.com/dragginzgame/ic-scheduling) for design,
grouped changelogs and local installed-canister qualification evidence.
