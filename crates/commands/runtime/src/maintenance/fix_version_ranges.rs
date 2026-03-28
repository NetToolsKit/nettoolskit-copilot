//! Normalize selected `.csproj` package versions into bounded ranges.

use anyhow::{anyhow, Context};
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use regex::{Captures, Regex};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::error::RuntimeFixVersionRangesCommandError;

/// Request payload for `fix-version-ranges`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeFixVersionRangesRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional single project file to normalize.
    pub project_file: Option<PathBuf>,
    /// Report changes without writing the `.csproj` files.
    pub dry_run: bool,
}

/// Runtime fix-version-ranges result status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeFixVersionRangesStatus {
    /// Command updated one or more projects or completed with no changes.
    Passed,
    /// Command only reported the projects that would change.
    DryRun,
}

/// One package adjustment applied to one project file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFixVersionRangesAdjustment {
    /// Package identifier that matched one normalization rule.
    pub package_id: String,
    /// Original version or range.
    pub previous_version: String,
    /// Normalized version range.
    pub updated_version: String,
}

/// One project result in `fix-version-ranges`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFixVersionRangesProjectResult {
    /// Project path that was processed.
    pub project_path: PathBuf,
    /// Package adjustments applied to this project.
    pub adjustments: Vec<RuntimeFixVersionRangesAdjustment>,
}

/// Result payload for `fix-version-ranges`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFixVersionRangesResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Project files discovered for processing.
    pub project_files: Vec<PathBuf>,
    /// Project files whose contents changed or would change.
    pub changed_projects: Vec<RuntimeFixVersionRangesProjectResult>,
    /// Final command status.
    pub status: RuntimeFixVersionRangesStatus,
    /// Process exit code equivalent.
    pub exit_code: i32,
}

/// Normalize selected package versions into capped ranges.
///
/// # Errors
///
/// Returns [`RuntimeFixVersionRangesCommandError`] when root resolution,
/// project discovery, or project normalization fails.
pub fn invoke_fix_version_ranges(
    request: &RuntimeFixVersionRangesRequest,
) -> Result<RuntimeFixVersionRangesResult, RuntimeFixVersionRangesCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeFixVersionRangesCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| RuntimeFixVersionRangesCommandError::ResolveWorkspaceRoot { source })?;
    let project_files = discover_project_files(&repo_root, request.project_file.as_deref())?;
    let mut changed_projects = Vec::new();

    for project_path in &project_files {
        let Some(project_result) = normalize_project(project_path, request.dry_run)
            .map_err(|source| RuntimeFixVersionRangesCommandError::NormalizeProjects { source })?
        else {
            continue;
        };
        changed_projects.push(project_result);
    }

    Ok(RuntimeFixVersionRangesResult {
        repo_root,
        project_files,
        changed_projects,
        status: if request.dry_run {
            RuntimeFixVersionRangesStatus::DryRun
        } else {
            RuntimeFixVersionRangesStatus::Passed
        },
        exit_code: 0,
    })
}

fn discover_project_files(
    repo_root: &Path,
    requested_project_file: Option<&Path>,
) -> Result<Vec<PathBuf>, RuntimeFixVersionRangesCommandError> {
    if let Some(project_file) = requested_project_file {
        let resolved = resolve_full_path(repo_root, project_file);
        if !resolved.is_file() || !is_csproj_file(&resolved) {
            return Err(RuntimeFixVersionRangesCommandError::ResolveProjectPath {
                project_path: resolved.display().to_string(),
            });
        }

        return Ok(vec![resolved]);
    }

    let mut project_files = WalkDir::new(repo_root)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .with_context(|| format!("failed to enumerate '{}'", repo_root.display()))
        .map_err(|source| RuntimeFixVersionRangesCommandError::DiscoverProjects { source })?
        .into_iter()
        .filter(|entry| entry.file_type().is_file() && is_csproj_file(entry.path()))
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();

    project_files.sort();
    Ok(project_files)
}

fn is_csproj_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("csproj"))
}

fn normalize_project(
    project_path: &Path,
    dry_run: bool,
) -> anyhow::Result<Option<RuntimeFixVersionRangesProjectResult>> {
    let original = fs::read_to_string(project_path)
        .with_context(|| format!("failed to read '{}'", project_path.display()))?;
    let limits = package_limits();
    let mut adjustments = Vec::new();
    let mut updated = original.clone();

    for (package_id, upper_limit) in &limits {
        updated = apply_explicit_version_rule(&updated, package_id, upper_limit, &mut adjustments)?;
        updated = apply_existing_range_rule(&updated, package_id, upper_limit, &mut adjustments)?;
    }

    if updated == original {
        return Ok(None);
    }

    if !dry_run {
        fs::write(project_path, updated)
            .with_context(|| format!("failed to write '{}'", project_path.display()))?;
    }

    Ok(Some(RuntimeFixVersionRangesProjectResult {
        project_path: project_path.to_path_buf(),
        adjustments,
    }))
}

fn package_limits() -> BTreeMap<&'static str, VersionTriple> {
    BTreeMap::from([
        ("AutoMapper", VersionTriple::new(14, 0, 0)),
        ("FluentAssertions", VersionTriple::new(8, 0, 0)),
        ("FluentValidation", VersionTriple::new(12, 0, 0)),
        ("MassTransit.Newtonsoft", VersionTriple::new(9, 0, 0)),
        ("MassTransit.RabbitMQ", VersionTriple::new(9, 0, 0)),
        ("MediatR", VersionTriple::new(13, 0, 0)),
    ])
}

