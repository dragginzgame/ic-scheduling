use super::*;

fn ns(value: &str) -> u64 {
    let date = value.parse::<DateTime<Utc>>().unwrap();
    (i128::from(date.timestamp()) * 1_000_000_000 + i128::from(date.timestamp_subsec_nanos()))
        .try_into()
        .unwrap()
}

#[test]
fn utc_calendar_rules() {
    for (rule, after, expected) in [
        ("0 9 * * *", "2026-03-29T00:00:00Z", "2026-03-29T09:00:00Z"),
        ("0 9 L * *", "2026-02-01T00:00:00Z", "2026-02-28T09:00:00Z"),
        (
            "0 9 * * MON#2",
            "2026-02-01T00:00:00Z",
            "2026-02-09T09:00:00Z",
        ),
        ("0 9 31 * *", "2026-04-01T00:00:00Z", "2026-05-31T09:00:00Z"),
        (
            "0 9 29 FEB *",
            "2099-03-01T00:00:00Z",
            "2104-02-29T09:00:00Z",
        ),
        (
            "*/15 9-17 * * MON-FRI",
            "2026-11-13T17:45:00Z",
            "2026-11-16T09:00:00Z",
        ),
    ] {
        assert_eq!(
            Schedule::parse(rule)
                .unwrap()
                .next_after(ns(after), ns(expected)),
            Some(ns(expected)),
            "{rule}"
        );
    }
}

#[test]
fn rejects_unsupported_layouts_and_invalid_rules() {
    for rule in [
        "* * * *",
        "0 * * * * *",
        "0 0 * * * UTC",
        "@daily",
        "60 * * * *",
        "0 9 * FOO *",
    ] {
        assert!(Schedule::parse(rule).is_err(), "{rule}");
    }
    assert!(Schedule::parse(&" ".repeat(257)).is_err());
}

#[test]
fn exclusive_lower_and_inclusive_upper_bounds() {
    let rule = Schedule::parse("* * * * *").unwrap();
    let at = ns("2026-01-01T00:01:00Z");
    assert_eq!(rule.next_after(at - 1, at), Some(at));
    assert_eq!(rule.next_after(at, at + 59_999_999_999), None);
    assert_eq!(
        rule.next_after(at, at + 60_000_000_000),
        Some(at + 60_000_000_000)
    );
    assert_eq!(rule.next_after(at, at), None);
    assert_eq!(rule.next_after(at, at - 1), None);
    assert_eq!(rule.next_after(0, 60_000_000_000), Some(60_000_000_000));
}

#[test]
fn impossible_rule_and_unsigned_timestamp_limits() {
    assert_eq!(
        Schedule::parse("0 9 31 FEB *")
            .unwrap()
            .next_after(0, u64::MAX),
        None
    );
    let rule = Schedule::parse("* * * * *").unwrap();
    let minute = 60_000_000_000;
    let last = u64::MAX / minute * minute;
    assert_eq!(rule.next_after(last - 1, u64::MAX), Some(last));
    assert_eq!(rule.next_after(last, u64::MAX), None);
    assert_eq!(rule.next_after(u64::MAX, u64::MAX), None);
}

#[test]
fn fixed_rate_keeps_its_anchor_and_checks_unsigned_limits() {
    let schedule = Schedule::fixed_rate(7, NonZeroU64::new(10).unwrap());
    for (after, through, expected) in [
        (0, 6, None),
        (0, 7, Some(7)),
        (7, 16, None),
        (7, 17, Some(17)),
        (15, 27, Some(17)),
        (17, 27, Some(27)),
        (1000, 1010, Some(1007)),
        (1007, 1007, None),
    ] {
        assert_eq!(schedule.next_after(after, through), expected);
    }
    let end = Schedule::fixed_rate(u64::MAX - 10, NonZeroU64::new(10).unwrap());
    assert_eq!(end.next_after(u64::MAX - 11, u64::MAX), Some(u64::MAX - 10));
    assert_eq!(end.next_after(u64::MAX - 10, u64::MAX), Some(u64::MAX));
    assert_eq!(end.next_after(u64::MAX, u64::MAX), None);
    let overflowing = Schedule::fixed_rate(1, NonZeroU64::new(u64::MAX).unwrap());
    assert_eq!(overflowing.next_after(1, u64::MAX), None);
    let unit = Schedule::fixed_rate(0, NonZeroU64::new(1).unwrap());
    assert_eq!(unit.next_after(u64::MAX - 1, u64::MAX), Some(u64::MAX));
}

#[test]
fn fixed_rate_reuses_bounded_catch_up_without_reanchoring() {
    let schedule = Schedule::fixed_rate(7, NonZeroU64::new(10).unwrap());
    let plan = schedule.catch_up(0, 107, crate::CatchUpLimit::new(30, 3).unwrap());
    assert_eq!(plan.occurrences(), &[77, 87, 97]);
    assert!(plan.has_excess());
    assert_eq!(plan.through(), 107);
}
