//! Canonical Codex MCP config application for runtime sync and CLI surfaces.

use anyhow::{anyhow, Context, Result};
use nettoolskit_core::{
    path_utils::repository::{resolve_full_path, resolve_repository_root},
    runtime_locations::resolve_codex_runtime_path,
};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::RuntimeSyncCodexMcpConfigCommandError;

use super::mcp_catalog::{
    convert_catalog_to_codex_manifest, read_codex_manifest, read_runtime_catalog,
    CodexManifestServer,
};

/// Request payload for `sync-codex-mcp-config`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeSyncCodexMcpConfigRequest {
    /// Optional explicit repository root.
    pub repo_root: Option<PathBuf>,
    /// Optional explicit MCP runtime catalog path.
    pub catalog_path: Option<PathBuf>,
    /// Optional explicit generated Codex manifest path.
    pub manifest_path: Option<PathBuf>,
    /// Optional explicit target Codex config path.
    pub target_config_path: Option<PathBuf>,
    /// Create a timestamped backup before writing.
    pub create_backup: bool,
    /// Print the rendered document without writing it.
    pub dry_run: bool,
}

/// Result payload for `sync-codex-mcp-config`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSyncCodexMcpConfigResult {
    /// Resolved repository root.
    pub repo_root: PathBuf,
    /// Resolved target Codex config path.
    pub target_config_path: PathBuf,
    /// Number of MCP servers applied.
    pub servers_applied: usize,
    /// Optional backup path created before writing.
    pub backup_path: Option<PathBuf>,
    /// Rendered output document.
    pub rendered_document: String,
    /// Whether the command ran in dry-run mode.
    pub dry_run: bool,
}

/// Apply the canonical MCP runtime catalog into a target Codex `config.toml`.
///
/// # Errors
///
/// Returns [`RuntimeSyncCodexMcpConfigCommandError`] when repository
/// resolution, catalog or manifest loading, target config loading, TOML
/// rendering, backup creation, or writing fails.
pub fn invoke_sync_codex_mcp_config(
    request: &RuntimeSyncCodexMcpConfigRequest,
) -> Result<RuntimeSyncCodexMcpConfigResult, RuntimeSyncCodexMcpConfigCommandError> {
    let current_dir = env::current_dir().map_err(|source| {
        RuntimeSyncCodexMcpConfigCommandError::ResolveWorkspaceRoot {
            source: source.into(),
        }
    })?;
    let repo_root = resolve_repository_root(request.repo_root.as_deref(), None, &current_dir)
        .map_err(|source| {
            RuntimeSyncCodexMcpConfigCommandError::ResolveWorkspaceRoot { source }
        })?;
    let target_config_path =
        resolve_target_config_path(&repo_root, request.target_config_path.as_deref());

    if !target_config_path.is_file() {
        return Err(RuntimeSyncCodexMcpConfigCommandError::TargetConfigNotFound {
            target_config_path: target_config_path.display().to_string(),
        });
    }

    let servers = resolve_manifest_servers(
        &repo_root,
        request.catalog_path.as_deref(),
        request.manifest_path.as_deref(),
    )
    .map_err(|source| RuntimeSyncCodexMcpConfigCommandError::ResolveServers { source })?;
    let original_document = fs::read_to_string(&target_config_path).map_err(|source| {
        RuntimeSyncCodexMcpConfigCommandError::ReadTargetConfig {
            source: source.into(),
        }
    })?;
    let base_lines = remove_mcp_sections(&original_document);
    let rendered_mcp_lines = render_mcp_toml(&servers).map_err(|source| {
        RuntimeSyncCodexMcpConfigCommandError::RenderConfig { source }
    })?;

    let mut output_lines = base_lines;
    if !output_lines.is_empty() {
        output_lines.push(String::new());
    }
    output_lines.extend(rendered_mcp_lines);
    let rendered_document = output_lines.join("\n");

    let backup_path = if request.create_backup {
        Some(
            create_backup(&target_config_path).map_err(|source| {
                RuntimeSyncCodexMcpConfigCommandError::CreateBackup { source }
            })?,
        )
    } else {
        None
    };

    if !request.dry_run {
        fs::write(&target_config_path, &rendered_document).map_err(|source| {
            RuntimeSyncCodexMcpConfigCommandError::WriteOutput {
                source: source.into(),
            }
        })?;
    }

    Ok(RuntimeSyncCodexMcpConfigResult {
        repo_root,
        target_config_path,
        servers_applied: servers.len(),
        backup_path,
        rendered_document,
        dry_run: request.dry_run,
    })
}

