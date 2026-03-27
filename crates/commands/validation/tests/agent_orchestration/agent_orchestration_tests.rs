//! Tests for agent_orchestration::agent_orchestration module.

use nettoolskit_validation::{
    invoke_validate_agent_orchestration, ValidateAgentOrchestrationRequest,
    ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::agent_orchestration_fixtures::{
    initialize_agent_contract_repo, remove_repo_path, write_pipeline_manifest,
};

#[test]
fn test_invoke_validate_agent_orchestration_passes_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());

    let result = invoke_validate_agent_orchestration(&ValidateAgentOrchestrationRequest {
        repo_root: Some(repo.path().to_path_buf()),
    })
    .expect("agent orchestration validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.required_directories_checked, 8);
    assert_eq!(result.agents_checked, 8);
    assert_eq!(result.stage_checks, 8);
}

#[test]
fn test_invoke_validate_agent_orchestration_reports_missing_required_file() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());
    remove_repo_path(repo.path(), "scripts/orchestration/stages/review-stage.ps1");

    let result = invoke_validate_agent_orchestration(&ValidateAgentOrchestrationRequest {
        repo_root: Some(repo.path().to_path_buf()),
    })
    .expect("agent orchestration validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Missing required file: scripts/orchestration/stages/review-stage.ps1")));
}

#[test]
fn test_invoke_validate_agent_orchestration_reports_invalid_pipeline_integrity() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());
    write_pipeline_manifest(
        repo.path(),
        r#"{
  "id": "default-dev-flow",
  "runtime": {
    "policyCatalogPath": ".github/governance/agent-runtime-policy.catalog.json",
    "modelRoutingCatalogPath": ".github/governance/agent-model-routing.catalog.json"
  },
  "stages": [
    { "id": "intake", "agentId": "super-agent", "mode": "review", "execution": { "scriptPath": "scripts/orchestration/stages/intake-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/super-agent-intake-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-intake-result.schema.json" }, "inputArtifacts": ["request"], "outputArtifacts": ["normalized-request", "intake-report"] },
    { "id": "closeout", "agentId": "release-engineer", "mode": "review", "execution": { "scriptPath": "scripts/orchestration/stages/closeout-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/closeout-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-closeout-result.schema.json" }, "inputArtifacts": ["review-report"], "outputArtifacts": ["closeout-report"] }
  ],
  "handoffs": [
    { "fromStage": "intake", "toStage": "missing-stage", "requiredArtifacts": ["normalized-request"] }
  ],
  "completionCriteria": {
    "requiredStages": ["intake", "missing-stage"],
    "requiredArtifacts": ["missing-artifact"]
  }
}"#,
    );

    let result = invoke_validate_agent_orchestration(&ValidateAgentOrchestrationRequest {
        repo_root: Some(repo.path().to_path_buf()),
    })
    .expect("agent orchestration validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Pipeline first stage must be mode 'plan'")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Handoff references unknown toStage")));
}

