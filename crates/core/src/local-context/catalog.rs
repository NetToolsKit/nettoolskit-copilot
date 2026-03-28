//! Catalog and file-discovery support for the local context index.

use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::path_utils::repository::resolve_full_path;

/// Chunking settings for the local context index.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LocalContextIndexChunking {
    /// Maximum characters per chunk before a flush occurs.
    pub max_chars: usize,
    /// Maximum lines per chunk before a flush occurs.
    pub max_lines: usize,
}

/// Query defaults for the local context index.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LocalContextIndexQueryDefaults {
    /// Default number of hits returned by a query.
    pub top: usize,
}

/// Versioned catalog describing what should be indexed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LocalContextIndexCatalog {
    /// Schema version.
    pub version: u32,
    /// Relative output directory used by the persisted index.
    pub index_root: String,
    /// Maximum file size, in kilobytes, admitted into the index.
    pub max_file_size_kb: u64,
    /// Chunking limits.
    pub chunking: LocalContextIndexChunking,
    /// Query defaults.
    pub query_defaults: LocalContextIndexQueryDefaults,
    /// Include globs evaluated against repository-relative forward-slash paths.
    pub include_globs: Vec<String>,
    /// Exclude globs evaluated against repository-relative forward-slash paths.
    pub exclude_globs: Vec<String>,
}

/// Catalog payload paired with the resolved file-system path it came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalContextIndexCatalogInfo {
    /// Resolved catalog path.
    pub path: PathBuf,
    /// Parsed catalog payload.
    pub catalog: LocalContextIndexCatalog,
}

/// Resolve the local context index catalog path.
#[must_use]
pub fn resolve_local_context_index_catalog_path(
    repo_root: &Path,
    catalog_path: Option<&Path>,
) -> PathBuf {
    let default_path = repo_root.join(".github/governance/local-context-index.catalog.json");

    match catalog_path {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => resolve_full_path(repo_root, path),
        None => default_path,
    }
}

/// Read the versioned local context index catalog.
///
/// # Errors
///
/// Returns an error when the catalog cannot be read or parsed.
pub fn read_local_context_index_catalog(
    repo_root: &Path,
    catalog_path: Option<&Path>,
) -> Result<LocalContextIndexCatalogInfo> {
    let catalog_path = resolve_local_context_index_catalog_path(repo_root, catalog_path);
    let payload = fs::read_to_string(&catalog_path).with_context(|| {
        format!(
            "failed to read local context catalog '{}'",
            catalog_path.display()
        )
    })?;
    let catalog = serde_json::from_str::<LocalContextIndexCatalog>(&payload)
        .with_context(|| format!("invalid local context catalog '{}'", catalog_path.display()))?;

    Ok(LocalContextIndexCatalogInfo {
        path: catalog_path,
        catalog,
    })
}

/// Resolve the output root used by the persisted local context index.
#[must_use]
pub fn resolve_local_context_index_root(
    repo_root: &Path,
    catalog: &LocalContextIndexCatalog,
    output_root: Option<&Path>,
) -> PathBuf {
    match output_root {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => resolve_full_path(repo_root, path),
        None => resolve_full_path(repo_root, Path::new(&catalog.index_root)),
    }
}

/// Test whether a repository-relative path should be indexed.
///
/// # Errors
///
/// Returns an error when any configured glob is invalid.
pub fn local_context_index_path_included(
    relative_path: &str,
    catalog: &LocalContextIndexCatalog,
) -> Result<bool> {
    let normalized_relative_path = relative_path
        .replace('\\', "/")
        .trim_start_matches("./")
        .to_string();
    let include_set = compile_globset(&catalog.include_globs)?;
    let exclude_set = compile_globset(&catalog.exclude_globs)?;

    Ok(include_set.is_match(&normalized_relative_path)
        && !exclude_set.is_match(&normalized_relative_path))
}

/// Enumerate repository files admitted into the local context index.
///
/// # Errors
///
/// Returns an error when the repository tree cannot be traversed or the glob
/// configuration is invalid.
pub fn local_context_index_file_candidates(
    repo_root: &Path,
    catalog: &LocalContextIndexCatalog,
) -> Result<Vec<PathBuf>> {
    let include_set = compile_globset(&catalog.include_globs)?;
    let exclude_set = compile_globset(&catalog.exclude_globs)?;
    let max_file_size_bytes = catalog.max_file_size_kb.saturating_mul(1024);
    let mut candidates = Vec::new();

    for entry in WalkDir::new(repo_root).follow_links(false) {
        let entry = entry
            .with_context(|| format!("failed to traverse repository '{}'", repo_root.display()))?;

        if !entry.file_type().is_file() {
            continue;
        }

        let relative_path = entry
            .path()
            .strip_prefix(repo_root)
            .with_context(|| {
                format!(
                    "failed to strip repository prefix '{}' from '{}'",
                    repo_root.display(),
                    entry.path().display()
                )
            })?
            .to_string_lossy()
            .replace('\\', "/");

        if !include_set.is_match(&relative_path) || exclude_set.is_match(&relative_path) {
            continue;
        }

        let metadata = entry
            .metadata()
            .with_context(|| format!("failed to inspect '{}'", entry.path().display()))?;
        if metadata.len() > max_file_size_bytes {
            continue;
        }

        candidates.push(entry.into_path());
    }

    candidates.sort();
    Ok(candidates)
}

fn compile_globset(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(
            Glob::new(pattern)
                .with_context(|| format!("invalid local context glob pattern '{pattern}'"))?,
        );
    }

    builder
        .build()
        .context("failed to compile local context glob patterns")
}