//! Consumer recipes; IC Build owns Cargo execution and exact Wasm artifacts.

use ic_build::artifacts::{
    CacheRetentionLimits, WasmBuildEvidence, WasmBuildSpec, prune_wasm_build_cache,
};
use ic_build::cargo::{
    environment::ConfigurationLimits,
    selection::{CargoFeatureSelection, CargoSelection},
};
use ic_host_fs::durable::{PublicationMode, WriteOptions, write_bytes};
use ic_host_process::{
    child::CleanupPolicy,
    tool::{OutputLimit, OutputLimits, ToolError, capture_command, run_group_command},
};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

// Qualification admits small Cargo configuration trees. Build owns bounded
// reads and include traversal; Timers owns these positive resource budgets.
const CONFIGURATION_LIMITS: ConfigurationLimits = ConfigurationLimits {
    file_bytes: 1024 * 1024,
    total_bytes: 16 * 1024 * 1024,
    files: 128,
    depth: 32,
};

/// Full qualification or an explicitly narrower development selection.
#[derive(clap::Subcommand, Debug, Eq, PartialEq)]
pub enum Qualification {
    /// Run the complete installed watchdog and runtime subject roster.
    Watchdog,
    /// Run exact runtime subjects; does not replace watchdog qualification.
    Focused {
        /// Exact test paths selected by the caller.
        #[arg(required = true, num_args = 1..)]
        subjects: Vec<String>,
    },
    /// Measure every policy cohort and both watchdog calibration subjects.
    Cohorts {
        /// Output receipt path, relative to the qualification workspace.
        #[arg(long, default_value = "target/timer-cohorts.json")]
        report: PathBuf,
        /// Optional admitted report used for a measurement comparison.
        #[arg(long)]
        baseline: Option<PathBuf>,
    },
}

impl Qualification {
    /// Re-enter the same parsed qualification in the supervised worker.
    pub fn configure_worker(&self, worker: &mut Command) {
        worker.args(["--worker", self.name()]);
        match self {
            Self::Watchdog => {}
            Self::Focused { subjects } => {
                worker.args(subjects);
            }
            Self::Cohorts { report, baseline } => {
                worker.arg("--report").arg(report);
                if let Some(baseline) = baseline {
                    worker.arg("--baseline").arg(baseline);
                }
            }
        }
    }

    /// Command name used when re-entering the supervised worker.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Watchdog => "watchdog",
            Self::Focused { .. } => "focused",
            Self::Cohorts { .. } => "cohorts",
        }
    }
}

use crate::cohort::Cohort;

const COHORT_SUBJECT: &str =
    "cohorts::comparable_policy_cohorts_report_size_and_instruction_subjects";

// These subjects anchor the maintained recovery contract, not incidental test counts.
const REQUIRED_WATCHDOG_SUBJECTS: &[&str] = &[
    "tests::lifecycle::runtime_composition_preserves_calls_cleanup_and_lifecycle",
    "tests::lifecycle::cached_baseline_restoration_preserves_timer_ownership_after_mutation",
    "tests::ordinary::absent_ordinary_demand_is_idle_and_explicit_cancellation_stays_distinct",
    "tests::watchdog::scheduler_abandonment_under_round_pressure_recovers_after_stop",
    "tests::ingress::host_inspection_refuses_timer_ingress_but_foreign_calls_still_authenticate",
    "tests::ingress::competing_global_timer_writes_trap_and_roll_back_timer_control",
    "tests::watchdog::trapped_work_keeps_committed_successor_and_executor_is_private",
    "tests::watchdog::instruction_exhaustion_leaves_successor_for_later_progress",
    "tests::watchdog::insufficient_cycles_defer_due_work_until_top_up",
    "tests::watchdog::frozen_canister_resumes_overdue_timer_after_top_up_only",
    "tests::watchdog::freeze_between_scheduler_and_work_resumes_after_top_up_only",
    "tests::ordinary::once_frozen_await_failure_retires_without_replay",
    "tests::ordinary::after_completion_frozen_await_failure_retires_without_replay",
    "tests::ordinary::dispatch_response_trap_cleanup_preserves_pending_work_for_retry",
    "tests::ordinary::ordinary_trap_before_await_retires_the_rolled_back_scheduled_delivery",
    "tests::ordinary::ordinary_continuation_trap_retires_running_work_and_releases_expired_authority",
    "tests::ordinary::accepted_ordinary_work_survives_dispatch_timeout_without_replay",
    "tests::lifecycle::shared_watchdog_survives_trap_and_upgrade",
    "tests::scheduling::maximum_capacity_dispatch_preserves_every_survivor_under_pressure_and_await",
];

