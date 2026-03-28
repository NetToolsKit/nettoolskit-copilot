//! Tests for agent_orchestration::agent_permissions module.

use nettoolskit_validation::{
    invoke_validate_agent_permissions, ValidateAgentPermissionsRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::agent_orchestration_fixtures::{
    initialize_agent_contract_repo, valid_permission_matrix_json, write_agents_manifest,
    write_permission_matrix,
};

#[test]
fn test_invoke_validate_agent_permissions_passes_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());

    let result = invoke_validate_agent_permissions(&ValidateAgentPermissionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAgentPermissionsRequest::default()
    })
    .expect("agent permission validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.agents_checked, 8);
    assert_eq!(result.stage_checks, 8);
}

#[test]
fn test_invoke_validate_agent_permissions_reports_matrix_mismatch() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());
    write_agents_manifest(
        repo.path(),
        r#"{
  "version": 1,
  "agents": [
    {
      "id": "super-agent",
      "role": "planner",
      "skill": "super-agent",
      "allowedPaths": [".github/**", ".codex/**", "planning/**", "scripts/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"],
      "blockedCommands": ["git checkout --"],
      "budget": { "maxSteps": 16, "maxDurationMinutes": 15, "maxFileEdits": 8, "maxTokens": 45000 }
    }
  ]
}"#,
    );

    let result = invoke_validate_agent_permissions(&ValidateAgentPermissionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAgentPermissionsRequest::default()
    })
    .expect("agent permission validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("missing required blocked command")));
}

#[test]
fn test_invoke_validate_agent_permissions_warns_for_matrix_only_agent() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());
    write_permission_matrix(
        repo.path(),
        r#"{
  "version": 1,
  "globalRules": {
    "requiredBlockedCommandPrefixes": ["git reset --hard", "git checkout --"],
    "allowedStageScriptPrefixes": ["scripts/orchestration/stages/"]
  },
  "agents": [
    { "agentId": "super-agent", "role": "planner", "skill": "super-agent", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/intake-stage.ps1"], "requiredBudget": { "maxSteps": 16, "maxDurationMinutes": 15, "maxFileEdits": 8, "maxTokens": 45000 } },
    { "agentId": "brainstormer", "role": "planner", "skill": "brainstorm-spec-architect", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/spec-stage.ps1"], "requiredBudget": { "maxSteps": 18, "maxDurationMinutes": 20, "maxFileEdits": 10, "maxTokens": 55000 } },
    { "agentId": "planner", "role": "planner", "skill": "plan-active-work-planner", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/plan-stage.ps1"], "requiredBudget": { "maxSteps": 20, "maxDurationMinutes": 20, "maxFileEdits": 12, "maxTokens": 70000 } },
    { "agentId": "router", "role": "router", "skill": "context-token-optimizer", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/route-stage.ps1"], "requiredBudget": { "maxSteps": 15, "maxDurationMinutes": 15, "maxFileEdits": 8, "maxTokens": 50000 } },
    { "agentId": "specialist", "role": "specialist", "skill": "dev-software-engineer", "allowedPathGlobs": ["src/**", "modules/**", "samples/**", "tests/**", "scripts/**", ".github/**", ".codex/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/implement-stage.ps1"], "requiredBudget": { "maxSteps": 45, "maxDurationMinutes": 50, "maxFileEdits": 40, "maxTokens": 180000 } },
    { "agentId": "tester", "role": "tester", "skill": "test-engineer", "allowedPathGlobs": ["tests/**", "src/**", "modules/**", "samples/**", "scripts/**", ".github/**", ".temp/**"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/validate-stage.ps1"], "requiredBudget": { "maxSteps": 25, "maxDurationMinutes": 30, "maxFileEdits": 20, "maxTokens": 120000 } },
    { "agentId": "reviewer", "role": "reviewer", "skill": "review-code-engineer", "allowedPathGlobs": ["src/**", "modules/**", "samples/**", "planning/**", "scripts/**", ".github/**", ".codex/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/review-stage.ps1"], "requiredBudget": { "maxSteps": 20, "maxDurationMinutes": 25, "maxFileEdits": 10, "maxTokens": 90000 } },
    { "agentId": "release-engineer", "role": "release", "skill": "release-closeout-engineer", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/closeout-stage.ps1"], "requiredBudget": { "maxSteps": 20, "maxDurationMinutes": 20, "maxFileEdits": 15, "maxTokens": 80000 } },
    { "agentId": "extra-agent", "role": "planner", "skill": "super-agent", "allowedPathGlobs": [".github/**"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/intake-stage.ps1"], "requiredBudget": { "maxSteps": 1, "maxDurationMinutes": 1, "maxFileEdits": 1, "maxTokens": 1 } }
  ]
}"#,
    );

    let result = invoke_validate_agent_permissions(&ValidateAgentPermissionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAgentPermissionsRequest::default()
    })
    .expect("agent permission validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Matrix has agent not present in manifest")));
}

#[test]
fn test_invoke_validate_agent_permissions_converts_failures_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());
    write_permission_matrix(
        repo.path(),
        valid_permission_matrix_json()
            .replace("\"role\": \"planner\"", "\"role\": \"router\"")
            .as_str(),
    );

    let result = invoke_validate_agent_permissions(&ValidateAgentPermissionsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateAgentPermissionsRequest::default()
    })
    .expect("agent permission validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Role mismatch for agent")));
}
