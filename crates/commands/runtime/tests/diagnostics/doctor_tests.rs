//! Tests for runtime doctor commands.

use crate::sync::provider_surface_test_support::initialize_minimal_provider_surface_projection;
use nettoolskit_runtime::{
    invoke_runtime_doctor, RuntimeDoctorCommandError, RuntimeDoctorRequest, RuntimeDoctorStatus,
};
use std::error::Error;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn write_runtime_install_profile_catalog(repo_root: &Path) {
    write_file(
        &repo_root.join(".github/governance/runtime-install-profiles.json"),
        r#"{"schemaVersion":1,"defaultProfile":"none","profiles":{"none":{"description":"none profile","install":{"bootstrap":false,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":false},"runtime":{"github":false,"codex":false,"claude":false}},"github":{"description":"github profile","install":{"bootstrap":true,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":true},"runtime":{"github":true,"codex":false,"claude":false}},"codex":{"description":"codex profile","install":{"bootstrap":true,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":true},"runtime":{"github":false,"codex":true,"claude":false}},"all":{"description":"all profile","install":{"bootstrap":true,"globalVscodeSettings":true,"globalVscodeSnippets":true,"localGitHooks":true,"globalGitAliases":true,"healthcheck":true},"runtime":{"github":true,"codex":true,"claude":true}}}}"#,
    );
}

fn initialize_repo_layout(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
    fs::create_dir_all(repo_root.join("scripts")).expect("scripts directory should be created");
    write_runtime_install_profile_catalog(repo_root);
}

fn mirror_file(source: &Path, destination: &Path) {
    let contents = fs::read_to_string(source).expect("source file should be readable");
    write_file(destination, &contents);
}

fn initialize_clean_codex_runtime(repo_root: &Path, runtime_root: &Path) {
    write_file(
        &repo_root.join(".codex/skills/runtime-skill/SKILL.md"),
        "# runtime-skill",
    );
    write_file(&repo_root.join(".codex/skills/README.md"), "# ignored");
    write_file(&repo_root.join(".codex/mcp/catalog.json"), "{}");
    write_file(
        &repo_root.join(".codex/scripts/root-tool.ps1"),
        "Write-Output 'tool'",
    );
    write_file(
        &repo_root.join(".codex/orchestration/flow.md"),
        "# orchestration flow",
    );
    write_file(
        &repo_root.join("scripts/common/common-bootstrap.ps1"),
        "Write-Output 'common'",
    );
    write_file(
        &repo_root.join("scripts/security/audit.ps1"),
        "Write-Output 'security'",
    );
    write_file(
        &repo_root.join("scripts/maintenance/cleanup.ps1"),
        "Write-Output 'maintenance'",
    );

    mirror_file(
        &repo_root.join(".codex/skills/runtime-skill/SKILL.md"),
        &runtime_root.join("agents-skills/runtime-skill/SKILL.md"),
    );
    mirror_file(
        &repo_root.join(".codex/mcp/catalog.json"),
        &runtime_root.join("codex/shared-mcp/catalog.json"),
    );
    mirror_file(
        &repo_root.join(".codex/scripts/root-tool.ps1"),
        &runtime_root.join("codex/shared-scripts/root-tool.ps1"),
    );
    mirror_file(
        &repo_root.join("scripts/common/common-bootstrap.ps1"),
        &runtime_root.join("codex/shared-scripts/common/common-bootstrap.ps1"),
    );
    mirror_file(
        &repo_root.join("scripts/security/audit.ps1"),
        &runtime_root.join("codex/shared-scripts/security/audit.ps1"),
    );
    mirror_file(
        &repo_root.join("scripts/maintenance/cleanup.ps1"),
        &runtime_root.join("codex/shared-scripts/maintenance/cleanup.ps1"),
    );
    mirror_file(
        &repo_root.join(".codex/orchestration/flow.md"),
        &runtime_root.join("codex/shared-orchestration/flow.md"),
    );
}

#[test]
fn test_invoke_runtime_doctor_detects_missing_runtime_files_for_github_profile() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(&repo.path().join(".github/AGENTS.md"), "# Agents");
    write_file(
        &repo.path().join("scripts/runtime/bootstrap.ps1"),
        "Write-Output 'bootstrap'",
    );
    write_file(
        &repo
            .path()
            .join(".github/skills/using-super-agent/SKILL.md"),
        "# legacy starter",
    );

    let result = invoke_runtime_doctor(&RuntimeDoctorRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_github_path: Some(repo.path().join(".runtime/github")),
        target_copilot_skills_path: Some(repo.path().join(".runtime/copilot-skills")),
        runtime_profile: Some("github".to_string()),
        ..RuntimeDoctorRequest::default()
    })
    .expect("doctor should execute");

    assert_eq!(result.runtime_profile_name, "github");
    assert_eq!(result.mappings_checked, 3);
    assert!(result.has_drift);
    assert!(!result.has_extras);
    assert_eq!(result.status, RuntimeDoctorStatus::Detected);
    assert!(result
        .reports
        .iter()
        .any(|report| report.name == ".github -> runtime"
            && report
                .missing_in_runtime
                .iter()
                .any(|path| path == "AGENTS.md")));
}

