//! Tests for `validate-authoritative-source-policy`.

use nettoolskit_validation::{
    invoke_validate_authoritative_source_policy, ValidateAuthoritativeSourcePolicyRequest,
    ValidationCheckStatus,
};
use std::fs;
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn initialize_repo_layout(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".github/governance"))
        .expect("governance directory should be created");
    fs::create_dir_all(repo_root.join(".github/instructions"))
        .expect("instruction directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
}

fn write_valid_source_map(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".github/governance/authoritative-source-map.json"),
        r#"{
  "version": 1,
  "defaultPolicy": {
    "repositoryContextFirst": true
  },
  "stackRules": [
    { "id": "dotnet", "displayName": ".NET", "keywords": ["dotnet"], "officialDomains": ["learn.microsoft.com"] },
    { "id": "github-copilot", "displayName": "GitHub Copilot", "keywords": ["copilot"], "officialDomains": ["docs.github.com"] },
    { "id": "vscode", "displayName": "VS Code", "keywords": ["vscode"], "officialDomains": ["code.visualstudio.com"] },
    { "id": "rust", "displayName": "Rust", "keywords": ["rust"], "officialDomains": ["doc.rust-lang.org"] },
    { "id": "vue", "displayName": "Vue", "keywords": ["vue"], "officialDomains": ["vuejs.org"] },
    { "id": "quasar", "displayName": "Quasar", "keywords": ["quasar"], "officialDomains": ["quasar.dev"] },
    { "id": "docker", "displayName": "Docker", "keywords": ["docker"], "officialDomains": ["docs.docker.com"] },
    { "id": "kubernetes", "displayName": "Kubernetes", "keywords": ["kubernetes"], "officialDomains": ["kubernetes.io"] },
    { "id": "postgresql", "displayName": "PostgreSQL", "keywords": ["postgresql"], "officialDomains": ["postgresql.org"] },
    { "id": "openai", "displayName": "OpenAI", "keywords": ["openai"], "officialDomains": ["platform.openai.com"] }
  ]
}"#,
    );
}

fn write_valid_instruction_assets(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".github/instructions/core/ntk-core-authoritative-sources.instructions.md"),
        r#"# Authoritative Sources

Use `.github/governance/authoritative-source-map.json`.
Repository context first.
Use official documentation.
Use community sources only as fallback.
"#,
    );
    write_file(
        &repo_root.join(".github/AGENTS.md"),
        r#"# AGENTS

Use `instructions/core/ntk-core-authoritative-sources.instructions.md`.
Use `.github/governance/authoritative-source-map.json`.
"#,
    );
    write_file(
        &repo_root.join(".github/copilot-instructions.md"),
        r#"# Global Instructions

Use `instructions/core/ntk-core-authoritative-sources.instructions.md`.
Use `.github/governance/authoritative-source-map.json`.
"#,
    );
    write_file(
        &repo_root.join(".github/instruction-routing.catalog.yml"),
        "always:\n  - path: instructions/core/ntk-core-authoritative-sources.instructions.md\n",
    );
}

#[test]
fn test_invoke_validate_authoritative_source_policy_passes_for_valid_assets() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_valid_source_map(repo.path());
    write_valid_instruction_assets(repo.path());

    let result =
        invoke_validate_authoritative_source_policy(&ValidateAuthoritativeSourcePolicyRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateAuthoritativeSourcePolicyRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.stack_rules_checked, 10);
    assert!(result.failures.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn test_invoke_validate_authoritative_source_policy_reports_invalid_source_map() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_valid_instruction_assets(repo.path());
    write_file(
        &repo
            .path()
            .join(".github/governance/authoritative-source-map.json"),
        r#"{
  "version": 1,
  "defaultPolicy": {
    "repositoryContextFirst": true
  },
  "stackRules": [
    {
      "id": "dotnet",
      "displayName": ".NET",
      "keywords": ["dotnet"],
      "officialDomains": ["https://learn.microsoft.com/dotnet/"]
    }
  ]
}"#,
    );

    let result =
        invoke_validate_authoritative_source_policy(&ValidateAuthoritativeSourcePolicyRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateAuthoritativeSourcePolicyRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("contains invalid official domain")));
}

#[test]
fn test_invoke_validate_authoritative_source_policy_reports_missing_agents_reference() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_valid_source_map(repo.path());
    write_valid_instruction_assets(repo.path());
    write_file(
        &repo.path().join(".github/AGENTS.md"),
        "# Temporary AGENTS\n\nThis file intentionally omits the authoritative source instruction reference.\n",
    );

    let result =
        invoke_validate_authoritative_source_policy(&ValidateAuthoritativeSourcePolicyRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateAuthoritativeSourcePolicyRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("AGENTS.md is missing required pattern")));
}

#[test]
fn test_invoke_validate_authoritative_source_policy_warns_for_duplicate_official_domains() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_valid_source_map(repo.path());
    write_valid_instruction_assets(repo.path());
    write_file(
        &repo
            .path()
            .join(".github/instructions/duplicate.instructions.md"),
        r#"---
applyTo: "**/*.rs"
priority: medium
---

Use learn.microsoft.com for .NET lookups in this temporary duplicate file.
"#,
    );

    let result =
        invoke_validate_authoritative_source_policy(&ValidateAuthoritativeSourcePolicyRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: false,
            ..ValidateAuthoritativeSourcePolicyRequest::default()
        })
        .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.failures.is_empty());
    assert!(result.warnings.iter().any(|message| {
        message.contains("Instruction duplicates official documentation domains")
    }));
}

#[test]
fn test_invoke_validate_authoritative_source_policy_converts_required_findings_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_valid_source_map(repo.path());
    write_valid_instruction_assets(repo.path());
    fs::remove_file(repo.path().join(".github/AGENTS.md"))
        .expect("temporary AGENTS file should be removed");

    let result =
        invoke_validate_authoritative_source_policy(&ValidateAuthoritativeSourcePolicyRequest {
            repo_root: Some(repo.path().to_path_buf()),
            warning_only: true,
            ..ValidateAuthoritativeSourcePolicyRequest::default()
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