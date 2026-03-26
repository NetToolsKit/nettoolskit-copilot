//! Runtime execution context helpers.
//!
//! These helpers port the shared PowerShell runtime execution contract used by
//! bootstrap, doctor, install, and healthcheck style commands.

use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::path_utils::repository::{resolve_full_path, resolve_repository_root};
use crate::runtime_install_profiles::{
    resolve_runtime_install_profile, ResolvedRuntimeInstallProfile,
};
use crate::runtime_locations::{
    built_in_runtime_location_catalog, effective_runtime_locations, resolve_agents_skills_path,
    resolve_claude_runtime_path, resolve_codex_runtime_path, resolve_copilot_skills_path,
    resolve_github_runtime_path, resolve_user_home_path, EffectiveRuntimeLocations,
    RuntimeLocationOverrides,
};

/// Resolved runtime target roots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeExecutionTargets {
    /// GitHub runtime root.
    pub github_runtime_root: PathBuf,
    /// Codex runtime root.
    pub codex_runtime_root: PathBuf,
    /// Picker-visible agent skills root.
    pub agents_skills_root: PathBuf,
    /// GitHub Copilot native skills root.
    pub copilot_skills_root: PathBuf,
    /// Claude runtime root.
    pub claude_runtime_root: PathBuf,
}

/// Canonical source roots derived from the repository root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeExecutionSources {
    /// Repository `.github` root.
    pub github_root: PathBuf,
    /// Repository `.codex` root.
    pub codex_root: PathBuf,
    /// Repository `scripts` root.
    pub scripts_root: PathBuf,
    /// Repository `.github/skills` root.
    pub github_skills_root: PathBuf,
    /// Repository `.codex/skills` root.
    pub codex_skills_root: PathBuf,
    /// Repository `.claude/skills` root.
    pub claude_skills_root: PathBuf,
    /// Repository `.codex/mcp` root.
    pub codex_mcp_root: PathBuf,
    /// Repository `.codex/scripts` root.
    pub codex_scripts_root: PathBuf,
    /// Repository `.codex/orchestration` root.
    pub codex_orchestration_root: PathBuf,
    /// Repository `scripts/common` root.
    pub common_scripts_root: PathBuf,
    /// Repository `scripts/security` root.
    pub security_scripts_root: PathBuf,
    /// Repository `scripts/maintenance` root.
    pub maintenance_scripts_root: PathBuf,
}

/// Shared runtime execution context for runtime commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeExecutionContext {
    /// Resolved repository root.
    pub resolved_repo_root: PathBuf,
    /// Resolved runtime install profile.
    pub runtime_profile: ResolvedRuntimeInstallProfile,
    /// Effective runtime locations derived from catalog plus overrides.
    pub effective_runtime_locations: EffectiveRuntimeLocations,
    /// Resolved target roots used by runtime commands.
    pub targets: RuntimeExecutionTargets,
    /// Canonical source roots derived from the repository.
    pub sources: RuntimeExecutionSources,
}

/// Standard target arguments built from a runtime execution context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTargetArguments {
    /// Optional repository root argument.
    pub repo_root: Option<PathBuf>,
    /// GitHub runtime root.
    pub target_github_path: PathBuf,
    /// Codex runtime root.
    pub target_codex_path: PathBuf,
    /// Picker-visible agent skills root.
    pub target_agents_skills_path: PathBuf,
    /// GitHub Copilot native skills root.
    pub target_copilot_skills_path: PathBuf,
    /// Optional runtime profile name.
    pub runtime_profile: Option<String>,
}

