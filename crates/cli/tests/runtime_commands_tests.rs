//! Tests for executable runtime command surfaces exposed by `ntk`.

use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn ntk() -> Command {
    cargo_bin_cmd!("ntk")
}

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

fn initialize_git_repo(repo_root: &Path) {
    let output = std::process::Command::new("git")
        .arg("init")
        .arg(repo_root)
        .output()
        .expect("git init should execute");
    assert!(output.status.success(), "git init should succeed");
}

fn initialize_runtime_repo_root(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github")).expect(".github should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect(".codex should be created");
}

fn write_runtime_install_profile_catalog(repo_root: &Path) {
    write_governance_file(
        repo_root,
        "runtime-install-profiles.json",
        r#"{"schemaVersion":1,"defaultProfile":"none","profiles":{"none":{"description":"none profile","install":{"bootstrap":false,"globalVscodeSettings":false,"globalVscodeSnippets":false,"localGitHooks":false,"globalGitAliases":false,"healthcheck":false},"runtime":{"github":false,"codex":false,"claude":false}}}}"#,
    );
}

fn write_validation_profile_catalog(repo_root: &Path) {
    write_governance_file(
        repo_root,
        "validation-profiles.json",
        r#"{"version":1,"defaultProfile":"dev","profiles":[{"id":"dev","warningOnly":false,"checkOrder":["validate-planning-structure"]}]}"#,
    );
}

fn write_mcp_runtime_catalog(repo_root: &Path) {
    write_governance_file(
        repo_root,
        "mcp-runtime.catalog.json",
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
        "args": ["@playwright/mcp@latest"]
      }
    },
    {
      "id": "microsoftdocs/mcp",
      "codexName": "microsoftdocs",
      "targets": {
        "vscode": { "include": true, "enabledByDefault": false },
        "codex": { "include": true }
      },
      "definition": {
        "type": "http",
        "url": "https://learn.microsoft.com/api/mcp"
      }
    }
  ]
}"#,
    );
}

