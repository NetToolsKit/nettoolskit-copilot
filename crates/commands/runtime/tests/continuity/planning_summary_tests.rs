//! Tests for planning summary export commands.

use nettoolskit_core::local_context::build_local_context_index;
use nettoolskit_runtime::{export_planning_summary, ExportPlanningSummaryRequest};
use std::fs;
use tempfile::TempDir;

fn write_local_context_catalog(repo_root: &std::path::Path) {
    let catalog_dir = repo_root.join(".github/governance");
    fs::create_dir_all(&catalog_dir).expect("catalog directory should be created");
    fs::write(
        catalog_dir.join("local-context-index.catalog.json"),
        r#"{"version":1,"indexRoot":".temp/context-index","maxFileSizeKb":16,"chunking":{"maxChars":160,"maxLines":10},"queryDefaults":{"top":5},"includeGlobs":["README.md","planning/**/*.md","scripts/**/*.ps1"],"excludeGlobs":[".temp/**"]}"#,
    )
    .expect("catalog should be written");
}

#[test]
fn test_export_planning_summary_renders_workspace_planning_surface_and_references() {
    let repo = TempDir::new().expect("temporary repository should be created");
    write_local_context_catalog(repo.path());
    fs::create_dir_all(repo.path().join("planning/active"))
        .expect("active planning directory should be created");
    fs::create_dir_all(repo.path().join("planning/specs/active"))
        .expect("active spec directory should be created");
    fs::create_dir_all(repo.path().join("scripts/runtime"))
        .expect("runtime scripts directory should be created");

    fs::write(
        repo.path().join("planning/active/plan-wave1.md"),
        "# Wave 1 Plan\n\n- Status: in progress\n- Current focus: runtime rewrite and local context continuity\n",
    )
    .expect("plan file should be written");
    fs::write(
        repo.path().join("planning/specs/active/spec-wave1.md"),
        "# Runtime Spec\n\nObjective: runtime rewrite and local context continuity\n",
    )
    .expect("spec file should be written");
    fs::write(
        repo.path().join("README.md"),
        "# Runtime Rewrite\nThis repository tracks runtime rewrite and local context continuity.\n",
    )
    .expect("readme should be written");
    fs::write(
        repo.path().join("scripts/runtime/demo.ps1"),
        "Write-Output 'runtime rewrite'",
    )
    .expect("runtime script should be written");

    let catalog_info =
        nettoolskit_core::local_context::read_local_context_index_catalog(repo.path(), None)
            .expect("catalog should be readable");
    build_local_context_index(repo.path(), &catalog_info, None, false)
        .expect("index should be built");

    let result = export_planning_summary(&ExportPlanningSummaryRequest {
        repo_root: Some(repo.path().to_path_buf()),
        output_path: None,
        print_only: true,
    })
    .expect("planning summary should render");

    assert_eq!(result.plan_root, "planning/active");
    assert_eq!(result.spec_root, "planning/specs/active");
    assert!(result.output_path.is_none());
    assert!(result.document.contains("## Active Plans"));
    assert!(result.document.contains("### Wave 1 Plan"));
    assert!(result.document.contains("`planning/active/plan-wave1.md`"));
    assert!(result.document.contains("## Active Specs"));
    assert!(result.document.contains("## Suggested Local References"));
    assert!(
        result.document.contains("`README.md`")
            || result
                .document
                .contains("`scripts/runtime/demo.ps1`")
    );
    assert!(result.document.contains("## Resume Instructions"));
}

#[test]
fn test_export_planning_summary_uses_build_fallback_when_workspace_planning_is_missing() {
    let repo = TempDir::new().expect("temporary repository should be created");
    fs::create_dir_all(repo.path().join(".build/super-agent/planning/active"))
        .expect("fallback planning directory should be created");
    fs::create_dir_all(repo.path().join(".build/super-agent/specs/active"))
        .expect("fallback spec directory should be created");
    fs::write(
        repo.path()
            .join(".build/super-agent/planning/active/plan-fallback.md"),
        "# Fallback Plan\n\nSummary: fallback active planning surface\n",
    )
    .expect("fallback plan should be written");
    fs::write(
        repo.path()
            .join(".build/super-agent/specs/active/spec-fallback.md"),
        "# Fallback Spec\n\nSummary: fallback active spec surface\n",
    )
    .expect("fallback spec should be written");

    let result = export_planning_summary(&ExportPlanningSummaryRequest {
        repo_root: Some(repo.path().to_path_buf()),
        output_path: None,
        print_only: true,
    })
    .expect("planning summary should render");

    assert_eq!(result.plan_root, ".build/super-agent/planning/active");
    assert_eq!(result.spec_root, ".build/super-agent/specs/active");
    assert!(result
        .document
        .contains("`.build/super-agent/planning/active/plan-fallback.md`"));
    assert!(result
        .document
        .contains("`.build/super-agent/specs/active/spec-fallback.md`"));
}

#[test]
fn test_export_planning_summary_writes_output_file_when_requested() {
    let repo = TempDir::new().expect("temporary repository should be created");
    fs::create_dir_all(repo.path().join("planning/active"))
        .expect("active planning directory should be created");
    fs::write(
        repo.path().join("planning/active/plan-wave1.md"),
        "# Wave 1 Plan\n\nStatus: done\n",
    )
    .expect("plan file should be written");

    let output_path = repo.path().join(".temp/context-handoff-custom.md");
    let result = export_planning_summary(&ExportPlanningSummaryRequest {
        repo_root: Some(repo.path().to_path_buf()),
        output_path: Some(output_path.clone()),
        print_only: false,
    })
    .expect("planning summary should be written");

    assert_eq!(result.output_path, Some(output_path.clone()));
    assert!(output_path.is_file());
    let persisted = fs::read_to_string(output_path).expect("persisted document should be readable");
    assert_eq!(persisted, result.document);
}
