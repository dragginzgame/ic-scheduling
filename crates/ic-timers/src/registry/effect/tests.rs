use super::*;
use crate::snapshot::TimerIdentity;

#[test]
fn dispatch_derives_both_roles_from_one_claim_and_generation() {
    let claim = RegistrationClaim {
        identity: TimerIdentity::try_new("test", "effects", "dispatch").unwrap(),
        claim_generation: 7,
    };
    let dispatch = WatchdogDispatch::new(&claim, 19);
    let work = dispatch.work();
    assert_eq!(dispatch.successor().claim(), work.claim());
    assert_eq!(dispatch.successor().claim(), &claim);
    assert_eq!(dispatch.successor().callback_generation, 19);
    assert_eq!(work.callback_generation, 19);
    assert_eq!(dispatch.successor().role(), CallbackRole::WatchdogScheduler);
    assert_eq!(work.role(), CallbackRole::WatchdogWork);
    assert_eq!(
        WakeupToken::ordinary(&claim, 19).token().role(),
        CallbackRole::OrdinaryWork
    );
    assert_eq!(
        WakeupToken::scheduler(&claim, 19).token(),
        dispatch.successor()
    );
}
