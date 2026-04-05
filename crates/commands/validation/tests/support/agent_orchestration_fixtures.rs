//! Shared fixtures for agent orchestration validation tests.

use std::fs;
use std::path::Path;

pub fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent directory should be created");
    }

    fs::write(path, contents).expect("file should be written");
}

pub fn write_governance_file(repo_root: &Path, file_name: &str, contents: &str) {
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
    write_file(
        &repo_root.join(".github/hooks/super-agent.bootstrap.json"),
        contents,
    );
}

pub fn write_agent_hooks_selector(repo_root: &Path, contents: &str) {
    write_file(
        &repo_root.join(".github/hooks/super-agent.selector.json"),
        contents,
    );
}

pub fn write_agent_hooks_common_script(repo_root: &Path, contents: &str) {
    write_file(
        &repo_root.join(".github/hooks/scripts/common.ps1"),
        contents,
    );
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

pub fn initialize_agent_contract_repo(repo_root: &Path) {
    fs::create_dir_all(repo_root.join(".codex"))
        .expect("codex directory should be created for repository resolution");

    write_file(repo_root.join(".github/AGENTS.md").as_path(), "# Agents\n");
    write_file(
        repo_root.join(".github/copilot-instructions.md").as_path(),
        "# Copilot Instructions\n",
    );
    write_file(
        repo_root
            .join(".github/instruction-routing.catalog.yml")
            .as_path(),
        "version: 1\nroutes: []\n",
    );
    write_file(
        repo_root
            .join(
                ".github/instructions/governance/ntk-governance-repository-operating-model.instructions.md",
            )
            .as_path(),
        "# Repository Operating Model\n",
    );

    write_agents_manifest(repo_root, valid_agents_manifest_json());
    write_permission_matrix(repo_root, valid_permission_matrix_json());
    write_pipeline_manifest(repo_root, valid_pipeline_manifest_json());
    write_eval_fixtures(repo_root, valid_eval_fixtures_json());
    write_handoff_template(repo_root, valid_handoff_template_json());
    write_run_artifact_template(repo_root, valid_run_artifact_template_json());
    write_trace_record_template(repo_root, r#"{ "type": "object" }"#);
    write_policy_evaluations_template(repo_root, r#"{ "type": "object" }"#);
    write_checkpoint_state_template(repo_root, r#"{ "type": "object" }"#);
    write_runtime_policy_catalog(repo_root, r#"{ "version": 1, "rules": [] }"#);
    write_model_routing_catalog(repo_root, r#"{ "version": 1, "rules": [] }"#);

    initialize_agent_skill(repo_root, "super-agent");
    initialize_agent_skill(repo_root, "brainstorm-spec-architect");
    initialize_agent_skill(repo_root, "plan-active-work-planner");
    initialize_agent_skill(repo_root, "context-token-optimizer");
    initialize_agent_skill(repo_root, "dev-software-engineer");
    initialize_agent_skill(repo_root, "test-engineer");
    initialize_agent_skill(repo_root, "review-code-engineer");
    initialize_agent_skill(repo_root, "release-closeout-engineer");

    for relative_path in [
        "scripts/common/agent-runtime-hardening.ps1",
        "scripts/runtime/run-agent-pipeline.ps1",
        "scripts/runtime/resume-agent-pipeline.ps1",
        "scripts/runtime/replay-agent-run.ps1",
        "scripts/runtime/evaluate-agent-pipeline.ps1",
        "scripts/orchestration/engine/invoke-codex-dispatch.ps1",
        "scripts/orchestration/engine/invoke-task-worker.ps1",
        "scripts/orchestration/stages/intake-stage.ps1",
        "scripts/orchestration/stages/spec-stage.ps1",
        "scripts/orchestration/stages/plan-stage.ps1",
        "scripts/orchestration/stages/route-stage.ps1",
        "scripts/orchestration/stages/implement-stage.ps1",
        "scripts/orchestration/stages/validate-stage.ps1",
        "scripts/orchestration/stages/review-stage.ps1",
        "scripts/orchestration/stages/closeout-stage.ps1",
    ] {
        write_repo_file(repo_root, relative_path, "Write-Output 'ok'\n");
    }

    for relative_path in [
        ".codex/orchestration/prompts/super-agent-intake-stage.prompt.md",
        ".codex/orchestration/prompts/spec-stage.prompt.md",
        ".codex/orchestration/prompts/planner-stage.prompt.md",
        ".codex/orchestration/prompts/router-stage.prompt.md",
        ".codex/orchestration/prompts/executor-task.prompt.md",
        ".codex/orchestration/prompts/task-spec-review.prompt.md",
        ".codex/orchestration/prompts/task-quality-review.prompt.md",
        ".codex/orchestration/prompts/reviewer-stage.prompt.md",
        ".codex/orchestration/prompts/closeout-stage.prompt.md",
    ] {
        write_repo_file(repo_root, relative_path, "# Prompt\n");
    }

    for relative_path in [
        ".github/schemas/agent.contract.schema.json",
        ".github/schemas/agent.pipeline.schema.json",
        ".github/schemas/agent.handoff.schema.json",
        ".github/schemas/agent.run-artifact.schema.json",
        ".github/schemas/agent.evals.schema.json",
        ".github/schemas/agent.stage-intake-result.schema.json",
        ".github/schemas/agent.stage-spec-result.schema.json",
        ".github/schemas/agent.stage-plan-result.schema.json",
        ".github/schemas/agent.stage-route-result.schema.json",
        ".github/schemas/agent.stage-implementation-result.schema.json",
        ".github/schemas/agent.stage-review-result.schema.json",
        ".github/schemas/agent.stage-closeout-result.schema.json",
        ".github/schemas/agent.task-review-result.schema.json",
        ".github/schemas/agent.trace-record.schema.json",
        ".github/schemas/agent.policy-evaluation.schema.json",
        ".github/schemas/agent.checkpoint-state.schema.json",
    ] {
        write_repo_file(repo_root, relative_path, r#"{ "type": "object" }"#);
    }
}

pub fn write_repo_file(repo_root: &Path, relative_path: &str, contents: &str) {
    write_file(&repo_root.join(relative_path), contents);
}

pub fn remove_repo_path(repo_root: &Path, relative_path: &str) {
    let path = repo_root.join(relative_path);
    if path.is_dir() {
        fs::remove_dir_all(&path).expect("directory should be removed");
    } else if path.is_file() {
        fs::remove_file(&path).expect("file should be removed");
    }
}

pub fn write_agents_manifest(repo_root: &Path, contents: &str) {
    write_repo_file(
        repo_root,
        ".codex/orchestration/agents.manifest.json",
        contents,
    );
}

pub fn write_permission_matrix(repo_root: &Path, contents: &str) {
    write_governance_file(repo_root, "agent-skill-permissions.matrix.json", contents);
}

pub fn write_pipeline_manifest(repo_root: &Path, contents: &str) {
    write_repo_file(
        repo_root,
        ".codex/orchestration/pipelines/default.pipeline.json",
        contents,
    );
}

pub fn write_eval_fixtures(repo_root: &Path, contents: &str) {
    write_repo_file(
        repo_root,
        ".codex/orchestration/evals/golden-tests.json",
        contents,
    );
}

pub fn write_handoff_template(repo_root: &Path, contents: &str) {
    write_repo_file(
        repo_root,
        ".codex/orchestration/templates/handoff.template.json",
        contents,
    );
}

pub fn write_run_artifact_template(repo_root: &Path, contents: &str) {
    write_repo_file(
        repo_root,
        ".codex/orchestration/templates/run-artifact.template.json",
        contents,
    );
}

pub fn write_trace_record_template(repo_root: &Path, contents: &str) {
    write_repo_file(
        repo_root,
        ".codex/orchestration/templates/trace-record.template.json",
        contents,
    );
}

pub fn write_policy_evaluations_template(repo_root: &Path, contents: &str) {
    write_repo_file(
        repo_root,
        ".codex/orchestration/templates/policy-evaluations.template.json",
        contents,
    );
}

pub fn write_checkpoint_state_template(repo_root: &Path, contents: &str) {
    write_repo_file(
        repo_root,
        ".codex/orchestration/templates/checkpoint-state.template.json",
        contents,
    );
}

pub fn write_runtime_policy_catalog(repo_root: &Path, contents: &str) {
    write_governance_file(repo_root, "agent-runtime-policy.catalog.json", contents);
}

pub fn write_model_routing_catalog(repo_root: &Path, contents: &str) {
    write_governance_file(repo_root, "agent-model-routing.catalog.json", contents);
}

pub fn write_agent_skill_markdown(repo_root: &Path, skill_name: &str, contents: &str) {
    write_repo_file(
        repo_root,
        &format!(".codex/skills/{skill_name}/SKILL.md"),
        contents,
    );
}

pub fn write_agent_skill_openai_yaml(repo_root: &Path, skill_name: &str, contents: &str) {
    write_repo_file(
        repo_root,
        &format!(".codex/skills/{skill_name}/agents/openai.yaml"),
        contents,
    );
}

fn initialize_agent_skill(repo_root: &Path, skill_name: &str) {
    write_agent_skill_markdown(
        repo_root,
        skill_name,
        &format!(
            "---\nname: {skill_name}\n---\nReference .github/AGENTS.md\nReference .github/copilot-instructions.md\nReference .github/instruction-routing.catalog.yml\nReference .github/instructions/governance/ntk-governance-repository-operating-model.instructions.md\n"
        ),
    );
    write_agent_skill_openai_yaml(repo_root, skill_name, "name: openai\n");
}

pub fn valid_agents_manifest_json() -> &'static str {
    r#"{
  "version": 1,
  "agents": [
    {
      "id": "super-agent",
      "role": "planner",
      "skill": "super-agent",
      "allowedPaths": [".github/**", ".codex/**", "planning/**", "scripts/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 16, "maxDurationMinutes": 15, "maxFileEdits": 8, "maxTokens": 45000 },
      "fallbackAgentId": "planner"
    },
    {
      "id": "brainstormer",
      "role": "planner",
      "skill": "brainstorm-spec-architect",
      "allowedPaths": [".github/**", ".codex/**", "planning/**", "scripts/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 18, "maxDurationMinutes": 20, "maxFileEdits": 10, "maxTokens": 55000 },
      "fallbackAgentId": "planner"
    },
    {
      "id": "planner",
      "role": "planner",
      "skill": "plan-active-work-planner",
      "allowedPaths": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 20, "maxDurationMinutes": 20, "maxFileEdits": 12, "maxTokens": 70000 },
      "fallbackAgentId": "router"
    },
    {
      "id": "router",
      "role": "router",
      "skill": "context-token-optimizer",
      "allowedPaths": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 15, "maxDurationMinutes": 15, "maxFileEdits": 8, "maxTokens": 50000 },
      "fallbackAgentId": "specialist"
    },
    {
      "id": "specialist",
      "role": "specialist",
      "skill": "dev-software-engineer",
      "allowedPaths": ["src/**", "modules/**", "samples/**", "tests/**", "scripts/**", ".github/**", ".codex/**", ".temp/**", "README.md", "CHANGELOG.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "approvalRequired": true,
      "approvalInstructions": "approval required",
      "budget": { "maxSteps": 45, "maxDurationMinutes": 50, "maxFileEdits": 40, "maxTokens": 180000 },
      "fallbackAgentId": "tester"
    },
    {
      "id": "tester",
      "role": "tester",
      "skill": "test-engineer",
      "allowedPaths": ["tests/**", "src/**", "modules/**", "samples/**", "scripts/**", ".github/**", ".temp/**"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 25, "maxDurationMinutes": 30, "maxFileEdits": 20, "maxTokens": 120000 },
      "fallbackAgentId": "reviewer"
    },
    {
      "id": "reviewer",
      "role": "reviewer",
      "skill": "review-code-engineer",
      "allowedPaths": ["src/**", "modules/**", "samples/**", "planning/**", "scripts/**", ".github/**", ".codex/**", ".temp/**", "README.md", "CHANGELOG.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "budget": { "maxSteps": 20, "maxDurationMinutes": 25, "maxFileEdits": 10, "maxTokens": 90000 },
      "fallbackAgentId": "release-engineer"
    },
    {
      "id": "release-engineer",
      "role": "release",
      "skill": "release-closeout-engineer",
      "allowedPaths": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"],
      "blockedCommands": ["git reset --hard", "git checkout --"],
      "approvalRequired": true,
      "approvalInstructions": "approval required",
      "budget": { "maxSteps": 20, "maxDurationMinutes": 20, "maxFileEdits": 15, "maxTokens": 80000 }
    }
  ]
}"#
}

pub fn valid_permission_matrix_json() -> &'static str {
    r#"{
  "version": 1,
  "defaultWarningOnly": true,
  "globalRules": {
    "requiredBlockedCommandPrefixes": ["git reset --hard", "git checkout --"],
    "allowedStageScriptPrefixes": ["scripts/orchestration/stages/"]
  },
  "agents": [
    { "agentId": "super-agent", "role": "planner", "skill": "super-agent", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/intake-stage.ps1"], "requiredBudget": { "maxSteps": 16, "maxDurationMinutes": 15, "maxFileEdits": 8, "maxTokens": 45000 } },
    { "agentId": "brainstormer", "role": "planner", "skill": "brainstorm-spec-architect", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/spec-stage.ps1"], "requiredBudget": { "maxSteps": 18, "maxDurationMinutes": 20, "maxFileEdits": 10, "maxTokens": 55000 } },
    { "agentId": "planner", "role": "planner", "skill": "plan-active-work-planner", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/plan-stage.ps1"], "requiredBudget": { "maxSteps": 20, "maxDurationMinutes": 20, "maxFileEdits": 12, "maxTokens": 70000 } },
    { "agentId": "router", "role": "router", "skill": "context-token-optimizer", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/route-stage.ps1"], "requiredBudget": { "maxSteps": 15, "maxDurationMinutes": 15, "maxFileEdits": 8, "maxTokens": 50000 } },
    { "agentId": "specialist", "role": "specialist", "skill": "dev-software-engineer", "allowedPathGlobs": ["src/**", "modules/**", "samples/**", "tests/**", "scripts/**", ".github/**", ".codex/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/implement-stage.ps1"], "requiredBudget": { "maxSteps": 45, "maxDurationMinutes": 50, "maxFileEdits": 40, "maxTokens": 180000 } },
    { "agentId": "tester", "role": "tester", "skill": "test-engineer", "allowedPathGlobs": ["tests/**", "src/**", "modules/**", "samples/**", "scripts/**", ".github/**", ".temp/**"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/validate-stage.ps1"], "requiredBudget": { "maxSteps": 25, "maxDurationMinutes": 30, "maxFileEdits": 20, "maxTokens": 120000 } },
    { "agentId": "reviewer", "role": "reviewer", "skill": "review-code-engineer", "allowedPathGlobs": ["src/**", "modules/**", "samples/**", "planning/**", "scripts/**", ".github/**", ".codex/**", ".temp/**", "README.md", "CHANGELOG.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/review-stage.ps1"], "requiredBudget": { "maxSteps": 20, "maxDurationMinutes": 25, "maxFileEdits": 10, "maxTokens": 90000 } },
    { "agentId": "release-engineer", "role": "release", "skill": "release-closeout-engineer", "allowedPathGlobs": [".github/**", ".codex/**", "planning/**", "scripts/**", ".temp/**", "README.md", "CHANGELOG.md", "CONTRIBUTING.md"], "allowedStageScriptGlobs": ["scripts/orchestration/stages/closeout-stage.ps1"], "requiredBudget": { "maxSteps": 20, "maxDurationMinutes": 20, "maxFileEdits": 15, "maxTokens": 80000 } }
  ]
}"#
}

pub fn valid_pipeline_manifest_json() -> &'static str {
    r#"{
  "id": "default-dev-flow",
  "version": 1,
  "description": "Validation fixture for agent orchestration.",
  "runtime": {
    "policyCatalogPath": "definitions/providers/github/governance/agent-runtime-policy.catalog.json",
    "modelRoutingCatalogPath": "definitions/providers/github/governance/agent-model-routing.catalog.json"
  },
  "stages": [
    { "id": "intake", "agentId": "super-agent", "mode": "plan", "execution": { "scriptPath": "scripts/orchestration/stages/intake-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/super-agent-intake-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-intake-result.schema.json" }, "inputArtifacts": ["request"], "outputArtifacts": ["normalized-request", "intake-report"], "onFailure": "retry-once" },
    { "id": "spec", "agentId": "brainstormer", "mode": "plan", "execution": { "scriptPath": "scripts/orchestration/stages/spec-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/spec-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-spec-result.schema.json" }, "inputArtifacts": ["request", "normalized-request", "intake-report"], "outputArtifacts": ["spec-summary", "active-spec"], "onFailure": "retry-once" },
    { "id": "plan", "agentId": "planner", "mode": "plan", "execution": { "scriptPath": "scripts/orchestration/stages/plan-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/planner-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-plan-result.schema.json" }, "inputArtifacts": ["request", "normalized-request", "intake-report", "spec-summary", "active-spec"], "outputArtifacts": ["task-plan", "task-plan-data", "context-pack", "active-plan"], "onFailure": "stop" },
    { "id": "route", "agentId": "router", "mode": "execute", "execution": { "scriptPath": "scripts/orchestration/stages/route-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/router-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-route-result.schema.json" }, "inputArtifacts": ["task-plan-data", "context-pack", "active-plan"], "outputArtifacts": ["route-selection", "specialist-context-pack"], "onFailure": "retry-once" },
    { "id": "implement", "agentId": "specialist", "mode": "execute", "execution": { "scriptPath": "scripts/orchestration/stages/implement-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/executor-task.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-implementation-result.schema.json" }, "inputArtifacts": ["task-plan", "task-plan-data", "context-pack", "route-selection", "specialist-context-pack", "active-plan"], "outputArtifacts": ["changeset", "implementation-log", "task-review-report"], "onFailure": "retry-once" },
    { "id": "validate", "agentId": "tester", "mode": "validate", "execution": { "scriptPath": "scripts/orchestration/stages/validate-stage.ps1", "dispatchMode": "scripted" }, "inputArtifacts": ["changeset", "implementation-log", "task-review-report"], "outputArtifacts": ["validation-report"], "onFailure": "retry-once" },
    { "id": "review", "agentId": "reviewer", "mode": "review", "execution": { "scriptPath": "scripts/orchestration/stages/review-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/reviewer-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-review-result.schema.json" }, "inputArtifacts": ["changeset", "validation-report", "task-review-report", "active-plan"], "outputArtifacts": ["review-report", "decision-log"], "onFailure": "stop" },
    { "id": "closeout", "agentId": "release-engineer", "mode": "review", "execution": { "scriptPath": "scripts/orchestration/stages/closeout-stage.ps1", "dispatchMode": "codex-exec", "promptTemplatePath": ".codex/orchestration/prompts/closeout-stage.prompt.md", "responseSchemaPath": ".github/schemas/agent.stage-closeout-result.schema.json" }, "inputArtifacts": ["changeset", "validation-report", "review-report", "decision-log", "active-plan"], "outputArtifacts": ["closeout-report", "release-summary", "completed-plan"], "onFailure": "stop" }
  ],
  "handoffs": [
    { "fromStage": "intake", "toStage": "spec", "requiredArtifacts": ["normalized-request", "intake-report"] },
    { "fromStage": "spec", "toStage": "plan", "requiredArtifacts": ["spec-summary", "active-spec"] },
    { "fromStage": "plan", "toStage": "route", "requiredArtifacts": ["task-plan-data", "context-pack", "active-plan"] },
    { "fromStage": "route", "toStage": "implement", "requiredArtifacts": ["route-selection", "specialist-context-pack"] },
    { "fromStage": "implement", "toStage": "validate", "requiredArtifacts": ["changeset", "implementation-log", "task-review-report"] },
    { "fromStage": "validate", "toStage": "review", "requiredArtifacts": ["validation-report"] },
    { "fromStage": "review", "toStage": "closeout", "requiredArtifacts": ["review-report", "decision-log"] }
  ],
  "completionCriteria": {
    "requiredStages": ["intake", "spec", "plan", "route", "implement", "validate", "review", "closeout"],
    "requiredArtifacts": ["intake-report", "spec-summary", "validation-report", "review-report", "decision-log", "closeout-report", "release-summary"]
  }
}"#
}

pub fn valid_eval_fixtures_json() -> &'static str {
    r#"{
  "version": 1,
  "cases": [
    {
      "id": "feature-implementation",
      "expectedPipelineId": "default-dev-flow",
      "expectedStageOrder": ["intake", "spec", "plan", "route", "implement", "validate", "review", "closeout"],
      "requiredAgents": ["super-agent", "brainstormer", "planner", "router", "specialist", "tester", "reviewer", "release-engineer"]
    }
  ]
}"#
}

pub fn valid_handoff_template_json() -> &'static str {
    r#"{
  "fromStage": "plan",
  "toStage": "implement",
  "artifacts": [
    { "name": "task-plan", "path": ".temp/task-plan.md", "checksum": "sha256:replace-me" }
  ]
}"#
}

pub fn valid_run_artifact_template_json() -> &'static str {
    r#"{
  "stages": [
    { "stageId": "intake", "agentId": "super-agent" },
    { "stageId": "spec", "agentId": "brainstormer" },
    { "stageId": "plan", "agentId": "planner" },
    { "stageId": "route", "agentId": "router" },
    { "stageId": "implement", "agentId": "specialist" },
    { "stageId": "validate", "agentId": "tester" },
    { "stageId": "review", "agentId": "reviewer" },
    { "stageId": "closeout", "agentId": "release-engineer" }
  ],
  "summary": {
    "stageCount": 8
  }
}"#
}