fn test_listing(
    command: &mut Command,
    diagnostics: &Path,
    stage: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // The owning worker supplies the elapsed deadline and process group. Host
    // bounds discovery output and preserves a failed producer's status/evidence.
    let result = capture_command(
        command,
        OutputLimits {
            stdout: OutputLimit::Terminate(64 * 1024),
            stderr: OutputLimit::Terminate(256 * 1024),
            timeout: None,
        },
    )
    .and_then(|output| output.require_success());
    let (stdout, stderr) = match &result {
        Ok(output) => (output.stdout.as_slice(), output.stderr.as_slice()),
        Err(error) => error.evidence().map_or((&[][..], &[][..]), |evidence| {
            (evidence.stdout.as_slice(), evidence.stderr.as_slice())
        }),
    };
    let summary = format!("{result:?}\n");
    let retained = (|| {
        for (suffix, bytes) in [
            ("stdout", stdout),
            ("stderr", stderr),
            ("status.txt", summary.as_bytes()),
        ] {
            write_bytes(
                &diagnostics.join(format!("{stage}.{suffix}")),
                WriteOptions {
                    mode: PublicationMode::Replace,
                    permissions: 0o600,
                },
                bytes,
            )?;
        }
        Ok::<_, Box<dyn std::error::Error>>(())
    })();
    if result.is_err() && retained.is_ok() {
        eprintln!(
            "discovery failed; stderr: {}",
            diagnostics.join(format!("{stage}.stderr")).display()
        );
    }
    if let Err(error) = retained {
        eprintln!("could not save {stage} evidence: {error}");
        // A secondary persistence failure must not replace the typed producer
        // failure (including its original bounded output and cleanup outcome).
        if result.is_ok() {
            return Err(error);
        }
    }
    Ok(result?.stdout)
}

fn discover_subjects(
    command: &mut Command,
    qualification: &Qualification,
    diagnostics: &Path,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = test_listing(command, diagnostics, "discovery")?;
    let names = std::str::from_utf8(&output)?
        .lines()
        .map(|line| line.strip_suffix(": test").ok_or("invalid test listing"))
        .collect::<Result<Vec<_>, _>>()?;
    let admitted = match qualification {
        Qualification::Watchdog => {
            names.iter().all(|name| name.starts_with("tests::"))
                && REQUIRED_WATCHDOG_SUBJECTS
                    .iter()
                    .all(|name| names.contains(name))
        }
        Qualification::Cohorts { .. } => names == [COHORT_SUBJECT],
        Qualification::Focused { subjects } => {
            let selected: BTreeSet<_> = subjects.iter().map(String::as_str).collect();
            let discovered: BTreeSet<_> = names.iter().copied().collect();
            !selected.is_empty()
                && selected.len() == subjects.len()
                && discovered.len() == names.len()
                && selected
                    .iter()
                    .all(|subject| subject.starts_with("tests::"))
                && selected == discovered
        }
    };
    if !admitted {
        return Err(format!(
            "missing {} qualification subjects in successful test listing",
            qualification.name()
        )
        .into());
    }
    // Listing includes ignored tests. Refuse ignored selected subjects rather than
    // silently qualifying a partial run or overriding an intentional ignore.
    let ignored = test_listing(command.arg("--ignored"), diagnostics, "ignored")?;
    if !ignored.is_empty() {
        return Err("qualification selection contains ignored subjects".into());
    }
    Ok(names.into_iter().map(str::to_owned).collect())
}

fn validate_scratch_path(path: &Path) -> std::io::Result<()> {
    if path
        .as_os_str()
        .as_encoded_bytes()
        .iter()
        .any(|byte| matches!(byte, b'\r' | b'\n'))
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "scratch directory contains CR/LF",
        ));
    }
    Ok(())
}

// Keep default qualification/test scratch inside this workspace's clean boundary.
// An explicit caller TMPDIR remains authoritative and is never changed globally.
pub(crate) fn scratch_parent() -> std::io::Result<PathBuf> {
    let configured = std::env::var_os("TMPDIR");
    let parent = configured.as_ref().map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/tmp"),
        PathBuf::from,
    );
    validate_scratch_path(&parent)?;
    if configured.is_none() {
        std::fs::create_dir_all(&parent)?;
    }
    let parent = parent.canonicalize()?;
    validate_scratch_path(&parent)?;
    Ok(parent)
}

