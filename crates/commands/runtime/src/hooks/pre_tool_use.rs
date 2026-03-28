//! Runtime `PreToolUse` hook payload normalization.

use nettoolskit_core::editorconfig::resolve_insert_final_newline_policy;
use serde_json::Value;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::error::RuntimePreToolUseCommandError;

/// Request payload for the runtime `PreToolUse` hook.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimePreToolUseRequest {
    /// Optional workspace root used to resolve `.editorconfig` policy.
    pub workspace_path: Option<PathBuf>,
    /// Hook tool name.
    pub tool_name: Option<String>,
    /// Raw hook tool input payload.
    pub tool_input: Option<Value>,
}

/// Result payload for the runtime `PreToolUse` hook.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimePreToolUseResult {
    /// Hook event name expected by the runtime.
    pub hook_event_name: String,
    /// Additional workspace context for the hook runtime.
    pub additional_context: Option<String>,
    /// Updated tool input when EOF normalization changed the payload.
    pub updated_input: Option<Value>,
}

/// Normalize EOF-sensitive hook payloads for the runtime `PreToolUse` event.
///
/// # Errors
///
/// Returns [`RuntimePreToolUseCommandError`] when the workspace `.editorconfig`
/// policy cannot be resolved for a managed file path.
pub fn invoke_pre_tool_use(
    request: &RuntimePreToolUseRequest,
) -> Result<RuntimePreToolUseResult, RuntimePreToolUseCommandError> {
    let tool_name = request.tool_name.as_deref().unwrap_or_default();
    let additional_context = if is_eof_sensitive_tool_name(tool_name) {
        Some(
            build_workspace_eof_policy_message(request.workspace_path.as_deref()).map_err(
                |source| RuntimePreToolUseCommandError::ResolveWorkspacePolicy { source },
            )?,
        )
    } else {
        None
    };
    let updated_input = if is_eof_sensitive_tool_name(tool_name) {
        normalize_tool_input(
            request.workspace_path.as_deref(),
            tool_name,
            request.tool_input.as_ref(),
        )
        .map_err(|source| RuntimePreToolUseCommandError::ResolveWorkspacePolicy { source })?
    } else {
        None
    };

    Ok(RuntimePreToolUseResult {
        hook_event_name: "PreToolUse".to_string(),
        additional_context,
        updated_input,
    })
}

fn build_workspace_eof_policy_message(workspace_path: Option<&Path>) -> anyhow::Result<String> {
    let Some(workspace_path) = workspace_path else {
        return Ok("Workspace EOF policy: preserve the current EOF state of touched files and do not change terminal newline behavior unless the workspace defines a narrower rule.".to_string());
    };

    let summary = load_insert_final_newline_policy_summary(workspace_path)?;
    Ok(match (summary.default_policy, summary.has_mixed_policy) {
        (Some(false), true) => "Workspace EOF policy: preserve exact file EOF. The repository default uses insert_final_newline = false, and narrower .editorconfig overrides may require a terminal newline for specific file types.".to_string(),
        (Some(true), true) => "Workspace EOF policy: preserve exact file EOF. The repository default uses insert_final_newline = true, and narrower .editorconfig overrides may omit the terminal newline for specific file types.".to_string(),
        (None, true) => "Workspace EOF policy: preserve exact file EOF. The workspace uses mixed .editorconfig insert_final_newline rules, so keep the file-specific terminal newline behavior.".to_string(),
        (Some(false), false) => "Workspace EOF policy: preserve exact file EOF, and do not append a terminal newline because .editorconfig uses insert_final_newline = false.".to_string(),
        (Some(true), false) => "Workspace EOF policy: preserve exact file EOF and keep a terminal newline where .editorconfig uses insert_final_newline = true.".to_string(),
        (None, false) => "Workspace EOF policy: preserve the current EOF state of touched files and do not change terminal newline behavior unless the workspace defines a narrower rule.".to_string(),
    })
}

fn normalize_tool_input(
    workspace_path: Option<&Path>,
    tool_name: &str,
    tool_input: Option<&Value>,
) -> anyhow::Result<Option<Value>> {
    let Some(workspace_path) = workspace_path else {
        return Ok(None);
    };
    let Some(mut updated_input) = tool_input.cloned() else {
        return Ok(None);
    };

    let mut changed = false;
    match tool_name {
        "createFile" => {
            changed |= normalize_root_string_field(
                workspace_path,
                &mut updated_input,
                "filePath",
                "content",
            )?;
        }
        "insertEdit" => {
            changed |= normalize_root_string_field(
                workspace_path,
                &mut updated_input,
                "filePath",
                "code",
            )?;
        }
        "replaceString" => {
            changed |= normalize_root_string_field(
                workspace_path,
                &mut updated_input,
                "filePath",
                "newString",
            )?;
        }
        "multiReplaceString" => {
            changed |= normalize_multi_replace_string(workspace_path, &mut updated_input)?;
        }
        _ => {}
    }

    if changed {
        Ok(Some(updated_input))
    } else {
        Ok(None)
    }
}

