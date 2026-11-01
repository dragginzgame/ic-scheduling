use super::*;

fn fixture_report() -> Report {
    Report {
        schema: SCHEMA,
        context: Context {
            lock_sha256: "lock".into(),
            server_sha256: "server".into(),
            host: "host".into(),
            profile: "timer-probe".into(),
            rustc: "rustc".into(),
            cargo: "cargo".into(),
            target: "wasm32-unknown-unknown".into(),
        },
        artifacts: Cohort::ALL
            .iter()
            .map(|cohort| {
                let name = cohort.name();
                (
                    name.into(),
                    Artifact {
                        wasm_sha256: format!("{name} bytes"),
                        build_fingerprint: "build".into(),
                        input_digest: "inputs".into(),
                        recipe_identity: "recipe".into(),
                        feature: name.into(),
                    },
                )
            })
            .collect(),
        measurements: Measurements::default(),
    }
}

#[test]
fn runtime_receipt_retains_build_identity_and_refuses_incomplete_or_existing_output() {
    let fixture =
        ic_host_fs::testing::FixtureDirectory::new(&std::env::temp_dir(), "timer-runtime-receipt")
            .unwrap();
    let report = fixture_report();
    let mut artifact = report.artifacts["watchdog"].clone();
    artifact.feature = "default".into();
    let build = BuildContext {
        schema: report.schema,
        context: report.context,
        artifacts: BTreeMap::from([("runtime".into(), artifact)]),
    };
    let subjects: Vec<String> = vec!["tests::first".into(), "tests::second".into()];
    let qualification = Qualification::Focused {
        subjects: subjects.clone(),
    };
    let output = fixture.path().join("runtime.json");
    for missing in ["compiler", "lock", "artifact"] {
        let mut invalid = build.clone();
        match missing {
            "compiler" => invalid.context.rustc.clear(),
            "lock" => invalid.context.lock_sha256.clear(),
            "artifact" => invalid
                .artifacts
                .get_mut("runtime")
                .unwrap()
                .wasm_sha256
                .clear(),
            _ => unreachable!(),
        }
        assert!(
            invalid
                .publish_runtime_receipt(&output, &qualification, &subjects)
                .is_err()
        );
        assert!(!output.exists());
    }
    let mut missing = build.clone();
    missing.artifacts.clear();
    assert!(
        missing
            .publish_runtime_receipt(&output, &qualification, &subjects)
            .is_err()
    );
    assert!(
        build
            .publish_runtime_receipt(&output, &qualification, &[])
            .is_err()
    );
    assert!(!output.exists());
    build
        .publish_runtime_receipt(&output, &qualification, &subjects)
        .unwrap();
    let bytes = fs::read(&output).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["selection"], "focused");
    assert_eq!(value["subjects"], serde_json::json!(subjects));
    assert_eq!(value["verification"], "passed");
    assert_eq!(value["build"], serde_json::to_value(&build).unwrap());
    assert!(
        build
            .publish_runtime_receipt(&output, &Qualification::Watchdog, &subjects)
            .is_err()
    );
    assert_eq!(fs::read(&output).unwrap(), bytes);
}

