//! Tests for native provider-surface rendering.

use crate::sync::provider_surface_test_support::{
    initialize_minimal_mcp_runtime_catalog, initialize_minimal_provider_surface_projection,
};
use nettoolskit_runtime::{
    invoke_render_provider_surfaces, RuntimeRenderProviderSurfacesRequest,
};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_invoke_render_provider_surfaces_supports_summary_only_for_direct_consumer() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_minimal_provider_surface_projection(repo.path());

    let result = invoke_render_provider_surfaces(&RuntimeRenderProviderSurfacesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        summary_only: true,
        ..RuntimeRenderProviderSurfacesRequest::default()
    })
    .expect("provider surface summary render should succeed");

    assert_eq!(result.consumer_name, "direct");
    assert_eq!(result.rendered_count, 0);
    assert!(result.summary_only);
    assert_eq!(result.selected_renderer_ids.len(), 9);
    assert!(!repo.path().join(".github/AGENTS.md").exists());
    assert!(!repo.path().join(".codex/mcp/servers.manifest.json").exists());
}

#[test]
fn test_invoke_render_provider_surfaces_renders_requested_direct_renderer_only() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_minimal_provider_surface_projection(repo.path());

    let result = invoke_render_provider_surfaces(&RuntimeRenderProviderSurfacesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        renderer_ids: vec!["github-instruction-surfaces".to_string()],
        ..RuntimeRenderProviderSurfacesRequest::default()
    })
    .expect("direct provider surface render should succeed");

    assert_eq!(result.consumer_name, "direct");
    assert_eq!(
        result.selected_renderer_ids,
        vec!["github-instruction-surfaces".to_string()]
    );
    assert_eq!(result.rendered_count, 1);
    assert!(repo.path().join(".github/AGENTS.md").is_file());
    assert!(repo
        .path()
        .join(".github/instructions/super-agent.instructions.md")
        .is_file());
    assert!(!repo.path().join(".vscode/profiles/profile-base.json").exists());
}

#[test]
fn test_invoke_render_provider_surfaces_preserves_bootstrap_gating() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_minimal_provider_surface_projection(repo.path());
    initialize_minimal_mcp_runtime_catalog(repo.path());

    let result = invoke_render_provider_surfaces(&RuntimeRenderProviderSurfacesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        consumer_name: Some("bootstrap".to_string()),
        enable_codex_runtime: true,
        enable_claude_runtime: false,
        ..RuntimeRenderProviderSurfacesRequest::default()
    })
    .expect("bootstrap provider surface render should succeed");

    assert_eq!(result.consumer_name, "bootstrap");
    assert_eq!(result.rendered_count, 6);
    assert!(repo.path().join(".github/AGENTS.md").is_file());
    assert!(repo.path().join(".vscode/profiles/profile-base.json").is_file());
    assert!(repo.path().join(".codex/mcp/README.md").is_file());
    assert!(repo.path().join(".codex/orchestration/flow.md").is_file());
    assert!(!repo.path().join(".claude/settings.json").exists());
}

#[test]
fn test_invoke_render_provider_surfaces_supports_mcp_runtime_renderer() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_minimal_provider_surface_projection(repo.path());
    initialize_minimal_mcp_runtime_catalog(repo.path());

    let result = invoke_render_provider_surfaces(&RuntimeRenderProviderSurfacesRequest {
        repo_root: Some(repo.path().to_path_buf()),
        renderer_ids: vec!["mcp-runtime-artifacts".to_string()],
        ..RuntimeRenderProviderSurfacesRequest::default()
    })
    .expect("mcp runtime renderer should succeed");

    assert_eq!(result.rendered_count, 1);
    assert!(repo.path().join(".vscode/mcp.tamplate.jsonc").is_file());
    assert!(repo.path().join(".codex/mcp/servers.manifest.json").is_file());
    let manifest = fs::read_to_string(repo.path().join(".codex/mcp/servers.manifest.json"))
        .expect("rendered manifest should be readable");
    assert!(manifest.contains("\"playwright\""));
}
