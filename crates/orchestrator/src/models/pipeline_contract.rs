//! Public contract models for orchestration pipeline manifests.
//!
//! This module keeps the pipeline JSON contract in a small, typed boundary so
//! later stage execution migration can reuse one canonical manifest model.

use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;

/// Repository-relative path to the canonical orchestration pipeline manifest.
pub const DEFAULT_PIPELINE_MANIFEST_PATH: &str =
    ".codex/orchestration/pipelines/default.pipeline.json";

/// Return the repository-relative path to the canonical orchestration pipeline manifest.
#[must_use]
pub const fn default_pipeline_manifest_path() -> &'static str {
    DEFAULT_PIPELINE_MANIFEST_PATH
}

/// Parse and validate an orchestration pipeline manifest from JSON text.
pub fn parse_pipeline_manifest(json: &str) -> Result<PipelineManifest, PipelineContractError> {
    let manifest = serde_json::from_str::<PipelineManifest>(json)
        .map_err(PipelineContractError::ParseManifest)?;
    manifest.validate()?;
    Ok(manifest)
}

/// Load and validate an orchestration pipeline manifest from disk.
pub fn load_pipeline_manifest(
    path: impl AsRef<Path>,
) -> Result<PipelineManifest, PipelineContractError> {
    let path = path.as_ref();
    let json = fs::read_to_string(path).map_err(|source| PipelineContractError::ReadManifest {
        path: path.display().to_string(),
        source,
    })?;
    parse_pipeline_manifest(&json)
}

/// Load the repository's canonical default orchestration pipeline manifest.
pub fn load_default_pipeline_manifest(
    repo_root: impl AsRef<Path>,
) -> Result<PipelineManifest, PipelineContractError> {
    load_pipeline_manifest(repo_root.as_ref().join(DEFAULT_PIPELINE_MANIFEST_PATH))
}

/// Typed representation of `.codex/orchestration/pipelines/*.pipeline.json`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PipelineManifest {
    /// Stable pipeline identifier.
    pub id: String,
    /// Incrementing manifest version.
    pub version: u32,
    /// Human-readable summary of the pipeline.
    pub description: String,
    /// Optional runtime defaults applied to the whole pipeline.
    #[serde(default)]
    pub runtime: Option<PipelineRuntime>,
    /// Ordered stage definitions.
    pub stages: Vec<PipelineStage>,
    /// Required handoffs between stages.
    pub handoffs: Vec<PipelineHandoff>,
    /// Required stages and artifacts for a successful run.
    pub completion_criteria: PipelineCompletionCriteria,
}

impl PipelineManifest {
    /// Find a stage by identifier.
    #[must_use]
    pub fn stage(&self, stage_id: &str) -> Option<&PipelineStage> {
        self.stages.iter().find(|stage| stage.id == stage_id)
    }

    /// Preserve the declared stage order from the manifest.
    #[must_use]
    pub fn stage_ids(&self) -> Vec<&str> {
        self.stages.iter().map(|stage| stage.id.as_str()).collect()
    }

    /// Return every artifact produced by pipeline stages.
    #[must_use]
    pub fn produced_artifacts(&self) -> BTreeSet<String> {
        self.stages
            .iter()
            .flat_map(|stage| stage.output_artifacts.iter().cloned())
            .collect()
    }

    /// Return every artifact referenced by stage inputs or outputs.
    #[must_use]
    pub fn referenced_artifacts(&self) -> BTreeSet<String> {
        self.stages
            .iter()
            .flat_map(|stage| {
                stage
                    .input_artifacts
                    .iter()
                    .chain(stage.output_artifacts.iter())
                    .cloned()
            })
            .collect()
    }

