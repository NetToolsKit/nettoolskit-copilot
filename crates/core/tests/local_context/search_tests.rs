//! Tests for local context search ranking.

use nettoolskit_core::local_context::{
    search_local_context_index_document, LocalContextChunk, LocalContextChunkKind,
    LocalContextIndexDocument,
};

fn sample_document() -> LocalContextIndexDocument {
    LocalContextIndexDocument {
        version: 1,
        generated_at: "2026-03-26T20:00:00Z".to_string(),
        repo_root: "C:/repo".to_string(),
        catalog_path: ".github/governance/local-context-index.catalog.json".to_string(),
        chunk_count: 2,
        files: Vec::new(),
        chunks: vec![
            LocalContextChunk {
                id: "planning/active/plan.md::0".to_string(),
                path: "planning/active/plan.md".to_string(),
                kind: LocalContextChunkKind::Markdown,
                heading: Some("Rust migration plan".to_string()),
                text: "The runtime rewrite tracks wave one and query contracts.".to_string(),
                search_text: "rust migration plan the runtime rewrite tracks wave one and query contracts".to_string(),
            },
            LocalContextChunk {
                id: "scripts/runtime/query-local-context-index.ps1::0".to_string(),
                path: "scripts/runtime/query-local-context-index.ps1".to_string(),
                kind: LocalContextChunkKind::Text,
                heading: None,
                text: "Query the local context index and emit json output.".to_string(),
                search_text: "scripts/runtime/query-local-context-index.ps1 query the local context index and emit json output".to_string(),
            },
        ],
    }
}

#[test]
fn test_search_local_context_index_document_prefers_path_and_heading_matches() {
    let hits = search_local_context_index_document("rust migration", &sample_document(), 5, &[]);

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].path, "planning/active/plan.md");
    assert_eq!(hits[0].heading.as_deref(), Some("Rust migration plan"));
}

#[test]
fn test_search_local_context_index_document_honors_excluded_paths() {
    let hits = search_local_context_index_document(
        "local context index",
        &sample_document(),
        5,
        &["scripts/runtime/query-local-context-index.ps1".to_string()],
    );

    assert!(hits.is_empty());
}

#[test]
fn test_search_local_context_index_document_limits_results() {
    let hits = search_local_context_index_document("context", &sample_document(), 1, &[]);

    assert_eq!(hits.len(), 1);
}