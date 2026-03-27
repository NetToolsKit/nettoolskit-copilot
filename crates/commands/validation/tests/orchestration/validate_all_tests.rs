//! Tests for validation `validate-all` orchestration.

use nettoolskit_validation::{invoke_validate_all, ValidateAllRequest, ValidationCheckStatus};
use std::fs;
use tempfile::TempDir;

use crate::support::instruction_graph_fixtures::{
    initialize_instruction_architecture_repo, initialize_validate_instructions_repo,
};
use crate::support::operational_hygiene_fixtures::{
    initialize_runtime_script_tests_repo, initialize_warning_baseline_repo,
    initialize_shell_hooks_repo, write_fake_shell_command, write_hook_file,
    write_runtime_test_script, write_warning_analyzer_report,
};

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn write_validation_profile_catalog(repo_root: &std::path::Path, check_order: &[&str]) {
    let checks = check_order
        .iter()
        .map(|check| format!("\"{check}\""))
        .collect::<Vec<_>>()
        .join(",");
    write_file(
        &repo_root.join(".github/governance/validation-profiles.json"),
        &format!(
            "{{\"version\":1,\"defaultProfile\":\"test\",\"profiles\":[{{\"id\":\"test\",\"warningOnly\":false,\"checkOrder\":[{checks}]}}]}}"
        ),
    );
}

fn initialize_repo_layout(repo_root: &std::path::Path, check_order: &[&str]) {
    fs::create_dir_all(repo_root.join(".github")).expect("github directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
    fs::create_dir_all(repo_root.join("scripts/validation"))
        .expect("validation directory should be created");
    write_validation_profile_catalog(repo_root, check_order);
}

fn write_default_readme_baseline(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".github/governance/readme-standards.baseline.json"),
        r#"{
  "version": 1,
  "global": {
    "requireFeaturesCheckmarks": true,
    "requireCodeFences": true,
    "requireTocLinks": true,
    "requireHorizontalSeparators": true
  },
  "files": [
    {
      "path": "README.md",
      "requiredSections": [
        "Features",
        "Contents|Table of Contents",
        "Installation",
        "Quick Start",
        "Usage Examples",
        "API Reference",
        "Dependencies",
        "References"
      ],
      "allowIntroductionPreamble": true
    }
  ]
}"#,
    );
}

fn write_valid_readme(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join("README.md"),
        r#"# Example

Intro paragraph.

---

## Features

- ✅ Deterministic validation

---

## Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
- [API Reference](#api-reference)
- [Dependencies](#dependencies)
- [References](#references)

---

## Installation

```sh
cargo test
```

## Quick Start

Run the validation flow.

## Usage Examples

Use the generated report.

## API Reference

Documented in the repo.

## Dependencies

- Rust

## References

- [README](#example)
"#,
    );
}

fn write_valid_instruction_metadata_fixtures(repo_root: &std::path::Path, broad_apply_to: bool) {
    let instruction_apply_to = if broad_apply_to {
        "**/*"
    } else {
        "**/*.{rs,md}"
    };
    write_file(
        &repo_root.join(".github/instructions/example.instructions.md"),
        &format!("---\napplyTo: \"{instruction_apply_to}\"\npriority: medium\n---\n\n# Example\n"),
    );
    write_file(
        &repo_root.join(".github/prompts/example.prompt.md"),
        "---\ndescription: Example prompt\nmode: ask\ntools: ['codebase']\n---\n\n# Example\n",
    );
    write_file(
        &repo_root.join(".github/chatmodes/example.chatmode.md"),
        "---\ndescription: Example chat mode\ntools: ['codebase']\n---\n\n# Example\n",
    );
}

fn write_routing_catalog_and_fixtures(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".github/instruction-routing.catalog.yml"),
        r#"version: 1
routing:
  - id: docs
    include:
      - path: instructions/readme.instructions.md
"#,
    );
    write_file(
        &repo_root.join(".github/instructions/readme.instructions.md"),
        "# readme",
    );
    write_file(
        &repo_root.join("scripts/validation/fixtures/routing-golden-tests.json"),
        r#"{
  "cases": [
    {
      "id": "docs-route",
      "expected_route_ids": ["docs"],
      "expected_selected_paths": ["instructions/readme.instructions.md"]
    }
  ]
}"#,
    );
}

