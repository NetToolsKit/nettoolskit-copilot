//! Tests for validation surface errors.

use nettoolskit_validation::{
    invoke_validate_all, invoke_validate_audit_ledger, invoke_validate_authoritative_source_policy,
    invoke_validate_instruction_architecture, invoke_validate_instruction_metadata,
    invoke_validate_instructions, invoke_validate_planning_structure,
    invoke_validate_readme_standards,
    invoke_validate_warning_baseline,
    invoke_validate_routing_coverage, invoke_validate_template_standards,
    invoke_validate_workspace_efficiency, require_validation_surface_contract, ValidateAllRequest,
    ValidateAuditLedgerRequest, ValidateAuthoritativeSourcePolicyRequest,
    ValidateInstructionArchitectureRequest, ValidateInstructionMetadataRequest,
    ValidateInstructionsRequest, ValidatePlanningStructureRequest, ValidateReadmeStandardsRequest,
    ValidateRoutingCoverageRequest, ValidateTemplateStandardsRequest,
    ValidateWarningBaselineRequest,
    ValidateWorkspaceEfficiencyRequest,
};

#[test]
fn test_validation_surface_error_mentions_missing_surface_id() {
    let error = require_validation_surface_contract("missing-validation")
        .expect_err("unknown validation surface should fail");

    assert_eq!(
        error.to_string(),
        "unknown validation surface contract: missing-validation"
    );
}

#[test]
fn test_validate_all_error_display_is_stable() {
    let error = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateAllRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve validation workspace root"
    );
}

#[test]
fn test_validate_planning_structure_error_display_is_stable() {
    let error = invoke_validate_planning_structure(&ValidatePlanningStructureRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidatePlanningStructureRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve planning structure workspace root"
    );
}

#[test]
fn test_validate_audit_ledger_error_display_is_stable() {
    let error = invoke_validate_audit_ledger(&ValidateAuditLedgerRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateAuditLedgerRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve audit ledger workspace root"
    );
}

#[test]
fn test_validate_authoritative_source_policy_error_display_is_stable() {
    let error =
        invoke_validate_authoritative_source_policy(&ValidateAuthoritativeSourcePolicyRequest {
            repo_root: Some(std::path::PathBuf::from("missing-repository")),
            ..ValidateAuthoritativeSourcePolicyRequest::default()
        })
        .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve authoritative source policy workspace root"
    );
}

#[test]
fn test_validate_instruction_architecture_error_display_is_stable() {
    let error = invoke_validate_instruction_architecture(&ValidateInstructionArchitectureRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateInstructionArchitectureRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve instruction architecture workspace root"
    );
}

#[test]
fn test_validate_instructions_error_display_is_stable() {
    let error = invoke_validate_instructions(&ValidateInstructionsRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateInstructionsRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve instruction validation workspace root"
    );
}

#[test]
fn test_validate_readme_standards_error_display_is_stable() {
    let error = invoke_validate_readme_standards(&ValidateReadmeStandardsRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateReadmeStandardsRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve readme standards workspace root"
    );
}

#[test]
fn test_validate_instruction_metadata_error_display_is_stable() {
    let error = invoke_validate_instruction_metadata(&ValidateInstructionMetadataRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateInstructionMetadataRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve instruction metadata workspace root"
    );
}

#[test]
fn test_validate_routing_coverage_error_display_is_stable() {
    let error = invoke_validate_routing_coverage(&ValidateRoutingCoverageRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateRoutingCoverageRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve routing coverage workspace root"
    );
}

#[test]
fn test_validate_template_standards_error_display_is_stable() {
    let error = invoke_validate_template_standards(&ValidateTemplateStandardsRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateTemplateStandardsRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve template standards workspace root"
    );
}

#[test]
fn test_validate_workspace_efficiency_error_display_is_stable() {
    let error = invoke_validate_workspace_efficiency(&ValidateWorkspaceEfficiencyRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateWorkspaceEfficiencyRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve workspace efficiency workspace root"
    );
}

#[test]
fn test_validate_warning_baseline_error_display_is_stable() {
    let error = invoke_validate_warning_baseline(&ValidateWarningBaselineRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateWarningBaselineRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve warning baseline workspace root"
    );
}