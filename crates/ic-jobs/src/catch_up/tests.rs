use super::*;
const MINUTE: u64 = 60_000_000_000;

#[test]
fn age_count_and_settled_cursor_bound_the_window() {
    let rule = Schedule::parse("* * * * *").unwrap();
    let plan = rule.catch_up(0, 10 * MINUTE, CatchUpLimit::new(5 * MINUTE, 3).unwrap());
    assert_eq!(plan.occurrences(), &[5 * MINUTE, 6 * MINUTE, 7 * MINUTE]);
    assert_eq!(plan.through(), 10 * MINUTE);
    assert!(plan.has_excess());
    let plan = rule.catch_up(
        8 * MINUTE,
        10 * MINUTE,
        CatchUpLimit::new(u64::MAX, 3).unwrap(),
    );
    assert_eq!(plan.occurrences(), &[9 * MINUTE, 10 * MINUTE]);
    assert!(!plan.has_excess());
    let rewind = rule.catch_up(10 * MINUTE, MINUTE, CatchUpLimit::new(u64::MAX, 3).unwrap());
    assert!(rewind.occurrences().is_empty());
    assert_eq!(rewind.through(), 10 * MINUTE);
}

#[test]
fn zero_age_and_unsigned_boundaries_do_not_wrap() {
    let rule = Schedule::parse("* * * * *").unwrap();
    let limit = CatchUpLimit::new(0, 1).unwrap();
    assert_eq!(rule.catch_up(0, MINUTE, limit).occurrences(), &[MINUTE]);
    assert!(rule.catch_up(0, MINUTE + 1, limit).occurrences().is_empty());
    assert!(rule.catch_up(0, 0, limit).occurrences().is_empty());
    let last = u64::MAX / MINUTE * MINUTE;
    let plan = rule.catch_up(
        last - MINUTE,
        u64::MAX,
        CatchUpLimit::new(u64::MAX, 64).unwrap(),
    );
    assert_eq!(plan.occurrences(), &[last]);
    assert_eq!(plan.through(), u64::MAX);
    assert!(!plan.has_excess());
}

#[test]
fn impossible_and_large_windows_remain_bounded() {
    assert_eq!(CatchUpLimit::new(0, 0), Err(CatchUpLimitError));
    assert_eq!(CatchUpLimit::new(0, 65), Err(CatchUpLimitError));
    let limit = CatchUpLimit::new(u64::MAX, 64).unwrap();
    let plan = Schedule::parse("* * * * *")
        .unwrap()
        .catch_up(0, u64::MAX, limit);
    assert_eq!(plan.occurrences().len(), 64);
    assert_eq!(plan.occurrences().first(), Some(&MINUTE));
    assert_eq!(plan.occurrences().last(), Some(&(64 * MINUTE)));
    assert!(plan.has_excess());
    let plan = Schedule::parse("0 0 31 FEB *")
        .unwrap()
        .catch_up(0, u64::MAX, limit);
    assert!(plan.occurrences().is_empty());
    assert!(!plan.has_excess());
}
