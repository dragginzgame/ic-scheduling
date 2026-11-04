use super::*;
#[cfg(unix)]
use ic_host_fs::testing::FixtureDirectory;
#[cfg(unix)]
use ic_host_process::testing::write_executable_script;

#[test]
fn cli_preserves_capture_arguments_and_requires_complete_optimizer_selection() {
    let args = Options::try_parse_from(["capture", "input with spaces", "out.wasm"]).unwrap();
    assert_eq!(args.input, Path::new("input with spaces"));
    assert!(args.optimization.is_none());
    for level in ["-O3", "-Os", "-Oz"] {
        let args = Options::try_parse_from([
            "capture",
            "input",
            "output",
            "--optimize",
            "optimizer tool",
            level,
        ])
        .unwrap();
        assert_eq!(args.optimization.as_ref().unwrap().level, level);
        assert_eq!(
            args.optimization.as_ref().unwrap().optimizer,
            Path::new("optimizer tool")
        );
    }
    for args in [
        vec!["capture", "input"],
        vec!["capture", "input", "output", "-Oz"],
        vec!["capture", "input", "output", "--optimize", "optimizer"],
        vec![
            "capture",
            "input",
            "output",
            "--optimize",
            "optimizer",
            "--all-features",
        ],
    ] {
        assert!(Options::try_parse_from(args).is_err());
    }
    assert_eq!(
        Options::try_parse_from(["capture", "--help"])
            .unwrap_err()
            .kind(),
        clap::error::ErrorKind::DisplayHelp
    );
}

// IC Build owns generic capture/retention proofs. Keep Jobs optimizer
// ordering here and the original-capture command path in consumer tooling.
#[cfg(unix)]
fn optimizer_fixture() -> (FixtureDirectory, PathBuf, PathBuf) {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "jobs-capture").unwrap();
    let root = fixture.path();
    let source = root.join("frozen input.wasm");
    fs::write(&source, b"exact frozen bytes\n").unwrap();
    let optimizer = root.join("optimizer tool");
    write_executable_script(&optimizer, b"#!/bin/sh\n[ \"$3\" = -o ] || exit 49\ncat \"$1\" > \"$4\"\nprintf '%s\\n' \"$2\" >> \"$4\"\necho optimizer-out\necho optimizer-err >&2\n").unwrap();
    (fixture, source, optimizer)
}

#[cfg(unix)]
#[test]
fn optimized_variants_capture_exact_flags_then_retire_only_the_producer() {
    let (fixture, source, optimizer) = optimizer_fixture();
    let root = fixture.path();
    for level in ["-O3", "-Os", "-Oz"] {
        let directory = root.join(level);
        fs::create_dir(&directory).unwrap();
        let destination = directory.join("consumer.wasm");
        assert_eq!(
            optimize(
                &source,
                &destination,
                optimizer.as_os_str(),
                OsStr::new(level)
            )
            .unwrap(),
            ExitCode::SUCCESS
        );
        assert_eq!(
            fs::read(&destination).unwrap(),
            format!("exact frozen bytes\n{level}\n").as_bytes()
        );
        assert!(!directory.join("producer.wasm").exists());
        let log = fs::read_to_string(directory.join("optimizer.log")).unwrap();
        assert!(log.contains("optimizer-out") && log.contains("optimizer-err"));
        assert_eq!(fs::read_dir(&directory).unwrap().count(), 2);
    }
    assert_eq!(fs::read(&source).unwrap(), b"exact frozen bytes\n");
}

