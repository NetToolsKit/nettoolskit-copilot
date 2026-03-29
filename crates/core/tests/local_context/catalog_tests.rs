//! Tests for local context catalog parsing and file discovery.

use nettoolskit_core::local_context::{
    local_context_index_file_candidates, local_context_index_path_included,
    read_local_context_index_catalog, resolve_local_context_index_root, LocalContextIndexCatalog,
};
use std::fs;
use tempfile::TempDir;

fn sample_catalog() -> LocalContextIndexCatalog {
    LocalContextIndexCatalog {
        version: 1,
        index_root: ".temp/context-index".to_string(),
        max_file_size_kb: 4,
        chunking: nettoolskit_core::local_context::LocalContextIndexChunking {
            max_chars: 100,
            max_lines: 10,
        },
        query_defaults: nettoolskit_core::local_context::LocalContextIndexQueryDefaults { top: 5 },
        include_globs: vec!["planning/**/*.md".to_string(), "README.md".to_string()],
        exclude_globs: vec![".temp/**".to_string()],
    }
}

#[test]
fn test_read_local_context_index_catalog_reads_versioned_document() {
    let repo = TempDir::new().expect("temporary repository should be created");
    let catalog_path = repo.path().join(".github/governance");
    fs::create_dir_all(&catalog_path).expect("catalog directory should be created");
    let catalog_file = catalog_path.join("local-context-index.catalog.json");
    fs::write(
        &catalog_file,
        r#"{"version":1,"indexRoot":".temp/context-index","maxFileSizeKb":4,"chunking":{"maxChars":100,"maxLines":10},"queryDefaults":{"top":5},"includeGlobs":["planning/**/*.md"],"excludeGlobs":[".temp/**"]}"#,
    )
    .expect("catalog file should be written");

    let catalog_info =
        read_local_context_index_catalog(repo.path(), None).expect("catalog should be parsed");

    assert_eq!(catalog_info.path, catalog_file);
    assert_eq!(catalog_info.catalog.version, 1);
    assert_eq!(catalog_info.catalog.query_defaults.top, 5);
}

#[test]
fn test_resolve_local_context_index_root_uses_catalog_default() {
    let repo = TempDir::new().expect("temporary repository should be created");
    let catalog = sample_catalog();

    let index_root = resolve_local_context_index_root(repo.path(), &catalog, None);

    assert_eq!(index_root, repo.path().join(".temp/context-index"));
}

#[test]
fn test_local_context_index_path_included_honors_include_and_exclude_globs() {
    let catalog = sample_catalog();

    assert!(
        local_context_index_path_included("planning/active/plan.md", &catalog)
            .expect("glob evaluation should succeed")
    );
    assert!(
        !local_context_index_path_included(".temp/context-index/index.json", &catalog)
            .expect("glob evaluation should succeed")
    );
}

#[test]
fn test_local_context_index_file_candidates_filters_size_and_excludes() {
    let repo = TempDir::new().expect("temporary repository should be created");
    let catalog = sample_catalog();

    fs::create_dir_all(repo.path().join("planning/active"))
        .expect("planning directory should be created");
    fs::create_dir_all(repo.path().join(".temp")).expect("temp directory should be created");
    fs::write(repo.path().join("planning/active/plan.md"), "# Plan")
        .expect("plan file should be written");
    fs::write(repo.path().join("README.md"), "readme").expect("readme should be written");
    fs::write(repo.path().join(".temp/ignored.md"), "ignored").expect("ignored file should exist");
    fs::write(
        repo.path().join("planning/active/too-large.md"),
        "x".repeat(5000),
    )
    .expect("large file should be written");

    let candidates = local_context_index_file_candidates(repo.path(), &catalog)
        .expect("candidate discovery should succeed");
    let relative_paths = candidates
        .iter()
        .map(|path| {
            path.strip_prefix(repo.path())
                .expect("candidate should be under repo")
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect::<Vec<_>>();

    assert_eq!(relative_paths, vec!["README.md", "planning/active/plan.md"]);
}
