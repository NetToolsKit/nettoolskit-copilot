//! Agent permission matrix validation.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::agent_orchestration::common::{
    matches_any_glob, normalize_path, read_required_json_document, resolve_repo_relative_path,
    resolve_validation_repo_root, AgentContract, AgentManifest, PermissionMatrix,
    PermissionMatrixAgent, PipelineManifest,
};
use crate::error::ValidateAgentPermissionsCommandError;
use crate::operational_hygiene::common::{derive_status, push_required_finding};
use crate::ValidationCheckStatus;

const DEFAULT_MATRIX_PATH: &str = ".github/governance/agent-skill-permissions.matrix.json";
const DEFAULT_AGENT_MANIFEST_PATH: &str = ".codex/orchestration/agents.manifest.json";
const DEFAULT_PIPELINE_PATH: &str = ".codex/orchestration/pipelines/default.pipeline.json";

/// Request payload for `validate-agent-permissions`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAgentPermissionsRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit permission matrix path.
    pub matrix_path: Option<PathBuf>,
    /// Optional explicit agent manifest path.
    pub agent_manifest_path: Option<PathBuf>,
    /// Optional explicit pipeline path.
    pub pipeline_path: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateAgentPermissionsRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            matrix_path: None,
            agent_manifest_path: None,
            pipeline_path: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-agent-permissions`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAgentPermissionsResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved permission matrix path.
    pub matrix_path: PathBuf,
    /// Resolved agent manifest path.
    pub agent_manifest_path: PathBuf,
    /// Resolved pipeline path.
    pub pipeline_path: PathBuf,
    /// Number of agents checked.
    pub agents_checked: usize,
    /// Number of pipeline stages checked.
    pub stage_checks: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Run the agent permission validation sweep.
///
/// # Errors
///
/// Returns [`ValidateAgentPermissionsCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_agent_permissions(
    request: &ValidateAgentPermissionsRequest,
) -> Result<ValidateAgentPermissionsResult, ValidateAgentPermissionsCommandError> {
    let repo_root = resolve_validation_repo_root(request.repo_root.as_deref()).map_err(|source| {
        ValidateAgentPermissionsCommandError::ResolveWorkspaceRoot { source }
    })?;
    let matrix_path =
        resolve_repo_relative_path(&repo_root, request.matrix_path.as_deref(), DEFAULT_MATRIX_PATH);
    let agent_manifest_path = resolve_repo_relative_path(
        &repo_root,
        request.agent_manifest_path.as_deref(),
        DEFAULT_AGENT_MANIFEST_PATH,
    );
    let pipeline_path = resolve_repo_relative_path(
        &repo_root,
        request.pipeline_path.as_deref(),
        DEFAULT_PIPELINE_PATH,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut agents_checked = 0usize;
    let mut stage_checks = 0usize;

    let matrix = read_required_json_document::<PermissionMatrix>(
        &matrix_path,
        "agent permission matrix",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let agent_manifest = read_required_json_document::<AgentManifest>(
        &agent_manifest_path,
        "agent manifest",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let pipeline = read_required_json_document::<PipelineManifest>(
        &pipeline_path,
        "pipeline manifest",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    if let (Some(matrix), Some(agent_manifest), Some(pipeline)) = (matrix, agent_manifest, pipeline)
    {
        let matrix_map =
            build_matrix_map(&matrix, request.warning_only, &mut warnings, &mut failures);
        let required_blocked_commands = &matrix.global_rules.required_blocked_command_prefixes;
        let allowed_stage_prefixes = &matrix.global_rules.allowed_stage_script_prefixes;
        let manifest_ids: HashSet<String> = agent_manifest
            .agents
            .iter()
            .map(|agent| agent.id.clone())
            .collect();

        agents_checked = agent_manifest.agents.len();
        for agent in &agent_manifest.agents {
            let Some(matrix_entry) = matrix_map.get(&agent.id) else {
                push_required_finding(
                    request.warning_only,
                    &mut warnings,
                    &mut failures,
                    format!("Agent missing matrix entry: {}", agent.id),
                );
                continue;
            };

            test_agent_permission_contract(
                agent,
                matrix_entry,
                required_blocked_commands,
                request.warning_only,
                &mut warnings,
                &mut failures,
            );
        }

        for matrix_agent_id in matrix_map.keys() {
            if !manifest_ids.contains(matrix_agent_id) {
                warnings.push(format!(
                    "Matrix has agent not present in manifest: {matrix_agent_id}"
                ));
            }
        }

        stage_checks = pipeline.stages.len();
        test_stage_permission_contract(
            &pipeline,
            &matrix_map,
            allowed_stage_prefixes,
            request.warning_only,
            &mut warnings,
            &mut failures,
        );
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateAgentPermissionsResult {
        repo_root,
        warning_only: request.warning_only,
        matrix_path,
        agent_manifest_path,
        pipeline_path,
        agents_checked,
        stage_checks,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn build_matrix_map(
    matrix: &PermissionMatrix,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> HashMap<String, PermissionMatrixAgent> {
    let mut map = HashMap::new();
    for entry in &matrix.agents {
        if entry.agent_id.trim().is_empty() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                "agent-skill-permissions matrix has an entry with blank agentId.".to_string(),
            );
            continue;
        }

        if map.contains_key(&entry.agent_id) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Duplicate agentId in matrix: {}", entry.agent_id),
            );
            continue;
        }

        map.insert(entry.agent_id.clone(), entry.clone());
    }
    map
}

fn test_agent_permission_contract(
    agent: &AgentContract,
    matrix_entry: &PermissionMatrixAgent,
    required_blocked_commands: &[String],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if !matrix_entry.role.trim().is_empty() && agent.role != matrix_entry.role {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Role mismatch for agent '{}': manifest='{}' matrix='{}'.",
                agent.id, agent.role, matrix_entry.role
            ),
        );
    }

    if !matrix_entry.skill.trim().is_empty() && agent.skill != matrix_entry.skill {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!(
                "Skill mismatch for agent '{}': manifest='{}' matrix='{}'.",
                agent.id, agent.skill, matrix_entry.skill
            ),
        );
    }

    for manifest_path in &agent.allowed_paths {
        if !matches_any_glob(
            manifest_path,
            &matrix_entry.allowed_path_globs,
            &format!("matrix allowed paths for agent '{}'", agent.id),
            warning_only,
            warnings,
            failures,
        ) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("Agent '{}' path not allowed by matrix: {manifest_path}", agent.id),
            );
        }
    }

    for required_blocked_command in required_blocked_commands {
        let has_required_command = agent.blocked_commands.iter().any(|blocked_command| {
            blocked_command.trim().eq_ignore_ascii_case(required_blocked_command.trim())
        });
        if !has_required_command {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Agent '{}' missing required blocked command: {required_blocked_command}",
                    agent.id
                ),
            );
        }
    }

    for (field_name, matrix_value, agent_value) in [
        (
            "maxSteps",
            matrix_entry.required_budget.max_steps,
            agent.budget.max_steps,
        ),
        (
            "maxDurationMinutes",
            matrix_entry.required_budget.max_duration_minutes,
            agent.budget.max_duration_minutes,
        ),
        (
            "maxFileEdits",
            matrix_entry.required_budget.max_file_edits,
            agent.budget.max_file_edits,
        ),
        (
            "maxTokens",
            matrix_entry.required_budget.max_tokens,
            agent.budget.max_tokens,
        ),
    ] {
        if matrix_value != agent_value {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Budget mismatch for agent '{}' field '{}': manifest={} matrix={}",
                    agent.id, field_name, agent_value, matrix_value
                ),
            );
        }
    }
}

