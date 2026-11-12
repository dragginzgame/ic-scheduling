//! Composition with the canister's existing ingress inspection policy.

/// Whether an update method is reserved for the timer runtime's self-dispatch.
///
/// In the host's **existing** `canister_inspect_message` hook, refuse this method
/// before accepting any ingress, then apply the host's usual admission policy to
/// other methods. This crate deliberately exports no inspection hook. This
/// predicate neither accepts messages nor grants timer control authority.
///
/// Inspection is non-replicated and does not run for inter-canister calls. The
/// dispatch endpoint independently requires the canister itself as caller.
/// Inspection can avoid replicated ingress execution cost, but cannot prevent
/// charged foreign inter-canister requests or replace endpoint authentication.
#[must_use]
pub fn is_timer_dispatch_method(method: &str) -> bool {
    method == crate::platform::DISPATCH_METHOD
}

#[cfg(test)]
mod tests;