    fn validate(&self) -> Result<(), PipelineContractError> {
        validate_unique_values(
            "pipeline stage ids",
            self.stages.iter().map(|stage| stage.id.as_str()),
        )?;

        let stage_positions: BTreeMap<&str, usize> = self
            .stages
            .iter()
            .enumerate()
            .map(|(index, stage)| (stage.id.as_str(), index))
            .collect();

        for stage in &self.stages {
            validate_unique_values(
                &format!("stage `{}` inputArtifacts", stage.id),
                stage.input_artifacts.iter().map(String::as_str),
            )?;
            validate_unique_values(
                &format!("stage `{}` outputArtifacts", stage.id),
                stage.output_artifacts.iter().map(String::as_str),
            )?;
        }

        let produced_artifacts = self.produced_artifacts();
        validate_unique_values(
            "pipeline completion requiredStages",
            self.completion_criteria
                .required_stages
                .iter()
                .map(String::as_str),
        )?;
        validate_unique_values(
            "pipeline completion requiredArtifacts",
            self.completion_criteria
                .required_artifacts
                .iter()
                .map(String::as_str),
        )?;

        for stage_id in &self.completion_criteria.required_stages {
            if !stage_positions.contains_key(stage_id.as_str()) {
                return Err(PipelineContractError::UnknownStageReference {
                    scope: "completionCriteria.requiredStages".to_string(),
                    stage_id: stage_id.clone(),
                });
            }
        }

        for artifact in &self.completion_criteria.required_artifacts {
            if !produced_artifacts.contains(artifact) {
                return Err(PipelineContractError::UnknownArtifactReference {
                    scope: "completionCriteria.requiredArtifacts".to_string(),
                    artifact: artifact.clone(),
                });
            }
        }

        let mut handoff_pairs = BTreeSet::new();
        for handoff in &self.handoffs {
            let pair = (handoff.from_stage.as_str(), handoff.to_stage.as_str());
            if !handoff_pairs.insert(pair) {
                return Err(PipelineContractError::DuplicateValue {
                    scope: "pipeline handoff pairs".to_string(),
                    value: format!("{} -> {}", handoff.from_stage, handoff.to_stage),
                });
            }
            validate_unique_values(
                &format!(
                    "handoff `{} -> {}` requiredArtifacts",
                    handoff.from_stage, handoff.to_stage
                ),
                handoff.required_artifacts.iter().map(String::as_str),
            )?;

            let from_stage = self.stage(&handoff.from_stage).ok_or_else(|| {
                PipelineContractError::UnknownStageReference {
                    scope: format!(
                        "handoff `{} -> {}` fromStage",
                        handoff.from_stage, handoff.to_stage
                    ),
                    stage_id: handoff.from_stage.clone(),
                }
            })?;
            let to_stage = self.stage(&handoff.to_stage).ok_or_else(|| {
                PipelineContractError::UnknownStageReference {
                    scope: format!(
                        "handoff `{} -> {}` toStage",
                        handoff.from_stage, handoff.to_stage
                    ),
                    stage_id: handoff.to_stage.clone(),
                }
            })?;

            let from_index = stage_positions[handoff.from_stage.as_str()];
            let to_index = stage_positions[handoff.to_stage.as_str()];
            if from_index >= to_index {
                return Err(PipelineContractError::InvalidHandoffOrder {
                    from_stage: handoff.from_stage.clone(),
                    to_stage: handoff.to_stage.clone(),
                });
            }

            for artifact in &handoff.required_artifacts {
                if !from_stage.output_artifacts.contains(artifact) {
                    return Err(PipelineContractError::InvalidHandoffArtifact {
                        from_stage: handoff.from_stage.clone(),
                        to_stage: handoff.to_stage.clone(),
                        artifact: artifact.clone(),
                        reason: HandoffArtifactReason::NotProducedByFromStage,
                    });
                }
                if !to_stage.input_artifacts.contains(artifact) {
                    return Err(PipelineContractError::InvalidHandoffArtifact {
                        from_stage: handoff.from_stage.clone(),
                        to_stage: handoff.to_stage.clone(),
                        artifact: artifact.clone(),
                        reason: HandoffArtifactReason::NotConsumedByToStage,
                    });
                }
            }
        }

        Ok(())
    }
}

