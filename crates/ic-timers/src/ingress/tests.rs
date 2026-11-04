use super::*;

#[test]
fn only_the_exact_reserved_method_is_classified() {
    assert!(is_timer_dispatch_method(crate::platform::DISPATCH_METHOD));
    for method in [
        "",
        "dispatch",
        "<ic-timers internal> dispatch ",
        "<ic-timers internal> dispatch_extra",
        "<ic-timers internal> Dispatch",
        "canister_update <ic-timers internal> dispatch",
    ] {
        assert!(!is_timer_dispatch_method(method), "{method:?}");
    }
}
