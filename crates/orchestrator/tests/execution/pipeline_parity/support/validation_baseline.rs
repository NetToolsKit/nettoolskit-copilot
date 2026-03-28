//! Temporary repository fixtures that make the scripted validation stage deterministic.

use std::fs;
use std::path::{Path, PathBuf};

const SHARED_POML_README_PATH: &str = "definitions/shared/prompts/poml/README.md";
const SCRIPTS_README_PATH: &str = "scripts/README.md";
const CODEOWNERS_PATH: &str = "CODEOWNERS";
const CHANGELOG_PATH: &str = "CHANGELOG.md";
const PR_TEMPLATE_PATH: &str = ".github/PULL_REQUEST_TEMPLATE.md";
const ISSUE_TEMPLATE_CONFIG_PATH: &str = ".github/ISSUE_TEMPLATE/config.yml";
const ISSUE_TEMPLATE_BUG_PATH: &str = ".github/ISSUE_TEMPLATE/bug-instructions.yml";
const ISSUE_TEMPLATE_SKILL_PATH: &str = ".github/ISSUE_TEMPLATE/new-skill-request.yml";
const ISSUE_TEMPLATE_RUNTIME_PATH: &str = ".github/ISSUE_TEMPLATE/runtime-sync-problem.yml";
const ISSUE_TEMPLATE_VALIDATION_PATH: &str = ".github/ISSUE_TEMPLATE/validation-gap.yml";
const VALIDATE_AGENT_SYSTEM_WORKFLOW_PATH: &str = ".github/workflows/validate-agent-system.yml";
const VALIDATE_RELEASE_GOVERNANCE_WORKFLOW_PATH: &str =
    ".github/workflows/validate-release-governance.yml";
const DEPENDENCY_RISK_WORKFLOW_PATH: &str = ".github/workflows/dependency-risk-observability.yml";
const ENTERPRISE_TRENDS_WORKFLOW_PATH: &str = ".github/workflows/enterprise-trends-dashboard.yml";
const SBOM_ATTESTATION_WORKFLOW_PATH: &str = ".github/workflows/sbom-attestation-observability.yml";
const SECURITY_STATIC_WORKFLOW_PATH: &str = ".github/workflows/security-static-observability.yml";
const PRE_COMMIT_HOOK_PATH: &str = ".githooks/pre-commit";
const POST_COMMIT_HOOK_PATH: &str = ".githooks/post-commit";
const POST_MERGE_HOOK_PATH: &str = ".githooks/post-merge";
const POST_CHECKOUT_HOOK_PATH: &str = ".githooks/post-checkout";

pub(crate) fn repo_validation_paths(repo_root: &Path) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = [
        SHARED_POML_README_PATH,
        SCRIPTS_README_PATH,
        CODEOWNERS_PATH,
        CHANGELOG_PATH,
        PR_TEMPLATE_PATH,
        ISSUE_TEMPLATE_CONFIG_PATH,
        ISSUE_TEMPLATE_BUG_PATH,
        ISSUE_TEMPLATE_SKILL_PATH,
        ISSUE_TEMPLATE_RUNTIME_PATH,
        ISSUE_TEMPLATE_VALIDATION_PATH,
        VALIDATE_AGENT_SYSTEM_WORKFLOW_PATH,
        VALIDATE_RELEASE_GOVERNANCE_WORKFLOW_PATH,
        DEPENDENCY_RISK_WORKFLOW_PATH,
        ENTERPRISE_TRENDS_WORKFLOW_PATH,
        SBOM_ATTESTATION_WORKFLOW_PATH,
        SECURITY_STATIC_WORKFLOW_PATH,
        PRE_COMMIT_HOOK_PATH,
        POST_COMMIT_HOOK_PATH,
        POST_MERGE_HOOK_PATH,
        POST_CHECKOUT_HOOK_PATH,
    ]
    .into_iter()
    .map(|relative_path| repo_root.join(relative_path))
    .collect();
    paths.extend(collect_directory_files(
        &repo_root.join(".github/prompts/poml"),
    ));
    paths
}

