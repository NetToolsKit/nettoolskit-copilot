//! Shared fixtures for instruction-graph validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
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

pub fn initialize_instruction_architecture_repo(repo_root: &Path) {
    write_valid_instruction_architecture_manifest(repo_root);
    write_instruction_architecture_documents(repo_root);
    write_canonical_instruction_documents(repo_root);
}

pub fn initialize_validate_instructions_repo(repo_root: &Path) {
    initialize_instruction_architecture_repo(repo_root);
    write_file(
        &repo_root.join("README.md"),
        "# Example\n\nSee [.github/AGENTS.md](.github/AGENTS.md).\n",
    );
    write_file(
        &repo_root.join(".github/chatmodes/example.chatmode.md"),
        "# Example Chatmode\n\nSee [Route Prompt](../prompts/route-instructions.prompt.md).\n",
    );
    write_file(
        &repo_root.join(".github/runbooks/README.md"),
        "# Runbooks\n\nSee [Runtime Drift](runtime-drift.runbook.md).\n",
    );
    write_file(
        &repo_root.join(".github/runbooks/runtime-drift.runbook.md"),
        "# Runtime Drift\n",
    );
    write_file(
        &repo_root.join("docs/samples/manifests/README.md"),
        "# Manifest Samples\n\nSee [App Manifest](app.manifest.yaml).\n",
    );
    write_file(
        &repo_root.join("docs/samples/manifests/app.manifest.yaml"),
        "name: sample-app\nruntime: service\n",
    );
    write_file(
        &repo_root.join(".codex/mcp/README.md"),
        "# MCP\n\nSee [Servers](servers.manifest.json).\n",
    );
    write_file(
        &repo_root.join(".github/schemas/instruction-routing.catalog.schema.json"),
        r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Instruction Routing Catalog",
  "type": "object",
  "properties": {
    "always": {
      "type": "array"
    }
  }
}"#,
    );
    write_governance_file(
        repo_root,
        "local-context-index.catalog.json",
        r#"{
  "includeGlobs": ["planning/**/*.md"]
}"#,
    );
    write_governance_file(
        repo_root,
        "authoritative-source-map.json",
        r#"{
  "stackRules": [
    { "id": "rust", "officialDomains": ["doc.rust-lang.org"] }
  ]
}"#,
    );
    write_governance_file(
        repo_root,
        "instruction-ownership.manifest.json",
        r#"{
  "version": 1,
  "layers": [
    {
      "id": "runtime-projection",
      "pathPatterns": [
        ".github/AGENTS.md"
      ]
    }
  ]
}"#,
    );
    write_governance_file(
        repo_root,
        "mcp-runtime.catalog.json",
        r#"{
  "servers": [
    { "id": "filesystem" }
  ]
}"#,
    );
    write_governance_file(
        repo_root,
        "provider-surface-projection.catalog.json",
        r#"{
  "renderers": [
    { "id": "github" }
  ],
  "surfaces": [
    { "id": "agents", "rendererId": "github" }
  ]
}"#,
    );
    write_governance_file(
        repo_root,
        "validation-profiles.json",
        r#"{
  "version": 1,
  "defaultProfile": "dev",
  "profiles": [
    {
      "id": "dev",
      "warningOnly": true,
      "checkOrder": ["validate-instructions"]
    }
  ]
}"#,
    );
    write_file(
        &repo_root.join(".codex/mcp/servers.manifest.json"),
        r#"{
  "servers": [
    { "id": "filesystem" }
  ]
}"#,
    );
    write_file(
        &repo_root.join(".vscode/base.code-workspace"),
        r#"{
  "folders": [],
  "extensions": {
    "recommendations": ["rust-lang.rust-analyzer"]
  }
}"#,
    );
    write_file(
        &repo_root.join(".vscode/settings.tamplate.jsonc"),
        r#"{
  "chat.instructionsFilesLocations": {
    "%USERPROFILE%\\.github\\instructions": true
  },
  "github.copilot.chat.reviewSelection.instructions": [
    { "file": "%USERPROFILE%\\.github\\AGENTS.md" }
  ],
  "files.exclude": {
    "**/.git": true
  },
  "extensions.autoUpdate": false
}"#,
    );
    write_file(
        &repo_root.join(".vscode/mcp.tamplate.jsonc"),
        r#"{
  "servers": {
    "filesystem": {
      "type": "stdio"
    }
  }
}"#,
    );
    write_file(
        &repo_root.join(".vscode/snippets/codex-cli.tamplate.code-snippets"),
        r#"{
  "Example": {
    "prefix": "codex",
    "body": [
      "Open .github/prompts/route-instructions.prompt.md"
    ]
  }
}"#,
    );
    write_file(
        &repo_root.join(".vscode/snippets/copilot.tamplate.code-snippets"),
        r#"{
  "Example": {
    "prefix": "copilot",
    "body": [
      "Open .github/AGENTS.md"
    ]
  }
}"#,
    );
    write_governance_file(
        repo_root,
        "workspace-efficiency.baseline.json",
        r#"{
  "requiredSettings": {
    "files.exclude": {
      "requiredKeys": ["**/.git"]
    }
  },
  "recommendedSettings": {
    "extensions.autoUpdate": false
  },
  "recommendedNumericUpperBounds": {},
  "forbiddenSettings": {},
  "allowedWorkspaceOverrideSettings": [],
  "heuristics": {
    "maxFolderCountWarning": 4
  }
}"#,
    );
    write_governance_file(
        repo_root,
        "template-standards.baseline.json",
        r#"{
  "templateRules": [
    { "path": ".github/templates/example.md" }
  ]
}"#,
    );
    write_file(
        &repo_root.join(".codex/skills/sample/agents/openai.yaml"),
        "display_name: Sample Skill\nshort_description: Example\ndefault_prompt: $sample\n",
    );
    write_file(
        &repo_root.join("scripts/validation/fixtures/routing-golden-tests.json"),
        r#"{
  "cases": [
    {
      "id": "repository-operating-model",
      "expected_route_ids": ["repo-guidance"],
      "expected_selected_paths": [
        "instructions/governance/ntk-governance-repository-operating-model.instructions.md"
      ]
    }
  ]
}"#,
    );
}