fn initialize_minimal_provider_surface_projection(repo_root: &Path) {
    write_governance_file(
        repo_root,
        "provider-surface-projection.catalog.json",
        r#"{"version":1,"renderers":[{"id":"github-instruction-surfaces","consumers":{"bootstrap":{"enabled":true,"order":10,"condition":"always"},"direct":{"enabled":true,"order":10}}},{"id":"vscode-profile-surfaces","consumers":{"bootstrap":{"enabled":true,"order":20,"condition":"always"},"direct":{"enabled":true,"order":20}}},{"id":"vscode-workspace-surfaces","consumers":{"bootstrap":{"enabled":true,"order":30,"condition":"always"},"direct":{"enabled":true,"order":30}}},{"id":"codex-compatibility-surfaces","consumers":{"bootstrap":{"enabled":true,"order":40,"condition":"codex"},"direct":{"enabled":true,"order":40}}},{"id":"codex-skill-surfaces","consumers":{"bootstrap":{"enabled":true,"order":50,"condition":"codex"},"direct":{"enabled":true,"order":50}}},{"id":"codex-orchestration-surfaces","consumers":{"bootstrap":{"enabled":true,"order":60,"condition":"codex"},"direct":{"enabled":true,"order":60}}},{"id":"claude-runtime-surfaces","consumers":{"bootstrap":{"enabled":true,"order":70,"condition":"claude"},"direct":{"enabled":true,"order":70}}},{"id":"claude-skill-surfaces","consumers":{"bootstrap":{"enabled":false,"order":80,"condition":"claude"},"direct":{"enabled":true,"order":80},"claudeSkillSync":{"enabled":true,"order":10}}},{"id":"mcp-runtime-artifacts","consumers":{"bootstrap":{"enabled":false,"order":90,"condition":"never"},"direct":{"enabled":true,"order":90}}}]}"#,
    );

    write_file(
        &repo_root.join("definitions/providers/github/root/AGENTS.md"),
        "# Managed agents",
    );
    write_file(
        &repo_root.join("definitions/providers/github/root/COMMANDS.md"),
        "# Managed commands",
    );
    write_file(
        &repo_root.join("definitions/providers/github/root/copilot-instructions.md"),
        "# Managed instructions",
    );
    write_file(
        &repo_root.join("definitions/providers/github/root/instruction-routing.catalog.yml"),
        "version: 1",
    );
    write_file(
        &repo_root.join("definitions/providers/github/agents/super-agent.agent.md"),
        "# Super Agent",
    );
    write_file(
        &repo_root.join("definitions/providers/github/chatmodes/demo.chatmode.md"),
        "# Demo Chatmode",
    );
    write_file(
        &repo_root.join("definitions/providers/github/hooks/scripts/session-start.ps1"),
        "Write-Output 'session-start'",
    );
    write_file(
        &repo_root.join("definitions/providers/github/prompts/route-instructions.prompt.md"),
        "# Route Instructions",
    );
    write_file(
        &repo_root.join("definitions/agents/super-agent/ntk-agents-super-agent.instructions.md"),
        "# Shared instruction",
    );
    write_file(
        &repo_root.join("definitions/templates/docs/readme-template.md"),
        "# Readme Template",
    );
    write_file(
        &repo_root.join("definitions/shared/prompts/poml/prompt-engineering-poml.md"),
        "# Shared POML",
    );

    write_file(
        &repo_root.join("definitions/providers/vscode/profiles/profile-base.json"),
        "{}",
    );
    write_file(
        &repo_root.join("definitions/providers/vscode/workspace/README.md"),
        "# VS Code Workspace",
    );
    write_file(
        &repo_root.join("definitions/providers/vscode/workspace/base.code-workspace"),
        "{}",
    );
    write_file(
        &repo_root.join("definitions/providers/vscode/workspace/settings.tamplate.jsonc"),
        "{}",
    );
    write_file(
        &repo_root
            .join("definitions/providers/vscode/workspace/snippets/demo.tamplate.code-snippets"),
        "{}",
    );

    write_file(
        &repo_root.join("definitions/providers/codex/scripts/root-tool.ps1"),
        "Write-Output 'tool'",
    );
    write_file(
        &repo_root.join("definitions/providers/codex/mcp/README.md"),
        "# Codex MCP",
    );
    write_file(
        &repo_root.join("definitions/providers/codex/mcp/codex.config.template.toml"),
        "[mcp]",
    );
    write_file(
        &repo_root.join("definitions/providers/codex/mcp/vscode.mcp.template.json"),
        "{}",
    );
    write_file(
        &repo_root.join("definitions/providers/codex/orchestration/flow.md"),
        "# orchestration flow",
    );
    write_file(
        &repo_root.join("definitions/providers/codex/skills/runtime-skill/SKILL.md"),
        "# runtime-skill",
    );

    write_file(
        &repo_root.join("definitions/providers/claude/runtime/settings.json"),
        "{\"theme\":\"repo\"}",
    );
    write_file(
        &repo_root.join("definitions/providers/claude/skills/review-code-engineer/SKILL.md"),
        "# review-code-engineer",
    );
}

fn initialize_runtime_health_repo(repo_root: &Path) {
    initialize_runtime_repo_root(repo_root);
    fs::create_dir_all(repo_root.join("scripts/runtime"))
        .expect("runtime directory should be created");
    fs::create_dir_all(repo_root.join("scripts/validation"))
        .expect("validation directory should be created");
    write_file(&repo_root.join("planning/README.md"), "# planning\n");
    write_file(&repo_root.join("planning/specs/README.md"), "# specs\n");
    write_runtime_install_profile_catalog(repo_root);
    write_validation_profile_catalog(repo_root);
}

fn write_local_context_catalog(repo_root: &Path) {
    write_governance_file(
        repo_root,
        "local-context-index.catalog.json",
        r#"{"version":1,"indexRoot":".temp/context-index","maxFileSizeKb":64,"chunking":{"maxChars":400,"maxLines":20},"queryDefaults":{"top":3},"includeGlobs":["README.md","planning/**/*.md","scripts/**/*.ps1",".github/**/*.md"],"excludeGlobs":[".temp/**"]}"#,
    );
}

