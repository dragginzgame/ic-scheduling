use ic_jobs::{Job, JobRecord};

mod schedules;
pub(crate) use schedules::recurring_schedules;

pub(crate) fn roundtrip(job: &Job) -> Job {
    let bytes = serde_json::to_vec(job).unwrap();
    let record: JobRecord = serde_json::from_slice(&bytes).unwrap();
    Job::restore(record).unwrap()
}
