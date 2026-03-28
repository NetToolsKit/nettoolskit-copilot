//! Tests for `validate-readme-standards`.

use nettoolskit_validation::{
    invoke_validate_readme_standards, ValidateReadmeStandardsRequest, ValidationCheckStatus,
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
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
}

fn write_baseline(repo_root: &std::path::Path, file_path: &str) {
    write_file(
        &repo_root.join(".github/governance/readme-standards.baseline.json"),
        &format!(
            r#"{{
  "version": 1,
  "global": {{
    "requireFeaturesCheckmarks": true,
    "requireCodeFences": true,
    "requireTocLinks": true,
    "requireHorizontalSeparators": true
  }},
  "files": [
    {{
      "path": "{file_path}",
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
      "allowIntroductionPreamble": false
    }}
  ]
}}"#
        ),
    );
}

fn write_valid_readme(repo_root: &std::path::Path, relative_path: &str) {
    write_file(
        &repo_root.join(relative_path),
        r#"# Example

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

Run it.

## Usage Examples

Use it.

## API Reference

Documented.

## Dependencies

- Rust

## References

- [Example](#example)
"#,
    );
}

#[test]
fn test_invoke_validate_readme_standards_passes_for_valid_baseline_and_readme() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path(), "README.md");
    write_valid_readme(repo.path(), "README.md");

    let result = invoke_validate_readme_standards(&ValidateReadmeStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateReadmeStandardsRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.files_checked, 1);
    assert!(result.failures.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn test_invoke_validate_readme_standards_reports_missing_required_sections() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path(), "README.md");
    write_file(
        &repo.path().join("README.md"),
        "# Example\n\n## Features\n\n- ✅ Only features\n",
    );

    let result = invoke_validate_readme_standards(&ValidateReadmeStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateReadmeStandardsRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Missing required section 'Contents|Table of Contents'")));
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("README must include at least one fenced code block")));
}

#[test]
fn test_invoke_validate_readme_standards_converts_required_findings_to_warnings() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path(), "README.md");
    write_file(
        &repo.path().join("README.md"),
        "# Example\n\n## Features\n\n- ✅ Only features\n",
    );

    let result = invoke_validate_readme_standards(&ValidateReadmeStandardsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: true,
        ..ValidateReadmeStandardsRequest::default()
    })
    .expect("validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Missing required section 'Contents|Table of Contents'")));
}
