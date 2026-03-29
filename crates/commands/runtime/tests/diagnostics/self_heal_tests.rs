//! Tests for runtime self-heal commands.

use crate::sync::provider_surface_test_support::initialize_minimal_provider_surface_projection;
use nettoolskit_runtime::{
    invoke_runtime_self_heal, RuntimeSelfHealRequest, RuntimeSelfHealStatus,
};
use std::fs;
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn write_runtime_install_profile_catalog(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".github/governance/runtime-install-profiles.json"),
        r#"{"schemaVersion":1,"defaultProfile":"none","profiles":{"none":{"description":"none profile","install":{"bootstrap":false,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":false},"runtime":{"github":false,"codex":false,"claude":false}}}}"#,
    );
}

fn write_validation_profile_catalog(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".github/governance/validation-profiles.json"),
        r#"{"version":1,"defaultProfile":"dev","profiles":[{"id":"dev","warningOnly":false,"checkOrder":["validate-policy"]}]}"#,
    );
}

fn write_repository_policy_catalog(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".github/policies/baseline.policy.json"),
        r#"{
  "id": "repository-baseline",
  "requiredFiles": ["README.md", "scripts/runtime/install.ps1"],
  "requiredDirectories": [".github/policies", ".githooks"],
  "forbiddenFiles": ["forbidden.txt"],
  "requiredGitHooks": ["pre-commit", "post-commit"]
}"#,
    );
}

fn initialize_repo_layout(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
    fs::create_dir_all(repo_root.join(".vscode")).expect("vscode directory should be created");
    fs::create_dir_all(repo_root.join(".githooks")).expect("githooks directory should be created");
    fs::create_dir_all(repo_root.join("scripts/runtime"))
        .expect("runtime directory should be created");
    fs::create_dir_all(repo_root.join("scripts/validation"))
        .expect("validation directory should be created");
    write_runtime_install_profile_catalog(repo_root);
    write_validation_profile_catalog(repo_root);
    write_repository_policy_catalog(repo_root);
    initialize_minimal_provider_surface_projection(repo_root);
    write_file(
        &repo_root.join("README.md"),
        "# Repo\n",
    );
    write_file(
        &repo_root.join("scripts/runtime/install.ps1"),
        "Write-Output 'install'\n",
    );
    write_file(
        &repo_root.join(".githooks/pre-commit"),
        "#!/bin/sh\n",
    );
    write_file(
        &repo_root.join(".githooks/post-commit"),
        "#!/bin/sh\n",
    );
}

fn runtime_request(repo_root: &std::path::Path) -> RuntimeSelfHealRequest {
    RuntimeSelfHealRequest {
        repo_root: Some(repo_root.to_path_buf()),
        target_github_path: Some(repo_root.join(".runtime/github")),
        target_codex_path: Some(repo_root.join(".runtime/codex")),
        target_agents_skills_path: Some(repo_root.join(".runtime/agents-skills")),
        target_copilot_skills_path: Some(repo_root.join(".runtime/copilot-skills")),
        runtime_profile: Some("none".to_string()),
        ..RuntimeSelfHealRequest::default()
    }
}

#[test]
fn test_invoke_runtime_self_heal_writes_report_and_log_for_passed_run() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());

    let result = invoke_runtime_self_heal(&RuntimeSelfHealRequest {
        output_path: Some(repo.path().join(".temp/custom-self-heal.json")),
        log_path: Some(repo.path().join(".temp/logs/custom-self-heal.log")),
        ..runtime_request(repo.path())
    })
    .expect("self-heal should execute");

    assert_eq!(result.overall_status, RuntimeSelfHealStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.total_steps, 2);
    assert_eq!(result.passed_steps, 2);
    assert_eq!(result.failed_steps, 0);
    assert!(result.output_path.is_file());
    assert!(result.log_path.is_file());
    assert!(result.report_json.contains("\"overallStatus\": \"passed\""));
    let persisted =
        fs::read_to_string(&result.output_path).expect("persisted report should be readable");
    assert_eq!(persisted, result.report_json);
    assert_eq!(result.steps[0].name, "runtime-bootstrap");
    assert_eq!(result.steps[1].name, "healthcheck");
    assert_eq!(
        result.steps[1].script,
        "rust:nettoolskit-runtime::healthcheck"
    );
}

#[test]
fn test_invoke_runtime_self_heal_applies_vscode_templates_when_requested() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(
        &repo.path().join(".vscode/settings.tamplate.jsonc"),
        "{\n  \"editor.tabSize\": 4\n}",
    );
    write_file(
        &repo.path().join(".vscode/mcp.tamplate.jsonc"),
        "{\n  \"servers\": {}\n}",
    );

    let result = invoke_runtime_self_heal(&RuntimeSelfHealRequest {
        apply_vscode_templates: true,
        ..runtime_request(repo.path())
    })
    .expect("self-heal should execute");

    assert_eq!(result.overall_status, RuntimeSelfHealStatus::Passed);
    assert_eq!(result.total_steps, 3);
    assert!(repo.path().join(".vscode/settings.json").is_file());
    assert!(repo.path().join(".vscode/mcp.json").is_file());
    let vscode_step = result
        .steps
        .iter()
        .find(|step| step.name == "apply-vscode-templates")
        .expect("vscode step should be present");
    assert_eq!(vscode_step.status, RuntimeSelfHealStatus::Passed);
    assert_eq!(
        vscode_step.script,
        "rust:nettoolskit-runtime::apply-vscode-templates"
    );
}

#[test]
fn test_invoke_runtime_self_heal_fails_when_vscode_template_files_are_missing() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());

    let result = invoke_runtime_self_heal(&RuntimeSelfHealRequest {
        apply_vscode_templates: true,
        ..runtime_request(repo.path())
    })
    .expect("self-heal should execute");

    assert_eq!(result.overall_status, RuntimeSelfHealStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert_eq!(result.total_steps, 3);
    assert_eq!(result.failed_steps, 1);
    let vscode_step = result
        .steps
        .iter()
        .find(|step| step.name == "apply-vscode-templates")
        .expect("vscode step should be present");
    assert_eq!(vscode_step.status, RuntimeSelfHealStatus::Failed);
    assert!(vscode_step
        .error
        .as_deref()
        .expect("missing template error should be reported")
        .contains("failed to apply runtime vscode templates"));
}
