//! MCP runtime artifact rendering for tracked VS Code and Codex projections.

use anyhow::Context;
use nettoolskit_core::path_utils::repository::{resolve_full_path, resolve_repository_root};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{
    RuntimeRenderMcpRuntimeArtifactsCommandError, RuntimeRenderVscodeMcpTemplateCommandError,
};

use super::mcp_catalog::{
    convert_catalog_to_codex_manifest, convert_catalog_to_vscode_document, read_runtime_catalog,
};

/// Request payload for `render-vscode-mcp-template`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeRenderVscodeMcpTemplateRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit MCP catalog path.
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit output path.
    pub output_path: Option<PathBuf>,
}

/// Result payload for `render-vscode-mcp-template`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRenderVscodeMcpTemplateResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved catalog path.
    pub catalog_path: PathBuf,
    /// Resolved output path.
    pub output_path: PathBuf,
    /// Number of inputs rendered into the document.
    pub input_count: usize,
    /// Number of servers rendered into the document.
    pub server_count: usize,
}

/// Request payload for `render-mcp-runtime-artifacts`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeRenderMcpRuntimeArtifactsRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit MCP catalog path.
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit VS Code output path.
    pub vscode_output_path: Option<PathBuf>,
    /// Optional explicit Codex manifest output path.
    pub codex_output_path: Option<PathBuf>,
}

/// Result payload for `render-mcp-runtime-artifacts`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeRenderMcpRuntimeArtifactsResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved catalog path.
    pub catalog_path: PathBuf,
    /// Resolved VS Code output path.
    pub vscode_output_path: PathBuf,
    /// Resolved Codex manifest output path.
    pub codex_output_path: PathBuf,
    /// Number of VS Code inputs rendered.
    pub input_count: usize,
    /// Number of VS Code servers rendered.
    pub vscode_server_count: usize,
    /// Number of Codex manifest servers rendered.
    pub codex_server_count: usize,
}

/// Render the tracked VS Code MCP template from the canonical runtime catalog.
///
/// # Errors
///
/// Returns [`RuntimeRenderVscodeMcpTemplateCommandError`] when repository
/// resolution, catalog loading, or output writing fails.
pub fn invoke_render_vscode_mcp_template(
    request: &RuntimeRenderVscodeMcpTemplateRequest,
) -> Result<RuntimeRenderVscodeMcpTemplateResult, RuntimeRenderVscodeMcpTemplateCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeRenderVscodeMcpTemplateCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| {
            RuntimeRenderVscodeMcpTemplateCommandError::ResolveWorkspaceRoot { source }
        })?;

    let (catalog_path, catalog) = read_runtime_catalog(&repo_root, request.catalog_path.as_deref())
        .map_err(|source| RuntimeRenderVscodeMcpTemplateCommandError::ReadCatalog { source })?;
    let output_path = resolve_output_path(
        &repo_root,
        request.output_path.as_deref(),
        Path::new(".vscode/mcp.tamplate.jsonc"),
    );
    let document = convert_catalog_to_vscode_document(&catalog);
    let serialized = serde_json::to_string_pretty(&document).map_err(|source| {
        RuntimeRenderVscodeMcpTemplateCommandError::RenderDocument {
            source: source.into(),
        }
    })?;

    write_document(&output_path, &serialized).map_err(|source| {
        RuntimeRenderVscodeMcpTemplateCommandError::WriteOutput { source }
    })?;

    Ok(RuntimeRenderVscodeMcpTemplateResult {
        repo_root,
        catalog_path,
        output_path,
        input_count: document.inputs.len(),
        server_count: document.servers.len(),
    })
}

/// Render both tracked MCP runtime artifacts from the canonical runtime catalog.
///
/// # Errors
///
/// Returns [`RuntimeRenderMcpRuntimeArtifactsCommandError`] when repository
/// resolution, catalog loading, rendering, or output writing fails.
pub fn invoke_render_mcp_runtime_artifacts(
    request: &RuntimeRenderMcpRuntimeArtifactsRequest,
) -> Result<
    RuntimeRenderMcpRuntimeArtifactsResult,
    RuntimeRenderMcpRuntimeArtifactsCommandError,
> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeRenderMcpRuntimeArtifactsCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| {
            RuntimeRenderMcpRuntimeArtifactsCommandError::ResolveWorkspaceRoot { source }
        })?;

    let (catalog_path, catalog) = read_runtime_catalog(&repo_root, request.catalog_path.as_deref())
        .map_err(|source| RuntimeRenderMcpRuntimeArtifactsCommandError::ReadCatalog { source })?;
    let vscode_output_path = resolve_output_path(
        &repo_root,
        request.vscode_output_path.as_deref(),
        Path::new(".vscode/mcp.tamplate.jsonc"),
    );
    let codex_output_path = resolve_output_path(
        &repo_root,
        request.codex_output_path.as_deref(),
        Path::new(".codex/mcp/servers.manifest.json"),
    );

    let vscode_document = convert_catalog_to_vscode_document(&catalog);
    let codex_manifest = convert_catalog_to_codex_manifest(&catalog).map_err(|source| {
        RuntimeRenderMcpRuntimeArtifactsCommandError::RenderDocument { source }
    })?;

    let vscode_serialized = serde_json::to_string_pretty(&vscode_document).map_err(|source| {
        RuntimeRenderMcpRuntimeArtifactsCommandError::RenderDocument {
            source: source.into(),
        }
    })?;
    let codex_serialized = serde_json::to_string_pretty(&codex_manifest).map_err(|source| {
        RuntimeRenderMcpRuntimeArtifactsCommandError::RenderDocument {
            source: source.into(),
        }
    })?;

    write_document(&vscode_output_path, &vscode_serialized).map_err(|source| {
        RuntimeRenderMcpRuntimeArtifactsCommandError::WriteOutput { source }
    })?;
    write_document(&codex_output_path, &codex_serialized).map_err(|source| {
        RuntimeRenderMcpRuntimeArtifactsCommandError::WriteOutput { source }
    })?;

    Ok(RuntimeRenderMcpRuntimeArtifactsResult {
        repo_root,
        catalog_path,
        vscode_output_path,
        codex_output_path,
        input_count: vscode_document.inputs.len(),
        vscode_server_count: vscode_document.servers.len(),
        codex_server_count: codex_manifest.servers.len(),
    })
}

fn resolve_output_path(repo_root: &Path, requested_path: Option<&Path>, default_path: &Path) -> PathBuf {
    match requested_path {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => resolve_full_path(repo_root, path),
        None => repo_root.join(default_path),
    }
}

fn write_document(path: &Path, document: &str) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create '{}'", parent.display()))?;
    }

    fs::write(path, document).with_context(|| format!("failed to write '{}'", path.display()))
}
