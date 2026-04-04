//! Insert blank lines between adjacent C# `#endregion` and `#region` markers.

use anyhow::{anyhow, Context};
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::error::RuntimeFixRegionSpacingCommandError;

/// Request payload for `fix-region-spacing`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeFixRegionSpacingRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional file or directory path to scan.
    pub path: Option<PathBuf>,
    /// Report changes without writing files.
    pub dry_run: bool,
}

/// Runtime fix-region-spacing result status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeFixRegionSpacingStatus {
    /// Command updated one or more files or completed with no changes.
    Passed,
    /// Command only reported the files that would change.
    DryRun,
}

/// Result payload for `fix-region-spacing`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFixRegionSpacingResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved target path that was scanned.
    pub target_path: PathBuf,
    /// C# files discovered for processing.
    pub discovered_files: Vec<PathBuf>,
    /// Files whose contents changed or would change.
    pub changed_files: Vec<PathBuf>,
    /// Final command status.
    pub status: RuntimeFixRegionSpacingStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Insert blank lines between adjacent `#endregion` and `#region` directives.
///
/// # Errors
///
/// Returns [`RuntimeFixRegionSpacingCommandError`] when root resolution, file
/// discovery, or content normalization fails.
pub fn invoke_fix_region_spacing(
    request: &RuntimeFixRegionSpacingRequest,
) -> Result<RuntimeFixRegionSpacingResult, RuntimeFixRegionSpacingCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeFixRegionSpacingCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| RuntimeFixRegionSpacingCommandError::ResolveWorkspaceRoot { source })?;
    let target_path = resolve_target_path(&repo_root, request.path.as_deref())?;
    let discovered_files = discover_csharp_files(&target_path)
        .map_err(|source| RuntimeFixRegionSpacingCommandError::DiscoverFiles { source })?;

    let mut changed_files = Vec::new();
    for file_path in &discovered_files {
        let changed = normalize_region_spacing(file_path, request.dry_run)
            .map_err(|source| RuntimeFixRegionSpacingCommandError::NormalizeFiles { source })?;
        if changed {
            changed_files.push(file_path.clone());
        }
    }

    Ok(RuntimeFixRegionSpacingResult {
        repo_root,
        target_path,
        discovered_files,
        changed_files,
        status: if request.dry_run {
            RuntimeFixRegionSpacingStatus::DryRun
        } else {
            RuntimeFixRegionSpacingStatus::Passed
        },
        exit_code: 0,
    })
}

fn resolve_target_path(
    repo_root: &Path,
    requested_path: Option<&Path>,
) -> Result<PathBuf, RuntimeFixRegionSpacingCommandError> {
    let resolved = match requested_path {
        Some(path) => resolve_full_path(repo_root, path),
        None => repo_root.to_path_buf(),
    };

    if !resolved.exists() {
        return Err(RuntimeFixRegionSpacingCommandError::ResolveTargetPath {
            target_path: resolved.display().to_string(),
        });
    }

    Ok(resolved)
}

fn discover_csharp_files(target_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if target_path.is_file() {
        return Ok(if is_csharp_file(target_path) {
            vec![target_path.to_path_buf()]
        } else {
            Vec::new()
        });
    }

    let mut files = WalkDir::new(target_path)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("failed to enumerate '{}'", target_path.display()))?
        .into_iter()
        .filter(|entry| entry.file_type().is_file() && is_csharp_file(entry.path()))
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();

    files.sort();
    Ok(files)
}

fn is_csharp_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("cs"))
}

fn normalize_region_spacing(path: &Path, dry_run: bool) -> anyhow::Result<bool> {
    let original =
        fs::read_to_string(path).with_context(|| format!("failed to read '{}'", path.display()))?;
    let normalized = normalize_region_spacing_text(&original)?;
    if normalized == original {
        return Ok(false);
    }

    if !dry_run {
        fs::write(path, normalized)
            .with_context(|| format!("failed to write '{}'", path.display()))?;
    }

    Ok(true)
}

fn normalize_region_spacing_text(text: &str) -> anyhow::Result<String> {
    let preferred_newline = detect_preferred_newline(text);
    let pattern = Regex::new(r"(?m)(#endregion[^\r\n]*)\r?\n([ \t]*#region)")
        .map_err(|source| anyhow!("failed to compile region-spacing pattern: {source}"))?;
    let normalized = pattern.replace_all(text, |captures: &regex::Captures<'_>| {
        format!(
            "{}{preferred_newline}{preferred_newline}{}",
            &captures[1], &captures[2]
        )
    });
    Ok(normalized.into_owned())
}

fn detect_preferred_newline(text: &str) -> &'static str {
    if text.contains("\r\n") {
        "\r\n"
    } else if text.contains('\n') {
        "\n"
    } else if text.contains('\r') {
        "\r"
    } else {
        "\n"
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_region_spacing_text;

    #[test]
    fn test_normalize_region_spacing_text_inserts_blank_line_for_lf() {
        let original = "class Sample\n{\n    #endregion\n    #region Build\n}\n";
        let normalized =
            normalize_region_spacing_text(original).expect("normalization should succeed");
        assert_eq!(
            normalized,
            "class Sample\n{\n    #endregion\n\n    #region Build\n}\n"
        );
    }

    #[test]
    fn test_normalize_region_spacing_text_preserves_existing_blank_line() {
        let original = "class Sample\r\n{\r\n    #endregion\r\n\r\n    #region Build\r\n}\r\n";
        let normalized =
            normalize_region_spacing_text(original).expect("normalization should succeed");
        assert_eq!(normalized, original);
    }
}