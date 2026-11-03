//! Native tooling for Jobs qualification and artifact capture.

/// Explicitly enabled native management transport for recovery tests and the demo.
#[cfg(feature = "harness")]
pub mod harness;
