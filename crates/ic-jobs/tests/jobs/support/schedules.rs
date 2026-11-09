use std::num::NonZeroU64;

use ic_jobs::{MissedRunPolicy, Schedule};

pub(crate) fn recurring_schedules(first_at_ns: u64, every_ns: u64) -> [Schedule; 3] {
    let every_ns = NonZeroU64::new(every_ns).unwrap();
    [
        Schedule::AfterCompletion {
            first_at_ns,
            every_ns,
        },
        Schedule::FixedRate {
            first_at_ns,
            every_ns,
            missed: MissedRunPolicy::Skip,
        },
        Schedule::FixedRate {
            first_at_ns,
            every_ns,
            missed: MissedRunPolicy::CatchUp,
        },
    ]
}

// Recurring policies whose successor advances past completion rather than
// retaining an overdue CatchUp occurrence. Keep construction in one roster.
pub(crate) fn non_catchup_schedules(first_at_ns: u64, every_ns: u64) -> [Schedule; 2] {
    let [after_completion, skip, _catch_up] = recurring_schedules(first_at_ns, every_ns);
    [after_completion, skip]
}