fn write_template_standards_fixture(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".github/governance/template-standards.baseline.json"),
        r#"{
  "version": 1,
  "requiredFiles": [
    ".github/templates/example.md"
  ],
  "templateRules": [
    {
      "path": ".github/templates/example.md",
      "requiredPatterns": ["^# Example", "Validation"],
      "forbiddenPatterns": ["Legacy"],
      "requiredPathReferences": [".github/governance/template-standards.baseline.json"]
    }
  ]
}"#,
    );
    write_file(
        &repo_root.join(".github/templates/example.md"),
        "# Example\n\nValidation content.\n",
    );
}

fn write_authoritative_source_policy_fixtures(repo_root: &std::path::Path) {
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
    write_file(
        &repo_root.join(".github/instructions/authoritative-sources.instructions.md"),
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

Use `instructions/authoritative-sources.instructions.md`.
Use `.github/governance/authoritative-source-map.json`.
"#,
    );
    write_file(
        &repo_root.join(".github/copilot-instructions.md"),
        r#"# Global Instructions

Use `instructions/authoritative-sources.instructions.md`.
Use `.github/governance/authoritative-source-map.json`.
"#,
    );
    write_file(
        &repo_root.join(".github/instruction-routing.catalog.yml"),
        "always:\n  - path: instructions/authoritative-sources.instructions.md\n",
    );
}

fn write_workspace_efficiency_baseline(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".github/governance/workspace-efficiency.baseline.json"),
        r#"{
  "version": 1,
  "templateWorkspacePaths": [
    ".vscode/base.code-workspace"
  ],
  "allowedWorkspaceOverrideSettings": [
    "chat.agent.maxRequests"
  ],
  "requiredSettings": {
    "git.autofetch": false,
    "files.exclude": {
      "requiredKeys": [
        "**/.git"
      ]
    }
  },
  "forbiddenSettings": {
    "git.openRepositoryInParentFolders": [
      "always"
    ]
  },
  "recommendedSettings": {
    "extensions.autoUpdate": false
  },
  "recommendedNumericUpperBounds": {
    "chat.agent.maxRequests": 100
  },
  "heuristics": {
    "maxFolderCountWarning": 4,
    "warnWhenMultipleProductFolders": true,
    "warnWhenSupportFoldersMixedWithProductFolders": true,
    "supportFolderPatterns": [
      "(?i)(?:^|[\\\\/])\\\\.github$",
      "(?i)(?:^|[\\\\/])\\\\.codex$"
    ]
  }
}"#,
    );
}

fn write_workspace_settings_template(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".vscode/settings.tamplate.jsonc"),
        r#"{
  // global template
  "git.autofetch": false,
  "extensions.autoUpdate": false,
  "files.exclude": {
    "**/.git": true
  }
}"#,
    );
}

fn write_workspace_fixture(repo_root: &std::path::Path, relative_path: &str, contents: &str) {
    write_file(&repo_root.join(relative_path), contents);
}

#[test]
fn test_invoke_validate_all_runs_selected_profile_and_writes_report_and_ledger() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-policy"]);
    write_file(
        &repo.path().join("scripts/validation/validate-policy.ps1"),
        "param([string]$RepoRoot)\nexit 0",
    );

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.profile_id, "test");
    assert_eq!(result.total_checks, 1);
    assert_eq!(result.passed_checks, 1);
    assert_eq!(result.overall_status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert!(result.output_path.is_file());
    assert!(result
        .ledger_path
        .as_ref()
        .expect("ledger path should exist")
        .is_file());
    assert!(result.report_json.contains("\"profile\": \"test\""));
}

