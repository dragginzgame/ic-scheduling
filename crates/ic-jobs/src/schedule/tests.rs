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
