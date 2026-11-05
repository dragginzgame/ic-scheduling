use super::*;
use ic_host_process::tool::ExecutionFailure;
use std::process::Stdio;

#[test]
fn subject_discovery_rejects_empty_unrelated_and_failed_producers() {
    let fixture = ic_host_fs::testing::FixtureDirectory::new(
        &crate::build::scratch_parent().unwrap(),
        "timer-discovery",
    )
    .unwrap();
    let watchdog = REQUIRED_WATCHDOG_SUBJECTS
        .iter()
        .map(|name| format!("{name}: test\n"))
        .collect::<String>();
    let missing = watchdog.lines().skip(1).collect::<Vec<_>>().join("\n");
    let legacy = watchdog.replace("tests::watchdog::", "tests::");
    let mixed = format!("{watchdog}report::tests::host_only: test\n");
    let cohort = format!("{COHORT_SUBJECT}: test\n");
    let watchdog_selection = Qualification::Watchdog;
    let cohort_selection = Qualification::Cohorts {
        report: "unused.json".into(),
        baseline: None,
    };
    for (qualification, listing, admitted) in [
        (&watchdog_selection, "", false),
        (
            &watchdog_selection,
            "report::tests::host_only: test\n",
            false,
        ),
        (&watchdog_selection, "tests::recovery: test\n", false),
        (&watchdog_selection, watchdog.as_str(), true),
        (&watchdog_selection, mixed.as_str(), false),
        (&watchdog_selection, missing.as_str(), false),
        (&watchdog_selection, legacy.as_str(), false),
        (&cohort_selection, "", false),
        (&cohort_selection, "cohorts::renamed_subject: test\n", false),
        (&cohort_selection, cohort.as_str(), true),
        (&watchdog_selection, "tests::fake: benchmark\n", false),
    ] {
        let mut command = Command::new("sh");
        command.args([
            "-c",
            "case \"${2:-}\" in --ignored) exit 0;; esac; printf '%s' \"$1\"",
            "fixture",
            listing,
        ]);
        assert_eq!(
            discover_subjects(&mut command, qualification, fixture.path()).is_ok(),
            admitted
        );
    }
    let mut ignored = Command::new("sh");
    ignored.args(["-c", "printf '%s' \"$1\"", "fixture", &cohort]);
    assert!(
        discover_subjects(&mut ignored, &cohort_selection, fixture.path())
            .unwrap_err()
            .to_string()
            .contains("ignored")
    );
    // A valid-looking listing on stdout cannot hide Cargo/compiler failure.
    let mut command = Command::new("sh");
    command.args(["-c", "printf '%s' \"$1\"; exit 7", "fixture", &cohort]);
    let error = discover_subjects(&mut command, &cohort_selection, fixture.path()).unwrap_err();
    let Some(ToolError::Execution(error)) = error.downcast_ref::<ToolError>() else {
        panic!("expected retained producer failure: {error}");
    };
    assert_eq!(error.evidence.status.unwrap().code(), Some(7));
    assert_eq!(error.evidence.stdout, cohort.as_bytes());
}

#[test]
fn focused_discovery_requires_every_exact_nonignored_runtime_subject() {
    let fixture = ic_host_fs::testing::FixtureDirectory::new(
        &crate::build::scratch_parent().unwrap(),
        "timer-focused-discovery",
    )
    .unwrap();
    let selection = Qualification::Focused {
        subjects: vec!["tests::ordinary::one".into(), "tests::watchdog::two".into()],
    };
    for (listing, admitted) in [
        (
            "tests::watchdog::two: test\ntests::ordinary::one: test\n",
            true,
        ),
        ("", false),
        ("tests::ordinary::one: test\n", false),
        (
            "tests::ordinary::one: test\ntests::watchdog::two_extra: test\n",
            false,
        ),
        (
            "tests::ordinary::one: test\ntests::watchdog::two: test\ntests::extra: test\n",
            false,
        ),
    ] {
        let mut producer = Command::new("sh");
        producer.args([
            "-c",
            "case \"${2:-}\" in --ignored) exit 0;; esac; printf '%s' \"$1\"",
            "fixture",
            listing,
        ]);
        assert_eq!(
            discover_subjects(&mut producer, &selection, fixture.path()).is_ok(),
            admitted
        );
    }
    for subjects in [
        vec![],
        vec!["tests::ordinary::one", "tests::ordinary::one"],
        vec!["build::one"],
    ] {
        let mut producer = Command::new("sh");
        producer.args(["-c", "printf 'tests::ordinary::one: test\\n'"]);
        assert!(
            discover_subjects(
                &mut producer,
                &Qualification::Focused {
                    subjects: subjects.into_iter().map(str::to_owned).collect(),
                },
                fixture.path(),
            )
            .is_err()
        );
    }
    let mut ignored = Command::new("sh");
    ignored.args([
        "-c",
        "printf 'tests::ordinary::one: test\\ntests::watchdog::two: test\\n'",
    ]);
    assert!(
        discover_subjects(&mut ignored, &selection, fixture.path())
            .unwrap_err()
            .to_string()
            .contains("ignored")
    );
}

