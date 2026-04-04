//! Repository and workspace path resolution helpers.
//!
//! These utilities provide the Rust-owned equivalents of the shared
//! PowerShell path helpers used by runtime and validation scripts.

use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

const ROOT_ASCENT_LIMIT: usize = 6;

/// Resolve the repository root from an explicit path, script-root heuristic,
/// or current-directory ancestry.
///
/// The resolved directory must contain both `.github` and `.codex`.
///
/// # Errors
///
/// Returns an error when none of the candidates resolves to a repository root.
pub fn resolve_repository_root(
    requested_root: Option<&Path>,
    script_root: Option<&Path>,
    current_dir: &Path,
) -> Result<PathBuf> {
    let mut candidates = Vec::new();

    if let Some(requested_root) = requested_root {
        candidates.push(require_existing_directory(
            requested_root,
            "requested repository root",
        )?);
    }

    if let Some(script_root) = script_root {
        let repository_candidate = normalize_path(&script_root.join("..").join(".."));
        if repository_candidate.is_dir() {
            candidates.push(fs::canonicalize(&repository_candidate).with_context(|| {
                format!(
                    "failed to canonicalize repository candidate '{}'",
                    repository_candidate.display()
                )
            })?);
        }
    }

    candidates.push(require_existing_directory(
        current_dir,
        "current directory",
    )?);

    for candidate in unique_paths(candidates) {
        let mut current = Some(candidate.as_path());
        for _ in 0..ROOT_ASCENT_LIMIT {
            let Some(path) = current else {
                break;
            };

            if has_repository_layout(path) {
                return fs::canonicalize(path).with_context(|| {
                    format!(
                        "failed to canonicalize repository root '{}'",
                        path.display()
                    )
                });
            }

            current = path.parent();
        }
    }

    bail!("could not detect repository root containing both .github and .codex")
}

/// Resolve a workspace root without requiring the stricter repository layout.
///
/// # Errors
///
/// Returns an error when the requested or fallback directory does not exist.
pub fn resolve_workspace_root(
    requested_root: Option<&Path>,
    fallback_path: Option<&Path>,
) -> Result<PathBuf> {
    if let Some(requested_root) = requested_root {
        return require_existing_directory(requested_root, "requested workspace root");
    }

    let candidate = fallback_path.unwrap_or_else(|| Path::new("."));
    require_existing_directory(candidate, "workspace root")
}

/// Resolve a git checkout root when possible and otherwise fall back to the
/// provided directory.
///
/// # Errors
///
/// Returns an error when the explicit path is provided but does not exist.
pub fn resolve_git_root_or_current_path(
    requested_root: Option<&Path>,
    fallback_path: &Path,
) -> Result<PathBuf> {
    if let Some(requested_root) = requested_root {
        return require_existing_directory(requested_root, "requested repository root");
    }

    if let Some(git_root) = try_git_root(fallback_path)? {
        return Ok(git_root);
    }

    Ok(absolute_from_base(&std::env::current_dir()?, fallback_path))
}

/// Resolve an explicit path or require a git checkout root.
///
/// # Errors
///
/// Returns an error when no explicit path is provided and the start path is not
/// inside a git repository.
pub fn resolve_explicit_or_git_root(
    requested_root: Option<&Path>,
    start_path: &Path,
) -> Result<PathBuf> {
    if let Some(requested_root) = requested_root {
        return require_existing_directory(requested_root, "requested repository root");
    }

    try_git_root(start_path)?
        .ok_or_else(|| anyhow!("could not detect a git repository root; provide an explicit path"))
}