#[test]
fn test_invoke_validate_all_converts_missing_script_to_warning_when_warning_only_enabled() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-policy"]);

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 1);
    assert_eq!(result.warning_checks, 1);
    assert_eq!(result.overall_status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.checks[0]
        .error
        .as_deref()
        .expect("missing script error should be present")
        .contains("script not found"));
}

#[test]
fn test_invoke_validate_all_archives_broken_ledger_and_enforces_failure() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-policy"]);
    write_file(
        &repo.path().join("scripts/validation/validate-policy.ps1"),
        "param([string]$RepoRoot)\nexit 1",
    );
    write_file(
        &repo.path().join(".temp/audit/validation-ledger.jsonl"),
        "not-json",
    );

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.failed_checks, 1);
    assert_eq!(result.overall_status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result.archived_broken_ledger_path.is_some());
    assert!(result
        .ledger_path
        .as_ref()
        .expect("ledger path should exist")
        .is_file());
}

#[test]
fn test_invoke_validate_all_runs_native_planning_and_ledger_checks() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(
        repo.path(),
        &["validate-planning-structure", "validate-audit-ledger"],
    );
    write_file(&repo.path().join("planning/README.md"), "# planning");
    write_file(&repo.path().join("planning/specs/README.md"), "# specs");

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 2);
    assert_eq!(result.passed_checks, 2);
    assert!(result
        .checks
        .iter()
        .any(|check| check.script == "rust:nettoolskit-validation::validate-planning-structure"));
    assert!(result
        .checks
        .iter()
        .any(|check| check.script == "rust:nettoolskit-validation::validate-audit-ledger"));
}

#[test]
fn test_invoke_validate_all_runs_native_documentation_checks() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(
        repo.path(),
        &["validate-readme-standards", "validate-instruction-metadata"],
    );
    write_default_readme_baseline(repo.path());
    write_valid_readme(repo.path());
    write_valid_instruction_metadata_fixtures(repo.path(), false);

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 2);
    assert_eq!(result.passed_checks, 2);
    assert!(result
        .checks
        .iter()
        .any(|check| check.script == "rust:nettoolskit-validation::validate-readme-standards"));
    assert!(result.checks.iter().any(|check| {
        check.script == "rust:nettoolskit-validation::validate-instruction-metadata"
    }));
}

#[test]
fn test_invoke_validate_all_preserves_warning_status_for_native_checks() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-instruction-metadata"]);
    write_valid_instruction_metadata_fixtures(repo.path(), true);

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 1);
    assert_eq!(result.warning_checks, 1);
    assert_eq!(result.overall_status, ValidationCheckStatus::Warning);
    assert_eq!(result.checks[0].status, ValidationCheckStatus::Warning);
    assert_eq!(result.checks[0].exit_code, 0);
}

#[test]
fn test_invoke_validate_all_runs_native_governance_checks() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(
        repo.path(),
        &["validate-routing-coverage", "validate-template-standards"],
    );
    write_routing_catalog_and_fixtures(repo.path());
    write_template_standards_fixture(repo.path());

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 2);
    assert_eq!(result.passed_checks, 2);
    assert!(result
        .checks
        .iter()
        .any(|check| check.script == "rust:nettoolskit-validation::validate-routing-coverage"));
    assert!(result
        .checks
        .iter()
        .any(|check| check.script == "rust:nettoolskit-validation::validate-template-standards"));
}

#[test]
fn test_invoke_validate_all_runs_native_workspace_efficiency_check() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-workspace-efficiency"]);
    write_workspace_efficiency_baseline(repo.path());
    write_workspace_settings_template(repo.path());
    write_workspace_fixture(
        repo.path(),
        "workspace.code-workspace",
        r#"{
  "folders": [
    { "path": "App" }
  ],
  "settings": {
    "chat.agent.maxRequests": 80
  }
}"#,
    );

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 1);
    assert_eq!(result.passed_checks, 1);
    assert_eq!(result.checks[0].status, ValidationCheckStatus::Passed);
    assert_eq!(
        result.checks[0].script,
        "rust:nettoolskit-validation::validate-workspace-efficiency"
    );
}

