use super::*;

#[test]
fn latest_work_request_replaces_previous_proposals_and_is_consumed_once() {
    CHECKPOINT.with_borrow_mut(|state| state.desired_scheduled = true);
    stop_on_next_work();
    continue_immediately_on_next_work();
    schedule_at_on_next_work((42,));
    assert_eq!(take_work_decision(), WatchdogDecision::ScheduleAt(42));
    assert!(CHECKPOINT.with_borrow(|state| state.desired_scheduled));
    assert_eq!(take_work_decision(), WatchdogDecision::Continue);

    schedule_at_on_next_work((99,));
    continue_immediately_on_next_work();
    assert_eq!(take_work_decision(), WatchdogDecision::ContinueImmediately);
    assert_eq!(take_work_decision(), WatchdogDecision::Continue);

    continue_immediately_on_next_work();
    stop_on_next_work();
    assert_eq!(take_work_decision(), WatchdogDecision::Stop);
    assert!(!CHECKPOINT.with_borrow(|state| state.desired_scheduled));
    assert_eq!(take_work_decision(), WatchdogDecision::Continue);
}