#[test]
fn discovery_failures_save_bounded_bytes_and_keep_the_original_error() {
    use std::os::unix::fs::PermissionsExt;

    let fixture = ic_host_fs::testing::FixtureDirectory::new(
        &crate::build::scratch_parent().unwrap(),
        "timer-discovery-evidence",
    )
    .unwrap();
    for stage in ["discovery", "ignored"] {
        let mut producer = Command::new("sh");
        producer.args([
            "-c",
            "printf 'partial listing'; printf 'compiler error\\377' >&2; exit 7",
        ]);
        let error = test_listing(&mut producer, fixture.path(), stage).unwrap_err();
        let Some(ToolError::Execution(error)) = error.downcast_ref::<ToolError>() else {
            panic!("expected original execution failure: {error}");
        };
        assert_eq!(error.evidence.status.unwrap().code(), Some(7));
        for (suffix, expected) in [
            ("stdout", error.evidence.stdout.as_slice()),
            ("stderr", error.evidence.stderr.as_slice()),
        ] {
            let path = fixture.path().join(format!("{stage}.{suffix}"));
            assert_eq!(std::fs::read(&path).unwrap(), expected);
            assert_eq!(
                std::fs::metadata(path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
        assert_eq!(error.evidence.stderr, b"compiler error\xff");
        let summary =
            std::fs::read_to_string(fixture.path().join(format!("{stage}.status.txt"))).unwrap();
        assert!(summary.contains("ExitStatus"));
        assert!(summary.contains("stderr_truncated: false"));
    }
    let oversized = fixture.path().join("oversized-listing");
    std::fs::write(&oversized, vec![b'x'; 64 * 1024 + 1]).unwrap();
    let mut producer = Command::new("cat");
    producer.arg(&oversized);
    let error = test_listing(&mut producer, fixture.path(), "oversized").unwrap_err();
    let error = error.downcast_ref::<ToolError>().unwrap();
    let evidence = error.evidence().unwrap();
    assert!(evidence.stdout_truncated);
    assert!(evidence.stdout.len() <= 64 * 1024);
    assert_eq!(
        std::fs::read(fixture.path().join("oversized.stdout")).unwrap(),
        evidence.stdout
    );
    assert!(
        std::fs::read_to_string(fixture.path().join("oversized.status.txt"))
            .unwrap()
            .contains("stdout_truncated: true")
    );
    // Persistence errors must never hide the compiler failure or its bytes.
    let blocked = fixture.path().join("not-a-directory");
    std::fs::write(&blocked, "keep me").unwrap();
    let mut producer = Command::new("sh");
    producer.args(["-c", "printf 'original failure' >&2; exit 9"]);
    let error = test_listing(&mut producer, &blocked, "discovery").unwrap_err();
    let Some(ToolError::Execution(error)) = error.downcast_ref::<ToolError>() else {
        panic!("evidence write failure replaced producer failure: {error}");
    };
    assert_eq!(error.evidence.status.unwrap().code(), Some(9));
    assert_eq!(error.evidence.stderr, b"original failure");
    assert_eq!(std::fs::read(&blocked).unwrap(), b"keep me");
}

#[test]
fn qualification_deadline_covers_descendants_and_preserves_failure() {
    let mut command = Command::new("sh");
    command
        .args(["-c", "sleep 60 & wait"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let error = qualify(&mut command, Duration::from_millis(200)).unwrap_err();
    let ToolError::Execution(error) = error else {
        panic!("expected execution failure")
    };
    assert!(matches!(error.failure, ExecutionFailure::TimedOut));
    assert!(
        error.cleanup.is_none(),
        "process group cleanup failed: {error:?}"
    );

    let mut command = Command::new("sh");
    command
        .args(["-c", "printf 'fixture failure' >&2; exit 7"])
        .stderr(Stdio::piped());
    let error = qualify(&mut command, Duration::from_secs(10)).unwrap_err();
    let ToolError::Execution(error) = error else {
        panic!("expected execution failure")
    };
    assert!(matches!(error.failure, ExecutionFailure::ExitStatus));
    assert_eq!(error.evidence.status.unwrap().code(), Some(7));
    assert_eq!(error.evidence.stderr, b"fixture failure");
}

#[test]
fn baseline_output_alias_is_rejected_before_any_build_or_write() {
    let fixture = ic_host_fs::testing::FixtureDirectory::new(
        &crate::build::scratch_parent().unwrap(),
        "timer-baseline-alias",
    )
    .unwrap();
    let baseline = fixture.path().join("baseline.json");
    let hard_link = fixture.path().join("hard-link.json");
    let symlink = fixture.path().join("symlink.json");
    let before = b"reviewed baseline";
    std::fs::write(&baseline, before).unwrap();
    std::fs::hard_link(&baseline, &hard_link).unwrap();
    std::os::unix::fs::symlink(&baseline, &symlink).unwrap();
    for alias in [fixture.path().join("./baseline.json"), hard_link, symlink] {
        let error = run(&Qualification::Cohorts {
            report: alias,
            baseline: Some(baseline.clone()),
        })
        .unwrap_err();
        assert!(
            matches!(
                error.downcast_ref::<ic_host_fs::durable::NonAliasingWriteError>(),
                Some(ic_host_fs::durable::NonAliasingWriteError::Alias { .. })
            ),
            "{error}"
        );
        assert_eq!(std::fs::read(&baseline).unwrap(), before);
    }
}

#[test]
fn scratch_path_admission_refuses_line_breaks_and_accepts_spaces() {
    for path in ["scratch\nchild", "scratch\rchild", "scratch\r\nchild"] {
        assert_eq!(
            validate_scratch_path(Path::new(path)).unwrap_err().kind(),
            std::io::ErrorKind::InvalidInput
        );
    }
    validate_scratch_path(Path::new("scratch with spaces/child")).unwrap();
}
