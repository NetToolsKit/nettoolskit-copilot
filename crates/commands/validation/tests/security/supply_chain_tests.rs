//! Tests for security::supply_chain module.

use nettoolskit_validation::{
    invoke_validate_supply_chain, ValidateSupplyChainRequest, ValidationCheckStatus,
};
use tempfile::TempDir;

use crate::support::security_fixtures::{
    initialize_supply_chain_repo, write_repo_file, write_supply_chain_baseline,
};

#[test]
fn test_invoke_validate_supply_chain_passes_for_valid_manifests() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_supply_chain_repo(repo.path());

    let result = invoke_validate_supply_chain(&ValidateSupplyChainRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSupplyChainRequest::default()
    })
    .expect("supply chain validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.dependency_manifests, 4);
    assert_eq!(result.packages_discovered, 5);
    assert!(result
        .sbom_path
        .as_ref()
        .expect("sbom path should be present")
        .is_file());
}

#[test]
fn test_invoke_validate_supply_chain_reports_blocked_dependency() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_supply_chain_repo(repo.path());
    write_repo_file(
        repo.path(),
        "package.json",
        r#"{
  "dependencies": {
    "event-stream": "^3.3.6"
  }
}"#,
    );

    let result = invoke_validate_supply_chain(&ValidateSupplyChainRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSupplyChainRequest::default()
    })
    .expect("supply chain validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Blocked dependency detected: event-stream (npm) in package.json")));
}

#[test]
fn test_invoke_validate_supply_chain_warns_for_invalid_package_json() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_supply_chain_repo(repo.path());
    write_repo_file(repo.path(), "package.json", "{ invalid json ");

    let result = invoke_validate_supply_chain(&ValidateSupplyChainRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSupplyChainRequest::default()
    })
    .expect("supply chain validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Skipping invalid package.json parse: package.json")));
}

#[test]
fn test_invoke_validate_supply_chain_reports_missing_required_license_evidence() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_supply_chain_repo(repo.path());
    write_supply_chain_baseline(
        repo.path(),
        r#"{
  "version": 1,
  "sbomOutputPath": ".temp/audit/sbom.latest.json",
  "licenseEvidencePath": ".temp/audit/licenses.latest.json",
  "requireLicenseEvidence": true,
  "warnOnMissingLicenseEvidence": false,
  "warnOnEmptyDependencySet": false,
  "excludedPathGlobs": [".temp/**"],
  "blockedDependencyPatterns": [],
  "sensitiveDependencyPatterns": []
}"#,
    );

    let result = invoke_validate_supply_chain(&ValidateSupplyChainRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateSupplyChainRequest::default()
    })
    .expect("supply chain validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert!(result.failures.iter().any(|message| {
        message.contains("License evidence file is required but missing: .temp/audit/licenses.latest.json")
    }));
}

#[test]
fn test_invoke_validate_supply_chain_converts_required_findings_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_supply_chain_repo(repo.path());
    write_repo_file(
        repo.path(),
        "package.json",
        r#"{
  "dependencies": {
    "event-stream": "^3.3.6"
  }
}"#,
    );

    let result = invoke_validate_supply_chain(&ValidateSupplyChainRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateSupplyChainRequest::default()
    })
    .expect("supply chain validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Blocked dependency detected: event-stream (npm) in package.json")));
}