#[test]
fn missing_or_incomplete_candidate_never_replaces_a_previous_report() {
    let fixture =
        ic_host_fs::testing::FixtureDirectory::new(&std::env::temp_dir(), "timer-report-admission")
            .unwrap();
    let output = fixture.path().join("previous.json");
    let candidate = fixture.path().join("new-invocation.json");
    let complete = fixture_report();
    let expected = BuildContext {
        schema: complete.schema,
        context: complete.context.clone(),
        artifacts: complete.artifacts.clone(),
    };
    let previous = serde_json::to_vec_pretty(&complete).unwrap();
    let completed = expected
        .clone()
        .complete(complete.measurements.clone())
        .unwrap();
    assert_eq!(serde_json::to_vec_pretty(&completed).unwrap(), previous);
    let mut incomplete = expected.clone();
    incomplete.context.rustc.clear();
    assert!(incomplete.complete(Measurements::default()).is_err());
    fs::write(&output, &previous).unwrap();
    assert!(Report::read_candidate(&candidate, &expected).is_err());
    // Exercise the on-disk boundary, including fields now required by the types.
    for pointer in [
        "/measurements",
        "/measurements/watchdog-calibration-minimal",
        "/measurements/once",
        "/measurements/once/wasm_bytes",
        "/context/rustc",
        "/context/lock_sha256",
        "/artifacts/once",
        "/artifacts/once/wasm_sha256",
        "/artifacts/once/build_fingerprint",
        "/artifacts/once/input_digest",
        "/artifacts/once/recipe_identity",
        "/artifacts/once/feature",
    ] {
        let value: serde_json::Value = serde_json::from_slice(&previous).unwrap();
        for replacement in [
            None,
            Some(serde_json::Value::Null),
            Some(serde_json::json!({})),
        ] {
            let mut changed = value.clone();
            let (parent, key) = pointer.rsplit_once('/').unwrap();
            let object = changed
                .pointer_mut(parent)
                .unwrap()
                .as_object_mut()
                .unwrap();
            if let Some(replacement) = replacement {
                object.insert(key.into(), replacement);
            } else {
                object.remove(key);
            }
            fs::write(&candidate, serde_json::to_vec(&changed).unwrap()).unwrap();
            assert!(
                Report::read_candidate(&candidate, &expected).is_err(),
                "{pointer}"
            );
            assert_eq!(fs::read(&output).unwrap(), previous);
        }
    }
    for pointer in [
        "",
        "/context",
        "/artifacts/once",
        "/measurements",
        "/measurements/once",
    ] {
        let mut changed: serde_json::Value = serde_json::from_slice(&previous).unwrap();
        changed
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unknown".into(), 1.into());
        fs::write(&candidate, serde_json::to_vec(&changed).unwrap()).unwrap();
        assert!(
            Report::read_candidate(&candidate, &expected).is_err(),
            "{pointer}"
        );
    }
    for change in [
        "context",
        "artifact",
        "recipe",
        "schema",
        "legacy-schema",
        "malformed",
    ] {
        let mut changed = complete.clone();
        match change {
            "context" => changed.context.lock_sha256 = "other invocation".into(),
            "artifact" => {
                changed.artifacts.get_mut("once").unwrap().wasm_sha256 = "other bytes".into()
            }
            "recipe" => {
                changed.artifacts.get_mut("once").unwrap().recipe_identity =
                    "other configuration".into()
            }
            "schema" => changed.schema = SCHEMA + 1,
            "legacy-schema" => changed.schema = 1,
            _ => {}
        }
        let bytes = if change == "malformed" {
            b"not JSON".to_vec()
        } else {
            serde_json::to_vec(&changed).unwrap()
        };
        fs::write(&candidate, bytes).unwrap();
        assert!(
            Report::read_candidate(&candidate, &expected).is_err(),
            "{change}"
        );
    }
    fs::write(&candidate, &previous).unwrap();
    let admitted = Report::read_candidate(&candidate, &expected).unwrap();
    // Comparison and publication use the admitted object, even if the child's
    // candidate is no longer available. No second read can select other bytes.
    fs::remove_file(&candidate).unwrap();
    admitted.compare(&output).unwrap();
    admitted.publish(&output).unwrap();
    assert_eq!(fs::read(&output).unwrap(), previous);
}

#[test]
fn report_publication_replaces_the_entry_without_truncating_hard_linked_bytes() {
    let fixture = ic_host_fs::testing::FixtureDirectory::new(
        &std::env::temp_dir(),
        "timer-report-publication",
    )
    .unwrap();
    let baseline = fixture.path().join("baseline.json");
    let output = fixture.path().join("candidate.json");
    let before = b"reviewed baseline";
    fs::write(&baseline, before).unwrap();
    fs::hard_link(&baseline, &output).unwrap();
    let mut report = fixture_report();
    report.context.rustc.clear();
    assert!(report.publish(&output).is_err());
    assert_eq!(fs::read(&output).unwrap(), before);
    report.context.rustc = "rustc".into();
    report.publish(&output).unwrap();
    assert_eq!(fs::read(&baseline).unwrap(), before);
    assert_eq!(
        fs::read(&output).unwrap(),
        serde_json::to_vec_pretty(&report).unwrap()
    );
}

