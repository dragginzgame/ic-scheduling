use super::*;
use ic_timers_test_protocol::rpc::WorkDecision;

#[test]
fn latest_work_request_replaces_previous_proposals_and_is_consumed_once() {
    CHECKPOINT.with_borrow_mut(|state| state.desired_scheduled = true);
    set_work_decision((WorkDecision::Stop,));
    set_work_decision((WorkDecision::ContinueImmediately,));
    set_work_decision((WorkDecision::ScheduleAt(42),));
    assert_eq!(take_work_decision(), WatchdogDecision::ScheduleAt(42));
    assert!(CHECKPOINT.with_borrow(|state| state.desired_scheduled));
    assert_eq!(take_work_decision(), WatchdogDecision::Continue);

    set_work_decision((WorkDecision::ScheduleAt(99),));
    set_work_decision((WorkDecision::ContinueImmediately,));
    assert_eq!(take_work_decision(), WatchdogDecision::ContinueImmediately);
    assert_eq!(take_work_decision(), WatchdogDecision::Continue);

    set_work_decision((WorkDecision::ContinueImmediately,));
    set_work_decision((WorkDecision::Stop,));
    assert_eq!(take_work_decision(), WatchdogDecision::Stop);
    assert!(!CHECKPOINT.with_borrow(|state| state.desired_scheduled));
    assert_eq!(take_work_decision(), WatchdogDecision::Continue);
}