fn write_canonical_instruction_documents(repo_root: &Path) {
    write_file(
        &repo_root.join("definitions/instructions/README.md"),
        "# Instructions\n\nCanonical instruction root.\n",
    );
    write_file(
        &repo_root
            .join("definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md"),
        "# Repository Operating Model\n",
    );
    write_file(
        &repo_root
            .join("definitions/instructions/governance/ntk-governance-authoritative-sources.instructions.md"),
        r#"# Authoritative Sources

Use `governance/authoritative-source-map.json`.
repository context first.
Use official documentation.
Use community sources only as fallback.
"#,
    );
    write_file(
        &repo_root.join(
            "definitions/instructions/governance/ntk-governance-artifact-layout.instructions.md",
        ),
        "# Artifact Layout\n",
    );
    write_file(
        &repo_root
            .join("definitions/instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md"),
        "# Subagent Planning Workflow\n",
    );
    write_file(
        &repo_root
            .join("definitions/instructions/governance/ntk-governance-workflow-optimization.instructions.md"),
        "# Workflow Optimization\n",
    );
    write_file(
        &repo_root.join(
            "definitions/instructions/governance/ntk-governance-feedback-changelog.instructions.md",
        ),
        "# Feedback Changelog\n",
    );
    write_file(
        &repo_root
            .join("definitions/instructions/operations/ntk-operations-powershell-execution.instructions.md"),
        "# PowerShell Execution\n",
    );
    write_file(
        &repo_root.join("definitions/agents/super-agent/ntk-agents-super-agent.instructions.md"),
        "# Super Agent\n",
    );
    write_file(
        &repo_root.join("definitions/providers/github/root/AGENTS.md"),
        r#"# AGENTS

Use `instructions/governance/ntk-governance-repository-operating-model.instructions.md`.
Use `instructions/governance/ntk-governance-authoritative-sources.instructions.md`.
Use `governance/authoritative-source-map.json`.
"#,
    );
    write_file(
        &repo_root.join("definitions/providers/github/root/copilot-instructions.md"),
        r#"# Global Instructions

Use `instructions/governance/ntk-governance-repository-operating-model.instructions.md`.
Use `instructions/governance/ntk-governance-authoritative-sources.instructions.md`.
Use `governance/authoritative-source-map.json`.
"#,
    );
    write_file(
        &repo_root.join("definitions/providers/github/root/instruction-routing.catalog.yml"),
        r#"always:
  - path: AGENTS.md
  - path: copilot-instructions.md
  - path: agents/super-agent/ntk-agents-super-agent.instructions.md
  - path: instructions/governance/ntk-governance-repository-operating-model.instructions.md
  - path: instructions/governance/ntk-governance-artifact-layout.instructions.md
  - path: instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md
  - path: instructions/governance/ntk-governance-workflow-optimization.instructions.md
  - path: instructions/governance/ntk-governance-authoritative-sources.instructions.md
  - path: instructions/operations/ntk-operations-powershell-execution.instructions.md
  - path: instructions/governance/ntk-governance-feedback-changelog.instructions.md
routing:
  - id: repo-guidance
    triggers:
      - repository
    include:
      - path: instructions/governance/ntk-governance-repository-operating-model.instructions.md
"#,
    );
    write_file(
        &repo_root.join("definitions/providers/github/governance/authoritative-source-map.json"),
        r#"{
  "version": 1,
  "defaultPolicy": {
    "repositoryContextFirst": true
  },
  "stackRules": [
    { "id": "dotnet", "displayName": ".NET", "keywords": ["dotnet"], "officialDomains": ["learn.microsoft.com"] },
    { "id": "github-copilot", "displayName": "GitHub Copilot", "keywords": ["copilot"], "officialDomains": ["docs.github.com"] },
    { "id": "vscode", "displayName": "VS Code", "keywords": ["vscode"], "officialDomains": ["code.visualstudio.com"] },
    { "id": "rust", "displayName": "Rust", "keywords": ["rust"], "officialDomains": ["doc.rust-lang.org"] },
    { "id": "vue", "displayName": "Vue", "keywords": ["vue"], "officialDomains": ["vuejs.org"] },
    { "id": "quasar", "displayName": "Quasar", "keywords": ["quasar"], "officialDomains": ["quasar.dev"] },
    { "id": "docker", "displayName": "Docker", "keywords": ["docker"], "officialDomains": ["docs.docker.com"] },
    { "id": "kubernetes", "displayName": "Kubernetes", "keywords": ["kubernetes"], "officialDomains": ["kubernetes.io"] },
    { "id": "postgresql", "displayName": "PostgreSQL", "keywords": ["postgresql"], "officialDomains": ["postgresql.org"] },
    { "id": "openai", "displayName": "OpenAI", "keywords": ["openai"], "officialDomains": ["platform.openai.com"] }
  ]
}"#,
    );
    write_file(
        &repo_root.join("definitions/providers/github/prompts/route-instructions.prompt.md"),
        r#"---
description: Route a request
mode: ask
tools: ['readFile']
---

# Route Instructions

Hard cap: at most 5 selected instruction files (excluding mandatory).
"#,
    );
    write_file(
        &repo_root.join("definitions/providers/github/prompts/example.prompt.md"),
        r#"---
description: Example prompt
mode: ask
tools: ['readFile']
---

# Example Prompt

Use the routing catalog.
"#,
    );
    write_file(
        &repo_root.join("definitions/providers/github/chatmodes/example.chatmode.md"),
        "# Example Chatmode\n\nSee [Route Prompt](../prompts/route-instructions.prompt.md).\n",
    );
    write_file(
        &repo_root.join("definitions/providers/codex/mcp/README.md"),
        "# MCP\n\nSee [Config](codex.config.template.toml).\n",
    );
    write_file(
        &repo_root.join("definitions/providers/codex/mcp/codex.config.template.toml"),
        "[mcp]\nenabled = true\n",
    );
    write_file(
        &repo_root.join("definitions/providers/codex/skills/sample/agents/openai.yaml"),
        "display_name: Sample Skill\nshort_description: Example\ndefault_prompt: $sample\n",
    );
    write_file(
        &repo_root.join("definitions/providers/codex/skills/sample/SKILL.md"),
        r#"---
name: sample
description: sample skill
---

# Sample Skill

Load `ntk-governance-repository-operating-model.instructions.md`.
"#,
    );
    write_file(
        &repo_root.join("definitions/templates/docs/example.md"),
        "# Example Template\n\nUse this as a reusable artifact.\n",
    );
}

