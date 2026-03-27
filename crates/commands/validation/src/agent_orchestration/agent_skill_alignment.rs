//! Agent skill alignment validation.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use crate::agent_orchestration::common::{
    read_required_json_document, read_skill_frontmatter_map, resolve_repo_relative_path,
    resolve_validation_repo_root, AgentManifest, EvalFixtures, PipelineManifest,
};
use crate::error::ValidateAgentSkillAlignmentCommandError;
use crate::operational_hygiene::common::derive_status;
use crate::ValidationCheckStatus;

const DEFAULT_AGENT_MANIFEST_PATH: &str = ".codex/orchestration/agents.manifest.json";
const DEFAULT_PIPELINE_PATH: &str = ".codex/orchestration/pipelines/default.pipeline.json";
const DEFAULT_EVAL_FIXTURE_PATH: &str = ".codex/orchestration/evals/golden-tests.json";
const DEFAULT_SKILLS_ROOT_PATH: &str = ".codex/skills";

/// Request payload for `validate-agent-skill-alignment`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAgentSkillAlignmentRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit agent manifest path.
    pub agent_manifest_path: Option<PathBuf>,
    /// Optional explicit pipeline path.
    pub pipeline_path: Option<PathBuf>,
    /// Optional explicit eval fixture path.
    pub eval_fixture_path: Option<PathBuf>,
    /// Optional explicit skills root path.
    pub skills_root_path: Option<PathBuf>,
}

impl Default for ValidateAgentSkillAlignmentRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            agent_manifest_path: None,
            pipeline_path: None,
            eval_fixture_path: None,
            skills_root_path: None,
        }
    }
}

/// Result payload for `validate-agent-skill-alignment`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAgentSkillAlignmentResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved agent manifest path.
    pub agent_manifest_path: PathBuf,
    /// Resolved pipeline path.
    pub pipeline_path: PathBuf,
    /// Resolved eval fixture path.
    pub eval_fixture_path: PathBuf,
    /// Resolved skills root path.
    pub skills_root_path: PathBuf,
    /// Number of agents checked.
    pub agents_checked: usize,
    /// Number of stage checks.
    pub stage_checks: usize,
    /// Number of eval case checks.
    pub eval_case_checks: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Run the agent skill alignment validation sweep.
