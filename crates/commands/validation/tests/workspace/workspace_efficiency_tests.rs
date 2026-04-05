//! Tests for `validate-workspace-efficiency`.

use nettoolskit_validation::{
    invoke_validate_workspace_efficiency, ValidateWorkspaceEfficiencyRequest, ValidationCheckStatus,
};
use std::fs;
use tempfile::TempDir;

fn write_file(path: &std::path::Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn write_governance_file(repo_root: &std::path::Path, file_name: &str, contents: &str) {
    write_file(
        &repo_root
            .join("definitions/providers/github/governance")
            .join(file_name),
        contents,
    );
    write_file(&repo_root.join(".github/governance").join(file_name), contents);
}

fn initialize_repo_layout(repo_root: &std::path::Path) {
    fs::create_dir_all(repo_root.join(".vscode")).expect("vscode directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect("codex directory should be created");
}

fn write_baseline(repo_root: &std::path::Path, template_entries: &[&str]) {
    let template_paths = template_entries
        .iter()
        .map(|entry| format!("\"{entry}\""))
        .collect::<Vec<_>>()
        .join(",");
    write_governance_file(
        repo_root,
        "workspace-efficiency.baseline.json",
        &format!(
            r#"{{
  "version": 1,
  "templateWorkspacePaths": [{template_paths}],
  "allowedWorkspaceOverrideSettings": [
    "chat.agent.maxRequests"
  ],
  "requiredSettings": {{
    "git.autofetch": false,
    "files.exclude": {{
      "requiredKeys": [
        "**/.git"
      ]
    }}
  }},
  "forbiddenSettings": {{
    "git.openRepositoryInParentFolders": [
      "always"
    ]
  }},
  "recommendedSettings": {{
    "extensions.autoUpdate": false
  }},
  "recommendedNumericUpperBounds": {{
    "chat.agent.maxRequests": 100
  }},
  "heuristics": {{
    "maxFolderCountWarning": 4,
    "warnWhenMultipleProductFolders": true,
    "warnWhenSupportFoldersMixedWithProductFolders": true,
    "supportFolderPatterns": [
      "(?i)(?:^|[\\\\\\\\/])\\\\.github$",
      "(?i)(?:^|[\\\\\\\\/])\\\\.codex$",
      "(?i)(?:^|[\\\\\\\\/])copilot-instructions$"
    ]
  }}
}}"#
        ),
    );
}

fn write_settings_template(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".vscode/settings.tamplate.jsonc"),
        r#"{
  // global workspace defaults
  "git.autofetch": false,
  "extensions.autoUpdate": false,
  "files.exclude": {
    "**/.git": true,
  },
}"#,
    );
}

fn write_workspace(repo_root: &std::path::Path, relative_path: &str, contents: &str) {
    write_file(&repo_root.join(relative_path), contents);
}

#[test]
fn test_invoke_validate_workspace_efficiency_passes_for_valid_workspace() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path(), &[".vscode/base.code-workspace"]);
    write_settings_template(repo.path());
    write_workspace(
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

    let result = invoke_validate_workspace_efficiency(&ValidateWorkspaceEfficiencyRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateWorkspaceEfficiencyRequest::default()
    })
    .expect("workspace validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.workspace_files_checked, 1);
    assert!(result.failures.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn test_invoke_validate_workspace_efficiency_allows_declared_template_workspace() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path(), &["template-base.code-workspace"]);
    write_settings_template(repo.path());
    write_workspace(
        repo.path(),
        "template-base.code-workspace",
        r#"{
  "folders": [],
  "extensions": {
    "recommendations": [
      "mhutchie.git-graph"
    ]
  }
}"#,
    );

    let result = invoke_validate_workspace_efficiency(&ValidateWorkspaceEfficiencyRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateWorkspaceEfficiencyRequest::default()
    })
    .expect("workspace validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Passed);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.workspace_files_checked, 1);
}

