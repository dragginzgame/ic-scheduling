use ic_timers_pocketic::{Qualification, qualify, run};

fn command() -> clap::Command {
    clap::Command::new("ic-timers-pocketic")
        .about("Build and run timer qualification or exact development subjects")
        .arg(
            clap::Arg::new("worker")
                .long("worker")
                .hide(true)
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand_required(true)
        .subcommand(clap::Command::new("watchdog"))
        .subcommand(
            clap::Command::new("focused")
                .about("Run exact runtime subjects; does not replace watchdog qualification")
                .arg(clap::Arg::new("subjects").required(true).num_args(1..)),
        )
        .subcommand(
            clap::Command::new("cohorts")
                .arg(
                    clap::Arg::new("report")
                        .long("report")
                        .default_value("target/timer-cohorts.json")
                        .value_parser(clap::value_parser!(std::path::PathBuf)),
                )
                .arg(
                    clap::Arg::new("baseline")
                        .long("baseline")
                        .value_parser(clap::value_parser!(std::path::PathBuf)),
                ),
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = command().get_matches();
    let (mode, options) = args.subcommand().expect("required subcommand");
    let qualification = match mode {
        "watchdog" => Qualification::Watchdog,
        "focused" => Qualification::Focused {
            subjects: options
                .get_many::<String>("subjects")
                .expect("required subjects")
                .cloned()
                .collect(),
        },
        "cohorts" => Qualification::Cohorts {
            report: options
                .get_one::<std::path::PathBuf>("report")
                .expect("default report path")
                .clone(),
            baseline: options.get_one::<std::path::PathBuf>("baseline").cloned(),
        },
        _ => unreachable!("Clap admits only declared subcommands"),
    };
    if args.get_flag("worker") {
        run(&qualification)
    } else {
        let mut worker = std::process::Command::new(std::env::current_exe()?);
        worker.args(["--worker", qualification.name()]);
        if let Qualification::Focused { subjects } = &qualification {
            worker.args(subjects);
        }
        if let Qualification::Cohorts { report, baseline } = &qualification {
            worker.arg("--report").arg(report);
            if let Some(baseline) = baseline {
                worker.arg("--baseline").arg(baseline);
            }
        }
        qualify(&mut worker, std::time::Duration::from_secs(30 * 60))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focused_command_requires_subjects_and_preserves_worker_selection() {
        assert!(
            command()
                .try_get_matches_from(["driver", "focused"])
                .is_err()
        );
        let args = command()
            .try_get_matches_from(["driver", "--worker", "focused", "tests::one", "tests::two"])
            .unwrap();
        assert!(args.get_flag("worker"));
        assert_eq!(
            args.subcommand_matches("focused")
                .unwrap()
                .get_many::<String>("subjects")
                .unwrap()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["tests::one", "tests::two"]
        );
    }
}
