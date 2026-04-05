//! Shared MCP catalog parsing and projection helpers for runtime sync commands.

use anyhow::{anyhow, Context, Result};
use nettoolskit_core::path_utils::repository::resolve_full_path;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const CANONICAL_CATALOG_RELATIVE_PATH: &str =
    "definitions/providers/github/governance/mcp-runtime.catalog.json";
const LEGACY_CATALOG_RELATIVE_PATH: &str = ".github/governance/mcp-runtime.catalog.json";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct McpRuntimeCatalog {
    #[serde(default)]
    pub inputs: Vec<McpRuntimeInput>,
    #[serde(default)]
    pub servers: Vec<McpRuntimeCatalogServer>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct McpRuntimeInput {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub description: String,
    pub password: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct McpRuntimeCatalogServer {
    pub id: String,
    #[serde(default)]
    pub codex_name: Option<String>,
    #[serde(default)]
    pub targets: McpRuntimeTargets,
    pub definition: McpServerDefinition,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub(crate) struct McpRuntimeTargets {
    #[serde(default)]
    pub vscode: Option<McpRuntimeTargetSelection>,
    #[serde(default)]
    pub codex: Option<McpRuntimeTargetSelection>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct McpRuntimeTargetSelection {
    #[serde(default)]
    pub include: bool,
    #[serde(default)]
    pub enabled_by_default: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct McpServerDefinition {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub env: BTreeMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gallery: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub(crate) struct VscodeMcpDocument {
    pub inputs: Vec<McpRuntimeInput>,
    pub servers: BTreeMap<String, McpServerDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct CodexManifestServer {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub env: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct CodexManifestDocument {
    pub version: u32,
    pub servers: Vec<CodexManifestServer>,
}

pub(crate) fn resolve_catalog_path(repo_root: &Path, catalog_path: Option<&Path>) -> PathBuf {
    match catalog_path {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => resolve_full_path(repo_root, path),
        None => {
            let canonical_path = repo_root.join(CANONICAL_CATALOG_RELATIVE_PATH);
            if canonical_path.is_file() {
                canonical_path
            } else {
                repo_root.join(LEGACY_CATALOG_RELATIVE_PATH)
            }
        }
    }
}

pub(crate) fn read_runtime_catalog(
    repo_root: &Path,
    catalog_path: Option<&Path>,
) -> Result<(PathBuf, McpRuntimeCatalog)> {
    let resolved_catalog_path = resolve_catalog_path(repo_root, catalog_path);
    if !resolved_catalog_path.is_file() {
        return Err(anyhow!(
            "MCP runtime catalog not found: {}",
            resolved_catalog_path.display()
        ));
    }

    let catalog_document = fs::read_to_string(&resolved_catalog_path)
        .with_context(|| format!("failed to read '{}'", resolved_catalog_path.display()))?;
    let catalog: McpRuntimeCatalog =
        serde_json::from_str(&catalog_document).with_context(|| {
            format!(
                "invalid MCP runtime catalog '{}'",
                resolved_catalog_path.display()
            )
        })?;

    if catalog.servers.is_empty() {
        return Err(anyhow!(
            "MCP runtime catalog has no servers: {}",
            resolved_catalog_path.display()
        ));
    }

    Ok((resolved_catalog_path, catalog))
}

pub(crate) fn resolve_manifest_path(repo_root: &Path, manifest_path: &Path) -> PathBuf {
    if manifest_path.is_absolute() {
        manifest_path.to_path_buf()
    } else {
        resolve_full_path(repo_root, manifest_path)
    }
}

pub(crate) fn read_codex_manifest(
    repo_root: &Path,
    manifest_path: &Path,
) -> Result<(PathBuf, CodexManifestDocument)> {
    let resolved_manifest_path = resolve_manifest_path(repo_root, manifest_path);
    if !resolved_manifest_path.is_file() {
        return Err(anyhow!(
            "manifest not found: {}",
            resolved_manifest_path.display()
        ));
    }

    let manifest_document = fs::read_to_string(&resolved_manifest_path)
        .with_context(|| format!("failed to read '{}'", resolved_manifest_path.display()))?;
    let manifest: CodexManifestDocument =
        serde_json::from_str(&manifest_document).with_context(|| {
            format!(
                "invalid Codex MCP manifest '{}'",
                resolved_manifest_path.display()
            )
        })?;
    if manifest.servers.is_empty() {
        return Err(anyhow!(
            "manifest has no servers: {}",
            resolved_manifest_path.display()
        ));
    }

    Ok((resolved_manifest_path, manifest))
}

pub(crate) fn convert_catalog_to_vscode_document(catalog: &McpRuntimeCatalog) -> VscodeMcpDocument {
    let mut servers = BTreeMap::new();

    for server in &catalog.servers {
        let Some(vscode_target) = &server.targets.vscode else {
            continue;
        };
        if !vscode_target.include {
            continue;
        }

        let mut definition = server.definition.clone();
        definition.disabled = (!vscode_target.enabled_by_default).then_some(true);
        servers.insert(server.id.clone(), definition);
    }

    VscodeMcpDocument {
        inputs: catalog.inputs.clone(),
        servers,
    }
}

pub(crate) fn convert_catalog_to_codex_manifest(
    catalog: &McpRuntimeCatalog,
) -> Result<CodexManifestDocument> {
    let mut servers = Vec::new();

    for server in &catalog.servers {
        let Some(codex_target) = &server.targets.codex else {
            continue;
        };
        if !codex_target.include {
            continue;
        }

        let codex_name = server.codex_name.clone().ok_or_else(|| {
            anyhow!(
                "MCP runtime catalog server '{}' is missing codexName.",
                server.id
            )
        })?;

        servers.push(CodexManifestServer {
            name: codex_name,
            kind: server.definition.kind.clone(),
            command: server.definition.command.clone(),
            args: server.definition.args.clone(),
            url: server.definition.url.clone(),
            headers: server.definition.headers.clone(),
            env: server.definition.env.clone(),
        });
    }

    if servers.is_empty() {
        return Err(anyhow!("MCP runtime catalog has no Codex servers"));
    }

    Ok(CodexManifestDocument {
        version: 1,
        servers,
    })
}