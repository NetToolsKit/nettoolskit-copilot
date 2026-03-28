//! Tests for validation surface errors.

use nettoolskit_validation::{
    invoke_refactor_tests_to_aaa, invoke_validate_agent_hooks, invoke_validate_agent_orchestration,
    invoke_validate_agent_permissions, invoke_validate_agent_skill_alignment, invoke_validate_all,
    invoke_validate_architecture_boundaries, invoke_validate_audit_ledger,
    invoke_validate_authoritative_source_policy, invoke_validate_compatibility_lifecycle_policy,
    invoke_validate_deploy_preflight, invoke_validate_dotnet_standards,
    invoke_validate_instruction_architecture, invoke_validate_instruction_metadata,
    invoke_validate_instructions, invoke_validate_planning_structure, invoke_validate_policy,
    invoke_validate_powershell_standards, invoke_validate_readme_standards,
    invoke_validate_release_governance, invoke_validate_release_provenance,
    invoke_validate_routing_coverage, invoke_validate_runtime_script_tests,
    invoke_validate_security_baseline, invoke_validate_shared_script_checksums,
    invoke_validate_shell_hooks, invoke_validate_supply_chain, invoke_validate_template_standards,
    invoke_validate_test_naming, invoke_validate_warning_baseline,
    invoke_validate_workspace_efficiency, require_validation_surface_contract,
    RefactorTestsToAaaRequest, ValidateAgentHooksRequest, ValidateAgentOrchestrationRequest,
    ValidateAgentPermissionsRequest, ValidateAgentSkillAlignmentRequest, ValidateAllRequest,
    ValidateArchitectureBoundariesRequest, ValidateAuditLedgerRequest,
    ValidateAuthoritativeSourcePolicyRequest, ValidateCompatibilityLifecyclePolicyRequest,
    ValidateDeployPreflightRequest, ValidateDotnetStandardsRequest,
    ValidateInstructionArchitectureRequest, ValidateInstructionMetadataRequest,
    ValidateInstructionsRequest, ValidatePlanningStructureRequest, ValidatePolicyRequest,
    ValidatePowerShellStandardsRequest, ValidateReadmeStandardsRequest,
    ValidateReleaseGovernanceRequest, ValidateReleaseProvenanceRequest,
    ValidateRoutingCoverageRequest, ValidateRuntimeScriptTestsRequest,
    ValidateSecurityBaselineRequest, ValidateSharedScriptChecksumsRequest,
    ValidateShellHooksRequest, ValidateSupplyChainRequest, ValidateTemplateStandardsRequest,
    ValidateTestNamingRequest, ValidateWarningBaselineRequest, ValidateWorkspaceEfficiencyRequest,
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
fn test_validate_compatibility_lifecycle_policy_error_display_is_stable() {
    let error = invoke_validate_compatibility_lifecycle_policy(
        &ValidateCompatibilityLifecyclePolicyRequest {
            repo_root: Some(std::path::PathBuf::from("missing-repository")),
            ..ValidateCompatibilityLifecyclePolicyRequest::default()
        },
    )
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve compatibility lifecycle policy workspace root"
    );
}

#[test]
fn test_validate_dotnet_standards_error_display_is_stable() {
    let error = invoke_validate_dotnet_standards(&ValidateDotnetStandardsRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateDotnetStandardsRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve dotnet standards workspace root"
    );
}

#[test]
fn test_validate_powershell_standards_error_display_is_stable() {
    let error = invoke_validate_powershell_standards(&ValidatePowerShellStandardsRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidatePowerShellStandardsRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve powershell standards workspace root"
    );
}

#[test]
fn test_validate_architecture_boundaries_error_display_is_stable() {
    let error = invoke_validate_architecture_boundaries(&ValidateArchitectureBoundariesRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateArchitectureBoundariesRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve architecture boundaries workspace root"
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

#[test]
fn test_validate_runtime_script_tests_error_display_is_stable() {
    let error = invoke_validate_runtime_script_tests(&ValidateRuntimeScriptTestsRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateRuntimeScriptTestsRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve runtime script test workspace root"
    );
}

#[test]
fn test_refactor_tests_to_aaa_error_display_is_stable() {
    let error = invoke_refactor_tests_to_aaa(&RefactorTestsToAaaRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        test_file: std::path::PathBuf::from("tests/example.rs"),
        dry_run: true,
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve AAA refactor workspace root"
    );
}

#[test]
fn test_validate_test_naming_error_display_is_stable() {
    let error = invoke_validate_test_naming(&ValidateTestNamingRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateTestNamingRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve test naming workspace root"
    );
}

#[test]
fn test_validate_deploy_preflight_error_display_is_stable() {
    let error = invoke_validate_deploy_preflight(&ValidateDeployPreflightRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateDeployPreflightRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve deploy preflight workspace root"
    );
}

#[test]
fn test_validate_shell_hooks_error_display_is_stable() {
    let error = invoke_validate_shell_hooks(&ValidateShellHooksRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateShellHooksRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve shell hooks workspace root"
    );
}

#[test]
fn test_validate_agent_hooks_error_display_is_stable() {
    let error = invoke_validate_agent_hooks(&ValidateAgentHooksRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateAgentHooksRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve agent hooks workspace root"
    );
}

#[test]
fn test_validate_agent_permissions_error_display_is_stable() {
    let error = invoke_validate_agent_permissions(&ValidateAgentPermissionsRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateAgentPermissionsRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve agent permissions workspace root"
    );
}

#[test]
fn test_validate_agent_skill_alignment_error_display_is_stable() {
    let error = invoke_validate_agent_skill_alignment(&ValidateAgentSkillAlignmentRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateAgentSkillAlignmentRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve agent skill alignment workspace root"
    );
}

#[test]
fn test_validate_agent_orchestration_error_display_is_stable() {
    let error = invoke_validate_agent_orchestration(&ValidateAgentOrchestrationRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve agent orchestration workspace root"
    );
}

#[test]
fn test_validate_policy_error_display_is_stable() {
    let error = invoke_validate_policy(&ValidatePolicyRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidatePolicyRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(error.to_string(), "failed to resolve policy workspace root");
}

#[test]
fn test_validate_security_baseline_error_display_is_stable() {
    let error = invoke_validate_security_baseline(&ValidateSecurityBaselineRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateSecurityBaselineRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve security baseline workspace root"
    );
}

#[test]
fn test_validate_shared_script_checksums_error_display_is_stable() {
    let error = invoke_validate_shared_script_checksums(&ValidateSharedScriptChecksumsRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateSharedScriptChecksumsRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve shared script checksum workspace root"
    );
}

#[test]
fn test_validate_supply_chain_error_display_is_stable() {
    let error = invoke_validate_supply_chain(&ValidateSupplyChainRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateSupplyChainRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve supply chain workspace root"
    );
}

#[test]
fn test_validate_release_governance_error_display_is_stable() {
    let error = invoke_validate_release_governance(&ValidateReleaseGovernanceRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateReleaseGovernanceRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve release governance workspace root"
    );
}

#[test]
fn test_validate_release_provenance_error_display_is_stable() {
    let error = invoke_validate_release_provenance(&ValidateReleaseProvenanceRequest {
        repo_root: Some(std::path::PathBuf::from("missing-repository")),
        ..ValidateReleaseProvenanceRequest::default()
    })
    .expect_err("missing repository should fail");

    assert_eq!(
        error.to_string(),
        "failed to resolve release provenance workspace root"
    );
}
