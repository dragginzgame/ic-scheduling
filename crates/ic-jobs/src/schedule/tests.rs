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
    for rule in ["* * * *", "0 * * * * *", "0 0 * * * UTC", "@daily"] {
        assert!(
            matches!(Schedule::parse(rule), Err(ScheduleError::InvalidLayout)),
            "{rule}"
        );
    }
    for rule in ["60 * * * *", "0 9 * FOO *"] {
        assert!(
            matches!(Schedule::parse(rule), Err(ScheduleError::Parse(_))),
            "{rule}"
        );
    }
    let too_long = format!("* * * *{}*", " ".repeat(250));
    assert!(matches!(
        Schedule::parse(&too_long),
        Err(ScheduleError::InvalidLayout)
    ));
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

#[test]
fn inclusive_zero_and_unsigned_end_catch_up() {
    let schedule = Schedule::fixed_rate(0, NonZeroU64::new(10).unwrap());
    let plan = schedule.catch_up_from(None, 30, crate::CatchUpLimit::new(30, 3).unwrap());
    assert_eq!(plan.occurrences(), &[0, 10, 20]);
    assert!(plan.has_excess());
    assert_eq!(
        schedule
            .catch_up(0, 30, crate::CatchUpLimit::new(30, 3).unwrap())
            .occurrences(),
        &[10, 20, 30]
    );
    assert_eq!(schedule.next_from(0, 0), Some(0));
    let cron = Schedule::parse("* * * * *").unwrap();
    assert_eq!(cron.next_from(0, 0), Some(0));
    assert_eq!(cron.next_from(1, 60_000_000_000), Some(60_000_000_000));
    let end = Schedule::fixed_rate(u64::MAX, NonZeroU64::new(1).unwrap());
    let plan = end.catch_up(
        u64::MAX - 1,
        u64::MAX,
        crate::CatchUpLimit::new(0, 3).unwrap(),
    );
    assert_eq!(plan.occurrences(), &[u64::MAX]);
    assert!(!plan.has_excess());
}

#[test]
fn one_shot_windows_and_catch_up_exhaust_after_one_occurrence() {
    for at in [0, 1, 17, u64::MAX] {
        let schedule = Schedule::once(at);
        assert_eq!(schedule.next_from(at, at), Some(at));
        assert_eq!(schedule.next_after(at, u64::MAX), None);
        if at > 0 {
            assert_eq!(schedule.next_after(at - 1, at), Some(at));
            assert_eq!(schedule.next_from(0, at - 1), None);
        }
        let plan = schedule.catch_up_from(
            None,
            u64::MAX,
            crate::CatchUpLimit::new(u64::MAX, 64).unwrap(),
        );
        assert_eq!(plan.occurrences(), &[at]);
        assert!(!plan.has_excess());
        assert!(
            schedule
                .catch_up(
                    at,
                    u64::MAX,
                    crate::CatchUpLimit::new(u64::MAX, 64).unwrap()
                )
                .occurrences()
                .is_empty()
        );
    }
    assert!(
        Schedule::once(10)
            .catch_up_from(None, 20, crate::CatchUpLimit::new(9, 1).unwrap())
            .occurrences()
            .is_empty()
    );
}

#[test]
fn completion_relative_deadlines_use_settlement_not_the_original_anchor() {
    let schedule = Schedule::after_completion(7, NonZeroU64::new(10).unwrap());
    assert_eq!(schedule.after_settlement(None), Some(Some(7)));
    assert_eq!(schedule.after_settlement(Some(5)), Some(Some(7)));
    assert_eq!(schedule.after_settlement(Some(7)), Some(Some(17)));
    assert_eq!(schedule.after_settlement(Some(123)), Some(Some(133)));
    assert_eq!(
        schedule.after_settlement(Some(u64::MAX - 10)),
        Some(Some(u64::MAX))
    );
    assert_eq!(schedule.after_settlement(Some(u64::MAX - 9)), Some(None));
    assert_eq!(schedule.next_after(0, 7), Some(7));
    assert_eq!(
        schedule.next_after(7, u64::MAX),
        None,
        "preview cannot invent completion times"
    );
    assert_eq!(Schedule::once(7).after_settlement(None), None);
}