pub(crate) fn seed_validation_green_baseline(repo_root: &Path) {
    ensure_file_exists(repo_root, SHARED_POML_README_PATH, poml_readme_contents());
    mirror_directory_if_missing(
        &repo_root.join("definitions/shared/prompts/poml"),
        &repo_root.join(".github/prompts/poml"),
    );
    ensure_file_exists(repo_root, SCRIPTS_README_PATH, scripts_readme_contents());
    ensure_file_exists(repo_root, CODEOWNERS_PATH, codeowners_contents());
    ensure_file_exists(repo_root, CHANGELOG_PATH, changelog_contents());
    ensure_file_exists(repo_root, PR_TEMPLATE_PATH, pr_template_contents());
    ensure_file_exists(
        repo_root,
        ISSUE_TEMPLATE_CONFIG_PATH,
        issue_template_config_contents(),
    );
    ensure_file_exists(
        repo_root,
        ISSUE_TEMPLATE_BUG_PATH,
        issue_template_contents(
            "Bug Instructions",
            "Report a repository instruction or workflow bug.",
        ),
    );
    ensure_file_exists(
        repo_root,
        ISSUE_TEMPLATE_SKILL_PATH,
        issue_template_contents(
            "New Skill Request",
            "Request a new skill for the repository agent system.",
        ),
    );
    ensure_file_exists(
        repo_root,
        ISSUE_TEMPLATE_RUNTIME_PATH,
        issue_template_contents(
            "Runtime Sync Problem",
            "Report a runtime sync or projection issue.",
        ),
    );
    ensure_file_exists(
        repo_root,
        ISSUE_TEMPLATE_VALIDATION_PATH,
        issue_template_contents(
            "Validation Gap",
            "Report a missing or incorrect validation rule.",
        ),
    );
    ensure_file_exists(
        repo_root,
        VALIDATE_AGENT_SYSTEM_WORKFLOW_PATH,
        workflow_contents("Validate Agent System"),
    );
    ensure_file_exists(
        repo_root,
        VALIDATE_RELEASE_GOVERNANCE_WORKFLOW_PATH,
        workflow_contents("Validate Release Governance"),
    );
    ensure_file_exists(
        repo_root,
        DEPENDENCY_RISK_WORKFLOW_PATH,
        workflow_contents("Dependency Risk Observability"),
    );
    ensure_file_exists(
        repo_root,
        ENTERPRISE_TRENDS_WORKFLOW_PATH,
        workflow_contents("Enterprise Trends Dashboard"),
    );
    ensure_file_exists(
        repo_root,
        SBOM_ATTESTATION_WORKFLOW_PATH,
        workflow_contents("SBOM Attestation Observability"),
    );
    ensure_file_exists(
        repo_root,
        SECURITY_STATIC_WORKFLOW_PATH,
        workflow_contents("Security Static Observability"),
    );
    ensure_file_exists(repo_root, PRE_COMMIT_HOOK_PATH, hook_contents("pre-commit"));
    ensure_file_exists(
        repo_root,
        POST_COMMIT_HOOK_PATH,
        hook_contents("post-commit"),
    );
    ensure_file_exists(repo_root, POST_MERGE_HOOK_PATH, hook_contents("post-merge"));
    ensure_file_exists(
        repo_root,
        POST_CHECKOUT_HOOK_PATH,
        hook_contents("post-checkout"),
    );
}

pub(crate) fn cleanup_validation_green_baseline(repo_root: &Path) {
    for relative_dir in [".githooks", ".github/ISSUE_TEMPLATE"] {
        let dir_path = repo_root.join(relative_dir);
        let is_empty = dir_path
            .read_dir()
            .map(|mut entries| entries.next().is_none())
            .unwrap_or(false);
        if is_empty {
            let _ = fs::remove_dir(&dir_path);
        }
    }
}

