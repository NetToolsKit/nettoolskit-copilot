//! Tests for runtime healthcheck commands.

use crate::sync::provider_surface_test_support::initialize_minimal_provider_surface_projection;
use nettoolskit_runtime::{
    invoke_runtime_healthcheck, RuntimeHealthcheckRequest, RuntimeHealthcheckStatus,
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
        r#"{"schemaVersion":1,"defaultProfile":"none","profiles":{"none":{"description":"none profile","install":{"bootstrap":false,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":false},"runtime":{"github":false,"codex":false,"claude":false}},"github":{"description":"github profile","install":{"bootstrap":true,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":true},"runtime":{"github":true,"codex":false,"claude":false}}}}"#,
    );
}

fn write_validation_profile_catalog(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".github/governance/validation-profiles.json"),
        r#"{"version":1,"defaultProfile":"dev","profiles":[{"id":"dev","warningOnly":false,"checkOrder":["validate-planning-structure"]}]}"#,
    );
}

fn initialize_repo_layout(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
    fs::create_dir_all(repo_root.join("scripts/runtime"))
        .expect("runtime directory should be created");
    fs::create_dir_all(repo_root.join("scripts/validation"))
        .expect("validation directory should be created");
    write_file(&repo_root.join("planning/README.md"), "# planning\n");
    write_file(&repo_root.join("planning/specs/README.md"), "# specs\n");
    write_runtime_install_profile_catalog(repo_root);
    write_validation_profile_catalog(repo_root);
    initialize_minimal_provider_surface_projection(repo_root);
}

#[test]
fn test_invoke_runtime_healthcheck_writes_report_and_log_for_passed_run() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());

    let result = invoke_runtime_healthcheck(&RuntimeHealthcheckRequest {
        repo_root: Some(repo.path().to_path_buf()),
        runtime_profile: Some("none".to_string()),
        output_path: Some(repo.path().join(".temp/custom-healthcheck.json")),
        log_path: Some(repo.path().join(".temp/logs/custom-healthcheck.log")),
        ..RuntimeHealthcheckRequest::default()
    })
    .expect("healthcheck should execute");

    assert_eq!(result.overall_status, RuntimeHealthcheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.total_checks, 2);
    assert!(result.output_path.is_file());
    assert!(result.log_path.is_file());
    assert!(result.report_json.contains("\"overallStatus\": \"passed\""));
    let persisted =
        fs::read_to_string(&result.output_path).expect("persisted report should be readable");
    assert_eq!(persisted, result.report_json);
}

#[test]
fn test_invoke_runtime_healthcheck_converts_runtime_drift_to_warning_when_configured() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(&repo.path().join(".github/AGENTS.md"), "# Agents");

    let result = invoke_runtime_healthcheck(&RuntimeHealthcheckRequest {
        repo_root: Some(repo.path().to_path_buf()),
        runtime_profile: Some("github".to_string()),
        target_github_path: Some(repo.path().join(".runtime/github")),
        target_copilot_skills_path: Some(repo.path().join(".runtime/copilot-skills")),
        warning_only: false,
        treat_runtime_drift_as_warning: true,
        ..RuntimeHealthcheckRequest::default()
    })
    .expect("healthcheck should execute");

    assert_eq!(result.overall_status, RuntimeHealthcheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.warning_checks, 1);
    let doctor_check = result
        .checks
        .iter()
        .find(|check| check.name == "runtime-doctor")
        .expect("doctor check should exist");
    assert_eq!(doctor_check.status, RuntimeHealthcheckStatus::Warning);
    assert_eq!(doctor_check.exit_code, 1);
}

#[test]
fn test_invoke_runtime_healthcheck_fails_when_runtime_drift_warning_override_is_disabled() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(&repo.path().join(".github/AGENTS.md"), "# Agents");

    let result = invoke_runtime_healthcheck(&RuntimeHealthcheckRequest {
        repo_root: Some(repo.path().to_path_buf()),
        runtime_profile: Some("github".to_string()),
        target_github_path: Some(repo.path().join(".runtime/github")),
        target_copilot_skills_path: Some(repo.path().join(".runtime/copilot-skills")),
        warning_only: false,
        treat_runtime_drift_as_warning: false,
        ..RuntimeHealthcheckRequest::default()
    })
    .expect("healthcheck should execute");

    assert_eq!(result.overall_status, RuntimeHealthcheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert_eq!(result.failed_checks, 1);
}

#[test]
fn test_invoke_runtime_healthcheck_sync_runtime_uses_rust_bootstrap_path() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(&repo.path().join(".github/AGENTS.md"), "# Agents");
    let result = invoke_runtime_healthcheck(&RuntimeHealthcheckRequest {
        repo_root: Some(repo.path().to_path_buf()),
        runtime_profile: Some("github".to_string()),
        target_github_path: Some(repo.path().join(".runtime/github")),
        target_copilot_skills_path: Some(repo.path().join(".runtime/copilot-skills")),
        sync_runtime: true,
        warning_only: false,
        treat_runtime_drift_as_warning: false,
        ..RuntimeHealthcheckRequest::default()
    })
    .expect("healthcheck should execute");

    assert_eq!(result.overall_status, RuntimeHealthcheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.total_checks, 3);
    assert!(result
        .checks
        .iter()
        .any(|check| check.name == "runtime-bootstrap"
            && check.script == "rust:nettoolskit-runtime::bootstrap"));
}