/// Apply the canonical MCP runtime catalog into the default Codex config path.
///
/// # Errors
///
/// Returns an error when the runtime catalog or target config cannot be loaded
/// or rewritten safely.
pub(crate) fn apply_mcp_runtime_catalog_to_codex_config(
    repo_root: &Path,
    codex_path: &Path,
    backup_config: bool,
) -> Result<()> {
    let target_config = codex_path.join("config.toml");
    invoke_sync_codex_mcp_config(&RuntimeSyncCodexMcpConfigRequest {
        repo_root: Some(repo_root.to_path_buf()),
        target_config_path: Some(target_config),
        create_backup: backup_config,
        dry_run: false,
        ..RuntimeSyncCodexMcpConfigRequest::default()
    })
    .map(|_| ())
    .map_err(|error| anyhow!(error.to_string()))
}

fn resolve_target_config_path(repo_root: &Path, target_config_path: Option<&Path>) -> PathBuf {
    match target_config_path {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => resolve_full_path(repo_root, path),
        None => resolve_codex_runtime_path().join("config.toml"),
    }
}

fn resolve_manifest_servers(
    repo_root: &Path,
    catalog_path: Option<&Path>,
    manifest_path: Option<&Path>,
) -> Result<Vec<CodexManifestServer>> {
    if let Some(manifest_path) = manifest_path {
        let (_, manifest) = read_codex_manifest(repo_root, manifest_path)?;
        return Ok(manifest.servers);
    }

    let (_, catalog) = read_runtime_catalog(repo_root, catalog_path)?;
    Ok(convert_catalog_to_codex_manifest(&catalog)?.servers)
}

fn create_backup(target_config_path: &Path) -> Result<PathBuf> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let backup_path = target_config_path.with_file_name(format!(
        "{}.bak.{}",
        target_config_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("config.toml"),
        timestamp
    ));
    fs::copy(target_config_path, &backup_path).with_context(|| {
        format!(
            "failed to create MCP config backup '{}' from '{}'",
            backup_path.display(),
            target_config_path.display()
        )
    })?;
    Ok(backup_path)
}

fn remove_mcp_sections(document: &str) -> Vec<String> {
    let lines: Vec<&str> = document.lines().collect();
    let mut result = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim();
        if trimmed.starts_with("[mcp_servers.") || trimmed == "[mcp_servers]" {
            index += 1;
            while index < lines.len() {
                let probe = lines[index].trim();
                if probe.starts_with('[')
                    && !probe.starts_with("[mcp_servers.")
                    && probe != "[mcp_servers]"
                {
                    break;
                }
                index += 1;
            }
            continue;
        }

        result.push(line.to_string());
        index += 1;
    }

    while result.last().is_some_and(|line| line.trim().is_empty()) {
        result.pop();
    }

    result
}

fn render_mcp_toml(servers: &[CodexManifestServer]) -> Result<Vec<String>> {
    let mut lines = Vec::new();

    for server in servers {
        if server.name.trim().is_empty() {
            return Err(anyhow!("each MCP server must include a non-empty name"));
        }

        lines.push(format!("[mcp_servers.{}]", server.name));
        if let Some(command) = &server.command {
            if !command.trim().is_empty() {
                lines.push(format!("command = \"{}\"", escape_toml_string(command)));
            }
        }
        if !server.args.is_empty() {
            let args = server
                .args
                .iter()
                .map(|value| format!("\"{}\"", escape_toml_string(value)))
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!("args = [{args}]"));
        }
        if let Some(url) = &server.url {
            if !url.trim().is_empty() {
                lines.push(format!("url = \"{}\"", escape_toml_string(url)));
            }
        }
        if !server.headers.is_empty() {
            lines.push(format!("[mcp_servers.{}.headers]", server.name));
            for (key, value) in &server.headers {
                lines.push(format!(
                    "\"{}\" = \"{}\"",
                    escape_toml_string(key),
                    escape_toml_string(value)
                ));
            }
        }
        if !server.env.is_empty() {
            lines.push(format!("[mcp_servers.{}.env]", server.name));
            for (key, value) in &server.env {
                lines.push(format!(
                    "\"{}\" = \"{}\"",
                    escape_toml_string(key),
                    escape_toml_string(value)
                ));
            }
        }
        lines.push(String::new());
    }

    while lines.last().is_some_and(|line| line.trim().is_empty()) {
        lines.pop();
    }

    Ok(lines)
}

fn escape_toml_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