#[test]
fn test_runtime_pre_tool_use_emits_hook_specific_output_json() {
    let workspace = TempDir::new().expect("temporary workspace should be created");
    write_file(
        &workspace.path().join(".editorconfig"),
        "root = true\n\n[*]\ninsert_final_newline = false\n",
    );
    let payload = json!({
        "cwd": workspace.path(),
        "tool_name": "createFile",
        "tool_input": {
            "filePath": "README.md",
            "content": "# Title\n"
        }
    });

    ntk()
        .args(["runtime", "pre-tool-use"])
        .write_stdin(payload.to_string())
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r##""hookEventName":"PreToolUse""##,
        ))
        .stdout(predicate::str::contains(
            r##""updatedInput":{"content":"# Title""##,
        ))
        .stdout(predicate::str::contains(r##""filePath":"README.md""##));
}

#[test]
fn test_runtime_trim_trailing_blank_lines_reports_git_changed_only_files() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_file(
        &repo.path().join(".editorconfig"),
        "root = true\n\n[*]\ninsert_final_newline = false\n",
    );
    initialize_git_repo(repo.path());
    write_file(
        &repo.path().join("changed.cs"),
        "public sealed class Changed { }\n\n",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "trim-trailing-blank-lines", "--git-changed-only"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Git changed files mode: enabled"))
        .stdout(predicate::str::contains("Files found: 2"))
        .stdout(predicate::str::contains(".editorconfig"))
        .stdout(predicate::str::contains("changed.cs"));

    assert_eq!(
        fs::read_to_string(repo.path().join("changed.cs"))
            .expect("changed file should be readable"),
        "public sealed class Changed { }"
    );
}

#[test]
fn test_runtime_trim_trailing_blank_lines_supports_plain_git_repos_without_runtime_markers() {
    let repo = TempDir::new().expect("temporary repository should be created");
    write_file(
        &repo.path().join(".editorconfig"),
        "root = true\n\n[*]\ninsert_final_newline = false\n",
    );
    initialize_git_repo(repo.path());
    write_file(
        &repo.path().join("changed.cs"),
        "public sealed class Changed { }\n\n",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "trim-trailing-blank-lines", "--git-changed-only"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Files found: 2"))
        .stdout(predicate::str::contains(".editorconfig"))
        .stdout(predicate::str::contains("changed.cs"));

    assert_eq!(
        fs::read_to_string(repo.path().join("changed.cs"))
            .expect("changed file should be readable"),
        "public sealed class Changed { }"
    );
}

#[test]
fn test_runtime_update_local_context_index_builds_the_index_document() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_local_context_catalog(repo.path());
    write_file(
        &repo.path().join("README.md"),
        "# Demo\n\nContinuity summary for the local context index.",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "update-local-context-index"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Local context index updated:"))
        .stdout(predicate::str::contains("Files indexed:"));

    assert!(repo.path().join(".temp/context-index/index.json").is_file());
}

#[test]
fn test_runtime_query_local_context_index_supports_json_output() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_local_context_catalog(repo.path());
    write_file(
        &repo.path().join("README.md"),
        "# Demo\n\nContinuity summary for the local context index.",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "update-local-context-index"])
        .assert()
        .success();

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "query-local-context-index",
            "--query-text",
            "continuity summary",
            "--path-prefix",
            "README",
            "--json-output",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""schema_version": 1"#))
        .stdout(predicate::str::contains(
            r#""schema_kind": "local_context_query""#,
        ))
        .stdout(predicate::str::contains(r#""backend": "sqlite-default""#))
        .stdout(predicate::str::contains(r#""result_count": 1"#))
        .stdout(predicate::str::contains(r#""path": "README.md""#))
        .stdout(predicate::str::contains(r#""memory_db_path": "#));
}

#[test]
fn test_runtime_query_local_context_index_supports_compatibility_json_flag() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_local_context_catalog(repo.path());
    write_file(
        &repo.path().join("README.md"),
        "# Demo\n\nCompatibility JSON fallback remains available.",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "update-local-context-index"])
        .assert()
        .success();

    fs::remove_file(repo.path().join(".temp/context-memory/context.db"))
        .expect("sqlite memory db should be removable");

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "query-local-context-index",
            "--query-text",
            "compatibility fallback",
            "--use-json-index",
            "--json-output",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""schema_version": 1"#))
        .stdout(predicate::str::contains(
            r#""schema_kind": "local_context_query""#,
        ))
        .stdout(predicate::str::contains(
            r#""backend": "json-compatibility""#,
        ))
        .stdout(predicate::str::contains(r#""result_count": 1"#))
        .stdout(predicate::str::contains(r#""path": "README.md""#));
}

#[test]
fn test_runtime_update_local_memory_builds_sqlite_store() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_local_context_catalog(repo.path());
    write_file(
        &repo.path().join("README.md"),
        "# Demo\n\nSQLite local memory continuity summary.",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "update-local-memory"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Local memory store updated:"))
        .stdout(predicate::str::contains("Compatibility index:"));

    assert!(repo
        .path()
        .join(".temp/context-memory/context.db")
        .is_file());
    assert!(repo.path().join(".temp/context-index/index.json").is_file());
}

#[test]
fn test_runtime_query_local_memory_supports_filters_and_json_output() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_local_context_catalog(repo.path());
    write_file(
        &repo.path().join("README.md"),
        "# Demo\n\nSQLite local memory continuity summary.",
    );
    write_file(
        &repo.path().join("planning/active/plan.md"),
        "# Wave 1\n\nSQLite memory for continuity routing.",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "update-local-memory"])
        .assert()
        .success();

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "query-local-memory",
            "--query-text",
            "sqlite memory",
            "--path-prefix",
            "planning/",
            "--heading-contains",
            "wave",
            "--json-output",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""schema_version": 1"#))
        .stdout(predicate::str::contains(
            r#""schema_kind": "local_memory_query""#,
        ))
        .stdout(predicate::str::contains(r#""result_count": 1"#))
        .stdout(predicate::str::contains(
            r#""path": "planning/active/plan.md""#,
        ))
        .stdout(predicate::str::contains(r#""memory_db_path": "#));
}

#[test]
fn test_runtime_export_planning_summary_prints_active_plan_context() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_local_context_catalog(repo.path());
    write_file(
        &repo.path().join("planning/active/plan-wave4.md"),
        "# Wave 4 Plan\n\n- Status: in progress\n- Current focus: retire continuity wrappers.\n",
    );
    write_file(
        &repo.path().join("planning/specs/active/spec-wave4.md"),
        "# Wave 4 Spec\n\nObjective: move continuity execution to ntk runtime.\n",
    );
    write_file(
        &repo.path().join("README.md"),
        "# Runtime Rewrite\n\nRetire continuity wrappers with native entrypoints.",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "update-local-context-index"])
        .assert()
        .success();

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "export-planning-summary", "--print-only"])
        .assert()
        .success()
        .stdout(predicate::str::contains("## Active Plans"))
        .stdout(predicate::str::contains("Wave 4 Plan"))
        .stdout(predicate::str::contains("## Resume Instructions"));
}

#[test]
fn test_runtime_apply_vscode_templates_copies_workspace_templates() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_file(
        &repo.path().join(".vscode/settings.tamplate.jsonc"),
        "{\n  \"editor.tabSize\": 4\n}",
    );
    write_file(
        &repo.path().join(".vscode/mcp.tamplate.jsonc"),
        "{\n  \"servers\": []\n}",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "apply-vscode-templates"])
        .assert()
        .success()
        .stdout(predicate::str::contains("VS Code template apply summary"))
        .stdout(predicate::str::contains("applied: 2"));

    assert!(repo.path().join(".vscode/settings.json").is_file());
    assert!(repo.path().join(".vscode/mcp.json").is_file());
}

#[test]
fn test_runtime_render_vscode_mcp_template_cli_writes_requested_output_path() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_mcp_runtime_catalog(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "render-vscode-mcp-template",
            "--output-path",
            ".temp/vscode.mcp.generated.json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated:"))
        .stdout(predicate::str::contains("Servers rendered: 2"));

    assert!(repo
        .path()
        .join(".temp/vscode.mcp.generated.json")
        .is_file());
}

#[test]
fn test_runtime_render_provider_surfaces_cli_supports_summary_only() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    initialize_minimal_provider_surface_projection(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "render-provider-surfaces", "--summary-only"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Provider surface render selection",
        ))
        .stdout(predicate::str::contains("Consumer: direct"))
        .stdout(predicate::str::contains("Selected renderers: 9"))
        .stdout(predicate::str::contains("github-instruction-surfaces"));

    assert!(!repo.path().join(".github/AGENTS.md").exists());
}

#[test]
fn test_runtime_render_provider_surfaces_cli_renders_requested_renderer() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    initialize_minimal_provider_surface_projection(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "render-provider-surfaces",
            "--renderer-id",
            "github-instruction-surfaces",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Renderers invoked: 1"))
        .stdout(predicate::str::contains("github-instruction-surfaces"));

    assert!(repo.path().join(".github/AGENTS.md").is_file());
    assert!(repo
        .path()
        .join(".github/agents/super-agent.agent.md")
        .is_file());
    assert!(repo
        .path()
        .join(".github/templates/docs/readme-template.md")
        .is_file());
}

#[test]
fn test_runtime_render_mcp_runtime_artifacts_cli_writes_tracked_outputs() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_mcp_runtime_catalog(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "render-mcp-runtime-artifacts"])
        .assert()
        .success()
        .stdout(predicate::str::contains("MCP runtime render summary"))
        .stdout(predicate::str::contains("Codex servers: 2"));

    assert!(repo.path().join(".vscode/mcp.tamplate.jsonc").is_file());
    assert!(repo
        .path()
        .join(".codex/mcp/servers.manifest.json")
        .is_file());
}

