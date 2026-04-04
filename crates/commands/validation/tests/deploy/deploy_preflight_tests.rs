//! Tests for deploy::deploy_preflight module.

use nettoolskit_validation::{
    invoke_validate_deploy_preflight, ValidateDeployPreflightRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::deploy_fixtures::{
    initialize_deploy_preflight_repo, write_file, write_valid_deploy_layout,
};

#[test]
fn test_invoke_validate_deploy_preflight_passes_when_no_assets_are_detected() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_deploy_preflight_repo(repo.path());

    let result = invoke_validate_deploy_preflight(&ValidateDeployPreflightRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateDeployPreflightRequest::default()
    })
    .expect("deploy preflight should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert!(!result.deploy_assets_detected);
    assert!(result.warnings.is_empty());
    assert!(result.failures.is_empty());
}

#[test]
fn test_invoke_validate_deploy_preflight_passes_for_valid_layout() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_deploy_preflight_repo(repo.path());
    write_valid_deploy_layout(repo.path());

    let result = invoke_validate_deploy_preflight(&ValidateDeployPreflightRequest {
        repo_root: Some(repo.path().to_path_buf()),
        image_name: Some("sample/api".to_string()),
        image_tag: Some("latest".to_string()),
        api_port: Some(5000),
        api_port_https: Some(5001),
        seq_port: Some(8082),
        warning_only: false,
        ..ValidateDeployPreflightRequest::default()
    })
    .expect("deploy preflight should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert!(result.deploy_assets_detected);
    assert!(result.deploy_directory.is_some());
    assert!(result.deploy_compose_file.is_some());
    assert!(result.dockerfile_path.is_some());
    assert!(result.warnings.is_empty());
    assert!(result.failures.is_empty());
}

#[test]
fn test_invoke_validate_deploy_preflight_reports_missing_assets_and_invalid_image_settings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_deploy_preflight_repo(repo.path());
    write_file(
        &repo.path().join("docker/docker-compose.deploy.yaml"),
        "version: '3.9'\n",
    );

    let result = invoke_validate_deploy_preflight(&ValidateDeployPreflightRequest {
        repo_root: Some(repo.path().to_path_buf()),
        dockerfile_path: Some("src/API/Missing.Dockerfile".into()),
        image_name: Some("Invalid Image".to_string()),
        image_tag: Some("bad tag".to_string()),
        require_env_file: true,
        warning_only: false,
        ..ValidateDeployPreflightRequest::default()
    })
    .expect("deploy preflight should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result.deploy_assets_detected);
    assert!(result
        .failures
        .iter()
        .any(|message| { message.contains("does not declare a services block") }));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Missing Dockerfile")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Invalid Docker image name")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Invalid Docker image tag")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Optional deploy environment file not found")));
}