/// Build the selected artifacts, run admitted subjects and publish their receipt.
pub fn run(qualification: &Qualification) -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .canonicalize()?;
    let (report_path, baseline) = match qualification {
        Qualification::Watchdog | Qualification::Focused { .. } => (None, None),
        Qualification::Cohorts { report, baseline } => (
            Some(root.join(report)),
            baseline.as_ref().map(|path| root.join(path)),
        ),
    };
    if let (Some(output), Some(baseline)) = (&report_path, &baseline) {
        let baseline = baseline.canonicalize()?;
        ic_host_fs::durable::validate_non_aliasing(output, &[&baseline], 40)?;
    }
    // Retain invocation evidence on success and failure without consulting a
    // previous final report. Host owns unique private directory allocation.
    let scratch = ic_host_fs::directory::create_unique_directory(
        &scratch_parent()?,
        "timer-qualification",
        0o700,
        std::num::NonZeroUsize::new(128),
    )?;
    eprintln!("qualification diagnostics: {}", scratch.display());
    let candidate = scratch.join("cohorts.json");
    let mut tests = Command::new("cargo");
    tests.current_dir(&root).args([
        "test",
        "-p",
        "ic-timers-pocketic",
        "--test",
        "runtime",
        "--locked",
        "--",
    ]);
    match qualification {
        Qualification::Watchdog => {
            tests.arg("tests::");
        }
        Qualification::Focused { subjects } => {
            tests.arg("--exact").args(subjects);
        }
        Qualification::Cohorts { .. } => {
            tests.args(["--exact", COHORT_SUBJECT]);
        }
    }
    let mut discovery = Command::new(tests.get_program());
    discovery
        .current_dir(&root)
        .args(tests.get_args())
        .args(["--list", "--format", "terse"]);
    tests.arg("--nocapture");
    let subjects = discover_subjects(&mut discovery, qualification, &scratch)?;
    std::fs::write(
        scratch.join("subjects.txt"),
        format!("{}\n", subjects.join("\n")),
    )?;
    let mut builds = Vec::new();
    let mut report = crate::report::BuildContextBuilder::new(&root)?;
    let recipes = if !matches!(qualification, Qualification::Cohorts { .. }) {
        vec![(
            "runtime",
            "IC_TIMERS_PROBE_WASM",
            "ic-timers-runtime-probe",
            None,
        )]
    } else {
        Cohort::ALL
            .iter()
            .copied()
            .map(|cohort| {
                (
                    cohort.name(),
                    cohort.environment(),
                    "ic-timers-size-probe",
                    Some(cohort.name()),
                )
            })
            .collect()
    };
    for (label, variable, package, feature) in recipes {
        let target = root.join("target/timers").join(format!("probe-{label}"));
        let selection = CargoSelection {
            profile: Some("timer-probe".to_owned()),
            locked: true,
            features: CargoFeatureSelection {
                features: feature.map(str::to_owned).into_iter().collect(),
                default_features: feature.is_none(),
                ..CargoFeatureSelection::default()
            },
            ..CargoSelection::default()
        };
        let spec = WasmBuildSpec::new(
            &root,
            &target,
            &[package],
            std::env::vars_os(),
            CONFIGURATION_LIMITS,
        )
        .with_shared_incremental_target(root.join("target/timers").join(format!("cargo-{label}")))
        .with_cargo_program("cargo")
        .with_rustc_identity_program("rustc")
        .with_cargo_selection(selection);
        // Build 0.83 leaves cache maintenance with the caller. Preserve the
        // existing exact-cache limits at this qualification-build boundary.
        prune_wasm_build_cache(
            &target,
            CacheRetentionLimits::new()
                .with_max_age(Duration::from_secs(7 * 24 * 60 * 60))
                .with_max_size_bytes(512 * 1024 * 1024),
        )?;
        let built =
            WasmBuildEvidence::acquire(spec, ic_host_process::child::OwnedChild::spawn_direct)?;
        let [artifact] = built.record().artifacts() else {
            return Err(format!("expected exactly one Wasm artifact for {label}").into());
        };
        tests.env(variable, artifact);
        report.record_build(
            label,
            feature.unwrap_or("default"),
            built.record(),
            artifact,
        )?;
        // Retain exact artifacts until the child tests finish reading them.
        builds.push(built);
    }
    let report = report.finish()?;
    if report_path.is_some() {
        tests.env("IC_TIMERS_COHORT_REPORT", &candidate);
        tests.env("IC_TIMERS_COHORT_CONTEXT", serde_json::to_string(&report)?);
    }
    tests.env("IC_TIMERS_DIAGNOSTICS", &scratch);
    let status = tests.status()?;
    for built in &builds {
        built.verify_current()?;
    }
    report.verify_context(&root)?;
    drop(builds);
    if !status.success() {
        return Err(format!(
            "{} PocketIC subjects failed: {status}",
            qualification.name()
        )
        .into());
    }
    if let Some(path) = &report_path {
        let completed = crate::report::Report::read_candidate(&candidate, &report)?;
        if let Some(baseline) = &baseline {
            completed.compare(baseline)?;
        }
        completed.publish(path)?;
    } else {
        report.publish_runtime_receipt(&scratch.join("runtime.json"), qualification, &subjects)?;
    }
    Ok(())
}

/// Supervise builds, Cargo tests and synchronous instance deletion in one group.
/// Separately grouped Testkit servers also have their own hard TTL.
pub fn qualify(command: &mut Command, timeout: Duration) -> Result<(), ToolError> {
    run_group_command(
        command,
        OutputLimits {
            stdout: OutputLimit::Truncate(16 * 1024),
            stderr: OutputLimit::Truncate(16 * 1024),
            timeout: Some(timeout),
        },
        CleanupPolicy::TermThenKill {
            grace: Duration::from_secs(2),
            reap_timeout: Duration::from_secs(5),
        },
        None,
        || false,
    )?
    .require_success()?;
    Ok(())
}

#[cfg(all(test, unix))]
mod tests;
