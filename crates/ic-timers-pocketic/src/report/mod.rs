//! Host-only qualification receipts and cohort comparisons; never deployed in probes.

use crate::{
    build::Qualification,
    cohort::{Cohort, Measurements},
};
use ic_build::artifacts::WasmBuildRecord;
use ic_host_artifacts::artifact::{hash_reader, read_reader};
use ic_host_fs::durable::{PublicationMode, WriteOptions, write_bytes};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const SCHEMA: u32 = 2;

// Each field is declared once for construction, strict decoding and comparison.
macro_rules! metrics {
    ($name:ident { $($field:ident),* $(,)? }) => {
        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $name {
            $(pub $field: u128,)*
        }
        impl $name {
            fn deltas(&self, baseline: &Self, subject: &str, changes: &mut BTreeMap<String, i128>) -> Result<()> {
                $(changes.insert(
                    format!("{subject}/{}", stringify!($field)),
                    i128::try_from(self.$field)?.checked_sub(i128::try_from(baseline.$field)?)
                        .ok_or("delta overflow")?,
                );)*
                Ok(())
            }
        }
    };
}

metrics!(PolicyMetrics {
    start_instructions,
    duplicate_instructions,
    cancel_instructions,
    snapshot_instructions,
    inventory_instructions,
    scheduler_instructions,
    work_instructions,
    memory_sampling_empty,
    memory_sampling_sampled,
    memory_sampling_overhead,
    dispatch_cycles,
    wasm_bytes,
});
metrics!(CalibrationMetrics {
    scheduler_instructions,
    work_instructions,
    dispatch_cycles
});

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Context {
    lock_sha256: String,
    server_sha256: String,
    host: String,
    profile: String,
    rustc: String,
    cargo: String,
    target: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Artifact {
    wasm_sha256: String,
    build_fingerprint: String,
    input_digest: String,
    recipe_identity: String,
    feature: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildContext {
    schema: u32,
    context: Context,
    artifacts: BTreeMap<String, Artifact>,
}

/// Only a completed measurement has the on-disk report shape.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    schema: u32,
    context: Context,
    artifacts: BTreeMap<String, Artifact>,
    measurements: Measurements,
}

// A receipt only exists after one whole build identity has been admitted.
// The pending builder is never serialized as a qualification context.
pub(crate) struct BuildContextBuilder {
    environment: BuildEnvironment,
    context: Option<Context>,
    artifacts: BTreeMap<String, Artifact>,
}

struct BuildEnvironment {
    lock_sha256: String,
    server_sha256: String,
    host: String,
    profile: String,
}

impl BuildEnvironment {
    fn read(root: &Path) -> Result<Self> {
        let server = std::env::var_os("POCKET_IC_BIN").ok_or("missing admitted POCKET_IC_BIN")?;
        Ok(Self {
            lock_sha256: digest(&root.join("Cargo.lock"))?,
            server_sha256: digest(Path::new(&server))?,
            host: format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS),
            profile: "timer-probe".into(),
        })
    }
}

impl BuildContextBuilder {
    pub(crate) fn new(root: &Path) -> Result<Self> {
        Ok(Self {
            environment: BuildEnvironment::read(root)?,
            context: None,
            artifacts: BTreeMap::new(),
        })
    }

    pub(crate) fn record_build(
        &mut self,
        label: &str,
        feature: &str,
        record: &WasmBuildRecord,
        artifact: &Path,
    ) -> Result<()> {
        let facts = record.facts();
        let context = Context {
            lock_sha256: self.environment.lock_sha256.clone(),
            server_sha256: self.environment.server_sha256.clone(),
            host: self.environment.host.clone(),
            profile: self.environment.profile.clone(),
            rustc: String::from_utf8(facts.rustc_identity.clone())?,
            cargo: String::from_utf8(facts.cargo_identity.clone())?,
            target: facts.target.clone(),
        };
        let artifact = Artifact {
            wasm_sha256: digest(artifact)?,
            build_fingerprint: record.fingerprint().to_string(),
            input_digest: record.input_digest().to_string(),
            recipe_identity: record.recipe_identity().to_string(),
            feature: feature.into(),
        };
        self.admit_context(context)?;
        self.artifacts.insert(label.into(), artifact);
        Ok(())
    }

    fn admit_context(&mut self, context: Context) -> Result<()> {
        if !context.is_complete() {
            return Err("incomplete qualification build identity".into());
        }
        if self
            .context
            .as_ref()
            .is_some_and(|previous| previous != &context)
        {
            return Err("probe build identity changed during builds".into());
        }
        self.context.get_or_insert(context);
        Ok(())
    }

    pub(crate) fn finish(self) -> Result<BuildContext> {
        let build = BuildContext {
            schema: SCHEMA,
            context: self
                .context
                .ok_or("qualification has no successful build")?,
            artifacts: self.artifacts,
        };
        validate_build_context(build.schema, &build.context, &build.artifacts)?;
        Ok(build)
    }
}

impl Context {
    fn is_complete(&self) -> bool {
        [
            &self.lock_sha256,
            &self.server_sha256,
            &self.host,
            &self.profile,
            &self.rustc,
            &self.cargo,
            &self.target,
        ]
        .iter()
        .all(|value| !value.is_empty())
    }
}

impl BuildContext {
    pub(crate) fn verify_context(&self, root: &Path) -> Result<()> {
        let current = BuildEnvironment::read(root)?;
        if self.context.lock_sha256 != current.lock_sha256
            || self.context.server_sha256 != current.server_sha256
            || self.context.host != current.host
            || self.context.profile != current.profile
        {
            return Err("qualification context changed during measurement".into());
        }
        Ok(())
    }

