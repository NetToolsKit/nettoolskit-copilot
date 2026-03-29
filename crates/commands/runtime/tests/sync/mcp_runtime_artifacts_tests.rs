//! Tests for native MCP runtime artifact rendering.

use nettoolskit_runtime::{
    invoke_render_mcp_runtime_artifacts, invoke_render_vscode_mcp_template,
    RuntimeRenderMcpRuntimeArtifactsRequest, RuntimeRenderVscodeMcpTemplateRequest,
};
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

fn initialize_repo_root(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github/governance"))
        .expect(".github/governance should be created");
    fs::create_dir_all(repo_root.join(".codex/mcp")).expect(".codex/mcp should be created");
    fs::create_dir_all(repo_root.join(".vscode")).expect(".vscode should be created");
}

fn write_catalog(repo_root: &Path) {
    write_file(
        &repo_root.join(".github/governance/mcp-runtime.catalog.json"),
        r#"{
  "version": 1,
  "inputs": [
    {
      "id": "Authorization",
      "type": "promptString",
      "description": "Token",
      "password": true
    }
  ],
  "servers": [
    {
      "id": "microsoft/playwright-mcp",
      "codexName": "playwright",
      "targets": {
        "vscode": { "include": true, "enabledByDefault": true },
        "codex": { "include": true }
      },
      "definition": {
        "type": "stdio",
        "command": "npx",
        "args": ["@playwright/mcp@latest"],
        "gallery": "https://example.invalid/gallery",
        "version": "1.0.0"
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
fn test_invoke_render_vscode_mcp_template_supports_custom_output_path() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    write_catalog(repo.path());

    let output_path = repo.path().join(".temp/vscode.mcp.generated.json");
    let result = invoke_render_vscode_mcp_template(&RuntimeRenderVscodeMcpTemplateRequest {
        repo_root: Some(repo.path().to_path_buf()),
        output_path: Some(output_path.clone()),
        ..RuntimeRenderVscodeMcpTemplateRequest::default()
    })
    .expect("vscode mcp template render should succeed");

    assert_eq!(result.input_count, 1);
    assert_eq!(result.server_count, 2);
    let document: Value = serde_json::from_str(
        &fs::read_to_string(output_path).expect("rendered template should be readable"),
    )
    .expect("rendered template should parse");
    assert_eq!(
        document["servers"]["vscode-only/example"]["disabled"].as_bool(),
        Some(true)
    );
}

#[test]
fn test_invoke_render_mcp_runtime_artifacts_writes_tracked_outputs() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_repo_root(repo.path());
    write_catalog(repo.path());

    let result = invoke_render_mcp_runtime_artifacts(&RuntimeRenderMcpRuntimeArtifactsRequest {
        repo_root: Some(repo.path().to_path_buf()),
        ..RuntimeRenderMcpRuntimeArtifactsRequest::default()
    })
    .expect("tracked artifact render should succeed");

    assert_eq!(result.input_count, 1);
    assert_eq!(result.vscode_server_count, 2);
    assert_eq!(result.codex_server_count, 1);
    assert!(repo.path().join(".vscode/mcp.tamplate.jsonc").is_file());
    assert!(repo.path().join(".codex/mcp/servers.manifest.json").is_file());

    let manifest: Value = serde_json::from_str(
        &fs::read_to_string(repo.path().join(".codex/mcp/servers.manifest.json"))
            .expect("rendered manifest should be readable"),
    )
    .expect("rendered manifest should parse");
    assert_eq!(manifest["servers"].as_array().map(Vec::len), Some(1));
    assert_eq!(
        manifest["servers"][0]["name"].as_str(),
        Some("playwright")
    );
}
