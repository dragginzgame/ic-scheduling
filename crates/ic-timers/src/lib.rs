//! Retained timers for isolated Internet Computer work.
//!
//! The private Once core and retained bindings are implemented and tested.
//! Public initialization, IC transport and executor integration remain unwired.

// Remove this temporary expectation when the authenticated transport uses the
// core. Keeping it private avoids exposing an unqualified dispatch API.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "Once core awaits transport integration")
)]
mod engine;

#[cfg_attr(
    not(test),
    expect(dead_code, reason = "retained bindings await authenticated transport")
)]
mod bindings;