pub fn write_valid_instruction_architecture_manifest(repo_root: &Path) {
    write_file(
        &repo_root
            .join("definitions/providers/github/governance/instruction-ownership.manifest.json"),
        r#"{
  "version": 1,
  "intentionalGlobalExceptions": [
    {
      "concern": "Global context must remain visible.",
      "ownedBy": "agent-control"
    }
  ],
  "architectureConstraints": {
    "globalCoreMaxChars": {
      "AGENTS.md": 14000,
      "copilot-instructions.md": 14000
    },
    "routing": {
      "maxAlwaysFiles": 10,
      "maxSelectedFiles": 5,
      "requiredAlwaysPaths": [
        "AGENTS.md",
        "copilot-instructions.md",
        "agents/super-agent/ntk-agents-super-agent.instructions.md",
        "instructions/governance/ntk-governance-repository-operating-model.instructions.md",
        "instructions/governance/ntk-governance-artifact-layout.instructions.md",
        "instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md",
        "instructions/governance/ntk-governance-workflow-optimization.instructions.md",
        "instructions/governance/ntk-governance-authoritative-sources.instructions.md",
        "instructions/operations/ntk-operations-powershell-execution.instructions.md",
        "instructions/governance/ntk-governance-feedback-changelog.instructions.md"
      ]
    }
  },
  "layers": [
    {
      "id": "global-core",
      "pathPatterns": [
        "definitions/providers/github/root/AGENTS.md",
        "definitions/providers/github/root/copilot-instructions.md"
      ]
    },
    {
      "id": "agent-control",
      "pathPatterns": [
        "definitions/agents/*/*.instructions.md"
      ]
    },
    {
      "id": "repository-operating-model",
      "pathPatterns": [
        "definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md"
      ]
    },
    {
      "id": "cross-cutting-policies",
      "pathPatterns": [
        "definitions/instructions/governance/ntk-governance-authoritative-sources.instructions.md",
        "definitions/instructions/governance/ntk-governance-artifact-layout.instructions.md",
        "definitions/providers/github/governance/*"
      ]
    },
    {
      "id": "domain-instructions",
      "pathPatterns": [
        "definitions/instructions/**/*.instructions.md"
      ],
      "excludePatterns": [
        "definitions/instructions/governance/ntk-governance-authoritative-sources.instructions.md",
        "definitions/instructions/governance/ntk-governance-artifact-layout.instructions.md",
        "definitions/instructions/governance/ntk-governance-repository-operating-model.instructions.md"
      ]
    },
    {
      "id": "prompts",
      "pathPatterns": [
        "definitions/providers/github/prompts/*"
      ],
      "forbiddenOwnershipMarkers": [
        "single source of truth",
        "global rules live here",
        "always applied"
      ]
    },
    {
      "id": "templates",
      "pathPatterns": [
        "definitions/templates/**/*",
        "definitions/providers/vscode/workspace/*.tamplate.jsonc",
        "definitions/providers/vscode/workspace/snippets/*.tamplate.code-snippets"
      ],
      "forbiddenOwnershipMarkers": [
        "single source of truth",
        "global rules live here",
        "always applied"
      ]
    },
    {
      "id": "codex-skills",
      "pathPatterns": [
        "definitions/providers/codex/skills/*/SKILL.md"
      ],
      "forbiddenOwnershipMarkers": [
        "single source of truth",
        "global rules live here"
      ]
    },
    {
      "id": "orchestration",
      "pathPatterns": [
        "definitions/providers/codex/orchestration/**/*",
        "scripts/orchestration/*"
      ]
    },
    {
      "id": "runtime-projection",
      "pathPatterns": [
        "scripts/runtime/*"
      ]
    }
  ]
}"#,
    );
}

