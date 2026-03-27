//! Shared fixtures for agent orchestration validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

pub fn initialize_agent_hooks_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".github/hooks/scripts"))
        .expect("hook script directory should be created");
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");
    write_agent_hooks_bootstrap(
        repo_root,
        r#"{
  "hooks": {
    "SessionStart": [{ "type": "command", "command": "pwsh -File session-start.ps1" }],
    "PreToolUse": [{ "type": "command", "command": "pwsh -File pre-tool-use.ps1" }],
    "SubagentStart": [{ "type": "command", "command": "pwsh -File subagent-start.ps1" }]
  }
}"#,
    );
    write_agent_hooks_selector(
        repo_root,
        r#"{
  "version": 1,
  "defaultAgent": {
    "skillName": "super-agent",
    "displayName": "Super Agent"
  },
  "overrideSources": {
    "environment": {
      "skillVariable": "COPILOT_SUPER_AGENT_SKILL",
      "displayVariable": "COPILOT_SUPER_AGENT_NAME"
    },
    "localOverrideFile": "super-agent.selector.local.json"
  }
}"#,
    );
    write_agent_hooks_common_script(
        repo_root,
        "workspace-adapter\nglobal-runtime\n.build/super-agent/planning/active\n.build/super-agent/specs/active\n",
    );
    write_agent_hooks_script(repo_root, "session-start.ps1", "Write-Output 'session'\n");
    write_agent_hooks_script(repo_root, "pre-tool-use.ps1", "Write-Output 'pre'\n");
    write_agent_hooks_script(repo_root, "subagent-start.ps1", "Write-Output 'subagent'\n");
}

pub fn write_agent_hooks_bootstrap(repo_root: &Path, contents: &str) {
    write_file(&repo_root.join(".github/hooks/super-agent.bootstrap.json"), contents);
}

pub fn write_agent_hooks_selector(repo_root: &Path, contents: &str) {
    write_file(&repo_root.join(".github/hooks/super-agent.selector.json"), contents);
}

pub fn write_agent_hooks_common_script(repo_root: &Path, contents: &str) {
    write_file(&repo_root.join(".github/hooks/scripts/common.ps1"), contents);
}

pub fn write_agent_hooks_script(repo_root: &Path, file_name: &str, contents: &str) {
    let path = repo_root.join(".github/hooks/scripts").join(file_name);
    if contents.is_empty() {
        if path.is_file() {
            fs::remove_file(&path).expect("existing hook script should be removed");
        }
        return;
    }

    write_file(&path, contents);
}