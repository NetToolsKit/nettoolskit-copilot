//! Tests for agent_orchestration::agent_hooks module.

use nettoolskit_validation::{
    invoke_validate_agent_hooks, ValidateAgentHooksRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::agent_orchestration_fixtures::{
    initialize_agent_hooks_repo, write_agent_hooks_bootstrap, write_agent_hooks_common_script,
    write_agent_hooks_script, write_agent_hooks_selector,
};

#[test]
fn test_invoke_validate_agent_hooks_passes_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_hooks_repo(repo.path());

    let result = invoke_validate_agent_hooks(&ValidateAgentHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAgentHooksRequest::default()
    })
    .expect("agent hook validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
}

#[test]
fn test_invoke_validate_agent_hooks_reports_missing_bootstrap_event() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_hooks_repo(repo.path());
    write_agent_hooks_bootstrap(
        repo.path(),
        r#"{
  "hooks": {
    "SessionStart": [{ "type": "command", "command": "pwsh -File session-start.ps1" }],
    "PreToolUse": [{ "type": "command", "command": "pwsh -File pre-tool-use.ps1" }]
  }
}"#,
    );

    let result = invoke_validate_agent_hooks(&ValidateAgentHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAgentHooksRequest::default()
    })
    .expect("agent hook validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| { message.contains("Hook event 'SubagentStart' is missing") }));
}

#[test]
fn test_invoke_validate_agent_hooks_reports_invalid_selector() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_hooks_repo(repo.path());
    write_agent_hooks_selector(
        repo.path(),
        r#"{
  "version": 1,
  "defaultAgent": {
    "skillName": "",
    "displayName": "Super Agent"
  },
  "overrideSources": {
    "environment": {
      "skillVariable": "COPILOT_SUPER_AGENT_SKILL",
      "displayVariable": ""
    },
    "localOverrideFile": ""
  }
}"#,
    );

    let result = invoke_validate_agent_hooks(&ValidateAgentHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAgentHooksRequest::default()
    })
    .expect("agent hook validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result.failures.iter().any(|message| {
        message.contains("super-agent.selector.json must define defaultAgent.skillName")
    }));
}

#[test]
fn test_invoke_validate_agent_hooks_reports_missing_common_markers() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_hooks_repo(repo.path());
    write_agent_hooks_common_script(repo.path(), "Write-Output 'hook'\n");

    let result = invoke_validate_agent_hooks(&ValidateAgentHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAgentHooksRequest::default()
    })
    .expect("agent hook validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Hook helper contract missing required marker")));
}

#[test]
fn test_invoke_validate_agent_hooks_converts_failures_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_agent_hooks_repo(repo.path());
    write_agent_hooks_script(repo.path(), "subagent-start.ps1", "");

    let result = invoke_validate_agent_hooks(&ValidateAgentHooksRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateAgentHooksRequest::default()
    })
    .expect("agent hook validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Referenced hook script missing")));
}
