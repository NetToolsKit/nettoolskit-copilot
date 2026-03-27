# nettoolskit-runtime

> Runtime command boundary contracts for the PowerShell-to-Rust migration.

## Purpose

This crate locks the owned runtime surfaces before full implementation lands. It is the Rust home for:

- runtime operator commands
- runtime lifecycle hooks
- maintenance operator flows
- git hook install and hygiene flows
- local context index update and query commands
- runtime drift diagnosis, remediation, and hygiene checks
- runtime healthcheck orchestration and report generation
- runtime asset bootstrap synchronization
- runtime VS Code template application
- runtime self-heal orchestration and repair reporting

## Scope

The current contract covers `54` legacy PowerShell scripts:

- `42` runtime commands
- `4` runtime hooks
- `5` maintenance commands
- `3` git hook commands

## Public API

- `RUNTIME_SURFACE_CONTRACTS`
- `runtime_surface_contract`
- `require_runtime_surface_contract`
- `runtime_surface_script_total`
- `update_local_context_index`
- `query_local_context_index`
- `export_planning_summary`
- `invoke_runtime_doctor`
- `invoke_runtime_healthcheck`
- `invoke_runtime_bootstrap`
- `invoke_apply_vscode_templates`
- `invoke_runtime_self_heal`
- `UpdateLocalContextIndexRequest`
- `QueryLocalContextIndexRequest`
- `ExportPlanningSummaryRequest`
- `RuntimeDoctorRequest`
- `RuntimeHealthcheckRequest`
- `RuntimeBootstrapRequest`
- `RuntimeApplyVscodeTemplatesRequest`
- `RuntimeSelfHealRequest`