use super::*;

#[test]
fn cadence_is_positive_bounded_and_checked() {
    assert_eq!(
        TimerCadence::new(Duration::ZERO),
        Err(ScheduleError::ZeroCadence)
    );
    assert_eq!(
        TimerCadence::new(Duration::from_secs(u64::MAX)),
        Err(ScheduleError::DelayOutOfRange)
    );

    let cadence =
        TimerCadence::new(Duration::from_nanos(5)).expect("positive cadence should be valid");
    assert_eq!(cadence.as_nanos(), 5);
    assert_eq!(cadence.deadline_after(10), Ok(15));
    assert_eq!(
        cadence.deadline_after(u64::MAX),
        Err(ScheduleError::DeadlineOverflow)
    );
}

#[test]
fn schedules_and_directives_resolve_without_truncation() {
    assert_eq!(
        TimerSchedule::After(Duration::from_nanos(5)).resolve(10),
        Ok(ResolvedSchedule {
            deadline_ns: 15,
            requested_delay_ns: Some(5),
            mode: TimerSchedulingMode::Once,
        })
    );
    assert_eq!(
        TimerSchedule::At(7).resolve(10),
        Ok(ResolvedSchedule {
            deadline_ns: 7,
            requested_delay_ns: None,
            mode: TimerSchedulingMode::Deadline,
        })
    );
    assert_eq!(
        OrdinaryDirective::Stop.resolve(10, None),
        Ok((None, TimerDirectiveSnapshot::Stop))
    );
    assert_eq!(
        OrdinaryDirective::ContinueImmediately.resolve(10, None),
        Ok((
            Some(ResolvedSchedule {
                deadline_ns: 10,
                requested_delay_ns: Some(0),
                mode: TimerSchedulingMode::Continuation,
            }),
            TimerDirectiveSnapshot::ContinueImmediately
        ))
    );
    for (now_ns, delay_ns, deadline_ns) in [
        (10, 5, 15),
        (u64::MAX, 0, u64::MAX),
        (0, u64::MAX, u64::MAX),
    ] {
        assert_eq!(
            OrdinaryDirective::RetryAfter(Duration::from_nanos(delay_ns)).resolve(now_ns, None),
            Ok((
                Some(ResolvedSchedule {
                    deadline_ns,
                    requested_delay_ns: Some(delay_ns),
                    mode: TimerSchedulingMode::Retry,
                }),
                TimerDirectiveSnapshot::RetryAfter { delay_ns }
            ))
        );
    }
    assert_eq!(
        OrdinaryDirective::ScheduleAt(7).resolve(10, None),
        Ok((
            Some(ResolvedSchedule {
                deadline_ns: 7,
                requested_delay_ns: None,
                mode: TimerSchedulingMode::Deadline,
            }),
            TimerDirectiveSnapshot::ScheduleAt { deadline_ns: 7 }
        ))
    );

    let cadence = TimerCadence::from_nanos(9).expect("fixture cadence should be valid");
    assert_eq!(
        OrdinaryDirective::RecurAfterCompletion.resolve(10, Some(cadence)),
        Ok((
            Some(ResolvedSchedule {
                deadline_ns: 19,
                requested_delay_ns: Some(9),
                mode: TimerSchedulingMode::AfterCompletion,
            }),
            TimerDirectiveSnapshot::RecurAfterCompletion
        ))
    );
    assert_eq!(
        OrdinaryDirective::RecurAfterCompletion.resolve(10, None),
        Err(TimerControlFailure::DirectiveNotAllowed)
    );
    assert_eq!(
        OrdinaryDirective::RetryAfter(Duration::from_nanos(1)).resolve(u64::MAX, None),
        Err(TimerControlFailure::DeadlineOverflow)
    );
    assert_eq!(
        OrdinaryDirective::RetryAfter(Duration::MAX).resolve(0, None),
        Err(TimerControlFailure::DelayOutOfRange)
    );
    assert_eq!(
        OrdinaryDirective::RecurAfterCompletion.resolve(u64::MAX, Some(cadence)),
        Err(TimerControlFailure::DeadlineOverflow)
    );
}