#[cfg(unix)]
#[test]
fn failed_optimization_and_failed_capture_retain_producer_and_diagnostics() {
    let (fixture, source, optimizer) = optimizer_fixture();
    let root = fixture.path();
    let failed = root.join("failed");
    fs::create_dir(&failed).unwrap();
    write_executable_script(
        &optimizer,
        b"#!/bin/sh\necho partial > \"$4\"\necho failed >&2\nexit 37\n",
    )
    .unwrap();
    assert_eq!(
        optimize(
            &source,
            &failed.join("consumer.wasm"),
            optimizer.as_os_str(),
            OsStr::new("-O3")
        )
        .unwrap(),
        ExitCode::from(37)
    );
    assert_eq!(
        fs::read(failed.join("producer.wasm")).unwrap(),
        b"partial\n"
    );
    assert_eq!(fs::read(failed.join("optimizer.log")).unwrap(), b"failed\n");
    assert!(!failed.join("consumer.wasm").exists());

    let signaled = root.join("signaled");
    fs::create_dir(&signaled).unwrap();
    write_executable_script(
        &optimizer,
        b"#!/bin/sh\necho partial > \"$4\"\necho terminated >&2\nkill -TERM $$\n",
    )
    .unwrap();
    assert_eq!(
        optimize(
            &source,
            &signaled.join("consumer.wasm"),
            optimizer.as_os_str(),
            OsStr::new("-O3")
        )
        .unwrap(),
        ExitCode::from(143)
    );
    assert_eq!(
        fs::read(signaled.join("producer.wasm")).unwrap(),
        b"partial\n"
    );
    assert_eq!(
        fs::read(signaled.join("optimizer.log")).unwrap(),
        b"terminated\n"
    );
    assert!(!signaled.join("consumer.wasm").exists());

    write_executable_script(&optimizer, b"#!/bin/sh\ncat \"$1\" > \"$4\"\n").unwrap();
    let refused = root.join("refused");
    fs::create_dir(&refused).unwrap();
    let destination = refused.join("occupied");
    fs::create_dir(&destination).unwrap();
    let error = optimize(
        &source,
        &destination,
        optimizer.as_os_str(),
        OsStr::new("-O3"),
    )
    .unwrap_err();
    let retained = error
        .get_ref()
        .unwrap()
        .downcast_ref::<ic_build::artifacts::CaptureRetentionError>()
        .unwrap();
    assert_eq!(
        fs::read(refused.join("producer.wasm")).unwrap(),
        b"exact frozen bytes\n"
    );
    assert_eq!(fs::read(&source).unwrap(), b"exact frozen bytes\n");
    assert!(refused.join("optimizer.log").is_file());
    assert_eq!(fs::read_dir(destination).unwrap().count(), 0);
    // Failure preserves its diagnostic inputs. Remove them explicitly only
    // after that proof, then require the retained copy to own the exact bytes.
    fs::remove_file(refused.join("producer.wasm")).unwrap();
    fs::remove_file(source).unwrap();
    assert_eq!(
        fs::read(&retained.retained_path).unwrap(),
        b"exact frozen bytes\n"
    );
}

#[cfg(unix)]
#[test]
fn invalid_flags_and_existing_evidence_refuse_before_optimizer_dispatch() {
    let (fixture, source, optimizer) = optimizer_fixture();
    let root = fixture.path();
    let directory = root.join("invalid");
    fs::create_dir(&directory).unwrap();
    let destination = directory.join("consumer.wasm");
    assert!(
        Options::try_parse_from([
            OsStr::new("capture"),
            source.as_os_str(),
            destination.as_os_str(),
            OsStr::new("--optimize"),
            optimizer.as_os_str(),
            OsStr::new("--all-features"),
        ])
        .is_err()
    );
    assert_eq!(fs::read_dir(&directory).unwrap().count(), 0);
    fs::write(directory.join("producer.wasm"), b"existing evidence").unwrap();
    assert!(
        optimize(
            &source,
            &destination,
            optimizer.as_os_str(),
            OsStr::new("-O3")
        )
        .is_err()
    );
    assert_eq!(
        fs::read(directory.join("producer.wasm")).unwrap(),
        b"existing evidence"
    );
    assert!(!directory.join("optimizer.log").exists());
}