#[test]
fn test_invoke_validate_workspace_efficiency_reports_missing_settings_object() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path(), &[".vscode/base.code-workspace"]);
    write_settings_template(repo.path());
    write_workspace(
        repo.path(),
        "missing-settings.code-workspace",
        r#"{
  "folders": [
    { "path": "App" }
  ]
}"#,
    );

    let result = invoke_validate_workspace_efficiency(&ValidateWorkspaceEfficiencyRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateWorkspaceEfficiencyRequest::default()
    })
    .expect("workspace validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Workspace must define a settings object")));
}

#[test]
fn test_invoke_validate_workspace_efficiency_reports_forbidden_setting() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path(), &[".vscode/base.code-workspace"]);
    write_settings_template(repo.path());
    write_workspace(
        repo.path(),
        "forbidden-setting.code-workspace",
        r#"{
  "folders": [
    { "path": "App" }
  ],
  "settings": {
    "git.openRepositoryInParentFolders": "always",
    "chat.agent.maxRequests": 80
  }
}"#,
    );

    let result = invoke_validate_workspace_efficiency(&ValidateWorkspaceEfficiencyRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateWorkspaceEfficiencyRequest::default()
    })
    .expect("workspace validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result.failures.iter().any(|message| {
        message
            .contains("Workspace setting 'git.openRepositoryInParentFolders' must not be 'always'")
    }));
}

#[test]
fn test_invoke_validate_workspace_efficiency_reports_duplicate_folder_paths() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path(), &[".vscode/base.code-workspace"]);
    write_settings_template(repo.path());
    write_workspace(
        repo.path(),
        "duplicate-paths.code-workspace",
        r#"{
  "folders": [
    { "path": "App" },
    { "path": "./App" }
  ],
  "settings": {
    "chat.agent.maxRequests": 80
  }
}"#,
    );

    let result = invoke_validate_workspace_efficiency(&ValidateWorkspaceEfficiencyRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateWorkspaceEfficiencyRequest::default()
    })
    .expect("workspace validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result
        .failures
        .iter()
        .any(|message| message.contains("Workspace contains duplicate folder path")));
}

#[test]
fn test_invoke_validate_workspace_efficiency_emits_heuristic_warnings_without_failing() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path(), &[".vscode/base.code-workspace"]);
    write_settings_template(repo.path());
    write_workspace(
        repo.path(),
        "warning-only.code-workspace",
        r#"{
  "folders": [
    { "path": ".codex" },
    { "path": ".github" },
    { "path": "copilot-instructions" },
    { "path": "AppA" },
    { "path": "AppB" }
  ],
  "settings": {
    "chat.agent.maxRequests": 80
  }
}"#,
    );

    let result = invoke_validate_workspace_efficiency(&ValidateWorkspaceEfficiencyRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateWorkspaceEfficiencyRequest::default()
    })
    .expect("workspace validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Warning);
    assert_eq!(result.exit_code, 0);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("recommended maximum is 4")));
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("Workspace mixes") && message.contains("product folders")));
    assert!(result
        .warnings
        .iter()
        .any(|message| message.contains("shared AI/config folders with product code")));
}

#[test]
fn test_invoke_validate_workspace_efficiency_reports_redundant_workspace_setting() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_layout(repo.path());
    write_baseline(repo.path(), &[".vscode/base.code-workspace"]);
    write_settings_template(repo.path());
    write_workspace(
        repo.path(),
        "redundant-setting.code-workspace",
        r#"{
  "folders": [
    { "path": "App" }
  ],
  "settings": {
    "extensions.autoUpdate": false,
    "chat.agent.maxRequests": 80
  }
}"#,
    );

    let result = invoke_validate_workspace_efficiency(&ValidateWorkspaceEfficiencyRequest {
        repo_root: Some(repo.path().to_path_buf()),
        warning_only: false,
        ..ValidateWorkspaceEfficiencyRequest::default()
    })
    .expect("workspace validation should execute");

    assert_eq!(result.status, ValidationCheckStatus::Failed);
    assert_eq!(result.exit_code, 1);
    assert!(result.failures.iter().any(|message| {
        message
            .contains("Workspace setting 'extensions.autoUpdate' is redundant in workspace scope")
    }));
}