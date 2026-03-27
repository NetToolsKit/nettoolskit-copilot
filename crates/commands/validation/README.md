# nettoolskit-validation

> Validation command boundary and Wave 2 execution surface for the PowerShell-to-Rust migration.

## Purpose

This crate is the Rust home for:

- repository validation orchestration
- structural and evidence validation commands
- security gates
- governance automation
- documentation validation
- deploy preflight checks

## Current Coverage

The current contract covers `41` legacy PowerShell scripts:

- `31` validation commands
- `6` security commands
- `2` governance commands
- `1` documentation command
- `1` deploy preflight command

The current executable Rust coverage includes:

- `validate-all` orchestration with profile selection, delegated check sequencing, report generation, and hash-chained ledger write/repair
- `validate-planning-structure`
- `validate-audit-ledger`
- `validate-readme-standards`
- `validate-instruction-metadata`
- `validate-routing-coverage`
- `validate-template-standards`
- `validate-workspace-efficiency`

## Module Layout

- `orchestration/` for top-level validation suite orchestration
- `structure/` for repository and planning workspace layout checks
- `evidence/` for ledger and parity evidence validation
- `documentation/` for README and authoring metadata checks
- `governance/` for routing, template, and repository-policy checks
- `workspace/` for VS Code workspace and local authoring footprint efficiency checks

## Public API

- `VALIDATION_SURFACE_CONTRACTS`
- `validation_surface_contract`
- `require_validation_surface_contract`
- `validation_surface_script_total`
- `invoke_validate_all`
- `invoke_validate_planning_structure`
- `invoke_validate_audit_ledger`
- `invoke_validate_readme_standards`
- `invoke_validate_instruction_metadata`
- `invoke_validate_routing_coverage`
- `invoke_validate_template_standards`
- `invoke_validate_workspace_efficiency`