#[test]
fn receipt_round_trip_preserves_costs_and_rejects_incomparable_contexts() {
    let baseline = fixture_report();
    let mut current = baseline.clone();
    current.measurements[Cohort::Once].wasm_bytes = 10;
    assert_eq!(current.deltas(&baseline).unwrap()["once/wasm_bytes"], 10);
    // Source and artifact identities may differ across the sources under review.
    current.artifacts.get_mut("once").unwrap().wasm_sha256 = "new bytes".into();
    current.artifacts.get_mut("once").unwrap().input_digest = "new sources".into();
    current.artifacts.get_mut("once").unwrap().build_fingerprint = "new cache identity".into();
    assert!(current.deltas(&baseline).is_ok());
    let baseline_json = serde_json::to_value(&baseline).unwrap();
    for key in [
        "rustc",
        "cargo",
        "server_sha256",
        "lock_sha256",
        "host",
        "profile",
        "target",
    ] {
        let mut changed = baseline_json.clone();
        changed["context"][key] = "different context".into();
        let changed: Report = serde_json::from_value(changed).unwrap();
        assert!(current.deltas(&changed).is_err(), "{key}");
    }
    for change in [
        "artifact",
        "schema",
        "empty-context",
        "empty-artifact",
        "recipe",
        "empty-recipe",
        "legacy-schema",
        "feature",
    ] {
        let mut changed = baseline.clone();
        match change {
            "artifact" => {
                changed.artifacts.remove("once");
            }
            "recipe" => {
                changed.artifacts.get_mut("once").unwrap().recipe_identity =
                    "other configuration".into()
            }
            "schema" => changed.schema = SCHEMA + 1,
            "legacy-schema" => changed.schema = 1,
            "empty-context" => changed.context.rustc.clear(),
            "empty-artifact" => changed
                .artifacts
                .get_mut("once")
                .unwrap()
                .input_digest
                .clear(),
            "empty-recipe" => changed
                .artifacts
                .get_mut("once")
                .unwrap()
                .recipe_identity
                .clear(),
            "feature" => changed.artifacts.get_mut("once").unwrap().feature = "watchdog".into(),
            _ => unreachable!(),
        }
        assert!(current.deltas(&changed).is_err(), "{change}");
    }
    let policy = serde_json::to_string(&current.measurements[Cohort::Once]).unwrap();
    let duplicate = serde_json::to_string(&current.measurements)
        .unwrap()
        .replacen("\"once\":", &format!("\"once\":{policy},\"once\":"), 1);
    assert!(serde_json::from_str::<Measurements>(&duplicate).is_err());
    current.measurements[Cohort::Once].dispatch_cycles = u128::MAX;
    let decoded: Report = serde_json::from_slice(&serde_json::to_vec(&current).unwrap()).unwrap();
    assert_eq!(
        decoded.measurements[Cohort::Once].dispatch_cycles,
        u128::MAX
    );
    assert!(
        decoded.deltas(&baseline).is_err(),
        "unrepresentable signed delta must reject"
    );
}

#[test]
fn build_identity_is_admitted_whole_and_mismatches_preserve_the_first_build() {
    let report = fixture_report();
    let expected = report.context;
    let mut builder = BuildContextBuilder {
        environment: BuildEnvironment {
            lock_sha256: expected.lock_sha256.clone(),
            server_sha256: expected.server_sha256.clone(),
            host: expected.host.clone(),
            profile: expected.profile.clone(),
        },
        context: None,
        artifacts: report.artifacts,
    };
    for key in ["rustc", "cargo", "target"] {
        let mut invalid = serde_json::to_value(&expected).unwrap();
        invalid[key] = "".into();
        assert!(
            builder
                .admit_context(serde_json::from_value(invalid).unwrap())
                .is_err()
        );
        assert!(builder.context.is_none());
    }
    builder.admit_context(expected.clone()).unwrap();
    for key in ["rustc", "cargo", "target"] {
        let mut changed = serde_json::to_value(&expected).unwrap();
        changed[key] = "changed".into();
        assert!(
            builder
                .admit_context(serde_json::from_value(changed).unwrap())
                .is_err()
        );
        assert_eq!(builder.context.as_ref(), Some(&expected));
    }
    builder.admit_context(expected.clone()).unwrap();
    assert_eq!(builder.finish().unwrap().context, expected);
}