fn normalize_root_string_field(
    workspace_path: &Path,
    input: &mut Value,
    path_field: &str,
    content_field: &str,
) -> anyhow::Result<bool> {
    let Some(target_file_path) = input
        .get(path_field)
        .and_then(Value::as_str)
        .map(str::to_string)
    else {
        return Ok(false);
    };
    if !is_workspace_managed_file_path(workspace_path, &target_file_path) {
        return Ok(false);
    }
    if resolve_insert_final_newline_policy_for_tool_file(workspace_path, &target_file_path)?
        != Some(false)
    {
        return Ok(false);
    }

    let Some(content_value) = input
        .get(content_field)
        .and_then(Value::as_str)
        .map(str::to_string)
    else {
        return Ok(false);
    };
    let normalized = remove_terminal_newline(&content_value);
    if normalized == content_value {
        return Ok(false);
    }

    if let Some(object) = input.as_object_mut() {
        object.insert(content_field.to_string(), Value::String(normalized));
        return Ok(true);
    }

    Ok(false)
}

fn normalize_multi_replace_string(
    workspace_path: &Path,
    input: &mut Value,
) -> anyhow::Result<bool> {
    let Some(replacements) = input.get_mut("replacements").and_then(Value::as_array_mut) else {
        return Ok(false);
    };

    let mut changed = false;
    for replacement in replacements {
        let Some(target_file_path) = replacement
            .get("filePath")
            .and_then(Value::as_str)
            .map(str::to_string)
        else {
            continue;
        };
        if !is_workspace_managed_file_path(workspace_path, &target_file_path) {
            continue;
        }
        if resolve_insert_final_newline_policy_for_tool_file(workspace_path, &target_file_path)?
            != Some(false)
        {
            continue;
        }

        let Some(new_string) = replacement
            .get("newString")
            .and_then(Value::as_str)
            .map(str::to_string)
        else {
            continue;
        };
        let normalized = remove_terminal_newline(&new_string);
        if normalized == new_string {
            continue;
        }

        if let Some(object) = replacement.as_object_mut() {
            object.insert("newString".to_string(), Value::String(normalized));
            changed = true;
        }
    }

    Ok(changed)
}

fn resolve_insert_final_newline_policy_for_tool_file(
    workspace_path: &Path,
    file_path: &str,
) -> anyhow::Result<Option<bool>> {
    let target_path = resolve_candidate_file_path(workspace_path, file_path);
    resolve_insert_final_newline_policy(workspace_path, &target_path)
}

fn is_workspace_managed_file_path(workspace_path: &Path, file_path: &str) -> bool {
    let normalized_workspace = normalize_path(workspace_path);
    let candidate = resolve_candidate_file_path(workspace_path, file_path);
    candidate.starts_with(&normalized_workspace)
}

fn resolve_candidate_file_path(workspace_path: &Path, file_path: &str) -> PathBuf {
    let candidate = PathBuf::from(file_path);
    if candidate.is_absolute() {
        normalize_path(&candidate)
    } else {
        normalize_path(&workspace_path.join(candidate))
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }

    normalized
}

fn remove_terminal_newline(text: &str) -> String {
    text.trim_end_matches(['\r', '\n']).to_string()
}

fn is_eof_sensitive_tool_name(tool_name: &str) -> bool {
    matches!(
        tool_name,
        "applyPatch" | "createFile" | "insertEdit" | "replaceString" | "multiReplaceString"
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InsertFinalNewlinePolicySummary {
    default_policy: Option<bool>,
    has_mixed_policy: bool,
}

fn load_insert_final_newline_policy_summary(
    workspace_path: &Path,
) -> anyhow::Result<InsertFinalNewlinePolicySummary> {
    let editorconfig_path = workspace_path.join(".editorconfig");
    if !editorconfig_path.is_file() {
        return Ok(InsertFinalNewlinePolicySummary {
            default_policy: None,
            has_mixed_policy: false,
        });
    }

    let document = fs::read_to_string(&editorconfig_path)?;
    let mut current_section_pattern: Option<String> = None;
    let mut default_policy = None;
    let mut has_true_rule = false;
    let mut has_false_rule = false;

    for raw_line in document.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section_pattern = Some(line[1..line.len() - 1].trim().to_string());
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if key.trim() != "insert_final_newline" {
            continue;
        }

        let Some(policy) = parse_bool(value.trim()) else {
            continue;
        };
        if policy {
            has_true_rule = true;
        } else {
            has_false_rule = true;
        }

        if current_section_pattern.as_deref() == Some("*") {
            default_policy = Some(policy);
        }
    }

    Ok(InsertFinalNewlinePolicySummary {
        default_policy,
        has_mixed_policy: has_true_rule && has_false_rule,
    })
}

fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}