/// Optional runtime defaults for a pipeline manifest.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PipelineRuntime {
    /// Preferred execution backend for the pipeline.
    pub execution_backend: Option<PipelineExecutionBackend>,
    /// Default delay before retrying failed stages.
    pub default_retry_delay_seconds: Option<u32>,
    /// Maximum total pipeline duration.
    pub max_pipeline_duration_seconds: Option<u32>,
    /// Whether stage failures may be tolerated.
    pub continue_on_stage_failure: Option<bool>,
    /// Whether stage run state should be persisted.
    pub write_run_state: Option<bool>,
    /// Repository-relative policy catalog path.
    pub policy_catalog_path: Option<String>,
    /// Repository-relative model routing catalog path.
    pub model_routing_catalog_path: Option<String>,
}

/// Declared stage in the orchestration pipeline.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PipelineStage {
    /// Stable stage identifier.
    pub id: String,
    /// Agent identifier assigned to the stage.
    pub agent_id: String,
    /// Stage behavior category.
    pub mode: PipelineStageMode,
    /// How this stage is executed.
    pub execution: PipelineExecutionSpec,
    /// Artifacts required before the stage can run.
    pub input_artifacts: Vec<String>,
    /// Artifacts produced by the stage.
    pub output_artifacts: Vec<String>,
    /// Failure behavior for this stage.
    pub on_failure: PipelineFailurePolicy,
}

/// Execution settings for a pipeline stage.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PipelineExecutionSpec {
    /// Repository-relative stage entrypoint script.
    pub script_path: String,
    /// Optional stage-specific dispatch mode override.
    #[serde(default)]
    pub dispatch_mode: Option<PipelineDispatchMode>,
    /// Optional repository-relative prompt template path.
    #[serde(default)]
    pub prompt_template_path: Option<String>,
    /// Optional repository-relative response schema path.
    #[serde(default)]
    pub response_schema_path: Option<String>,
    /// Optional stage timeout budget in seconds.
    #[serde(default)]
    pub timeout_seconds: Option<u32>,
}

impl PipelineExecutionSpec {
    /// Resolve the effective dispatch mode for the stage.
    #[must_use]
    pub fn effective_dispatch_mode(&self) -> PipelineDispatchMode {
        self.dispatch_mode.unwrap_or(PipelineDispatchMode::Scripted)
    }
}

/// Contract for artifact handoff between stages.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PipelineHandoff {
    /// Upstream stage identifier.
    pub from_stage: String,
    /// Downstream stage identifier.
    pub to_stage: String,
    /// Artifacts required for the handoff.
    pub required_artifacts: Vec<String>,
}

/// Completion constraints for a pipeline run.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PipelineCompletionCriteria {
    /// Stages that must complete for the run to be considered successful.
    pub required_stages: Vec<String>,
    /// Artifacts that must exist for the run to be considered successful.
    pub required_artifacts: Vec<String>,
}

/// Stage execution mode from the manifest contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PipelineStageMode {
    /// Intake, normalize, or plan work.
    Plan,
    /// Perform implementation or task execution.
    Execute,
    /// Run validations or test gates.
    Validate,
    /// Review, closeout, or sign off work.
    Review,
}

impl PipelineStageMode {
    /// Return the schema string for the stage mode.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Plan => "plan",
            Self::Execute => "execute",
            Self::Validate => "validate",
            Self::Review => "review",
        }
    }
}

/// Dispatch backend for an individual stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PipelineDispatchMode {
    /// Invoke a scripted stage directly.
    Scripted,
    /// Dispatch the stage through Codex exec.
    CodexExec,
}

impl PipelineDispatchMode {
    /// Return the schema string for the dispatch mode.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scripted => "scripted",
            Self::CodexExec => "codex-exec",
        }
    }
}

/// Runtime execution backend for the whole pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PipelineExecutionBackend {
    /// Use scripts as the execution backend.
    ScriptOnly,
    /// Use Codex exec as the execution backend.
    CodexExec,
}

/// Failure policy for a stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PipelineFailurePolicy {
    /// Stop the pipeline immediately.
    Stop,
    /// Retry the stage once before failing.
    RetryOnce,
    /// Continue even when the stage fails.
    Continue,
}

