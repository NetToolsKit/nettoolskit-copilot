//! Agent orchestration integrity validation.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::agent_orchestration::common::{
    read_required_json_document, read_required_json_value, read_required_pipeline_manifest,
    resolve_governance_default_path, resolve_repo_relative_path, resolve_validation_repo_root,
    AgentManifest, EvalFixtures, HandoffTemplate, PipelineDispatchMode, PipelineManifest,
    PipelineStageMode, RunArtifactTemplate,
};
use crate::error::ValidateAgentOrchestrationCommandError;
use crate::operational_hygiene::common::derive_status;
use crate::ValidationCheckStatus;

const AGENT_MANIFEST_PATH: &str = ".codex/orchestration/agents.manifest.json";
const AGENT_MANIFEST_SCHEMA_PATH: &str = ".github/schemas/agent.contract.schema.json";
const PIPELINE_PATH: &str = ".codex/orchestration/pipelines/default.pipeline.json";
const PIPELINE_SCHEMA_PATH: &str = ".github/schemas/agent.pipeline.schema.json";
const HANDOFF_TEMPLATE_PATH: &str = ".codex/orchestration/templates/handoff.template.json";
const HANDOFF_TEMPLATE_SCHEMA_PATH: &str = ".github/schemas/agent.handoff.schema.json";
const RUN_ARTIFACT_TEMPLATE_PATH: &str =
    ".codex/orchestration/templates/run-artifact.template.json";
const RUN_ARTIFACT_TEMPLATE_SCHEMA_PATH: &str = ".github/schemas/agent.run-artifact.schema.json";
const EVAL_FIXTURES_PATH: &str = ".codex/orchestration/evals/golden-tests.json";
const EVAL_FIXTURES_SCHEMA_PATH: &str = ".github/schemas/agent.evals.schema.json";

const REQUIRED_DIRECTORIES: &[&str] = &[
    ".codex/orchestration",
    ".codex/orchestration/pipelines",
    ".codex/orchestration/prompts",
    ".codex/orchestration/templates",
    ".codex/orchestration/evals",
    ".github/schemas",
    "scripts/orchestration/stages",
    "scripts/orchestration/engine",
];

const REQUIRED_FILES: &[&str] = &[
    "scripts/common/agent-runtime-hardening.ps1",
    "scripts/runtime/run-agent-pipeline.ps1",
    "scripts/runtime/resume-agent-pipeline.ps1",
    "scripts/runtime/replay-agent-run.ps1",
    "scripts/runtime/evaluate-agent-pipeline.ps1",
    "scripts/orchestration/engine/invoke-codex-dispatch.ps1",
    "scripts/orchestration/engine/invoke-task-worker.ps1",
    "scripts/orchestration/stages/intake-stage.ps1",
    "scripts/orchestration/stages/spec-stage.ps1",
    "scripts/orchestration/stages/plan-stage.ps1",
    "scripts/orchestration/stages/route-stage.ps1",
    "scripts/orchestration/stages/implement-stage.ps1",
    "scripts/orchestration/stages/validate-stage.ps1",
    "scripts/orchestration/stages/review-stage.ps1",
    "scripts/orchestration/stages/closeout-stage.ps1",
    ".github/schemas/agent.stage-intake-result.schema.json",
    ".github/schemas/agent.stage-spec-result.schema.json",
    ".github/schemas/agent.stage-plan-result.schema.json",
    ".github/schemas/agent.stage-route-result.schema.json",
    ".github/schemas/agent.stage-implementation-result.schema.json",
    ".github/schemas/agent.stage-review-result.schema.json",
    ".github/schemas/agent.stage-closeout-result.schema.json",
    ".github/schemas/agent.task-review-result.schema.json",
    ".github/schemas/agent.trace-record.schema.json",
    ".github/schemas/agent.policy-evaluation.schema.json",
    ".github/schemas/agent.checkpoint-state.schema.json",
    ".codex/orchestration/prompts/super-agent-intake-stage.prompt.md",
    ".codex/orchestration/prompts/spec-stage.prompt.md",
    ".codex/orchestration/prompts/planner-stage.prompt.md",
    ".codex/orchestration/prompts/router-stage.prompt.md",
    ".codex/orchestration/prompts/executor-task.prompt.md",
    ".codex/orchestration/prompts/task-spec-review.prompt.md",
    ".codex/orchestration/prompts/task-quality-review.prompt.md",
    ".codex/orchestration/prompts/reviewer-stage.prompt.md",
    ".codex/orchestration/prompts/closeout-stage.prompt.md",
    ".codex/orchestration/templates/trace-record.template.json",
    ".codex/orchestration/templates/policy-evaluations.template.json",
    ".codex/orchestration/templates/checkpoint-state.template.json",
];

