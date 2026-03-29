//! Tests for local context document building and persistence.

use nettoolskit_core::local_context::{
    build_local_context_chunks_for_file, build_local_context_index,
    read_local_context_index_document, write_local_context_index_document, LocalContextChunkKind,
    LocalContextIndexCatalog, LocalContextIndexCatalogInfo, LocalContextIndexChunking,
    LocalContextIndexDocument, LocalContextIndexQueryDefaults,
};
use std::fs;
use tempfile::TempDir;

fn sample_catalog() -> LocalContextIndexCatalog {
    LocalContextIndexCatalog {
        version: 1,
        index_root: ".temp/context-index".to_string(),
        max_file_size_kb: 16,
        chunking: LocalContextIndexChunking {
            max_chars: 120,
            max_lines: 4,
        },
        query_defaults: LocalContextIndexQueryDefaults { top: 5 },
        include_globs: vec![
            "planning/**/*.md".to_string(),
            "scripts/**/*.ps1".to_string(),
        ],
        exclude_globs: vec![".temp/**".to_string()],
    }
}

#[test]
fn test_build_local_context_chunks_for_markdown_uses_heading_chunks() {
    let repo = TempDir::new().expect("temporary repository should be created");
    fs::create_dir_all(repo.path().join("planning/active"))
        .expect("planning directory should be created");
    let file_path = repo.path().join("planning/active/plan.md");
    fs::write(
        &file_path,
        "# Intro\nalpha beta gamma\n## Details\ndelta epsilon zeta",
    )
    .expect("markdown file should be written");

    let chunks = build_local_context_chunks_for_file(repo.path(), &file_path, &sample_catalog())
        .expect("chunks should be built");

    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0].kind, LocalContextChunkKind::Markdown);
    assert_eq!(chunks[0].heading.as_deref(), Some("Intro"));
    assert_eq!(chunks[1].heading.as_deref(), Some("Details"));
}

#[test]
fn test_build_local_context_index_reuses_unchanged_files() {
    let repo = TempDir::new().expect("temporary repository should be created");
    let catalog = sample_catalog();
    let catalog_dir = repo.path().join(".github/governance");
    fs::create_dir_all(&catalog_dir).expect("catalog directory should be created");
    let catalog_path = catalog_dir.join("local-context-index.catalog.json");
    fs::write(
        &catalog_path,
        r#"{"version":1,"indexRoot":".temp/context-index","maxFileSizeKb":16,"chunking":{"maxChars":120,"maxLines":4},"queryDefaults":{"top":5},"includeGlobs":["planning/**/*.md","scripts/**/*.ps1"],"excludeGlobs":[".temp/**"]}"#,
    )
    .expect("catalog file should be written");
    fs::create_dir_all(repo.path().join("planning/active"))
        .expect("planning directory should be created");
    fs::write(
        repo.path().join("planning/active/plan.md"),
        "# Intro\nalpha beta gamma",
    )
    .expect("plan file should be written");

    let catalog_info = LocalContextIndexCatalogInfo {
        path: catalog_path,
        catalog,
    };
    let first_report = build_local_context_index(repo.path(), &catalog_info, None, false)
        .expect("first build should succeed");
    let second_report = build_local_context_index(repo.path(), &catalog_info, None, false)
        .expect("second build should succeed");

    assert_eq!(first_report.indexed_file_count, 1);
    assert_eq!(first_report.rebuilt_file_count, 1);
    assert_eq!(first_report.reused_file_count, 0);
    assert_eq!(second_report.indexed_file_count, 1);
    assert_eq!(second_report.rebuilt_file_count, 0);
    assert_eq!(second_report.reused_file_count, 1);
    assert_eq!(
        second_report.document.chunk_count,
        second_report.document.chunks.len()
    );
}

#[test]
fn test_write_and_read_local_context_index_document_round_trip() {
    let repo = TempDir::new().expect("temporary repository should be created");
    let document = LocalContextIndexDocument {
        version: 1,
        generated_at: "2026-03-26T20:00:00Z".to_string(),
        repo_root: repo.path().to_string_lossy().to_string(),
        catalog_path: ".github/governance/local-context-index.catalog.json".to_string(),
        chunk_count: 0,
        files: Vec::new(),
        chunks: Vec::new(),
    };

    let index_root = repo.path().join(".temp/context-index");
    let index_path = write_local_context_index_document(&index_root, &document)
        .expect("document should be written");
    let loaded_document = read_local_context_index_document(&index_root)
        .expect("document should be read")
        .expect("document should exist");

    assert_eq!(index_path, index_root.join("index.json"));
    assert_eq!(loaded_document, document);
}
