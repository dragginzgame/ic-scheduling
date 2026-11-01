//! Consumer recipes; IC Build owns Cargo execution and exact Wasm artifacts.

use ic_build::artifacts::{CacheRetentionLimits, WasmBuildEvidence, WasmBuildSpec};
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
pub(crate) enum Qualification {
    Watchdog,
    Focused {
        subjects: Vec<String>,
    },
    Cohorts {
        report: PathBuf,
        baseline: Option<PathBuf>,
    },
}

impl Qualification {
    pub(crate) const fn name(&self) -> &'static str {
        match self {
            Self::Watchdog => "watchdog",
            Self::Focused { .. } => "focused",
            Self::Cohorts { .. } => "cohorts",
        }
    }
}

/// Policy cohorts admitted by both the build driver and measurement/report consumers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Cohort {
    Baseline,
    Once,
    AfterCompletion,
    Watchdog,
}

impl Cohort {
    pub(crate) const ALL: [Self; 4] = [
        Self::Baseline,
        Self::Once,
        Self::AfterCompletion,
        Self::Watchdog,
    ];

    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::Once => "once",
            Self::AfterCompletion => "after-completion",
            Self::Watchdog => "watchdog",
        }
    }

    pub(crate) const fn environment(self) -> &'static str {
        match self {
            Self::Baseline => "IC_TIMERS_COHORT_BASELINE_WASM",
            Self::Once => "IC_TIMERS_COHORT_ONCE_WASM",
            Self::AfterCompletion => "IC_TIMERS_COHORT_AFTER_COMPLETION_WASM",
            Self::Watchdog => "IC_TIMERS_COHORT_WATCHDOG_WASM",
        }
    }
}

const COHORT_SUBJECT: &str =
    "cohorts::comparable_policy_cohorts_report_size_and_instruction_subjects";

// These subjects anchor the maintained recovery contract, not incidental test counts.
const REQUIRED_WATCHDOG_SUBJECTS: &[&str] = &[
    "tests::absent_ordinary_demand_is_idle_and_explicit_cancellation_stays_distinct",
    "tests::scheduler_abandonment_under_round_pressure_recovers_after_stop",
    "tests::host_inspection_refuses_timer_ingress_but_foreign_calls_still_authenticate",
    "tests::competing_global_timer_writes_trap_and_roll_back_timer_control",
    "tests::trapped_work_keeps_committed_successor_and_executor_is_private",
    "tests::instruction_exhaustion_leaves_successor_for_later_progress",
    "tests::insufficient_cycles_defer_due_work_until_top_up",
    "tests::frozen_canister_resumes_overdue_timer_after_top_up_only",
    "tests::freeze_between_scheduler_and_work_resumes_after_top_up_only",
    "tests::once_frozen_await_failure_retires_without_replay",
    "tests::after_completion_frozen_await_failure_retires_without_replay",
    "tests::dispatch_response_trap_cleanup_preserves_pending_work_for_retry",
    "tests::ordinary_trap_before_await_retires_the_rolled_back_scheduled_delivery",
    "tests::ordinary_continuation_trap_retires_running_work_and_releases_expired_authority",
    "tests::accepted_ordinary_work_survives_dispatch_timeout_without_replay",
    "tests::shared_watchdog_survives_trap_and_upgrade",
    "tests::maximum_capacity_dispatch_preserves_every_survivor_under_pressure_and_await",
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
        Qualification::Watchdog => REQUIRED_WATCHDOG_SUBJECTS
            .iter()
            .all(|name| names.contains(name)),
        Qualification::Cohorts { .. } => names == [COHORT_SUBJECT],
        Qualification::Focused { subjects } => {
            !subjects.is_empty()
                && names.len() == subjects.len()
                && subjects.iter().all(|subject| {
                    subject.starts_with("tests::")
                        && names.iter().filter(|name| **name == subject).count() == 1
                })
                && subjects
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
                    == subjects.len()
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

pub(crate) fn run(qualification: &Qualification) -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
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
        &std::env::temp_dir().canonicalize()?,
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
        "--bin",
        "ic-timers-pocketic",
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
    let mut report = crate::report::BuildContext::new(&root)?;
    let recipes = if !matches!(qualification, Qualification::Cohorts { .. }) {
        vec![(
            "runtime",
            "IC_TIMERS_PROBE_WASM",
            "ic-timers-runtime-probe",
            None,
        )]
    } else {
        Cohort::ALL
            .into_iter()
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
        .with_isolated_builds()
        .with_prune_policy(
            CacheRetentionLimits::new()
                .with_max_age(Duration::from_secs(7 * 24 * 60 * 60))
                .with_max_size_bytes(512 * 1024 * 1024),
        )
        .with_cargo_program("cargo")
        .with_rustc_identity_program("rustc")
        .with_cargo_selection(selection);
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

// One parent process group covers builds, Cargo tests and synchronous instance
// deletion. Separately grouped Testkit servers also have their own hard TTL.
pub(crate) fn qualify(command: &mut Command, timeout: Duration) -> Result<(), ToolError> {
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
