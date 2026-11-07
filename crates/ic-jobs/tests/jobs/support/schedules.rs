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
