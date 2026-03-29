//! Minimal `.editorconfig` helpers for file-level EOF policy lookups.

use anyhow::{Context, Result};
use globset::Glob;
use std::fs;
use std::path::Path;

/// Resolve the effective `insert_final_newline` policy for a target file.
///
/// The lookup is intentionally minimal and only supports the subset of
/// EditorConfig patterns currently used by this repository. Later matching
/// sections override earlier ones.
///
/// # Errors
///
/// Returns an error when `.editorconfig` exists but cannot be read or when a
/// matching glob pattern is invalid.
pub fn resolve_insert_final_newline_policy(
    workspace_root: &Path,
    target_path: &Path,
) -> Result<Option<bool>> {
    let editorconfig_path = workspace_root.join(".editorconfig");
    if !editorconfig_path.is_file() {
        return Ok(None);
    }

    let document = fs::read_to_string(&editorconfig_path)
        .with_context(|| format!("failed to read '{}'", editorconfig_path.display()))?;
    let relative_path = target_path
        .strip_prefix(workspace_root)
        .unwrap_or(target_path)
        .to_string_lossy()
        .replace('\\', "/");
    let file_name = target_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    let mut effective_policy = None;
    let mut active_section_pattern: Option<String> = None;

    for raw_line in document.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            active_section_pattern = Some(line[1..line.len() - 1].trim().to_string());
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if key.trim() != "insert_final_newline" {
            continue;
        }

        let Some(insert_final_newline) = parse_bool(value.trim()) else {
            continue;
        };

        let Some(section_pattern) = active_section_pattern.as_deref() else {
            effective_policy = Some(insert_final_newline);
            continue;
        };

        if editorconfig_pattern_matches(section_pattern, &relative_path, file_name)? {
            effective_policy = Some(insert_final_newline);
        }
    }

    Ok(effective_policy)
}

fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn editorconfig_pattern_matches(
    pattern: &str,
    relative_path: &str,
    file_name: &str,
) -> Result<bool> {
    for expanded_pattern in expand_brace_pattern(pattern) {
        let normalized_pattern = expanded_pattern.replace('\\', "/");
        let matches = if normalized_pattern.contains('/') {
            compile_glob(&normalized_pattern)?
                .compile_matcher()
                .is_match(relative_path)
        } else {
            compile_glob(&normalized_pattern)?
                .compile_matcher()
                .is_match(file_name)
        };

        if matches {
            return Ok(true);
        }
    }

    Ok(false)
}

fn compile_glob(pattern: &str) -> Result<Glob> {
    Glob::new(pattern).with_context(|| format!("invalid .editorconfig glob pattern '{pattern}'"))
}

fn expand_brace_pattern(pattern: &str) -> Vec<String> {
    let Some(open_index) = pattern.find('{') else {
        return vec![pattern.to_string()];
    };
    let Some(close_index) = pattern[open_index + 1..].find('}') else {
        return vec![pattern.to_string()];
    };

    let close_index = open_index + 1 + close_index;
    let prefix = &pattern[..open_index];
    let suffix = &pattern[close_index + 1..];

    pattern[open_index + 1..close_index]
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("{prefix}{value}{suffix}"))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::resolve_insert_final_newline_policy;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_resolve_insert_final_newline_policy_prefers_last_matching_section() {
        let workspace = TempDir::new().expect("temp dir should exist");
        fs::write(
            workspace.path().join(".editorconfig"),
            "root = true\n\n[*]\ninsert_final_newline = false\n\n[*.{rs,toml,lock}]\ninsert_final_newline = true\n",
        )
        .expect(".editorconfig should be written");
        fs::create_dir_all(workspace.path().join("src")).expect("src directory should exist");
        fs::write(workspace.path().join("src/lib.rs"), "pub fn sample() {}\n")
            .expect("rust file should exist");
        fs::write(workspace.path().join("README.md"), "# docs").expect("readme should exist");

        let rust_policy = resolve_insert_final_newline_policy(
            workspace.path(),
            &workspace.path().join("src/lib.rs"),
        )
        .expect("policy resolution should succeed");
        let readme_policy = resolve_insert_final_newline_policy(
            workspace.path(),
            &workspace.path().join("README.md"),
        )
        .expect("policy resolution should succeed");

        assert_eq!(rust_policy, Some(true));
        assert_eq!(readme_policy, Some(false));
    }

    #[test]
    fn test_resolve_insert_final_newline_policy_returns_none_without_editorconfig() {
        let workspace = TempDir::new().expect("temp dir should exist");
        fs::write(workspace.path().join("notes.md"), "alpha").expect("file should exist");

        let policy = resolve_insert_final_newline_policy(
            workspace.path(),
            &workspace.path().join("notes.md"),
        )
        .expect("policy resolution should succeed");

        assert_eq!(policy, None);
    }
}
