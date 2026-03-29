//! Tests for runtime `PreToolUse` hook normalization.

use nettoolskit_runtime::{invoke_pre_tool_use, RuntimePreToolUseRequest};
use serde_json::json;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn new_workspace() -> TempDir {
    let workspace = TempDir::new().expect("temporary workspace should be created");
    write_file(
        &workspace.path().join(".editorconfig"),
        "root = true\n\n[*]\ninsert_final_newline = false\n\n[*.{rs,toml,lock}]\ninsert_final_newline = true\n",
    );
    workspace
}

#[test]
fn test_invoke_pre_tool_use_normalizes_create_file_for_default_false_policy() {
    let workspace = new_workspace();
    let result = invoke_pre_tool_use(&RuntimePreToolUseRequest {
        workspace_path: Some(workspace.path().to_path_buf()),
        tool_name: Some("createFile".to_string()),
        tool_input: Some(json!({
            "filePath": "README.md",
            "content": "# Title\n"
        })),
    })
    .expect("pre-tool-use should execute");

    assert_eq!(result.hook_event_name, "PreToolUse");
    assert_eq!(
        result.additional_context.as_deref(),
        Some("Workspace EOF policy: preserve exact file EOF. The repository default uses insert_final_newline = false, and narrower .editorconfig overrides may require a terminal newline for specific file types.")
    );
    assert_eq!(
        result.updated_input,
        Some(json!({
            "filePath": "README.md",
            "content": "# Title"
        }))
    );
}

#[test]
fn test_invoke_pre_tool_use_keeps_rust_file_newline_when_override_requires_it() {
    let workspace = new_workspace();
    let result = invoke_pre_tool_use(&RuntimePreToolUseRequest {
        workspace_path: Some(workspace.path().to_path_buf()),
        tool_name: Some("createFile".to_string()),
        tool_input: Some(json!({
            "filePath": "src/lib.rs",
            "content": "pub fn sample() {}\n"
        })),
    })
    .expect("pre-tool-use should execute");

    assert_eq!(
        result.additional_context.as_deref(),
        Some("Workspace EOF policy: preserve exact file EOF. The repository default uses insert_final_newline = false, and narrower .editorconfig overrides may require a terminal newline for specific file types.")
    );
    assert_eq!(result.updated_input, None);
}

#[test]
fn test_invoke_pre_tool_use_normalizes_multi_replace_string_only_for_managed_false_policy_files() {
    let workspace = new_workspace();
    let outside_path = workspace.path().join("..").join("external.md");
    let outside_text = outside_path.to_string_lossy().to_string();

    let result = invoke_pre_tool_use(&RuntimePreToolUseRequest {
        workspace_path: Some(workspace.path().to_path_buf()),
        tool_name: Some("multiReplaceString".to_string()),
        tool_input: Some(json!({
            "replacements": [
                {
                    "filePath": "docs/guide.md",
                    "newString": "guide body\n"
                },
                {
                    "filePath": "src/lib.rs",
                    "newString": "pub fn sample() {}\n"
                },
                {
                    "filePath": outside_text,
                    "newString": "outside\n"
                }
            ]
        })),
    })
    .expect("pre-tool-use should execute");

    assert_eq!(
        result.updated_input,
        Some(json!({
            "replacements": [
                {
                    "filePath": "docs/guide.md",
                    "newString": "guide body"
                },
                {
                    "filePath": "src/lib.rs",
                    "newString": "pub fn sample() {}\n"
                },
                {
                    "filePath": outside_text,
                    "newString": "outside\n"
                }
            ]
        }))
    );
}

#[test]
fn test_invoke_pre_tool_use_ignores_non_editing_tools() {
    let workspace = new_workspace();
    let result = invoke_pre_tool_use(&RuntimePreToolUseRequest {
        workspace_path: Some(workspace.path().to_path_buf()),
        tool_name: Some("listFiles".to_string()),
        tool_input: Some(json!({
            "path": "."
        })),
    })
    .expect("pre-tool-use should execute");

    assert_eq!(result.hook_event_name, "PreToolUse");
    assert_eq!(result.additional_context, None);
    assert_eq!(result.updated_input, None);
}