    /// The driver calls this only after tests, Build and context checks succeed.
    pub(crate) fn publish_runtime_receipt(
        &self,
        path: &Path,
        qualification: &Qualification,
        subjects: &[String],
    ) -> Result<()> {
        validate_build_context(self.schema, &self.context, &self.artifacts)?;
        if matches!(qualification, Qualification::Cohorts { .. })
            || subjects.is_empty()
            || self.artifacts.len() != 1
            || !self.artifacts.contains_key("runtime")
        {
            return Err("incomplete runtime receipt".into());
        }
        write_bytes(
            path,
            WriteOptions {
                mode: PublicationMode::CreateNew,
                permissions: 0o600,
            },
            &serde_json::to_vec_pretty(&serde_json::json!({
                "selection": qualification.name(),
                "subjects": subjects,
                "verification": "passed",
                "build": self,
            }))?,
        )?;
        println!("runtime receipt: {}", path.display());
        Ok(())
    }

    /// Check installed bytes against the admitted build identity.
    ///
    /// # Panics
    /// Panics if the cohort is absent or its Wasm digest differs from the receipt.
    pub fn verify_artifact(&self, cohort: &str, wasm: &[u8]) {
        assert_eq!(
            self.artifacts[cohort].wasm_sha256,
            ic_host_artifacts::artifact::Sha256Digest::compute(wasm).to_string(),
            "measured artifact differs from its build receipt"
        );
    }

    /// Attach measurements to the complete admitted cohort build.
    ///
    /// # Errors
    /// Returns an error for an incomplete identity, artifact set or cohort selection.
    pub fn complete(self, measurements: Measurements) -> Result<Report> {
        let report = Report {
            schema: self.schema,
            context: self.context,
            artifacts: self.artifacts,
            measurements,
        };
        report.validate()?;
        Ok(report)
    }
}

impl Report {
    /// Admit only a complete report from this invocation's child and build context.
    pub(crate) fn read_candidate(path: &Path, expected: &BuildContext) -> Result<Self> {
        let candidate = Self::read(path)?;
        if candidate.schema != expected.schema
            || candidate.context != expected.context
            || candidate.artifacts != expected.artifacts
        {
            return Err("mismatched invocation build context".into());
        }
        candidate.validate()?;
        Ok(candidate)
    }

    fn validate(&self) -> Result<()> {
        validate_build_context(self.schema, &self.context, &self.artifacts)?;
        if self.artifacts.len() != Cohort::ALL.len()
            || !Cohort::ALL.iter().all(|cohort| {
                let name = cohort.name();
                self.artifacts
                    .get(name)
                    .is_some_and(|artifact| artifact.feature == name)
            })
        {
            return Err("incomplete cohort report".into());
        }
        Ok(())
    }

    fn read(path: &Path) -> Result<Self> {
        Ok(serde_json::from_slice(&read_reader(
            fs::File::open(path)?,
            1024 * 1024,
        )?)?)
    }

    pub(crate) fn publish(&self, path: &Path) -> Result<()> {
        self.validate()?;
        write_bytes(
            path,
            WriteOptions {
                mode: PublicationMode::Replace,
                permissions: 0o600,
            },
            &serde_json::to_vec_pretty(self)?,
        )?;
        println!("cohort receipt: {}", path.display());
        Ok(())
    }

    pub(crate) fn compare(&self, baseline: &Path) -> Result<()> {
        let changes = self.deltas(&Self::read(baseline)?)?;
        println!("cohort comparison (review evidence; positive deltas are increases):");
        for (subject, delta) in changes {
            println!("  {subject}: {delta:+}");
        }
        Ok(())
    }

    fn deltas(&self, baseline: &Self) -> Result<BTreeMap<String, i128>> {
        self.validate()?;
        baseline.validate()?;
        if self.context != baseline.context {
            return Err("incomparable cohort compiler/server/host/lock/profile/target; qualify a new baseline explicitly".into());
        }
        if !self.artifacts.iter().all(|(cohort, artifact)| {
            baseline
                .artifacts
                .get(cohort)
                .is_some_and(|previous| previous.recipe_identity == artifact.recipe_identity)
        }) {
            return Err(
                "incomparable cohort build recipes; qualify a new baseline explicitly".into(),
            );
        }
        let current = &self.measurements;
        let previous = &baseline.measurements;
        let mut changes = BTreeMap::new();
        for &cohort in Cohort::ALL {
            current[cohort].deltas(&previous[cohort], cohort.name(), &mut changes)?;
        }
        current.minimal.deltas(
            &previous.minimal,
            "watchdog-calibration-minimal",
            &mut changes,
        )?;
        current.representative.deltas(
            &previous.representative,
            "watchdog-calibration-representative",
            &mut changes,
        )?;
        Ok(changes)
    }
}

fn validate_build_context(
    schema: u32,
    context: &Context,
    artifacts: &BTreeMap<String, Artifact>,
) -> Result<()> {
    if schema != SCHEMA
        || !context.is_complete()
        || artifacts.is_empty()
        || artifacts.values().any(|artifact| {
            [
                &artifact.feature,
                &artifact.wasm_sha256,
                &artifact.build_fingerprint,
                &artifact.input_digest,
                &artifact.recipe_identity,
            ]
            .iter()
            .any(|value| value.is_empty())
        })
    {
        return Err("incomplete qualification build context".into());
    }
    Ok(())
}

pub(crate) fn digest(path: &Path) -> Result<String> {
    Ok(hash_reader(fs::File::open(path)?, 1024 * 1024 * 1024)?
        .sha256
        .to_string())
}

#[cfg(test)]
mod tests;