/// Resolve the shared runtime execution context.
///
/// # Errors
///
/// Returns an error when repository root or profile resolution fails.
#[allow(clippy::too_many_arguments)]
pub fn resolve_runtime_execution_context(
    requested_repo_root: Option<&Path>,
    profile_name: Option<&str>,
    fallback_profile_name: Option<&str>,
    requested_target_github_path: Option<&Path>,
    requested_target_codex_path: Option<&Path>,
    requested_target_agents_skills_path: Option<&Path>,
    requested_target_copilot_skills_path: Option<&Path>,
    requested_target_claude_path: Option<&Path>,
    current_dir: &Path,
) -> Result<RuntimeExecutionContext> {
    let resolved_repo_root = resolve_repository_root(requested_repo_root, None, current_dir)?;
    let runtime_profile = resolve_runtime_install_profile(
        &resolved_repo_root,
        profile_name,
        fallback_profile_name,
    )?;
    let effective_runtime_locations = effective_runtime_locations(
        &built_in_runtime_location_catalog(),
        &RuntimeLocationOverrides::default(),
        &resolve_user_home_path()?,
    );

    let github_runtime_root = requested_target_github_path
        .map(|path| resolve_full_path(&resolved_repo_root, path))
        .unwrap_or_else(resolve_github_runtime_path);
    let codex_runtime_root = requested_target_codex_path
        .map(|path| resolve_full_path(&resolved_repo_root, path))
        .unwrap_or_else(resolve_codex_runtime_path);
    let agents_skills_root = requested_target_agents_skills_path
        .map(|path| resolve_full_path(&resolved_repo_root, path))
        .unwrap_or_else(resolve_agents_skills_path);
    let copilot_skills_root = requested_target_copilot_skills_path
        .map(|path| resolve_full_path(&resolved_repo_root, path))
        .unwrap_or_else(resolve_copilot_skills_path);
    let claude_runtime_root = requested_target_claude_path
        .map(|path| resolve_full_path(&resolved_repo_root, path))
        .unwrap_or_else(resolve_claude_runtime_path);

    let github_root = resolved_repo_root.join(".github");
    let codex_root = resolved_repo_root.join(".codex");
    let scripts_root = resolved_repo_root.join("scripts");

    Ok(RuntimeExecutionContext {
        resolved_repo_root: resolved_repo_root.clone(),
        runtime_profile,
        effective_runtime_locations,
        targets: RuntimeExecutionTargets {
            github_runtime_root,
            codex_runtime_root,
            agents_skills_root,
            copilot_skills_root,
            claude_runtime_root,
        },
        sources: RuntimeExecutionSources {
            github_root: github_root.clone(),
            codex_root: codex_root.clone(),
            scripts_root: scripts_root.clone(),
            github_skills_root: github_root.join("skills"),
            codex_skills_root: codex_root.join("skills"),
            claude_skills_root: resolved_repo_root.join(".claude").join("skills"),
            codex_mcp_root: codex_root.join("mcp"),
            codex_scripts_root: codex_root.join("scripts"),
            codex_orchestration_root: codex_root.join("orchestration"),
            common_scripts_root: scripts_root.join("common"),
            security_scripts_root: scripts_root.join("security"),
            maintenance_scripts_root: scripts_root.join("maintenance"),
        },
    })
}

/// Build a standard runtime target argument set for downstream commands.
#[must_use]
pub fn runtime_target_arguments(
    context: &RuntimeExecutionContext,
    include_repo_root: bool,
    include_runtime_profile: bool,
) -> RuntimeTargetArguments {
    RuntimeTargetArguments {
        repo_root: include_repo_root.then(|| context.resolved_repo_root.clone()),
        target_github_path: context.targets.github_runtime_root.clone(),
        target_codex_path: context.targets.codex_runtime_root.clone(),
        target_agents_skills_path: context.targets.agents_skills_root.clone(),
        target_copilot_skills_path: context.targets.copilot_skills_root.clone(),
        runtime_profile: include_runtime_profile.then(|| context.runtime_profile.name.clone()),
    }
}

/// Build a standard runtime target argument set resolved against a repository
/// root.
#[must_use]
pub fn resolved_runtime_target_arguments(
    context: &RuntimeExecutionContext,
    resolved_repo_root: &Path,
    include_repo_root: bool,
    include_runtime_profile: bool,
) -> RuntimeTargetArguments {
    RuntimeTargetArguments {
        repo_root: include_repo_root.then(|| resolved_repo_root.to_path_buf()),
        target_github_path: resolve_full_path(resolved_repo_root, &context.targets.github_runtime_root),
        target_codex_path: resolve_full_path(resolved_repo_root, &context.targets.codex_runtime_root),
        target_agents_skills_path: resolve_full_path(
            resolved_repo_root,
            &context.targets.agents_skills_root,
        ),
        target_copilot_skills_path: resolve_full_path(
            resolved_repo_root,
            &context.targets.copilot_skills_root,
        ),
        runtime_profile: include_runtime_profile.then(|| context.runtime_profile.name.clone()),
    }
}