fn test_stage_permission_contract(
    pipeline: &PipelineManifest,
    matrix_map: &HashMap<String, PermissionMatrixAgent>,
    allowed_stage_prefixes: &[String],
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for stage in &pipeline.stages {
        let Some(matrix_entry) = matrix_map.get(&stage.agent_id) else {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Stage '{}' references agent without matrix entry: {}",
                    stage.id, stage.agent_id
                ),
            );
            continue;
        };

        let script_path = normalize_path(&stage.execution.script_path);
        let has_allowed_prefix = allowed_stage_prefixes.iter().any(|prefix| {
            let normalized_prefix = normalize_path(prefix);
            !normalized_prefix.trim().is_empty()
                && script_path
                    .to_ascii_lowercase()
                    .starts_with(&normalized_prefix.to_ascii_lowercase())
        });

        if !has_allowed_prefix {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Stage '{}' uses script outside global allowed prefixes: {}",
                    stage.id, stage.execution.script_path
                ),
            );
        }

        if !matches_any_glob(
            &stage.execution.script_path,
            &matrix_entry.allowed_stage_script_globs,
            &format!("stage script rules for agent '{}'", stage.agent_id),
            warning_only,
            warnings,
            failures,
        ) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Stage '{}' script not allowed for agent '{}': {}",
                    stage.id, stage.agent_id, stage.execution.script_path
                ),
            );
        }
    }
}