pub fn write_instruction_architecture_documents(repo_root: &Path) {
    write_file(
        &repo_root.join(".github/AGENTS.md"),
        r#"# AGENTS

Use `instructions/governance/ntk-governance-repository-operating-model.instructions.md`.
Use `instructions/governance/ntk-governance-authoritative-sources.instructions.md`.
"#,
    );
    write_file(
        &repo_root.join(".github/copilot-instructions.md"),
        r#"# Global Instructions

Use `instructions/governance/ntk-governance-repository-operating-model.instructions.md`.
Use `instructions/governance/ntk-governance-authoritative-sources.instructions.md`.
"#,
    );
    write_file(
        &repo_root.join(".github/instruction-routing.catalog.yml"),
        r#"always:
  - path: AGENTS.md
  - path: copilot-instructions.md
  - path: agents/super-agent/ntk-agents-super-agent.instructions.md
  - path: instructions/governance/ntk-governance-repository-operating-model.instructions.md
  - path: instructions/governance/ntk-governance-artifact-layout.instructions.md
  - path: instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md
  - path: instructions/governance/ntk-governance-workflow-optimization.instructions.md
  - path: instructions/governance/ntk-governance-authoritative-sources.instructions.md
  - path: instructions/operations/ntk-operations-powershell-execution.instructions.md
  - path: instructions/governance/ntk-governance-feedback-changelog.instructions.md
routing:
  - id: repo-guidance
    triggers:
      - repository
      - operating model
    include:
      - path: instructions/governance/ntk-governance-repository-operating-model.instructions.md
"#,
    );
    write_file(
        &repo_root.join(".github/prompts/route-instructions.prompt.md"),
        r#"---
description: Route a request
mode: ask
tools: ['readFile']
---

# Route Instructions

Hard cap: at most 5 selected instruction files (excluding mandatory).
"#,
    );
    write_file(
        &repo_root.join(".github/prompts/example.prompt.md"),
        r#"---
description: Example prompt
mode: ask
tools: ['readFile']
---

# Example Prompt

Use the routing catalog.
"#,
    );
    write_file(
        &repo_root.join(".github/templates/example.md"),
        "# Example Template\n\nUse this as a reusable artifact.\n",
    );
    write_file(
        &repo_root.join(".github/policies/example.policy.md"),
        "# Example Policy\n",
    );
    write_file(
        &repo_root.join(".github/instructions/governance/ntk-governance-repository-operating-model.instructions.md"),
        "# Repository Operating Model\n",
    );
    write_file(
        &repo_root.join(
            ".github/instructions/governance/ntk-governance-authoritative-sources.instructions.md",
        ),
        "# Authoritative Sources\n",
    );
    write_file(
        &repo_root.join(".github/agents/super-agent.agent.md"),
        "# Super Agent\n",
    );
    write_file(
        &repo_root
            .join(".github/instructions/governance/ntk-governance-artifact-layout.instructions.md"),
        "# Artifact Layout\n",
    );
    write_file(
        &repo_root.join(".github/instructions/governance/ntk-governance-subagent-planning-workflow.instructions.md"),
        "# Subagent Planning Workflow\n",
    );
    write_file(
        &repo_root.join(
            ".github/instructions/governance/ntk-governance-workflow-optimization.instructions.md",
        ),
        "# Workflow Optimization\n",
    );
    write_file(
        &repo_root.join(
            ".github/instructions/operations/ntk-operations-powershell-execution.instructions.md",
        ),
        "# PowerShell Execution\n",
    );
    write_file(
        &repo_root.join(
            ".github/instructions/governance/ntk-governance-feedback-changelog.instructions.md",
        ),
        "# Feedback Changelog\n",
    );
    write_file(
        &repo_root.join(".github/instructions/example-domain.instructions.md"),
        "# Domain Instruction\n",
    );
    write_file(
        &repo_root.join(".codex/skills/sample/SKILL.md"),
        r#"---
name: sample
description: sample skill
---

# Sample Skill

Load `ntk-governance-repository-operating-model.instructions.md`.
"#,
    );
    write_file(
        &repo_root.join("scripts/orchestration/example.ps1"),
        "Write-Host 'orchestration'\n",
    );
    write_file(
        &repo_root.join("scripts/runtime/bootstrap.ps1"),
        "Write-Host 'runtime'\n",
    );
}