///
/// # Errors
///
/// Returns [`ValidateAgentSkillAlignmentCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_agent_skill_alignment(
    request: &ValidateAgentSkillAlignmentRequest,
) -> Result<ValidateAgentSkillAlignmentResult, ValidateAgentSkillAlignmentCommandError> {
    let repo_root = resolve_validation_repo_root(request.repo_root.as_deref()).map_err(|source| {
        ValidateAgentSkillAlignmentCommandError::ResolveWorkspaceRoot { source }
    })?;
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
    let eval_fixture_path = resolve_repo_relative_path(
        &repo_root,
        request.eval_fixture_path.as_deref(),
        DEFAULT_EVAL_FIXTURE_PATH,
    );
    let skills_root_path = resolve_repo_relative_path(
        &repo_root,
        request.skills_root_path.as_deref(),
        DEFAULT_SKILLS_ROOT_PATH,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut agents_checked = 0usize;
    let mut stage_checks = 0usize;
    let mut eval_case_checks = 0usize;

    let agent_manifest = read_required_json_document::<AgentManifest>(
        &agent_manifest_path,
        "agents manifest",
        false,
        &mut warnings,
        &mut failures,
    );
    let pipeline_manifest = read_required_json_document::<PipelineManifest>(
        &pipeline_path,
        "pipeline manifest",
        false,
        &mut warnings,
        &mut failures,
    );
    let eval_fixture = read_required_json_document::<EvalFixtures>(
        &eval_fixture_path,
        "eval fixture",
        false,
        &mut warnings,
        &mut failures,
    );

    if !skills_root_path.is_dir() {
        failures.push(format!("Skills root not found: {}", skills_root_path.display()));
    }

    if failures.is_empty() {
        let agent_manifest = agent_manifest.unwrap_or_default();
        let pipeline_manifest = pipeline_manifest.unwrap_or_default();
        let eval_fixture = eval_fixture.unwrap_or_default();

        agents_checked = agent_manifest.agents.len();
        stage_checks = pipeline_manifest.stages.len();
        eval_case_checks = eval_fixture.cases.len();

        if agent_manifest.agents.is_empty() {
            failures.push("Agent manifest has no agents.".to_string());
        }
        if pipeline_manifest.stages.is_empty() {
            failures.push("Pipeline has no stages.".to_string());
        }
        if eval_fixture.cases.is_empty() {
            failures.push("Eval fixture has no cases.".to_string());
        }

        let agent_map = build_agent_map(&agent_manifest, &mut failures);
        validate_fallback_references(&agent_manifest, &agent_map, &mut failures);
        validate_agent_skills(
            &agent_manifest,
            &skills_root_path,
            &mut warnings,
            &mut failures,
        );
        let pipeline_agent_set =
            validate_pipeline_stages(&repo_root, &pipeline_manifest, &agent_map, &mut warnings, &mut failures);
        validate_eval_required_agents(
            &eval_fixture,
            &agent_map,
            &pipeline_agent_set,
            &mut failures,
        );
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateAgentSkillAlignmentResult {
        repo_root,
        agent_manifest_path,
        pipeline_path,
        eval_fixture_path,
        skills_root_path,
        agents_checked,
        stage_checks,
        eval_case_checks,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn build_agent_map(
    agent_manifest: &AgentManifest,
    failures: &mut Vec<String>,
) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for agent in &agent_manifest.agents {
        if agent.id.trim().is_empty() {
            failures.push("Agent manifest contains an entry with blank id.".to_string());
            continue;
        }

        if map.insert(agent.id.clone(), agent.role.clone()).is_some() {
            failures.push(format!("Duplicate agent id in manifest: {}", agent.id));
        }
    }
    map
}

fn validate_fallback_references(
    agent_manifest: &AgentManifest,
    agent_map: &HashMap<String, String>,
    failures: &mut Vec<String>,
) {
    for agent in &agent_manifest.agents {
        let Some(fallback_agent_id) = agent.fallback_agent_id.as_deref() else {
            continue;
        };
        if fallback_agent_id.trim().is_empty() {
            continue;
        }
        if !agent_map.contains_key(fallback_agent_id) {
            failures.push(format!(
                "Agent '{}' references unknown fallback agent '{}'.",
                agent.id, fallback_agent_id
            ));
        }
    }
}

fn validate_agent_skills(
    agent_manifest: &AgentManifest,
    skills_root_path: &std::path::Path,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    for agent in &agent_manifest.agents {
        if agent.skill.trim().is_empty() {
            failures.push(format!("Agent '{}' has empty skill reference.", agent.id));
            continue;
        }

        let skill_folder = skills_root_path.join(&agent.skill);
        let skill_file = skill_folder.join("SKILL.md");
        let openai_file = skill_folder.join("agents/openai.yaml");

        if !skill_folder.is_dir() {
            failures.push(format!(
                "Agent '{}' references missing skill folder: {}",
                agent.id,
                skill_folder.display()
            ));
            continue;
        }

        if !skill_file.is_file() {
            failures.push(format!(
                "Agent '{}' skill missing SKILL.md: {}",
                agent.id,
                skill_file.display()
            ));
            continue;
        }

        if !openai_file.is_file() {
            failures.push(format!(
                "Agent '{}' skill missing agents/openai.yaml: {}",
                agent.id,
                openai_file.display()
            ));
        }

        let frontmatter = read_skill_frontmatter_map(&skill_file, false, warnings, failures);
        match frontmatter.get("name") {
            Some(name) if name == &agent.skill => {}
            Some(name) => failures.push(format!(
                "Skill frontmatter name mismatch for agent '{}': expected '{}' found '{}'.",
                agent.id, agent.skill, name
            )),
            None => failures.push(format!(
                "Skill frontmatter missing 'name': {}",
                skill_file.display()
            )),
        }

        let Ok(skill_text) = fs::read_to_string(&skill_file) else {
            failures.push(format!(
                "Skill for agent '{}' is not readable: {}",
                agent.id,
                skill_file.display()
            ));
            continue;
        };

        for required_reference in [
            ".github/AGENTS.md",
            ".github/copilot-instructions.md",
            ".github/instruction-routing.catalog.yml",
        ] {
            if !skill_text.contains(required_reference) {
                failures.push(format!(
                    "Skill for agent '{}' missing required reference '{}': {}",
                    agent.id,
                    required_reference,
                    skill_file.display()
                ));
            }
        }

        if !skill_text.contains(".github/instructions/") {
            warnings.push(format!(
                "Skill for agent '{}' has no explicit .github/instructions reference: {}",
                agent.id,
                skill_file.display()
            ));
        }
    }
}

fn validate_pipeline_stages(
    repo_root: &std::path::Path,
    pipeline_manifest: &PipelineManifest,
    agent_map: &HashMap<String, String>,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> HashSet<String> {
    let mut pipeline_agent_set = HashSet::new();
    for stage in &pipeline_manifest.stages {
        pipeline_agent_set.insert(stage.agent_id.clone());

        if !agent_map.contains_key(&stage.agent_id) {
            failures.push(format!(
                "Pipeline stage '{}' references unknown agent id: {}",
                stage.id, stage.agent_id
            ));
            continue;
        }

        if stage.execution.script_path.trim().is_empty() {
            failures.push(format!(
                "Pipeline stage '{}' has empty execution.scriptPath.",
                stage.id
            ));
        } else if !repo_root.join(&stage.execution.script_path).is_file() {
            failures.push(format!(
                "Pipeline stage '{}' script not found: {}",
                stage.id, stage.execution.script_path
            ));
        }

        let expected_role = expected_role_for_stage(&stage.id, &stage.mode);
        if let Some(expected_role) = expected_role {
            let actual_role = agent_map
                .get(&stage.agent_id)
                .map(|role| role.to_ascii_lowercase())
                .unwrap_or_default();
            if actual_role != expected_role {
                failures.push(format!(
                    "Pipeline stage '{}' mode '{}' expects role '{}' but agent '{}' has role '{}'.",
                    stage.id, stage.mode, expected_role, stage.agent_id, actual_role
                ));
            }
        } else {
            warnings.push(format!(
                "Pipeline stage '{}' has non-standard mode '{}'.",
                stage.id, stage.mode
            ));
        }
    }

    pipeline_agent_set
}

fn expected_role_for_stage(stage_id: &str, mode: &str) -> Option<String> {
    let normalized_stage = stage_id.to_ascii_lowercase();
    let normalized_mode = mode.to_ascii_lowercase();
    if normalized_stage == "route" {
        return Some("router".to_string());
    }
    if normalized_stage == "implement" {
        return Some("specialist".to_string());
    }
    if normalized_stage == "closeout" {
        return Some("release".to_string());
    }

    match normalized_mode.as_str() {
        "plan" => Some("planner".to_string()),
        "execute" => Some("executor".to_string()),
        "validate" => Some("tester".to_string()),
        "review" => Some("reviewer".to_string()),
        _ => None,
    }
}

fn validate_eval_required_agents(
    eval_fixture: &EvalFixtures,
    agent_map: &HashMap<String, String>,
    pipeline_agent_set: &HashSet<String>,
    failures: &mut Vec<String>,
) {
    for eval_case in &eval_fixture.cases {
        for required_agent in &eval_case.required_agents {
            if !agent_map.contains_key(required_agent) {
                failures.push(format!(
                    "Eval case '{}' references unknown required agent: {}",
                    eval_case.id, required_agent
                ));
                continue;
            }

            if !pipeline_agent_set.contains(required_agent) {
                failures.push(format!(
                    "Eval case '{}' requires agent not present in pipeline stages: {}",
                    eval_case.id, required_agent
                ));
            }
        }
    }
}