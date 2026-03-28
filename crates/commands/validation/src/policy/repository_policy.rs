//! Repository policy file validation.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::{Map, Value};

use crate::agent_orchestration::common::{
    read_required_json_value, resolve_repo_relative_path, resolve_validation_repo_root,
};
use crate::error::ValidatePolicyCommandError;
use crate::operational_hygiene::common::derive_status;
use crate::ValidationCheckStatus;

const DEFAULT_POLICY_DIRECTORY: &str = ".github/policies";
const ALLOWED_KEYS: &[&str] = &[
    "id",
    "description",
    "requiredFiles",
    "requiredDirectories",
    "forbiddenFiles",
    "requiredGitHooks",
    "requiredGitConfig",
];

/// Request payload for `validate-policy`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatePolicyRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit policy directory.
    pub policy_directory: Option<PathBuf>,
}

impl Default for ValidatePolicyRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            policy_directory: None,
        }
    }
}

/// Result payload for `validate-policy`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatePolicyResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved policy directory.
    pub policy_directory: PathBuf,
    /// Number of policy files checked.
    pub policies_checked: usize,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Run the repository policy validation sweep.
///
/// # Errors
///
/// Returns [`ValidatePolicyCommandError`] when the repository root cannot be
/// resolved.
pub fn invoke_validate_policy(
    request: &ValidatePolicyRequest,
) -> Result<ValidatePolicyResult, ValidatePolicyCommandError> {
    let repo_root = resolve_validation_repo_root(request.repo_root.as_deref())
        .map_err(|source| ValidatePolicyCommandError::ResolveWorkspaceRoot { source })?;
    let policy_directory = resolve_repo_relative_path(
        &repo_root,
        request.policy_directory.as_deref(),
        DEFAULT_POLICY_DIRECTORY,
    );

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let mut policies_checked = 0usize;
    let policy_directory_label = request.policy_directory.as_ref().map_or_else(
        || DEFAULT_POLICY_DIRECTORY.to_string(),
        |path| path.to_string_lossy().to_string(),
    );

    if !policy_directory.is_dir() {
        failures.push(format!(
            "Policy directory not found: {policy_directory_label}"
        ));
    } else {
        let mut policy_files = fs::read_dir(&policy_directory)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "json"))
                    .map(|entry| entry.path())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        policy_files.sort();

        if policy_files.is_empty() {
            failures.push(format!(
                "No policy files found in: {policy_directory_label}"
            ));
        } else {
            policies_checked = policy_files.len();
            for policy_file in policy_files {
                let relative_path = policy_file
                    .strip_prefix(&repo_root)
                    .map_or_else(
                        |_| policy_file.display().to_string(),
                        |path| path.display().to_string(),
                    )
                    .replace('\\', "/");
                let Some(policy_value) = read_required_json_value(
                    &policy_file,
                    &format!("policy file {relative_path}"),
                    false,
                    &mut warnings,
                    &mut failures,
                ) else {
                    continue;
                };
                let Some(policy_object) = policy_value.as_object() else {
                    failures.push(format!(
                        "Invalid JSON in policy file {relative_path} :: expected object root"
                    ));
                    continue;
                };

                validate_policy_contract(
                    &repo_root,
                    &relative_path,
                    policy_object,
                    &mut warnings,
                    &mut failures,
                );
            }
        }
    }

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidatePolicyResult {
        repo_root,
        policy_directory,
        policies_checked,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn validate_policy_contract(
    repo_root: &std::path::Path,
    policy_path: &str,
    policy_object: &Map<String, Value>,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let policy_id = policy_object
        .get("id")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| {
            std::path::Path::new(policy_path).file_stem().map_or_else(
                || "unnamed-policy".to_string(),
                |stem| stem.to_string_lossy().to_string(),
            )
        });

    for key in policy_object.keys() {
        if !ALLOWED_KEYS.contains(&key.as_str()) {
            warnings.push(format!(
                "Policy has unknown key '{}' in {}",
                key, policy_path
            ));
        }
    }

    for relative_path in value_as_string_array(policy_object.get("requiredFiles")) {
        if !repo_root.join(&relative_path).is_file() {
            failures.push(format!(
                "Missing required file '{}' (policy: {})",
                relative_path, policy_id
            ));
        }
    }
    for relative_path in value_as_string_array(policy_object.get("requiredDirectories")) {
        if !repo_root.join(&relative_path).is_dir() {
            failures.push(format!(
                "Missing required directory '{}' (policy: {})",
                relative_path, policy_id
            ));
        }
    }
    for relative_path in value_as_string_array(policy_object.get("forbiddenFiles")) {
        if repo_root.join(&relative_path).is_file() {
            failures.push(format!(
                "Forbidden file present '{}' (policy: {})",
                relative_path, policy_id
            ));
        }
    }
    for hook_name in value_as_string_array(policy_object.get("requiredGitHooks")) {
        if !repo_root.join(".githooks").join(&hook_name).is_file() {
            failures.push(format!(
                "Missing required git hook '.githooks/{}' (policy: {})",
                hook_name, policy_id
            ));
        }
    }

    if let Some(required_git_config) = policy_object
        .get("requiredGitConfig")
        .and_then(Value::as_object)
    {
        if Command::new("git").arg("--version").output().is_err() {
            warnings.push(format!(
                "Git command not found; skipping requiredGitConfig checks (policy: {})",
                policy_id
            ));
        } else {
            for (key, value) in required_git_config {
                let expected_value = value
                    .as_str()
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| value.to_string());
                let output = Command::new("git")
                    .arg("-C")
                    .arg(repo_root)
                    .arg("config")
                    .arg("--local")
                    .arg("--get")
                    .arg(key)
                    .output();
                match output {
                    Ok(output) if output.status.success() => {
                        let current_value =
                            String::from_utf8_lossy(&output.stdout).trim().to_string();
                        if current_value.is_empty() {
                            failures.push(format!(
                                "Missing required git config '{}' (policy: {})",
                                key, policy_id
                            ));
                        } else if current_value != expected_value {
                            failures.push(format!(
                                "Git config '{}' expected '{}' but found '{}' (policy: {})",
                                key, expected_value, current_value, policy_id
                            ));
                        }
                    }
                    _ => failures.push(format!(
                        "Missing required git config '{}' (policy: {})",
                        key, policy_id
                    )),
                }
            }
        }
    }
}

fn value_as_string_array(value: Option<&Value>) -> Vec<String> {
    match value {
        None => Vec::new(),
        Some(Value::String(text)) => vec![text.to_string()],
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect(),
        Some(other) => vec![other.to_string()],
    }
}