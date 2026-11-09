//! Jobs-owned optimizer recipe and evidence names around IC Build's exact capture.
use clap::{Args, Parser};
use ic_build::artifacts::{CapturedArtifact, produce_and_retain_artifact};
use ic_host_process::status::shell_status_code;
use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

fn capture(source: &Path, destination: &Path) -> io::Result<()> {
    let parent = destination
        .parent()
        .ok_or_else(|| io::Error::other("missing evidence parent"))?;
    let limit = usize::try_from(fs::metadata(source)?.len()).map_err(io::Error::other)?;
    CapturedArtifact::capture(source, parent, limit)?
        .retain_to(destination)
        .map(|_| ())
        .map_err(io::Error::other)
}

fn optimize(
    source: &Path,
    destination: &Path,
    optimizer: &OsStr,
    level: &OsStr,
) -> io::Result<ExitCode> {
    let parent = destination
        .parent()
        .ok_or_else(|| io::Error::other("missing evidence parent"))?;
    let producer = parent.join("producer.wasm");
    // Admit the complete optimizer output, as in the original capture path.
    // Build owns exclusive producer/log admission and capture-before-removal.
    let status = produce_and_retain_artifact(
        &producer,
        &parent.join("optimizer.log"),
        destination,
        usize::MAX,
        |producer, log| {
            Command::new(optimizer)
                .arg(source)
                .arg(level)
                .arg("-o")
                .arg(producer)
                .stdout(log.try_clone()?)
                .stderr(log)
                .status()
        },
    )?;
    let code = shell_status_code(status).unwrap_or(1);
    Ok(ExitCode::from(u8::try_from(code).unwrap_or(1)))
}

#[derive(Debug, Parser)]
#[command(
    name = "jobs-capture-artifact",
    about = "Capture qualification Wasm and retain optimizer evidence",
    version
)]
struct Options {
    #[arg(value_name = "input")]
    input: PathBuf,
    #[arg(value_name = "destination")]
    destination: PathBuf,
    #[command(flatten)]
    optimization: Option<Optimization>,
}

#[derive(Args, Debug)]
#[group(requires_all = ["optimizer", "level"], multiple = true)]
struct Optimization {
    #[arg(long = "optimize", value_name = "OPTIMIZER", required = false)]
    optimizer: PathBuf,
    #[arg(value_name = "LEVEL", required = false, allow_hyphen_values = true, value_parser = ["-O3", "-Os", "-Oz"])]
    level: String,
}

fn main() -> io::Result<ExitCode> {
    let args = Options::parse();
    let status = if let Some(optimization) = args.optimization {
        optimize(
            &args.input,
            &args.destination,
            optimization.optimizer.as_os_str(),
            OsStr::new(&optimization.level),
        )?
    } else {
        capture(&args.input, &args.destination)?;
        ExitCode::SUCCESS
    };
    if status == ExitCode::SUCCESS {
        eprintln!("Retained capture: {}", args.destination.display());
    }
    Ok(status)
}

#[cfg(test)]
mod tests;
