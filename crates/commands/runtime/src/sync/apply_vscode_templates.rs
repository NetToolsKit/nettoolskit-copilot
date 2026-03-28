//! Runtime VS Code template application for workspace-managed files.

use anyhow::Context;
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::RuntimeApplyVscodeTemplatesCommandError;

/// Request payload for `apply-vscode-templates`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeApplyVscodeTemplatesRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit VS Code workspace folder path.
    pub vscode_path: Option<PathBuf>,
    /// Overwrite existing target files.
    pub force: bool,
    /// Skip applying `settings.tamplate.jsonc`.
    pub skip_settings: bool,
    /// Skip applying `mcp.tamplate.jsonc`.
    pub skip_mcp: bool,
}

/// One template application result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeApplyVscodeTemplateFileResult {
    /// Logical template name.
    pub name: String,
    /// Canonical source template path.
    pub source_path: PathBuf,
    /// Canonical target file path.
    pub target_path: PathBuf,
    /// Whether the file was written.
    pub applied: bool,
    /// Whether the file was skipped because the target already existed.
    pub skipped: bool,
}

/// Result payload for `apply-vscode-templates`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeApplyVscodeTemplatesResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved VS Code workspace folder.
    pub vscode_path: PathBuf,
    /// Number of templates applied.
    pub applied_count: usize,
    /// Number of templates skipped.
    pub skipped_count: usize,
    /// File-level results for every attempted template.
    pub files: Vec<RuntimeApplyVscodeTemplateFileResult>,
}

/// Apply tracked VS Code workspace templates into active files.
///
/// # Errors
///
/// Returns [`RuntimeApplyVscodeTemplatesCommandError`] when repository or
/// VS Code path resolution fails, when the VS Code folder does not exist, or
/// when a template cannot be copied.
pub fn invoke_apply_vscode_templates(
    request: &RuntimeApplyVscodeTemplatesRequest,
) -> Result<RuntimeApplyVscodeTemplatesResult, RuntimeApplyVscodeTemplatesCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeApplyVscodeTemplatesCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(
            |source| RuntimeApplyVscodeTemplatesCommandError::ResolveWorkspaceRoot { source },
        )?;
    let vscode_path = resolve_vscode_path(&repo_root, request.vscode_path.as_deref())
        .map_err(|source| RuntimeApplyVscodeTemplatesCommandError::ResolveVscodePath { source })?;

    if !vscode_path.is_dir() {
        return Err(
            RuntimeApplyVscodeTemplatesCommandError::VscodePathNotFound {
                vscode_path: vscode_path.display().to_string(),
            },
        );
    }

    let mut files = Vec::new();

    if !request.skip_settings {
        files.push(
            copy_template_file(
                "settings",
                &vscode_path.join("settings.tamplate.jsonc"),
                &vscode_path.join("settings.json"),
                request.force,
            )
            .map_err(|source| RuntimeApplyVscodeTemplatesCommandError::ApplyTemplates { source })?,
        );
    }

    if !request.skip_mcp {
        files.push(
            copy_template_file(
                "mcp",
                &vscode_path.join("mcp.tamplate.jsonc"),
                &vscode_path.join("mcp.json"),
                request.force,
            )
            .map_err(|source| RuntimeApplyVscodeTemplatesCommandError::ApplyTemplates { source })?,
        );
    }

    let applied_count = files.iter().filter(|file| file.applied).count();
    let skipped_count = files.iter().filter(|file| file.skipped).count();

    Ok(RuntimeApplyVscodeTemplatesResult {
        repo_root,
        vscode_path,
        applied_count,
        skipped_count,
        files,
    })
}

fn resolve_vscode_path(repo_root: &Path, requested_path: Option<&Path>) -> anyhow::Result<PathBuf> {
    match requested_path {
        Some(path) if path.is_absolute() => Ok(path.to_path_buf()),
        Some(path) => Ok(resolve_full_path(repo_root, path)),
        None => Ok(repo_root.join(".vscode")),
    }
}

fn copy_template_file(
    name: &str,
    source_path: &Path,
    target_path: &Path,
    overwrite: bool,
) -> anyhow::Result<RuntimeApplyVscodeTemplateFileResult> {
    if !source_path.is_file() {
        return Err(anyhow::anyhow!(
            "template file not found: {}",
            source_path.display()
        ));
    }

    if target_path.exists() && !overwrite {
        return Ok(RuntimeApplyVscodeTemplateFileResult {
            name: name.to_string(),
            source_path: source_path.to_path_buf(),
            target_path: target_path.to_path_buf(),
            applied: false,
            skipped: true,
        });
    }

    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }

    fs::copy(source_path, target_path).with_context(|| {
        format!(
            "failed to copy '{}' to '{}'",
            source_path.display(),
            target_path.display()
        )
    })?;

    Ok(RuntimeApplyVscodeTemplateFileResult {
        name: name.to_string(),
        source_path: source_path.to_path_buf(),
        target_path: target_path.to_path_buf(),
        applied: true,
        skipped: false,
    })
}