#[test]
fn test_invoke_validate_agent_orchestration_warns_for_eval_order_divergence() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());
    write_pipeline_manifest(
        repo.path(),
        r#"{
  "id": "default-dev-flow",
  "runtime": {
    "policyCatalogPath": ".github/governance/agent-runtime-policy.catalog.json",
    "modelRoutingCatalogPath": ".github/governance/agent-model-routing.catalog.json"
  },
  "stages": [
    { "id": "intake", "agentId": "super-agent", "mode": "plan", "execution": { "scriptPath": "scripts/orchestration/stages/intake-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/super-agent-intake-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-intake-result.schema.json" }, "inputArtifacts": ["request"], "outputArtifacts": ["normalized-request", "intake-report"] },
    { "id": "spec", "agentId": "brainstormer", "mode": "plan", "execution": { "scriptPath": "scripts/orchestration/stages/spec-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/spec-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-spec-result.schema.json" }, "inputArtifacts": ["request", "normalized-request", "intake-report"], "outputArtifacts": ["spec-summary", "active-spec"] },
    { "id": "plan", "agentId": "planner", "mode": "plan", "execution": { "scriptPath": "scripts/orchestration/stages/plan-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/planner-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-plan-result.schema.json" }, "inputArtifacts": ["request", "normalized-request", "intake-report", "spec-summary", "active-spec"], "outputArtifacts": ["task-plan", "task-plan-data", "context-pack", "active-plan"] },
    { "id": "route", "agentId": "router", "mode": "execute", "execution": { "scriptPath": "scripts/orchestration/stages/route-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/router-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-route-result.schema.json" }, "inputArtifacts": ["task-plan-data", "context-pack", "active-plan"], "outputArtifacts": ["route-selection", "specialist-context-pack"] },
    { "id": "implement", "agentId": "specialist", "mode": "execute", "execution": { "scriptPath": "scripts/orchestration/stages/implement-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/executor-task.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-implementation-result.schema.json" }, "inputArtifacts": ["task-plan", "task-plan-data", "context-pack", "route-selection", "specialist-context-pack", "active-plan"], "outputArtifacts": ["changeset", "implementation-log", "task-review-report"] },
    { "id": "validate", "agentId": "tester", "mode": "validate", "execution": { "scriptPath": "scripts/orchestration/stages/validate-stage.ps1", "dispatchMode": "scripted" }, "inputArtifacts": ["changeset", "implementation-log", "task-review-report"], "outputArtifacts": ["validation-report"] },
    { "id": "review", "agentId": "reviewer", "mode": "review", "execution": { "scriptPath": "scripts/orchestration/stages/review-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/reviewer-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-review-result.schema.json" }, "inputArtifacts": ["changeset", "validation-report", "task-review-report", "active-plan"], "outputArtifacts": ["review-report", "decision-log"] },
    { "id": "closeout", "agentId": "release-engineer", "mode": "review", "execution": { "scriptPath": "scripts/orchestration/stages/closeout-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/closeout-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-closeout-result.schema.json" }, "inputArtifacts": ["changeset", "validation-report", "review-report", "decision-log", "active-plan"], "outputArtifacts": ["closeout-report", "release-summary", "completed-plan"] }
  ],
  "handoffs": [
    { "fromStage": "intake", "toStage": "spec", "requiredArtifacts": ["normalized-request", "intake-report"] },
    { "fromStage": "spec", "toStage": "plan", "requiredArtifacts": ["spec-summary", "active-spec"] },
    { "fromStage": "plan", "toStage": "route", "requiredArtifacts": ["task-plan-data", "context-pack", "active-plan"] },
    { "fromStage": "route", "toStage": "implement", "requiredArtifacts": ["route-selection", "specialist-context-pack"] },
    { "fromStage": "implement", "toStage": "validate", "requiredArtifacts": ["changeset", "implementation-log", "task-review-report"] },
    { "fromStage": "validate", "toStage": "review", "requiredArtifacts": ["validation-report"] },
    { "fromStage": "review", "toStage": "closeout", "requiredArtifacts": ["review-report", "decision-log"] }
  ],
  "completionCriteria": {
    "requiredStages": ["intake", "spec", "plan", "route", "implement", "validate", "review", "closeout"],
    "requiredArtifacts": ["intake-report", "spec-summary", "validation-report", "review-report", "decision-log", "closeout-report", "release-summary"]
  }
}"#,
    );
    crate::support::agent_orchestration_fixtures::write_eval_fixtures(
        repo.path(),
        r#"{
  "version": 1,
  "cases": [
    {
      "id": "feature-implementation",
      "expectedPipelineId": "default-dev-flow",
      "expectedStageOrder": ["plan", "spec", "intake", "route", "implement", "validate", "review", "closeout"],
      "requiredAgents": ["super-agent", "brainstormer", "planner", "router", "specialist", "tester", "reviewer", "release-engineer"]
    }
  ]
}"#,
    );

    let result = invoke_validate_agent_orchestration(&ValidateAgentOrchestrationRequest {
        repo_root: Some(repo.path().to_path_buf()),
    })
    .expect("agent orchestration validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("stage order diverges from pipeline order")));
}