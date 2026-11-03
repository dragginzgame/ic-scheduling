use clap::Parser;
use ic_timers_pocketic::{Qualification, qualify, run};

#[derive(Parser)]
#[command(
    name = "ic-timers-pocketic",
    about = "Build and run timer qualification or exact development subjects"
)]
struct Options {
    #[arg(long, hide = true)]
    worker: bool,
    #[command(subcommand)]
    qualification: Qualification,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();
    if options.worker {
        run(&options.qualification)
    } else {
        let mut worker = std::process::Command::new(std::env::current_exe()?);
        options.qualification.configure_worker(&mut worker);
        qualify(&mut worker, std::time::Duration::from_secs(30 * 60))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qualifications_roundtrip_through_supervised_worker_arguments() {
        assert!(Options::try_parse_from(["driver", "focused"]).is_err());
        assert!(Options::try_parse_from(["driver"]).is_err());
        for qualification in [
            Qualification::Watchdog,
            Qualification::Focused {
                subjects: vec!["tests::one".into(), "tests::two".into()],
            },
            Qualification::Cohorts {
                report: "reports/my cohort.json".into(),
                baseline: Some("baseline.json".into()),
            },
        ] {
            let mut worker = std::process::Command::new("driver");
            qualification.configure_worker(&mut worker);
            let options = Options::try_parse_from(
                std::iter::once(worker.get_program()).chain(worker.get_args()),
            )
            .unwrap();
            assert!(options.worker);
            assert_eq!(options.qualification, qualification);
        }
        let options = Options::try_parse_from(["driver", "cohorts"]).unwrap();
        assert_eq!(
            options.qualification,
            Qualification::Cohorts {
                report: "target/timer-cohorts.json".into(),
                baseline: None
            }
        );
    }
}
