//! Tests for native Codex MCP config sync commands.

use nettoolskit_runtime::{invoke_sync_codex_mcp_config, RuntimeSyncCodexMcpConfigRequest};
use serde_json::Value;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

fn write_governance_file(repo_root: &Path, file_name: &str, contents: &str) {
    write_file(
        &repo_root
            .join("definitions/providers/github/governance")
            .join(file_name),
        contents,
    );
    write_file(
        &repo_root.join(".github/governance").join(file_name),
        contents,
    );
}

fn initialize_repo_root(repo_root: &Path) {
    fs::create_dir_all(repo_root.join("definitions/providers/github/governance"))
        .expect("canonical governance directory should be created");
    fs::create_dir_all(repo_root.join(".github/governance"))
        .expect("legacy governance directory should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect(".codex should be created");
}

fn write_catalog(repo_root: &Path) {
    write_governance_file(
        repo_root,
        "mcp-runtime.catalog.json",
        r#"{
  "version": 1,
  "inputs": [],
  "servers": [
    {
      "id": "microsoftdocs/mcp",
      "codexName": "microsoftdocs",
      "targets": {
        "vscode": { "include": true, "enabledByDefault": true },
        "codex": { "include": true }
      },
      "definition": {
        "type": "http",
        "url": "https://learn.microsoft.com/api/mcp",
        "gallery": "https://example.invalid/gallery",
        "version": "1.0.0"
      }
    },
    {
      "id": "microsoft/playwright-mcp",
      "codexName": "playwright",
      "targets": {
        "codex": { "include": true }
      },
      "definition": {
        "type": "stdio",
        "command": "npx",
        "args": ["@playwright/mcp@latest"]
      }
    },
    {
      "id": "vscode-only/example",
      "targets": {
        "vscode": { "include": true, "enabledByDefault": false }
      },
      "definition": {
        "type": "http",
        "url": "https://example.invalid/vscode-only"
      }
    }
  ]
}"#,
    );
}

#[test]
fn test_invoke_sync_codex_mcp_config_supports_manifest_input_and_dry_run() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());

    let manifest_path = repo.path().join("servers.manifest.json");
    let config_path = repo.path().join("config.toml");

    write_file(
        &manifest_path,
        r#"{
  "version": 1,
  "servers": [
    {
      "name": "playwright",
      "type": "stdio",
      "command": "npx",
      "args": ["@playwright/mcp@latest"]
    },
    {
      "name": "microsoftdocs",
      "type": "http",
      "url": "https://learn.microsoft.com/api/mcp"
    }
  ]
}"#,
    );
    write_file(
        &config_path,
        "model = \"gpt-5\"\n\n[tools]\nsearch = true\n",
    );

    let result = invoke_sync_codex_mcp_config(&RuntimeSyncCodexMcpConfigRequest {
        repo_root: Some(repo.path().to_path_buf()),
        manifest_path: Some(manifest_path),
        target_config_path: Some(config_path.clone()),
        create_backup: true,
        dry_run: true,
        ..RuntimeSyncCodexMcpConfigRequest::default()
    })
    .expect("dry-run sync should succeed");

    assert_eq!(result.servers_applied, 2);
    assert!(result
        .backup_path
        .as_ref()
        .is_some_and(|path| path.is_file()));
    assert!(result
        .rendered_document
        .contains("[mcp_servers.playwright]"));
    assert!(result
        .rendered_document
        .contains("url = \"https://learn.microsoft.com/api/mcp\""));
    assert_eq!(
        fs::read_to_string(config_path).expect("config should be readable"),
        "model = \"gpt-5\"\n\n[tools]\nsearch = true\n"
    );
}

#[test]
fn test_invoke_sync_codex_mcp_config_supports_catalog_input() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    write_catalog(repo.path());

    let config_path = repo.path().join("config.toml");
    write_file(
        &config_path,
        "model = \"gpt-5\"\n\n[tools]\nsearch = true\n",
    );

    let result = invoke_sync_codex_mcp_config(&RuntimeSyncCodexMcpConfigRequest {
        repo_root: Some(repo.path().to_path_buf()),
        target_config_path: Some(config_path.clone()),
        ..RuntimeSyncCodexMcpConfigRequest::default()
    })
    .expect("catalog-driven sync should succeed");

    assert_eq!(result.servers_applied, 2);
    let content = fs::read_to_string(config_path).expect("config should be readable");
    assert!(content.contains("[mcp_servers.microsoftdocs]"));
    assert!(content.contains("[mcp_servers.playwright]"));
    assert!(!content.contains("vscode-only"));

    let parsed_manifest: Value = serde_json::from_str(r#"{"servers":[{"name":"microsoftdocs"}]}"#)
        .expect("json should parse");
    assert_eq!(
        parsed_manifest["servers"][0]["name"].as_str(),
        Some("microsoftdocs")
    );
}