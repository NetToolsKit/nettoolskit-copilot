//! Shared provider surface projection test support.

use std::fs;
use std::path::Path;

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

/// Initialize the minimal provider projection catalog plus authoritative
/// definition trees required by the Rust bootstrap render path.
pub fn initialize_minimal_provider_surface_projection(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github")).expect(".github should be created");
    fs::create_dir_all(repo_root.join(".codex")).expect(".codex should be created");
    fs::create_dir_all(repo_root.join(".vscode")).expect(".vscode should be created");
    fs::create_dir_all(repo_root.join(".claude")).expect(".claude should be created");

    write_file(
        &repo_root.join(".github/governance/provider-surface-projection.catalog.json"),
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
        &repo_root.join("definitions/shared/instructions/super-agent.instructions.md"),
        "# Shared instruction",
    );
    write_file(
        &repo_root.join("definitions/shared/templates/readme-template.md"),
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

/// Initialize a minimal canonical MCP runtime catalog for Codex config tests.
pub fn initialize_minimal_mcp_runtime_catalog(repo_root: &Path) {
    write_file(
        &repo_root.join(".github/governance/mcp-runtime.catalog.json"),
        r#"{"version":1,"inputs":[],"servers":[{"id":"microsoftdocs/mcp","codexName":"microsoftdocs","targets":{"vscode":{"include":true,"enabledByDefault":true},"codex":{"include":true}},"definition":{"type":"http","url":"https://learn.microsoft.com/api/mcp","gallery":"https://example.invalid/gallery","version":"1.0.0"}},{"id":"microsoft/playwright-mcp","codexName":"playwright","targets":{"codex":{"include":true}},"definition":{"type":"stdio","command":"npx","args":["@playwright/mcp@latest"]}},{"id":"vscode-only/example","targets":{"vscode":{"include":true,"enabledByDefault":false}},"definition":{"type":"http","url":"https://example.invalid/vscode-only"}}]}"#,
    );
}