#[test]
fn test_runtime_sync_codex_mcp_config_cli_supports_manifest_dry_run() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
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
    }
  ]
}"#,
    );
    write_file(&config_path, "model = \"gpt-5\"\n");

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "sync-codex-mcp-config",
            "--manifest-path",
            "servers.manifest.json",
            "--target-config-path",
            "config.toml",
            "--create-backup",
            "--dry-run",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("[mcp_servers.playwright]"))
        .stdout(predicate::str::contains(
            "Dry-run only. No file changes were written.",
        ));

    assert_eq!(
        fs::read_to_string(repo.path().join("config.toml")).expect("config should be readable"),
        "model = \"gpt-5\"\n"
    );
}

#[test]
fn test_runtime_healthcheck_cli_writes_report_to_requested_output_path() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_health_repo(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "healthcheck",
            "--runtime-profile",
            "none",
            "--validation-profile",
            "dev",
            "--warning-only",
            "false",
            "--output-path",
            ".temp/audit-report.json",
            "--log-path",
            ".temp/logs/audit-report.log",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Output path:"));

    assert!(repo.path().join(".temp/audit-report.json").is_file());
    assert!(repo.path().join(".temp/logs/audit-report.log").is_file());
}

#[test]
fn test_runtime_healthcheck_cli_supports_json_output_with_control_schema() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_health_repo(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "healthcheck",
            "--runtime-profile",
            "none",
            "--json-output",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#""schema_kind": "runtime_healthcheck""#,
        ))
        .stdout(predicate::str::contains(
            r#""runtime_profile_name": "none""#,
        ))
        .stdout(predicate::str::contains(r#""validation_profile": "dev""#))
        .stdout(predicate::str::contains(r#""overall_status": "passed""#));
}

#[test]
fn test_runtime_self_heal_cli_writes_report_to_requested_output_path() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_health_repo(repo.path());
    initialize_minimal_provider_surface_projection(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "self-heal",
            "--runtime-profile",
            "none",
            "--output-path",
            ".temp/self-heal-report.json",
            "--log-path",
            ".temp/logs/self-heal.log",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Runtime profile: none"))
        .stdout(predicate::str::contains("Total steps: 2"))
        .stdout(predicate::str::contains("Failed steps: 0"));

    assert!(repo.path().join(".temp/self-heal-report.json").is_file());
    assert!(repo.path().join(".temp/logs/self-heal.log").is_file());
}

#[test]
fn test_runtime_self_heal_cli_supports_json_output_with_control_schema() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_health_repo(repo.path());
    initialize_minimal_provider_surface_projection(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "self-heal",
            "--runtime-profile",
            "none",
            "--json-output",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#""schema_kind": "runtime_self_heal""#,
        ))
        .stdout(predicate::str::contains(
            r#""runtime_profile_name": "none""#,
        ))
        .stdout(predicate::str::contains(r#""overall_status": "passed""#))
        .stdout(predicate::str::contains(r#""healthcheck_output_path": "#));
}

#[test]
fn test_runtime_doctor_cli_reports_clean_runtime_for_none_profile() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_health_repo(repo.path());

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "doctor", "--runtime-profile", "none"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: clean"))
        .stdout(predicate::str::contains("Runtime profile: none"));
}

#[test]
fn test_runtime_doctor_cli_supports_json_output_with_control_schema() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_health_repo(repo.path());

    ntk()
        .current_dir(repo.path())
        .args([
            "runtime",
            "doctor",
            "--runtime-profile",
            "none",
            "--json-output",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""schema_version": 1"#))
        .stdout(predicate::str::contains(
            r#""schema_kind": "runtime_doctor""#,
        ))
        .stdout(predicate::str::contains(
            r#""runtime_profile_name": "none""#,
        ))
        .stdout(predicate::str::contains(r#""status": "clean""#))
        .stdout(predicate::str::contains(r#""mappings": []"#));
}

#[test]
fn test_runtime_clean_build_artifacts_cli_supports_dry_run() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    fs::create_dir_all(repo.path().join(".build/cargo-target/debug"))
        .expect("build target should be created");
    fs::create_dir_all(repo.path().join("src/MyProject/bin/Debug"))
        .expect("bin directory should be created");
    write_file(
        &repo.path().join(".build/cargo-target/debug/output.bin"),
        "build-output",
    );
    write_file(
        &repo.path().join("src/MyProject/bin/Debug/app.dll"),
        "binary-output",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "clean-build-artifacts", "--dry-run"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: dry-run"))
        .stdout(predicate::str::contains(
            "Discovered artifact directories: 2",
        ))
        .stdout(predicate::str::contains("Discovered bytes:"))
        .stdout(predicate::str::contains(".build"))
        .stdout(predicate::str::contains("src/MyProject/bin"));

    assert!(repo.path().join(".build").exists());
    assert!(repo.path().join("src/MyProject/bin").exists());
}

#[test]
fn test_runtime_clean_build_artifacts_cli_removes_directories_when_forced() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    fs::create_dir_all(repo.path().join(".build/cargo-target/debug"))
        .expect("build target should be created");
    fs::create_dir_all(repo.path().join("src/MyProject/obj/Debug"))
        .expect("obj directory should be created");
    write_file(
        &repo.path().join(".build/cargo-target/debug/output.bin"),
        "build-output",
    );
    write_file(
        &repo.path().join("src/MyProject/obj/Debug/app.obj"),
        "object-output",
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "clean-build-artifacts", "--force"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status: passed"))
        .stdout(predicate::str::contains("Removed artifact directories: 2"))
        .stdout(predicate::str::contains("Reclaimed bytes:"));

    assert!(!repo.path().join(".build").exists());
    assert!(!repo.path().join("src/MyProject/obj").exists());
}

#[test]
fn test_runtime_export_enterprise_trends_cli_writes_dashboard_outputs() {
    let repo = TempDir::new().expect("temporary repository should be created");
    initialize_runtime_repo_root(repo.path());
    write_file(
        &repo.path().join(".temp/audit/validation-ledger.jsonl"),
        "{\"generatedAt\":\"2026-03-29T01:00:00Z\",\"profile\":\"release\",\"warningOnly\":true,\"payloadJson\":\"{\\\"summary\\\":{\\\"totalChecks\\\":4,\\\"passed\\\":3,\\\"warnings\\\":1,\\\"failed\\\":0},\\\"checks\\\":[{\\\"durationMs\\\":5},{\\\"durationMs\\\":7}]}\"}\n",
    );
    write_file(
        &repo.path().join(".temp/audit/validate-all.latest.json"),
        r#"{"profile":"release","summary":{"totalChecks":4,"passed":3,"warnings":1,"failed":0,"suiteWarnings":0},"performance":{"totalDurationMs":12,"averageCheckDurationMs":6.0}}"#,
    );

    ntk()
        .current_dir(repo.path())
        .args(["runtime", "export-enterprise-trends"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Enterprise trends JSON written:"))
        .stdout(predicate::str::contains(
            "Enterprise trends summary written:",
        ));

    assert!(repo
        .path()
        .join(".temp/audit/enterprise-trends.latest.json")
        .is_file());
    assert!(repo
        .path()
        .join(".temp/audit/enterprise-trends.latest.md")
        .is_file());
}