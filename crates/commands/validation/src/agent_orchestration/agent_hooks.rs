//! Repository-owned VS Code agent hook validation.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use serde_json::Value;

use crate::error::ValidateAgentHooksCommandError;
use crate::operational_hygiene::common::{derive_status, push_required_finding};
use crate::ValidationCheckStatus;

const DEFAULT_HOOKS_ROOT: &str = ".github/hooks";

/// Request payload for `validate-agent-hooks`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAgentHooksRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit hooks root.
    pub hooks_root: Option<PathBuf>,
    /// Convert required findings to warnings instead of failures.
    pub warning_only: bool,
}

impl Default for ValidateAgentHooksRequest {
    fn default() -> Self {
        Self {
            repo_root: None,
            hooks_root: None,
            warning_only: true,
        }
    }
}

/// Result payload for `validate-agent-hooks`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateAgentHooksResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Effective warning-only mode.
    pub warning_only: bool,
    /// Resolved hooks root.
    pub hooks_root: PathBuf,
    /// Warning messages emitted by the command.
    pub warnings: Vec<String>,
    /// Failure messages emitted by the command.
    pub failures: Vec<String>,
    /// Final command status.
    pub status: ValidationCheckStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Run the repository-owned agent hook validation.
///
/// # Errors
///
/// Returns [`ValidateAgentHooksCommandError`] when the repository root cannot be resolved.
pub fn invoke_validate_agent_hooks(
    request: &ValidateAgentHooksRequest,
) -> Result<ValidateAgentHooksResult, ValidateAgentHooksCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        ValidateAgentHooksCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| ValidateAgentHooksCommandError::ResolveWorkspaceRoot { source })?;
    let hooks_root = match request.hooks_root.as_deref() {
        Some(path) => resolve_full_path(&repo_root, path),
        None => repo_root.join(DEFAULT_HOOKS_ROOT),
    };

    let mut warnings = Vec::new();
    let mut failures = Vec::new();
    let scripts_root = hooks_root.join("scripts");

    if !hooks_root.is_dir() {
        push_required_finding(
            request.warning_only,
            &mut warnings,
            &mut failures,
            "Missing .github/hooks directory.".to_string(),
        );
    }

    let bootstrap_document = read_json_document(
        &hooks_root.join("super-agent.bootstrap.json"),
        ".github/hooks/super-agent.bootstrap.json",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    let selector_document = read_json_document(
        &hooks_root.join("super-agent.selector.json"),
        ".github/hooks/super-agent.selector.json",
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    validate_bootstrap_document(
        bootstrap_document.as_ref(),
        &scripts_root,
        request.warning_only,
        &mut warnings,
        &mut failures,
    );
    validate_selector_document(
        selector_document.as_ref(),
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    for required_script in [
        "common.ps1",
        "session-start.ps1",
        "pre-tool-use.ps1",
        "subagent-start.ps1",
    ] {
        let script_path = scripts_root.join(required_script);
        if !script_path.is_file() {
            push_required_finding(
                request.warning_only,
                &mut warnings,
                &mut failures,
                format!("Missing hook helper script: .github/hooks/scripts/{required_script}"),
            );
        }
    }

    validate_common_hook_contract(
        &repo_root,
        &scripts_root.join("common.ps1"),
        request.warning_only,
        &mut warnings,
        &mut failures,
    );

    let status = derive_status(&warnings, &failures);
    let exit_code = if failures.is_empty() { 0 } else { 1 };

    Ok(ValidateAgentHooksResult {
        repo_root,
        warning_only: request.warning_only,
        hooks_root,
        warnings,
        failures,
        status,
        exit_code,
    })
}

fn read_json_document(
    path: &Path,
    label: &str,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) -> Option<Value> {
    if !path.is_file() {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            format!("Missing required hook file {label}."),
        );
        return None;
    }

    let document = match fs::read_to_string(path) {
        Ok(document) => document,
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("{label} is not readable: {error}"),
            );
            return None;
        }
    };

    match serde_json::from_str::<Value>(&document) {
        Ok(value) => Some(value),
        Err(error) => {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!("{label} is not valid JSON: {error}"),
            );
            None
        }
    }
}

