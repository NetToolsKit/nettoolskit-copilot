//! Tests for runtime surface errors.

use anyhow::anyhow;
use nettoolskit_runtime::{
    require_runtime_surface_contract, LocalContextCommandError, PlanningSummaryCommandError,
    RuntimeApplyVscodeTemplatesCommandError, RuntimeBootstrapCommandError,
    RuntimeDoctorCommandError, RuntimeHealthcheckCommandError,
    RuntimePreCommitEofHygieneCommandError, RuntimeSelfHealCommandError,
    RuntimeSetupGitHooksCommandError, RuntimeSetupGlobalGitAliasesCommandError,
};
use std::error::Error;

#[test]
fn test_runtime_surface_error_mentions_missing_surface_id() {
    let error = require_runtime_surface_contract("missing-runtime")
        .expect_err("unknown runtime surface should fail");

    assert_eq!(
        error.to_string(),
        "unknown runtime surface contract: missing-runtime"
    );
}

#[test]
fn test_local_context_command_error_mentions_missing_index_path() {
    let error = LocalContextCommandError::IndexNotFound {
        index_path: "C:/repo/.temp/context-index/index.json".to_string(),
    };

    assert_eq!(
        error.to_string(),
        "local context index not found: C:/repo/.temp/context-index/index.json"
    );
}

#[test]
fn test_local_context_command_error_preserves_source_message() {
    let error = LocalContextCommandError::BuildIndex {
        source: anyhow!("boom"),
    };

    assert_eq!(error.to_string(), "failed to build local context index");
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "boom"
    );
}

#[test]
fn test_planning_summary_error_display_is_stable() {
    let error = PlanningSummaryCommandError::WriteOutput {
        source: anyhow!("disk full"),
    };

    assert_eq!(error.to_string(), "failed to write planning summary output");
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "disk full"
    );
}

#[test]
fn test_runtime_doctor_error_display_is_stable() {
    let error = RuntimeDoctorCommandError::BuildReport {
        source: anyhow!("hash failure"),
    };

    assert_eq!(error.to_string(), "failed to build runtime doctor report");
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "hash failure"
    );
}

#[test]
fn test_runtime_doctor_sync_error_display_is_stable() {
    let error = RuntimeDoctorCommandError::SynchronizeRuntime {
        source: anyhow!("bootstrap failure"),
    };

    assert_eq!(
        error.to_string(),
        "failed to synchronize runtime doctor drift remediation"
    );
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "bootstrap failure"
    );
}

#[test]
fn test_runtime_bootstrap_error_display_is_stable() {
    let error = RuntimeBootstrapCommandError::SyncAssets {
        source: anyhow!("copy failure"),
    };

    assert_eq!(
        error.to_string(),
        "failed to synchronize runtime bootstrap assets"
    );
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "copy failure"
    );
}

#[test]
fn test_runtime_healthcheck_error_display_is_stable() {
    let error = RuntimeHealthcheckCommandError::WriteOutput {
        source: anyhow!("disk full"),
    };

    assert_eq!(
        error.to_string(),
        "failed to write runtime healthcheck output"
    );
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "disk full"
    );
}

#[test]
fn test_runtime_apply_vscode_templates_error_display_is_stable() {
    let error = RuntimeApplyVscodeTemplatesCommandError::ApplyTemplates {
        source: anyhow!("missing template"),
    };

    assert_eq!(
        error.to_string(),
        "failed to apply runtime vscode templates"
    );
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "missing template"
    );
}

#[test]
fn test_runtime_setup_global_git_aliases_error_display_is_stable() {
    let error = RuntimeSetupGlobalGitAliasesCommandError::ConfigureAliases {
        source: anyhow!("git config failed"),
    };

    assert_eq!(
        error.to_string(),
        "failed to configure runtime global git aliases"
    );
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "git config failed"
    );
}

#[test]
fn test_runtime_setup_git_hooks_error_display_is_stable() {
    let error = RuntimeSetupGitHooksCommandError::PersistSettings {
        source: anyhow!("write failed"),
    };

    assert_eq!(
        error.to_string(),
        "failed to persist runtime git hook settings"
    );
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "write failed"
    );
}

#[test]
fn test_runtime_pre_commit_eof_hygiene_error_display_is_stable() {
    let error = RuntimePreCommitEofHygieneCommandError::InspectGitState {
        source: anyhow!("git diff failed"),
    };

    assert_eq!(
        error.to_string(),
        "failed to inspect runtime pre-commit eof staged files"
    );
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "git diff failed"
    );
}

#[test]
fn test_runtime_self_heal_error_display_is_stable() {
    let error = RuntimeSelfHealCommandError::PrepareArtifacts {
        source: anyhow!("mkdir failure"),
    };

    assert_eq!(
        error.to_string(),
        "failed to prepare runtime self-heal artifacts"
    );
    assert_eq!(
        error
            .source()
            .expect("source should be preserved")
            .to_string(),
        "mkdir failure"
    );
}
