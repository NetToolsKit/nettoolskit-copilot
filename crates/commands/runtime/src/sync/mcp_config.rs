//! Canonical Codex MCP config application for bootstrap-owned runtime sync.

use anyhow::{anyhow, Context, Result};
use serde_json::{Map, Value};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
struct CodexManifestServer {
    name: String,
    command: Option<String>,
    args: Vec<String>,
    url: Option<String>,
    headers: Vec<(String, String)>,
    env: Vec<(String, String)>,
}

/// Apply the canonical MCP runtime catalog into a target Codex `config.toml`.
///
/// # Errors
///
/// Returns an error when the canonical catalog or target config is missing,
/// when the catalog is invalid for Codex projection, or when the target file
/// cannot be rewritten safely.
pub(crate) fn apply_mcp_runtime_catalog_to_codex_config(
    repo_root: &Path,
    codex_path: &Path,
    backup_config: bool,
) -> Result<()> {
    let catalog_path = repo_root.join(".github/governance/mcp-runtime.catalog.json");
    let target_config = codex_path.join("config.toml");

    if !catalog_path.is_file() {
        return Err(anyhow!(
            "MCP runtime catalog missing: {}",
            catalog_path.display()
        ));
    }
    if !target_config.is_file() {
        return Err(anyhow!(
            "target Codex config missing: {}",
            target_config.display()
        ));
    }

    let catalog_document = fs::read_to_string(&catalog_path)
        .with_context(|| format!("failed to read '{}'", catalog_path.display()))?;
    let catalog: Value = serde_json::from_str(&catalog_document)
        .with_context(|| format!("invalid MCP runtime catalog '{}'", catalog_path.display()))?;
    let servers = convert_catalog_to_codex_servers(&catalog)?;
    let original_document = fs::read_to_string(&target_config)
        .with_context(|| format!("failed to read '{}'", target_config.display()))?;
    let base_lines = remove_mcp_sections(&original_document);
    let rendered_mcp_lines = render_mcp_toml(&servers)?;

    let mut output_lines = base_lines;
    if !output_lines.is_empty() {
        output_lines.push(String::new());
    }
    output_lines.extend(rendered_mcp_lines);
    let output_document = output_lines.join("\n");

    if backup_config {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        let backup_path = target_config.with_file_name(format!(
            "{}.bak.{}",
            target_config
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("config.toml"),
            timestamp
        ));
        fs::copy(&target_config, &backup_path).with_context(|| {
            format!(
                "failed to create MCP config backup '{}' from '{}'",
                backup_path.display(),
                target_config.display()
            )
        })?;
    }

    fs::write(&target_config, output_document)
        .with_context(|| format!("failed to write '{}'", target_config.display()))?;
    Ok(())
}

fn convert_catalog_to_codex_servers(catalog: &Value) -> Result<Vec<CodexManifestServer>> {
    let servers = catalog
        .get("servers")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("MCP runtime catalog has no servers"))?;
    let mut result = Vec::new();

    for server in servers {
        let include_codex = server
            .get("targets")
            .and_then(|targets| targets.get("codex"))
            .and_then(|target| target.get("include"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        if !include_codex {
            continue;
        }

        let codex_name = server
            .get("codexName")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                anyhow!(
                    "MCP runtime catalog server '{}' is missing codexName.",
                    server
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")
                )
            })?;
        let definition = server
            .get("definition")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                anyhow!(
                    "MCP runtime catalog server '{}' is missing definition.",
                    server
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")
                )
            })?;

        result.push(CodexManifestServer {
            name: codex_name.to_string(),
            command: optional_string(definition, "command"),
            args: optional_string_array(definition, "args"),
            url: optional_string(definition, "url"),
            headers: optional_string_map(definition, "headers"),
            env: optional_string_map(definition, "env"),
        });
    }

    if result.is_empty() {
        return Err(anyhow!("MCP runtime catalog has no Codex servers"));
    }

    Ok(result)
}

fn optional_string(object: &Map<String, Value>, key: &str) -> Option<String> {
    object.get(key).and_then(Value::as_str).map(str::to_string)
}

fn optional_string_array(object: &Map<String, Value>, key: &str) -> Vec<String> {
    object
        .get(key)
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn optional_string_map(object: &Map<String, Value>, key: &str) -> Vec<(String, String)> {
    object
        .get(key)
        .and_then(Value::as_object)
        .map(|map| {
            map.iter()
                .filter_map(|(map_key, map_value)| {
                    map_value
                        .as_str()
                        .map(|value| (map_key.to_string(), value.to_string()))
                })
                .collect()
        })
        .unwrap_or_default()
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