/// Validation and IO errors for the pipeline manifest contract.
#[derive(Debug)]
pub enum PipelineContractError {
    /// The manifest file could not be read from disk.
    ReadManifest {
        /// Path that failed to load.
        path: String,
        /// Underlying IO failure.
        source: std::io::Error,
    },
    /// The manifest JSON could not be parsed.
    ParseManifest(serde_json::Error),
    /// A list contains a duplicate value where uniqueness is required.
    DuplicateValue {
        /// Logical scope that contains the duplicate.
        scope: String,
        /// Offending duplicated value.
        value: String,
    },
    /// A stage reference points to a stage that is not declared.
    UnknownStageReference {
        /// Logical scope that contains the unknown reference.
        scope: String,
        /// Stage identifier that could not be resolved.
        stage_id: String,
    },
    /// An artifact reference points to an artifact that is not declared.
    UnknownArtifactReference {
        /// Logical scope that contains the unknown reference.
        scope: String,
        /// Artifact identifier that could not be resolved.
        artifact: String,
    },
    /// A handoff points backwards or to the same stage.
    InvalidHandoffOrder {
        /// Upstream stage identifier.
        from_stage: String,
        /// Downstream stage identifier.
        to_stage: String,
    },
    /// A handoff requires an artifact that violates producer/consumer rules.
    InvalidHandoffArtifact {
        /// Upstream stage identifier.
        from_stage: String,
        /// Downstream stage identifier.
        to_stage: String,
        /// Artifact identifier with the invalid contract.
        artifact: String,
        /// Exact reason the artifact contract is invalid.
        reason: HandoffArtifactReason,
    },
}

impl Display for PipelineContractError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadManifest { path, source } => {
                write!(f, "failed to read pipeline manifest `{path}`: {source}")
            }
            Self::ParseManifest(source) => write!(f, "failed to parse pipeline manifest JSON: {source}"),
            Self::DuplicateValue { scope, value } => {
                write!(f, "duplicate value `{value}` found in {scope}")
            }
            Self::UnknownStageReference { scope, stage_id } => {
                write!(f, "unknown stage `{stage_id}` referenced in {scope}")
            }
            Self::UnknownArtifactReference { scope, artifact } => {
                write!(f, "unknown artifact `{artifact}` referenced in {scope}")
            }
            Self::InvalidHandoffOrder { from_stage, to_stage } => {
                write!(f, "handoff `{from_stage} -> {to_stage}` must point to a later stage")
            }
            Self::InvalidHandoffArtifact {
                from_stage,
                to_stage,
                artifact,
                reason,
            } => write!(
                f,
                "handoff `{from_stage} -> {to_stage}` references invalid artifact `{artifact}`: {reason}"
            ),
        }
    }
}

impl std::error::Error for PipelineContractError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReadManifest { source, .. } => Some(source),
            Self::ParseManifest(source) => Some(source),
            Self::DuplicateValue { .. }
            | Self::UnknownStageReference { .. }
            | Self::UnknownArtifactReference { .. }
            | Self::InvalidHandoffOrder { .. }
            | Self::InvalidHandoffArtifact { .. } => None,
        }
    }
}

/// Reason why a handoff artifact reference is invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffArtifactReason {
    /// The upstream stage does not list the artifact in its outputs.
    NotProducedByFromStage,
    /// The downstream stage does not list the artifact in its inputs.
    NotConsumedByToStage,
}

impl Display for HandoffArtifactReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotProducedByFromStage => write!(f, "fromStage does not produce it"),
            Self::NotConsumedByToStage => write!(f, "toStage does not consume it"),
        }
    }
}

fn validate_unique_values<'a>(
    scope: &str,
    values: impl IntoIterator<Item = &'a str>,
) -> Result<(), PipelineContractError> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(PipelineContractError::DuplicateValue {
                scope: scope.to_string(),
                value: value.to_string(),
            });
        }
    }
    Ok(())
}
