//! Compiler admission cases for the consumer's actual inherited lint policy.

/// A documented safe API must compile under each selected member policy.
#[cfg(feature = "valid")]
pub const fn admitted_value() -> u8 {
    1
}

/// An ignored Result must fail before this consumer can be admitted.
#[cfg(feature = "must-use")]
pub fn ignored_result() {
    Ok::<(), ()>(());
}

/// An unsafe declaration must fail even when its body contains no operations.
///
/// # Safety
/// This deliberately forbidden declaration exists only for compiler rejection.
#[cfg(feature = "unsafe")]
pub unsafe fn forbidden_unsafe() {}

/// A warning-only diagnostic exercises Cargo admission without compiler denial.
#[cfg(feature = "warning")]
pub fn warning_only() {
    let unused_value = 1;
}
