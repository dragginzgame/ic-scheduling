//! A fresh process has no installed timer runtime.
#![cfg(feature = "execution")]

use ic_jobs::{
    Attempt, Checkpoint, ControlState, Error, Outcome, Policy, RetryPolicy, Schedule,
    UnresolvedPhase, bind, restore, restore_unresolved,
};

#[test]
fn uninitialized_cron_refuses_before_reading_ic_time() {
    let result = bind(
        ["tests", "cron", "uninitialized"],
        Schedule::parse("* * * * *").unwrap(),
        Policy::SkipLate { lateness_ns: 0 },
        RetryPolicy::NONE,
        |_| async { Outcome::NoWork },
    );

    assert!(matches!(
        result,
        Err(Error::Timer(ic_timers::Error::Uninitialized))
    ));
}

#[test]
fn restoration_validates_checkpoint_before_timer_admission() {
    let identity = ["tests", "once", "restore"];
    let policy = Policy::SkipLate { lateness_ns: 0 };
    let work = |_| async { Outcome::NoWork };
    let checkpoint =
        Checkpoint::new(7, 1, None, Some(10), ControlState::Active, None, None).unwrap();
    assert!(matches!(
        restore(
            identity,
            Schedule::once(11),
            policy,
            RetryPolicy::NONE,
            checkpoint.clone(),
            work,
        ),
        Err(Error::InvalidCheckpoint)
    ));
    assert!(matches!(
        restore(
            identity,
            Schedule::once(10),
            policy,
            RetryPolicy::NONE,
            checkpoint,
            work,
        ),
        Err(Error::Timer(ic_timers::Error::Uninitialized))
    ));

    let unresolved = Checkpoint::new(7, 1, None, None, ControlState::Paused, None, None)
        .unwrap()
        .with_unresolved(Attempt::new(1, 10, 1).unwrap(), UnresolvedPhase::Uncertain)
        .unwrap();
    assert!(matches!(
        restore_unresolved(
            identity,
            Schedule::once(10),
            policy,
            RetryPolicy::NONE,
            unresolved,
            work,
        ),
        Err(Error::Timer(ic_timers::Error::Uninitialized))
    ));
}