#[test]
fn test_invoke_runtime_doctor_reports_clean_with_extras_when_strict_extras_disabled() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    let runtime_root = repo.path().join(".runtime");
    initialize_clean_codex_runtime(repo.path(), &runtime_root);
    write_file(
        &runtime_root.join("codex/shared-scripts/common/extra-local.ps1"),
        "Write-Output 'extra'",
    );

    let result = invoke_runtime_doctor(&RuntimeDoctorRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_codex_path: Some(runtime_root.join("codex")),
        target_agents_skills_path: Some(runtime_root.join("agents-skills")),
        runtime_profile: Some("codex".to_string()),
        strict_extras: false,
        ..RuntimeDoctorRequest::default()
    })
    .expect("doctor should execute");

    assert!(!result.has_drift);
    assert!(result.has_extras);
    assert_eq!(result.status, RuntimeDoctorStatus::CleanWithExtras);
    let common_report = result
        .reports
        .iter()
        .find(|report| report.name == "scripts/common -> runtime")
        .expect("common mapping should exist");
    assert_eq!(
        common_report.extra_in_runtime,
        vec!["extra-local.ps1".to_string()]
    );
    assert!(common_report.is_healthy);
}

#[test]
fn test_invoke_runtime_doctor_treats_extras_as_drift_when_strict_extras_enabled() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    let runtime_root = repo.path().join(".runtime");
    initialize_clean_codex_runtime(repo.path(), &runtime_root);
    write_file(
        &runtime_root.join("codex/shared-scripts/common/extra-local.ps1"),
        "Write-Output 'extra'",
    );

    let result = invoke_runtime_doctor(&RuntimeDoctorRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_codex_path: Some(runtime_root.join("codex")),
        target_agents_skills_path: Some(runtime_root.join("agents-skills")),
        runtime_profile: Some("codex".to_string()),
        strict_extras: true,
        ..RuntimeDoctorRequest::default()
    })
    .expect("doctor should execute");

    assert!(result.has_drift);
    assert!(result.has_extras);
    assert_eq!(result.status, RuntimeDoctorStatus::Detected);
    let common_report = result
        .reports
        .iter()
        .find(|report| report.name == "scripts/common -> runtime")
        .expect("common mapping should exist");
    assert!(!common_report.is_healthy);
}

#[test]
fn test_invoke_runtime_doctor_reports_codex_skill_duplicates() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    let runtime_root = repo.path().join(".runtime");
    initialize_clean_codex_runtime(repo.path(), &runtime_root);
    write_file(
        &runtime_root.join("codex/skills/runtime-skill/SKILL.md"),
        "# duplicate runtime skill",
    );

    let result = invoke_runtime_doctor(&RuntimeDoctorRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_codex_path: Some(runtime_root.join("codex")),
        target_agents_skills_path: Some(runtime_root.join("agents-skills")),
        runtime_profile: Some("codex".to_string()),
        ..RuntimeDoctorRequest::default()
    })
    .expect("doctor should execute");

    assert!(result.has_drift);
    let duplicate_report = result
        .reports
        .iter()
        .find(|report| report.name == "repo-managed skill duplicates in runtime .codex/skills")
        .expect("duplicate report should exist");
    assert_eq!(
        duplicate_report.extra_in_runtime,
        vec!["runtime-skill".to_string()]
    );
    assert!(!duplicate_report.is_healthy);
}

#[test]
fn test_invoke_runtime_doctor_syncs_missing_runtime_files_when_requested() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(&repo.path().join(".github/AGENTS.md"), "# Agents");
    initialize_minimal_provider_surface_projection(repo.path());

    let target_github_path = repo.path().join(".runtime/github");
    let result = invoke_runtime_doctor(&RuntimeDoctorRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_github_path: Some(target_github_path.clone()),
        target_copilot_skills_path: Some(repo.path().join(".runtime/copilot-skills")),
        runtime_profile: Some("github".to_string()),
        sync_on_drift: true,
        ..RuntimeDoctorRequest::default()
    })
    .expect("doctor should execute");

    assert!(!result.has_drift);
    assert!(!result.has_extras);
    assert_eq!(result.status, RuntimeDoctorStatus::Clean);
    assert!(result.sync_attempted);
    assert!(result.sync_resolved_drift);
    assert!(target_github_path.join("AGENTS.md").is_file());
    assert!(repo
        .path()
        .join(".github/prompts/route-instructions.prompt.md")
        .is_file());
}

#[test]
fn test_invoke_runtime_doctor_returns_sync_error_when_remediation_fails() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_file(&repo.path().join(".github/AGENTS.md"), "# Agents");

    let error = invoke_runtime_doctor(&RuntimeDoctorRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_github_path: Some(repo.path().join(".runtime/github")),
        target_copilot_skills_path: Some(repo.path().join(".runtime/copilot-skills")),
        runtime_profile: Some("github".to_string()),
        sync_on_drift: true,
        ..RuntimeDoctorRequest::default()
    })
    .expect_err("doctor remediation should fail without provider surface catalog");

    assert!(matches!(
        error,
        RuntimeDoctorCommandError::SynchronizeRuntime { .. }
    ));
    assert_eq!(
        error.to_string(),
        "failed to synchronize runtime doctor drift remediation"
    );
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "failed to render runtime bootstrap provider surfaces"
    );
}

#[test]
fn test_invoke_runtime_doctor_returns_clean_when_runtime_profile_disables_surfaces() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());

    let result = invoke_runtime_doctor(&RuntimeDoctorRequest {
        repo_root: Some(repo.path().to_path_buf()),
        runtime_profile: Some("none".to_string()),
        ..RuntimeDoctorRequest::default()
    })
    .expect("doctor should execute");

    assert_eq!(result.runtime_profile_name, "none");
    assert_eq!(result.mappings_checked, 0);
    assert!(!result.has_drift);
    assert!(!result.has_extras);
    assert_eq!(result.status, RuntimeDoctorStatus::Clean);
}
