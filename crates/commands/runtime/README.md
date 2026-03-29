# nettoolskit-runtime

> Rust runtime command boundary for repository-managed assets, drift diagnosis, and repair flows.

---

## Introduction

`nettoolskit-runtime` owns the Rust surfaces that replace the legacy runtime, maintenance, hook, and drift-management scripts. It keeps synchronization, inspection, and self-healing logic in one package so the migration can cut over in controlled slices. PowerShell wrappers are retained only where shell-based compatibility or local operator invocation is still required.

---

## Features

- ✅ Locked migration contracts for `54` legacy scripts across runtime, hook, maintenance, and git-hook surfaces
- ✅ Local context index update and query commands for repository navigation
- ✅ Planning summary export for handoff and execution reviews
- ✅ Bootstrap projection for `.github/`, `.codex/`, `.claude/`, and runtime templates
- ✅ Drift diagnosis, healthcheck orchestration, and self-heal repair flows
- ✅ Native application of tracked VS Code workspace templates

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
  - [Example 1: Bootstrap runtime surfaces](#example-1-bootstrap-runtime-surfaces)
  - [Example 2: Update and query the local context index](#example-2-update-and-query-the-local-context-index)
  - [Example 3: Run healthcheck and self-heal](#example-3-run-healthcheck-and-self-heal)
- [API Reference](#api-reference)
  - [Surface Contracts](#surface-contracts)
  - [Context Continuity](#context-continuity)
  - [Sync and Templates](#sync-and-templates)
  - [Diagnostics](#diagnostics)
  - [Data Shapes](#data-shapes)
  - [Errors](#errors)
- [References](#references)
- [License](#license)

---

## Installation

Add the crate as a workspace/path dependency:

```toml
[dependencies]
nettoolskit-runtime = { path = "../commands/runtime" }
```

---

## Quick Start

Use the default bootstrap request to synchronize the runtime projection and inspect the result:

```rust
use nettoolskit_runtime::{invoke_runtime_bootstrap, RuntimeBootstrapRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = invoke_runtime_bootstrap(&RuntimeBootstrapRequest::default())?;
    println!("runtime profile: {}", result.runtime_profile_name);
    println!("provider render: {}", result.provider_rendered);
    Ok(())
}
```

---

## Usage Examples

### Example 1: Bootstrap runtime surfaces

```rust
use nettoolskit_runtime::{invoke_runtime_bootstrap, RuntimeBootstrapRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = invoke_runtime_bootstrap(&RuntimeBootstrapRequest::default())?;
    println!("github enabled: {}", result.github_runtime_enabled);
    println!("codex enabled: {}", result.codex_runtime_enabled);
    Ok(())
}
```

### Example 2: Update and query the local context index

```rust
use nettoolskit_runtime::{
    query_local_context_index, update_local_context_index, QueryLocalContextIndexRequest,
    UpdateLocalContextIndexRequest,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let update = update_local_context_index(&UpdateLocalContextIndexRequest {
        force_full_rebuild: false,
        ..Default::default()
    })?;

    let query = query_local_context_index(&QueryLocalContextIndexRequest {
        repo_root: None,
        query_text: "planning".to_string(),
        catalog_path: None,
        output_root: None,
        top: Some(5),
        exclude_paths: Vec::new(),
    })?;

    println!("indexed files: {}", update.indexed_file_count);
    println!("query hits: {}", query.result_count);
    Ok(())
}
```

### Example 3: Run healthcheck and self-heal

```rust
use nettoolskit_runtime::{
    invoke_runtime_healthcheck, invoke_runtime_self_heal, RuntimeHealthcheckRequest,
    RuntimeSelfHealRequest,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let healthcheck = invoke_runtime_healthcheck(&RuntimeHealthcheckRequest {
        sync_runtime: true,
        strict_extras: true,
        ..Default::default()
    })?;

    let repair = invoke_runtime_self_heal(&RuntimeSelfHealRequest {
        apply_vscode_templates: true,
        ..Default::default()
    })?;

    println!("healthcheck exit code: {}", healthcheck.exit_code);
    println!("self-heal exit code: {}", repair.exit_code);
    Ok(())
}
```

---

## API Reference

### Surface Contracts

```rust
pub enum MigrationWave { Wave1, Wave2, Wave3 }
pub enum RuntimeSurfaceKind { RuntimeCommands, RuntimeHooks, MaintenanceCommands, GitHookCommands }
pub struct RuntimeSurfaceContract {
    pub surface_id: &'static str,
    pub legacy_root: &'static str,
    pub legacy_script_count: usize,
    pub kind: RuntimeSurfaceKind,
    pub wave: MigrationWave,
}

pub const RUNTIME_SURFACE_CONTRACTS: &[RuntimeSurfaceContract];
pub fn runtime_surface_contract(surface_id: &str) -> Option<&'static RuntimeSurfaceContract>;
pub fn runtime_surface_script_total() -> usize;
pub fn require_runtime_surface_contract(
    surface_id: &str,
) -> Result<&'static RuntimeSurfaceContract, RuntimeSurfaceError>;
```

### Context Continuity

```rust
pub struct UpdateLocalContextIndexRequest {
    pub repo_root: Option<std::path::PathBuf>,
    pub catalog_path: Option<std::path::PathBuf>,
    pub output_root: Option<std::path::PathBuf>,
    pub force_full_rebuild: bool,
}

pub struct QueryLocalContextIndexRequest {
    pub repo_root: Option<std::path::PathBuf>,
    pub query_text: String,
    pub catalog_path: Option<std::path::PathBuf>,
    pub output_root: Option<std::path::PathBuf>,
    pub top: Option<usize>,
    pub exclude_paths: Vec<String>,
}

pub struct ExportPlanningSummaryRequest {
    pub repo_root: Option<std::path::PathBuf>,
    pub output_path: Option<std::path::PathBuf>,
    pub print_only: bool,
}

pub fn update_local_context_index(
    request: &UpdateLocalContextIndexRequest,
) -> Result<UpdateLocalContextIndexResult, LocalContextCommandError>;
pub fn query_local_context_index(
    request: &QueryLocalContextIndexRequest,
) -> Result<QueryLocalContextIndexResult, LocalContextCommandError>;
pub fn export_planning_summary(
    request: &ExportPlanningSummaryRequest,
) -> Result<ExportPlanningSummaryResult, PlanningSummaryCommandError>;
```

### Sync and Templates

```rust
pub struct RuntimeBootstrapRequest {
    pub repo_root: Option<std::path::PathBuf>,
    pub target_github_path: Option<std::path::PathBuf>,
    pub target_codex_path: Option<std::path::PathBuf>,
    pub target_agents_skills_path: Option<std::path::PathBuf>,
    pub target_copilot_skills_path: Option<std::path::PathBuf>,
    pub runtime_profile: Option<String>,
    pub fallback_runtime_profile: Option<String>,
    pub mirror: bool,
    pub apply_mcp_config: bool,
    pub backup_config: bool,
}

pub struct RuntimeApplyVscodeTemplatesRequest {
    pub repo_root: Option<std::path::PathBuf>,
    pub vscode_path: Option<std::path::PathBuf>,
    pub force: bool,
    pub skip_settings: bool,
    pub skip_mcp: bool,
}

pub fn invoke_runtime_bootstrap(
    request: &RuntimeBootstrapRequest,
) -> Result<RuntimeBootstrapResult, RuntimeBootstrapCommandError>;
pub fn invoke_apply_vscode_templates(
    request: &RuntimeApplyVscodeTemplatesRequest,
) -> Result<RuntimeApplyVscodeTemplatesResult, RuntimeApplyVscodeTemplatesCommandError>;
```

### Diagnostics

```rust
pub struct RuntimeDoctorRequest {
    pub repo_root: Option<std::path::PathBuf>,
    pub target_github_path: Option<std::path::PathBuf>,
    pub target_codex_path: Option<std::path::PathBuf>,
    pub target_agents_skills_path: Option<std::path::PathBuf>,
    pub target_copilot_skills_path: Option<std::path::PathBuf>,
    pub runtime_profile: Option<String>,
    pub fallback_runtime_profile: Option<String>,
    pub strict_extras: bool,
    pub sync_on_drift: bool,
}

pub struct RuntimeHealthcheckRequest {
    pub repo_root: Option<std::path::PathBuf>,
    pub target_github_path: Option<std::path::PathBuf>,
    pub target_codex_path: Option<std::path::PathBuf>,
    pub target_agents_skills_path: Option<std::path::PathBuf>,
    pub target_copilot_skills_path: Option<std::path::PathBuf>,
    pub runtime_profile: Option<String>,
    pub fallback_runtime_profile: Option<String>,
    pub sync_runtime: bool,
    pub mirror: bool,
    pub strict_extras: bool,
    pub validation_profile: String,
    pub warning_only: bool,
    pub treat_runtime_drift_as_warning: bool,
    pub output_path: Option<std::path::PathBuf>,
    pub log_path: Option<std::path::PathBuf>,
}

pub struct RuntimeSelfHealRequest {
    pub repo_root: Option<std::path::PathBuf>,
    pub target_github_path: Option<std::path::PathBuf>,
    pub target_codex_path: Option<std::path::PathBuf>,
    pub target_agents_skills_path: Option<std::path::PathBuf>,
    pub target_copilot_skills_path: Option<std::path::PathBuf>,
    pub runtime_profile: Option<String>,
    pub fallback_runtime_profile: Option<String>,
    pub mirror: bool,
    pub apply_mcp_config: bool,
    pub backup_config: bool,
    pub apply_vscode_templates: bool,
    pub strict_extras: bool,
    pub output_path: Option<std::path::PathBuf>,
    pub log_path: Option<std::path::PathBuf>,
}

pub fn invoke_runtime_doctor(
    request: &RuntimeDoctorRequest,
) -> Result<RuntimeDoctorResult, RuntimeDoctorCommandError>;
pub fn invoke_runtime_healthcheck(
    request: &RuntimeHealthcheckRequest,
) -> Result<RuntimeHealthcheckResult, RuntimeHealthcheckCommandError>;
pub fn invoke_runtime_self_heal(
    request: &RuntimeSelfHealRequest,
) -> Result<RuntimeSelfHealResult, RuntimeSelfHealCommandError>;
```

### Data Shapes

| Type | Field | Description | Example |
| --- | --- | --- | --- |
| `RuntimeBootstrapRequest` | `mirror` | Mirror source folders into the target runtime tree. | `true` |
| `RuntimeBootstrapRequest` | `apply_mcp_config` | Apply the tracked MCP catalog into Codex config. | `true` |
| `RuntimeBootstrapRequest` | `backup_config` | Create a backup before MCP config application. | `false` |
| `QueryLocalContextIndexRequest` | `query_text` | Search text used against the local context index. | `"planning"` |
| `QueryLocalContextIndexRequest` | `top` | Maximum number of hits to return. | `5` |
| `RuntimeHealthcheckRequest` | `validation_profile` | Validation profile passed into `validate-all`. | `"dev"` |
| `RuntimeHealthcheckRequest` | `sync_runtime` | Run bootstrap before the rest of the healthcheck. | `true` |
| `RuntimeSelfHealRequest` | `apply_vscode_templates` | Apply VS Code templates before the follow-up healthcheck. | `true` |

### Errors

```rust
pub enum RuntimeSurfaceError { UnknownSurface { surface_id: String } }
pub enum LocalContextCommandError { ... }
pub enum PlanningSummaryCommandError { ... }
pub enum RuntimeDoctorCommandError { ... }
pub enum RuntimeBootstrapCommandError { ... }
pub enum RuntimeHealthcheckCommandError { ... }
pub enum RuntimeApplyVscodeTemplatesCommandError { ... }
pub enum RuntimeSelfHealCommandError { ... }
```

---

## References

- [crates/core/README.md](../core/README.md)
- [planning/completed/plan-repository-unification-and-rust-migration.md](../../../planning/completed/plan-repository-unification-and-rust-migration.md)
- [scripts/README.md](../../../scripts/README.md)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---