fn ensure_file_exists(repo_root: &Path, relative_path: &str, contents: impl AsRef<[u8]>) {
    let absolute_path = repo_root.join(relative_path);
    if absolute_path.exists() {
        return;
    }
    if let Some(parent) = absolute_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(absolute_path, contents).expect("fixture file should be written");
}

fn collect_directory_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if !root.exists() {
        return files;
    }

    let mut stack = vec![root.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = fs::read_dir(&current) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                files.push(path);
            }
        }
    }

    files
}

fn mirror_directory_if_missing(source: &Path, destination: &Path) {
    if destination.exists() {
        return;
    }

    fs::create_dir_all(destination).expect("destination directory should be created");
    copy_directory_contents(source, destination);
}

fn copy_directory_contents(source: &Path, destination: &Path) {
    let entries = fs::read_dir(source).expect("source directory should be readable");
    for entry in entries.flatten() {
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            fs::create_dir_all(&destination_path).expect("nested directory should be created");
            copy_directory_contents(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).expect("projected file should be copied");
        }
    }
}

fn poml_readme_contents() -> &'static str {
    "# Shared POML Library

> Canonical POML prompt library for reusable repository prompts.

---

## Introduction

This folder is the authoritative source for the reusable POML prompt library.

---

## Features

- Shared POML prompts remain in a single authoritative location.
- The projected GitHub prompt surface stays aligned with the shared source.

---

## References

- [Repository README](/README.md)
- [Shared Definitions](/definitions/README.md)
- [Shared Prompt Assets](/definitions/shared/prompts/README.md)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.
"
}

fn scripts_readme_contents() -> &'static str {
    "# Scripts

> Repository-owned operational entrypoints for bootstrap, projection, validation, health, and maintenance.

---

## Introduction

`scripts/` is the supported execution layer for repository operations.

---

## Features

- Bootstrap and sync repository runtime surfaces from versioned assets.
- Validate README, instruction, policy, and workspace standards.
- Run health, remediation, security, and maintenance entrypoints.

---

## References

- [Repository README](../README.md)
- [Planning README](../planning/README.md)
- [Definitions README](../definitions/README.md)
- [AGENTS](../.github/AGENTS.md)
- [Copilot Instructions](../.github/copilot-instructions.md)
- [Bootstrap](runtime/bootstrap.ps1)
- [Render Provider Surfaces](runtime/render-provider-surfaces.ps1)
- [Healthcheck](runtime/healthcheck.ps1)
- [Self-Heal](runtime/self-heal.ps1)
- [Validate All](validation/validate-all.ps1)
- [Validate README Standards](validation/validate-readme-standards.ps1)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.
"
}

fn codeowners_contents() -> &'static str {
    "* @nettoolskit/owners
.github/ @nettoolskit/owners
.githooks/ @nettoolskit/owners
scripts/ @nettoolskit/owners
"
}

fn changelog_contents() -> &'static str {
    "# Changelog

All notable changes to this project will be documented in this file.

## [9.9.9] - 2026-03-20

### Added
- Temporary parity harness governance baseline for the scripted orchestration tests.

## [9.9.8] - 2026-03-19

### Added
- Prior stable release placeholder.
"
}

fn pr_template_contents() -> &'static str {
    "## Summary

- Describe the change.

## Validation

- Describe the validation performed.
"
}

fn issue_template_config_contents() -> &'static str {
    "blank_issues_enabled: false
contact_links: []
"
}

fn issue_template_contents(name: &str, about: &str) -> String {
    format!(
        "name: {name}
description: {about}
title: \"[{name}] \"
labels:
  - triage
body:
  - type: textarea
    id: details
    attributes:
      label: Details
      description: Provide the relevant context.
    validations:
      required: true
"
    )
}

fn workflow_contents(name: &str) -> String {
    format!(
        "name: {name}
on:
  workflow_dispatch:
jobs:
  placeholder:
    runs-on: ubuntu-latest
    steps:
      - run: echo \"placeholder workflow for parity harness\"
"
    )
}

fn hook_contents(name: &str) -> String {
    format!("#!/usr/bin/env pwsh\nWrite-Host \"{name} placeholder\"\n")
}