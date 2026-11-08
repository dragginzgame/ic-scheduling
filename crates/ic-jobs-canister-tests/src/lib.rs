//! Native tooling for Jobs qualification and artifact capture.

/// Explicitly enabled native management transport for recovery tests.
#[cfg(feature = "harness")]
pub mod harness;