/// Resolve a repository/layout root by looking for a solution file or
/// `src`/`modules`/`.github` layout markers.
///
/// # Errors
///
/// Returns an error when no suitable root can be found within the search
/// window.
pub fn resolve_solution_or_layout_root(
    requested_root: Option<&Path>,
    start_path: &Path,
) -> Result<PathBuf> {
    if let Some(requested_root) = requested_root {
        return require_existing_directory(requested_root, "requested repository root");
    }

    let start_directory = require_existing_directory(start_path, "start path")?;
    let mut candidate = Some(start_directory.as_path());

    for _ in 0..ROOT_ASCENT_LIMIT {
        let Some(path) = candidate else {
            break;
        };

        if has_solution_or_layout_markers(path)? {
            return fs::canonicalize(path)
                .with_context(|| format!("failed to canonicalize '{}'", path.display()));
        }

        candidate = path.parent();
    }

    bail!(
        "could not auto-detect repository root from '{}'",
        start_path.display()
    )
}

/// Resolve a path relative to the repository root.
#[must_use]
pub fn resolve_repo_path(root: &Path, path: &Path) -> PathBuf {
    absolute_from_base(root, path)
}

/// Resolve a path relative to an arbitrary base directory.
#[must_use]
pub fn resolve_full_path(base_path: &Path, candidate: &Path) -> PathBuf {
    absolute_from_base(base_path, candidate)
}

/// Convert an absolute path to a repository-relative forward-slash path.
///
/// # Errors
///
/// Returns an error when the target path is outside the repository root.
pub fn convert_to_relative_repo_path(root: &Path, path: &Path) -> Result<String> {
    let normalized_root = require_existing_directory(root, "repository root")?;
    let normalized_path = if path.exists() {
        fs::canonicalize(path)
            .with_context(|| format!("failed to canonicalize '{}'", path.display()))?
    } else {
        absolute_from_base(&normalized_root, path)
    };
    let relative_path = normalized_path
        .strip_prefix(&normalized_root)
        .with_context(|| {
            format!(
                "path '{}' is not under repository root '{}'",
                normalized_path.display(),
                normalized_root.display()
            )
        })?;

    Ok(relative_path.to_string_lossy().replace('\\', "/"))
}

/// Return the parent directory of a path when one exists.
#[must_use]
pub fn parent_directory_path(path: &Path) -> Option<PathBuf> {
    path.parent().map(Path::to_path_buf)
}

fn require_existing_directory(path: &Path, label: &str) -> Result<PathBuf> {
    if !path.is_dir() {
        bail!("{label} not found: {}", path.display());
    }

    fs::canonicalize(path)
        .with_context(|| format!("failed to canonicalize {label} '{}'", path.display()))
}

fn try_git_root(start_path: &Path) -> Result<Option<PathBuf>> {
    let start_path = absolute_from_base(&std::env::current_dir()?, start_path);
    let output = Command::new("git")
        .arg("-C")
        .arg(&start_path)
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output();

    let Ok(output) = output else {
        return Ok(None);
    };

    if !output.status.success() {
        return Ok(None);
    }

    let git_root = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if git_root.is_empty() {
        return Ok(None);
    }

    let git_root = PathBuf::from(git_root);
    Ok(Some(fs::canonicalize(&git_root).with_context(|| {
        format!("failed to canonicalize git root '{}'", git_root.display())
    })?))
}

fn has_repository_layout(path: &Path) -> bool {
    path.join(".github").is_dir() && path.join(".codex").is_dir()
}

fn has_solution_or_layout_markers(path: &Path) -> Result<bool> {
    let has_solution = fs::read_dir(path)
        .with_context(|| format!("failed to enumerate '{}'", path.display()))?
        .filter_map(std::result::Result::ok)
        .any(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "sln")
        });

    let has_src = path.join("src").is_dir();
    let has_modules = path.join("modules").is_dir();
    let has_github = path.join(".github").is_dir();

    Ok(has_solution || (has_src && (has_modules || has_github)))
}

fn unique_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut unique = Vec::new();
    for path in paths {
        if !unique.contains(&path) {
            unique.push(path);
        }
    }

    unique
}

fn absolute_from_base(base_path: &Path, candidate: &Path) -> PathBuf {
    if candidate.is_absolute() {
        return normalize_path(candidate);
    }

    normalize_path(&base_path.join(candidate))
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();

    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = normalized.pop();
            }
            Component::Normal(segment) => normalized.push(segment),
        }
    }

    normalized
}