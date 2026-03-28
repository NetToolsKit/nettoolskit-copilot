//! Tests for `validate-instruction-architecture`.

use nettoolskit_validation::{
    invoke_validate_instruction_architecture, ValidateInstructionArchitectureRequest,
    ValidationCheckStatus,
};
use std::fs;
use tempfile::TempDir;

use crate::support::instruction_graph_fixtures::{
    initialize_instruction_architecture_repo, write_file,
};

#[test]
fn test_invoke_validate_instruction_architecture_passes_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_instruction_architecture_repo(repo.path());

    let result =
        invoke_validate_instruction_architecture(&ValidateInstructionArchitectureRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateInstructionArchitectureRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.layers_checked, 9);
    assert_eq!(result.prompt_files_scanned, 2);
    assert_eq!(result.template_files_scanned, 1);
    assert_eq!(result.skill_files_scanned, 1);
    assert!(result.failures.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn test_invoke_validate_instruction_architecture_reports_invalid_manifest() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_instruction_architecture_repo(repo.path());
    write_file(
        &repo
            .path()
            .join(".github/governance/instruction-ownership.manifest.json"),
        r#"{
  "version": 1,
  "layers": [
    {
      "id": "prompts",
      "pathPatterns": [".github/prompts/*"]
    }
  ]
}"#,
    );

    let result =
        invoke_validate_instruction_architecture(&ValidateInstructionArchitectureRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateInstructionArchitectureRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("missing required layer 'global-core'")));
}

#[test]
fn test_invoke_validate_instruction_architecture_reports_missing_agents_reference() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_instruction_architecture_repo(repo.path());
    write_file(
        &repo.path().join(".github/AGENTS.md"),
        "# Temporary AGENTS\n\nThis file intentionally omits the required reference.\n",
    );

    let result =
        invoke_validate_instruction_architecture(&ValidateInstructionArchitectureRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateInstructionArchitectureRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("AGENTS.md is missing required architecture reference")));
}

#[test]
fn test_invoke_validate_instruction_architecture_reports_missing_skill_reference() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_instruction_architecture_repo(repo.path());
    let custom_skill_root = repo.path().join("skills");
    write_file(
        &custom_skill_root.join("sample/SKILL.md"),
        r#"---
name: sample-skill
description: temporary skill
---

# Sample Skill

Load `.github/AGENTS.md` and `.github/copilot-instructions.md`.
"#,
    );

    let result =
        invoke_validate_instruction_architecture(&ValidateInstructionArchitectureRequest {
            repo_root: Some(repo.path().to_path_buf()),
            skill_root: Some(custom_skill_root),
            warning_only: false,
            ..ValidateInstructionArchitectureRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result.failures.iter().any(|message| {
        message.contains("Skill is missing canonical repository-operating reference")
    }));
}

#[test]
fn test_invoke_validate_instruction_architecture_warns_for_prompt_ownership_markers() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_instruction_architecture_repo(repo.path());
    let prompt_root = repo.path().join("prompts");
    write_file(
        &prompt_root.join("ownership.prompt.md"),
        "# Prompt\n\nThis prompt is the single source of truth for the whole repository.\n",
    );

    let result =
        invoke_validate_instruction_architecture(&ValidateInstructionArchitectureRequest {
            repo_root: Some(repo.path().to_path_buf()),
            prompt_root: Some(prompt_root),
            warning_only: false,
            ..ValidateInstructionArchitectureRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("prompt may be owning policy instead of behavior")));
}

#[test]
fn test_invoke_validate_instruction_architecture_reports_missing_route_hard_cap() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_instruction_architecture_repo(repo.path());
    write_file(
        &repo
            .path()
            .join(".github/prompts/route-instructions.prompt.md"),
        r#"---
description: Temporary route prompt
mode: ask
tools: ['readFile']
---

# Route Instructions

Use the routing catalog and return JSON.
"#,
    );

    let result =
        invoke_validate_instruction_architecture(&ValidateInstructionArchitectureRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateInstructionArchitectureRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result.failures.iter().any(|message| {
        message.contains("Route prompt is missing deterministic hard-cap text")
    }));
}

#[test]
fn test_invoke_validate_instruction_architecture_warns_for_template_ownership_markers() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_instruction_architecture_repo(repo.path());
    let template_root = repo.path().join("templates");
    write_file(
        &template_root.join("settings.tamplate.jsonc"),
        "{\n  \"//\": \"global rules live here\"\n}\n",
    );

    let result =
        invoke_validate_instruction_architecture(&ValidateInstructionArchitectureRequest {
            repo_root: Some(repo.path().to_path_buf()),
            template_root: Some(template_root),
            warning_only: false,
            ..ValidateInstructionArchitectureRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| { message.contains("template may be owning policy instead of behavior") }));
}

#[test]
fn test_invoke_validate_instruction_architecture_converts_failures_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_instruction_architecture_repo(repo.path());
    fs::remove_file(repo.path().join(".github/AGENTS.md"))
        .expect("temporary AGENTS file should be removed");

    let result =
        invoke_validate_instruction_architecture(&ValidateInstructionArchitectureRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: true,
            ..ValidateInstructionArchitectureRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Missing AGENTS.md")));
}
