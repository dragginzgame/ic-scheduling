use super::*;

#[test]
fn validates_budget_and_delay_bounds() {
    for (attempts, initial, cap) in [(0, 1, 1), (1, 0, 1), (1, 1, 0), (2, 10, 9)] {
        assert_eq!(
            RetryPolicy::new(attempts, initial, cap),
            Err(RetryPolicyError)
        );
    }
    let policy = RetryPolicy::new(4, 3, 10).unwrap();
    assert_eq!(
        (
            policy.max_attempts(),
            policy.initial_delay_ns(),
            policy.max_delay_ns()
        ),
        (4, 3, 10)
    );
    assert_eq!(RetryPolicy::new(1, 1, 1).unwrap(), RetryPolicy::NONE);
}

#[test]
fn counts_the_initial_attempt_and_caps_backoff() {
    let policy = RetryPolicy::new(5, 3, 10).unwrap();
    for (attempt, expected) in [
        (0, None),
        (1, Some(103)),
        (2, Some(106)),
        (3, Some(110)),
        (4, Some(110)),
        (5, None),
        (6, None),
    ] {
        assert_eq!(policy.next_retry(attempt, 100), expected);
    }
    assert_eq!(RetryPolicy::NONE.next_retry(1, 0), None);
    assert_eq!(
        policy.next_retry(1, 500),
        Some(503),
        "anchor at failure completion"
    );
}

#[test]
fn large_counters_and_unsigned_deadlines_never_wrap() {
    let policy = RetryPolicy::new(u32::MAX, 1, u64::MAX).unwrap();
    assert_eq!(policy.next_retry(64, 0), Some(1_u64 << 63));
    assert_eq!(policy.next_retry(65, 0), Some(u64::MAX));
    assert_eq!(policy.next_retry(u32::MAX - 1, 0), Some(u64::MAX));
    assert_eq!(policy.next_retry(u32::MAX, 0), None);
    assert_eq!(policy.next_retry(1, u64::MAX - 1), Some(u64::MAX));
    assert_eq!(policy.next_retry(1, u64::MAX), None);
    let capped = RetryPolicy::new(u32::MAX, u64::MAX / 2 + 1, u64::MAX).unwrap();
    assert_eq!(capped.next_retry(2, 0), Some(u64::MAX));
    assert_eq!(capped.next_retry(2, 1), None);
    let small_cap = RetryPolicy::new(u32::MAX, 3, 10).unwrap();
    assert_eq!(small_cap.next_retry(u32::MAX - 1, 100), Some(110));
}
