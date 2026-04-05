# nettoolskit-validation

> Rust validation command boundary and Wave 2 execution surface for the PowerShell-to-Rust migration.

---

## Introduction

`nettoolskit-validation` owns the Rust surfaces that replace repository validation, security, governance, documentation, non-runtime test automation, and deploy preflight scripts. It is the top-level validation boundary for the migration program and the place where `validate-all` coordinates the native checks. PowerShell entrypoints remain only as compatibility wrappers for shell-based execution.

---

## Features

- ✅ Locked migration contracts for `43` legacy PowerShell scripts across validation, security, governance, documentation, test automation, and deploy surfaces
- ✅ Native `validate-all` orchestration with profile selection, report generation, and ledger writing
- ✅ Direct Rust checks for agent orchestration, instructions, policies, README standards, routing, templates, and workspace hygiene
- ✅ README standards enforcement backed by the executable repository baseline
- ✅ Dedicated modules for evidence, policy, instruction graph, operational hygiene, documentation, test automation, and workspace validation

---

## Contents

- [Introduction](#introduction)
- [Features](#features)
- [Contents](#contents)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
  - [Example 1: Run validate-all](#example-1-run-validate-all)
  - [Example 2: Validate README standards](#example-2-validate-readme-standards)
  - [Example 3: Inspect the surface catalog](#example-3-inspect-the-surface-catalog)
- [API Reference](#api-reference)
  - [Surface Contracts](#surface-contracts)
  - [Orchestration](#orchestration)
  - [Agent Surfaces](#agent-surfaces)
  - [Policy and Instructions](#policy-and-instructions)
  - [Evidence and Documentation](#evidence-and-documentation)
  - [Operational Hygiene](#operational-hygiene)
  - [Workspace](#workspace)
  - [Data Shapes](#data-shapes)
  - [Errors](#errors)
- [References](#references)
- [License](#license)

---

## Installation

Add the crate as a workspace/path dependency:

```toml
[dependencies]
nettoolskit-validation = { path = "../commands/validation" }
```

---

## Quick Start

Run the default validation orchestration and inspect the final status:

```rust
use nettoolskit_validation::{invoke_validate_all, ValidateAllRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = invoke_validate_all(&ValidateAllRequest::default())?;
    println!("status: {:?}, checks: {}", result.overall_status, result.total_checks);
    Ok(())
}
```

---

## Usage Examples

### Example 1: Run validate-all

```rust
use nettoolskit_validation::{invoke_validate_all, ValidateAllRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = invoke_validate_all(&ValidateAllRequest::default())?;
    println!("exit code: {}", result.exit_code);
    println!("warnings: {}", result.warning_checks);
    Ok(())
}
```

### Example 2: Validate README standards

```rust
use nettoolskit_validation::{
    invoke_validate_readme_standards, ValidateReadmeStandardsRequest,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = invoke_validate_readme_standards(&ValidateReadmeStandardsRequest::default())?;
    println!("files checked: {}", result.files_checked);
    println!("status: {:?}", result.status);
    Ok(())
}
```

### Example 3: Inspect the surface catalog

```rust
use nettoolskit_validation::{
    validation_surface_contract, validation_surface_script_total, MigrationWave,
    ValidationSurfaceKind,
};

fn main() {
    let validation = validation_surface_contract("validation-commands").unwrap();
    println!("surface: {}", validation.surface_id);
    println!("script total: {}", validation_surface_script_total());
    println!("wave: {:?}", MigrationWave::Wave2);
    println!("kind: {:?}", ValidationSurfaceKind::ValidationCommands);
}
```

---

## API Reference

### Surface Contracts

```rust
pub enum MigrationWave { Wave1, Wave2, Wave3 }
pub enum ValidationSurfaceKind {
    ValidationCommands,
    SecurityCommands,
    GovernanceCommands,
    DocumentationCommands,
    DeployCommands,
}

pub struct ValidationSurfaceContract {
    pub surface_id: &'static str,
    pub legacy_root: &'static str,
    pub legacy_script_count: usize,
    pub kind: ValidationSurfaceKind,
    pub wave: MigrationWave,
}

pub const VALIDATION_SURFACE_CONTRACTS: &[ValidationSurfaceContract];
pub fn validation_surface_contract(
    surface_id: &str,
) -> Option<&'static ValidationSurfaceContract>;
pub fn validation_surface_script_total() -> usize;
```

### Orchestration

```rust
pub enum ValidationCheckStatus { Passed, Warning, Failed }

pub struct ValidateAllRequest {
    pub repo_root: Option<std::path::PathBuf>,
    pub validation_profile: Option<String>,
    pub validation_profiles_path: Option<std::path::PathBuf>,
    pub include_all_powershell_scripts: bool,
    pub strict_powershell_standards: bool,
    pub skip_ps_script_analyzer: bool,
    pub warning_only: bool,
    pub write_ledger: bool,
    pub ledger_path: Option<std::path::PathBuf>,
    pub output_path: Option<std::path::PathBuf>,
}

pub struct ValidationCheckResult {
    pub name: String,
    pub script: String,
    pub arguments: Vec<String>,
    pub status: ValidationCheckStatus,
    pub exit_code: i32,
    pub duration_ms: u128,
    pub error: Option<String>,
}

pub struct ValidateAllResult {
    pub repo_root: std::path::PathBuf,
    pub profile_id: String,
    pub warning_only: bool,
    pub output_path: std::path::PathBuf,
    pub ledger_path: Option<std::path::PathBuf>,
    pub archived_broken_ledger_path: Option<std::path::PathBuf>,
    pub checks: Vec<ValidationCheckResult>,
    pub total_checks: usize,
    pub passed_checks: usize,
    pub warning_checks: usize,
    pub failed_checks: usize,
    pub supplemental_failures: usize,
    pub overall_status: ValidationCheckStatus,
    pub exit_code: i32,
    pub suite_warning_messages: Vec<String>,
    pub report_json: String,
}

pub fn invoke_validate_all(
    request: &ValidateAllRequest,
) -> Result<ValidateAllResult, ValidateAllCommandError>;
```

### Agent Surfaces

```rust
pub struct ValidateAgentHooksRequest { ... }
pub struct ValidateAgentOrchestrationRequest { ... }
pub struct ValidateAgentPermissionsRequest { ... }
pub struct ValidateAgentSkillAlignmentRequest { ... }

pub fn invoke_validate_agent_hooks(
    request: &ValidateAgentHooksRequest,
) -> Result<ValidateAgentHooksResult, ValidateAgentHooksCommandError>;
pub fn invoke_validate_agent_orchestration(
    request: &ValidateAgentOrchestrationRequest,
) -> Result<ValidateAgentOrchestrationResult, ValidateAgentOrchestrationCommandError>;
pub fn invoke_validate_agent_permissions(
    request: &ValidateAgentPermissionsRequest,
) -> Result<ValidateAgentPermissionsResult, ValidateAgentPermissionsCommandError>;
pub fn invoke_validate_agent_skill_alignment(
    request: &ValidateAgentSkillAlignmentRequest,
) -> Result<ValidateAgentSkillAlignmentResult, ValidateAgentSkillAlignmentCommandError>;
```

### Policy and Instructions

```rust
pub struct ValidatePolicyRequest { ... }
pub struct ValidateInstructionsRequest { ... }
pub struct ValidatePlanningStructureRequest { ... }
pub struct ValidateAuthoritativeSourcePolicyRequest { ... }
pub struct ValidateInstructionArchitectureRequest { ... }

pub fn invoke_validate_policy(
    request: &ValidatePolicyRequest,
) -> Result<ValidatePolicyResult, ValidatePolicyCommandError>;
pub fn invoke_validate_instructions(
    request: &ValidateInstructionsRequest,
) -> Result<ValidateInstructionsResult, ValidateInstructionsCommandError>;
pub fn invoke_validate_planning_structure(
    request: &ValidatePlanningStructureRequest,
) -> Result<ValidatePlanningStructureResult, ValidatePlanningStructureCommandError>;
pub fn invoke_validate_authoritative_source_policy(
    request: &ValidateAuthoritativeSourcePolicyRequest,
) -> Result<ValidateAuthoritativeSourcePolicyResult, ValidateAuthoritativeSourcePolicyCommandError>;
pub fn invoke_validate_instruction_architecture(
    request: &ValidateInstructionArchitectureRequest,
) -> Result<ValidateInstructionArchitectureResult, ValidateInstructionArchitectureCommandError>;
```

### Evidence and Documentation

```rust
pub struct ValidateAuditLedgerRequest { ... }
pub struct ValidateReadmeStandardsRequest {
    pub repo_root: Option<std::path::PathBuf>,
    pub baseline_path: Option<std::path::PathBuf>,
    pub warning_only: bool,
}
pub struct ValidateInstructionMetadataRequest { ... }
pub struct ValidateRoutingCoverageRequest { ... }
pub struct ValidateTemplateStandardsRequest { ... }

pub fn invoke_validate_audit_ledger(
    request: &ValidateAuditLedgerRequest,
) -> Result<ValidateAuditLedgerResult, ValidateAuditLedgerCommandError>;
pub fn invoke_validate_readme_standards(
    request: &ValidateReadmeStandardsRequest,
) -> Result<ValidateReadmeStandardsResult, ValidateReadmeStandardsCommandError>;
pub fn invoke_validate_instruction_metadata(
    request: &ValidateInstructionMetadataRequest,
) -> Result<ValidateInstructionMetadataResult, ValidateInstructionMetadataCommandError>;
pub fn invoke_validate_routing_coverage(
    request: &ValidateRoutingCoverageRequest,
) -> Result<ValidateRoutingCoverageResult, ValidateRoutingCoverageCommandError>;
pub fn invoke_validate_template_standards(
    request: &ValidateTemplateStandardsRequest,
) -> Result<ValidateTemplateStandardsResult, ValidateTemplateStandardsCommandError>;
```

### Operational Hygiene

```rust
pub struct ValidateRuntimeScriptTestsRequest { ... }
pub struct RefactorTestsToAaaRequest { ... }
pub struct ValidateTestNamingRequest { ... }
pub struct ValidateDeployPreflightRequest { ... }
pub struct ValidateShellHooksRequest { ... }
pub struct ValidateWarningBaselineRequest { ... }

pub fn invoke_refactor_tests_to_aaa(
    request: &RefactorTestsToAaaRequest,
) -> Result<RefactorTestsToAaaResult, RefactorTestsToAaaCommandError>;
pub fn invoke_validate_runtime_script_tests(
    request: &ValidateRuntimeScriptTestsRequest,
) -> Result<ValidateRuntimeScriptTestsResult, ValidateRuntimeScriptTestsCommandError>;
pub fn invoke_validate_test_naming(
    request: &ValidateTestNamingRequest,
) -> Result<ValidateTestNamingResult, ValidateTestNamingCommandError>;
pub fn invoke_validate_deploy_preflight(
    request: &ValidateDeployPreflightRequest,
) -> Result<ValidateDeployPreflightResult, ValidateDeployPreflightCommandError>;
pub fn invoke_validate_shell_hooks(
    request: &ValidateShellHooksRequest,
) -> Result<ValidateShellHooksResult, ValidateShellHooksCommandError>;
pub fn invoke_validate_warning_baseline(
    request: &ValidateWarningBaselineRequest,
) -> Result<ValidateWarningBaselineResult, ValidateWarningBaselineCommandError>;
```

### Workspace

```rust
pub struct ValidateWorkspaceEfficiencyRequest { ... }

pub fn invoke_validate_workspace_efficiency(
    request: &ValidateWorkspaceEfficiencyRequest,
) -> Result<ValidateWorkspaceEfficiencyResult, ValidateWorkspaceEfficiencyCommandError>;
```

### Data Shapes

| Type | Field | Description | Example |
| --- | --- | --- | --- |
| `ValidateAllRequest` | `validation_profile` | Validation profile selected by `validate-all`. | `"dev"` |
| `ValidateAllRequest` | `warning_only` | Convert failures into warnings. | `true` |
| `ValidateAllRequest` | `write_ledger` | Append the run to the validation ledger. | `true` |
| `ValidateReadmeStandardsRequest` | `baseline_path` | Optional override for the README baseline file. | `"definitions/providers/github/governance/readme-standards.baseline.json"` |
| `ValidationSurfaceContract` | `surface_id` | Stable validation surface identifier. | `"validation-commands"` |
| `ValidationSurfaceContract` | `wave` | Migration wave used for cutover planning. | `Wave2` |

### Errors

```rust
pub enum ValidationSurfaceError { UnknownSurface { surface_id: String } }
pub enum ValidateAllCommandError { ResolveWorkspaceRoot { ... } }
pub enum ValidateReadmeStandardsCommandError { ResolveWorkspaceRoot { ... } }
pub enum ValidateDeployPreflightCommandError { ResolveWorkspaceRoot { ... } }
pub enum RefactorTestsToAaaCommandError { ResolveWorkspaceRoot { ... }, ResolveTestFilePath { ... }, ReadTestFile { ... }, WriteTestFile { ... } }
pub enum ValidateTestNamingCommandError { ResolveWorkspaceRoot { ... }, InvalidRequiredUnderscores { ... } }
```

---

## References

- [planning/completed/plan-repository-unification-and-rust-migration.md](../../../planning/completed/plan-repository-unification-and-rust-migration.md)
- [README standards baseline](../../../definitions/providers/github/governance/readme-standards.baseline.json)
- [crates/commands/runtime/README.md](../runtime/README.md)

---

## License

This project is licensed under the MIT License. See the LICENSE file at the repository root for details.

---