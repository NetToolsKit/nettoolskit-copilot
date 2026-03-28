//! Clean build artifacts from the repository tree.

use anyhow::Context;
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use std::cmp::Reverse;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::RuntimeCleanBuildArtifactsCommandError;

const ARTIFACT_DIRECTORY_NAMES: &[&str] = &[".build", ".deployment", "bin", "obj"];

/// Request payload for `clean-build-artifacts`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeCleanBuildArtifactsRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional target path to clean.
    pub path: Option<PathBuf>,
    /// Skip the interactive confirmation contract and apply changes.
    pub force: bool,
    /// Return the planned removals without touching the filesystem.
    pub dry_run: bool,
}

/// Runtime clean-build-artifacts result status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeCleanBuildArtifactsStatus {
    /// Command removed the requested artifact directories.
    Passed,
    /// Command only reported the artifact directories that would be removed.
    DryRun,
    /// Command stopped because the caller did not approve deletion.
    ConfirmationRequired,
}

/// Result payload for `clean-build-artifacts`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeCleanBuildArtifactsResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved target path that was scanned.
    pub target_path: PathBuf,
    /// Artifact directories discovered under the target path.
    pub discovered_directories: Vec<PathBuf>,
    /// Artifact directories removed from disk.
    pub removed_directories: Vec<PathBuf>,
    /// Final command status.
    pub status: RuntimeCleanBuildArtifactsStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Remove common repository build artifact directories.
///
/// # Errors
///
/// Returns [`RuntimeCleanBuildArtifactsCommandError`] when root resolution,
/// target resolution, artifact discovery, or removal fails.
pub fn invoke_clean_build_artifacts(
    request: &RuntimeCleanBuildArtifactsRequest,
) -> Result<RuntimeCleanBuildArtifactsResult, RuntimeCleanBuildArtifactsCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeCleanBuildArtifactsCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(
            |source| RuntimeCleanBuildArtifactsCommandError::ResolveWorkspaceRoot { source },
        )?;
    let target_path = resolve_target_path(&repo_root, request.path.as_deref())?;
    let discovered_directories = discover_artifact_directories(&target_path)
        .map_err(|source| RuntimeCleanBuildArtifactsCommandError::DiscoverArtifacts { source })?;

    if discovered_directories.is_empty() {
        return Ok(RuntimeCleanBuildArtifactsResult {
            repo_root,
            target_path,
            discovered_directories,
            removed_directories: Vec::new(),
            status: RuntimeCleanBuildArtifactsStatus::Passed,
            exit_code: 0,
        });
    }

    if request.dry_run {
        return Ok(RuntimeCleanBuildArtifactsResult {
            repo_root,
            target_path,
            discovered_directories,
            removed_directories: Vec::new(),
            status: RuntimeCleanBuildArtifactsStatus::DryRun,
            exit_code: 0,
        });
    }

    if !request.force {
        return Ok(RuntimeCleanBuildArtifactsResult {
            repo_root,
            target_path,
            discovered_directories,
            removed_directories: Vec::new(),
            status: RuntimeCleanBuildArtifactsStatus::ConfirmationRequired,
            exit_code: 1,
        });
    }

    let removed_directories = remove_artifact_directories(&discovered_directories)
        .map_err(|source| RuntimeCleanBuildArtifactsCommandError::RemoveArtifacts { source })?;

    Ok(RuntimeCleanBuildArtifactsResult {
        repo_root,
        target_path,
        discovered_directories,
        removed_directories,
        status: RuntimeCleanBuildArtifactsStatus::Passed,
        exit_code: 0,
    })
}

fn resolve_target_path(
    repo_root: &Path,
    requested_path: Option<&Path>,
) -> Result<PathBuf, RuntimeCleanBuildArtifactsCommandError> {
    let resolved = match requested_path {
        Some(path) => resolve_full_path(repo_root, path),
        None => repo_root.to_path_buf(),
    };

    if !resolved.exists() {
        return Err(RuntimeCleanBuildArtifactsCommandError::ResolveTargetPath {
            target_path: resolved.display().to_string(),
        });
    }

    if resolved.is_file() {
        return Ok(resolved.parent().unwrap_or(repo_root).to_path_buf());
    }

    Ok(resolved)
}

fn discover_artifact_directories(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if root.is_file() {
        return Ok(Vec::new());
    }

    let mut matches = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(current) = stack.pop() {
        let entries = fs::read_dir(&current)
            .with_context(|| format!("failed to enumerate '{}'", current.display()))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if is_artifact_directory_name(&entry.file_name()) {
                    matches.push(path.clone());
                }
                stack.push(path);
            }
        }
    }

    matches.sort_by_key(|path| Reverse(path.components().count()));
    matches.dedup();
    Ok(matches)
}

fn remove_artifact_directories(paths: &[PathBuf]) -> anyhow::Result<Vec<PathBuf>> {
    let mut removed = Vec::new();
    for path in paths {
        if path.exists() {
            fs::remove_dir_all(path)
                .with_context(|| format!("failed to remove '{}'", path.display()))?;
            removed.push(path.clone());
        }
    }

    Ok(removed)
}

fn is_artifact_directory_name(name: &std::ffi::OsStr) -> bool {
    let name = name.to_string_lossy();
    ARTIFACT_DIRECTORY_NAMES
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(&name))
}