#[test]
fn test_invoke_validate_all_runs_native_authoritative_source_policy_check() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-authoritative-source-policy"]);
    write_authoritative_source_policy_fixtures(repo.path());

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 1);
    assert_eq!(result.passed_checks, 1);
    assert_eq!(
        result.checks[0].script,
        "rust:nettoolskit-validation::validate-authoritative-source-policy"
    );
}

#[test]
fn test_invoke_validate_all_runs_native_instruction_architecture_check() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-instruction-architecture"]);
    initialize_instruction_architecture_repo(repo.path());

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 1);
    assert_eq!(result.passed_checks, 1);
    assert_eq!(
        result.checks[0].script,
        "rust:nettoolskit-validation::validate-instruction-architecture"
    );
}

#[test]
fn test_invoke_validate_all_runs_native_instruction_validation_check() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-instructions"]);
    initialize_validate_instructions_repo(repo.path());

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 1);
    assert_eq!(result.passed_checks, 1);
    assert_eq!(
        result.checks[0].script,
        "rust:nettoolskit-validation::validate-instructions"
    );
}

#[test]
fn test_invoke_validate_all_runs_native_warning_baseline_check() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-warning-baseline"]);
    initialize_warning_baseline_repo(repo.path());
    let analyzer_report_path = repo.path().join(".temp/audit/analyzer-warning-report.json");
    write_warning_analyzer_report(
        &analyzer_report_path,
        &[("PSAvoidUsingWriteHost", "scripts/example.ps1")],
    );
    write_file(
        &repo.path().join(".github/governance/validation-profiles.json"),
        r#"{
  "version": 1,
  "defaultProfile": "test",
  "profiles": [
    {
      "id": "test",
      "warningOnly": false,
      "checkOrder": ["validate-warning-baseline"],
      "checkOptions": {
        "validate-warning-baseline": {
          "AnalyzerReportPath": ".temp/audit/analyzer-warning-report.json"
        }
      }
    }
  ]
}"#,
    );

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 1);
    assert_eq!(result.passed_checks, 1);
    assert_eq!(
        result.checks[0].script,
        "rust:nettoolskit-validation::validate-warning-baseline"
    );
}

#[test]
fn test_invoke_validate_all_runs_native_runtime_script_tests_check() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-runtime-script-tests"]);
    initialize_runtime_script_tests_repo(repo.path());
    write_runtime_test_script(
        repo.path(),
        "pass.tests.ps1",
        "param([string]$RepoRoot)\nexit 0\n",
    );

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 1);
    assert_eq!(result.passed_checks, 1);
    assert_eq!(
        result.checks[0].script,
        "rust:nettoolskit-validation::validate-runtime-script-tests"
    );
}

#[test]
fn test_invoke_validate_all_runs_native_shell_hooks_check() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path(), &["validate-shell-hooks"]);
    initialize_shell_hooks_repo(repo.path());
    for hook_name in ["pre-commit", "post-commit", "post-merge", "post-checkout"] {
        write_hook_file(repo.path(), hook_name, "#!/bin/sh\necho ok\n");
    }
    let shell_path = write_fake_shell_command(repo.path());
    write_file(
        &repo.path().join(".github/governance/validation-profiles.json"),
        &format!(
            r#"{{
  "version": 1,
  "defaultProfile": "test",
  "profiles": [
    {{
      "id": "test",
      "warningOnly": false,
      "checkOrder": ["validate-shell-hooks"],
      "checkOptions": {{
        "validate-shell-hooks": {{
          "ShellPath": "{}"
        }}
      }}
    }}
  ]
}}"#,
            shell_path.display().to_string().replace('\\', "\\\\")
        ),
    );

    let result = invoke_validate_all(&ValidateAllRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateAllRequest::default()
    })
    .expect("validate-all should execute");

    assert_eq!(result.total_checks, 1);
    assert_eq!(result.passed_checks, 1);
    assert_eq!(
        result.checks[0].script,
        "rust:nettoolskit-validation::validate-shell-hooks"
    );
}