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
mod tests;