fn apply_explicit_version_rule(
    text: &str,
    package_id: &str,
    upper_limit: &VersionTriple,
    adjustments: &mut Vec<RuntimeFixVersionRangesAdjustment>,
) -> anyhow::Result<String> {
    let pattern = Regex::new(&format!(
        r#"(?i)<PackageReference\s+Include\s*=\s*"{}"\s+Version\s*=\s*"(\d+\.\d+\.\d+)"\s*/>"#,
        regex::escape(package_id)
    ))
    .map_err(|source| anyhow!("failed to compile explicit-version pattern: {source}"))?;

    let updated = pattern.replace_all(text, |captures: &Captures<'_>| {
        let current_version = captures
            .get(1)
            .expect("capture 1 should exist")
            .as_str()
            .to_string();
        let Ok(parsed_version) = VersionTriple::parse(&current_version) else {
            return captures[0].to_string();
        };
        if parsed_version >= *upper_limit {
            return captures[0].to_string();
        }

        let updated_version = format!("[{current_version},{upper_limit})");
        adjustments.push(RuntimeFixVersionRangesAdjustment {
            package_id: package_id.to_string(),
            previous_version: current_version.clone(),
            updated_version: updated_version.clone(),
        });
        format!(r#"<PackageReference Include="{package_id}" Version="{updated_version}" />"#)
    });

    Ok(updated.into_owned())
}

fn apply_existing_range_rule(
    text: &str,
    package_id: &str,
    upper_limit: &VersionTriple,
    adjustments: &mut Vec<RuntimeFixVersionRangesAdjustment>,
) -> anyhow::Result<String> {
    let pattern = Regex::new(&format!(
        r#"(?i)<PackageReference\s+Include\s*=\s*"{}"\s+Version\s*=\s*"\[(\d+\.\d+\.\d+),\s*(\d+\.\d+\.\d+)\)"\s*/>"#,
        regex::escape(package_id)
    ))
    .map_err(|source| anyhow!("failed to compile range-version pattern: {source}"))?;

    let updated = pattern.replace_all(text, |captures: &Captures<'_>| {
        let lower_bound = captures
            .get(1)
            .expect("capture 1 should exist")
            .as_str()
            .to_string();
        let previous_upper_bound = captures
            .get(2)
            .expect("capture 2 should exist")
            .as_str()
            .to_string();
        let Ok(parsed_upper_bound) = VersionTriple::parse(&previous_upper_bound) else {
            return captures[0].to_string();
        };
        if parsed_upper_bound >= *upper_limit {
            return captures[0].to_string();
        }

        let previous_version = format!("[{lower_bound},{previous_upper_bound})");
        let updated_version = format!("[{lower_bound},{upper_limit})");
        adjustments.push(RuntimeFixVersionRangesAdjustment {
            package_id: package_id.to_string(),
            previous_version,
            updated_version: updated_version.clone(),
        });
        format!(r#"<PackageReference Include="{package_id}" Version="{updated_version}" />"#)
    });

    Ok(updated.into_owned())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct VersionTriple {
    major: u32,
    minor: u32,
    patch: u32,
}

impl VersionTriple {
    const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    fn parse(text: &str) -> anyhow::Result<Self> {
        let mut parts = text.split('.');
        let major = parts
            .next()
            .ok_or_else(|| anyhow!("missing major version"))?
            .parse::<u32>()
            .with_context(|| format!("invalid major version '{text}'"))?;
        let minor = parts
            .next()
            .ok_or_else(|| anyhow!("missing minor version"))?
            .parse::<u32>()
            .with_context(|| format!("invalid minor version '{text}'"))?;
        let patch = parts
            .next()
            .ok_or_else(|| anyhow!("missing patch version"))?
            .parse::<u32>()
            .with_context(|| format!("invalid patch version '{text}'"))?;
        if parts.next().is_some() {
            return Err(anyhow!("unexpected version format '{text}'"));
        }

        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

impl std::fmt::Display for VersionTriple {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_existing_range_rule, apply_explicit_version_rule, VersionTriple};

    #[test]
    fn test_apply_explicit_version_rule_caps_matching_package() {
        let mut adjustments = Vec::new();
        let updated = apply_explicit_version_rule(
            r#"<PackageReference Include="AutoMapper" Version="13.0.1" />"#,
            "AutoMapper",
            &VersionTriple::new(14, 0, 0),
            &mut adjustments,
        )
        .expect("normalization should succeed");

        assert_eq!(
            updated,
            r#"<PackageReference Include="AutoMapper" Version="[13.0.1,14.0.0)" />"#
        );
        assert_eq!(adjustments.len(), 1);
        assert_eq!(adjustments[0].package_id, "AutoMapper");
    }

    #[test]
    fn test_apply_existing_range_rule_updates_lower_upper_bound_only() {
        let mut adjustments = Vec::new();
        let updated = apply_existing_range_rule(
            r#"<PackageReference Include="MediatR" Version="[12.1.0,12.5.0)" />"#,
            "MediatR",
            &VersionTriple::new(13, 0, 0),
            &mut adjustments,
        )
        .expect("normalization should succeed");

        assert_eq!(
            updated,
            r#"<PackageReference Include="MediatR" Version="[12.1.0,13.0.0)" />"#
        );
        assert_eq!(adjustments.len(), 1);
        assert_eq!(adjustments[0].updated_version, "[12.1.0,13.0.0)");
    }
}
