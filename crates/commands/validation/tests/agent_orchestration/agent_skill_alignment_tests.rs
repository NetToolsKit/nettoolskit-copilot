//! Tests for agent_orchestration::agent_skill_alignment module.

use nettoolskit_validation::{
    invoke_validate_agent_skill_alignment, ValidateAgentSkillAlignmentRequest,
    ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::agent_orchestration_fixtures::{
    initialize_agent_contract_repo, remove_repo_path, write_agent_skill_markdown,
    write_eval_fixtures,
};

#[test]
fn test_invoke_validate_agent_skill_alignment_passes_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());

    let result = invoke_validate_agent_skill_alignment(&ValidateAgentSkillAlignmentRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidateAgentSkillAlignmentRequest::default()
    })
    .expect("agent skill alignment should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.agents_checked, 8);
    assert_eq!(result.stage_checks, 8);
    assert_eq!(result.eval_case_checks, 1);
}

#[test]
fn test_invoke_validate_agent_skill_alignment_reports_missing_skill_folder() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());
    remove_repo_path(repo.path(), ".codex/skills/test-engineer");

    let result = invoke_validate_agent_skill_alignment(&ValidateAgentSkillAlignmentRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidateAgentSkillAlignmentRequest::default()
    })
    .expect("agent skill alignment should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("references missing skill folder")));
}

#[test]
fn test_invoke_validate_agent_skill_alignment_warns_for_missing_instruction_reference() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());
    write_agent_skill_markdown(
        repo.path(),
        "super-agent",
        "---\nname: super-agent\n---\nReference .github/AGENTS.md\nReference .github/copilot-instructions.md\nReference .github/instruction-routing.catalog.yml\n",
    );

    let result = invoke_validate_agent_skill_alignment(&ValidateAgentSkillAlignmentRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidateAgentSkillAlignmentRequest::default()
    })
    .expect("agent skill alignment should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("has no explicit .github/instructions reference")));
}

#[test]
fn test_invoke_validate_agent_skill_alignment_reports_unknown_eval_agent() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_contract_repo(repo.path());
    write_eval_fixtures(
        repo.path(),
        r#"{
  "version": 1,
  "cases": [
    {
      "id": "feature-implementation",
      "expectedPipelineId": "default-dev-flow",
      "expectedStageOrder": ["intake", "spec", "plan", "route", "implement", "validate", "review", "closeout"],
      "requiredAgents": ["missing-agent"]
    }
  ]
}"#,
    );

    let result = invoke_validate_agent_skill_alignment(&ValidateAgentSkillAlignmentRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..ValidateAgentSkillAlignmentRequest::default()
    })
    .expect("agent skill alignment should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("references unknown required agent")));
}
