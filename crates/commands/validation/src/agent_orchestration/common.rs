//! Shared types and helpers for agent orchestration validation.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use nettoolskit_orchestrator::load_pipeline_manifest;
pub(crate) use nettoolskit_orchestrator::{
    PipelineDispatchMode, PipelineManifest, PipelineStageMode,
};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;

use crate::operational_hygiene::common::push_required_finding;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentManifest {
    #[serde(default)]
    pub agents: Vec<AgentContract>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentContract {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub skill: String,
    #[serde(default)]
    pub allowed_paths: Vec<String>,
    #[serde(default)]
    pub blocked_commands: Vec<String>,
    #[serde(default)]
    pub budget: AgentBudget,
    #[serde(default)]
    pub fallback_agent_id: Option<String>,
    #[serde(default)]
    pub approval_required: bool,
    #[serde(default)]
    pub approval_instructions: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AgentBudget {
    #[serde(default)]
    pub max_steps: i64,
    #[serde(default)]
    pub max_duration_minutes: i64,
    #[serde(default)]
    pub max_file_edits: i64,
    #[serde(default)]
    pub max_tokens: i64,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PermissionMatrix {
    #[serde(default)]
    pub global_rules: PermissionGlobalRules,
    #[serde(default)]
    pub agents: Vec<PermissionMatrixAgent>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PermissionGlobalRules {
    #[serde(default)]
    pub required_blocked_command_prefixes: Vec<String>,
    #[serde(default)]
    pub allowed_stage_script_prefixes: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PermissionMatrixAgent {
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub skill: String,
    #[serde(default)]
    pub allowed_path_globs: Vec<String>,
    #[serde(default)]
    pub allowed_stage_script_globs: Vec<String>,
    #[serde(default)]
    pub required_budget: AgentBudget,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EvalFixtures {
    #[serde(default)]
    pub cases: Vec<EvalCase>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EvalCase {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub expected_pipeline_id: String,
    #[serde(default)]
    pub expected_stage_order: Vec<String>,
    #[serde(default)]
    pub required_agents: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HandoffTemplate {
    #[serde(default)]
    pub from_stage: String,
    #[serde(default)]
    pub to_stage: String,
    #[serde(default)]
    pub artifacts: Vec<HandoffArtifact>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HandoffArtifact {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub checksum: String,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RunArtifactTemplate {
    #[serde(default)]
    pub stages: Vec<RunArtifactStage>,
    #[serde(default)]
    pub summary: RunArtifactSummary,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RunArtifactStage {
    #[serde(default)]
    pub stage_id: String,
    #[serde(default)]
    pub agent_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RunArtifactSummary {
    #[serde(default)]
    pub stage_count: i64,
}

pub(crate) fn resolve_validation_repo_root(
    repo_root: Option<&Path>,
) -> Result<PathBuf, anyhow::Error> {
    let current_dir = env::current_dir()?;
    resolve_repository_root(repo_root, None, &current_dir)
}

pub(crate) fn resolve_repo_relative_path(
    repo_root: &Path,
    override_path: Option<&Path>,
    default_path: &str,
) -> PathBuf {
    match override_path {
        Some(path) => resolve_full_path(repo_root, path),
        None => repo_root.join(default_path),
    }
}

pub(crate) fn read_required_json_document<T: DeserializeOwned>(
    path: &Path,
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<T> {
    let value = read_required_json_value(path, label, warning_only, warnings, failures)?;
    match serde_json::from_value::<T>(value) {
        Ok(document) => Some(document),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid JSON structure in {label}: {error}"),
            );
            None
        }
    }
}

pub(crate) fn read_required_pipeline_manifest(
    path: &Path,
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<PipelineManifest> {
    match load_pipeline_manifest(path) {
        Ok(manifest) => Some(manifest),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid {label}: {error}"),
            );
            None
        }
    }
}

pub(crate) fn read_required_json_value(
    path: &Path,
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<Value> {
    if !path.is_file() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Missing {label}: {}", path.display()),
        );
        return None;
    }

    let document = match fs::read_to_string(path) {
        Ok(document) => document,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("{label} is not readable: {error}"),
            );
            return None;
        }
    };

    match serde_json::from_str::<Value>(&document) {
        Ok(value) => Some(value),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid JSON in {label}: {error}"),
            );
            None
        }
    }
}

pub(crate) fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}

pub(crate) fn matches_any_glob(
    text: &str,
    patterns: &[String],
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> bool {
    let Some(globset) = compile_globset(patterns, label, warning_only, warnings, failures) else {
        return false;
    };
    globset.is_match(normalize_path(text))
}

pub(crate) fn compile_globset(
    patterns: &[String],
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        if pattern.trim().is_empty() {
            continue;
        }

        match Glob::new(&normalize_path(pattern)) {
            Ok(glob) => {
                builder.add(glob);
            }
            Err(error) => {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!("Invalid glob in {label}: {pattern} ({error})"),
                );
                return None;
            }
        }
    }

    match builder.build() {
        Ok(globset) => Some(globset),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Invalid globset in {label}: {error}"),
            );
            None
        }
    }
}

pub(crate) fn read_skill_frontmatter_map(
    path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> BTreeMap<String, String> {
    let Ok(contents) = fs::read_to_string(path) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Skill file is not readable: {}", path.display()),
        );
        return BTreeMap::new();
    };

    let lines: Vec<&str> = contents.lines().collect();
    if lines.len() < 3 {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Invalid SKILL.md frontmatter (too short): {}",
                path.display()
            ),
        );
        return BTreeMap::new();
    }

    if lines.first().is_none_or(|line| line.trim() != "---") {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Missing SKILL.md frontmatter start marker: {}",
                path.display()
            ),
        );
        return BTreeMap::new();
    }

    let Some(end_index) = lines.iter().skip(1).position(|line| line.trim() == "---") else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Missing SKILL.md frontmatter end marker: {}",
                path.display()
            ),
        );
        return BTreeMap::new();
    };

    let end_index = end_index + 1;
    let mut map = BTreeMap::new();
    for line in &lines[1..end_index] {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };

        let normalized = value
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .to_string();
        map.insert(key.trim().to_string(), normalized);
    }

    map
}