fn validate_bootstrap_document(
    document: Option<&Value>,
    scripts_root: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let Some(document) = document else {
        return;
    };

    let expected_events = [
        ("SessionStart", "session-start.ps1"),
        ("PreToolUse", "pre-tool-use.ps1"),
        ("SubagentStart", "subagent-start.ps1"),
    ];
    for (event_name, script_name) in expected_events {
        let entries = document
            .get("hooks")
            .and_then(|hooks| hooks.get(event_name))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if entries.is_empty() {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Hook event '{event_name}' is missing from .github/hooks/super-agent.bootstrap.json."
                ),
            );
            continue;
        }

        for entry in entries {
            if entry.get("type").and_then(Value::as_str) != Some("command") {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!("Hook event '{event_name}' must use type 'command'."),
                );
            }
            if entry
                .get("command")
                .and_then(Value::as_str)
                .is_none_or(str::is_empty)
            {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!("Hook event '{event_name}' must define a command."),
                );
            }

            let expected_script_path = scripts_root.join(script_name);
            if !expected_script_path.is_file() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    format!("Referenced hook script missing: .github/hooks/scripts/{script_name}"),
                );
            }
        }
    }
}

fn validate_selector_document(
    document: Option<&Value>,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    let Some(document) = document else {
        return;
    };

    for path in [
        "defaultAgent.skillName",
        "defaultAgent.displayName",
        "overrideSources.environment.skillVariable",
        "overrideSources.environment.displayVariable",
        "overrideSources.localOverrideFile",
    ] {
        if value_at_json_path(document, path)
            .and_then(Value::as_str)
            .is_none_or(str::is_empty)
        {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(".github/hooks/super-agent.selector.json must define {path}."),
            );
        }
    }
}

fn validate_common_hook_contract(
    repo_root: &Path,
    common_script_path: &Path,
    warning_only: bool,
    warnings: &mut Vec<String>,
    failures: &mut Vec<String>,
) {
    if !common_script_path.is_file() {
        return;
    }

    let Ok(common_script_content) = fs::read_to_string(common_script_path) else {
        push_required_finding(
            warning_only,
            warnings,
            failures,
            "Could not read hook helper script: .github/hooks/scripts/common.ps1".to_string(),
        );
        return;
    };

    let canonical_content =
        if common_script_content.contains("Resolve-ProjectedRuntimeHookScriptPath") {
            let canonical_path = repo_root.join("scripts/runtime/hooks/common.ps1");
            if !canonical_path.is_file() {
                push_required_finding(
                    warning_only,
                    warnings,
                    failures,
                    "Canonical runtime hook helper missing: scripts/runtime/hooks/common.ps1."
                        .to_string(),
                );
                return;
            }

            match fs::read_to_string(canonical_path) {
                Ok(content) => content,
                Err(error) => {
                    push_required_finding(
                        warning_only,
                        warnings,
                        failures,
                        format!("Could not read canonical runtime hook helper: {error}"),
                    );
                    return;
                }
            }
        } else {
            common_script_content
        };

    for required_marker in [
        "workspace-adapter",
        "global-runtime",
        ".build/super-agent/planning/active",
        ".build/super-agent/specs/active",
    ] {
        if !canonical_content.contains(required_marker) {
            push_required_finding(
                warning_only,
                warnings,
                failures,
                format!(
                    "Hook helper contract missing required marker '{required_marker}' in .github/hooks/scripts/common.ps1."
                ),
            );
        }
    }
}

fn value_at_json_path<'a>(document: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = document;
    for segment in path.split('.') {
        current = current.get(segment)?;
    }
    Some(current)
}