const REQUIRED_GOVERNANCE_FILES: &[&str] = &[
    "agent-runtime-policy.catalog.json",
    "agent-model-routing.catalog.json",
];

/// Request payload for `validate-agent-orchestration`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValidateAgentOrchestrationRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
}

/// Result payload for `validate-agent-orchestration`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAgentOrchestrationResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Number of required directories checked.
    pub required_directories_checked: usize,
    /// Number of required files checked.
    pub required_files_checked: usize,
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

/// Run the agent orchestration integrity validation.
///
/// # Errors
///
/// Returns [`ValidateAgentOrchestrationCommandError`] when the repository root
/// cannot be resolved.
pub fn invoke_validate_agent_orchestration(
    request: &ValidateAgentOrchestrationRequest,
) -> Result<ValidateAgentOrchestrationResult, ValidateAgentOrchestrationCommandError> {
    let repo_root =
        resolve_validation_repo_root(request.repo_root.as_deref()).map_err(|source| {
            ValidateAgentOrchestrationCommandError::ResolveWorkspaceRoot { source }
        })?;

    let mut warnings = Vec::new();
    let mut failures = Vec::new();

    for relative_path in REQUIRED_DIRECTORIES {
        let absolute_path = repo_root.join(relative_path);
        if !absolute_path.is_dir() {
            failures.push(format!("Missing required directory: {relative_path}"));
        }
    }
    for relative_path in REQUIRED_FILES {
        let absolute_path = repo_root.join(relative_path);
        if !absolute_path.is_file() {
            failures.push(format!("Missing required file: {relative_path}"));
        }
    }
    for file_name in REQUIRED_GOVERNANCE_FILES {
        let absolute_path = resolve_governance_default_path(&repo_root, file_name);
        if !absolute_path.is_file() {
            failures.push(format!(
                "Missing required file: definitions/providers/github/governance/{file_name}"
            ));
        }
    }

    let agent_manifest = read_contract_document::<AgentManifest>(
        &repo_root,
        AGENT_MANIFEST_PATH,
        AGENT_MANIFEST_SCHEMA_PATH,
        "agents manifest",
        &mut failures,
    );
    let _pipeline_schema = read_required_json_value(
        &repo_root.join(PIPELINE_SCHEMA_PATH),
        PIPELINE_SCHEMA_PATH,
        false,
        &mut warnings,
        &mut failures,
    );
    let pipeline_manifest = read_required_pipeline_manifest(
        &resolve_repo_relative_path(&repo_root, None, PIPELINE_PATH),
        "pipeline manifest",
        false,
        &mut warnings,
        &mut failures,
    );
    let handoff_template = read_contract_document::<HandoffTemplate>(
        &repo_root,
        HANDOFF_TEMPLATE_PATH,
        HANDOFF_TEMPLATE_SCHEMA_PATH,
        "handoff template",
        &mut failures,
    );
    let run_artifact_template = read_contract_document::<RunArtifactTemplate>(
        &repo_root,
        RUN_ARTIFACT_TEMPLATE_PATH,
        RUN_ARTIFACT_TEMPLATE_SCHEMA_PATH,
        "run artifact template",
        &mut failures,
    );
    let eval_fixtures = read_contract_document::<EvalFixtures>(
        &repo_root,
        EVAL_FIXTURES_PATH,
        EVAL_FIXTURES_SCHEMA_PATH,
        "eval fixtures",
        &mut failures,
    );

    for (path, label) in [
        (
            ".codex/orchestration/templates/trace-record.template.json",
            "trace record template",
        ),
        (
            ".codex/orchestration/templates/policy-evaluations.template.json",
            "policy evaluations template",
        ),
        (
            ".codex/orchestration/templates/checkpoint-state.template.json",
            "checkpoint state template",
        ),
    ] {
        let _ = read_required_json_value(
            &repo_root.join(path),
            label,
            false,
            &mut warnings,
            &mut failures,
        );
    }

    let agents_checked = agent_manifest
        .as_ref()
        .map_or(0usize, |manifest| manifest.agents.len());
    let stage_checks = pipeline_manifest
        .as_ref()
        .map_or(0usize, |pipeline| pipeline.stages.len());

    if let Some(agent_manifest) = agent_manifest.as_ref() {
        validate_agent_manifest_integrity(&repo_root, agent_manifest, &mut failures);
    }
    if let (Some(pipeline_manifest), Some(agent_manifest)) =
        (pipeline_manifest.as_ref(), agent_manifest.as_ref())
    {
        validate_pipeline_manifest_integrity(
            &repo_root,
            pipeline_manifest,
            agent_manifest,
            &mut failures,
        );
    }
    if let (Some(handoff_template), Some(pipeline_manifest)) =
        (handoff_template.as_ref(), pipeline_manifest.as_ref())
    {
        validate_handoff_template_integrity(handoff_template, pipeline_manifest, &mut failures);
    }
    if let (Some(run_artifact_template), Some(pipeline_manifest), Some(agent_manifest)) = (
        run_artifact_template.as_ref(),
        pipeline_manifest.as_ref(),
        agent_manifest.as_ref(),
    ) {
        validate_run_artifact_template_integrity(
            run_artifact_template,
            pipeline_manifest,
            agent_manifest,
            &mut failures,
        );
    }
    if let (Some(eval_fixtures), Some(pipeline_manifest), Some(agent_manifest)) = (
        eval_fixtures.as_ref(),
        pipeline_manifest.as_ref(),
        agent_manifest.as_ref(),
    ) {
        validate_eval_fixtures_integrity(
            eval_fixtures,
            pipeline_manifest,
            agent_manifest,
            &mut warnings,
            &mut failures,
        );
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateAgentOrchestrationResult {
        repo_root,
        required_directories_checked: REQUIRED_DIRECTORIES.len(),
        required_files_checked: REQUIRED_FILES.len() + REQUIRED_GOVERNANCE_FILES.len(),
        agents_checked,
        stage_checks,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn read_contract_document<T: serde::de::DeserializeOwned>(
    repo_root: &std::path::Path,
    document_path: &str,
    schema_path: &str,
    label: &str,
    failures: &mut Vec<String>,
) -> Option<T> {
    let _schema = read_required_json_value(
        &repo_root.join(schema_path),
        schema_path,
        false,
        &mut Vec::new(),
        failures,
    )?;
    read_required_json_document::<T>(
        &resolve_repo_relative_path(repo_root, None, document_path),
        label,
        false,
        &mut Vec::new(),
        failures,
    )
}

fn validate_agent_manifest_integrity(
    repo_root: &std::path::Path,
    manifest: &AgentManifest,
    failures: &mut Vec<String>,
) {
    let mut agent_ids = HashSet::new();
    let mut roles = HashSet::new();

    for agent in &manifest.agents {
        if !agent_ids.insert(agent.id.clone()) {
            failures.push(format!("Duplicate agent id in manifest: {}", agent.id));
        }

        if !agent.role.trim().is_empty() {
            roles.insert(agent.role.clone());
        }

        let skill_markdown = repo_root.join(format!(".codex/skills/{}/SKILL.md", agent.skill));
        if !skill_markdown.is_file() {
            failures.push(format!(
                "Agent {} references missing skill markdown: .codex/skills/{}/SKILL.md",
                agent.id, agent.skill
            ));
        }

        let skill_openai =
            repo_root.join(format!(".codex/skills/{}/agents/openai.yaml", agent.skill));
        if !skill_openai.is_file() {
            failures.push(format!(
                "Agent {} references missing skill config: .codex/skills/{}/agents/openai.yaml",
                agent.id, agent.skill
            ));
        }

        if agent.approval_required
            && agent
                .approval_instructions
                .as_deref()
                .map(str::trim)
                .is_none_or(str::is_empty)
        {
            failures.push(format!(
                "Agent {} requires approval but does not declare approvalInstructions.",
                agent.id
            ));
        }

        for path in &agent.allowed_paths {
            if path.trim().is_empty() {
                failures.push(format!("Agent {} has blank allowed path entry.", agent.id));
            }
            if path == "*" || path == "/**" {
                failures.push(format!(
                    "Agent {} uses overly broad allowed path pattern: {}",
                    agent.id, path
                ));
            }
        }
    }

    for agent in &manifest.agents {
        let Some(fallback) = agent.fallback_agent_id.as_deref() else {
            continue;
        };
        if !fallback.trim().is_empty() && !agent_ids.contains(fallback) {
            failures.push(format!(
                "Agent {} references unknown fallbackAgentId: {}",
                agent.id, fallback
            ));
        }
    }

    for required_role in [
        "planner",
        "router",
        "specialist",
        "reviewer",
        "release",
        "tester",
    ] {
        if !roles.contains(required_role) {
            failures.push(format!(
                "Agent manifest missing required role: {required_role}"
            ));
        }
    }
}

fn validate_pipeline_manifest_integrity(
    repo_root: &std::path::Path,
    pipeline: &PipelineManifest,
    manifest: &AgentManifest,
    failures: &mut Vec<String>,
) {
    let agent_ids: HashSet<String> = manifest
        .agents
        .iter()
        .map(|agent| agent.id.clone())
        .collect();
    let mut stage_ids = HashSet::new();
    let mut stage_outputs: HashMap<String, HashSet<String>> = HashMap::new();
    let stage_map: HashMap<String, _> = pipeline
        .stages
        .iter()
        .map(|stage| (stage.id.clone(), stage))
        .collect();

    for stage in &pipeline.stages {
        if !stage_ids.insert(stage.id.clone()) {
            failures.push(format!("Duplicate pipeline stage id: {}", stage.id));
        }

        if !agent_ids.contains(&stage.agent_id) {
            failures.push(format!(
                "Pipeline stage {} references unknown agentId: {}",
                stage.id, stage.agent_id
            ));
        }

        if stage.execution.script_path.trim().is_empty() {
            failures.push(format!(
                "Pipeline stage {} has empty execution.scriptPath.",
                stage.id
            ));
        } else if !repo_root.join(&stage.execution.script_path).is_file() {
            failures.push(format!(
                "Pipeline stage {} execution script not found: {}",
                stage.id, stage.execution.script_path
            ));
        }

        if matches!(
            stage.execution.dispatch_mode,
            Some(PipelineDispatchMode::CodexExec)
        ) {
            let prompt_template_path = stage.execution.prompt_template_path.as_deref();
            if prompt_template_path.is_none_or(|path| path.trim().is_empty()) {
                failures.push(format!(
                    "Pipeline stage {} dispatchMode codex-exec requires promptTemplatePath.",
                    stage.id
                ));
            } else if !repo_root
                .join(prompt_template_path.expect("checked above"))
                .is_file()
            {
                failures.push(format!(
                    "Pipeline stage {} prompt template not found: {}",
                    stage.id,
                    prompt_template_path.expect("checked above")
                ));
            }

            let response_schema_path = stage.execution.response_schema_path.as_deref();
            if response_schema_path.is_none_or(|path| path.trim().is_empty()) {
                failures.push(format!(
                    "Pipeline stage {} dispatchMode codex-exec requires responseSchemaPath.",
                    stage.id
                ));
            } else if !repo_root
                .join(response_schema_path.expect("checked above"))
                .is_file()
            {
                failures.push(format!(
                    "Pipeline stage {} response schema not found: {}",
                    stage.id,
                    response_schema_path.expect("checked above")
                ));
            }
        }

        stage_outputs.insert(
            stage.id.clone(),
            stage.output_artifacts.iter().cloned().collect(),
        );
    }

    if let Some(first_stage) = pipeline.stages.first() {
        if first_stage.mode != PipelineStageMode::Plan {
            failures.push(format!(
                "Pipeline first stage must be mode 'plan', found '{}'.",
                first_stage.mode.as_str()
            ));
        }
        if !first_stage
            .input_artifacts
            .iter()
            .any(|artifact| artifact == "request")
        {
            failures.push("Pipeline first stage must consume 'request' artifact.".to_string());
        }
    }
    if let Some(last_stage) = pipeline.stages.last() {
        if last_stage.mode != PipelineStageMode::Review {
            failures.push(format!(
                "Pipeline last stage must be mode 'review', found '{}'.",
                last_stage.mode.as_str()
            ));
        }
    }

    for handoff in &pipeline.handoffs {
        if !stage_ids.contains(&handoff.from_stage) {
            failures.push(format!(
                "Handoff references unknown fromStage: {}",
                handoff.from_stage
            ));
            continue;
        }
        if !stage_ids.contains(&handoff.to_stage) {
            failures.push(format!(
                "Handoff references unknown toStage: {}",
                handoff.to_stage
            ));
            continue;
        }

        let from_outputs = stage_outputs
            .get(&handoff.from_stage)
            .cloned()
            .unwrap_or_default();
        let target_stage = stage_map.get(&handoff.to_stage).copied();
        for artifact in &handoff.required_artifacts {
            if !from_outputs.contains(artifact) {
                failures.push(format!(
                    "Handoff {}->{} requires artifact not produced by {}: {}",
                    handoff.from_stage, handoff.to_stage, handoff.from_stage, artifact
                ));
            }
            if let Some(target_stage) = target_stage {
                if !target_stage
                    .input_artifacts
                    .iter()
                    .any(|input| input == artifact)
                {
                    failures.push(format!(
                        "Handoff {}->{} requires artifact not consumed by target stage {}: {}",
                        handoff.from_stage, handoff.to_stage, handoff.to_stage, artifact
                    ));
                }
            }
        }
    }

    for required_stage in &pipeline.completion_criteria.required_stages {
        if !stage_ids.contains(required_stage) {
            failures.push(format!(
                "Completion criteria references unknown stage: {}",
                required_stage
            ));
        }
    }

    let all_outputs: HashSet<String> = stage_outputs
        .values()
        .flat_map(|set| set.iter().cloned())
        .collect();
    for artifact in &pipeline.completion_criteria.required_artifacts {
        if !all_outputs.contains(artifact) {
            failures.push(format!(
                "Completion criteria references artifact not produced by any stage: {}",
                artifact
            ));
        }
    }

    let Some(runtime) = pipeline.runtime.as_ref() else {
        failures.push("Pipeline manifest must declare a runtime configuration object.".to_string());
        return;
    };

    for (property_name, path) in [
        ("policyCatalogPath", runtime.policy_catalog_path.as_deref()),
        (
            "modelRoutingCatalogPath",
            runtime.model_routing_catalog_path.as_deref(),
        ),
    ] {
        if path.is_none_or(|value| value.trim().is_empty()) {
            failures.push(format!("Pipeline runtime is missing {property_name}."));
            continue;
        }
        let path = path.expect("checked above");
        if !repo_root.join(path).is_file() {
            failures.push(format!(
                "Pipeline runtime catalog not found for {property_name}: {path}"
            ));
        }
    }
}

fn validate_handoff_template_integrity(
    handoff_template: &HandoffTemplate,
    pipeline: &PipelineManifest,
    failures: &mut Vec<String>,
) {
    if handoff_template.artifacts.is_empty() {
        failures.push("Handoff template must include at least one artifact entry.".to_string());
    }

    let stage_ids: HashSet<String> = pipeline
        .stages
        .iter()
        .map(|stage| stage.id.clone())
        .collect();
    if !stage_ids.contains(&handoff_template.from_stage) {
        failures.push(format!(
            "Handoff template references unknown fromStage: {}",
            handoff_template.from_stage
        ));
    }
    if !stage_ids.contains(&handoff_template.to_stage) {
        failures.push(format!(
            "Handoff template references unknown toStage: {}",
            handoff_template.to_stage
        ));
    }
}

fn validate_run_artifact_template_integrity(
    run_artifact_template: &RunArtifactTemplate,
    pipeline: &PipelineManifest,
    manifest: &AgentManifest,
    failures: &mut Vec<String>,
) {
    if run_artifact_template.stages.is_empty() {
        failures.push("Run artifact template must include at least one stage entry.".to_string());
        return;
    }

    let pipeline_stage_ids: HashSet<String> = pipeline
        .stages
        .iter()
        .map(|stage| stage.id.clone())
        .collect();
    let agent_ids: HashSet<String> = manifest
        .agents
        .iter()
        .map(|agent| agent.id.clone())
        .collect();
    for stage in &run_artifact_template.stages {
        if !pipeline_stage_ids.contains(&stage.stage_id) {
            failures.push(format!(
                "Run artifact template references unknown stageId: {}",
                stage.stage_id
            ));
        }
        if !agent_ids.contains(&stage.agent_id) {
            failures.push(format!(
                "Run artifact template references unknown agentId: {}",
                stage.agent_id
            ));
        }
    }

    if run_artifact_template.summary.stage_count != run_artifact_template.stages.len() as i64 {
        failures.push(format!(
            "Run artifact summary stageCount ({}) must match stages length ({}).",
            run_artifact_template.summary.stage_count,
            run_artifact_template.stages.len()
        ));
    }
}

fn validate_eval_fixtures_integrity(
    eval_fixtures: &EvalFixtures,
    pipeline: &PipelineManifest,
    manifest: &AgentManifest,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let agent_ids: HashSet<String> = manifest
        .agents
        .iter()
        .map(|agent| agent.id.clone())
        .collect();
    let pipeline_order: Vec<String> = pipeline
        .stages
        .iter()
        .map(|stage| stage.id.clone())
        .collect();

    for case in &eval_fixtures.cases {
        if case.expected_pipeline_id != pipeline.id {
            failures.push(format!(
                "Eval case {} expectedPipelineId mismatch. Expected {}, found {}",
                case.id, pipeline.id, case.expected_pipeline_id
            ));
        }
        if case.expected_stage_order != pipeline_order {
            warnings.push(format!(
                "Eval case {} stage order diverges from pipeline order.",
                case.id
            ));
        }
        for required_agent in &case.required_agents {
            if !agent_ids.contains(required_agent) {
                failures.push(format!(
                    "Eval case {} references unknown required agent: {}",
                    case.id, required_agent
                ));
            }
        }
    }
}